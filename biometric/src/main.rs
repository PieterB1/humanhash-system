use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use ark_bn254::{Bn254, Fr};
use ark_ff::PrimeField;
use ark_groth16::{Groth16, ProvingKey, VerifyingKey};
use ark_snark::SNARK;
use ark_std::rand::thread_rng;
use arkworks_gadgets::mimc::MiMC;
use lazy_static::lazy_static;
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Deserialize)]
struct EnrollRequest {
    user_id: String,
    biometric_data: String,
}

#[derive(Deserialize)]
struct VerifyRequest {
    biometric_data: String,
    challenge: String,
}

#[derive(Serialize)]
struct EnrollResponse {
    human_hash: String,
    status: String,
    timestamp: String,
}

#[derive(Serialize)]
struct VerifyResponse {
    status: String,
    timestamp: String,
}

struct BiometricCircuit {
    biometric: Fr,
    hash: Fr,
}

impl ark_relations::r1cs::ConstraintSynthesizer<Fr> for BiometricCircuit {
    fn generate_constraints(
        self,
        cs: ark_relations::r1cs::ConstraintSystemRef<Fr>,
    ) -> ark_relations::r1cs::Result<()> {
        let mimc = MiMC::<Fr>::new(91, 220, vec![Fr::from(0u8); 220]);
        let biometric_var = cs.new_input_variable(|| Ok(self.biometric))?;
        let hash_var = cs.new_witness_variable(|| Ok(self.hash))?;
        let computed_hash = mimc.mimc(cs.clone(), &[biometric_var], None)?;
        cs.enforce_constraint(
            ark_relations::r1cs::lc!() + hash_var,
            ark_relations::r1cs::lc!() + (Fr::from(1u8), ark_relations::r1cs::Variable::One),
            ark_relations::r1cs::lc!() + computed_hash,
        )?;
        Ok(())
    }
}

lazy_static! {
    static ref STORAGE: Mutex<HashMap<String, Fr>> = Mutex::new(HashMap::new());
    static ref PROVING_KEY: ProvingKey<Bn254> = {
        let rng = &mut thread_rng();
        let circuit = BiometricCircuit {
            biometric: Fr::from(0u8),
            hash: Fr::from(0u8),
        };
        let (pk, _) = Groth16::<Bn254>::circuit_specific_setup(circuit, rng).unwrap();
        pk
    };
    static ref VERIFYING_KEY: VerifyingKey<Bn254> = {
        let rng = &mut thread_rng();
        let circuit = BiometricCircuit {
            biometric: Fr::from(0u8),
            hash: Fr::from(0u8),
        };
        let (_, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, rng).unwrap();
        vk
    };
}

#[post("/enroll")]
async fn enroll(req: web::Json<EnrollRequest>) -> impl Responder {
    info!("Received /enroll request: {:?}", req);
    let biometric = match Fr::from_str(&req.biometric_data) {
        Ok(b) => b,
        Err(e) => {
            error!("Invalid biometric data: {}", e);
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid biometric data format"
            }));
        }
    };

    let mut storage = STORAGE.lock().unwrap();
    storage.insert(req.user_id.clone(), biometric);

    HttpResponse::Ok().json(EnrollResponse {
        human_hash: "blue-whale".to_string(),
        status: "enrolled".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

#[post("/verify")]
async fn verify(req: web::Json<VerifyRequest>) -> impl Responder {
    info!("Received /verify request: {:?}", req);
    let biometric = match Fr::from_str(&req.biometric_data) {
        Ok(b) => b,
        Err(e) => {
            error!("Invalid biometric data: {}", e);
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid biometric data format"
            }));
        }
    };

    let stored_hash = STORAGE.lock().unwrap().values().next().cloned().unwrap_or(Fr::from(0u8));
    let circuit = BiometricCircuit {
        biometric,
        hash: stored_hash,
    };

    let rng = &mut thread_rng();
    let proof = match Groth16::<Bn254>::prove(&PROVING_KEY, circuit, rng) {
        Ok(p) => p,
        Err(e) => {
            error!("Proof generation failed: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "error": "Proof generation failed"
            }));
        }
    };

    let is_valid = Groth16::<Bn254>::verify(&VERIFYING_KEY, &[stored_hash], &proof).unwrap_or(false);
    if !is_valid {
        return HttpResponse::Unauthorized().json(json!({
            "error": "Verification failed"
        }));
    }

    HttpResponse::Ok().json(VerifyResponse {
        status: "verified".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Attempting to start server on 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(enroll)
            .service(verify)
    })
    .workers(1)
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    info!("Server bound successfully");
    Ok(())
}

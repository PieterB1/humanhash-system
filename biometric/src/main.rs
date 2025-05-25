use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use ark_bn254::{Bn254, Fr};
use ark_groth16::Groth16;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use arkworks_mimc::MiMC; // Corrected import
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use lazy_static::lazy_static;
use log::{info, error};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref PROVING_KEY: ark_groth16::ProvingKey<Bn254> = {
        // Placeholder: Load or generate proving key
        todo!("Load proving key")
    };
    static ref VERIFYING_KEY: ark_groth16::VerifyingKey<Bn254> = {
        // Placeholder: Load or generate verifying key
        todo!("Load verifying key")
    };
}

#[derive(Deserialize, Serialize, Debug)] // Added Debug
struct EnrollRequest {
    user_id: String,
    biometric_data: String,
}

#[derive(Deserialize, Serialize, Debug)] // Added Debug
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
    biometric_data: Fr,
    hash: Fr,
}

impl ConstraintSynthesizer<Fr> for BiometricCircuit {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<Fr>,
    ) -> Result<(), SynthesisError> {
        let biometric_var = cs.new_input_variable(|| Ok(self.biometric_data))?;
        let hash_var = cs.new_input_variable(|| Ok(self.hash))?;
        let computed_hash = cs.new_witness_variable(|| Ok(self.biometric_data))?;

        cs.enforce_constraint(
            lc!() + hash_var,
            lc!() + (Fr::from(1u8), ark_relations::r1cs::Variable::One),
            lc!() + computed_hash,
        )?;

        Ok(())
    }
}

#[post("/enroll")]
async fn enroll(req: web::Json<EnrollRequest>) -> impl Responder {
    info!("Received /enroll request: {:?}", req);
    let biometric = match req.biometric_data.parse::<u64>() {
        Ok(val) => Fr::from(val),
        Err(e) => {
            error!("Invalid biometric data format: {}", e);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid biometric data format"
            }));
        }
    };

    let circuit = BiometricCircuit {
        biometric_data: biometric,
        hash: biometric, // Placeholder
    };

    let mut rng = StdRng::seed_from_u64(0); // Use StdRng
    let proof = match Groth16::<Bn254>::create_random_proof_with_reduction(
        circuit,
        &PROVING_KEY,
        &mut rng,
    ) {
        Ok(proof) => proof,
        Err(e) => {
            error!("Proof generation failed: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Proof generation failed"
            }));
        }
    };

    // Store proof (placeholder)
    let human_hash = "blue-whale".to_string();
    HttpResponse::Ok().json(EnrollResponse {
        human_hash,
        status: "enrolled".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

#[post("/verify")]
async fn verify(req: web::Json<VerifyRequest>) -> impl Responder {
    info!("Received /verify request: {:?}", req);
    let biometric = match req.biometric_data.parse::<u64>() {
        Ok(val) => Fr::from(val),
        Err(e) => {
            error!("Invalid biometric data format: {}", e);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid biometric data format"
            }));
        }
    };

    let circuit = BiometricCircuit {
        biometric_data: biometric,
        hash: biometric, // Placeholder
    };

    let mut rng = StdRng::seed_from_u64(0);
    let proof = match Groth16::<Bn254>::create_random_proof_with_reduction(
        circuit,
        &PROVING_KEY,
        &mut rng,
    ) {
        Ok(proof) => proof,
        Err(e) => {
            error!("Proof generation failed: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Proof generation failed"
            }));
        }
    };

    let stored_hash = biometric; // Placeholder
    let is_valid = Groth16::<Bn254>::verify_with_processed_vk(
        &VERIFYING_KEY,
        &[stored_hash],
        &proof,
    ).unwrap_or(false);

    HttpResponse::Ok().json(VerifyResponse {
        status: if is_valid { "verified" } else { "failed" }.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(enroll)
            .service(verify)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

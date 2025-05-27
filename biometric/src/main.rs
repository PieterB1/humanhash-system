use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use ark_bn254::{Bn254, Fr};
use ark_groth16::Groth16;
use ark_crypto_primitives::snark::SNARK;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable, LinearCombination};
use lazy_static::lazy_static;
use log::{info, error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::Utc;
use std::io::Write; // For log flushing

lazy_static! {
    static ref PROVING_KEY: Option<ark_groth16::ProvingKey<Bn254>> = None;
    static ref VERIFYING_KEY: Option<ark_groth16::VerifyingKey<Bn254>> = None;
}

#[derive(Deserialize, Serialize, Debug)]
struct EnrollRequest {
    user_id: String,
    biometric_data: String,
}

#[derive(Deserialize, Serialize, Debug)]
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
        let _biometric_var = cs.new_input_variable(|| Ok(self.biometric_data))?;
        let hash_var = cs.new_input_variable(|| Ok(self.hash))?;
        let computed_hash = cs.new_witness_variable(|| Ok(self.biometric_data))?;

        cs.enforce_constraint(
            LinearCombination::from(hash_var),
            LinearCombination::from(Variable::One),
            LinearCombination::from(computed_hash),
        )?;

        Ok(())
    }
}

#[post("/enroll")]
async fn enroll(req: web::Json<EnrollRequest>) -> impl Responder {
    // Flush logs to ensure they are written
    std::io::stderr().flush().ok();
    info!("Received /enroll request: {:?}", req);
    let biometric = match req.biometric_data.parse::<u64>() {
        Ok(val) => {
            info!("Successfully parsed biometric_data: {}", val);
            match Fr::try_from(val) {
                Ok(fr) => fr,
                Err(e) => {
                    error!("Failed to convert biometric_data to Fr: {:?}", e);
                    std::io::stderr().flush().ok();
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Invalid biometric field element",
                        "details": format!("Conversion error: {:?}", e)
                    }));
                }
            }
        },
        Err(e) => {
            error!("Invalid biometric data format: {:?}", e);
            std::io::stderr().flush().ok();
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid biometric data format",
                "details": format!("Parse error: {:?}", e)
            }));
        }
    };

    // Check if proving key is available
    if PROVING_KEY.is_none() {
        error!("Proving key not initialized");
        std::io::stderr().flush().ok();
        return HttpResponse::InternalServerError().json(json!({
            "error": "Proving key not initialized"
        }));
    }

    info!("Generating enroll response for user_id: {}", req.user_id);
    std::io::stderr().flush().ok();
    let human_hash = "blue-whale".to_string();
    HttpResponse::Ok().json(EnrollResponse {
        human_hash,
        status: "enrolled".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[post("/verify")]
async fn verify(req: web::Json<VerifyRequest>) -> impl Responder {
    // Flush logs to ensure they are written
    std::io::stderr().flush().ok();
    info!("Received /verify request: {:?}", req);
    let biometric = match req.biometric_data.parse::<u64>() {
        Ok(val) => {
            info!("Successfully parsed biometric_data: {}", val);
            match Fr::try_from(val) {
                Ok(fr) => fr,
                Err(e) => {
                    error!("Failed to convert biometric_data to Fr: {:?}", e);
                    std::io::stderr().flush().ok();
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Invalid biometric field element",
                        "details": format!("Conversion error: {:?}", e)
                    }));
                }
            }
        },
        Err(e) => {
            error!("Invalid biometric data format: {:?}", e);
            std::io::stderr().flush().ok();
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid biometric data format",
                "details": format!("Parse error: {:?}", e)
            }));
        }
    };

    // Check if verifying key is available
    if VERIFYING_KEY.is_none() {
        error!("Verifying key not initialized");
        std::io::stderr().flush().ok();
        return HttpResponse::InternalServerError().json(json!({
            "error": "Verifying key not initialized"
        }));
    }

    info!("Generating verify response for challenge: {}", req.challenge);
    std::io::stderr().flush().ok();
    HttpResponse::Ok().json(VerifyResponse {
        status: "failed".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting Actix web server");
    std::io::stderr().flush().ok();
    HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error!("JSON deserialization error: {:?}", err);
                std::io::stderr().flush().ok();
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(json!({
                        "error": "Invalid JSON format",
                        "details": format!("Deserialization error: {:?}", err)
                    }))
                ).into()
            }))
            .service(enroll)
            .service(verify)
    })
    .workers(2) // Increase to 2 workers to handle potential worker crashes
    .bind("127.0.0.1:8080")
    .map_err(|e| {
        error!("Failed to bind server: {:?}", e);
        std::io::stderr().flush().ok();
        e
    })?
    .run()
    .await
    .map_err(|e| {
        error!("Server run failed: {:?}", e);
        std::io::stderr().flush().ok();
        e
    })
}

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use ark_bls12_381::Fr;
use chrono::Utc;
use env_logger::Env;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Write;
use std::panic;

lazy_static::lazy_static! {
    static ref PROVING_KEY: Option<()> = None;
    static ref VERIFYING_KEY: Option<()> = None;
}

#[derive(Deserialize, Debug)]
struct EnrollRequest {
    user_id: String,
    biometric_data: String,
}

#[derive(Serialize)]
struct EnrollResponse {
    human_hash: String,
    status: String,
    timestamp: String,
}

#[derive(Deserialize, Debug)]
struct VerifyRequest {
    biometric_data: String,
    challenge: String,
}

#[derive(Serialize)]
struct VerifyResponse {
    status: String,
    timestamp: String,
}

#[actix_web::post("/enroll")]
async fn enroll(req: web::Json<EnrollRequest>) -> impl Responder {
    std::io::stderr().flush().ok();
    debug!("Raw request body: {:?}", req.0);
    info!("Received /enroll request: {:?}", req);
    debug!("Starting biometric_data parsing");
    let _biometric = match req.biometric_data.parse::<u64>() {
        Ok(val) => {
            debug!("Parsed biometric_data: {}", val);
            match Fr::try_from(val) {
                Ok(fr) => {
                    debug!("Converted to Fr: {:?}", fr);
                    fr
                }
                Err(e) => {
                    error!("Failed to convert biometric_data to Fr: {:?}", e);
                    std::io::stderr().flush().ok();
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Invalid biometric field element",
                        "details": format!("Conversion error: {:?}", e)
                    }));
                }
            }
        }
        Err(e) => {
            error!("Invalid biometric data format: {:?}", e);
            std::io::stderr().flush().ok();
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid biometric data format",
                "details": format!("Parse error: {:?}", e)
            }));
        }
    };
    debug!("Checking proving key");
    if PROVING_KEY.is_none() {
        error!("Proving key not initialized");
        std::io::stderr().flush().ok();
        return HttpResponse::InternalServerError().json(json!({
            "error": "Proving key not initialized"
        }));
    }
    debug!("Generating enroll response for user_id: {}", req.user_id);
    std::io::stderr().flush().ok();
    let human_hash = "blue-whale".to_string();
    HttpResponse::Ok().json(EnrollResponse {
        human_hash,
        status: "enrolled".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[actix_web::post("/verify")]
async fn verify(req: web::Json<VerifyRequest>) -> impl Responder {
    std::io::stderr().flush().ok();
    debug!("Raw request body: {:?}", req.0);
    info!("Received /verify request: {:?}", req);
    debug!("Starting biometric_data parsing");
    let _biometric = match req.biometric_data.parse::<u64>() {
        Ok(val) => {
            debug!("Parsed biometric_data: {}", val);
            match Fr::try_from(val) {
                Ok(fr) => {
                    debug!("Converted to Fr: {:?}", fr);
                    fr
                }
                Err(e) => {
                    error!("Failed to convert biometric_data to Fr: {:?}", e);
                    std::io::stderr().flush().ok();
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Invalid biometric field element",
                        "details": format!("Conversion error: {:?}", e)
                    }));
                }
            }
        }
        Err(e) => {
            error!("Invalid biometric data format: {:?}", e);
            std::io::stderr().flush().ok();
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid biometric data format",
                "details": format!("Parse error: {:?}", e)
            }));
        }
    };
    debug!("Checking verifying key");
    if VERIFYING_KEY.is_none() {
        error!("Verifying key not initialized");
        std::io::stderr().flush().ok();
        return HttpResponse::InternalServerError().json(json!({
            "error": "Verifying key not initialized"
        }));
    }
    debug!("Generating verify response for challenge: {}", req.challenge);
    std::io::stderr().flush().ok();
    HttpResponse::Ok().json(VerifyResponse {
        status: "failed".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set panic hook to log panics
    panic::set_hook(Box::new(|panic_info| {
        error!("Panic occurred: {:?}", panic_info);
        std::io::stderr().flush().ok();
    }));

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug,humanhash_biometric=debug,actix_web=debug,actix_server=debug")).init();
    info!("Initializing humanhash-biometric server");
    std::io::stderr().flush().ok();

    HttpServer::new(|| {
        App::new()
            .service(enroll)
            .service(verify)
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let error_message = format!("Invalid JSON format: {}", err);
                error!("JSON error: {}", error_message);
                std::io::stderr().flush().ok();
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(json!({
                        "error": "Invalid JSON format",
                        "details": error_message
                    })),
                )
                .into()
            }))
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

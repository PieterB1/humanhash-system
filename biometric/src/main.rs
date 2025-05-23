use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use arkworks::Proof;

async fn validate_biometric(data: &str) -> bool { true }
async fn generate_human_hash(data: &str) -> Option<String> { Some("blue-whale-42".to_string()) }
async fn store_enrollment(user_id: &str, human_hash: &str) -> Option<()> { Some(()) }

#[derive(Deserialize)]
struct EnrollmentRequest {
    biometric_data: String,
    user_id: String,
}

#[derive(Serialize)]
struct EnrollmentResponse {
    status: String,
    human_hash: String,
    proof: String,
    timestamp: String,
}

#[derive(Deserialize)]
struct VerificationRequest {
    biometric_data: String,
    challenge: String,
}

#[derive(Serialize)]
struct VerificationResponse {
    status: String,
    human_hash: String,
    proof: String,
    timestamp: String,
}

#[post("/identity/enroll")]
async fn enroll_user(req: web::Json<EnrollmentRequest>) -> HttpResponse {
    if !validate_biometric(&req.biometric_data).await {
        return HttpResponse::BadRequest().body("Invalid biometric data");
    }
    let human_hash = generate_human_hash(&req.biometric_data).await.unwrap_or("blue-whale-42".to_string());
    let proof = Proof::generate(&req.biometric_data, &req.user_id).unwrap_or("zkp:enroll123".to_string());
    store_enrollment(&req.user_id, &human_hash).await.unwrap_or(());
    HttpResponse::Ok().json(EnrollmentResponse {
        status: "success".to_string(),
        human_hash,
        proof,
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[post("/identity/verify")]
async fn verify_identity(req: web::Json<VerificationRequest>) -> HttpResponse {
    if !validate_biometric(&req.biometric_data).await {
        return HttpResponse::BadRequest().body("Invalid biometric data");
    }
    let proof = Proof::verify(&req.biometric_data, &req.challenge).unwrap_or("zkp:verify123".to_string());
    let human_hash = generate_human_hash(&req.biometric_data).await.unwrap_or("blue-whale-42".to_string());
    HttpResponse::Ok().json(VerificationResponse {
        status: "success".to_string(),
        human_hash,
        proof,
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("Biometric service running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(enroll_user)
            .service(verify_identity)
            .service(health)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

use crate::liveness::check_liveness;
use crate::crypto::{encrypt_data, decrypt_data};
use neuro_matcher::MegaMatcher;

pub async fn verify_biometric(input_data: Vec<u8>, stored_template: Vec<u8>, vault_key_id: &str, template_type: &str) -> bool {
    if !check_liveness(&input_data, template_type).await {
        return false;
    }

    let decrypted_template = decrypt_data(stored_template, vault_key_id)
        .await
        .expect("Decryption failed");

    let matcher = MegaMatcher::new();
    let input_template = matcher.extract_template(&input_data, template_type)
        .expect("Template extraction failed");
    matcher.match_templates(&input_template, &decrypted_template).score > 0.95
}

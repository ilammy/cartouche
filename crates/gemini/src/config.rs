use rustls::{ClientConfig, ServerCertVerifier};
use std::sync::Arc;

pub fn new_shared_config(verifier: Arc<dyn ServerCertVerifier>) -> Arc<ClientConfig> {
    let mut config = ClientConfig::new();
    config.dangerous().set_certificate_verifier(verifier);
    Arc::new(config)
}

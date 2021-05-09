use cartouche_gemini::config;
use cartouche_gemini::request::Request;
use cartouche_gemini::response::Error;
use cartouche_gemini::verify::{
    CertificateTrustCache, CertificateVerifier, Response, TrustDecision, VerificationDelegate,
    VerificationIssue,
};
use std::sync::Arc;
use webpki::DNSNameRef;
use x509_parser::certificate::X509Certificate;

struct DummyCertificateTrustCache;

impl CertificateTrustCache for DummyCertificateTrustCache {
    fn get_certificate_trust(
        &self,
        _certificate: &X509Certificate<'_>,
        _dns_name: DNSNameRef<'_>,
    ) -> Response {
        Response::UnknownCertificate
    }

    fn trust_certificate_once(
        &self,
        _certificate: &X509Certificate<'_>,
        _dns_name: DNSNameRef<'_>,
    ) {
    }

    fn trust_certificate_always(
        &self,
        _certificate: &X509Certificate<'_>,
        _dns_name: DNSNameRef<'_>,
    ) {
    }
}

struct DummyVerificationDelegate;

impl VerificationDelegate for DummyVerificationDelegate {
    fn decide_certificate_trust(
        &self,
        _certificate: &X509Certificate<'_>,
        _dns_name: DNSNameRef<'_>,
        _issue: VerificationIssue,
    ) -> TrustDecision {
        TrustDecision::TrustTemporary
    }
}

fn main() {
    let verifier = CertificateVerifier::new(DummyVerificationDelegate, DummyCertificateTrustCache);
    let config = config::new_shared_config(Arc::new(verifier));

    // URL without a terminal slash results in a permanent redirect which is not supported yet.
    let mut response = Request::perform("gemini://gemini.circumlunar.space/", &config).expect("request");

    loop {
        let mut buffer = vec![0; 4096];
        match response.read(&mut buffer) {
            Ok(0) => {
                println!("[!] EOF");
                break;
            }
            Ok(read) => {
                buffer.resize(read, 0);
                let string = String::from_utf8(buffer).expect("valid UTF-8");
                println!("[R] received: {}", string);
            }
            Err(Error::Interrupted) => {
                continue;
            }
            Err(Error::Terminated) => {
                println!("[+] terminating connection");
                break;
            }
            Err(Error::Protocol(err)) => {
                println!("[!] protocol error: {}", err);
                break;
            }
            Err(Error::IO(err)) => {
                println!("[!] IO error: {}", err);
                break;
            }
            Err(Error::TLS(err)) => {
                println!("[!] TLS error: {}", err);
                break;
            }
        }
    }
}

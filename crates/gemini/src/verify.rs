use crate::x509;
use rustls::{Certificate, RootCertStore, ServerCertVerified, ServerCertVerifier, TLSError};
use webpki::DNSNameRef;
use x509_parser::certificate::X509Certificate;

/// `ServerCertVerifier` implementing TOFU-style verification.
///
/// Reference: gemini://drewdevault.com/2020/09/21/Gemini-TOFU.gmi
pub struct CertificateVerifier<D, C> {
    delegate: D,
    trust_cache: C,
}

impl<D, C> CertificateVerifier<D, C>
where
    D: VerificationDelegate,
    C: CertificateTrustCache,
{
    pub fn new(delegate: D, trust_cache: C) -> Self {
        Self {
            delegate,
            trust_cache,
        }
    }
}

impl<D, C> ServerCertVerifier for CertificateVerifier<D, C>
where
    D: VerificationDelegate,
    C: CertificateTrustCache,
{
    fn verify_server_cert(
        &self,
        _roots: &RootCertStore,
        presented_certs: &[Certificate],
        dns_name: DNSNameRef<'_>,
        _ocsp_response: &[u8],
    ) -> Result<ServerCertVerified, TLSError> {
        let certificate = presented_certs
            .first()
            .ok_or(TLSError::NoCertificatesPresented)?;

        // TODO(2021-02-14): can we use a lighter parser here?
        // First, parse the alleged certificate data. If it does not parse, that's not valid.
        let (_, certificate) = x509_parser::parse_x509_certificate(certificate.as_ref())
            .map_err(|_| TLSError::WebPKIError(webpki::Error::BadDER))?;

        // Then check whether the certificate is valid for requested domain.
        let validity = x509::check_certificate_for_domain(&certificate, dns_name);
        let issue = match validity {
            // If the certificate is valid, check the cache if it's already trusted.
            // If it's not trusted, postpone the decision for a while.
            Ok(()) => match self
                .trust_cache
                .get_certificate_trust(&certificate, dns_name)
            {
                Response::UnknownCertificate => VerificationIssue::UnknownCertificate,
                Response::FingerprintMismatch => VerificationIssue::FingerprintMismatch,
                Response::TrustedCertificate => {
                    return Ok(ServerCertVerified::assertion());
                }
            },
            Err(err) => VerificationIssue::InvalidCertificate(err),
        };

        // If we see any issues with certificate, ask the delegate wat do.
        match self
            .delegate
            .decide_certificate_trust(&certificate, dns_name, issue)
        {
            TrustDecision::Abort => Err(TLSError::General("certificate rejected".to_owned())),
            TrustDecision::TrustTemporary => {
                self.trust_cache
                    .trust_certificate_once(&certificate, dns_name);
                Ok(ServerCertVerified::assertion())
            }
            TrustDecision::TrustAlways => {
                self.trust_cache
                    .trust_certificate_always(&certificate, dns_name);
                Ok(ServerCertVerified::assertion())
            }
        }
    }
}

pub trait VerificationDelegate: Send + Sync {
    fn decide_certificate_trust(
        &self,
        certificate: &X509Certificate<'_>,
        dns_name: DNSNameRef<'_>,
        issue: VerificationIssue,
    ) -> TrustDecision;
}

pub enum VerificationIssue {
    UnknownCertificate,
    InvalidCertificate(webpki::Error),
    FingerprintMismatch,
}

pub enum TrustDecision {
    Abort,
    TrustTemporary,
    TrustAlways,
}

pub trait CertificateTrustCache: Send + Sync {
    fn get_certificate_trust(
        &self,
        certificate: &X509Certificate<'_>,
        dns_name: DNSNameRef<'_>,
    ) -> Response;

    fn trust_certificate_once(&self, certificate: &X509Certificate<'_>, dns_name: DNSNameRef<'_>);

    fn trust_certificate_always(&self, certificate: &X509Certificate<'_>, dns_name: DNSNameRef<'_>);
}

pub enum Response {
    UnknownCertificate,
    TrustedCertificate,
    FingerprintMismatch,
}

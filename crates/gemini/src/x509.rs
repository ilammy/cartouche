use webpki::DNSNameRef;
use x509_parser::{
    certificate::X509Certificate,
    extensions::{GeneralName, ParsedExtension},
    time::ASN1Time,
};

/// Checks that given certificate is valid for given domain.
///
/// # Security
///
/// This function implements lightweight verification for TOFU-style trust management.
/// If you're here to copy-paste the algorithm and you're not writing a Gemini client,
/// please reconsider.
pub fn check_certificate_for_domain(
    certificate: &X509Certificate<'_>,
    dns_name: DNSNameRef<'_>,
) -> Result<(), webpki::Error> {
    // Check the certificate timestamp for validity.
    let now = ASN1Time::now();
    if now < certificate.validity().not_before {
        return Err(webpki::Error::CertNotValidYet);
    }
    if now > certificate.validity().not_after {
        return Err(webpki::Error::CertExpired);
    }
    // And check that the certificate is issued to the entity we expect.
    if !certificate_matches_domain(certificate, dns_name) {
        return Err(webpki::Error::CertNotValidForName);
    }
    Ok(())
}

fn certificate_matches_domain(certificate: &X509Certificate, dns_name: DNSNameRef<'_>) -> bool {
    let dns_name = dns_name.into();
    // SANs go first. That's where most modern certificates encode their domain.
    // Consider only SAN extensions which contain a DNS name. Disregard others.
    for extension in certificate.extensions().values() {
        if let ParsedExtension::SubjectAlternativeName(san) = extension.parsed_extension() {
            for name in &san.general_names {
                if let GeneralName::DNSName(cert_name) = name {
                    if dns_name_matches(dns_name, cert_name) {
                        return true;
                    }
                }
            }
        }
    }
    // If SANs don't match (or are empty), check the Common Name value(s).
    // We allow 'legacy' behavior of not having a SAN but stating the domain in the Common Name.
    for cn in certificate.subject().iter_common_name() {
        if let Ok(cert_name) = cn.as_str() {
            if dns_name_matches(dns_name, cert_name) {
                return true;
            }
        }
    }
    // Well, the certificate does not seem to be issued to anything that matches.
    false
}

// "webpki" crate refuses to export its DNSName matching for the sake of "security".
// Hence, I implement ilammy's sans-bullshit DNS name matching™ here.
// This is incomplet and incorrekt subset of RFC 6125.
// Known issues include handling of internationalized domain names.
fn dns_name_matches(dns_name: &str, pattern: &str) -> bool {
    if dns_name == pattern {
        return true; // Lucky!
    }
    debug_assert!(dns_name.is_ascii());
    debug_assert!(pattern.is_ascii());
    // Skip possible trailing dots which are allowed in absolute DNS names,
    // then proceed to comparison by DNS name components, in reverse order.
    let mut dns_name_components = dns_name.trim_end_matches('.').split('.').rev();
    let mut pattern_components = pattern.trim_end_matches('.').split('.').rev();
    loop {
        match (dns_name_components.next(), pattern_components.next()) {
            // If all components matched so far, we're good.
            (None, None) => return true,
            // If any of the name runs out of components first, we're not good.
            (Some(_), None) | (None, Some(_)) => return false,
            // If the certificate contains a wildcard, special rules apply.
            // The wildcard is expected to be the leftmost component which matches
            // a single component of `dns_name`, which must also be leftmost.
            (Some(_), Some("*")) => {
                let last_dns_name_component = dns_name_components.next().is_none();
                let last_pattern_component = pattern_components.next().is_none();
                return last_dns_name_component && last_pattern_component;
            }
            // Otherwise, components must match ignoring case. They are expected to be ASCII-only.
            (Some(component), Some(pattern)) => {
                if !component.eq_ignore_ascii_case(pattern) {
                    return false;
                }
            }
        }
    }
}

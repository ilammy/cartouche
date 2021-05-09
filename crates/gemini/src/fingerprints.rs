use sha2::{Digest, Sha256};
use std::fmt;

pub enum Fingerprint {
    SHA256(Vec<u8>),
}

impl Fingerprint {
    pub fn new(data: impl AsRef<[u8]>) -> Self {
        Self::SHA256(Sha256::digest(data.as_ref()).to_vec())
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (tag, bytes) = match self {
            Fingerprint::SHA256(bytes) => ("SHA-256", bytes),
        };
        write!(f, "{}", tag)?;
        for byte in bytes {
            write!(f, ":{:02X}", byte)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_fingerprint() {
        let fingerprint = Fingerprint::new(&[]);
        assert_eq!(
            format!("{}", fingerprint),
            "SHA-256:E3:B0:C4:42:98:FC:1C:14:9A:FB:F4:C8:99:6F:B9:24:27:AE:41:E4:64:9B:93:4C:A4:95:99:1B:78:52:B8:55",
        );
    }
}

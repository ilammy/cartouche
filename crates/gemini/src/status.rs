use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Input               = 10,
    SensitiveInput      = 11,
    Success             = 20,
    TemporaryRedirect   = 30,
    PermanentRedirect   = 31,
    TemporaryFailure    = 40,
    ServerUnavailable   = 41,
    CGIError            = 42,
    ProxyError          = 43,
    SlowDown            = 44,
    PermanentFailure    = 50,
    NotFound            = 51,
    Gone                = 52,
    ProxyRequestRefused = 53,
    BadRequest          = 59,
    ClientCertRequired  = 60,
    NotAuthorized       = 61,
    CertNotValid        = 62,
}

impl TryFrom<u8> for Status {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            10 => Ok(Self::Input),
            11 => Ok(Self::SensitiveInput),
            20 => Ok(Self::Success),
            30 => Ok(Self::TemporaryRedirect),
            31 => Ok(Self::PermanentRedirect),
            40 => Ok(Self::TemporaryFailure),
            41 => Ok(Self::ServerUnavailable),
            42 => Ok(Self::CGIError),
            43 => Ok(Self::ProxyError),
            44 => Ok(Self::SlowDown),
            50 => Ok(Self::PermanentFailure),
            51 => Ok(Self::NotFound),
            52 => Ok(Self::Gone),
            53 => Ok(Self::ProxyRequestRefused),
            59 => Ok(Self::BadRequest),
            60 => Ok(Self::ClientCertRequired),
            61 => Ok(Self::NotAuthorized),
            62 => Ok(Self::CertNotValid),
            other => Err(other),
        }
    }
}

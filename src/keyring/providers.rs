//! Signature providers (i.e. backends/plugins)
pub mod fortanixdsm;
use std::fmt::{self, Display};

/// Enumeration of signing key providers
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SigningProvider {
    /// Fortanix DSM signer    
    FortanixDsm,
}

impl Display for SigningProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigningProvider::FortanixDsm => write!(f, "fortanixdsm"),
        }
    }
}

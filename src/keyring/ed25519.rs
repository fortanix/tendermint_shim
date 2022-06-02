//! Ed25519 signing keys

pub use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};

use crate::{
    error::{Error, ErrorKind::*},
    keyring::SigningProvider,
    prelude::*, fortanixdsm_req::{PluginSigner, PluginRequest},
};
use std::sync::Arc;
use tendermint::TendermintKey;

#[allow(clippy::redundant_allocation)]

/// Ed25519 signer
#[derive(Clone)]
pub struct Signer {
    /// Provider for this signer
    provider: SigningProvider,

    /// Tendermint public key
    public_key: TendermintKey,

    /// Signer trait object
    signer: Arc<Box<dyn signature::Signer<Signature> + Send + Sync>>,
    /// Plugin Signer trait object
    plugin_signer: Arc<Box<dyn PluginSigner + Send + Sync>>
}
#[allow(non_camel_case_types)]

impl Signer {
    /// Create a new signer
    pub fn new(
        provider: SigningProvider,
        public_key: TendermintKey,
        signer: Box<dyn signature::Signer<Signature> + Send + Sync>,
        plugin_signer: Box<dyn PluginSigner +Send + Sync>
    ) -> Self {
        Self {
            provider,
            public_key,
            signer: Arc::new(signer),
            plugin_signer: Arc::new(plugin_signer)
        }
    }

    /// Get the Tendermint public key for this signer
    pub fn public_key(&self) -> TendermintKey {
        self.public_key
    }

    /// Get the provider for this signer
    pub fn provider(&self) -> SigningProvider {
        self.provider
    }
    /// Sign the given message using this signer
    pub fn sign(&self, msg: &[u8]) -> Result<Signature, Error> {
        Ok(self
            .signer
            .try_sign(msg)
            .map_err(|e| format_err!(SigningError, "{}", e))?)
    }
    /// Sign the given message using this plugin signer
    pub fn plugin_sign(&self, req: &PluginRequest)->Result<Signature, Error>{
         Ok(self
            .plugin_signer
            .try_sign(req)
            .map_err(|e| format_err!(SigningError, "{}", e))?)
    }
}

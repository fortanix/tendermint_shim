//! Allows calling the fortanixdsm plugin
use serde::{Deserialize, Serialize};
use ed25519_dalek::{Signature as Ed25519Signature};
use sdkms::api_model::Blob;
use sdkms::{SdkmsClient};
use uuid::Uuid;
use tendermint::consensus;
use crate::{
    error::{Error, ErrorKind::*}
};
use std::sync::Arc;

use crate::{
    prelude::*,
    amino_types::{
        SignedMsgType, 
    },
};


/// The struct for plugin requests to fortanixdsm
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginRequest {
    #[serde(default)] 
    /// Req type can be 0x01 (1), 0x02 (2), 0x20(32)
    pub req_type: u32,
    /// Round is used in double sign check
    pub round: u32,
    /// pol_round is used in double sign check
    pub height: u64,
    /// step is used in double sign check
    pub step: i8,
    /// Tendermint provided block::Id
    pub block_id: Option<tendermint::block::Id>,
    /// Bytes to sign by the plugin
    pub data: Vec<u8>,
}

impl PluginRequest{
    /// Generates a new Plugin Request instance with passed args
    pub fn new(request_state: consensus::State, msg_type: SignedMsgType, data: &Vec<u8>)-> PluginRequest{
        let height:u64   = request_state.height.value();
        let round:u32    = request_state.round.value();
        let step:i8     = request_state.step;
        let block_id:Option<tendermint::block::Id> = request_state.block_id;
        let data_field = data.to_owned();
        info!("req_type: {:?}, round: {:?}, height: {:?}, step: {:?}, block_id: {:?}, data: {:?}", msg_type.to_u32(), round, height, step, block_id, data);
        PluginRequest { req_type: msg_type.to_u32(), round: round, height: height, step: step, block_id: block_id, data: data_field.into()}
    } 
}

/// The struct for pluginSigningKey
pub struct PluginSigningKey {
    client: Arc<SdkmsClient>,
    plugin_uuid: Uuid
}

/// Constructor for PluginSigningKey
impl PluginSigningKey{
    /// Generate a new PluginSigningKey
    pub fn new(
        client: Arc<SdkmsClient>,
        uuid: Uuid
    )->Result<Self, Error>{
        Ok(PluginSigningKey{
            client,
            plugin_uuid: uuid,
        })
    }
}

/// Trait to implement sign methods with plugin
pub trait PluginSigner {
    /// Take pluginRequest as input and return Ed25519 Signature or Error
    fn try_sign(&self, req: &PluginRequest) -> Result<Ed25519Signature, Error>;
}

/// try_sign method for PluginSigner trait
impl PluginSigner for PluginSigningKey{
    fn try_sign(&self, req: &PluginRequest)-> Result<Ed25519Signature, Error>{
        let response: PluginOutput = self.client.invoke_plugin_nice(&self.plugin_uuid, &req)?;
        match response.status {
            400 => {
                fail!(PanicError, &response.message.ok_or_else(|| format_err!(ParseError, "No message field found"))?)
            },
            500 => {
                fail!(DoubleSign, &response.message.ok_or_else(|| format_err!(ParseError, "No message field found"))?)
            }
            _ =>{
                info!("Response.status: {:?}", response.status)
            }
        };
        let signature = &response.signature.ok_or_else(|| format_err!(ParseError, "No signature field found"))?;
        let ed25519_sign = Ed25519Signature::from_bytes(&signature);
        // Map signature::Error to Error using From<signature::Error>
        ed25519_sign.map_err(Error::from)
        }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
/// The structure for the response from Plugin
pub struct PluginOutput{
    /// status code of return
    pub status: u32,
    /// Response message
    pub message: Option<String>,
    /// Signature of data sent to plugin
    pub signature: Option<Blob>,
}


//! Configuration file builder

use super::networks::Network;
use std::{
    fmt::{self, Display},
    path::Path,
};

/// Header to place at the top of `shim.toml`
pub const KMS_CONFIG_HEADER: &str = "# Tendermint KMS configuration file";

/// Configuration file builder
pub struct ConfigBuilder {
    /// Path to the KMS home directory (as a string)
    kms_home: String,

    /// Networks to include in configuration
    networks: Vec<Network>,

    /// Contents of the configuration file in-progress
    contents: String,
}

impl ConfigBuilder {
    /// Create config builder in the default state
    pub fn new(kms_home: impl AsRef<Path>, networks: &[Network]) -> Self {
        let mut result = Self {
            // We need to template the KMS homedir into a config file so we have
            // to convert it into a string
            kms_home: kms_home.as_ref().display().to_string(),
            networks: networks.to_vec(),
            contents: String::new(),
        };

        result.add_str(KMS_CONFIG_HEADER);
        result.add_str("\n\n");

        result
    }

    /// Generate configuration, returning a serialized TOML string
    pub fn generate(mut self) -> String {
        self.add_chain_config();
        self.add_provider_config();
        self.add_validator_config();
        self.contents
    }

    /// Add a comment describing a particular section
    fn add_section_comment(&mut self, section: &str) {
        self.add_str(&format!("## {}\n\n", section));
    }

    /// Add `[[chain]]` configurations
    fn add_chain_config(&mut self) {
        self.add_section_comment("Chain Configuration");

        for network in &self.networks.clone() {
            self.add_template(match network {
                Network::Columbus => include_str!("templates/networks/columbus.toml"),
                Network::CosmosHub => include_str!("templates/networks/cosmoshub.toml"),
                Network::IrisHub => include_str!("templates/networks/irishub.toml"),
                Network::SentinelHub => include_str!("templates/networks/sentinelhub.toml"),
                Network::Osmosis => include_str!("templates/networks/osmosis.toml"),
                Network::Persistence => include_str!("templates/networks/persistence.toml"),
            });
        }
    }

    /// Add `[[provider]]` configuration (customized for enabled signing providers)
    fn add_provider_config(&mut self) {
        self.add_section_comment("Signing Provider Configuration");
        self.add_fortanixdsm_provider_config();
    }

    /// Add `[[validator]]` configurations
    fn add_validator_config(&mut self) {
        self.add_section_comment("Validator Configuration");
        self.add_template_with_chain_id(include_str!("templates/validator.toml"));
    }

    

    /// Add `[[provider.fortanixdsm]]` configuration
    
    fn add_fortanixdsm_provider_config(&mut self) {
        self.add_str("### Fortanix DSM Signer Configuration\n\n");
        self.add_template_with_chain_id(include_str!("templates/keyring/fortanixdsm.toml"));
    }

    /// Append a template to the config file, substituting `$KMS_HOME`
    fn add_template(&mut self, template: &str) {
        self.add_str(&format_template(
            template,
            &[("$KMS_HOME", self.kms_home.as_ref())],
        ));

        self.add_str("\n\n");
    }

    /// Append a template to the config file, substituting `$KMS_HOME` and `$CHAIN_ID`
    fn add_template_with_chain_id(&mut self, template: &str) {
        for network in self.networks.clone() {
            self.add_str(&format_template(
                template,
                &[
                    ("$KMS_HOME", self.kms_home.as_ref()),
                    ("$CHAIN_ID", network.chain_id()),
                ],
            ));

            self.add_str("\n\n");
        }
    }

    /// Add a string to `self.contents`
    fn add_str(&mut self, str: impl AsRef<str>) {
        self.contents.push_str(str.as_ref())
    }
}

impl Display for ConfigBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.contents)
    }
}

/// Apply the given set of substitutions and trim newlines
fn format_template(template: &str, substitutions: &[(&str, &str)]) -> String {
    substitutions.iter().fold(
        template.trim_end().to_owned(),
        |string, (name, replacement)| string.replace(name, replacement),
    )
}

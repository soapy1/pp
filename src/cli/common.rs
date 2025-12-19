use oci_client::{secrets::RegistryAuth, Reference};
use docker_credential::{CredentialRetrievalError, DockerCredential};

pub const PIXI_LOCK: &str  = "pixi.lock";
pub const PIXI_TOML: &str  = "pixi.toml";
pub const LAYER_PIXI_CONFIG_MEDIA_TYPE: &str = "application/vnd.pixi.config.v1+toml";
pub const LAYER_PIXI_TOML_MEDIA_TYPE: &str = "application/vnd.pixi.toml.v1+toml";
pub const LAYER_PIXI_LOCK_MEDIA_TYPE: &str = "application/vnd.pixi.lock.v1+yaml";

pub fn build_client_config() -> oci_client::client::ClientConfig {
    // TODO: support configuring the client 
    let protocol = oci_client::client::ClientProtocol::Http;

    oci_client::client::ClientConfig {
        protocol,
        ..Default::default()
    }
}

pub fn build_auth(reference: &Reference) -> RegistryAuth {
    let server = reference
        .resolve_registry()
        .strip_suffix('/')
        .unwrap_or_else(|| reference.resolve_registry());

    match docker_credential::get_credential(server) {
        Err(CredentialRetrievalError::ConfigNotFound) => RegistryAuth::Anonymous,
        Err(CredentialRetrievalError::NoCredentialConfigured) => RegistryAuth::Anonymous,
        Err(e) => panic!("Error handling docker configuration file: {e}"),
        Ok(DockerCredential::UsernamePassword(username, password)) => {
            RegistryAuth::Basic(username, password)
        }
        Ok(DockerCredential::IdentityToken(_)) => {
            RegistryAuth::Anonymous
        }
    }
}

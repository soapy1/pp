use clap::Parser;
use std::path::Path;
use std::collections::BTreeMap;
use oci_client::{
    annotations,
    client::{Config, ImageLayer},
    manifest,
    Client,
    Reference,
};
use crate::cli::common;

#[derive(Parser, Debug, Default)]
pub struct Args {
    /// path of project to push
    #[arg()]
    path: String,

    /// name of the tag
    #[arg()]
    tag: String,
}

pub async fn execute(args: Args)  {
    let pixi_toml_file = format!("{}/{}", args.path, common::PIXI_TOML);
    let pixi_lock_file = format!("{}/{}", args.path, common::PIXI_LOCK);
    let pixi_toml_path = Path::new(&pixi_toml_file);
    let pixi_lock_path = Path::new(&pixi_lock_file);

    if !(pixi_lock_path.is_file() && pixi_toml_path.is_file()) {
        eprintln!(
            "Error: Could not find {} or {} in the specified path.",
            common::PIXI_TOML, common::PIXI_LOCK
        );
        std::process::exit(1)
    }

    let reference: Reference = args.tag.parse().expect("Not a valid image reference");
    let client_config = common::build_client_config();
    let client = Client::new(client_config);
    let auth = common::build_auth(&reference);

    let mut annotations: BTreeMap<String, String> = BTreeMap::new();
    annotations.insert(
        annotations::ORG_OPENCONTAINERS_IMAGE_DESCRIPTION.to_string(),
        args.tag.clone(),
    );

    let pixi_toml_data: Vec<u8> = tokio::fs::read(pixi_toml_path)
        .await
        .expect("Cannot read pixi.toml from disk");
    let pixi_lock_data: Vec<u8> = tokio::fs::read(pixi_lock_path)
        .await
        .expect("Cannot read pixi.lock from disk");

    let mut annotations_layer1: BTreeMap<String, String> = BTreeMap::new();
    annotations_layer1.insert(
        annotations::ORG_OPENCONTAINERS_IMAGE_TITLE.to_string(),
        "pixi.toml".to_string(),
    );
    let layer_1 = ImageLayer::new(
            pixi_toml_data,
            common::LAYER_PIXI_TOML_MEDIA_TYPE.to_string(),
            Some(annotations_layer1),
        );
    let mut annotations_layer2: BTreeMap<String, String> = BTreeMap::new();
    annotations_layer2.insert(
        annotations::ORG_OPENCONTAINERS_IMAGE_TITLE.to_string(),
        "pixi.lock".to_string(),
    );
    let layer_2 = ImageLayer::new(
            pixi_lock_data,
            common::LAYER_PIXI_LOCK_MEDIA_TYPE.to_string(),
            Some(annotations_layer2),
        );

    let layers = vec![layer_1, layer_2];

    let config_data = serde_json::json!({}).to_string().into_bytes();
    let config = Config {
        data: config_data,
        media_type: common::LAYER_PIXI_CONFIG_MEDIA_TYPE.to_string(),
        annotations: None,
    };

    let image_manifest = manifest::OciImageManifest::build(&layers, &config, Some(annotations));

    let response = client
        .push(&reference, &layers, config, &auth, Some(image_manifest))
        .await
        .map(|push_response| push_response.manifest_url)
        .expect("Cannot push pixi environment");

    println!("Successfully pushed: {response:?}");
}

use clap::Parser;
use std::path::Path;
use oci_client::{
    annotations,
    Client,
    Reference,
};
use crate::cli::common;


#[derive(Parser, Debug, Default)]
pub struct Args {
    /// destination path to pull project to. Must be a directory.
    #[arg()]
    path: String,

    /// name of the tag
    #[arg()]
    tag: String,
}

pub async fn execute(args: Args) {
    // Create output directory if it doesn't exist
    tokio::fs::create_dir_all(&args.path).await.expect("Cannot create output directory");

    let reference: Reference = args.tag.parse().expect("Not a valid image reference");
    let client_config = common::build_client_config();
    let client = Client::new(client_config);
    let auth = common::build_auth(&reference);

    let media_types = vec![
        common::LAYER_PIXI_CONFIG_MEDIA_TYPE,
        common::LAYER_PIXI_LOCK_MEDIA_TYPE,
        common::LAYER_PIXI_TOML_MEDIA_TYPE,
    ];

    let artifact = client
        .pull(&reference, &auth, media_types)
        .await
        .expect("Cannot pull pixi environment");

    for (index, layer) in artifact.layers.iter().enumerate() {
        // Get filename from layer annotations or use default
        let filename = layer
            .annotations
            .as_ref()
            .and_then(|ann| ann.get(&annotations::ORG_OPENCONTAINERS_IMAGE_TITLE.to_string()))
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("layer_{}", index));

        let file_path = Path::new(&args.path).join(&filename);

        tokio::fs::write(file_path, &layer.data)
            .await
            .expect("Cannot write to file");
    }
}

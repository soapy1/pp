use clap::Parser;

#[derive(Parser, Debug, Default)]
pub struct Args {
    /// destination path to pull project to
    #[arg()]
    path: String,

    /// name of the tag
    #[arg()]
    tag: String,
}

pub async fn execute(_args: Args) {
    println!("Pulling?")
}

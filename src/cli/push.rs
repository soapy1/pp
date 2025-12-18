use clap::Parser;

#[derive(Parser, Debug, Default)]
pub struct Args {
    /// name of the tag
    #[arg(help = "Name of the tag")]
    tag: String,
}

pub async fn execute(_args: Args) {
    println!("Pushing?")
}

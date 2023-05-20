use clap::Parser;

use crate::rfc_index::get_rfc_index;
use crate::rfc_parser::Rfc;

mod rfc_index;
mod rfc_parser;

#[derive(Debug, Parser)]
#[command(name = "rfc-datasource")]
#[clap(author="Nasqueron project", version, about="Download and print RFC index", long_about=None)]
pub struct RfcArgs {
    /// The format string to use
    #[arg(long, short = 'f')]
    format: String,

    /// The path to the RFC index source
    #[arg(long, short = 's')]
    source: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = RfcArgs::parse(); // Will exit if argument is missing or --help/--version provided.

    let document = get_rfc_index(args.source).await
        .expect("Can't read or fetch RFC index");

    for rfc in Rfc::parse_document(&document) {
        println!("{}", rfc.format(&args.format));
    }
}

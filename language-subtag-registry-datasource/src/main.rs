use clap::Parser;

use crate::registry::get_registry;
use crate::language_parser::Language;

mod registry;
mod language_parser;

#[derive(Debug, Parser)]
#[command(name = "language-subtag-registry-datasource")]
#[clap(author="Nasqueron project", version, about="Download and print language subtag registry", long_about=None)]
pub struct Args {
    /// The format string to use
    #[arg(long, short = 'f')]
    format: String,

    /// The aggregation separator
    #[arg(long, short = 'a', default_value = " / ")]
    aggregation_separator: String,

    /// The path to the registry source
    #[arg(long, short = 's')]
    source: Option<String>,

    /// Restricts parsing to language type
    #[arg(long, short = 'l', default_value_t = false)]
    languages_only: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse(); // Will exit if argument is missing or --help/--version provided.

    let document = get_registry(args.source).await
        .expect("Can't read or fetch registry");

    for language in Language::parse_document(&document, args.languages_only) {
        println!("{}", language.format(&args.format, &args.aggregation_separator));
    }
}

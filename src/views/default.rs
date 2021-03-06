use clap::Parser;

// use super::super::claping;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(default_value_t = String::from("alice"), value_parser)]
    name: String,
}

pub fn cli_default() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name);
}
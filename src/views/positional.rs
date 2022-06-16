use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    name: Option<String>,
}

pub fn cli_positional() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name.as_deref());
}
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)] //  is optional
pub struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    pub name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    pub count: u8,
}

pub fn get_args() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}

// #![allow(unused_imports)]
// use regex::Regex;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use super::super::images::{
    // fractal::fractal,
    mandelbrot::mandelbrot,
    // fractal::saludo,
    // mandelbrot::saludo,
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[clap(value_parser)]
    name: Option<String>,

    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "mandelbrot.png")]
    // config: Option<PathBuf>,
    out: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long, action)]
        list: bool,
    },
}

pub fn cli_quick() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        // // (?i)\.(jpg|png)$
        // let re = Regex::new(r"(?i)\.(jpg|png)$").unwrap();
        // let caps = re.captures(name).unwrap();
        if  name.ends_with( ".jpg" ) || name.ends_with( ".png" ) {
            // println!("{}", name);
            // fractal(&name)
            mandelbrot(&name);
        }
        else {
            mandelbrot(&vec![name, ".png"].concat());
        }

        // fractal();
    }

    if let Some(config_path) = cli.out.as_deref() {
        println!("Value for output: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}
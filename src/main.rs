use anyhow::{Context, Result};
use args::Args;
use clap::Parser;
use mini_markup::mxml_to_xml;
use std::fs;

mod args;

struct Printer {
    verbose: bool,
}

impl Printer {
    fn print(&self, s: &str) {
        // print if not silenced
        if self.verbose {
            print!("{s}");
        }
    }

    fn println(&self, s: &str) {
        if self.verbose {
            println!("{s}");
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let p = Printer {
        verbose: args.verbose,
    };
    p.println("Program started, arguments parsed");

    p.print("Reading file... ");

    let f = fs::read_to_string(&args.input_file).context("failed to read file")?;

    p.println("File read success");
    p.print("Performing conversion... ");

    let result = mxml_to_xml(f).context("mxml conversion failed")?;
    p.println("Conversion success");

    p.println("Attempting file write");
    fs::write(args.output_file, result).context("Failed to write to file")?;
    p.println("Sucess!");

    Ok(())
}

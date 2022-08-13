use std::{fs};
use clap::Parser;
use anyhow::{Result, Context};
use mini_markup::{mxml_string_to_xml, replace_bracket_escapes};

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
    let args = args::Args::parse();

    let p = Printer {verbose: args.verbose};
    p.println("Program started, arguments parsed");

    p.print("Reading file... ");

    let f = fs::read_to_string(&args.input_file).context("failed to read file")?;

    p.println("File read success");
    p.print("Performing conversion... ");

    let result = mxml_string_to_xml(f).context("mxml conversion failed")?;
    p.println("Conversion success");

    p.print("Replacing custom escape characters");
    let result = replace_bracket_escapes(result);
    p.println("Successfully replaced curly bracket escapes");

    p.println("Attempting file write");
    fs::write(args.output_file, result).context("Failed to write to file")?;
    p.println("Sucess!");

    Ok(())
}

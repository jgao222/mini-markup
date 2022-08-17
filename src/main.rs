use anyhow::{Context, Result};
use args::Args;
use clap::Parser;
use mini_markup::{html_to_mxml, mxml_to_html, mxml_to_xml, xml_to_mxml};
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

    let file_string = fs::read_to_string(&args.input_file).context("failed to read file")?;

    p.println("File read success");
    p.print("Performing conversion... ");

    let function = match args.target {
        Some(args::Target::Mxml) => {
            if args.html {
                html_to_mxml
            } else {
                xml_to_mxml
            }
        }
        Some(args::Target::Xml) | None => {
            if args.html {
                mxml_to_html
            } else {
                mxml_to_xml
            }
        }
    };

    let result = function(file_string).context("conversion failed")?;
    p.println("Conversion success");

    p.println("Attempting file write");
    fs::write(args.output_file, result).context("Failed to write to file")?;
    p.println("Sucess!");

    Ok(())
}

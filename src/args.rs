use std::path::PathBuf;

use clap::{clap_derive::ArgEnum, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
pub struct Args {
    #[clap(short, long, action)]
    /// Print verbose output
    pub verbose: bool,
    #[clap(short, long, action)]
    /// Use HTML void-element aware conversion (otherwise HTML conversion may not work correctly)
    pub html: bool,
    #[clap(short, long, arg_enum, value_parser)]
    /// The target (output) format, defaults to XML
    pub target: Option<Target>,
    #[clap(value_parser)]
    /// The file path of the file to convert from
    pub input_file: PathBuf,
    #[clap(value_parser)]
    /// The file path of the output file
    pub output_file: PathBuf,
}

#[derive(Clone, Copy, ArgEnum)]
pub enum Target {
    Mxml,
    Xml,
}

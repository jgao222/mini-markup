use std::path::PathBuf;

use clap::{Parser, clap_derive::ArgEnum};

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
pub struct Args {
    #[clap(short, long, action)]
    /// Whether the program should print verbose output
    pub verbose: bool,
    #[clap(short, long, arg_enum, value_parser)]
    /// The target format, defaults to XML
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
    MXML,
    XML
}
use clap::{Args, Parser};

#[derive(Debug,Parser)]
#[command (author, version, about)]
pub struct LpfArgs {
    /// Input file containing .lpf format
    #[arg(short, long, value_name = "FILE")]
    pub input_file: String,
    /// Output file where .html should be written to
    #[arg(short, long, value_name = "FILE",default_value= "index.html")]
    pub output_file: String
}


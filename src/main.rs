mod lpftokens;
mod args;

use args::LpfArgs;
use clap::Parser;
use lpftokens::Slide;

use std::{fs::File, io::{Read, Write}};

fn main() {
    let args = LpfArgs::parse();
    
    // Reading logic
    let mut content = String::new();
    let mut input_file = File::open(args.input_file).unwrap();
    input_file.read_to_string(&mut content);

    // Create a new slide
    let slide = Slide::new(&content, args.css);
    
    // Writing to output file
    let mut output_file = File::create(&args.output_file).unwrap();
    output_file.write_all(slide.to_html().as_bytes());

    println!("Successfully written to {}",&args.output_file);
}

mod tests;
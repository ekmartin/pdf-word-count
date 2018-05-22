extern crate clap;
extern crate pdf_word_count;

use clap::{App, Arg};
use pdf_word_count::Collector;
use std::fs::File;
use std::io;

fn main() {
    let args = App::new("pdf-wc")
        .version("0.1.0")
        .about("Displays the number of lines, words, and characters in a PDF.")
        .arg(
            Arg::with_name("file")
                .help("File to read from")
                .required(false),
        )
        .get_matches();

    let word_count = match args.value_of("file") {
        Some(filename) => {
            let file = File::open(filename).expect("Failed opening file");
            Collector::process_document(file)
        }
        None => Collector::process_document(io::stdin()),
    };

    println!("{}", word_count);
}

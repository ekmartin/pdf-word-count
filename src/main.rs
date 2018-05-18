extern crate pdf_word_count;

use std::io;
use pdf_word_count::Collector;

fn main() {
    println!("{}", Collector::process_document(io::stdin()))
}

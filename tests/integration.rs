extern crate pdf_word_count;

use std::fs::File;

use pdf_word_count::{Collector, WordCount};

fn process(fixture: &str) -> WordCount {
    let filename = format!("tests/fixtures/{}.pdf", fixture);
    Collector::process_document(File::open(filename).unwrap())
}

#[test]
fn single_word() {
    let wc = process("single");
    assert_eq!(wc.words, 1);
    assert_eq!(wc.lines, 1);
    assert_eq!(wc.characters, 6);
}

#[test]
fn multiple_words() {
    let wc = process("multiple");
    assert_eq!(wc.words, 5);

    // TODO: This should be 3, not 9!
    assert_eq!(wc.lines, 9);
    assert_eq!(wc.characters, 46);
}

#[test]
fn empty() {
    let wc = process("empty");
    assert_eq!(wc.words, 0);
    assert_eq!(wc.lines, 0);
    assert_eq!(wc.characters, 0);
}

#[test]
fn hyphen() {
    let wc = process("hyphen");
    assert_eq!(wc.words, 6);
    // TODO: Technically, this is wrong since the word is hyphenated onto a second line. Not sure
    // if it matters too much though.
    assert_eq!(wc.lines, 1);
    assert_eq!(wc.characters, 82);
}

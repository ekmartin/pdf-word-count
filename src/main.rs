extern crate lopdf;

use std::collections::BTreeMap;
use std::io;

use lopdf::content::Content;
use lopdf::{Document, Object, ObjectId};

// ¯\_(ツ)_/¯
const SPACE_THRESHOLD: i64 = 100;

struct Collector {
    text: String,
}

impl Collector {
    fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    fn collect_text(&mut self, encoding: Option<&str>, operands: &[Object]) {
        for operand in operands.iter() {
            match operand {
                Object::String(ref bytes, _) => {
                    let decoded_text = Document::decode_text(encoding, bytes);
                    self.text.push_str(&decoded_text);
                }
                Object::Array(ref arr) => {
                    self.collect_text(encoding, arr);
                }
                Object::Real(f) if f.abs() > SPACE_THRESHOLD as f64 => {
                    self.text.push(' ');
                }
                Object::Integer(i) if i.abs() > SPACE_THRESHOLD => {
                    self.text.push(' ');
                }
                _op => {}
            }
        }
    }

    fn process_page(&mut self, document: &Document, page_id: ObjectId) {
        let fonts = document.get_page_fonts(page_id);
        let encodings = fonts
            .into_iter()
            .map(|(name, font)| (name, document.get_font_encoding(font)))
            .collect::<BTreeMap<String, &str>>();
        let raw_content = document.get_page_content(page_id).unwrap();
        let content = Content::decode(&raw_content).unwrap();
        let mut current_encoding = None;
        for operation in content.operations.iter() {
            match operation.operator.as_ref() {
                "Tf" => {
                    let current_font = operation.operands[0].as_name_str().unwrap();
                    current_encoding = encodings.get(current_font).cloned();
                }
                "Tj" | "TJ" => {
                    self.collect_text(current_encoding, &operation.operands);
                }
                "ET" => if !self.text.ends_with('\n') {
                    self.text.push('\n')
                },
                "Td" | "TD" | "T*" => if !self.text.ends_with(' ') {
                    // TODO: This should remove end-of-line dashes.
                    self.text.push(' ')
                },
                _op => {}
            }
        }
    }
}

fn main() {
    let document = Document::load_from(io::stdin()).unwrap();
    let mut collector = Collector::new();
    let pages = document.get_pages();
    for page_id in pages.values().into_iter() {
        collector.process_page(&document, *page_id);
    }

    println!("{}", collector.text);
}

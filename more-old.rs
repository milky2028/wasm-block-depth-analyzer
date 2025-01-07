use std::{env, fs};

use wast::{
    core::Module,
    parser::{self, Parse, ParseBuffer, Parser, Result},
};

pub struct Functions<'a> {
    functions: Vec<&'a String>,
}

impl<'a> Parse<'a> for Functions<'a> {
    fn parse(parser: Parser<'a>) -> Result<Functions<'a>> {
        let functions: Vec<&String> = Vec::new();

        Ok(Functions { functions })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Failed to provide file path argument.");

    println!("Reading file contents...");
    let file_contents = fs::read_to_string(file_path).expect("Failed to read file contents.");

    let file_contents = r#"(module
  (type $t0 (func (param i32 i32) (result i32)))
  (func $addTwo (export "addTwo") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    (i32.add
      (local.get $p0)
      (local.get $p1))))"#;

    println!("Creating buffer...");
    let buffer = ParseBuffer::new(&file_contents).expect("Failed to create parser buffer.");

    println!("Parsing WAT...");
    let module: Functions = parser::parse(&buffer).expect("Failed to parse from buffer.");
}

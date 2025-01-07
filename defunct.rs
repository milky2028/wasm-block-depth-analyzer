use anyhow::Result;
use std::{fs::File, io::Read};
use wabt::{wasm2wat, wasm2wat_with_features, Features};
use wasmparser::{Parser, Payload::*};

fn parse(mut reader: impl Read) -> Result<()> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let parser = Parser::new(0);

    let mut times_run = 0;
    for payload in parser.parse_all(&buf) {
        match payload? {
            // Sections for WebAssembly modules
            Version { .. } => { /* ... */ }
            TypeSection(_) => { /* ... */ }
            ImportSection(_) => { /* ... */ }
            FunctionSection(_) => {}
            TableSection(_) => { /* ... */ }
            MemorySection(_) => { /* ... */ }
            TagSection(_) => { /* ... */ }
            GlobalSection(_) => { /* ... */ }
            ExportSection(_) => { /* ... */ }
            StartSection { .. } => { /* ... */ }
            ElementSection(_) => { /* ... */ }
            DataCountSection { .. } => { /* ... */ }
            DataSection(_) => { /* ... */ }

            // Here we know how many functions we'll be receiving as
            // `CodeSectionEntry`, so we can prepare for that, and
            // afterwards we can parse and handle each function
            // individually.
            CodeSectionStart {
                size: _,
                count,
                range: _,
            } => println!("Number of functions: {}", count),
            CodeSectionEntry(body) => {
                let mut reader = body.get_binary_reader();
                let bytes = reader
                    .read_bytes(reader.bytes_remaining())
                    .expect("Invalid bytes.");

                times_run = times_run + 1;
                if times_run < 2 {
                    println!("{:?}", bytes);
                }
                // here we can iterate over `body` to parse the function
                // and its locals
            }

            // Sections for WebAssembly components
            ModuleSection { .. } => { /* ... */ }
            InstanceSection(_) => { /* ... */ }
            CoreTypeSection(_) => { /* ... */ }
            ComponentSection { .. } => { /* ... */ }
            ComponentInstanceSection(_) => { /* ... */ }
            ComponentAliasSection(_) => { /* ... */ }
            ComponentTypeSection(_) => { /* ... */ }
            ComponentCanonicalSection(_) => { /* ... */ }
            ComponentStartSection { .. } => { /* ... */ }
            ComponentImportSection(_) => { /* ... */ }
            ComponentExportSection(_) => { /* ... */ }

            CustomSection(_) => { /* ... */ }

            // Once we've reached the end of a parser we either resume
            // at the parent parser or the payload iterator is at its
            // end and we're done.
            End(_) => {}

            // most likely you'd return an error here, but if you want
            // you can also inspect the raw contents of unknown sections
            other => {
                match other.as_section() {
                    Some(_) => { /* ... */ }
                    None => { /* ... */ }
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).expect("Failed to provide file path argument.");
    let file = File::open(file_path).expect("Failed to locate file.");
    parse(file).expect("Failed to parse file.");
}

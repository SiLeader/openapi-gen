use crate::converter::generate;
use clap::Parser;
use log::debug;
use parser::{parse, SourceFileContent};
use std::collections::HashSet;
use std::fs::{read_to_string, File};
use std::path::Path;

mod converter;
mod openapi;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Input source file")]
    input: String,

    #[arg(short, long, help = "Include search directory")]
    include: Vec<String>,

    #[arg(short, long, default_value = "openapi.yml", help = "Output file")]
    output: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let objects = {
        let mut objects = Vec::new();

        let mut loaded_files = HashSet::new();
        let mut imported_but_unread = {
            let mut v = Vec::new();
            v.push(args.input.to_string());
            v
        };

        loop {
            let file = match imported_but_unread.pop() {
                Some(f) => f,
                None => break,
            };
            if loaded_files.contains(&file) {
                continue;
            }

            let (file, file_content) = load_and_parse(file.as_str());
            let parent = Path::new(file.as_str()).parent().unwrap();

            loaded_files.insert(file.clone());
            objects.extend_from_slice(file_content.objects.as_slice());

            imported_but_unread.extend(
                file_content
                    .imports
                    .into_iter()
                    .map(|i| parent.join(i.file).to_str().unwrap().to_string()),
            );
        }

        objects
    };

    let openapi = generate(objects);

    let file = File::create(args.output.as_str()).unwrap();
    serde_yaml::to_writer(file, &openapi).unwrap();
}

fn load_and_parse(file: &str) -> (String, SourceFileContent) {
    debug!("i. {file}");
    let file = Path::new(file)
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    debug!("k. {file}");
    let content = read_to_string(file.as_str()).unwrap();
    debug!(
        "load imported file: {file} ({} bytes)",
        content.as_bytes().len()
    );
    (file, parse(content.as_str()).unwrap())
}

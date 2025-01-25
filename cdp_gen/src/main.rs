mod parser;
use parser::*;

mod r#gen;
use r#gen::*;

use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source: PathBuf,
    out_directory: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let data: Vec<u8> = std::fs::read(args.source).unwrap();
    let resp: Main = serde_json::from_str(&String::from_utf8(data).unwrap()).unwrap();

    let out_directory = args.out_directory.clone().as_path().canonicalize().unwrap();
    assert!(out_directory.is_dir());
    assert_eq!(std::fs::read_dir(&out_directory).unwrap().count(), 0);

    let mut file = File::create_new(out_directory.join("mod.rs")).unwrap();

    let scope = r#modules(&resp.domains);
    file.write_all(scope.to_string().as_bytes()).unwrap();

    let scope = r#main_client(&resp.domains);
    file.write_all(scope.to_string().as_bytes()).unwrap();

    for domain in resp.domains {
        let mut file = File::create_new(
            out_directory.join(format!("{}.rs", field_name(domain.domain.clone()))),
        )
        .unwrap();
        let scope = r#gen(domain);
        file.write_all(scope.to_string().as_bytes()).unwrap();
    }
}

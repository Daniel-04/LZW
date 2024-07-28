use std::io::{stdout, Write, stdin, Read};
use std::fs;
use std::process::exit;

use lzw::{encode, decode};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input = vec![];
    match args.len() {
        3 => input = fs::read(&args[2]).expect("Failed to read file"),
        2 => {
            input = vec![];
            stdin().read_to_end(&mut input).expect("Failed to read stdin");
        },
        _ => incorrect_arguments(),
    }

    match &args[1][..] {
        "encode" => {
            let encoded = encode(&input);
            let bytes: Vec<u8> = encoded.iter().flat_map(|code| code.to_le_bytes()).collect();
            stdout().write_all(&bytes).ok();
        },
        "decode" => {
            let codes = input.chunks_exact(8).map(|chunk|
                                                  u64::from_le_bytes(chunk.try_into().unwrap()
                                                  )).collect::<Vec<u64>>();
            let decoded = decode(&codes);
            stdout().write_all(&decoded).ok();
        },
        _ => incorrect_arguments(),
    }
    stdout().flush().ok();
}

fn incorrect_arguments() {
    eprintln!("Usage: lzw [encode|decode] file");
    eprintln!("       lzw [encode|decode]");
    exit(1);
}

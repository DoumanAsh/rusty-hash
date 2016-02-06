///Script to calculate checksums
extern crate crypto;
extern crate memmap;

use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::sha2::{Sha256, Sha512};

use memmap::{Mmap, Protection};

use std::path;
use std::env::args as cmd_args;
use std::io::{Read, Write};

mod checksum;
use checksum::*;

const USAGE: &'static str = "Usage: rusty_hash [-c | -o | -p] [algorithms] <input>...

Algorithms:
    --md5       Enables md5 calculation.
    --sha[num]  Enables sha calculation. num can be [1, 256, 512]

Mode:
    Mutually exclusive.
    -c --check  Verifies checksum from files.
    -o --output Write calculations into files with corresponding extension.
    -p --print  Prints checksums to stdout. Default.
";

macro_rules! input {
    ($msg:expr) => {{
        use std::io::{Read, Write};
        print!($msg);
        std::io::stdout().flush().unwrap();
        std::io::stdin().bytes().next();
    }}
}

enum FlagType {
    Print,
    Output,
    Check,
}

///Parses cli arguments and returns tuple with following elements:
///
///* 1 - Vec of existings paths.
///* 2 - Vec of checksums.
///* 3 - Processing type.
fn parse_args() -> Option<(Vec<String>, Vec<Checksum>, FlagType)> {
    let mut paths: Vec<String> = Vec::new();
    let mut checksums: Vec<Checksum> = Vec::new();
    let mut flag: Option<FlagType> = None;

    for arg in cmd_args().skip(1) {
        if arg.starts_with("-") {
            match arg.as_ref() {
                "--md5" =>  checksums.push(Checksum::new("MD5".to_string(), Md5::new())),
                arg if arg.starts_with("--sha") => {
                    match &arg[5..] {
                        "1" => checksums.push(Checksum::new("SHA1".to_string(), Sha1::new())),
                        "256" => checksums.push(Checksum::new("SHA256".to_string(), Sha256::new())),
                        "512" => checksums.push(Checksum::new("SHA512".to_string(), Sha512::new())),
                        arg @ _ => {
                            println!(">>>Invalid sha option {}", arg);
                            return None;
                        }
                    }
                },
                "-o" | "--output" => {
                    if flag.is_some() {
                        println!(">>>Multiple mode options are set!");
                        return None;
                    }
                    flag = Some(FlagType::Output)
                },
                "-c" | "--check" => flag = {
                    if flag.is_some() {
                        println!(">>>Multiple mode options are set!");
                        return None;
                    }
                    Some(FlagType::Check)
                },
                "-p" | "--print" => flag = {
                    if flag.is_some() {
                        println!(">>>Multiple mode options are set!");
                        return None;
                    }
                    Some(FlagType::Print)
                },
                arg @ _ => {
                    println!(">>>Invalid option {}", arg);
                    return None;
                },
            }
        }
        else if path::Path::new(&arg).is_file() {
            paths.push(arg);
        }
        else {
            println!(">>>Not valid file: {}", &arg);
        }
    }

    if checksums.len() == 0 {
        checksums.push(Checksum::new("MD5".to_string(), Md5::new()));
        checksums.push(Checksum::new("SHA1".to_string(), Sha1::new()));
        checksums.push(Checksum::new("SHA256".to_string(), Sha256::new()));
        checksums.push(Checksum::new("SHA512".to_string(), Sha512::new()));
    }

    if flag.is_none() {
        flag = Some(FlagType::Print);
    }

    Some((paths, checksums, flag.unwrap()))
}

fn main() {
    if cmd_args().len() < 2 {
        println!("{}", USAGE);
        return;
    }

    if let Some((paths, mut checksums, flag)) = parse_args() {
        for path in paths {
            if let Ok(file) = std::fs::File::open(&path) {
                println!(">>>File: {}", &path);

                let file = Mmap::open(&file, Protection::Read).expect("Failed to create file map for reading");
                let bytes: &[u8] = unsafe { file.as_slice() };

                for algo in checksums.iter_mut() {
                    algo.reset();
                    algo.input(bytes);
                }

                for algo in checksums.iter_mut() {
                    match flag {
                        FlagType::Output => {
                            let file_name = format!("{}.{}", &path, algo.get_file_ext());
                            if let Ok(mut file) = std::fs::File::create(&file_name) {
                                file.write_fmt(format_args!("{}\n", algo.checksum())).unwrap();
                                println!("{}{}", algo.get_type_string(), &file_name);
                            }
                            else {
                                println!("{}Unable to create file with checksum!", algo.get_type_string());
                            }
                        },
                        FlagType::Check => {
                            let file_name = format!("{}.{}", &path, algo.get_file_ext());
                            if let Ok(mut file) = std::fs::File::open(&file_name) {
                                let mut expected_checksum = String::new();
                                if file.read_to_string(&mut expected_checksum).is_ok() {
                                    if expected_checksum.trim() == algo.checksum() {
                                        println!("{}OK", algo.get_type_string());
                                    }
                                    else {
                                        println!("{}NOT_OK", algo.get_type_string());
                                    }
                                }
                                else {
                                    println!("{}Failed to get checksum from file!", algo.get_type_string());
                                }
                            }
                            else {
                                println!("{}No checksum file!", algo.get_type_string());
                            }
                        },
                        FlagType::Print => {
                            println!("{}", algo.result());
                        },
                    }
                }
            }
            else {
                println!(">>>{}: failed to open", &path);
            }
            println!("=======================================================\n");
        }
    }
    else {
        println!("{}", USAGE);
        return;
    }

    //this is done mostly for convenient drag and drop.
    if cfg!(windows) {
        input!("Press Enter to exit...");
    }
}

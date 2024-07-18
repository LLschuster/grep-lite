use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use regex::Regex;
use clap::{Arg, Command};

const CTX_SIZE: u32 = 1;

fn cli() -> Command {
    Command::new("grep-lite")
    .arg(
        Arg::new("pattern")
        .short('p')
        .required(true)
        .help("Pattern to search for")
        .value_parser(clap::value_parser!(String)))
    .arg(
        Arg::new("file")
        .short('f')
        .required(true)
        .help("File to search")
        .value_parser(clap::value_parser!(PathBuf)))
}

fn main() {
    println!("Start of Grep-lite");

    let cli_command = cli();

    let arg_matches = cli_command.get_matches();
    let pattern = Regex::new(arg_matches.get_one::<String>("pattern").unwrap()).unwrap();
    let file_path: &PathBuf = arg_matches.get_one::<PathBuf>("file").unwrap();
    
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let reader_content: Vec<String> = reader.lines().map(|line_| line_.unwrap()).collect();

    let mut match_indexes: Vec<usize> = vec![];

    for (index, line) in reader_content.iter().enumerate(){
        match pattern.find(&line) {
            Some(_) => {
                match_indexes.push(index);
            },
            None => (),
        };
    }

    for (index, line) in reader_content.iter().enumerate(){
        for match_index in &match_indexes {
            let lower_bound = match_index.saturating_sub(CTX_SIZE as usize);
            let upper_bound = match_index.saturating_add(CTX_SIZE as usize);

            match index {
                i if i == *match_index => {
                    println!("Found match here >>>> {}. {}", index + 1, line);
                },
                i if i >= lower_bound && i <= upper_bound => {
                    println!("{}. {}", index + 1, line);
                },
                _ => (),
            }
        }
    }

}

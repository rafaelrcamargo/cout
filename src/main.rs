//! This tool is used to colorize the output of the stdin stream.
//! Tokens are colored according to their type, and then streamed back to stdout.

use clap::{arg, command}; // Argument parsing.
use colored::Colorize; // Coloring terminal.
use regex::Regex;

use std::io; // Std lib.

mod profiles;
use profiles::{get_profile, Profile};

/// Main - Reads from stdin and writes to stdout
fn main() {
    let matches = command!()
        .arg(
            arg!(<PROFILE>)
                .required(false)
                .help("Pattern highlighting profile to use."),
        )
        .get_matches();

    let profile: Profile = match matches.get_one::<String>("PROFILE") {
        Some(profile) => get_profile(profile),
        None => get_profile("default"),
    };

    let lines = io::stdin().lines();
    for line in lines {
        let mut line = line.expect("Failed to read line.");
        for pattern in &profile.patterns {
            let re = Regex::new(&pattern[0]).expect("Failed to parse regex.");
            line = re
                .replace_all(&line, |caps: &regex::Captures| {
                    caps[0]
                        .color(pattern.get(1).expect("Failed to get color.").to_string())
                        .to_string()
                })
                .to_string();
        }
        println!("{}", line);
    }
}

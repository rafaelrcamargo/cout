//! This tool is used to colorize the output of the stdin stream.
//! Tokens are colored according to their type, and then streamed back to stdout.

use colored::{Color, Colorize}; // Coloring terminal.
use regex::Regex; // Regex matching.

use std::io; // Std lib.

/// Main - Reads from stdin and writes to stdout
fn main() {
    let lines = io::stdin().lines();
    for line in lines {
        let mut line = line.unwrap();
        let tokens = [
            (
                // URLS.
                r"(https?://|www.)[^\s][^\)|\s]+",
                Color::BrightBlue,
            ),
            (
                // -
                r"-+",
                Color::Black,
            ),
            (
                // Dates
                r"[0-9]?[0-9]/([1][0-2])?[1-9]?/[0-9]?[0-9][0-9][0-9]",
                Color::Yellow,
            ),
            (
                // MS for timing. :)
                r"[^0-9a-zA-Z=:]?ms\.?",
                Color::Cyan,
            ),
            (
                // All error messages and variants.
                r"\b(?:error|Error|ERROR|Err|ERR|err)\b",
                Color::Red,
            ),
            (
                // All warn messages and variants.
                r"\b(?:warn|Warn|WARN|Warning|WARNING|warning)\b",
                Color::Yellow,
            ),
            (
                // All info messages and variants.
                r"\b(?:info|Info|INFO|Information|INFORMATION|information)\b",
                Color::Green,
            ),
            (
                // All debug messages and variants.
                r"\b(?:debug|Debug|DEBUG|Debugging|DEBUGGING|debugging)\b",
                Color::Blue,
            ),
            (
                // All full CAPS words.
                r"\b[A-Z_][A-Z_]*\b",
                Color::Yellow,
            ),
            (
                // Color everything inside of quotes.
                r#"["|'][^"|']*["|']"#,
                Color::Yellow,
            ),
            /* (
                // All numbers.
                r"[0-9][0-9a-f]*:?:*:?",
                Color::Magenta,
            ), */
            (
                // Color everything inside of brackets.
                r"\[[^\]]*\]",
                Color::BrightCyan,
            ),
            (
                // Color everything inside of braces.
                r"\{[^\}]*\}",
                Color::BrightMagenta,
            ),
            (
                // Color everything inside of braces.
                r".{2,}:[\n| ]",
                Color::Green,
            ),
        ];
        for token in tokens.iter() {
            let re = Regex::new(token.0).unwrap();
            line = re
                .replace_all(&line, |caps: &regex::Captures| {
                    caps[0].color(token.1).to_string()
                })
                .to_string();
        }
        println!("{}", line);
    }
}

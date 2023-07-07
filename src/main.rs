#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let start_time = Instant::now();

    let args = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("could not open file `{}`", args.path.display()));
    let reader = BufReader::new(file.context("could not read file")?);

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            }
            Err(e) => {
                eprintln!("could not read line: {}", e);
            }
        }
    }

    let elapsed = start_time.elapsed();
    println!("\nElapsed: {:?}", elapsed);
    Ok(())
}

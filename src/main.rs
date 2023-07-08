#![allow(unused)]

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write as BufWrite};
use std::thread;
use std::time::{Duration, Instant};
use std::{cmp::min, fmt::Write};

use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use log::{info, trace, warn};

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

    let mut processed = 0;
    let file_size = std::fs::metadata(&args.path)?.len();
    let pb = get_progress_bar(file_size);

    trace!("opening the file - size: {}", file_size);
    let file = File::open(&args.path)
        .with_context(|| format!("could not open file `{}`", args.path.display()));
    let reader = BufReader::new(file.context("could not read file")?);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    trace!("reading the file");
    for line_result in reader.lines() {
        let new = min(processed + (file_size / 100), file_size);
        pb.set_position(new);

        match line_result {
            Ok(line) => {
                trace!("read line: {}", line);
                if line.contains(&args.pattern) {
                    info!("found matching line: {}", line);
                    writeln!(handle, "{}", line);
                }
            }
            Err(e) => {
                warn!("could not read line: {}", e);
                eprintln!("could not read line: {}", e);
            }
        }
    }

    pb.finish_with_message("Done!\n");
    let elapsed = start_time.elapsed();
    println!("  - Finished in {:?}\n", elapsed);
    Ok(())
}

fn get_progress_bar(file_size: u64) -> ProgressBar {
    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
    pb
}

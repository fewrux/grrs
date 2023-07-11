#![allow(unused)]

#[macro_use]
extern crate log;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write as BufWrite};
use std::thread;
use std::time::{Duration, Instant};
use std::{cmp::min, fmt::Write};

use anyhow::{Context, Result};
use clap::Parser;
use env_logger::Builder;
use log::{info, trace, warn};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let start_time = Instant::now();

    trace!("parsing command line arguments");
    let args = Cli::parse();

    trace!("configuring logger");
    let mut builder = config_logger(&args);

    let file_size = std::fs::metadata(&args.path)?.len();

    trace!("opening file - size: {}", file_size);
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    trace!("creating buffer writer");
    let mut writer = io::BufWriter::new(io::stdout().lock());

    trace!("reading file");
    find_matches(&content, &args.pattern, writer);

    let elapsed = start_time.elapsed();
    info!("finished in {:?}", elapsed);
    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        trace!("read line: {}", line);
        if line.contains(pattern) {
            info!("found matching line: {}", line);
            writeln!(writer, "{}", line);
        }
    }
}

// fn config_progress_bar(file_size: u64) -> ProgressBar {
//     let pb = ProgressBar::new(file_size);
//     pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
//         .unwrap()
//         .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
//         .progress_chars("#>-"));
//     pb
// }

fn config_logger(args: &Cli) -> Builder {
    let mut builder = Builder::from_default_env();
    match args.verbosity.log_level() {
        Some(log::Level::Error) => builder.filter(None, log::LevelFilter::Error),
        Some(log::Level::Warn) => builder.filter(None, log::LevelFilter::Warn),
        Some(log::Level::Info) => builder.filter(None, log::LevelFilter::Info),
        Some(log::Level::Debug) => builder.filter(None, log::LevelFilter::Debug),
        Some(log::Level::Trace) => builder.filter(None, log::LevelFilter::Trace),
        _ => builder.filter(None, log::LevelFilter::Off),
    };
    builder.init();
    builder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_a_match() {
        let mut result = Vec::new();
        find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }
}

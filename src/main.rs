#![allow(unused)]

#[macro_use]
extern crate log;

// use anyhow::{Context, Result};
// use clap::Parser;
// use env_logger::Builder;
// use log::{info, trace, warn};
use grrs::{Cli, Context, Parser};
use std::fs::File;
use std::time::{Duration, Instant};

fn main() -> grrs::Result<()> {
    let start_time = Instant::now();

    trace!("parsing command line arguments");
    let args = Cli::parse();

    trace!("configuring logger");
    let mut builder = grrs::config_logger(&args);

    let file_size = std::fs::metadata(&args.path)?.len();

    trace!("opening file - size: {}", file_size);
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    trace!("reading file");
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    let elapsed = start_time.elapsed();
    info!("finished in {:?}", elapsed);
    println!("finished in {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_a_match() {
        let mut result = Vec::new();
        grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }
}

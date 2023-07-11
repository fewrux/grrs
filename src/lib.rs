pub use anyhow::{Context, Result};
pub use clap::Parser;
pub use env_logger::Builder;
pub use log::{info, trace, warn};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,
    /// The path to the file to read
    pub path: std::path::PathBuf,
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        trace!("read line: {}", line);
        if line.contains(pattern) {
            info!("found matching line: {}", line);
            writeln!(writer, "{}", line)
                .with_context(|| format!("could not write line `{}`", line))?;
        }
    }
    Ok(())
}

pub fn config_logger(args: &Cli) -> Builder {
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

# grrs - Rust

> <strong>grrs</strong> is a grep-like CLI tool implemented in Rust. It allows you to search
> for a pattern in a file and display the lines that contain it.

## Table of Contents

-   [Installation](#installation)
-   [Usage](#usage)
-   [Examples](#examples)
-   [Features](#features)
-   [Dependencies](#dependencies)
-   [Error Handling](#error-handling)
-   [Performance Considerations](#performance-considerations)
-   [Limitations](#limitations)
-   [How It Works](#how-it-works)

## Installation

To use this tool, you need to have Rust installed on your system. If you don't have Rust installed, you can install it by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can build the tool using the following command:
`$ cargo build --release`

This will create an executable file in the target/release/ directory.

## Usage

To use <strong>grrs</strong> CLI tool, run the following command:
`$ ./target/release/grrs <pattern> <path_to_file>`

Replace <pattern> with the pattern you want to search for, and <path_to_file> with the path to the file you want to search in.

The tool will read the file line by line, searching for the pattern in each line. It will display the lines that contain the pattern.

## Examples

Here are a few examples of how to use the grep-like CLI tool:

`$ ./target/release/grrs "error" /path/to/file.log`

`$ ./target/release/grrs "TODO" /path/to/project -vv`

## Features

-   Search for a pattern in a file and display the lines that contain it.
-   Progress bar: The tool displays a progress bar that shows the progress of reading the file.
-   Logging: The tool uses the `env_logger` crate to configure and display log messages at different verbosity levels.

## Dependencies

The tool has the following dependencies:

-   `clap` (version 4.2.7) - A powerful and flexible command-line argument parsing library for Rust. It is used to parse command-line arguments.
-   `anyhow` (version 1.0.71) - A library for representing and handling errors in a convenient way. It is used to handle errors and provide meaningful error messages.
-   `indicatif` (version 0.17.3) - A library for creating interactive command-line progress bars. It is used to display the progress of reading the file.
-   `log` (version 0.4.17) - A lightweight logging facade for Rust. It is used for logging messages at different verbosity levels.
-   `env_logger` (version 0.10.0) - A logger implementation for the log crate that is based on environment variables. It is used to configure and display log messages.
-   `clap-verbosity-flag` (version 2.0.1) - A library for parsing verbosity level command-line arguments. It is used to set the verbosity level of the logger.

These dependencies are automatically managed by Cargo, the Rust package manager.

To add these dependencies to your project, include the following lines in your `Cargo.toml` file:

```
[dependencies]
clap = { version = "4.2.7", features = ["derive"] }
anyhow = "1.0.71"
indicatif = "0.17.3"
log = "0.4.17"
env_logger = "0.10.0"
clap-verbosity-flag = "2.0.1"
```

You can then build and compile the project using the `cargo build` command.

These dependencies provide essential functionality and enable features such as command-line argument parsing, error handling, progress bar display, logging, and verbosity control.

Feel free to modify and customize the dependencies as per your project's requirements.

## Error Handling

The tool handles errors gracefully. If there is an error reading a line or opening a file, a warning message is logged, and an error message is printed to the console. The tool continues processing the remaining lines in the file.

## Performance Considerations

The tool is designed to handle large files efficiently. It reads the file line by line, minimizing memory usage. Additionally, it uses a progress bar to provide visual feedback on the progress of reading the file.

## Limitations

-   The tool currently only supports searching for patterns within text files. It may not work as expected for binary files.
-   The search functionality is case-sensitive. It does not support case-insensitive searching.
-   The tool may experience performance degradation with extremely large files or in low-memory environments.

## How It Works

1. The tool reads the command-line arguments to get the pattern and file path.
2. It configures the logger based on the verbosity level specified in the command-line arguments.
3. The tool opens the file and creates a buffer reader to read the file line by line.
4. It configures and displays a progress bar based on the size of the file.
5. The tool iterates over each line in the file, checking if the line contains the specified pattern.
6. If a line contains the pattern, it is displayed, and the line is written to the output file.
7. If there is an error reading a line, a warning message is logged, and an error message is printed.
8. The tool sleeps for a short duration after processing each line to allow the progress bar to update.
9. Once all lines in the file have been processed, the elapsed time is logged, and the progress bar is finished.
10. The tool prints a message indicating the completion and the total elapsed time.

That's it! You can now use the Rust grep-like CLI tool to search for patterns in files efficiently.

<!--
## Features
<ul>
  <li>Find the first occurrence of a given pattern in a file</li>
  <li>[] Search all occurrences of the given pattern</li>
  <li>[] Count the total number of occurrences </li>
</ul>

<br>

## How to use

This is the main command base structure:
`cargo run <pattern_to_search> <path_to_file>`

<br>

<ol>
  <li>Replace the `<pattern_to_search>` by the desired word, e.g. "main"</li>
  <li>Replace the `<path_to_file>` by the path to the desired file, e.g. "src/main.rs"</li>
</ol>

<br>

The above command should return:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/grrs main src/main.rs`
  [00:00:00] [##################################################################################] 3.23 KiB/3.23 KiB (0.0s)
  - Finished in 542.477144ms

fn main() -> Result<()> {
```
-->

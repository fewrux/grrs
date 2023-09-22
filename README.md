# grrs

<strong>grrs</strong> is a grep-like CLI tool implemented in <strong>Rust</strong>. It allows you to search for a pattern in a file and display the lines that contain it.

<br>

## Table of Contents

-   [Installation](#installation)
-   [Usage](#usage)
-   [Options](#options)
-   [Examples](#examples)
-   [Features](#features)
-   [Dependencies](#dependencies)
-   [Error Handling](#error-handling)
-   [Performance Considerations](#performance-considerations)
-   [Limitations](#limitations)
-   [How It Works](#how-it-works)

<br>

## Installation

To use this tool, you need to have Rust installed on your system. If you don't have Rust installed, you can install it by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can build the tool using the following command:
`$ cargo build --release`

This will create an executable file in the target/release/ directory.

<br>

## Usage

To use <strong>grrs</strong> CLI tool, run the following command:

```
$ ./target/release/grrs <pattern> <path_to_file> [OPTIONS]
```

Replace <pattern> with the pattern you want to search for, and <path_to_file> with the path to the file you want to search in.

<strong>grrs</strong> will read the file line by line, searching for the pattern in each line. It will display the lines that contain the pattern.

<br>

## Options

<strong>grrs</strong> supports the following options:

<ul>
  <li><strong>-q</strong>: Silence the output. Only errors will be displayed.</li>
  <li>
    <strong>-v</strong>: Increase the verbosity level. Specify this flag multiple times to increase the verbosity level further.<br>The available verbosity levels are as follows:
    <ul>
      <li><strong>-v</strong>: Show warnings.</li>
      <li><strong>-vv</strong>: Show info.</li>
      <li><strong>-vvv</strong>: Show debug.</li>
      <li><strong>-vvvv</strong>: Show trace.</li>
    </ul>
  </li>
</ul>

<br>

## Examples

-   Search for the pattern error in the file `/path/to/file.log` and display only the lines containing the pattern (silencing the output):

```
$ ./target/release/grrs "error" /path/to/file.log -q
```

<br>

-   Search for the pattern "TODO" in the directory `/path/to/project`, showing warnings and info messages:

```
$ ./target/release/grrs TODO /path/to/project -vv
```

<br>

Feel free to adjust the verbosity flag commands or provide additional examples to match your tool's functionality and use cases.

<br>

## Features

-   Search for a pattern in a file and display the lines that contain it.
-   Progress bar: The tool displays a progress bar that shows the progress of reading the file.
-   Logging: The tool uses the `env_logger` crate to configure and display log messages at different verbosity levels.

<br>

## Dependencies

<strong>grrs</strong> has the following dependencies:

-   [clap](https://crates.io/crates/clap) - A powerful and flexible command-line argument parsing library for Rust. It is used to parse command-line arguments.
-   [anyhow](https://crates.io/crates/anyhow) - A library for representing and handling errors in a convenient way. It is used to handle errors and provide meaningful error messages.
-   [indicatif](https://crates.io/crates/indicatif) - A library for creating interactive command-line progress bars. It is used to display the progress of reading the file.
-   [log](https://crates.io/crates/log) - A lightweight logging facade for Rust. It is used for logging messages at different verbosity levels.
-   [env_logger](https://crates.io/crates/env_logger) - A logger implementation for the log crate that is based on environment variables. It is used to configure and display log messages.
-   [clap-verbosity-flag](https://crates.io/crates/clap-verbosity-flag) - A library for parsing verbosity level command-line arguments. It is used to set the verbosity level of the logger.

<br>

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

<br>

## Error Handling

<strong>grrs</strong> handles errors gracefully. If there is an error reading a line or opening a file, a warning message is logged, and an error message is printed to the console. The tool continues processing the remaining lines in the file.

<br>

## Performance Considerations

<strong>grrs</strong> is designed to handle large files efficiently. It reads the file line by line, minimizing memory usage. Additionally, it uses a progress bar to provide visual feedback on the progress of reading the file.

<br>

## Limitations

-   <strong>grrs</strong> currently only supports searching for patterns within text files. It may not work as expected for binary files.
-   The search functionality is case-sensitive. It does not support case-insensitive searching.
-   The tool may experience performance degradation with extremely large files or in low-memory environments.

<br>

## How It Works

1. <strong>grrs</strong></a> reads the command-line arguments to get the pattern and file path.
2. Configures the logger based on the verbosity level specified in the command-line arguments.
3. Opens the file and creates a buffer reader to read the file line by line.
4. Configures and displays a progress bar based on the size of the file.
5. Iterates over each line in the file, checking if the line contains the specified pattern.
6. If a line contains the pattern, it is displayed, and the line is written to the output file.
7. If there is an error reading a line, a warning message is logged, and an error message is printed.
8. Once all lines in the file have been processed, the elapsed time is logged, and the progress bar is finished.
9. Prints a message indicating the completion and the total elapsed time.

That's it! You can now use the Rust <strong>grrs</strong> CLI tool to search for patterns in files efficiently.

# File Alert

File Alert is a lightweight Rust utility that can detect changes inside a file and play a sound when
a string from the pre-defined list.

## Usage
Using File Alert is straight forward, once you have the binary, you can run it, and it will prompt you for a file to watch,
sound file to play and a list of strings to watch for. All these are saved in `%LOCALAPPDATA%/FileAlert/config.toml` and on
subsequent executions you can just use the last values.

## Installation
No binary releases are provided for now, if you wish to install this utility you will need to compile the source.


# sphinx-passwords
A command line password generator implemented in Rust

## How to use (for the moment using Rust, binaries are work in progress)

`cargo build --release && ./target/release/sphinx-passwords`

### Usage Options

- Use `-l` or `--length` with a number to set the length of password for example: `-l 34` returns 34 characters. This option defaults to a random length between 10 and 20 if is not passed.
- Use `-s` to mute all messages, only returns the password. This is useful to use the binary in a script or an automation tool

## Tests

Tests are in tests folder, to run tests use `cargo test`

## To do

- [x] Use colors in terminal
- [x] Copy the generated password to clipboard
- [x] Add silently option
- [ ] Binaries for Linux, OSX and Windows
- [x] Tests in Rust!

# cmd-exists
[![Crate](https://img.shields.io/crates/v/cmd-exists.svg)](https://crates.io/crates/cmd-exists)

Incredibly tiny library with the sole purpose of determining whether a command/program exists in the user's shell.

## Usage
```rust
use cmd_exists::*;

fn main() {
    assert!(cmd_exists("neofetch").is_ok());
}
```
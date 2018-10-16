# Rust GBG - CLI workshop

## Getting started

1. Clone this repository.
1. Install Rust: Go to rustup.rs and follow instructions.
1. Install and make the latest nightly toolchain the default. Because we will use the
   soon-to-be-released Rust 2018 edition!
   ```bash
   rustup default nightly
   ```
1. Install the code formatter. This can later be invoked with `cargo fmt`.
   ```bash
   rustup component add rustfmt-preview
   ```
1. Try building this project
   ```bash
   cargo build
   ```
1. Try running this project
   ```bash
   cargo run -- --help
   cargo run
   cargo run -- --delay 1000 /tmp
   ```

## Exercises

Do these if you want to. Otherwise do whatever you want to the program. Or create a completely new
CLI tool.

1. Extend the program to allow filtering out which type of events it prints.
1. The `println!` macro has to lock stdout for each call to the macro. This is not good for
   performance. Try locking stdout in the beginning of the program and make it use that.
1. Make the program take an `-o <path>` argument and write the same there as to stdout.

If you feel a filesystem monitor is the most stupid thing ever, then write some other CLI tool.
Here are some recommendations:

1. Something `curl`/`wget`-esque. I would recommend using the `reqwest` library for http.
1. A `grep` alternative. Could be fun to get the file read buffering right. As well as borrowing
   strings all over the place. Support regex with the `regex` crate?

## The Rust CLI working group

The Rust community has a number of "working groups". Each focus on a different area and tries to
improve Rust in that area. The CLI working group can be found over at
https://github.com/rust-lang-nursery/cli-wg

More specifically, *their* tutorial on getting started writing a CLI app is located at 
https://github.com/rust-lang-nursery/cli-wg/blob/master/src/tutorial/README.md

## More inspiration

There are quite a lot of useful CLI tools written in Rust. Here I will mention a few that tries to
be replacements for very common tools. It's likely that you will already know approximately
what they do.

* `ripgrep` - Insanely fast `grep` replacement. You will never want to use `grep` after trying this.
* `fd` - Fast and user-friendly alternative to `find`.
* `fselect` - Another `find` alternative. This one has an SQL like syntax for querying.
* `exa` - A modern replacement for `ls`.
* `bat` - A `cat` clone, but with syntax highlighting, line numbers and Git integration.

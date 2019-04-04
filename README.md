# Rust GBG - CLI workshop

## Getting started

1. Clone this repository (github.com/rustgbg/rustgbg-cli-quickstart).
1. Install Rust: Go to rustup.rs and follow instructions. Get the latest stable toolchain (1.33).
   This crate uses Rust 2018 edition, which is available from Rust 1.31 and up.
1. Install the code formatter. So if you ever find yourself ever wondering "How would a rustacean
   format this code", you can just run `cargo fmt` and know the answer instantly.
   ```bash
   # Install formatter:
   rustup component add rustfmt

   # Format all code in current crate:
   cargo fmt
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

Do these if you want to, in any order. Otherwise do whatever you want to the program. Or create a
completely different Rust program.

### 1. Filtering feature
Extend the program to allow filtering out which type of events it prints.

You want to watch a large directory to monitor which files are deleted in real time. But some
process is constantly writing data to a logfile deep down in this tree. Make it possible to ignore
these write events.

### 2. The optimization freak
Only one thread can write to `stdout` at a time to not cause messy output. So the `println!`
macro in the Rust standard library has a mutex lock that it locks around each write operation.
This is not good if you want to squeeze out every last drop of performance in your program.

There are ways to lock stdout once and then write to that locked version. Try to make the program
do that. Hint: `std::io::stdout()`.

### 3. Persisting the event log
Make the program take an `-o <path>` argument and write the same thing there as it already does to
`stdout`.

### 4. The callback
Maybe you use this tool to recompile some program every time someone writes to the source code.
Then it would be handy if you could trigger your build process every time there is change.

Implement way to give a command to this tool and have it be spawned an subprocess for every event.
Or for a subset of events. Hint: `std::process::Command`.


## Other Rusty things to do

If you feel that a filesystem monitor is the most stupid thing ever, then write some other type
of program. Here are some recommendations:

1. Something `curl`/`wget`-esque. I would recommend using the [`reqwest`] library for HTTP.
1. A `grep` alternative. Could be fun to get the file read buffering right. As well as borrowing
   strings all over the place. Support regex with the `regex` crate?
   Dare to compare your performance with [`ripgrep`]?

[`reqwest`]: https://crates.io/crates/reqwest
[`ripgrep`]: https://crates.io/crates/ripgrep


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

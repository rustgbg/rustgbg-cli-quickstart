use std::{env, path::PathBuf, time::Duration};

/// Rust struct representing all the options the user can specify on the command line.
pub struct Settings {
    pub watch_path: PathBuf,
    pub delay: Duration,
}

/// Uses the `clap` crate to generate help/usage printing as well as parse the given arguments.
pub fn parse_arguments() -> Settings {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("PATH")
                .help("The path to watch. Uses current working directory if not specified")
                .index(1),
        )
        .arg(
            clap::Arg::with_name("delay")
                .help("Set the event delay in ms. Helps group chained events into single events")
                .short("d")
                .long("delay")
                .default_value("100"),
        )
        .get_matches();

    // Pull out the PATH argument. Fall back to the current working directory if it was not given.
    let watch_path = match matches.value_of_os("PATH") {
        Some(path) => PathBuf::from(path),
        None => env::current_dir().unwrap(),
    };

    // Get the delay value and try to parse it into an u64. If this fails clap will print an error
    // and make the program exit.
    let delay_ms = clap::value_t!(matches.value_of("delay"), u64).unwrap_or_else(|e| e.exit());
    let delay = Duration::from_millis(delay_ms);

    Settings { watch_path, delay }
}

use notify::Watcher;
use std::{env, path::PathBuf, sync::mpsc, time::Duration};

fn main() {
    let settings = parse_arguments();

    // Set up the filesystem watcher. Events will be send back over the `event_rx` channel receiver.
    let (event_tx, event_rx) = mpsc::channel();
    let mut watcher = notify::watcher(event_tx, settings.delay).unwrap();
    watcher
        .watch(settings.watch_path, notify::RecursiveMode::Recursive)
        .unwrap();

    for event in event_rx {
        use notify::DebouncedEvent::*;
        match event {
            NoticeWrite(_path) => (),
            NoticeRemove(_path) => (),
            Create(path) => println!("create {}", path.display()),
            Write(path) => println!("write {}", path.display()),
            Chmod(path) => println!("chmod {}", path.display()),
            Remove(path) => println!("remove {}", path.display()),
            Rename(from, to) => println!("rename {} => {}", from.display(), to.display()),
            Rescan => (),
            Error(error, None) => eprintln!("error: {}", error),
            Error(error, Some(path)) => eprintln!("error at {}: {}", path.display(), error),
        }
    }
    // Since we want to listen to events until the user exits with ctrl-c, it's always an error to
    // reach the end of the program.
    eprintln!("Notify has crashed");
    ::std::process::exit(1);
}

struct Settings {
    watch_path: PathBuf,
    delay: Duration,
}

/// Uses the `clap` crate to generate help/usage printing as well as parse the given arguments.
fn parse_arguments() -> Settings {
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

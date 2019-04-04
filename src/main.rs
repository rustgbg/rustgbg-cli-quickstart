use notify::Watcher;
use std::sync::mpsc;

/// A module containing all the code that parses the command line arguments and turns them into
/// a Rust struct with correctly typed values.
mod cli;

fn main() {
    let settings = cli::parse_arguments();

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

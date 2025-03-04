use std::{
    path::Path,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

fn handle_change(debounce_time: Duration, last_change: &mut Instant) {
    let now = Instant::now();
    if now.duration_since(*last_change) > debounce_time {
        println!("Change detected");

        *last_change = now;
    }
}

fn main() {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    let debounce_time = Duration::from_millis(250);
    let mut last_change = Instant::now();

    for res in rx {
        match res {
            Ok(_) => handle_change(debounce_time, &mut last_change),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event};
use std::{process::Command, sync::mpsc::channel, path::Path};

fn main() -> anyhow::Result<()> {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
    watcher.watch(Path::new("/workspace"), RecursiveMode::NonRecursive)?;

    for event in rx {
        if let Ok(ev) = event {
            handle_event(ev);
        }
    }
    Ok(())
}

fn handle_event(event: Event) {
    if event.paths.iter().any(|p| p.ends_with("results.json")) {
        let _ = Command::new("/usr/local/bin/grade_push")
            .arg(&event.paths[0])
            .status();
    }
}

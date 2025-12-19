use crate::core::fs_watcher;
use crate::storage::sqlite_manager::SqliteStore;
use log::info;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub struct SentinelAgent {
    watch_path: String,
    watcher: RecommendedWatcher,
    store: SqliteStore,
}

impl SentinelAgent {
    pub fn new(path: String) -> Self {
        info!("SentinelAgent initialized");
        let store = SqliteStore::new("sentinel.db");
        let (tx, rx) = channel();
        
        let mut watcher = RecommendedWatcher::new(tx, Config::default())
            .expect("Failed to initialize FS watcher");
        
        watcher
            .watch(Path::new(&path), RecursiveMode::Recursive)
            .expect("Failed to watch path");
        
        // Start the event processing thread
        thread::spawn(move || {
            fs_watcher::process_events(rx);
        });
        
        Self { 
            watch_path: path, 
            watcher,
            store,
        }
    }

    pub fn run(&self) {
        info!("SentinelAgent running");
        
        loop {
            thread::sleep(Duration::from_secs(5));
            info!("Agent heartbeat");
        }
    }
}

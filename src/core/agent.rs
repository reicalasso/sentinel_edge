use crate::core::fs_watcher;
use crate::storage::sqlite_manager::SqliteStore;
use log::info;
use std::thread;

pub struct SentinelAgent {
    watch_path: String,
}

impl SentinelAgent {
    pub fn new(path: String) -> Self {
        info!("SentinelAgent initialized");
        Self { watch_path: path }
    }

    pub fn run(&self) {
        info!("SentinelAgent running");

        let path = self.watch_path.clone();
        let store = SqliteStore::new("sentinel.db");

        thread::spawn(move || {
            fs_watcher::start_watching(&path, store);
        });

        loop {
            thread::sleep(std::time::Duration::from_secs(5));
            info!("Agent heartbeat");
        }
    }
}

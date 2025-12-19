use crate::core::fs_watcher;
use log::info;
use std::thread;
use std::time::Duration;

pub struct SentinelAgent {
    running: bool,
}

impl SentinelAgent {
    pub fn new() -> Self {
        info!("SentinelAgent initialized");
        Self { running: true }
    }

    pub fn run(&mut self) {
        info!("SentinelAgent running");
        
        // Start FS watcher in separate thread
        thread::spawn(|| {
            fs_watcher::start_watching("./watch_test");
        });
        
        // Main daemon loop - keeps the program alive
        while self.running {
            thread::sleep(Duration::from_secs(5));
            info!("Agent heartbeat");
        }
    }
}

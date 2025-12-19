mod core;
mod storage;
mod util;

use core::agent::SentinelAgent;

fn main() {
    util::log::init();

    // İstediğin bir klasör (test için)
    let watch_path = "./watch_test".to_string();

    std::fs::create_dir_all(&watch_path).ok();

    let agent = SentinelAgent::new(watch_path);
    agent.run();
}

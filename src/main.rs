mod core;
mod storage;
mod util;

use core::agent::SentinelAgent;

fn main() {
    util::log::init();

    let mut agent = SentinelAgent::new();
    agent.run();
}

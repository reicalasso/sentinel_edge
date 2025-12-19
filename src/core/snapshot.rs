use std::path::PathBuf;

#[derive(Debug)]
pub struct FileSnapshot {
    pub path: PathBuf,
    pub size: u64,
    pub sha256: String,
    pub timestamp: i64,
}

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read};
use chrono::Utc;

pub fn take_snapshot(path: &PathBuf) -> Option<FileSnapshot> {
    let mut file = File::open(path).ok()?;
    let metadata = file.metadata().ok()?;

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer).ok()?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let hash = hex::encode(hasher.finalize());

    Some(FileSnapshot {
        path: path.clone(),
        size: metadata.len(),
        sha256: hash,
        timestamp: Utc::now().timestamp(),
    })
}

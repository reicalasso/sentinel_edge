use crate::core::event::{FsEvent, FsEventType};
use crate::core::file_identity::resolve_file_id;
use crate::core::snapshot::take_snapshot;
use crate::storage::sqlite_manager::SqliteStore;
use chrono::Utc;
use log::{info, warn};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use uuid::Uuid;

pub fn start_watching(path: &str, store: SqliteStore) {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .expect("Failed to initialize FS watcher");

    watcher
        .watch(Path::new(path), RecursiveMode::Recursive)
        .expect("Failed to watch path");

    info!("Watching directory: {}", path);

    for res in rx {
        match res {
            Ok(event) => handle_event(event, &store),
            Err(e) => warn!("Watch error: {:?}", e),
        }
    }
}

fn persist(event_type: FsEventType, path: &std::path::Path, store: &SqliteStore) {
    let id = Uuid::new_v4().to_string();
    let ts = Utc::now().timestamp();

    store.insert_event(
        &id,
        path.to_string_lossy().as_ref(),
        format!("{:?}", event_type).as_str(),
        ts,
    );
}

fn handle_event(event: Event, store: &SqliteStore) {
    match event.kind {
        EventKind::Create(_) => {
            for p in event.paths {
                let file_id = resolve_file_id(store, p.to_string_lossy().as_ref());
                persist(FsEventType::Create, &p, store);
                if let Some(snapshot) = take_snapshot(&p) {
                    if !store.has_duplicate_snapshot(&file_id, &snapshot.sha256) {
                        let snap_id = Uuid::new_v4().to_string();
                        store.insert_snapshot(
                            &snap_id,
                            &file_id,
                            snapshot.path.to_string_lossy().as_ref(),
                            snapshot.size,
                            &snapshot.sha256,
                            snapshot.timestamp,
                        );
                    } else {
                        info!("[SKIP] Content identical, snapshot ignored for {:?}", p);
                    }
                }
            }
        }
        EventKind::Modify(kind) => {
            match kind {
                notify::event::ModifyKind::Name(_) => {
                    for p in event.paths {
                        persist(FsEventType::Rename, &p, store);
                    }
                }
                _ => {
                    for p in event.paths {
                        let file_id = resolve_file_id(store, p.to_string_lossy().as_ref());
                        persist(FsEventType::Modify, &p, store);
                        if let Some(snapshot) = take_snapshot(&p) {
                            if !store.has_duplicate_snapshot(&file_id, &snapshot.sha256) {
                                let snap_id = Uuid::new_v4().to_string();
                                store.insert_snapshot(
                                    &snap_id,
                                    &file_id,
                                    snapshot.path.to_string_lossy().as_ref(),
                                    snapshot.size,
                                    &snapshot.sha256,
                                    snapshot.timestamp,
                                );
                            } else {
                                info!("[SKIP] Content identical, snapshot ignored for {:?}", p);
                            }
                        }
                    }
                }
            }
        }
        EventKind::Remove(_) => {
            for p in event.paths {
                persist(FsEventType::Delete, &p, store);
            }
        }
        _ => {}
    }
}

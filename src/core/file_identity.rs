use crate::storage::sqlite_manager::SqliteStore;
use uuid::Uuid;

pub fn resolve_file_id(store: &SqliteStore, path: &str) -> String {
    if let Some(id) = store.find_file_by_path(path) {
        return id;
    }

    let id = Uuid::new_v4().to_string();
    store.insert_file(&id, path);
    id
}

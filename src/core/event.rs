use std::path::PathBuf;

#[derive(Debug)]
pub enum FsEventType {
    Create,
    Modify,
    Delete,
    Rename,
}

#[derive(Debug)]
pub struct FsEvent {
    pub id: String,
    pub path: PathBuf,
    pub event_type: FsEventType,
    pub timestamp: i64,
}

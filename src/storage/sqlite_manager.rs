use rusqlite::{params, Connection};

pub struct SqliteStore {
    conn: Connection,
}

impl SqliteStore {
    pub fn new(path: &str) -> Self {
        let conn = Connection::open(path).expect("Failed to open DB");

        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;

            CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                path TEXT NOT NULL,
                event_type TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS files (
                file_id TEXT PRIMARY KEY,
                current_path TEXT NOT NULL,
                created_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS snapshots (
                id TEXT PRIMARY KEY,
                file_id TEXT NOT NULL,
                path TEXT NOT NULL,
                size INTEGER NOT NULL,
                sha256 TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            );
            "#,
        )
        .expect("Failed to init DB");

        Self { conn }
    }

    pub fn insert_event(&self, id: &str, path: &str, event_type: &str, timestamp: i64) {
        self.conn
            .execute(
                "INSERT INTO events (id, path, event_type, timestamp)
                 VALUES (?1, ?2, ?3, ?4)",
                params![id, path, event_type, timestamp],
            )
            .expect("Failed to insert event");
    }

    pub fn insert_snapshot(
        &self,
        id: &str,
        file_id: &str,
        path: &str,
        size: u64,
        sha256: &str,
        timestamp: i64,
    ) {
        self.conn.execute(
            "INSERT INTO snapshots (id, file_id, path, size, sha256, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![id, file_id, path, size, sha256, timestamp],
        ).expect("Failed to insert snapshot");
    }

    pub fn find_file_by_path(&self, path: &str) -> Option<String> {
        let mut stmt = self
            .conn
            .prepare("SELECT file_id FROM files WHERE current_path = ?1")
            .ok()?;

        let mut rows = stmt.query(rusqlite::params![path]).ok()?;
        
        if let Some(row) = rows.next().ok()? {
            row.get(0).ok()
        } else {
            None
        }
    }

    pub fn insert_file(&self, file_id: &str, path: &str) {
        let timestamp = chrono::Utc::now().timestamp();
        self.conn.execute(
            "INSERT INTO files (file_id, current_path, created_at)
             VALUES (?1, ?2, ?3)",
            rusqlite::params![file_id, path, timestamp],
        ).expect("Failed to insert file");
    }

    pub fn update_file_path(&self, file_id: &str, new_path: &str) {
        self.conn.execute(
            "UPDATE files SET current_path = ?1 WHERE file_id = ?2",
            rusqlite::params![new_path, file_id],
        ).expect("Failed to update file path");
    }

    pub fn has_duplicate_snapshot(&self, file_id: &str, sha256: &str) -> bool {
        let mut stmt = self
            .conn
            .prepare("SELECT 1 FROM snapshots WHERE file_id = ?1 AND sha256 = ?2 LIMIT 1")
            .expect("Failed to prepare duplicate check query");

        let mut rows = stmt
            .query(rusqlite::params![file_id, sha256])
            .expect("Failed to execute duplicate check query");
        
        rows.next().is_ok_and(|row| row.is_some())
    }
}

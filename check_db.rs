use rusqlite::{params, Connection};

fn main() {
    let conn = Connection::open("sentinel.db").expect("Failed to open DB");
    
    println!("=== SNAPSHOTS ===");
    let mut stmt = conn.prepare("SELECT path, size, sha256, timestamp FROM snapshots ORDER BY timestamp").unwrap();
    
    let snapshot_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, u64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i64>(3)?,
        ))
    }).unwrap();
    
    for snapshot in snapshot_iter {
        let (path, size, sha256, timestamp) = snapshot.unwrap();
        println!("Path: {}", path);
        println!("Size: {} bytes", size);
        println!("SHA256: {}", sha256);
        println!("Timestamp: {}", timestamp);
        println!("---");
    }
    
    println!("\n=== EVENTS ===");
    let mut stmt = conn.prepare("SELECT path, event_type, timestamp FROM events ORDER BY timestamp").unwrap();
    
    let event_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
        ))
    }).unwrap();
    
    for event in event_iter {
        let (path, event_type, timestamp) = event.unwrap();
        println!("Path: {} | Event: {} | Timestamp: {}", path, event_type, timestamp);
    }
}

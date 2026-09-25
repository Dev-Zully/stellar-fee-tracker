//! Configures WAL mode for reliable concurrent SQLite access on Windows.
use rusqlite::{Connection, Result as SqlResult};

pub fn enable_wal_mode(conn: &Connection) -> SqlResult<()> {
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    conn.pragma_update(None, "busy_timeout", 5000)?;
    Ok(())
}

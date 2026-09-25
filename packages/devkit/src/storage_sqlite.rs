//! SQLite-backed storage for FeeRecord rows.
use rusqlite::{params, Connection, Result as SqlResult};

pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    pub fn open(path: &str) -> SqlResult<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS fee_record (
                ledger INTEGER NOT NULL,
                fee_charged INTEGER NOT NULL,
                max_fee INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn insert(&self, ledger: u32, fee_charged: i64, max_fee: i64) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO fee_record (ledger, fee_charged, max_fee) VALUES (?1, ?2, ?3)",
            params![ledger, fee_charged, max_fee],
        )?;
        Ok(())
    }
}

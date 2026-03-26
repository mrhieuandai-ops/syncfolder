use rusqlite::{Connection, Result as SqliteResult};
use std::path::Path;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: &Path) -> SqliteResult<Self> {
        let conn = Connection::open(db_path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        
        // Enable foreign keys
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        
        // Run migrations with version tracking
        self.run_migrations(&conn)?;
        
        Ok(())
    }

    fn run_migrations(&self, conn: &Connection) -> SqliteResult<()> {
        // List of migrations in order
        let migrations: Vec<(&str, &str, i32)> = vec![
            ("0001_init.sql", include_str!("../../../migrations/0001_init.sql"), 1),
            ("0002_sync_jobs.sql", include_str!("../../../migrations/0002_sync_jobs.sql"), 2),
            ("0003_sync_events.sql", include_str!("../../../migrations/0003_sync_events.sql"), 3),
        ];

        for (filename, sql, version) in migrations {
            // Check if migration already applied
            let already_applied: bool = conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM schema_version WHERE version = ?)",
                [version],
                |row| row.get(0),
            ).unwrap_or(false);

            if already_applied {
                log::info!("Migration {} already applied, skipping", filename);
                continue;
            }

            log::info!("Running migration: {}", filename);
            conn.execute_batch(sql)?;
            log::info!("Migration {} applied successfully", filename);
        }

        Ok(())
    }
}

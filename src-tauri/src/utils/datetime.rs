//! DateTime utilities for SyncFolder

/// Format a timestamp for display
pub fn format_timestamp(timestamp: &str) -> String {
    // Parse RFC3339 timestamp and format for display
    // This is a simple implementation; can be enhanced with chrono formatting
    timestamp.to_string()
}

/// Get current UTC timestamp in RFC3339 format
pub fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// Calculate duration between two timestamps in milliseconds
pub fn duration_ms(start: &str, end: &str) -> i64 {
    use chrono::DateTime;
    use chrono::Utc;
    
    let start_dt = DateTime::parse_from_rfc3339(start)
        .ok()
        .map(|dt| dt.with_timezone(&Utc));
    let end_dt = DateTime::parse_from_rfc3339(end)
        .ok()
        .map(|dt| dt.with_timezone(&Utc));
    
    match (start_dt, end_dt) {
        (Some(s), Some(e)) => (e - s).num_milliseconds(),
        _ => 0,
    }
}

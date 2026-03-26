//! Event system for SyncFolder
//! 
//! Events emitted during sync operations:
//! - sync:queued - Job queued for execution
//! - sync:started - Job started
//! - sync:progress - Job progress update
//! - sync:completed - Job completed successfully
//! - sync:failed - Job failed

pub mod sync_events;
pub use sync_events::*;

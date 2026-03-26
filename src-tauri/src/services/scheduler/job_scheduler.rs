use crate::models::{SyncProfile, JobSource, SyncJob};
use crate::repositories::{JobsRepository, ProfilesRepository};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::interval;

/// Schedule intervals in minutes
pub const SCHEDULE_INTERVALS: &[i32] = &[30, 60, 90, 1440];

/// Scheduler message types
#[derive(Debug)]
pub enum SchedulerMessage {
    TriggerSync(String), // profile_id
    Shutdown,
}

/// Job scheduler service
pub struct JobScheduler {
    profiles_repo: Arc<ProfilesRepository>,
    jobs_repo: Arc<JobsRepository>,
    sender: mpsc::Sender<SchedulerMessage>,
}

impl JobScheduler {
    pub fn new(
        profiles_repo: Arc<ProfilesRepository>,
        jobs_repo: Arc<JobsRepository>,
    ) -> Self {
        let (sender, _receiver) = mpsc::channel::<SchedulerMessage>(100);
        Self {
            profiles_repo,
            jobs_repo,
            sender,
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<SchedulerMessage> {
        self.sender.clone()
    }

    /// Check if a profile has a running job (anti-overlap)
    pub fn is_profile_running(&self, profile_id: &str) -> bool {
        match self.jobs_repo.get_running_for_profile(profile_id) {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(_) => false,
        }
    }

    /// Create a scheduled job for a profile
    pub fn create_scheduled_job(&self, profile_id: &str) -> Result<Option<SyncJob>, String> {
        // Check anti-overlap
        if self.is_profile_running(profile_id) {
            return Ok(None);
        }

        // Verify profile exists
        let profile = match self.profiles_repo.get_by_id(profile_id)? {
            Some(p) => p,
            None => return Err(format!("Profile not found: {}", profile_id)),
        };

        // Create job
        let job = SyncJob::new(profile_id.to_string(), JobSource::Scheduled);
        self.jobs_repo.create(&job)?;

        log::info!("Created scheduled job {} for profile {}", job.id, profile_id);
        Ok(Some(job))
    }

    /// Check and trigger due scheduled jobs
    pub fn check_and_trigger_scheduled_jobs(&self) -> Result<Vec<String>, String> {
        let profiles = self.profiles_repo.get_auto_sync_enabled()?;
        let now = chrono::Utc::now();
        let mut triggered = Vec::new();

        for profile in profiles {
            if let Some(interval) = profile.schedule_interval {
                let should_run = if let Some(last_run) = &profile.last_run_at {
                    // Parse last run time and check if interval has passed
                    if let Ok(last_run_time) = chrono::DateTime::parse_from_rfc3339(last_run) {
                        let elapsed = now.signed_duration_since(last_run_time.with_timezone(&chrono::Utc));
                        elapsed.num_minutes() >= interval as i64
                    } else {
                        true // No valid last run, should run
                    }
                } else {
                    true // Never run, should run
                };

                if should_run && !self.is_profile_running(&profile.id) {
                    match self.create_scheduled_job(&profile.id) {
                        Ok(Some(job)) => {
                            triggered.push(job.id);
                            log::info!("Triggered scheduled job for profile {}", profile.name);
                        }
                        Ok(None) => {
                            // Skipped due to overlap
                            log::debug!("Skipped scheduled job for profile {} - overlap prevented", profile.name);
                        }
                        Err(e) => {
                            log::error!("Error creating scheduled job for profile {}: {}", profile.name, e);
                        }
                    }
                }
            }
        }

        Ok(triggered)
    }
}

/// Start the scheduler background task
pub fn start_scheduler_task(
    scheduler: Arc<JobScheduler>,
    mut receiver: mpsc::Receiver<SchedulerMessage>,
) {
    tokio::spawn(async move {
        let mut check_interval = interval(Duration::from_secs(60)); // Check every minute

        loop {
            tokio::select! {
                _ = check_interval.tick() => {
                    // Check for scheduled jobs
                    if let Err(e) = scheduler.check_and_trigger_scheduled_jobs() {
                        log::error!("Scheduler error: {}", e);
                    }
                }
                msg = receiver.recv() => {
                    match msg {
                        Some(SchedulerMessage::TriggerSync(profile_id)) => {
                            log::info!("Manual sync triggered for profile {}", profile_id);
                            if let Err(e) = scheduler.check_and_trigger_scheduled_jobs() {
                                log::error!("Manual sync error: {}", e);
                            }
                        }
                        Some(SchedulerMessage::Shutdown) | None => {
                            log::info!("Scheduler shutting down");
                            break;
                        }
                    }
                }
            }
        }
    });
}

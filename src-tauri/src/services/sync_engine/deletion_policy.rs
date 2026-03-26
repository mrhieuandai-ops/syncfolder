use super::scanner::ScanResult;
use serde::{Deserialize, Serialize};

/// Deletion policy rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeletionPolicy {
    /// Two-way sync: deleted on one side = delete on other side
    TwoWaySync,
    /// Archive deleted files instead of removing
    Archive,
    /// Never delete, only copy
    NeverDelete,
}

impl Default for DeletionPolicy {
    fn default() -> Self {
        DeletionPolicy::TwoWaySync
    }
}

/// Deletion decision with reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletionDecision {
    pub relative_path: String,
    pub policy_applied: DeletionPolicy,
    pub is_risky: bool,
    pub reason: String,
    /// Source of deletion: true if deleted from source, false if deleted from dest
    pub deleted_from_source: bool,
}

/// Deletion safety assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyAssessment {
    pub is_safe: bool,
    pub risky_paths: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Deletion policy evaluator
pub struct DeletionPolicyEvaluator {
    policy: DeletionPolicy,
}

impl DeletionPolicyEvaluator {
    pub fn new(policy: DeletionPolicy) -> Self {
        Self { policy }
    }

    /// Assess if deletion operations are safe to proceed
    pub fn assess_safety(&self, scan_result: &ScanResult) -> SafetyAssessment {
        let mut risky_paths = Vec::new();
        let mut recommendations = Vec::new();

        // Check for inconsistent state
        // If there are many files only on one side, might indicate an issue
        let imbalance_threshold = 0.5; // 50% imbalance
        let total_files = scan_result.source_manifest.file_count + scan_result.dest_manifest.file_count;
        
        if total_files > 0 {
            let source_ratio = scan_result.source_only.len() as f64 / total_files as f64;
            let dest_ratio = scan_result.dest_only.len() as f64 / total_files as f64;

            if source_ratio > imbalance_threshold || dest_ratio > imbalance_threshold {
                risky_paths.push("Directory structure imbalance detected".to_string());
                recommendations.push("Verify both directories are accessible before proceeding".to_string());
                recommendations.push("Consider running a manual sync to establish baseline".to_string());
            }
        }

        // Check for very recent modifications that might indicate active work
        let now = chrono::Utc::now().timestamp();
        for (path, meta) in &scan_result.source_manifest.files {
            if !meta.is_dir && now - meta.mtime < 300 {
                // Modified within last 5 minutes
                recommendations.push(format!("File '{}' was recently modified", path));
            }
        }

        let is_safe = risky_paths.is_empty() && recommendations.is_empty();

        SafetyAssessment {
            is_safe,
            risky_paths,
            recommendations,
        }
    }

    /// Generate deletion decisions based on scan result and policy
    pub fn evaluate(&self, scan_result: &ScanResult) -> Vec<DeletionDecision> {
        let mut decisions = Vec::new();

        match self.policy {
            DeletionPolicy::TwoWaySync => {
                // Files only in source were deleted from dest
                // So we need to delete from dest to match source
                for path in &scan_result.source_only {
                    decisions.push(DeletionDecision {
                        relative_path: path.clone(),
                        policy_applied: self.policy.clone(),
                        is_risky: false,
                        reason: "File exists in source but not in dest - will sync deletion".to_string(),
                        deleted_from_source: false, // deleted from dest (two-way sync)
                    });
                }

                // Files only in dest were deleted from source
                // So we need to delete from source to match dest
                for path in &scan_result.dest_only {
                    decisions.push(DeletionDecision {
                        relative_path: path.clone(),
                        policy_applied: self.policy.clone(),
                        is_risky: false,
                        reason: "File exists in dest but not in source - will sync deletion".to_string(),
                        deleted_from_source: true, // deleted from source (two-way sync)
                    });
                }
            }
            DeletionPolicy::Archive => {
                // For archive policy, we don't delete, we move to a trash location
                for path in &scan_result.source_only {
                    decisions.push(DeletionDecision {
                        relative_path: path.clone(),
                        policy_applied: self.policy.clone(),
                        is_risky: false,
                        reason: "Archive policy: file would be moved to trash".to_string(),
                        deleted_from_source: false,
                    });
                }
            }
            DeletionPolicy::NeverDelete => {
                // No deletion operations
            }
        }

        decisions
    }

    /// Check if deletions should be skipped based on safety assessment
    pub fn should_skip_deletions(&self, safety: &SafetyAssessment) -> bool {
        !safety.is_safe || self.policy == DeletionPolicy::NeverDelete
    }
}

impl Clone for DeletionPolicyEvaluator {
    fn clone(&self) -> Self {
        Self::new(self.policy.clone())
    }
}

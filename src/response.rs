//! Health check response types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Health check status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    /// Check passed
    Healthy,
    /// Check failed
    Unhealthy,
    /// Check status unknown
    Unknown,
}

/// Individual check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    /// Status of this check
    pub status: CheckStatus,
    /// Optional message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Check duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

impl CheckResult {
    /// Create a healthy check result
    pub fn healthy() -> Self {
        Self {
            status: CheckStatus::Healthy,
            message: None,
            duration_ms: None,
        }
    }

    /// Create a healthy check result with a message
    pub fn healthy_with_message(message: impl Into<String>) -> Self {
        Self {
            status: CheckStatus::Healthy,
            message: Some(message.into()),
            duration_ms: None,
        }
    }

    /// Create an unhealthy check result
    pub fn unhealthy(message: impl Into<String>) -> Self {
        Self {
            status: CheckStatus::Unhealthy,
            message: Some(message.into()),
            duration_ms: None,
        }
    }

    /// Create an unknown status check result
    pub fn unknown(message: impl Into<String>) -> Self {
        Self {
            status: CheckStatus::Unknown,
            message: Some(message.into()),
            duration_ms: None,
        }
    }

    /// Set duration
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }
}

/// Complete health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Overall status (healthy if all checks pass)
    pub status: CheckStatus,
    /// Service name
    pub service: String,
    /// Individual check results
    pub checks: HashMap<String, CheckResult>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
    /// Service version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl HealthResponse {
    /// Create a new health response
    pub fn new(service: impl Into<String>) -> Self {
        Self {
            status: CheckStatus::Healthy,
            service: service.into(),
            checks: HashMap::new(),
            timestamp: Utc::now(),
            version: None,
        }
    }

    /// Set service version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Add a check result
    pub fn add_check(mut self, name: impl Into<String>, result: CheckResult) -> Self {
        let name = name.into();

        // Update overall status if this check is unhealthy
        if result.status == CheckStatus::Unhealthy {
            self.status = CheckStatus::Unhealthy;
        }

        self.checks.insert(name, result);
        self
    }

    /// Check if all checks are healthy
    pub fn is_healthy(&self) -> bool {
        self.status == CheckStatus::Healthy
    }
}

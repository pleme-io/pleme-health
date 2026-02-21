//! Health check builder for composable health checks

use crate::checks::HealthCheck;
use crate::response::{HealthResponse, CheckResult};
use crate::routes::HealthRoutes;
use std::collections::HashMap;
use std::sync::Arc;

/// Builder for composable health checks
pub struct HealthCheckBuilder {
    service_name: String,
    version: Option<String>,
    checks: HashMap<String, HealthCheck>,
}

impl HealthCheckBuilder {
    /// Create a new health check builder
    pub fn new(service_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            version: Some(version.into()),
            checks: HashMap::new(),
        }
    }

    /// Create a builder without version
    pub fn without_version(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            version: None,
            checks: HashMap::new(),
        }
    }

    /// Add a health check
    pub fn add_check(mut self, name: impl Into<String>, check: HealthCheck) -> Self {
        self.checks.insert(name.into(), check);
        self
    }

    /// Build the health check system
    pub fn build(self) -> HealthRoutes {
        HealthRoutes {
            service_name: Arc::new(self.service_name),
            version: self.version.map(Arc::new),
            checks: Arc::new(self.checks),
        }
    }
}

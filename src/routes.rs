//! Axum route integration for health checks

use crate::checks::HealthCheck;
use crate::response::{HealthResponse, CheckStatus};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Health routes for Axum integration
#[derive(Clone)]
pub struct HealthRoutes {
    pub(crate) service_name: Arc<String>,
    pub(crate) version: Option<Arc<String>>,
    pub(crate) checks: Arc<HashMap<String, HealthCheck>>,
}

impl HealthRoutes {
    /// Create Axum routes for health endpoints
    ///
    /// Adds:
    /// - `GET /health` - Liveness probe (always returns 200)
    /// - `GET /ready` - Readiness probe (200 if healthy, 503 if not)
    pub fn routes(&self) -> Router {
        let health_handler = self.clone();
        let ready_handler = self.clone();

        Router::new()
            .route("/health", get(move || health_endpoint(health_handler)))
            .route("/ready", get(move || readiness_endpoint(ready_handler)))
    }
}

/// Health endpoint handler (liveness probe)
///
/// Always returns 200 OK with basic service info
async fn health_endpoint(routes: HealthRoutes) -> Json<HealthResponse> {
    let mut response = HealthResponse::new(routes.service_name.as_str());

    if let Some(version) = &routes.version {
        response = response.with_version(version.as_str());
    }

    Json(response)
}

/// Readiness endpoint handler (readiness probe)
///
/// Returns 200 OK if all checks pass, 503 Service Unavailable otherwise
async fn readiness_endpoint(routes: HealthRoutes) -> Response {
    let mut response = HealthResponse::new(routes.service_name.as_str());

    if let Some(version) = &routes.version {
        response = response.with_version(version.as_str());
    }

    // Run all health checks
    for (name, check) in routes.checks.iter() {
        let result = check().await;
        response = response.add_check(name, result);
    }

    // Return appropriate status code
    let status_code = if response.is_healthy() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(response)).into_response()
}

/// Standalone health check routes (simpler API)
///
/// Create health routes directly without builder
pub fn health_routes(
    service_name: impl Into<String>,
    version: Option<String>,
    checks: HashMap<String, HealthCheck>,
) -> Router {
    HealthRoutes {
        service_name: Arc::new(service_name.into()),
        version: version.map(Arc::new),
        checks: Arc::new(checks),
    }
    .routes()
}

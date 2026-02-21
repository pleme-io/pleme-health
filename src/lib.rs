//! Pleme Health - Health check patterns for Nexus platform services
//!
//! This library provides standardized health check patterns for:
//! - Kubernetes liveness and readiness probes
//! - Composable health check builders
//! - Built-in checks for PostgreSQL, Redis, HTTP endpoints
//! - Axum integration helpers
//!
//! # Example
//!
//! ```rust,no_run
//! use pleme_health::{HealthCheckBuilder, postgres_check, redis_check};
//! use axum::Router;
//! use sqlx::PgPool;
//!
//! # async fn example(pool: PgPool, redis_url: String) {
//! // Build health checks
//! let health = HealthCheckBuilder::new("my-service", "1.0.0")
//!     .add_check("database", postgres_check(pool.clone()))
//!     .add_check("cache", redis_check(redis_url))
//!     .build();
//!
//! // Add to Axum router
//! let app = Router::new()
//!     .merge(health.routes());
//! # }
//! ```

pub mod checks;
pub mod builder;
pub mod response;
pub mod routes;

// Re-export commonly used types
pub use builder::HealthCheckBuilder;
pub use response::{HealthResponse, CheckStatus};
pub use checks::{postgres_check, redis_check, http_check};
pub use routes::health_routes;

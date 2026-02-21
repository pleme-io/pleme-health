//! Built-in health checks for common dependencies

use crate::response::CheckResult;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

/// Type alias for async health check functions
pub type HealthCheck = Box<dyn Fn() -> Pin<Box<dyn Future<Output = CheckResult> + Send>> + Send + Sync>;

/// Create a PostgreSQL health check
///
/// Executes `SELECT 1` to verify database connectivity
pub fn postgres_check(pool: sqlx::PgPool) -> HealthCheck {
    Box::new(move || {
        let pool = pool.clone();
        Box::pin(async move {
            let start = Instant::now();

            match sqlx::query("SELECT 1")
                .fetch_one(&pool)
                .await
            {
                Ok(_) => {
                    let duration = start.elapsed().as_millis() as u64;
                    CheckResult::healthy()
                        .with_duration(duration)
                }
                Err(e) => CheckResult::unhealthy(format!("Database connection failed: {}", e)),
            }
        })
    })
}

/// Create a Redis health check
///
/// Executes `PING` to verify Redis connectivity
pub fn redis_check(redis_url: String) -> HealthCheck {
    Box::new(move || {
        let redis_url = redis_url.clone();
        Box::pin(async move {
            let start = Instant::now();

            match redis::Client::open(redis_url.as_str()) {
                Ok(client) => {
                    match client.get_multiplexed_async_connection().await {
                        Ok(mut con) => {
                            use redis::AsyncCommands;
                            // Use SET/GET instead of PING for health check
                            match con.get::<&str, Option<String>>("__health_check__").await {
                                Ok(_) => {
                                    let duration = start.elapsed().as_millis() as u64;
                                    CheckResult::healthy()
                                        .with_duration(duration)
                                }
                                Err(e) => CheckResult::unhealthy(format!("Redis check failed: {}", e)),
                            }
                        }
                        Err(e) => CheckResult::unhealthy(format!("Redis connection failed: {}", e)),
                    }
                }
                Err(e) => CheckResult::unhealthy(format!("Redis client creation failed: {}", e)),
            }
        })
    })
}

/// Create an HTTP endpoint health check
///
/// Makes a GET request to the specified URL
pub fn http_check(url: String, expected_status: u16) -> HealthCheck {
    Box::new(move || {
        let url = url.clone();
        Box::pin(async move {
            let start = Instant::now();

            match reqwest::get(&url).await {
                Ok(response) => {
                    let duration = start.elapsed().as_millis() as u64;
                    let status = response.status().as_u16();

                    if status == expected_status {
                        CheckResult::healthy_with_message(format!("HTTP {} OK", status))
                            .with_duration(duration)
                    } else {
                        CheckResult::unhealthy(format!(
                            "Expected status {}, got {}",
                            expected_status, status
                        ))
                    }
                }
                Err(e) => CheckResult::unhealthy(format!("HTTP request failed: {}", e)),
            }
        })
    })
}

/// Create a custom health check from an async function
pub fn custom_check<F, Fut>(f: F) -> HealthCheck
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = CheckResult> + Send + 'static,
{
    Box::new(move || Box::pin(f()))
}

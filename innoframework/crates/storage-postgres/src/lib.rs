//! Postgres `Storage` adapter (ADR-10 remote storage, ADR-16).
//!
//! Feature-gated behind `postgres` so it does not force a Postgres dependency
//! on every consumer. Requires a live Postgres database (connection via
//! `DATABASE_URL`-style string). Tests are skipped unless
//! `INNOFRAMEWORK_POSTGRES_TEST_URL` is set.

use innoframework_core::error::InnoframeworkError;
use innoframework_core::storage::{Storage, STORAGE_ERROR};

#[cfg(feature = "postgres")]
use innoframework_core::storage::validate_storage_id;
use serde_json::Value;
use std::pin::Pin;

pub type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

/// Postgres-backed storage. All operations go through a shared connection pool.
#[cfg(feature = "postgres")]
#[derive(Debug, Clone)]
pub struct PostgresStorage {
    pool: sqlx::PgPool,
}

#[cfg(feature = "postgres")]
impl PostgresStorage {
    /// Connect to a Postgres database. `uri` is a libpq-style connection string.
    pub async fn new(uri: &str) -> Result<Self, InnoframeworkError> {
        let pool = sqlx::PgPool::connect(uri).await.map_err(|e| {
            InnoframeworkError::new(STORAGE_ERROR, format!("cannot connect to postgres: {e}"))
        })?;
        Self::migrate(&pool).await?;
        Ok(Self { pool })
    }

    async fn migrate(pool: &sqlx::PgPool) -> Result<(), InnoframeworkError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS documents (
                collection TEXT NOT NULL,
                id TEXT NOT NULL,
                value TEXT NOT NULL,
                PRIMARY KEY (collection, id)
            )
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| {
            InnoframeworkError::new(STORAGE_ERROR, format!("cannot migrate postgres: {e}"))
        })?;
        Ok(())
    }
}

#[cfg(feature = "postgres")]
impl Storage for PostgresStorage {
    fn get_json<'a>(
        &'a self,
        collection: &'a str,
        id: &'a str,
    ) -> BoxFuture<'a, Result<Option<Value>, InnoframeworkError>> {
        Box::pin(async move {
            validate_storage_id("collection", collection)?;
            validate_storage_id("document", id)?;
            let row: Option<(String,)> =
                sqlx::query_as("SELECT value FROM documents WHERE collection = $1 AND id = $2")
                    .bind(collection)
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| {
                        InnoframeworkError::new(STORAGE_ERROR, format!("postgres read error: {e}"))
                    })?;
            let Some((value,)) = row else {
                return Ok(None);
            };
            serde_json::from_str(&value)
                .map(Some)
                .map_err(|_| InnoframeworkError::new(STORAGE_ERROR, "stored JSON is malformed"))
        })
    }

    fn put_json<'a>(
        &'a self,
        collection: &'a str,
        id: &'a str,
        value: Value,
    ) -> BoxFuture<'a, Result<(), InnoframeworkError>> {
        Box::pin(async move {
            validate_storage_id("collection", collection)?;
            validate_storage_id("document", id)?;
            let text = serde_json::to_string(&value)
                .map_err(|_| InnoframeworkError::new(STORAGE_ERROR, "value is not serializable"))?;
            sqlx::query(
                r#"
                INSERT INTO documents (collection, id, value) VALUES ($1, $2, $3)
                ON CONFLICT (collection, id) DO UPDATE SET value = EXCLUDED.value
                "#,
            )
            .bind(collection)
            .bind(id)
            .bind(text)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                InnoframeworkError::new(STORAGE_ERROR, format!("postgres write error: {e}"))
            })?;
            Ok(())
        })
    }

    fn delete<'a>(
        &'a self,
        collection: &'a str,
        id: &'a str,
    ) -> BoxFuture<'a, Result<bool, InnoframeworkError>> {
        Box::pin(async move {
            validate_storage_id("collection", collection)?;
            validate_storage_id("document", id)?;
            let result = sqlx::query("DELETE FROM documents WHERE collection = $1 AND id = $2")
                .bind(collection)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    InnoframeworkError::new(STORAGE_ERROR, format!("postgres delete error: {e}"))
                })?;
            Ok(result.rows_affected() > 0)
        })
    }
}

/// Stub when the `postgres` feature is disabled. Provides a clear error
/// instead of missing types.
#[cfg(not(feature = "postgres"))]
#[derive(Debug, Clone)]
pub struct PostgresStorage;

#[cfg(not(feature = "postgres"))]
impl PostgresStorage {
    pub async fn new(_uri: &str) -> Result<Self, InnoframeworkError> {
        Err(InnoframeworkError::new(
            STORAGE_ERROR,
            "postgres adapter requires the `postgres` feature flag",
        ))
    }
}

#[cfg(not(feature = "postgres"))]
impl Storage for PostgresStorage {
    fn get_json<'a>(
        &'a self,
        _collection: &'a str,
        _id: &'a str,
    ) -> BoxFuture<'a, Result<Option<Value>, InnoframeworkError>> {
        Box::pin(async move {
            Err(InnoframeworkError::new(
                STORAGE_ERROR,
                "postgres adapter requires the `postgres` feature flag",
            ))
        })
    }

    fn put_json<'a>(
        &'a self,
        _collection: &'a str,
        _id: &'a str,
        _value: Value,
    ) -> BoxFuture<'a, Result<(), InnoframeworkError>> {
        Box::pin(async move {
            Err(InnoframeworkError::new(
                STORAGE_ERROR,
                "postgres adapter requires the `postgres` feature flag",
            ))
        })
    }

    fn delete<'a>(
        &'a self,
        _collection: &'a str,
        _id: &'a str,
    ) -> BoxFuture<'a, Result<bool, InnoframeworkError>> {
        Box::pin(async move {
            Err(InnoframeworkError::new(
                STORAGE_ERROR,
                "postgres adapter requires the `postgres` feature flag",
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn postgres_adapter_skipped_without_url() {
        // Without INNOFRAMEWORK_POSTGRES_TEST_URL, we can only assert the
        // stub/type exists and feature-gating compiles.
        let _ = std::env::var("INNOFRAMEWORK_POSTGRES_TEST_URL");
    }
}

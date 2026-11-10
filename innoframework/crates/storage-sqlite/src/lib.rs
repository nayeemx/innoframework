//! SQLite `Storage` adapter (ADR-10 dev storage, ADR-16).
//!
//! Stores one JSON document per row in a `documents` table:
//!   CREATE TABLE documents (collection TEXT, id TEXT, value TEXT,
//!                           PRIMARY KEY (collection, id))
//!
//! Uses sqlx + SQLite (in-memory or file-backed). Suitable for local
//! development and single-user desktop scenarios. Postgres adapter lives in
//! a separate crate behind its own feature gate.

use innoframework_core::error::InnoframeworkError;
use innoframework_core::storage::{validate_storage_id, Storage, STORAGE_ERROR};
use serde_json::Value;
use sqlx::sqlite::SqlitePool;
use std::path::Path;
use std::pin::Pin;

pub type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

/// SQLite-backed storage. All operations go through a shared connection pool.
#[derive(Debug, Clone)]
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    /// Connect to a SQLite database. `uri` can be a file path or `:memory:`.
    pub async fn new(uri: &str) -> Result<Self, InnoframeworkError> {
        let pool = SqlitePool::connect(uri).await.map_err(|e| {
            InnoframeworkError::new(STORAGE_ERROR, format!("cannot connect to sqlite: {e}"))
        })?;
        Self::migrate(&pool).await?;
        Ok(Self { pool })
    }

    /// Connect to a SQLite database file at `path`.
    pub async fn new_file(path: &Path) -> Result<Self, InnoframeworkError> {
        let uri = format!("{}", path.display());
        Self::new(&uri).await
    }

    async fn migrate(pool: &SqlitePool) -> Result<(), InnoframeworkError> {
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
            InnoframeworkError::new(STORAGE_ERROR, format!("cannot migrate sqlite: {e}"))
        })?;
        Ok(())
    }
}

impl Storage for SqliteStorage {
    fn get_json<'a>(
        &'a self,
        collection: &'a str,
        id: &'a str,
    ) -> BoxFuture<'a, Result<Option<Value>, InnoframeworkError>> {
        Box::pin(async move {
            validate_storage_id("collection", collection)?;
            validate_storage_id("document", id)?;
            let row: Option<(String,)> =
                sqlx::query_as("SELECT value FROM documents WHERE collection = ?1 AND id = ?2")
                    .bind(collection)
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| {
                        InnoframeworkError::new(STORAGE_ERROR, format!("sqlite read error: {e}"))
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
                INSERT INTO documents (collection, id, value) VALUES (?1, ?2, ?3)
                ON CONFLICT(collection, id) DO UPDATE SET value = excluded.value
                "#,
            )
            .bind(collection)
            .bind(id)
            .bind(text)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                InnoframeworkError::new(STORAGE_ERROR, format!("sqlite write error: {e}"))
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
            let result = sqlx::query("DELETE FROM documents WHERE collection = ?1 AND id = ?2")
                .bind(collection)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    InnoframeworkError::new(STORAGE_ERROR, format!("sqlite delete error: {e}"))
                })?;
            Ok(result.rows_affected() > 0)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn sqlite_storage_round_trip() {
        let store = SqliteStorage::new(":memory:").await.unwrap();

        store
            .put_json("items", "a1", json!({ "name": "x", "n": 1 }))
            .await
            .unwrap();

        let got = store.get_json("items", "a1").await.unwrap().unwrap();
        assert_eq!(got["name"], "x");
        assert_eq!(got["n"], 1);

        assert!(store.get_json("items", "missing").await.unwrap().is_none());
        assert!(store.delete("items", "a1").await.unwrap());
        assert!(!store.delete("items", "a1").await.unwrap());
    }

    #[tokio::test]
    async fn sqlite_storage_rejects_bad_ids() {
        let store = SqliteStorage::new(":memory:").await.unwrap();

        assert!(store.get_json("../evil", "x").await.is_err());
        assert!(store.get_json("items", "../evil").await.is_err());
        assert!(store.put_json("", "x", json!({})).await.is_err());
    }

    #[tokio::test]
    async fn sqlite_storage_upsert_overwrites() {
        let store = SqliteStorage::new(":memory:").await.unwrap();

        store
            .put_json("items", "a1", json!({ "v": 1 }))
            .await
            .unwrap();
        store
            .put_json("items", "a1", json!({ "v": 2 }))
            .await
            .unwrap();

        let got = store.get_json("items", "a1").await.unwrap().unwrap();
        assert_eq!(got["v"], 2);
    }
}

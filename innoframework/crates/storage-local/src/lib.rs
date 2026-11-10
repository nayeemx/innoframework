//! Local JSON file-backed `Storage` adapter (ADR-10 dev storage, ADR-16).
//!
//! Stores one JSON document per file under `<root>/<collection>/<id>.json`.
//! Intended for local development and tests; Postgres/SQLite adapters are
//! deferred to later P6 work behind their own feature gates.

use innoframework_core::error::InnoframeworkError;
use innoframework_core::storage::{validate_storage_id, Storage, STORAGE_ERROR};
use serde_json::Value;
use std::path::{Path, PathBuf};
use tokio::fs;

/// File-system backed storage. All operations are relative to `root`.
#[derive(Debug, Clone)]
pub struct LocalStorage {
    root: PathBuf,
}

impl LocalStorage {
    /// Create a `LocalStorage` rooted at `root`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn doc_path(&self, collection: &str, id: &str) -> PathBuf {
        self.root.join(collection).join(format!("{id}.json"))
    }

    async fn ensure_parent(path: &Path) -> Result<(), InnoframeworkError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                InnoframeworkError::new(STORAGE_ERROR, format!("cannot create storage dir: {e}"))
            })?;
        }
        Ok(())
    }
}

impl Storage for LocalStorage {
    fn get_json<'a>(
        &'a self,
        collection: &'a str,
        id: &'a str,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Option<Value>, InnoframeworkError>> + Send + 'a,
        >,
    > {
        Box::pin(async move {
            validate_storage_id("collection", collection)?;
            validate_storage_id("document", id)?;
            let path = self.doc_path(collection, id);
            match fs::read_to_string(&path).await {
                Ok(text) => serde_json::from_str(&text).map(Some).map_err(|_| {
                    InnoframeworkError::new(STORAGE_ERROR, "stored JSON is malformed")
                }),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(InnoframeworkError::new(
                    STORAGE_ERROR,
                    format!("cannot read storage document: {e}"),
                )),
            }
        })
    }

    fn put_json<'a>(
        &'a self,
        collection: &'a str,
        id: &'a str,
        value: Value,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), InnoframeworkError>> + Send + 'a>,
    > {
        Box::pin(async move {
            validate_storage_id("collection", collection)?;
            validate_storage_id("document", id)?;
            let path = self.doc_path(collection, id);
            Self::ensure_parent(&path).await?;
            let text = serde_json::to_string_pretty(&value)
                .map_err(|_| InnoframeworkError::new(STORAGE_ERROR, "value is not serializable"))?;
            fs::write(&path, text).await.map_err(|e| {
                InnoframeworkError::new(
                    STORAGE_ERROR,
                    format!("cannot write storage document: {e}"),
                )
            })?;
            Ok(())
        })
    }

    fn delete<'a>(
        &'a self,
        collection: &'a str,
        id: &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<bool, InnoframeworkError>> + Send + 'a>,
    > {
        Box::pin(async move {
            validate_storage_id("collection", collection)?;
            validate_storage_id("document", id)?;
            let path = self.doc_path(collection, id);
            match fs::remove_file(&path).await {
                Ok(()) => Ok(true),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
                Err(e) => Err(InnoframeworkError::new(
                    STORAGE_ERROR,
                    format!("cannot delete storage document: {e}"),
                )),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    async fn temp_root() -> std::io::Result<PathBuf> {
        let dir = std::env::temp_dir().join(format!("inno-storage-test-{}", std::process::id()));
        fs::create_dir_all(&dir).await?;
        Ok(dir)
    }

    #[tokio::test]
    async fn local_storage_round_trip() {
        let root = temp_root().await.unwrap();
        let store = LocalStorage::new(&root);

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
    async fn local_storage_rejects_bad_ids() {
        let root = temp_root().await.unwrap();
        let store = LocalStorage::new(&root);

        assert!(store.get_json("../evil", "x").await.is_err());
        assert!(store.get_json("items", "../evil").await.is_err());
        assert!(store.put_json("", "x", json!({})).await.is_err());
    }
}

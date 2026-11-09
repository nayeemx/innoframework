//! Storage trait (ADR-05, ADR-16).
//!
//! Concrete adapters (Firebase dev, Postgres, SQLite) are provided outside core.
//! The trait is defined here so adapter choice does not block P1-P5/P6 wiring.
//! Core stays dependency-light; adapters own transport/credentials.

use crate::error::{ErrorCode, InnoframeworkError};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;

pub const STORAGE_ERROR: ErrorCode = ErrorCode::new_const(1600);

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Minimal collection/document storage contract (P6 adapters implement this).
pub trait Storage: Send + Sync {
    fn get_json<'a>(
        &'a self,
        collection: &'a str,
        id: &'a str,
    ) -> BoxFuture<'a, Result<Option<Value>, InnoframeworkError>>;

    fn put_json<'a>(
        &'a self,
        collection: &'a str,
        id: &'a str,
        value: Value,
    ) -> BoxFuture<'a, Result<(), InnoframeworkError>>;

    fn delete<'a>(
        &'a self,
        collection: &'a str,
        id: &'a str,
    ) -> BoxFuture<'a, Result<bool, InnoframeworkError>>;
}

pub fn validate_storage_id(kind: &str, value: &str) -> Result<(), InnoframeworkError> {
    let ok = !value.is_empty()
        && value.len() <= 256
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'));
    if ok {
        Ok(())
    } else {
        Err(InnoframeworkError::new(
            STORAGE_ERROR,
            format!("invalid {kind} id"),
        ))
    }
}

pub async fn get_typed<S: Storage + ?Sized, T: DeserializeOwned>(
    store: &S,
    collection: &str,
    id: &str,
) -> Result<Option<T>, InnoframeworkError> {
    validate_storage_id("collection", collection)?;
    validate_storage_id("document", id)?;
    let Some(value) = store.get_json(collection, id).await? else {
        return Ok(None);
    };
    serde_json::from_value(value).map(Some).map_err(|_| {
        InnoframeworkError::new(STORAGE_ERROR, "stored value does not match expected type")
    })
}

pub async fn put_typed<S: Storage + ?Sized, T: Serialize + Sync>(
    store: &S,
    collection: &str,
    id: &str,
    value: &T,
) -> Result<(), InnoframeworkError> {
    validate_storage_id("collection", collection)?;
    validate_storage_id("document", id)?;
    let json = serde_json::to_value(value)
        .map_err(|_| InnoframeworkError::new(STORAGE_ERROR, "value is not serializable"))?;
    store.put_json(collection, id, json).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[derive(Default)]
    struct MemStore(Mutex<HashMap<(String, String), Value>>);

    impl Storage for MemStore {
        fn get_json<'a>(
            &'a self,
            collection: &'a str,
            id: &'a str,
        ) -> BoxFuture<'a, Result<Option<Value>, InnoframeworkError>> {
            Box::pin(async move {
                Ok(self
                    .0
                    .lock()
                    .unwrap()
                    .get(&(collection.to_string(), id.to_string()))
                    .cloned())
            })
        }

        fn put_json<'a>(
            &'a self,
            collection: &'a str,
            id: &'a str,
            value: Value,
        ) -> BoxFuture<'a, Result<(), InnoframeworkError>> {
            Box::pin(async move {
                self.0
                    .lock()
                    .unwrap()
                    .insert((collection.to_string(), id.to_string()), value);
                Ok(())
            })
        }

        fn delete<'a>(
            &'a self,
            collection: &'a str,
            id: &'a str,
        ) -> BoxFuture<'a, Result<bool, InnoframeworkError>> {
            Box::pin(async move {
                Ok(self
                    .0
                    .lock()
                    .unwrap()
                    .remove(&(collection.to_string(), id.to_string()))
                    .is_some())
            })
        }
    }

    #[derive(Debug, Serialize, serde::Deserialize, PartialEq)]
    struct Item {
        name: String,
    }

    #[tokio::test]
    async fn typed_round_trip() {
        let store = MemStore::default();
        put_typed(&store, "items", "a1", &Item { name: "x".into() })
            .await
            .unwrap();
        let got: Option<Item> = get_typed(&store, "items", "a1").await.unwrap();
        assert_eq!(got, Some(Item { name: "x".into() }));
    }

    #[test]
    fn rejects_path_traversal_ids() {
        assert!(validate_storage_id("document", "../etc").is_err());
        assert!(validate_storage_id("collection", "").is_err());
        assert!(validate_storage_id("document", "ok_1.2:3").is_ok());
    }
}

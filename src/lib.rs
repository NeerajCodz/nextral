pub mod contracts;
pub mod graph;
pub mod memory;
pub mod retrieval;
pub mod runtime;
pub mod scoring;

pub use contracts::{CoreError, CoreResult};

#[cfg(test)]
mod tests {
    use crate::{memory::MemoryRecord, runtime};

    #[tokio::test]
    async fn scored_search_returns_matches() {
        let records = vec![
            MemoryRecord {
                id: "1".to_string(),
                content: "tokio runtime boundary".to_string(),
            },
            MemoryRecord {
                id: "2".to_string(),
                content: "graph traversal".to_string(),
            },
        ];

        let scored = runtime::scored_keyword_search(&records, "tokio")
            .await
            .expect("search should succeed");
        assert_eq!(scored.len(), 1);
        assert_eq!(scored[0].id, "1");
    }
}

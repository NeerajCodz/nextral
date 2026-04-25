pub mod contracts;
pub mod graph;
pub mod ingestion;
pub mod memory;
pub mod prospective;
pub mod retrieval;
pub mod runtime;
pub mod scoring;
pub mod store;

pub use contracts::{CoreError, CoreResult};

#[cfg(test)]
mod tests {
    use crate::{
        ingestion::{ingest_memory, IngestMemoryRequest, IngestStatus},
        memory::{ContentType, MemoryRecord, MemoryStatus, MemoryType, PrivacyLevel, SourceType},
        prospective::{ReminderRecord, ReminderStatus},
        retrieval::{retrieve, RetrievalRequest, SourcePath},
        runtime,
        store::{LocalMemoryStore, MemoryIndexStore, ReminderStore},
    };

    #[tokio::test]
    async fn scored_search_returns_matches() {
        let records = vec![
            MemoryRecord::new(
                "1",
                "usr_1",
                "tokio runtime boundary",
                ContentType::Note,
                MemoryType::Semantic,
                SourceType::Manual,
            ),
            MemoryRecord::new(
                "2",
                "usr_1",
                "graph traversal",
                ContentType::Note,
                MemoryType::Semantic,
                SourceType::Manual,
            ),
        ];

        let scored = runtime::scored_keyword_search(&records, "tokio")
            .await
            .expect("search should succeed");
        assert_eq!(scored.len(), 1);
        assert_eq!(scored[0].id, "1");
    }

    #[test]
    fn memory_contract_validation_and_transition_work() {
        let mut record = MemoryRecord::new(
            "mem_1",
            "usr_1",
            "Use PostgreSQL for Atlas",
            ContentType::Decision,
            MemoryType::Semantic,
            SourceType::Realtime,
        );
        assert!(record.validate().is_ok());
        let transition = record
            .transition(MemoryStatus::SoftDeleted, "usr_1", "forget requested")
            .expect("transition should be valid");
        assert_eq!(transition.from, MemoryStatus::Active);
        assert_eq!(record.status, MemoryStatus::SoftDeleted);
        assert!(record
            .transition(MemoryStatus::Archived, "usr_1", "bad")
            .is_err());
    }

    #[test]
    fn local_store_filters_by_user_privacy_and_status() {
        let mut store = LocalMemoryStore::new();
        let mut private = MemoryRecord::new(
            "mem_1",
            "usr_1",
            "private fact",
            ContentType::Fact,
            MemoryType::Semantic,
            SourceType::Manual,
        );
        private.privacy_level = PrivacyLevel::Private;
        store.upsert_memory(private).unwrap();
        let mut shared = MemoryRecord::new(
            "mem_2",
            "usr_2",
            "other user fact",
            ContentType::Fact,
            MemoryType::Semantic,
            SourceType::Manual,
        );
        shared.privacy_level = PrivacyLevel::Shared;
        store.upsert_memory(shared).unwrap();
        let records = store
            .list_memories("usr_1", &[PrivacyLevel::Private], false)
            .unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].id, "mem_1");
    }

    #[test]
    fn ingestion_retrieval_graph_and_reminders_work() {
        let mut store = LocalMemoryStore::new();
        let mut request = IngestMemoryRequest::semantic_fact(
            "usr_1",
            "Rajan leads backend for Project Atlas using PostgreSQL",
        );
        request.entities = vec!["Project Atlas".to_string(), "PostgreSQL".to_string()];
        request.importance_score = 0.9;

        let response = ingest_memory(&mut store, request).unwrap();
        assert_eq!(response.status, IngestStatus::Accepted);

        let retrieval = retrieve(
            &mut store,
            RetrievalRequest::local("usr_1", "PostgreSQL"),
        )
        .unwrap();
        assert_eq!(retrieval.items.len(), 1);
        assert!(matches!(
            retrieval.items[0].source_path,
            SourcePath::Vector | SourcePath::Both
        ));

        let memory_id = response.record_id.unwrap();
        let mut reminder = ReminderRecord::new(
            "usr_1",
            memory_id,
            crate::prospective::ReminderKind::FollowUp,
            "Check migration",
            "9999999999",
            "Asia/Kolkata",
        )
        .unwrap();
        reminder
            .transition(ReminderStatus::Due, "system", "window reached")
            .unwrap();
        store.upsert_reminder(reminder).unwrap();
        assert_eq!(store.list_due_reminders("usr_1", "9999999999").unwrap().len(), 1);
    }
}

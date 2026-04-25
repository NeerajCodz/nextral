pub mod adapters;
pub mod config;
pub mod contracts;
pub mod graph;
pub mod ingestion;
pub mod memory;
pub mod providers;
pub mod prospective;
pub mod retrieval;
pub mod runtime;
pub mod scoring;
pub mod store;

pub use contracts::{CoreError, CoreResult};

#[cfg(test)]
mod tests {
    use crate::{
        config::{
            AuthConfig, CacheConfig, EmbeddingProviderConfig, EmbeddingProviderKind,
            ExtractionProviderConfig, ExtractionProviderKind, IngestionPolicy, NextralConfig,
            ObservabilityConfig, RetrievalPolicy, RuntimeBackend, ScoringWeights, ServiceConfig,
            StoreConfig,
        },
        ingestion::{ingest_memory, IngestMemoryRequest, IngestStatus},
        memory::{ContentType, MemoryRecord, MemoryStatus, MemoryType, PrivacyLevel, SourceType},
        prospective::{ReminderRecord, ReminderStatus},
        retrieval::{retrieve, RetrievalRequest, SourcePath},
        runtime,
        store::{MemoryIndexStore, ReminderStore, TestMemoryStore},
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
        let mut store = TestMemoryStore::new();
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
        let mut store = TestMemoryStore::new();
        let mut request = IngestMemoryRequest::new(
            "usr_1",
            "Rajan leads backend for Project Atlas using PostgreSQL",
            ContentType::Fact,
            MemoryType::Semantic,
            SourceType::Manual,
            IngestionPolicy {
                min_importance_score: 0.2,
                min_confidence_score: 0.2,
            },
        );
        request.entities = vec!["Project Atlas".to_string(), "PostgreSQL".to_string()];
        request.importance_score = 0.9;
        request.confidence_score = Some(0.8);

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

    #[test]
    fn production_config_requires_all_store_and_provider_settings() {
        let config = NextralConfig {
            backend: RuntimeBackend::ProductionStores,
            stores: Some(StoreConfig {
                postgres_url: "postgres://nextral".to_string(),
                redis_url: "redis://nextral".to_string(),
                qdrant_url: "http://qdrant:6334".to_string(),
                neo4j_url: "neo4j://neo4j:7687".to_string(),
                s3_endpoint: "http://minio:9000".to_string(),
                s3_bucket: "nextral".to_string(),
                s3_region: "us-east-1".to_string(),
                s3_access_key_env: "NEXTRAL_S3_ACCESS_KEY".to_string(),
                s3_secret_key_env: "NEXTRAL_S3_SECRET_KEY".to_string(),
            }),
            embedding: EmbeddingProviderConfig {
                kind: EmbeddingProviderKind::OpenAiCompatible,
                model: "configured-by-user".to_string(),
                dimension: 1536,
                endpoint: Some("https://example.invalid/v1/embeddings".to_string()),
                api_key_env: Some("NEXTRAL_EMBEDDING_API_KEY".to_string()),
            },
            extraction: ExtractionProviderConfig {
                kind: ExtractionProviderKind::Http,
                model: "configured-by-user".to_string(),
                endpoint: Some("https://example.invalid/extract".to_string()),
                api_key_env: Some("NEXTRAL_EXTRACTION_API_KEY".to_string()),
            },
            ingestion_policy: IngestionPolicy {
                min_importance_score: 0.4,
                min_confidence_score: 0.6,
            },
            retrieval_policy: RetrievalPolicy {
                privacy_scope: vec![PrivacyLevel::Private],
                token_budget: 1200,
                top_k_vector: 12,
                max_graph_hops: 2,
                scoring_weights: ScoringWeights {
                    semantic_similarity: 0.5,
                    recency: 0.2,
                    importance: 0.2,
                    access: 0.1,
                },
            },
            cache: CacheConfig {
                key_prefix: "nextral".to_string(),
                session_ttl_seconds: 7200,
                retrieval_ttl_seconds: 120,
                policy_ttl_seconds: 300,
            },
            service: ServiceConfig {
                http_bind: Some("127.0.0.1:8080".to_string()),
                grpc_bind: None,
                graphql_bind: None,
            },
            auth: AuthConfig {
                issuer: None,
                audience: None,
                jwks_url: None,
                service_token_env: Some("NEXTRAL_SERVICE_TOKEN".to_string()),
            },
            observability: ObservabilityConfig {
                enabled: false,
                otlp_endpoint: None,
                prometheus_bind: None,
            },
        };

        assert!(config.validate().is_ok());
    }
}

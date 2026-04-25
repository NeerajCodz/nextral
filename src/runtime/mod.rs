use crate::{
    contracts::CoreResult,
    ingestion::{ingest_memory, IngestMemoryRequest, IngestMemoryResponse},
    memory::MemoryRecord,
    retrieval::{self, RetrievalRequest, RetrievalResponse},
    scoring::{self, ScoredRecord},
    store::LocalMemoryStore,
};

#[derive(Debug, Clone, Default)]
pub struct LocalRuntime {
    pub store: LocalMemoryStore,
}

impl LocalRuntime {
    pub fn new() -> Self {
        Self {
            store: LocalMemoryStore::new(),
        }
    }

    pub fn ingest(&mut self, request: IngestMemoryRequest) -> CoreResult<IngestMemoryResponse> {
        ingest_memory(&mut self.store, request)
    }

    pub fn retrieve(&mut self, request: RetrievalRequest) -> CoreResult<RetrievalResponse> {
        retrieval::retrieve(&mut self.store, request)
    }
}

pub async fn scored_keyword_search(
    records: &[MemoryRecord],
    query: &str,
) -> CoreResult<Vec<ScoredRecord>> {
    tokio::task::yield_now().await;
    let matches = retrieval::keyword_search(records, query)?;
    scoring::rank_records(&matches, query)
}

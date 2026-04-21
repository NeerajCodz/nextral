use crate::{
    contracts::CoreResult,
    memory::MemoryRecord,
    retrieval,
    scoring::{self, ScoredRecord},
};

pub async fn scored_keyword_search(
    records: &[MemoryRecord],
    query: &str,
) -> CoreResult<Vec<ScoredRecord>> {
    tokio::task::yield_now().await;
    let matches = retrieval::keyword_search(records, query)?;
    scoring::rank_records(&matches, query)
}

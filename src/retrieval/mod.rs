use crate::{
    contracts::{CoreError, CoreResult},
    memory::MemoryRecord,
};

pub fn keyword_search(records: &[MemoryRecord], query: &str) -> CoreResult<Vec<MemoryRecord>> {
    let normalized = query.trim().to_lowercase();
    if normalized.is_empty() {
        return Err(CoreError::InvalidInput("query cannot be empty".to_string()));
    }

    Ok(records
        .iter()
        .filter(|record| record.content.to_lowercase().contains(&normalized))
        .cloned()
        .collect())
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryRecord {
    pub id: String,
    pub content: String,
}

pub fn upsert(records: &mut Vec<MemoryRecord>, record: MemoryRecord) {
    if let Some(existing) = records
        .iter_mut()
        .find(|candidate| candidate.id == record.id)
    {
        *existing = record;
        return;
    }

    records.push(record);
}

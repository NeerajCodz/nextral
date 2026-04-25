use crate::{
    contracts::{CoreError, CoreResult},
    graph::{GraphEdge, GraphNode},
    memory::{MemoryRecord, MemoryStatus, PrivacyLevel},
    prospective::ReminderRecord,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    WriteAttempt,
    WriteAccepted,
    WriteRejected,
    ValidationFailed,
    Transition,
    ReminderScheduled,
    ReminderTransition,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditEvent {
    pub id: String,
    pub actor: String,
    pub action: AuditAction,
    pub target_id: Option<String>,
    pub reason: String,
    pub created_at: String,
}

pub trait MemoryIndexStore {
    fn upsert_memory(&mut self, record: MemoryRecord) -> CoreResult<()>;
    fn get_memory(&self, user_id: &str, id: &str) -> CoreResult<Option<MemoryRecord>>;
    fn list_memories(
        &self,
        user_id: &str,
        privacy_scope: &[PrivacyLevel],
        include_inactive: bool,
    ) -> CoreResult<Vec<MemoryRecord>>;
    fn update_memory(&mut self, record: MemoryRecord) -> CoreResult<()>;
}

pub trait GraphStore {
    fn merge_node(&mut self, node: GraphNode) -> CoreResult<()>;
    fn merge_edge(&mut self, edge: GraphEdge) -> CoreResult<()>;
    fn graph_memory_ids(&self, user_id: &str, query: &str, max_hops: u8)
        -> CoreResult<Vec<String>>;
}

pub trait AuditSink {
    fn emit_audit(&mut self, event: AuditEvent) -> CoreResult<()>;
}

pub trait ReminderStore {
    fn upsert_reminder(&mut self, reminder: ReminderRecord) -> CoreResult<()>;
    fn list_due_reminders(
        &self,
        user_id: &str,
        due_at_or_before: &str,
    ) -> CoreResult<Vec<ReminderRecord>>;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestMemoryStore {
    pub memories: Vec<MemoryRecord>,
    pub graph_nodes: Vec<GraphNode>,
    pub graph_edges: Vec<GraphEdge>,
    pub reminders: Vec<ReminderRecord>,
    pub audit_events: Vec<AuditEvent>,
}

impl TestMemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MemoryIndexStore for TestMemoryStore {
    fn upsert_memory(&mut self, record: MemoryRecord) -> CoreResult<()> {
        record.validate()?;
        if let Some(existing) = self
            .memories
            .iter_mut()
            .find(|memory| memory.id == record.id)
        {
            *existing = record;
            return Ok(());
        }
        self.memories.push(record);
        Ok(())
    }

    fn get_memory(&self, user_id: &str, id: &str) -> CoreResult<Option<MemoryRecord>> {
        Ok(self
            .memories
            .iter()
            .find(|memory| memory.user_id == user_id && memory.id == id)
            .cloned())
    }

    fn list_memories(
        &self,
        user_id: &str,
        privacy_scope: &[PrivacyLevel],
        include_inactive: bool,
    ) -> CoreResult<Vec<MemoryRecord>> {
        Ok(self
            .memories
            .iter()
            .filter(|memory| memory.user_id == user_id)
            .filter(|memory| privacy_scope.contains(&memory.privacy_level))
            .filter(|memory| include_inactive || memory.status == MemoryStatus::Active)
            .cloned()
            .collect())
    }

    fn update_memory(&mut self, record: MemoryRecord) -> CoreResult<()> {
        if let Some(existing) = self
            .memories
            .iter_mut()
            .find(|memory| memory.user_id == record.user_id && memory.id == record.id)
        {
            *existing = record;
            return Ok(());
        }
        Err(CoreError::NotFound("memory record not found".to_string()))
    }
}

impl GraphStore for TestMemoryStore {
    fn merge_node(&mut self, node: GraphNode) -> CoreResult<()> {
        if !self.graph_nodes.iter().any(|existing| {
            existing.user_id == node.user_id
                && existing.label == node.label
                && existing.canonical_name == node.canonical_name
        }) {
            self.graph_nodes.push(node);
        }
        Ok(())
    }

    fn merge_edge(&mut self, edge: GraphEdge) -> CoreResult<()> {
        if let Some(existing) = self.graph_edges.iter_mut().find(|existing| {
            existing.user_id == edge.user_id
                && existing.from_key == edge.from_key
                && existing.relationship_type == edge.relationship_type
                && existing.to_key == edge.to_key
        }) {
            existing.last_confirmed_at = edge.last_confirmed_at;
            existing.confidence = existing.confidence.max(edge.confidence);
            if !existing
                .source_memory_ids
                .contains(&edge.source_memory_ids[0])
            {
                existing
                    .source_memory_ids
                    .extend(edge.source_memory_ids.iter().cloned());
            }
            return Ok(());
        }
        self.graph_edges.push(edge);
        Ok(())
    }

    fn graph_memory_ids(
        &self,
        user_id: &str,
        query: &str,
        _max_hops: u8,
    ) -> CoreResult<Vec<String>> {
        let normalized = query.trim().to_lowercase();
        if normalized.is_empty() {
            return Ok(Vec::new());
        }
        let node_keys: Vec<String> = self
            .graph_nodes
            .iter()
            .filter(|node| {
                node.user_id == user_id
                    && (node.name.to_lowercase().contains(&normalized)
                        || normalized.contains(&node.name.to_lowercase()))
            })
            .map(|node| node.key.clone())
            .collect();
        let mut ids = Vec::new();
        for edge in self
            .graph_edges
            .iter()
            .filter(|edge| edge.user_id == user_id)
        {
            if node_keys.contains(&edge.from_key) || node_keys.contains(&edge.to_key) {
                ids.extend(edge.source_memory_ids.iter().cloned());
            }
        }
        ids.sort();
        ids.dedup();
        Ok(ids)
    }
}

impl AuditSink for TestMemoryStore {
    fn emit_audit(&mut self, event: AuditEvent) -> CoreResult<()> {
        self.audit_events.push(event);
        Ok(())
    }
}

impl ReminderStore for TestMemoryStore {
    fn upsert_reminder(&mut self, reminder: ReminderRecord) -> CoreResult<()> {
        if self.reminders.iter().any(|existing| {
            existing.id != reminder.id && existing.dedupe_key == reminder.dedupe_key
        }) {
            return Err(CoreError::Conflict(
                "duplicate reminder dedupe key".to_string(),
            ));
        }
        if let Some(existing) = self
            .reminders
            .iter_mut()
            .find(|existing| existing.id == reminder.id)
        {
            *existing = reminder;
            return Ok(());
        }
        self.reminders.push(reminder);
        Ok(())
    }

    fn list_due_reminders(
        &self,
        user_id: &str,
        due_at_or_before: &str,
    ) -> CoreResult<Vec<ReminderRecord>> {
        Ok(self
            .reminders
            .iter()
            .filter(|reminder| reminder.user_id == user_id)
            .filter(|reminder| {
                reminder
                    .next_attempt_at
                    .as_deref()
                    .unwrap_or(&reminder.due_at)
                    <= due_at_or_before
            })
            .filter(|reminder| reminder.is_due_visible())
            .cloned()
            .collect())
    }
}

use crate::{
    contracts::{CoreError, CoreResult},
    ingestion::{ingest_memory, IngestMemoryRequest},
    memory::{MemoryStatus, PrivacyLevel},
    prospective::{ReminderKind, ReminderRecord},
    retrieval::{retrieve, RetrievalRequest},
    store::{AuditAction, AuditEvent, AuditSink, LocalMemoryStore, MemoryIndexStore, ReminderStore},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForgetRequest {
    pub user_id: String,
    pub memory_id: String,
    pub actor: String,
    pub reason: String,
    #[serde(default)]
    pub redact: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReminderScheduleRequest {
    pub user_id: String,
    pub source_memory_id: String,
    pub kind: ReminderKind,
    pub title: String,
    #[serde(default)]
    pub details: String,
    pub due_at: String,
    pub timezone: String,
}

pub fn ingest_file_json(path: impl Into<PathBuf>, request_json: &str) -> CoreResult<String> {
    let path = path.into();
    let request: IngestMemoryRequest = serde_json::from_str(request_json)?;
    let mut store = LocalMemoryStore::load(&path)?;
    let response = ingest_memory(&mut store, request)?;
    store.save(&path)?;
    Ok(serde_json::to_string_pretty(&response)?)
}

pub fn retrieve_file_json(path: impl Into<PathBuf>, request_json: &str) -> CoreResult<String> {
    let path = path.into();
    let mut request: RetrievalRequest = serde_json::from_str(request_json)?;
    if request.privacy_scope.is_empty() {
        request.privacy_scope = vec![PrivacyLevel::Private, PrivacyLevel::Sensitive, PrivacyLevel::Shared];
    }
    let mut store = LocalMemoryStore::load(&path)?;
    let response = retrieve(&mut store, request)?;
    store.save(&path)?;
    Ok(serde_json::to_string_pretty(&response)?)
}

pub fn forget_file_json(path: impl Into<PathBuf>, request_json: &str) -> CoreResult<String> {
    let path = path.into();
    let request: ForgetRequest = serde_json::from_str(request_json)?;
    let mut store = LocalMemoryStore::load(&path)?;
    let mut record = store
        .get_memory(&request.user_id, &request.memory_id)?
        .ok_or_else(|| CoreError::NotFound("memory record not found".to_string()))?;
    let first = record.transition(
        MemoryStatus::SoftDeleted,
        request.actor.clone(),
        request.reason.clone(),
    )?;
    let second = if request.redact {
        Some(record.transition(
            MemoryStatus::Redacted,
            request.actor.clone(),
            request.reason.clone(),
        )?)
    } else {
        None
    };
    store.update_memory(record)?;
    store.emit_audit(AuditEvent {
        id: crate::memory::deterministic_id(&[
            &request.user_id,
            &request.memory_id,
            &crate::memory::now_timestamp(),
            "forget",
        ]),
        actor: request.actor,
        action: AuditAction::Transition,
        target_id: Some(request.memory_id),
        reason: request.reason,
        created_at: crate::memory::now_timestamp(),
    })?;
    store.save(&path)?;
    Ok(serde_json::to_string_pretty(&(first, second))?)
}

pub fn schedule_reminder_file_json(path: impl Into<PathBuf>, request_json: &str) -> CoreResult<String> {
    let path = path.into();
    let request: ReminderScheduleRequest = serde_json::from_str(request_json)?;
    let mut store = LocalMemoryStore::load(&path)?;
    let mut reminder = ReminderRecord::new(
        request.user_id,
        request.source_memory_id,
        request.kind,
        request.title,
        request.due_at,
        request.timezone,
    )?;
    reminder.details = request.details;
    store.upsert_reminder(reminder.clone())?;
    store.emit_audit(AuditEvent {
        id: crate::memory::deterministic_id(&[&reminder.user_id, &reminder.id, "reminder_scheduled"]),
        actor: reminder.user_id.clone(),
        action: AuditAction::ReminderScheduled,
        target_id: Some(reminder.id.clone()),
        reason: "reminder scheduled".to_string(),
        created_at: crate::memory::now_timestamp(),
    })?;
    store.save(&path)?;
    Ok(serde_json::to_string_pretty(&reminder)?)
}

pub fn due_reminders_file_json(path: impl Into<PathBuf>, user_id: &str, due_at_or_before: &str) -> CoreResult<String> {
    let store = LocalMemoryStore::load(path)?;
    let reminders = store.list_due_reminders(user_id, due_at_or_before)?;
    Ok(serde_json::to_string_pretty(&reminders)?)
}

pub fn status_file_json(path: impl Into<PathBuf>) -> CoreResult<String> {
    let store = LocalMemoryStore::load(path)?;
    #[derive(Serialize)]
    struct Status {
        memories: usize,
        graph_nodes: usize,
        graph_edges: usize,
        reminders: usize,
        audit_events: usize,
    }
    Ok(serde_json::to_string_pretty(&Status {
        memories: store.memories.len(),
        graph_nodes: store.graph_nodes.len(),
        graph_edges: store.graph_edges.len(),
        reminders: store.reminders.len(),
        audit_events: store.audit_events.len(),
    })?)
}

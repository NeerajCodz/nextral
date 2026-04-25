use crate::contracts::{CoreError, CoreResult};
use crate::{
    adapters::{postgres::PostgresAdapter, redis::RedisAdapter},
    config::IngestionPolicy,
    ingestion::IngestMemoryRequest,
    memory::{ContentType, MemoryRecord, MemoryType, SourceType},
    ports::{AuditWrite, CacheEntry, PostgresPort, RedisPort, TenantUserScope},
    retrieval::RetrievalRequest,
    runtime::{
        consolidation::{consolidate_session, ConsolidationLane, ConsolidationRequest},
        governance::{forget_memory, ForgetMemoryRequest},
        reembed::plan_reembed,
        reminders::{schedule_reminder, ScheduleReminderRequest},
        session::{append_session_message, assemble_working_context, AppendSessionMessageRequest},
    },
    store::TestMemoryStore,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterSmokeRequest {
    pub postgres_url: String,
    pub redis_url: String,
    pub redis_key_prefix: String,
    pub tenant_id: String,
    pub user_id: String,
    pub session_id: String,
    pub memory_id: String,
    pub memory_content: String,
    pub embedding_provider: String,
    pub embedding_model: String,
    pub embedding_dim: u32,
    pub reminder_title: String,
    pub reminder_due_at: String,
    pub timezone: String,
    pub cache_key: String,
    pub cache_value_json: String,
    pub cache_ttl_seconds: u64,
}

pub fn e2e_smoke_json() -> Result<String, PackageError> {
    let mut store = TestMemoryStore::new();
    let appended = append_session_message(
        &mut store,
        AppendSessionMessageRequest {
            tenant_id: "test_tenant".to_string(),
            user_id: "test_user".to_string(),
            session_id: "test_session".to_string(),
            role: "user".to_string(),
            content: "Use PostgreSQL for Atlas and remind me Friday".to_string(),
            idempotency_key: Some("testkit-message".to_string()),
        },
        20,
    )?;
    let consolidated = consolidate_session(
        &mut store,
        ConsolidationRequest {
            tenant_id: "test_tenant".to_string(),
            user_id: "test_user".to_string(),
            session_id: "test_session".to_string(),
            lane: ConsolidationLane::Fast,
            policy: IngestionPolicy {
                min_importance_score: 0.1,
                min_confidence_score: 0.1,
            },
        },
    )?;
    let memory_id = consolidated.accepted[0]
        .record_id
        .clone()
        .unwrap_or_default();
    let scheduled = schedule_reminder(
        &mut store,
        ScheduleReminderRequest {
            tenant_id: "test_tenant".to_string(),
            user_id: "test_user".to_string(),
            source_memory_id: memory_id.clone(),
            kind: crate::prospective::ReminderKind::FollowUp,
            title: "Check Atlas migration".to_string(),
            due_at: "9999999999".to_string(),
            timezone: "configured-by-user".to_string(),
        },
    )?;
    let mut retrieval = RetrievalRequest::test("test_tenant", "test_user", "PostgreSQL");
    retrieval.session_id = Some("test_session".to_string());
    let context = assemble_working_context(&mut store, retrieval, 20)?;
    let forgotten = forget_memory(
        &mut store,
        ForgetMemoryRequest {
            tenant_id: "test_tenant".to_string(),
            user_id: "test_user".to_string(),
            memory_id,
            actor: "test_user".to_string(),
            reason: "testkit redaction".to_string(),
            redact: true,
        },
    )?;
    let payload = serde_json::json!({
        "status": "ok",
        "backend": "testkit",
        "message_id": appended.message_id,
        "consolidation_job": consolidated.job.id,
        "reminder_id": scheduled.reminder.id,
        "working_context_items": context.retrieved_memory_ids.len(),
        "redacted": forgotten.redaction_transition.is_some()
    });
    Ok(payload.to_string())
}

pub fn adapter_smoke_json(request_json: &str) -> Result<String, PackageError> {
    let request: AdapterSmokeRequest =
        serde_json::from_str(request_json).map_err(CoreError::from)?;
    validate_adapter_smoke_request(&request)?;

    let postgres = PostgresAdapter::new(&request.postgres_url)?;
    let redis = RedisAdapter::new(&request.redis_url, &request.redis_key_prefix)?;
    postgres.migrate()?;

    let mut record = MemoryRecord::new(
        &request.memory_id,
        &request.tenant_id,
        &request.user_id,
        &request.memory_content,
        ContentType::Fact,
        MemoryType::Semantic,
        SourceType::Manual,
    );
    record.session_id = Some(request.session_id.clone());
    record.embedding_provider = Some(request.embedding_provider.clone());
    record.embedding_model = Some(request.embedding_model.clone());
    record.embedding_dim = Some(request.embedding_dim);
    record.entities = vec![request.memory_id.clone()];
    postgres.upsert_memory(&record)?;

    let scope = TenantUserScope {
        tenant_id: request.tenant_id.clone(),
        user_id: request.user_id.clone(),
    };
    let loaded = postgres
        .get_memory(&scope, &request.memory_id)?
        .ok_or_else(|| CoreError::NotFound("adapter smoke memory was not loaded".to_string()))?;
    let message_id = postgres.append_session_message(
        &scope,
        &request.session_id,
        "user",
        &request.memory_content,
    )?;
    let reminder = crate::prospective::ReminderRecord::new(
        &request.tenant_id,
        &request.user_id,
        &request.memory_id,
        crate::prospective::ReminderKind::FollowUp,
        &request.reminder_title,
        &request.reminder_due_at,
        &request.timezone,
    )?;
    postgres.upsert_reminder(&reminder)?;
    postgres.write_audit(&AuditWrite {
        tenant_id: request.tenant_id.clone(),
        user_id: Some(request.user_id.clone()),
        actor_id: request.user_id.clone(),
        action: "adapter_smoke".to_string(),
        target_type: "memory".to_string(),
        target_id: Some(request.memory_id.clone()),
        reason: "adapter smoke verification".to_string(),
        request_id: None,
        trace_id: None,
        metadata_json: "{}".to_string(),
    })?;
    let outbox_id = postgres.enqueue_outbox(
        &request.tenant_id,
        "adapter_smoke.completed",
        &request.memory_id,
        "{}",
    )?;

    redis.put_cache(&CacheEntry {
        key: request.cache_key.clone(),
        value_json: request.cache_value_json.clone(),
        ttl_seconds: request.cache_ttl_seconds,
    })?;
    let cached = redis.get_cache(&request.cache_key)?.ok_or_else(|| {
        CoreError::NotFound("adapter smoke cache value was not loaded".to_string())
    })?;
    let lease_key = format!("lease:{}", request.cache_key);
    let lease_owner = format!("{}:{}", request.tenant_id, request.user_id);
    let lease_acquired =
        redis.acquire_lease(&lease_key, &lease_owner, request.cache_ttl_seconds)?;
    redis.release_lease(&lease_key, &lease_owner)?;

    let payload = serde_json::json!({
        "status": "ok",
        "backend": "production_adapters",
        "memory_id": loaded.id,
        "message_id": message_id,
        "reminder_id": reminder.id,
        "outbox_id": outbox_id,
        "cache_key": request.cache_key,
        "cache_value": cached,
        "lease_acquired": lease_acquired
    });
    Ok(payload.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReembedPlanRequest {
    pub tenant_id: String,
    pub source_collection: String,
    pub shadow_collection: String,
    pub target_embedding_provider: String,
    pub target_embedding_model: String,
}

pub fn reembed_plan_json(request_json: &str) -> Result<String, PackageError> {
    let request: ReembedPlanRequest =
        serde_json::from_str(request_json).map_err(CoreError::from)?;
    let plan = plan_reembed(
        &request.tenant_id,
        &request.source_collection,
        &request.shadow_collection,
        &request.target_embedding_provider,
        &request.target_embedding_model,
    )?;
    Ok(serde_json::to_string(&plan).map_err(CoreError::from)?)
}

pub fn ingest_request_schema_json() -> String {
    let request = IngestMemoryRequest::new(
        "tenant_id",
        "user_id",
        "content",
        ContentType::Fact,
        MemoryType::Semantic,
        SourceType::Manual,
        IngestionPolicy {
            min_importance_score: 0.0,
            min_confidence_score: 0.0,
        },
    );
    serde_json::to_string(&request).unwrap_or_else(|_| "{}".to_string())
}

fn validate_adapter_smoke_request(request: &AdapterSmokeRequest) -> CoreResult<()> {
    let required = [
        ("postgres_url", request.postgres_url.as_str()),
        ("redis_url", request.redis_url.as_str()),
        ("redis_key_prefix", request.redis_key_prefix.as_str()),
        ("tenant_id", request.tenant_id.as_str()),
        ("user_id", request.user_id.as_str()),
        ("session_id", request.session_id.as_str()),
        ("memory_id", request.memory_id.as_str()),
        ("memory_content", request.memory_content.as_str()),
        ("embedding_provider", request.embedding_provider.as_str()),
        ("embedding_model", request.embedding_model.as_str()),
        ("reminder_title", request.reminder_title.as_str()),
        ("reminder_due_at", request.reminder_due_at.as_str()),
        ("timezone", request.timezone.as_str()),
        ("cache_key", request.cache_key.as_str()),
        ("cache_value_json", request.cache_value_json.as_str()),
    ];
    for (name, value) in required {
        if value.trim().is_empty() {
            return Err(CoreError::InvalidInput(format!("{name} is required")));
        }
    }
    if request.embedding_dim == 0 {
        return Err(CoreError::InvalidInput(
            "embedding_dim must be greater than zero".to_string(),
        ));
    }
    if request.cache_ttl_seconds == 0 {
        return Err(CoreError::InvalidInput(
            "cache_ttl_seconds must be greater than zero".to_string(),
        ));
    }
    serde_json::from_str::<serde_json::Value>(&request.cache_value_json)
        .map_err(CoreError::from)?;
    Ok(())
}

impl From<CoreError> for PackageError {
    fn from(error: CoreError) -> Self {
        let code = match &error {
            CoreError::InvalidInput(_) => "invalid_input",
            CoreError::NotFound(_) => "not_found",
            CoreError::Conflict(_) => "conflict",
            CoreError::Io(_) => "io",
            CoreError::Serialization(_) => "serialization",
        };
        Self {
            code: code.to_string(),
            message: error.to_string(),
        }
    }
}

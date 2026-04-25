use crate::contracts::CoreResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbeddingBatch {
    pub model: String,
    pub dimension: u32,
    pub vectors: Vec<Vec<f32>>,
}

pub trait EmbeddingProvider {
    fn embed(&self, request: EmbeddingRequest) -> CoreResult<EmbeddingBatch>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExtractionRequest {
    pub model: String,
    pub content: String,
    pub entities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExtractedRelationship {
    pub from_label: String,
    pub from_name: String,
    pub relationship_type: String,
    pub to_label: String,
    pub to_name: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExtractionOutput {
    pub entities: Vec<String>,
    pub relationships: Vec<ExtractedRelationship>,
}

pub trait ExtractionProvider {
    fn extract(&self, request: ExtractionRequest) -> CoreResult<ExtractionOutput>;
}

pub trait TokenEstimator {
    fn estimate_tokens(&self, text: &str) -> CoreResult<u32>;
}

pub trait IdGenerator {
    fn memory_id(&self, tenant_id: &str, user_id: &str, content: &str) -> CoreResult<String>;
}

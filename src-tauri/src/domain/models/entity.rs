use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Entity types in the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Person,
    Place,
    Organization,
    Concept,
    Topic,
}

/// An entity (person, place, concept) extracted from documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub entity_type: EntityType,
    pub first_seen: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    pub mention_count: u32,
    pub aliases: Vec<String>,
    pub metadata: serde_json::Value,
}

/// A relationship between two entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub rel_type: String,
    pub weight: f64,
    pub first_seen: Option<DateTime<Utc>>,
    pub context_chunks: Vec<String>,
}

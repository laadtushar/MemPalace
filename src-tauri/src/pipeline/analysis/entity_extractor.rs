use chrono::Utc;
use uuid::Uuid;

use crate::domain::models::entity::{Entity, EntityType};
use crate::domain::ports::llm_provider::{ILlmProvider, LlmParams};
use crate::error::AppError;
use crate::prompts::templates::{render_template, ENTITY_EXTRACTION_V1};

/// Extract entities from a batch of text chunks using the LLM.
pub async fn extract_entities(
    chunks: &[(String, String)], // (chunk_id, text)
    llm: &dyn ILlmProvider,
    max_chunks: usize,
) -> Result<Vec<(Entity, Vec<String>)>, AppError> {
    let mut entities_with_sources: Vec<(Entity, Vec<String>)> = Vec::new();
    let mut seen_names: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for (chunk_id, text) in chunks.iter().take(max_chunks) {
        let prompt = render_template(ENTITY_EXTRACTION_V1, &[("text", text.as_str())]);

        let params = LlmParams {
            temperature: Some(0.2),
            max_tokens: Some(500),
            ..Default::default()
        };

        let response = match llm.complete(&prompt, &params).await {
            Ok(r) => r,
            Err(e) => {
                log::warn!("Entity extraction failed for chunk {}: {}", chunk_id, e);
                continue;
            }
        };

        let parsed = parse_entity_response(&response);
        for entry in parsed {
            let name_key = entry.name.to_lowercase();
            if let Some(&idx) = seen_names.get(&name_key) {
                // Update existing entity: bump mention_count and add source chunk
                entities_with_sources[idx].0.mention_count += 1;
                entities_with_sources[idx].0.last_seen = Some(Utc::now());
                if !entities_with_sources[idx].1.contains(chunk_id) {
                    entities_with_sources[idx].1.push(chunk_id.clone());
                }
            } else {
                let entity = Entity {
                    id: Uuid::new_v4().to_string(),
                    name: entry.name.clone(),
                    entity_type: parse_entity_type(&entry.entity_type),
                    first_seen: Some(Utc::now()),
                    last_seen: Some(Utc::now()),
                    mention_count: 1,
                    aliases: vec![],
                    metadata: serde_json::json!({}),
                };
                seen_names.insert(name_key, entities_with_sources.len());
                entities_with_sources.push((entity, vec![chunk_id.clone()]));
            }
        }
    }

    Ok(entities_with_sources)
}

fn parse_entity_type(s: &str) -> EntityType {
    match s.to_lowercase().as_str() {
        "person" => EntityType::Person,
        "place" => EntityType::Place,
        "organization" => EntityType::Organization,
        "concept" | "topic" => EntityType::Concept,
        _ => EntityType::Concept,
    }
}

#[derive(serde::Deserialize)]
struct EntityEntry {
    name: String,
    entity_type: String,
}

fn parse_entity_response(response: &str) -> Vec<EntityEntry> {
    // Extract JSON array from potentially wrapped response
    let json_str = if let Some(start) = response.find('[') {
        if let Some(end) = response.rfind(']') {
            &response[start..=end]
        } else {
            response
        }
    } else {
        response
    };

    serde_json::from_str(json_str).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entity_response() {
        let response = r#"[
            {"name": "Sarah", "entity_type": "person"},
            {"name": "London", "entity_type": "place"},
            {"name": "remote work", "entity_type": "concept"}
        ]"#;
        let entities = parse_entity_response(response);
        assert_eq!(entities.len(), 3);
        assert_eq!(entities[0].name, "Sarah");
        assert_eq!(entities[1].entity_type, "place");
    }

    #[test]
    fn test_parse_entity_type() {
        assert_eq!(parse_entity_type("person"), EntityType::Person);
        assert_eq!(parse_entity_type("Place"), EntityType::Place);
        assert_eq!(parse_entity_type("ORGANIZATION"), EntityType::Organization);
        assert_eq!(parse_entity_type("topic"), EntityType::Concept);
        assert_eq!(parse_entity_type("unknown"), EntityType::Concept);
    }
}

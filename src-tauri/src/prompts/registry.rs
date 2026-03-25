use std::collections::HashMap;

/// Tracks which prompt version produced which output, enabling reproducibility.
pub struct PromptRegistry {
    versions: HashMap<String, String>,
}

impl PromptRegistry {
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, version: &str) {
        self.versions.insert(name.to_string(), version.to_string());
    }

    pub fn get_version(&self, name: &str) -> Option<&String> {
        self.versions.get(name)
    }
}

impl Default for PromptRegistry {
    fn default() -> Self {
        let mut reg = Self::new();
        reg.register("theme_extraction", "v1");
        reg.register("sentiment", "v1");
        reg.register("belief_extraction", "v1");
        reg.register("entity_extraction", "v1");
        reg.register("insight_generation", "v1");
        reg.register("query_classification", "v1");
        reg.register("rag_response", "v1");
        reg
    }
}

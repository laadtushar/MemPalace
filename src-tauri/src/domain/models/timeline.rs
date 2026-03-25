use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single data point on the timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelinePoint {
    pub timestamp: DateTime<Utc>,
    pub theme_label: String,
    pub intensity: f64,
    pub sentiment: Option<f64>,
    pub document_count: usize,
}

/// Timeline data for a given range, used by the frontend visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineData {
    pub points: Vec<TimelinePoint>,
    pub themes: Vec<String>,
    pub range_start: DateTime<Utc>,
    pub range_end: DateTime<Utc>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A snapshot of a theme within a time window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSnapshot {
    pub id: String,
    pub theme_label: String,
    pub description: Option<String>,
    pub time_window_start: DateTime<Utc>,
    pub time_window_end: DateTime<Utc>,
    pub intensity_score: f64,
    pub representative_chunks: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Tracks how a theme/belief evolved across time windows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionArc {
    pub id: String,
    pub subject: String,
    pub snapshots: Vec<String>,
    pub trend_direction: TrendDirection,
    pub confidence: f64,
    pub narrative_summary: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Oscillating,
}

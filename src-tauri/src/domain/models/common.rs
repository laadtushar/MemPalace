use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A time range for queries and analysis windows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Supported source platforms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourcePlatform {
    Obsidian,
    Markdown,
    DayOne,
    WhatsApp,
    Telegram,
    Twitter,
    Instagram,
    Facebook,
    Reddit,
    LinkedIn,
    GoogleTakeout,
    AppleNotes,
    Notion,
    PlainText,
    Custom,
}

impl std::fmt::Display for SourcePlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Obsidian => write!(f, "obsidian"),
            Self::Markdown => write!(f, "markdown"),
            Self::DayOne => write!(f, "dayone"),
            Self::WhatsApp => write!(f, "whatsapp"),
            Self::Telegram => write!(f, "telegram"),
            Self::Twitter => write!(f, "twitter"),
            Self::Instagram => write!(f, "instagram"),
            Self::Facebook => write!(f, "facebook"),
            Self::Reddit => write!(f, "reddit"),
            Self::LinkedIn => write!(f, "linkedin"),
            Self::GoogleTakeout => write!(f, "google_takeout"),
            Self::AppleNotes => write!(f, "apple_notes"),
            Self::Notion => write!(f, "notion"),
            Self::PlainText => write!(f, "plain_text"),
            Self::Custom => write!(f, "custom"),
        }
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub offset: usize,
    pub limit: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 50,
        }
    }
}

// TypeScript mirrors of Rust domain models

export type SourcePlatform =
  | "obsidian"
  | "markdown"
  | "dayone"
  | "whatsapp"
  | "telegram"
  | "twitter"
  | "instagram"
  | "facebook"
  | "reddit"
  | "linkedin"
  | "google_takeout"
  | "apple_notes"
  | "notion"
  | "plain_text"
  | "custom";

export interface TimeRange {
  start: string; // ISO 8601
  end: string;
}

export interface Document {
  id: string;
  source_platform: SourcePlatform;
  raw_text: string;
  timestamp: string;
  participants: string[];
  metadata: Record<string, unknown>;
  content_hash: string;
  created_at: string;
  updated_at: string;
}

export interface Chunk {
  id: string;
  document_id: string;
  text: string;
  token_count: number;
  position: number;
  created_at: string;
}

export type EntityType = "person" | "place" | "organization" | "concept" | "topic";

export interface Entity {
  id: string;
  name: string;
  entity_type: EntityType;
  first_seen: string | null;
  last_seen: string | null;
  mention_count: number;
  aliases: string[];
  metadata: Record<string, unknown>;
}

export interface Relationship {
  id: string;
  source_entity_id: string;
  target_entity_id: string;
  rel_type: string;
  weight: number;
  first_seen: string | null;
  context_chunks: string[];
}

export interface ThemeSnapshot {
  id: string;
  theme_label: string;
  description: string | null;
  time_window_start: string;
  time_window_end: string;
  intensity_score: number;
  representative_chunks: string[];
  created_at: string;
}

export type TrendDirection = "increasing" | "decreasing" | "stable" | "oscillating";

export interface EvolutionArc {
  id: string;
  subject: string;
  snapshots: string[];
  trend_direction: TrendDirection;
  confidence: number;
  narrative_summary: string | null;
  created_at: string;
}

export type FactCategory = "belief" | "preference" | "fact" | "self_description";

export interface MemoryFact {
  id: string;
  fact_text: string;
  source_chunks: string[];
  confidence: number;
  category: FactCategory;
  first_seen: string;
  last_updated: string;
  contradicted_by: string[];
  is_active: boolean;
}

export type InsightType =
  | "theme_shift"
  | "sentiment_change"
  | "belief_contradiction"
  | "new_pattern"
  | "milestone_detected";

export interface Insight {
  id: string;
  insight_type: InsightType;
  title: string;
  body: string;
  time_range_start: string | null;
  time_range_end: string | null;
  supporting_evidence: string[];
  generated_at: string;
  prompt_version: string | null;
}

export interface TimelinePoint {
  timestamp: string;
  theme_label: string;
  intensity: number;
  sentiment: number | null;
  document_count: number;
}

export interface TimelineData {
  points: TimelinePoint[];
  themes: string[];
  range_start: string;
  range_end: string;
}

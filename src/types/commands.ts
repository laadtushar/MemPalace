// Request/response types for Tauri commands

import type { Insight, MemoryFact, TimelineData, TimeRange } from "./domain";

export interface ImportSummary {
  documents_imported: number;
  chunks_created: number;
  duplicates_skipped: number;
  errors: string[];
  duration_ms: number;
}

export interface ImportProgress {
  import_id: string;
  stage: string;
  current: number;
  total: number;
  message: string;
}

export interface SearchResult {
  chunk_id: string;
  document_id: string;
  text: string;
  score: number;
  timestamp: string;
  source_platform: string;
}

export interface AnalysisStatus {
  stage: string;
  progress: number;
  message: string;
  is_complete: boolean;
  is_cancelled: boolean;
}

export interface OllamaStatus {
  connected: boolean;
  models: string[];
  version: string | null;
}

export interface AppSettings {
  ollama_url: string;
  ollama_embedding_model: string;
  ollama_llm_model: string;
  claude_api_key_set: boolean;
  encryption_enabled: boolean;
}

// Command function signatures (for reference)
export interface Commands {
  greet: (args: { name: string }) => Promise<string>;
  import_obsidian: (args: { vaultPath: string }) => Promise<ImportSummary>;
  import_markdown: (args: { dirPath: string }) => Promise<ImportSummary>;
  import_dayone: (args: { filePath: string }) => Promise<ImportSummary>;
  semantic_search: (args: { query: string; topK: number }) => Promise<SearchResult[]>;
  keyword_search: (args: { query: string; topK: number }) => Promise<SearchResult[]>;
  hybrid_search: (args: { query: string; topK: number }) => Promise<SearchResult[]>;
  get_timeline_data: (args: { range?: TimeRange }) => Promise<TimelineData>;
  get_insights: (args: { count: number }) => Promise<Insight[]>;
  refresh_insights: () => Promise<Insight[]>;
  run_analysis: () => Promise<AnalysisStatus>;
  cancel_analysis: () => Promise<void>;
  get_memory_facts: (args: {
    category?: string;
    page?: number;
  }) => Promise<MemoryFact[]>;
  get_settings: () => Promise<AppSettings>;
  update_settings: (args: { settings: AppSettings }) => Promise<void>;
  test_ollama_connection: () => Promise<OllamaStatus>;
}

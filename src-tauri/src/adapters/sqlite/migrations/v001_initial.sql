-- Migration v001: Initial schema
-- All core tables for Memory Palace

-- Documents table
CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY,
    source_platform TEXT NOT NULL,
    raw_text TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    participants TEXT NOT NULL DEFAULT '[]',
    metadata TEXT NOT NULL DEFAULT '{}',
    content_hash TEXT UNIQUE NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_documents_timestamp ON documents(timestamp);
CREATE INDEX IF NOT EXISTS idx_documents_source ON documents(source_platform);

-- Chunks table
CREATE TABLE IF NOT EXISTS chunks (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    token_count INTEGER NOT NULL,
    position INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_chunks_document ON chunks(document_id);

-- Entities table (graph nodes)
CREATE TABLE IF NOT EXISTS entities (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    first_seen TEXT,
    last_seen TEXT,
    mention_count INTEGER NOT NULL DEFAULT 0,
    aliases TEXT NOT NULL DEFAULT '[]',
    metadata TEXT NOT NULL DEFAULT '{}'
);
CREATE INDEX IF NOT EXISTS idx_entities_type ON entities(entity_type);
CREATE INDEX IF NOT EXISTS idx_entities_name ON entities(name);

-- Relationships table (graph edges)
CREATE TABLE IF NOT EXISTS relationships (
    id TEXT PRIMARY KEY,
    source_entity_id TEXT NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    target_entity_id TEXT NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    rel_type TEXT NOT NULL,
    weight REAL NOT NULL DEFAULT 1.0,
    first_seen TEXT,
    context_chunks TEXT NOT NULL DEFAULT '[]'
);
CREATE INDEX IF NOT EXISTS idx_rel_source ON relationships(source_entity_id);
CREATE INDEX IF NOT EXISTS idx_rel_target ON relationships(target_entity_id);

-- Theme snapshots
CREATE TABLE IF NOT EXISTS theme_snapshots (
    id TEXT PRIMARY KEY,
    theme_label TEXT NOT NULL,
    description TEXT,
    time_window_start TEXT NOT NULL,
    time_window_end TEXT NOT NULL,
    intensity_score REAL NOT NULL DEFAULT 0.0,
    representative_chunks TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_themes_window ON theme_snapshots(time_window_start, time_window_end);

-- Evolution arcs
CREATE TABLE IF NOT EXISTS evolution_arcs (
    id TEXT PRIMARY KEY,
    subject TEXT NOT NULL,
    snapshots TEXT NOT NULL DEFAULT '[]',
    trend_direction TEXT NOT NULL DEFAULT 'stable',
    confidence REAL NOT NULL DEFAULT 0.0,
    narrative_summary TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Memory facts (Mem0-style)
CREATE TABLE IF NOT EXISTS memory_facts (
    id TEXT PRIMARY KEY,
    fact_text TEXT NOT NULL,
    source_chunks TEXT NOT NULL DEFAULT '[]',
    confidence REAL NOT NULL DEFAULT 1.0,
    category TEXT NOT NULL,
    first_seen TEXT NOT NULL,
    last_updated TEXT NOT NULL,
    contradicted_by TEXT NOT NULL DEFAULT '[]',
    is_active INTEGER NOT NULL DEFAULT 1
);
CREATE INDEX IF NOT EXISTS idx_facts_category ON memory_facts(category);
CREATE INDEX IF NOT EXISTS idx_facts_active ON memory_facts(is_active);

-- Insights
CREATE TABLE IF NOT EXISTS insights (
    id TEXT PRIMARY KEY,
    insight_type TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    time_range_start TEXT,
    time_range_end TEXT,
    supporting_evidence TEXT NOT NULL DEFAULT '[]',
    generated_at TEXT NOT NULL,
    prompt_version TEXT
);

-- Config store
CREATE TABLE IF NOT EXISTS config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- LLM response cache
CREATE TABLE IF NOT EXISTS llm_cache (
    prompt_hash TEXT PRIMARY KEY,
    response TEXT NOT NULL,
    model TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

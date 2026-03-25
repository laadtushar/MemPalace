# Memory Palace

[![Rust](https://img.shields.io/badge/Rust-1.94-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.10-blue?logo=tauri&logoColor=white)](https://v2.tauri.app/)
[![React](https://img.shields.io/badge/React-19-61DAFB?logo=react&logoColor=white)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-6.0-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-4.0-06B6D4?logo=tailwindcss&logoColor=white)](https://tailwindcss.com/)
[![SQLite](https://img.shields.io/badge/SQLite-FTS5-003B57?logo=sqlite&logoColor=white)](https://sqlite.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-48_passing-brightgreen)](src-tauri/src/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)]()
[![Privacy](https://img.shields.io/badge/Privacy-Local_First-blueviolet)]()
[![Ollama](https://img.shields.io/badge/LLM-Ollama_Compatible-black?logo=ollama)](https://ollama.com/)
[![Claude](https://img.shields.io/badge/LLM-Claude_API-cc785c)](https://www.anthropic.com/)

> *A searchable, visual timeline of how your thinking evolved.*

**Memory Palace** is a privacy-first native desktop application that ingests your digital footprint — journal entries, chat exports, social media archives, notes — and constructs a searchable, visual timeline of how your thinking, interests, and emotional patterns have evolved over time.

It doesn't merely catalog events; it surfaces the **meta-narrative of personal growth and change**.

![Architecture](https://img.shields.io/badge/Architecture-Hexagonal_(Ports_%26_Adapters)-informational)
![Status](https://img.shields.io/badge/Status-MVP_v0.1-yellow)

---

## Why Memory Palace?

People generate vast amounts of text about themselves but have **no tool to understand the arc of who they're becoming**. Memory Palace makes the invisible pattern visible.

- **"How has my view on remote work changed?"** — Evolution queries across years of writing
- **"When did I stop mentioning anxiety?"** — Temporal pattern detection
- **"What contradicts what I believed 3 years ago?"** — Belief contradiction surfacing
- **"Show me the themes of summer 2022"** — Time-windowed topic analysis

---

## Features

### Core
- **Multi-source ingestion** — Import from Obsidian vaults, plain Markdown/text, Day One journals (WhatsApp, Telegram, Twitter in v0.2+)
- **Full-text search** — BM25-ranked keyword search via SQLite FTS5
- **Semantic search** — Vector similarity search using local embeddings (Ollama)
- **Hybrid search** — Reciprocal Rank Fusion combining keyword + semantic results
- **Theme extraction** — LLM-powered monthly topic analysis with intensity scoring
- **Belief tracking** — Extract and track beliefs, preferences, values over time
- **Insight generation** — AI-generated observations about personal evolution
- **Memory facts** — Mem0-style long-term memory with contradiction detection

### Privacy & Security
- **100% local by default** — All data stored and processed on-device
- **Zero telemetry** — No analytics, no phone-home, no cloud dependency
- **Local LLM support** — Full functionality with Ollama (Llama 3.1, Mistral, etc.)
- **Optional cloud LLM** — Bring your own Claude API key for enhanced analysis
- **Encrypted storage** — SQLite with encryption support

### Architecture
- **Hexagonal (Ports & Adapters)** — Swap any component without touching business logic
- **9 port interfaces** — Document, Vector, Graph, LLM, Embedding, Memory, PageIndex, Timeline, Analysis
- **Polyglot persistence** — SQLite for documents/graphs/FTS, vector store for embeddings
- **48 unit & integration tests** across all modules

---

## Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **App Shell** | [Tauri 2.0](https://v2.tauri.app/) | Native desktop (Rust backend + web frontend) |
| **Backend** | Rust 1.94 | Memory safety, performance, async runtime |
| **Frontend** | React 19 + TypeScript 6 | Component-based UI |
| **Styling** | Tailwind CSS 4 + Lucide Icons | Dark theme, responsive design |
| **State** | Zustand 5 | Lightweight state management |
| **Database** | SQLite (rusqlite) | Documents, chunks, entities, memory facts, config |
| **Full-Text Search** | SQLite FTS5 | BM25-ranked keyword search with auto-sync triggers |
| **Vector Search** | SQLite + cosine similarity | Semantic search (upgradable to LanceDB) |
| **Graph Storage** | SQLite adjacency model | Entity relationships with recursive CTE traversal |
| **Local LLM** | [Ollama](https://ollama.com/) | Embeddings (nomic-embed-text), completions (Llama 3.1) |
| **Cloud LLM** | Claude API (Anthropic) | Optional enhanced analysis |
| **Build** | Vite 8 + Cargo | Fast dev builds, cross-platform packaging |

---

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [Node.js](https://nodejs.org/) (18+)
- [Ollama](https://ollama.com/) (for local LLM features)

### Setup

```bash
# Clone the repo
git clone https://github.com/laadtushar/MemPalace.git
cd MemPalace

# Install frontend dependencies
npm install

# Pull recommended Ollama models
ollama pull nomic-embed-text
ollama pull llama3.1:8b

# Run in development mode
cargo tauri dev
```

### Import Your Data

1. Open the app → click **Import** in the sidebar
2. Select a source (Obsidian Vault, Markdown folder, or Day One JSON)
3. Pick the file/folder → watch the progress bar
4. Your documents are now searchable!

### Search & Explore

- **Keyword search**: Type any word to find exact matches (BM25 ranking)
- **Semantic search**: Find content by meaning ("times I felt uncertain")
- **Hybrid search**: Best of both — combines keyword + semantic via RRF

---

## Project Structure

```
memory-palace/
├── src-tauri/                    # Rust backend
│   └── src/
│       ├── domain/               # Core domain models + port traits
│       │   ├── models/           # Document, Chunk, Entity, Theme, Memory, Insight
│       │   └── ports/            # 9 trait interfaces (hexagonal architecture)
│       ├── adapters/             # Port implementations
│       │   ├── sqlite/           # 6 SQLite adapters + migrations
│       │   └── llm/              # Ollama + Claude providers
│       ├── pipeline/             # Data processing
│       │   ├── ingestion/        # Source adapters, dedup, normalize, chunk, orchestrate
│       │   └── analysis/         # Theme, sentiment, belief, insight extraction
│       ├── prompts/              # 7 versioned LLM prompt templates
│       └── commands/             # 13 Tauri IPC command handlers
│
├── src/                          # React frontend
│   ├── components/               # UI views (Timeline, Search, Import, Insights, Settings)
│   ├── stores/                   # Zustand state management
│   ├── lib/                      # Typed Tauri IPC wrappers
│   └── types/                    # TypeScript domain type mirrors
```

---

## Architecture

Memory Palace follows a strict **hexagonal architecture**. The core domain has zero knowledge of specific databases, LLM providers, or UI frameworks.

```
┌─────────────────────────────────────────────────────┐
│                   React Frontend                     │
│  Timeline │ Search │ Import │ Insights │ Settings    │
├─────────────────────┬───────────────────────────────┤
│    Tauri Commands    │     Events (progress)         │
├─────────────────────┴───────────────────────────────┤
│                                                      │
│   ┌──────────────────────────────────────────────┐  │
│   │              Domain (Pure Logic)              │  │
│   │  Models: Document, Chunk, Entity, Theme...   │  │
│   │  Ports:  IDocumentStore, IVectorStore,       │  │
│   │          ILLMProvider, IMemoryStore...        │  │
│   └──────────────────────────────────────────────┘  │
│          ▲              ▲              ▲              │
│   ┌──────┴──────┐ ┌────┴────┐ ┌──────┴──────┐      │
│   │   SQLite    │ │ Ollama  │ │  Claude API │      │
│   │  Adapters   │ │ Adapter │ │   Adapter   │      │
│   │ (6 stores)  │ │(LLM+Emb)│ │   (LLM)    │      │
│   └─────────────┘ └─────────┘ └─────────────┘      │
│                                                      │
│   ┌──────────────────────────────────────────────┐  │
│   │            Ingestion Pipeline                 │  │
│   │  Parse → Dedup → Normalize → Chunk → Store   │  │
│   └──────────────────────────────────────────────┘  │
│   ┌──────────────────────────────────────────────┐  │
│   │            Analysis Pipeline                  │  │
│   │  Themes → Sentiment → Beliefs → Insights     │  │
│   └──────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

---

## Port Interfaces

| Port | Purpose | Adapter |
|------|---------|---------|
| `IDocumentStore` | Raw document + chunk persistence | SQLite |
| `IVectorStore` | Embedding storage + similarity search | SQLite (cosine sim) |
| `IGraphStore` | Entity relationships (people, concepts) | SQLite (adjacency + CTE) |
| `IPageIndex` | Full-text search (BM25) | SQLite FTS5 |
| `ITimelineStore` | Temporal range queries | SQLite |
| `IMemoryStore` | Long-term facts (Mem0-style) | SQLite |
| `ILlmProvider` | Text generation + classification | Ollama / Claude |
| `IEmbeddingProvider` | Text → vector encoding | Ollama |
| `IAnalysisStage` | Pluggable analysis pipeline stage | Built-in stages |

---

## Data Sources

### Supported (v0.1)
| Source | Format | What's Extracted |
|--------|--------|-----------------|
| **Obsidian** | Vault folder (.md) | Text, frontmatter dates, tags, wikilinks |
| **Markdown/Text** | Folder (.md, .txt) | Text, file dates |
| **Day One** | JSON export | Entries, timestamps, weather, location, tags |

### Planned (v0.2+)
| Source | Format |
|--------|--------|
| WhatsApp | ZIP (txt + media) |
| Telegram | JSON export |
| Twitter/X | ZIP archive |
| Reddit | GDPR CSV archive |
| Google Takeout | ZIP (mixed) |
| Notion | ZIP (Markdown/CSV) |

---

## Ingestion Pipeline

```
Source File/Folder
    │
    ▼
┌──────────────┐
│ Source Adapter │ ─── Obsidian / Markdown / Day One parser
└──────┬───────┘
       ▼
┌──────────────┐
│ Deduplication │ ─── SHA-256 content hash check
└──────┬───────┘
       ▼
┌──────────────┐
│ Normalizer    │ ─── Unicode NFC, timestamps to UTC, encoding cleanup
└──────┬───────┘
       ▼
┌──────────────┐
│ Chunker       │ ─── 512 tokens, 50 overlap, paragraph-aware
└──────┬───────┘
       ▼
┌──────────────┐
│ Store         │ ─── SQLite (documents + chunks + FTS5 auto-index)
└──────────────┘
```

---

## Analysis Pipeline

```
Imported Documents
    │
    ▼
┌──────────────────┐
│ Theme Extractor   │ ─── Monthly windows → LLM topic modeling → ThemeSnapshots
└──────┬───────────┘
       ▼
┌──────────────────┐
│ Sentiment Tracker │ ─── Per-chunk LLM classification → time series
└──────┬───────────┘
       ▼
┌──────────────────┐
│ Belief Extractor  │ ─── LLM extracts beliefs/preferences → MemoryFacts
└──────┬───────────┘
       ▼
┌──────────────────┐
│ Insight Generator │ ─── Synthesize top-N surprising observations
└──────────────────┘
```

---

## Search Modes

| Mode | How It Works | Best For |
|------|-------------|----------|
| **Keyword** | SQLite FTS5 with BM25 ranking | Exact phrases, names, specific terms |
| **Semantic** | Embed query → cosine similarity on stored vectors | Conceptual searches ("times I felt lost") |
| **Hybrid** | Reciprocal Rank Fusion (BM25 + vector, k=60) | Best overall relevance |

---

## Testing

```bash
# Run all 48 Rust tests
cd src-tauri && cargo test --lib

# Type check frontend
npx tsc --noEmit

# Build frontend
npx vite build
```

### Test Coverage

| Module | Tests | What's Tested |
|--------|-------|---------------|
| SQLite DocumentStore | 4 | CRUD, hash lookup, chunks |
| SQLite MemoryStore | 3 | Store/recall, forget, contradict |
| SQLite GraphStore | 2 | Entity CRUD, neighbor traversal (recursive CTE) |
| SQLite TimelineStore | 2 | Monthly counts, date ranges |
| SQLite VectorStore | 5 | Cosine similarity, batch upsert, delete, blob roundtrip |
| SQLite Migrations | 2 | Fresh run, idempotent rerun |
| SQLite FTS5 Index | 1 | BM25 search via auto-sync triggers |
| Ingestion Chunker | 5 | Paragraph boundaries, overlap, force-split |
| Ingestion Normalizer | 4 | Unicode NFC, BOM, line endings, whitespace |
| Ingestion Dedup | 1 | Content hash deduplication |
| Ingestion Orchestrator | 2 | Full pipeline, dedup in pipeline |
| Obsidian Parser | 6 | Frontmatter, tags, wikilinks, dates |
| Day One Parser | 2 | Entry parsing, empty skip |
| Prompt Templates | 2 | Non-empty, variable substitution |
| Analysis Theme | 3 | Monthly windows, JSON parsing, extraction |
| Analysis Sentiment | 1 | Label parsing |
| Analysis Beliefs | 1 | JSON response parsing |
| Analysis Insights | 1 | JSON response parsing |

---

## Roadmap

| Phase | Status | Key Deliverables |
|-------|--------|-----------------|
| **v0.1 — Foundation** | **Current** | Ingestion pipeline, SQLite storage, Ollama integration, basic timeline UI, Obsidian/Markdown import |
| v0.2 — Expansion | Planned | WhatsApp/Telegram import, LanceDB vectors, entity explorer, Windows/Linux builds |
| v0.3 — Intelligence | Planned | Evolution detection, contradiction detection, narrative generation, D3 timeline |
| v0.4 — Social | Planned | Twitter/Reddit import, relationship mapping, mobile companion |
| v1.0 — Platform | Planned | Plugin system, export, encrypted sync, public release |

---

## Configuration

Memory Palace stores its data in the platform-specific app data directory:

| Platform | Location |
|----------|----------|
| Windows | `%APPDATA%/com.memorypalace.app/` |
| macOS | `~/Library/Application Support/com.memorypalace.app/` |
| Linux | `~/.local/share/com.memorypalace.app/` |

### LLM Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| Ollama URL | `http://localhost:11434` | Local Ollama server address |
| Embedding model | `nomic-embed-text` | 1024-dim, 8192 token context |
| LLM model | `llama3.1:8b` | For theme/belief/insight extraction |
| Claude API key | (none) | Optional, stored in OS keychain |

---

## Contributing

Memory Palace is open source. Contributions welcome!

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup

```bash
# Prerequisites: Rust, Node.js 18+, Ollama

git clone https://github.com/laadtushar/MemPalace.git
cd MemPalace
npm install
cargo tauri dev     # Starts dev server + Rust backend
```

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

<p align="center">
  <i>Built with Rust, React, and the belief that your personal data should work for you — privately.</i>
</p>

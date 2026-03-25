  
**MEMORY PALACE**

Technical Requirements Document

*A searchable, visual timeline of how your thinking evolved.*

Version 1.0  |  March 2026  |  DRAFT  
Author: Tushar

# **Table of Contents**

[**Table of Contents	2**](#heading=)

[**1\. Executive Summary	3**](#heading=)

[**2\. Product Vision and Principles	4**](#heading=)

[2.1 Vision Statement	4](#heading=)

[2.2 Design Principles	4](#heading=)

[**3\. System Architecture	5**](#heading=)

[3.1 Architectural Pattern: Hexagonal (Ports & Adapters)	5](#heading=)

[3.2 High-Level Architecture Layers	5](#heading=)

[3.3 Port Interfaces (Core Contracts)	5](#heading=)

[**4\. Data Ingestion Pipeline	7**](#heading=)

[4.1 GDPR Data Takeout Strategy	7](#heading=)

[4.2 Supported Data Sources (v1)	7](#heading=)

[4.3 Ingestion Pipeline Stages	7](#heading=)

[**5\. Storage Architecture	9**](#heading=)

[5.1 Storage Engine Matrix	9](#heading=)

[5.2 Storage Initialization and Migration	9](#heading=)

[5.3 Data Model (Core Domain Entities)	9](#heading=)

[**6\. Analysis Engine	11**](#heading=)

[6.1 Analysis Pipeline Stages	11](#heading=)

[6.2 Time Window Strategy	11](#heading=)

[**7\. LLM Integration Layer	12**](#heading=)

[7.1 Provider Strategy	12](#heading=)

[7.2 LLM Usage Patterns	12](#heading=)

[7.3 Prompt Architecture	13](#heading=)

[**8\. Query and Retrieval System	14**](#heading=)

[8.1 Query Modes	14](#heading=)

[8.2 Retrieval Pipeline (RAG)	14](#heading=)

[**9\. Native Application Architecture	15**](#heading=)

[9.1 Platform Strategy	15](#heading=)

[9.2 Local-First Data Architecture	15](#heading=)

[9.3 UI Components	15](#heading=)

[**10\. Privacy and Security Architecture	17**](#heading=)

[10.1 Privacy Principles	17](#heading=)

[10.2 Threat Model	17](#heading=)

[**11\. Plugin and Extension Architecture	18**](#heading=)

[11.1 Extension Points	18](#heading=)

[11.2 Plugin API Contract	18](#heading=)

[**12\. Technology Stack (All Open Source)	19**](#heading=)

[**13\. MVP Scope (v0.1)	21**](#heading=)

[13.1 In Scope for MVP	21](#heading=)

[13.2 Deferred to v0.2+	21](#heading=)

[13.3 Success Metrics (MVP)	21](#heading=)

[**14\. Roadmap	22**](#heading=)

[**15\. Open Questions and Decisions	23**](#heading=)

[**16\. Appendix: Port Interface Definitions (Pseudocode)	24**](#heading=)

[16.1 IDocumentStore	24](#heading=)

[16.2 IVectorStore	24](#heading=)

[16.3 IGraphStore	24](#heading=)

[16.4 ILLMProvider	24](#heading=)

[16.5 IMemoryStore	24](#heading=)

# **1\. Executive Summary**

Memory Palace is a privacy-first, native application that ingests a user’s digital footprint — journal entries, chat exports, social media archives, notes, emails — and constructs a searchable, visual timeline of how their thinking, interests, and emotional patterns have evolved over time. It does not merely catalog events; it surfaces the meta-narrative of personal growth and change.

**Core Insight:** People generate vast amounts of text about themselves but have no tool to understand the arc of who they’re becoming. Memory Palace makes the invisible pattern visible.

**Target User:** Reflective individuals (journalers, therapists doing self-work, quantified-self enthusiasts, writers, anyone going through a life transition) who want deeper self-understanding.

**Key Differentiator:** All processing can happen on-device using local LLMs (Ollama, llama.cpp). Users optionally connect their own API keys (Claude, GPT, Gemini) or use OAuth for cloud inference. No data ever leaves the device unless the user explicitly opts in.

# **2\. Product Vision and Principles**

## **2.1 Vision Statement**

To give every person a living, evolving map of their inner world — built from their own words, owned entirely by them.

## **2.2 Design Principles**

* Privacy by Default: All data stored and processed locally. Cloud is always opt-in, never assumed.

* Your Data, Your Infrastructure: Users choose their own LLM provider, embedding model, and storage backend. No vendor lock-in.

* Insight Over Archive: The product surfaces patterns and evolution, not just stores documents. The timeline shows how you changed, not what happened.

* Open Source Core: All storage engines, pipeline components, and analysis modules are open source, composable, and replaceable.

* Progressive Complexity: Simple to start (drag-and-drop a journal export), powerful at depth (custom analysis pipelines, plugin architecture).

* Hexagonal Architecture: Every major subsystem communicates through ports and adapters — swap any component without touching business logic.

# **3\. System Architecture**

## **3.1 Architectural Pattern: Hexagonal (Ports & Adapters)**

The system follows a strict hexagonal architecture. The core domain (ingestion, analysis, timeline construction) has zero knowledge of specific databases, LLM providers, or UI frameworks. All external dependencies are accessed through port interfaces with swappable adapter implementations.

## **3.2 High-Level Architecture Layers**

| Layer | Responsibility | Key Components |
| :---- | :---- | :---- |
| Presentation | Native UI, visual timeline, search interface | React Native / Tauri / Flutter shell, timeline renderer, search UI |
| Application | Orchestration, use cases, pipeline coordination | Ingestion orchestrator, analysis scheduler, query coordinator |
| Domain | Core business logic, models, port interfaces | Document model, insight model, timeline model, evolution tracker |
| Infrastructure | Adapter implementations for storage, LLM, embeddings | Vector DB adapter, graph DB adapter, LLM adapter, file system adapter |

## **3.3 Port Interfaces (Core Contracts)**

Each port defines a technology-agnostic interface. Adapters implement these ports for specific backends.

| Port | Purpose | Example Adapters |
| :---- | :---- | :---- |
| IDocumentStore | Raw document persistence and retrieval | SQLite, PostgreSQL, filesystem |
| IVectorStore | Semantic embedding storage and similarity search | ChromaDB, Qdrant, Weaviate, Milvus, LanceDB |
| IGraphStore | Entity and relationship graph (people, themes, concepts) | Neo4j, Apache AGE, Memgraph, Kuzu |
| ILLMProvider | Text generation, summarization, analysis | Ollama (local), Claude API, OpenAI API, Gemini API, llama.cpp |
| IEmbeddingProvider | Text-to-vector encoding | Ollama, sentence-transformers (local), OpenAI embeddings, Cohere |
| IMemoryStore | Persistent memory / long-term context (mem0-style) | Mem0, custom SQLite-backed memory, Zep |
| IPageIndex | Full-text search with ranking | Tantivy, MeiliSearch, SQLite FTS5, Typesense |
| ITimelineStore | Temporal indexing and range queries | TimescaleDB, SQLite with temporal extensions, custom B-tree index |
| IAnalysisPipeline | Pluggable analysis stages (theme extraction, sentiment, etc.) | Built-in pipeline stages, user-defined plugins |

# **4\. Data Ingestion Pipeline**

## **4.1 GDPR Data Takeout Strategy**

Users exercise their GDPR/CCPA right to data portability and download their data archives from each platform. Memory Palace provides guided workflows for each supported platform’s export format.

## **4.2 Supported Data Sources (v1)**

| Platform / Source | Export Format | Key Data Extracted |
| :---- | :---- | :---- |
| WhatsApp | ZIP (txt \+ media) | Messages, timestamps, participants, sentiment per conversation |
| Telegram | JSON export | Messages, channels, timestamps, reply chains |
| Twitter/X | ZIP (JS/JSON archive) | Tweets, likes, bookmarks, DMs, reply threads |
| Instagram | ZIP (JSON \+ media) | DMs, captions, comments, stories text |
| Facebook | ZIP (JSON/HTML) | Posts, comments, messages, reactions |
| Google Takeout | ZIP (mixed formats) | Gmail, Keep notes, Docs, Search history, YouTube comments |
| Apple Notes | Export via Shortcuts/iCloud | Notes, checklists, embedded content |
| Notion | ZIP (Markdown/CSV) | Pages, databases, journal entries |
| Obsidian | Vault folder (Markdown) | Notes, backlinks, tags, daily notes |
| Day One / Journey | JSON export | Journal entries, photos, metadata, weather, location |
| Reddit | GDPR archive (CSV) | Posts, comments, saved items, upvote history |
| LinkedIn | ZIP (CSV) | Messages, posts, connections, endorsements |
| Plain text files | TXT / MD / CSV | Any unstructured text the user wants to include |
| Custom / Manual | Drag-and-drop any text | Freeform text with user-assigned timestamps |

## **4.3 Ingestion Pipeline Stages**

The ingestion pipeline is modular and each stage is independently replaceable:

1. Source Adapter: Parses platform-specific export format into a unified Document model (text, timestamp, source, metadata, participants).

2. Deduplication: Content-hash based dedup across sources. Same message appearing in WhatsApp export and Google Takeout is stored once.

3. Normalization: Unicode normalization, language detection, timestamp standardization to UTC, encoding cleanup.

4. Chunking: Intelligent chunking strategy — respects conversation boundaries, paragraph breaks, and semantic coherence. Configurable chunk size (default 512 tokens, overlapping).

5. Embedding Generation: Each chunk is embedded using the configured IEmbeddingProvider and stored in the IVectorStore.

6. Entity Extraction: People, places, topics, emotions, and recurring themes are extracted and stored in the IGraphStore as nodes and edges.

7. Temporal Indexing: Documents and chunks are indexed by timestamp in the ITimelineStore for efficient range queries.

8. Full-Text Indexing: Raw text is indexed in the IPageIndex for keyword search.

9. Memory Consolidation: Key facts, beliefs, preferences, and self-descriptions are extracted and stored in the IMemoryStore for long-term context.

# **5\. Storage Architecture**

Memory Palace uses a polyglot persistence strategy. Each storage engine is optimized for a specific query pattern, accessed exclusively through port interfaces.

## **5.1 Storage Engine Matrix**

| Store Type | Purpose | Default (Local) | Scalable Alternative |
| :---- | :---- | :---- | :---- |
| Document Store | Raw content, metadata, source tracking | SQLite | PostgreSQL |
| Vector Store | Semantic search, similarity, clustering | ChromaDB / LanceDB | Qdrant / Weaviate / Milvus |
| Graph Store | Entity relationships, concept maps, theme evolution | Kuzu (embedded) / SQLite \+ adjacency | Neo4j / Apache AGE / Memgraph |
| Page Index | Full-text keyword search with BM25 ranking | SQLite FTS5 / Tantivy | MeiliSearch / Typesense |
| Timeline Store | Temporal range queries, date-based retrieval | SQLite with temporal indexes | TimescaleDB |
| Memory Store | Long-term facts, beliefs, preferences (mem0-style) | SQLite-backed custom store | Mem0 / Zep |
| Config Store | User preferences, pipeline config, adapter settings | SQLite / JSON file | SQLite / JSON file |
| Cache Store | LLM response cache, embedding cache | SQLite / LMDB | Redis / DragonflyDB |

## **5.2 Storage Initialization and Migration**

On first launch, Memory Palace initializes all default local storage backends. A migration framework (inspired by Alembic/Flyway patterns) handles schema evolution. Users can swap backends at any time; a migration tool exports data from one adapter and imports into another.

## **5.3 Data Model (Core Domain Entities)**

| Entity | Key Fields | Stored In |
| :---- | :---- | :---- |
| Document | id, source\_platform, raw\_text, timestamp, participants\[\], metadata{}, content\_hash | Document Store |
| Chunk | id, document\_id, text, token\_count, position, embedding\_vector | Document Store \+ Vector Store |
| Entity (Person/Place/Concept) | id, name, type, first\_seen, last\_seen, mention\_count, aliases\[\] | Graph Store |
| Relationship | id, source\_entity\_id, target\_entity\_id, type, weight, first\_seen, context\_chunks\[\] | Graph Store |
| ThemeSnapshot | id, theme\_label, description, time\_window\_start, time\_window\_end, intensity\_score, representative\_chunks\[\] | Document Store \+ Vector Store |
| EvolutionArc | id, subject, snapshots\[\], trend\_direction, confidence, narrative\_summary | Document Store |
| MemoryFact | id, fact\_text, source\_chunks\[\], confidence, category, first\_seen, last\_updated, contradicted\_by\[\] | Memory Store |
| Insight | id, type, title, body, time\_range, supporting\_evidence\[\], generated\_at | Document Store |

# **6\. Analysis Engine**

The analysis engine is the intellectual core of Memory Palace. It transforms raw ingested data into insights about personal evolution. It operates as a configurable pipeline of analysis stages.

## **6.1 Analysis Pipeline Stages**

| Stage | Input | Output | Technique |
| :---- | :---- | :---- | :---- |
| Theme Extraction | Chunks within a time window | ThemeSnapshot\[\] | LLM-based topic modeling \+ embedding clustering (HDBSCAN/UMAP) |
| Sentiment Tracking | Chunks with timestamps | Sentiment time series per topic/person/context | LLM sentiment classification \+ rolling aggregation |
| Belief Extraction | Chunks containing opinions, values, self-descriptions | MemoryFact\[\] with category=belief | LLM extraction with confidence scoring, contradiction detection |
| Relationship Mapping | Chunks mentioning people | Entity nodes \+ Relationship edges with temporal weights | NER \+ co-occurrence \+ LLM relationship classification |
| Evolution Detection | ThemeSnapshots across time windows | EvolutionArc\[\] | Comparing theme intensity, sentiment shift, belief changes across periods |
| Narrative Generation | EvolutionArc\[\] \+ supporting evidence | Insight\[\] with human-readable narratives | LLM synthesis: turns patterns into prose the user can understand |
| Contradiction Detection | MemoryFact\[\] across time | Flagged contradictions and belief reversals | Embedding similarity \+ LLM verification of semantic contradiction |

## **6.2 Time Window Strategy**

Analysis operates at multiple temporal granularities:

* Micro (daily/weekly): Mood, active conversations, immediate concerns.

* Meso (monthly/quarterly): Shifting interests, relationship dynamics, recurring themes.

* Macro (yearly/multi-year): Life chapters, core belief evolution, identity shifts.

The system uses sliding windows with configurable overlap. Users can define custom time boundaries (e.g., “before/after moving to London”, “during university”).

# **7\. LLM Integration Layer**

## **7.1 Provider Strategy**

Memory Palace is LLM-agnostic. The ILLMProvider port defines a minimal interface (complete, embed, classify) that adapters implement for each provider.

| Provider | Type | Connection Method | Best For |
| :---- | :---- | :---- | :---- |
| Ollama | Local | HTTP API (localhost) | Privacy-first users, offline use, no API costs |
| llama.cpp | Local | Native binding or HTTP server | Maximum performance on local hardware |
| MLX (Apple Silicon) | Local | Native binding | Mac users wanting GPU-accelerated local inference |
| Claude (Anthropic) | Cloud | API key or OAuth | Best reasoning quality, long context analysis |
| GPT (OpenAI) | Cloud | API key | Wide model selection, function calling |
| Gemini (Google) | Cloud | API key or OAuth | Large context window, multimodal |
| Mistral | Cloud | API key | European hosting, good price/performance |
| Custom/Self-hosted | Either | Configurable HTTP endpoint | Enterprise or advanced users with their own infra |

## **7.2 LLM Usage Patterns**

| Task | Model Size Needed | Privacy Sensitivity | Recommended Default |
| :---- | :---- | :---- | :---- |
| Embedding generation | Small (300M-1B) | High (runs on all content) | Local: nomic-embed-text via Ollama |
| Theme extraction | Medium (7B-13B) | High | Local: Mistral 7B / Llama 3 8B via Ollama |
| Sentiment classification | Small-Medium (3B-7B) | High | Local: Phi-3 / Llama 3 8B |
| Narrative generation | Large (13B+ or cloud) | Medium (works on aggregated themes, not raw text) | Cloud: Claude Sonnet / GPT-4o (user’s API key) |
| Contradiction detection | Large (13B+ or cloud) | Medium | Cloud: Claude Sonnet |
| Interactive Q\&A about your data | Large | Medium-High (depends on query) | User’s choice: local for sensitive, cloud for quality |

## **7.3 Prompt Architecture**

All prompts are versioned, templated, and stored as configuration — not hardcoded. Prompt templates use a Jinja2-style template engine with variables for context injection. Each analysis stage has its own prompt template set that can be customized or replaced by the user.

A prompt registry tracks which prompt version produced which insight, enabling reproducibility and A/B testing of prompt strategies.

# **8\. Query and Retrieval System**

Memory Palace supports multiple query modes, each combining different storage backends for optimal results.

## **8.1 Query Modes**

| Query Mode | Description | Backends Used | Example Query |
| :---- | :---- | :---- | :---- |
| Semantic Search | Find content by meaning, not keywords | Vector Store \+ Document Store | "times I felt uncertain about my career" |
| Keyword Search | Traditional full-text search | Page Index | "London flat deposit" |
| Temporal Query | Find content within a time range | Timeline Store \+ Document Store | "what was I writing about in summer 2022?" |
| Entity Query | Explore content about a specific person/topic | Graph Store \+ Document Store | "all conversations involving Sarah about the startup" |
| Evolution Query | Track how a theme/belief changed over time | Vector Store \+ Timeline Store \+ Graph Store | "how has my view on remote work changed?" |
| Hybrid (RAG) | Natural language question answered with personal context | All stores via fusion retrieval | "what was the turning point in my relationship with work?" |

## **8.2 Retrieval Pipeline (RAG)**

The RAG pipeline uses reciprocal rank fusion to combine results from multiple retrieval strategies:

10. Query Understanding: LLM classifies the query type (semantic, temporal, entity, evolution) and extracts key parameters.

11. Multi-Store Retrieval: Parallel queries to relevant stores based on query classification.

12. Rank Fusion: Reciprocal rank fusion merges results from different stores into a unified ranking.

13. Context Assembly: Top-ranked chunks are assembled into a coherent context window, respecting token limits.

14. Memory Augmentation: Relevant MemoryFacts from the Memory Store are prepended as persistent context.

15. LLM Generation: The assembled context \+ query is sent to the configured LLM for response generation.

16. Citation Linking: Response is annotated with links back to source documents and timestamps.

# **9\. Native Application Architecture**

## **9.1 Platform Strategy**

Memory Palace is a native desktop-first application with mobile companion support. Desktop-first because: (a) data imports are large and complex, (b) local LLM inference benefits from desktop hardware, (c) the timeline visualization needs screen real estate.

| Platform | Framework | Rationale |
| :---- | :---- | :---- |
| Desktop (macOS, Windows, Linux) | Tauri 2.0 (Rust backend \+ web frontend) | Small binary, native performance, Rust ecosystem for local LLM bindings, open source |
| Mobile (iOS, Android) | React Native or Flutter (companion app) | Lightweight: view timeline, quick capture, voice notes. Heavy processing on desktop. |
| Web (optional, self-hosted) | Same frontend as Tauri webview | For users who want to self-host on a home server or NAS |

## **9.2 Local-First Data Architecture**

All data lives on the user’s device by default. The application uses an embedded database layer (SQLite as the foundation) with optional sync:

* Primary storage: SQLite (via rusqlite in Tauri’s Rust backend) for document store, config, cache, and memory store.

* Vector storage: ChromaDB or LanceDB running as an embedded process (no external server needed).

* Graph storage: Kuzu (embedded graph database) or SQLite with adjacency list modeling for v1.

* Full-text search: SQLite FTS5 or embedded Tantivy for v1.

* Optional sync: If the user opts in, encrypted sync between desktop and mobile via a user-controlled backend (self-hosted Syncthing, iCloud, or custom S3-compatible storage).

## **9.3 UI Components**

* Timeline View: The primary interface. A zoomable, scrollable timeline showing theme intensities, mood arcs, and key insights as layers. Zoom from decade-level to individual days.

* Search Interface: Unified search bar supporting all query modes. Results displayed with source attribution, timestamps, and relevance scores.

* Evolution Explorer: Select a topic/belief and see a visual diff of how it changed across time periods. Side-by-side comparison of your language and sentiment then vs. now.

* Graph Explorer: Interactive node-link diagram of people, concepts, and themes in your data. Filter by time range to see how your social and intellectual graph evolved.

* Insight Feed: A curated stream of AI-generated observations ("You stopped mentioning anxiety around Q3 2023 — that’s when you started journaling about boundaries").

* Import Wizard: Step-by-step guided import for each platform’s data takeout format. Progress tracking, dedup reporting, and error handling.

* Settings and Privacy Dashboard: Full visibility into what data is stored, where, and which LLM provider is being used. One-click data deletion, export, and provider switching.

# **10\. Privacy and Security Architecture**

## **10.1 Privacy Principles**

* Zero-knowledge by default: No telemetry, no analytics, no phone-home. The app works fully offline.

* Encryption at rest: All local databases encrypted with a user-provided passphrase (SQLCipher for SQLite, native encryption for other stores).

* Encryption in transit: All cloud API calls use TLS 1.3. API keys stored in the OS keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service).

* No data aggregation: Memory Palace never collects, aggregates, or has access to user data. There is no backend service.

* Transparent cloud usage: When cloud LLMs are used, the app clearly shows what data is being sent, to which provider, and logs every API call locally for user review.

## **10.2 Threat Model**

| Threat | Mitigation |
| :---- | :---- |
| Device theft / unauthorized access | Database encryption at rest \+ OS-level authentication required to unlock app |
| Cloud LLM provider logging user data | Clear disclosure, local-first default, option to use only local models |
| Malicious plugin/adapter | Plugin sandboxing, capability-based permissions, code signing for official adapters |
| Data export to unencrypted location | Warnings when exporting unencrypted data, secure export with password-protected archives |
| Memory store leaking sensitive facts | User-reviewable memory store with manual delete/edit, automatic PII detection and flagging |

# **11\. Plugin and Extension Architecture**

Memory Palace is designed to be extended. The hexagonal architecture means any port can have new adapters added without modifying core logic.

## **11.1 Extension Points**

| Extension Point | What It Enables | Example Plugins |
| :---- | :---- | :---- |
| Source Adapter Plugin | Import from new platforms | Slack export parser, Kindle highlights importer, Spotify listening history |
| Analysis Stage Plugin | New analysis capabilities | Dream pattern analyzer, language complexity tracker, writing style evolution |
| LLM Provider Plugin | New model providers | Anthropic Claude via MCP, local GGUF loader, Groq cloud inference |
| Storage Adapter Plugin | New storage backends | DuckDB adapter, SurrealDB adapter, Pinecone vector store |
| Visualization Plugin | New timeline views | Heatmap view, network graph view, word cloud evolution |
| Export Plugin | New export formats | Obsidian vault export, PDF report, shareable web timeline |

## **11.2 Plugin API Contract**

Plugins implement a standard interface: init(), capabilities(), execute(context), and teardown(). They declare required permissions (filesystem access, network access, LLM access) and operate within a sandboxed environment. A plugin manifest (JSON) describes metadata, dependencies, and required port interfaces.

# **12\. Technology Stack (All Open Source)**

| Layer | Technology | License | Why |
| :---- | :---- | :---- | :---- |
| App Shell | Tauri 2.0 | MIT/Apache 2.0 | Rust performance, small binary, native OS integration, active community |
| Frontend | React \+ TypeScript | MIT | Massive ecosystem, Tauri webview compatible, easy to hire for |
| UI Components | shadcn/ui \+ Tailwind | MIT | Accessible, composable, no runtime dependency |
| State Management | Zustand or Jotai | MIT | Lightweight, TypeScript-native, works well with Tauri IPC |
| Timeline Visualization | D3.js \+ custom Canvas/WebGL | ISC/MIT | Full control over timeline rendering, performant for large datasets |
| Backend (Tauri) | Rust | MIT/Apache 2.0 | Memory safety, performance, native bindings to local LLMs |
| Embedded DB | SQLite (via rusqlite) \+ SQLCipher | Public Domain / BSD | Battle-tested, embedded, encrypted |
| Vector DB | ChromaDB or LanceDB | Apache 2.0 | Embedded mode, Python/Rust bindings, good developer experience |
| Graph DB | Kuzu | MIT | Embedded, fast, Cypher-compatible, no external server |
| Full-Text Search | Tantivy or SQLite FTS5 | MIT / Public Domain | Embedded, Rust-native (Tantivy), zero-config (FTS5) |
| Local LLM Runtime | Ollama / llama.cpp | MIT | Broad model support, easy setup, active development |
| Embedding Models | nomic-embed-text / BGE / E5 | Apache 2.0 | Strong performance, small footprint, local execution |
| Pipeline Orchestration | Custom Rust pipeline (inspired by Apache Arrow DataFusion) | N/A | Composable, type-safe, streaming-capable |
| Memory Layer | Custom (inspired by Mem0) | N/A | Lightweight, local, configurable retention policies |
| Testing | Vitest (frontend) \+ Rust test \+ integration tests | MIT | Fast, modern, good DX |
| CI/CD | GitHub Actions | N/A | Free for open source, good Rust/Tauri support |

# **13\. MVP Scope (v0.1)**

The MVP focuses on a single, compelling vertical: journal/notes import with evolution timeline.

## **13.1 In Scope for MVP**

* Import: Obsidian vault, plain text/markdown files, Day One JSON export.

* Storage: SQLite (document \+ memory \+ config), ChromaDB (vectors), SQLite FTS5 (full-text).

* Analysis: Theme extraction (monthly windows), basic sentiment tracking, belief extraction.

* LLM: Ollama (local) \+ Claude API key (optional cloud).

* UI: Basic timeline view, semantic search, insight feed (5 most interesting observations).

* Platform: macOS desktop (Tauri). Linux and Windows in v0.2.

## **13.2 Deferred to v0.2+**

* Chat platform imports (WhatsApp, Telegram, etc.).

* Graph store and entity explorer.

* Mobile companion app.

* Plugin system.

* Multi-device sync.

* Advanced evolution visualization (side-by-side, diff view).

* Custom time boundaries and user-defined analysis pipelines.

## **13.3 Success Metrics (MVP)**

* User can import an Obsidian vault of 500+ notes in under 5 minutes.

* Timeline shows at least 3 meaningful theme evolution arcs.

* Semantic search returns relevant results in under 2 seconds (local).

* At least 1 insight per 100 imported documents that the user finds genuinely surprising or valuable.

* Full functionality with zero cloud dependency (Ollama only).

# **14\. Roadmap**

| Phase | Timeline | Key Deliverables |
| :---- | :---- | :---- |
| v0.1 — Foundation | Weeks 1–8 | Core ingestion pipeline, SQLite \+ ChromaDB, Ollama integration, basic timeline UI, Obsidian/markdown import |
| v0.2 — Expansion | Weeks 9–16 | WhatsApp/Telegram import, graph store (Kuzu), entity explorer, Windows/Linux support |
| v0.3 — Intelligence | Weeks 17–24 | Evolution detection, contradiction detection, narrative generation, advanced timeline visualization |
| v0.4 — Social | Weeks 25–32 | Twitter/Reddit/Instagram import, relationship mapping, mobile companion (read-only) |
| v1.0 — Platform | Weeks 33–40 | Plugin system, export capabilities, encrypted sync, public launch |

# **15\. Open Questions and Decisions**

| Question | Options | Recommendation | Decision Status |
| :---- | :---- | :---- | :---- |
| Tauri vs Electron vs Flutter Desktop? | Tauri (Rust), Electron (JS), Flutter (Dart) | Tauri — smallest binary, best perf, Rust ecosystem | Proposed |
| ChromaDB vs LanceDB vs Qdrant for embedded vector? | All support embedded mode | LanceDB — Rust-native, columnar, no Python dependency | Needs testing |
| How to handle multimedia (voice notes, images)? | Transcribe audio, OCR images, or ignore for v1 | Transcribe \+ OCR as a post-v1 plugin | Deferred |
| Monetization model? | Open core, donations, premium plugins, hosted version | Open core: free local app, paid cloud sync \+ premium analysis | To be decided |
| Should Memory Palace have a social/sharing dimension? | Keep private, allow anonymous sharing, enable collaboration | Private by default, opt-in shareable insights post-v1 | To be decided |
| How to handle data from deceased users? | Digital legacy, export, deletion | Research and define policy for v1.0 | Deferred |

# **16\. Appendix: Port Interface Definitions (Pseudocode)**

The following pseudocode defines the core port interfaces. Actual implementations will use Rust traits (backend) and TypeScript interfaces (frontend).

## **16.1 IDocumentStore**

save(doc: Document) \-\> DocumentId

getById(id: DocumentId) \-\> Document?

getBySource(platform: String, timeRange: TimeRange?) \-\> Document\[\]

getByContentHash(hash: String) \-\> Document?

delete(id: DocumentId) \-\> bool

## **16.2 IVectorStore**

upsert(id: ChunkId, vector: float\[\], metadata: Map) \-\> void

search(queryVector: float\[\], topK: int, filter: MetadataFilter?) \-\> ScoredResult\[\]

delete(id: ChunkId) \-\> void

cluster(nClusters: int, filter: MetadataFilter?) \-\> Cluster\[\]

## **16.3 IGraphStore**

addNode(entity: Entity) \-\> NodeId

addEdge(from: NodeId, to: NodeId, rel: Relationship) \-\> EdgeId

getNeighbors(nodeId: NodeId, depth: int, relType: String?) \-\> SubGraph

shortestPath(from: NodeId, to: NodeId) \-\> Path?

getByTimeRange(range: TimeRange) \-\> SubGraph

## **16.4 ILLMProvider**

complete(prompt: String, systemPrompt: String?, params: LLMParams) \-\> String

embed(text: String) \-\> float\[\]

classify(text: String, categories: String\[\]) \-\> Classification

streamComplete(prompt: String, systemPrompt: String?, params: LLMParams) \-\> Stream\<String\>

## **16.5 IMemoryStore**

store(fact: MemoryFact) \-\> FactId

recall(query: String, topK: int) \-\> MemoryFact\[\]

update(id: FactId, updatedFact: MemoryFact) \-\> void

contradict(id: FactId, contradictingFact: MemoryFact) \-\> void

forget(id: FactId) \-\> void

getAll(category: String?, timeRange: TimeRange?) \-\> MemoryFact\[\]
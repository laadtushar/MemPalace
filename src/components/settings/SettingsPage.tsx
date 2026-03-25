import { useEffect, useState } from "react";
import { commands, type OllamaStatus, type AppStats } from "@/lib/tauri";
import {
  Cpu,
  Database,
  CheckCircle,
  XCircle,
  RefreshCw,
  Shield,
  Terminal,
  HardDrive,
  Info,
} from "lucide-react";

export function SettingsPage() {
  const [ollamaStatus, setOllamaStatus] = useState<OllamaStatus | null>(null);
  const [appStats, setAppStats] = useState<AppStats | null>(null);
  const [testing, setTesting] = useState(false);

  const testConnection = async () => {
    setTesting(true);
    try {
      const status = await commands.testOllamaConnection();
      setOllamaStatus(status);
    } catch {
      setOllamaStatus({ connected: false, models: [] });
    }
    setTesting(false);
  };

  const loadStats = async () => {
    try {
      const stats = await commands.getAppStats();
      setAppStats(stats);
    } catch {
      /* ignore */
    }
  };

  useEffect(() => {
    testConnection();
    loadStats();
  }, []);

  const hasEmbeddingModel = ollamaStatus?.models.some((m) =>
    m.includes("nomic-embed-text"),
  );
  const hasLlmModel = ollamaStatus?.models.some(
    (m) => m.includes("llama3") || m.includes("mistral") || m.includes("phi"),
  );

  return (
    <div className="p-6 max-w-2xl mx-auto space-y-8 h-full overflow-y-auto">
      <h1 className="text-2xl font-semibold">Settings</h1>

      {/* LLM Provider */}
      <section className="space-y-4">
        <h2 className="text-lg font-medium flex items-center gap-2">
          <Cpu size={20} className="text-primary" /> LLM Provider
        </h2>
        <div className="rounded-lg border border-border bg-card p-4 space-y-3">
          <div className="flex items-center justify-between">
            <div>
              <p className="font-medium">Ollama (Local)</p>
              <p className="text-sm text-muted-foreground">
                http://localhost:11434
              </p>
            </div>
            <div className="flex items-center gap-2">
              {ollamaStatus?.connected ? (
                <span className="flex items-center gap-1 text-green-400 text-sm">
                  <CheckCircle size={16} /> Connected
                </span>
              ) : (
                <span className="flex items-center gap-1 text-red-400 text-sm">
                  <XCircle size={16} /> Not connected
                </span>
              )}
              <button
                onClick={testConnection}
                disabled={testing}
                className="rounded-md bg-secondary px-3 py-1.5 text-sm hover:bg-secondary/80 disabled:opacity-50"
              >
                <RefreshCw
                  size={14}
                  className={testing ? "animate-spin" : ""}
                />
              </button>
            </div>
          </div>

          {ollamaStatus?.connected && ollamaStatus.models.length > 0 && (
            <div>
              <p className="text-sm text-muted-foreground mb-1.5">
                Available models:
              </p>
              <div className="flex flex-wrap gap-1.5">
                {ollamaStatus.models.map((model) => {
                  const isEmbed = model.includes("nomic-embed") || model.includes("embed");
                  const isActive = model.includes("llama3") || model.includes("nomic-embed");
                  return (
                    <span
                      key={model}
                      className={`rounded-md px-2 py-0.5 text-xs border ${
                        isActive
                          ? "border-primary/30 bg-primary/10 text-primary"
                          : "border-border bg-secondary text-muted-foreground"
                      }`}
                    >
                      {model}
                      {isEmbed && " (embedding)"}
                    </span>
                  );
                })}
              </div>
            </div>
          )}

          {/* Model status checks */}
          {ollamaStatus?.connected && (
            <div className="space-y-1.5 pt-2 border-t border-border/50">
              <div className="flex items-center gap-2 text-sm">
                {hasEmbeddingModel ? (
                  <CheckCircle size={14} className="text-green-400" />
                ) : (
                  <XCircle size={14} className="text-yellow-400" />
                )}
                <span className={hasEmbeddingModel ? "text-foreground" : "text-muted-foreground"}>
                  Embedding model (nomic-embed-text)
                  {!hasEmbeddingModel && " — needed for semantic search"}
                </span>
              </div>
              <div className="flex items-center gap-2 text-sm">
                {hasLlmModel ? (
                  <CheckCircle size={14} className="text-green-400" />
                ) : (
                  <XCircle size={14} className="text-yellow-400" />
                )}
                <span className={hasLlmModel ? "text-foreground" : "text-muted-foreground"}>
                  LLM model (llama3.1:8b)
                  {!hasLlmModel && " — needed for analysis & RAG"}
                </span>
              </div>
            </div>
          )}

          {ollamaStatus && !ollamaStatus.connected && (
            <div className="rounded-md bg-secondary/50 p-3 space-y-2">
              <p className="text-sm text-muted-foreground">
                Ollama is not running. Install it from{" "}
                <span className="text-primary font-medium">ollama.com</span>,
                then run:
              </p>
              <div className="rounded bg-background px-3 py-2 font-mono text-xs text-muted-foreground space-y-1">
                <div className="flex items-center gap-2">
                  <Terminal size={12} />
                  <span>ollama pull nomic-embed-text</span>
                </div>
                <div className="flex items-center gap-2">
                  <Terminal size={12} />
                  <span>ollama pull llama3.1:8b</span>
                </div>
              </div>
            </div>
          )}
        </div>
      </section>

      {/* Model Configuration */}
      <section className="space-y-4">
        <h2 className="text-lg font-medium flex items-center gap-2">
          <Info size={20} className="text-primary" /> Model Configuration
        </h2>
        <div className="rounded-lg border border-border bg-card p-4">
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <p className="text-muted-foreground">Embedding Model</p>
              <p className="font-mono mt-0.5">nomic-embed-text</p>
              <p className="text-[11px] text-muted-foreground">1024 dims, 8192 token context</p>
            </div>
            <div>
              <p className="text-muted-foreground">LLM Model</p>
              <p className="font-mono mt-0.5">llama3.1:8b</p>
              <p className="text-[11px] text-muted-foreground">Theme/belief/insight extraction</p>
            </div>
            <div>
              <p className="text-muted-foreground">Chunk Size</p>
              <p className="font-mono mt-0.5">512 tokens</p>
              <p className="text-[11px] text-muted-foreground">50 token overlap, paragraph-aware</p>
            </div>
            <div>
              <p className="text-muted-foreground">Search Fusion</p>
              <p className="font-mono mt-0.5">RRF (k=60)</p>
              <p className="text-[11px] text-muted-foreground">Reciprocal Rank Fusion</p>
            </div>
          </div>
        </div>
      </section>

      {/* Data Overview */}
      <section className="space-y-4">
        <h2 className="text-lg font-medium flex items-center gap-2">
          <Database size={20} className="text-primary" /> Data Overview
        </h2>
        <div className="rounded-lg border border-border bg-card p-4">
          {appStats ? (
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <p className="text-2xl font-bold tabular-nums">
                    {appStats.total_documents}
                  </p>
                  <p className="text-sm text-muted-foreground">Documents</p>
                </div>
                <div>
                  <p className="text-2xl font-bold tabular-nums">
                    {appStats.total_memory_facts}
                  </p>
                  <p className="text-sm text-muted-foreground">Memory Facts</p>
                </div>
              </div>
              {appStats.date_range && (
                <div className="pt-2 border-t border-border/50">
                  <p className="text-sm text-muted-foreground">
                    Date range:{" "}
                    {new Date(appStats.date_range[0]).toLocaleDateString()} —{" "}
                    {new Date(appStats.date_range[1]).toLocaleDateString()}
                  </p>
                </div>
              )}
            </div>
          ) : (
            <p className="text-muted-foreground text-sm">Loading...</p>
          )}
        </div>
      </section>

      {/* Storage */}
      <section className="space-y-4">
        <h2 className="text-lg font-medium flex items-center gap-2">
          <HardDrive size={20} className="text-primary" /> Storage
        </h2>
        <div className="rounded-lg border border-border bg-card p-4 space-y-2 text-sm">
          <div className="flex justify-between">
            <span className="text-muted-foreground">Database</span>
            <span className="font-mono">SQLite (WAL mode)</span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted-foreground">Full-text search</span>
            <span className="font-mono">FTS5 (BM25)</span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted-foreground">Vector store</span>
            <span className="font-mono">SQLite (cosine sim)</span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted-foreground">Graph store</span>
            <span className="font-mono">SQLite (adjacency + CTE)</span>
          </div>
        </div>
      </section>

      {/* Privacy */}
      <section className="space-y-4">
        <h2 className="text-lg font-medium flex items-center gap-2">
          <Shield size={20} className="text-primary" /> Privacy
        </h2>
        <div className="rounded-lg border border-border bg-card p-4 space-y-2">
          <div className="flex items-center gap-2 text-sm">
            <CheckCircle size={14} className="text-green-400 shrink-0" />
            <span>All data stored locally on your device</span>
          </div>
          <div className="flex items-center gap-2 text-sm">
            <CheckCircle size={14} className="text-green-400 shrink-0" />
            <span>Zero telemetry — no analytics, no phone-home</span>
          </div>
          <div className="flex items-center gap-2 text-sm">
            <CheckCircle size={14} className="text-green-400 shrink-0" />
            <span>LLM processing via Ollama — nothing leaves your machine</span>
          </div>
          <div className="flex items-center gap-2 text-sm">
            <CheckCircle size={14} className="text-green-400 shrink-0" />
            <span>Cloud API keys (if used) stored in OS keychain</span>
          </div>
        </div>
      </section>

      {/* About */}
      <section className="text-center text-xs text-muted-foreground pb-8">
        <p>Memory Palace v0.1.0 — MVP</p>
        <p className="mt-1">
          Built with Rust, React, and the belief that your data should help you
          understand yourself.
        </p>
      </section>
    </div>
  );
}

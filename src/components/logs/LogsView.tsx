import { useEffect, useState, useRef, useCallback } from "react";
import {
  commands,
  type LogEntry,
} from "@/lib/tauri";
import {
  ScrollText,
  RefreshCw,
  Copy,
  ExternalLink,
  ChevronDown,
  Pause,
  Play,
  Filter,
} from "lucide-react";

const LEVEL_COLORS: Record<string, string> = {
  ERROR: "text-red-400",
  WARN: "text-yellow-400",
  INFO: "text-blue-400",
  DEBUG: "text-zinc-400",
  TRACE: "text-zinc-500",
};

const LEVEL_BG: Record<string, string> = {
  ERROR: "bg-red-500/10 border-red-500/20",
  WARN: "bg-yellow-500/10 border-yellow-500/20",
  INFO: "bg-blue-500/10 border-blue-500/20",
  DEBUG: "bg-zinc-500/10 border-zinc-500/20",
  TRACE: "bg-zinc-500/10 border-zinc-500/20",
};

export function LogsView() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [autoRefresh, setAutoRefresh] = useState(true);
  const [levelFilter, setLevelFilter] = useState<string>("ALL");
  const [logPath, setLogPath] = useState<string>("");
  const [copied, setCopied] = useState(false);
  const [limit, setLimit] = useState(200);
  const scrollRef = useRef<HTMLDivElement>(null);
  const bottomRef = useRef<HTMLDivElement>(null);
  const [autoScroll, setAutoScroll] = useState(true);

  const fetchLogs = useCallback(async () => {
    try {
      const entries = await commands.getAppLogs(
        limit,
        levelFilter === "ALL" ? undefined : levelFilter,
      );
      setLogs(entries);
    } catch {
      // Log fetch can fail if no log file exists yet
    }
    setLoading(false);
  }, [limit, levelFilter]);

  // Initial load + path
  useEffect(() => {
    fetchLogs();
    commands.getLogPath().then(setLogPath).catch(() => {});
  }, [fetchLogs]);

  // Auto-refresh every 3 seconds
  useEffect(() => {
    if (!autoRefresh) return;
    const interval = setInterval(fetchLogs, 3000);
    return () => clearInterval(interval);
  }, [autoRefresh, fetchLogs]);

  // Auto-scroll to bottom when new logs arrive
  useEffect(() => {
    if (autoScroll && bottomRef.current) {
      bottomRef.current.scrollIntoView({ behavior: "smooth" });
    }
  }, [logs, autoScroll]);

  const handleScroll = () => {
    if (!scrollRef.current) return;
    const { scrollTop, scrollHeight, clientHeight } = scrollRef.current;
    const isAtBottom = scrollHeight - scrollTop - clientHeight < 50;
    setAutoScroll(isAtBottom);
  };

  const copyLogs = async () => {
    const text = logs
      .map(
        (l) =>
          `${l.timestamp} ${l.level.padEnd(5)} ${l.target ? l.target + ": " : ""}${l.message}`,
      )
      .join("\n");
    await navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const loadMore = () => {
    setLimit((prev) => prev + 200);
  };

  const levels = ["ALL", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="flex items-center justify-between px-6 py-4 border-b border-border shrink-0">
        <h1 className="text-xl font-semibold flex items-center gap-2">
          <ScrollText size={22} className="text-primary" />
          Application Logs
        </h1>
        <div className="flex items-center gap-2">
          <button
            onClick={() => setAutoRefresh(!autoRefresh)}
            className={`flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs transition-colors ${
              autoRefresh
                ? "bg-green-500/10 text-green-400 border border-green-500/20"
                : "bg-secondary text-muted-foreground border border-border"
            }`}
            title={autoRefresh ? "Pause auto-refresh" : "Resume auto-refresh"}
          >
            {autoRefresh ? <Pause size={12} /> : <Play size={12} />}
            {autoRefresh ? "Live" : "Paused"}
          </button>
          <button
            onClick={fetchLogs}
            disabled={loading}
            className="flex items-center gap-1.5 rounded-md bg-secondary px-3 py-1.5 text-xs hover:bg-secondary/80"
            title="Refresh now"
          >
            <RefreshCw size={12} className={loading ? "animate-spin" : ""} />
            Refresh
          </button>
          <button
            onClick={copyLogs}
            className="flex items-center gap-1.5 rounded-md bg-secondary px-3 py-1.5 text-xs hover:bg-secondary/80"
            title="Copy logs to clipboard"
          >
            <Copy size={12} />
            {copied ? "Copied!" : "Copy"}
          </button>
          {logPath && (
            <button
              onClick={async () => {
                try {
                  const { open } = await import(
                    "@tauri-apps/plugin-shell"
                  );
                  await open(logPath);
                } catch {
                  // fallback: just show path
                }
              }}
              className="flex items-center gap-1.5 rounded-md bg-secondary px-3 py-1.5 text-xs hover:bg-secondary/80"
              title="Open log file in default editor"
            >
              <ExternalLink size={12} />
              Open file
            </button>
          )}
        </div>
      </div>

      {/* Level filter bar */}
      <div className="flex items-center gap-2 px-6 py-2 border-b border-border/50 shrink-0">
        <Filter size={14} className="text-muted-foreground" />
        {levels.map((level) => (
          <button
            key={level}
            onClick={() => setLevelFilter(level)}
            className={`rounded-md px-2.5 py-1 text-xs font-medium transition-colors ${
              levelFilter === level
                ? level === "ALL"
                  ? "bg-primary/10 text-primary border border-primary/20"
                  : `${LEVEL_BG[level] || ""} ${LEVEL_COLORS[level] || ""} border`
                : "text-muted-foreground hover:text-foreground hover:bg-secondary"
            }`}
          >
            {level}
          </button>
        ))}
        <span className="ml-auto text-xs text-muted-foreground tabular-nums">
          {logs.length} entries
        </span>
      </div>

      {/* Log entries */}
      <div
        ref={scrollRef}
        onScroll={handleScroll}
        className="flex-1 overflow-y-auto px-4 py-2 font-mono text-xs leading-relaxed"
      >
        {loading && logs.length === 0 ? (
          <div className="flex items-center justify-center h-32 text-muted-foreground">
            Loading logs...
          </div>
        ) : logs.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-32 text-muted-foreground gap-2">
            <ScrollText size={24} className="opacity-50" />
            <p>No log entries found.</p>
            <p className="text-[10px]">
              Logs will appear here as the application runs.
            </p>
          </div>
        ) : (
          <>
            {limit <= logs.length && (
              <button
                onClick={loadMore}
                className="flex items-center gap-1.5 mx-auto mb-2 rounded-md bg-secondary px-4 py-1.5 text-xs text-muted-foreground hover:text-foreground"
              >
                <ChevronDown size={12} className="rotate-180" />
                Load more
              </button>
            )}
            {logs.map((entry, i) => (
              <div
                key={i}
                className={`flex gap-2 py-0.5 border-b border-border/10 hover:bg-secondary/30 transition-colors ${
                  entry.level === "ERROR" ? "bg-red-500/5" : ""
                } ${entry.level === "WARN" ? "bg-yellow-500/5" : ""}`}
              >
                <span className="text-muted-foreground shrink-0 w-[200px] truncate">
                  {entry.timestamp}
                </span>
                <span
                  className={`shrink-0 w-[50px] font-semibold ${LEVEL_COLORS[entry.level] || "text-zinc-400"}`}
                >
                  {entry.level}
                </span>
                <span className="text-muted-foreground shrink-0 max-w-[250px] truncate" title={entry.target}>
                  {entry.target}
                </span>
                <span className="text-foreground break-all">
                  {entry.message}
                </span>
              </div>
            ))}
            <div ref={bottomRef} />
          </>
        )}
      </div>

      {/* Footer */}
      {logPath && (
        <div className="px-6 py-2 border-t border-border/50 text-[10px] text-muted-foreground shrink-0 truncate">
          Log file: {logPath}
        </div>
      )}
    </div>
  );
}

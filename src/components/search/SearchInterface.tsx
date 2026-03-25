import { useState } from "react";
import { commands, type SearchResult } from "@/lib/tauri";
import { Search, Clock, FileText, Zap, Type, Layers } from "lucide-react";

type SearchMode = "hybrid" | "keyword" | "semantic";

const modes: { id: SearchMode; label: string; icon: React.ReactNode }[] = [
  { id: "hybrid", label: "Hybrid", icon: <Layers size={14} /> },
  { id: "keyword", label: "Keyword", icon: <Type size={14} /> },
  { id: "semantic", label: "Semantic", icon: <Zap size={14} /> },
];

export function SearchInterface() {
  const [query, setQuery] = useState("");
  const [mode, setMode] = useState<SearchMode>("hybrid");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [searching, setSearching] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedDoc, setSelectedDoc] = useState<string | null>(null);
  const [docText, setDocText] = useState<string | null>(null);

  const doSearch = async () => {
    if (!query.trim()) return;
    setSearching(true);
    setError(null);
    setSelectedDoc(null);
    setDocText(null);
    try {
      let res: SearchResult[];
      if (mode === "keyword") {
        res = await commands.keywordSearch(query, 20);
      } else if (mode === "semantic") {
        res = await commands.semanticSearch(query, 20);
      } else {
        res = await commands.hybridSearch(query, 20);
      }
      setResults(res);
    } catch (e) {
      setError(String(e));
      setResults([]);
    }
    setSearching(false);
  };

  const viewDocument = async (docId: string) => {
    setSelectedDoc(docId);
    try {
      const text = await commands.getDocumentText(docId);
      setDocText(text);
    } catch (_e) {
      setDocText("Failed to load document.");
    }
  };

  return (
    <div className="flex h-full">
      {/* Search panel */}
      <div className="flex-1 flex flex-col p-6 overflow-hidden">
        <h1 className="text-2xl font-semibold mb-4">Search</h1>

        {/* Mode toggle */}
        <div className="flex gap-1 mb-3">
          {modes.map((m) => (
            <button
              key={m.id}
              onClick={() => setMode(m.id)}
              className={`flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs transition-colors ${
                mode === m.id
                  ? "bg-primary text-primary-foreground"
                  : "bg-secondary text-muted-foreground hover:text-foreground"
              }`}
            >
              {m.icon}
              {m.label}
            </button>
          ))}
        </div>

        {/* Search bar */}
        <div className="flex gap-2 mb-4">
          <div className="relative flex-1">
            <Search
              size={16}
              className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground"
            />
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && doSearch()}
              placeholder={
                mode === "semantic"
                  ? "Search by meaning..."
                  : mode === "keyword"
                    ? "Search by keywords..."
                    : "Search your documents..."
              }
              className="w-full rounded-md border border-input bg-background pl-9 pr-3 py-2 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
            />
          </div>
          <button
            onClick={doSearch}
            disabled={searching || !query.trim()}
            className="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
          >
            {searching ? "..." : "Search"}
          </button>
        </div>

        {/* Error */}
        {error && (
          <div className="mb-3 rounded-md bg-destructive/10 border border-destructive/20 p-3 text-sm text-destructive">
            {error}
          </div>
        )}

        {/* Results */}
        <div className="flex-1 overflow-y-auto space-y-2">
          {results.length === 0 && !searching && !error && query && (
            <p className="text-sm text-muted-foreground">No results found.</p>
          )}
          {results.map((r) => (
            <button
              key={r.chunk_id}
              onClick={() => viewDocument(r.document_id)}
              className={`w-full text-left rounded-lg border p-3 transition-colors ${
                selectedDoc === r.document_id
                  ? "border-primary bg-primary/5"
                  : "border-border bg-card hover:border-border/80"
              }`}
            >
              <p
                className="text-sm"
                dangerouslySetInnerHTML={{ __html: r.text }}
              />
              <div className="flex items-center gap-3 mt-2 text-xs text-muted-foreground">
                {r.timestamp && (
                  <span className="flex items-center gap-1">
                    <Clock size={12} />
                    {new Date(r.timestamp).toLocaleDateString()}
                  </span>
                )}
                {r.source_platform && (
                  <span className="flex items-center gap-1">
                    <FileText size={12} />
                    {r.source_platform}
                  </span>
                )}
                <span>Score: {r.score.toFixed(3)}</span>
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Document viewer */}
      {selectedDoc && (
        <div className="w-96 border-l border-border p-4 overflow-y-auto">
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-sm font-medium">Document</h3>
            <button
              onClick={() => {
                setSelectedDoc(null);
                setDocText(null);
              }}
              className="text-xs text-muted-foreground hover:text-foreground"
            >
              Close
            </button>
          </div>
          <div className="text-sm whitespace-pre-wrap text-muted-foreground">
            {docText ?? "Loading..."}
          </div>
        </div>
      )}
    </div>
  );
}

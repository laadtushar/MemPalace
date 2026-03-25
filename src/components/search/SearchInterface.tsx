import { useState } from "react";
import { commands, type SearchResult } from "@/lib/tauri";
import { Search, Clock, FileText } from "lucide-react";

export function SearchInterface() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [searching, setSearching] = useState(false);
  const [selectedDoc, setSelectedDoc] = useState<string | null>(null);
  const [docText, setDocText] = useState<string | null>(null);

  const doSearch = async () => {
    if (!query.trim()) return;
    setSearching(true);
    setSelectedDoc(null);
    setDocText(null);
    try {
      const res = await commands.keywordSearch(query, 20);
      setResults(res);
    } catch (_e) {
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
              placeholder="Search your documents..."
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

        {/* Results */}
        <div className="flex-1 overflow-y-auto space-y-2">
          {results.length === 0 && !searching && query && (
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
                <span>Score: {r.score.toFixed(2)}</span>
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

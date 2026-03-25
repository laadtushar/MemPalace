import { useEffect, useState } from "react";
import { commands, type MemoryFactResponse } from "@/lib/tauri";
import { Brain, Trash2, Lightbulb } from "lucide-react";

export function InsightFeed() {
  const [facts, setFacts] = useState<MemoryFactResponse[]>([]);
  const [loading, setLoading] = useState(true);
  const [filter, setFilter] = useState<string | undefined>(undefined);

  const loadFacts = async () => {
    setLoading(true);
    try {
      const result = await commands.getMemoryFacts(filter);
      setFacts(result);
    } catch (_e) {
      setFacts([]);
    }
    setLoading(false);
  };

  useEffect(() => {
    loadFacts();
  }, [filter]);

  const deleteFact = async (id: string) => {
    try {
      await commands.deleteMemoryFact(id);
      setFacts((prev) => prev.filter((f) => f.id !== id));
    } catch (_e) {
      /* ignore */
    }
  };

  const categories = ["belief", "preference", "fact", "self_description"];

  if (loading) {
    return (
      <div className="flex h-full items-center justify-center text-muted-foreground">
        Loading...
      </div>
    );
  }

  return (
    <div className="p-6 max-w-2xl mx-auto space-y-6 h-full overflow-y-auto">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold flex items-center gap-2">
          <Lightbulb size={24} /> Insights
        </h1>
      </div>

      {/* Category filter */}
      <div className="flex gap-2 flex-wrap">
        <button
          onClick={() => setFilter(undefined)}
          className={`rounded-md px-3 py-1 text-sm transition-colors ${
            !filter
              ? "bg-primary text-primary-foreground"
              : "bg-secondary text-muted-foreground hover:text-foreground"
          }`}
        >
          All
        </button>
        {categories.map((cat) => (
          <button
            key={cat}
            onClick={() => setFilter(cat)}
            className={`rounded-md px-3 py-1 text-sm capitalize transition-colors ${
              filter === cat
                ? "bg-primary text-primary-foreground"
                : "bg-secondary text-muted-foreground hover:text-foreground"
            }`}
          >
            {cat.replace("_", " ")}
          </button>
        ))}
      </div>

      {facts.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-12 space-y-3">
          <Brain size={48} className="text-muted-foreground/50" />
          <p className="text-muted-foreground text-sm">
            No memory facts yet. Import data and run analysis to generate
            insights.
          </p>
        </div>
      ) : (
        <div className="space-y-3">
          {facts.map((fact) => (
            <div
              key={fact.id}
              className="rounded-lg border border-border bg-card p-4 space-y-2"
            >
              <div className="flex items-start justify-between gap-3">
                <p className="text-sm">{fact.fact_text}</p>
                <button
                  onClick={() => deleteFact(fact.id)}
                  className="text-muted-foreground hover:text-destructive shrink-0"
                  title="Forget this fact"
                >
                  <Trash2 size={14} />
                </button>
              </div>
              <div className="flex items-center gap-3 text-xs text-muted-foreground">
                <span className="capitalize rounded bg-secondary px-1.5 py-0.5">
                  {fact.category.replace("_", " ")}
                </span>
                <span>
                  Confidence: {(fact.confidence * 100).toFixed(0)}%
                </span>
                <span>
                  {new Date(fact.first_seen).toLocaleDateString()}
                </span>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

import { useAppStore } from "@/stores/app-store";
import { CheckCircle, Loader2, X, AlertCircle } from "lucide-react";

const STAGE_LABELS: Record<string, string> = {
  scanning: "Scanning",
  parsing: "Parsing",
  dedup: "Deduplication",
  normalize: "Normalizing",
  storing: "Storing",
  embedding: "Embedding",
  sweep: "Sweep",
  analysis: "Analyzing",
  "analysis-complete": "Analysis Complete",
  complete: "Complete",
};

export function ImportProgressBanner() {
  const bg = useAppStore((s) => s.backgroundImport);
  const clear = useAppStore((s) => s.setBackgroundImport);

  if (!bg) return null;

  // Completed successfully — show brief summary then auto-dismiss
  if (!bg.running && bg.summary && !bg.error) {
    return (
      <div className="border-t border-border bg-card px-4 py-2 flex items-center justify-between text-sm animate-in slide-in-from-bottom-2">
        <div className="flex items-center gap-2 text-green-400">
          <CheckCircle size={14} />
          <span>
            Import complete: {bg.summary.documents_imported} documents,{" "}
            {bg.summary.chunks_created} chunks
            {bg.summary.duration_ms > 0 &&
              ` in ${(bg.summary.duration_ms / 1000).toFixed(1)}s`}
          </span>
        </div>
        <button
          onClick={() => clear(null)}
          className="text-muted-foreground hover:text-foreground"
        >
          <X size={14} />
        </button>
      </div>
    );
  }

  // Error
  if (!bg.running && bg.error) {
    return (
      <div className="border-t border-border bg-card px-4 py-2 flex items-center justify-between text-sm">
        <div className="flex items-center gap-2 text-destructive">
          <AlertCircle size={14} />
          <span>Import failed: {bg.error}</span>
        </div>
        <button
          onClick={() => clear(null)}
          className="text-muted-foreground hover:text-foreground"
        >
          <X size={14} />
        </button>
      </div>
    );
  }

  // Running — show progress
  const progress = bg.progress;
  const pct =
    progress && progress.total > 0
      ? Math.round((progress.current / progress.total) * 100)
      : null;

  return (
    <div className="border-t border-border bg-card px-4 py-2 flex items-center gap-3 text-sm">
      <Loader2 size={14} className="animate-spin text-primary shrink-0" />
      <span className="text-muted-foreground shrink-0">
        Importing {bg.sourceName}...
      </span>
      {progress && (
        <>
          <span className="text-xs text-muted-foreground shrink-0">
            {STAGE_LABELS[progress.stage] ?? progress.stage}
          </span>
          <div className="flex-1 h-1.5 rounded-full bg-secondary overflow-hidden max-w-xs">
            {pct !== null ? (
              <div
                className="h-full bg-primary rounded-full transition-all duration-300"
                style={{ width: `${pct}%` }}
              />
            ) : (
              <div className="h-full w-1/3 bg-primary/60 rounded-full animate-pulse" />
            )}
          </div>
          {progress.total > 0 && (
            <span className="text-xs font-mono text-muted-foreground shrink-0">
              {progress.current}/{progress.total}
            </span>
          )}
        </>
      )}
    </div>
  );
}

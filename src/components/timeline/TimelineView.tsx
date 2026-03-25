import { useEffect, useState } from "react";
import { commands, type TimelineDataResponse } from "@/lib/tauri";
import { BarChart3 } from "lucide-react";

export function TimelineView() {
  const [data, setData] = useState<TimelineDataResponse | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    commands
      .getTimelineData()
      .then(setData)
      .catch(() => setData(null))
      .finally(() => setLoading(false));
  }, []);

  if (loading) {
    return (
      <div className="flex h-full items-center justify-center text-muted-foreground">
        Loading timeline...
      </div>
    );
  }

  if (!data || data.total_documents === 0) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center space-y-3">
          <BarChart3 size={48} className="mx-auto text-muted-foreground/50" />
          <h2 className="text-xl font-semibold">No data yet</h2>
          <p className="text-sm text-muted-foreground max-w-sm">
            Import your journals, notes, or documents to see your personal
            timeline here.
          </p>
        </div>
      </div>
    );
  }

  const maxCount = Math.max(...data.months.map((m) => m.document_count));

  return (
    <div className="p-6 space-y-6 h-full overflow-y-auto">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold">Timeline</h1>
        <div className="text-sm text-muted-foreground">
          {data.total_documents} documents
          {data.date_range && (
            <span>
              {" "}
              · {new Date(data.date_range.start).toLocaleDateString()} —{" "}
              {new Date(data.date_range.end).toLocaleDateString()}
            </span>
          )}
        </div>
      </div>

      {/* Document activity chart (simple bar chart) */}
      <div className="rounded-lg border border-border bg-card p-4">
        <h3 className="text-sm font-medium mb-4">Document Activity by Month</h3>
        <div className="flex items-end gap-1 h-40">
          {data.months.map((m) => {
            const height = maxCount > 0 ? (m.document_count / maxCount) * 100 : 0;
            return (
              <div
                key={m.month}
                className="flex-1 flex flex-col items-center justify-end group"
              >
                <div className="relative w-full">
                  <div
                    className="w-full bg-primary/80 rounded-t transition-all group-hover:bg-primary"
                    style={{ height: `${Math.max(height, 2)}%`, minHeight: "2px" }}
                  />
                  <div className="absolute -top-6 left-1/2 -translate-x-1/2 hidden group-hover:block bg-popover text-popover-foreground text-xs rounded px-1.5 py-0.5 whitespace-nowrap border border-border">
                    {m.month}: {m.document_count}
                  </div>
                </div>
              </div>
            );
          })}
        </div>
        <div className="flex justify-between mt-2 text-[10px] text-muted-foreground">
          <span>{data.months[0]?.month}</span>
          <span>{data.months[data.months.length - 1]?.month}</span>
        </div>
      </div>

      {/* Monthly breakdown table */}
      <div className="rounded-lg border border-border bg-card overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-border">
              <th className="text-left px-4 py-2 font-medium text-muted-foreground">
                Month
              </th>
              <th className="text-right px-4 py-2 font-medium text-muted-foreground">
                Documents
              </th>
            </tr>
          </thead>
          <tbody>
            {data.months.map((m) => (
              <tr key={m.month} className="border-b border-border/50 last:border-0">
                <td className="px-4 py-2">{m.month}</td>
                <td className="px-4 py-2 text-right tabular-nums">
                  {m.document_count}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

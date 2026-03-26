import { useEffect, useState } from "react";
import { X, User, MapPin, Building, Lightbulb, Link, Clock, Hash, Loader2 } from "lucide-react";
import { commands, type EntityResponse, type EntityGraphResponse, type MemoryFactResponse } from "@/lib/tauri";

const typeConfig: Record<string, { color: string; badge: string; Icon: typeof User }> = {
  person: { color: "#3b82f6", badge: "bg-blue-500/10 text-blue-400 border-blue-500/20", Icon: User },
  place: { color: "#22c55e", badge: "bg-green-500/10 text-green-400 border-green-500/20", Icon: MapPin },
  organization: { color: "#a855f7", badge: "bg-purple-500/10 text-purple-400 border-purple-500/20", Icon: Building },
  concept: { color: "#f59e0b", badge: "bg-amber-500/10 text-amber-400 border-amber-500/20", Icon: Lightbulb },
  topic: { color: "#f59e0b", badge: "bg-amber-500/10 text-amber-400 border-amber-500/20", Icon: Lightbulb },
};

const defaultConfig = { color: "#71717a", badge: "bg-secondary border-border", Icon: Hash };

interface NodeDetailPanelProps {
  entity: EntityResponse;
  onClose: () => void;
}

export function NodeDetailPanel({ entity, onClose }: NodeDetailPanelProps) {
  const [connections, setConnections] = useState<EntityGraphResponse | null>(null);
  const [relatedFacts, setRelatedFacts] = useState<MemoryFactResponse[]>([]);
  const [loading, setLoading] = useState(true);

  const config = typeConfig[entity.entity_type] ?? defaultConfig;
  const TypeIcon = config.Icon;

  useEffect(() => {
    setLoading(true);
    Promise.all([
      commands.getEntityGraph(entity.id, 1).catch(() => null),
      commands.getMemoryFacts().catch(() => []),
    ]).then(([graph, facts]) => {
      setConnections(graph);
      // Find facts mentioning this entity's name
      const related = facts.filter((f) =>
        f.fact_text.toLowerCase().includes(entity.name.toLowerCase()),
      );
      setRelatedFacts(related.slice(0, 5));
      setLoading(false);
    });
  }, [entity.id, entity.name]);

  const connectedEntities = connections?.entities.filter((e) => e.id !== entity.id) ?? [];
  const relationships = connections?.relationships ?? [];

  return (
    <div className="w-80 border-l border-border bg-card/95 backdrop-blur-sm flex flex-col overflow-y-auto">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-border">
        <span className="text-xs font-medium text-muted-foreground uppercase tracking-wider">
          Entity Details
        </span>
        <button
          onClick={onClose}
          className="h-6 w-6 flex items-center justify-center rounded hover:bg-accent text-muted-foreground hover:text-foreground transition-colors"
        >
          <X size={14} />
        </button>
      </div>

      <div className="p-4 space-y-5">
        {/* Entity header */}
        <div className="flex items-start gap-3">
          <div
            className="rounded-xl p-2.5 shrink-0"
            style={{ backgroundColor: config.color + "20" }}
          >
            <TypeIcon size={20} style={{ color: config.color }} />
          </div>
          <div>
            <h2 className="text-lg font-bold leading-tight">{entity.name}</h2>
            <span
              className={`mt-1 inline-flex items-center rounded-full border px-2 py-0.5 text-[10px] font-medium ${config.badge}`}
            >
              {entity.entity_type}
            </span>
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-3 gap-2">
          <div className="rounded-lg bg-secondary/50 px-2.5 py-2 text-center">
            <p className="text-lg font-bold tabular-nums">{entity.mention_count}</p>
            <p className="text-[9px] text-muted-foreground">Mentions</p>
          </div>
          <div className="rounded-lg bg-secondary/50 px-2.5 py-2 text-center">
            <p className="text-lg font-bold tabular-nums">{connectedEntities.length}</p>
            <p className="text-[9px] text-muted-foreground">Connected</p>
          </div>
          <div className="rounded-lg bg-secondary/50 px-2.5 py-2 text-center">
            <p className="text-lg font-bold tabular-nums">{relatedFacts.length}</p>
            <p className="text-[9px] text-muted-foreground">Facts</p>
          </div>
        </div>

        {/* Timeline */}
        {(entity.first_seen || entity.last_seen) && (
          <div className="space-y-1.5">
            <h3 className="text-xs font-medium text-muted-foreground flex items-center gap-1.5">
              <Clock size={10} /> Timeline
            </h3>
            <div className="flex items-center gap-2 text-sm">
              {entity.first_seen && (
                <span>{new Date(entity.first_seen).toLocaleDateString(undefined, { year: "numeric", month: "short" })}</span>
              )}
              {entity.first_seen && entity.last_seen && <span className="text-muted-foreground">→</span>}
              {entity.last_seen && (
                <span>{new Date(entity.last_seen).toLocaleDateString(undefined, { year: "numeric", month: "short" })}</span>
              )}
            </div>
          </div>
        )}

        {/* Connected entities */}
        {loading ? (
          <div className="flex items-center gap-2 text-sm text-muted-foreground">
            <Loader2 size={12} className="animate-spin" /> Loading connections...
          </div>
        ) : (
          <>
            {connectedEntities.length > 0 && (
              <div className="space-y-2">
                <h3 className="text-xs font-medium text-muted-foreground flex items-center gap-1.5">
                  <Link size={10} /> Connections ({connectedEntities.length})
                </h3>
                <div className="space-y-1">
                  {connectedEntities.slice(0, 8).map((e) => {
                    const rel = relationships.find(
                      (r) =>
                        (r.source_entity_id === entity.id && r.target_entity_id === e.id) ||
                        (r.target_entity_id === entity.id && r.source_entity_id === e.id),
                    );
                    const eConfig = typeConfig[e.entity_type] ?? defaultConfig;
                    const EIcon = eConfig.Icon;
                    return (
                      <div
                        key={e.id}
                        className="flex items-center gap-2 rounded-md px-2 py-1.5 bg-secondary/30 hover:bg-secondary/60 transition-colors"
                      >
                        <EIcon size={12} style={{ color: eConfig.color }} />
                        <span className="text-sm truncate flex-1">{e.name}</span>
                        {rel && (
                          <span className="text-[9px] text-muted-foreground shrink-0">
                            {rel.rel_type}
                          </span>
                        )}
                      </div>
                    );
                  })}
                  {connectedEntities.length > 8 && (
                    <p className="text-[10px] text-muted-foreground pl-2">
                      +{connectedEntities.length - 8} more
                    </p>
                  )}
                </div>
              </div>
            )}

            {/* Related memory facts */}
            {relatedFacts.length > 0 && (
              <div className="space-y-2">
                <h3 className="text-xs font-medium text-muted-foreground flex items-center gap-1.5">
                  <Lightbulb size={10} /> Related Insights ({relatedFacts.length})
                </h3>
                <div className="space-y-1.5">
                  {relatedFacts.map((fact) => (
                    <div
                      key={fact.id}
                      className="rounded-md bg-secondary/30 px-2.5 py-2 text-xs leading-relaxed"
                    >
                      <p className="line-clamp-3">{fact.fact_text}</p>
                      <div className="flex items-center gap-2 mt-1 text-[9px] text-muted-foreground">
                        <span className="capitalize">{fact.category}</span>
                        <span>{Math.round(fact.confidence * 100)}%</span>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}

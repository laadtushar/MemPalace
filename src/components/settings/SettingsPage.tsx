import { useEffect, useState } from "react";
import { commands, type OllamaStatus, type AppStats } from "@/lib/tauri";
import { Cpu, Database, CheckCircle, XCircle, RefreshCw } from "lucide-react";

export function SettingsPage() {
  const [ollamaStatus, setOllamaStatus] = useState<OllamaStatus | null>(null);
  const [appStats, setAppStats] = useState<AppStats | null>(null);
  const [testing, setTesting] = useState(false);

  const testConnection = async () => {
    setTesting(true);
    try {
      const status = await commands.testOllamaConnection();
      setOllamaStatus(status);
    } catch (e) {
      setOllamaStatus({ connected: false, models: [] });
    }
    setTesting(false);
  };

  const loadStats = async () => {
    try {
      const stats = await commands.getAppStats();
      setAppStats(stats);
    } catch (_e) {
      /* ignore */
    }
  };

  useEffect(() => {
    testConnection();
    loadStats();
  }, []);

  return (
    <div className="p-6 max-w-2xl mx-auto space-y-8">
      <h1 className="text-2xl font-semibold">Settings</h1>

      {/* LLM Provider */}
      <section className="space-y-4">
        <h2 className="text-lg font-medium flex items-center gap-2">
          <Cpu size={20} /> LLM Provider
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
                <RefreshCw size={14} className={testing ? "animate-spin" : ""} />
              </button>
            </div>
          </div>

          {ollamaStatus?.connected && ollamaStatus.models.length > 0 && (
            <div>
              <p className="text-sm text-muted-foreground mb-1">
                Available models:
              </p>
              <div className="flex flex-wrap gap-1.5">
                {ollamaStatus.models.map((model) => (
                  <span
                    key={model}
                    className="rounded-md bg-secondary px-2 py-0.5 text-xs"
                  >
                    {model}
                  </span>
                ))}
              </div>
            </div>
          )}

          {ollamaStatus && !ollamaStatus.connected && (
            <p className="text-sm text-muted-foreground">
              Start Ollama to use local models. Visit{" "}
              <span className="text-primary">ollama.com</span> to install.
            </p>
          )}
        </div>
      </section>

      {/* Data Stats */}
      <section className="space-y-4">
        <h2 className="text-lg font-medium flex items-center gap-2">
          <Database size={20} /> Data Overview
        </h2>
        <div className="rounded-lg border border-border bg-card p-4">
          {appStats ? (
            <div className="grid grid-cols-2 gap-4">
              <div>
                <p className="text-2xl font-bold">{appStats.total_documents}</p>
                <p className="text-sm text-muted-foreground">Documents</p>
              </div>
              <div>
                <p className="text-2xl font-bold">
                  {appStats.total_memory_facts}
                </p>
                <p className="text-sm text-muted-foreground">Memory Facts</p>
              </div>
              {appStats.date_range && (
                <div className="col-span-2">
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

      {/* Privacy */}
      <section className="space-y-4">
        <h2 className="text-lg font-medium">Privacy</h2>
        <div className="rounded-lg border border-border bg-card p-4 space-y-2">
          <p className="text-sm text-muted-foreground">
            All data is stored locally on your device. No telemetry, no
            analytics, no cloud sync. LLM processing happens on-device via
            Ollama by default.
          </p>
        </div>
      </section>
    </div>
  );
}

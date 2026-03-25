import { Sidebar } from "./Sidebar";
import { useAppStore } from "@/stores/app-store";
import { TimelineView } from "@/components/timeline/TimelineView";
import { SearchInterface } from "@/components/search/SearchInterface";
import { InsightFeed } from "@/components/insights/InsightFeed";
import { ImportWizard } from "@/components/import/ImportWizard";
import { SettingsPage } from "@/components/settings/SettingsPage";

function MemoryPlaceholder() {
  return (
    <div className="flex h-full items-center justify-center text-muted-foreground">
      <div className="text-center">
        <h2 className="text-2xl font-semibold text-foreground mb-2">Memory Browser</h2>
        <p className="text-sm">Browse and manage extracted memory facts via the Insights tab.</p>
      </div>
    </div>
  );
}

const viewComponents: Record<string, React.ReactNode> = {
  timeline: <TimelineView />,
  search: <SearchInterface />,
  insights: <InsightFeed />,
  import: <ImportWizard />,
  memory: <MemoryPlaceholder />,
  settings: <SettingsPage />,
};

export function AppShell() {
  const currentView = useAppStore((s) => s.currentView);

  return (
    <div className="flex h-screen w-screen bg-background">
      <Sidebar />
      <main className="flex-1 overflow-hidden">
        {viewComponents[currentView]}
      </main>
    </div>
  );
}

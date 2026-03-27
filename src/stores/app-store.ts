import { create } from "zustand";
import type { ImportProgress, ImportSummary } from "@/lib/tauri";

export type View =
  | "timeline"
  | "activity"
  | "search"
  | "ask"
  | "insights"
  | "import"
  | "memory"
  | "entities"
  | "graph"
  | "evolution"
  | "logs"
  | "settings";

export type Theme = "dark" | "light";

export interface BackgroundImport {
  sourceName: string;
  progress: ImportProgress | null;
  summary: ImportSummary | null;
  error: string | null;
  running: boolean;
}

interface AppState {
  currentView: View;
  theme: Theme;
  isUnlocked: boolean;
  isOnboarded: boolean;
  backgroundImport: BackgroundImport | null;
  setView: (view: View) => void;
  toggleTheme: () => void;
  setUnlocked: (unlocked: boolean) => void;
  setOnboarded: (onboarded: boolean) => void;
  setBackgroundImport: (bg: BackgroundImport | null) => void;
  updateBackgroundImport: (update: Partial<BackgroundImport>) => void;
}

export const useAppStore = create<AppState>((set, get) => ({
  currentView: "timeline",
  theme: (localStorage.getItem("mp-theme") as Theme) || "dark",
  isUnlocked: false,
  isOnboarded: true, // assume onboarded until check completes
  backgroundImport: null,
  setView: (view) => set({ currentView: view }),
  setUnlocked: (unlocked) => set({ isUnlocked: unlocked }),
  setOnboarded: (onboarded) => set({ isOnboarded: onboarded }),
  setBackgroundImport: (bg) => set({ backgroundImport: bg }),
  updateBackgroundImport: (update) => {
    const current = get().backgroundImport;
    if (current) set({ backgroundImport: { ...current, ...update } });
  },
  toggleTheme: () => {
    const next = get().theme === "dark" ? "light" : "dark";
    localStorage.setItem("mp-theme", next);
    document.documentElement.classList.toggle("light", next === "light");
    set({ theme: next });
  },
}));

// Apply saved theme on load
const saved = localStorage.getItem("mp-theme");
if (saved === "light") {
  document.documentElement.classList.add("light");
}

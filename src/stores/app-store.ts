import { create } from "zustand";

export type View =
  | "timeline"
  | "search"
  | "insights"
  | "import"
  | "memory"
  | "settings";

interface AppState {
  currentView: View;
  setView: (view: View) => void;
}

export const useAppStore = create<AppState>((set) => ({
  currentView: "timeline",
  setView: (view) => set({ currentView: view }),
}));

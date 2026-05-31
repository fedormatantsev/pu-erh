import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";

import { rootId } from "./ipc";
import type { ViewMode } from "./types";

// All Block View state lives in the shell: the current selected block and the
// active view mode. The TreeView derives its columns (parent, siblings,
// children) from the current selected block, so no per-node expand/collapse
// state is held. Presentational components receive state via props/callbacks
// and never read it themselves.

type ShellState = {
  currentBlockId: string | null;
  rootError: string | null;
  viewMode: ViewMode;
  selectBlock: (id: string) => void;
  setViewMode: (mode: ViewMode) => void;
};

const ShellContext = createContext<ShellState | null>(null);

export function ShellProvider({ children }: { children: ReactNode }) {
  const [currentBlockId, setCurrentBlockId] = useState<string | null>(null);
  const [rootError, setRootError] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<ViewMode>("block");

  // The current selected block resolves to the root block on open. A fresh,
  // never-saved knowledge base has no root and root_id errors; surface it.
  useEffect(() => {
    let cancelled = false;
    rootId().then(
      (id) => {
        if (!cancelled) setCurrentBlockId(id);
      },
      (error) => {
        if (!cancelled) setRootError(String(error));
      },
    );
    return () => {
      cancelled = true;
    };
  }, []);

  const selectBlock = useCallback((id: string) => {
    setCurrentBlockId(id);
  }, []);

  const value = useMemo<ShellState>(
    () => ({
      currentBlockId,
      rootError,
      viewMode,
      selectBlock,
      setViewMode,
    }),
    [currentBlockId, rootError, viewMode, selectBlock],
  );

  return <ShellContext.Provider value={value}>{children}</ShellContext.Provider>;
}

export function useShell(): ShellState {
  const ctx = useContext(ShellContext);
  if (!ctx) {
    throw new Error("useShell must be used within a ShellProvider");
  }
  return ctx;
}

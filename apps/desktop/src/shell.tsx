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

// All Block View state lives in the shell: the current selected block, the
// active view mode, and which tree nodes are expanded. Presentational
// components receive this via props/callbacks and never read it themselves.

type ShellState = {
  currentBlockId: string | null;
  rootError: string | null;
  viewMode: ViewMode;
  expandedIds: ReadonlySet<string>;
  selectBlock: (id: string) => void;
  setViewMode: (mode: ViewMode) => void;
  toggleExpanded: (id: string) => void;
};

const ShellContext = createContext<ShellState | null>(null);

export function ShellProvider({ children }: { children: ReactNode }) {
  const [currentBlockId, setCurrentBlockId] = useState<string | null>(null);
  const [rootError, setRootError] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<ViewMode>("block");
  const [expandedIds, setExpandedIds] = useState<ReadonlySet<string>>(
    () => new Set(),
  );

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

  const toggleExpanded = useCallback((id: string) => {
    setExpandedIds((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  }, []);

  const value = useMemo<ShellState>(
    () => ({
      currentBlockId,
      rootError,
      viewMode,
      expandedIds,
      selectBlock,
      setViewMode,
      toggleExpanded,
    }),
    [currentBlockId, rootError, viewMode, expandedIds, selectBlock, toggleExpanded],
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

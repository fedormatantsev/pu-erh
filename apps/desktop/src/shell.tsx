import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";

import { createBlock, rootId } from "./ipc";
import type { ViewMode } from "./types";

// All Block View state lives in the shell: the current selected block and the
// active view mode. The TreeView derives its columns (parent, siblings,
// children) from the current selected block, so no per-node expand/collapse
// state is held. Presentational components receive state via props/callbacks
// and never read it themselves.

type ShellState = {
  currentBlockId: string | null;
  rootError: string | null;
  actionError: string | null;
  viewMode: ViewMode;
  // Bumped after a successful mutation so views derived from the current block
  // (e.g. the children column) re-read without changing the selection.
  refreshToken: number;
  selectBlock: (id: string) => void;
  setViewMode: (mode: ViewMode) => void;
  createChild: () => void;
};

const ShellContext = createContext<ShellState | null>(null);

export function ShellProvider({ children }: { children: ReactNode }) {
  const [currentBlockId, setCurrentBlockId] = useState<string | null>(null);
  const [rootError, setRootError] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<ViewMode>("block");
  const [refreshToken, setRefreshToken] = useState(0);

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

  // Append a child of the current selected block. This does not change the
  // current selected block and does not save (consistent with the no-save
  // policy); on success the refresh token is bumped so the children column
  // re-reads. Mutation errors are surfaced as the CoreError-derived value.
  const createChild = useCallback(() => {
    if (!currentBlockId) return;
    setActionError(null);
    createBlock(currentBlockId).then(
      () => setRefreshToken((n) => n + 1),
      (error) => setActionError(String(error)),
    );
  }, [currentBlockId]);

  const value = useMemo<ShellState>(
    () => ({
      currentBlockId,
      rootError,
      actionError,
      viewMode,
      refreshToken,
      selectBlock,
      setViewMode,
      createChild,
    }),
    [
      currentBlockId,
      rootError,
      actionError,
      viewMode,
      refreshToken,
      selectBlock,
      createChild,
    ],
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

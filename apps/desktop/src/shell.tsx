import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useReducer,
  useState,
  type ReactNode,
} from "react";

import { createBlock, rootId } from "./ipc";
import type { ViewMode } from "./types";

// All Block View state lives in the shell: the current selected block, the
// active view mode, and the navigation history stacks. Presentational
// components receive state via props/callbacks and never read it themselves.

// Navigation state is managed by a pure reducer so all transitions are atomic
// and safe under React StrictMode's double-invocation of updaters.
type NavState = {
  currentBlockId: string | null;
  backStack: string[];
  forwardStack: string[];
};

type NavAction =
  | { type: "init"; id: string }
  | { type: "select"; id: string }
  | { type: "back" }
  | { type: "forward" };

function navReducer(state: NavState, action: NavAction): NavState {
  switch (action.type) {
    case "init":
      // Root resolution on open — no history entry created.
      return { currentBlockId: action.id, backStack: [], forwardStack: [] };
    case "select": {
      if (state.currentBlockId === null) {
        return { ...state, currentBlockId: action.id };
      }
      return {
        currentBlockId: action.id,
        backStack: [...state.backStack, state.currentBlockId],
        forwardStack: [],
      };
    }
    case "back": {
      if (state.backStack.length === 0) return state;
      const target = state.backStack[state.backStack.length - 1];
      return {
        currentBlockId: target,
        backStack: state.backStack.slice(0, -1),
        forwardStack:
          state.currentBlockId !== null
            ? [state.currentBlockId, ...state.forwardStack]
            : state.forwardStack,
      };
    }
    case "forward": {
      if (state.forwardStack.length === 0) return state;
      const target = state.forwardStack[0];
      return {
        currentBlockId: target,
        backStack:
          state.currentBlockId !== null
            ? [...state.backStack, state.currentBlockId]
            : state.backStack,
        forwardStack: state.forwardStack.slice(1),
      };
    }
  }
}

type ShellState = {
  currentBlockId: string | null;
  rootError: string | null;
  actionError: string | null;
  viewMode: ViewMode;
  // Bumped after a successful mutation so views derived from the current block
  // (e.g. the children column) re-read without changing the selection.
  refreshToken: number;
  canGoBack: boolean;
  canGoForward: boolean;
  selectBlock: (id: string) => void;
  navigateBack: () => void;
  navigateForward: () => void;
  setViewMode: (mode: ViewMode) => void;
  createChild: () => void;
};

const ShellContext = createContext<ShellState | null>(null);

export function ShellProvider({ children }: { children: ReactNode }) {
  const [nav, dispatch] = useReducer(navReducer, {
    currentBlockId: null,
    backStack: [],
    forwardStack: [],
  });
  const [rootError, setRootError] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<ViewMode>("block");
  const [refreshToken, setRefreshToken] = useState(0);

  // The current selected block resolves to the root block on open. Uses the
  // "init" action so the root resolution does not create a history entry.
  useEffect(() => {
    let cancelled = false;
    rootId().then(
      (id) => {
        if (!cancelled) dispatch({ type: "init", id });
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
    dispatch({ type: "select", id });
  }, []);

  const navigateBack = useCallback(() => {
    dispatch({ type: "back" });
  }, []);

  const navigateForward = useCallback(() => {
    dispatch({ type: "forward" });
  }, []);

  // Append a child of the current selected block. This does not change the
  // current selected block and does not save (consistent with the no-save
  // policy); on success the refresh token is bumped so the children column
  // re-reads. Mutation errors are surfaced as the CoreError-derived value.
  const createChild = useCallback(() => {
    if (!nav.currentBlockId) return;
    setActionError(null);
    createBlock(nav.currentBlockId).then(
      () => setRefreshToken((n) => n + 1),
      (error) => setActionError(String(error)),
    );
  }, [nav.currentBlockId]);

  const value = useMemo<ShellState>(
    () => ({
      currentBlockId: nav.currentBlockId,
      rootError,
      actionError,
      viewMode,
      refreshToken,
      canGoBack: nav.backStack.length > 0,
      canGoForward: nav.forwardStack.length > 0,
      selectBlock,
      navigateBack,
      navigateForward,
      setViewMode,
      createChild,
    }),
    [
      nav,
      rootError,
      actionError,
      viewMode,
      refreshToken,
      selectBlock,
      navigateBack,
      navigateForward,
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

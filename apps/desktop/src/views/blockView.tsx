import {
  useEffect,
  useRef,
  useState,
  type KeyboardEvent,
  type ReactElement,
} from "react";

import { Text, TreeCell, TreeColumn, TreeColumns } from "@pu-erh/ui";

import { getBlock, getChildren, getParent } from "../ipc";
import { useShell } from "../shell";
import type { BlockDto, PropertyValue } from "../types";
import { DocumentView } from "./documentView";
import { resolveInlineView } from "./inlineView";

// A primary renderer represents the current selected block as the main Block
// View content. Selected by the block's `display` property via the router below.
export type BlockView = (props: { blockId: string }) => ReactElement;

// The three columns derived from the current selected block: its parent (left),
// the current block together with its siblings (center), and its children
// (right). Siblings are the parent's children, or just the current block when it
// is the root and therefore has no parent.
type Columns = {
  parent: BlockDto | null;
  siblings: BlockDto[];
  children: BlockDto[];
};

// The default Block View: a three-column TreeView (parent | current + siblings |
// children). Activating a cell or pressing an arrow key changes the current
// selected block and re-centers the columns (the default view's selection
// policy). Columns derive entirely from the current selected block.
export function DefaultBlockView({ blockId }: { blockId: string }) {
  const { selectBlock, refreshToken } = useShell();
  const [columns, setColumns] = useState<Columns | null>(null);
  const [error, setError] = useState<string | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    let cancelled = false;
    setColumns(null);
    setError(null);
    (async () => {
      try {
        const parent = await getParent(blockId);
        // Siblings come from the parent's children (which includes the current
        // block); at the root there is no parent, so the current block stands
        // alone in the center column.
        const siblings = parent
          ? await getChildren(parent.id)
          : [await getBlock(blockId)];
        const children = await getChildren(blockId);
        if (!cancelled) setColumns({ parent, siblings, children });
      } catch (err) {
        if (!cancelled) setError(String(err));
      }
    })();
    return () => {
      cancelled = true;
    };
    // refreshToken re-reads the columns (e.g. after a child is created) without
    // changing the current selected block.
  }, [blockId, refreshToken]);

  // Focus the container on each selection so arrow-key navigation works without
  // requiring a click first (and after a click re-centers and remounts cells).
  useEffect(() => {
    containerRef.current?.focus();
  }, [blockId, columns]);

  if (error) {
    return <Text>{error}</Text>;
  }
  if (columns === null) {
    return <></>;
  }

  const { parent, siblings, children } = columns;
  const currentIndex = siblings.findIndex((block) => block.id === blockId);

  // Arrow keys move the current selected block; each is a no-op when the target
  // does not exist (← at root, → with no children, ↑/↓ past the sibling ends).
  function onKeyDown(event: KeyboardEvent<HTMLDivElement>) {
    let target: string | null = null;
    switch (event.key) {
      case "ArrowLeft":
        target = parent ? parent.id : null;
        break;
      case "ArrowRight":
        target = children.length > 0 ? children[0].id : null;
        break;
      case "ArrowUp":
        target = currentIndex > 0 ? siblings[currentIndex - 1].id : null;
        break;
      case "ArrowDown":
        target =
          currentIndex >= 0 && currentIndex < siblings.length - 1
            ? siblings[currentIndex + 1].id
            : null;
        break;
      default:
        return;
    }
    if (target !== null) {
      event.preventDefault();
      selectBlock(target);
    }
  }

  function cell(block: BlockDto, current: boolean) {
    const { View } = resolveInlineView(block.properties.display);
    return (
      <TreeCell key={block.id} current={current}>
        <View block={block} onActivate={() => selectBlock(block.id)} />
      </TreeCell>
    );
  }

  return (
    <div
      className="pu-erh-tree-view"
      ref={containerRef}
      tabIndex={0}
      onKeyDown={onKeyDown}
    >
      <TreeColumns>
        <TreeColumn>{parent ? cell(parent, false) : null}</TreeColumn>
        <TreeColumn>
          {siblings.map((block) => cell(block, block.id === blockId))}
        </TreeColumn>
        <TreeColumn>{children.map((block) => cell(block, false))}</TreeColumn>
      </TreeColumns>
    </div>
  );
}

const blockRenderers: Record<string, BlockView> = {
  tree: DefaultBlockView,
  document: DocumentView,
};

export const BLOCK_VIEW_NAMES: string[] = Object.keys(blockRenderers);

export type BlockResolution = {
  View: BlockView;
  unrecognized: string | null;
};

export function resolveBlockView(display: PropertyValue | undefined): BlockResolution {
  if (typeof display === "string" && display.length > 0) {
    const view = blockRenderers[display];
    if (view) {
      return { View: view, unrecognized: null };
    }
    return { View: DefaultBlockView, unrecognized: display };
  }
  return { View: DefaultBlockView, unrecognized: null };
}

// The primary Block View for the current selected block: reads the block's
// `display` property and dispatches to the matching registered renderer
// (defaulting to the TreeView when absent or unrecognized). This is the router
// entry point used by the View Router for block mode.
export function PrimaryBlockView({ blockId }: { blockId: string }) {
  const [display, setDisplay] = useState<PropertyValue | undefined>(undefined);
  const [loaded, setLoaded] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setLoaded(false);
    setError(null);
    getBlock(blockId).then(
      (block) => {
        if (cancelled) return;
        setDisplay(block.properties.display);
        setLoaded(true);
      },
      (err) => {
        if (!cancelled) setError(String(err));
      },
    );
    return () => {
      cancelled = true;
    };
  }, [blockId]);

  if (error) {
    return <Text>{error}</Text>;
  }
  if (!loaded) {
    return <></>;
  }
  const { View } = resolveBlockView(display);
  return <View blockId={blockId} />;
}

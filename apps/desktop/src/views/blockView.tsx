import { useEffect, useState, type ReactElement } from "react";

import { Text, TreeNode } from "@pu-erh/ui";

import { getChildren } from "../ipc";
import { useShell } from "../shell";
import type { BlockDto, PropertyValue } from "../types";
import { resolveInlineView } from "./inlineView";

// A primary renderer represents the current selected block as the main Block
// View content. Selected by the block's `display` property via the router below.
export type BlockView = (props: { blockId: string }) => ReactElement;

// One level of the tree: the children of `blockId`, each rendered inline, each
// expandable to its own children. Children load lazily when first rendered.
function TreeBranch({ blockId }: { blockId: string }) {
  const { expandedIds, toggleExpanded, selectBlock } = useShell();
  const [children, setChildren] = useState<BlockDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    getChildren(blockId).then(
      (result) => {
        if (!cancelled) setChildren(result);
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
  if (children === null) {
    return null;
  }

  return (
    <>
      {children.map((child) => {
        const { View } = resolveInlineView(child.properties.display);
        const expanded = expandedIds.has(child.id);
        return (
          <TreeNode
            key={child.id}
            hasChildren={child.has_children}
            expanded={expanded}
            onToggle={() => toggleExpanded(child.id)}
            label={<View block={child} onActivate={() => selectBlock(child.id)} />}
          >
            {expanded ? <TreeBranch blockId={child.id} /> : null}
          </TreeNode>
        );
      })}
    </>
  );
}

// The default Block View displays the current block's children as a recursive
// tree. Activating a node selects it (the default view's selection policy).
export function DefaultBlockView({ blockId }: { blockId: string }) {
  return <TreeBranch blockId={blockId} />;
}

const blockRenderers: Record<string, BlockView> = {
  tree: DefaultBlockView,
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

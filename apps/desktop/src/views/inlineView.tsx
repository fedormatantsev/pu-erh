import type { ReactElement } from "react";

import { InlineBlock } from "@pu-erh/ui";

import type { BlockDto, PropertyValue } from "../types";

// An inline renderer represents a block when it appears inside another Block
// View (e.g. as a tree node). Selected by the block's `display` property via the
// Inline View router below.
export type InlineViewProps = {
  block: BlockDto;
  onActivate?: () => void;
};

export type InlineView = (props: InlineViewProps) => ReactElement;

// The default inline view shows the block's `title` (a reserved string
// property); when absent or non-string it shows the block id as a neutral
// placeholder — no invented descriptive copy.
export function DefaultInlineView({ block, onActivate }: InlineViewProps) {
  const title = block.properties.title;
  const untitled = typeof title !== "string" || title.length === 0;
  const label = untitled ? block.id : String(title);
  return <InlineBlock label={label} onActivate={onActivate} untitled={untitled} />;
}

// Registry of inline renderers keyed by `display`. Only the default ships.
const inlineRenderers: Record<string, InlineView> = {};

export type InlineResolution = {
  View: InlineView;
  unrecognized: string | null;
};

export function resolveInlineView(display: PropertyValue | undefined): InlineResolution {
  if (typeof display === "string" && display.length > 0) {
    const view = inlineRenderers[display];
    if (view) {
      return { View: view, unrecognized: null };
    }
    return { View: DefaultInlineView, unrecognized: display };
  }
  return { View: DefaultInlineView, unrecognized: null };
}

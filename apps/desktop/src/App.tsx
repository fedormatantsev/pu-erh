import { useEffect, useState } from "react";

import { ActionBar, Stack, Text } from "@pu-erh/ui";

import { getBlock } from "./ipc";
import { ShellProvider, useShell } from "./shell";
import type { BlockDto } from "./types";
import { resolveBlockView } from "./views/blockView";
import { PropertiesView } from "./views/PropertiesView";

// Resolves the current block's `display` to a primary renderer and renders it,
// surfacing an unrecognized `display` value before falling back to the default.
function BlockViewHost({ blockId }: { blockId: string }) {
  const [block, setBlock] = useState<BlockDto | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    getBlock(blockId).then(
      (result) => {
        if (!cancelled) setBlock(result);
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
  if (!block) {
    return null;
  }

  const { View, unrecognized } = resolveBlockView(block.properties.display);
  return (
    <>
      {unrecognized ? (
        <Text>Unrecognized display: {unrecognized}</Text>
      ) : null}
      <View blockId={blockId} />
    </>
  );
}

// Exactly one of the Block View / Properties View is shown at a time. The
// action bar overlay hosts the view-mode toggle and the create-child action.
function Workspace() {
  const {
    currentBlockId,
    rootError,
    actionError,
    viewMode,
    setViewMode,
    createChild,
  } = useShell();

  if (rootError) {
    return (
      <Stack>
        <Text as="h1">pu-erh</Text>
        <Text>{rootError}</Text>
      </Stack>
    );
  }
  if (!currentBlockId) {
    return null;
  }

  return (
    <Stack>
      <ActionBar
        mode={viewMode}
        onToggleMode={() =>
          setViewMode(viewMode === "block" ? "properties" : "block")
        }
        onCreateChild={createChild}
        canCreateChild={currentBlockId != null}
      />
      {actionError ? <Text>{actionError}</Text> : null}
      {viewMode === "block" ? (
        <BlockViewHost blockId={currentBlockId} />
      ) : (
        <PropertiesView blockId={currentBlockId} />
      )}
    </Stack>
  );
}

export function App() {
  return (
    <ShellProvider>
      <Workspace />
    </ShellProvider>
  );
}

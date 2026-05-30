import { useEffect, useState } from "react";

import { Button, PropertiesPanel, Stack, Text } from "@pu-erh/ui";

import { getBlock, save, setProperty } from "../ipc";

// Settings of the current Block View. Edits `display` of the current selected
// block through the set_property mutation (held in memory), and persists via an
// explicit Save control — no auto-save.
export function PropertiesView({ blockId }: { blockId: string }) {
  const [display, setDisplay] = useState("");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    getBlock(blockId).then(
      (block) => {
        if (cancelled) return;
        const value = block.properties.display;
        setDisplay(typeof value === "string" ? value : "");
      },
      (err) => {
        if (!cancelled) setError(String(err));
      },
    );
    return () => {
      cancelled = true;
    };
  }, [blockId]);

  const applyDisplay = () => {
    setError(null);
    setProperty(blockId, "display", display).catch((err) => setError(String(err)));
  };

  const persist = () => {
    setError(null);
    save().catch((err) => setError(String(err)));
  };

  return (
    <PropertiesPanel>
      <Text as="h1">Properties</Text>
      <label className="pu-erh-properties-field">
        <Text as="span">display</Text>
        <input
          value={display}
          onChange={(event) => setDisplay(event.target.value)}
        />
      </label>
      <Stack gap="0.5rem">
        <Button onClick={applyDisplay}>Apply display</Button>
        <Button onClick={persist}>Save</Button>
      </Stack>
      {error ? <Text>{error}</Text> : null}
    </PropertiesPanel>
  );
}

import { useEffect, useRef, useState } from "react";

import { Button, PropertiesPanel, Stack, Text } from "@pu-erh/ui";

import { getBlock, save, setProperty } from "../ipc";
import { BLOCK_VIEW_NAMES } from "./blockView";

// Properties whose values are rendered in dedicated layout slots above the
// generic property list. These keys are excluded from the generic list.
const LAYOUT_SLOT_PROPERTIES = new Set(["display"]);

// Settings of the current Block View. The `display` property is rendered in its
// own dedicated slot (a dropdown with no label) above the generic property list.
// Absent or unrecognized `display` values resolve to "default" implicitly and
// are written to storage on the next save.
export function PropertiesView({ blockId }: { blockId: string }) {
  const [display, setDisplay] = useState(BLOCK_VIEW_NAMES[0]);
  const needsWrite = useRef(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    getBlock(blockId).then(
      (block) => {
        if (cancelled) return;
        const value = block.properties.display;
        const resolved =
          typeof value === "string" && BLOCK_VIEW_NAMES.includes(value)
            ? value
            : BLOCK_VIEW_NAMES[0];
        needsWrite.current = resolved !== value;
        setDisplay(resolved);
      },
      (err) => {
        if (!cancelled) setError(String(err));
      },
    );
    return () => {
      cancelled = true;
    };
  }, [blockId]);

  const onDisplayChange = (value: string) => {
    setDisplay(value);
    needsWrite.current = false;
    setError(null);
    setProperty(blockId, "display", value).catch((err) => setError(String(err)));
  };

  const persist = async () => {
    setError(null);
    try {
      if (needsWrite.current) {
        await setProperty(blockId, "display", display);
        needsWrite.current = false;
      }
      await save();
    } catch (err) {
      setError(String(err));
    }
  };

  return (
    <PropertiesPanel>
      <Text as="h1">Properties</Text>
      <select
        value={display}
        onChange={(e) => onDisplayChange(e.target.value)}
      >
        {BLOCK_VIEW_NAMES.map((name) => (
          <option key={name} value={name}>
            {name}
          </option>
        ))}
      </select>
      <Stack gap="0.5rem">
        <Button onPress={persist}>Save</Button>
      </Stack>
      {error ? <Text>{error}</Text> : null}
    </PropertiesPanel>
  );
}

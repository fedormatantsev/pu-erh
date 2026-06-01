import { useEffect, useRef, useState } from "react";

import { Button, PropertiesPanel, Stack, Text } from "@pu-erh/ui";

import { getBlock, save, setProperty } from "../ipc";
import { BLOCK_VIEW_NAMES } from "./blockView";

// Settings of the current Block View. Well-known properties (`title`, `display`)
// each have a dedicated slot rendered before any generic property list.
// `display` absent or unrecognized values resolve to the default view name and
// are written to storage on the next save.
//
// `body` is a reserved property (serialized rich text owned by the Document
// View); like other reserved keys it is never surfaced as a generic, editable
// property item. This view renders no generic properties list yet, so reserved
// keys are inherently excluded; any future generic list MUST skip RESERVED_KEYS.
const RESERVED_KEYS = ["title", "display", "body"] as const;
void RESERVED_KEYS;
export function PropertiesView({ blockId }: { blockId: string }) {
  const [title, setTitle] = useState("");
  const [display, setDisplay] = useState(BLOCK_VIEW_NAMES[0]);
  const needsWrite = useRef(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    getBlock(blockId).then(
      (block) => {
        if (cancelled) return;
        const titleValue = block.properties.title;
        setTitle(typeof titleValue === "string" ? titleValue : "");
        const displayValue = block.properties.display;
        const resolved =
          typeof displayValue === "string" && BLOCK_VIEW_NAMES.includes(displayValue)
            ? displayValue
            : BLOCK_VIEW_NAMES[0];
        needsWrite.current = resolved !== displayValue;
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

  const onTitleChange = (value: string) => {
    setTitle(value);
    setError(null);
    setProperty(blockId, "title", value).catch((err) => setError(String(err)));
  };

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
      <label>
        Title
        <input
          type="text"
          value={title}
          onChange={(e) => onTitleChange(e.target.value)}
        />
      </label>
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

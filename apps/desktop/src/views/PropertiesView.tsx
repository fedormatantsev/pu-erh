import { useEffect, useRef, useState } from "react";

import { Button, PropertiesPanel, Stack, Text } from "@pu-erh/ui";

import { getBlock, removeProperty, save, setProperty } from "../ipc";
import { BLOCK_VIEW_NAMES } from "./blockView";

const RESERVED_KEYS = ["title", "display", "body"] as const;
type ReservedKey = (typeof RESERVED_KEYS)[number];
function isReserved(key: string): key is ReservedKey {
  return (RESERVED_KEYS as readonly string[]).includes(key);
}

export function PropertiesView({ blockId }: { blockId: string }) {
  const [title, setTitle] = useState("");
  const [display, setDisplay] = useState(BLOCK_VIEW_NAMES[0]);
  const needsWrite = useRef(false);
  const [error, setError] = useState<string | null>(null);

  // User properties: all keys not in RESERVED_KEYS.
  const [userProps, setUserProps] = useState<Record<string, string>>({});

  // Add-property form state.
  const [addKey, setAddKey] = useState("");
  const [addValue, setAddValue] = useState("");
  const [addError, setAddError] = useState<string | null>(null);

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

        // Collect user properties (non-reserved string and non-string values shown as strings).
        const user: Record<string, string> = {};
        for (const [k, v] of Object.entries(block.properties)) {
          if (!isReserved(k)) {
            user[k] = v == null ? "" : String(v);
          }
        }
        setUserProps(user);
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

  const onRemoveProperty = (key: string) => {
    setError(null);
    removeProperty(blockId, key).then(
      () => {
        setUserProps((prev) => {
          const next = { ...prev };
          delete next[key];
          return next;
        });
      },
      (err) => setError(String(err)),
    );
  };

  const onAddProperty = () => {
    const trimmedKey = addKey.trim();
    if (!trimmedKey) {
      setAddError("Key must not be empty.");
      return;
    }
    if (isReserved(trimmedKey)) {
      setAddError(`"${trimmedKey}" is a reserved key.`);
      return;
    }
    setAddError(null);
    setProperty(blockId, trimmedKey, addValue).then(
      () => {
        setUserProps((prev) => ({ ...prev, [trimmedKey]: addValue }));
        setAddKey("");
        setAddValue("");
      },
      (err) => setError(String(err)),
    );
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

      {Object.entries(userProps).map(([key, value]) => (
        <Stack key={key} gap="0.25rem">
          <span>{key}: {value}</span>
          <button type="button" onClick={() => onRemoveProperty(key)}>
            Remove
          </button>
        </Stack>
      ))}

      <Stack gap="0.25rem">
        <input
          type="text"
          placeholder="key"
          value={addKey}
          onChange={(e) => { setAddKey(e.target.value); setAddError(null); }}
        />
        <input
          type="text"
          placeholder="value"
          value={addValue}
          onChange={(e) => setAddValue(e.target.value)}
        />
        <button type="button" onClick={onAddProperty}>
          Add
        </button>
        {addError ? <Text>{addError}</Text> : null}
      </Stack>

      <Stack gap="0.5rem">
        <Button onPress={persist}>Save</Button>
      </Stack>
      {error ? <Text>{error}</Text> : null}
    </PropertiesPanel>
  );
}

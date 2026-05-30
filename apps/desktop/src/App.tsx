import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Stack, Text } from "@pu-erh/ui";

async function invokeOrError(command: string): Promise<string> {
  try {
    return await invoke<string>(command);
  } catch (error) {
    return String(error);
  }
}

export function App() {
  const [ping, setPing] = useState("…");
  const [rootId, setRootId] = useState("…");

  useEffect(() => {
    void invokeOrError("ping").then(setPing);
    void invokeOrError("root_id").then(setRootId);
  }, []);

  return (
    <Stack>
      <Text as="h1">pu-erh</Text>
      <Text>ping: {ping}</Text>
      <Text>root_id: {rootId}</Text>
    </Stack>
  );
}

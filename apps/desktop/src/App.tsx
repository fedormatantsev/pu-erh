import { ActionBar, Stack, Text } from "@pu-erh/ui";

import { ShellProvider, useShell } from "./shell";
import { viewRouter } from "./viewRouter";

// Exactly one view is shown at a time, chosen by the View Router as a function
// of (currentBlockId, viewMode). The action bar overlay renders the actions
// declared by the active view.
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

  const { View, actions } = viewRouter(currentBlockId, viewMode);
  const actionList = actions({
    setViewMode,
    createChild,
    canCreateChild: true,
  });

  return (
    <Stack>
      <ActionBar actions={actionList} />
      {actionError ? <Text>{actionError}</Text> : null}
      <View blockId={currentBlockId} />
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

import { ActionBar, Stack, Text } from "@pu-erh/ui";

import { ShellProvider, useShell } from "./shell";
import { viewRouter } from "./viewRouter";
import type { ActionBarAction } from "./types";

// Exactly one view is shown at a time, chosen by the View Router as a function
// of (currentBlockId, viewMode). The action bar overlay renders shell-level
// navigation actions (backward, forward) followed by the actions declared by
// the active view.
function Workspace() {
  const {
    currentBlockId,
    rootError,
    actionError,
    viewMode,
    setViewMode,
    createChild,
    navigateBack,
    navigateForward,
    canGoBack,
    canGoForward,
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
  const viewActions = actions({
    setViewMode,
    createChild,
    canCreateChild: true,
  });

  const navActions: ActionBarAction[] = [
    {
      id: "nav-back",
      label: "←",
      onPress: navigateBack,
      isDisabled: !canGoBack,
    },
    {
      id: "nav-forward",
      label: "→",
      onPress: navigateForward,
      isDisabled: !canGoForward,
    },
  ];

  return (
    <Stack>
      <ActionBar actions={[...navActions, ...viewActions]} />
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

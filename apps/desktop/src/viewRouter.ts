import type { ReactElement } from "react";

import { DefaultBlockView } from "./views/blockView";
import { PropertiesView } from "./views/PropertiesView";
import type { ActionBarAction, ViewMode } from "./types";

export type ViewActionContext = {
  setViewMode: (mode: ViewMode) => void;
  createChild: () => void;
  canCreateChild: boolean;
};

export type ViewDescriptor = {
  View: (props: { blockId: string }) => ReactElement;
  actions: (ctx: ViewActionContext) => ActionBarAction[];
};

export function viewRouter(
  _blockId: string,
  mode: ViewMode,
): ViewDescriptor {
  if (mode === "block") {
    return {
      View: DefaultBlockView,
      actions: ({ setViewMode, createChild, canCreateChild }) => [
        {
          id: "toggle",
          label: "Properties",
          onPress: () => setViewMode("properties"),
          pressed: false,
        },
        {
          id: "create-child",
          label: "New child",
          onPress: createChild,
          isDisabled: !canCreateChild,
        },
      ],
    };
  }

  return {
    View: PropertiesView,
    actions: ({ setViewMode }) => [
      {
        id: "toggle",
        label: "Block View",
        onPress: () => setViewMode("block"),
        pressed: true,
      },
    ],
  };
}

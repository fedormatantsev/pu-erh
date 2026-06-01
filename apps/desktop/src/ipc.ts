import { invoke } from "@tauri-apps/api/core";

import type { BlockDto, PropertyValue } from "./types";

// Thin wrappers over the Tauri commands. Errors propagate as thrown values
// carrying the backend's CoreError-derived string, surfaced as returned.

export function ping(): Promise<string> {
  return invoke<string>("ping");
}

export function rootId(): Promise<string> {
  return invoke<string>("root_id");
}

export function getBlock(id: string): Promise<BlockDto> {
  return invoke<BlockDto>("block", { id });
}

export function getParent(id: string): Promise<BlockDto | null> {
  return invoke<BlockDto | null>("parent", { id });
}

export function getChildren(id: string): Promise<BlockDto[]> {
  return invoke<BlockDto[]>("children", { id });
}

// Children in child-ordering order (sorted by the edge `order` property), which
// the unordered `children` command does not expose.
export function getChildrenOrdered(id: string): Promise<BlockDto[]> {
  return invoke<BlockDto[]>("children_ordered", { id });
}

export function setProperty(
  id: string,
  key: string,
  value: PropertyValue,
): Promise<void> {
  return invoke<void>("set_property", { id, key, value });
}

export function removeProperty(id: string, key: string): Promise<void> {
  return invoke<void>("remove_property", { id, key });
}

// Creates a child of `parent`. When `after` is a sibling id the child is placed
// immediately after it; otherwise it is appended last.
export function createBlock(parent: string, after?: string): Promise<string> {
  return invoke<string>("create_block", { parent, after: after ?? null });
}

export function deleteBlock(id: string): Promise<void> {
  return invoke<void>("delete_block", { id });
}

// Reorders a block among its existing siblings. `after` is the sibling it should
// follow, or omitted/null to move it first.
export function moveBlock(id: string, after?: string): Promise<void> {
  return invoke<void>("move_block", { id, after: after ?? null });
}

export function save(): Promise<void> {
  return invoke<void>("save");
}

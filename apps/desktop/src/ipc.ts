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

export function setProperty(
  id: string,
  key: string,
  value: PropertyValue,
): Promise<void> {
  return invoke<void>("set_property", { id, key, value });
}

export function createBlock(parent: string): Promise<string> {
  return invoke<string>("create_block", { parent });
}

export function save(): Promise<void> {
  return invoke<void>("save");
}

// Mirrors the desktop crate's BlockDto. PropertyValue serializes to a bare JSON
// scalar, so the properties map is a record of scalars.
export type PropertyValue = string | number | boolean | null;

export type BlockDto = {
  id: string;
  properties: Record<string, PropertyValue>;
  has_children: boolean;
};

export type ViewMode = "block" | "properties";

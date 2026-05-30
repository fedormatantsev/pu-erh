import type { CSSProperties, ReactNode } from "react";

import "./styles.css";

type TextProps = {
  children: ReactNode;
  as?: "p" | "span" | "h1";
};

export function Text({ children, as: Tag = "p" }: TextProps) {
  return <Tag className="pu-erh-text">{children}</Tag>;
}

type StackProps = {
  children: ReactNode;
  gap?: CSSProperties["gap"];
};

export function Stack({ children, gap = "0.5rem" }: StackProps) {
  return (
    <div className="pu-erh-stack" style={{ gap }}>
      {children}
    </div>
  );
}

type ButtonProps = {
  children: ReactNode;
  disabled?: boolean;
  onClick?: () => void;
  type?: "button" | "submit" | "reset";
};

export function Button({
  children,
  disabled = false,
  onClick,
  type = "button",
}: ButtonProps) {
  return (
    <button
      className="pu-erh-button"
      disabled={disabled}
      onClick={onClick}
      type={type}
    >
      {children}
    </button>
  );
}

// Presentational Block View building blocks. These are session-agnostic: they
// receive data and callbacks as props and never call IPC/Tauri APIs or assume a
// current selected block.

type InlineBlockProps = {
  label: ReactNode;
  onActivate?: () => void;
};

export function InlineBlock({ label, onActivate }: InlineBlockProps) {
  return (
    <button
      className="pu-erh-inline-block"
      onClick={onActivate}
      type="button"
    >
      {label}
    </button>
  );
}

type TreeNodeProps = {
  label: ReactNode;
  hasChildren: boolean;
  expanded: boolean;
  onToggle?: () => void;
  children?: ReactNode;
};

export function TreeNode({
  label,
  hasChildren,
  expanded,
  onToggle,
  children,
}: TreeNodeProps) {
  return (
    <div className="pu-erh-tree-node">
      <div className="pu-erh-tree-row">
        <button
          aria-label={expanded ? "collapse" : "expand"}
          className="pu-erh-tree-toggle"
          disabled={!hasChildren}
          onClick={onToggle}
          type="button"
        >
          {hasChildren ? (expanded ? "▾" : "▸") : "·"}
        </button>
        {label}
      </div>
      {expanded && children ? (
        <div className="pu-erh-tree-children">{children}</div>
      ) : null}
    </div>
  );
}

type ViewMode = "block" | "properties";

type ViewModeToggleProps = {
  mode: ViewMode;
  onChange: (mode: ViewMode) => void;
};

export function ViewModeToggle({ mode, onChange }: ViewModeToggleProps) {
  return (
    <div className="pu-erh-view-mode-toggle" role="group">
      <Button disabled={mode === "block"} onClick={() => onChange("block")}>
        Block View
      </Button>
      <Button
        disabled={mode === "properties"}
        onClick={() => onChange("properties")}
      >
        Properties
      </Button>
    </div>
  );
}

type PropertiesPanelProps = {
  children: ReactNode;
};

export function PropertiesPanel({ children }: PropertiesPanelProps) {
  return <section className="pu-erh-properties-panel">{children}</section>;
}

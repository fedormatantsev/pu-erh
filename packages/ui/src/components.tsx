import type { CSSProperties, ReactNode } from "react";
import {
  Button as AriaButton,
  type ButtonProps as AriaButtonProps,
  Separator,
} from "react-aria-components";

type BadgeProps = {
  children: ReactNode;
  variant?: "neutral" | "primary";
};

export function Badge({ children, variant = "neutral" }: BadgeProps) {
  return (
    <span className={`pu-erh-badge pu-erh-badge--${variant}`}>{children}</span>
  );
}

type CardProps = {
  children: ReactNode;
};

export function Card({ children }: CardProps) {
  return <div className="pu-erh-card">{children}</div>;
}

export function Divider() {
  return <Separator className="pu-erh-divider" />;
}

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

type ButtonProps = Pick<AriaButtonProps, "children" | "onPress" | "isDisabled" | "type">;

export function Button({ children, onPress, isDisabled, type = "button" }: ButtonProps) {
  return (
    <AriaButton className="pu-erh-button" onPress={onPress} isDisabled={isDisabled} type={type}>
      {children}
    </AriaButton>
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
    <AriaButton className="pu-erh-inline-block" onPress={onActivate} type="button">
      {label}
    </AriaButton>
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
        <AriaButton
          aria-label={expanded ? "collapse" : "expand"}
          className="pu-erh-tree-toggle"
          isDisabled={!hasChildren}
          onPress={onToggle}
          type="button"
        >
          {hasChildren ? (expanded ? "▾" : "▸") : "·"}
        </AriaButton>
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
      <Button isDisabled={mode === "block"} onPress={() => onChange("block")}>
        Block View
      </Button>
      <Button
        isDisabled={mode === "properties"}
        onPress={() => onChange("properties")}
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

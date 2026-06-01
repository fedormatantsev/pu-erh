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

// Three-column TreeView layout. Session-agnostic: the container and columns lay
// out whatever inline-block cells they are given; the current-block highlight is
// driven by the `current` prop on a cell. No data fetching or selection logic.

type TreeColumnsProps = {
  children: ReactNode;
};

export function TreeColumns({ children }: TreeColumnsProps) {
  return <div className="pu-erh-tree-columns">{children}</div>;
}

type TreeColumnProps = {
  label?: ReactNode;
  children?: ReactNode;
};

export function TreeColumn({ label, children }: TreeColumnProps) {
  return (
    <div className="pu-erh-tree-column">
      {label != null ? (
        <div className="pu-erh-tree-column-label">{label}</div>
      ) : null}
      <div className="pu-erh-tree-column-items">{children}</div>
    </div>
  );
}

type TreeCellProps = {
  current?: boolean;
  children: ReactNode;
};

export function TreeCell({ current = false, children }: TreeCellProps) {
  return (
    <div
      className={
        current ? "pu-erh-tree-cell pu-erh-tree-cell--current" : "pu-erh-tree-cell"
      }
    >
      {children}
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

// Presentational Structured Document View building blocks. Session-agnostic:
// they receive content and callbacks as props and never call IPC/Tauri APIs or
// assume a current selected block. The rich-text editor itself (Lexical) is
// composed in the application adapter and mounted inside DocumentBody.

type DocumentSurfaceProps = {
  children: ReactNode;
};

export function DocumentSurface({ children }: DocumentSurfaceProps) {
  return <article className="pu-erh-document">{children}</article>;
}

// Editable plain-text heading bound to the document block's `title`. Plain text
// only — no rich-text marks. Empty when the title is absent (no placeholder
// copy). Submitting (Enter) blurs rather than inserting a newline.
type DocumentHeadingProps = {
  value: string;
  onChange: (value: string) => void;
  ariaLabel?: string;
};

export function DocumentHeading({ value, onChange, ariaLabel }: DocumentHeadingProps) {
  return (
    <input
      className="pu-erh-document-heading"
      type="text"
      value={value}
      aria-label={ariaLabel}
      onChange={(event) => onChange(event.target.value)}
      onKeyDown={(event) => {
        if (event.key === "Enter") {
          event.preventDefault();
          event.currentTarget.blur();
        }
      }}
    />
  );
}

// The seven supported rich-text formats for paragraphs — fixed set, no others.
export type DocumentFormat =
  | "bold"
  | "italic"
  | "underline"
  | "strikethrough"
  | "code"
  | "link"
  | "quote";

const DOCUMENT_FORMATS: { format: DocumentFormat; label: string }[] = [
  { format: "bold", label: "B" },
  { format: "italic", label: "I" },
  { format: "underline", label: "U" },
  { format: "strikethrough", label: "S" },
  { format: "code", label: "Code" },
  { format: "link", label: "Link" },
  { format: "quote", label: "Quote" },
];

// Presentational formatting toolbar exposing exactly the supported formats.
// Active marks are reflected via aria-pressed; toggling delegates to the adapter.
type FormatToolbarProps = {
  active: Partial<Record<DocumentFormat, boolean>>;
  onToggle: (format: DocumentFormat) => void;
  isDisabled?: boolean;
};

export function FormatToolbar({ active, onToggle, isDisabled }: FormatToolbarProps) {
  return (
    <div className="pu-erh-format-toolbar" role="toolbar" aria-label="Formatting">
      {DOCUMENT_FORMATS.map(({ format, label }) => (
        <AriaButton
          key={format}
          className="pu-erh-format-button"
          type="button"
          isDisabled={isDisabled}
          aria-pressed={active[format] ?? false}
          aria-label={format}
          onPress={() => onToggle(format)}
        >
          {label}
        </AriaButton>
      ))}
    </div>
  );
}

// Styled container that hosts the Lexical content-editable surface (passed as
// children by the adapter).
type DocumentBodyProps = {
  children: ReactNode;
};

export function DocumentBody({ children }: DocumentBodyProps) {
  return <div className="pu-erh-document-body">{children}</div>;
}

// The action bar overlay: a compact floating panel of view-provided actions.
// Session-agnostic — it receives an ordered list of action descriptors as props
// and never reads shell state or calls IPC.

type ActionBarAction = {
  id: string;
  label: string;
  onPress: () => void;
  isDisabled?: boolean;
  pressed?: boolean;
};

type ActionBarProps = {
  actions: ActionBarAction[];
};

export function ActionBar({ actions }: ActionBarProps) {
  return (
    <div className="pu-erh-action-bar" role="toolbar" aria-label="Actions">
      {actions.map((action) => (
        <AriaButton
          key={action.id}
          className="pu-erh-button"
          onPress={action.onPress}
          isDisabled={action.isDisabled}
          type="button"
          aria-pressed={action.pressed}
        >
          {action.label}
        </AriaButton>
      ))}
    </div>
  );
}

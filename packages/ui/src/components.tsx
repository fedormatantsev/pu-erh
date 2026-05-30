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

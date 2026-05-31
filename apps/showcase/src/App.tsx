import { useEffect, useRef, useState, type ReactNode } from "react";
import {
  Badge,
  Button,
  Card,
  Divider,
  Stack,
  Text,
} from "@pu-erh/ui";

// ── Helpers ──────────────────────────────────────────────────────────────────

function resolveToken(name: string): string {
  return getComputedStyle(document.documentElement)
    .getPropertyValue(name)
    .trim();
}

// ── Layout primitives ─────────────────────────────────────────────────────────

const styles: Record<string, React.CSSProperties> = {
  page: {
    background: "var(--color-bg)",
    color: "var(--color-text)",
    fontFamily: "var(--font-sans)",
    minHeight: "100vh",
    padding: "var(--space-12) var(--space-8)",
  },
  container: {
    maxWidth: 720,
    margin: "0 auto",
  },
  pageTitle: {
    fontFamily: "var(--font-display)",
    fontSize: "var(--text-2xl)",
    fontWeight: "var(--weight-semibold)",
    lineHeight: "var(--leading-tight)",
    margin: 0,
    marginBottom: "var(--space-2)",
  },
  pageSubtitle: {
    fontFamily: "var(--font-sans)",
    fontSize: "var(--text-sm)",
    color: "var(--color-text-muted)",
    margin: 0,
    lineHeight: "var(--leading-base)",
  },
  sectionGap: {
    marginTop: "var(--space-12)",
  },
  sectionTitle: {
    fontFamily: "var(--font-display)",
    fontSize: "var(--text-base)",
    fontWeight: "var(--weight-semibold)",
    lineHeight: "var(--leading-tight)",
    margin: 0,
    marginBottom: "var(--space-6)",
    color: "var(--color-text)",
  },
  mono: {
    fontFamily: "var(--font-mono)",
    fontSize: "var(--text-xs)",
    color: "var(--color-text-muted)",
  },
};

// ── Section wrapper ────────────────────────────────────────────────────────────

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <div style={styles.sectionGap}>
      <h2 style={styles.sectionTitle}>{title}</h2>
      {children}
    </div>
  );
}

// ── Disclosure (progressive disclosure accordion) ─────────────────────────────

function Disclosure({
  summary,
  children,
}: {
  summary: string;
  children: ReactNode;
}) {
  const [open, setOpen] = useState(false);
  return (
    <div style={{ marginBottom: "var(--space-4)" }}>
      <button
        onClick={() => setOpen((v) => !v)}
        style={{
          display: "flex",
          alignItems: "center",
          gap: "var(--space-2)",
          background: "none",
          border: "none",
          cursor: "pointer",
          padding: 0,
          fontFamily: "var(--font-display)",
          fontSize: "var(--text-sm)",
          fontWeight: "var(--weight-medium)",
          color: "var(--color-text)",
          width: "100%",
          textAlign: "left",
        }}
        type="button"
      >
        <span
          style={{
            fontFamily: "var(--font-mono)",
            fontSize: "var(--text-xs)",
            color: "var(--color-text-muted)",
            display: "inline-block",
            width: "var(--space-4)",
          }}
        >
          {open ? "▾" : "▸"}
        </span>
        {summary}
      </button>
      {open && (
        <div
          style={{
            marginTop: "var(--space-4)",
            paddingLeft: "var(--space-6)",
          }}
        >
          {children}
        </div>
      )}
    </div>
  );
}

// ── Spacing section ────────────────────────────────────────────────────────────

const spacingTokens = [
  "--space-1",
  "--space-2",
  "--space-3",
  "--space-4",
  "--space-6",
  "--space-8",
  "--space-10",
  "--space-12",
  "--space-16",
];

function SpacingSection() {
  const [values, setValues] = useState<Record<string, string>>({});
  useEffect(() => {
    const resolved: Record<string, string> = {};
    for (const t of spacingTokens) resolved[t] = resolveToken(t);
    setValues(resolved);
  }, []);

  return (
    <Section title="Spacing">
      <Stack gap="var(--space-2)">
        {spacingTokens.map((token) => {
          const px = values[token] ?? "";
          return (
            <div
              key={token}
              style={{
                display: "flex",
                alignItems: "center",
                gap: "var(--space-4)",
              }}
            >
              <span style={{ ...styles.mono, width: 96, flexShrink: 0 }}>
                {token}
              </span>
              <div
                style={{
                  height: 4,
                  width: px,
                  background: "var(--color-primary-500)",
                  borderRadius: "var(--radius-full)",
                  flexShrink: 0,
                }}
              />
              <span style={{ ...styles.mono, color: "var(--color-text-muted)" }}>
                {px}
              </span>
            </div>
          );
        })}
      </Stack>
    </Section>
  );
}

// ── Color section ─────────────────────────────────────────────────────────────

const neutralTokens = [
  "--color-neutral-0",
  "--color-neutral-50",
  "--color-neutral-100",
  "--color-neutral-200",
  "--color-neutral-300",
  "--color-neutral-400",
  "--color-neutral-500",
  "--color-neutral-600",
  "--color-neutral-700",
  "--color-neutral-800",
  "--color-neutral-900",
];

const primaryTokens = [
  "--color-primary-400",
  "--color-primary-500",
  "--color-primary-600",
];

const semanticTokens = [
  "--color-bg",
  "--color-surface",
  "--color-border",
  "--color-text",
  "--color-text-muted",
];

function SwatchRow({ tokens }: { tokens: string[] }) {
  return (
    <div style={{ display: "flex", gap: "var(--space-3)", flexWrap: "wrap" }}>
      {tokens.map((token) => (
        <div key={token} style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: "var(--space-2)" }}>
          <div
            style={{
              width: 48,
              height: 48,
              background: `var(${token})`,
              borderRadius: "var(--radius-sm)",
              boxShadow: "0 0 0 1px rgba(0,0,0,0.06)",
            }}
          />
          <span style={{ ...styles.mono, textAlign: "center", maxWidth: 80, wordBreak: "break-all" }}>
            {token.replace("--color-", "")}
          </span>
        </div>
      ))}
    </div>
  );
}

function ColorSection() {
  return (
    <Section title="Color">
      <Stack gap="var(--space-8)">
        <div>
          <p style={{ ...styles.mono, marginBottom: "var(--space-3)" }}>neutral</p>
          <SwatchRow tokens={neutralTokens} />
        </div>
        <div>
          <p style={{ ...styles.mono, marginBottom: "var(--space-3)" }}>primary</p>
          <SwatchRow tokens={primaryTokens} />
        </div>
        <div>
          <p style={{ ...styles.mono, marginBottom: "var(--space-3)" }}>semantic</p>
          <SwatchRow tokens={semanticTokens} />
        </div>
      </Stack>
    </Section>
  );
}

// ── Typography section ────────────────────────────────────────────────────────

type TypographySample = {
  token: string;
  label: string;
  font: string;
  weight: string;
};

const typographySamples: TypographySample[] = [
  { token: "--text-2xl", label: "2xl · display", font: "var(--font-display)", weight: "var(--weight-semibold)" },
  { token: "--text-xl",  label: "xl · display",  font: "var(--font-display)", weight: "var(--weight-semibold)" },
  { token: "--text-lg",  label: "lg · display",  font: "var(--font-display)", weight: "var(--weight-medium)"   },
  { token: "--text-base",label: "base · body",   font: "var(--font-sans)",    weight: "var(--weight-normal)"   },
  { token: "--text-sm",  label: "sm · body",     font: "var(--font-sans)",    weight: "var(--weight-normal)"   },
  { token: "--text-xs",  label: "xs · mono",     font: "var(--font-mono)",    weight: "var(--weight-normal)"   },
];

function TypographySection() {
  return (
    <Section title="Typography">
      <Stack gap="var(--space-6)">
        {typographySamples.map(({ token, label, font, weight }) => (
          <div key={token} style={{ display: "flex", alignItems: "baseline", gap: "var(--space-6)" }}>
            <span style={{ ...styles.mono, width: 120, flexShrink: 0 }}>{label}</span>
            <span
              style={{
                fontFamily: font,
                fontSize: `var(${token})`,
                fontWeight: weight,
                lineHeight: "var(--leading-tight)",
                color: "var(--color-text)",
              }}
            >
              The quick brown fox
            </span>
          </div>
        ))}
      </Stack>
    </Section>
  );
}

// ── Radii section ─────────────────────────────────────────────────────────────

const radiusTokens = [
  { token: "--radius-sm",   label: "sm" },
  { token: "--radius-md",   label: "md" },
  { token: "--radius-lg",   label: "lg" },
  { token: "--radius-full", label: "full" },
];

function RadiiSection() {
  return (
    <Section title="Radii">
      <div style={{ display: "flex", gap: "var(--space-6)", flexWrap: "wrap" }}>
        {radiusTokens.map(({ token, label }) => (
          <div key={token} style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: "var(--space-3)" }}>
            <div
              style={{
                width: 56,
                height: 56,
                background: "var(--color-surface)",
                boxShadow: "0 0 0 1px var(--color-border)",
                borderRadius: `var(${token})`,
              }}
            />
            <span style={styles.mono}>{label}</span>
          </div>
        ))}
      </div>
    </Section>
  );
}

// ── Components section (progressive disclosure) ───────────────────────────────

function ComponentsSection() {
  return (
    <Section title="Components">
      <Disclosure summary="Button">
        <Stack gap="var(--space-3)">
          <div style={{ display: "flex", gap: "var(--space-3)", flexWrap: "wrap", alignItems: "center" }}>
            <Button>Default</Button>
            <Button disabled>Disabled</Button>
          </div>
          <span style={styles.mono}>Button — uses --font-sans, --space-2/4, --radius-sm</span>
        </Stack>
      </Disclosure>

      <Disclosure summary="Badge">
        <Stack gap="var(--space-3)">
          <div style={{ display: "flex", gap: "var(--space-3)", flexWrap: "wrap", alignItems: "center" }}>
            <Badge>neutral</Badge>
            <Badge variant="primary">primary</Badge>
          </div>
          <span style={styles.mono}>Badge — uses --font-mono, --radius-full, --text-xs</span>
        </Stack>
      </Disclosure>

      <Disclosure summary="Card">
        <Stack gap="var(--space-3)">
          <Card>
            <Text>A card surface with padding and a slightly elevated background. No border — space provides containment.</Text>
          </Card>
          <span style={styles.mono}>Card — uses --color-surface, --radius-md, --space-4</span>
        </Stack>
      </Disclosure>

      <Disclosure summary="Divider">
        <Stack gap="var(--space-3)">
          <div>
            <Text>Content above</Text>
            <Divider />
            <Text>Content below — separated by space, no visible line</Text>
          </div>
          <span style={styles.mono}>Divider — whitespace only, margin: var(--space-4) 0</span>
        </Stack>
      </Disclosure>

      <Disclosure summary="Text">
        <Stack gap="var(--space-3)">
          <Text as="h1">Heading h1 — display font</Text>
          <Text>Body paragraph — sans font, base size, base leading</Text>
          <Text as="span">Inline span</Text>
          <span style={styles.mono}>Text — uses --font-sans, --text-base, --leading-base</span>
        </Stack>
      </Disclosure>

      <Disclosure summary="Stack">
        <Stack gap="var(--space-3)">
          <Stack gap="var(--space-2)">
            <Badge>item one</Badge>
            <Badge>item two</Badge>
            <Badge>item three</Badge>
          </Stack>
          <span style={styles.mono}>Stack — flex column, gap prop maps to --space-* token</span>
        </Stack>
      </Disclosure>
    </Section>
  );
}

// ── App ───────────────────────────────────────────────────────────────────────

export function App() {
  const ref = useRef<HTMLDivElement>(null);
  return (
    <div ref={ref} style={styles.page}>
      <div style={styles.container}>
        <h1 style={styles.pageTitle}>pu-erh design system</h1>
        <p style={styles.pageSubtitle}>
          Tokens, components, and UI direction — a living reference.
        </p>

        <SpacingSection />
        <ColorSection />
        <TypographySection />
        <RadiiSection />
        <ComponentsSection />

        <div style={{ marginTop: "var(--space-16)" }} />
      </div>
    </div>
  );
}

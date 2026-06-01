import { useCallback, useEffect, useRef, useState } from "react";

import { LexicalComposer } from "@lexical/react/LexicalComposer";
import { useLexicalComposerContext } from "@lexical/react/LexicalComposerContext";
import { RichTextPlugin } from "@lexical/react/LexicalRichTextPlugin";
import { ContentEditable } from "@lexical/react/LexicalContentEditable";
import { LexicalErrorBoundary } from "@lexical/react/LexicalErrorBoundary";
import { HistoryPlugin } from "@lexical/react/LexicalHistoryPlugin";
import { LinkPlugin } from "@lexical/react/LexicalLinkPlugin";
import { $createQuoteNode, $isQuoteNode, QuoteNode } from "@lexical/rich-text";
import { $isLinkNode, LinkNode, TOGGLE_LINK_COMMAND } from "@lexical/link";
import { $setBlocksType } from "@lexical/selection";
import { mergeRegister } from "@lexical/utils";
import {
  $createParagraphNode,
  $getRoot,
  $getSelection,
  $isElementNode,
  $isRangeSelection,
  COMMAND_PRIORITY_LOW,
  FORMAT_TEXT_COMMAND,
  SELECTION_CHANGE_COMMAND,
  type EditorState,
  type LexicalNode,
} from "lexical";

import {
  Button,
  DocumentBody,
  DocumentHeading,
  DocumentSurface,
  FormatToolbar,
  Stack,
  Text,
  type DocumentFormat,
} from "@pu-erh/ui";

import {
  createBlock,
  deleteBlock,
  getBlock,
  getChildrenOrdered,
  save,
  setProperty,
} from "../ipc";
import type { BlockDto, PropertyValue } from "../types";

// Serialized form of a single paragraph child's content: a Lexical top-level
// node (paragraph or quote) including its descendants. Stored as the child
// block's reserved `body` property (a JSON string).
function serializeNode(node: LexicalNode): unknown {
  const json = node.exportJSON() as { children?: unknown[] };
  if ($isElementNode(node)) {
    json.children = node.getChildren().map(serializeNode);
  }
  return json;
}

// A bare empty paragraph used when a child has no (or a non-string) `body`.
const EMPTY_PARAGRAPH = {
  children: [],
  direction: null,
  format: "",
  indent: 0,
  type: "paragraph",
  version: 1,
} as const;

function bodyToNodeJson(body: PropertyValue): unknown {
  if (typeof body === "string" && body.length > 0) {
    try {
      return JSON.parse(body);
    } catch {
      return { ...EMPTY_PARAGRAPH };
    }
  }
  return { ...EMPTY_PARAGRAPH };
}

// Builds the initial Lexical editor-state JSON from the ordered child rows. When
// there are no children a single empty placeholder paragraph is shown; it stays
// unmapped (no block) until the user types into it.
function buildInitialEditorState(rows: Row[]): string {
  const children =
    rows.length > 0
      ? rows.map((row) => bodyToNodeJson(row.body))
      : [{ ...EMPTY_PARAGRAPH }];
  return JSON.stringify({
    root: {
      children,
      direction: "ltr",
      format: "",
      indent: 0,
      type: "root",
      version: 1,
    },
  });
}

type Row = { id: string; body: PropertyValue };

const ALL_FORMATS: DocumentFormat[] = [
  "bold",
  "italic",
  "underline",
  "strikethrough",
  "code",
  "link",
  "quote",
];

// The Structured Document View: the current selected block's `title` as an
// editable heading and its child blocks as editable rich-text paragraphs. Reads
// and mutates state through core::Session via the desktop IPC; the editor wiring
// (node <-> child block mapping, structural edits) lives here in the adapter,
// while the chrome (heading, toolbar, surface) is the session-agnostic UI.
export function DocumentView({ blockId }: { blockId: string }) {
  const [title, setTitle] = useState("");
  const [rows, setRows] = useState<Row[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setRows(null);
    setError(null);
    (async () => {
      try {
        const block = await getBlock(blockId);
        const children = await getChildrenOrdered(blockId);
        if (cancelled) return;
        const titleValue = block.properties.title;
        setTitle(typeof titleValue === "string" ? titleValue : "");
        setRows(
          children.map((child: BlockDto) => ({
            id: child.id,
            body: child.properties.body ?? null,
          })),
        );
      } catch (err) {
        if (!cancelled) setError(String(err));
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [blockId]);

  const onTitleChange = useCallback(
    (value: string) => {
      setTitle(value);
      setError(null);
      setProperty(blockId, "title", value).catch((err) => setError(String(err)));
    },
    [blockId],
  );

  if (error) {
    return <Text>{error}</Text>;
  }
  if (rows === null) {
    return <></>;
  }

  return (
    <DocumentSurface>
      <DocumentHeading value={title} onChange={onTitleChange} ariaLabel="Title" />
      <LexicalComposer
        // Remount the editor when the selected block changes so it re-seeds from
        // that block's children.
        key={blockId}
        initialConfig={{
          namespace: "pu-erh-document",
          editorState: buildInitialEditorState(rows),
          // Only the supported rich-text nodes are registered; list, heading,
          // image, table, and code-block nodes are intentionally omitted so
          // unsupported formats cannot be produced or pasted in.
          nodes: [QuoteNode, LinkNode],
          theme: {
            text: {
              bold: "pu-erh-doc-bold",
              italic: "pu-erh-doc-italic",
              underline: "pu-erh-doc-underline",
              strikethrough: "pu-erh-doc-strikethrough",
              code: "pu-erh-doc-code",
              underlineStrikethrough:
                "pu-erh-doc-underline pu-erh-doc-strikethrough",
            },
          },
          onError(err) {
            setError(String(err));
          },
        }}
      >
        <DocumentEditor
          docBlockId={blockId}
          initialRows={rows}
          onError={setError}
        />
      </LexicalComposer>
      {error ? <Text>{error}</Text> : null}
    </DocumentSurface>
  );
}

// The editor surface: toolbar, content-editable body, and an explicit Save
// control. Owns the node<->block reconciliation against core::Session.
function DocumentEditor({
  docBlockId,
  initialRows,
  onError,
}: {
  docBlockId: string;
  initialRows: Row[];
  onError: (message: string) => void;
}) {
  const [editor] = useLexicalComposerContext();
  const [active, setActive] = useState<Partial<Record<DocumentFormat, boolean>>>(
    {},
  );

  // Authoritative map from a top-level node key to its backing child block id.
  // Maintained by reconciliation (create/delete) and by the quote toggle (which
  // replaces the node but preserves the block). Nodes without an entry are
  // unmapped (a placeholder or a freshly typed paragraph awaiting a block).
  const keyToBlock = useRef<Map<string, string>>(new Map());
  // Last `body` value persisted per block, to skip redundant writes.
  const lastBody = useRef<Map<string, string>>(new Map());
  // Serializes async mutations so positions resolve in document order.
  const queue = useRef<Promise<void>>(Promise.resolve());

  const enqueue = useCallback((task: () => Promise<void>) => {
    queue.current = queue.current.then(task).catch((err) => onError(String(err)));
    return queue.current;
  }, [onError]);

  // Reconcile the editor's top-level nodes against the backing child blocks:
  // create a block for a non-placeholder unmapped node, delete a block whose
  // node is gone, and persist changed `body` values. All in memory; no save.
  const reconcile = useCallback(
    (editorState: EditorState) => {
      type Top = { key: string; blockId: string | null; body: string; size: number };
      const tops: Top[] = editorState.read(() =>
        $getRoot()
          .getChildren()
          .map((node) => ({
            key: node.getKey(),
            blockId: keyToBlock.current.get(node.getKey()) ?? null,
            body: JSON.stringify(serializeNode(node)),
            size: node.getTextContentSize(),
          })),
      );

      const presentIds = new Set(
        tops.map((top) => top.blockId).filter((id): id is string => id !== null),
      );
      const deletions = [...new Set(keyToBlock.current.values())].filter(
        (id) => !presentIds.has(id),
      );

      // Prune bookkeeping for deleted blocks synchronously, before enqueuing. A
      // single backspace-merge commits more than one editor state in the same
      // tick, so reconcile can run again before the async delete task executes;
      // pruning here ensures the next snapshot no longer sees the removed id and
      // therefore does not enqueue a second `delete_block` (which would fail as
      // "block not found").
      for (const id of deletions) {
        lastBody.current.delete(id);
        for (const [key, value] of [...keyToBlock.current]) {
          if (value === id) keyToBlock.current.delete(key);
        }
      }

      enqueue(async () => {
        for (const id of deletions) {
          await deleteBlock(id);
        }

        // Re-read each node's block id from the live map (not the snapshot) so a
        // create queued by an earlier reconcile is observed here instead of
        // being created a second time for the same node.
        let anyMapped = tops.some((top) => keyToBlock.current.has(top.key));
        let prevId: string | null = null;
        for (const top of tops) {
          let id = keyToBlock.current.get(top.key) ?? null;
          if (id === null) {
            if (anyMapped || top.size > 0) {
              id = await createBlock(docBlockId, prevId ?? undefined);
              keyToBlock.current.set(top.key, id);
              anyMapped = true;
              await setProperty(id, "body", top.body);
              lastBody.current.set(id, top.body);
            }
          } else if (lastBody.current.get(id) !== top.body) {
            await setProperty(id, "body", top.body);
            lastBody.current.set(id, top.body);
          }
          if (id !== null) prevId = id;
        }
      });
    },
    [docBlockId, enqueue],
  );

  // Recompute which formats are active for the current selection so the toolbar
  // can reflect them.
  const refreshActiveFormats = useCallback(() => {
    editor.getEditorState().read(() => {
      const selection = $getSelection();
      if (!$isRangeSelection(selection)) {
        setActive({});
        return;
      }
      const node = selection.anchor.getNode();
      const top = node.getTopLevelElement();
      const parent = node.getParent();
      setActive({
        bold: selection.hasFormat("bold"),
        italic: selection.hasFormat("italic"),
        underline: selection.hasFormat("underline"),
        strikethrough: selection.hasFormat("strikethrough"),
        code: selection.hasFormat("code"),
        link: $isLinkNode(parent) || $isLinkNode(node),
        quote: top != null && $isQuoteNode(top),
      });
    });
  }, [editor]);

  // Seed the node->block map from the initial rows (zip top-level nodes with the
  // ordered child ids), then watch for edits.
  useEffect(() => {
    editor.getEditorState().read(() => {
      const nodes = $getRoot().getChildren();
      nodes.forEach((node, index) => {
        const row = initialRows[index];
        if (!row) return;
        keyToBlock.current.set(node.getKey(), row.id);
        lastBody.current.set(row.id, JSON.stringify(serializeNode(node)));
      });
    });
    refreshActiveFormats();

    return mergeRegister(
      editor.registerUpdateListener(({ editorState }) => {
        reconcile(editorState);
        refreshActiveFormats();
      }),
      editor.registerCommand(
        SELECTION_CHANGE_COMMAND,
        () => {
          refreshActiveFormats();
          return false;
        },
        COMMAND_PRIORITY_LOW,
      ),
    );
    // Re-seed only when the editor instance changes (it remounts per block).
  }, [editor, initialRows, reconcile, refreshActiveFormats]);

  const onToggle = useCallback(
    (format: DocumentFormat) => {
      onError("");
      switch (format) {
        case "bold":
        case "italic":
        case "underline":
        case "strikethrough":
        case "code":
          editor.dispatchCommand(FORMAT_TEXT_COMMAND, format);
          break;
        case "link": {
          const url = window.prompt("Link URL");
          editor.dispatchCommand(
            TOGGLE_LINK_COMMAND,
            url && url.length > 0 ? url : null,
          );
          break;
        }
        case "quote": {
          editor.update(() => {
            const selection = $getSelection();
            if (!$isRangeSelection(selection)) return;
            const top = selection.anchor.getNode().getTopLevelElement();
            const wasQuote = top != null && $isQuoteNode(top);
            const oldKey = top?.getKey();
            const blockId = oldKey
              ? keyToBlock.current.get(oldKey)
              : undefined;
            $setBlocksType(selection, () =>
              wasQuote ? $createParagraphNode() : $createQuoteNode(),
            );
            // The block-level node was replaced; carry the block id onto the new
            // node so reconciliation treats it as a content change, not a
            // delete + create.
            const newTop = $getSelection()
              ?.getNodes()[0]
              ?.getTopLevelElement();
            const newKey = newTop?.getKey();
            if (oldKey && newKey && blockId) {
              keyToBlock.current.delete(oldKey);
              keyToBlock.current.set(newKey, blockId);
            }
          });
          break;
        }
      }
    },
    [editor, onError],
  );

  // Flush any pending reconciliation, then persist to disk. Save is the only
  // path that writes to disk — there is no auto-save, save-on-blur, or
  // save-on-close.
  const onSave = useCallback(() => {
    onError("");
    reconcile(editor.getEditorState());
    enqueue(async () => {
      await save();
    });
  }, [editor, enqueue, onError, reconcile]);

  return (
    <Stack gap="0.75rem">
      <FormatToolbar active={active} onToggle={onToggle} />
      <DocumentBody>
        <RichTextPlugin
          contentEditable={
            <ContentEditable aria-label="Document body" />
          }
          placeholder={null}
          ErrorBoundary={LexicalErrorBoundary}
        />
        <HistoryPlugin />
        <LinkPlugin />
      </DocumentBody>
      <Stack gap="0.5rem">
        <Button onPress={onSave}>Save</Button>
      </Stack>
    </Stack>
  );
}

// Keep the supported-formats list referenced so the fixed set is documented at
// the call site even though the toolbar enumerates it internally.
export const DOCUMENT_FORMATS = ALL_FORMATS;

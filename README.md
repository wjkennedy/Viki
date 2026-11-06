# Viki prototype

## Bootstrap a research corpus

Open **Import local documents**, enter a local folder path, and select
**Discover**. Expand candidates to preview their text, check the documents you
want, then select **Import selected**. Imported notes immediately participate in
the existing search and link-suggestion flows; their original source path appears
in the note pane. Source files are never modified.

Discovery handles UTF-8 `.md`, `.markdown` and `.txt` files recursively. It skips
hidden entries, symlinks, `node_modules`, `target` and `vendor`; limits are 500
candidates, 12 directory levels and 1 MB per document. Warnings report unreadable
or oversized documents. Use a focused research folder rather than a whole disk.
Markdown is preserved verbatim. Reimporting the same source path skips it to
preserve local edits; refreshing changed sources and PDF/Word conversion are
not implemented. Imported documents are snapshots, not synchronized files.

MCP clients can call `discover_documents({root})`, inspect previews, then call
`import_documents({root, paths})` with selected relative paths. Restart the MCP
client to discover the new tools. The local HTTP server must be running.

Run `cargo test --offline` for discovery scope, content preservation and
duplicate-import regression coverage. Intake supplies the corpus to the current
term-vector prototype; learned embeddings and chunk-level RAG remain pending.

## Editing and links

Select a note and use **Edit** or **Delete**. Deletion removes incoming stored
links and asks for confirmation. **Suggest links** ranks other notes by the
prototype's term-vector cosine similarity; **Accept** persists a related link.
Use **Unlink** to remove it. Suggestions are not learned semantic embeddings.

## Local MCP testing

Build and start the Rust server with `cargo run --offline`, then register the
stdio adapter (Python 3 standard library only):

```sh
codex mcp add viki -- /opt/homebrew/bin/python3 /Users/wjk/Code/Viki/scripts/mcp.py
```

Restart the MCP client after registration. Tools: `list_notes`, `search_notes`
(keyword substring), `create_note`, `update_note`, `delete_note`. The adapter
requires the HTTP server running on localhost:8787; all clients share its
serialized writes. Link suggestions currently live in the browser UI.

Global registration and live HTTP tests were blocked by this development
sandbox's filesystem and socket permissions. Run the command above locally.

Authoring format: **Markdown with wikitext-style links and native HTML**.
See the [markup specification](docs/markup.md) for syntax and graph semantics.
The UI renders Markdown and sanitized HTML, with wiki page/section links and
missing-page creation. Formatting help is available inside the note editor.
Graph extraction and backlinks are still pending.

Run parser regression checks with `node tests/markup.cjs`.
Marked 15.0.12 (MIT) and DOMPurify 3.2.6 (Apache-2.0 OR MPL-2.0) are vendored
with license notices in their headers; no browser CDN connection is required.

Run `cargo run --offline` and open http://127.0.0.1:8787.

The Rust server serves an embedded browser UI and persists notes to
`data/notes.json`. First launch installs six example notes. Add notes,
search by keyword or term-vector similarity, combine the two, and follow
explicit links in the reading desk.

This is a functional interaction prototype. Vector search currently uses
normalized term-frequency vectors and a small synonym dictionary in the
browser; it does not use learned embeddings. MCP, editing, link proposals,
Markdown import and SQLite are not yet implemented. The next retrieval
milestone is a local embedding model and Rust-side search with evaluation
against the lexical baseline.

Commands: `cargo fmt --check`, `cargo check --offline`,
`cargo build --offline`. No frontend build or database service is required.
The server binds only to localhost. Local data is excluded from version control.
# Viki

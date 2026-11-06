# Viki: a local knowledge workspace for people and agents

## Thesis

Organizations and small teams accumulate useful context faster than they can keep it available. Wikis such as MediaWiki and Confluence solve many enterprise problems, but their scale, administration, and document-first model can be disproportionate for a local knowledge base. Meanwhile, a vector database alone can retrieve related passages but does not give people a natural place to build, inspect, correct, and trust shared context.

Viki is a small, local-first workspace where knowledge is captured as connected, attributable vertices. A vertex can represent a note, claim, decision, person, project, source, or task. Links express why items relate: supports, contradicts, follows from, belongs to, or is about. Semantic retrieval makes the graph useful even when the user does not know the right words or exact location.

The product should make a durable source of truth easier to use than a pile of files, a chat history, or an opaque RAG index.

## Two first-class users

Humans need a calm writing and browsing environment. They should be able to add a note, see its sources and neighbors, correct a relationship, and understand what an answer is based on. The graph should assist rather than demand that every contributor think in graph theory.

Agents need a bounded, inspectable interface for discovering and improving context. They should retrieve relevant vertices, propose tags and links, summarize a cluster, flag uncertainty, and draft updates. They must not silently turn inference into fact. Every automated change needs provenance, a clear author, and a review state.

MCP is the preferred agent boundary. It offers a task-oriented interface without requiring every agent to learn raw SQL or a bespoke REST surface. A small initial tool set can expose search, read, create a proposed vertex or edge, and submit a reviewed update.

## Product principles

1. **Local first.** A useful personal or team knowledge base should run with minimal infrastructure and keep its data portable.
2. **Connected context over document hoarding.** Retrieval should combine semantic similarity with explicit relationships and metadata.
3. **Provenance by default.** Facts, claims, summaries, and agent output must retain links to their originating material.
4. **Human control.** Agents can curate and propose; humans can inspect, accept, revise, or reject.
5. **Simple surfaces, rich internals.** The interface should feel closer to a lightweight wiki than a database console, even when it uses embeddings and a graph underneath.

## Prototype goal

Build a vertical slice that proves whether Viki can make a small body of knowledge easier to retrieve and maintain than plain Markdown files. The prototype is successful when a person can ingest a few local notes, find an answer with inspectable sources, and use an agent to propose a useful connection without losing control of the underlying record.

### In scope

- A local workspace containing Markdown source notes.
- A small vertex and edge model stored locally, with stable identifiers and source references.
- Chunking and embedding of note content for semantic search.
- Hybrid retrieval: keyword or metadata filtering, vector similarity, and one-hop graph expansion.
- A minimal web or terminal interface for browsing a vertex, its links, and supporting passages.
- An MCP server with read/search tools and a proposal-only write path.
- Reviewable agent proposals for links, tags, and short summaries.

### Explicitly out of scope

- Multi-tenant hosting, enterprise permissions, or broad Confluence/MediaWiki replacement.
- Autonomous publishing or deletion by agents.
- A general-purpose workflow engine, chat platform, or collaborative editor.
- Perfect ontology design; the initial model should remain easy to change.

## Smallest useful data model

Markdown is the primary authoring language, including its native inline and
block HTML support. Wikitext-style links such as `[[Page]]` and
`[[Page#Section|label]]` connect notes. These authored references provide
backlinks and navigable context without implying evidentiary relationships.
The [markup specification](docs/markup.md) defines the intended syntax and
resolution behavior.

Each vertex needs an ID, type, title, body or reference, tags, timestamps, author, provenance, and review status. Each edge needs an ID, source vertex, target vertex, relationship type, creator, evidence, confidence, and review status. Preserve original Markdown as the human-editable source; generated embeddings and indexes are derived data that can be rebuilt.

Start with a deliberately small relationship vocabulary: `related_to`, `about`, `supports`, `contradicts`, and `derived_from`. Add types only after real retrieval or browsing work reveals a need.

## Questions the prototype must answer

- Does graph expansion improve answers over vector search alone for a small local corpus?
- Can a contributor see why a result was returned and correct it quickly?
- Are agent proposals materially helpful, or do they create review overhead?
- What minimum provenance and review metadata earns user trust?
- Which MCP tools let agents contribute effectively while keeping writes safe?

## Prototype evaluation

Seed the workspace with a coherent set of 20–50 notes and a short list of realistic questions. For each question, compare plain text search, vector-only retrieval, and Viki hybrid retrieval. Record answer quality, source traceability, and time to correct a missing or misleading connection. Treat these observations as product evidence, not merely benchmark scores.

The next iteration should follow the evidence: improve retrieval, browsing, provenance, or the agent workflow—whichever most limits trust and usefulness.



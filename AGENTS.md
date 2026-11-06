# Repository Guidelines

## Project Structure & Module Organization

This repository is currently an early product-concept workspace. `thesis.md` is the source of truth for the Viki vision: a small, local, vertex-oriented RAG knowledge system for people and information agents. Keep product rationale, assumptions, and proposed workflows in Markdown at the repository root until implementation begins.

When code is introduced, use a clear top-level layout such as `src/` for application code, `tests/` for automated tests, `docs/` for longer-lived design material, and `assets/` for static files. Keep modules focused on one concern (for example, ingestion, graph/vector storage, retrieval, or MCP integration).

## Build, Test, and Development Commands

No build, runtime, package manager, or test commands are configured yet. Do not document or add placeholder commands that cannot run. When introducing a toolchain, add its canonical commands to the project README and keep them reproducible, for example `npm run dev`, `npm test`, and `npm run lint` for a Node-based implementation.

## Coding Style & Naming Conventions

Match the conventions of the selected language and configure a formatter and linter with the first implementation. Prefer descriptive, domain-oriented names: `vertexStore`, `retrieveContext`, and `mcpClient` are clearer than generic names such as `data` or `helper`.

Use lowercase, hyphen-separated names for Markdown documents (for example, `architecture-overview.md`). Keep prose direct, preserve the distinction between human and agent workflows, and update `thesis.md` when a design decision changes the core product direction.

## Testing Guidelines

There is no test framework or coverage target yet. New executable behavior should arrive with automated tests in `tests/` (or colocated tests if the chosen ecosystem strongly prefers it). Name tests after observable behavior, such as `retrieval_returns_related_vertices`. Include regression coverage for bugs involving retrieval quality, data isolation, or agent access.

## Commit & Pull Request Guidelines

This directory has no Git history, so no established commit convention exists. Use concise imperative commit subjects, such as `Add local vector index prototype` or `Document MCP retrieval flow`. Keep each commit narrowly scoped.

Pull requests should state the problem, summarize the approach, describe verification performed, and link relevant issues or design notes. Include screenshots or sample requests/responses when changing user-facing flows or agent interfaces. Never commit credentials, private knowledge-base exports, or local database contents.

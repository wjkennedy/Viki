# Viki markup

## Authoring contract

Markdown is Viki's primary writing language. Use Markdown headings, paragraphs,
emphasis, lists, quotations, code and images. Use wikitext conventions for links
between pages. This is Markdown with wiki links, not a full MediaWiki parser.

The references are [Markdown syntax](https://daringfireball.net/projects/markdown/syntax)
and Wikipedia's [wikitext cheatsheet](https://en.wikipedia.org/wiki/Help:Cheatsheet).
The choices below define Viki's intended subset; they do not imply full MediaWiki
compatibility. The prototype renders Markdown and sanitized HTML and resolves
wiki links in the browser. Link extraction and persisted backlinks remain pending.
Titles currently match exactly after trimming surrounding whitespace; duplicates
are marked ambiguous. Heading anchors replace whitespace with underscores and
suffix duplicate headings with `_2`, `_3`, and so on. Rename aliases are pending.

## Links

| Source | Meaning |
| --- | --- |
| `[[Local storage]]` | Link to a page by title |
| `[[Local storage|storage decision]]` | Link with different display text |
| `[[Local storage#Backups]]` | Link to a section of another page |
| `[[Local storage#Backups|backup policy]]` | Section link with display text |
| `[[#Backups]]` | Link to a section of the current page |
| `[https://example.org Reference]` | Wikitext external link with a label |
| `[Reference](https://example.org)` | Standard Markdown link, also supported |

Markdown reference-style links and angle-bracket autolinks remain valid. Parse
double-bracket wiki links before ordinary Markdown links; distinguish wikitext
external links by an explicit URL scheme. Code spans, code blocks, escaped
brackets and HTML attributes must not be interpreted as wiki links.

MediaWiki templates, category/file directives, automatic link trails and the
pipe trick are outside the initial subset. Use Markdown images and explicit
display text. Namespace semantics require a separate design decision.

## HTML is part of Markdown

Inline and block HTML are supported authoring syntax, not a separate document
format. For example, use `<sup>2</sup>` inline or an HTML table when needed.
Following the original Markdown rules, separate block HTML from surrounding
paragraphs with blank lines and do not assume Markdown is expanded inside those
blocks. Use HTML markup inside them. Wiki links likewise remain literal inside
raw HTML blocks for the initial implementation.

Preserve original source on save. The eventual browser renderer must sanitize
rendered HTML and URL schemes without rewriting the stored document; HTML
support does not require execution of scripts or event handlers.

## Resolution and graph behavior

Resolve page titles to stable vertex IDs within the workspace. Preserve the
authored target, optional fragment and display text. Missing targets remain
visible as unresolved links; opening one should offer to create the page.
Ambiguous titles require disambiguation instead of silently choosing a vertex.
Title normalization, rename aliases and heading-anchor generation must be
specified consistently before implementing resolution.

A resolved wiki link creates a directed document-link record and a backlink
on its target. It means the author referenced that page; it does not establish
`supports`, `contradicts` or any other evidentiary relationship. Track the source
vertex, revision and occurrence so links can be rebuilt after edits. Search can
use these references for neighborhood expansion while distinguishing them from
typed relationships and inferred similarity.

## Example note

```markdown
# Local storage

Keep notes **portable** and available offline. See [[Search]] and
[[Agent access|how agents contribute]].

## Backups

The [[Storage decision#Recovery|recovery procedure]] explains restoration.
See [the reference](https://example.org) for additional context.

<table>
<tr><th>Artifact</th><th>Role</th></tr>
<tr><td>Markdown</td><td>Authored source</td></tr>
<tr><td>Vector index</td><td>Rebuildable retrieval data</td></tr>
</table>
```

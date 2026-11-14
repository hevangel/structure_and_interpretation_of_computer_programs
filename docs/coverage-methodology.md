# Coverage methodology: what “all editions” means

## Four related source traditions

This companion distinguishes the original 1984 Scheme text, the revised 1996 Scheme text, the 2022 JavaScript adaptation, and the developing 2026 SICPy/Python adaptation. The latter two are language adaptations, **not** the third and fourth numbered Scheme editions.

## Example-level statuses

A subsection introduction describes a concept but is **not a code port**. A representative Rust program with tests is **concept coverage**. To claim **example parity**, a future record must identify the original source item's edition, chapter/subsection, program/snippet identifier, source link, and an independently authored `main.rs` with behavioral tests. To claim **exercise parity**, solutions (if supplied) require separate explicit tracking.

| Status | Meaning | May count toward exact edition parity? |
| --- | --- | --- |
| `unmapped` | No edition-specific source item has been inventoried | No |
| `inventoried` | Identified exact original snippet and source location | No |
| `implemented` | Independent Rust counterpart exists and builds | Not until tested |
| `verified` | Tests document and validate the specific item's semantics | Yes |
| `different` | Port intentionally diverges and the reason is explained | No, until parity is resolved |

**Current repository status:** 59 runnable concept examples, 22 section introductions, 95 numbered subsection notes. This is not a source-item parity count. There is currently no exhaustive edition-by-edition inventory or verified exercise-solution parity.

## Reliable source inventory workflow

1. Pin the source version and commit SHA. For 1996 Scheme/2022 JavaScript/2026 Python, start at [source-academy/sicp](https://github.com/source-academy/sicp). Distinguish the historical 1984 edition from the revised 1996 text.
2. Use the upstream build's `yarn programs` and per-section JSON exports to enumerate real program/snippet identities in each source edition. Generated `programs_scm`, `programs_js`, and `programs_py` are normally gitignored; they will not appear in a plain repository tree listing.
3. Create a machine-readable source record: `edition`, `source_commit`, `section`, `snippet_id`, `url`, `type` (expression/program/exercise), `status`, `rust_path`, and `semantic_notes`.
4. Deduplicate concepts for student navigation, **not** source item IDs. Where two editions differ in evaluation order, mutation semantics, parsing, or runtime structure, track separate source items.
5. Use tests with explicit input/output claims; don't count a section as complete merely because it has one runnable example.

## Licensing and provenance

The original Scheme and newer adaptation projects provide explicit licenses (the Scheme/JS/Python projects may differ for code and prose). This repository writes its own Rust code and explanatory text rather than importing verbatim program listings. Record attribution and links while evaluating whether any future copied source would require license compliance and redistribution obligations.

[Edition guide](editions.md) · [Source repository](https://github.com/source-academy/sicp) · [Coverage ledger](../COVERAGE.md)

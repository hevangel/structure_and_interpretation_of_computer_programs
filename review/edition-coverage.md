# SICP Edition Coverage Review & Agent Implementation Guide

This review audits the [SICP Rust Companion](../README.md) (`41` runnable `[[bin]]` examples, `22` section introductions, `95` subsection notes, and [`manifest.json`](../manifest.json)) against the local textbook transcriptions in [`learning/textbooks/WizardBook`](../../textbooks/WizardBook) and specifies concrete, agent-ready work items for every missing or stubbed system.

---

## 1. Executive Summary

- **Current baseline:** [`Cargo.toml`](../Cargo.toml) (`name = "sicp-rust-companion"`, edition `2021`, `rust-version = "1.85"`, zero external crates) declares **41 explicit `[[bin]]` targets** (`10` in Ch 1, `11` in Ch 2, `8` in Ch 3, `6` in Ch 4, `6` in Ch 5). [`manifest.json`](../manifest.json) indexes **208 entries** (`4` top-level guides, `5` chapters, `22` sections, `95` subsections, `41` example READMEs, and `41` Rust source files) for [`index.html`](../index.html).
- **Breadth vs. depth finding:** Every existing `main.rs` compiles as a small **17–79 line** teaching sketch. Across the **94 numbered subsections** shared by the 1996 Scheme and 2022 JavaScript volumes (plus Source Academy web subsection `3.2.5`), over half of the subsections lack a dedicated runnable example, and SICP's flagship multi-part software systems—frame-mapped picture painters (`§2.2.4`), data-directed dispatch (`§2.4.3`), numeric tower coercion (`§2.5.2`), symbolic polynomial algebra/GCD (`§2.5.3`), mutable tables (`§3.3.3`), full wire/agenda adders (`§3.3.4`), retractable constraints (`§3.3.5`), serialized concurrency/deadlock control (`§3.4.2`), shared memoized streams & power series (`§3.5.1–3.5.5`), full metacircular & analyzing evaluators (`§4.1.1–4.1.7`), normal-order lazy evaluation (`§4.2`), continuation-passing `amb` (`§4.3`), unification & rule query evaluation (`§4.4`), assembled register simulator (`§5.2`), vector stop-and-copy GC (`§5.3`), tail-recursive explicit-control evaluator (`§5.4`), and `preserving` lexical-addressing register compiler (`§5.5`)—are either absent or represented only by narrow toy models.
- **Actionable deliverables in this report:**
  1. **31 prioritized Rust implementation/upgrade cards (`SICP-01` – `SICP-31`, Section 6)** covering Chapters 1–5 across shared, Scheme-specific, and JavaScript-adaptation semantics.
  2. **5 verified metadata, template, and CI fixes (Section 7)** covering `rustfmt`/`clippy` CI enforcement in [`.github/workflows/rust.yml`](../.github/workflows/rust.yml#L15-L20), the `javascript-2022` vs `2022-javascript` mismatch in [`docs/editions.md`](../docs/editions.md#L11), two under-tagged `1996-scheme` examples (`ch02-picture-composition`, `ch03-stream-integration`), four non-uniform example `README.md` templates, and explicit sourcing for subsection `3.2.5 The CSE Machine`.

---

## 2. Agent Implementation Protocol & Repository Contract

An AI agent adding or upgrading examples in `learning/structure_and_interpretation_of_computer_programs` must follow the layout defined in [`docs/porting-guide.md`](../docs/porting-guide.md), [`Cargo.toml`](../Cargo.toml), [`manifest.json`](../manifest.json), and [`.github/workflows/rust.yml`](../.github/workflows/rust.yml):

### 2.1 Zero-Dependency Crate & Determinism Contract
- Keep [`Cargo.toml`](../Cargo.toml) dependency-free (`std` only, MSRV `1.85`, edition `2021`). Do **not** add external crates.
- Prefer deterministic `BTreeMap`/`BTreeSet` (or deterministic seeded LCG PRNGs) when printed output or unit assertions inspect map/set order or randomized trials.
- Expand functions and modules normally formatted via `cargo fmt --all` rather than single-line compressed `main.rs` files.

### 2.2 Four-Step Registration for New or Upgraded Examples

When adding a new example `examples/chXX/section-YY/<slug>/` (with binary name `chXX-<slug>`):

1. **Create `examples/chXX/section-YY/<slug>/main.rs`:**
   - Mirror the structure of [`examples/ch01/section-01/newton-sqrt/main.rs`](../examples/ch01/section-01/newton-sqrt/main.rs): self-contained types, concise invariant comments, a clear `fn main()` demonstration, and a `#[cfg(test)] mod tests` suite with deterministic assertions.
   - Author original code and examples; **never copy textbook prose, code listings, figures, or exercise solutions**.
2. **Create `examples/chXX/section-YY/<slug>/README.md`:**
   - Follow the canonical 37-example house structure seen in [`examples/ch01/section-01/newton-sqrt/README.md`](../examples/ch01/section-01/newton-sqrt/README.md):
     - `# <Title>`
     - `**Book concepts:** Section X.Y (Subsection X.Y.Z). **Edition applicability:** <shared | 1996-scheme | 2022-javascript>.`
     - `## What this demonstrates`
     - `## Implementation walkthrough`
     - `## Complexity and correctness`
     - `## Run it` (`cargo run --bin chXX-<slug>` and `cargo test --bin chXX-<slug>`)
     - `## Try this`
     - Footer links: `[Section introduction](../../../../chapters/chXX/section-YY/INTRO.md) · [Coverage](../../../../COVERAGE.md)`
3. **Register the `[[bin]]` target in [`Cargo.toml`](../Cargo.toml):**
   ```toml
   [[bin]]
   name = "chXX-<slug>"
   path = "examples/chXX/section-YY/<slug>/main.rs"
   ```
4. **Update [`manifest.json`](../manifest.json), [`COVERAGE.md`](../COVERAGE.md), and [`README.md`](../README.md):**
   - Insert matching `"kind": "example"` and `"kind": "source"` JSON objects into [`manifest.json`](../manifest.json) under the corresponding section (keeping `"edition"` strictly within `shared`, `1996-scheme`, or `2022-javascript`).
   - Update the total example count in [`README.md#L3`](../README.md#L3) and per-chapter/total counts in [`COVERAGE.md#L3-L12`](../COVERAGE.md#L3-L12).

### 2.3 Verification Gates
Run from `learning/structure_and_interpretation_of_computer_programs`:

```sh
cargo run --bin chXX-<slug>
cargo test --bin chXX-<slug>
cargo test --all-targets
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

*(Note: [`index.html`](../index.html#L57-L69) fetches `manifest.json` and relative Markdown/Rust files at runtime, so serve via `python3 -m http.server` or GitHub Pages when visually verifying new catalog entries.)*

---

## 3. Local Textbook Markdown Corpus Audit & Exact Line-Number Index

Two complete SICP transcriptions reside under [`learning/textbooks/WizardBook`](../../textbooks/WizardBook) (plus both earlier Prefaces embedded in both volumes):

| Volume | Local File | Lines | Structure & Verified Observations |
| --- | --- | ---: | --- |
| **1984 1st Edition** *(Scheme)* | *None locally* | — | Audited via the 1984 Preface ([`2022 L341–L360`](../../textbooks/WizardBook/SICP_JavaScript_Ed_2022.md#L341-L360), [`1996 L273`](../../textbooks/WizardBook/SICP_2nd_Ed_1996.md#L273)) and the 1996 edition revision notes (`1996 L247–L272`). |
| **1996 2nd Edition** *(Scheme)* | [`SICP_2nd_Ed_1996.md`](../../textbooks/WizardBook/SICP_2nd_Ed_1996.md) | 22,936 | Plain-text headings (`1.1.1 Expressions`, etc.); includes all 5 chapters, 22 sections, 94 subsections (`1.1.1`–`5.5.7`), References (`L20603`), and Index (`L20885`). |
| **2022 JavaScript Edition** | [`SICP_JavaScript_Ed_2022.md`](../../textbooks/WizardBook/SICP_JavaScript_Ed_2022.md) | 23,221 | Clean `#` / `##` / `###` / `####` / `#####` Markdown headings; embeds 2022 Preface (`L296`), 1996 Preface (`L324`), 1984 Preface (`L341`), Ch 1–5 (`L422–L19865`), References (`L19866`), Index (`L19975`), and List of Exercises (`L23218`). |

### Complete 22-Section & 95-Subsection Line-Number Crosswalk

*(Use `SICP_JavaScript_Ed_2022.md` for clean structural navigation and exercise bounds, and `SICP_2nd_Ed_1996.md` for Scheme-specific syntax and listings. **Note:** Subsection `3.2.5` exists in the repo's Source Academy crosswalk [`chapters/ch03/section-02/subsection-05/INTRO.md`](../chapters/ch03/section-02/subsection-05/INTRO.md) but does not appear in either local book file.)*

| Section / Subsection Range | `SICP_2nd_Ed_1996.md` | `SICP_JavaScript_Ed_2022.md` | Existing `[[bin]]` Examples |
| --- | --- | --- | --- |
| **Ch 1** Building Abstractions (`1.1`–`1.3`, 18 subsecs) | `L373–L2960` | `L422–L2816` | **10 examples** |
| — `1.1` (`1.1.1`–`1.1.8`) Elements of Programming | `L441` (`1.1.1 L463` … `1.1.8 L1203`) | `L449` (`1.1.1 L461` … `1.1.8 L1124`) | `ch01-newton-sqrt` |
| — `1.2` (`1.2.1`–`1.2.6`) Procedures & Processes | `L1347` (`1.2.1 L1369` … `1.2.6 L1951`) | `L1246` (`1.2.1 L1256` … `1.2.6 L1728`) | `ch01-factorial-processes`, `fast-exponentiation`, `euclidean-gcd`, `fibonacci-processes`, `prime-testing` |
| — `1.3` (`1.3.1`–`1.3.4`) Higher-Order Abstractions | `L2135` (`1.3.1 L2159` … `1.3.4 L2749`) | `L1892` (`1.3.1 L1914` … `1.3.4 L2453`) | `ch01-accumulate`, `fixed-point`, `simpson-integration`, `continued-fraction` |
| **Ch 2** Building Abstractions with Data (`2.1`–`2.5`, 18 subsecs) | `L2961–L7508` | `L2817–L6764` | **11 examples** |
| — `2.1` (`2.1.1`–`2.1.4`) Data Abstraction | `L3025` (`2.1.1 L3031` … `2.1.4 L3349`) | `L2856` (`2.1.1 L2862` … `2.1.4 L3107`) | `ch02-rational-numbers`, `interval-arithmetic`, `church-numerals` |
| — `2.2` (`2.2.1`–`2.2.4`) Hierarchical Data & Picture Lang | `L3499` (`2.2.1 L3539` … `2.2.4 L4713`) | `L3237` (`2.2.1 L3255` … `2.2.4 L4336`) | `ch02-recursive-trees`, `matrix-operations`, `picture-composition` |
| — `2.3` (`2.3.1`–`2.3.4`) Symbolic Data | `L5153` (`2.3.1 L5157` … `2.3.4 L5829`) | `L4695` (`2.3.1 L4699` … `2.3.4 L5218`) | `ch02-symbolic-differentiation`, `binary-search-tree`, `huffman-coding` |
| — `2.4` (`2.4.1`–`2.4.3`) Multiple Representations | `L6085` (`2.4.1 L6129` … `2.4.3 L6429`) | `L5445` (`2.4.1 L5465` … `2.4.3 L5697`) | `ch02-complex-representations` |
| — `2.5` (`2.5.1`–`2.5.3`) Systems with Generic Operations | `L6683` (`2.5.1 L6709` … `2.5.3 L7071`) | `L5926` (`2.5.1 L5936` … `2.5.3 L6257`) | `ch02-generic-arithmetic` |
| **Ch 3** Modularity, Objects & State (`3.1`–`3.5`, 19 book + `3.2.5`) | `L7509–L12144` | `L6765–L11081` | **8 examples** |
| — `3.1` (`3.1.1`–`3.1.3`) Assignment & Local State | `L7535` (`3.1.1 L7549` … `3.1.3 L7887`) | `L6782` (`3.1.1 L6790` … `3.1.3 L7109`) | `ch03-bank-account` |
| — `3.2` (`3.2.1`–`3.2.4`, web `+3.2.5`) Environment Model | `L8105` (`3.2.1 L8131` … `3.2.4 L8465`) | `L7319` (`3.2.1 L7339` … `3.2.4 L7585`; no `3.2.5`) | `ch03-lexical-environments` |
| — `3.3` (`3.3.1`–`3.3.5`) Mutable Data, Tables, Circuits, Constraints | `L8559` (`3.3.1 L8577` … `3.3.5 L9703`) | `L7706` (`3.3.1 L7718` … `3.3.5 L8840`) | `ch03-mutable-queue`, `digital-circuit-simulator`, `constraint-propagation` |
| — `3.4` (`3.4.1`–`3.4.2`) Concurrency: Time Is of the Essence | `L10153` (`3.4.1 L10177`, `3.4.2 L10273`) | `L9273` (`3.4.1 L9295`, `3.4.2 L9353`) | `ch03-serialized-transfers` |
| — `3.5` (`3.5.1`–`3.5.5`) Streams | `L10719` (`3.5.1 L10743` … `3.5.5 L11991`) | `L9748` (`3.5.1 L9760` … `3.5.5 L10789`) | `ch03-lazy-streams`, `stream-integration` |
| **Ch 4** Metalinguistic Abstraction (`4.1`–`4.4`, 17 subsecs) | `L12145–L16852` | `L11082–L16260` | **6 examples** |
| — `4.1` (`4.1.1`–`4.1.7`) Metacircular Evaluator (+ JS Parse) | `L12201` (`4.1.1 L12245` … `4.1.7 L13345`) | `L11108` (`4.1.1 L11125` … `4.1.7 L12349`) | `ch04-tiny-evaluator`, `lexical-closures`, `javascript-parser` |
| — `4.2` (`4.2.1`–`4.2.3`) Lazy Evaluation | `L13549` (`4.2.1 L13557` … `4.2.3 L13931`) | `L12573` (`4.2.1 L12577` … `4.2.3 L12973`) | `ch04-memoized-thunk` |
| — `4.3` (`4.3.1`–`4.3.3`) Nondeterministic Computing (`amb`) | `L14049` (`4.3.1 L14099` … `4.3.3 L14561`) | `L13080` (`4.3.1 L13121` … `4.3.3 L13594`) | `ch04-n-queens-search` |
| — `4.4` (`4.4.1`–`4.4.4.8`) Logic Programming | `L15019` (`4.4.1 L15083` … `4.4.4.8 L16679`) | `L14123` (`4.4.1 L14166` … `4.4.4.8 L15867`) | `ch04-logic-relations` |
| **Ch 5** Computing with Register Machines (`5.1`–`5.5`, 22 subsecs) | `L16853–L20602` | `L16261–L19865` | **6 examples** |
| — `5.1` (`5.1.1`–`5.1.5`) Designing Register Machines | `L16931` (`5.1.1 L16979` … `5.1.5 L17407`) | `L16274` (`5.1.1 L16318` … `5.1.5 L16562`) | `ch05-register-gcd` |
| — `5.2` (`5.2.1`–`5.2.4`) Register-Machine Simulator | `L17451` (`5.2.1 L17519` … `5.2.4 L18151`) | `L16602` (`5.2.1 L16658` … `5.2.4 L17243`) | `ch05-register-simulator` |
| — `5.3` (`5.3.1`–`5.3.2`) Vector Memory & Stop-and-Copy GC | `L18245` (`5.3.1 L18261`, `5.3.2 L18437`) | `L17359` (`5.3.1 L17365`, `5.3.2 L17513`) | `ch05-mark-sweep-gc` |
| — `5.4` (`5.4.1`–`5.4.4`) Explicit-Control Evaluator | `L18595` (`5.4.1 L18617` … `5.4.4 L18889`) | `L17636` (`5.4.1 L17652` … `5.4.4 L18057`) | `ch05-explicit-control` |
| — `5.5` (`5.5.1`–`5.5.7`) Compilation (+ JS Return/Tail Discipline) | `L19111` (`5.5.1 L19167` … `5.5.7 L20387`) | `L18284` (`5.5.1 L18319` … `5.5.7 L19382`) | `ch05-bytecode-compiler`, `javascript-tail-calls` |

---

## 4. Edition Crosswalk (`1984 1e` vs `1996 2e Scheme` vs `2022 JS Adaptation`)

Per the **1996 Preface** ([`2022 L324–L340`](../../textbooks/WizardBook/SICP_JavaScript_Ed_2022.md#L324-L340)) and **2022 Preface** ([`2022 L296–L319`](../../textbooks/WizardBook/SICP_JavaScript_Ed_2022.md#L296-L319)):

| Dimension | 1984 1st Edition *(Scheme)* | 1996 2nd Edition *(Scheme)* | 2022 JavaScript Adaptation |
| --- | --- | --- | --- |
| Core focus | Expression-based Scheme fundamentals; initial evaluator & compiler | Elevated **time & state** theme; redesigned generic arithmetic, interpreters, register simulator, compiler; IEEE Scheme compliant | Direct adaptation of 1996's 5 chapters and 22 sections to ECMAScript 2015+/2020 strict mode using `sicp` list primitives |
| Chapter 2 highlights | Symbolic lists, differentiation, early generic ops | Added `§2.2.4` Henderson picture language; redesigned `§2.5` generic arithmetic & symbolic algebra | Retains all `§2.1–2.5` sections including `§2.2.4` (`L4336–L4694`); strings/tokens replace Scheme quoted symbols in `§2.3.1` |
| Chapter 3 highlights | Local state, streams, queues, circuits | Added **`§3.4` Concurrency & Serializers**, expanded **`§3.5` Streams**; explicit time models | Retains `§3.1–3.5` (including `§3.5.4` `integral`/`solve` at `L10658–L10788`); block-scoped `const`/`let` declarations; Source Academy adds web supplement `3.2.5 The CSE Machine` |
| Chapter 4 highlights | Original metacircular evaluator & logic queries | Separated `§4.1.7` syntactic analysis; added **`§4.2` Lazy Evaluation** & **`§4.3` `amb` Nondeterminism** | **`§4.1.2` replaces Scheme `read`/lists with `parse()`** (`L11342`), models **statement blocks & `return` value propagation** (`§4.1.1`, `§4.1.3`), and uses `unparse` in `§4.4.4.7` (`L15732`) |
| Chapter 5 highlights | Register machines, explicit-control evaluator, compiler | Redesigned **`§5.2` simulator**, explicit stop-and-copy GC `§5.3`, **`§5.5` preserving compiler** & compiled-code interface | Explicit **block environments & `return` compilation** (`§5.4.2–5.4.3`, `§5.5.3`) and tail-call stack bounds via explicit registers / host trampoline concepts |

---

## 5. Section-by-Section Coverage Audit (Chapters 1–5)

Status legend: **Covered** (faithful runnable implementation of core section mechanism), **Partial (Stub)** (tiny 18–79 line illustration omitting the book's central abstraction/data-directed architecture), **Missing** (no dedicated runnable example).

- **Chapter 1 (`§1.1–1.3`, 10 binaries):**
  - `§1.1` (`ch01-newton-sqrt` = **Partial**): covers `1.1.7` square root; lacks explicit applicative vs normal-order evaluator trace (`1.1.5`) and block-scoped internal closure rules (`1.1.8`). → **`SICP-01`**
  - `§1.2` (`ch01-factorial`, `fast-exp`, `gcd`, `fibonacci`, `prime` = **Partial**): `1.2.1`, `1.2.4`, `1.2.5` covered; `1.2.2` omits counting change (`L1445`) & $O(\lg n)$ matrix/transform Fibonacci (`Ex 1.19`); `1.2.6` tests deterministic trial division (`22 lines`), omitting Fermat (`expmod`), Carmichael witnesses (`Ex 1.27`), and Miller–Rabin (`Ex 1.28`). → **`SICP-02`**, **`SICP-03`**
  - `§1.3` (`ch01-accumulate`, `fixed-point`, `simpson`, `continued-fraction` = **Partial**): covers `1.3.1` & `1.3.3` basics; omits `filtered-accumulate` (`Ex 1.33`), half-interval bisection (`1.3.3`), Euler $e$/$\tan x$ continued fractions (`Ex 1.38–1.39`), and **all of `1.3.4` procedures/functions as returned values** (`deriv`, `newton-transform`, `smooth`, `repeated`, `iterative-improve`). → **`SICP-04`**, **`SICP-05`**

- **Chapter 2 (`§2.1–2.5`, 11 binaries):**
  - `§2.1` (`rational-numbers`, `interval-arithmetic`, `church-numerals` = **Partial**): interval arithmetic (`29 lines`) implements only `mul` and `reciprocal` (missing `add`, `sub`, `div`, `center_percent`, 9-sign `mul`, and the `par1` vs `par2` identity dependency bug `Ex 2.14–2.16`). → **`SICP-06`**
  - `§2.2` (`recursive-trees`, `matrix-operations`, `picture-composition` = **Partial**): `2.2.3` omits `fold-left`/`fold-right`, `flatmap` prime pairs, & N-queens nested mapping; **`2.2.4` `picture-composition` (`24 lines`) only concatenates ASCII string rows**, omitting vector frames (`frame-coord-map`), `segments->painter`, affine `transform-painter`, `corner-split`, `square-of-four`, and `square-limit`. → **`SICP-07`**, **`SICP-08`**
  - `§2.3` (`symbolic-differentiation`, `binary-search-tree`, `huffman-coding` = **Partial**): `huffman-coding` (`75 lines`) is solid; `2.3.2` lacks exponentiation (`Ex 2.56`), variadic terms (`Ex 2.57`), & infix parsing (`Ex 2.58`); `2.3.3` BST (`34 lines`) supports only `insert` & `contains` (missing $O(n)$ `list->tree`, `tree->list`, and $O(n)$ set `union`/`intersection` `Ex 2.63–2.65`). → **`SICP-09`**
  - **`§2.4` (`complex-representations`, `25 lines` = Stub)**: uses a closed 2-variant enum instead of **tagged data + `(op, type-tag)` data-directed dispatch table (`put`/`get`/`apply-generic`) vs message passing** (`2.4.2–2.4.3`). → **`SICP-10`**
  - **`§2.5` (`generic-arithmetic`, `39 lines` = Stub)**: promotes 3 hardcoded variants (`Int`, `Rat`, `Real`), missing Complex integration, general coercion (`apply-generic` tower `raise`/`project`/`drop`, `2.5.1–2.5.2`) and **all of `§2.5.3` symbolic algebra** (polynomial arithmetic, dense/sparse term lists, pseudo-remainder GCD, and rational functions). → **`SICP-11`**, **`SICP-12`**

- **Chapter 3 (`§3.1–3.5`, 8 binaries):**
  - `§3.1` (`bank-account`, `32 lines` = **Partial**): covers `3.1.1`; omits `3.1.2` Monte Carlo Cesàro $\pi$ estimation & `make-rand` state reset. → **`SICP-13`**
  - `§3.2` (`lexical-environments`, `39 lines` = **Partial**): covers frame lookup only (`values: HashMap`); omits mutable shared bindings (`set!`), closure frame linkage, and **`3.2.5` Control–Stash–Environment (CSE) step machine** referenced in [`chapters/ch03/section-02/subsection-05/INTRO.md`](../chapters/ch03/section-02/subsection-05/INTRO.md). → **`SICP-14`**
  - `§3.3` (`mutable-queue`, `digital-circuit-simulator`, `constraint-propagation` = **Partial / Gaps**):
    - `3.3.1` cycle detection (`count-pairs` / Floyd `Ex 3.18–3.19`) and `3.3.2` `Deque` (`Ex 3.23`) are **Missing** → **`SICP-15`**.
    - **`3.3.3` Representing Tables (1D, 2D, N-dimensional local dispatch tables & memoization) is completely Missing** → **`SICP-16`**.
    - `3.3.4` circuit simulator (`46 lines`) hardcodes a single 3-wire AND gate; lacks wires with subscriber closures, segmented agenda, `inverter`, `or-gate`, `half-adder`, `full-adder`, and `ripple-carry-adder` → **`SICP-17`**.
    - `3.3.5` constraint propagator (`60 lines`) lacks `Connector` informants & `forget_value` retraction, `averager`, and `squarer` → **`SICP-18`**.
  - `§3.4` (`serialized-transfers`, `36 lines` = **Partial**): locks one fixed array mutex rather than modeling the book's `make-serializer` binary mutex abstraction, race interleaving enumeration (`Ex 3.38`), and **deadlock-avoiding numbered `serialized-exchange` (`Ex 3.48`)** → **`SICP-19`**.
  - `§3.5` (`lazy-streams`, `stream-integration` = **Stub**): uses Rust `Iterator` (single-pass unmemoized, recomputes exponentially on recursive streams!), missing shared **memoized `delay`/`force` streams (`fibs`, `primes` sieve, Hamming numbers)**, **`3.5.2` power series (`invert-unit-series`, $\sin/\cos/\tan$)**, **`3.5.3` Euler/tableau stream acceleration & Ramanujan infinite pairs**, and **`3.5.4` second-order feedback ODE `solve`**. → **`SICP-20`**, **`SICP-21`**

- **Chapter 4 (`§4.1–4.4`, 6 binaries):**
  - **`§4.1` (`tiny-evaluator`, `lexical-closures`, `javascript-parser` = Stubs)**: lacks `quote`, `if`/`cond`, `begin`, `set!`, recursive `define`/`scan-out-defines`, derived `let`/`let*`, S-expr parser, **`4.1.7` separating syntactic analysis from execution**, and JS blocks/`return` unwinding (`§4.1.1–4.1.4 JS`). → **`SICP-22`**, **`SICP-23`**, **`SICP-24`**
  - **`§4.2` (`memoized-thunk`, `32 lines` = Stub)**: tests one single thunk struct; has **no lazy-evaluation interpreter** (`actual-value`, call-by-name vs call-by-need, `4.2.3` lazy cons lists). → **`SICP-25`**
  - **`§4.3` (`n-queens-search`, `25 lines` = Stub)**: standard DFS backtracking; **no continuation-passing `amb` evaluator**, `permanent-set!`, `if-fail`, logic puzzles, or `amb` natural-language parser. → **`SICP-26`**
  - **`§4.4` (`logic-relations`, `38 lines` = Stub)**: matches one flat pattern against 2-tuples; has **no rules, variable-renaming unification, occurs check, compound `and`/`or`/`not`/`lisp-value` frame streams, or recursive logic evaluator (`§4.4.4`)**. → **`SICP-27`**

- **Chapter 5 (`§5.1–5.5`, 6 binaries):**
  - `§5.1` (`register-gcd`, `26 lines` = **Partial**): models hardcoded `while` loop without controller instruction diagram/stack discipline for recursion (`5.1.4`). (Covered cleanly together with `SICP-28`.)
  - **`§5.2` (`register-simulator`, `46 lines` = Stub)**: executes 6 pre-built enum opcodes over `[i64; 3]`; lacks register-machine description language assembler, arbitrary named registers/stacks/labels, operation primitives (`op`), stack/instruction statistics (`5.2.4`), and recursive machine controllers. → **`SICP-28`**
  - **`§5.3` (`mark-sweep-gc`, `46 lines` = Stub)**: does stop-the-world `Vec<Option<Node>>` mark-sweep (briefly noted in a 1996 footnote), instead of **`§5.3` vector-memory `the-cars`/`the-cdrs` typed pointers and Cheney stop-and-copy GC with `broken-heart` forwarding (`5.3.1–5.3.2`)**. → **`SICP-29`**
  - **`§5.4` (`explicit-control`, `37 lines` = Stub)**: evaluates only `Expr::Add(Box, Box)`; omits the **7-register explicit-control evaluator** (`exp, env, val, continue, proc, argl, unev`), procedure application, tail recursion measurement (`5.4.4`), and error handling. → **`SICP-30`**
  - **`§5.5` (`bytecode-compiler`, `44 lines` & `javascript-tail-calls`, `22 lines` = Stubs)**: compiles `+`/`*` literals to a 3-instruction stack VM; omits **`§5.5.1–5.5.7` target-register/linkage code generator**, `InstructionSequence` `needs`/`modifies` static analysis, `preserving` register saving, conditionals, lambdas/calls, lexical addressing (`(frame, offset)`), and compiled-to-evaluator interop! → **`SICP-31`**

---

## 6. Prioritized Work-Item Backlog & Module Specifications (`SICP-01` – `SICP-31`)

Implement **`P1`** items first (core multi-subsection systems missing or stubbed), followed by **`P2`** items. Each item either **Upgrades** an existing binary in place or **Adds** a new `examples/chXX/section-YY/<slug>/` binary registered per Section 2.2.

### Track A: Chapter 1 (`SICP-01` – `SICP-05`)

#### `SICP-01` [P2] Evaluation Orders & Block Scope Simulation
- **Add:** `ch01-evaluation-orders` at `examples/ch01/section-01/evaluation-orders/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§1.1.5, §1.1.8` (`1996 L745, L1203`; `2022 L717, L1124`)
- **Design:** Small expression AST (`Num`, `Var`, `Call`, `If`, `Block`) with two pure step evaluators (`eval_applicative` vs `eval_normal`) tracking evaluation count and diverging-argument behavior (`if p() == 0 then 0 else fail()`, demonstrating `Ex 1.5` and `1.1.8` lexical internal declarations). Verify applicative diverges/exceeds step bound on the non-terminating argument while normal order terminates cleanly in finite steps.

#### `SICP-02` [P1] Counting Change, Pascal & $O(\lg n)$ Transformed Fibonacci
- **Add:** `ch01-counting-change` at `examples/ch01/section-02/counting-change/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§1.2.2, §1.2.4` (`1996 L1533, L1743`; `2022 L1384, L1445, L1657`, Exercises `1.11`, `1.12`, `1.19`)
- **Design:** Implement `count_change_tree(amount, coins)` + DP `count_change_dp`, `pascal(row, col)`, and $O(\lg n)$ `fib_log(n)` using the `(p, q) -> (p^2+q^2, 2pq+q^2)` transformation (`Ex 1.19`). Assert `count_change_tree(100, &[50,25,10,5,1]) == 292 == count_change_dp(...)`, `fib_log(n) == fib_iter(n)` for `0..=90`, and Pascal binomial identities.

#### `SICP-03` [P1] Fermat Primality, Carmichael Witnesses & Miller–Rabin
- **Add:** `ch01-fermat-miller-rabin` at `examples/ch01/section-02/fermat-miller-rabin/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§1.2.6` (`1996 L1951`; `2022 L1728–L1891`, Exercises `1.27–1.28`)
- **Design:** Implement `expmod(base, exp, m)` using `u128` modular squaring, deterministic Fermat pass (`all_coprime_fermat_pass`), Carmichael detector proving `561, 1105, 1729, 2465, 2821, 6601` pass Fermat for every coprime base yet fail `miller_rabin_witness`, and `is_probable_prime_mr` matching trial division across `2..10_000`.

#### `SICP-04` [P2] Filtered Accumulation, Half-Interval Roots & Continued Fractions
- **Add:** `ch01-half-interval-filtered` at `examples/ch01/section-03/half-interval-filtered/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§1.3.1, §1.3.3` (`1996 L2159, L2553`; `2022 L1914, L2298`, Exercises `1.33, 1.37–1.39`)
- **Design:** Implement `filtered_accumulate`, `half_interval_method(f, a, b)` (checking opposite-sign contract), `fixed_point_damped`, Euler's $e - 2$ continued fraction (`1, 2, 1, 1, 4, ...`), and Lambert's `tan_cf(x, k)`. Verify $\sin x = 0$ on $[2, 4] \to \pi$, Euler $e$, and `tan_cf(0.7, 20)` matching `0.7_f64.tan()`.

#### `SICP-05` [P1] First-Class Returned Functions: Derivatives, Newton Transform, Smoothing & Iterative Improvement
- **Add:** `ch01-newton-transform-smoothed` at `examples/ch01/section-03/newton-transform-smoothed/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§1.3.4` (`1996 L2749`; `2022 L2453–L2640`, Exercises `1.40–1.46`)
- **Design:** Use Rust `Rc<dyn Fn(f64) -> f64>` combinators to return first-class functions: `average_damp`, `deriv(g, dx)`, `newton_transform`, `newtons_method`, `compose`, `repeated(f, n)`, `smooth(f, dx)`, `nth_root(x, n)` via $\lfloor \lg n \rfloor$ average damps (`Ex 1.45`), and `iterative_improve(good_enough, improve)` (`Ex 1.46`). Test cubic roots (`Ex 1.40`) and $x^7 = 128$ convergence.

### Track B: Chapter 2 (`SICP-06` – `SICP-12`)

#### `SICP-06` [P2] Complete Interval Arithmetic & Dependency Identity (`par1` vs `par2`)
- **Upgrade:** `ch02-interval-arithmetic` ([`examples/ch02/section-01/interval-arithmetic/main.rs`](../examples/ch02/section-01/interval-arithmetic/main.rs)) | **Edition:** `shared`
- **Anchor:** `§2.1.4` (`1996 L3349`; `2022 L3107`, Exercises `2.7–2.16`)
- **Design:** Add `add`, `sub`, `div` (guarding zero-straddling intervals), `make_center_percent(c, pct)`, 9-case endpoint-sign `mul_fast`, and parallel-resistor `par1(r1, r2)` vs `par2(r1, r2)` demonstrating width inflation when repeated interval variables appear algebraically.

#### `SICP-07` [P2] Binary Mobiles, Folds & Nested Flatmap Permutations
- **Add:** `ch02-nested-mappings-mobiles` at `examples/ch02/section-02/nested-mappings-mobiles/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§2.2.2–2.2.3` (`1996 L3919, L4163`; `2022 L3606, L3848, L4211`, Exercises `2.29, 2.38–2.42`)
- **Design:** Implement recursive `Mobile`/`Branch` with `total_weight` & `is_balanced` (`Ex 2.29`), generic `fold_right` vs `fold_left` (`Ex 2.38`), `flatmap`, `prime_sum_pairs(n)`, and `permutations(slice)` via flatmap removal. Assert commutativity/associativity criteria for folds and permutation cardinality $n!$.

#### `SICP-08` [P1] Geometric Frame-Mapped Picture Language & Painters
- **Add:** `ch02-frame-painters` at `examples/ch02/section-02/frame-painters/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§2.2.4` (`1996 L4713–L5152`; `2022 L4336–L4694`, Exercises `2.44–2.52`)
- **Design:** Build `Vect(f64, f64)`, `Segment(Vect, Vect)`, `Frame { origin, edge1, edge2 }`, `frame_coord_map(&Frame) -> impl Fn(Vect) -> Vect`, deterministic `Painter` closure (`Rc<dyn Fn(&Frame, &mut Canvas)>`), `segments_to_painter`, `transform_painter`, `flip_vert`, `flip_horiz`, `rotate90`, `beside`, `below`, `right_split`, `up_split`, `corner_split`, `square_of_four`, and `square_limit`. Verify transformed coordinates lie within `[0, 1]^2` and `rotate90` composed 4 times equals identity!

#### `SICP-09` [P2] Balanced Ordered-Set BST Operations & Infix Differentiation
- **Add:** `ch02-ordered-sets-infix-deriv` at `examples/ch02/section-03/ordered-sets-infix-deriv/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§2.3.2–2.3.3` (`1996 L5287, L5513`; `2022 L4795, L4968`, Exercises `2.56–2.65`)
- **Design:** Extend symbolic differentiation with exponentiation `Pow(base, exp)` (`Ex 2.56`) + infix parser/formatter (`Ex 2.58`), and pair it with `tree_to_list` (`Ex 2.63`), $O(n)$ `list_to_balanced_tree` (`partial-tree` `Ex 2.64`), and $O(n)$ sorted-list `union_set`/`intersection_set` on BSTs (`Ex 2.65`). Assert log-height invariants on large sorted inputs (`size 1..1023`).

#### `SICP-10` [P1] Data-Directed Dispatch Table (`put`/`get`/`apply_generic`)
- **Add:** `ch02-data-directed-dispatch` at `examples/ch02/section-04/data-directed-dispatch/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§2.4.2–2.4.3` (`1996 L6257–L6682`; `2022 L5558–L5925`, Exercises `2.73–2.76`)
- **Design:** Implement an explicit `(Op, Vec<TypeTag>) -> DynFn` dispatch registry (`DispatchTable`), install rectangular & polar complex packages + additive symbolic-differentiation rule plugins (`Ex 2.73`), run `apply_generic(op, &[Datum])`, and contrast with message-passing objects (`Ex 2.75–2.76`). Verify adding a new type or rule requires zero modification to core dispatch code!

#### `SICP-11` [P1] Four-Level Numeric Tower, Coercion, `raise` & `drop`
- **Add:** `ch02-numeric-tower-coercion` at `examples/ch02/section-05/numeric-tower-coercion/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§2.5.1–2.5.2` (`1996 L6709–L7070`; `2022 L5936–L6256`, Exercises `2.77–2.85`)
- **Design:** Model all 4 ordered tower levels (`Int < Rat < Real < Complex` using tagged complex from `SICP-10`), `raise` upward coercions across multi-step gaps (`Int + Complex`), `project` downward tests, `drop` canonical simplification (`Complex(Rat(6,1), 0) => Int(6)`), and generic `equ`/`is_zero`.

#### `SICP-12` [P1] Symbolic Polynomial Algebra, Pseudo-Remainder GCD & Rational Functions
- **Add:** `ch02-symbolic-polynomials` at `examples/ch02/section-05/symbolic-polynomials/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§2.5.3` (`1996 L7071–L7508`; `2022 L6257–L6764`, Exercises `2.87–2.97`)
- **Design:** Represent sparse term lists `BTreeMap<usize, i64>` (descending/ascending power order) over arbitrary variable symbols plus polynomial addition, subtraction, multiplication, long division returning `(quotient, remainder)` (`Ex 2.91`), `pseudoremainder_terms` integer-preserving GCD (`gcd_poly`, `Ex 2.94–2.97`), and reduced rational functions `reduce_poly_fraction(num, den)`. Verify $(x^2 - 1)/(x - 1) = x + 1$ and $(p_1 q + p_2 p_1)$ fraction reduction.

### Track C: Chapter 3 (`SICP-13` – `SICP-21`)

#### `SICP-13` [P2] Monte Carlo Simulation, Cesàro $\pi$ & Resettable PRNG
- **Add:** `ch03-monte-carlo-pi` at `examples/ch03/section-01/monte-carlo-pi/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§3.1.2` (`1996 L7757`; `2022 L7013`, Exercises `3.1–3.6`)
- **Design:** Implement stateful `Rand::Generate` / `Rand::Reset(seed)`, generic `monte_carlo(trials, experiment)`, Cesàro coprime `gcd` probability $\sqrt{6 / p} \approx \pi$, and 2D unit-disk `estimate_integral`. Assert seeded reproducibility and reset determinism.

#### `SICP-14` [P2] Environment Model Mutation & CSE Machine Step Evaluator
- **Add:** `ch03-cse-environment-machine` at `examples/ch03/section-02/cse-environment-machine/{main.rs,README.md}` | **Edition:** `2022-javascript` *(also relates to `1996 §3.2.1–3.2.4`)*
- **Anchor:** `§3.2.1–3.2.5` (`1996 L8105–L8558`; `2022 L7319–L7705`; [`subsection-05/INTRO.md`](../chapters/ch03/section-02/subsection-05/INTRO.md))
- **Design:** Store lexical frames in a mutable arena (`Vec<Frame>`) so closures share pointers to enclosing bindings across calls (`make_withdraw` bank account closure demo!), plus an explicit **Control–Stash–Environment (`C, S, E`)** step-by-step state machine tracing every push/pop and frame extension. Verify two closures from the same factory maintain independent local state while sharing outer frame mutations.

#### `SICP-15` [P2] Mutable Pair Cycle Detection & `O(1)` Deque
- **Add:** `ch03-cycle-detection-deque` at `examples/ch03/section-03/cycle-detection-deque/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§3.3.1–3.3.2` (`1996 L8577, L8875`; `2022 L7718, L8027`, Exercises `3.16–3.19, 3.23`)
- **Design:** Index-arena `ConsCell` heap (`car`, `cdr` mutators `set_car!`/`set_cdr!`) showing `count_pairs` on 3-cell graphs returning `3`, `4`, `7`, or cycle vs visited-set unique count (`Ex 3.16–3.17`) and Floyd's $O(1)$-space tortoise-and-hare cycle detector (`Ex 3.19`), paired with an `O(1)` arena doubly-linked `Deque` (`Ex 3.23`).

#### `SICP-16` [P1] 1D/2D Local Tables, Trie Lookup & Dynamic Memoization
- **Add:** `ch03-local-tables-memoize` at `examples/ch03/section-03/local-tables-memoize/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§3.3.3` (`1996 L9029–L9260`; `2022 L8174–L8403`, Exercises `3.24–3.27`)
- **Design:** Implement recursive multi-key `Table<K, V>` (`insert(&[K], V)`, `lookup(&[K])`) with pluggable equality tolerance (`Ex 3.24`), and `memoize` wrapping recursive Fibonacci/LCS (`Ex 3.27`) with call counters proving $O(n)$ invocations instead of exponential blowup.

#### `SICP-17` [P1] Full Agenda Digital-Circuit Simulator & Ripple-Carry Adder
- **Upgrade:** `ch03-digital-circuit-simulator` ([`examples/ch03/section-03/digital-circuit-simulator/main.rs`](../examples/ch03/section-03/digital-circuit-simulator/main.rs)) | **Edition:** `shared`
- **Anchor:** `§3.3.4` (`1996 L9261–L9702`; `2022 L8404–L8839`, Exercises `3.28–3.32`)
- **Design:** Upgrade from hardcoded 3-wire AND logic to composable `Wire` objects (`Rc<RefCell<WireState>>`) with subscriber action callbacks, `Agenda` time-bucket queue, `inverter` (delay `2`), `and_gate` (delay `3`), `or_gate` (delay `5`), `half_adder`, `full_adder`, and parameterized $n$-bit `ripple_carry_adder` verified across all $4$-bit additions `(a + b) mod 16` and exact critical-path propagation timestamps.

#### `SICP-18` [P2] Retractable Constraint Propagation (`forget!`, `averager`, `squarer`)
- **Upgrade:** `ch03-constraint-propagation` ([`examples/ch03/section-03/constraint-propagation/main.rs`](../examples/ch03/section-03/constraint-propagation/main.rs)) | **Edition:** `shared`
- **Anchor:** `§3.3.5` (`1996 L9703–L10152`; `2022 L8840–L9272`, Exercises `3.33–3.37`)
- **Design:** Add `Connector` ownership (`informant: Option<ConstraintId>`), `set_value` & `forget_value` (retracting only values propagated downstream from the forgotten source without erasing independent user constraints!), plus `averager` (`Ex 3.33`), `squarer` (`sqrt` solving backward, `Ex 3.35`), and `c+`/`c*`/`cv` combinators (`Ex 3.37`).

#### `SICP-19` [P2] Concurrency Interleavings & Deadlock-Free Serialized Exchange
- **Upgrade:** `ch03-serialized-transfers` ([`examples/ch03/section-04/serialized-transfers/main.rs`](../examples/ch03/section-04/serialized-transfers/main.rs)) | **Edition:** `shared`
- **Anchor:** `§3.4.1–3.4.2` (`1996 L10153–L10718`; `2022 L9273–L9747`, Exercises `3.38–3.48`)
- **Design:** Enumerate all distinct balance states achievable under unserialized interleavings of concurrent transfers (`Ex 3.38`), model a `Serializer` wrapper with ordered account IDs (`id_min < id_max`), and prove `serialized_exchange` completes without deadlock under simultaneous opposing cross-thread transfers `swap(A, B)` and `swap(B, A)` (`Ex 3.48`).

#### `SICP-20` [P1] Memoized Lazy Streams, Sieve & Formal Power Series
- **Add:** `ch03-memoized-streams-series` at `examples/ch03/section-05/memoized-streams-series/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§3.5.1–3.5.2` (`1996 L10743–L11400`; `2022 L9760–L10279`, Exercises `3.50–3.62`)
- **Design:** Build a shared memoized `Stream<T>` (`Rc<RefCell<StreamNode<T>>>` caching forced tails once!) and implement `stream_map`, `stream_filter`, `add_streams`, `mul_streams`, `partial_sums`, `sieve` (infinite primes), Hamming merge (`Ex 3.56`), and power-series combinators (`integrate_series`, `mul_series`, `invert_unit_series`, `div_series`, $\exp$, $\sin$, $\cos$, $\tan$). Verify `sin^2(x) + cos^2(x) = 1` coefficients up to degree 12 and once-only thunk forcing count!

#### `SICP-21` [P2] Accelerated Streams, Infinite Pairs & Delayed-Feedback ODE Solver
- **Add:** `ch03-stream-acceleration-pairs` at `examples/ch03/section-05/stream-acceleration-pairs/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§3.5.3–3.5.5` (`1996 L11401–L12144`; `2022 L10280–L11081`, Exercises `3.64–3.82`)
- **Design:** Implement `sqrt_stream`, `euler_transform`, `make_tableau` + `accelerated_sequence` converging to $\pi/4$ in ~8 terms instead of thousands (`§3.5.3`), `interleave` + `pairs` + weighted pair merge producing Ramanujan's `1729` and `4104` (`Ex 3.71`), delayed-integrand feedback `solve(f, y0, dt)` (`§3.5.4`), and functional stream Monte Carlo (`§3.5.5`).

### Track D: Chapter 4 (`SICP-22` – `SICP-27`)

#### `SICP-22` [P1] Full Metacircular-Style Evaluator (`quote`, `cond`, `define`, `set!`, Derived `let`, `scan_out_defines`)
- **Add:** `ch04-metacircular-evaluator` at `examples/ch04/section-01/metacircular-evaluator/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§4.1.1–4.1.6` (`1996 L12201–L13344`; `2022 L11108–L12348`, Exercises `4.1–4.21`)
- **Design:** Parse S-expressions into a full `Expr` AST and evaluate in shared `Rc<RefCell<Frame>>` environments supporting `Number`, `Bool`, `Symbol`, `Pair`, `Nil`, `Primitive`, and `CompoundClosure`; handle `quote`, `if`, `cond`, `and`/`or`, `begin`, `set!`, mutual recursive `define` via `scan_out_defines` (`§4.1.6`), derived `let`/`let*`, and Y-combinator recursion without `define` (`Ex 4.21`).

#### `SICP-23` [P1] Analyzing Evaluator (Separating Syntax Analysis from Execution)
- **Add:** `ch04-analyzing-evaluator` at `examples/ch04/section-01/analyzing-evaluator/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§4.1.7` (`1996 L13345–L13548`; `2022 L12349–L12572`, Exercises `4.22–4.24`)
- **Design:** Translate every AST form once in `analyze(&Expr) -> ExecFn` (`Rc<dyn Fn(&Env) -> Result<Value, Error>>`) capturing already-compiled sub-closures; add analysis vs execution step counters demonstrating zero AST re-traversal inside iterative loops (`fact(10)`, `let` expansion).

#### `SICP-24` [P2] JavaScript Block, Declaration & `return`-Statement Parser/Evaluator
- **Upgrade:** `ch04-javascript-parser` ([`examples/ch04/section-01/javascript-parser/main.rs`](../examples/ch04/section-01/javascript-parser/main.rs)) | **Edition:** `2022-javascript`
- **Anchor:** `§4.1.1–4.1.4` (`2022 L11125–L12162`)
- **Design:** Upgrade from arithmetic expressions to statement-oriented JS subset: lexer/Pratt+recursive parser for `const`/`let`/`function` declarations, blocks `{ ... }` with sequential declaration scanning (`§4.1.6` JS), `if/else`, and `ReturnValue` control-flow propagation that short-circuits surrounding statements inside a function call while preserving block lexical scopes.

#### `SICP-25` [P1] Normal-Order Lazy Evaluator with Memoized Thunks & Lazy Lists
- **Add:** `ch04-lazy-evaluator` at `examples/ch04/section-02/lazy-evaluator/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§4.2.1–4.2.3` (`1996 L13549–L14048`; `2022 L12573–L13079`, Exercises `4.25–4.34`)
- **Design:** Extend interpreter values with `Value::Thunk(Rc<RefCell<ThunkState>>)` where `ThunkState` transitions `Unevaluated(Expr, Env) -> Evaluated(Value)`; force via `actual_value` only at primitive applications, `if` predicates, and operator position while passing unevaluated argument thunks to compound functions; run `unless(false, 1, 1/0)` and procedural lazy `cons`/`car`/`cdr` infinite `ones`/`integers` streams inside the interpreted language itself!

#### `SICP-26` [P1] Nondeterministic Continuation-Passing `amb` Evaluator
- **Add:** `ch04-amb-evaluator` at `examples/ch04/section-03/amb-evaluator/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§4.3.1–4.3.3` (`1996 L14049–L15018`; `2022 L13080–L14122`, Exercises `4.35–4.54`)
- **Design:** Implement `analyze_amb` producing continuation-passing executors `Fn(&Env, SucceedFn, FailFn)` supporting `amb(c1, ..., ck)`, `require`, reversible `set!` (undone in `fail` thunk!) vs `permanent-set!` (`Ex 4.51`), `if-fail` (`Ex 4.52`), and interactive `try-again` enumeration collecting all solutions to Pythagorean triples, the multiple-dwelling puzzle (`§4.3.2`), and an ambiguous DCG sentence parser.

#### `SICP-27` [P1] Logic Query Engine with Rules, Variable-Renaming Unification & Frame Streams
- **Add:** `ch04-query-unification-engine` at `examples/ch04/section-04/query-unification-engine/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§4.4.1–4.4.4.8` (`1996 L15019–L16852`; `2022 L14123–L16260`, Exercises `4.55–4.79`)
- **Design:** Implement nested term patterns (`Atom`, `Var(id, name)`, `Pair`), database of assertions & `(rule conclusion body)` entries, `pattern_match`, full two-sided `unify_match` with `depends_on` cyclic occurs check (`§4.4.4.4`), rule variable alpha-renaming (`rule_counter`), and `qeval` stream combinators (`and` flatmap, `or` interleave, `not` filter, `unique`, `eval_pred`). Test multi-hop `outranked_by`, `append_to_form`, and nested cyclic failure!

### Track E: Chapter 5 (`SICP-28` – `SICP-31`)

#### `SICP-28` [P1] Full Register-Machine Assembler, Stack Monitor & Recursive Controllers
- **Upgrade:** `ch05-register-simulator` ([`examples/ch05/section-02/register-simulator/main.rs`](../examples/ch05/section-02/register-simulator/main.rs)) | **Edition:** `shared`
- **Anchor:** `§5.1.4, §5.2.1–5.2.4` (`1996 L17253–L18244`; `2022 L16470–L17358`, Exercises `5.4–5.19`)
- **Design:** Replace the 3-register fixed enum with general named `RegisterTable`, explicit `Stack` tracking `total_pushes`, `current_depth`, and `max_depth`, text/AST assembler resolving branch/goto labels (`assign`, `test`, `branch`, `goto(label|reg)`, `save`, `restore`, `perform`), pluggable primitive operations `op`, instruction counting + trace hook (`5.2.4`), and assembled recursive `fact` and double-recursive `fib` controllers verified against analytical stack formulas.

#### `SICP-29` [P1] Vector-Memory Pairs & Cheney Stop-and-Copy Garbage Collector
- **Add:** `ch05-stop-and-copy-gc` at `examples/ch05/section-03/stop-and-copy-gc/{main.rs,README.md}` | **Edition:** `shared`
- **Anchor:** `§5.3.1–5.3.2` (`1996 L18245–L18594`; `2022 L17359–L17635`, Exercises `5.20–5.22`)
- **Design:** Separate garbage collection model from `ch05-mark-sweep-gc` by implementing SICP's two-memory vector representation (`the_cars`/`the_cdrs` vs `new_cars`/`new_cdrs` with tagged `Pointer::{Pair(usize), Int(i64), Sym(usize), Nil, BrokenHeart(usize)}`) and executing the iterative `free`/`scan` Cheney copying loop (`relocate_old_result_in_new`, `gc_flip`) over shared DAGs and cyclic lists! Verify live graph isomorphism, exact compaction, and multiple consecutive GC flip cycles.

#### `SICP-30` [P1] 7-Register Explicit-Control Evaluator with Tail-Call Stack Discipline
- **Upgrade:** `ch05-explicit-control` ([`examples/ch05/section-04/explicit-control/main.rs`](../examples/ch05/section-04/explicit-control/main.rs)) | **Edition:** `shared`
- **Anchor:** `§5.4.1–5.4.4` (`1996 L18595–L19110`; `2022 L17636–L18283`, Exercises `5.23–5.30`)
- **Design:** Build the genuine 7-register (`exp, env, val, continue, proc, argl, unev`) explicit-control state machine running on top of `SICP-28` registers & stack; support self-evaluating, variables, conditionals, assignments, definitions, lambdas, sequences, and multi-argument calls. Verify `ev_sequence` tail-call optimization keeps **constant max stack depth** on tail-recursive `sum(n, 0)` for $n = 10$ vs $n = 500$, while recursive `fact(n)` grows linearly in `max_depth` (`total_pushes = 32n - 16`, `depth = 5n + 3` formula check from `Ex 5.27`)!

#### `SICP-31` [P1] SICP Register Compiler (`preserving`, Linkages & Lexical Addressing)
- **Upgrade / Add:** Upgrade `ch05-bytecode-compiler` or add `ch05-register-compiler` at `examples/ch05/section-05/register-compiler/{main.rs,README.md}` | **Edition:** `shared` *(with `2022-javascript` note for `return` linkage)*
- **Anchor:** `§5.5.1–5.5.7` (`1996 L19111–L20602`; `2022 L18284–L19865`, Exercises `5.31–5.52`)
- **Design:** Implement `InstructionSequence { needs: BTreeSet<Reg>, modifies: BTreeSet<Reg>, statements: Vec<Instr> }`, `append_instruction_sequences`, `preserving(regs, seq1, seq2)` inserting `save`/`restore` **only** when a register is modified by `seq1` and needed by `seq2`, linkage descriptors (`Next`, `Return`, `Label`), compile-time environment lexical addressing (`(frame_idx, disp_idx)` bypassing hash-map name lookup, `§5.5.6`), and execution inside the `SICP-28` simulator! Verify zero unnecessary stack saves on non-interfering operands and strictly fewer stack pushes than `SICP-30` on compiled `factorial`.

---

## 7. Immediate Metadata, Template & CI Fixes Checklist

Apply these small housekeeping fixes before or alongside feature work:

1. **Enforce `rustfmt` & `clippy` in CI ([`.github/workflows/rust.yml`](../.github/workflows/rust.yml#L15-L20)):**
   - Run `cargo fmt --all` and `cargo clippy --all-targets -- -D warnings` across all 41 existing examples (normalizing single-line compressed functions in Ch 2–5).
   - Remove `continue-on-error: true` from [`L16`](../.github/workflows/rust.yml#L16) (`cargo fmt`) and [`L19`](../.github/workflows/rust.yml#L19) (`cargo clippy`).
2. **Fix inconsistent JS edition tag key in [`docs/editions.md`](../docs/editions.md#L11):**
   - Change ``marked `javascript-2022` in the example metadata`` on `L11` to ``marked `2022-javascript` in the example metadata`` matching [`manifest.json`](../manifest.json#L4-L8) and [`COVERAGE.md#L14`](../COVERAGE.md#L14), and note accurately that `ch04-javascript-parser` and `ch05-javascript-tail-calls` carry `2022-javascript` while `ch05-bytecode-compiler` is currently `shared`.
3. **Widen two under-tagged `1996-scheme` entries to `shared`:**
   - `ch02-picture-composition`: `SICP_JavaScript_Ed_2022.md` explicitly covers `§2.2.4 Example: A Picture Language` ([`L4336–L4694`](../../textbooks/WizardBook/SICP_JavaScript_Ed_2022.md#L4336-L4694)); update `"1996-scheme"` → `"shared"` in [`manifest.json#L586,L596`](../manifest.json#L586-L596) and [`examples/ch02/section-02/picture-composition/README.md#L3`](../examples/ch02/section-02/picture-composition/README.md#L3).
   - `ch03-stream-integration`: `SICP_JavaScript_Ed_2022.md` explicitly covers `§3.5.4 Streams and Delayed Evaluation` (`integral`, `solve` ODEs at [`L10658–L10788`](../../textbooks/WizardBook/SICP_JavaScript_Ed_2022.md#L10658-L10788)); update `"1996-scheme"` → `"shared"` in [`manifest.json#L1152,L1162`](../manifest.json#L1152-L1162) and [`examples/ch03/section-05/stream-integration/README.md#L3`](../examples/ch03/section-05/stream-integration/README.md#L3).
4. **Standardize 4 outlier example `README.md` files:**
   - Normalize [`ch03-digital-circuit-simulator/README.md`](../examples/ch03/section-03/digital-circuit-simulator/README.md), [`ch03-constraint-propagation/README.md`](../examples/ch03/section-03/constraint-propagation/README.md), [`ch03-stream-integration/README.md`](../examples/ch03/section-05/stream-integration/README.md), and [`ch04-lexical-closures/README.md`](../examples/ch04/section-01/lexical-closures/README.md) from `**Book section:** / ## Concept / ## Rust implementation / ## Complexity and invariants / ## Run and test / ## Extension exercise` to the house template used by the other 37 examples (`**Book concepts:** / **Edition applicability:** / ## What this demonstrates / ## Implementation walkthrough / ## Complexity and correctness / ## Run it / ## Try this` + `[Coverage](../../../../COVERAGE.md)` footer).
5. **Document the Source Academy origin of subsection `3.2.5 The CSE Machine`:**
   - Note in [`chapters/ch03/section-02/subsection-05/INTRO.md`](../chapters/ch03/section-02/subsection-05/INTRO.md) and [`README.md#L3`](../README.md#L3) that `3.2.5` is a Source Academy web-comparison supplement rather than a numbered subsection in either [`SICP_2nd_Ed_1996.md`](../../textbooks/WizardBook/SICP_2nd_Ed_1996.md) or [`SICP_JavaScript_Ed_2022.md`](../../textbooks/WizardBook/SICP_JavaScript_Ed_2022.md) (which both contain `94` numbered subsections `1.1.1`–`5.5.7`).

---

## 8. Review Caveats

- **1984 First Edition:** No local `1984` Markdown transcription exists under `learning/textbooks/WizardBook`; differences between the 1984 original and 1996 second edition rely on the embedded Prefaces ([`2022 L324–L360`](../../textbooks/WizardBook/SICP_JavaScript_Ed_2022.md#L324-L360)) and the companion's [`docs/editions.md`](../docs/editions.md).
- **Execution status:** No `cargo fmt`, `cargo clippy`, or `cargo test` commands were run during this documentation-only review; all source/manifest/line counts (`41` binaries, `208` manifest entries, `17–79` line bounds across all 41 `main.rs` files) come from direct file inspection.

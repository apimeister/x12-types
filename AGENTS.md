# AGENTS.md

Guidance for AI coding agents working in this repository.

`x12-types` is a Rust library providing typed bindings (structs) for the ASC X12 EDI
standard, with parsing (via `nom`) and rendering (via `Display`).

## Workspace

This is a Cargo workspace with two members:

- `.` — the `x12-types` library crate.
- `x12-types-macros/` — the derive macros (`DisplaySegment`, `ParseSegment`, `DisplayX12`,
  `ParseX12`) used throughout the library. It is a path dependency with a version, so it
  stays publishable; publish `x12-types-macros` first, then bump the dep in the root
  `Cargo.toml`. The generated code references `crate::util::*`, so the macros only work
  inside `x12-types`.

## Validate before done (these are the CI gate — all must pass)

- `cargo fmt --all` — rustfmt must report no diffs.
- `cargo clippy --all -- -D warnings` — warnings are errors; the build fails on any.
- `cargo test`
- MSRV is **1.82** (`rust-version` in Cargo.toml); don't use newer-than-1.82 APIs.

## Feature flags

Each X12 version is a module gated by a feature: `v003030`, `v004010`, `v004030`,
`v005010`, `v005030`. All five are enabled by default, so `cargo test` covers everything.
To test one version in isolation: `cargo test --no-default-features --features v005010`.

`Cargo.lock` is gitignored (this is a library). Test-data files matching `*nocheckin*` are
gitignored — use that suffix for scratch EDI samples you don't want committed.

## Module layout (per version, follow the v005010 pattern)

Each `src/v<version>/` module is organized as:

- `mod.rs` — thin: imports, `pub use segment::*;`, the version's `Transmission`/
  `FunctionalGroup` envelope types, and one `mod _<NNN>_doc;` + `mod _<NNN>_test;` line per
  transaction set.
- `_<NNN>_doc.rs` — one file per transaction set: the `_<NNN>` struct, its loop structs, and
  their parsers (all derived — see below).
- `_<NNN>_test.rs` — that transaction set's tests; `segments_test.rs` for segment tests.
- segments in `segment/<letter>.rs` (newer versions) or a single `segment.rs` (older ones).

## Code conventions

- **Segments**: fields are numbered `_01`, `_02`, … each with `#[serde(rename = "01")]`.
  Required elements are `String`; optional elements are `Option<String>`. Segment structs
  derive `DisplaySegment, ParseSegment`. See `@docs/segment-template.md` for the exact shape,
  including the `REF | ID | NAME | REPEAT | REQ | TYPE | MIN/MAX` doc-comment table.
- **Transaction sets and loops** derive `DisplayX12, ParseX12` — do **not** hand-write
  `Display`/`parse` impls. `ParseX12` generates the parser from the struct's fields:
  - plain segment `T` → mandatory `T::parse`
  - `Option<T>` → `opt(T::parse)`
  - `Vec<T>` (segment) → `many0(T::parse)`
  - `Vec<Loop>` → annotate with `#[x12(loop_trigger = "HL")]` (the segment that opens the
    loop; use `"A|B"` when more than one segment can start it). The loop struct must also
    derive `ParseX12` so its body parses recursively.
- Almost every transaction set uses `ParseX12`. The exception is `v005010` 276 and 277, which
  dispatch their top-level loops on a segment *field value* (`HL._03`), not a segment name —
  `loop_trigger` can't express that, so those keep hand-written parsers. Don't add new
  hand-written parsers for anything `ParseX12` can handle.

## When adding a new transaction set or segment

Update all of these in the same change:

- `CHANGELOG.md` — add an entry under a new version heading (matches the existing manual style).
- `README.md` — add the set under its version in the "Supported Bindings" list.
- `Cargo.toml` — bump the package `version`.
- Add a `_<NNN>_test.rs` and a sample `test-data/<version>_<NNN>.edi`.

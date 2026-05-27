---
id: rfc-0009
title: "Module System"
date: '2026-05-21'
status: accepted
---

## Summary

Design the module system: how source files map to modules, how names are imported and exported, the `use` keyword semantics, visibility (`pub`), and re-exports (`pub use`). This is the largest deferred feature — it blocks the standard library, multi-file programs, and all visibility control.

---

## Motivation

All v0.1 programs are single-file. Adding a module system unlocks:

- Multi-file programs and code organisation
- A standard library (math, string, io, collections)
- Visibility control — `pub` to export, private by default
- Re-exports for public API shaping

The `use` keyword is already a reserved word in the grammar.

---

## Design

### File-to-module mapping

Modules are declared explicitly with `mod`. A `mod name;` statement in a file means the compiler looks for `name.mln` or `name/mod.mln` alongside the declaring file. The module tree is therefore explicit — not inferred from the filesystem — and the root is always an entry point file (e.g. `main.mln`).

```
src/
  main.mln       -- mod parser; mod evaluator;
  parser.mln     -- or: parser/mod.mln
  evaluator/
    mod.mln      -- mod expr; mod stmt;
    expr.mln
    stmt.mln
```

The `mod` declaration must appear at the top level of the declaring file. The declared module's contents are in the resolved file; they are not inlined into the declaring file.

### `use` syntax

Imports use `::` path separators, Rust-style. The root of a path is either `crate` (the current crate's root), `std` (the standard library), or a bare module name declared via `mod` in the current file.

```moonlane
use std::math;
use std::collections::{Map, Set};
use crate::parser::Ast;
use crate::parser::{Ast, Token, ParseError};
```

- `use path::to::Name` — imports `Name` into the current scope
- `use path::to::{A, B, C}` — imports multiple names from the same path
- `use path::to::*` — glob import (imports all `pub` names from the module)
- All `use` statements must appear at the top level of a file, before any declarations

Path components use `::`. The `crate` keyword refers to the root of the current compilation unit (the file containing the entry point). There is no `super::` or `self::` in v0.5.0 — relative paths are deferred.

### Visibility

All declarations are **module-private by default**. A declaration is accessible from outside its module only if annotated with `pub`.

```moonlane
pub struct Token { kind: TokenKind, span: Span }   // exported
struct InternalState { ... }                        // module-private

pub fun parse(tokens: Token[]) -> Ast { ... }      // exported
fun helper(t: Token) -> Bool { ... }               // module-private
```

`pub` is valid on: `struct`, `enum`, `fun`, `linear struct`, `linear enum`, `aspect`, and top-level `let`/`mut` bindings.

Within a module, all names (including private ones) are accessible without qualification. From outside the module, only `pub` names are accessible via their import path.

### `pub use` re-exports

A `pub use` statement re-exports a name from the current module's public API, regardless of where it was defined. This allows a module to shape its public interface independently of its internal file structure.

```moonlane
// parser/mod.mln
mod ast;
mod lexer;

pub use ast::Ast;          // Ast is now accessible as crate::parser::Ast
pub use lexer::Token;      // Token re-exported from lexer submodule
                           // lexer itself remains private — not pub mod
```

Re-exported names are indistinguishable from names defined in the re-exporting module from the caller's perspective. This is the mechanism for facade modules and clean public API surfaces.

### Circular imports

Circular imports are a **compile error**. If module A imports from module B and module B imports from module A (directly or transitively), the compiler rejects the program with a clear cycle-detection error listing the import chain.

This enforces a directed acyclic dependency graph and keeps the module resolution algorithm simple.

### Standard library path

The standard library is accessible via the reserved `std` root. It is not a user-defined module and does not appear in `mod` declarations. The compiler resolves `std::*` paths to the bundled standard library regardless of the project structure.

```moonlane
use std::math;
use std::string;
use std::io;
use std::collections::Map;
```

User modules may not declare a top-level module named `std`.

### Single-file compatibility

Existing single-file programs are **fully valid** without modification. A `.mln` file with no `mod` or `use` declarations is a complete, self-contained program. The module system is purely additive — it is only activated when `mod` or `use` appears.

In a single-file program, the implicit module is the file itself. All top-level names are in scope without import. The built-in types (`Perhaps`, `Result`, `Bool`, `Int`, `Float`, `String`) and built-in functions remain globally available in all programs regardless of module structure.

---

## Grammar additions

```
mod-decl  ::= 'mod' identifier ';'
use-decl  ::= 'use' use-path ';'
            | 'pub' 'use' use-path ';'
use-path  ::= path-root '::' use-tree
path-root ::= 'crate' | 'std' | identifier
use-tree  ::= identifier
            | '{' use-tree (',' use-tree)* '}'
            | '*'
            | identifier '::' use-tree
pub-ann   ::= 'pub'   -- prefix on struct, enum, fun, let, mut declarations
```

---

## Open Questions

*(All resolved — see Decision section below.)*

---

## Decision

**Outcome:** Accepted — v0.5.0

| Question | Decision |
|---|---|
| File-to-module mapping | Explicit `mod` declarations; `name.mln` or `name/mod.mln` |
| `use` syntax | `use path::to::Name` with `::` separators; `crate` and `std` roots |
| Visibility default | Private by default; `pub` to export |
| `pub use` re-exports | Included in v0.5.0 |
| Circular imports | Compile error |
| Standard library path | Reserved `std` root |
| Single-file compatibility | Fully preserved — module system is additive |

**Target:** v0.5.0

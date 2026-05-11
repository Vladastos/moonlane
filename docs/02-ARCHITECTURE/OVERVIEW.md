# Interpreter Architecture Overview

> For the rationale behind these architectural choices, see [ADR-0004](../06-DECISIONS/closed/ADR-0004-interpreter-architecture.md).

## Pipeline

```
.yolo source file
       │
       ▼
  ┌─────────┐
  │  Parser │  pest PEG grammar → concrete syntax tree (CST)
  └─────────┘
       │  ast::Program
       ▼
  ┌──────────────┐
  │ Type Checker │  untyped AST → typed AST  (errors reported here)
  └──────────────┘
       │  typed_ast::TypedProgram
       ▼
  ┌─────────────┐
  │  Evaluator  │  typed AST → program output  (tree-walking)
  └─────────────┘
```

Each stage is a separate Rust module. No stage is skipped.

---

## Crate Structure

```
yoloscript/
├── Cargo.toml
└── src/
    ├── main.rs          — CLI entry point: reads a .yolo file, runs the pipeline
    ├── grammar.pest     — pest PEG grammar for the full v0.1 language
    ├── parser/          — drives pest, builds untyped AST from CST
    ├── ast/             — untyped AST node definitions
    ├── types/           — concrete type representation (Type enum)
    ├── typeinference/   — HM inference engine: type vars, unification, schemes
    ├── typechecker/     — two-pass type checker; produces typed AST
    ├── typed_ast/       — typed AST node definitions
    ├── evaluator/       — tree-walking evaluator, environment, runtime values
    └── error/           — unified error type covering all pipeline stages
```

---

## Component Boundaries

The data structures that cross component boundaries:

| Data | Type | Produced by | Consumed by |
|------|------|-------------|-------------|
| Untyped program | `ast::Program` | parser | typechecker |
| Typed program | `typed_ast::TypedProgram` | typechecker | evaluator |
| Errors | `YoloscriptError` | any stage | caller / CLI |

---

## Error Design

All errors use a unified `YoloscriptError` type so the CLI and any future tooling has a single error surface:

```rust
enum YoloscriptError {
    ParseError   { code: ErrorCode, message: String, start: usize, end: usize, filename: String },
    TypeError    { code: ErrorCode, message: String, start: usize, end: usize, filename: String },
    RuntimePanic { message: String, start: usize, end: usize, filename: String },
    Internal     { message: String },
}
```

Type errors carry an `ErrorCode` (E0001–E0004) for structured diagnostics. Runtime panics (`.yolo()` on `nope`, out-of-bounds, division by zero) terminate with a non-zero exit code — see the spec for panic semantics.

---

## Component Documentation

Each component's implementation details live in its own subfolder:

| Component | Source | Docs |
|-----------|--------|------|
| Parser | `src/parser/`, `src/grammar.pest` | [parser/](./parser/) |
| Type Checker | `src/typechecker/` | [typechecker/](./typechecker/) |
| Type Inference Engine | `src/typeinference/` | [typeinference/](./typeinference/) |
| Evaluator | `src/evaluator/` | [evaluator/](./evaluator/) |

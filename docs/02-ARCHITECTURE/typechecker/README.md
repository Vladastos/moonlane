# Type Checker

**Source:** `src/typechecker/mod.rs`
**Entry point:** `typechecker::check(program: ast::Program) -> Result<typed_ast::TypedProgram, YoloscriptError>`

The type checker is a two-pass pipeline over the untyped AST:

**Pass 1 — Inference:** Walks the AST, emits type constraints using the HM inference engine, and solves them. Polymorphic function declarations are solved and generalized inline (Algorithm W style) so each call site gets a fresh instantiation.

**Pass 2 — Construction:** Re-walks the AST with the solved substitution to build the fully typed `TypedProgram`. Each expression node receives a concrete `Type`. Polymorphic functions whose body cannot be typed at a single concrete instantiation carry a `FunBody::Generic(Block)` instead of a typed body — the evaluator uses runtime values, not type annotations, so this is safe.

The inference engine (`src/typeinference/`) is the algorithmic core. The typechecker orchestrates it: pre-pass hoisting, scope management, constraint collection, and typed AST construction.

## Key Files

- `src/typechecker/mod.rs` — two-pass orchestration
- `src/typeinference/mod.rs` — HM engine (type vars, unification, schemes, constraints)
- `src/typed_ast/mod.rs` — typed AST node definitions
- `src/types/mod.rs` — concrete `Type` enum

## Detailed Docs

- [typeinference/](../typeinference/) — concepts, implementation guide, algorithm details

## Tests

Integration tests (full pipeline from source to typed AST) live in `tests/typechecking/`. Unit tests for the inference engine live in `tests/typeinference/`.

```bash
cargo test --test lib typechecking_tests
cargo test --test lib typeinference_tests
```

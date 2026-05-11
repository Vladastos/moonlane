# Yoloscript

A Rust-inspired programming language with a tree-walk interpreter written in Rust.

## Why?

Looking for a side project, a colleague suggested writing an interpreter. "Nice one, very funny" was my first response — until I found out he was completely serious and pointed me to the Crafting Interpreters book.

I started following the book (using Rust) but soon decided I wanted to design my own language. Designing a language proved to be a non-trivial task — what type system? expression-oriented or statement-oriented? garbage collected or not? Rather than overthink it, I took the only reasonable approach: just go for it.

Hence the name: **Yoloscript**.

The first iteration proved you can only yolo so much before getting lost in the sauce. This second one tries to learn from that — structure and a clear path tend to get you further than raw momentum.

The language ended up drawing heavily from Rust — strong static typing, algebraic data types, explicit error handling — but without the borrow checker complexity. The goal was something that felt safe and expressive without requiring a PhD to understand ownership.

Whether that goal was achieved is left as an exercise to the reader.

## What?

Yoloscript is a statically typed, expression-oriented programming language. It features:

- **Strong static typing** with local type inference (Hindley-Milner)
- **Algebraic data types** — enums with data-carrying variants
- **Exhaustive pattern matching**
- **Explicit nullability** via `Perhaps<T>` (no null pointers)
- **Explicit error handling** via `Result<T, E>`
- **First-class functions** and closures
- **Generics** with compile-time monomorphization
- **Traits** for ad-hoc polymorphism
- **Memory managed by the runtime** (reference counting)

See the [Language Specification](./docs/01-SPEC/LANGUAGE-SPEC.md) for the complete definition.


## How?

The spec and the interpreter are developed in parallel, in a tight loop:

```
Define a feature in the spec
        ↓
Implement it in the interpreter
        ↓
Write real programs using it
        ↓
Observe gaps, wrong assumptions, usability issues
        ↓
Refine the spec  →  implement the refinement  →  next feature
```

The spec (`docs/01-SPEC/LANGUAGE-SPEC.md`) is the source of truth within each iteration — no code diverges from it — but the spec itself is a living document expected to evolve through usage. The tree-walk interpreter is the feedback mechanism: fast enough to iterate on, disposable enough not to over-invest in.

Work is organised into epics under `docs/04-TASKS/`, each broken into tasks that move through `open → in-progress → done`. Architecture decisions are recorded in `docs/05-DECISIONS/` so the reasoning behind non-obvious choices isn't lost. The current phase and its milestones are defined in `docs/03-PLANNING/`.

## Quick Start

### Prerequisites

- Rust 1.70+
- Cargo

### Build

```bash
cd tree-walk-interpreter
cargo build --release
```

### Run a Program

```bash
cargo run -- path/to/program.yolo
```

### Run Tests

```bash
# All tests
cargo test

# Type inference unit tests
cargo test --test lib typeinference_tests

# Typechecking integration tests
cargo test --test lib typechecking_tests
```

## Example

```yoloscript
fun factorial(n: Int) -> Int {
    if (n <= 1) { 1 } else { n * factorial(n - 1) }
}

let result = factorial(5);
```

## Project Structure

```
Yoloscript/
├── tree-walk-interpreter/
│   ├── src/
│   │   ├── parser/         # PEG grammar (pest) → untyped AST
│   │   ├── ast/            # Untyped AST node definitions
│   │   ├── typeinference/  # HM inference engine
│   │   ├── typechecker/    # Two-pass type checker → typed AST
│   │   ├── typed_ast/      # Typed AST node definitions
│   │   ├── evaluator/      # Tree-walking evaluator
│   │   ├── types/          # Concrete type representation
│   │   └── error/          # Unified error type
│   ├── tests/
│   │   ├── lib.rs
│   │   ├── typeinference/  # HM engine unit tests (phases 1–7)
│   │   ├── typechecking/   # Full pipeline integration tests
│   │   └── parsing/        # Parser tests
│   └── Cargo.toml
│
└── docs/
    ├── 00-PROCESS/         # How to work on this project
    ├── 01-SPEC/            # Language specification (source of truth)
    ├── 02-ARCHITECTURE/    # System overview and component guides
    ├── 03-PLANNING/        # Phase definitions and milestones
    ├── 04-TASKS/           # Epic-based task tracking
    └── 05-DECISIONS/       # Architecture decision records (ADRs)
```

## Current Status

This is Phase 01: Proof-of-Concept. The goal is to validate the language specification through a working interpreter — correctness over production quality.

| Milestone | Status |
|-----------|--------|
| M1: All 10 test programs parse | Done |
| M2: All 10 test programs type-check | In progress |
| M3: All 10 test programs execute correctly | Not started |
| M4: All spec sections interpreter-validated | Not started |

See [PHASE-01-POC.md](./docs/03-PLANNING/PHASE-01-POC.md) for the full plan.

## Documentation

| Folder | Purpose | Start here |
|--------|---------|------------|
| [00-PROCESS/](./docs/00-PROCESS/) | Development workflow and conventions | [PROCESS.md](./docs/00-PROCESS/PROCESS.md) |
| [01-SPEC/](./docs/01-SPEC/) | Language specification | [LANGUAGE-SPEC.md](./docs/01-SPEC/LANGUAGE-SPEC.md) |
| [02-ARCHITECTURE/](./docs/02-ARCHITECTURE/) | System overview and component guides | [OVERVIEW.md](./docs/02-ARCHITECTURE/OVERVIEW.md) |
| [03-PLANNING/](./docs/03-PLANNING/) | Phase definitions and milestones | [PHASE-01-POC.md](./docs/03-PLANNING/PHASE-01-POC.md) |
| [04-TASKS/](./docs/04-TASKS/) | Epics and task tracking | [epic-005-typechecker-integration/](./docs/04-TASKS/epic-005-typechecker-integration/) |
| [05-DECISIONS/](./docs/05-DECISIONS/) | Architecture decision records | [README.md](./docs/05-DECISIONS/README.md) |

## License

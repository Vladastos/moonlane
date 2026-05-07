# Yoloscript

A Rust-inspired programming language with a tree-walk interpreter written in Rust.

## Why?

Looking for a side project, a colleague suggested writing an interpreter. "Nice one, very funny" was my first response - until I found out he was completely serious and pointed me to The Crafting Interpreters book.

I started following the book (using Rust) but soon decided that I want to design my own language.

Designing a language however proved to be a non-trivial task meant making a lot of decisions without a safety net - what type system? expression-oriented or statement-oriented? garbage collected or not? Rather than overthink it, I took the only reasonable approach: just go for it.

Hence the name: **Yoloscript**.

The first iteration proved you can only yolo so much before getting lost in the sauce. This second one tries to learn from that — structure and a clear path tend to get you further than raw momentum.

The language ended up drawing heavily from Rust — strong static typing, algebraic data types, explicit error handling — but without the borrow checker complexity. The goal was something that felt safe and expressive without requiring a PhD to understand ownership.

Whether that goal was achieved is left as an exercise to the reader.

## How?

The language specification in `docs/01-SPEC/LANGUAGE-SPEC.md` is the source of truth — implementation follows spec, not the other way around. The spec itself is a work in progress, expanded incrementally as features are designed. When implementation reveals an ambiguity, the spec is updated first, then the code.

Work is organized into epics under `docs/05-TASKS/`, each broken into tasks that move through `open → in-progress → done`. Architecture decisions are recorded in `docs/06-DECISIONS/` so the reasoning behind choices isn't lost.

The current implementation target is a tree-walk interpreter — enough to validate that the spec is complete and consistent before committing to anything heavier.

## What?

Yoloscript is a statically typed, expression-oriented programming language designed with inspiration from Rust. It features:

- **Strong static typing** with local type inference
- **Algebraic data types** (enums with data-carrying variants)
- **Exhaustive pattern matching**
- **Explicit nullability** via `Perhaps<T>` (no null pointers)
- **Explicit error handling** via `Result<T, E>`
- **First-class functions** and closures
- **Generics** with compile-time monomorphization
- **Traits** for ad-hoc polymorphism
- **Memory managed by the runtime** (reference counting)

See [Language Specification](./docs/01-SPEC/LANGUAGE-SPEC.md) for the complete definition.

## Quick Start

### Prerequisites
- Rust 1.70+
- Cargo

### Building

```bash
cd tree-walk-interpreter
cargo build --release
```

### Running a Program

```bash
cargo run -- path/to/program.yolo
```

### Running Tests

```bash
# All tests
cargo test

# Type inference tests specifically
cargo test --test typeinference_tests
```

## Example Program

```yoloscript
fun factorial(n: Int) -> Int {
    if (n <= 1) {
        1
    } else {
        n * factorial(n - 1)
    }
}

let result = factorial(5);
// result: 120
```

## Project Structure

```
Yoloscript/
├── tree-walk-interpreter/      # Main interpreter implementation (Rust)
│   ├── src/
│   │   ├── main.rs             # Entry point
│   │   ├── lib.rs              # Library exports
│   │   ├── parser/             # Parsing (pest grammar)
│   │   ├── ast/                # Abstract syntax tree
│   │   ├── typeinference/      # Type inference engine
│   │   ├── typechecker/        # Type checking pass
│   │   ├── evaluator/          # Runtime evaluation
│   │   ├── error/              # Error types
│   │   └── types/              # Type system
│   ├── tests/                  # Integration & unit tests
│   └── Cargo.toml
│
├── docs/                        # All documentation
│   ├── 00-PROCESS/             # How to work on the project
│   ├── 01-SPEC/                # Language specification
│   ├── 02-ARCHITECTURE/        # Architecture & design
│   ├── 03-COMPONENTS/          # Component implementation guides
│   ├── 04-PLANNING/            # Strategic roadmaps
│   ├── 05-TASKS/               # Issue tracking & task breakdown
│   └── 06-DECISIONS/           # Architecture decision records (ADRs)
│
└── README.md                    # This file
```

## Documentation

Navigate to [docs/](./docs/) for complete documentation:

| Folder | Purpose | Start Here |
|--------|---------|-----------|
| **00-PROCESS** | How to work on this project | [PROCESS.md](./docs/00-PROCESS/PROCESS.md) |
| **01-SPEC** | Language specification | [LANGUAGE-SPEC.md](./docs/01-SPEC/LANGUAGE-SPEC.md) |
| **02-ARCHITECTURE** | Architecture & design | [INTERPRETER-DESIGN.md](./docs/02-ARCHITECTURE/INTERPRETER-DESIGN.md) |
| **03-COMPONENTS** | Implementation guides | [typeinference/](./docs/03-COMPONENTS/typeinference/) |
| **04-PLANNING** | Roadmap & strategic plans | [MEDIUM-TERM-PLAN.md](./docs/04-PLANNING/MEDIUM-TERM-PLAN.md) |
| **05-TASKS** | Current work & issues | [epic-001-typechecker/](./docs/05-TASKS/epic-001-typechecker/) |
| **06-DECISIONS** | Architecture decision records | [README.md](./docs/06-DECISIONS/README.md) |

## Current Status

### v0.1

The v0.1 interpreter focuses on **language spec validation**: proving the specification is complete, consistent, and implementable.

**v0.1 includes:**
- Parser (PEG grammar via pest)
- AST representation
- Error handling framework
- Type system definition
- Type inference engine with let-polymorphism
- Type checking pass
- Expression evaluation
- Generics & monomorphization
- Trait system
- Standard library functions
- REPL (interactive shell)

See [MEDIUM-TERM-PLAN.md](./docs/04-PLANNING/MEDIUM-TERM-PLAN.md) for the detailed roadmap.


## Architecture Overview

```
source code
    ↓
[Parser] → CST (via pest)
    ↓
[AST Builder]
    ↓
untyped AST
    ↓
[Type Checker] → generics resolved, monomorphized
    ↓
typed AST
    ↓
[Tree-Walking Evaluator]
    ↓
result (value or error)
```

Key design decisions documented in [docs/06-DECISIONS/](./docs/06-DECISIONS/).

## Testing

### Run All Tests
```bash
cargo test
```

### Run Specific Test Suite
```bash
# Type inference tests
cargo test --test typeinference_tests

# Specific phase
cargo test --test typeinference_tests phase_2

# With output
cargo test --test typeinference_tests phase_2 -- --nocapture
```

## References

- **Language Spec**: [docs/01-SPEC/LANGUAGE-SPEC.md](./docs/01-SPEC/LANGUAGE-SPEC.md)
- **Process Guide**: [docs/00-PROCESS/PROCESS.md](./docs/00-PROCESS/PROCESS.md)
- **Task Convention**: [docs/00-PROCESS/TASK-CONVENTION.md](./docs/00-PROCESS/TASK-CONVENTION.md)
- **Interpreter Design**: [docs/02-ARCHITECTURE/INTERPRETER-DESIGN.md](./docs/02-ARCHITECTURE/INTERPRETER-DESIGN.md)
- **Type Inference Guide**: [docs/03-COMPONENTS/typeinference/](./docs/03-COMPONENTS/typeinference/)
- **Architecture Decisions**: [docs/06-DECISIONS/](./docs/06-DECISIONS/)
- **Roadmap**: [docs/04-PLANNING/MEDIUM-TERM-PLAN.md](./docs/04-PLANNING/MEDIUM-TERM-PLAN.md)
- **Current Tasks**: [docs/05-TASKS/](./docs/05-TASKS/)

## License

# Type Inference System

**Source:** `src/typeinference/mod.rs`
**Tests:** `tests/typeinference/typeinference_tests.rs`

Implements the Hindley-Milmer algorithm with let-polymorphism and constraint-based solving. Built in 8 incremental phases, all of which are complete.

## Documents

- **[CONCEPTS.md](./CONCEPTS.md)** — theoretical background: type schemes, generalization, env_fvs, let-polymorphism
- **[ROADMAP.md](./ROADMAP.md)** — the 8-phase implementation breakdown (complete; useful as implementation reference)
- **[GUIDE.md](./GUIDE.md)** — workflow and tips for extending the system

## The 8 Phases

| Phase | Component | Status |
|-------|-----------|--------|
| 1 | Type variables (`TypeVar`, `TypeVarGenerator`) | Done |
| 2 | Inference types (`InferType` enum) | Done |
| 3 | Substitution | Done |
| 4 | Unification algorithm | Done |
| 5 | Constraints | Done |
| 6 | Type schemes (let-polymorphism) | Done |
| 7 | Inference context (`InferContext`) | Done |
| 8 | Integration with typechecker | Done |

## Running Tests

```bash
# All inference unit tests
cargo test --test lib typeinference_tests

# Specific phase
cargo test --test lib typeinference_tests phase_3
```

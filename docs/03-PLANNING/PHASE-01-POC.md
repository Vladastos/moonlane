# Phase 01: Proof-of-Concept Interpreter

## Goal

Build a usable interpreter for a defined subset of Yoloscript — and use it to iteratively stabilise the spec for that subset.

The spec is not complete at the start of this phase. It defines the intended v0.1 feature set, but real usage of the interpreter will reveal gaps, wrong assumptions, and design issues that pure design work misses. The interpreter and the spec are developed in parallel: each iteration refines both.

Once the v0.1 subset is implemented, usable for real programs, and its spec is stable, this phase is complete. Phase 02 then builds a production-quality implementation from the validated spec.

---

## Iteration Model

Phase 01 uses an agile-like loop applied to the spec itself:

```
Define feature in spec
        ↓
Implement in interpreter
        ↓
Write real programs using it
        ↓
Observe gaps, wrong assumptions, usability issues
        ↓
Refine spec
        ↓
Implement refinement
        ↓
Next feature  (repeat)
```

The interpreter is the feedback mechanism for the spec, not just a deliverable. An issue found through usage is treated the same as an implementation finding: stop, update the spec, then continue.

The PROCESS.md spec-first rule still holds within each iteration — no code diverges from the spec — but the spec itself is expected to evolve through usage.

---

## Success Criterion

The v0.1 feature set is fully implemented, the interpreter can execute real Yoloscript programs, and the spec for that subset is stable — no known gaps or open design questions. All implemented spec sections are tagged `> ✓ Interpreter-validated (v0.1)`.

---

## Milestones

| # | Milestone | Status |
|---|-----------|--------|
| M1 | All 10 test programs parse without error | Done |
| M2 | All 10 test programs type-check without error | In progress |
| M3 | All 10 test programs execute with correct output | Not started |
| M4 | Interpreter usable for real programs; v0.1 spec stable | Not started |

M2 is tracked in epic-005 (typechecker integration). M3 requires epics 002 (evaluator), 003 (generics), and 004 (traits). M4 follows from M3 — it is the usage and stabilisation loop, not just a technical gate.

---

## Scope

**v0.1 feature set (in scope):**
- PEG parser (stub quality — correct AST, no error recovery)
- Hindley-Milner type checker with let-polymorphism
- Tree-walk evaluator
- Basic error reporting with source locations
- The language features currently defined in `docs/01-SPEC/LANGUAGE-SPEC.md`

The feature set is defined but not frozen. Features may be added, trimmed, or redesigned based on what usage reveals. Changes go through the spec first.

**Explicitly deferred to Phase 02:**
- Parser error recovery and diagnostic suggestions
- Performance optimisation at any stage
- LLVM or bytecode compiler backend
- Standard library (begins after Phase 01 completes)
- Production-quality error messages

---

## Design Philosophy

All components are built with the understanding that they will likely be rewritten in Phase 02 once the language has stabilised:

- Prioritise **speed of iteration** over architectural perfection
- Focus on **correctness**, not optimisation
- Write code that is easy to modify as the spec evolves
- Do not over-engineer for extensibility — let the design emerge from usage

The AST (`src/ast/`) is the one exception: it is the contract between parser and type checker, and is designed to remain stable even if other components are rewritten.

---

## Next Phase

Phase 02 builds a production-quality implementation — compiler or optimised interpreter — using the stable, fully interpreter-validated v0.1 spec as its ground truth. Scope and approach are defined when Phase 01 completes.

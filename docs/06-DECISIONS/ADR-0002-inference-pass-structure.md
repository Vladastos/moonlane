# ADR-0002: Inference Pass Structure

**Status:** proposed  
**Date:** 2026-05-07  
**Related:** [ADR-0001 — TypeRegistry Structure and Location](./ADR-0001-type-registry.md)

## Context

The current two-pass design in task 0005 is:

- **Pass 1 — Inference**: walk the AST with `InferContext`, emit constraints,
  return `InferType`s, solve all constraints at the end with `ctx.solve()`
- **Pass 2 — Construction**: walk the AST again with the final `Substitution`,
  convert `InferType → Type`, build `TypedExpr` / `TypedDecl` nodes

Two questions need answering before Stage 2 begins:

1. **How does Pass 1 surface per-node types to Pass 2?** The `InferType` for
   each node is on the call stack during Pass 1 and discarded when the recursive
   call returns. Pass 2 has no direct access to these values.

2. **Is the two-pass split worth the complexity?** A single pass that solves
   constraints eagerly would be simpler — but may not be powerful enough.

### Link to ADR-0001

ADR-0001 is deciding whether `TypeRegistry` lives inside `InferContext` or is
pre-built and injected. If the "inject" option is taken, it reflects a broader
design philosophy: **the pre-pass produces immutable, fully-resolved state;
`InferContext` consumes it**.

Applied consistently, the same pattern would extend to the **initial variable
environment** — the top-level function type schemes registered during the
pre-pass currently go directly into `InferContext.poly_env`. Under the inject
philosophy they would instead be returned as an `InitialEnv` value and passed
into `InferContext::new`:

```rust
let (type_registry, initial_env) = pre_pass(&program);
let mut ctx = InferContext::new(type_registry, initial_env);
```

This narrows `InferContext`'s role to **constraint accumulation and scope
management** rather than owning all inference state. That narrower role directly
affects which pass structure is most natural — a leaner `InferContext` favours
the single-pass approach, because there is less shared mutable state to reason
about across two traversals.

## Options Considered

### Option A: Two-pass with side table

Pass 1 populates a `HashMap<NodeId, InferType>` as it walks the AST. Pass 2
indexes into this table to retrieve each node's type, then applies the final
`Substitution` to convert to `Type` and constructs the `TypedAST`.

**Requires:** stable, unique `NodeId`s on every AST node — a non-trivial change
to the AST definition.

**Pros:**
- Pass 1 and Pass 2 are fully decoupled; each has a clear, single responsibility
- Side table can be inspected independently for debugging

**Cons:**
- AST nodes need `NodeId` fields added throughout — significant structural change
- Extra allocation for the side table

### Option B: Two-pass with skeleton tree

Pass 1 produces a parallel "inference tree" that mirrors the structure of the
AST but carries `InferType`s at every node. Pass 2 zips the AST and inference
tree together, applies the `Substitution`, and builds `TypedAST` nodes.

**Pros:**
- No `NodeId`s required
- Clear data flow: Pass 1 output is an explicit value, not a side effect

**Cons:**
- Requires defining and populating a third tree representation alongside the
  untyped AST and the `TypedAST` — the most code of any option
- High memory usage (three tree-shaped structures in memory simultaneously)

### Option C: Two-pass with re-derivation

Pass 1 walks the AST, emits constraints, solves → gets `Substitution`.
Pass 2 walks the AST again and **re-runs the same inference logic**, but with
the solved `Substitution` applied inline so every type immediately resolves to
concrete — then builds `TypedAST` nodes directly.

**Pros:**
- No side table or skeleton needed; no `NodeId`s
- Minimal extra data structures

**Cons:**
- The inference logic runs twice — once to collect constraints, once to rebuild
  types. Any bug in the logic must be fixed in both contexts, or the two paths
  must share code carefully
- Tightly couples Pass 1 and Pass 2: they share the same traversal code, making
  it harder to reason about each independently

### Option D: Single pass with eager constraint solving

Walk the AST once. After inferring each sub-expression, immediately solve the
local constraints accumulated so far, get a concrete type, and build the
`TypedAST` node in place. No second traversal needed.

**Pros:**
- Simplest control flow — one traversal, one responsibility
- Pairs naturally with a narrow `InferContext` (pure scope manager + eager
  solver); if ADR-0001 goes with the "inject" philosophy and the initial env is
  also injected, `InferContext` becomes small enough that single-pass is clean
- No surfacing problem: the type is available immediately at the point of
  construction

**Cons:**
- Eager solving may fail on programs that require whole-program unification.
  Specifically: **let-polymorphism** and **mutual recursion** both require that
  constraints from the full body are solved before a type scheme is generalised
  or a recursive call is resolved. Eager local solving cannot satisfy this.
- This constraint means the pre-pass must be more aggressive: all top-level
  function signatures must be fully annotated, or mutually recursive functions
  must be handled specially.

## Decision

*(pending — status: proposed)*

## Consequences

*(to be filled in once decision is accepted)*

## References

- Task: [0005 — Typechecker Integration](../05-TASKS/epic-001-typechecker/in-progress/0005-typechecker-integration.md) (open question: Pass 1 → Pass 2 type transfer)
- ADR: [ADR-0001 — TypeRegistry Structure and Location](./ADR-0001-type-registry.md) (inject philosophy; decision here is downstream of that one)

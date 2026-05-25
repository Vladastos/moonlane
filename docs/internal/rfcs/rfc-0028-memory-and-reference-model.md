---
id: rfc-0028
title: "Memory and Reference Model"
date: '2026-05-24'
status: draft
target: v0.3
supersedes:
  - rfc-0001
  - rfc-0024
---

## Summary

Define Moonlane's unified memory and reference model. The model has three interlocking parts:

- **Linear types** — opt-in, statically checked exactly-once ownership for resources that require deterministic release
- **Read references** (`@T`) — expression-scoped, non-storable views of a linear value that do not consume it
- **Pointers** — regular RC-backed `*T` for non-linear shared state; unique `unique *T` as a linear-compatible heap indirection

The three parts are inseparable: linear types require read references to be usable, unique pointers are the bridge that allows linear values to be heap-allocated and passed indirectly, and regular pointers are restricted to non-linear types to preserve the aliasing model. They must be designed and implemented together.

This RFC supersedes RFC-0001 (Pointer Syntax and Semantics) and RFC-0024 (Linear Types), incorporating all resolved decisions from both and carrying forward their open questions in unified form.

---

## Motivation

Moonlane's default memory model is runtime-managed reference counting. This is ergonomic for most code but insufficient for systems-level use cases:

- Resources that must be explicitly released (file handles, sockets, buffers)
- Allocation and deallocation that must be deterministic and zero-overhead
- Use-after-free and resource leaks caught at compile time
- Sharing a single mutable value across multiple bindings
- Building self-referential or recursive data structures

Linear types address the first group; pointers address the second. Unique pointers address both simultaneously — a heap-allocated linear value with a single transferable handle.

---

## Proposal

### Part 1 — Linear Types

#### 1.1 Declaring linear types

The `linear` keyword annotates a `struct` or `enum` declaration:

```moonlane
linear struct Buffer {
    ptr: Int,
    len: Int,
}

linear enum Connection {
    Open { socket: Int },
    Closed,
}
```

A struct or enum that contains a `linear` field is itself treated as linear automatically. The `linear` keyword need not be repeated on the outer type — linearity propagates transitively:

```moonlane
struct Request {
    body: Buffer,    // Buffer is linear → Request is implicitly linear
    url: String,
}
```

**Open question:** whether implicit propagation should require an explicit `linear` annotation on the outer type or emit a warning. See OQ-4.

#### 1.2 Linearity sigil at use sites

Linearity must be visible at every use site — in variable declarations, function parameters, and return types. The form `Buffer` alone is a type error if `Buffer` is declared linear; the marked form is required everywhere.

**Open question:** the exact sigil or keyword for the use-site marker is not yet decided. See OQ-1.

#### 1.3 Consumption

A linear value is **consumed** by any of:

- Passing it as an argument to a function
- Returning it from a function or block
- Rebinding it to a new name via `let` (the original binding becomes dead)
- Destructuring it in `match` or a `let` destructure

Consuming an already-consumed linear binding is a compile error. A linear binding that reaches the end of its scope without being consumed is a compile error.

```moonlane
let f = FileHandle::open("data.txt");
f.close();   // consumed — ok

let f2 = FileHandle::open("data.txt");
// scope ends — ERROR: f2 not consumed

let f3 = FileHandle::open("data.txt");
f3.close();
f3.close();  // ERROR: f3 already consumed
```

#### 1.4 Mutation via consume-and-return

There are no mutable references for linear types. Mutation is expressed by consuming the value and returning a new one. Methods on linear types take `self` and return `Self`:

```moonlane
fun write(buf: <linear Buffer>, data: Bytes) -> <linear Buffer> { ... }

let buf = write(buf, data);   // buf consumed; new buf bound
```

Method chaining is the idiomatic form for sequential operations:

```moonlane
buf.write(header).write(body).flush().free();
```

#### 1.5 Branching

Every branch of an `if` or `match` must leave all in-scope linear bindings in the same consumption state at the merge point:

```moonlane
// Correct:
if condition {
    buf.write(data);
    buf.free();
} else {
    buf.free();
}
```

#### 1.6 Loops

A linear value created outside a loop body may not be consumed inside it — the consumption count would be unpredictable. A linear value created inside a loop body is fine; it is created and consumed once per iteration.

#### 1.7 `drop` — explicit discard

```moonlane
drop(buf);   // consumed; satisfies the linearity checker
```

`drop` has the signature `fun<T: Linear>(val: T)`. It does not call a destructor method — the programmer must call the destructor explicitly before dropping if needed.

**Open question:** whether a `Drop` trait with automatic call on unconsumed scope exit should be introduced. See OQ-5.

#### 1.8 Destructuring

Destructuring a linear value consumes the outer binding and introduces each field as a new binding. Each extracted linear field must itself be consumed. Ignoring a linear field with `_` or `..` is a type error.

---

### Part 2 — Read References (`@T`)

#### 2.1 Overview

`@T` is a non-owning, expression-scoped view of a linear value. It allows inspection without consumption, making it possible to call read-only functions without transferring ownership.

`@T` is formed with the `@` prefix operator:

```moonlane
linear struct Buffer { ptr: Int, len: Int }

fun buf_len(b: @Buffer) -> Int { b.len }

let buf = Buffer::alloc(1024);
let len = buf_len(@buf);   // buf is not consumed
buf.free();
```

#### 2.2 Placement rules

- `@T` may only appear in **expression position** — it cannot be bound to a `let`, stored in a struct field, or appear in a function return type
- `@T` is not itself linear — it may be used any number of times within its expression scope
- A function accepting `@T` may read from the value but cannot consume it

Because `@T` cannot be stored, it cannot outlive the expression it appears in. No lifetime annotations are needed.

#### 2.3 No mutable read references

`@mut T` is not introduced by this RFC. Mutation is handled by consume-and-return (§1.4). If in-place mutation without consumption becomes a demonstrated need, a future RFC may revisit it — but it would require borrow-checker-adjacent exclusivity tracking and is deliberately deferred.

#### 2.4 Relationship to `&`

`@x` and `&x` are distinct operators with no overlap:

| Operator | Result type | Storable | Runtime cost | Valid on |
|---|---|---|---|---|
| `&x` | `*T` | yes | RC increment | non-linear `x` only |
| `@x` | `@T` | no | none | linear `x` only |

`&x` where `x` is linear is a type error. `@x` where `x` is non-linear is a type error.

---

### Part 3 — Pointers

#### 3.1 Regular pointers (`*T`, `*mut T`)

A new type-expression variant: `*T` — a pointer to a value of type `T`. RC-backed, cloneable, storable. For non-linear types only.

```moonlane
mut x: Int = 42;
let p: *Int = &x;
let q: *mut Int = &mut x;
```

`*T` and `*mut T` cannot point to linear values. `&x` where `x` is linear is a type error.

**Address-of operators:**

| Expression | Result type | Condition |
|---|---|---|
| `&x` | `*T` | always — `x` may be `let` or `mut` |
| `&mut x` | `*mut T` | type error if `x` is a `let` binding |

**Dereference:** `*p` reads the value. `*p = v` writes through (only valid for `*mut T`).

**Mutability subtyping:** `*mut T` coerces to `*T` implicitly (downgrade safe; upgrade never allowed).

**No auto-deref:** field access through a pointer requires explicit dereference: `(*p).field`. See OQ-6.

**No pointer arithmetic.** `*Int + 1` is a type error.

**Null safety:** absent pointers use `Perhaps<*T>`. There is no implicit null. See OQ-8.

#### 3.2 Unique pointers

A unique pointer `unique *T` is a pointer with exactly one live handle. The handle itself is linear — it cannot be cloned, and it must be consumed exactly once. This makes unique pointers the only mechanism for heap-allocating a linear value while preserving the exactly-once guarantee.

Valid for both linear and non-linear `T`:

| Type | T linear | Handle cloneable |
|---|---|---|
| `*T` | no — type error | yes |
| `unique *T` | yes or no | no |

Recursive linear data structures become possible:

```moonlane
linear struct Node {
    value: Int,
    next: Perhaps<unique *Node>,
}
```

**Open questions:** unique pointer syntax, allocation, and reading through a unique pointer. See OQ-2 and OQ-3.

---

### Part 4 — Typechecker Changes

#### 4.1 Linearity pass

A **linearity environment** (`LinearEnv`) runs as a pass after type inference, once all types are concrete. It maps each in-scope binding to `Unconsumed` or `Consumed(location)`.

| Event | Action |
|---|---|
| `let x = <linear expr>` | Add `x → Unconsumed` |
| Use of linear `x` | If `Unconsumed`: mark `Consumed`. If `Consumed`: error |
| `@x` | Do not mark consumed; verify `x` is `Unconsumed` |
| Scope exit | Error if any linear binding is still `Unconsumed` |
| `if`/`match` merge | Verify `LinearEnv` state is identical across all branches |
| Loop body entry | Snapshot outer linear bindings; forbid consuming any inside body |

#### 4.2 Pointer type additions

- `InferType::Pointer(Box<InferType>, /*mutable*/ bool)` — new variant; `unify` gains pointer cases
- `Type::Pointer(Box<Type>, bool)` — new resolved type variant
- `UnaryOp::AddressOf`, `UnaryOp::AddressOfMut`, `UnaryOp::Deref` — new AST variants
- All `match` on `TypeExpr`, `InferType`, `Type`, and `UnaryOp` gain new arms (exhaustiveness-checked by the compiler)

---

## Open Questions

### OQ-1 — Linearity sigil at use sites

Linearity must be visible at every use site. The exact sigil or keyword is undecided. Requirements: easy to type, visually distinct from `&` and `@`, unambiguous in type position.

Candidates discussed:
- `!Buffer` — concise; `!` is already logical NOT in expression position but unambiguous in type position
- `|Buffer` — unused in the grammar; no strong semantic connection to linearity
- `linear Buffer` — verbose; reuses the declaration keyword; no new syntax

This is the highest-priority open question. All other decisions that reference "the linear type handle" depend on it.

### OQ-2 — Unique pointer syntax and allocation

The working syntax is `unique *T`. Alternatives:
- A sigil form that composes with the linearity sigil from OQ-1
- A keyword other than `unique`

Allocation syntax is also open. Candidate: `Box::alloc(value) -> unique *T` as a standard-library constructor.

### OQ-3 — Reading through a unique pointer

If `p: unique *Buffer`, reading the inner value without consuming `p` requires a mechanism. Options:

- `@p` produces `@Buffer` — the read reference is taken through the pointer automatically
- `@(*p)` — explicit deref then read reference
- Method dispatch on `unique *T` implicitly threads ownership (consume-and-return pattern lifted to the pointer level)

This interacts with OQ-1: if the linear type has a use-site sigil, the type of `@p` must compose correctly.

### OQ-4 — Transitivity warnings

When a non-annotated struct becomes implicitly linear because of a linear field, should the compiler warn or require an explicit `linear` annotation on the outer type?

### OQ-5 — Destructor protocol

Should the language define a `Drop` trait with a `drop(self)` method called automatically when a linear value goes out of scope unconsumed — converting a compile error into an implicit destructor call? This eases migration but weakens the "must be explicit" guarantee.

Related: should `#[auto_drop]` be available as an opt-in per type?

### OQ-6 — Auto-deref for field access

`(*p).field` is the proposed form. A `p->field` operator (C-style) or Go-style implicit deref at field/method boundaries are the alternatives. Implicit deref violates the no-implicit-conversions principle; `->` adds a new operator.

### OQ-7 — Addressability rules

Can `&(x + 1)` take the address of a non-variable expression? Proposed: only named bindings and indexed locations are addressable, consistent with Go.

### OQ-8 — Pointer equality

Should `p == q` compare addresses (identity) or values (`*p == *q`)? Go compares addresses. A `ptr_eq` built-in vs operator overloading vs no equality at all are the options.

### OQ-9 — Linear vs affine types

This RFC proposes linear (exactly once). Affine (at most once — may be dropped silently) is an alternative that prevents double-use without requiring explicit cleanup. A possible design: two keywords (`linear` and `affine`), or a modifier on the declaration. The distinction matters for types that carry no external resource (where silent drop is safe) vs types that own a file handle or socket (where silent drop leaks).

### OQ-10 — Linear type parameters

Can a generic parameter be constrained to linear: `fun<T: Linear>(val: T)`? Required for `drop`. The interaction with v0.2 generics needs design.

---

## References

- Language spec: `docs/public/spec.md`, `docs/public/spec/types.md`
- Typechecker notes: `moonlane-interpreter/docs/typechecker.md`
- Superseded: RFC-0001 (`rfc-0001-pointer-syntax.md`), RFC-0024 (`rfc-0024-linear-types.md`)
- Cluster report: `docs/internal/rfc-cluster-memory-model.md`
- RFC-0006: closure capture — `move fun` syntax depends on this RFC (linear capture semantics)
- RFC-0025: region allocation — `Region` is a linear type; depends on this RFC
- RFC-0026: unsafe blocks — linearity checker relaxed inside `unsafe`; depends on this RFC
- Prior art: Linear Haskell (Bernardy et al. 2018), Rust `Box<T>` and ownership model, Cyclone regions

---
id: rfc-0032
title: "Field-Level Visibility"
date: '2026-05-30'
status: draft
target:
---

## Summary

Make struct field visibility independent of the struct's own visibility. Fields are currently all-public when the struct is `pub`. This RFC proposes making fields **module-private by default** and requiring an explicit `pub` annotation to expose them. This is a **breaking change** for existing `pub struct` definitions.

---

## Motivation

The current spec states: *"Fields of a `pub struct` are public."* There is no mechanism to expose a type's name and API while keeping its internal state hidden. This makes it impossible to enforce invariants on public types — any module can read or write any field directly.

### The core problem

```metel
// token.mln
pub struct Token {
    kind:   TokenKind,   // can't make this read-only from outside
    span:   Span,        // can't hide implementation detail
    offset: Int,         // internal implementation detail — exposed anyway
}
```

Without field-level visibility, `Token` cannot:
- Prevent callers from reading `offset` (an internal detail).
- Prevent callers from constructing `Token` directly with arbitrary field values (bypassing validation logic in a constructor function).
- Expose a stable read-only surface while keeping internal layout flexible.

### Cross-language precedent

All major statically-typed languages with module/package systems support field-level visibility:

| Language | Default field visibility | Opt-in for visibility |
|---|---|---|
| **Rust** | Private to enclosing module | `pub` on field |
| **Swift** | `internal` (module-level) | `private`, `public`, etc. |
| **Kotlin** | `public` | `private`, `internal`, etc. |
| **Java** | Package-private | `public`, `private`, etc. |
| **C#** | `private` | `public`, `internal`, etc. |

Metel's current all-public-or-nothing model is an outlier. The Rust model (fields private by default, `pub` to expose) is the right fit given Metel's Rust-inspired syntax and module system.

---

## Proposal

### Rule: fields are module-private by default

A field with no visibility annotation is accessible only within the module that declares the struct. A field annotated with `pub` is accessible from any module that can see the struct type.

```metel
pub struct Token {
    pub kind:   TokenKind,   // externally readable
    pub span:   Span,        // externally readable
        offset: Int,         // module-private implementation detail
}
```

The struct name (`Token`) follows its own `pub` annotation independently. Field annotations are orthogonal.

### Visibility matrix

| Struct annotation | Field annotation | Type accessible externally? | Field accessible externally? |
|---|---|---|---|
| `pub struct` | `pub field` | Yes | Yes |
| `pub struct` | *(none)* | Yes | No |
| *(none)* `struct` | `pub field` | No | No (struct not nameable) |
| *(none)* `struct` | *(none)* | No | No |

A `pub` annotation on a field of a non-`pub` struct is syntactically valid but has no effect — the field cannot be accessed because the type itself is not externally nameable. It is not an error (it may become relevant if the struct is later made public).

### Construction

Constructing a `pub struct` with any private field from outside the module is a compile error (T0xxx — new error code, see Open Questions):

```metel
// outside token.mln
let t = Token { kind: TokenKind::Ident, span: s, offset: 0 };  // ERROR: field `offset` is private
```

The intended pattern is a public constructor function:

```metel
// token.mln
pub struct Token {
    pub kind:   TokenKind,
    pub span:   Span,
        offset: Int,
}

pub fun Token::new(kind: TokenKind, span: Span) -> Token {
    Token { kind, span, offset: span.start }
}
```

From outside the module, callers use `Token::new(...)`. This enforces invariants at the boundary.

### Pattern matching

Pattern matching from outside the declaring module must use `..` (struct rest pattern) to skip private fields. Explicitly naming a private field in a pattern is a compile error (same error as construction):

```metel
// outside token.mln
match token {
    Token { kind: TokenKind::Ident, span, .. } => { ... }  // OK — offset skipped with ..
    Token { kind, span, offset }               => { ... }  // ERROR: field `offset` is private
}
```

Pattern matching **within** the declaring module has no restrictions — all fields are visible regardless of annotation.

Exhaustiveness checking: a struct with any private fields cannot be exhaustively matched by an external pattern that does not use `..`. The compiler must enforce this.

### `linear struct` and `linear enum`

The same rules apply to `linear struct`. Linear types are still constructable from outside only if all fields are `pub`, or via a public constructor function.

### Enum struct variants

Struct-variant fields in an enum follow the same rules. Tuple-variant fields are positional and cannot be individually annotated — they are all public if the enum is `pub`. (See Open Questions for whether to support per-field `pub` on tuple variants.)

```metel
pub enum Shape {
    Circle { pub radius: Float },            // radius is public
    Rect { pub width: Float, height: Float } // width public, height private
}
```

### Breaking change scope

This RFC changes the default for fields of `pub struct` from implicitly public to module-private. Every existing `pub struct` that expects external field access must be updated to add `pub` on those fields. The compiler must emit a clear migration error.

Because Metel is pre-1.0, this breaking change is acceptable. The CHANGELOG for the target version must document it.

---

## Alternatives Considered

### A — Additive `priv` keyword (non-breaking)

Fields inherit the struct's visibility by default; a `priv` keyword restricts a field to module-private:

```metel
pub struct Token {
    kind:       TokenKind,
    span:       Span,
    priv offset: Int,   // restricted
}
```

**Pros:** Non-breaking; no migration needed.

**Cons:** Unusual — virtually no language uses this model. The default (public fields) is the wrong default for encapsulation. Developers unfamiliar with this choice will accidentally expose fields they intended to keep private. The long-term design pressure will be toward the Rust default anyway; accepting this option defers the breaking change, not eliminates it.

**Verdict:** Rejected. The breaking change at pre-1.0 is the correct time to establish the right default.

### B — Type sealing (OCaml `private` type)

Rather than per-field visibility, seal the struct's *constructor*: the type is visible and field values are readable, but external code cannot construct a value of the type using struct-literal syntax:

```metel
sealed pub struct Token {
    kind:   TokenKind,
    span:   Span,
    offset: Int,
}
```

All fields remain readable; only construction is blocked.

**Pros:** No per-field annotation clutter; read access always works; simple mental model.

**Cons:** Does not support hiding fields (callers can still read `offset`). Does not support partial exposure (some fields public, some private). Less granular than the proposed design. Adding read-hiding later would still require per-field annotations.

**Verdict:** May be useful as a complementary feature alongside field visibility (a sealed type with all-public fields provides a constructor barrier). Not a replacement. Deferred.

### C — Getter/setter asymmetry (Swift / Kotlin model)

Allow a field to have a public getter and a private setter:

```metel
pub struct Counter {
    pub(set: priv) count: Int,
}
```

**Pros:** Expressive; common pattern (read-only external access).

**Cons:** Adds syntax complexity; requires understanding of "properties" vs. raw fields. The same effect can be achieved today with a private field and a public accessor function. Metel does not yet have computed properties, making this premature.

**Verdict:** Deferred until computed properties or properties-as-first-class-syntax are designed.

### D — Module-path scoped visibility (`pub(super)`, `pub(in path)`)

Add scoped visibility modifiers mirroring Rust:

```metel
pub(super) kind: TokenKind,  // visible to parent module
pub(in parser) span: Span,   // visible within parser module subtree
```

**Pros:** Very fine-grained access control; enables crate-internal APIs.

**Cons:** High complexity; Metel's current module system has no `crate`-equivalent scope. `pub` vs. module-private is sufficient for the stated motivation. Can be added later without a breaking change.

**Verdict:** Deferred. The two-level system (`pub` / private) is sufficient for v1.0 scope.

---

## Open Questions

### OQ-1 — New error code for private field access

Two new error conditions need codes:
- Constructing a struct and naming a private field (from outside the module).
- Pattern-matching a struct and naming a private field (from outside the module).

Should these share one error code or be distinct? Recommendation: one code (e.g. T0013) covering "private field accessed from outside declaring module," with the diagnostic message distinguishing construction from pattern-match context.

### OQ-2 — Tuple-variant field visibility

Struct-variant fields can be individually annotated (see Proposal). Tuple-variant fields are positional and harder to annotate:

```metel
pub enum Tree {
    Leaf(Int),            // can we restrict this Int?
    Node(Box<Tree>, Box<Tree>),
}
```

Options:
- **No per-field visibility on tuple variants** (simplest; Rust's model for enums).
- **All-or-nothing**: a `pub` annotation on the variant makes all its fields public; no annotation makes all private.
- **Positional annotation**: `Leaf(pub Int)` — unusual syntax.

Recommendation: **No per-field visibility on tuple variants** for this RFC. Tuple variant fields are all public if the enum is `pub`. Struct-variant fields follow the field annotation rules above. This matches Rust.

### OQ-3 — `pub` field on non-`pub` struct: warning or silent?

If a developer writes `pub field: T` on a struct without `pub`, the annotation is inert. Should the compiler:
- Silently accept it (current proposal text).
- Emit a warning: "field is annotated `pub` but the enclosing struct is not `pub` — annotation has no effect."

Recommendation: warning, not error. It's a common oversight and the developer's intent is clear.

### OQ-4 — Spec update scope

The visibility section of `docs/public/spec/modules.md` currently states "Fields of a `pub struct` are public." Accepting this RFC requires updating that statement and adding the construction-with-private-fields rule. The pattern-matching rule also needs a dedicated section. This should be treated as a spec update in the same commit as RFC acceptance.

---

## Timing Recommendation

This RFC should be resolved before any feature that adds new struct-based types to `std::core` or the standard library — once public structs with all-public fields proliferate in shipped library code, the migration cost grows.

The earliest sensible target is the sprint that first adds library types with private implementation details. It should **not** block v0.6.x or v0.7.0 unless those versions introduce such types.

---

## References

- Language spec: `docs/public/spec/modules.md` — Visibility section
- Implementation report: `metel-interpreter/docs/module-system-report.md` §7.4 (Field-level visibility)
- GitHub issue: #158
- Rust reference: [Visibility and Privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
- Swift: [Access Control](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/accesscontrol/)
- OCaml: [Private Types](https://ocaml.org/manual/5.1/privatetypes.html)

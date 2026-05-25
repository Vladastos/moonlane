---
id: rfc-0008
title: "Aspect Objects (dyn Aspect)"
date: '2026-05-21'
status: draft
---

## Summary

Add runtime polymorphism via aspect objects: values whose concrete type is erased at compile time and dispatch happens through a vtable. The open question is the full semantics — fat-pointer representation, sizing, lifetime, and syntax.

---

## Motivation

The v0.2 aspect system uses only static dispatch (generics + monomorphization). This is sufficient for most cases but requires the concrete type to be known at the call site. Aspect objects enable:

- Heterogeneous collections (`Shape[]` holding `Circle` and `Rectangle`)
- Functions that return an opaque type without generic parameters at the call site
- Plugin-style APIs where the set of types is open

The spec ([Static Dispatch Only](../../public/spec/declarations.md#static-dispatch-only)) explicitly defers `dyn Aspect` to a future version.

---

## Open Questions

- **Syntax**: `dyn Aspect` (Rust-style), just `Aspect` as a type (Go-style), or something else?
- **Sizing**: aspect objects are unsized. Does Moonlane need a `Box<dyn Aspect>` / heap-allocated wrapper, or does the runtime's RC model absorb this?
- **Object safety**: which aspects can be used as aspect objects? Methods with `Self` in position other than receiver break object safety in Rust. Does Moonlane adopt the same rule?
- **Interaction with generics**: can a generic function accept `dyn Aspect` as a type argument?

---

## Decision

**Outcome:** *(pending)*  
**Target:** *(blank until accepted)*

*(Decision rationale goes here when the RFC is evaluated.)*

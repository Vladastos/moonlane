# ADR-0017: Operand Type Validation (T0005) in Pass 2, Not Pass 1

**Status:** Accepted  
**Sprint:** 7 (v0.4.1)  
**Issue:** #141

---

## Context

`TypeErrorCode::T0005` covers "Invalid operand types" — e.g. arithmetic on Bool, unary negation on String. Before v0.4.1, T0005 was defined but never emitted. The type checker silently accepted `true + false` (type: `Bool`) and `-"hello"` (type: `String`).

The question was: where in the two-pass pipeline should operand type validation be inserted?

**Pass 1 (inference, `inference.rs`):**  
`infer_binop` uses HM unification: it creates a fresh type variable `result` and adds constraints `lhs_ty ~ result` and `rhs_ty ~ result`. This ensures both operands have the same type. But HM unification has no notion of "must be numeric" without a typeclass or ad-hoc polymorphism mechanism. Adding a check here would require knowing the resolved type, but Pass 1 works with `InferType` values that may still contain unresolved type variables.

**Pass 2 (construction, `construction.rs`):**  
`construct_binop` runs after the substitution is applied. At this point, all types are fully resolved to concrete `Type` values — `Int`, `Float`, `Bool`, etc. Checking operand types here is straightforward: `matches!(t, Type::Int | Type::Float)`.

## Decision

Operand type validation for T0005 is performed in Pass 2 (`construct_binop` and `construct_unaryop`) where types are fully resolved. The checks are:

- Arithmetic operators (`+`, `-`, `*`, `/`, `%`): operands must be `Int | Float | Never`
- Unary negation (`-`): operand must be `Int | Float | Never`
- Ordering comparisons (`<`, `<=`, `>`, `>=`): operands must be `Int | Float | Str | Never`

`Type::Never` (the bottom type) is always allowed because it coerces to any type — a `Never`-typed expression can appear in any position without being a programmer error.

## Consequences

- `true + false` is now a type error (T0005) rather than silently producing `Bool`.
- The check uses `lhs.ty()` (the left operand's resolved type). Since Pass 1 already ensures `lhs_ty ~ rhs_ty`, checking only the LHS is sufficient.
- Equality comparisons (`==`, `!=`) are deliberately not restricted — they are valid for any pair of equal types, including user-defined structs and enums.

## Constraints for future contributors

- If numeric typeclass overloading is added (e.g. a `Numeric` aspect), the T0005 check in `construct_binop` should be relaxed to allow any type that implements `Numeric`, not just `Int | Float`. At that point, the check should move to a dispatch lookup rather than a hard-coded `matches!`.
- Do not add T0005 validation to Pass 1 (`inference.rs`). Type variables are not yet resolved there; the check would need to be deferred regardless, and Pass 2 is the correct place.
- New operators (e.g. bitwise ops, string concatenation via `+`) must decide whether to add their own operand type checks in `construct_binop` or handle them with unification constraints in `infer_binop`.

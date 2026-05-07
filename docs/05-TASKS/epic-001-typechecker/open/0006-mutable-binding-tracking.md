# Task 0006: Mutable Binding Tracking and Assignment Type Checking

**Status:** open  
**Epic:** epic-001-typechecker  
**Component:** typechecker  
**Spec Link:** docs/01-SPEC/LANGUAGE-SPEC.md#41-immutable-bindings-let  
**Blocked By:** 0005

## What

Extend `InferContext` to track mutability alongside each binding's type, then
implement assignment statement type checking using that information. Currently
the environment stores only `InferType`; there is no way for the typechecker to
distinguish `let` from `mut` bindings or to validate writes.

## Architecture

### Environment entry type change

`mono_env` entries change from `InferType` to `(InferType, bool)`, where the
`bool` is `is_mutable`. All existing `bind_mono` / `lookup` call sites are
updated accordingly. `lookup` (read path) continues to return just `InferType` —
callers do not need to know mutability for reads.

A new method `lookup_for_write` handles the write path:

```
fn lookup_for_write(&self, name: &str, span: &Span)
    -> Result<InferType, YolangError>
```

It performs three checks in sequence:
1. **Existence** — name is in scope; error: "use of undeclared variable `x`"
2. **Mutability** — binding was declared `mut`; error: "cannot assign to immutable binding `x`"
3. Returns the binding's `InferType` for unification at the call site.

### Assignment type checking

`Stmt::Assign { target, op, value }`:
1. Call `lookup_for_write(target)` → get `target_ty`
2. Infer `value` → get `value_ty`
3. For plain `=`: emit constraint `target_ty == value_ty`
4. For compound (`+=`, `-=`, etc.): apply the same BinOp constraint rules as
   `Expr::BinOp` (both operands numeric for arithmetic ops), then emit
   `target_ty == result_ty`

## Acceptance Criteria

- [ ] `mono_env` entries are `(InferType, bool)`; all existing read call sites updated with no behaviour change
- [ ] `bind_mono` gains an `is_mutable: bool` parameter; callers updated (`Decl::Let` → `false`, `Decl::Mut` → `true`, function params → `false`)
- [ ] `lookup_for_write` is implemented and returns the three errors above
- [ ] `Stmt::Assign` with plain `=` type-checks correctly
- [ ] `Stmt::Assign` with compound operators (`+=`, `-=`, `*=`, `/=`, `%=`) type-checks correctly
- [ ] Assigning to a `let` binding produces a `YolangError::TypeError`
- [ ] Assigning to an undeclared name produces a `YolangError::TypeError`
- [ ] Type mismatch on the right-hand side produces a `YolangError::TypeError`
- [ ] All Stage 1 test programs still pass

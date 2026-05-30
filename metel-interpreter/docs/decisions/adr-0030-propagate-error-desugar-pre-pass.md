---
id: adr-0030
title: "? Operator Desugared in path_normalizer Pre-Pass"
date: '2026-05-30'
status: active
---

## Context

Before v0.6.3, `Expr::PropagateError` was a first-class AST node carried through every stage of the pipeline: name resolution, type inference, construction, and the evaluator each had to handle it explicitly. This spread knowledge of one operator's semantics across four passes and made the AST/TypedAST `PropagateError` variants permanent coupling points.

The `?` operator desugars deterministically into a `match` expression:

```metel
expr?
// becomes:
match expr {
    Result::Ok { value } => value,
    Result::Err { error } => return Result::Err { error: error as E },
}
```

where `E` is the enclosing function's declared error type. Importantly, `as E` performs a `From`-based cast using the `ImplMethodKey::FromImpl` dispatch path already in the evaluator — no special evaluator logic is needed once the match is materialised.

## Decision

`PropagateError` is desugared by `path_normalizer::desugar_propagate_error`, called at the start of the normalization pre-pass (before `resolve` rewrites qualified paths). After this point, `Expr::PropagateError` must not appear anywhere in the AST. Enforcement: `path_normalizer::normalize_expr`, `typechecker::inference::infer_expr`, `typechecker::construction::construct_expr`, and `evaluator::eval_expr` all carry `unreachable!("PropagateError must be desugared before ...")` guards in their `PropagateError` arms.

The desugar node produced is a standard `Expr::Match` with a dummy span. Span quality for the generated nodes is tracked separately in #227.

## Alternatives considered

1. **Desugar during type inference** — rejected. The desugared match requires the enclosing function's return type to construct `Result::Err { error: error as E }`. At inference time, this type is still a type variable in the general case; resolving it would require threading additional context through inference.

2. **Handle `PropagateError` as a first-class inference node** — rejected. Would require inference, construction, and evaluator all to understand `?` semantics separately, tripling the surface area for bugs and future changes.

3. **Desugar in the parser** — rejected. The parser does not know the enclosing function's return type; it cannot fill in the `as E` cast.

## Consequences

- `Expr::PropagateError` is removed from the typed pipeline — inference and construction never see it.
- The `PropagateError` variant is retained in `ast::Expr` (for the parser to produce) but must not survive past normalization. The `unreachable!` guards enforce this.
- Span quality of desugared nodes is lower than hand-written code (dummy spans used). See #227.

## Constraints for future contributors

- Do not add handling for `Expr::PropagateError` in inference, construction, or the evaluator. If it appears there, a pass ordering bug has been introduced.
- The desugar pass (`desugar_propagate_error`) must be called before `normalize` in the pipeline. See `path_normalizer::normalize_graph`.
- If the `?` desugaring semantics ever change (e.g. to support `Perhaps` or other result-like types), change `desugar_propagate_error` only — the inference and evaluator layers are unaffected.

# Parser

**Source:** `src/parser/mod.rs`, `src/grammar.pest`
**Entry point:** `parser::parse(source: &str, filename: &str) -> Result<ast::Program, YoloscriptError>`

The parser uses the [pest](https://pest.rs) PEG parser generator. `grammar.pest` defines the complete v0.1 grammar; `parser/mod.rs` drives pest and transforms the resulting CST into an `ast::Program`.

The parser is stub-quality — it produces a correct AST but has no error recovery or diagnostic suggestions. Improving it is explicitly deferred (see `docs/04-PLANNING/MEDIUM-TERM-PLAN.md`).

## Key Files

- `src/grammar.pest` — complete grammar (authoritative source for syntax)
- `src/parser/mod.rs` — CST → AST transformation
- `src/ast/mod.rs` — untyped AST node definitions

## Tests

Parsing is tested in `tests/parsing/` via `tests/lib.rs`. Test sources live in `tests/parsing/sources/`.

```bash
cargo test --test lib parsing_tests
```

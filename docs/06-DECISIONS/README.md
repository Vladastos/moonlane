# Architecture Decision Records

This folder tracks **decisions** — choices that are non-obvious, have meaningful
alternatives, and whose rationale would otherwise be lost in task comments or
conversation history.

## When to write an ADR

Write one when:
- A question has two or more real options with genuine tradeoffs
- The wrong choice would require significant rework to reverse
- The rationale will not be obvious from the code or spec alone

Do NOT write one for:
- Implementation details with a clear single answer
- Choices that are trivially reversible
- Things already documented in the spec or architecture docs

## Workflow

```
1. Question arises in a task or discussion
   ↓
2. Create ADR with status "proposed" — document context and options
   ↓
3. Decision is made — update to "accepted", fill in Decision and Consequences
   ↓
4. Reference the ADR from the relevant task(s) and remove the open question
   ↓
5. If the decision is later reversed — mark "superseded by ADR-NNN", create new ADR
```

## Naming

Files are named `ADR-NNNN-short-slug.md`, e.g. `ADR-0001-type-registry-location.md`.
Numbers are assigned sequentially. The slug is lowercase-hyphenated.

## Referencing from tasks

Add a `**Decisions:**` field to the task header listing any ADRs that govern the
task's design. Example: `**Decisions:** [ADR-0001](../../06-DECISIONS/ADR-0001-type-registry-location.md)`

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [ADR-0001](./ADR-0001-type-registry.md) | TypeRegistry Structure and Location | proposed |
| [ADR-0002](./ADR-0002-inference-pass-structure.md) | Inference Pass Structure | proposed |

# Yolang Documentation Architecture

This document visualizes how Yolang's knowledge is organized and how the pieces connect.

---

## The Documentation Graph

```
                         ┌─────────────────────────────┐
                         │   Language Spec.md          │
                         │  (Source of Truth)          │
                         │  How does the language work?│
                         └──────────────┬──────────────┘
                                        │
                    ┌───────────────────┼───────────────────┐
                    │                   │                   │
                    ↓                   ↑                   ↑
         ┌──────────────────┐  ┌─────────────────┐  ┌──────────────┐
         │  Backlog.md      │  │ decisions/      │  │ processes/   │
         │  (Open work)     │  │ (Why chosen)    │  │ (How we work)│
         │                  │  │                 │  │              │
         │ - open           │  │ - Superseded    │  │ - PROCESS.md │
         │ - deferred       │  │ - Non-obvious   │  │ - LINKING.md │
         │ - in-progress    │  │   choices       │  │ - others...  │
         └────────┬─────────┘  └────────┬────────┘  └──────────────┘
                  │                     │
                  └──────────┬──────────┘
                             ↓
                    ┌─────────────────────┐
                    │  tasks/             │
                    │  (Concrete Work)    │
                    │                     │
                    │ - implement         │
                    │ - validate          │
                    │ - fix               │
                    │ - deliver           │
                    └─────────────────────┘
                             ↓
                    (Implementation)
                             ↓
                    Spec updated with:
                    ✓ Interpreter-validated
                    ✓ Compiler-validated
```

---

## Document Roles

### 1. Language Spec (`spec/Language Spec.md`)

**Role:** Source of truth for the language.

**Contains:**
- How the language works
- Syntax and semantics
- Validation levels: Designed → Interpreter-validated → Compiler-validated

**Relationships:**
- Referenced by **tasks** (what they implement)
- Referenced by **decisions** (what they decide)
- Validated by **tasks** (when complete, spec is tagged)

**Ownership:** Language designers and implementers together

---

### 2. Backlog (`spec/Backlog.md`)

**Role:** Track what still needs design work.

**Contains:**
- Open design questions
- Deferred features (with reasons)
- Items in progress (actively being designed)

**Relationships:**
- Feeds into **decisions** (open → decided)
- Feeds into **spec** (decided → written)
- Disappears when resolved (becomes spec entry)

**Ownership:** Language designers

---

### 3. Decisions (`decisions/NNNN-slug.md`)

**Role:** Record why non-obvious choices were made.

**Contains:**
- Context: why the decision was needed
- Alternatives: what else was considered
- Rationale: why this choice was made
- Links to spec sections being decided

**Relationships:**
- Triggered by **backlog** items (open questions)
- Inform **spec** sections (design choices)
- Referenced by **tasks** (implementation approaches)

**Ownership:** Language designers (immutable once written; superseded with new records)

---

### 4. Tasks (`tasks/NNNN-slug.md`)

**Role:** Track concrete units of work.

**Contains:**
- What to implement/fix/validate
- How it will be done
- Acceptance criteria and status
- Links to spec sections and decisions

**Relationships:**
- Implement or validate **spec** sections
- May depend on **decisions** (approach reasoning)
- Can block other **tasks** (dependencies)
- Result: spec tagged with validation level

**Ownership:** Implementers

---

### 5. Process & Guidance (`docs/Yolang/*.md`)

**Role:** How we work.

**Contains:**
- PROCESS.md: Overall workflow and principles
- LINKING.md: How to cross-reference documents
- CONVENTION.md (in tasks/): Task format and lifecycle
- ARCHITECTURE.md (this file): How documents relate

**Ownership:** Everyone (living documentation)

---

## Key Workflows

### Adding a Feature: Design → Code

```
1. Question arises
   → Add to Backlog (status: open)

2. Design discussion
   → Backlog item (status: in-progress)

3. Decision made
   → Write decision record if non-obvious
   → Update decision record status to "decided"

4. Feature designed
   → Write into Spec
   → Link to decision record
   → Remove from Backlog

5. Implementation assigned
   → Create Task
   → Link to Spec section
   → Link to Decision record (if exists)

6. Implementation work begins
   → Task status: in-progress
   → Update Technical Approach as discoveries happen

7. Implementation complete
   → Task status: completed
   → Update Spec: add validation tag
   → (Interpreter-validated if tested, Compiler-validated if compiled)

8. Backlog cleaned
   → (Already removed when spec entry was created)

Result:
  Backlog.md: removed
  Spec: tagged with validation level
  Decision: linked from spec
  Task: marked completed
```

### Fixing a Bug

```
1. Bug discovered in implementation
   → Create Task describing the bug
   → Link to relevant Spec section

2. Fix implemented and tested
   → Task status: completed
   → Spec validation level unchanged (spec was already correct)

3. Done
```

### Validating Implementation

```
1. Feature implemented, tested in interpreter
   → Update Spec section: `✓ Interpreter-validated (vX.X)`

2. Feature compiled and validated
   → Update Spec section: `✓ Compiler-validated (vX.X)`

Result: Spec tagged with validation level = proof of work
```

---

## Linking Strategies

### Within the Spec

```markdown
> ✓ Interpreter-validated (v0.1)

## Pattern Matching: Syntax

See decision [0008](../../decisions/0008-pattern-matching-syntax.md) for why this syntax was chosen.
```

### Within a Task

```markdown
| **Related Feature** | `spec/Language Spec.md#Pattern-Matching-Syntax` |
| **Related Decision** | `decisions/0008-pattern-matching-syntax.md` |
```

### Within a Decision

```markdown
## Context

Backlog item "Pattern Matching: Syntax Design" required a decision on pattern syntax.

## Rationale

This approach keeps pattern syntax consistent with existing destructuring syntax
(see `spec/Language Spec.md#Destructuring`).
```

---

## Validation Levels

Every spec section can be at one of three levels:

| Level | Meaning | Symbol |
|-------|---------|--------|
| Designed | Written in spec; not yet implemented | (no tag) |
| Interpreter-validated | Implemented and tested in interpreter | `✓ Interpreter-validated (vX.X)` |
| Compiler-validated | Implemented in compiler; codegen verified | `✓ Compiler-validated (vX.X)` |

Progress: Designed → Interpreter-validated → Compiler-validated

---

## State Consistency Rules

The system is consistent when:

1. ✅ Every task references a spec section or backlog item
2. ✅ Every spec section either is in the spec (designed) or in the backlog (not yet designed)
3. ✅ Backlog items are removed when written into the spec
4. ✅ Decisions explain choices in the spec
5. ✅ Task status reflects reality (no stale `in-progress` items)
6. ✅ Spec validation tags reflect actual validation
7. ✅ Implementation matches spec (never diverges)

---

## Tools & Searches

**Find all tasks for a subsystem:**
```bash
grep -l "spec/Language Spec.md#Lexing" tasks/*.md
```

**Find all decisions affecting a topic:**
```bash
grep -l "Error Recovery" decisions/*.md
```

**Track implementation status of a spec section:**
```bash
grep "✓ Interpreter-validated" spec/Language\ Spec.md
```

**Find open backlog items:**
```bash
grep "^- \`open\`" spec/Backlog.md
```

**Find blocked tasks:**
```bash
grep -l "blocked" tasks/*.md | xargs grep "Blocked By"
```

---

## Summary

The four documents form a complete knowledge system:

- **Spec** = "what we're building" (the spec)
- **Backlog** = "what we haven't designed yet" (design work)
- **Decisions** = "why we chose X" (design rationale)
- **Tasks** = "who's doing what" (implementation work)

They link together to form a traceable chain from question → decision → design → implementation → validation.

No orphaned work, no forgotten decisions, no mystery choices.

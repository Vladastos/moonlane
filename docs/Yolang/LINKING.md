# Cross-Document Linking Guide

This guide shows how to reference and link between Yolang's documentation, spec, decisions, and tasks to maintain a coherent knowledge structure.

---

## Document Types and Their Purposes

| Document | Purpose | Location | Linking |
|----------|---------|----------|---------|
| **Language Spec** | Source of truth for how the language works | `spec/Language Spec.md` | Reference sections with `#Anchor` |
| **Backlog** | Design questions and missing features | `spec/Backlog.md` | Reference items by title or line |
| **Decision Records** | Why non-obvious choices were made | `decisions/NNNN-slug.md` | Reference by filename or title |
| **Tasks** | Concrete units of work (implementation, validation, fixes) | `tasks/NNNN-slug.md` | Reference by filename or task ID |
| **Design Documents** | Deep dives into subsystem architecture | `docs/Yolang/SUBSYSTEM/DESIGN.md` | Reference sections with `#Anchor` |
| **Process Docs** | How we work (this one, CONVENTION.md, etc.) | `docs/Yolang/*.md` | Reference by filename |

---

## Linking Patterns

### In Metadata Tables (Tasks and Decisions)

Use **absolute paths** relative to project root for clarity:

```markdown
| **Related Feature** | `spec/Language Spec.md#Control-Flow-If-Statements` |
| **Related Decision** | `decisions/0005-error-recovery-strategy.md` |
| **Blocking** | Tasks 0043, 0044 (see task metadata for full context) |
```

### In Body Text

For references within body text, use descriptive links:

```markdown
See the pattern matching syntax in the spec [here](spec/Language Spec.md#Patterns).

This implements the decision documented in [ADR-0005](decisions/0005-error-recovery-strategy.md).

Task 0001 ([implement-basic-parser](tasks/0001-implement-basic-parser.md)) must complete first.
```

### Spec Sections

When the spec is complete enough to validate, tag it with its readiness level. Use standard anchor formatting:

```markdown
> ✓ Interpreter-validated (v0.1)

## Control Flow: If Statements

... spec content ...
```

When linking to this section from a task or decision:

```markdown
| **Related Feature** | `spec/Language Spec.md#Control-Flow-If-Statements` |
```

---

## Common Linking Scenarios

### A Task Implementing a Spec Feature

Task metadata:

```markdown
| **Related Feature** | `spec/Language Spec.md#Pattern-Matching-Basic-Syntax` |
| **Related Decision** | `decisions/0008-pattern-matching-syntax.md` |
```

Task summary:

```markdown
Implement pattern matching as specified in `spec/Language Spec.md#Pattern-Matching-Basic-Syntax`.
This resolves the backlog item "Pattern Matching: Syntax Design" and implements the approach
decided in [decisions/0008](decisions/0008-pattern-matching-syntax.md).
```

When the task is completed, update the spec section tag:

```markdown
> ✓ Interpreter-validated (v0.1)

## Pattern Matching: Basic Syntax
```

### A Backlog Item with Open Questions

Backlog format (`spec/Backlog.md`):

```markdown
## Pattern Matching: Destructuring Rules

**Status:** open

What should happen when a pattern doesn't cover all cases?
See decision [decisions/0008](decisions/0008-pattern-matching-syntax.md) for syntax.
Task [0042](tasks/0042-implement-pattern-matching.md) will implement once this is decided.
```

### A Decision Reversing a Prior Decision

New decision record:

```markdown
# Decision 0012: Simplify Pattern Matching Destructuring

## Supersedes

[Decision 0008: Pattern Matching Syntax](decisions/0008-pattern-matching-syntax.md) — specifically the section on destructuring rules.

...
```

Update the old decision record's status at the top:

```markdown
> ⚠️ Superseded by [Decision 0012](decisions/0012-simplify-pattern-destructuring.md)

# Decision 0008: Pattern Matching Syntax
```

---

## Reference Quick-Check

Before linking, ask:

1. **Does this document exist?** If not, create it or link to the backlog item instead.
2. **Is it the right document type?**
   - **Design question?** → Backlog
   - **Why was a choice made?** → Decision record
   - **How the feature works?** → Spec
   - **How to implement it?** → Task
   - **How a subsystem is built?** → Design document
3. **Is the link accurate?** Check the section anchor exists in the target.
4. **Is it spec-first?** A task should always reference a spec section; a decision should always explain a choice about something in the spec.

---

## Maintaining Links

When updating a document:

1. **If you move or rename a section,** search for backlinks and update them.
2. **If you update a decision,** check if dependent tasks or spec sections need updates.
3. **If you implement a task,** update the spec with the validation level tag.
4. **If a backlog item is resolved,** remove it from the backlog and ensure it's in the spec.

Use your editor's search-in-project feature to find references:

```bash
# Find references to a task
grep -r "0042" docs/ tasks/ decisions/ spec/

# Find references to a spec section
grep -r "Pattern-Matching" docs/ tasks/ decisions/
```

---

## Examples

### Complete Workflow

1. **Open question** → Added to `spec/Backlog.md`:
   ```
   ## String Escaping: Behavior with Unicode
   **Status:** open
   Should string escapes like \uXXXX be validated at lex time or parse time?
   ```

2. **Decision made** → Create `decisions/0009-string-escape-validation.md`:
   ```markdown
   # Decision 0009: Validate String Escapes at Lex Time
   See backlog item "String Escaping: Behavior with Unicode" for context.
   ```

3. **Feature designed** → Add to `spec/Language Spec.md#Strings`:
   ```markdown
   > ✓ Designed (not yet validated)

   ## Strings: Escape Sequences
   String escapes are validated at lex time (see [Decision 0009](decisions/0009-string-escape-validation.md)).
   ```

4. **Work assigned** → Create `tasks/0050-implement-string-escape-validation.md`:
   ```markdown
   | **Related Feature** | `spec/Language Spec.md#Strings-Escape-Sequences` |
   | **Related Decision** | `decisions/0009-string-escape-validation.md` |
   ```

5. **Implemented** → Update spec tag when task is completed:
   ```markdown
   > ✓ Interpreter-validated (v0.1)
   ```

6. **Backlog cleaned** → Remove from backlog, move to completed:
   - Delete "String Escaping: Behavior with Unicode" from backlog
   - Proof it's done: it's in the spec with validation tag

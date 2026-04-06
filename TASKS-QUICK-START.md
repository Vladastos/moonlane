# Tasks Quick Start

Welcome to the Yolang task management system. This is a 2-minute overview.

## The System in One Picture

```
BACKLOG (Design questions)
    ↓ (question answered)
DECISION RECORD (Why we chose X)
    ↓ (decision made)
LANGUAGE SPEC (Feature designed)
    ↓ (ready to code)
TASK (Implementation work)
    ↓ (work done)
SPEC (tagged as ✓ Interpreter-validated)
```

## Four Kinds of Documents

| Document | Purpose | Status Examples |
|----------|---------|---|
| **Backlog** | What still needs design | open, in-progress, deferred |
| **Decisions** | Why we chose X over Y | superseded, active |
| **Spec** | How the language works | ✓ Designed, ✓ Interpreter-validated |
| **Tasks** | Who's doing what | open, in-progress, completed |

## Creating a Task (5 Steps)

1. **Copy the template**
   ```bash
   cp tasks/0000-template.md tasks/NNNN-slug.md
   ```
   (Replace `NNNN` with next ID from `ls tasks/ | grep '^[0-9]'`)

2. **Fill in the metadata**
   - Status: `open` (or `in-progress` if starting immediately)
   - Priority: `high` / `medium` / `low`
   - Effort: `1` (quick) to `5` (major)
   - Owner: who's responsible
   - Dates: today's date

3. **Link to the spec**
   ```markdown
   | **Related Feature** | `spec/Language Spec.md#Section-Name` |
   ```
   → Every task must reference a spec section or backlog item

4. **Write the summary and criteria**
   - Summary: one paragraph on what needs doing
   - Criteria: checklist of what "done" looks like

5. **That's it!** Set status to `open` and you're ready

## Updating a Task

**When you start work:**
```markdown
| **Status** | in-progress |
| **Owner** | @yourname |
| **Updated** | 2026-04-07 |
```

**When you finish:**
```markdown
| **Status** | completed |
| **Updated** | 2026-04-07 |
```

Then **update the spec** with the validation tag:
```markdown
> ✓ Interpreter-validated (v0.1)
```

## Linking Tasks to Everything

- **Task → Spec**: `| **Related Feature** | spec/Language Spec.md#Section |`
- **Task → Decision**: `| **Related Decision** | decisions/0005-slug.md |`
- **Task → Task**: `| **Blocked By** | Task 0042 |`

This way, everything is traceable:
- Design question ← → Backlog item
- Backlog item ← → Decision record
- Decision record ← → Spec section
- Spec section ← → Task

## Key Commands

```bash
# List all tasks
ls tasks/ | grep -v template

# Find in-progress work
grep -l "in-progress" tasks/*.md

# Find what blocks you
grep "Blocked By" tasks/NNNN-slug.md

# Find what you block
grep "blocking" tasks/*.md

# Search the spec for a feature
grep "Pattern Matching" spec/"Language Spec.md"
```

## One Golden Rule

**Always update the spec before (or with) implementation.**

Never implement something that's not in the spec. If the spec is unclear:
1. Stop
2. Clarify in the spec (or write a decision record)
3. Continue with implementation

The spec and implementation must stay in sync. When they diverge, that's a bug.

## Quick Links

- **Full task convention**: `docs/Yolang/tasks/CONVENTION.md`
- **Task checklist**: `docs/Yolang/tasks/CHECKLIST.md`
- **How to link documents**: `docs/Yolang/LINKING.md`
- **Overall architecture**: `docs/Yolang/ARCHITECTURE.md`
- **Development process**: `docs/Yolang/PROCESS.md`

## Status Meanings

- **open** — ready to start, not yet claimed
- **in-progress** — actively being worked on
- **blocked** — waiting for something (another task, decision, clarification)
- **completed** — done, spec updated with validation tag
- **deferred** — intentionally postponed (reason in Notes)

## Example Task

```markdown
# Task 0042: Implement pattern matching syntax validation

## Metadata

| Field | Value |
|-------|-------|
| **Status** | in-progress |
| **Priority** | high |
| **Estimated Effort** | 3 |
| **Related Feature** | `spec/Language Spec.md#Pattern-Matching-Syntax` |
| **Related Decision** | `decisions/0008-pattern-matching-syntax.md` |
| **Blocking** | 0043 (pattern matching codegen) |
| **Blocked By** | none |
| **Owner** | @alice |
| **Created** | 2026-04-06 |
| **Updated** | 2026-04-07 |

## Summary

Implement the pattern matching validator for the interpreter.
The syntax and rules are defined in the spec (see Related Feature).
This validates patterns match the spec definition and provides useful error messages.

## Acceptance Criteria

- [ ] All pattern syntax from spec is recognized and validated
- [ ] Invalid patterns produce clear error messages
- [ ] All test cases in tests/patterns/ pass
- [ ] No regressions in existing tests
- [ ] Code reviewed

## Notes

- Started parsing the spec section on 2026-04-07
- Found an ambiguity: what about nested patterns with alternative?
- Created decision record 0008 to clarify
```

---

**Start here:**
1. Read this file (2 min)
2. Copy `tasks/0000-template.md`
3. Fill it in
4. Link to the spec
5. Done!

For more details, see the full documentation in `docs/Yolang/tasks/`.

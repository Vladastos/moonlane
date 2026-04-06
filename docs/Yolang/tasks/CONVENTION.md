# Yolang Task Management Convention

This document defines how tasks are organized, named, and linked within the Yolang project. Tasks are concrete units of work that implement, validate, or fix a feature described in the spec.

---

## Overview

Tasks are stored in `/tasks/` at the project root. Each task is a single Markdown file with a standardized format. Tasks are used to track implementation work, validation, and fixes—not design work (which lives in the backlog).

**Key principle:** A task has a clear outcome: implement a feature, validate it works, fix a bug, or complete a concrete deliverable. Design discussions belong in the backlog; execution belongs in tasks.

---

## Task Lifecycle

```
open → in-progress → completed
  ↓                      ↑
  └──────→ blocked ──────┘
  ↓
  deferred
```

- **open** — not yet started
- **in-progress** — actively being worked on
- **blocked** — waiting for something external (another task, decision, clarification)
- **completed** — done and verified
- **deferred** — intentionally postponed; reason given

---

## Naming Convention

Tasks are named: `NNNN-kebab-case-slug.md`

- `NNNN` — zero-padded sequential ID (0001, 0042, etc.)
- `kebab-case-slug` — short, descriptive, action-oriented

**Good examples:**
- `0001-implement-basic-parser.md`
- `0042-fix-memory-leak-in-lexer.md`
- `0015-validate-pattern-matching.md`

**Poor examples:**
- `implement parser` — spaces, not kebab-case
- `bug-fix` — not descriptive enough
- `refactoring-the-whole-interpreter-to-be-better` — too vague and broad

---

## Task File Format

Every task file follows this structure:

```markdown
# Task NNNN: Task Title

## Metadata

| Field | Value |
|-------|-------|
| **Status** | open / in-progress / completed / blocked / deferred |
| **Priority** | high / medium / low |
| **Estimated Effort** | 1–5 scale (1 = quick, 5 = major undertaking) |
| **Related Feature** | `spec/Language Spec.md#Section-Name` |
| **Related Decision** | `decisions/0001-slug.md` or "none" |
| **Blocking** | task IDs if this blocks other work |
| **Blocked By** | task IDs or external items if this is blocked |
| **Owner** | who is responsible (or "unassigned") |
| **Created** | YYYY-MM-DD |
| **Updated** | YYYY-MM-DD |

## Summary

One paragraph explaining what needs to be done and why. Should be concrete and outcome-focused.

## Acceptance Criteria

What does "done" look like? Use a checklist:

- [ ] Feature implemented and tested
- [ ] No regressions in existing tests
- [ ] Code reviewed
- [ ] Spec updated if ambiguities were found (link to decision if non-obvious)

## Technical Approach

Optional. Describe *how* you plan to approach this task if the approach is non-trivial or if you want to document it before starting.

## Notes

Running notes: discoveries, blockers encountered, decisions made during implementation.

---

## Example Task

```markdown
# Task 0042: Fix lexer memory leak in string tokens

## Metadata

| Field | Value |
|-------|-------|
| **Status** | in-progress |
| **Priority** | high |
| **Estimated Effort** | 2 |
| **Related Feature** | `spec/Language Spec.md#Strings` |
| **Related Decision** | none |
| **Blocking** | 0043 (memory profiling), 0044 (release v0.2) |
| **Blocked By** | none |
| **Owner** | @myself |
| **Created** | 2026-04-06 |
| **Updated** | 2026-04-07 |

## Summary

The lexer is not deallocating string token buffers in certain error paths. This causes memory to leak when malformed strings are encountered. Needs to be fixed before the interpreter can be memory-profiled (Task 0043).

## Acceptance Criteria

- [ ] Reproducer for the leak identified and confirmed
- [ ] Root cause in lexer/token.rs identified
- [ ] Fix implemented
- [ ] valgrind or LSAN shows no leak on test suite
- [ ] Code reviewed

## Technical Approach

1. Run the interpreter on the test suite under valgrind/LSAN to identify the leak location
2. Trace the allocation/deallocation path for string tokens
3. Ensure error paths in the lexer properly clean up allocated buffers
4. Consider if a RAII pattern would prevent this in the future

## Notes

- Encountered on 2026-04-07 during test suite profiling
- Likely related to early return in error recovery (Task 0001)
- Will check if error recovery paths are also affected
```

---

## Linking Tasks to Other Documents

### To the Language Spec

If a task implements a feature in the spec, link to it in metadata:

```markdown
| **Related Feature** | `spec/Language Spec.md#Control-Flow-If-Statements` |
```

When a task is **completed and validated**, update the spec to reflect the validation level (see `PROCESS.md`):

```markdown
> ✓ Interpreter-validated (v0.1)
```

### To Decision Records

If a task's approach or acceptance criteria depend on a design decision, link to the decision record:

```markdown
| **Related Decision** | `decisions/0005-error-recovery-strategy.md` |
```

### Inter-Task Dependencies

Use the **Blocking** and **Blocked By** fields to declare task dependencies:

```markdown
| **Blocking** | 0043, 0044 |
| **Blocked By** | 0041 |
```

When a blocking task is completed, update the dependent task to remove the blocker.

### To Backlog Items

If a task is resolving a backlog item, reference it in the **Summary** section:

```
## Summary

This task implements the pattern matching feature described in the spec (see `spec/Backlog.md` — item "Pattern Matching: Design").
```

When the task is completed, the backlog item should be removed and the feature should be in the spec.

---

## Task Lifecycle in Practice

### Creating a New Task

1. Ensure the work is captured in the spec or backlog (it describes what, the task describes how).
2. Create a new file: `NNNN-slug.md` using the next sequential ID.
3. Fill in the metadata header, summary, and acceptance criteria.
4. Set **Status** to `open`.
5. If you know the approach, add a **Technical Approach** section.

### Starting a Task

1. Set **Status** to `in-progress`.
2. Update **Updated** date.
3. If it depends on other work, set **Blocked By** and keep status as `blocked` until dependencies are resolved.

### Completing a Task

1. Verify all acceptance criteria are met.
2. Update the **Related Feature** section in the spec with the validation level tag if applicable.
3. If implementation revealed spec ambiguities, create a decision record and link it.
4. Update any tasks that this task was blocking.
5. Set **Status** to `completed`.
6. Update **Updated** date.

### Deferring a Task

1. If a task needs to be postponed, set **Status** to `deferred`.
2. Add a one-line reason in the **Notes** section.

---

## Status Reporting

To get a quick overview of work in progress:

```bash
grep -l "in-progress" tasks/*.md
```

To see what's blocked:

```bash
grep -l "blocked" tasks/*.md
```

To see what's completed:

```bash
grep -l "completed" tasks/*.md
```

---

## What *Not* to Do

- **Do not use tasks for design discussions.** Open questions go in the backlog. Tasks are for execution.
- **Do not create a task until the feature is in the spec (or a backlog item).** A task without a spec reference is orphaned work.
- **Do not leave tasks in `in-progress` indefinitely.** Regularly review and either complete, defer, or mark as blocked.
- **Do not modify acceptance criteria after starting.** If scope changes, create a new task or update the metadata but document the change in **Notes**.
- **Do not let tasks get out of sync with reality.** If you stop work, mark it blocked or deferred; update the **Updated** date and explain in **Notes**.

---

## Template

A blank template is available at `tasks/0000-template.md`. Copy and rename it to create a new task.

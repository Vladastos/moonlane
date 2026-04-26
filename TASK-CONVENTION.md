# Task Management for Yolang

Simple task tracking system. One file per unit of work.

## Quick Reference

**Create a task:** Copy `docs/Yolang/tasks/0000-template.md` → rename to `NNNN-slug.md`

**Update status:** Change `**Status:**` field (open → in-progress → done → blocked)

**When done:**
1. Check all acceptance criteria
2. Update spec section if it reveals ambiguities
3. Mark status as `done`

## Task Fields

```markdown
# Task NNNN: Brief Title

**Status:**      open | in-progress | done | blocked
**Component:**   interpreter | repl | parser | typechecker | evaluator | error-handling | spec
**Spec Link:**   spec/Language Spec.md#Section-Name (or Backlog item)
**Blocked By:**  task IDs or "none"

## What
What needs doing and why.

## Acceptance Criteria
- [ ] Testable outcome 1
- [ ] Testable outcome 2
- [ ] No regressions

## Notes
(Optional) Progress and discoveries
```

## Rules

1. **Every task links to the spec** (or a backlog item if not yet speced)
2. **Components:** Connect tasks to the subsystems they affect
   - `interpreter` — overall interpreter
   - `repl` — interactive shell
   - `parser` — parsing and grammar
   - `typechecker` — type inference/checking
   - `evaluator` — runtime execution
   - `error-handling` — error messages and recovery
   - `spec` — spec work only
3. **Status is honest:** If you haven't touched a task in days, mark it `blocked` with reason
4. **Acceptance criteria are testable:** Not "improve error messages" but "error reports include X and Y"

## Current Tasks

- `0001-error-recovery.md` — Error recovery in parser (in-progress)
- `0002-no-match-error.md` — Better error messages (open, blocked by 0001)
- `0003-repl-with-debug.md` — Add interactive REPL mode (open, blocked by 0001, 0002)
- `0004-repl-debug-commands.md` — Debug commands for REPL (open, blocked by 0003)

## Workflow

```
1. Create task with status "open"
   ↓
2. Start work → status "in-progress"
   ↓
3. If you get stuck → status "blocked" (with reason)
   ↓
4. Finish → check criteria, update spec, status "done"
```

That's it. See `docs/Yolang/tasks/README.md` for more details or `docs/Yolang/tasks/0000-template.md` to create a task.

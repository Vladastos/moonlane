# Task Creation & Management Checklist

Use this checklist when creating or updating tasks.

---

## Creating a New Task

- [ ] **Is there a spec section or backlog item for this work?**
  - If no → Stop. Either add to backlog first or update spec first.
  - If yes → Proceed.

- [ ] **Choose the next task ID**
  - Check what IDs are already used: `ls tasks/ | grep -oE '^[0-9]+' | sort -n`
  - Use the next sequential number

- [ ] **Write the task file**
  - [ ] Copy `tasks/0000-template.md`
  - [ ] Rename to `NNNN-slug.md`
  - [ ] Fill in metadata: status, priority, effort, owner, dates

- [ ] **Complete the summary**
  - [ ] One clear paragraph
  - [ ] Explains what needs doing and why
  - [ ] Links to relevant spec section

- [ ] **Define acceptance criteria**
  - [ ] Use checklist format
  - [ ] Each criterion is testable/verifiable
  - [ ] Include code review and test coverage

- [ ] **Add technical approach** (if non-trivial)
  - [ ] Describe the planned approach
  - [ ] Note any key decisions or unknowns
  - [ ] Flag dependencies on other tasks

- [ ] **Link to related documents**
  - [ ] `Related Feature`: link to spec section (required unless "none")
  - [ ] `Related Decision`: link to decision record if one exists
  - [ ] `Blocked By` / `Blocking`: other task IDs if dependencies exist

- [ ] **Set initial status**
  - [ ] If unassigned or blocked → `open`
  - [ ] If assigned and ready → `open` (not `in-progress` yet)
  - [ ] Update when actually starting work

---

## Starting a Task (Moving to In-Progress)

- [ ] **Confirm prerequisites**
  - [ ] All blocking tasks are completed
  - [ ] Owner is assigned
  - [ ] Spec section exists and is understood

- [ ] **Update task metadata**
  - [ ] Change status to `in-progress`
  - [ ] Verify owner is set correctly
  - [ ] Update `Updated` date

- [ ] **Expand Technical Approach**
  - [ ] Add any discoveries from spec reading
  - [ ] Link to specific locations in code or spec
  - [ ] Flag unknowns or risks

---

## During Implementation

- [ ] **Keep Notes section current**
  - [ ] Document discoveries
  - [ ] Record blockers as they arise
  - [ ] Note any spec ambiguities found

- [ ] **If spec ambiguities surface**
  - [ ] Stop and resolve in the spec first
  - [ ] Create a decision record if non-obvious
  - [ ] Update task to reference the decision

- [ ] **Track blockers**
  - [ ] If blocked on another task → update `Blocked By`
  - [ ] If blocking other tasks → notify their owners

- [ ] **Keep the task file honest**
  - [ ] Update `Updated` date when making changes
  - [ ] Update acceptance criteria only if scope genuinely changes

---

## Completing a Task

- [ ] **All acceptance criteria met?**
  - [ ] Every checkbox is checked
  - [ ] Code review completed
  - [ ] Tests pass, no regressions

- [ ] **Update the spec**
  - [ ] Add validation level tag to the related spec section:
    - Interpreter-validated: `> ✓ Interpreter-validated (v0.X)`
    - Compiler-validated: `> ✓ Compiler-validated (v0.X)`

- [ ] **Create or update decision record** (if needed)
  - [ ] If implementation revealed spec ambiguities → decision record created
  - [ ] Link the decision record from the task

- [ ] **Unblock dependent tasks**
  - [ ] Find any tasks where `Blocked By` lists this task
  - [ ] Update their status if they can now proceed

- [ ] **Update task status**
  - [ ] Change status to `completed`
  - [ ] Update `Updated` date

- [ ] **Clean up backlog** (if applicable)
  - [ ] If this task resolved a backlog item → remove from backlog
  - [ ] Proof: the feature is now in the spec with a validation tag

---

## Deferring a Task

- [ ] **Document the reason**
  - [ ] Add note to `Notes` section: "Deferred because: ..."
  - [ ] This will be important context later

- [ ] **Update status**
  - [ ] Change status to `deferred`
  - [ ] Update `Updated` date

- [ ] **Notify affected tasks**
  - [ ] Any task in this task's `Blocking` list should be updated

---

## Reviewing Task Health

**Regular (weekly or sprint-based):**

- [ ] Check for stale `in-progress` tasks
  - [ ] `Updated` date is recent (within last week)
  - [ ] If stale → task owner should explain or move to `blocked`/`deferred`

- [ ] Check for cyclic blockers
  - [ ] Task A blocks Task B, Task B blocks Task A?
  - [ ] Resolve by breaking the cycle or deferring one

- [ ] Verify task links are accurate
  - [ ] Related spec sections still exist and are relevant
  - [ ] Related decisions are correctly referenced

**Before a release:**

- [ ] Find all `completed` tasks for this release
- [ ] Verify their corresponding spec sections are validation-tagged
- [ ] Verify no `deferred` task affects this release's features

---

## Common Mistakes to Avoid

❌ Creating a task without a spec section or backlog item
- → Tasks are implementation of design, not design themselves

❌ Leaving a task `in-progress` for weeks with no updates
- → Make the status honest: mark as `blocked` with reason or `deferred`

❌ Implementing something not in the spec
- → Stop and update spec first (or create a decision record)

❌ Updating spec *after* implementation is done
- → Update spec first, then implement against it

❌ Forgetting to link related documents
- → Orphaned tasks are hard to understand later

❌ Vague acceptance criteria
- → "Make it work" is not an acceptance criterion
- → "Implement XYZ such that [test/behavior] is true" is

❌ Not updating `Updated` date
- → Stale dates make it hard to know if task is actually active

---

## Quick Reference

| I want to... | Do this |
|---|---|
| Create a new task | Copy `0000-template.md`, fill in all fields, set status to `open` |
| Start working on a task | Move status to `in-progress`, assign owner, set today's date |
| I found an ambiguity in the spec | Stop work, update spec, create decision record, continue |
| Finish a task | Check all criteria, update spec with validation tag, mark `completed` |
| Stop working on a task | Mark `blocked` (if waiting) or `deferred` (if deprioritized) |
| See what's blocking me | Check the `Blocked By` field and follow up on those tasks |
| See what I'm blocking | Check the `Blocking` field and notify those owners when done |

---

For full details, see:
- `docs/Yolang/tasks/CONVENTION.md` — Complete task format and lifecycle
- `docs/Yolang/LINKING.md` — How to link documents correctly
- `docs/Yolang/PROCESS.md` — Overall language development process

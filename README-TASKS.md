# Task Management in Yolang

Quick reference for how tasks fit into the Yolang project workflow.

---

## The Four Documents

Your documentation is organized into four types, each with a specific role:

1. **`spec/Language Spec.md`** — The source of truth for how Yolang works
   - Written when a feature is designed and accepted
   - Updated when implementation reveals gaps or errors
   - Tagged with validation levels: `✓ Interpreter-validated`, `✓ Compiler-validated`

2. **`spec/Backlog.md`** — What still needs to be designed
   - Open design questions
   - Deferred features with reasons
   - Tracks progress from "not yet designed" → "spec-ready"

3. **`decisions/NNNN-slug.md`** — Why non-obvious choices were made
   - Explains the reasoning for spec decisions that had alternatives
   - Never modified; superseded decisions get new records
   - Referenced from tasks and spec sections

4. **`tasks/NNNN-slug.md`** — Concrete units of work
   - Implement a feature, validate it, or fix a bug
   - Always reference a spec section or backlog item
   - Tracked from open → in-progress → completed (or deferred)

---

## Task Structure

Each task has a consistent format:

```markdown
# Task NNNN: Title

## Metadata
- Status: open | in-progress | completed | blocked | deferred
- Priority: high | medium | low
- Effort: 1-5 scale
- Related Feature: link to spec section
- Related Decision: link to decision record (if applies)
- Blocking/Blocked By: other task IDs
- Owner: who's responsible
- Created/Updated: dates

## Summary
What needs to be done and why.

## Acceptance Criteria
What does "done" look like? (checklist)

## Technical Approach
[Optional] How you plan to solve it.

## Notes
[Optional] Progress, discoveries, blockers.
```

**See:** `docs/Yolang/tasks/CONVENTION.md` for the full convention and examples.

---

## Linking Everything Together

Documents link to each other to maintain a coherent picture:

- **Tasks** link to **spec sections** (what they implement)
- **Tasks** link to **decision records** (why the approach was chosen)
- **Decision records** link to **spec sections** (what they decide)
- **Spec sections** are tagged with **validation status** (when a task completes)

**Example:**

```
spec/Backlog.md (open question)
    ↓
decisions/0005.md (decision made)
    ↓
spec/Language Spec.md (feature designed)
    ↓
tasks/0042.md (implementation work)
    ↓
spec/Language Spec.md (tagged ✓ Interpreter-validated)
```

**See:** `docs/Yolang/LINKING.md` for detailed patterns and examples.

---

## Workflow

### Adding a New Feature

1. **Design question?** → Add to `spec/Backlog.md` with status `open`
2. **Decision made?** → Write decision record in `decisions/` (if non-obvious)
3. **Feature ready?** → Write into `spec/Language Spec.md`
4. **Time to code?** → Create task in `tasks/` linking to the spec section
5. **Task done?** → Update spec section with `✓ Interpreter-validated` tag
6. **Backlog cleaned** → Remove the item from backlog (it's now in the spec)

### Fixing a Bug

1. Create a task in `tasks/` describing the bug and what "fixed" means
2. Link to the relevant spec section
3. When fixed, update spec validation level and mark task completed

### Validating a Feature

1. The interpreter validates it works as spec'd → tag spec: `✓ Interpreter-validated (v0.1)`
2. The compiler implements it → tag spec: `✓ Compiler-validated (v0.1)`

---

## Current Tasks

Your existing tasks live in `/tasks/`:

- `0001-error-recovery.md` — Implement error recovery during parsing
- `0002-no-match-error.md` — [See the task file]
- `0003-repl-with-debug.md` — [See the task file]

Use `0000-template.md` as a starting point when creating new tasks.

---

## Quick Commands

```bash
# List all tasks
ls -1 tasks/ | grep -v template

# See in-progress work
grep -l "in-progress" tasks/*.md

# See blockers
grep -l "blocked" tasks/*.md

# See completed work
grep -l "completed" tasks/*.md

# Find a task by name
ls tasks/ | grep "keyword"

# Search for references to a task
grep -r "0042" docs/ decisions/ spec/
```

---

## Key Principles

- **Spec-first:** Always update the spec before (or alongside) implementing.
- **No divergence:** Implementation and spec must stay in sync. If they diverge, it's a bug in one or the other.
- **Link everything:** Design decisions, task assignments, and validation status are all connected.
- **Track status:** Keep task status and dates current. A stale task is misleading.
- **Defer explicitly:** If something won't make this release, mark it deferred with a reason.

---

## For More Details

- **Task convention:** `docs/Yolang/tasks/CONVENTION.md`
- **Linking patterns:** `docs/Yolang/LINKING.md`
- **Overall process:** `docs/Yolang/PROCESS.md`
- **Task template:** `tasks/0000-template.md`

# Planning

This folder contains phase definitions for the Yoloscript project. Each file describes one development phase.

## Convention

**One file per phase.** A phase is a broad development era with a distinct goal, quality target, and definition of done — not a sprint or a milestone.

**File naming:** `PHASE-NN-slug.md`

**Update triggers:**
- A phase completes and the next phase begins
- A milestone is reached or added
- The scope of the current phase changes significantly (something large added or dropped)

**Not update triggers:** individual epic or task completion, implementation findings (those go to the spec or an ADR), or routine development progress.

## Structure of a Phase Doc

Each phase doc answers five questions:

1. **Goal** — what are we building and why at this scale?
2. **Success criterion** — how do we know the phase is complete?
3. **Milestones** — observable checkpoints within the phase, in order
4. **Scope** — what is in scope; what is explicitly deferred and why
5. **Next phase** — one-line preview of what follows

## Current Phases

| File | Phase | Status |
|------|-------|--------|
| [PHASE-01-POC.md](./PHASE-01-POC.md) | Proof of Concept interpreter | In progress |
| [PHASE-02-PRODUCTION.md](./PHASE-02-PRODUCTION.md) | Production implementation | Not started |

# /sprint-end

Close a sprint: create the review issue summarising outcomes, carry over incomplete issues, and update the board.

**Arguments:** `$ARGUMENTS` — sprint number, e.g. `3`

## Steps

1. **Fetch the sprint kickoff issue** to retrieve the sprint goal and planned issue list:
```bash
wsl gh issue list --repo Vladastos/Yoloscript \
  --label "sprint:kickoff" \
  --search "Sprint <N> Kickoff" \
  --json number,title,body
```

2. **Categorise planned issues** into completed and carried-over:
```bash
wsl gh issue list --repo Vladastos/Yoloscript \
  --label "status:in-progress" \
  --json number,title,state,milestone
```
Issues still open → carried over. Issues closed during the sprint → completed.

3. **Move carried-over issues back to backlog:**
```bash
wsl gh issue edit <N> --repo Vladastos/Yoloscript \
  --remove-label "status:in-progress" \
  --add-label "status:backlog"
```

4. **Ensure all tests pass on the sprint branch:**
```bash
cd tree-walk-interpreter && cargo test
```
If any tests fail, do not proceed — fix them first.

5. **Create the sprint review issue:**
```bash
wsl gh issue create \
  --repo Vladastos/Yoloscript \
  --title "Sprint <N> Review" \
  --label "sprint:review" \
  --body "## Sprint Goal
<goal from kickoff issue>

## Completed
$(for each closed issue: - [x] #N Title)

## Carried Over
$(for each open issue: - [ ] #N Title)

## Epic Progress
<!-- How does this sprint advance the milestone? -->

## Spec Notes
<!-- Did any spec ambiguities surface? Link to any RFC or docs/public/spec/ changes. -->

## Next Sprint Seeds
<!-- Issues or ideas for the next sprint -->"
```

6. **Open a pull request** from `sprint/<N>` → `main`:
```bash
gh pr create \
  --repo Vladastos/Yoloscript \
  --base main \
  --head sprint/<N> \
  --title "Sprint <N> — <theme>" \
  --body "$(cat <<'EOF'
Sprint review: #<review-issue-number>

Closes #<kickoff-issue-number>
EOF
)"
```
The PR body must link the sprint review issue. The PR diff is the authoritative record of all sprint changes.

7. **Close the kickoff issue** for this sprint:
```bash
gh issue close <kickoff-issue-number> --repo Vladastos/Yoloscript
```

8. **Report** the PR URL, the review issue URL, and a one-line summary of completed vs carried-over.

## Notes
- The review issue stays open for the user to fill in Epic Progress and Spec Notes.
- A sprint with 0 completed issues should still produce a review issue — record why.
- If spec ambiguities surfaced during the sprint, prompt the user to open a `/new-rfc`.
- The sprint branch is deleted after the PR is merged — not before.
- Remind the user: merge the PR on GitHub, then delete the `sprint/<N>` branch.

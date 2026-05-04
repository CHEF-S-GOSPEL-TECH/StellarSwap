#!/bin/bash
# Run this after: gh auth login
# Creates all Layer 1 (LP token) issues on GitHub

REPO="CHEF-S-GOSPEL-TECH/The-Gospel-Dex"
DIR="$(dirname "$0")/issues"

gh issue create --repo "$REPO" \
  --title "GD-LP-001: LP token — implement initialize" \
  --label "layer-1,good first issue" \
  --body-file "$DIR/issue-01.md"

gh issue create --repo "$REPO" \
  --title "GD-LP-002: LP token — implement SEP-41 read functions" \
  --label "layer-1,good first issue" \
  --body-file "$DIR/issue-02.md"

gh issue create --repo "$REPO" \
  --title "GD-LP-003: LP token — implement transfer and transfer_from" \
  --label "layer-1" \
  --body-file "$DIR/issue-03.md"

gh issue create --repo "$REPO" \
  --title "GD-LP-004: LP token — implement approve and allowance" \
  --label "layer-1,good first issue" \
  --body-file "$DIR/issue-04.md"

gh issue create --repo "$REPO" \
  --title "GD-LP-005: LP token — implement mint (admin-only)" \
  --label "layer-1" \
  --body-file "$DIR/issue-05.md"

gh issue create --repo "$REPO" \
  --title "GD-LP-006: LP token — implement burn (admin-only)" \
  --label "layer-1" \
  --body-file "$DIR/issue-06.md"

gh issue create --repo "$REPO" \
  --title "GD-LP-007: LP token — full test suite" \
  --label "layer-1,tests" \
  --body-file "$DIR/issue-07.md"

echo "Done. All 7 Layer 1 issues created."

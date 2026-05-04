#!/bin/bash
# Usage: ./scripts/create-issues.sh [layer]
#
# Examples:
#   ./scripts/create-issues.sh 1        # create Layer 1 issues (LP token)
#   ./scripts/create-issues.sh 2        # create Layer 2 issues (pair token helpers)
#   ./scripts/create-issues.sh all      # create all layers
#
# Prerequisites:
#   gh auth login   (run once to authenticate)

set -e

REPO="CHEF-S-GOSPEL-TECH/The-Gospel-Dex"
DIR="$(dirname "$0")/issues"
LAYER="${1:-1}"

create_issue() {
  local file="$1"
  local title="$2"
  local labels="$3"

  echo "Creating: $title"
  gh issue create \
    --repo "$REPO" \
    --title "$title" \
    --label "$labels" \
    --body-file "$file"
  sleep 2  # avoid GitHub rate limiting
}

create_labels() {
  echo "Ensuring labels exist..."
  gh label create "layer-1" --color "0075ca" --description "LP token — no dependencies"       --repo "$REPO" 2>/dev/null || true
  gh label create "layer-2" --color "e4e669" --description "Pair token helpers"               --repo "$REPO" 2>/dev/null || true
  gh label create "layer-3" --color "d93f0b" --description "Pair core logic"                  --repo "$REPO" 2>/dev/null || true
  gh label create "layer-4" --color "b60205" --description "Factory"                          --repo "$REPO" 2>/dev/null || true
  gh label create "layer-5" --color "1d76db" --description "SDK"                              --repo "$REPO" 2>/dev/null || true
  gh label create "layer-6" --color "5319e7" --description "Integration and deployment"       --repo "$REPO" 2>/dev/null || true
  gh label create "tests"   --color "cccccc" --description "Test coverage"                    --repo "$REPO" 2>/dev/null || true
}

layer1() {
  create_issue "$DIR/issue-01.md" "GD-LP-001: LP token — implement initialize"              "layer-1,good first issue"
  create_issue "$DIR/issue-02.md" "GD-LP-002: LP token — implement SEP-41 read functions"  "layer-1,good first issue"
  create_issue "$DIR/issue-03.md" "GD-LP-003: LP token — implement transfer and transfer_from" "layer-1"
  create_issue "$DIR/issue-04.md" "GD-LP-004: LP token — implement approve and allowance"  "layer-1,good first issue"
  create_issue "$DIR/issue-05.md" "GD-LP-005: LP token — implement mint (admin-only)"      "layer-1"
  create_issue "$DIR/issue-06.md" "GD-LP-006: LP token — implement burn (admin-only)"      "layer-1"
  create_issue "$DIR/issue-07.md" "GD-LP-007: LP token — full test suite"                  "layer-1,tests"
}

# Add layer2(), layer3() etc. here as you write the issue body files

create_labels

case "$LAYER" in
  1)   layer1 ;;
  all) layer1 ;;
  *)   echo "Unknown layer: $LAYER. Use 1 or all."; exit 1 ;;
esac

echo ""
echo "Done. View issues at: https://github.com/$REPO/issues"

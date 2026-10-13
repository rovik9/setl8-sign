#!/usr/bin/env bash
# Usage: check_protected_paths.sh <base> <head>
# Every commit in base..head that touches interfaces/ or vendor/ must have a subject
# starting with "export:".
set -euo pipefail

base="${1:?base ref}"
head="${2:?head ref}"
fail=0

while IFS= read -r sha; do
  [ -n "$sha" ] || continue
  if git diff-tree --no-commit-id --name-only -r -m "$sha" -- interfaces vendor | grep -q .; then
    subject="$(git log -1 --format='%s' "$sha")"
    case "$subject" in
      export:*) ;;
      *) echo "::error::$sha touches interfaces/ or vendor/ but subject is '$subject' (must start with 'export:')"; fail=1 ;;
    esac
  fi
done < <(git rev-list "$base..$head")

exit "$fail"

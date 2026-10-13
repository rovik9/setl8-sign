#!/usr/bin/env bash
# Usage: check_identity.sh <base> <head>
# Fails unless every commit in base..head is authored and committed by exactly
# "Rovik <therovikdev@gmail.com>" and no message contains Co-Authored-By or Claude-Session.
set -euo pipefail

base="${1:?base ref}"
head="${2:?head ref}"
want="Rovik <therovikdev@gmail.com>"
fail=0

while IFS= read -r sha; do
  [ -n "$sha" ] || continue
  author="$(git log -1 --format='%an <%ae>' "$sha")"
  committer="$(git log -1 --format='%cn <%ce>' "$sha")"
  if [ "$author" != "$want" ]; then
    echo "::error::$sha author is '$author', expected '$want'"; fail=1
  fi
  if [ "$committer" != "$want" ]; then
    echo "::error::$sha committer is '$committer', expected '$want'"; fail=1
  fi
  if git log -1 --format='%B' "$sha" | grep -qiE 'co-authored-by|claude-session'; then
    echo "::error::$sha message contains a forbidden trailer (Co-Authored-By / Claude-Session)"; fail=1
  fi
done < <(git rev-list "$base..$head")

exit "$fail"

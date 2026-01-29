#!/usr/bin/env bash
set -e
url="$(git remote get-url origin 2>/dev/null || true)"
if [ -z "$url" ]; then
  echo "UNKNOWN/UNKNOWN"
  exit 0
fi
# normalize
# ssh: git@github.com:owner/repo.git
if echo "$url" | grep -qE '^git@github\.com:'; then
  slug="$(echo "$url" | sed -E 's#^git@github\.com:##; s#\.git$##')"
  echo "$slug"
  exit 0
fi
# https: https://github.com/owner/repo.git
if echo "$url" | grep -qE '^https?://github\.com/'; then
  slug="$(echo "$url" | sed -E 's#^https?://github\.com/##; s#\.git$##')"
  echo "$slug"
  exit 0
fi
# fallback
echo "UNKNOWN/UNKNOWN"

#!/usr/bin/env bash
set -e
slug="$(tools/release/repo_slug.sh)"
# Use tag format v<version>
echo "https://github.com/${slug}/releases/download/v{{current_version}}/update.json"

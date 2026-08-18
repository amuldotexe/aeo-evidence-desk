#!/usr/bin/env bash
set -euo pipefail

readonly script_path="scripts/release_macos_notarized_dmg.sh"

test -f "$script_path"
grep -Fq -- '--keychain-profile' "$script_path"
grep -Fq -- 'notarytool submit' "$script_path"
grep -Fq -- 'stapler staple' "$script_path"
grep -Fq -- 'spctl --assess --type open' "$script_path"

if grep -Fq -- '--password' "$script_path"; then
  echo "release script must not hold a password" >&2
  exit 1
fi

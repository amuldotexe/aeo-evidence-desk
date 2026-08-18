#!/usr/bin/env bash
set -euo pipefail

readonly script_path="scripts/release_macos_notarized_dmg.sh"

test -f "$script_path"
grep -Fq -- '--keychain-profile' "$script_path"
grep -Fq -- 'notarytool submit' "$script_path"
grep -Fq -- 'stapler staple' "$script_path"
grep -Fq -- 'spctl --assess --type open' "$script_path"
grep -Fq -- 'hdiutil attach' "$script_path"
grep -Fq -- 'hdiutil detach' "$script_path"
grep -Fq -- 'readonly bundle_directory="$repository_root/target/$target/release/bundle"' "$script_path"
grep -Fq -- 'codesign_metadata="$(codesign -d --verbose=4 "$application_path" 2>&1)"' "$script_path"
grep -Fq -- 'LC_ALL=C shasum -a 256 "$artifact_path"' "$script_path"
grep -Fq -- 'if [[ -d "$mount_directory" ]]; then' "$script_path"

if grep -Fq -- '--password' "$script_path"; then
  echo "release script must not hold a password" >&2
  exit 1
fi

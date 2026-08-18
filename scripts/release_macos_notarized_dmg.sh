#!/usr/bin/env bash
set -euo pipefail

readonly profile_name="aeo-evidence-desk-notary"
readonly release_version="0.0.2"

if [[ "$#" -ne 1 ]]; then
  echo "Usage: $0 <aarch64-apple-darwin|x86_64-apple-darwin>" >&2
  exit 64
fi

readonly target="$1"

case "$target" in
  aarch64-apple-darwin)
    readonly architecture_label="arm64"
    ;;
  x86_64-apple-darwin)
    readonly architecture_label="x64"
    ;;
  *)
    echo "Unsupported macOS target: $target" >&2
    exit 64
    ;;
esac

if [[ -z "${APPLE_SIGNING_IDENTITY:-}" ]]; then
  echo "APPLE_SIGNING_IDENTITY must name a valid Developer ID Application identity." >&2
  exit 78
fi

readonly repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly tauri_directory="$repository_root/src-tauri"
readonly bundle_directory="$repository_root/target/$target/release/bundle"
readonly artifact_directory="$repository_root/artifacts/v$release_version"
readonly artifact_name="AEO-Fixture-Evidence-Desk-v$release_version-macos-$architecture_label.dmg"
readonly artifact_path="$artifact_directory/$artifact_name"

if ! security find-identity -v -p codesigning | grep -Fq -- "$APPLE_SIGNING_IDENTITY"; then
  echo "APPLE_SIGNING_IDENTITY is not available to codesign." >&2
  exit 78
fi

(
  cd "$tauri_directory"
  env LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 LC_CTYPE=en_US.UTF-8 \
    cargo tauri build --bundles dmg --target "$target"
)

shopt -s nullglob
dmg_candidates=("$bundle_directory"/dmg/*.dmg)
shopt -u nullglob

if [[ "${#dmg_candidates[@]}" -ne 1 ]]; then
  echo "Expected exactly one DMG for $target." >&2
  exit 70
fi

readonly dmg_path="${dmg_candidates[0]}"
readonly mount_directory="$(mktemp -d "${TMPDIR:-/tmp}/aeo-dmg-verification.XXXXXX")"
mounted_dmg=false

cleanup_mounted_dmg() {
  if [[ "$mounted_dmg" == true ]]; then
    hdiutil detach "$mount_directory" -quiet || true
  fi
  if [[ -d "$mount_directory" ]]; then
    rmdir "$mount_directory" || true
  fi
}

trap cleanup_mounted_dmg EXIT

hdiutil attach -nobrowse -readonly -mountpoint "$mount_directory" "$dmg_path" >/dev/null
mounted_dmg=true

shopt -s nullglob
app_candidates=("$mount_directory"/*.app)
shopt -u nullglob

if [[ "${#app_candidates[@]}" -ne 1 ]]; then
  echo "Expected exactly one application inside the DMG for $target." >&2
  exit 70
fi

readonly application_path="${app_candidates[0]}"

codesign --verify --deep --strict --verbose=4 "$application_path"
codesign_metadata="$(codesign -d --verbose=4 "$application_path" 2>&1)"
if ! grep -Fq -- "Runtime Version=" <<<"$codesign_metadata"; then
  echo "The built application does not have hardened runtime enabled." >&2
  exit 70
fi

hdiutil detach "$mount_directory" -quiet
mounted_dmg=false
rmdir "$mount_directory"

xcrun notarytool submit "$dmg_path" --keychain-profile "$profile_name" --wait
xcrun stapler staple "$dmg_path"
xcrun stapler validate "$dmg_path"
spctl --assess --type open --context context:primary-signature --verbose=4 "$dmg_path"

mkdir -p "$artifact_directory"
cp "$dmg_path" "$artifact_path"

xcrun stapler validate "$artifact_path"
spctl --assess --type open --context context:primary-signature --verbose=4 "$artifact_path"
LC_ALL=C shasum -a 256 "$artifact_path"

printf "Verified release asset: %s\n" "$artifact_path"

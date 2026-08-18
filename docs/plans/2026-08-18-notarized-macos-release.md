# Notarized macOS Release Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Produce and publish v0.0.2 as a direct-download macOS release that passes the Developer ID and notarization trust path on Apple Silicon and Intel Macs.

**Architecture:** The fixture app remains unchanged. A named profile in the local macOS Keychain holds Apple notarization credentials. A target-specific shell release script builds a signed Tauri DMG, submits and staples it, then verifies it before GitHub CLI publishes it.

**Tech Stack:** Tauri 2, Rust/Cargo, Vite/TypeScript, `codesign`, `xcrun notarytool`, `xcrun stapler`, `spctl`, and `gh`.

---

### Task 1: Establish the secure credential boundary

**Files:**

- Create (local, unversioned): `/Users/amuldotexe/Documents/store_notary_password_keychain.sh`
- Test: `xcrun notarytool history --keychain-profile aeo-evidence-desk-notary`

**Step 1: Verify the Keychain profile interface.** Run `xcrun notarytool store-credentials --help`; it must support profile-name, Apple-ID, team-ID, and app-specific-password inputs.

**Step 2: Create a no-secret helper.** The Documents script prompts locally using `read -s` and calls `xcrun notarytool store-credentials "aeo-evidence-desk-notary"`. It contains no account identifier, team identifier, signing identity, or password.

**Step 3: Store and validate the profile.** Run `xcrun notarytool history --keychain-profile "aeo-evidence-desk-notary"`; expected result is successful authentication with no credential committed to Git.

### Task 2: Add a testable notarized-release script

**Files:**

- Create: `scripts/release_macos_notarized_dmg.sh`
- Create: `tests/release_macos_notarized_dmg_contract.sh`
- Test: `bash tests/release_macos_notarized_dmg_contract.sh`

**Step 1: Write the failing test.** The test asserts that the release script exists, contains `--keychain-profile`, `notarytool submit`, `stapler staple`, and `spctl --assess --type open`, and does not contain `--password`.

**Step 2: Verify RED.** Run `bash tests/release_macos_notarized_dmg_contract.sh`; expected result is failure because the release script is absent.

**Step 3: Write minimal implementation.** The script accepts only `aarch64-apple-darwin` or `x86_64-apple-darwin`, requires `APPLE_SIGNING_IDENTITY`, runs `cargo tauri build --bundles dmg --target`, verifies the `.app`, submits the DMG through Keychain profile `aeo-evidence-desk-notary`, waits for acceptance, staples, validates, Gatekeeper-assesses, and copies to `artifacts/v0.0.2/` with a stable architecture-specific name.

**Step 4: Verify GREEN.** Run `bash tests/release_macos_notarized_dmg_contract.sh`; expected result is success.

**Step 5: Commit.** Stage only the new release script and contract test, then commit with message `build: add notarized macOS release script`.

### Task 3: Align all version metadata

**Files:**

- Modify: `Cargo.toml`
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/tauri.conf.json`
- Modify: `ui/package.json`
- Modify: `ui/package-lock.json` if it records the root package version

**Step 1: Write a failing version-consistency test.** Assert that the four version-bearing files contain `0.0.2`.

**Step 2: Verify RED.** Run the test while versions remain `0.0.1`; expected result is failure.

**Step 3: Write minimal implementation.** Bump versions to `0.0.2` only. Do not alter fixture data, the app identifier, capabilities, or CSP.

**Step 4: Verify GREEN.** Run the version test, `cargo test --workspace`, `npm --prefix ui run test`, and `npm --prefix ui run build`; expected result is success.

**Step 5: Commit.** Stage metadata and lockfiles, then commit with `release: prepare v0.0.2 metadata`.

### Task 4: Build and notarize both architectures

**Files:**

- Create (ignored): `artifacts/v0.0.2/*.dmg`

**Step 1: Confirm prerequisites.** Verify an available Developer ID identity, both macOS Rust targets, and `xcrun notarytool history --keychain-profile "aeo-evidence-desk-notary"`.

**Step 2: Build Apple Silicon.** Run the script with `aarch64-apple-darwin`; expected output is `AEO-Fixture-Evidence-Desk-v0.0.2-macos-arm64.dmg`.

**Step 3: Build Intel.** Run the script with `x86_64-apple-darwin`; expected output is `AEO-Fixture-Evidence-Desk-v0.0.2-macos-x64.dmg`.

**Step 4: Inspect artefacts.** Run `xcrun stapler validate`, `spctl --assess --type open`, and `shasum -a 256` for both; expected result is a notarized acceptance and recorded checksums.

### Task 5: Document and publish v0.0.2

**Files:**

- Modify: `README.md`
- Create: `Narrative/v002/README.md`

**Step 1: Add release guidance.** Describe the Apple Silicon vs Intel choice, fixture-data limitation, and notarized release path. Do not claim success before artifact validation.

**Step 2: Commit documentation.** Stage README and Narrative v002, then commit with `docs: describe v0.0.2 macOS release`.

**Step 3: Publish through GitHub CLI.** Run `gh release create v0.0.2` with the two validated DMGs and their checksums.

**Step 4: Verify publication.** Run `gh release view v0.0.2 --json url,assets,isDraft,isPrerelease`; expected result is non-draft/non-prerelease with both DMGs.

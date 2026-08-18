# Notarized macOS v0.0.2 Release Design

## Decision

Ship two direct-download DMG assets for v0.0.2: one for Apple Silicon and one
for Intel Macs. Each DMG is signed with the local Developer ID Application
identity, notarized by Apple, stapled, and assessed before publication.

## Considered Approaches

1. Ask evaluators to remove quarantine or recompile the source. This is quick
   but transfers the distribution failure to the evaluator and is not suitable
   for an external release.
2. Upload an unsigned ZIP alongside the existing artefact. This does not solve
   Gatekeeper trust and creates a confusing installation path.
3. Release signed, notarized DMGs for both supported macOS architectures. This
   is the chosen approach because it uses Apple's normal direct-distribution
   trust path and keeps installation to drag-and-drop.

## Architecture

```text
local Developer ID identity
             |
             v
Tauri build --target <architecture> --bundles dmg
             |
             v
signed application inside DMG
             |
             v
Apple notary service (Keychain-backed credentials)
             |
             v
stapled DMG -> local signature/Gatekeeper checks -> GitHub Release assets
```

The existing fixture-only application and its Tauri commands are unchanged.
The release boundary is packaging only: version metadata, a repeatable local
release script, and release notes that state which DMG a person should choose.

## Credential Boundary

The app-specific password is stored only in a named macOS Keychain profile.
It must never appear in a repository file, `.env` file, GitHub issue, release
notes, or command output. The release script refers only to the profile name.

## Acceptance Criteria

- `REQ-TAURI-010.1`: WHEN v0.0.2 is built for `aarch64-apple-darwin` and
  `x86_64-apple-darwin`, THEN the release procedure SHALL produce one DMG for
  each architecture.
- `REQ-TAURI-011.1`: WHEN either DMG is released, THEN its enclosed `.app`
  SHALL be Developer-ID signed with hardened runtime and pass `codesign` and
  `spctl` assessment.
- `REQ-TAURI-012.1`: WHEN either DMG has been submitted to Apple's notary
  service, THEN the procedure SHALL wait for acceptance, staple the ticket,
  and validate that staple before publication.
- `REQ-TAURI-013.1`: WHEN the release is documented, THEN it SHALL identify
  the Apple Silicon and Intel asset names and state that the app uses fixture
  data.

## Failure Handling

- A missing signing identity or Keychain profile fails before a build or upload.
- A notarization rejection stops publication; no unsigned fallback is released.
- A failing signature, staple, or Gatekeeper assessment stops publication.
- Release assets are uploaded only after both architecture-specific DMGs pass
  their checks.

## Sources

- Apple: <https://developer.apple.com/developer-id/>
- Apple: <https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution>
- Tauri: <https://v2.tauri.app/distribute/sign/macos/>

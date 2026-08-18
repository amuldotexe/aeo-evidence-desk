# AEO Fixture Evidence Desk — Narrative v002

**Build documented:** `v0.0.2` (macOS direct-distribution release)

v0.0.2 does not change the product claim, fixture set, or decision workflow.
It changes the delivery boundary: people evaluating the demonstrator can use a
normal macOS disk-image install rather than recompiling source or working
around Gatekeeper.

## What is unchanged

- The desktop app remains an offline fixture demonstrator.
- It shows a fixed beauty-and-hair-care evidence scope, not live provider data.
- Its purpose is still to make one content-investigation decision traceable to
  the underlying labelled fixtures.

The v0.0.1 screenshots therefore remain accurate; no new product screen is
claimed for this release.

## What the release adds

| Asset | Choose it for | Install path |
| --- | --- | --- |
| `AEO-Fixture-Evidence-Desk-v0.0.2-macos-arm64.dmg` | Apple Silicon Macs (`M1` and newer) | Open DMG → drag the app to Applications → open normally. |
| `AEO-Fixture-Evidence-Desk-v0.0.2-macos-x64.dmg` | Intel Macs | Open DMG → drag the app to Applications → open normally. |

Each asset is released only after this verification sequence succeeds:

```text
Developer ID sign -> inspect app inside final DMG
                  -> Apple notarization accepted
                  -> staple ticket to DMG
                  -> Gatekeeper assessment
```

That boundary matters: it demonstrates care for the evaluator's installation
experience without overstating the fixture app as a live AEO measurement
product.

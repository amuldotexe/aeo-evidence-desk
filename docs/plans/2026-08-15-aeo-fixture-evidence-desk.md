# AEO Fixture Evidence Desk — implementation plan

> **Purpose:** Deliver the complete offline decision loop specified in `D03`: embedded authored answers → deterministic mention score → lowest-presence intent → inspectable evidence → one labelled action hypothesis.

## Boundary decisions

- Use a small Cargo workspace: a framework-light Rust core at the repository root and a `src-tauri` app crate.
- Use a Vite + TypeScript DOM interface; no frontend state, routing, database, or HTTP client.
- Compile the authored fixture JSON into the core with `include_str!`; all calculation and validation stay in Rust.
- Treat the desktop application's offline promise as **no external/provider HTTP or privileged I/O**. The local Vite development server is only a development host, not a product data source.
- Borrow only the Tauri workspace and typed-command conventions from the local assignment apps; create all AEO domain code and fixture data here.

## Test-first delivery sequence

1. Add the Cargo/Vite/Tauri manifests, fixture pack, expected-summary oracle, and a dependency-policy test.
2. Write the first Rust contract test for embedded fixture loading and run it red.
3. Implement fixture types, serializable errors, parsing, cardinality/referential validation, then return to green.
4. Add red tests for case-insensitive whole-word matching, matrix/cohort aggregation, deterministic priority selection, and evidence filtering.
5. Implement the pure calculation modules and use the expected-summary fixture as the exact regression oracle.
6. Add command contract tests, then wire two thin Tauri commands over the core.
7. Add mocked TypeScript DOM tests for dashboard, error, and intent-selection flows; implement only the UI needed to make them pass.
8. Run static scope checks, Rust checks, frontend checks, a production desktop build, and a credential-free development launch.

## Module contracts

| Module | Responsibility | Verified by |
| --- | --- | --- |
| `fixture_data` | parse embedded data and validate cardinality/links | TEST-RUST-UNIT-001, 002 |
| `presence_metrics` | whole-word matching and deterministic aggregation | TEST-RUST-UNIT-003, 004 |
| `priority_action` | lowest-rate selection with stable tie-break | TEST-RUST-UNIT-005, 006 |
| `evidence_lookup` | filter and decorate the six selected-intent observations | TEST-RUST-INTEG-007, 008 |
| `src-tauri/commands` | serialize core results/errors as two narrow command surfaces | TEST-RUST-INTEG-013 |
| `ui` | render the decision loop from mocked typed command results | TEST-UI-009, 010, 011 |
| `tests/policy_scope` | reject forbidden network/provider/persistence dependencies | TEST-POLICY-012 |

## Verification evidence

The final handoff will record the exact output of:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --all-targets --all-features
npm --prefix ui test -- --run
npm --prefix ui run build
npm --prefix ui run tauri:build
```

The development launch is a manual smoke check only; it must not require a key, network-backed fixture, or privileged Tauri plugin.

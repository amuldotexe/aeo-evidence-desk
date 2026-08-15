# AEO Fixture Evidence Desk — minimal PRD and executable specification

**Status:** proposed implementation packet
**Product mode:** fully offline, fixture-only Tauri desktop app
**Primary user:** L’Oréal Paris hair-care content manager
**Single decision supported:** which one intent gap should the manager investigate first?

## Product requirement = identify one evidence-backed content priority

### Problem

The assignment asks how a brand could understand customer intent, measure presence in generative-AI answers, and suggest an improvement. A two-day prototype cannot credibly measure live, changing AI products or prove that a content change caused later visibility uplift.

The smallest honest product is therefore an **offline evidence desk**. It demonstrates the complete decision loop using a deliberately small, authored fixture pack:

```text
intent cohort
    -> frozen provider-labelled answers
    -> deterministic presence calculation
    -> evidence inspection
    -> one prioritised action hypothesis
```

### Product thesis

The app helps a content manager identify the weakest fixture-backed L’Oréal hair-care intent, inspect the exact answer evidence behind that conclusion, and understand the proposed next content investigation.

### Job to be done

> When I have several possible content gaps to investigate, help me choose one based on transparent observed-answer evidence rather than a generic “AI visibility” score.

### Success criteria

A reviewer can, without credentials or network access:

1. see coverage across three provider labels and three hair-care intent cohorts;
2. identify the lowest-presence cohort;
3. open the exact six fixture answers behind that cohort;
4. see how each answer was scored; and
5. understand one clearly labelled, non-causal action hypothesis.

## Scope = one offline L’Oréal Paris hair-care decision loop

### In scope

- One market label: `en-US`.
- One vertical: Beauty and cosmetics.
- One micro-category: L’Oréal Paris shampoo for colour-treated, dry, and frizz-prone hair.
- Three intent cohorts: discovery, comparison, and concern-solving.
- Six authored prompt cases: two per cohort.
- Three provider labels: ChatGPT, Claude, and Gemini.
- Eighteen authored response fixtures: six prompts multiplied by three provider labels.
- A deterministic mention metric, intent-by-provider matrix, evidence panel, and one ranked action hypothesis.
- Embedded fixture loading, Rust unit tests, Tauri command tests, TypeScript UI tests, and a clean local run path.

### Explicit non-goals

- No API keys, live model calls, scraping, web search, or scheduled scans.
- No claim that a fixture response represents the current consumer product experience.
- No database, authentication, multi-tenancy, uploads, exports, notifications, or cloud deployment.
- No prompt generation at runtime.
- No natural-language recommendation generation at runtime.
- No claim that publishing an action will change model recommendations.

### Fixture-only truth label

Every aggregate metric, answer card, and action panel SHALL display `Fixture analysis — authored demonstration data; not a live provider measurement.` The application must make this boundary obvious enough that a screenshot cannot be mistaken for a current market result.

## Fixture pack = small, embedded, and auditable

The fixture pack is authored by us and compiled into the Rust binary. It is not read from disk at runtime and does not require a database or filesystem permission.

| Entity | Exact minimum | Required fields |
| --- | ---: | --- |
| Vertical | 1 | `id`, `label` |
| Intent cohort | 3 | `id`, `label`, `intent_type`, `action_template` |
| Prompt case | 6 | `id`, `intent_id`, `prompt_text`, `market` |
| Provider label | 3 | `id`, `label` |
| Brand alias | 4 | `id`, `alias_text` |
| Response observation | 18 | `id`, `prompt_id`, `provider_id`, `answer_text`, `source_type` |
| Expected score | 18 | `observation_id`, `brand_mentioned` |

### Authored fixture content rules

- All response observations SHALL have `source_type: "authored_demo_fixture"`.
- `answer_text` SHALL be fictional demonstration text, not represented as a live API response.
- Every prompt SHALL have exactly one observation for each provider label.
- Every observation SHALL be linked to exactly one expected score.
- Every prompt SHALL target exactly one intent cohort and the fixed `en-US` market.
- The brand aliases SHALL be simple, unambiguous L’Oréal Paris/product labels chosen only for deterministic fixture matching.

### Deterministic measurement rules

For each observation:

```text
brand_mentioned = answer contains a configured alias
                  using case-insensitive whole-word matching
```

For each intent/provider cell:

```text
presence_rate = mentioned observations / all observations
                for that intent and provider
```

For each intent cohort:

```text
cohort_presence_rate = mentioned observations / all observations
                        for that intent across all providers
```

The priority cohort is the lowest `cohort_presence_rate`. Ties SHALL be resolved by ascending `intent_id`, so the result is reproducible. The suggested action SHALL use that cohort’s authored `action_template` and include the evidence denominator, for example: `Investigate comparison content because L’Oréal was mentioned in 1 of 6 fixture observations.`

## Reviewer experience = dashboard, evidence, action

### Screen 1: Fixture dashboard

The app opens directly into the dashboard. It shows:

- a fixture-only warning banner;
- the fixed scope: `Beauty > Hair care > L’Oréal Paris shampoo > en-US`;
- an intent-by-provider presence matrix with `mentioned / total` values;
- the priority intent cohort; and
- a short explanation of the deterministic scoring rule.

### Screen 2: Intent evidence

Selecting an intent cohort shows:

- the cohort label and its calculated rate;
- the six observations for that cohort;
- each prompt, provider label, fixture answer, and mention/not-mentioned result;
- the matching alias when the result is positive; and
- the source-type label on every answer card.

### Screen 3: Priority action

The dashboard and evidence screen both show the same action panel:

```text
Priority: <lowest-presence cohort>
Observed fixture evidence: <mentioned> of <total>
Action hypothesis: <authored action template>
Boundary: This is a content-investigation hypothesis, not proof of causal lift.
```

## Implementation base = keep the Tauri shell, replace the domain

Use the local Confido desktop assignment as a **scaffold reference**, not as a domain dependency:

- retain the Tauri 2 + Cargo workspace shape, Vite/TypeScript setup, typed invoke style, build scripts, basic window configuration, and test conventions;
- keep Tauri commands thin, as in the related assignment apps;
- replace healthcare prompt parsing, credential handling, remote OpenAI calls, persistence, patching, and domain UI with the fixture-only AEO core;
- do not copy response data, healthcare-specific labels, or remote-provider behavior.

## Tauri work mode

- **Spec Mode:** success is defined by a transparent fixture decision loop.
- **App Architecture Mode:** the frontend only renders typed results; Rust owns fixture parsing, scoring, priority selection, and errors.
- **Desktop Security Mode:** a single `main` window receives only `core:default`; no filesystem, shell, network, store, updater, or sidecar permission is needed.

## Executable requirements

### REQ-AEO-001.0: Load the embedded fixture experiment

**WHEN** the desktop application starts
**THEN** the Rust core SHALL load exactly one embedded fixture pack without network, filesystem, database, or user credential access
**AND** SHALL validate that it contains exactly three intent cohorts, six prompts, three providers, and eighteen observations
**SHALL** return a serializable `InvalidFixture` error if parsing, counts, or referential integrity fail.

### REQ-AEO-002.0: Calculate transparent presence rates

**WHEN** the core receives a valid fixture pack
**THEN** it SHALL calculate mention outcomes with case-insensitive whole-word alias matching
**AND** SHALL return mentioned and total counts for every intent/provider cell and intent cohort
**SHALL** preserve the fixture’s provider and intent identifiers in the returned result.

### REQ-AEO-003.0: Select one deterministic priority cohort

**WHEN** cohort presence rates have been calculated
**THEN** the core SHALL select the cohort with the lowest calculated cohort presence rate
**AND** SHALL attach its authored action template and the supporting mentioned/total denominator
**SHALL** select the ascending `intent_id` when two or more cohorts have the same rate.

### REQ-AEO-004.0: Return inspectable intent evidence

**WHEN** the caller requests evidence for a valid intent identifier
**THEN** the core SHALL return only the prompt and provider observations belonging to that cohort
**AND** SHALL include prompt text, provider label, answer text, fixture source type, calculated mention result, and matched alias when present
**SHALL** return a serializable `IntentNotFound` error for an unknown identifier.

### REQ-TAURI-001.0: Render the fixture dashboard

**WHEN** the main window becomes ready
**THEN** the frontend SHALL invoke `get_fixture_dashboard_data` once and render the fixture-only warning, scope label, presence matrix, scoring rule, and priority action
**AND** SHALL display counts as `mentioned / total`, not a percentage without a denominator
**SHALL** render a readable error state if the command returns a serializable error.

### REQ-TAURI-002.0: Reveal evidence from a selected cohort

**WHEN** the reviewer selects an intent cohort from the matrix
**THEN** the frontend SHALL invoke `get_intent_evidence_data` with that cohort’s stable identifier
**AND** SHALL display every returned observation without hiding negative results
**SHALL** retain the fixture-only warning in the evidence view.

### REQ-TAURI-003.0: Keep the desktop app offline and least-privilege

**WHEN** the fixture-only prototype runs
**THEN** it SHALL make no HTTP request, read no user-selected file, write no file, and request no API key
**AND** its `main` window capability SHALL contain only the minimum core permission set
**SHALL** omit provider-client, database, filesystem-plugin, shell-plugin, and updater-plugin dependencies.

### REQ-TAURI-004.0: Preserve typed desktop boundaries

**WHEN** a frontend invoke call fails because the fixture is invalid or an intent is unknown
**THEN** the command layer SHALL return `Result<T, AppError>` with a serializable error code and user-readable message
**AND** SHALL not panic, expose an internal Rust error, or use `unwrap()` or `expect()` in a user-reachable command path
**SHALL** centralize the two command registrations in the Tauri application entrypoint.

## Tauri design = thin commands over a framework-light Rust core

| Area | Minimal design |
| --- | --- |
| Frontend | Vite + TypeScript; one typed invoke wrapper per command; dashboard and evidence views; no state library required. |
| Tauri commands | `get_fixture_dashboard_data` and `get_intent_evidence_data`; validate request shape and delegate. |
| Rust core | Embedded fixture loading, schema validation, deterministic matching, aggregation, priority selection, evidence filtering. |
| Managed state | None. The immutable embedded fixture can be loaded per command or initialized as an immutable validated value; no mutable global state is needed. |
| Persistence | None. No SQLite, plugin-store, local file, or cloud service. |
| Capabilities | One `main` window with `core:default` only. |
| Lifecycle | One ordinary window; no tray, updater, sidecar, deep link, or single-instance feature. |
| CSP | `default-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:` with no external `connect-src` source. |

### Planned module shape

```text
aeo-fixture-desk/
├── fixtures/
│   ├── beauty_loreal_fixture.json
│   └── expected_fixture_summary.json
├── src/
│   ├── fixture_data.rs
│   ├── presence_metrics.rs
│   ├── priority_action.rs
│   ├── evidence_lookup.rs
│   └── app_error.rs
├── src-tauri/
│   └── src/
│       ├── commands.rs
│       └── lib.rs
├── ui/src/
│   ├── api.ts
│   ├── app.ts
│   ├── types.ts
│   └── app.test.ts
└── tests/
    └── fixture_core_contracts.rs
```

### Four-word generated names

New non-public functions and commands SHALL follow `verb_constraint_target_qualifier` where feasible:

- `load_embedded_fixture_bundle`
- `validate_fixture_bundle_shape`
- `calculate_intent_presence_rates`
- `select_priority_intent_gap`
- `find_intent_evidence_rows`
- `get_fixture_dashboard_data`
- `get_intent_evidence_data`

## Test matrix

| req_id | test_id | type | assertion | target |
| --- | --- | --- | --- | --- |
| REQ-AEO-001.0 | TEST-RUST-UNIT-001 | Rust unit | embedded fixture parses and has the exact required entity counts | fixture integrity |
| REQ-AEO-001.0 | TEST-RUST-UNIT-002 | Rust unit | malformed JSON, duplicate IDs, missing observation links, and incorrect counts return `InvalidFixture` | invalid input |
| REQ-AEO-002.0 | TEST-RUST-UNIT-003 | Rust unit | known fixture answers produce the committed expected counts and mention outcomes | metric correctness |
| REQ-AEO-002.0 | TEST-RUST-UNIT-004 | Rust unit | alias matching is case-insensitive and rejects substring-only matches | extraction boundary |
| REQ-AEO-003.0 | TEST-RUST-UNIT-005 | Rust unit | lowest rate selects the expected priority cohort and evidence denominator | priority correctness |
| REQ-AEO-003.0 | TEST-RUST-UNIT-006 | Rust unit | tied rates select the ascending intent identifier | deterministic tie-break |
| REQ-AEO-004.0 | TEST-RUST-INTEG-007 | Rust integration | selected cohort returns only its six evidence observations with positive and negative outcomes | evidence traceability |
| REQ-AEO-004.0 | TEST-RUST-INTEG-008 | Rust integration | unknown cohort returns serializable `IntentNotFound` | error boundary |
| REQ-TAURI-001.0 | TEST-UI-009 | TypeScript unit | initial dashboard renders warning, matrix counts, and action from mocked typed response | UI contract |
| REQ-TAURI-001.0 | TEST-UI-010 | TypeScript unit | command error renders a readable error state | UI failure path |
| REQ-TAURI-002.0 | TEST-UI-011 | TypeScript unit | selecting a matrix cohort requests its stable ID and renders all six observations | evidence interaction |
| REQ-TAURI-003.0 | TEST-POLICY-012 | static policy check | manifests contain no provider SDK, HTTP client, database, filesystem, shell, store, or updater dependency | offline scope |
| REQ-TAURI-004.0 | TEST-RUST-INTEG-013 | Rust integration | both command functions preserve the serializable `AppError` shape | desktop boundary |

## TDD plan

### 1. STUB

1. Create the fixture bundle and expected-summary fixture before implementation.
2. Write failing Rust tests for bundle validation, mention matching, rates, priority selection, and evidence lookup.
3. Define TypeScript request/response types and mocked invoke tests for dashboard, evidence, and errors.
4. Add the dependency-policy test before adding dependencies.

### 2. RED

1. Run the Rust test target and confirm failures report missing module/function behavior.
2. Run the frontend test target and confirm failures report missing typed invoke wrappers or rendered elements.
3. Confirm the policy test fails if a forbidden dependency is deliberately introduced, then remove it before proceeding.

### 3. GREEN

1. Implement fixture parsing and validation.
2. Implement deterministic alias matching and aggregation.
3. Implement priority selection and evidence lookup.
4. Add two thin Tauri commands and typed TypeScript wrappers.
5. Render the dashboard first, then the selected-intent evidence view.

### 4. REFACTOR

1. Remove duplicate calculations between dashboard and evidence pathways.
2. Keep matching and aggregation in Rust, not template strings or TypeScript.
3. Keep the fixture warning in one reusable UI renderer.
4. Confirm names are descriptive and four words where feasible.

### 5. VERIFY

1. Run every test in the matrix.
2. Build the Rust workspace and frontend.
3. Launch the desktop app without network credentials.
4. Manually follow the three-screen reviewer experience.
5. Confirm every `REQ-*` row still has a linked passing test.

## Pre-commit quality gates

- [ ] The fixture bundle and expected-summary fixture are committed and contain no external-response claim.
- [ ] Every requirement ID above has at least one linked test ID.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes.
- [ ] `cargo test --all-targets --all-features` passes.
- [ ] `cargo build --all-targets --all-features` passes.
- [ ] The repository’s frontend test command passes.
- [ ] The repository’s frontend build command passes.
- [ ] A Tauri development launch succeeds without credentials or network access.
- [ ] No `TODO`, `STUB`, or `FIXME` appears in production code.
- [ ] No user-reachable command contains `unwrap()` or `expect()`.
- [ ] No added dependency enables provider access, HTTP, database access, filesystem access, shell access, storage, or updating.
- [ ] The UI visibly labels all content as authored fixture analysis.

## Open questions and fixed assumptions

| Item | Decision for this packet | Effect if changed later |
| --- | --- | --- |
| Live providers | Excluded. All answers are authored fixtures. | Add provider adapters, credentials, run persistence, time/locale metadata, cost controls, and a different validation protocol. |
| Market | Fixed to `en-US`. | Add locale-aware taxonomy, client catalog, prompt packs, and sampling rules. |
| Product category | Shampoo only. | Add product/entity taxonomy and client-overlay expansion. |
| Packaging | A local `tauri dev` demo is required; signed distribution is not. | Add platform packaging, signing, and release verification. |
| Intervention proof | Not attempted. Action is an investigation hypothesis only. | Add intervention registry, publication verification, time delay, and a later controlled re-measurement plan. |

## Definition of done

The prototype is done when a clean local checkout can launch one offline desktop window, a reviewer can trace the lowest fixture presence rate through the six underlying answer cards, and the app produces one explicitly non-causal L’Oréal content-investigation hypothesis with no keys, network requests, or hidden data source.

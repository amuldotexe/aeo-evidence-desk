# AEO Evidence Desk

> A small, offline desktop reference app for making one evidence-backed
> answer-engine-optimization decision.

![AEO Evidence Desk dashboard](Narrative/v001/01-dashboard.jpg)

## The problem is not “build a dashboard”

People increasingly ask answer engines questions such as, “What shampoo is
best for colour-treated hair?” A brand team needs to know where it appears in
those answers, where competitors appear instead, and what to investigate next.

The useful customer question is therefore:

> **When we have several possible content gaps, which one should we investigate
> first—and what answer evidence supports that choice?**

AEO Evidence Desk is a deliberately narrow answer. It helps a content manager
trace one priority from a presence matrix to every underlying answer, then to a
clearly labelled next-step hypothesis.

This README uses a decision-first product lens: a metric is valuable only if it
helps a specific person make a better decision and can be explained when they
ask, “Why should I trust this?”

## What you can see in the app

The shipped `v0.0.1` demonstrator focuses on one fixed scope:

```text
Beauty > Hair care > L'Oréal Paris shampoo > en-US
```

It contains:

- three customer-intent cohorts: discovery, comparison, and concern-solving;
- six authored shopper prompts, two per cohort;
- three provider labels: ChatGPT, Claude, and Gemini;
- eighteen authored answer fixtures; and
- a deterministic, inspectable mention calculation.

The dashboard shows `mentioned / total` for each intent and provider. It then
identifies the lowest-presence cohort and supplies one action hypothesis. A
reviewer can select a cohort and inspect every answer that produced the number,
including negative results.

![Scope and priority detail](Narrative/v001/02-priority-detail.jpg)

## How the decision loop works

```text
authored answer fixtures
    -> deterministic alias matching
    -> intent-by-provider presence counts
    -> lowest-presence intent cohort
    -> evidence drill-down
    -> one non-causal action hypothesis
```

For this reference build, a positive observation means that an answer contains
a configured L'Oréal alias using case-insensitive whole-word matching. The app
does not hide this rule behind a proprietary score or an LLM judgement.

## An important honesty boundary

**This is not live ChatGPT, Claude, or Gemini data.** Every answer is authored
fixture data embedded in the application. The interface labels that fact on the
dashboard and evidence view.

That constraint is intentional. It lets a reviewer rerun the app and obtain
the same inputs, counts, priority, and evidence trail. It does **not** claim to
measure current consumer-facing AI behaviour or prove that publishing content
will change a platform's recommendation.

## Why this is useful as an OSS reference

Many AEO/GEO tools begin with a top-line visibility score. That can be a useful
navigation aid, but it is not a decision by itself. A score can hide the prompt
set, provider, location, date, answer, citation, and classification rule that
produced it.

This project instead starts with the smaller, harder promise:

1. show the denominator behind an aggregate;
2. preserve negative evidence;
3. make the scoring rule visible;
4. let a reviewer inspect the source answers; and
5. label recommendations as hypotheses until they are tested.

The result is a compact reference implementation for teams exploring the
evidence and product-design layer of answer-engine measurement.

## Architecture: a small proof today, an engine tomorrow

The shipped app implements the final review loop with frozen inputs. The
production design keeps that review loop, but adds the data and evaluation
layers needed to make live measurements trustworthy.

```text
IMPLEMENTED IN v0.0.1: offline fixture desk

+--------------------+
| Desktop reviewer    |
| dashboard + drill   |
+--------------------+
          |
          v
+--------------------+
| Typed Tauri         |
| commands            |
+--------------------+
          |
          v
+--------------------+
| Rust fixture core   |
| validate, score,    |
| prioritise, explain |
+--------------------+
          |
          v
+--------------------+
| Embedded fixtures   |
| prompts + answers   |
+--------------------+

PROPOSED PRODUCTION ENGINE: not implemented in v0.0.1

+------------------------------------------------+
| Client facts, owned pages, approved claims     |
+------------------------------------------------+
                        |
                        v
+------------------------------------------------+
| Versioned vertical taxonomy + client overlay   |
+------------------------------------------------+
                        |
                        v
+------------------------------------------------+
| Prompt compiler + provider adapters            |
+------------------------------------------------+
                        |
                        v
+------------------------------------------------+
| Evidence ledger: runs, answers, citations,     |
| timestamps, configuration, and hashes          |
+------------------------------------------------+
                        |
                        v
+------------------------------------------------+
| Evaluation: rules, LLM judges, human gold set, |
| uncertainty, and metric checks                 |
+------------------------------------------------+
                        |
                        v
+------------------------------------------------+
| Decision desk: drill-down, approved action,    |
| and a later measurement rerun                  |
+------------------------------------------------+
```

The frontend renders typed dashboard and evidence data. The Rust core owns
fixture parsing, validation, matching, aggregation, priority selection, and
serializable errors. The prototype needs no API key, network access, database,
filesystem access, or mutable user state.

## Run locally

### Prerequisites

- Rust `1.80` or newer;
- Node.js and npm; and
- the platform prerequisites for [Tauri 2](https://v2.tauri.app/start/prerequisites/).

### Launch the desktop app

```bash
npm --prefix ui install
npm --prefix ui run tauri:dev
```

### Run checks

```bash
cargo test --all-targets --all-features
npm --prefix ui test
npm --prefix ui run build
```

## What this version deliberately does not do

- call live answer-engine APIs or automate consumer AI interfaces;
- scrape or claim access to private AI conversations;
- generate prompts or recommendations at runtime;
- use LLM-as-a-judge for simple facts such as explicit brand mentions;
- persist users, projects, run histories, or raw external payloads; or
- claim causal visibility or revenue lift.

Those are production questions, not details to disguise in a two-day prototype.

## The production path

A real AEO product would extend the same evidence contract:

```text
versioned vertical taxonomy + client/product overlay
    -> reviewed prompt cohort
    -> repeated provider runs with locale and model metadata
    -> immutable answers and citations
    -> rules first; LLM judges only for ambiguous classifications
    -> human calibration and confidence checks
    -> evidence-linked recommendation
    -> later measurement rerun
```

An LLM judge is useful when the question is fuzzy—for example, whether a brand
was genuinely recommended or merely mentioned. It should be versioned,
calibrated against human labels, and never treated as an unquestionable source
of truth. Explicit facts should remain deterministic whenever possible.

## Repository guide

- [FAQ](FAQ.md) — plain-English explanation of the client, decision, fixtures,
  LLM judges, limits, and production path.
- [Original assignment brief](docs/D01-problem-statement.md) — the broader
  vertical, client, measurement, validation, and architecture challenge.
- [Research and production design](docs/D02-initial-analysis-20260812.md) —
  taxonomy, provider adapters, scalable schema, validation, and limitations.
- [Executable prototype specification](docs/D03-minimal-fixture-prd-specs.md)
  — exact fixture contract, requirements, and test matrix.
- [Product narrative and screenshots](Narrative/v001/README.md) — the shipped
  screen-by-screen review flow.

## Status

This repository is an educational reference implementation and assignment
artifact. It is a working desktop application, but it is not presented as a
production AEO platform.

The goal is simple: **make one answer-engine content decision more traceable,
not make an unearned promise about changing AI recommendations.**

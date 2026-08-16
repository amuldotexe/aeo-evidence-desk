# AEO Evidence Desk

> A small, offline desktop reference app for making one evidence-backed
> answer-engine-optimization decision.

![AEO Evidence Desk dashboard](Narrative/v001/01-dashboard.jpg)

## The problem is not “build a dashboard”

What people typically ask a chat app:

“What shampoo is best for colour-treated hair?” 

A brand team needs to know

1. where Brand appears in those answers, as in at least you should be visible or in the radar of the chat apps. 
2. how are you being placed as compared to your competitor
3. and know what to investigate next, and the decision will be how to improve your content game


AEO Evidence Desk helps you narrow down on that answer to trace what should be the next step in your content journey to become positively visible to your consumers. 


## What you can see in the app

The shipped `v0.0.1` demonstrator focuses on one fixed scope:

```text
Beauty > Hair care > L'Oréal Paris shampoo > en-US
```

It contains:

- three customer-intent cohorts: discovery, comparison, and concern-solving;
- limited labelled prompts and responses to help zero in on the content-visibility problem;


![Scope and priority detail](Narrative/v001/02-priority-detail.jpg)



## Key Caveat

**This is not live ChatGPT, Claude, or Gemini data.** 

This is just sample data

## Architecture: figure out what is useful first, we can engineer it for efficiency and scalability later

A minimalist approach - Tauri app starting from the side of "what a decision maker wants to see" and deliberately light on the production architecture - which will depend heavily on the way we source that data in context of the decision maker

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

```


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


## The production path

The key hypothesis is that, if a decision-maker finds this kind of evidence useful, we can work backwards towards a trustworthy prompt-evidence data source: a licensed/publicly available provider, or a repeatable prompt-sampling system using the relevant APIs where permitted.

The important question is not merely whether this can be engineered. It is whether the result gives a content decision-maker enough confidence to investigate the right issue and improve content in the right direction.

Possible production backlog:

- **Evidence provenance and run history:** retain the prompt, provider, model/version where available, collection time, source, and raw response snapshot for every observation.
- **Context labels:** add an explicit, calibrated label such as `recommended`, `neutral`, `cautioned`, or `not mentioned`. In an early release these would be reviewer-authored labels; an LLM-as-judge workflow would need human calibration and disagreement checks before it could be trusted.
- **Named competitor comparison:** record the competitors mentioned in each answer and their context, so the decision-maker can compare the brand's inclusion with named alternatives for the same prompt and intent cohort.
- **Repeatable sampling:** rerun a stable prompt set across providers and dates, then show change over time rather than treating a single answer as a durable market signal.
- **Decision validation:** connect each proposed action to a falsifiable content-investigation hypothesis and, where practical, test it through a controlled content change. Observed co-occurrence is not proof of causation.

The feasibility, cost, provider terms, and privacy constraints of collection are product decisions to resolve before this becomes a live measurement system.

## Additional documentation
- [Executable prototype specification](docs/D03-minimal-fixture-prd-specs.md)
  — exact fixture contract, requirements, and test matrix.

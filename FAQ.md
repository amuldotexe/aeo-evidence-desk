# AEO Evidence Desk FAQ

## What is this project?

**AEO Evidence Desk is a small, transparent prototype for making one better
content decision.** It helps a brand team see where the brand is absent from
AI answers, inspect the underlying evidence, and choose one next investigation.

It is not a magic machine that makes ChatGPT recommend a brand.

## Who is the customer?

In the assignment, **ABC.ai** is the company building the product and
**L'Oréal** is the hypothetical paying client. The day-to-day user is likely a
L'Oréal content, SEO, brand, or growth manager.

Their question is simple: *When prospective customers ask AI for beauty advice,
where are we missing and what should we investigate next?*

## What decision should the product improve?

The product should help a team decide **where to spend its next content or
technical-investment hour**.

For example: rather than publishing ten generic articles, investigate why
L'Oréal appears in only `1 of 6` comparison fixtures and decide whether a
clearer comparison page, product-fact page, or source-coverage improvement is
worth testing.

This is the decision-first mindset used in this FAQ: a feature is useful only
if it helps a specific person make a better choice.

## What does v0.0.1 actually demonstrate?

It demonstrates this complete but deliberately tiny loop:

```text
authored answer fixtures
    -> deterministic brand-mention check
    -> intent-by-provider counts
    -> weakest cohort
    -> evidence drill-down
    -> one non-causal action hypothesis
```

The reviewer can start with a summary number, open the relevant intent cohort,
and see every answer behind that number—including the negative answers.

## Is this live ChatGPT, Claude, or Gemini data? Why use fixtures?

No. Every answer is **authored fixture data** compiled into the app. The app
labels this boundary prominently so a dashboard screenshot cannot be mistaken
for a current market measurement.

That is intentional: this version proves the evidence-to-decision workflow
without pretending that a small assignment demo has live, representative data.

Fixtures make the prototype reproducible. A reviewer can rerun it and get the
same inputs, counts, priority, and evidence trail.

Live answer engines are variable: answers can change by day, model version,
location, account state, and prompt wording. A production product needs to
record those variables; a two-day demonstrator should not hide them behind a
fake sense of precision.

## How would a production product collect real data?

It would combine four kinds of evidence:

1. A stable prompt library built from customer research, search data, support
   questions, product areas, and approved human review.
2. Repeated, locale-aware runs against supported answer engines, through an
   appropriate provider API, browser-based collection, or both.
3. Immutable raw artifacts: prompt, provider/model/configuration, timestamp,
   answer, citation/source data, errors, and response metadata.
4. Client signals such as product catalog facts, approved claims, AI referral
   traffic, cited pages, crawlability, and competitors.

The dashboard is the last step. The raw run and its provenance are the product's
real foundation.

## Do we need lots of LLM-as-a-judge?

No. Use simple deterministic rules for simple facts.

| Question | Preferred method |
| --- | --- |
| Did the answer contain a configured L'Oréal alias? | Deterministic alias matching |
| Which prompt, provider, and date produced it? | Stored run metadata |
| Was L'Oréal genuinely recommended or merely mentioned? | LLM judge plus human calibration |
| Is a response relevant to the intent or claims-safe? | LLM-assisted review plus domain/human approval |

An LLM judge is a fallible classifier, not an oracle. Save its model, prompt,
version, output, and confidence; compare it with a human-labelled gold set;
route disputed or low-confidence cases to a person.

## Why not create one big “AEO score”?

A single score can be useful for navigation, but it is not the customer outcome.
It can hide which prompts, providers, markets, competitors, or evidence drove a
change.

This app therefore shows `mentioned / total` before any percentage, preserves
the provider split, and lets the user inspect raw answers. A metric earns trust
only when it points to a decision and can be explained.

## What is the current action recommendation worth?

It is a **content-investigation hypothesis**:

> “Comparison is the weakest fixture cohort. Investigate content that explains
> L'Oréal Paris shampoo alternatives for colour-treated hair.”

It is not proof that publishing a page will increase live visibility. A real
claim of improvement requires publication, enough time for retrieval systems to
change, and a controlled rerun of the same measurement protocol.

## What is intentionally missing from v0.0.1?

- live provider adapters and credential handling;
- a versioned beauty taxonomy and client/product catalog;
- prompt generation, paraphrase testing, and demand weighting;
- a database with run history, citations, and raw payload storage;
- human-labelled calibration data for LLM judges;
- causal measurement of a published intervention; and
- multi-client authentication, permissions, exports, alerts, and workflows.

These are production capabilities, not omissions hidden by the prototype.

## How would the product scale beyond L'Oréal shampoo?

Keep the system configuration-driven:

```text
vertical taxonomy + client overlay + prompt cohort + provider run
    -> observation + citation + scoring version
    -> recommendation evidence + validation rerun
```

A new client should add client, product, alias, market, and competitor records.
A new vertical should add a versioned taxonomy package. Neither should require
rewriting the measurement engine.

## What should a hiring manager take away?

The point is not that this small desktop app is already an enterprise AEO
platform. The point is that it makes the smallest trustworthy promise:

> A reviewer can trace one content priority from a multi-provider measurement
> to every underlying answer, understand its limits, and decide what to test
> next.

That is a stronger foundation than a polished dashboard with unexplained scores.

## Further reading

- [Original assignment brief](docs/D01-problem-statement.md)
- [Research, production architecture, schema, and validation plan](docs/D02-initial-analysis-20260812.md)
- [Fixture-only executable specification](docs/D03-minimal-fixture-prd-specs.md)
- [Screen-by-screen product narrative](Narrative/v001/README.md)

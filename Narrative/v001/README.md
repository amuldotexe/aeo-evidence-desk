# AEO Fixture Evidence Desk — Narrative v001

**Build documented:** `v0.0.1` (macOS, offline fixture demonstrator)

This is a deliberately small desktop product: it turns a fixed set of six
authored provider-answer fixtures into one content-investigation decision. It
does **not** retrieve live answers or claim real provider visibility.

## Captures

### 1. Decision dashboard

![Full decision dashboard](01-dashboard.jpg)

This is the starting screen. It answers the first operator question: *which
intent cohort should I investigate, and why?*

- The **fixture-mode banner** prevents the screen from being mistaken for a
  live measurement product.
- The **fixed evaluation scope** states exactly which category, entity, and
  locale the six fixtures represent, plus the matching rule used to count an
  answer.
- The **priority investigation** panel selects the weakest cohort, reports its
  observed fixture count (`1 of 6`), and proposes a concrete content
  investigation without claiming a causal visibility outcome.
- The **presence matrix** compares the three intent cohorts across ChatGPT,
  Claude, and Gemini. Its underlined cohort labels are the route into the
  evidence review.

### 2. Decision rationale detail

![Scope and priority detail](02-priority-detail.jpg)

This is a close-up capture of the evidence boundary and the action rationale
from the dashboard. It is intentionally a detail of the same screen—not a
second product route—so a reader can audit the qualifier text and the exact
action hypothesis without losing the context of the main decision.

## Other deliberate screen states

The application uses one native window and replaces its contents rather than
adding navigation chrome. That keeps the demo focused while preserving a
traceable path from summary to source evidence.

| State | How it is reached | What it does |
| --- | --- | --- |
| **Evidence review** | Select an underlined intent cohort in the presence matrix. | Shows all six underlying fixture answers for that cohort, including provider, prompt, answer text, source type, positive/negative mention status, and matched alias. The same fixture warning, scope, and priority rationale remain visible so a reviewer can audit the summary without losing its boundary conditions. |
| **Fixture error** | Load an incomplete, invalid, or mismatched fixture contract. | Stops the normal dashboard and presents an explicit error state instead of silently calculating an untrustworthy matrix. This makes data assumptions visible in a demo that has no live backend. |

## Intended review path

1. Read the fixture-mode warning and scope.
2. Inspect the cohort totals in the matrix.
3. Open a cohort and read every underlying fixture answer.
4. Decide whether the stated content investigation is credible, knowing it is
   a hypothesis grounded in fixture data—not proof of causal lift.

## Screenshot notes

Both images are direct captures from the shipped macOS application. The second
is cropped only to make the decision boundary and priority panel legible; no
UI content was altered.

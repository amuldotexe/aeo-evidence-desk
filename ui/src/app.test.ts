import { describe, expect, it, vi } from "vitest";

import { mountFixtureEvidenceDesk } from "./app";
import type { DashboardData, IntentEvidenceData, TauriInvoke } from "./types";

const dashboardData: DashboardData = {
  fixture_notice: "Fixture analysis — authored demonstration data; not a live provider measurement.",
  scope_label: "Beauty > Hair care > L’Oréal Paris shampoo > en-US",
  scoring_rule: "An answer is counted when it contains a configured alias using case-insensitive whole-word matching.",
  providers: [
    { id: "chatgpt", label: "ChatGPT" },
    { id: "claude", label: "Claude" },
    { id: "gemini", label: "Gemini" },
  ],
  intent_rates: [
    {
      intent_id: "discovery",
      intent_label: "Discovery",
      mentioned: 3,
      total: 6,
      provider_cells: [
        { provider_id: "chatgpt", provider_label: "ChatGPT", mentioned: 2, total: 2 },
        { provider_id: "claude", provider_label: "Claude", mentioned: 0, total: 2 },
        { provider_id: "gemini", provider_label: "Gemini", mentioned: 1, total: 2 },
      ],
    },
    {
      intent_id: "comparison",
      intent_label: "Comparison",
      mentioned: 1,
      total: 6,
      provider_cells: [
        { provider_id: "chatgpt", provider_label: "ChatGPT", mentioned: 1, total: 2 },
        { provider_id: "claude", provider_label: "Claude", mentioned: 0, total: 2 },
        { provider_id: "gemini", provider_label: "Gemini", mentioned: 0, total: 2 },
      ],
    },
    {
      intent_id: "concern-solving",
      intent_label: "Concern solving",
      mentioned: 3,
      total: 6,
      provider_cells: [
        { provider_id: "chatgpt", provider_label: "ChatGPT", mentioned: 1, total: 2 },
        { provider_id: "claude", provider_label: "Claude", mentioned: 1, total: 2 },
        { provider_id: "gemini", provider_label: "Gemini", mentioned: 1, total: 2 },
      ],
    },
  ],
  priority: {
    intent_id: "comparison",
    intent_label: "Comparison",
    mentioned: 1,
    total: 6,
    action_template: "Investigate comparison content that explains L’Oréal Paris shampoo alternatives.",
    boundary_label: "This is a content-investigation hypothesis, not proof of causal lift.",
  },
};

const comparisonEvidence: IntentEvidenceData = {
  fixture_notice: dashboardData.fixture_notice,
  scope_label: dashboardData.scope_label,
  scoring_rule: dashboardData.scoring_rule,
  intent: dashboardData.intent_rates[1],
  priority: dashboardData.priority,
  evidence_rows: Array.from({ length: 6 }, (_, index) => ({
    observation_id: `comparison-${index}`,
    intent_id: "comparison",
    prompt_id: `comparison-${Math.floor(index / 3) + 1}`,
    prompt_text: "What should I compare before choosing a colour-care shampoo?",
    provider_id: ["chatgpt", "claude", "gemini"][index % 3],
    provider_label: ["ChatGPT", "Claude", "Gemini"][index % 3],
    answer_text: `Authored fixture answer ${index + 1}.`,
    source_type: "authored_demo_fixture",
    mention: {
      brand_mentioned: index === 0,
      matched_alias: index === 0 ? "L’Oréal Paris" : null,
    },
  })),
};

function createSuccessfulInvokeMock(): ReturnType<typeof vi.fn> {
  return vi.fn(async (command: string) => {
    if (command === "get_fixture_dashboard_data") {
      return dashboardData;
    }
    if (command === "get_intent_evidence_data") {
      return comparisonEvidence;
    }
    throw new Error(`Unexpected command: ${command}`);
  });
}

describe("AEO Fixture Evidence Desk", () => {
  it("TEST-UI-009 renders the fixture warning, matrix counts, and action", async () => {
    const root = document.createElement("div");
    const invokeMock = createSuccessfulInvokeMock();
    const desk = mountFixtureEvidenceDesk(root, invokeMock as TauriInvoke);

    await desk.loadDashboardData();

    expect(root.textContent).toContain("Fixture analysis — authored demonstration data");
    expect(root.textContent).toContain("1 / 6");
    expect(root.textContent).toContain("Investigate comparison content");
    expect(root.querySelectorAll("[data-intent-id]")).toHaveLength(3);
  });

  it("TEST-UI-010 renders a readable command error", async () => {
    const root = document.createElement("div");
    const invokeMock = vi.fn().mockRejectedValue({
      code: "InvalidFixture",
      message: "Fixture data could not be loaded.",
    });
    const desk = mountFixtureEvidenceDesk(root, invokeMock as TauriInvoke);

    await desk.loadDashboardData();

    expect(root.textContent).toContain("Unable to load fixture analysis.");
    expect(root.textContent).toContain("Fixture data could not be loaded.");
  });

  it("TEST-UI-011 requests a stable intent ID and renders all evidence rows", async () => {
    const root = document.createElement("div");
    const invokeMock = createSuccessfulInvokeMock();
    const desk = mountFixtureEvidenceDesk(root, invokeMock as TauriInvoke);

    await desk.loadDashboardData();
    root.querySelector<HTMLButtonElement>("[data-intent-id='comparison']")?.click();
    await new Promise((resolve) => window.setTimeout(resolve, 0));

    expect(invokeMock).toHaveBeenCalledWith("get_intent_evidence_data", { intentId: "comparison" });
    expect(root.querySelectorAll("[data-evidence-card]")).toHaveLength(6);
    expect(root.textContent).toContain("authored_demo_fixture");
    expect(root.textContent).toContain("Fixture analysis — authored demonstration data");
  });
});

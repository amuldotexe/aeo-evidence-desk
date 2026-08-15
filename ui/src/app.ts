import { createFixtureDeskClient } from "./api";
import type {
  DashboardData,
  IntentEvidenceData,
  IntentPresenceRate,
  PriorityIntentGap,
  TauriInvoke,
} from "./types";

export type FixtureDeskController = {
  loadDashboardData: () => Promise<void>;
};

export function mountFixtureEvidenceDesk(
  rootElement: HTMLElement,
  invokeCommand: TauriInvoke,
): FixtureDeskController {
  const fixtureDeskClient = createFixtureDeskClient(invokeCommand);
  let evidenceContainer: HTMLElement | undefined;

  async function loadDashboardData(): Promise<void> {
    renderLoadingState(rootElement, "Loading the authored fixture analysis…");

    try {
      const dashboardData = await fixtureDeskClient.getFixtureDashboardData();
      evidenceContainer = renderFixtureDashboardView(rootElement, dashboardData, loadIntentEvidenceData);
    } catch (errorValue) {
      renderCommandErrorState(rootElement, "Unable to load fixture analysis.", errorValue);
    }
  }

  async function loadIntentEvidenceData(intentId: string): Promise<void> {
    if (!evidenceContainer) {
      return;
    }

    renderLoadingState(evidenceContainer, "Loading the selected answer evidence…");

    try {
      const evidenceData = await fixtureDeskClient.getIntentEvidenceData(intentId);
      evidenceContainer.replaceChildren(renderIntentEvidencePanel(evidenceData));
    } catch (errorValue) {
      renderCommandErrorState(
        evidenceContainer,
        "Unable to load evidence for the selected intent.",
        errorValue,
      );
    }
  }

  return { loadDashboardData };
}

function renderFixtureDashboardView(
  rootElement: HTMLElement,
  dashboardData: DashboardData,
  selectIntentEvidence: (intentId: string) => Promise<void>,
): HTMLElement {
  const appShell = createDomElementWithText("main", "app-shell");
  appShell.append(
    createApplicationMasthead(),
    createFixtureWarningBanner(dashboardData.fixture_notice),
    createScopeSummaryPanel(dashboardData.scope_label, dashboardData.scoring_rule),
    createPriorityActionPanel(dashboardData.priority),
  );

  const analysisSection = createDomElementWithText("section", "analysis-section");
  const sectionHeading = createDomElementWithText("div", "section-heading");
  sectionHeading.append(
    createDomElementWithText("p", "eyebrow", "Presence matrix"),
    createDomElementWithText(
      "h2",
      "section-title",
      "Open an intent to inspect every underlying fixture answer.",
    ),
  );
  analysisSection.append(
    sectionHeading,
    createIntentMatrixTable(dashboardData, selectIntentEvidence),
  );

  const evidenceSection = createDomElementWithText("section", "evidence-section");
  evidenceSection.setAttribute("aria-live", "polite");
  evidenceSection.append(createEvidenceEmptyState());
  appShell.append(analysisSection, evidenceSection);

  rootElement.replaceChildren(appShell);
  return evidenceSection;
}

function createApplicationMasthead(): HTMLElement {
  const masthead = createDomElementWithText("header", "masthead");
  const brandLockup = createDomElementWithText("div", "brand-lockup");
  brandLockup.append(
    createDomElementWithText("p", "brand-kicker", "AEO · EVIDENCE DESK"),
    createDomElementWithText("h1", "brand-title", "One content decision. Traceable evidence."),
  );
  masthead.append(
    brandLockup,
    createDomElementWithText("p", "masthead-mode", "OFFLINE FIXTURE MODE"),
  );
  return masthead;
}

function createFixtureWarningBanner(fixtureNotice: string): HTMLElement {
  const warningBanner = createDomElementWithText("aside", "fixture-notice", fixtureNotice);
  warningBanner.setAttribute("role", "note");
  return warningBanner;
}

function createScopeSummaryPanel(scopeLabel: string, scoringRule: string): HTMLElement {
  const scopePanel = createDomElementWithText("section", "scope-panel");
  scopePanel.append(
    createDomElementWithText("p", "eyebrow", "Fixed evaluation scope"),
    createDomElementWithText("p", "scope-label", scopeLabel),
    createDomElementWithText("p", "scoring-rule", scoringRule),
  );
  return scopePanel;
}

function createPriorityActionPanel(priority: PriorityIntentGap): HTMLElement {
  const priorityPanel = createDomElementWithText("section", "priority-panel");
  const priorityHeader = createDomElementWithText("div", "priority-header");
  priorityHeader.append(
    createDomElementWithText("p", "eyebrow", "Priority investigation"),
    createDomElementWithText("p", "priority-intent", priority.intent_label),
  );
  const priorityEvidence = createDomElementWithText(
    "p",
    "priority-evidence",
    `Observed fixture evidence: ${priority.mentioned} of ${priority.total}`,
  );
  const actionHypothesis = createDomElementWithText(
    "p",
    "action-hypothesis",
    priority.action_template,
  );
  const boundaryLabel = createDomElementWithText("p", "boundary-label", priority.boundary_label);
  priorityPanel.append(priorityHeader, priorityEvidence, actionHypothesis, boundaryLabel);
  return priorityPanel;
}

function createIntentMatrixTable(
  dashboardData: DashboardData,
  selectIntentEvidence: (intentId: string) => Promise<void>,
): HTMLElement {
  const tableWrapper = createDomElementWithText("div", "matrix-scroll");
  const matrixTable = createDomElementWithText("table", "presence-matrix");
  const tableHeader = createDomElementWithText("thead", "matrix-header");
  const headerRow = createDomElementWithText("tr", "matrix-header-row");
  headerRow.append(createDomElementWithText("th", "intent-column", "Intent cohort"));

  for (const provider of dashboardData.providers) {
    headerRow.append(createDomElementWithText("th", "provider-column", provider.label));
  }
  headerRow.append(createDomElementWithText("th", "cohort-column", "Cohort total"));
  tableHeader.append(headerRow);

  const tableBody = createDomElementWithText("tbody", "matrix-body");
  for (const intentRate of dashboardData.intent_rates) {
    tableBody.append(createIntentMatrixRow(intentRate, selectIntentEvidence));
  }

  matrixTable.append(tableHeader, tableBody);
  tableWrapper.append(matrixTable);
  return tableWrapper;
}

function createIntentMatrixRow(
  intentRate: IntentPresenceRate,
  selectIntentEvidence: (intentId: string) => Promise<void>,
): HTMLElement {
  const tableRow = createDomElementWithText("tr", "matrix-row");
  const intentCell = createDomElementWithText("td", "intent-cell");
  const intentButton = createDomElementWithText("button", "intent-button", intentRate.intent_label);
  intentButton.type = "button";
  intentButton.dataset.intentId = intentRate.intent_id;
  intentButton.setAttribute("aria-label", `Open ${intentRate.intent_label} fixture evidence`);
  intentButton.addEventListener("click", () => {
    void selectIntentEvidence(intentRate.intent_id);
  });
  intentCell.append(intentButton);
  tableRow.append(intentCell);

  for (const providerCell of intentRate.provider_cells) {
    tableRow.append(
      createDomElementWithText(
        "td",
        "matrix-count",
        `${providerCell.mentioned} / ${providerCell.total}`,
      ),
    );
  }
  tableRow.append(
    createDomElementWithText("td", "matrix-total", `${intentRate.mentioned} / ${intentRate.total}`),
  );
  return tableRow;
}

function createEvidenceEmptyState(): HTMLElement {
  const emptyState = createDomElementWithText("div", "evidence-empty-state");
  emptyState.append(
    createDomElementWithText("p", "eyebrow", "Evidence review"),
    createDomElementWithText(
      "p",
      "empty-state-copy",
      "Select an intent cohort above to reveal all six authored provider-labelled answers, including negative results.",
    ),
  );
  return emptyState;
}

function renderIntentEvidencePanel(evidenceData: IntentEvidenceData): HTMLElement {
  const evidencePanel = createDomElementWithText("div", "evidence-panel");
  const evidenceHeading = createDomElementWithText("div", "evidence-heading");
  evidenceHeading.append(
    createDomElementWithText("p", "eyebrow", "Evidence review"),
    createDomElementWithText(
      "h2",
      "section-title",
      `${evidenceData.intent.intent_label}: ${evidenceData.intent.mentioned} / ${evidenceData.intent.total}`,
    ),
    createDomElementWithText(
      "p",
      "evidence-subtitle",
      "Every authored answer is shown. A negative result is evidence too.",
    ),
  );
  const evidenceGrid = createDomElementWithText("div", "evidence-grid");
  for (const evidenceRow of evidenceData.evidence_rows) {
    evidenceGrid.append(createEvidenceAnswerCard(evidenceRow));
  }

  evidencePanel.append(
    createFixtureWarningBanner(evidenceData.fixture_notice),
    evidenceHeading,
    createPriorityActionPanel(evidenceData.priority),
    evidenceGrid,
  );
  return evidencePanel;
}

function createEvidenceAnswerCard(evidenceRow: IntentEvidenceData["evidence_rows"][number]): HTMLElement {
  const evidenceCard = createDomElementWithText("article", "evidence-card");
  evidenceCard.dataset.evidenceCard = evidenceRow.observation_id;
  const cardMeta = createDomElementWithText("div", "card-meta");
  cardMeta.append(
    createDomElementWithText("span", "provider-chip", evidenceRow.provider_label),
    createDomElementWithText("span", "source-chip", evidenceRow.source_type),
  );
  const scoreText = evidenceRow.mention.brand_mentioned
    ? `Mentioned: ${evidenceRow.mention.matched_alias}`
    : "Not mentioned";
  const scoreClass = evidenceRow.mention.brand_mentioned ? "mention-positive" : "mention-negative";
  evidenceCard.append(
    cardMeta,
    createDomElementWithText("p", "prompt-label", "Prompt"),
    createDomElementWithText("p", "prompt-text", evidenceRow.prompt_text),
    createDomElementWithText("p", "answer-label", "Fixture answer"),
    createDomElementWithText("p", "answer-text", evidenceRow.answer_text),
    createDomElementWithText("p", scoreClass, scoreText),
  );
  return evidenceCard;
}

function renderLoadingState(containerElement: HTMLElement, loadingMessage: string): void {
  containerElement.replaceChildren(createDomElementWithText("p", "loading-state", loadingMessage));
}

function renderCommandErrorState(
  containerElement: HTMLElement,
  titleText: string,
  errorValue: unknown,
): void {
  const errorPanel = createDomElementWithText("section", "command-error");
  errorPanel.setAttribute("role", "alert");
  errorPanel.append(
    createDomElementWithText("p", "error-title", titleText),
    createDomElementWithText("p", "error-message", getUserSafeErrorMessage(errorValue)),
  );
  containerElement.replaceChildren(errorPanel);
}

function getUserSafeErrorMessage(errorValue: unknown): string {
  if (
    typeof errorValue === "object" &&
    errorValue !== null &&
    "message" in errorValue &&
    typeof errorValue.message === "string"
  ) {
    return errorValue.message;
  }

  return "The desktop command did not return a readable error.";
}

function createDomElementWithText<TagName extends keyof HTMLElementTagNameMap>(
  tagName: TagName,
  className: string,
  textContent?: string,
): HTMLElementTagNameMap[TagName] {
  const element = document.createElement(tagName);
  element.className = className;
  if (textContent !== undefined) {
    element.textContent = textContent;
  }
  return element;
}

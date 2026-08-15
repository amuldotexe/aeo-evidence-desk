export type AppError = {
  code: "InvalidFixture" | "IntentNotFound";
  message: string;
};

export type ProviderLabel = {
  id: string;
  label: string;
};

export type ProviderPresenceCell = {
  provider_id: string;
  provider_label: string;
  mentioned: number;
  total: number;
};

export type IntentPresenceRate = {
  intent_id: string;
  intent_label: string;
  mentioned: number;
  total: number;
  provider_cells: ProviderPresenceCell[];
};

export type PriorityIntentGap = {
  intent_id: string;
  intent_label: string;
  mentioned: number;
  total: number;
  action_template: string;
  boundary_label: string;
};

export type DashboardData = {
  fixture_notice: string;
  scope_label: string;
  scoring_rule: string;
  providers: ProviderLabel[];
  intent_rates: IntentPresenceRate[];
  priority: PriorityIntentGap;
};

export type MentionOutcome = {
  brand_mentioned: boolean;
  matched_alias: string | null;
};

export type IntentEvidenceRow = {
  observation_id: string;
  intent_id: string;
  prompt_id: string;
  prompt_text: string;
  provider_id: string;
  provider_label: string;
  answer_text: string;
  source_type: string;
  mention: MentionOutcome;
};

export type IntentEvidenceData = {
  fixture_notice: string;
  scope_label: string;
  scoring_rule: string;
  intent: IntentPresenceRate;
  priority: PriorityIntentGap;
  evidence_rows: IntentEvidenceRow[];
};

export type TauriInvoke = <ResultData>(
  command: string,
  commandArguments?: Record<string, unknown>,
) => Promise<ResultData>;

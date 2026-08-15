import { invoke } from "@tauri-apps/api/core";

import type { DashboardData, IntentEvidenceData, TauriInvoke } from "./types";

export type FixtureDeskClient = {
  getFixtureDashboardData: () => Promise<DashboardData>;
  getIntentEvidenceData: (intentId: string) => Promise<IntentEvidenceData>;
};

export const invokeTauriCommand: TauriInvoke = (command, argumentsValue) =>
  invoke(command, argumentsValue);

export function createFixtureDeskClient(invokeCommand: TauriInvoke): FixtureDeskClient {
  return {
    getFixtureDashboardData: () => invokeCommand<DashboardData>("get_fixture_dashboard_data"),
    getIntentEvidenceData: (intentId: string) =>
      invokeCommand<IntentEvidenceData>("get_intent_evidence_data", { intentId }),
  };
}

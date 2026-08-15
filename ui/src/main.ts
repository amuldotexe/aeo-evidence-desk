import { mountFixtureEvidenceDesk } from "./app";
import { invokeTauriCommand } from "./api";
import "./styles.css";

const rootElement = document.querySelector<HTMLElement>("#app");

if (rootElement) {
  const fixtureDesk = mountFixtureEvidenceDesk(rootElement, invokeTauriCommand);
  void fixtureDesk.loadDashboardData();
}

pub mod commands;

pub use commands::{get_fixture_dashboard_data, get_intent_evidence_data};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run_desktop_fixture_application() {
    let application_result = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_fixture_dashboard_data,
            get_intent_evidence_data
        ])
        .run(tauri::generate_context!());

    if let Err(application_error) = application_result {
        eprintln!("Unable to start the AEO Fixture Evidence Desk: {application_error}");
    }
}

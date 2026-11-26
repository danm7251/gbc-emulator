use std::{env, fs};

use chrono::Utc;
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::{fmt::{self, format::FmtSpan}, prelude::*};

// The directory path to the output.
const OUTPUT_PATH: &'static str = "target/traces";

/// Initialises standard output and Chrome tracing.
///
/// Keep the returned guard alive until tracing is finished.
/// Dropping the guard flushes the Chrome trace to `trace.json`.
pub fn init_tracing(with_spans: bool) -> impl Drop {
    // Generate unique filename for output file and create path if it doesn't exist.
    let filename = generate_filename(OUTPUT_PATH);

    // The ChromeLayer writes to the file on Drop therefore a guard is necessary.
    let (chrome_layer, guard) = ChromeLayerBuilder::new()
        .file(filename)
        .include_args(true)
        .build();

    let mut stdout_layer = fmt::layer()
        .with_target(false)
        .without_time();

    if with_spans {
        stdout_layer = stdout_layer.with_span_events(FmtSpan::ACTIVE);
    }

    tracing_subscriber::registry()
        .with(chrome_layer)
        .with(stdout_layer)
        .init();

    guard
}

fn generate_filename(output_path: &str) -> String {
    // Creates the output directory path if it does not already exist.
    fs::create_dir_all(output_path)
        .unwrap_or_else(|e| panic!("Failed to create {OUTPUT_PATH}: {e}"));

    // Extracts name of binary being traced.
    let binary_name = env::current_exe()
        .unwrap_or_else(|e| panic!("Failed to obtain binary path: {e}"))
        .file_stem()
        .unwrap_or_else(|| panic!("Failed to extract binary name from path"))
        .to_string_lossy()
        .into_owned();

    let timestamp = Utc::now().format("%m-%d_%H-%M-%S");

    format!("{output_path}/{binary_name}_trace_{timestamp}.json")
}
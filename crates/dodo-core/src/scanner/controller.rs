use crate::scanner::ignore::build_ignore_matcher;
use anyhow::Result;
use dodo_ai::{AiEngine, ConfignetEngine};
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use serde_json;

/// Main entry point: scan project, run Magika per file, send results to AI
pub fn scan_with_magika(test_dirs: &[String]) -> anyhow::Result<()> {
    let ignore_matcher = build_ignore_matcher(test_dirs)?;

    let walker = WalkBuilder::new(".")
        .hidden(false)
        .standard_filters(true)
        .build();

    let files_to_process: Vec<PathBuf> = walker
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file() && path.exists())
        .filter(|path| !is_hidden(path) && !ignore_matcher.matched(path, false).is_ignore())
        .collect();

    for path in &files_to_process {
        if let Err(err) = process_file(path) {
            eprintln!("Error processing {}: {}", path.display(), err);
        }
    }

    Ok(())
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .map(|name| name.to_string_lossy().starts_with('.'))
        .unwrap_or(false)
}

// TODO: Decouple Magika later. Implement in dodo-filetype.
fn process_file(path: &Path) -> anyhow::Result<()> {
    // Run Magika with JSON output
    let output = Command::new("magika")
        .arg("--json")
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait_with_output()?;

    if !output.status.success() {
        anyhow::bail!("Magika failed on {}", path.display());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse JSON output
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| anyhow::anyhow!("Failed to parse Magika JSON: {}", e))?;

    // Extract both path and label from JSON
    let (json_path, label) = parsed.get(0)
        .and_then(|entry| {
            let path = entry["path"].as_str();
            let label = entry["result"]["value"]["dl"]["label"].as_str();
            path.zip(label)
        })
        .unwrap_or(("unknown_path", "unknown"));

    // Pass both values to AI processing
    send_to_ai(json_path, label)?;

    Ok(())
}

// Update to accept both values
pub fn send_to_ai(path: &str, magika_label: &str) -> Result<()> {
    println!("{}: {}", path, magika_label);
    let engine = ConfignetEngine::new()?;
    // Assuming engine method now takes string path instead of Path
    engine.process_file_with_magika(Path::new(path), magika_label)?;
    Ok(())
}

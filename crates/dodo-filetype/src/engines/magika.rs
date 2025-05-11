use crate::engine::{FiletypeEngine, FiletypeResult};
use std::path::Path;
use serde_json;

pub struct MagikaEngine;

impl FiletypeEngine for MagikaEngine {
    fn detect(&self, file_path: &Path) -> anyhow::Result<FiletypeResult> {
        // Run Magika with JSON output format
        let output = std::process::Command::new("magika")
            .arg("--json")
            .arg(file_path)
            .output()?;

        if !output.status.success() {
            anyhow::bail!("Magika detection failed");
        }

        // Convert output to string
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse JSON output
        let parsed: serde_json::Value = serde_json::from_str(&stdout)?;

        // Extract the simple label (magika_label) instead of MIME type
        let label = parsed["magika_label"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        Ok(FiletypeResult {
            file_path: file_path.to_path_buf(),
            mime: label,
            confidence: parsed["confidence"]
                .as_f64()
                .unwrap_or(0.0) as f32,
            source: "Magika".to_string(),
        })
    }

    fn name(&self) -> &'static str {
        "Magika"
    }
}

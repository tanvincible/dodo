use crate::engine::{FiletypeEngine, FiletypeResult};
use std::path::{Path, PathBuf};

pub struct MagikaEngine;

impl FiletypeEngine for MagikaEngine {
    fn detect(&self, file_path: &Path) -> anyhow::Result<FiletypeResult> {
        // Simplified example using CLI
        let output = std::process::Command::new("magika")
            .arg(file_path)
            .arg("--output-format=json")
            .output()?;

        if !output.status.success() {
            anyhow::bail!("Magika detection failed");
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let parsed: serde_json::Value = serde_json::from_str(&stdout)?;

        Ok(FiletypeResult {
            file_path: file_path.to_path_buf(),
            mime: parsed["predicted_mime"].as_str().unwrap_or("unknown").to_string(),
            confidence: parsed["confidence"].as_f64().unwrap_or(0.0) as f32,
            source: "Magika".to_string(),
        })
    }

    fn name(&self) -> &'static str {
        "Magika"
    }
}

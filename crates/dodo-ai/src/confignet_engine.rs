use std::path::Path;
use anyhow::{Result, anyhow};
use confignet::ConfigClassifier;
use crate::AiEngine;

pub struct ConfignetEngine {
    classifier: ConfigClassifier,
}

use std::fs;

impl ConfignetEngine {
    pub fn new() -> Result<Self> {
        let csv_data = include_str!("../data/labeled/ci_cd.csv");

        let classifier = ConfigClassifier::from_csv_str(csv_data)?;

        Ok(Self { classifier })
    }
}

impl AiEngine for ConfignetEngine {
    fn process_file_with_magika(&self, path: &Path, magika_output: &str) -> Result<()> {
        // Extract MIME type from Magika's output
        let mime = extract_mime(magika_output)?;
        let file_name = path.file_name()
            .ok_or_else(|| anyhow!("Invalid file path: no file name"))?
            .to_string_lossy();

        match self.classifier.classify(&*file_name, &mime) {
            Some(result) => {
                println!(
                    "{{ \"file_name\": \"{}\", \"is_ci_cd\": {} }}",
                    result.file_name, result.is_ci_cd
                );
            }
            None => {
                println!("{{ \"file_name\": \"{}\", \"is_ci_cd\": false }}", file_name);
            }
        }

        Ok(())
    }
}

fn extract_mime(magika_output: &str) -> Result<String> {
    let parsed: serde_json::Value = serde_json::from_str(magika_output)?;
    Ok(parsed["mime"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing or invalid MIME field"))?
        .to_string())
}

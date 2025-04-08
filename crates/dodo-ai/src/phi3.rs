use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use anyhow::{anyhow, Result};
use ureq;
use serde_json::{json, Value};

use crate::AiEngine;

pub struct Phi3MiniEngine;

impl Phi3MiniEngine {
    pub fn new() -> Result<Self> {
        let res = ureq::get("http://localhost:11434").call();

        match res {
            Ok(_) => Ok(Self),
            Err(ureq::Error::Http(status)) => {
                Err(anyhow!("Phi-3 Mini server returned status code: {}", status))
            }
            Err(e) => Err(anyhow!("Failed to connect to Phi-3 Mini at localhost:11434: {e}")),
        }
    }
}

impl AiEngine for Phi3MiniEngine {
    fn process_file_with_magika(&self, path: &Path, magika_output: &str) -> Result<()> {
        let prompt = format!(
            "Given this Magika output, classify the file:\n\n{}\n\nFile path: {}",
            magika_output,
            path.display()
        );

        let response = ureq::post("http://localhost:11434/api/generate")
            .header("Content-Type", "application/json")
            .send_json(json!({
                "model": "phi3:mini-128k",
                "prompt": prompt,
                "stream": true
            }))?;

        // Split response into headers and body
        let (_, body) = response.into_parts();
        // Get reader from Body
        let reader = body.into_reader();

        let mut stdout = std::io::stdout();
        for line in std::io::BufReader::new(reader).lines() {
            let line = line.map_err(|e| anyhow!("Stream read error: {}", e))?;
            
            if line.trim().is_empty() {
                continue;
            }

            let json: Value = serde_json::from_str(&line)
                .map_err(|e| anyhow!("JSON parse error: {} in line: {}", e, line))?;
            
            if let Some(chunk) = json.get("response").and_then(|v| v.as_str()) {
                write!(stdout, "{}", chunk)
                    .map_err(|e| anyhow!("Output write error: {}", e))?;
                stdout.flush()?;
            }
        }

        println!();
        Ok(())
    }
}

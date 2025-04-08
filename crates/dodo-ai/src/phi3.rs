use std::io::{BufRead, Write};
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
            r#"
        You are an AI embedded in a CI configuration analysis tool.
        
        ## INPUT:
        You will be given:
        - A file path
        - Its content type (from Magika)
        
        ## OBJECTIVE:
        Analyze the file and output strict JSON based on the following:
        
        ---
        
        ### STEP 1 — Is it a config file?
        - If NO, respond exactly with:
          {{ "is_config": false }}
        
        ---
        
        ### STEP 2 — If YES, check if it's CI-related:
        - Only extract metadata for CI-related config files.
        - CI-related = used for build, lint, test, or deploy in CI.
        
        Use these known mappings:
        - **build**: Cargo.toml, package.json, pom.xml, Makefile, CMakeLists.txt
        - **lint**: .eslintrc.*, .stylelintrc.*, flake8, pylint, etc.
        - **test**: pytest.ini, tox.ini, tests/*.py, jest.config.*, etc.
        - **deploy**: Dockerfile, vercel.json, deploy.yml, Netlify config, etc.
        
        ---
        
        ### STEP 3 — Output ONLY this strict JSON if CI-related:
        
        {{
          "is_config": true,
          "ci_phase": "<build|lint|test|deploy|unknown>",
          "tool": "<tool name or unknown>",
          "version": "<version or unknown>",
          "dependencies": [ "<dep1>", "<dep2>", ... ],
          "targets": [ "<glob or path>" ],
          "environment": "<e.g. nodejs, rust, python, etc.>"
        }}
        
        ---
        
        ### RULES (MANDATORY):
        - JSON only — no text, markdown, or comments
        - Always valid JSON
        - If values are missing, use "unknown"
        - Dependencies and targets must always be arrays
        - Do NOT extract metadata if the config is not CI-related
        
        ### FILE PATH:
        {}
        
        ### MAGIKA TYPE:
        {}
        "#,
            path.display(),
            magika_output.trim()
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

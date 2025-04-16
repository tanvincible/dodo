use std::io::{BufRead, Write};
use std::path::Path;

use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use ureq;

use crate::AiEngine;

pub struct Phi3MiniEngine;

const PHI3_SAFE_BYTE_LIMIT: usize = 256 * 1024; // 256 KB safe truncation

impl Phi3MiniEngine {
    pub fn new() -> Result<Self> {
        let res = ureq::get("http://localhost:11434").call();

        match res {
            Ok(_) => Ok(Self),
            Err(ureq::Error::Http(status)) => Err(anyhow!(
                "Phi-3 Mini server returned status code: {}",
                status
            )),
            Err(e) => Err(anyhow!(
                "Failed to connect to Phi-3 Mini at localhost:11434: {e}"
            )),
        }
    }
}

fn read_and_truncate_file(path: &Path, byte_limit: usize) -> Result<String> {
    let bytes = std::fs::read(path)?;
    let truncated = if bytes.len() > byte_limit {
        let cut = &bytes[..byte_limit];
        match std::str::from_utf8(cut) {
            Ok(s) => s.to_string(),
            Err(e) => {
                let valid = &cut[..e.valid_up_to()];
                String::from_utf8_lossy(valid).to_string()
            }
        }
    } else {
        String::from_utf8_lossy(&bytes).to_string()
    };
    Ok(truncated)
}

impl AiEngine for Phi3MiniEngine {
    fn process_file_with_magika(&self, path: &Path, magika_output: &str) -> Result<()> {
        let content = read_and_truncate_file(path, PHI3_SAFE_BYTE_LIMIT)?;

        let prompt = format!(
            r#"
Analyze configuration-related files and extract structured metadata.

---

## INPUT
You will be given:
- A file path (string)
- A Magika-generated file type (string)
- The file content (truncated text, always under 100KB)

---

## OBJECTIVE
Step-by-step behavior:

STEP 1 — Is it a configuration file?
- If Magika says the type is source code (e.g. "Rust source", "Python source", "JavaScript source", etc.), respond:
  {{ "is_config": false }}
  Then skip to the next input.
- Otherwise, if it is a TOML, YAML, JSON, INI, or known CI-related format (see mappings), continue to STEP 2.

STEP 2 — Is it CI-related?
- If NO CI phase can be confidently inferred (e.g., it's a general settings file), return:
  {{ "is_config": true, "ci_phase": "unknown" }}
  Then skip to the next input.
- If YES, continue to STEP 3.

STEP 3 — Extract the following STRICT JSON format:
{{
  "is_config": true,
  "ci_phase": "<build|lint|test|deploy|unknown>",
  "tool": "<tool name or unknown>",
  "version": "<version or unknown>",
  "dependencies": ["<dep1>", "<dep2>", ...],
  "targets": ["<glob or file path>", ...],
  "environment": "<nodejs|rust|python|unknown>"
}}

---

## RULES
- Output must be strictly valid JSON — no prose, comments, or explanations.
- Keys must appear in this exact order.
- If a field is unknown or not extractable, use "unknown" or an empty array ([]).
- Do not guess or hallucinate tools, versions, environments, or dependencies.
- If the file is not configuration-related (e.g. it’s just source code), always return: {{ "is_config": false }}

---

## MAPPINGS

### CI PHASES:
- **Build**: Cargo.toml, package.json, Makefile, CMakeLists.txt, pom.xml, build.gradle, etc.
- **Lint**: .eslintrc.*, .stylelintrc.*, .flake8, pylintrc, pyproject.toml (with lint config), etc.
- **Test**: pytest.ini, tox.ini, jest.config.*, files inside tests/, etc.
- **Deploy**: Dockerfile, vercel.json, netlify.toml, deploy.yml, *.deploy.yml, etc.

---

## EXAMPLES

### ✅ Example 1: Rust Cargo.toml

**MAGIKA TYPE:** Tom's Obvious, Minimal Language (TOML)  
**CONTENT:**
```
[package]
name = "my-rust-app"
version = "0.1.0"
edition = "2021"

[dependencies]
```

**EXPECTED OUTPUT:**
```json
{{
  "is_config": true,
  "ci_phase": "build",
  "tool": "cargo",
  "version": "0.1.0",
  "dependencies": [],
  "targets": [".github/workflows/build.yml"],
  "environment": "rust"
}}
```

---

### ✅ Example 2: Rust source code

**MAGIKA TYPE:** Rust source (code)  
**CONTENT:**
```rust
fn main() {{
    println!("Hello, world!");
}}
```

**EXPECTED OUTPUT:**
```json
{{ "is_config": false }}
```

---

### ✅ Example 3: Dockerfile

**MAGIKA TYPE:** YAML data  
**CONTENT:**
```
FROM node:18
RUN npm install
```

**EXPECTED OUTPUT:**
```json
{{
  "is_config": true,
  "ci_phase": "deploy",
  "tool": "unknown",
  "version": "unknown",
  "dependencies": [],
  "targets": ["./Dockerfile"],
  "environment": "nodejs"
}}
```

---

### FILE PATH:
{filepath}

### MAGIKA TYPE:
{filetype}

### TRUNCATED FILE CONTENT:
{filecontent}

## FINAL NOTE
Respond **only** with the JSON block matching the logic above. Never explain your answer.

"#,
            filepath = path.display(),
            filetype = magika_output.trim(),
            filecontent = content,
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
                write!(stdout, "{}", chunk).map_err(|e| anyhow!("Output write error: {}", e))?;
                stdout.flush()?;
            }
        }

        println!();
        Ok(())
    }
}

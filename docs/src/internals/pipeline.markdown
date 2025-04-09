# Generation of `dodo.toml`

1. 🧠 Magika detects file types
   → Fast, parallel, respects .gitignore and exclusions

2. 📥 Phi-3 checks if path (or name) indicates it's a config file
   → e.g., pyproject.toml, ruff.toml, .prettierrc, go.mod, Cargo.toml
   → If "maybe config", proceed. Otherwise skip.

3. 🔄 Convert config file to JSON
   → Use a modular "parser adapter" interface
     - `toml → json`: [`toml`](https://docs.rs/toml)
     - `yaml → json`: [`serde_yaml`](https://docs.rs/serde_yaml)
     - `ini → json`: [`configparser`](https://crates.io/crates/configparser) or write mini-parser
     - `go.mod`, etc.: external Go tool → output JSON

4. ✅ Send JSON to CUE
   → If schema available, validate and normalize
   → If schema *not* available, attempt inference (CUE supports structural inference)

5. 🧩 CUE returns clean, schema-aligned JSON
   → Structure now looks like:
     ```json
     {
       "tool": "ruff",
       "version": "0.4.3",
       "config": { ... }
     }
     ```

6. 🧠 Phi-3 Mini processes clean JSON
   → Now it's not hallucinating keys, confused by indentation or nesting
   → Output: `{ tool: "ruff", deps: [...], config_version: "0.4.3" }`

7. 📤 Dodo generates or updates `dodo.toml`
   → Merge strategy can be:
     - `strict` (overwrite)
     - `merge` (preserve and add)
     - `interactive` (prompt if conflict, optional)

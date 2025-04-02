---
layout: page
title: Commands
parent: Configuration
---

# Dodo Commands

## **🚀 Key Commands:**  

- **`dodo init`**  
  - Scans the project directory (using tools like Trustfall) to detect language-specific configuration files (e.g., `Cargo.toml`, `pyproject.toml`, etc.).  
  - Uses **AI-assisted parsing** to identify tools for linting, testing, building, and deploying.  
  - Automatically generates a **default** `dodo.toml` based on detected tools.  
  - **Caches detected files** for faster processing in future operations.  

- **`dodo build`**  
  - Reads `dodo.toml`, fetches the appropriate base workflow template from `raphus.io`.  
  - **Substitutes placeholders** based on project-specific settings and **generates** `.github/workflows/ci.yml`.  
  - **Creates/updates `dodo.lock`** to lock versions and ensure reproducibility.  

- **`dodo add <external-ci>`**  
  - Integrates **external workflow snippets** (e.g., **security scans, Dependabot, pre-commit hooks**).  
  - Fetches snippets from `raphus.io` or user-supplied URLs.  
  - Updates both `dodo.toml` and `.github/workflows/*.yml`.  

- **`dodo update`**  
  - Detects **newer versions** of tools in `dodo.toml` and updates dependencies.  
  - Uses **cached project structure** for **blazing-fast** parsing.  
  - Updates `dodo.lock` if newer versions are available.  
  - **Auto-parses new config files** if they appear, ensuring CI/CD stays in sync.  

- **`dodo upgrade`**  
  - Combines `dodo update` + `dodo build`.  
  - Updates **both** `dodo.toml` and `dodo.lock`, then rebuilds `.github/workflows/*.yml`.  

- **`dodo clean [tool]`**  
  - **Removes tools (test, lint, build, deploy) from CI/CD**, even if their config files exist.  
  - **Example:** `dodo clean ruff` → Removes Ruff from CI/CD but keeps Flake8.  
  - **Without arguments:** Removes **all tools** **(test, lint, build, deploy)** **not backed by an existing config file**.  
  - **Preserves external integrations** like **Dependabot, pre-commit, security checks**.  

---

### **🛠 Configuration Files:**  

- **`dodo.toml`**  
  - Defines **CI/CD workflow configurations** based on project tools.  
  - **Automatically generated** by `dodo init`.  
  - Stores linting, testing, build, and deploy settings.  

- **`dodo.lock`**  
  - Locks **resolved versions** of actions, workflow templates, and dependencies.  
  - Ensures **consistent CI/CD runs**.  
  - Only modified by `dodo build`, and `dodo upgrade`.  

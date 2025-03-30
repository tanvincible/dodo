---
layout: page
title: Internals
permalink: /internals/
---

# Internal Working of Dodo

## **Overview**
Dodo is an AI-powered CI/CD automation tool that intelligently parses a project’s configuration files, detects tools for various development workflows (linting, testing, building, deploying), and generates optimized GitHub Actions workflows. It caches detected configurations for fast lookups and ensures that changes in project dependencies are seamlessly integrated into CI/CD pipelines.

## Project Parsing
1. **Full Project Scan:**
   - Uses [Magika by Google](https://github.com/google/magika) to identify and classify files.
   - Uses AI to determine whether a file is a configuration or an "interest" file (i.e., relevant to linting, testing, building, or deployment).

2. **Parsing Configuration Files:**
   - Once files of interest are identified, Dodo uses **Lark** or another parsing tool to extract relevant tool configurations.
   - Extracts tool names, versions, dependencies, and settings from files like:
     - `pyproject.toml`, `setup.py`, `requirements.txt` (Python)
     - `Cargo.toml` (Rust)
     - `package.json` (Node.js)
     - `Makefile`, `CMakeLists.txt` (C/C++)
     - `clippy.toml`, `.prettierrc`, `.eslint.json` (Linters and formatters)

3. **Generating `dodo.toml`:**
   - AI processes the extracted data and generates a structured `dodo.toml` with:
     - Detected tools and their versions
     - CI/CD configurations for linting, testing, building, and deployment
     - Additional settings for caching, notifications, hooks, and security checks

## **2. Dependency Updates & CI/CD Syncing**
### **Dependency Management**
- Many users rely on **Dependabot** or manual updates to bump dependency versions.
- Once new versions are pulled locally, running `dodo update` will:
  - Use the **cached file locations** to rapidly find relevant config files.
  - Parse them for updated versions and reflect the changes in `dodo.toml`.
  - Update the corresponding `dodo.lock` file.

### **CI/CD Workflow Synchronization**
- `dodo update` or `dodo upgrade` also ensures:
  - **Only new/modified files are re-parsed**, optimizing execution speed.
  - If a new configuration file is detected (e.g., `clippy.toml` added), it gets automatically included in `dodo.toml`.

---

## **3. Adding External Integrations**
Dodo allows users to integrate external CI/CD tools using:

```sh
dodo add <external-ci>
```

- This fetches workflow templates from `raphus.io` (Dodo’s template registry) and adds them to `.github/workflows/*.yml`.
- Example integrations:
  - **Security Scans:** Trivy, Snyk, CodeQL
  - **Dependency Updaters:** Dependabot, Renovate
  - **Pre-commit Hooks:** Black, Prettier, pre-commit
- `dodo add` ensures these integrations persist **even when running `dodo clean`**.

---

## **4. Building & Regenerating CI/CD Workflows**
### **Fetching & Adapting CI/CD Templates**
- When running `dodo build`, Dodo:
  - Fetches **base workflow templates** from `raphus.io`.
  - **Uses AI** to adapt them according to project structure.
  - Generates `.github/workflows/ci.yml`.

### **Updating vs. Upgrading**
- `dodo update`: Updates dependency versions in `dodo.toml` and `dodo.lock`.
- `dodo upgrade`: Updates dependency versions **and regenerates** `.github/workflows/*.yml`.

---

## **5. Cleaning CI/CD Configuration**
### **Implicit Cleanup**
```sh
dodo clean
```
- Removes **all tools** (testing, linting, building, deploying) from CI/CD **if their config files are not present in the project**.
- **Does not remove external integrations** (e.g., Dependabot, pre-commit, security scans).

### **Explicit Tool Removal**
```sh
dodo clean <tool>
```
- Removes **specific tools from CI/CD** even if their config files still exist.
- Example:
  ```sh
  dodo clean ruff
  ```
  - Removes **Ruff** from CI/CD but keeps **Flake8** if both are present.

---

## **Final Thoughts**
Dodo is designed to **minimize manual CI/CD management** by ensuring:
- **Automatic detection** of project tools.
- **Smart dependency updates** leveraging cached file locations.
- **Seamless workflow synchronization**.
- **Fast and flexible cleanup operations**.

By integrating AI-powered parsing with a robust caching mechanism, Dodo ensures blazing-fast execution while maintaining an up-to-date and optimized CI/CD pipeline. 🚀

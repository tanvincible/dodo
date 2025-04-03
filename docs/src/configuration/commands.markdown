# Dodo Commands

## 🚀 Key Commands:

### `dodo init`
- Scans the project directory, detecting relevant files while skipping unnecessary ones (`.github/`, `.git/`, and test directories specified by the user).
- Uses **Magika** to identify file types and sends them to **Phi-3 Mini** for classification.
- If a file is a config file, Phi-3 extracts tool versions, dependencies, and relevant settings, then caches it.
- Non-config files are marked accordingly to avoid redundant processing.
- Builds `dodo.toml` incrementally as new configurations are detected.

### `dodo build`
- Reads `dodo.toml` and `dodo.lock`, ensuring an up-to-date understanding of the project.
- Fetches appropriate templates from `raphus.io`.
- Uses Phi-3 and cached data to adapt templates and generate optimized workflows in `.github/workflows/`.
- Updates `dodo.lock` incrementally, recording file paths and ensuring consistency across runs.

### `dodo add <external-ci>`
- Allows users to integrate external workflow snippets like security scans, Dependabot, or pre-commit hooks.
- Fetches predefined snippets from `raphus.io` or user-defined sources.
- Updates `dodo.toml` and `.github/workflows/*.yml` accordingly.

### `dodo update`
- Checks for changes in cached config files, such as version updates or dependency modifications.
- Since everything is incremental, only modified files are reprocessed.
- Ensures minimal overhead by avoiding a full project scan.

### `dodo sync`
- Detects new configuration files that weren’t previously included in `dodo.toml`.
- If a new file (like `ruff.toml`) is found, Magika identifies it, Phi-3 classifies it, and the file is added to the cache.
- Users can also remove stale configuration files that no longer exist in the project.
- Offers an interactive CLI prompt to confirm additions and removals.

### `dodo upgrade`
- Combines `dodo update` and `dodo build` in a single command.
- Ensures that workflows are fully up-to-date with the latest project changes.

### `dodo clean [tool]`
- Removes a specific tool (like `ruff`, `clippy`, etc.) from CI/CD while keeping its config file intact.
- Running without arguments removes tools not backed by existing config files.
- External integrations such as security checks remain untouched.

---

## 🛠 Configuration Files:

### `dodo.toml`
- Incrementally built during `dodo init` and `dodo sync`.
- Defines project-specific CI/CD settings based on detected tools.
- Updated only when new configurations are detected.

### `dodo.lock`
- Stores resolved tool versions, workflow template sources, and config file paths.
- Ensures that workflows remain consistent over time.
- Built incrementally to maintain efficiency.

---

Dodo keeps things **fast and efficient** by working incrementally—only scanning and updating what's necessary. No redundant processing, no unnecessary slowdowns. Whether you're setting up workflows for the first time or keeping them updated, Dodo ensures everything runs smoothly with minimal effort.

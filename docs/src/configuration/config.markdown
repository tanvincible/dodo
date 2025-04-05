# `dodo.toml` Configuration

`dodo.toml` is the configuration file for Dodo. It lets you customize your CI/CD workflows, defining build steps, linting, testing, deployment, and more in a simple, readable format.

## Default Configuration (`dodo init`)

Running `dodo init` creates a default `dodo.toml` file. Here’s a basic example:

```toml
[project]
name = "my-awesome-project"
language = "your-language"
ci_provider = "github-actions"

[ci]
auto_update = true
workflow_file = ".github/workflows/ci.yml"
branch_protection = true

[build]
enabled = true
tools = ["build_tool"]
build_tool = { version = "x.x.x", options = ["option1", "option2"] }

[lint]
enabled = true
tools = ["lint_tool"]
lint_tool = { version = "x.x.x", args = "--your-args" }

[test]
enabled = true
tools = ["test_tool"]
test_tool = { args = "--your-args" }

[deploy]
enabled = false
tools = ["deploy_tool"]
deploy_tool = { image = "your-registry/your-project", tag = "latest", branch = "deploy-branch" }

[env]
VAR_1 = "value1"
VAR_2 = "value2"

[matrix]
your_language = ["version1", "version2", "version-nightly"]

[cache]
enabled = true
paths = ["path/to/cache1", "path/to/cache2"]

[notifications]
enabled = true
tools = ["notify_tool"]
notify_tool = { webhook_url = "your-webhook-url", channel = "#your-channel" }

[hooks]
pre_commit = ["command1", "command2"]
post_merge = ["command3"]

[security]
dependabot = true
vulnerability_check = true

[docs]
enabled = true
tools = ["docs_tool"]
docs_tool = { version = "x.x.x", theme = "your-theme", output_dir = "docs/output" }

[plugins]
python = "https://github.com/kurajo/dodo-python"
rust = "https://github.com/kurajo/dodo-rust"

[ai]
enabled = true
model = "phi-3-mini-128k"
```

---

## Sections Overview

### `[project]`
Defines basic project settings.
- `name`: Project name.
- `language`: Programming language (`rust`, `python`, `node`, etc.).
- `ci_provider`: CI/CD provider (currently supports `github-actions`).

### `[ci]`
Manages global CI/CD settings.
- `auto_update`: Automatically updates workflows.
- `workflow_file`: Location of the generated workflow file.
- `branch_protection`: Enforces branch protection rules.

### `[build]`
Defines build settings.
- `enabled`: Enables/disables the build process.
- `tools`: List of build tools (e.g., cargo, go, setuptools).

### `[lint]`
Manages linting tools.
- `enabled`: Enables/disables linting.
- `tools`: List of linters (e.g., clippy, ruff, eslint).

### `[test]`
Configures testing tools.
- `enabled`: Runs tests if `true`.
- `tools`: Testing tools (e.g., cargo-test, pytest, go-test).

### `[deploy]`
Handles deployment settings.
- `enabled`: Enables/disables deployment.
- `tools`: Deployment options (e.g., Docker, GitHub Pages).

### `[env]`
Defines environment variables.

### `[matrix]`
Supports matrix builds for testing multiple versions.

### `[cache]`
Defines caching strategies to speed up builds.

### `[notifications]`
Sets up build status notifications (e.g., Slack, Discord).

### `[hooks]`
Runs automated scripts before or after specific actions.

### `[security]`
Configures security checks.

### `[docs]`
Defines documentation generation settings.

### `[plugins]`
Adds custom configurations via plugins.

---

## Example Configurations

### Rust Project

```toml
[project]
name = "rust-app"
language = "rust"
ci_provider = "github-actions"

[build]
enabled = true
tools = ["cargo"]
cargo = { version = "1.74.0", features = ["default"] }

[lint]
enabled = true
tools = ["clippy"]
clippy = { version = "latest", args = "--fix" }

[test]
enabled = true
tools = ["cargo-test"]
cargo-test = { args = "--release" }

[ci]
auto_update = true
workflow_file = ".github/workflows/ci.yml"
branch_protection = true

[plugins]
rust = "https://github.com/kurajo/dodo"
```

### Node.js Project

```toml
[project]
name = "node-app"
language = "node"
ci_provider = "github-actions"

[build]
enabled = true
tools = ["npm"]

[lint]
enabled = true
tools = ["eslint"]
eslint = { version = "latest", args = "--fix" }

[test]
enabled = true
tools = ["npm-test"]

[deploy]
enabled = true
tools = ["docker"]
docker = { image = "ghcr.io/username/node-app", tag = "latest" }

[custom_workflows]
pre_ci = ["echo 'Starting CI for Node.js app'"]
post_ci = ["echo 'CI process complete'"]

[env]
NODE_ENV = "production"

[notifications]
enabled = true
tools = ["slack"]
slack = { webhook_url = "https://hooks.slack.com/services/...", channel = "#ci-updates" }

[plugins]
python = "https://github.com/kurajo/dodo-python"
```

---

## How Dodo Uses `dodo.toml`

1. Parsing: Reads `dodo.toml` from the project root.
2. Workflow Generation: Creates `.github/workflows/ci.yml` based on your settings.
3. Automation: Reduces manual YAML editing and ensures consistency.

---

## Get Involved

Have suggestions or feature requests? Open an issue or contribute at [GitHub](https://github.com/tanvincible/dodo).

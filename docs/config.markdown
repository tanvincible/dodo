---
layout: page
title: Configuration
permalink: /config/
---

# `dodo.toml` Configuration

`dodo.toml` is the configuration file for Dodo. It allows you to fully customize your CI/CD workflows by defining build steps, linting, testing, deployment, and much more—all in a simple, human-readable format.

> **Tip:** If `dodo.toml` is missing from your project root, Dodo will fall back to default settings.

## **Default Configuration (`dodo init`)**

Running `dodo init` creates a default `dodo.toml` file. Below is a simplified example that you can use as a starting point:

```toml
# Example dodo.toml with placeholders

[project]
name = "my-awesome-project"         # Your project name
language = "your-language"          # Options: rust, python, node, go, etc.
ci_provider = "github-actions"      # Reserved for future CI/CD providers

[ci]
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
deploy_tool = { image = "your-registry/your-project", tag = "latest", branch = "deploy-branch", cname = "your-domain.com" }

[ci]
auto_update = true
workflow_file = "path/to/your/workflow.yml"
branch_protection = true

[custom_workflows]
pre_ci = ["echo 'Starting CI process'"]
post_ci = ["echo 'CI complete'"]

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
```

---

## **Sections Overview**

### **1. `[project]`**
Defines the overall project settings.
- **`name`**: Name of the project.
- **`language`**: Programming language (e.g., `rust`, `python`, `node`, `go`).
- **`ci_provider`**: CI/CD provider. (Currently supports `github-actions`.)

### **2. `[build]`**
Specifies how to build your project.
- **`enabled`**: Toggle build steps.
- **`tools`**: List of build tools (e.g., cargo, go, setuptools).
- Tool-specific configurations allow you to define versions, features, and arguments.

### **3. `[lint]`**
Configures static analysis tools.
- **`enabled`**: Enable or disable linting.
- **`tools`**: List of linters (e.g., clippy, ruff, golangci-lint) and their configurations.

### **4. `[test]`**
Manages your testing frameworks.
- **`enabled`**: Run tests if set to true.
- **`tools`**: Specify testing tools (e.g., cargo-test, pytest, go-test) with additional arguments.

### **5. `[deploy]`**
Handles deployment settings.
- **`enabled`**: Enable or disable deployment.
- **`tools`**: Deployment options such as Docker or GitHub Pages.

### **6. `[ci]`**
Contains global CI/CD settings.
- **`auto_update`**: Automatically update workflows.
- **`workflow_file`**: Location of the generated workflow file.
- **`branch_protection`**: Enforce branch protection rules.

### **7. `[custom_workflows]`**
Define project-specific pre- and post-CI actions.
- **`pre_ci`**: Commands to run before the CI process.
- **`post_ci`**: Commands to run after the CI process.

### **8. `[env]`**
Sets environment variables for your workflows.
- Configure environment-specific variables to tailor the runtime behavior.

### **9. `[matrix]`**
Supports matrix builds for testing across multiple versions.
- Lists versions for languages like Rust, Python, and Go.

### **10. `[cache]`**
Defines caching strategies to speed up builds.
- **`enabled`**: Toggle caching.
- **`paths`**: Specify directories to cache.

### **11. `[notifications]`**
Configures notifications (e.g., Slack, Discord) for build status.
- Enable notifications and set webhook URLs and channels.

### **12. `[hooks]`**
Automates pre- and post-commit/merge actions.
- **`pre_commit`**: Commands to execute before commits.
- **`post_merge`**: Commands to run after merging code.

### **13. `[security]`**
Implements security checks.
- **`dependabot`**: Enable dependency scanning.
- **`vulnerability_check`**: Run security audits.

### **14. `[docs]`**
Generates project documentation.
- **`enabled`**: Enable or disable docs generation.
- **`tools`**: Choose from tools like mdbook and sphinx with their respective settings.

---

## **Example Use Cases**

### **1. Basic Rust Project**

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

[deploy]
enabled = false

[ci]
auto_update = true
workflow_file = ".github/workflows/ci.yml"
branch_protection = true
```

### **2. Node.js Project with Custom Steps**

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
slack = { webhook_url = "https://hooks.slack.com/services/...", channel = "#ci-updates" }
```

---

## **How Dodo Uses `dodo.toml`**

1. **Parsing**: Dodo reads the `dodo.toml` file located in the project root.
2. **Workflow Generation**: Based on your configuration, Dodo generates a `.github/workflows/dodo.yml` file.
3. **Automation**: This setup automates your CI/CD pipeline, reducing manual YAML editing and ensuring consistency.

---

## **Get Involved**

Have suggestions or feature requests? Open an issue or contribute at [GitHub](https://github.com/tanvincible/dodo).

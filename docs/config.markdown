---
layout: page
title: Config
permalink: /config/
---

# `dodo.toml` Configuration  

`dodo.toml` is the configuration file for Dodo, allowing you to customize and automate GitHub Actions workflows for your project.  
This file defines the workflow structure, CI/CD steps, and custom settings in a simple, human-readable format.

## **Default Configuration (`dodo init`)**  

When you run `dodo init`, a default `dodo.toml` file is created:

```toml
[project]
name = "my-awesome-project"
language = "rust"  # Options: rust, python, node, go, etc.
ci_provider = "github-actions"  # Reserved for future CI/CD providers

[ci]
enable = true  # Set to false to disable workflow generation
workflow_name = "CI Pipeline"
branch = "main"
runs_on = "ubuntu-latest"

test = true      # Run tests
lint = false     # Run linter (if false, specify files below)
build = true     # Compile/build the project
deploy = false   # Deploy the project (set to true if applicable)

[custom]
extra_steps = ["echo 'Custom step executed'"]
```

---

## **Sections Overview**  

### **1. `[project]`**  
Defines general project settings.

| Key        | Type   | Description |
|------------|--------|-------------|
| `name`      | String | Name of the project. |
| `language`  | String | Programming language (e.g., `rust`, `python`, `node`). |
| `ci_provider` | String | CI/CD provider (only `github-actions` supported for now). |

---

### **2. `[ci]`**  
Controls workflow generation and execution settings.

| Key            | Type    | Description |
|---------------|--------|-------------|
| `enable`      | Boolean | Enables or disables CI workflow generation. |
| `workflow_name` | String | Name of the workflow. |
| `branch`      | String  | Branch to trigger CI/CD (e.g., `main`). |
| `runs_on`     | String  | Runner environment (`ubuntu-latest`, `macos-latest`, etc.). |
| `test`        | Boolean | Run tests (`true` to enable). |
| `lint`        | Boolean | Run linting checks. |
| `build`       | Boolean | Build the project. |
| `deploy`      | Boolean | Deploy the project (set to `true` if required). |

---

### **3. `[custom]`**  
Allows additional custom steps to be added.

| Key         | Type     | Description |
|-------------|----------|-------------|
| `extra_steps` | Array of Strings | List of extra shell commands to execute. |

---

## **Example Use Cases**  

### **1. Basic Rust Project**  
```toml
[project]
name = "rust-app"
language = "rust"

[ci]
enable = true
workflow_name = "Rust CI"
branch = "main"
runs_on = "ubuntu-latest"

test = true
lint = true
build = true
deploy = false
```

### **2. Node.js Project with Custom Steps**  
```toml
[project]
name = "node-app"
language = "node"

[ci]
enable = true
workflow_name = "Node.js CI/CD"
branch = "main"
runs_on = "ubuntu-latest"

test = true
lint = true
build = true
deploy = true

[custom]
extra_steps = ["npm install", "npm run test"]
```

---

## **How Dodo Uses `dodo.toml`**  
1. Reads the `dodo.toml` file in the project root.  
2. Generates a single `.github/workflows/dodo.yml` file based on the configuration.  
3. Automates workflow creation, reducing manual YAML editing.  

**Note:** If `dodo.toml` is missing, Dodo will fall back to default settings.  

---

## **Get Involved**  
Have suggestions or feature requests? Open an issue or contribute at [GitHub](https://github.com/tanvincible/dodo).

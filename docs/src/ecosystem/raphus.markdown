# raphus.io

**raphus.io** is the official CI/CD workflow registry for Dodo. It provides a structured repository of standardized GitHub Actions workflow templates and plugins that enable Dodo to generate and manage CI/CD pipelines reliably, consistently, and locally — without relying on cloud AI.

This documentation covers the current architecture, integration flow with Dodo, usage recommendations, and contribution guidelines.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Integration with Dodo](#integration-with-dodo)
- [Usage Guidelines](#usage-guidelines)
- [Frequently Asked Questions](#frequently-asked-questions)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License & Support](#license--support)

---

## Overview

**raphus.io** is a versioned, community-driven registry of workflow templates and plugins that power Dodo's local generation and management of GitHub Actions pipelines. Inspired by platforms like `crates.io`, it aims to be the definitive source for scalable, high-quality, and reproducible workflows.

### Key Features

- **Registry for Templates & Plugins:** Hosts reusable, customizable workflow templates and plugins.
- **Local AI Parsing:** Leverages **Phi-3 Mini 128K**, running locally, to extract context-aware details from project environments.
- **Template System:** Maps languages and frameworks to predefined workflows using the [raphus.io-index](https://github.com/dodomatic/raphus.io-index).
- **Plugin System:** Enables logic extension (e.g., environment setup, secrets handling) in a modular, reusable way.
- **No Cloud Dependency:** All processing is performed locally; no external AI or cloud inference APIs are involved.

---

## Architecture

### 1. Template System

- Templates are structured YAML files with placeholders (`{{ }}`) for project-specific substitutions.
- Templates are indexed using the `raphus.io-index` repository.
- Each template:
  - Supports **semantic versioning**
  - Includes metadata such as supported languages, frameworks, and toolchains
  - Can declare **optional plugin hooks** for pre-/post-generation logic

### 2. Plugin System

- Plugins are executable logic components (e.g., environment validators, version matchers, secret injectors).
- Hosted in the same registry as templates.
- Can be:
  - **Native (Rust) binaries**
  - **Scripts** or **WASM modules**
- Executed **locally** as part of Dodo’s `build` pipeline.

### 3. Local AI Integration

- Phi-3 Mini 128K is used to:
  - Extract metadata from source files (e.g., detect build systems, testing tools, or frameworks)
  - Assist in template selection and plugin orchestration
- All inference is performed **locally** with no cloud access.

---

## Integration with Dodo

### How raphus.io Fits into Dodo’s Workflow

1. **Initialization (`dodo init`)**
   - Dodo scans the project environment.
   - Phi-3 Mini 128K extracts relevant information.
   - Dodo fetches a suitable template and plugin set from `raphus.io` using `raphus.io-index`.

2. **Configuration (`dodo.toml`)**
   - The user configures build, lint, test, and deploy steps.
   - This config customizes template rendering.

3. **Build (`dodo build`)**
   - Templates are rendered locally using the settings in `dodo.toml`.
   - Plugins are executed if needed to adjust or verify behavior.
   - Final CI/CD workflow is saved in `.github/workflows/ci.yml`.

4. **(Optional) Auto-Update**
   - Users can enable workflow and plugin version auto-updates using `dodo.toml`.

---

## Usage Guidelines

### Getting Started

1. **Run Initialization**
   ```bash
   dodo init
   ```
   This creates a `dodo.toml` file and fetches a recommended template from `raphus.io`.

2. **Configure Your Project**
   - Edit `dodo.toml` to define:
     - Build tools
     - Lint/test commands
     - Deployment configuration
     - Plugin preferences

3. **Generate Your Workflow**
   ```bash
   dodo build
   ```
   This generates `.github/workflows/ci.yml` using templates and plugins from raphus.io.

4. **Commit & Push**
   - Commit your `.github/workflows/` directory to your VCS.
   - Your CI/CD pipeline is now live.

---

## Frequently Asked Questions

### Q: Is raphus.io cloud-based?

**No.** All processing is done **locally**. Phi-3 Mini 128K is bundled with Dodo and used for inference on your machine.

### Q: How are templates selected?

Templates are chosen using:
- Language/framework mappings from `raphus.io-index`
- Metadata extracted by Phi-3 from your codebase
- Overrides or hints in `dodo.toml`

### Q: Can I override templates or plugins?

Yes. You can:
- Provide a local template path in `dodo.toml`
- Override plugin behavior by specifying local versions
- Disable plugins with `plugins.disabled = [...]`

### Q: Will raphus.io become decentralized or blockchain-based?

Not in the MVP. Decentralization may be explored later if scaling demands it.

---

## Troubleshooting

| Problem                     | Solution                                                                 |
|----------------------------|--------------------------------------------------------------------------|
| No workflow generated      | Check for errors in `dodo.toml`. Verify your project type is supported. |
| Network issues             | raphus.io is only used for downloading templates/plugins — retry later. |
| Template does not apply    | Try updating Dodo or specifying a fallback template in `dodo.toml`.      |
| Plugin fails to run        | Ensure local plugin dependencies are met (e.g., `wasmtime` if using WASM).|

---

## Contributing

We welcome contributions to raphus.io!

1. **Fork** the repository
2. **Add your template/plugin**
   - Follow template guidelines in `TEMPLATE_GUIDELINES.md`
   - Follow plugin interface in `PLUGIN_GUIDELINES.md`
3. **Submit a Pull Request**
   - Include metadata (version, language tags, etc.)
   - Add a test project if possible

---

## License & Support

**raphus.io** is licensed under the [Elastic License 2.0 (ELv2)](https://www.elastic.co/licensing/elastic-license).

You **may**:
- Use, modify, and distribute the source freely for personal or internal use

You **may not**:
- Host raphus.io as a service or provide managed access to it
- Remove or obscure copyright
- Circumvent license enforcement, if any

For more details, read the full [Elastic License 2.0](https://www.elastic.co/licensing/elastic-license).

---

Happy building! 🚀  
Questions or feedback? Join our GitHub Discussions or file an issue.

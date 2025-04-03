# raphus.io

raphus.io is the official CI/CD workflow registry for Dodo. It centralizes predefined GitHub Actions workflows so that Dodo can generate and manage CI/CD pipelines in a consistent, reliable, and scalable manner.

This documentation covers the architecture, integration process, usage scenarios, and contribution guidelines for raphus.io.

## Table of Contents

- [Overview](#overview)
- [Architecture & Workflow Lifecycle](#architecture--workflow-lifecycle)
- [Integration with Dodo](#integration-with-dodo)
- [Usage Guidelines](#usage-guidelines)
- [Frequently Asked Questions](#frequently-asked-questions)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License & Support](#license--support)

## Overview

raphus.io acts as a registry that hosts community-driven CI/CD workflow templates for various programming languages and frameworks. Its primary goals are to:

- **Standardize Workflows:** Offer a centralized, versioned collection of tested GitHub Actions workflows.
- **Automate Integration:** Enable Dodo to dynamically adjust workflow templates based on a user’s project configuration.
- **Encourage Collaboration:** Allow the community to contribute new workflows and improvements, similar to how crates.io fosters the sharing of Rust libraries.

## Architecture & Workflow Lifecycle

### 1. Template Repository

- **Centralized Storage:** raphus.io hosts a collection of workflow templates organized by language and framework.
- **Versioning:** Each template is versioned to ensure reproducibility and stability. Users can select specific versions to match their project needs.
- **Index File:** A mapping file (e.g., `template.json`) acts as the index for all available templates. It maps a project’s language and configuration to the appropriate workflow template.

### 2. Workflow Generation Process

1. **Initialization:**
   - When a user runs `dodo init`, Dodo reads the project's `dodo.toml` configuration.
   - The language and other settings are used to determine the appropriate workflow template.

2. **Template Fetching:**
   - Dodo queries raphus.io (via the mapping file) to fetch the corresponding template.
   - The registry supports multiple languages and frameworks, ensuring that the right template is retrieved for each project type.

3. **Dynamic Adjustment:**
   - The fetched template contains placeholders for various settings (e.g., build tools, testing commands).
   - Dodo replaces these placeholders with values defined in the user’s `dodo.toml`, tailoring the workflow to the specific project.

4. **Workflow Generation:**
   - The adjusted workflow is saved as `.github/workflows/ci.yml` in the user’s repository.
   - This generated file becomes the active CI/CD configuration for the project.

### 3. Continuous Updates

- **Auto-Update:** With auto-update enabled (as configured in `dodo.toml`), Dodo can check for newer versions of templates on raphus.io.
- **Branch Protection:** The generated workflows can include branch protection settings to ensure safe deployments and testing.

## Integration with Dodo

### How Dodo Uses raphus.io

- **Mapping via `template.json`:** Dodo uses a JSON-based mapping file from raphus.io that correlates programming languages and project configurations with available workflow templates.
- **Customization:** After fetching a template, Dodo dynamically modifies it based on the user's `dodo.toml` settings. This includes updating environment variables, tool versions, and custom commands.
- **Deployment:** The final, customized workflow is automatically placed in the repository under `.github/workflows/ci.yml`.

### When to Use raphus.io

- **New Projects:** Start with a default workflow by running `dodo init`, which immediately fetches a suitable template from raphus.io.
- **Upgrading Pipelines:** As new templates or improvements are added to raphus.io, projects can update their CI/CD pipelines without rewriting workflow files from scratch.
- **Multi-Language Projects:** Benefit from a unified, centralized registry that supports various languages and frameworks.

## Usage Guidelines

### Setting Up

1. **Initialize Your Project:**
   - Run `dodo init` to create a default `dodo.toml` file. Adjust the configuration as needed.

2. **Customize Your Configuration:**
   - Edit `dodo.toml` to specify build tools, linting, testing, and deployment options.
   - The configuration parameters are used to dynamically adjust the workflow template fetched from raphus.io.

3. **Commit & Deploy:**
   - Once your configuration is set, commit the generated `.github/workflows/ci.yml` to trigger the CI/CD process.

### Best Practices

- **Keep Configurations Updated:** Regularly review and update your `dodo.toml` to take advantage of improvements in workflow templates.
- **Leverage Auto-Update:** Enable auto-update to always use the latest approved templates from raphus.io.
- **Version Control:** Use versioned templates to ensure reproducibility across different project stages.


## Frequently Asked Questions

### Q: What happens if my project’s language is not supported?

A: If no matching template is found for your language, Dodo will notify you with a prompt. You can then either add a custom template or contribute a new workflow to raphus.io.

### Q: Can I customize the fetched templates further?

A: Yes, after the template is fetched, Dodo replaces placeholders based on your `dodo.toml`. You can also extend your configuration with additional custom commands in the `[custom_workflows]` section.

### Q: How do version updates work?

A: Dodo checks the version of the template against the one specified (or defaulted) in your configuration. When a new version is available on raphus.io, Dodo can update the workflow automatically if auto-update is enabled.

## Troubleshooting

- **Workflow Fails to Generate:**  
  Check that your `dodo.toml` is correctly configured and that the mapping in `template.json` includes your project’s language.

- **Template Mismatch:**  
  Ensure that the version of the template being fetched is compatible with your configuration settings. Refer to the version history on raphus.io for details.

- **Connection Issues:**  
  If Dodo cannot reach raphus.io, verify your network settings and check for any service outages on the raphus.io status page.

## Contributing

Contributions are vital for making raphus.io a robust resource. To contribute:

- **Review Guidelines:** See our [CONTRIBUTING.md](../CONTRIBUTING.md) for details on coding standards and template quality.
- **Submit Pull Requests:** Propose new templates or improvements to existing ones.
- **Community Engagement:** Join our discussion forums and GitHub issues to collaborate with other contributors.

## License & Support

raphus.io is released under the [MIT License](../LICENSE).  
For support, report issues on our [GitHub Issues page](https://github.com/dodomatic/raphus.io/).

---

Happy automating your CI/CD workflows with raphus.io and Dodo!

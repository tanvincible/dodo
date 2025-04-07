# Overview

Kurajo is an integrated suite of tools designed to streamline CI/CD workflow management for developers.  
The system is composed of several interlocking components that work together to automate the creation, validation, publishing, and maintenance of workflow templates. 

Here’s a high-level view of each component and how they interact:

## 1. **Dodo CLI Tool**  
- **Purpose:**  
  Dodo is the command-line interface that developers interact with. Its primary functions include initializing project configurations, generating CI/CD workflows, and managing updates.  

- See [commands](../configuration/commands.markdown) and [configs](../configuration/config.markdown) for additional information. 

## 2. **raphus.io Registry**

- **Purpose:**  
  `raphus.io` is the centralized, community-driven registry for CI/CD workflow templates, plugins, and AI extensions. Think of it as a “marketplace” for composable automation logic—similar to how `crates.io` serves Rust packages.

- **Structure:**  
  - **Internal Templates:** Pre-vetted, high-quality workflow templates for major languages (e.g., Rust, Python, Go, Node.js), maintained by the core Dodomatic team.
  - **Plugin System:** Raphus.io will support a wide range of plugins—not just workflow templates, but also integrations for alternative CI/CD tools, locally hosted AI models, and even custom template transformers.
  - **raphus.io-index:**  
    A dedicated repository (`raphus.io-index`) acts as the authoritative index for all templates and plugins. It contains version information, metadata, descriptions, and compatibility tags.
  
- **Future Vision:**  
  As the ecosystem matures, `raphus.io` will evolve into a full plugin platform. Key features include:
  - Fine-grained search and filtering
  - Version pinning and upgrade paths
  - Quality-gated community contributions
  - Support for AI plugin discovery and updates
  - Toolchain-specific extensions (e.g., Bazel, Nix, custom GitLab runners)

`raphus.io` ensures that Dodo can stay modular, adaptable, and open to a wide range of automation setups—from basic GitHub Actions to advanced, custom pipelines and local model integrations.

## 3. **Test Repositories (dodo-tests & raphus-tests)**
- **Purpose:**  
  Dedicated repositories that serve as automated test beds for verifying that generated workflows run correctly in real GitHub Actions environments.
- **Workflow:**  
  - When a PR is submitted to either the Dodo or raphus.io repositories, a bot (Dox) creates a corresponding test branch in the test repo.
  - The generated workflow is committed and executed using GitHub Actions and locally with tools like `act`.
  - The bot monitors test results and posts feedback (success or detailed error logs) on the original PR.
  - Test branches are retained for debugging and then cleaned up after a set period.

## 4. **Automation & Bot (Dox)**
- **Purpose:**  
  Dox is the automation bot responsible for managing test workflows, verifying PRs, and handling routine maintenance tasks. It ensures that any workflow changes are validated before being merged.
- **Key Functions:**  
  - Create trigger commits and test branches for new or updated workflows.
  - Run CI/CD tests in a sandboxed environment.
  - Post automated feedback on PRs regarding workflow success or failure.
  - Clean up stale test branches after PRs are merged or closed.

## 5. **Integration & Workflow**
- **Initialization:**  
  A developer starts by running `dodo init` in their project, which auto-generates a configuration file (`dodo.toml`) based on the detected project structure.
- **Workflow Generation:**  
  When `dodo build` is executed, Dodo pulls the base workflow template from `raphus.io`, adapts it to the project-specific details (e.g., selecting the right test, lint, build, and deploy commands), and writes the final workflow to the repository's `.github/workflows/` folder.
- **Continuous Updates:**  
  `dodo update` periodically checks for newer versions of the actions used within the workflow, ensuring that the CI/CD process stays current.
- **External Contributions:**  
  In later phases, additional workflows can be added to raphus.io through a controlled publishing process (using `dodo publish`), but for the MVP, contributions are limited to issues and PRs to ensure stability.

## 6. **Scalability & Future Enhancements**
- **Modular Design:**  
  Each component is designed to work independently but also integrate seamlessly. This allows the system to scale as more templates and languages are added.
- **AI Integration (Future Phase):**  
  Future enhancements might include AI-driven optimization for workflow generation and intelligent debugging of CI/CD pipelines.
- **Community-Driven Growth:**  
  Once the core system is stable, Dodomatic plans to open up community publishing to make raphus.io a thriving marketplace of CI/CD templates.

---

## Conclusion

From a developer’s point of view, Kurajo's architecture is built to automate and simplify the often tedious process of managing CI/CD workflows. The integration of Dodo, raphus.io, automated testing repositories, and an intelligent bot (Dox) ensures that workflows are not only generated dynamically but are also kept up-to-date and thoroughly validated. 
This modular, scalable design sets the stage for long-term growth, community involvement, and a robust, open-source ecosystem that will empower developers for years to come.

Feel free to dive into each component’s documentation for more detailed insights. 

Happy coding!
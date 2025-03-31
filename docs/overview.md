---
layout: page
title: Overview
permalink: /overview/
---

# Overview

Raphus Systems is an integrated suite of tools designed to streamline CI/CD workflow management for developers.  
The system is composed of several interlocking components that work together to automate the creation, validation, publishing, and maintenance of workflow templates. 

Here’s a high-level view of each component and how they interact:

## 1. **Dodo CLI Tool**  
- **Purpose:**  
  Dodo is the command-line interface that developers interact with. Its primary functions include initializing project configurations, generating CI/CD workflows, and managing updates.  

- See [commands](commands.markdown) for key commands and configs. 

## 2. **raphus.io Registry**
- **Purpose:**  
  raphus.io is the centralized, community-driven registry for CI/CD workflow templates. Think of it as a “marketplace” for workflow templates, much like crates.io is for Rust packages.
- **Structure:**  
  - **Internal Templates:** Pre-vetted, curated workflows for major languages (e.g., Rust, Python, Go, Node.js) that are maintained by Dodomatic’s core team.
  - **Metadata Index:** A global index (e.g., `template.json`) that maps languages to their respective templates, including versioning information, tags, and descriptions.
- **Future Vision:**  
  As the ecosystem grows, raphus.io will support advanced features like robust search, versioning, and community publishing (initially gated to ensure quality).

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
  When `dodo build` is executed, Dodo pulls the base workflow template from raphus.io, adapts it to the project-specific details (e.g., selecting the right test, lint, build, and deploy commands), and writes the final workflow to the repository.
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

From a developer’s point of view, Dodomatic’s architecture is built to automate and simplify the often tedious process of managing CI/CD workflows. The integration of Dodo, raphus.io, automated testing repositories, and an intelligent bot (Dox) ensures that workflows are not only generated dynamically but are also kept up-to-date and thoroughly validated. 
This modular, scalable design sets the stage for long-term growth, community involvement, and a robust, open-source ecosystem that will empower developers for years to come.

Feel free to dive into each component’s documentation for more detailed insights. Happy coding!
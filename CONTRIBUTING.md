# Contributing to Dodo

Welcome to Dodo!  
We're thrilled that you're interested in contributing.  
Dodo is an AI-powered CI/CD automation tool that ensures consistency and efficiency in workflow generation. We value inclusivity, respect, and helpfulness in our community. This guide will help you get started and maintain high-quality contributions.

## How to Contribute

### 1. Reporting Issues
If you encounter a bug or have a feature request, please open an issue in our GitHub repository. Include:
- A clear and descriptive title.
- Steps to reproduce the issue.
- Expected vs. actual behavior.
- Relevant logs or screenshots.

### 2. Submitting Code Changes
1. **Fork the Repository:** Click the fork button on GitHub and clone your fork locally.
2. **Create a New Branch:** Name it something descriptive, e.g., `fix-cache-bug` or `add-new-linter`.
3. **Write Code & Tests:** Follow our coding guidelines (see below) and include tests where applicable.
4. **Run Tests Locally:** Ensure your changes pass the existing test suite.
5. **Open a Pull Request (PR):** Include a clear description of your changes and link any related issues.

## Coding Guidelines
To maintain a consistent codebase and high quality, please follow these guidelines:

- **Code Style:**
  - Write clean, readable code with proper indentation and spacing.
  - Use meaningful variable and function names.
  - Follow [Rust's official style guidelines](https://rust-lang.github.io/rustfmt/) for core implementation, and adhere to similar best practices for other languages.
  
- **Documentation:**
  - Document your functions, modules, and any non-obvious code with comments.
  - Update or add documentation in the relevant Markdown files if your change affects user behavior.
  
- **Testing:**
  - Write tests for new features or bug fixes.
  - Ensure that your code passes the entire test suite before submission.
  - Use descriptive test names and assertions to clearly indicate what is being verified.
  
- **Commit Messages:**
  - Write clear, concise commit messages that describe the purpose of the change.
  - Reference issue numbers where applicable (e.g., `Fixes #123`).

- **Modularity and Reusability:**
  - Keep functions and modules focused on a single task.
  - Avoid duplicating code—refactor common functionality into reusable components.

- **Error Handling:**
  - Handle errors gracefully and provide meaningful messages.
  - Use Rust’s error handling mechanisms (e.g., `Result` and `Option`) effectively, or equivalent constructs in other languages.

- **Performance Considerations:**
  - Strive for efficient code, especially since Dodo is designed to be blazing fast.
  - Avoid unnecessary I/O operations and prefer asynchronous or parallel processing where appropriate.

- **Security Practices:**
  - Do not hardcode sensitive information or credentials.
  - Validate inputs and ensure that configuration files are parsed safely.
  - Follow best practices for dependency management and version locking.

## Community Guidelines
- **Be Respectful:** Treat everyone with kindness and patience.
- **Stay Helpful:** Offer guidance to new contributors and engage constructively in discussions.
- **Keep It Inclusive:** We welcome contributions from everyone regardless of background. Diversity makes our community stronger.

## Getting Help
If you're unsure about anything, please ask in a GitHub issue or join our discussion thread.  
We're here to help!

## License
By contributing to Dodo, you agree that your contributions will be licensed under the [MIT License](LICENSE).

Happy coding, and thank you for helping shape the future of CI/CD automation! 🚀

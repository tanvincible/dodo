# Dodo

Dodo is a CLI tool that automates the creation and optimization of GitHub Actions workflows. Instead of manually writing `.github/workflows/*.yml` files, Dodo generates, validates, and updates them based on project structure and user configurations.

## Features (Planned)
- Automatic workflow generation based on best practices.
- Fetch and update actions from GitHub Actions Marketplace.
- Adapts workflows based on language and repository setup.
- Fast and reliable, built in Rust.
- Configurable via `dodo.toml`.

## Getting Started (Not Implemented Yet)

```sh
# Install Dodo (Coming Soon)
cargo install dodo-cli

# Initialize a configuration file
dodo init

# Generate workflows
dodo gen
```

## Project Status
Dodo is in early development (only 3 commits in). Contributions, feedback, and ideas are welcome.

## License

Dodo is open-source and licensed under the MIT License. Every contribution is considered to be under MIT unless stated otherwise.  
See the [LICENSE](LICENSE) file for more details.

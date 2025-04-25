# Frequently Asked Questions

## raphus.io

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

---
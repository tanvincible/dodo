# Troubleshooting

## raphus.io

| Problem                     | Solution                                                                 |
|----------------------------|--------------------------------------------------------------------------|
| No workflow generated      | Check for errors in `dodo.toml`. Verify your project type is supported. |
| Network issues             | raphus.io is only used for downloading templates/plugins — retry later. |
| Template does not apply    | Try updating Dodo or specifying a fallback template in `dodo.toml`.      |
| Plugin fails to run        | Ensure local plugin dependencies are met (e.g., `wasmtime` if using WASM).|

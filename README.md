# Zed-like GPUI Workspace

A Rust workspace using Zed's `gpui` crate with release/cherry-pick infrastructure.

## Build

```bash
cargo check --workspace
cargo build --release --bin app
```

## Release

```bash
script/trigger-release stable
```

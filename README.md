<h1 align="center">
  <a href="https://rewire.run/">
    <img alt="banner" src="https://github.com/user-attachments/assets/4859413d-89b2-424c-a378-8a15260de384">
  </a>
</h1>

<p align="center">
  <a href="https://github.com/rewire-run/rewire-extras/actions/workflows/ci.yaml">
    <img alt="CI" src="https://github.com/rewire-run/rewire-extras/actions/workflows/ci.yaml/badge.svg">
  </a>
  <a href="https://github.com/rewire-run/rewire-extras/blob/main/LICENSE">
    <img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-blue">
  </a>
  <a href="https://pixi.sh">
    <img alt="Powered by" src="https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/prefix-dev/pixi/main/assets/badge/v0.json">
  </a>
</p>

Shared types for the [Rewire](https://rewire.run) ecosystem. Provides custom
[Rerun](https://rerun.io) archetypes used by both the bridge and the viewer.

## Types

- **`ROS2TopicInfo`** — custom Rerun archetype for ROS 2 topic metadata (name, type, publisher/subscriber counts)
- **`ROS2NodeInfo`** — custom Rerun archetype for ROS 2 node metadata (name, pub/sub counts, transport)
- **`HeartbeatTracker`** — tracks bridge connection status via heartbeat timestamps

## Build

Requires Rust 1.82+.

```bash
cargo build
cargo test
```

Or with [pixi](https://pixi.sh):

```bash
pixi run sanity   # check + fmt + lint + test
```

## Dependencies

- [rerun](https://github.com/rerun-io/rerun) v0.30 — visualization framework
- [re_types_core](https://github.com/rerun-io/rerun) v0.30 — component serialization

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

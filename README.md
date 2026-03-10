# ros2env

[![CI](https://github.com/alvgaona/ros2env/actions/workflows/ci.yml/badge.svg)](https://github.com/alvgaona/ros2env/actions/workflows/ci.yml)
[![Release](https://github.com/alvgaona/ros2env/actions/workflows/release.yml/badge.svg)](https://github.com/alvgaona/ros2env/actions/workflows/release.yml)
[![crates.io](https://img.shields.io/crates/v/ros2env)](https://crates.io/crates/ros2env)
[![prefix.dev](https://img.shields.io/badge/prefix.dev-ros2env-yellow)](https://prefix.dev/channels/ros2env)

A ROS 2 distribution environment manager for [Pixi](https://pixi.sh) global installations.

Switch between multiple ROS 2 distributions (Humble, Jazzy, Rolling, etc.) installed via Pixi with a single
command — no more manually sourcing setup scripts or juggling environment variables.

## Why ros2env?

- **Install once, use everywhere**: A single Pixi global installation of each ROS 2 distribution is shared across
  all your workspaces. Without ros2env, every Pixi workspace that depends on ROS 2 downloads and stores its own
  copy — easily 3–5 GB per distro. With ros2env, you pay that cost once.
- **Multiple distros, one machine**: Install and manage several ROS 2 distributions side by side without conflicts.
- **Clean environment switching**: Activating a distribution strips stale paths and variables from a previously
  active distro, so you always get a clean environment.
- **No system ROS install required**: Pixi handles the installation; ros2env handles the environment.
  Works on any Linux or macOS system without needing `apt` or the official ROS 2 installer.
- **Shell integration**: A lightweight shell function wraps the CLI so `rosenv activate humble` just works,
  including tab completion.
- **Pixi workspace support**: Detects ROS distributions inside Pixi workspaces and can merge them with global
  installations for local colcon-based development.

## Installation

### From crates.io

```bash
cargo install ros2env
```

### Pre-built binaries (via cargo-binstall)

```bash
cargo binstall ros2env
```

### From prefix.dev

```bash
pixi global install -c ros2env rosenv
```

## Quick start

```bash
# 1. Set up symlinks from pixi installations to /opt/ros
rosenv setup

# 2. Add shell integration to your shell config
rosenv init zsh >> ~/.zshrc   # or bash
source ~/.zshrc

# 3. Activate a distribution
rosenv activate humble

# 4. Check status
rosenv status

# 5. Switch to another distribution
rosenv activate jazzy

# 6. Deactivate when done
rosenv deactivate
```

## Commands

| Command              | Description                                                    |
|----------------------|----------------------------------------------------------------|
| `setup`              | Auto-detect Pixi ROS installations and create symlinks         |
| `list`               | List available distributions (`--names-only`, `--short`)       |
| `status`             | Show the currently active distribution and environment details |
| `activate <distro>`  | Activate a ROS 2 distribution                                  |
| `deactivate`         | Deactivate the current distribution                            |
| `info <distro>`      | Show detailed info about a distribution                        |
| `init <shell>`       | Generate shell integration code (`zsh` or `bash`)              |
| `remove <distro>`    | Remove a distribution symlink                                  |
| `cleanup`            | Remove all distribution symlinks                               |
| `refresh`            | Re-scan Pixi installations and update symlinks                 |
| `doctor`             | Diagnose common issues with your setup                         |
| `pixi activate`      | Activate a ROS distribution inside a Pixi workspace            |
| `setup-guide`        | Open the Pixi + ROS 2 setup guide                              |

## Documentation

See the [documentation site](https://alvgaona.github.io/ros2env/) for the full getting started guide,
commands reference, and contributing guidelines.

## Build from source

```bash
cargo build --release
```

## License

MIT

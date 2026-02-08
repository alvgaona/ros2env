# ros2env - ROS 2 Distribution Manager

CLI tool for managing multiple ROS 2 distributions installed via pixi global.

**Binary:** `rosenv`  
**Package:** `ros2env`

## Features

- Auto-detect pixi ROS installations in `~/.pixi/envs/`
- Create and manage symlinks in `/opt/ros/`
- Switch between distributions with environment isolation
- Verify installation with diagnostic commands

## Overview

ROS 2 tools expect distributions at `/opt/ros/<distro>`. When using pixi global for ROS 2 installations, this tool creates the necessary symlinks and manages environment variables for switching between distributions.

## Installation

From source:

```bash
cargo build --release
cp target/release/rosenv ~/.local/bin/
```

Requires `~/.local/bin` in PATH.

## Usage

### Initial Setup

1. Install ROS distributions via pixi:
```bash
pixi global install --environment ros-humble-desktop -c robostack-staging ros-humble-desktop
```

2. Create `/opt/ros` with write permissions:
```bash
sudo mkdir -p /opt/ros
sudo chown $USER /opt/ros
```

3. Create symlinks:
```bash
rosenv setup
```

4. Add shell integration:
```bash
rosenv init zsh >> ~/.zshrc
source ~/.zshrc
```

5. Activate a distribution:
```bash
rosenv activate humble
```

## Commands

### Core Commands

```bash
rosenv setup         # Auto-detect and create symlinks
rosenv list          # List available distributions
rosenv status        # Show current active distribution
rosenv doctor        # Verify installation
rosenv setup-guide   # Open setup guide in browser
```

### Distribution Management

```bash
rosenv activate <distro>  # Activate a ROS distribution
rosenv deactivate         # Deactivate current distribution
rosenv info <distro>      # Show distribution details
rosenv remove <distro>    # Remove a symlink
rosenv cleanup            # Remove all symlinks
rosenv refresh            # Update all symlinks
```

### Shell Integration

```bash
rosenv init zsh      # Generate zsh integration code
rosenv init bash     # Generate bash integration code
```

## Implementation

1. Scans `~/.pixi/envs/` for directories matching `ros-*-*` pattern
2. Creates symlinks: `/opt/ros/<distro>` → `~/.pixi/envs/ros-<distro>-*`
3. Unsets ROS environment variables before sourcing new distribution
4. Sources distribution-specific setup scripts from `/opt/ros/<distro>`

## Architecture

```
/opt/ros/
├── humble -> /Users/you/.pixi/envs/ros-humble-desktop
└── jazzy  -> /Users/you/.pixi/envs/ros-jazzy-desktop

~/.pixi/envs/
├── ros-humble-desktop/  (3.3GB - actual installation)
└── ros-jazzy-desktop/   (3.3GB - actual installation)
```

## Common Operations

List distributions:
```bash
rosenv list
```

Switch distribution:
```bash
rosenv activate humble
rosenv activate jazzy
```

Deactivate:
```bash
rosenv deactivate
```

Add new distribution:
```bash
pixi global install --environment ros-iron -c robostack-staging ros-iron-desktop
rosenv refresh
```

Verify installation:
```bash
rosenv doctor
```

## Pixi Integration

rosenv manages symlinks and environment switching. Pixi handles package installation and updates.

## Shell Integration

The `rosenv init` command generates a shell function that wraps the binary:

- `rosenv activate <distro>` - Evaluates activation script in current shell
- `rosenv deactivate` - Evaluates deactivation script in current shell
- `rosenv status` - Checks current shell's `ROS_DISTRO` variable
- Other commands - Pass through to binary

This allows activation without manual `eval` commands.

## Environment Variables

When switching distributions, these variables are unset:
- `AMENT_PREFIX_PATH`, `CMAKE_PREFIX_PATH`, `COLCON_PREFIX_PATH`
- `PYTHONPATH`, `LD_LIBRARY_PATH`, `DYLD_LIBRARY_PATH`
- `PKG_CONFIG_PATH`
- `PATH` (filters `/opt/ros/*` entries)
- `ROS_DISTRO`, `ROS_VERSION`, `ROS_PYTHON_VERSION`

The new distribution's setup script is then sourced.

## Comparison

| Aspect | Traditional ROS 2 | rosenv + pixi |
|--------|------------------|---------------|
| Installation | apt/source build | pixi global |
| Path | /opt/ros/<distro> | /opt/ros/<distro> (symlink) |
| Switching | Manual source | rosenv activate |
| Environment cleanup | Manual | Automatic |
| Updates | apt upgrade | pixi global update |

## License

MIT

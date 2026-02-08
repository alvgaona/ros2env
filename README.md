# rosenv - ROS 2 Distribution Manager

A clean, simple CLI tool for managing multiple ROS 2 distributions on macOS (and Linux).

## Features

- 🔗 **Auto-detect** pixi ROS installations and create symlinks
- 🔄 **Easy switching** between ROS distributions
- 🧹 **Clean environment** management (no variable conflicts)
- 🔍 **Health checks** with `rosenv doctor`
- 📦 **Minimal dependencies** - just Rust and clap

## Why?

ROS 2's traditional installation expects `/opt/ros/<distro>` paths. When using pixi global to install ROS 2, you need symlinks to maintain compatibility with standard ROS 2 workflows. `rosenv` automates this and provides clean distribution switching.

## Installation

```bash
cd ~/git/rosenv
cargo build --release
cp target/release/rosenv ~/.local/bin/
```

Make sure `~/.local/bin` is in your PATH.

## Quick Start

### 1. View Setup Guide (Recommended for First-Time Users)

```bash
rosenv setup-guide
```

This shows comprehensive installation instructions, including how to install ROS 2 with pixi.

### 2. Setup Symlinks

After installing ROS distributions with pixi global:

```bash
pixi global install --environment ros-humble-desktop -c robostack-staging ros-humble-desktop
pixi global install --environment ros-jazzy-desktop -c robostack-staging ros-jazzy-desktop
```

Run setup to create symlinks:

```bash
rosenv setup
```

This will:
- Scan `~/.pixi/envs/` for ROS installations
- Create symlinks in `/opt/ros/` (you may need to create this directory first)
- Show next steps

### 3. Add Shell Integration

Generate shell integration code:

```bash
rosenv init zsh >> ~/.zshrc
source ~/.zshrc
```

### 4. Use rosenv

```bash
rosenv activate humble    # Switch to Humble
rosenv activate jazzy     # Switch to Jazzy
rosenv status            # Show current status
rosenv deactivate        # Deactivate ROS environment
```

## Commands

### Core Commands

```bash
rosenv setup         # Auto-detect and create symlinks
rosenv list          # List available distributions
rosenv status        # Show current active distribution
rosenv doctor        # Verify installation
rosenv setup-guide   # Show pixi installation guide
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

## How It Works

1. **Detection**: Scans `~/.pixi/envs/` for directories matching `ros-*-*` pattern
2. **Symlinks**: Creates `/opt/ros/<distro>` → `~/.pixi/envs/ros-<distro>-desktop`
3. **Switching**: Cleans old ROS environment variables before sourcing new distribution
4. **Zero Duplication**: Symlinks use negligible disk space, pixi manages actual installations

## Architecture

```
/opt/ros/
├── humble -> /Users/you/.pixi/envs/ros-humble-desktop
└── jazzy  -> /Users/you/.pixi/envs/ros-jazzy-desktop

~/.pixi/envs/
├── ros-humble-desktop/  (3.3GB - actual installation)
└── ros-jazzy-desktop/   (3.3GB - actual installation)
```

## Permissions Setup

`rosenv` needs `/opt/ros` to be writable. One-time setup:

```bash
sudo mkdir -p /opt/ros
sudo chown $USER /opt/ros
```

Then `rosenv` works without sudo.

## Examples

### Basic Usage

```bash
# Check what's available
rosenv list

# Show current distribution
rosenv status

# Switch to Humble
rosenv activate humble

# Switch to Jazzy
rosenv activate jazzy

# Deactivate ROS
rosenv deactivate
```

### After Installing a New Distribution

```bash
# Install with pixi
pixi global install --environment ros-iron -c robostack-staging ros-iron-desktop

# Update symlinks
rosenv refresh
```

### Troubleshooting

```bash
# Run diagnostics
rosenv doctor

# If symlinks are broken
rosenv cleanup
rosenv setup
```

### Information

```bash
# Show details about Humble
rosenv info humble

# List just names (for scripting)
rosenv list --names-only
```

## Integration with Pixi

`rosenv` complements pixi, it doesn't wrap it:

- **Pixi handles**: Installation, updates, package management
- **rosenv handles**: Symlink management, distribution switching

## Shell Integration

The `rosenv init` command generates a comprehensive shell function that intercepts key commands:

- **`rosenv activate <distro>`** - Transparently activates a distribution
- **`rosenv deactivate`** - Cleans up the ROS environment
- **`rosenv status`** - Shows current shell status accurately
- **Other commands** - Pass through to the binary

This provides a conda-like experience where:
1. Activation happens in the current shell (no manual `eval` needed)
2. Status accurately reflects the current shell environment
3. All environment cleanup is handled automatically

## Environment Variables

When switching, `rosenv` cleans these variables:
- `AMENT_PREFIX_PATH`
- `CMAKE_PREFIX_PATH`
- `COLCON_PREFIX_PATH`
- `PYTHONPATH`
- `LD_LIBRARY_PATH` / `DYLD_LIBRARY_PATH`
- `PKG_CONFIG_PATH`
- PATH (removes `/opt/ros/*` entries)

Then sources the new distribution cleanly.

## Differences from Traditional ROS 2

| Feature | Traditional ROS 2 | rosenv + pixi |
|---------|------------------|---------------|
| Installation | apt/source build | pixi global |
| Path | /opt/ros/<distro> | Same (via symlink) |
| Switching | Manual sourcing | `rosenv activate <distro>` |
| Cleanup | Manual | Automatic |
| Deactivation | Manual unset | `rosenv deactivate` |
| Updates | apt upgrade | pixi global update |
| macOS Support | Build from source | Binary packages |

## Contributing

This is a personal tool but improvements welcome:
- Better error messages
- Additional platforms
- Shell completion scripts
- Config file support

## License

MIT

## Credits

Built to solve the ROS 2 + pixi + macOS workflow challenges.

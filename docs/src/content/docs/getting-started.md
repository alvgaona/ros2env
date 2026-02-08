---
title: Getting Started
description: Install and configure rosenv for managing ROS 2 distributions
---

import { Tabs, TabItem, Aside } from '@astrojs/starlight/components';
import { VERSION } from '../../version.ts';

<Aside type="note">
This guide is for rosenv v{VERSION}. Check [releases](https://github.com/alvgaona/ros2env/releases) for updates.
</Aside>

## Prerequisites

### Install pixi

<Tabs>
  <TabItem label="macOS/Linux">

```bash
curl -fsSL https://pixi.sh/install.sh | bash
```

  </TabItem>
  <TabItem label="Homebrew (macOS)">

```bash
brew install pixi
```

  </TabItem>
</Tabs>

Verify installation:

```bash
pixi --version
```

### Setup /opt/ros Directory

Create the directory where rosenv will manage symlinks:

<Tabs>
  <TabItem label="macOS">

```bash
sudo mkdir -p /opt/ros
sudo chown $USER /opt/ros
```

  </TabItem>
  <TabItem label="Linux">

```bash
sudo mkdir -p /opt/ros
sudo chown $USER /opt/ros
```

  </TabItem>
</Tabs>

After this, rosenv works without sudo.

## Installing ROS 2 Distributions

Install ROS 2 distributions globally using pixi. Each distribution is installed to its own environment (~3.3GB each).

### Humble Hawksbill (LTS)

```bash
pixi global install --environment ros-humble-desktop \
  -c robostack-staging ros-humble-desktop
```

### Iron Irwini

```bash
pixi global install --environment ros-iron-desktop \
  -c robostack-staging ros-iron-desktop
```

### Jazzy Jalisco

```bash
pixi global install --environment ros-jazzy-desktop \
  -c robostack-staging ros-jazzy-desktop
```

### Rolling

```bash
pixi global install --environment ros-rolling-desktop \
  -c robostack-staging ros-rolling-desktop
```

<Aside>
Check [RoboStack](https://robostack.github.io/) for the latest available distributions.
</Aside>

## After Installation

### 1. Create Symlinks

After installing your first distribution:

```bash
rosenv setup
```

This will:
- Scan `~/.pixi/envs/` for ROS 2 installations
- Create symlinks in `/opt/ros/`
- Show what was configured

### 2. Add Shell Integration

Generate and add shell integration to your shell config:

<Tabs>
  <TabItem label="zsh">

```bash
rosenv init zsh >> ~/.zshrc
source ~/.zshrc
```

  </TabItem>
  <TabItem label="bash">

```bash
rosenv init bash >> ~/.bashrc
source ~/.bashrc
```

  </TabItem>
</Tabs>

### 3. Verify Installation

Check that everything is configured correctly:

```bash
rosenv doctor
```

This verifies:
- `/opt/ros` directory exists and is writable
- Symlinks are valid
- ROS 2 setup files are present
- Shell integration is configured

### 4. Activate a Distribution

```bash
rosenv activate humble
```

### 5. Test ROS 2

```bash
ros2 --version
ros2 topic list
```

## Managing Installations

### Update a Distribution

Update an existing ROS 2 distribution:

```bash
pixi global update ros-humble-desktop
rosenv refresh
```

### List Global Installations

See what pixi has installed:

```bash
pixi global list
```

### Add More Distributions

Install another distribution:

```bash
pixi global install --environment ros-jazzy-desktop \
  -c robostack-staging ros-jazzy-desktop
rosenv refresh
```

Switch between distributions:

```bash
rosenv activate jazzy
rosenv activate humble
```

### Remove a Distribution

Remove from rosenv:

```bash
rosenv remove humble
```

Remove from pixi:

```bash
pixi global remove ros-humble-desktop
```

## Disk Space

- **Each distribution:** ~3.3GB
- **Installation location:** `~/.pixi/envs/ros-<distro>-desktop/`
- **Symlinks:** Negligible space

Example with two distributions:

<Tabs>
  <TabItem label="macOS">

```
~/.pixi/envs/
├── ros-humble-desktop/  (~3.3GB)
└── ros-jazzy-desktop/   (~3.3GB)
Total: ~6.6GB

/opt/ros/
├── humble -> /Users/you/.pixi/envs/ros-humble-desktop
└── jazzy  -> /Users/you/.pixi/envs/ros-jazzy-desktop
Total: negligible
```

  </TabItem>
  <TabItem label="Linux">

```
~/.pixi/envs/
├── ros-humble-desktop/  (~3.3GB)
└── ros-jazzy-desktop/   (~3.3GB)
Total: ~6.6GB

/opt/ros/
├── humble -> /home/you/.pixi/envs/ros-humble-desktop
└── jazzy  -> /home/you/.pixi/envs/ros-jazzy-desktop
Total: negligible
```

  </TabItem>
</Tabs>

## Troubleshooting

### /opt/ros Not Writable

```bash
sudo chown $USER /opt/ros
```

### Symlinks Broken

```bash
rosenv cleanup
rosenv setup
```

### Shell Integration Not Working

<Tabs>
  <TabItem label="zsh">

```bash
rosenv init zsh >> ~/.zshrc
source ~/.zshrc

# Verify function exists
type rosenv  # Should show "rosenv is a shell function"
```

  </TabItem>
  <TabItem label="bash">

```bash
rosenv init bash >> ~/.bashrc
source ~/.bashrc

# Verify function exists
type rosenv  # Should show "rosenv is a shell function"
```

  </TabItem>
</Tabs>

### Distribution Not Found

Check if pixi installed it:

```bash
pixi global list | grep ros
```

Refresh symlinks:

```bash
rosenv refresh
```

List available distributions:

```bash
rosenv list
```

### ROS Commands Not Working

Verify activation:

```bash
echo $ROS_DISTRO  # Should show your distribution
which ros2        # Should show /opt/ros/<distro>/bin/ros2
```

Try manual reactivation:

```bash
rosenv deactivate
rosenv activate humble
```

Check setup files:

```bash
rosenv info humble
```

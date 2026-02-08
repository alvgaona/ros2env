---
title: Core Commands
description: Essential rosenv commands for setup and information
---

import { Tabs, TabItem, Aside } from '@astrojs/starlight/components';

## rosenv setup

Auto-detect pixi ROS installations and create symlinks.

### Usage

```bash
rosenv setup
```

### Description

Scans `~/.pixi/envs/` for directories matching the `ros-*-*` pattern, validates they contain ROS 2 setup files, and creates corresponding symlinks in `/opt/ros/`.

### Prerequisites

- `/opt/ros` must exist and be writable by current user
- At least one ROS 2 distribution installed via pixi global

### Example Output

```
Scanning ~/.pixi/envs for ROS 2 installations...

Found distributions:
  • ros-humble-desktop
  • ros-jazzy-desktop

Creating symlinks:
  ✓ /opt/ros/humble → ~/.pixi/envs/ros-humble-desktop
  ✓ /opt/ros/jazzy → ~/.pixi/envs/ros-jazzy-desktop

Setup complete!
```

<Aside type="tip">
Run `rosenv setup` after installing new ROS 2 distributions with pixi.
</Aside>

## rosenv list

List available ROS 2 distributions.

### Usage

```bash
rosenv list [--names-only] [--short]
```

### Flags

- `--names-only` - Output only distribution names, one per line
- `--short` - Output space-separated distribution names on single line

### Examples

<Tabs>
  <TabItem label="Default">

```bash
$ rosenv list
Available ROS distributions:
  * humble (active)
    jazzy
```

  </TabItem>
  <TabItem label="Names Only">

```bash
$ rosenv list --names-only
humble
jazzy
```

  </TabItem>
  <TabItem label="Short">

```bash
$ rosenv list --short
humble jazzy
```

  </TabItem>
</Tabs>

## rosenv status

Show current active distribution.

### Usage

```bash
rosenv status
```

### Description

Displays information about the currently active ROS 2 distribution, including environment variables and setup file locations.

### Example Output

When active:

```
ROS 2 humble is active

Environment:
  ROS_VERSION:       2
  ROS_DISTRO:        humble
  AMENT_PREFIX_PATH: /opt/ros/humble

Setup file:
  ✓ /opt/ros/humble/setup.zsh
```

When inactive:

```
No ROS 2 distribution active

Available distributions:
  - humble
  - jazzy

Activate: rosenv activate <distro>
```

## rosenv doctor

Verify installation and diagnose issues.

### Usage

```bash
rosenv doctor
```

### Description

Runs comprehensive diagnostics to verify:
- `/opt/ros` directory exists and is writable
- Symlinks are valid and point to existing directories
- Setup files are present
- Key directories (bin, lib, share) exist

### Example Output

```
Checking ROS 2 environment setup...

✓ /opt/ros directory exists
✓ /opt/ros is writable
✓ Found 2 distributions in /opt/ros

Distribution: humble
  ✓ Symlink valid
  ✓ Target exists: ~/.pixi/envs/ros-humble-desktop
  ✓ Setup files present
  ✓ Binary and library directories exist

Distribution: jazzy
  ✓ Symlink valid
  ✓ Target exists: ~/.pixi/envs/ros-jazzy-desktop
  ✓ Setup files present
  ✓ Binary and library directories exist

All checks passed!
```

### Common Issues

<Aside type="caution" title="Permission Denied">
If `/opt/ros` is not writable:

```bash
sudo chown $USER /opt/ros
```
</Aside>

<Aside type="caution" title="Broken Symlink">
If symlinks are broken:

```bash
rosenv cleanup
rosenv setup
```
</Aside>

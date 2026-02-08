---
title: Distribution Management
description: Commands for managing ROS 2 distributions
---

import { Tabs, TabItem, Aside } from '@astrojs/starlight/components';

## rosenv activate

Activate a ROS 2 distribution.

### Usage

```bash
rosenv activate <distro>
```

### Description

Activates the specified ROS 2 distribution in the current shell by:
1. Cleaning previous ROS environment variables
2. Setting `ROS_DISTRO`
3. Sourcing the distribution's setup script

<Aside type="note">
This command requires shell integration. See `rosenv init` for setup.
</Aside>

### Arguments

- `<distro>` - Distribution name (e.g., `humble`, `jazzy`)

### Examples

```bash
# Activate Humble
rosenv activate humble

# Activate Jazzy
rosenv activate jazzy
```

### Environment Variables Set

When activating, these variables are configured:
- `ROS_DISTRO` - Distribution name
- `ROS_VERSION` - ROS version (2)
- `AMENT_PREFIX_PATH` - Package search paths
- `CMAKE_PREFIX_PATH` - CMake package paths
- `PYTHONPATH` - Python module paths
- `PATH` - Binary paths
- Platform-specific library paths (`LD_LIBRARY_PATH` or `DYLD_LIBRARY_PATH`)

## rosenv deactivate

Deactivate the current ROS 2 environment.

### Usage

```bash
rosenv deactivate
```

### Description

Cleans up all ROS-related environment variables and removes `/opt/ros` entries from PATH.

<Aside type="note">
This command requires shell integration. See `rosenv init` for setup.
</Aside>

### Environment Variables Unset

- `ROS_DISTRO`, `ROS_VERSION`, `ROS_PYTHON_VERSION`
- `AMENT_PREFIX_PATH`, `CMAKE_PREFIX_PATH`, `COLCON_PREFIX_PATH`
- `PYTHONPATH`, `LD_LIBRARY_PATH`, `DYLD_LIBRARY_PATH`
- `PKG_CONFIG_PATH`
- PATH entries containing `/opt/ros/`

### Example

```bash
$ rosenv activate humble
✓ Switched to ROS 2 humble

$ echo $ROS_DISTRO
humble

$ rosenv deactivate
✓ Deactivated ROS 2 humble

$ echo $ROS_DISTRO

```

## rosenv info

Show detailed information about a distribution.

### Usage

```bash
rosenv info <distro>
```

### Description

Displays detailed information about a specific distribution including:
- Path and type (symlink or directory)
- Available setup files
- Key directories and entry counts

### Arguments

- `<distro>` - Distribution name

### Example Output

<Tabs>
  <TabItem label="macOS">

```bash
$ rosenv info humble
Distribution: humble
Path:         /opt/ros/humble
Type:         Symlink
Target:       /Users/you/.pixi/envs/ros-humble-desktop

Setup files:
  ✓ setup.bash
  ✓ setup.zsh
  ✓ setup.sh

Key directories:
  ✓ bin (245 entries)
  ✓ lib (1823 entries)
  ✓ share (398 entries)
  ✓ include (89 entries)
```

  </TabItem>
  <TabItem label="Linux">

```bash
$ rosenv info humble
Distribution: humble
Path:         /opt/ros/humble
Type:         Symlink
Target:       /home/you/.pixi/envs/ros-humble-desktop

Setup files:
  ✓ setup.bash
  ✓ setup.zsh
  ✓ setup.sh

Key directories:
  ✓ bin (245 entries)
  ✓ lib (1823 entries)
  ✓ share (398 entries)
  ✓ include (89 entries)
```

  </TabItem>
</Tabs>

## rosenv remove

Remove a distribution symlink.

### Usage

```bash
rosenv remove <distro>
```

### Description

Removes the symlink for a distribution from `/opt/ros/`. This does not remove the actual pixi installation.

<Aside type="caution">
This command prompts for confirmation before removing the symlink.
</Aside>

### Arguments

- `<distro>` - Distribution name

### Example

```bash
$ rosenv remove humble
Remove /opt/ros/humble? [y/N] y
✓ Removed /opt/ros/humble

Note: The pixi installation remains at:
  ~/.pixi/envs/ros-humble-desktop

To reinstall the symlink: rosenv setup
```

### Removing pixi Installation

To completely remove a distribution:

```bash
# First remove symlink
rosenv remove humble

# Then remove pixi installation
pixi global remove ros-humble-desktop
```

---
title: Utility Commands
description: Additional management and configuration commands
---

import { Tabs, TabItem, Aside } from '@astrojs/starlight/components';

## rosenv cleanup

Remove all distribution symlinks.

### Usage

```bash
rosenv cleanup
```

### Description

Removes all distribution symlinks from `/opt/ros/`. This does not affect the actual pixi installations.

<Aside type="caution">
This command prompts for confirmation before removing all symlinks.
</Aside>

### Example

```bash
$ rosenv cleanup
Found symlinks:
  - /opt/ros/humble
  - /opt/ros/jazzy

Remove all symlinks? [y/N] y
✓ Removed /opt/ros/humble
✓ Removed /opt/ros/jazzy

Cleanup complete.

Note: Pixi installations remain in ~/.pixi/envs/
To recreate symlinks: rosenv setup
```

## rosenv refresh

Update all symlinks.

### Usage

```bash
rosenv refresh
```

### Description

Scans for changes in pixi installations and updates symlinks accordingly. Useful after:
- Installing new distributions
- Updating existing distributions
- Moving pixi installations

### Example

```bash
$ rosenv refresh
Scanning for changes...

Existing symlinks:
  ✓ humble: up to date
  ✓ jazzy: up to date

New distributions found:
  + iron → ~/.pixi/envs/ros-iron-desktop

Creating symlinks:
  ✓ /opt/ros/iron → ~/.pixi/envs/ros-iron-desktop

Refresh complete.
```

<Aside type="tip">
Run `rosenv refresh` after installing new distributions with pixi.
</Aside>

## rosenv init

Generate shell integration code.

### Usage

```bash
rosenv init <shell>
```

### Description

Generates shell function code for integration with your shell. This enables transparent activation/deactivation without manual `eval` commands.

### Arguments

- `<shell>` - Shell type (`zsh` or `bash`)

### Examples

<Tabs>
  <TabItem label="zsh">

```bash
# Generate and append to config
rosenv init zsh >> ~/.zshrc

# Or preview the code
rosenv init zsh
```

  </TabItem>
  <TabItem label="bash">

```bash
# Generate and append to config
rosenv init bash >> ~/.bashrc

# Or preview the code
rosenv init bash
```

  </TabItem>
</Tabs>

### Generated Function

The shell function intercepts these commands:
- `rosenv activate <distro>` - Evaluates activation script in current shell
- `rosenv deactivate` - Evaluates deactivation script in current shell
- `rosenv status` - Checks current shell's `ROS_DISTRO` variable
- Other commands pass through to the binary

This provides a conda-like experience without manual `eval` commands.

## rosenv setup-guide

Open the setup guide in your browser.

### Usage

```bash
rosenv setup-guide
```

### Description

Opens the comprehensive setup guide on GitHub in your default browser.

<Tabs>
  <TabItem label="macOS">

Uses the `open` command to launch the default browser.

  </TabItem>
  <TabItem label="Linux">

Uses the `xdg-open` command to launch the default browser.

  </TabItem>
</Tabs>

### Example

```bash
$ rosenv setup-guide
Opening ROS 2 Setup Guide in your browser...

URL: https://github.com/alvgaona/ros2env/blob/main/SETUP_GUIDE.md

✓ Setup guide opened in your default browser
```

<Aside>
If the browser doesn't open automatically, the URL will be displayed for manual access.
</Aside>

## rosenv --help

Show help information.

### Usage

```bash
rosenv --help
rosenv <command> --help
```

### Description

Displays help information for rosenv or a specific command.

### Examples

```bash
# General help
rosenv --help

# Command-specific help
rosenv activate --help
rosenv list --help
```

## rosenv --version

Show version information.

### Usage

```bash
rosenv --version
```

### Description

Displays the current version of rosenv.

### Example

```bash
$ rosenv --version
rosenv 0.1.0
```

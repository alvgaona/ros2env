---
title: Utility Commands
description: Maintenance, configuration, and troubleshooting tools
---

import { Tabs, TabItem, Aside, Card, CardGrid, Steps } from '@astrojs/starlight/components';

Utility commands help you maintain, configure, and troubleshoot your rosenv setup. Use these tools for shell
integration, symlink maintenance, and system-wide operations.

---

## rosenv cleanup

**Remove all distribution symlinks**

Removes all ROS 2 distribution symlinks from `/opt/ros/`. This is useful for a clean slate or when troubleshooting
symlink issues.

### Usage

```bash
rosenv cleanup
```

### What It Does

<Steps>

1. **Scans `/opt/ros/`**

   Identifies all ROS 2 distribution symlinks

2. **Displays findings**

   Shows which symlinks will be removed

3. **Prompts for confirmation**

   Asks you to confirm before removing anything

4. **Removes symlinks**

   Deletes all confirmed symlinks

5. **Reports results**

   Confirms what was removed

</Steps>

### Interactive Confirmation

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

<Aside type="caution" title="Safe Operation">
Cleanup only removes symlinks. Your actual pixi installations in `~/.pixi/envs/` are never touched.
</Aside>

### When to Use

<CardGrid>
  <Card title="🔄 Fresh Start" icon="refresh">
    Want to recreate all symlinks from scratch? Run `cleanup` then `setup`.
  </Card>
  <Card title="🐛 Troubleshooting" icon="warning">
    Broken or incorrect symlinks? Cleanup removes them so you can start fresh.
  </Card>
  <Card title="🧹 Housekeeping" icon="approve-check">
    Uninstalled distributions but symlinks remain? Cleanup removes orphaned links.
  </Card>
</CardGrid>

### Restoring After Cleanup

```bash
# Remove all symlinks
rosenv cleanup

# Verify they're gone
rosenv list
# (should show no distributions)

# Recreate symlinks
rosenv setup

# Verify restoration
rosenv list
# (should show all distributions again)
```

<Aside type="tip" title="Non-Destructive">
Since pixi installations are preserved, you can safely cleanup and recreate symlinks anytime without losing data or
having to reinstall distributions.
</Aside>

---

## rosenv refresh

**Update symlinks after installing new distributions**

Scans for changes in pixi installations and updates symlinks accordingly. Automatically creates symlinks for new
distributions while preserving existing ones.

### Usage

```bash
rosenv refresh
```

### What It Does

<Steps>

1. **Scans pixi environments**

   Looks in `~/.pixi/envs/` for `ros-*-desktop` installations

2. **Checks existing symlinks**

   Verifies current symlinks in `/opt/ros/`

3. **Identifies changes**

   Finds new distributions and validates existing ones

4. **Updates symlinks**

   Creates symlinks for new distributions, updates invalid ones

5. **Reports results**

   Shows what was added, updated, or kept

</Steps>

### Example Output

```ansi
Scanning for changes...

Existing symlinks:
  ✓ humble: up to date

New distributions found:
  + jazzy → ~/.pixi/envs/ros-jazzy-desktop

Creating symlinks:
  ✓ /opt/ros/jazzy → ~/.pixi/envs/ros-jazzy-desktop

Refresh complete.
```

### When to Use

<CardGrid>
  <Card title="➕ New Installation" icon="add-document">
    Just installed a new distribution with pixi? Run `refresh` to create its symlink.
  </Card>
  <Card title="🔄 Pixi Update" icon="random">
    Updated distributions with pixi? Refresh ensures symlinks point to the right locations.
  </Card>
  <Card title="🔍 Verification" icon="magnifier">
    Not sure if symlinks are up to date? Refresh checks and fixes any issues.
  </Card>
</CardGrid>

### Workflow Example

```bash
# Install a new distribution
pixi global install --environment ros-jazzy-desktop \
  -c robostack-jazzy ros-jazzy-desktop

# Update rosenv
rosenv refresh

# Verify new distribution
rosenv list
# Should now show jazzy

# Activate immediately
rosenv activate jazzy
```

<Aside type="tip" title="Safe to Run Anytime">
`refresh` is non-destructive and idempotent. It only creates or updates symlinks as needed, never removing valid
existing ones.
</Aside>

### Difference from `setup`

| Command | When to Use |
|---------|-------------|
| `setup` | Initial configuration after installing rosenv |
| `refresh` | After adding new distributions with pixi |

Both commands are safe to run multiple times, but `refresh` provides more detailed reporting about changes.

---

## rosenv init

**Generate shell integration code**

Generates shell-specific code that enables seamless activation and deactivation without manual `eval` commands. This
provides a conda-like experience.

### Usage

```bash
rosenv init <shell>
```

### Arguments

- `<shell>` - Shell type: `zsh` or `bash`

### Installation

<Tabs>
  <TabItem label="zsh">

**One-time setup:**

```bash
# Generate and append to config
rosenv init zsh >> ~/.zshrc

# Reload your shell
source ~/.zshrc
```

**Or preview first:**

```bash
# See what will be added
rosenv init zsh

# If it looks good, add it
rosenv init zsh >> ~/.zshrc
source ~/.zshrc
```

  </TabItem>
  <TabItem label="bash">

**One-time setup:**

```bash
# Generate and append to config
rosenv init bash >> ~/.bashrc

# Reload your shell
source ~/.bashrc
```

**Or preview first:**

```bash
# See what will be added
rosenv init bash

# If it looks good, add it
rosenv init bash >> ~/.bashrc
source ~/.bashrc
```

  </TabItem>
</Tabs>

### What It Provides

<CardGrid>
  <Card title="🎯 Direct Activation" icon="star">
    Run `rosenv activate humble` directly without `eval` commands
  </Card>
  <Card title="🔄 Environment Switching" icon="random">
    Switch distributions seamlessly with automatic cleanup
  </Card>
  <Card title="📊 Status Checking" icon="list-format">
    Check active distribution with `rosenv status`
  </Card>
  <Card title="🔌 Deactivation" icon="approve-check">
    Exit ROS 2 environment cleanly with `rosenv deactivate`
  </Card>
</CardGrid>

### How It Works

The shell integration creates a wrapper function that:

1. **Intercepts commands** - Catches `activate`, `deactivate`, and `status`
2. **Evaluates in current shell** - Runs commands in your active shell session
3. **Passes through others** - All other commands go directly to the binary

### Generated Function

The shell function enables these commands:

| Command | What It Does |
|---------|-------------|
| `rosenv activate <distro>` | Evaluates activation script in current shell |
| `rosenv deactivate` | Evaluates deactivation script in current shell |
| `rosenv status` | Checks `ROS_DISTRO` in current shell |
| All others | Pass through to rosenv binary |

<Aside type="note" title="Required for Activation">
Without shell integration, `activate`, `deactivate`, and `status` commands won't work. They need to modify your
current shell's environment, which requires this integration.
</Aside>

### Verification

After adding shell integration, verify it's working:

```bash
# Check function exists (zsh)
type rosenv
# Should show: rosenv is a shell function

# Or for bash
type -a rosenv
# Should show: rosenv is a function

# Test it works
rosenv activate humble
rosenv status
# Should show: Active: ROS 2 humble
```

### Troubleshooting

<Aside type="caution" title="Integration not working?">

**Check if it was added:**

```bash
# For zsh
grep "rosenv" ~/.zshrc

# For bash
grep "rosenv" ~/.bashrc
```

**If not found, add it:**

```bash
rosenv init zsh >> ~/.zshrc  # or bash
source ~/.zshrc              # or ~/.bashrc
```

**Still not working? Check for errors:**

```bash
# Manually load and check for errors
source ~/.zshrc  # or ~/.bashrc
```

</Aside>

---

## rosenv setup-guide

**Open the online setup guide**

Opens the comprehensive setup guide in your default web browser. Useful for quick reference or when sharing setup
instructions.

### Usage

```bash
rosenv setup-guide
```

### Example

```bash
$ rosenv setup-guide
Opening ROS 2 Setup Guide in your browser...

URL: https://alvgaona.github.io/ros2env/

✓ Setup guide opened in your default browser
```

<Tabs>
  <TabItem label="macOS">

Uses the `open` command to launch your default browser.

  </TabItem>
  <TabItem label="Linux">

Uses the `xdg-open` command to launch your default browser.

  </TabItem>
</Tabs>

<Aside>
If your browser doesn't open automatically, the URL will be displayed so you can copy and paste it manually.
</Aside>

---

## rosenv --help

**Display help information**

Shows help for rosenv or specific commands, including usage, arguments, and options.

### Usage

```bash
# General help
rosenv --help

# Command-specific help
rosenv <command> --help
```

### Examples

```bash
# See all available commands
rosenv --help

# Get help for a specific command
rosenv activate --help
rosenv list --help
rosenv info --help
```

### Help Output Format

Help text includes:

- **Usage syntax** - How to invoke the command
- **Description** - What the command does
- **Arguments** - Required and optional parameters
- **Examples** - Common use cases

<Aside type="tip" title="Quick Command Reference">
Forget a command's syntax? Just add `--help` to see usage examples.
</Aside>

---

## rosenv --version

**Show version information**

Displays the currently installed version of rosenv.

### Usage

```bash
rosenv --version
```

### Example

```bash
$ rosenv --version
rosenv 0.1.0
```

<Aside type="note">
The version shown matches the installed rosenv package. Update with `pixi global update ros2env` to get the latest
version.
</Aside>

---

## Command Workflows

### Initial Setup Flow

```bash
# 1. Install rosenv
pixi global install ros2env

# 2. Setup /opt/ros
sudo mkdir -p /opt/ros
sudo chown $USER /opt/ros

# 3. Install ROS 2
pixi global install --environment ros-humble-desktop \
  -c robostack-humble ros-humble-desktop

# 4. Create symlinks
rosenv setup

# 5. Add shell integration
rosenv init zsh >> ~/.zshrc
source ~/.zshrc

# 6. Start using it
rosenv activate humble
```

### Maintenance Flow

```bash
# Install new distribution
pixi global install --environment ros-jazzy-desktop \
  -c robostack-jazzy ros-jazzy-desktop

# Update rosenv
rosenv refresh

# Verify
rosenv list

# Clean up if needed
rosenv cleanup
rosenv setup
```

### Troubleshooting Flow

```bash
# Check current state
rosenv list
rosenv status

# Get detailed info
rosenv info humble

# Clean slate
rosenv cleanup
rosenv setup

# Verify shell integration
type rosenv

# Re-add if needed
rosenv init zsh >> ~/.zshrc
source ~/.zshrc
```

---

## Best Practices

<CardGrid>
  <Card title="🔄 Regular Refresh" icon="refresh">
    Run `rosenv refresh` after installing new distributions to keep symlinks up to date.
  </Card>
  
  <Card title="🧪 Test After Changes" icon="rocket">
    After cleanup or refresh, verify with `rosenv list` and test activation with `rosenv activate`.
  </Card>
  
  <Card title="📝 Shell Integration Once" icon="document">
    Add shell integration to your config file once. No need to re-run after shell restarts.
  </Card>
  
  <Card title="💾 Safe Cleanup" icon="shield">
    Cleanup only removes symlinks, never pixi installations. Safe to use for troubleshooting.
  </Card>
</CardGrid>

---

## Next Steps

<CardGrid>
  <Card title="🔄 Distribution Management" icon="random">
    Learn how to activate and manage distributions
    
    [Distribution Commands →](/ros2env/commands/distribution/)
  </Card>
  
  <Card title="📖 Core Commands" icon="star">
    Master essential commands like list and status
    
    [Core Commands →](/ros2env/commands/core/)
  </Card>
  
  <Card title="📚 Commands Overview" icon="open-book">
    See all available commands
    
    [Commands Overview →](/ros2env/commands/)
  </Card>
</CardGrid>

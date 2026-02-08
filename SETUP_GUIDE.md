# ROS 2 Setup Guide - Installing with pixi global

This guide shows you how to install and manage ROS 2 distributions using `pixi global` and `rosenv`.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installing ROS 2 Distributions](#installing-ros-2-distributions)
- [After Installation](#after-installation)
- [Managing Installations](#managing-installations)
- [Complete Workflow Example](#complete-workflow-example)
- [Disk Space & Tips](#disk-space--tips)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Install pixi

If you don't have pixi installed yet:

**macOS/Linux:**
```bash
curl -fsSL https://pixi.sh/install.sh | bash
```

**Or with Homebrew (macOS):**
```bash
brew install pixi
```

Verify installation:
```bash
pixi --version
```

### Setup /opt/ros Directory

Create the directory where rosenv will manage symlinks (one-time setup):

```bash
sudo mkdir -p /opt/ros
sudo chown $USER /opt/ros
```

After this, rosenv works without sudo!

---

## Installing ROS 2 Distributions

Install ROS 2 distributions globally using pixi. Each distribution is installed to its own environment (~3.3GB each).

### Humble Hawksbill (LTS - Recommended)

```bash
pixi global install --environment ros-humble-desktop -c robostack-staging ros-humble-desktop
```

### Iron Irwini

```bash
pixi global install --environment ros-iron-desktop -c robostack-staging ros-iron-desktop
```

### Jazzy Jalisco (Latest)

```bash
pixi global install --environment ros-jazzy-desktop -c robostack-staging ros-jazzy-desktop
```

### Rolling (Bleeding Edge)

```bash
pixi global install --environment ros-rolling-desktop -c robostack-staging ros-rolling-desktop
```

### Pattern for Any Distribution

```bash
pixi global install --environment ros-<DISTRO>-desktop -c robostack-staging ros-<DISTRO>-desktop
```

**Available distributions:** humble, iron, jazzy, rolling

> **Note:** Check [RoboStack](https://robostack.github.io/) for the latest available distributions.

---

## After Installation

### 1. Create Symlinks (First Time)

After installing your first distribution:

```bash
rosenv setup
```

This will:
- Scan `~/.pixi/envs/` for ROS 2 installations
- Create symlinks in `/opt/ros/`
- Show you what was set up

### 2. Add Shell Integration

Generate and add the shell integration to your shell config:

**For zsh:**
```bash
rosenv init zsh >> ~/.zshrc
source ~/.zshrc
```

**For bash:**
```bash
rosenv init bash >> ~/.bashrc
source ~/.bashrc
```

### 3. Verify Installation

Check that everything is set up correctly:

```bash
rosenv doctor
```

This will verify:
- `/opt/ros` directory exists and is writable
- Symlinks are valid
- ROS 2 setup files are present
- Shell integration is configured

### 4. Activate a Distribution

```bash
rosenv activate humble
```

You should see:
```
✓ Switched to ROS 2 humble
```

### 5. Test ROS 2

```bash
ros2 --version
ros2 topic list
```

---

## Managing Installations

### Update a Distribution

Update an existing ROS 2 distribution to the latest packages:

```bash
pixi global update ros-humble-desktop
```

After updating, refresh rosenv symlinks:

```bash
rosenv refresh
```

### List All Global Installations

See what pixi has installed globally:

```bash
pixi global list
```

### Add More Distributions

Install another distribution (e.g., Jazzy):

```bash
pixi global install --environment ros-jazzy-desktop -c robostack-staging ros-jazzy-desktop
rosenv refresh
```

Now you can switch between them:

```bash
rosenv activate jazzy
```

### Remove a Distribution

First, remove from rosenv:

```bash
rosenv remove humble
```

Then remove from pixi:

```bash
pixi global remove ros-humble-desktop
```

---

## Complete Workflow Example

Here's a complete workflow from scratch:

### Step 1: Install Humble

```bash
# Install with pixi
pixi global install --environment ros-humble-desktop -c robostack-staging ros-humble-desktop

# Create symlinks
rosenv setup

# Add shell integration (if not done yet)
rosenv init zsh >> ~/.zshrc
source ~/.zshrc

# Verify
rosenv doctor
```

### Step 2: Activate and Use

```bash
# Activate Humble
rosenv activate humble

# Check it works
ros2 --version

# Try a ROS 2 command
ros2 pkg list | head -10
```

### Step 3: Install Another Distribution

```bash
# Install Jazzy
pixi global install --environment ros-jazzy-desktop -c robostack-staging ros-jazzy-desktop

# Refresh symlinks
rosenv refresh

# Check available distributions
rosenv list
```

### Step 4: Switch Between Distributions

```bash
# Switch to Jazzy
rosenv activate jazzy
echo $ROS_DISTRO  # Shows: jazzy

# Switch back to Humble
rosenv activate humble
echo $ROS_DISTRO  # Shows: humble
```

### Step 5: Deactivate When Done

```bash
# Clean up ROS environment
rosenv deactivate

# ROS_DISTRO should now be empty
echo $ROS_DISTRO
```

---

## Disk Space & Tips

### Disk Usage

- **Each distribution:** ~3.3GB
- **Installation location:** `~/.pixi/envs/ros-<distro>-desktop/`
- **Symlinks:** Negligible (just pointers to pixi environments)

Example with two distributions:
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

### Why Global Installation?

**Benefits:**
- ✅ Share one installation across multiple ROS workspaces
- ✅ No duplication of 3.3GB per workspace/project
- ✅ Faster project setup (no repeated downloads)
- ✅ Consistent environment across all projects
- ✅ Easy to update all workspaces at once

**Comparison:**

| Approach | Disk Space (3 workspaces) | Update Process |
|----------|---------------------------|----------------|
| **Per-workspace install** | 9.9GB (3 × 3.3GB) | Update each workspace separately |
| **Global install (rosenv)** | 3.3GB (shared) | Update once, affects all workspaces |

### Compatibility

- ✅ **macOS:** Full support via RoboStack
- ✅ **Linux:** Full support via RoboStack  
- ✅ **Path compatibility:** Uses standard `/opt/ros/<distro>` paths
- ✅ **Tutorial compatibility:** Works with existing ROS 2 tutorials and scripts
- ✅ **Tool compatibility:** Compatible with colcon, ament, and all ROS 2 tools

---

## Troubleshooting

### rosenv doctor Shows Errors

Run diagnostics:

```bash
rosenv doctor
```

Common fixes:

**Problem:** `/opt/ros` not writable
```bash
sudo chown $USER /opt/ros
```

**Problem:** Symlinks broken
```bash
rosenv cleanup
rosenv setup
```

**Problem:** Shell integration not working
```bash
# Add shell integration again
rosenv init zsh >> ~/.zshrc
source ~/.zshrc

# Verify function exists
type rosenv  # Should show "rosenv is a shell function"
```

### Distribution Not Found

**Check if pixi installed it:**
```bash
pixi global list | grep ros
```

**Refresh symlinks:**
```bash
rosenv refresh
```

**List available distributions:**
```bash
rosenv list
```

### ROS 2 Commands Not Working After Activation

**Verify activation:**
```bash
echo $ROS_DISTRO  # Should show your distribution
which ros2        # Should show /opt/ros/<distro>/bin/ros2
```

**Try manual activation:**
```bash
rosenv deactivate
rosenv activate humble
```

**Check setup files:**
```bash
rosenv info humble
```

### Multiple ROS Versions Conflicting

**Deactivate current environment:**
```bash
rosenv deactivate
```

**Clean activation:**
```bash
rosenv activate humble
```

The shell integration automatically cleans previous ROS environment variables before activating a new distribution.

### Want to Remove Everything

**Remove all rosenv symlinks:**
```bash
rosenv cleanup
```

**Remove pixi installations:**
```bash
pixi global remove ros-humble-desktop
pixi global remove ros-jazzy-desktop
# ... etc
```

**Remove shell integration:**
Edit your `~/.zshrc` or `~/.bashrc` and remove the rosenv section.

---

## Additional Resources

- **rosenv commands:** `rosenv --help`
- **Diagnose issues:** `rosenv doctor`
- **Distribution info:** `rosenv info <distro>`
- **RoboStack documentation:** https://robostack.github.io/
- **pixi documentation:** https://pixi.sh/

---

## Quick Reference

### Essential Commands

```bash
# Installation
pixi global install --environment ros-humble-desktop -c robostack-staging ros-humble-desktop
rosenv setup

# Daily use
rosenv activate humble
rosenv activate jazzy
rosenv deactivate

# Management
rosenv list
rosenv status
rosenv doctor
rosenv refresh

# Information
rosenv info humble
rosenv --help
```

### Typical Daily Workflow

```bash
# Morning - start work
rosenv activate humble

# Work on your ROS project
cd ~/ros_workspace
colcon build
ros2 launch my_package my_launch.py

# Test on different distribution
rosenv activate jazzy
colcon build
ros2 launch my_package my_launch.py

# End of day - clean up
rosenv deactivate
```

---

**Happy ROS 2 development! 🤖**

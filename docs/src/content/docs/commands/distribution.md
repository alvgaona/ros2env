---
title: Distribution Management
description: Commands for activating, switching, and managing ROS 2 distributions
---

import { Tabs, TabItem, Aside, Card, CardGrid, Steps } from '@astrojs/starlight/components';

Distribution management commands let you activate, switch between, and manage your ROS 2 distributions. These are the
commands you'll use most frequently in your day-to-day ROS 2 workflow.

<Aside type="note" title="Shell Integration Required">
The `activate`, `deactivate`, and `status` commands require shell integration. Run `rosenv init <shell>` and add it
to your shell config to enable these features.
</Aside>

---

## rosenv activate

**Activate a ROS 2 distribution in your current shell**

Switches to the specified ROS 2 distribution by configuring all necessary environment variables and sourcing the
distribution's setup script.

### Usage

```bash
rosenv activate <distro>
```

### Arguments

- `<distro>` - Distribution name (e.g., `humble`, `jazzy`)

### How It Works

<Steps>

1. **Cleans previous environment**

   Removes any existing ROS-related environment variables to prevent conflicts

2. **Sets core variables**

   Configures `ROS_DISTRO`, `ROS_VERSION`, and related variables

3. **Sources setup script**

   Runs the distribution's `setup.bash` or `setup.zsh` to configure all paths

4. **Validates activation**

   Verifies the environment is correctly configured

</Steps>

### Example

```bash
# Activate Humble
$ rosenv activate humble
✓ Switched to ROS 2 humble

# Verify it's active
$ echo $ROS_DISTRO
humble

# Use ROS 2 tools
$ ros2 --version
ros2 doctor 0.10.3
```

### Environment Variables Set

When you activate a distribution, these variables are automatically configured:

<CardGrid>
  <Card title="Core Variables" icon="document">
    - `ROS_DISTRO` - Distribution name
    - `ROS_VERSION` - Always `2` for ROS 2
    - `ROS_PYTHON_VERSION` - Python version (usually `3`)
  </Card>
  <Card title="Path Variables" icon="open-book">
    - `AMENT_PREFIX_PATH` - Package search paths
    - `CMAKE_PREFIX_PATH` - CMake package paths
    - `COLCON_PREFIX_PATH` - Colcon paths
    - `PYTHONPATH` - Python module paths
  </Card>
  <Card title="Library Paths" icon="rocket">
    - `LD_LIBRARY_PATH` (Linux)
    - `DYLD_LIBRARY_PATH` (macOS)
    - `PKG_CONFIG_PATH`
  </Card>
  <Card title="Binary Paths" icon="laptop">
    - `PATH` - Adds `/opt/ros/<distro>/bin`
    - Enables all ROS 2 CLI tools
  </Card>
</CardGrid>

### Switching Distributions

You can switch between distributions seamlessly. rosenv automatically cleans up the previous environment:

```bash
# Start with Humble
$ rosenv activate humble
✓ Switched to ROS 2 humble

$ echo $ROS_DISTRO
humble

# Switch to Jazzy
$ rosenv activate jazzy
✓ Switched to ROS 2 jazzy

$ echo $ROS_DISTRO
jazzy
```

<Aside type="tip" title="No Manual Cleanup Needed">
You don't need to run `rosenv deactivate` before switching. The `activate` command handles cleanup automatically.
</Aside>

### Troubleshooting

<Aside type="caution" title="Distribution not found?">

Make sure the distribution is installed and symlinked:

```bash
# Check available distributions
rosenv list

# If missing, install with pixi
pixi global install --environment ros-humble-desktop \
  -c robostack-humble ros-humble-desktop

# Then refresh rosenv
rosenv refresh
```

</Aside>

<Aside type="caution" title="Command not found?">

Ensure shell integration is configured:

```bash
# For zsh
rosenv init zsh >> ~/.zshrc
source ~/.zshrc

# For bash
rosenv init bash >> ~/.bashrc
source ~/.bashrc
```

</Aside>

---

## rosenv deactivate

**Exit the current ROS 2 environment**

Removes all ROS-related environment variables and restores your shell to a clean state.

### Usage

```bash
rosenv deactivate
```

### What It Does

<Steps>

1. **Identifies active distribution**

   Checks `ROS_DISTRO` to see what's currently active

2. **Unsets ROS variables**

   Removes all ROS-related environment variables

3. **Cleans PATH**

   Removes `/opt/ros/` entries from your PATH

4. **Reports success**

   Confirms which distribution was deactivated

</Steps>

### Example

```bash
# While a distribution is active
$ rosenv status
Active: ROS 2 humble

$ rosenv deactivate
✓ Deactivated ROS 2 humble

# Verify deactivation
$ echo $ROS_DISTRO

$ rosenv status
No ROS 2 distribution is currently active
```

### Environment Variables Removed

All ROS-related variables are unset:

- `ROS_DISTRO`, `ROS_VERSION`, `ROS_PYTHON_VERSION`
- `AMENT_PREFIX_PATH`, `CMAKE_PREFIX_PATH`, `COLCON_PREFIX_PATH`
- `PYTHONPATH`, `LD_LIBRARY_PATH`, `DYLD_LIBRARY_PATH`
- `PKG_CONFIG_PATH`
- PATH entries containing `/opt/ros/`

<Aside type="tip" title="When to Deactivate">
Deactivate when:
- Switching to non-ROS work
- Running system Python without ROS
- Debugging environment issues
- Ending your ROS 2 session
</Aside>

---

## rosenv info

**Display detailed information about a distribution**

Shows comprehensive information about a specific ROS 2 distribution, including paths, setup files, and directory
statistics.

### Usage

```bash
rosenv info <distro>
```

### Arguments

- `<distro>` - Distribution name (e.g., `humble`, `jazzy`)

### Example Output

<Tabs>
  <TabItem label="macOS">

```ansi
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

```ansi
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

### What It Shows

<CardGrid>
  <Card title="Path Information" icon="open-book">
    - Symlink location (`/opt/ros/<distro>`)
    - Target path (`~/.pixi/envs/...`)
    - Link type (symlink vs directory)
  </Card>
  <Card title="Setup Files" icon="document">
    - Available shell setup scripts
    - setup.bash, setup.zsh, setup.sh
    - Indicates which are present
  </Card>
  <Card title="Directory Stats" icon="list-format">
    - Key directories (bin, lib, share, include)
    - Number of entries in each
    - Helps verify complete installation
  </Card>
</CardGrid>

### Use Cases

<Aside type="tip" title="Diagnostic Tool">
Use `rosenv info` to:
- **Verify installations** - Check if all key directories exist
- **Troubleshoot issues** - Identify missing or broken symlinks
- **Compare distributions** - See differences in package counts
- **Validate setup** - Ensure setup files are present
</Aside>

---

## rosenv remove

**Remove a distribution symlink**

Deletes the symlink for a specific distribution from `/opt/ros/`. The actual pixi installation remains untouched.

### Usage

```bash
rosenv remove <distro>
```

### Arguments

- `<distro>` - Distribution name (e.g., `humble`, `jazzy`)

### Interactive Confirmation

For safety, this command always asks for confirmation:

```bash
$ rosenv remove humble
Remove /opt/ros/humble? [y/N] y
✓ Removed /opt/ros/humble

Note: The pixi installation remains at:
  ~/.pixi/envs/ros-humble-desktop

To reinstall the symlink: rosenv setup
```

<Aside type="caution" title="Confirmation Required">
You must type `y` and press Enter to confirm removal. Any other input cancels the operation.
</Aside>

### What It Does

<Steps>

1. **Verifies symlink exists**

   Checks that `/opt/ros/<distro>` is present

2. **Prompts for confirmation**

   Asks you to confirm the removal

3. **Removes symlink only**

   Deletes the symlink but leaves pixi installation intact

4. **Shows next steps**

   Explains how to restore the symlink if needed

</Steps>

### Completely Removing a Distribution

If you want to remove both the symlink AND the pixi installation:

```bash
# Step 1: Remove symlink
rosenv remove humble

# Step 2: Remove pixi installation
pixi global remove ros-humble-desktop
```

<Aside type="note" title="Two-Step Process">
rosenv only manages symlinks. To completely uninstall a distribution, you must also remove the pixi environment.
</Aside>

### Restoring Removed Symlinks

If you accidentally remove a symlink, restore it easily:

```bash
# Recreate all symlinks (including removed ones)
rosenv setup

# Or refresh to scan for changes
rosenv refresh
```

---

## Command Workflow

### Daily Usage Pattern

```bash
# Morning: Start working with ROS 2
rosenv activate humble

# Check environment
rosenv status

# Work on your project...
ros2 run my_package my_node

# Need to switch?
rosenv activate jazzy

# Evening: Clean up
rosenv deactivate
```

### Maintenance Pattern

```bash
# Check what's installed
rosenv list

# Get detailed info
rosenv info humble
rosenv info jazzy

# Remove unused distribution
rosenv remove iron

# Verify remaining distributions
rosenv list
```

---

## Best Practices

<CardGrid>
  <Card title="✅ Always Verify" icon="approve-check">
    Use `rosenv status` before building or running ROS 2 projects to confirm you're using the intended distribution.
  </Card>
  
  <Card title="🔄 Clean Switches" icon="random">
    When switching distributions, let `activate` handle the cleanup. No need to manually `deactivate` first.
  </Card>
  
  <Card title="📊 Regular Checks" icon="list-format">
    Run `rosenv info <distro>` periodically to verify your installations are healthy and complete.
  </Card>
  
  <Card title="🎯 Single Active" icon="star">
    Only activate one distribution per shell session to avoid conflicts and confusion.
  </Card>
</CardGrid>

---

## Next Steps

<CardGrid>
  <Card title="🛠️ Utility Commands" icon="setting">
    Learn about cleanup, refresh, and init commands
    
    [Utility Commands →](/ros2env/commands/management/)
  </Card>
  
  <Card title="📖 Core Commands" icon="star">
    Master list, status, and setup commands
    
    [Core Commands →](/ros2env/commands/core/)
  </Card>
  
  <Card title="📚 Commands Overview" icon="open-book">
    See all available commands
    
    [Commands Overview →](/ros2env/commands/)
  </Card>
</CardGrid>

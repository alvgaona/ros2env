---
title: Commands Overview
description: Complete reference for all rosenv commands
---

import { Card, CardGrid, Aside, LinkCard, Badge, Steps } from '@astrojs/starlight/components';

## Command Categories

<Badge text="13 Commands" variant="success" size="large" />
<Badge text="3 Categories" variant="note" size="large" />

rosenv provides a comprehensive set of commands organized into three main categories:

<CardGrid>
  <Card title="🎯 Core Commands" icon="star">
    Essential commands for daily ROS 2 workflow
    
    - **list** - View available distributions
    - **status** - Check active distribution
    - **setup** - Initial symlink creation
    
    [View Core Commands →](/ros2env/commands/core/)
  </Card>
  
  <Card title="🔄 Distribution Management" icon="random">
    Activate, switch, and manage ROS 2 distributions
    
    - **activate** - Switch to a distribution
    - **deactivate** - Exit ROS 2 environment
    - **info** - Detailed distribution info
    - **remove** - Delete distribution symlink
    
    [View Distribution Commands →](/ros2env/commands/distribution/)
  </Card>
  
  <Card title="🛠️ Utility Commands" icon="setting">
    Maintenance and configuration tools
    
    - **cleanup** - Remove all symlinks
    - **refresh** - Update symlinks
    - **init** - Shell integration setup
    
    [View Utility Commands →](/ros2env/commands/management/)
  </Card>
</CardGrid>

---

## Quick Reference

### Most Used Commands

```bash
# View all available distributions
rosenv list

# Activate a distribution (requires shell integration)
rosenv activate humble

# Check what's currently active
rosenv status

# View detailed info about a distribution
rosenv info humble

# Deactivate current distribution
rosenv deactivate
```

<Aside type="tip" title="Shell Integration Required">
Commands like `activate`, `deactivate`, and `status` require shell integration. Run `rosenv init <shell>` and add
it to your shell config to enable these features.
</Aside>

---

## Command Syntax

All rosenv commands follow this general pattern:

```bash
rosenv <command> [arguments] [options]
```

### Getting Help

Every command supports the `--help` flag:

```bash
# General help
rosenv --help

# Command-specific help
rosenv activate --help
rosenv list --help
```

---

## Command Groups

### Setup & Discovery

Commands for initial setup and discovering distributions:

<CardGrid>
  <LinkCard
    title="setup"
    description="Create symlinks for all detected distributions"
    href="/ros2env/commands/core/#rosenv-setup"
  />
  <LinkCard
    title="list"
    description="Show all available distributions"
    href="/ros2env/commands/core/#rosenv-list"
  />
  <LinkCard
    title="info"
    description="Display detailed distribution information"
    href="/ros2env/commands/distribution/#rosenv-info"
  />
</CardGrid>

### Environment Management

Commands for activating and switching between distributions:

<CardGrid>
  <LinkCard
    title="activate"
    description="Switch to a ROS 2 distribution"
    href="/ros2env/commands/distribution/#rosenv-activate"
  />
  <LinkCard
    title="deactivate"
    description="Exit the current ROS 2 environment"
    href="/ros2env/commands/distribution/#rosenv-deactivate"
  />
  <LinkCard
    title="status"
    description="Check the currently active distribution"
    href="/ros2env/commands/core/#rosenv-status"
  />
</CardGrid>

### Maintenance

Commands for maintaining and updating your setup:

<CardGrid>
  <LinkCard
    title="refresh"
    description="Update symlinks after installing new distributions"
    href="/ros2env/commands/management/#rosenv-refresh"
  />
  <LinkCard
    title="cleanup"
    description="Remove all distribution symlinks"
    href="/ros2env/commands/management/#rosenv-cleanup"
  />
  <LinkCard
    title="remove"
    description="Remove a specific distribution symlink"
    href="/ros2env/commands/distribution/#rosenv-remove"
  />
</CardGrid>

### Configuration

Commands for setting up shell integration:

<CardGrid>
  <LinkCard
    title="init"
    description="Generate shell integration code"
    href="/ros2env/commands/management/#rosenv-init"
  />
</CardGrid>

---

## Common Workflows

### Initial Setup

<Steps>

1. **Install ROS 2 with pixi**

   ```bash
   pixi global install --environment ros-humble-desktop \
     -c robostack-humble ros-humble-desktop
   ```

2. **Create symlinks**

   ```bash
   rosenv setup
   ```

3. **Add shell integration**

   ```bash
   rosenv init zsh >> ~/.zshrc
   source ~/.zshrc
   ```

4. **Verify installation**

   ```bash
   rosenv list
   ```

</Steps>

### Daily Usage

<Steps>

1. **Start working with ROS 2**

   ```bash
   rosenv activate humble
   ```

2. **Check your environment**

   ```bash
   rosenv status
   ```

3. **Work with ROS 2 tools**

   ```bash
   ros2 run demo_nodes_cpp talker
   ```

4. **Switch distributions** (if needed)

   ```bash
   rosenv activate jazzy
   ```

5. **Deactivate when done**

   ```bash
   rosenv deactivate
   ```

</Steps>

### Adding New Distributions

<Steps>

1. **Install with pixi**

   ```bash
   pixi global install --environment ros-jazzy-desktop \
     -c robostack-jazzy ros-jazzy-desktop
   ```

2. **Update rosenv**

   ```bash
   rosenv refresh
   ```

3. **Verify detection**

   ```bash
   rosenv list
   ```

4. **Activate new distribution**

   ```bash
   rosenv activate jazzy
   ```

</Steps>

### Troubleshooting

<Steps>

1. **Check available distributions**

   ```bash
   rosenv list
   ```

2. **Get detailed distribution info**

   ```bash
   rosenv info humble
   ```

3. **Recreate all symlinks**

   ```bash
   rosenv cleanup
   rosenv setup
   ```

4. **Verify environment is clean**

   ```bash
   rosenv status
   ```

</Steps>

<Aside type="tip" title="Common Issues">
- **Distribution not detected?** Ensure pixi environment follows `ros-<distro>-desktop` naming
- **Permission denied?** Check `/opt/ros` is owned by your user: `sudo chown $USER /opt/ros`
- **Activation not working?** Verify shell integration: `type rosenv` should show "shell function"
</Aside>

---

## Exit Codes

rosenv uses standard exit codes:

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error (invalid arguments, command failed) |
| `2` | Distribution not found |
| `3` | Permission denied |
| `4` | Shell integration not configured |

<Aside type="note">
You can check the exit code of the last command with `echo $?` in your shell.
</Aside>

---

## Next Steps

<CardGrid>
  <Card title="📖 Core Commands" icon="star">
    Learn about essential commands like `list`, `status`, and `setup`
    
    [Core Commands →](/ros2env/commands/core/)
  </Card>
  
  <Card title="🔄 Distribution Management" icon="random">
    Master activation, deactivation, and distribution info
    
    [Distribution Commands →](/ros2env/commands/distribution/)
  </Card>
  
  <Card title="🛠️ Utility Commands" icon="setting">
    Explore maintenance and configuration tools
    
    [Utility Commands →](/ros2env/commands/management/)
  </Card>
</CardGrid>

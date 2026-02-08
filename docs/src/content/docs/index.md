---
title: Getting Started
description: Complete guide to installing and setting up ros2env
---

import { Tabs, TabItem, Aside, Card, CardGrid, Steps, Code, Badge, LinkButton, FileTree } from '@astrojs/starlight/components';

<Badge text="v0.1.0" variant="success" size="large" />
<Badge text="macOS & Linux" variant="note" size="large" />
<Badge text="Humble & Jazzy" variant="tip" size="large" />

## What is ros2env?

**ros2env** is a ROS 2 distribution manager for [pixi](https://pixi.sh) global installations. It bridges the gap
between pixi's environment management and ROS 2's traditional `/opt/ros/` structure.

<CardGrid>
  <Card title="🔗 Symlink Management" icon="link">
    Automatically creates and maintains symlinks in `/opt/ros/` for full compatibility with ROS 2 tools and
    tutorials.
  </Card>
  <Card title="🔄 Easy Switching" icon="random">
    Switch between ROS 2 distributions instantly with automatic environment variable management.
  </Card>
  <Card title="🛡️ Environment Isolation" icon="shield">
    Clean separation between distributions prevents conflicts and ensures reproducible builds.
  </Card>
  <Card title="🔍 Auto-Detection" icon="magnifier">
    Automatically discovers ROS 2 installations in `~/.pixi/envs/` without manual configuration.
  </Card>
</CardGrid>

<Aside type="tip" title="Why use ros2env?">
ROS 2 tools and tutorials expect distributions at `/opt/ros/<distro>`. When using pixi global for installations,
**rosenv** (the binary) creates the necessary symlinks and manages environment variables so everything works
seamlessly.
</Aside>

---

## Prerequisites

Before installing ros2env, ensure you have:

<CardGrid>
  <Card title="pixi" icon="package">
    Install from [prefix.dev](https://prefix.dev/docs/pixi/installation)
    
    ```bash
    curl -fsSL https://pixi.sh/install.sh | bash
    ```
  </Card>
  <Card title="Shell" icon="laptop">
    Supported shells:
    - **zsh** (macOS default)
    - **bash** (Linux default)
  </Card>
</CardGrid>

---

## Installation

<Steps>

1. **Install rosenv via pixi**

   ```bash
   pixi global install ros2env
   ```

2. **Verify installation**

   ```bash
   rosenv --version
   ```

   You should see: `rosenv 0.1.0`

3. **Setup `/opt/ros` directory**

   <Tabs>
     <TabItem label="macOS">

   ```bash
   # Create directory and set ownership
   sudo mkdir -p /opt/ros
   sudo chown $USER /opt/ros
   ```

     </TabItem>
     <TabItem label="Linux">

   ```bash
   # Create directory and set ownership
   sudo mkdir -p /opt/ros
   sudo chown $USER /opt/ros
   ```

     </TabItem>
   </Tabs>

   <Aside type="note">
   This is a **one-time setup**. After this, rosenv works without sudo.
   </Aside>

</Steps>

---

## Installing ROS 2 Distributions

Install your desired ROS 2 distributions using pixi. Each distribution requires ~3.3GB of disk space.

<Aside type="note" title="Supported Distributions">
Currently supported: **Humble Hawksbill** (LTS) and **Jazzy Jalisco** (Latest)
</Aside>

<Tabs>
  <TabItem label="🐢 Humble (LTS) - Recommended">

**Humble Hawksbill** is the Long-Term Support release, maintained until 2027.

```bash
pixi global install --environment ros-humble-desktop \
  -c robostack-humble ros-humble-desktop
```

<Aside type="tip">
Recommended for production use and long-term projects.
</Aside>

  </TabItem>
  <TabItem label="🎷 Jazzy (Latest)">

**Jazzy Jalisco** is the latest ROS 2 release with cutting-edge features.

```bash
pixi global install --environment ros-jazzy-desktop \
  -c robostack-jazzy ros-jazzy-desktop
```

<Aside>
Best for exploring new features and staying current with ROS 2 development.
</Aside>

  </TabItem>
</Tabs>

<Aside type="caution" title="Installation Time">
Installing a ROS 2 distribution takes **5-15 minutes** depending on your internet connection. The download size is
approximately 1GB, expanding to 3.3GB on disk.
</Aside>

---

## Setup rosenv

After installing at least one ROS 2 distribution, configure rosenv to manage your environments.

<Aside type="note" title="What rosenv Creates">
rosenv creates symlinks in `/opt/ros/` that point to your pixi environments. Here's the resulting structure:

<FileTree>

- /opt/ros/
  - humble/ → /Users/you/.pixi/envs/ros-humble-desktop/ (symlink)
  - jazzy/ → /Users/you/.pixi/envs/ros-jazzy-desktop/ (symlink)

</FileTree>
</Aside>

<Steps>

1. **Create symlinks for installed distributions**

   ```bash
   rosenv setup
   ```

   **Expected output:**
   ```
   Scanning for ROS 2 distributions in ~/.pixi/envs/...
   
   Found distributions:
     • humble
   
   Creating symlinks:
     ✓ /opt/ros/humble → ~/.pixi/envs/ros-humble-desktop
   
   Setup complete! Run 'rosenv list' to verify.
   ```

2. **Verify symlinks were created**

   ```bash
   rosenv list
   ```

   **Expected output:**
   ```
   Available ROS 2 distributions:
   
     • humble  /opt/ros/humble
   
   Use 'rosenv activate <distro>' to activate a distribution
   ```

3. **Add shell integration**

   Shell integration enables seamless activation/deactivation without manual `eval` commands.

   <Tabs>
     <TabItem label="zsh">

   ```bash
   # Add to ~/.zshrc
   rosenv init zsh >> ~/.zshrc
   
   # Reload shell configuration
   source ~/.zshrc
   ```

     </TabItem>
     <TabItem label="bash">

   ```bash
   # Add to ~/.bashrc
   rosenv init bash >> ~/.bashrc
   
   # Reload shell configuration
   source ~/.bashrc
   ```

     </TabItem>
   </Tabs>

   <Aside type="tip" title="What does shell integration do?">
   It creates a shell function that intercepts `rosenv activate` and `rosenv deactivate` commands, evaluating
   them in your current shell session for instant environment changes.
   </Aside>

</Steps>

---

## Quick Start Guide

Now you're ready to use rosenv! Here's a typical workflow:

<Steps>

1. **Activate a ROS 2 distribution**

   ```bash
   rosenv activate humble
   ```

   **Output:**
   ```
   ✓ Switched to ROS 2 humble
   ```

2. **Verify activation**

   ```bash
   rosenv status
   ```

   **Output:**
   ```
   Active: ROS 2 humble
   
   Key environment variables:
     ROS_DISTRO=humble
     ROS_VERSION=2
     AMENT_PREFIX_PATH=/opt/ros/humble
   ```

3. **Use ROS 2 tools**

   All standard ROS 2 commands now work:

   ```bash
   # Check ROS 2 version
   ros2 --version
   
   # List available packages
   ros2 pkg list
   
   # Run a demo node
   ros2 run demo_nodes_cpp talker
   ```

4. **Switch distributions (if you have multiple)**

   ```bash
   rosenv activate jazzy
   ```

   rosenv automatically cleans up the previous environment and activates the new one.

5. **Deactivate when done**

   ```bash
   rosenv deactivate
   ```

   **Output:**
   ```
   ✓ Deactivated ROS 2 humble
   ```

</Steps>

---

## Common Tasks

<CardGrid>
  <Card title="📋 List distributions" icon="list-format">
    ```bash
    rosenv list
    ```
    Shows all available ROS 2 distributions and their symlink status.
  </Card>
  
  <Card title="ℹ️ Distribution info" icon="information">
    ```bash
    rosenv info humble
    ```
    Displays detailed information about a specific distribution including setup files and directories.
  </Card>
  
  <Card title="🔄 Refresh symlinks" icon="refresh">
    ```bash
    rosenv refresh
    ```
    Scans for new distributions and updates symlinks. Run after installing new distributions with pixi.
  </Card>
  
  <Card title="🧹 Cleanup" icon="warning">
    ```bash
    rosenv cleanup
    ```
    Removes all distribution symlinks. The actual pixi installations remain untouched.
  </Card>
</CardGrid>

---

## Troubleshooting

<Aside type="caution" title="Permission denied when creating symlinks?">

Ensure `/opt/ros` is owned by your user:

```bash
sudo chown -R $USER /opt/ros
```

</Aside>

<Aside type="caution" title="Distribution not detected?">

Make sure your pixi environment follows the naming convention `ros-<distro>-desktop`:

```bash
# Correct naming
pixi global install --environment ros-humble-desktop ...

# Incorrect (won't be detected)
pixi global install --environment humble-ros ...
```

</Aside>

<Aside type="caution" title="Shell integration not working?">

Verify the shell function was added:

```bash
# For zsh
grep "rosenv" ~/.zshrc

# For bash
grep "rosenv" ~/.bashrc
```

If not found, re-run:

```bash
rosenv init zsh >> ~/.zshrc  # or bash
source ~/.zshrc              # or ~/.bashrc
```

</Aside>

---

## Next Steps

<CardGrid stagger>
  <Card title="📖 Commands Reference" icon="open-book">
    Explore all available commands and their options
    
    <LinkButton href="/ros2env/commands/" variant="primary">
      View Commands
    </LinkButton>
  </Card>
  
  <Card title="🤝 Contributing" icon="github">
    Help improve ros2env by contributing code or documentation
    
    <LinkButton href="/ros2env/contributing/" variant="secondary">
      Contributing Guide
    </LinkButton>
  </Card>
  
  <Card title="🐛 Report Issues" icon="warning">
    Found a bug or have a feature request?
    
    <LinkButton href="https://github.com/alvgaona/ros2env/issues" variant="minimal">
      Open an Issue
    </LinkButton>
  </Card>
</CardGrid>

---
title: Commands Reference
description: Complete command reference for ros2env
---

## Command Overview

**ros2env** provides commands via the `rosenv` binary, organized into three categories:

### Core Commands

Commands for basic setup and information:

- `rosenv setup` - Auto-detect and create symlinks
- `rosenv list` - List available distributions
- `rosenv status` - Show current active distribution
- `rosenv doctor` - Verify installation

See [Core Commands](/ros2env/commands/core/) for details.

### Distribution Management

Commands for working with distributions:

- `rosenv activate` - Activate a distribution
- `rosenv deactivate` - Deactivate current distribution
- `rosenv info` - Show distribution details
- `rosenv remove` - Remove a symlink

See [Distribution Management](/ros2env/commands/distribution/) for details.

### Utility Commands

Additional management commands:

- `rosenv cleanup` - Remove all symlinks
- `rosenv refresh` - Update all symlinks
- `rosenv init` - Generate shell integration
- `rosenv setup-guide` - Open setup guide

See [Utility Commands](/ros2env/commands/management/) for details.

## Command Syntax

All commands follow the format:

```bash
rosenv <command> [arguments] [flags]
```

Use `rosenv --help` to see all available commands.

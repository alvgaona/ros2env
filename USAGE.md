# rosenv Usage Guide

## ✅ Installation Complete!

The `rosenv` tool has been built and installed to `~/.local/bin/rosenv`.

Your `.zshrc` has been updated with the new configuration.

## How to Use

### New to ROS 2 + pixi?

If you need help installing ROS 2 distributions with pixi:

```bash
rosenv setup-guide
```

This opens a comprehensive guide in your browser with:
- Pixi installation instructions
- ROS 2 distribution installation commands
- Complete workflow examples
- Management and troubleshooting tips

### In a New Terminal Session

1. **Open a new terminal** (or run `source ~/.zshrc`)

2. **The unified `rosenv` command is now available:**

```bash
# Show status
rosenv status

# Switch to Humble
rosenv activate humble

# Switch to Jazzy
rosenv activate jazzy

# Deactivate ROS environment
rosenv deactivate
```

### Direct rosenv Commands

```bash
# Open setup guide in browser (helpful for new users!)
rosenv setup-guide

# List distributions
rosenv list

# Show current status
rosenv status

# Get info about a distribution
rosenv info humble

# Verify installation
rosenv doctor

# Refresh symlinks (after installing new distributions)
rosenv refresh
```

## How It Works

The `rosenv` command uses a **shell function wrapper** (conda-style) that intercepts key commands:

- **`rosenv activate`** - Calls the binary, evals output, activates in current shell
- **`rosenv deactivate`** - Calls the binary, evals output, cleans environment
- **`rosenv status`** - Checks `$ROS_DISTRO` directly for accurate results
- **Other commands** - Pass through to the binary

This means **no manual `eval` needed** - just use `rosenv activate humble` and it works!

The shell function in your `.zshrc`:

```zsh
rosenv() {
    case "$1" in
        activate)
            # Intercepts activate, handles eval automatically
            local script
            script=$(command rosenv activate "$2" 2>&1)
            if [ $? -eq 0 ]; then
                eval "$script"
                echo "✓ Switched to ROS 2 $2"
            fi
            ;;
        # ... other cases ...
    esac
}
```

## Testing in Current Shell

**Open a new terminal** and try:

```bash
# Check status (auto-activated to humble)
rosenv status

# Switch to Jazzy
rosenv activate jazzy

# Verify it worked
echo $ROS_DISTRO  # Should print: jazzy
which ros2        # Should show: /opt/ros/jazzy/bin/ros2

# Switch back to Humble
rosenv activate humble

# Verify
echo $ROS_DISTRO  # Should print: humble
which ros2        # Should show: /opt/ros/humble/bin/ros2

# Deactivate ROS
rosenv deactivate

# Verify cleanup
echo $ROS_DISTRO  # Should be empty
```

## Auto-Activation

Your `.zshrc` includes:

```zsh
rosenv activate humble >/dev/null 2>&1
```

This **silently** auto-activates Humble when you open a new terminal. You can:
- Comment it out if you prefer manual activation
- Change `humble` to `jazzy` or another default
- Remove it entirely if you always want to start clean
- Remove `>/dev/null 2>&1` if you want to see activation messages

## Adding New Distributions

```bash
# Install with pixi
pixi global install --environment ros-iron -c robostack-staging ros-iron-desktop

# Update symlinks
rosenv refresh

# Now you can switch to it
rosenv activate iron
```

## Troubleshooting

```bash
# Run diagnostics
rosenv doctor

# If symlinks are broken
rosenv cleanup
rosenv setup

# If rosenv commands don't work
source ~/.zshrc

# Check shell function is loaded
type rosenv         # Should show "rosenv is a shell function"

# If nothing works, check:
which rosenv        # Should be ~/.local/bin/rosenv (for binary)
ls -la /opt/ros/    # Should show symlinks
rosenv list         # Should show distributions
```

## Workflow Examples

### Daily Use

```bash
# Morning: Start with Humble (or already auto-activated)
rosenv activate humble
colcon build
ros2 launch my_package my_launch.py

# Afternoon: Test on Jazzy
rosenv activate jazzy
colcon build
ros2 launch my_package my_launch.py

# Evening: Clean up
rosenv deactivate
```

### After System Update

```bash
# Update pixi installations
pixi global update ros-humble-desktop
pixi global update ros-jazzy-desktop

# Symlinks still work (they point to the same locations)
rosenv doctor  # Verify everything is OK
```

## What Changed in Your .zshrc

**Old approach:**
- `ros-distro` shell function (simple wrapper)
- Manual `eval $(rosenv activate ...)` required

**New approach (conda-style):**
- Unified `rosenv` shell function
- Intercepts activate/deactivate/status commands
- Handles eval automatically - transparent to user
- Accurate status checking from shell environment
- Silent auto-activation on shell startup

The new configuration is at the end of your `.zshrc` (approximately lines 94-150).

## Next Steps

1. **Open a new terminal** to load the new configuration
2. **Check auto-activation**: `rosenv status` (should show Humble is active)
3. **Test switching**: `rosenv activate jazzy`
4. **Test deactivation**: `rosenv deactivate`
5. **Verify**: `rosenv doctor`
6. **Enjoy!** You now have a professional conda-style ROS 2 distribution manager

## Support

- Check README.md for full documentation
- Run `rosenv --help` for command reference
- Run `rosenv doctor` for diagnostics

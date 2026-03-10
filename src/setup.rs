use anyhow::Result;

use crate::distro::scan_pixi_ros_installations;
use crate::symlink::{check_opt_ros_writable, create_symlink};

pub fn cmd_setup() -> Result<()> {
    println!("Scanning ~/.pixi/envs for ROS 2 installations...\n");

    let distros = scan_pixi_ros_installations()?;

    if distros.is_empty() {
        println!("No ROS distributions found in ~/.pixi/envs/\n");
        println!("Install with pixi global:");
        println!("  pixi global install --environment ros-humble -c robostack-staging ros-humble-desktop");
        println!(
            "  pixi global install --environment ros-jazzy -c robostack-staging ros-jazzy-desktop"
        );
        println!("\nThen: rosenv setup");
        return Ok(());
    }

    println!("Found distributions:");
    for distro in &distros {
        println!("  • ros-{}-*  → {}", distro.name, distro.path.display());
    }
    println!();

    println!("Checking /opt/ros permissions...");
    check_opt_ros_writable()?;
    println!("✓ /opt/ros is writable\n");

    println!("Creating symlinks:");
    for distro in &distros {
        create_symlink(&distro.name, &distro.path, false)?;
    }

    println!("\nSetup complete!\n");
    println!("Next steps:");
    println!("  1. Add shell integration: rosenv init zsh >> ~/.zshrc");
    println!("  2. Reload shell: source ~/.zshrc");
    println!("  3. Switch distributions: ros-distro humble");

    Ok(())
}

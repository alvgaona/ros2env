use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs as unix_fs;
use std::path::Path;

use crate::distro::{get_ros_root, list_distributions, scan_pixi_ros_installations};

pub fn check_opt_ros_writable() -> Result<()> {
    let ros_root = get_ros_root();

    if !ros_root.exists() {
        anyhow::bail!(
            "/opt/ros does not exist\n\nRun these commands first:\n  sudo mkdir -p /opt/ros\n  sudo chown $USER /opt/ros\n\nThen: rosenv setup"
        );
    }

    let test_file = ros_root.join(".rosenv-test");
    match fs::File::create(&test_file) {
        Ok(_) => {
            let _ = fs::remove_file(&test_file);
            Ok(())
        }
        Err(_) => {
            anyhow::bail!(
                "/opt/ros is not writable\n\nFix:\n  sudo chown $USER /opt/ros\n\nThen: rosenv setup"
            )
        }
    }
}

pub fn create_symlink(distro: &str, target: &Path, force: bool) -> Result<()> {
    let link_path = get_ros_root().join(distro);

    if link_path.exists() {
        if !force {
            println!("  ⚠ /opt/ros/{} already exists", distro);

            if link_path.is_symlink() {
                if let Ok(existing_target) = fs::read_link(&link_path) {
                    if existing_target == target {
                        println!("    (already points to correct location)");
                        return Ok(());
                    }
                    println!(
                        "    Current: {} → {}",
                        link_path.display(),
                        existing_target.display()
                    );
                }
            }

            print!("    Overwrite? [y/N] ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if !input.trim().eq_ignore_ascii_case("y") {
                println!("    Skipped");
                return Ok(());
            }
        }

        if link_path.is_symlink() {
            fs::remove_file(&link_path)?;
        } else if link_path.is_dir() {
            fs::remove_dir_all(&link_path)?;
        }
    }

    unix_fs::symlink(target, &link_path)
        .context(format!("Failed to create symlink for {}", distro))?;

    println!("  ✓ /opt/ros/{} → {}", distro, target.display());
    Ok(())
}

pub fn remove_symlink(distro: &str) -> Result<()> {
    let link_path = get_ros_root().join(distro);

    if !link_path.exists() {
        anyhow::bail!("Distribution '{}' not found in /opt/ros", distro);
    }

    print!("Remove /opt/ros/{}? [y/N] ", distro);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Cancelled");
        return Ok(());
    }

    if link_path.is_symlink() {
        fs::remove_file(&link_path)?;
    } else if link_path.is_dir() {
        fs::remove_dir_all(&link_path)?;
    }

    println!("✓ Removed /opt/ros/{}", distro);

    if let Ok(distros) = scan_pixi_ros_installations() {
        for d in distros {
            if d.name == distro {
                println!("\nNote: The pixi installation remains at:");
                println!("  {}", d.path.display());
                println!("\nTo reinstall the symlink: rosenv setup");
                break;
            }
        }
    }

    Ok(())
}

pub fn cmd_cleanup() -> Result<()> {
    let distros = list_distributions()?;

    if distros.is_empty() {
        println!("No symlinks found in /opt/ros");
        return Ok(());
    }

    println!("Found symlinks:");
    for distro in &distros {
        println!("  - /opt/ros/{}", distro);
    }
    println!();

    print!("Remove all symlinks? [y/N] ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Cancelled");
        return Ok(());
    }

    for distro in &distros {
        let link_path = get_ros_root().join(distro);
        if link_path.is_symlink() {
            fs::remove_file(&link_path)?;
            println!("✓ Removed /opt/ros/{}", distro);
        }
    }

    println!("\nCleanup complete.");
    println!("\nNote: Pixi installations remain in ~/.pixi/envs/");
    println!("To recreate symlinks: rosenv setup");

    Ok(())
}

pub fn cmd_refresh() -> Result<()> {
    println!("Scanning for changes...\n");

    let existing = list_distributions()?;
    let pixi_distros = scan_pixi_ros_installations()?;

    if !existing.is_empty() {
        println!("Existing symlinks:");
        for distro in &existing {
            let link_path = get_ros_root().join(distro);
            if link_path.is_symlink() {
                if let Ok(target) = fs::read_link(&link_path) {
                    if target.exists() {
                        println!("  ✓ {}: up to date", distro);
                    } else {
                        println!("  ✗ {}: broken symlink", distro);
                    }
                }
            }
        }
        println!();
    }

    let mut new_distros = Vec::new();
    for pixi_distro in &pixi_distros {
        if !existing.contains(&pixi_distro.name) {
            new_distros.push(pixi_distro);
        }
    }

    if !new_distros.is_empty() {
        println!("New distributions found:");
        for distro in &new_distros {
            println!("  + {} → {}", distro.name, distro.path.display());
        }
        println!();

        check_opt_ros_writable()?;

        println!("Creating symlinks:");
        for distro in new_distros {
            create_symlink(&distro.name, &distro.path, false)?;
        }
    } else {
        println!("No new distributions found.");
    }

    println!("\nRefresh complete.");

    Ok(())
}

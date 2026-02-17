use anyhow::Result;
use std::fs;

use crate::distro::{get_ros_root, list_distributions};
use crate::symlink::check_opt_ros_writable;

pub fn cmd_doctor() -> Result<()> {
    println!("Checking ROS 2 environment setup...\n");

    let mut errors = 0;
    let mut warnings = 0;

    let ros_root = get_ros_root();
    if !ros_root.exists() {
        println!("✗ /opt/ros directory does not exist");
        println!("  Fix: sudo mkdir -p /opt/ros && sudo chown $USER /opt/ros\n");
        errors += 1;
    } else {
        println!("✓ /opt/ros directory exists");

        match check_opt_ros_writable() {
            Ok(_) => println!("✓ /opt/ros is writable"),
            Err(_) => {
                println!("✗ /opt/ros is not writable");
                println!("  Fix: sudo chown $USER /opt/ros\n");
                errors += 1;
            }
        }
    }

    let distros = list_distributions()?;
    if distros.is_empty() {
        println!("\n⚠ No distributions found in /opt/ros");
        println!("  Run: rosenv setup\n");
        warnings += 1;
    } else {
        println!("✓ Found {} distributions in /opt/ros\n", distros.len());

        for distro in &distros {
            println!("Distribution: {}", distro);
            let distro_path = ros_root.join(distro);

            if distro_path.is_symlink() {
                println!("  ✓ Symlink valid");

                match fs::read_link(&distro_path) {
                    Ok(target) => {
                        if target.exists() {
                            println!("  ✓ Target exists: {}", target.display());
                        } else {
                            println!("  ✗ Target does not exist: {}", target.display());
                            println!("    Fix: rosenv remove {} && rosenv setup", distro);
                            errors += 1;
                        }

                        let setup_zsh = target.join("setup.zsh");
                        let setup_bash = target.join("setup.bash");
                        if setup_zsh.exists() || setup_bash.exists() {
                            println!("  ✓ Setup files present");
                        } else {
                            println!("  ✗ Setup files missing");
                            errors += 1;
                        }

                        let bin_dir = target.join("bin");
                        let lib_dir = target.join("lib");
                        if bin_dir.exists() && lib_dir.exists() {
                            println!("  ✓ Binary and library directories exist");
                        } else {
                            println!("  ⚠ Some directories missing");
                            warnings += 1;
                        }
                    }
                    Err(_) => {
                        println!("  ✗ Could not read symlink");
                        errors += 1;
                    }
                }
            } else {
                println!("  ⚠ Not a symlink (regular directory)");
                warnings += 1;
            }
            println!();
        }
    }

    if let Some(home) = dirs::home_dir() {
        let zshrc = home.join(".zshrc");
        if zshrc.exists() {
            if let Ok(content) = fs::read_to_string(&zshrc) {
                if content.contains("ros-distro()") || content.contains("rosenv") {
                    println!("✓ Shell integration detected in ~/.zshrc");
                } else {
                    println!("⚠ Shell integration not found in ~/.zshrc");
                    println!("  Add: rosenv init zsh >> ~/.zshrc\n");
                    warnings += 1;
                }
            }
        }
    }

    println!();
    if errors == 0 && warnings == 0 {
        println!("All checks passed!");
    } else {
        if errors > 0 {
            println!("{} error(s) found", errors);
        }
        if warnings > 0 {
            println!("{} warning(s) found", warnings);
        }
    }

    Ok(())
}

use anyhow::Result;
use std::fs;

use crate::distro::{get_current_distro, get_ros_root, list_distributions, validate_distro};

const HELPERS: &str = include_str!("assets/helpers.sh");
const ACTIVATE_TEMPLATE: &str = include_str!("assets/activate.sh");
const DEACTIVATE: &str = include_str!("assets/deactivate.sh");
const INIT_TEMPLATE: &str = include_str!("assets/init.sh");

pub fn generate_activation_script(distro: &str, _shell: &str) -> Result<String> {
    let distro_path = validate_distro(distro)?;
    let ros_root = distro_path.display().to_string();

    let script = format!("{}\n{}", HELPERS, ACTIVATE_TEMPLATE)
        .replace("{distro}", distro)
        .replace("{ros_root}", &ros_root);

    Ok(script)
}

pub fn generate_deactivation_script() -> String {
    DEACTIVATE.to_string()
}

pub fn generate_shell_integration(shell: &str) -> String {
    INIT_TEMPLATE.replace("{shell}", shell)
}

pub fn cmd_list(names_only: bool, short: bool) -> Result<()> {
    let distros = list_distributions()?;

    if distros.is_empty() {
        if !names_only && !short {
            println!("No ROS distributions found in /opt/ros");
            println!("\nRun: rosenv setup");
        }
        return Ok(());
    }

    if short {
        println!("{}", distros.join(" "));
    } else if names_only {
        for distro in distros {
            println!("{}", distro);
        }
    } else {
        println!("Available ROS distributions:");
        let current = get_current_distro();

        for distro in distros {
            if Some(&distro) == current.as_ref() {
                println!("  * {} (active)", distro);
            } else {
                println!("    {}", distro);
            }
        }
    }

    Ok(())
}

pub fn cmd_status() -> Result<()> {
    match get_current_distro() {
        Some(distro) => {
            println!("ROS 2 {} is active\n", distro);

            println!("Environment:");
            if let Ok(ros_version) = std::env::var("ROS_VERSION") {
                println!("  ROS_VERSION:       {}", ros_version);
            }
            println!("  ROS_DISTRO:        {}", distro);

            if let Ok(ament) = std::env::var("AMENT_PREFIX_PATH") {
                println!("  AMENT_PREFIX_PATH: {}", ament);
            }

            let setup_path = get_ros_root().join(&distro).join("setup.zsh");
            if setup_path.exists() {
                println!("\nSetup file:");
                println!("  ✓ {}", setup_path.display());
            }
        }
        None => {
            println!("No ROS 2 distribution active\n");

            let distros = list_distributions()?;
            if !distros.is_empty() {
                println!("Available distributions:");
                for distro in distros {
                    println!("  - {}", distro);
                }
                println!("\nActivate: ros-distro <distro>");
            } else {
                println!("Run: rosenv setup");
            }
        }
    }

    Ok(())
}

pub fn cmd_info(distro: &str) -> Result<()> {
    let distro_path = validate_distro(distro)?;

    println!("Distribution: {}", distro);
    println!("Path:         {}", distro_path.display());

    if distro_path.is_symlink() {
        println!("Type:         Symlink");
        if let Ok(target) = fs::read_link(&distro_path) {
            println!("Target:       {}", target.display());
        }
    } else {
        println!("Type:         Directory");
    }

    println!("\nSetup files:");
    for setup in &["setup.bash", "setup.zsh", "setup.sh"] {
        let setup_path = distro_path.join(setup);
        if setup_path.exists() {
            println!("  ✓ {}", setup);
        } else {
            println!("  ✗ {}", setup);
        }
    }

    println!("\nKey directories:");
    for dir in &["bin", "lib", "share", "include"] {
        let dir_path = distro_path.join(dir);
        if dir_path.exists() {
            if let Ok(entries) = fs::read_dir(&dir_path) {
                let count = entries.count();
                println!("  ✓ {} ({} entries)", dir, count);
            } else {
                println!("  ✓ {}", dir);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_deactivation_script() {
        let script = generate_deactivation_script();

        assert!(script.contains("unset ROS_DISTRO"));
        assert!(script.contains("unset ROS_VERSION"));
        assert!(script.contains("unset AMENT_PREFIX_PATH"));
        assert!(script.contains("unset CMAKE_PREFIX_PATH"));
        assert!(script.contains("PATH="));
    }

    #[test]
    fn test_generate_shell_integration_zsh() {
        let script = generate_shell_integration("zsh");

        assert!(script.contains("rosenv()"));
        assert!(script.contains("rosenv init zsh"));
        assert!(script.contains("case \"$1\" in"));
        assert!(script.contains("activate)"));
        assert!(script.contains("deactivate)"));
        assert!(script.contains("status)"));
    }

    #[test]
    fn test_generate_shell_integration_bash() {
        let script = generate_shell_integration("bash");

        assert!(script.contains("rosenv init bash"));
        assert!(script.contains("rosenv()"));
    }

    #[test]
    fn test_shell_integration_contains_all_commands() {
        let script = generate_shell_integration("zsh");

        let required_elements = vec![
            "activate",
            "deactivate",
            "status",
            "command rosenv",
            "eval",
            "ROS_DISTRO",
        ];

        for element in required_elements {
            assert!(
                script.contains(element),
                "Shell integration missing: {}",
                element
            );
        }
    }

    #[test]
    fn test_deactivation_script_unsets_all_vars() {
        let script = generate_deactivation_script();

        let required_unsets = vec![
            "ROS_DISTRO",
            "ROS_VERSION",
            "ROS_PYTHON_VERSION",
            "AMENT_PREFIX_PATH",
            "CMAKE_PREFIX_PATH",
            "COLCON_PREFIX_PATH",
            "PYTHONPATH",
            "PKG_CONFIG_PATH",
        ];

        for var in required_unsets {
            assert!(
                script.contains(&format!("unset {}", var)),
                "Deactivation script missing unset for: {}",
                var
            );
        }
    }
}

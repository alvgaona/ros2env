use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Distribution {
    pub name: String,
    pub path: PathBuf,
}

pub fn get_ros_root() -> PathBuf {
    PathBuf::from("/opt/ros")
}

pub fn get_pixi_envs_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not determine home directory")
        .join(".pixi")
        .join("envs")
}

pub fn scan_pixi_ros_installations() -> Result<Vec<Distribution>> {
    let pixi_envs = get_pixi_envs_dir();

    if !pixi_envs.exists() {
        return Ok(Vec::new());
    }

    let mut distributions = Vec::new();

    for entry in fs::read_dir(&pixi_envs).context("Failed to read ~/.pixi/envs")? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let dir_name = path.file_name().unwrap().to_string_lossy();

        if dir_name.starts_with("ros-") {
            let parts: Vec<&str> = dir_name.split('-').collect();
            if parts.len() >= 2 {
                let distro_name = parts[1].to_string();

                let setup_bash = path.join("setup.bash");
                let setup_zsh = path.join("setup.zsh");

                if setup_bash.exists() || setup_zsh.exists() {
                    distributions.push(Distribution {
                        name: distro_name,
                        path: path.clone(),
                    });
                }
            }
        }
    }

    distributions.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(distributions)
}

pub fn list_distributions() -> Result<Vec<String>> {
    let ros_root = get_ros_root();

    if !ros_root.exists() {
        return Ok(Vec::new());
    }

    let mut distros = Vec::new();
    for entry in fs::read_dir(&ros_root).context("Failed to read /opt/ros")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() || path.is_symlink() {
            if let Some(name) = path.file_name() {
                distros.push(name.to_string_lossy().to_string());
            }
        }
    }

    distros.sort();
    Ok(distros)
}

pub fn get_current_distro() -> Option<String> {
    std::env::var("ROS_DISTRO").ok()
}

pub fn validate_distro(distro: &str) -> Result<PathBuf> {
    let path = get_ros_root().join(distro);
    if !path.exists() {
        anyhow::bail!("Distribution '{}' not found in /opt/ros", distro);
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribution_struct() {
        let distro = Distribution {
            name: "humble".to_string(),
            path: PathBuf::from("/test/path"),
        };

        assert_eq!(distro.name, "humble");
        assert_eq!(distro.path, PathBuf::from("/test/path"));
    }

    #[test]
    fn test_get_ros_root() {
        let root = get_ros_root();
        assert_eq!(root, PathBuf::from("/opt/ros"));
    }

    #[test]
    fn test_get_pixi_envs_dir() {
        let envs_dir = get_pixi_envs_dir();
        assert!(envs_dir.to_string_lossy().contains(".pixi"));
        assert!(envs_dir.to_string_lossy().contains("envs"));
    }

    #[test]
    fn test_get_current_distro_none() {
        std::env::remove_var("ROS_DISTRO");
        assert_eq!(get_current_distro(), None);
    }

    #[test]
    fn test_get_current_distro_set() {
        std::env::set_var("ROS_DISTRO", "humble");
        assert_eq!(get_current_distro(), Some("humble".to_string()));
        std::env::remove_var("ROS_DISTRO");
    }
}

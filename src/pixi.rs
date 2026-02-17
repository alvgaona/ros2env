use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::distro::get_ros_root;

const HELPERS: &str = include_str!("assets/helpers.sh");
const PIXI_BASE: &str = include_str!("assets/pixi_activate_base.sh");
const PIXI_GLOBAL: &str = include_str!("assets/pixi_activate_global.sh");

pub fn detect_pixi_ros_distro() -> Option<String> {
    let pixi_env = PathBuf::from(".pixi/envs/default/conda-meta");
    if !pixi_env.exists() {
        return None;
    }

    let distros = ["humble", "jazzy", "iron", "rolling"];

    for distro in &distros {
        if let Ok(entries) = fs::read_dir(&pixi_env) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("ros2-distro-mutex") && name.contains(distro) {
                    return Some(distro.to_string());
                }
            }
        }
    }

    for distro in &distros {
        if let Ok(entries) = fs::read_dir(&pixi_env) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with(&format!("ros-{}-", distro)) {
                    return Some(distro.to_string());
                }
            }
        }
    }

    None
}

fn generate_pixi_activate_script(distro: &str, append_global: bool) -> String {
    let ros_root = format!("/opt/ros/{}", distro);

    let mut script = String::new();
    script.push_str(HELPERS);
    script.push('\n');
    script.push_str(PIXI_BASE);

    if append_global {
        script.push('\n');
        script.push_str(PIXI_GLOBAL);
    }

    script.push_str("\nunset -f _rosenv_strip _rosenv_append\n");

    script
        .replace("{distro}", distro)
        .replace("{ros_root}", &ros_root)
}

pub fn cmd_pixi_activate() -> Result<()> {
    let pixi_distro = detect_pixi_ros_distro();

    let mut script = String::new();

    match pixi_distro.as_deref() {
        Some(distro) => {
            let global_path = get_ros_root().join(distro);
            if global_path.exists() {
                script.push_str(&format!(
                    "# rosenv: pixi has ROS {distro}, appending global /opt/ros/{distro}\n"
                ));
                script.push_str(&generate_pixi_activate_script(distro, true));
            } else {
                script.push_str(&format!(
                    "# rosenv: pixi has ROS {distro}, no matching global found\n"
                ));
                script.push_str(&generate_pixi_activate_script(distro, false));
            }
        }
        None => {
            script.push_str("# rosenv: no ROS detected in pixi environment\n");
        }
    }

    if PathBuf::from("install/setup.bash").exists() {
        script.push_str("source install/setup.bash\n");
        script.push_str("unset LD_LIBRARY_PATH\n");
        script.push_str("unset DYLD_LIBRARY_PATH\n");
    } else if PathBuf::from("install/setup.sh").exists() {
        script.push_str("source install/setup.sh\n");
        script.push_str("unset LD_LIBRARY_PATH\n");
        script.push_str("unset DYLD_LIBRARY_PATH\n");
    }

    print!("{}", script);
    Ok(())
}

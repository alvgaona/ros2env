use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::distro::get_ros_root;

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
    let mut script = String::new();

    script.push_str(
        r#"_rosenv_strip() {
  echo "$1" | tr ':' '\n' | grep -v "/opt/ros/" | tr '\n' ':' | sed 's/:$//'
}

_rosenv_append() {
  local var_name="$1" dir="$2"
  if [ -d "$dir" ]; then
    local current
    eval "current=\$$var_name"
    if [[ ":${current}:" != *":${dir}:"* ]]; then
      eval "export $var_name=\"${current:+${current}:}${dir}\""
    fi
  fi
}

"#,
    );

    script.push_str("# Strip inherited /opt/ros paths from parent shell\n");
    for var in &[
        "PATH",
        "PYTHONPATH",
        "PKG_CONFIG_PATH",
        "CMAKE_PREFIX_PATH",
        "AMENT_PREFIX_PATH",
    ] {
        script.push_str(&format!(
            "export {var}=$(_rosenv_strip \"${var}\")\n",
            var = var
        ));
    }

    script.push_str(&format!("\nexport ROS_DISTRO=\"{}\"\n", distro));

    if append_global {
        let ros_root = format!("/opt/ros/{}", distro);
        script.push_str(&format!("\n# Append global ROS {} paths\n", distro));
        script.push_str(&format!(
            "_rosenv_append AMENT_PREFIX_PATH \"{ros_root}\"\n"
        ));
        script.push_str(&format!(
            "_rosenv_append CMAKE_PREFIX_PATH \"{ros_root}\"\n"
        ));
        script.push_str(&format!("_rosenv_append PATH \"{ros_root}/bin\"\n"));
        script.push_str(&format!(
            "_rosenv_append PKG_CONFIG_PATH \"{ros_root}/lib/pkgconfig\"\n"
        ));

        script.push_str(&format!(
            "for _rosenv_pypath in \"{ros_root}\"/lib/python*/site-packages; do\n"
        ));
        script.push_str("  _rosenv_append PYTHONPATH \"$_rosenv_pypath\"\n");
        script.push_str("done\n");
        script.push_str("unset _rosenv_pypath\n");
    }

    script.push_str("\nunset -f _rosenv_strip _rosenv_append\n");

    script
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

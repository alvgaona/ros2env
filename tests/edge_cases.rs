use std::fs;
use std::os::unix::fs as unix_fs;
use tempfile::TempDir;

#[test]
fn test_broken_symlink_detection() {
    let temp_dir = TempDir::new().unwrap();
    let ros_root = temp_dir.path().join("opt").join("ros");
    fs::create_dir_all(&ros_root).unwrap();

    let non_existent_target = temp_dir.path().join("nonexistent");
    let broken_link = ros_root.join("broken");

    unix_fs::symlink(&non_existent_target, &broken_link).unwrap();

    assert!(!broken_link.exists());
    assert!(broken_link.symlink_metadata().is_ok());
}

#[test]
fn test_empty_pixi_envs_directory() {
    let temp_dir = TempDir::new().unwrap();
    let pixi_envs = temp_dir.path().join(".pixi").join("envs");
    fs::create_dir_all(&pixi_envs).unwrap();

    let entries: Vec<_> = fs::read_dir(&pixi_envs).unwrap().collect();
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_multiple_ros_versions() {
    let temp_dir = TempDir::new().unwrap();
    let pixi_envs = temp_dir.path().join(".pixi").join("envs");

    for distro in &["humble", "iron", "jazzy", "rolling"] {
        let env_dir = pixi_envs.join(format!("ros-{}-desktop", distro));
        fs::create_dir_all(&env_dir).unwrap();
        fs::write(env_dir.join("setup.bash"), "# setup").unwrap();
    }

    let entries: Vec<_> = fs::read_dir(&pixi_envs)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("ros-")
        })
        .collect();

    assert_eq!(entries.len(), 4);
}

#[test]
fn test_pixi_env_without_setup_files() {
    let temp_dir = TempDir::new().unwrap();
    let env_dir = temp_dir
        .path()
        .join(".pixi")
        .join("envs")
        .join("ros-humble-desktop");
    fs::create_dir_all(&env_dir).unwrap();

    assert!(!env_dir.join("setup.bash").exists());
    assert!(!env_dir.join("setup.zsh").exists());
}

#[test]
fn test_pixi_env_with_only_bash_setup() {
    let temp_dir = TempDir::new().unwrap();
    let env_dir = temp_dir
        .path()
        .join(".pixi")
        .join("envs")
        .join("ros-humble-desktop");
    fs::create_dir_all(&env_dir).unwrap();
    fs::write(env_dir.join("setup.bash"), "# bash setup").unwrap();

    assert!(env_dir.join("setup.bash").exists());
    assert!(!env_dir.join("setup.zsh").exists());
}

#[test]
fn test_pixi_env_with_only_zsh_setup() {
    let temp_dir = TempDir::new().unwrap();
    let env_dir = temp_dir
        .path()
        .join(".pixi")
        .join("envs")
        .join("ros-jazzy-desktop");
    fs::create_dir_all(&env_dir).unwrap();
    fs::write(env_dir.join("setup.zsh"), "# zsh setup").unwrap();

    assert!(!env_dir.join("setup.bash").exists());
    assert!(env_dir.join("setup.zsh").exists());
}

#[test]
fn test_non_ros_pixi_environments() {
    let temp_dir = TempDir::new().unwrap();
    let pixi_envs = temp_dir.path().join(".pixi").join("envs");

    fs::create_dir_all(pixi_envs.join("python-312")).unwrap();
    fs::create_dir_all(pixi_envs.join("nodejs-20")).unwrap();
    fs::create_dir_all(pixi_envs.join("rust-stable")).unwrap();

    let ros_like: Vec<_> = fs::read_dir(&pixi_envs)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("ros-")
        })
        .collect();

    assert_eq!(ros_like.len(), 0);
}

#[test]
fn test_ros_env_with_incomplete_name() {
    let temp_dir = TempDir::new().unwrap();
    let pixi_envs = temp_dir.path().join(".pixi").join("envs");

    fs::create_dir_all(pixi_envs.join("ros-")).unwrap();
    fs::create_dir_all(pixi_envs.join("ros")).unwrap();

    let ros_envs: Vec<_> = fs::read_dir(&pixi_envs)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| {
            let name = e.path().file_name().unwrap().to_string_lossy().to_string();
            let parts: Vec<&str> = name.split('-').collect();
            name.starts_with("ros-") && parts.len() >= 3 && !parts[1].is_empty()
        })
        .collect();

    assert_eq!(ros_envs.len(), 0);
}

#[test]
fn test_symlink_pointing_to_symlink() {
    let temp_dir = TempDir::new().unwrap();
    let target = temp_dir.path().join("actual_target");
    fs::create_dir_all(&target).unwrap();

    let intermediate = temp_dir.path().join("intermediate");
    unix_fs::symlink(&target, &intermediate).unwrap();

    let final_link = temp_dir.path().join("final");
    unix_fs::symlink(&intermediate, &final_link).unwrap();

    assert!(final_link.exists());
    assert!(fs::read_link(&final_link).unwrap() == intermediate);
}

#[test]
fn test_special_characters_in_paths() {
    let temp_dir = TempDir::new().unwrap();
    let special_dir = temp_dir.path().join("path with spaces");
    fs::create_dir_all(&special_dir).unwrap();

    assert!(special_dir.exists());
}

#[test]
fn test_concurrent_symlink_creation() {
    let temp_dir = TempDir::new().unwrap();
    let ros_root = temp_dir.path().join("opt").join("ros");
    fs::create_dir_all(&ros_root).unwrap();

    let target = temp_dir.path().join("target");
    fs::create_dir_all(&target).unwrap();

    let link1 = ros_root.join("link1");
    let link2 = ros_root.join("link2");

    unix_fs::symlink(&target, &link1).unwrap();
    unix_fs::symlink(&target, &link2).unwrap();

    assert!(link1.exists());
    assert!(link2.exists());
}

#[test]
fn test_very_long_path_names() {
    let temp_dir = TempDir::new().unwrap();
    let long_name = "a".repeat(200);
    let long_path = temp_dir.path().join(long_name);

    fs::create_dir_all(&long_path).unwrap();
    assert!(long_path.exists());
}

#[test]
fn test_empty_setup_files() {
    let temp_dir = TempDir::new().unwrap();
    let env_dir = temp_dir.path().join("env");
    fs::create_dir_all(&env_dir).unwrap();

    fs::write(env_dir.join("setup.bash"), "").unwrap();
    fs::write(env_dir.join("setup.zsh"), "").unwrap();

    let bash_content = fs::read_to_string(env_dir.join("setup.bash")).unwrap();
    let zsh_content = fs::read_to_string(env_dir.join("setup.zsh")).unwrap();

    assert_eq!(bash_content.len(), 0);
    assert_eq!(zsh_content.len(), 0);
}

#[test]
fn test_binary_files_named_setup() {
    let temp_dir = TempDir::new().unwrap();
    let env_dir = temp_dir.path().join("env");
    fs::create_dir_all(&env_dir).unwrap();

    fs::write(env_dir.join("setup.bash"), [0u8, 1, 2, 3, 255]).unwrap();

    let content = fs::read(env_dir.join("setup.bash")).unwrap();
    assert_eq!(content, vec![0, 1, 2, 3, 255]);
}

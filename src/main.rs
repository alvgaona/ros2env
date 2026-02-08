use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "rosenv")]
#[command(version, about = "ROS 2 distribution environment manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Auto-detect pixi ROS installations and create symlinks
    Setup,

    /// List available ROS distributions
    List {
        /// Only output distribution names (for scripting)
        #[arg(long)]
        names_only: bool,

        /// Output short format (space-separated names)
        #[arg(long)]
        short: bool,
    },

    /// Show current active distribution
    Status,

    /// Generate shell commands to activate a distribution
    Activate {
        /// Distribution name (e.g., humble, jazzy)
        distro: String,
    },

    /// Generate shell commands to deactivate ROS environment
    Deactivate,

    /// Show information about a distribution
    Info {
        /// Distribution name
        distro: String,
    },

    /// Generate shell integration code
    Init {
        /// Shell type (zsh, bash)
        shell: String,
    },

    /// Remove a distribution symlink
    Remove {
        /// Distribution name
        distro: String,
    },

    /// Remove all distribution symlinks
    Cleanup,

    /// Refresh/update all symlinks
    Refresh,

    /// Verify installation and diagnose issues
    Doctor,

    /// Show guide for installing ROS 2 with pixi global
    #[command(name = "setup-guide")]
    SetupGuide,
}

#[derive(Debug)]
struct Distribution {
    name: String,
    path: PathBuf,
}

fn get_ros_root() -> PathBuf {
    PathBuf::from("/opt/ros")
}

fn get_pixi_envs_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not determine home directory")
        .join(".pixi")
        .join("envs")
}

fn scan_pixi_ros_installations() -> Result<Vec<Distribution>> {
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

        // Match ros-*-* pattern (e.g., ros-humble-desktop, ros-jazzy-desktop)
        if dir_name.starts_with("ros-") {
            // Extract distro name (second component)
            let parts: Vec<&str> = dir_name.split('-').collect();
            if parts.len() >= 2 {
                let distro_name = parts[1].to_string();

                // Verify it has setup files
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

fn list_distributions() -> Result<Vec<String>> {
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

fn get_current_distro() -> Option<String> {
    std::env::var("ROS_DISTRO").ok()
}

fn validate_distro(distro: &str) -> Result<PathBuf> {
    let path = get_ros_root().join(distro);
    if !path.exists() {
        anyhow::bail!("Distribution '{}' not found in /opt/ros", distro);
    }
    Ok(path)
}

fn check_opt_ros_writable() -> Result<()> {
    let ros_root = get_ros_root();

    if !ros_root.exists() {
        anyhow::bail!(
            "/opt/ros does not exist\n\nRun these commands first:\n  sudo mkdir -p /opt/ros\n  sudo chown $USER /opt/ros\n\nThen: rosenv setup"
        );
    }

    // Test if writable by trying to create a temporary file
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

fn create_symlink(distro: &str, target: &Path, force: bool) -> Result<()> {
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

        // Remove existing symlink/directory
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

fn remove_symlink(distro: &str) -> Result<()> {
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

    // Show note about pixi installation
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

fn generate_activation_script(distro: &str, shell: &str) -> Result<String> {
    let distro_path = validate_distro(distro)?;

    let setup_file = match shell {
        "zsh" => "setup.zsh",
        "bash" => "setup.bash",
        _ => "setup.sh",
    };

    let setup_path = distro_path.join(setup_file);
    if !setup_path.exists() {
        anyhow::bail!("Setup file not found: {}", setup_path.display());
    }

    // Generate cleanup and activation commands
    let mut script = String::new();

    // Clean up previous ROS environment
    script.push_str("# Clean up previous ROS environment\n");
    script.push_str("if [ -n \"$ROS_DISTRO\" ]; then\n");
    script.push_str(
        "  export PATH=$(echo $PATH | tr ':' '\\n' | grep -v '/opt/ros/' | tr '\\n' ':')\n",
    );
    script.push_str("  unset AMENT_PREFIX_PATH\n");
    script.push_str("  unset CMAKE_PREFIX_PATH\n");
    script.push_str("  unset COLCON_PREFIX_PATH\n");
    script.push_str("  unset PYTHONPATH\n");
    script.push_str("  unset LD_LIBRARY_PATH\n");
    script.push_str("  unset DYLD_LIBRARY_PATH\n");
    script.push_str("  unset PKG_CONFIG_PATH\n");
    script.push_str("fi\n\n");

    // Source new distribution
    script.push_str(&format!("# Activate ROS 2 {}\n", distro));
    script.push_str(&format!("export ROS_DISTRO={}\n", distro));
    script.push_str(&format!("source {}\n", setup_path.display()));

    Ok(script)
}

fn generate_deactivation_script() -> String {
    r#"# Deactivate ROS 2 environment
export PATH=$(echo $PATH | tr ':' '\n' | grep -v '/opt/ros/' | tr '\n' ':')
unset ROS_DISTRO
unset ROS_VERSION
unset ROS_PYTHON_VERSION
unset AMENT_PREFIX_PATH
unset CMAKE_PREFIX_PATH
unset COLCON_PREFIX_PATH
unset PYTHONPATH
unset LD_LIBRARY_PATH
unset DYLD_LIBRARY_PATH
unset PKG_CONFIG_PATH
"#
    .to_string()
}

fn generate_shell_integration(shell: &str) -> String {
    format!(
        r#"# ROS 2 Environment Manager (rosenv)
# Generated by: rosenv init {}

rosenv() {{
    case "$1" in
        activate)
            if [ -z "$2" ]; then
                echo "Error: rosenv activate requires a distribution name"
                echo "Available: $(command rosenv list --short 2>/dev/null || echo 'run rosenv setup')"
                return 1
            fi
            
            local script
            script=$(command rosenv activate "$2" 2>&1)
            if [ $? -eq 0 ]; then
                eval "$script"
                echo "✓ Switched to ROS 2 $2"
            else
                echo "$script" >&2
                return 1
            fi
            ;;
            
        deactivate)
            if [ -z "$ROS_DISTRO" ]; then
                echo "No ROS distribution active"
                return 1
            fi
            
            local distro="$ROS_DISTRO"
            eval "$(command rosenv deactivate)"
            echo "✓ Deactivated ROS 2 $distro"
            ;;
            
        status)
            if [ -n "$ROS_DISTRO" ]; then
                echo "ROS 2 $ROS_DISTRO is active"
                echo ""
                [ -n "$ROS_VERSION" ] && echo "  ROS_VERSION:       $ROS_VERSION"
                echo "  ROS_DISTRO:        $ROS_DISTRO"
                [ -n "$AMENT_PREFIX_PATH" ] && echo "  AMENT_PREFIX_PATH: ${{AMENT_PREFIX_PATH%%:*}}..."
                
                if command -v ros2 &>/dev/null; then
                    echo ""
                    echo "ROS 2 CLI:"
                    echo "  ✓ $(which ros2)"
                fi
            else
                command rosenv status
            fi
            ;;
            
        *)
            command rosenv "$@"
            ;;
    esac
}}

# Auto-activate default distribution on shell startup (optional)
# rosenv activate humble >/dev/null 2>&1
"#,
        shell
    )
}

fn cmd_setup() -> Result<()> {
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

    // Check permissions
    println!("Checking /opt/ros permissions...");
    check_opt_ros_writable()?;
    println!("✓ /opt/ros is writable\n");

    // Create symlinks
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

fn cmd_list(names_only: bool, short: bool) -> Result<()> {
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

fn cmd_status() -> Result<()> {
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

            // Show path to setup file
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

fn cmd_info(distro: &str) -> Result<()> {
    let distro_path = validate_distro(distro)?;

    println!("Distribution: {}", distro);
    println!("Path:         {}", distro_path.display());

    // Check if it's a symlink
    if distro_path.is_symlink() {
        println!("Type:         Symlink");
        if let Ok(target) = fs::read_link(&distro_path) {
            println!("Target:       {}", target.display());
        }
    } else {
        println!("Type:         Directory");
    }

    // Check for setup files
    println!("\nSetup files:");
    for setup in &["setup.bash", "setup.zsh", "setup.sh"] {
        let setup_path = distro_path.join(setup);
        if setup_path.exists() {
            println!("  ✓ {}", setup);
        } else {
            println!("  ✗ {}", setup);
        }
    }

    // Show some key directories
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

fn cmd_cleanup() -> Result<()> {
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

fn cmd_refresh() -> Result<()> {
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

    // Find new distributions
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

fn cmd_doctor() -> Result<()> {
    println!("Checking ROS 2 environment setup...\n");

    let mut errors = 0;
    let mut warnings = 0;

    // Check /opt/ros
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

    // Check distributions
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

                        // Check setup files
                        let setup_zsh = target.join("setup.zsh");
                        let setup_bash = target.join("setup.bash");
                        if setup_zsh.exists() || setup_bash.exists() {
                            println!("  ✓ Setup files present");
                        } else {
                            println!("  ✗ Setup files missing");
                            errors += 1;
                        }

                        // Check key directories
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

    // Check shell integration
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

    // Summary
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

fn cmd_setup_guide() -> Result<()> {
    const GUIDE_URL: &str = "https://github.com/alvgaona/ros2env/blob/main/SETUP_GUIDE.md";

    println!("Opening ROS 2 Setup Guide in your browser...\n");
    println!("URL: {}\n", GUIDE_URL);

    // Detect platform and use appropriate command to open URL
    let open_cmd = if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "linux") {
        "xdg-open"
    } else {
        anyhow::bail!("Unsupported platform for opening URLs");
    };

    match std::process::Command::new(open_cmd).arg(GUIDE_URL).spawn() {
        Ok(_) => {
            println!("✓ Setup guide opened in your default browser");
            println!("\nIf the browser didn't open, visit:");
            println!("  {}", GUIDE_URL);
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ Failed to open browser: {}", e);
            println!("\nPlease visit the guide manually:");
            println!("  {}", GUIDE_URL);
            Ok(())
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup => cmd_setup(),
        Commands::List { names_only, short } => cmd_list(names_only, short),
        Commands::Status => cmd_status(),
        Commands::Activate { distro } => {
            // Detect shell from parent process or default to bash
            let shell = std::env::var("SHELL")
                .ok()
                .and_then(|s| s.split('/').next_back().map(String::from))
                .unwrap_or_else(|| "bash".to_string());

            match generate_activation_script(&distro, &shell) {
                Ok(script) => {
                    print!("{}", script);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Deactivate => {
            print!("{}", generate_deactivation_script());
            Ok(())
        }
        Commands::Info { distro } => cmd_info(&distro),
        Commands::Init { shell } => {
            println!("{}", generate_shell_integration(&shell));
            Ok(())
        }
        Commands::Remove { distro } => remove_symlink(&distro),
        Commands::Cleanup => cmd_cleanup(),
        Commands::Refresh => cmd_refresh(),
        Commands::Doctor => cmd_doctor(),
        Commands::SetupGuide => cmd_setup_guide(),
    }
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
            "LD_LIBRARY_PATH",
            "DYLD_LIBRARY_PATH",
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

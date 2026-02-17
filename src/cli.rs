use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rosenv")]
#[command(version, about = "ROS 2 distribution environment manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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

    /// Generate activation script for pixi workspaces
    Pixi {
        #[command(subcommand)]
        command: PixiCommands,
    },
}

#[derive(Subcommand)]
pub enum PixiCommands {
    /// Generate activation script for pixi workspace
    Activate,
}

mod cli;
mod distro;
mod doctor;
mod pixi;
mod setup;
mod shell;
mod symlink;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, PixiCommands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup => setup::cmd_setup(),
        Commands::List { names_only, short } => shell::cmd_list(names_only, short),
        Commands::Status => shell::cmd_status(),
        Commands::Activate { distro } => {
            let sh = std::env::var("SHELL")
                .ok()
                .and_then(|s| s.split('/').next_back().map(String::from))
                .unwrap_or_else(|| "bash".to_string());

            let script = shell::generate_activation_script(&distro, &sh)?;
            print!("{}", script);
            Ok(())
        }
        Commands::Deactivate => {
            print!("{}", shell::generate_deactivation_script());
            Ok(())
        }
        Commands::Info { distro } => shell::cmd_info(&distro),
        Commands::Init { shell: sh } => {
            println!("{}", shell::generate_shell_integration(&sh));
            Ok(())
        }
        Commands::Remove { distro } => symlink::remove_symlink(&distro),
        Commands::Cleanup => symlink::cmd_cleanup(),
        Commands::Refresh => symlink::cmd_refresh(),
        Commands::Doctor => doctor::cmd_doctor(),
        Commands::SetupGuide => setup::cmd_setup_guide(),
        Commands::Pixi { command } => match command {
            PixiCommands::Activate => pixi::cmd_pixi_activate(),
        },
    }
}

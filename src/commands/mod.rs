pub mod gas;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Gas-related commands
    #[command(subcommand)]
    Gas(gas::GasCommands),
}

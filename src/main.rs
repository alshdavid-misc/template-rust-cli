mod cmd;

use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[command(version)]
struct Command {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  /// Update the binary to the latest release
  #[clap(name = "upgrade", visible_aliases = ["up", "u"])]
  Upgrade(cmd::upgrade::UpgradeCommand),
}

fn main() -> anyhow::Result<()> {
  match Command::parse().command {
    Commands::Upgrade(args) => cmd::upgrade::main(&args),
  }
}

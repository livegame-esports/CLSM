use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, about, version, long_about = None)]
pub struct CLI {
    /// Server actions
    #[clap(subcommand)]
    pub action: Actions,
}

/// -----------------------------------
/// Available commands:
/// - start <start a server>
/// - stop <stop a server>
/// - restart <re-start a server>
/// -----------------------------------
#[derive(Subcommand, Debug)]
pub enum Actions {
    /// Start server
    #[clap(arg_required_else_help = true)]
    Start(StartCommand),
}

#[derive(Args, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct StartCommand {
    /// Enter server port number | e.g: 27015
    #[clap(short, long)]
    pub port: u16,
}

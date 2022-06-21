/// ----------------------------------------------------------------------------------------
/// CLSM - Command Line Server Manager                                                     |
/// Licensed at MIT - https://github.com/livegame-esports/CLSM/blob/production/LICENSE     |
/// ----------------------------------------------------------------------------------------
use clap::Parser;
use clsm::cli::{Actions, CLI};

fn main() {
    let args = CLI::parse();

    match args.action {
        Actions::Start(res) => println!("{:?}", res.port),
    }
}

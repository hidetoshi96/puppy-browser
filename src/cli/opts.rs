//! This module defines options of `puppy` command.

use super::subcommand::*;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
pub struct Opts {
    #[command(flatten)]
    pub common_opts: CommonOpts,

    #[command(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Args, Debug)]
pub struct CommonOpts {
    #[command(flatten)]
    pub verbose: Verbosity,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Open(open::Opts),
    #[command(subcommand)]
    Completion(completion::Opts),
    #[command(name = "js")]
    JavaScript,
}

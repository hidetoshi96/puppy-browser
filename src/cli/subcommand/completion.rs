//! This module defines `completion` subcommand.

use std::io;

use clap::{CommandFactory, Subcommand};
use clap_complete::{generate, Shell};

use crate::cli::CommonOpts;

/// `Opts` defines possible options for the `completion` subcommand.
#[derive(Subcommand, Debug)]
pub enum Opts {
    Zsh,
    Bash,
    Fish,
}

/// `run` emits a completion script for some shell environments.
pub fn run(_common_opts: CommonOpts, opts: Opts) -> i32 {
    let shell = match opts {
        Opts::Bash => Shell::Bash,
        Opts::Zsh => Shell::Zsh,
        Opts::Fish => Shell::Fish,
    };
    completion(shell);

    return 0;
}

fn completion(s: Shell) {
    let mut app = super::super::Opts::command();
    generate(s, &mut app, env!("CARGO_PKG_NAME"), &mut io::stdout())
}

use clap::Parser;
use puppy::cli;

/// `main` is an entrypoint of puppy.
fn main() {
    let opts: cli::Opts = cli::Opts::parse();

    let exit_code = match opts.sub_command {
        cli::SubCommand::Open(sub_opts) => cli::subcommand::open::run(opts.common_opts, sub_opts),
        cli::SubCommand::JavaScript => cli::subcommand::javascript::run(opts.common_opts),
        cli::SubCommand::Completion(sub_opts) => {
            cli::subcommand::completion::run(opts.common_opts, sub_opts)
        }
    };

    std::process::exit(exit_code)
}

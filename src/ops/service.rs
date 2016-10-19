
#[macro_use]
extern crate clap;

use super::Operation;
use clap::{ Subcommand, Arg, ArgMatches };

impl Operation {

	fn build_args() -> Subcommand {

		return SubCommand::with_name("service")
				.about("Manages the running daemon that handles project fetching and commit hooks.")

				.subcommand(SubCommand::with_name("start")
						.about("Starts the daemon"))

				.subcommand(SubCommand::with_name("stop")
						.about("Stops the daemon"))

				.subcommand(SubCommand::with_name("install")
						.about("Installs the daemon"))

				.subcommand(SubCommand::with_name("uninstall")
						.about("Uninstalls the daemon"))

				.subcommand(SubCommand::with_name("status")
						.about("Displays the status of the daemon (started/stopped, etc.)"));

	}

	fn execute(args: ArgMatches) {
		//
	}

}

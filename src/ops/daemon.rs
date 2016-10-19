/***/

#[macro_use]
extern crate clap;

use super::Operation;
use clap::{ Subcommand, Arg, ArgMatches };

impl Operation {

	fn build_args() -> Subcommand {

		return SubCommand::with_name("daemon")
			.about("Runs the gin management daemon");

	}

	fn execute(args: ArgMatches) {
		//
	}

}

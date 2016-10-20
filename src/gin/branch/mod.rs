/***/

use clap::{ App, SubCommand, Arg, ArgMatches };

#[macro_use]
use util::println_stderr;

pub fn build_args<'a, 'b>() -> App<'a, 'b> {

	return SubCommand::with_name("branch")
		.about("Handles branch operations")
		.version("0.1")

		.arg(Arg::with_name("source")
			.help("The repository source (github, bitbucket, custom, etc).")
			.takes_value(true)
			.index(1))

		.arg(Arg::with_name("repo")
			.help("The repository to check out")
			.takes_value(true)
			.index(2));

}

pub fn execute(args: &ArgMatches) {
	match args.subcommand() {
		(&_, _)                    => list_branches(&args)
	}
}

fn list_branches(args: &ArgMatches) {
	println!("Listing Branches");
	// TODO: get list of branches, with tracking and descriptions.
	// TODO: Print to stdout with colours (ideally fetched from .gitconfig)
	println_stderr!("Not actually implemented!")
}

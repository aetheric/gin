/***/

use clap::{ App, SubCommand, Arg, ArgMatches };

pub fn build_args<'a, 'b>() -> App<'a, 'b> {

	return SubCommand::with_name("checkout")
		.about("Checks a copy of a repo out to a directory")
		.version("0.1")
		.aliases(&[ "co" ])

		.arg(Arg::with_name("source")
			.help("The repository source (github, bitbucket, custom, etc).")
			.takes_value(true)
			.index(1))

		.arg(Arg::with_name("repo")
			.help("The repository to check out")
			.required(true)
			.takes_value(true)
			.index(2));

}

pub fn execute(args: &ArgMatches) {
	unimplemented!();
}

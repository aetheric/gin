/***/

use clap::{ App, SubCommand, Arg, ArgMatches };

pub fn build_args<'a, 'b>() -> App<'a, 'b> {

	return SubCommand::with_name("commit")
		.about("Commits staged code to the graph")
		.version("0.1")
		.aliases(&[ "cm" ]);

}

pub fn execute(args: &ArgMatches) {
	unimplemented!();
}

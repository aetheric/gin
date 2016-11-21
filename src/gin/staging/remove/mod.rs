//!

use clap::{ App, SubCommand, Arg, ArgMatches };

pub fn build_args<'a, 'b>() -> App<'a, 'b> {

	return SubCommand::with_name("remove")
		.about("Removes files from the index")
		.version("0.1")
		.aliases(&[ "rem", "--" ])

}

pub fn execute(args: &ArgMatches) {
	unimplemented!();
}

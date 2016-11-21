//!

use clap::{ App, SubCommand, ArgMatches };

pub fn build_args<'a, 'b>() -> App<'a, 'b> {

	return SubCommand::with_name("add")
		.about("Adds files to the index")
		.version("0.1")
		.aliases(&[ "++" ])

}

pub fn execute(args: &ArgMatches) {
	unimplemented!();
}

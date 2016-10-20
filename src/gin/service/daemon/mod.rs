/***/

use clap::{ App, SubCommand, ArgMatches };

pub fn build_args<'a, 'b>() -> App<'a, 'b> {
	return SubCommand::with_name("daemon")
		.about("Runs the gin service. Should be run in separate thread.");
}

pub fn execute(args: &ArgMatches) {
	unimplemented!();
}

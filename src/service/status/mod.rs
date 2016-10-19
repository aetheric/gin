/***/

use clap::{ App, SubCommand, ArgMatches };

pub fn build_args<'a, 'b>() -> App<'a, 'b> {
	return SubCommand::with_name("status")
		.about("Displays the status of the daemon (started/stopped, etc.)");
}

pub fn execute(args: &ArgMatches) {
	unimplemented!();
}

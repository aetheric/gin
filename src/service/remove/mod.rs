/***/

use clap::{ App, SubCommand, ArgMatches };

pub fn build_args<'a, 'b>() -> App<'a, 'b> {
	return SubCommand::with_name("remove")
		.about("Uninstalls the daemon");
}

pub fn execute(args: &ArgMatches) {
	unimplemented!();
}

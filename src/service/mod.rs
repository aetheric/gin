/***/

mod daemon;
mod install;
mod remove;
mod start;
mod status;
mod stop;

use clap::{ App, SubCommand, ArgMatches };

pub fn build_args<'a, 'b>() -> App<'a, 'b> {

	return SubCommand::with_name("service")
		.about("Manages the running daemon that handles project fetching and commit hooks.")

		.subcommand(daemon::build_args())
		.subcommand(install::build_args())
		.subcommand(remove::build_args())
		.subcommand(start::build_args())
		.subcommand(start::build_args())
		.subcommand(stop::build_args());

}

pub fn execute(args: &ArgMatches) {
	match args.subcommand() {
		("daemon",  Some(sub_m))  => daemon::execute(&sub_m),
		("install",  Some(sub_m)) => install::execute(&sub_m),
		("remove",  Some(sub_m))  => remove::execute(&sub_m),
		("start",  Some(sub_m))   => start::execute(&sub_m),
		("status",  Some(sub_m))  => status::execute(&sub_m),
		("stop",  Some(sub_m))    => stop::execute(&sub_m),
		(&_, _)                   => ()
	}
}

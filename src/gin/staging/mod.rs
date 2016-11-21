/***/

mod add;
mod remove;

use clap::{ App, SubCommand, Arg, ArgMatches };

pub fn build_args<'a, 'b>() -> App<'a, 'b> {

	return SubCommand::with_name("staging")
		.about("Manipulates the index")
		.version("0.1")
		.aliases(&[ "stage", ])

		.subcommand(add::build_args())
		.subcommand(remove::build_args());

}

pub fn execute(args: &ArgMatches) {

	/// Figure out what subcommand (if any) has been called.
	match args.subcommand() {
		("add",    Some(sub_m)) => add::execute(&sub_m),
		("remove", Some(sub_m)) => remove::execute(&sub_m),
		(&_, _)                 => println!("{}", args.usage())
	}

}

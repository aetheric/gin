/***/

mod github;
mod gitlab;
mod jira;

use clap::{ App, SubCommand, Arg, ArgMatches };

pub fn build_args<'a, 'b>() -> App<'a, 'b> {

	return SubCommand::with_name("link")
		.about("Links a service to gin functions")
		.version("0.1")
		.aliases(&[ "ln", ])

		.subcommand(github::build_args())
		.subcommand(gitlab::build_args())
		.subcommand(jira::build_args());

}

pub fn execute(args: &ArgMatches) {

	/// Figure out what subcommand (if any) has been called.
	match args.subcommand() {
		("github", Some(sub_m)) => github::execute(&sub_m),
		("gitlab", Some(sub_m)) => gitlab::execute(&sub_m),
		("jira",   Some(sub_m)) => jira::execute(&sub_m),
		(&_, _)                 => println!("{}", args.usage())
	}

}

/***/

mod branch;
mod checkout;
mod service;

use clap::{ App, Arg, ArgMatches };
use log4rs::config::{ Config, Appender, Root };
use log4rs::append::console::ConsoleAppender;
use log::LogLevelFilter;

pub fn build_args<'a, 'b>() -> App<'a, 'b> {

	return App::new(env!("CARGO_PKG_NAME"))
		.version(crate_version!())
		.author(crate_authors!())
		.about(env!("CARGO_PKG_DESCRIPTION"))

		.arg(Arg::with_name("verbose")
			.help("Sets the level of logging.")
			.long("verbose")
			.short("v")
			.multiple(true))

		.arg(Arg::with_name("quiet")
			.help("Decreases the level of logging.")
			.long("quiet")
			.short("q")
			.multiple(true))

		.subcommand(branch::build_args())
		.subcommand(checkout::build_args())
		.subcommand(service::build_args());

}

pub fn execute(args: &ArgMatches) {

	let log_level = ( args.occurrences_of("v") - args.occurrences_of("q") ) as i32;
	build_logging(match log_level {
		-3 => LogLevelFilter::Off,
		-2 => LogLevelFilter::Error,
		-1 => LogLevelFilter::Warn,
		 0 => LogLevelFilter::Info,
		 1 => LogLevelFilter::Debug,
		 _ => LogLevelFilter::Trace
	});

	match args.subcommand() {
		("checkout",  Some(sub_m)) => checkout::execute(&sub_m),
		("service",  Some(sub_m))  => service::execute(&sub_m),
		("branch",  Some(sub_m))   => branch::execute(&sub_m),
		(&_, _)                    => println!("{}", args.usage())
	}

}

fn build_logging(level: LogLevelFilter) -> Config {

	// This needs to be like this for the Box::new invocation.
	let stdout = ConsoleAppender::builder().build();

	return Config::builder()

		.appender(Appender::builder()
			.build("stdout", Box::new(stdout)))

		.build(Root::builder()
			.appender("stdout")
			.build(level))
		.unwrap();

}

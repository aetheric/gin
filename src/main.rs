/***/

#![feature(plugin)]
#![feature(test)]
#![cfg_attr(test, plugin(stainless))]

#[cfg(test)]
mod main_tests;

mod branch;
mod checkout;
mod service;

#[cfg(test)]
extern crate test;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate clap;

use clap::{ App, Arg };
use log4rs::config::{ Config, Appender, Root };
use log4rs::append::console::ConsoleAppender;
use log::LogLevelFilter;

fn main() {

	// Parse the console arguments and attempt to make use of them.
	let args = build_args()
		.get_matches();

	// Initialise logging from the logging config file.
	build_logging(match args.occurrences_of("v") {
		0 => LogLevelFilter::Warn,
		1 => LogLevelFilter::Info,
		2 => LogLevelFilter::Debug,
		_ => LogLevelFilter::Trace
	});

	match args.subcommand() {
		("checkout",  Some(sub_m)) => checkout::execute(&sub_m),
		("service",  Some(sub_m))  => service::execute(&sub_m),
		("branch",  Some(sub_m))   => branch::execute(&sub_m),
		(&_, _)                    => println!("{}", args.usage())
	}

}

fn build_args<'a, 'b>() -> App<'a, 'b> {

	return App::new(env!("CARGO_PKG_NAME"))
		.version(crate_version!())
		.author(crate_authors!())
		.about(env!("CARGO_PKG_DESCRIPTION"))

		.arg(Arg::with_name("verbose")
			.help("Sets the level of logging")
			.short("v")
			.long("verbose")
			.multiple(true))

		.subcommand(branch::build_args())
		.subcommand(checkout::build_args())
		.subcommand(service::build_args());

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

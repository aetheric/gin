
#[cfg(test)]
mod main_tests;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate clap;

use clap::App;
use ops::checkout;

fn main() {

	// Parse the console arguments and attempt to make use of them.
	let args = App::from_yaml(load_yaml!("cli.yml"))
			.name(env!("CARGO_PKG_NAME"))
			.version(crate_version!())
			.author(crate_authors!())
			.about(env!("CARGO_PKG_DESCRIPTION"))
			.get_matches();

	// Initialise logging from the logging config file.
	let logging = log4rs::init_file("log4rs.yml", Default::default()).unwrap();

	// TODO: Change logging level for console based on verbosity.
	match args.occurrences_of("v") {
		0 => trace!("Keeping log level at WARN"),
		1 => trace!("Upgrading log level to INFO"),
		2 => trace!("Upgrading log level to DEBUG"),
		3 => trace!("Upgrading log level to TRACE"),
		_ => trace!("There are no higher log levels.")
	}

	trace!("Logging initialised. Root logger level at ??");

	match matches.subcommand() {
		"checkout" => ops::checkout::checkout()
	}

}

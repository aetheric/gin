//! All the crates and features, etc are defined here. Not much else.

#![crate_name = "gin"]
#![crate_type = "bin"]

#![feature(plugin)]
#![feature(test)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use]
mod util;

#[cfg(test)]
mod main_tests;

mod gin;

#[cfg(test)]
extern crate test;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate hyper;

#[macro_use]
extern crate mime;

extern crate ease;

fn main() {

	// Parse the console arguments and attempt to make use of them.
	let args = gin::build_args()
		.get_matches();

	gin::execute(&args);

}


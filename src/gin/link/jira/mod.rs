//! `gin link jira <username> <password>`

use std::collections::HashMap;
use ease::{ Url, Request };
use clap::{ App, SubCommand, Arg, ArgMatches };
use hyper::header::{ Headers, ContentType };
use mime;

header! { (AtlassianUser, "X-AUSERNAME") => [String] }
header! { (AtlassianToken, "X-Atlassian-Token") => [String] }

pub fn build_args<'a, 'b>() -> App<'a, 'b> {

	return SubCommand::with_name("jira")
		.about("Makes gin able to communicate with jira via REST")
		.version("0.1")

		.arg(Arg::with_name("hostname")
			.help("The hostname that jira is using.")
			.takes_value(true)
			.required(true)
			.index(1))

		.arg(Arg::with_name("username")
			.help("The username you use to authenticate with jira")
			.takes_value(true)
			.required(true)
			.index(2))

		.arg(Arg::with_name("password")
			.help("The password you use to authenticate with jira")
			.takes_value(true)
			.required(true)
			.index(3));

}

pub fn execute(args: &ArgMatches) {
	let host = args.value_of("hostname").unwrap();
	let user = args.value_of("username").unwrap();
	let pass = args.value_of("password").unwrap();

	let url = Url::parse(&format!("{}/rest/api/latest/", host)).unwrap();
	let response = Request::new(url)
		.header(ContentType(mime!(Application/Json; Charset=Utf8)))
		.header(AtlassianUser(String::from("")))
		.header(AtlassianToken(String::from("")))
		.param("foo", "bar")
		.get().unwrap();
}


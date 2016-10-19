/**
 * Will expose commands to the main cli entry point.
 */

pub use self::checkout;
mod checkout;

pub use self::service;
mod service;

trait Operation {
	fn build_args() -> Subcommand;
	fn execute(args: ArgMatches);
}

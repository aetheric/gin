/***/

#[macro_export]
macro_rules! println_stderr (
	($($arg:tt)*) => { {

		#[macro_use]
		use std::io::Write;

		let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
		r.expect("failed printing to stderr");

	} }
);

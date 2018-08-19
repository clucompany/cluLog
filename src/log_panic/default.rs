
use log_panic::LogPanic;
use std::fmt::Arguments;
use std::io::Write;
use std::io;
use log_shape::LogShape;

#[derive(Debug)]
pub enum DefaultPanic { }

impl<Shape: LogShape> LogPanic<Shape> for DefaultPanic {
	#[inline(always)]
	fn panic<'a, W: Write>(_write: W, arg: Arguments<'a>) -> io::Result<()> {
		panic!("{}", arg);
	}
}


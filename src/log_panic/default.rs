
use log_core::LogPanic;
use std::fmt::Arguments;
use std::io::Write;
use std::io;

#[derive(Debug)]
pub enum DefaultPanic { }

impl LogPanic for DefaultPanic {
	#[inline(always)]
	fn panic<'a, W: Write>(_write: W, arg: Arguments<'a>) -> io::Result<()> {
		panic!("{}", arg);
	}
}


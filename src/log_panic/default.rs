
use log_panic::LogPanic;
use std::fmt::Arguments;
use std::io::Write;
use std::io;
use log_write::LogWrite;

#[derive(Debug)]
pub enum DefaultPanic { }

impl<WRITER: LogWrite> LogPanic<WRITER> for DefaultPanic {
	#[inline(always)]
	fn panic<'a, W: Write>(_write: W, arg: Arguments<'a>) -> io::Result<()> {
		panic!("{}", arg);
	}
}


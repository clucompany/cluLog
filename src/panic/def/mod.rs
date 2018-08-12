
use ::panic::LogPanic;
use std::fmt::Arguments;
use std::io::Write;
use std::io;
use ::write::LogWrite;

#[derive(Debug)]
pub enum StdPanic { }

impl LogPanic for StdPanic {
	#[inline(always)]
	fn panic<'a, WRITER: LogWrite, W: Write>(_write: W, arg: Arguments<'a>) -> io::Result<()> {
		
		/*if let Err(e) = WRITER::panic(write, arg) {
			return Err(e);
		}*/
		
		panic!("{}", arg);
	}
}


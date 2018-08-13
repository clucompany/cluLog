
pub mod def;

use std::fmt::Arguments;
use std::io::Write;
use log_write::LogWrite;
use std::io;

pub type DefTypeProgramPanic = self::def::DefaultPanic;


pub trait LogPanic {
	#[inline(always)]
	fn panic<'a, WRITER: LogWrite, W: Write>(write: W, arg: Arguments<'a>) -> io::Result<()>;
}


pub mod def;

use std::fmt::Arguments;
use std::io::Write;
use ::write::LogWrite;
use std::io;

pub type DefTypeProgramPanic = self::def::StdPanic;


pub trait LogPanic {
	fn panic<'a, WRITER: LogWrite, W: Write>(write: W, arg: Arguments<'a>) -> io::Result<()>;
}

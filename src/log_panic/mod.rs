
pub mod default;

use std::fmt::Arguments;
use std::io::Write;
use log_write::LogWrite;
use std::io;

pub type DefTypeProgramPanic = self::default::DefaultPanic;


pub trait LogPanic {
	fn panic<'a, WRITER: LogWrite, W: Write>(write: W, arg: Arguments<'a>) -> io::Result<()>;
}

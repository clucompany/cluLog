
pub mod default;

use std::fmt::Arguments;
use std::io::Write;
use log_write::LogWrite;
use std::io;
use log_write::DefLogWrite;

pub type DefTypeProgramPanic = self::default::DefaultPanic;


pub trait LogPanic<WRITER: LogWrite = DefLogWrite> {
	fn panic<'a, W: Write>(write: W, arg: Arguments<'a>) -> io::Result<()>;
}


pub mod def;

use std::fmt::Debug;
use std::fmt::Arguments;
use std::io::Write;
use ::write::LogWrite;
use std::io;

pub type DefTypeProgramPanic = self::def::StdPanic;


pub trait LogPanic: Debug + Send + Sync {
	fn panic<'a, WRITER: LogWrite>(write: &mut Write, arg: Arguments<'a>) -> io::Result<()>;
}

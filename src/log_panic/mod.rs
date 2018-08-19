
pub mod default;

use std::fmt::Debug;
use std::fmt::Arguments;
use std::io::Write;
use log_shape::LogShape;
use std::io;
use log_shape::DefLogShape;

pub type DefaultPanic = self::default::DefaultPanic;


pub trait LogPanic<Shape: LogShape = DefLogShape>: Debug {
	fn panic<'a, W: Write>(write: W, arg: Arguments<'a>) -> io::Result<()>;
}

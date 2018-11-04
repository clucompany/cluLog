
mod default;

pub use self::default::*;

use std::fmt::Debug;
use std::fmt::Arguments;
use std::io::Write;
use log_shape::LogShape;
use std::io;
use log_shape::DefLogShape;

pub type DefLogPanic = self::default::DefaultPanic;


pub trait LogPanic<Shape: LogShape = DefLogShape>: Debug {
	fn panic<'a, W: Write>(write: W, arg: Arguments<'a>) -> io::Result<()>;
}

impl<'l, A: LogPanic<L>, L: LogShape> LogPanic<L> for &'l A {
	#[inline(always)]
	fn panic<'a, W: Write>(write: W, arg: Arguments<'a>) -> io::Result<()> {
		A::panic(write, arg)
	}
}

impl<'l, A: LogPanic<L>, L: LogShape> LogPanic<L> for &'l mut A {
	#[inline(always)]
	fn panic<'a, W: Write>(write: W, arg: Arguments<'a>) -> io::Result<()> {
		A::panic(write, arg)
	}
}
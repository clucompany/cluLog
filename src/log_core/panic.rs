

use std::fmt::Debug;
use std::io::Write;
use std::fmt::Arguments;
use std::io;

pub trait LogPanic: Debug {
	fn panic<'a, W: Write>(write: W, arg: Arguments<'a>) -> io::Result<()>;
}

impl<'l, A: LogPanic> LogPanic for &'l A {
	#[inline(always)]
	fn panic<'a, W: Write>(write: W, arg: Arguments<'a>) -> io::Result<()> {
		A::panic(write, arg)
	}
}

impl<'l, A: LogPanic> LogPanic for &'l mut A {
	#[inline(always)]
	fn panic<'a, W: Write>(write: W, arg: Arguments<'a>) -> io::Result<()> {
		A::panic(write, arg)
	}
}
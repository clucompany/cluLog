


use std::fmt::Arguments;
use std::io;

///Generalization of the basic methods of information output
pub trait LogBase<'a> {
	fn warning<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn info<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn error<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn panic<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn unknown<'s>(&'a self, name: &'static str, args: Arguments<'s>) -> io::Result<()>;

	fn trace<'s>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()>;
	
	fn print<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;

	fn eprint<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
}


impl<'a, A: LogBase<'a>> LogBase<'a> for &'a A {
	#[inline(always)]
	fn warning<'s>(&self, args: Arguments<'s>) -> io::Result<()> {
		(**self).warning(args)
	}
	
	#[inline(always)]
	fn info<'s>(&self, args: Arguments<'s>) -> io::Result<()> {
		(**self).info(args)
	}
	
	#[inline(always)]
	fn error<'s>(&self, args: Arguments<'s>) -> io::Result<()> {
		(**self).error(args)
	}
	
	#[inline(always)]
	fn panic<'s>(&self, args: Arguments<'s>) -> io::Result<()> {
		(**self).panic(args)
	}
	
	#[inline(always)]
	fn unknown<'s>(&self, name: &'static str, args: Arguments<'s>) -> io::Result<()> {
		(**self).unknown(name, args)
	}

	#[inline(always)]
	fn trace<'s>(&self, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()> {
		(**self).trace(line, pos, file, args)
	}
	
	#[inline(always)]
	fn print<'s>(&self, args: Arguments<'s>) -> io::Result<()> {
		(**self).print(args)
	}

	#[inline(always)]
	fn eprint<'s>(&self, args: Arguments<'s>) -> io::Result<()> {
		(**self).eprint(args)
	}
}


impl<'a, A: LogBase<'a>> LogBase<'a> for &'a mut A {
	#[inline(always)]
	fn warning<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		(**self).warning(args)
	}
	
	#[inline(always)]
	fn info<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		(**self).info(args)
	}
	
	#[inline(always)]
	fn error<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		(**self).error(args)
	}
	
	#[inline(always)]
	fn panic<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		(**self).panic(args)
	}
	
	#[inline(always)]
	fn unknown<'s>(&'a self, name: &'static str, args: Arguments<'s>) -> io::Result<()> {
		(**self).unknown(name, args)
	}

	#[inline(always)]
	fn trace<'s>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()> {
		(**self).trace(line, pos, file, args)
	}
	
	#[inline(always)]
	fn print<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		(**self).print(args)
	}

	#[inline(always)]
	fn eprint<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		(**self).eprint(args)
	}
}
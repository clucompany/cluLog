
use std::fmt::Arguments;
use std::io::Write;
use std::fmt::Debug;
use std::io;


///Method of writing the data log
pub trait LogShape: Debug {
	fn unknown<'s, W: Write>(write: W, name: &'static str, display: Arguments<'s>) -> io::Result<()>;
	//[UNK] - unknown 
	
	fn trace<'s, W: Write>(write: W, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()>;
	//[TRACE][src/main.rs][38:29] - trace


	fn warning<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[WAR] - warning value
	
	fn info<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[INF] - info value
	
	fn error<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - err value
	
	fn panic<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[PANIC] - panic program

	fn print<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - print value
	
	fn eprint<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - print value
}

impl<'a, A: LogShape> LogShape for &'a A {
	#[inline(always)]
	fn warning<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::warning(write, display)
	}
	//[WAR] - warning value
	
	#[inline(always)]
	fn info<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::info(write, display)
	}
	//[INF] - info value
	
	
	#[inline(always)]
	fn error<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::error(write, display)
	}
	//[ERR] - err value
	
	#[inline(always)]
	fn panic<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::panic(write, display)
	}
	//[PANIC] - panic program
	
	#[inline(always)]
	fn unknown<'s, W: Write>(write: W, name: &'static str, display: Arguments<'s>) -> io::Result<()> {
		A::unknown(write, name, display)
	}
	//[UNK] - unknown 
	
	#[inline(always)]
	fn trace<'s, W: Write>(write: W, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()>{
		A::trace(write, line, pos, file, args)
	}
	//[TRACE][src/main.rs][38:29] - trace

	#[inline(always)]
	fn print<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::print(write, display)
	}
	//[ERR] - print value
	
	#[inline(always)]
	fn eprint<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::eprint(write, display)
	}
	//[ERR] - print value
}




impl<'a, A: LogShape> LogShape for &'a mut A {
	#[inline(always)]
	fn warning<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::warning(write, display)
	}
	//[WAR] - warning value
	
	#[inline(always)]
	fn info<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::info(write, display)
	}
	//[INF] - info value
	
	#[inline(always)]
	fn error<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::error(write, display)
	}
	//[ERR] - err value
	
	#[inline(always)]
	fn panic<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::panic(write, display)
	}
	//[PANIC] - panic program
	
	#[inline(always)]
	fn unknown<'s, W: Write>(write: W, name: &'static str, display: Arguments<'s>) -> io::Result<()> {
		A::unknown(write, name, display)
	}
	//[UNK] - unknown 
	
	#[inline(always)]
	fn trace<'s, W: Write>(write: W, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()>{
		A::trace(write, line, pos, file, args)
	}
	//[TRACE][src/main.rs][38:29] - trace

	#[inline(always)]
	fn print<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::print(write, display)
	}
	//[ERR] - print value
	
	#[inline(always)]
	fn eprint<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()> {
		A::eprint(write, display)
	}
	//[ERR] - print value
}

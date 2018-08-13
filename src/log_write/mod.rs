
pub mod def;
pub mod color;

use std::fmt::Arguments;
use std::io::Write;
use std::io;


pub type DefLogWrite = self::color::cluLogColorWrite;
pub type DefColorWrite = self::color::cluLogColorWrite;


///Method of writing the data log
pub trait LogWrite {
	#[inline(always)]
	fn warning<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[WAR] - warning value
	
	#[inline(always)]
	fn info<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[INF] - info value
	
	#[inline(always)]
	fn error<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - err value
	
	#[inline(always)]
	fn panic<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[PANIC] - panic program
	
	#[inline(always)]
	fn unknown<'s, W: Write>(write: W, name: &'static str, display: Arguments<'s>) -> io::Result<()>;
	//[UNK] - unknown 
	
	#[inline(always)]
	fn trace<'s, W: Write>(write: W, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()>;
	//[TRACE][src/main.rs][38:29] - trace

	#[inline(always)]
	fn print<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - print value
	
	#[inline(always)]
	fn eprint<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - print value
}






pub mod default;
pub mod default_color;

use std::fmt::Debug;
use std::fmt::Arguments;
use std::io::Write;
use std::io;

pub type DefColorShape = self::default_color::cluLogColorShape;
pub type DefNoColorShape = self::default::cluLogShape;
pub type DefLogShape = self::default_color::cluLogColorShape;


///Method of writing the data log
pub trait LogShape: Debug {
	fn warning<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[WAR] - warning value
	
	fn info<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[INF] - info value
	
	fn error<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - err value
	
	fn panic<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[PANIC] - panic program
	
	fn unknown<'s, W: Write>(write: W, name: &'static str, display: Arguments<'s>) -> io::Result<()>;
	//[UNK] - unknown 
	
	fn trace<'s, W: Write>(write: W, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()>;
	//[TRACE][src/main.rs][38:29] - trace

	fn print<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - print value
	
	fn eprint<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - print value
}





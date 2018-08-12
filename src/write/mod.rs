
pub mod def;
pub mod color;

use std::fmt::Arguments;
use std::io::Write;
use std::io;


pub type DefLogWrite = self::color::cluLogColorWrite;
pub type DefColorWrite = self::color::cluLogColorWrite;


///Method of writing the data log
pub trait LogWrite {
	fn warning<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[WAR] - warning value
	
	fn info<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[INF] - info value
	
	fn error<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - err value
	
	fn panic<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[PANIC] - panic program
	
	fn unknown<'s, W: Write>(write: W, name: &'s str, display: Arguments<'s>) -> io::Result<()>;
	//[UNK] - unknown 
	
	
	fn print<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - print value
	
	fn eprint<'s, W: Write>(write: W, display: Arguments<'s>) -> io::Result<()>;
	//[ERR] - print value
}





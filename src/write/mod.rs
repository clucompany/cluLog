
pub mod def;
pub mod color;

use std::fmt::Debug;
use std::fmt::Arguments;
//use std::fmt::Write;
use std::io::Write;
use std::io;


//pub type DefTypeLogWrite = self::def::cluLogWrite;
pub type DefTypeLogWrite = self::color::cluLogColorWrite;


pub trait LogWrite: Debug + Send + Sync {
	fn warning<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()>;
	//[WAR] - warning value
	
	fn info<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()>;
	//[INF] - info value
	
	fn error<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()>;
	//[ERR] - err value
	
	fn panic<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()>;
	//[PANIC] - panic program
	
	fn unknown<'a>(write: &mut Write, name: &'a str, display: Arguments<'a>) -> io::Result<()>;
	//[UNK] - unknown 
	
	
	fn print<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()>;
	//[ERR] - print value
	
	fn eprint<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()>;
	//[ERR] - print value
}





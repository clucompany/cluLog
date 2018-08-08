
use std::fmt::Arguments;
use std::io::Write;
use ::write::LogWrite;
use std::io;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum cluLogWrite {}

impl LogWrite for cluLogWrite {	
	
	#[inline(always)]
	fn warning<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("[WAR] {}\n", display)	)
	}
	//[WAR] - warning value
	
	#[inline(always)]
	fn info<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("[INF] {}\n", display)	)
	}
	//[INF] - info value
	
	#[inline(always)]
	fn error<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("[ERR] {}\n", display)		)
	}
	//[ERR] - err value
	
	#[inline(always)]
	fn panic<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("[PANIC] {}\n", display)	)
	}
	//[PANIC] - panic program
	
	#[inline(always)]
	fn unknown<'a>(write: &mut Write, name: &'a str, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("[{}] {}\n", name, display)	)
	}
	//[UNK] - unknown 
	
	
	
	#[inline(always)]
	fn print<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("[OUT] {}", display)		)
	}
	//[OUT] - unknown 
	
	#[inline(always)]
	fn eprint<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("[EOUT] {}", display)		)
	}
	//[EOUT] - unknown 
}

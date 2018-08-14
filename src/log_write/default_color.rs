
use std::io::Write;
use std::fmt::Arguments;
use log_write::LogWrite;
use ::std::io;

use clucolor::colors::*;
use clucolor::cluColor;

pub type PanicColor = 	BrightRed;
pub type ErrColor = 	BrightRed;


pub type WarningColor = 	BrightYellow;
pub type InfoColor = 	BrightCyan;
//pub type TraceColor = 	BrightYellow;
pub type UnkColor = 	BrightBlue;

pub type PrintColor = 	BrightWhite;
pub type EPrintColor = 	BrightWhite;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum cluLogColorWrite {}

impl LogWrite for cluLogColorWrite {	
	
	#[inline(always)]
	fn warning<'a, W: Write>(write: W, display: Arguments<'a>) -> io::Result<()> {
		writen_color!(write, WarningColor, "[WAR] {}", color_args!(bright_white, display))
	}
	//[WAR] - warning value
	
	#[inline(always)]
	fn info<'a, W: Write>(write: W, display: Arguments<'a>) -> io::Result<()> {
		writen_color!(write, InfoColor, "[INF] {}", color_args!(bright_white, display))
	}
	//[INF] - info value
	
	#[inline(always)]
	fn error<'a, W: Write>(write: W, display: Arguments<'a>) -> io::Result<()> {
		writen_color!(write, ErrColor, "[ERR] {}", color_args!(bright_white, display))
	}
	//[ERR] - err value
	
	#[inline(always)]
	fn panic<'a, W: Write>(write: W, display: Arguments<'a>) -> io::Result<()> {
		writen_color!(write, PanicColor, "[PANIC] {}", color_args!(bright_white, display))
	}
	//[PANIC] - panic program
	
	#[inline(always)]
	fn unknown<'a, W: Write>(write: W, name: &'static str, display: Arguments<'a>) -> io::Result<()> {
		writen_color!(write, UnkColor, "[{}] {}", name, color_args!(bright_white, display))
	}
	//[UNK] - unknown 
	
	#[inline(always)]
	fn trace<'s, W: Write>(mut write: W, line: u32, pos: u32, file: &'static str, display: Arguments<'s>) -> io::Result<()> {
		write.write_fmt(format_args!( 
			"{}{} {}\n", 
			
			color_args!(bright_yellow, "[TRACE]"), 
			color_args!(bright_blue, bold, format_args!("[{}][{}:{}]", file, line, pos)), 
			color_args!(bright_white, display) 

		))
		
		//writen_color!(write, TraceColor, "{}[{}][{}:{}] {}", color_args!(bright_white, "[TRACE]"), file, line, pos, display)
	}	
	
	#[inline(always)]
	fn print<'a, W: Write>(mut write: W, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(display)
	}
	//[OUT] - unknown 
	
	#[inline(always)]
	fn eprint<'a, W: Write>(mut write: W, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(display)
	}
	//[EOUT] - unknown 
}


use std::io::Write;
use std::fmt::Arguments;
use ::write::LogWrite;
use ::std::io;

use clucolor::colors::*;
use clucolor::cluColor;

pub type PanicColor = 	BrightRed;
pub type ErrColor = 	BrightRed;


pub type WarningColor = 	BrightYellow;
pub type InfoColor = 	BrightCyan;
pub type UnkColor = 	BrightBlue;

pub type PrintColor = 	BrightWhite;
pub type EPrintColor = 	BrightWhite;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum cluLogColorWrite {}

impl LogWrite for cluLogColorWrite {	
	
	#[inline(always)]
	fn warning<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		writen_color!(write, WarningColor, "[WAR] {}", color_args!(bright_white, display))
	}
	//[WAR] - warning value
	
	#[inline(always)]
	fn info<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		writen_color!(write, InfoColor, "[INF] {}", color_args!(bright_white, display))
	}
	//[INF] - info value
	
	#[inline(always)]
	fn error<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		writen_color!(write, ErrColor, "[ERR] {}", color_args!(bright_white, display))
	}
	//[ERR] - err value
	
	#[inline(always)]
	fn panic<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		writen_color!(write, PanicColor, "[PANIC] {}", color_args!(bright_white, display))
	}
	//[PANIC] - panic program
	
	#[inline(always)]
	fn unknown<'a>(write: &mut Write, name: &'a str, display: Arguments<'a>) -> io::Result<()> {
		writen_color!(write, UnkColor, "[{}] {}", name, color_args!(bright_white, display))
	}
	//[UNK] - unknown 
	
	
	
	#[inline(always)]
	fn print<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(display)
	}
	//[OUT] - unknown 
	
	#[inline(always)]
	fn eprint<'a>(write: &mut Write, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(display)
	}
	//[EOUT] - unknown 
}

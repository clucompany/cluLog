#![feature(const_fn)]

#[macro_use]
extern crate clucolor;

pub mod log;
pub mod panic;
pub mod write;
mod macros;

use log::empty::LogEmpty;
use log::default::LogStd;
use log::cluLog;
use write::LogWrite;
use panic::LogPanic;
use std::sync::{Once, ONCE_INIT};


pub type DefLogWrite = 			self::write::			DefLogWrite;
pub type DefLogPanic = 			self::panic::			DefTypeProgramPanic;
/*pub type Log = cluLog<
	OutWrite = ::std::io::StdoutLock<'static>, 
	ErrWrite = ::std::io::StderrLock<'static>
>;
*/

static mut LOGGER: &'static cluLog<'static> = &LogEmpty;
static LOGGER_INIT: Once = ONCE_INIT;

/*
pub fn set_log(log: &'static cluLog) -> bool {
	unsafe {
		if LOGGER_INIT {
			return false;
		}
		LOGGER_INIT = true;
		
		LOGGER = log;
		true
	}
}*/


#[inline]
pub fn set_logger(log: &'static cluLog<'static>) {
	LOGGER_INIT.call_once(|| {
		unsafe {
			LOGGER = log;
		}
	});
}



#[inline]
pub fn set_boxed_logger(log: Box<cluLog<'static>>) {
	set_logger( unsafe { &*Box::into_raw(log) } )
}


///Obtaining a link to active logging
#[inline(always)]
pub fn as_log<'a>() -> &'a cluLog<'static> {
	unsafe { LOGGER }
}


#[macro_export]
macro_rules! init_clulog {
	() => {
		{
			clulog::set_boxed_logger(clulog::log::default::LogStd::default_box());
		}
	};
	
	(null) => {
		{
			use clulog::log::empty::total::LogTotalEmpty;

			clulog::set_logger(&LogTotalEmpty)
		}
	};
	
	(none) => {
		{
			use clulog::log::empty::LogEmpty;
			clulog::set_logger(&LogEmpty)
			//clulog::set_boxed_logger(Box::new(LogEmpty))
		}
	};
	
	(panic, $panic:tt) => {
		{
			use clulog::DefLogWrite;
			clulog::set_logger(&clulog::init_std::<DefLogWrite, $panic>())
		}
	};
	
	(write, $write:tt) => {
		{
			use clulog::DefLogPanic;
			clulog::set_logger(&clulog::init_std::<$write, DefLogPanic>())
		}
	};
	
	($write:tt, $panic:tt) => {
		//{
			//let std = clulog::init_std::<$write, $panic>();
			//clulog::set_logger(&LogStd::<$write, $panic>::new());
			clucolor::set_boxed_logger(LogStd::default_box());
		//}
	};
}


///Obtaining a link to active logging
#[macro_export]
macro_rules! as_log {
	() => {
		clulog::as_log()
	};
}


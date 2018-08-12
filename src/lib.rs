
#[macro_use]
extern crate clucolor;

pub mod log;
pub mod log_panic;
pub mod log_write;
mod macros;

use log::empty::LogEmpty;
use log::cluLog;
use std::sync::{Once, ONCE_INIT};


pub type DefLogWrite = 			self::log_write::			DefLogWrite;
pub type DefLogPanic = 			self::log_panic::			DefTypeProgramPanic;
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
macro_rules! init_cluLog {
	() => {
		{
			cluLog::set_boxed_logger(cluLog::log::default::LogStd::default_box());
		}
	};
	(default) => {
		init_cluLog!();
	};
	
	(null) => {
		{
			use cluLog::log::empty::total::LogTotalEmpty;

			cluLog::set_logger(&LogTotalEmpty)
		}
	};
	
	(none) => {
		{
			use cluLog::log::empty::LogEmpty;
			cluLog::set_logger(&LogEmpty)
			//cluLog::set_boxed_logger(Box::new(LogEmpty))
		}
	};
	
	(panic, $panic:tt) => {
		{
			use cluLog::DefLogWrite;
			cluLog::set_logger(&cluLog::init_std::<DefLogWrite, $panic>())
		}
	};
	
	(write, $write:tt) => {
		{
			use cluLog::DefLogPanic;
			cluLog::set_logger(&cluLog::init_std::<$write, DefLogPanic>())
		}
	};
	
	($write:tt, $panic:tt) => {
		//{
			//let std = cluLog::init_std::<$write, $panic>();
			//cluLog::set_logger(&LogStd::<$write, $panic>::new());
			clucolor::set_boxed_logger(LogStd::default_box());
		//}
	};
}


///Obtaining a link to active logging
#[macro_export]
macro_rules! as_log {
	() => {
		cluLog::as_log()
	};
}


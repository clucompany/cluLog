
#[macro_use]
extern crate clucolor;

pub mod log;
pub mod log_panic;
pub mod log_write;
pub mod log_addition;
mod macros;

use log::cluLog;
use std::sync::{Once, ONCE_INIT};
use log_addition::empty::default::LogEmpty;


pub type DefLogWrite = 			self::log_write::			DefLogWrite;
pub type DefLogPanic = 			self::log_panic::			DefTypeProgramPanic;


static mut LOGGER: &'static cluLog<'static> = &LogEmpty;
static LOGGER_INIT: Once = ONCE_INIT;


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
			cluLog::set_boxed_logger(cluLog::log::default::LogStd::default_box());
		}
	};
	(default) => {
		init_cluLog!();
	};
	
	(null) => {
		{
			use cluLog::log_addition::empty::total::LogTotalEmpty;

			cluLog::set_logger(&LogTotalEmpty)
		}
	};
	
	(none) => {
		{
			use cluLog::log_addition::empty::default::LogEmpty;

			cluLog::set_logger(&LogEmpty)
		}
	};
	(union, $a:expr, $b:expr) => {
		{
			
			clucolor::set_boxed_logger($a.union($b).to_box());
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


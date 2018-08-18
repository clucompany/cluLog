
#[macro_use]
extern crate clucolor;

pub mod log;
pub mod log_panic;
pub mod log_write;
pub mod log_addition;
mod macros;

use log_addition::empty::total::LogTotalEmpty;
use log::LogStatic;
use std::sync::{Once, ONCE_INIT};


pub type DefLogWrite = 			self::log_write::			DefLogWrite;
pub type DefLogPanic = 			self::log_panic::			DefaultPanic;


static mut LOGGER: &'static LogStatic<'static> = &LogTotalEmpty;
static LOGGER_INIT: Once = ONCE_INIT;


#[inline]
pub fn set_logger(log: &'static LogStatic<'static>) {
	LOGGER_INIT.call_once(|| {
		unsafe {
			LOGGER = log;
		}
	});
}



#[inline]
pub fn set_boxed_logger(log: Box<LogStatic<'static>>) {
	set_logger( unsafe { &*Box::into_raw(log) } )
}


///Obtaining a link to active logging
#[inline(always)]
pub fn as_log<'a>() -> &'a LogStatic<'static> {
	unsafe { LOGGER }
}


#[macro_export]
macro_rules! init_clulog {
	(null) => {
		use cluLog::log_addition::empty::total::LogTotalEmpty;

		cluLog::set_logger(&LogTotalEmpty)
	};
	
	(none) => {
		use cluLog::log_addition::empty::default::LogEmpty;
		cluLog::set_boxed_logger(Box::new(LogEmpty::default()))
	};
	(total_none) => {
		use cluLog::log_addition::empty::total::LogTotalEmpty;

		cluLog::set_logger(&LogTotalEmpty);
	};

	(one) => {
		use cluLog::log::default_one::LogOneDefault;
		cluLog::set_boxed_logger(Box::new(LogOneDefault::default()));
	};
	(one, $e:expr) => {
		use cluLog::log::default_one::LogOneDefault;
		cluLog::set_boxed_logger(Box::new(LogOneDefault::new($e)));
	};

	(union, $panic:tt, $a:expr, $b:expr) => {
		cluLog::set_boxed_logger(Box::new($a.union::<$panic, _>($b)));
	};
	(union, $a:expr, $b:expr) => {
		cluLog::set_boxed_logger(Box::new($a.default_union($b)));
	};
	
	/*(panic, $panic:tt) => {
		use cluLog::DefLogWrite;
		cluLog::set_logger(&cluLog::init_std::<DefLogWrite, $panic>())
	};
	
	(write, $write:tt) => {
		use cluLog::DefLogPanic;
		cluLog::set_logger(&cluLog::init_std::<$write, DefLogPanic>())
	};
	
	($write:tt, $panic:tt) => {
		use cluLog::log::default::LogDefault;
		cluLog::set_boxed_logger(LogDefault::<$write, $panic>::default_box());
	};*/


	() => {
		use cluLog::log::default::LogDefault;
		cluLog::set_boxed_logger(Box::new(LogDefault::default()));
	};
	($e: expr) => {
		use cluLog::log::default_one::LogOneDefault;
		cluLog::set_boxed_logger(LogOneDefault::new($e));
	};
	($e: expr, $e2: expr) => {
		use cluLog::log::default::LogDefault;
		cluLog::set_boxed_logger(LogDefault::new($e, $e2));
	};
}


///Obtaining a link to active logging
#[macro_export]
macro_rules! as_log {
	() => {
		cluLog::as_log()
	};
}


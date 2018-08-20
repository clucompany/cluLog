
#[macro_use]
extern crate clucolor;

pub mod log;
pub mod log_panic;
pub mod log_shape;
pub mod log_addition;
pub mod log_lock;
pub mod log_write;
mod macros;

use log_addition::empty::total::LogTotalEmpty;
use log::LogStatic;
use std::sync::{Once, ONCE_INIT};


pub type DefLogShape = 			self::log_shape::			DefLogShape;
pub type DefLogPanic = 			self::log_panic::			DefaultPanic;


static mut LOGGER: &'static LogStatic<'static> = &LogTotalEmpty;
static LOGGER_INIT: Once = ONCE_INIT;


#[inline]
pub fn set_slice_logger(log: &'static LogStatic<'static>) {
	LOGGER_INIT.call_once(|| {
		unsafe {
			LOGGER = log;
		}
	});
}

#[inline(always)]
pub fn set_logger<S: 'static + LogStatic<'static>>(log: S) {
	set_boxed_logger(Box::new(log))
}

#[inline]
pub fn set_boxed_logger(log: Box<LogStatic<'static>>) {
	set_slice_logger( unsafe { &*Box::into_raw(log) } )
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

		cluLog::set_slice_logger(&LogTotalEmpty)
	};
	
	(none) => {
		use cluLog::log_addition::empty::default::LogEmpty;
		cluLog::set_logger(LogEmpty::default())
	};
	(total_none) => {
		use cluLog::log_addition::empty::total::LogTotalEmpty;

		cluLog::set_slice_logger(&LogTotalEmpty);
	};

	(one) => {
		use cluLog::log::default_one::LogOneDefault;
		cluLog::set_logger(LogOneDefault::default());
	};
	(one, $e:expr) => {
		use cluLog::log::default_one::LogOneDefault;
		cluLog::set_logger(LogOneDefault::new($e));
	};

	(union, $panic:tt, $a:expr, $b:expr) => {
		cluLog::set_logger($a.union::<$panic, _>($b));
	};
	(union, $a:expr, $b:expr) => {
		cluLog::set_logger($a.default_union($b));
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
		cluLog::set_logger(LogDefault::default());
	};
	($e: expr) => {
		use cluLog::log::default_one::LogOneDefault;
		cluLog::set_logger(LogOneDefault::new($e));
	};
	($e: expr, $e2: expr) => {
		use cluLog::log::default::LogDefault;
		cluLog::set_logger(LogDefault::new($e, $e2));
	};
}


///Obtaining a link to active logging
#[macro_export]
macro_rules! as_log {
	() => {
		cluLog::as_log()
	};
}


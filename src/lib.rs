
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
	set_boxed_logger(Box::new( log ))
}

#[inline]
pub fn set_boxed_logger(log: Box<LogStatic<'static>>) {
	LOGGER_INIT.call_once(|| {
		unsafe {
			LOGGER = &*Box::into_raw(log);
		}
	});
}


///Obtaining a link to active logging
#[inline(always)]
pub fn as_log() -> &'static LogStatic<'static> {
	unsafe { LOGGER }
}


#[macro_export]
macro_rules! init_clulog {
	(null) => {
		use $crate::log_addition::empty::total::LogTotalEmpty;

		$crate::set_slice_logger(&LogTotalEmpty)
	};
	
	(none) => {
		use $crate::log_addition::empty::default::LogEmpty;
		$crate::set_logger(LogEmpty::default())
	};
	(total_none) => {
		use $crate::log_addition::empty::total::LogTotalEmpty;

		$crate::set_slice_logger(&LogTotalEmpty);
	};

	(one) => {
		use $crate::log::default_one::LogOneDefault;
		$crate::set_logger(LogOneDefault::default());
	};
	(one, $e:expr) => {
		use $crate::log::default_one::LogOneDefault;
		$crate::set_logger(LogOneDefault::new($e));
	};

	(union, $panic:tt, $a:expr, $b:expr) => {
		$crate::set_logger($a.union::<$panic, _>($b));
	};
	(union, $a:expr, $b:expr) => {
		$crate::set_logger($a.default_union($b));
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
		use $crate::log::default::LogDefault;
		$crate::set_logger(LogDefault::default());
	};
	($e: expr) => {
		use $crate::log::default_one::LogOneDefault;
		$crate::set_logger(LogOneDefault::new($e));
	};
	($e: expr, $e2: expr) => {
		use $crate::log::default::LogDefault;
		$crate::set_logger(LogDefault::new($e, $e2));
	};
}


///Obtaining a link to active logging
#[macro_export]
macro_rules! as_log {
	() => {
		$crate::as_log()
	};
}


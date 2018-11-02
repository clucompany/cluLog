

#[macro_use]
extern crate clucolor;

mod log;
pub mod log_panic;
pub mod log_shape;
pub mod log_addition;
pub mod log_lock;
pub mod log_write;
pub mod log_core;

mod macros;

pub use self::log::*;
use log_core::LogStatic;
use log_addition::empty::total::LogTotalEmpty;
use std::sync::{Once, ONCE_INIT};


pub type DefLogShape = 			self::log_shape::			DefLogShape;
pub type DefLogPanic = 			self::log_panic::			DefaultPanic;


static mut LOGGER: &'static LogStatic<'static> = &LogTotalEmpty;
static LOGGER_INIT: Once = ONCE_INIT;


#[inline]
pub fn set_slice_logger(log: &'static LogStatic<'static>) -> bool {
	let mut is_set = false;
	LOGGER_INIT.call_once(|| {
		unsafe {
			LOGGER = log;
		}
		is_set = true;
	});
	is_set
}

#[inline(always)]
pub fn set_logger<S: 'static + LogStatic<'static>>(log: S) -> bool {
	let mut is_set = false;
	LOGGER_INIT.call_once(move || {
		unsafe {
			let log = Box::new(log);
			LOGGER = &*Box::into_raw(log);
		}
		is_set = true;
	});
	is_set
}

#[inline]
pub fn set_boxed_logger(log: Box<LogStatic<'static>>) -> bool {
	let mut is_set = false;
	LOGGER_INIT.call_once(|| {
		unsafe {
			LOGGER = &*Box::into_raw(log);
		}
		is_set = true;
	});
	is_set
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
		use $crate::LogOneDefault;
		$crate::set_logger(LogOneDefault::default());
	};
	(one, $e:expr) => {
		use $crate::LogOneDefault;
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
		use $crate::LogDefault;
		$crate::set_logger(LogDefault::default());
	};
	($e: expr) => {
		use $crate::LogOneDefault;
		$crate::set_logger(LogOneDefault::new($e));
	};
	($e: expr, $e2: expr) => {
		use $crate::LogDefault;
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


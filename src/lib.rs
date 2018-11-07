

#[macro_use]
extern crate clucolor;

pub mod log_shape;
pub mod log_addition;
pub mod log_write;
pub mod log_core;

mod macros;
mod macros_out;

pub use self::log_addition::LogDefault;
pub use self::log_addition::LogOneDefault;
pub use self::log_addition::LogEmpty;
pub use self::log_addition::LogTotalEmpty;
pub use self::log_addition::LogUnion;
pub use self::log_addition::LogFile;

use log_core::LogStatic;
use std::sync::{Once, ONCE_INIT};

pub type DefLogColorShape = self::log_shape::DefLogColorShape;
pub type DefLogShape = self::log_shape::DefLogShape;
pub type DefLog<'a> = self::log_addition::DefLog<'a>;


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
pub fn set_logger<S: LogStatic<'static> + 'static>(log: S) -> bool {
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


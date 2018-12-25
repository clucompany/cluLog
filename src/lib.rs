
#[macro_use]
extern crate clucolor;

mod macros;
mod macros_init;

mod def_static_log;

pub mod log_core;
pub mod log_union;
pub mod log_shape;

mod log_write;


mod log_file;
pub use self::log_file::*;



use crate::log_core::LogStatic;
use std::sync::Once;
use std::sync::ONCE_INIT;
pub use self::log_write::*;



static mut LOGGER: &'static LogStatic<'static> = &def_static_log::LogDefStaticLog::new();
static LOGGER_INIT: Once = ONCE_INIT;


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

///Obtaining a link to active logging
#[macro_export]
macro_rules! as_log {
	() => {
		$crate::as_log()
	};
}



#[macro_use]
mod core;
#[macro_use]
pub mod core_union;

mod def_static_log;
use self::def_static_log::LogDefStaticLog;

pub mod core_block;

#[macro_use]
mod macros;
#[allow(unused_imports)]
use self::macros::*;

use std::sync::ONCE_INIT;
use std::sync::Once;
pub use self::core::*;




static mut LOGGER: &'static LogStatic<'static> = &LogDefStaticLog::new();
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

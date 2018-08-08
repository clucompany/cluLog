
#[macro_use]
extern crate clucolor;

pub mod log;
pub mod panic;
pub mod write;
pub mod macros;

use log::empty::LogEmpty;
use log::std::LogStd;
use log::cluLog;
use write::LogWrite;
use panic::LogPanic;
use std::sync::{Once, ONCE_INIT};


pub type DefLogWrite = 			self::write::			DefTypeLogWrite;
pub type DefLogPanic = 			self::panic::			DefTypeProgramPanic;
/*pub type Log = cluLog<
	OutWrite = ::std::io::StdoutLock<'static>, 
	ErrWrite = ::std::io::StderrLock<'static>
>;
*/
pub type Log = cluLog;


static mut LOGGER: &'static Log = &LogEmpty;
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
pub fn set_logger(log: &'static Log) {
	LOGGER_INIT.call_once(|| {
		unsafe {
			LOGGER = log;
		}
	});
}



#[inline]
pub fn set_boxed_logger(log: Box<Log>) {
	set_logger( unsafe { &*Box::into_raw(log) } )
}


#[inline]
pub fn init_std<WRITER: LogWrite + 'static, PANIC: LogPanic + 'static>() {	
	set_boxed_logger( Box::new(LogStd::<WRITER, PANIC>::new()) )
}


///Obtaining a link to active logging
#[inline(always)]
pub fn as_log<'a>() -> &'a Log {
	unsafe { LOGGER }
}


#[macro_export]
macro_rules! init_clulog {
	() => {
		{
			use clulog::DefLogPanic;
			use clulog::DefLogWrite;
			init_clulog!(DefLogWrite, DefLogPanic);
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
			clulog::init_std::<DefLogWrite, $panic>()
		}
	};
	
	(write, $write:tt) => {
		{
			use clulog::DefLogPanic;
			clulog::init_std::<$write, DefLogPanic>()
		}
	};
	
	($write:tt, $panic:tt) => {
		{
			clulog::init_std::<$write, $panic>()
		}
	};
}


///Obtaining a link to active logging
#[macro_export]
macro_rules! as_log {
	() => {
		clulog::as_log()
	};
}




use std::io::Write;
use log_write::FlushWrite;

///Secure outflow blocking
#[allow(non_camel_case_types)]
pub trait LogLockIO<'a> {
	///Blocking threads with automatic cleaning
	fn lock_out(&'a self) -> FlushWrite<Box<Write + 'a>> {
		FlushWrite::new(self.raw_lock_out())
	}

	///Blocking threads with automatic cleaning
	fn lock_err(&'a self) -> FlushWrite<Box<Write + 'a>> {
		FlushWrite::new(self.raw_lock_err())	
	}

	///Flow blocking without self-cleaning
	fn raw_lock_out(&'a self) -> Box<Write + 'a>;

	///Flow blocking without self-cleaning
	fn raw_lock_err(&'a self) -> Box<Write + 'a>;
}


impl<'a, A: LogLockIO<'a>> LogLockIO<'a> for &'a A {
	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_out(&'a self) -> FlushWrite<Box<Write + 'a>> {
		(**self).lock_out()
	}

	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_err(&'a self) -> FlushWrite<Box<Write + 'a>> {
		(**self).lock_err()
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		(**self).raw_lock_out()
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		(**self).raw_lock_err()
	}
}


impl<'a, A: LogLockIO<'a>> LogLockIO<'a> for &'a mut A {
	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_out(&'a self) -> FlushWrite<Box<Write + 'a>> {
		(**self).lock_out()
	}

	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_err(&'a self) -> FlushWrite<Box<Write + 'a>> {
		(**self).lock_err()
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		(**self).raw_lock_out()
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		(**self).raw_lock_err()
	}
}



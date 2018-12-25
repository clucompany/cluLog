

use cluExtIO::FlushDropWrite;
use std::io::Write;

///Secure outflow blocking
pub trait LogLockIO<'a> {
	///Blocking threads with automatic cleaning
	fn lock_out(&'a self) -> FlushDropWrite<Box<Write + 'a>> {
		self.raw_lock_out().into()
	}

	///Blocking threads with automatic cleaning
	fn lock_err(&'a self) -> FlushDropWrite<Box<Write + 'a>> {
		self.raw_lock_err().into()
	}

	///Flow blocking without self-cleaning
	fn raw_lock_out(&'a self) -> Box<Write + 'a>;

	///Flow blocking without self-cleaning
	fn raw_lock_err(&'a self) -> Box<Write + 'a>;
}


impl<'a, 'l, A: LogLockIO<'a>> LogLockIO<'a> for &'l A {
	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_out(&'a self) -> FlushDropWrite<Box<Write + 'a>> {
		A::lock_out(self)
	}

	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_err(&'a self) -> FlushDropWrite<Box<Write + 'a>> {
		A::lock_err(self)
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		A::raw_lock_out(self)
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		A::raw_lock_err(self)
	}
}


impl<'a, 'l, A: LogLockIO<'a>> LogLockIO<'a> for &'l mut A {
	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_out(&'a self) -> FlushDropWrite<Box<Write + 'a>> {
		A::lock_out(self)
	}

	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_err(&'a self) -> FlushDropWrite<Box<Write + 'a>> {
		A::lock_err(self)
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		A::raw_lock_out(self)
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		A::raw_lock_err(self)
	}
}



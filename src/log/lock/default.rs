

use log_addition::empty::LogEmptyConst;
use std::ops::Deref;
use std::fmt::Debug;
use log_addition::empty::empty_write::EmptyWrite;
use std::ops::DerefMut;
use std::marker::PhantomData;
use std::io::Write;

///Blocking threads with automatic cleaning
#[allow(non_camel_case_types)]
pub struct cluLogLock<'a, W: Write + 'a>(W, PhantomData<&'a ()>);

impl<'a, W: Write + 'a> cluLogLock<'a, W> {
	#[inline]
	pub fn new(out: W) -> Self {
		cluLogLock(out, PhantomData)
	}

	pub fn boxed(out: W) -> Box<'a + DerefMut<Target = Write + 'a>> {
		Box::new(Self::new(out))
	}
}

impl<'a> LogEmptyConst for cluLogLock<'a, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite)
	}
}



impl<'a, W: Write + 'a> Debug for cluLogLock<'a, W> {
	#[inline]
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("cluLogLock { .. }")
	}
}

impl<'a, W: Write + 'a> Deref for cluLogLock<'a, W> {
	type Target = Write + 'a ;

	#[inline(always)]
	fn deref<'l>(&'l self) -> &'l Self::Target {
		&self.0
	}
}

impl<'a, W: Write + 'a> DerefMut for cluLogLock<'a, W> {
	#[inline(always)]
	fn deref_mut<'l>(&'l mut self) -> &'l mut Self::Target {
		&mut self.0
	}
}

impl<'a, W: Write + 'a> Drop for cluLogLock<'a, W> {
	#[inline]
	fn drop(&mut self) {
		let _e = self.0.flush();
	}
}

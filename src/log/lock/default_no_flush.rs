
use log_addition::empty::LogEmptyConst;
use std::ops::Deref;
use log_addition::empty::empty_write::EmptyWrite;
use std::ops::DerefMut;
use std::marker::PhantomData;
use std::io::Write;
use std::fmt::Debug;

///Flow blocking without self-cleaning
#[allow(non_camel_case_types)]
pub struct cluLogLockNoFlush<'a, W: Write + 'a>(W, PhantomData<&'a ()>);

impl<'a, W: Write + 'a> cluLogLockNoFlush<'a, W> {
	#[inline]
	pub fn new(out: W) -> Self {
		cluLogLockNoFlush(out, PhantomData)
	}

	pub fn boxed(out: W) -> Box<'a + DerefMut<Target = Write + 'a>> {
		Box::new(cluLogLockNoFlush(out, PhantomData))
	}

	#[inline]
	pub fn impled(out: W) -> impl DerefMut<Target = Write + 'a> + 'a {
		cluLogLockNoFlush(out, PhantomData)
	}
}


impl<'a> LogEmptyConst for cluLogLockNoFlush<'a, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		cluLogLockNoFlush::new(EmptyWrite)
	}
}


impl<'a, W: Write + 'a> Debug for cluLogLockNoFlush<'a, W> {
	#[inline]
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("cluLogLockNoFlush { .. }")
	}
}

impl<'a, W: Write + 'a> Deref for cluLogLockNoFlush<'a, W> {
	type Target = Write + 'a ;

	#[inline(always)]
	fn deref<'l>(&'l self) -> &'l Self::Target {
		&self.0
	}
}

impl<'a, W: Write + 'a> DerefMut for cluLogLockNoFlush<'a, W> {
	#[inline(always)]
	fn deref_mut<'l>(&'l mut self) -> &'l mut Self::Target {
		&mut self.0
	}
}



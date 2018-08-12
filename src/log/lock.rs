

use std::marker::PhantomData;
use std::fmt::Debug;
use log::empty::write::EmptyWrite;
use std::ops::DerefMut;
use std::ops::Deref;
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
impl<'a> cluLogLock<'a, EmptyWrite> {
	///Use an empty lock
	#[inline(always)]
	pub fn empty() -> Self {
		Self::new(EmptyWrite)
	}

	pub fn empty_boxed() -> Box<'a + DerefMut<Target = Write + 'a>> {
		Box::new(Self::empty())
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
impl<'a> cluLogLockNoFlush<'a, EmptyWrite> {
	///Use an empty lock
	#[inline(always)]
	pub fn empty() -> Self {
		Self::new(EmptyWrite)
	}

	///Use an empty lock, in the heap
	pub fn empty_boxed() -> Box<'a + DerefMut<Target = Write + 'a>> {
		Box::new(Self::empty())
	}

	#[inline(always)]
	pub fn impled_boxed() -> impl DerefMut<Target = Write + 'a> + 'a {
		Self::empty()
	}
}

impl<'a, W: Write + 'a> Debug for cluLogLockNoFlush<'a, W> {
	#[inline]
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("cluLogLock { .. }")
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



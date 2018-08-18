
use std::ops::DerefMut;
use std::ops::Deref;
use std::sync::Mutex;
use std::io::Write;
use std::io;

#[derive(Debug, Clone)]
pub struct EmptyWrite;

impl EmptyWrite {
	#[inline]
	pub fn new() -> Self {
		EmptyWrite
	}

	#[inline]
	pub fn impled() -> impl Write {
		Self::new()
	}

	#[inline]
	pub fn boxed() -> Box<Write> {
		Box::new(Self::new())
	}
}

impl Write for EmptyWrite {
	#[inline(always)]
	fn write<'a>(&mut self, _buf: &'a [u8]) -> io::Result<usize> {
		Ok( 0 )
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()> {
		Ok( () )
	}
}

/*
#[derive(Debug)]
pub struct EmptyMutexWrite(Mutex<EmptyWrite>);

impl EmptyMutexWrite {
	#[inline]
	pub fn new() -> Self {
		EmptyMutexWrite(Mutex::new(EmptyWrite))
	}

	#[inline]
	pub fn impled() -> impl Write {
		Self::new()
	}

	#[inline]
	pub fn boxed() -> Box<Write> {
		Box::new(Self::new())
	}
}

impl Write for EmptyMutexWrite {
	#[inline(always)]
	fn write<'a>(&mut self, _buf: &'a [u8]) -> io::Result<usize> {
		Ok( 0 )
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()> {
		Ok( () )
	}
}


impl Clone for EmptyMutexWrite {
	#[inline]
	fn clone(&self) -> Self {
		Self::new()
	}
}

impl Deref for EmptyMutexWrite {
	type Target = Mutex<Write>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for EmptyMutexWrite {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}*/
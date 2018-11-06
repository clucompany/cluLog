
/*
use log_write::LogWrite;
use std::marker::PhantomData;
use std::io::Write;
use std::io;

pub struct AutoCloseWrite<'a, L: LogWrite<'a, LOCK> + 'a, LOCK: Write + 'a>(L, PhantomData<LOCK>, PhantomData<&'a L>);

impl<'a, L: LogWrite<'a, LOCK> + 'a, LOCK: Write + 'a> AutoCloseWrite<'a, L, LOCK> {
     #[inline]
     pub fn new(a: L) -> Self {
          AutoCloseWrite(a, PhantomData, PhantomData)
     }
}

impl<'a, L: LogWrite<'a, LOCK> + 'a, LOCK: Write + 'a> LogWrite<'a, LOCK> for AutoCloseWrite<'a, L, LOCK> {
     #[inline]
     fn lock(&'a self) -> LOCK {
          self.0.lock()
     }
}


impl<'a, L: LogWrite<'a, LOCK> + 'a, LOCK: Write + 'a> WriteWithMut<'a> for AutoCloseWrite<'a, L, LOCK> {
     #[inline(always)]
     fn write(&'a self, buf: &[u8]) -> io::Result<usize> {
          self.0.lock().write(buf)
     }

     #[inline(always)]
     fn flush(&'a self) -> io::Result<()> {
          self.0.lock().flush()
     }

	#[inline(always)]
	fn write_all(&'a self, buf: &[u8]) -> ::std::io::Result<()> {
		self.0.lock().write_all(buf)
	}

	#[inline(always)]
	fn write_fmt(&'a self, fmt: ::std::fmt::Arguments) -> ::std::io::Result<()> {
          self.0.lock().write_fmt(fmt)
	}
}

pub trait WriteWithMut<'a> {
     #[inline(always)]
     fn write(&'a self, buf: &[u8]) -> io::Result<usize>;

     #[inline(always)]
     fn flush(&'a self) -> io::Result<()>;

	#[inline(always)]
	fn write_all(&'a self, buf: &[u8]) -> ::std::io::Result<()>;

	#[inline(always)]
	fn write_fmt(&'a self, fmt: ::std::fmt::Arguments) -> ::std::io::Result<()>;
}
*/
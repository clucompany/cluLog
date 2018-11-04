

use std::sync::MutexGuard;
use std::marker::PhantomData;
use std::io::Write;
use std::sync::Mutex;
use log_write::LogWrite;
use std::io;
use std::fmt;

#[derive(Debug)]
pub struct MutexWrite<'a, T: 'a + Write>(Mutex<T>, PhantomData<&'a ()>);

impl<'a, T: 'a + Write> MutexWrite<'a, T> {
     #[inline]
     pub fn new(t: T) -> Self {
          Self::mutex(Mutex::new(t))
     }
     #[inline]
     pub fn mutex(m: Mutex<T>) -> Self {
          MutexWrite(m, PhantomData)
     }

     #[inline(always)]
     fn _lock(&'a self) -> MutexGuard<'a, T> {
          match self.0.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          }
     }
}

impl<'a, T: 'a + Write> Write for MutexWrite<'a, T> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          self._lock().write(buf)
     }

     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          self._lock().flush()
     }

     #[inline(always)]
     fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
          self._lock().write_all(buf)
     }

     #[inline(always)]
     fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> { 
          self._lock().write_fmt(fmt)
     }
}

impl<'a, T: 'a + Write> LogWrite<'a, WriteGuard<'a, T>> for MutexWrite<'a, T> {
     #[inline]
     fn lock(&'a self) -> WriteGuard<'a, T> {
          WriteGuard::guard(self._lock())
     }
     #[inline(always)]
     fn un_flush(&self) -> io::Result<()> {
          self._lock().flush()
     }
}


impl<'a, T: 'a + Write + Clone> Clone for MutexWrite<'a, T> {
     #[inline]
     fn clone(&self) -> Self {
          MutexWrite::new(
               self._lock().clone()
          )
     }
}



#[derive(Debug)]
pub struct WriteGuard<'a, T: Write + 'a>(MutexGuard<'a, T>, PhantomData<&'a ()>);

impl<'a, T: Write + 'a> WriteGuard<'a, T> {
     #[inline]
     pub fn guard(t: MutexGuard<'a, T>) -> Self {
          WriteGuard(t, PhantomData)
     }

     #[inline]
     pub fn boxed(t: MutexGuard<'a, T>) -> Box<Self> {
          Box::new(Self::guard(t))
     }
}

impl<'a, T: Write + 'a> Write for WriteGuard<'a, T> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          self.0.write(buf)
     }

     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          self.0.flush()
     }

     #[inline(always)]
     fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
          self.0.write_all(buf)
     }

     #[inline(always)]
     fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> { 
          self.0.write_fmt(fmt)
     }
}




use std::sync::MutexGuard;
use std::io::Write;
use std::sync::Mutex;
use log_write::LogWrite;
use std::io;
use std::fmt;

#[derive(Debug)]
pub struct MutexWrite<T: Write>(Mutex<T>);

impl<T: Write> MutexWrite<T> {
     #[inline]
     pub fn new(t: T) -> Self {
          Self::mutex(Mutex::new(t))
     }
     #[inline]
     pub fn mutex(m: Mutex<T>) -> Self {
          MutexWrite(m)
     }

     #[inline(always)]
     fn _lock<'a>(&'a self) -> MutexGuard<'a, T> {
          match self.0.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          }
     }
}

impl<T: Write> Write for MutexWrite<T> {
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

impl<'a, T: 'a +  Write> LogWrite<'a> for MutexWrite<T> {
     type Lock = GuardWrite<'a, T>;
     #[inline]
     fn lock(&'a self) -> Self::Lock {
          GuardWrite::guard(self._lock())
     }
}


#[derive(Debug)]
pub struct GuardWrite<'a, T: 'a +  Write>(MutexGuard<'a, T>);

impl<'a, T: Write> GuardWrite<'a, T> {
     #[inline]
     pub fn guard(t: MutexGuard<'a, T>) -> Self {
          GuardWrite(t)
     }

     #[inline]
     pub fn boxed(t: MutexGuard<'a, T>) -> Box<Self> {
          Box::new(Self::guard(t))
     }
}

impl<'a, T: 'a + Write> Write for GuardWrite<'a, T> {
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


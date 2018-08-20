

use std::sync::MutexGuard;
use std::marker::PhantomData;
use std::io::Write;
use log_lock::mutex_nf::LogSafeMutexLockNF;
use std::sync::Mutex;
use log_lock::mutex::LogSafeMutexLock;
use log_write::LogWrite;
use std::io;

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
     #[inline]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          let mut lock = self._lock();
          lock.write(buf)
     }
     #[inline]
     fn flush(&mut self) -> io::Result<()> {
          let mut lock = self._lock();
          lock.flush()
     }
}

impl<'a, T: 'a + Write> LogWrite<'a, LogSafeMutexLock<'a, T>> for MutexWrite<'a, T> {
     #[inline]
     fn lock(&'a self) -> LogSafeMutexLock<'a, T> {
          LogSafeMutexLock::new(self._lock())
     }
     #[inline(always)]
     fn un_flush(&self) -> io::Result<()> {
          self._lock().flush()
     }
}

impl<'a, T: 'a + Write> LogWrite<'a, LogSafeMutexLockNF<'a, T>> for MutexWrite<'a, T> {
     #[inline]
     fn lock(&'a self) -> LogSafeMutexLockNF<'a, T> {
          LogSafeMutexLockNF::new(self._lock())
     }
     #[inline(always)]
     fn un_flush(&self) -> io::Result<()> {
          self._lock().flush()
     }
}
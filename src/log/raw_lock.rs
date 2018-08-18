

use log_lock::default_nf::LogSafeLockNF;
use log_lock::default::LogSafeLock;
use std::sync::MutexGuard;
use std::sync::Mutex;
use std::fs::File;
use log_addition::empty::empty_write::EmptyWrite;
use std::io::StderrLock;
use std::io::Stderr;
use std::io::StdoutLock;
use std::io::Write;
use std::io::Stdout;

//Raw internal locking method
#[allow(non_camel_case_types)]
pub trait LogLockRawIO<'a, W: Write + 'a = Self> {
     ///Internal method
	fn lock(&'a self) -> W;
     
     ///Alternate startup name
     #[inline(always)]
     fn _lock(&'a self) -> W {
          self.lock()
     }
}

impl<'a> LogLockRawIO<'a, StdoutLock<'a>> for Stdout {
     #[inline(always)]
     fn lock(&'a self) -> StdoutLock<'a> {
          self.lock()
     }
}

impl<'a> LogLockRawIO<'a, StderrLock<'a>> for Stderr {
     #[inline(always)]
     fn lock(&'a self) -> StderrLock<'a> {
          self.lock()
     }
}

impl<'a> LogLockRawIO<'a> for EmptyWrite {
     #[inline(always)]
     fn lock(&'a self) -> EmptyWrite {
          self.clone()
     }
}


impl<'a> LogLockRawIO<'a, LogSafeLock<'a, Self>> for &'a File {
     #[inline(always)]
     fn lock(&'a self) -> LogSafeLock<'a, Self> {
          LogSafeLock::new(self)
     }
}

impl<'a> LogLockRawIO<'a, LogSafeLockNF<'a, Self>> for &'a File {
     #[inline(always)]
     fn lock(&'a self) -> LogSafeLockNF<'a, Self> {
          LogSafeLockNF::new(self)
     }
}



pub struct MutexWriter<'a, T: 'a + Write>(MutexGuard<'a, T>);
impl<'a, T: 'a + Write> MutexWriter<'a, T> {
     #[inline]
     pub fn new(guard: MutexGuard<'a, T>) -> Self {
          MutexWriter(guard)
     }
     pub fn lock(m: &'a Mutex<T>) -> Self {
          let lock = match m.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          };
          Self::new(lock)
     }
}

impl<'a, T: 'a + Write> Write for MutexWriter<'a, T> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
          self.0.write(buf)
     }

     #[inline(always)]
     fn flush(&mut self) -> ::std::io::Result<()> {
          self.0.flush()
     }
}


impl<'a> LogLockRawIO<'a, LogSafeLock<'a, MutexWriter<'a, File>>> for Mutex<File> {
     #[inline]
     fn lock(&'a self) -> LogSafeLock<'a, MutexWriter<'a, File>> {
          LogSafeLock::new(MutexWriter::lock(self))
     }
}

impl<'a> LogLockRawIO<'a, LogSafeLockNF<'a, MutexWriter<'a, File>>> for Mutex<File> {
     #[inline]
     fn lock(&'a self) -> LogSafeLockNF<'a, MutexWriter<'a, File>> {
          LogSafeLockNF::new(MutexWriter::lock(self))
     }
}
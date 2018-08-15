

use log_addition::empty::empty_write::EmptyWrite;
use std::io::StderrLock;
use std::io::Stderr;
use std::io::StdoutLock;
use std::io::Write;
use std::io::Stdout;

//Raw internal locking method
#[allow(non_camel_case_types)]
pub trait LogLockRawIO<'a, W: Write + 'a = Self>: Write + Send + Sync {
     ///Internal method
	fn lock(&'a self) -> W;
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



use std::io::StderrLock;
use std::io::Stderr;
use std::io::StdoutLock;
use std::io::Write;
use std::io::Stdout;

//Raw internal locking method
#[allow(non_camel_case_types)]
pub trait cluLogRawIOLock<'a, W: Write + 'a>: Write + Send + Sync {
	fn lock(&'a self) -> W;
}

impl<'a> cluLogRawIOLock<'a, StdoutLock<'a>> for Stdout {

     ///Internal method
     #[inline(always)]
     fn lock(&'a self) -> StdoutLock<'a> {
          self.lock()
     }
}

impl<'a> cluLogRawIOLock<'a, StderrLock<'a>> for Stderr {

     ///Internal method
     #[inline(always)]
     fn lock(&'a self) -> StderrLock<'a> {
          self.lock()
     }
}





use std::io::Stderr;
use std::io::StderrLock;
use log_write::LogWrite;
use std::io::StdoutLock;
use std::io::Stdout;

impl<'a> LogWrite<'a> for Stdout {
     type Lock = StdoutLock<'a>;
     #[inline(always)]
     fn lock(&'a self) -> Self::Lock {
          self.lock()
     }
}

impl<'a> LogWrite<'a> for Stderr {
     type Lock = StderrLock<'a>;
     #[inline(always)]
     fn lock(&'a self) -> Self::Lock {
          self.lock()
     }
}

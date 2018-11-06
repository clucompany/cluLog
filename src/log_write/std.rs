

use std::io::Stderr;
use std::io::StderrLock;
use log_write::LogWrite;
use std::io::StdoutLock;
use std::io::Stdout;

impl<'a> LogWrite<'a, StdoutLock<'a>> for Stdout {
     #[inline(always)]
     fn lock(&'a self) -> StdoutLock<'a> {
          self.lock()
     }
}

impl<'a> LogWrite<'a, StderrLock<'a>> for Stderr {
     #[inline(always)]
     fn lock(&'a self) -> StderrLock<'a> {
          self.lock()
     }
}



use std::io::Stderr;
use std::io::StderrLock;
use log_write::LogWrite;
use std::io::StdoutLock;
use std::io::Stdout;
use std::io;
use std::io::Write;

impl<'a> LogWrite<'a, StdoutLock<'a>> for Stdout {
     #[inline(always)]
     fn lock(&'a self) -> StdoutLock<'a> {
          self.lock()
     }
     #[inline(always)]
     fn un_flush(&self) -> io::Result<()> {
          self.lock().flush()
     }
}

impl<'a> LogWrite<'a, StderrLock<'a>> for Stderr {
     #[inline(always)]
     fn lock(&'a self) -> StderrLock<'a> {
          self.lock()
     }
     #[inline(always)]
     fn un_flush(&self) -> io::Result<()> {
          self.lock().flush()
     }
}

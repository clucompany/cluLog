
use log_write::LogWrite;
use log_addition::empty::empty_write::EmptyWrite;
use std::io;

impl<'a> LogWrite<'a, EmptyWrite> for EmptyWrite {
     #[inline(always)]
     fn lock(&'a self) -> EmptyWrite {
          self.clone()
     }

     #[inline(always)]
     fn un_flush(&self) -> io::Result<()> {
          Ok( () )
     }
}
/*
impl<'a> LogWrite<'a, LogSafeWriteLock<'a, EmptyWrite>> for EmptyWrite {
     #[inline(always)]
     fn lock(&'a self) -> LogSafeWriteLock<'a, EmptyWrite> {
          LogSafeWriteLock::new(self.clone())
     }
}

impl<'a> LogWrite<'a, LogSafeWriteNFLock<'a, EmptyWrite>> for EmptyWrite {
     #[inline(always)]
     fn lock(&'a self) -> LogSafeWriteNFLock<'a, EmptyWrite> {
          LogSafeWriteNFLock::new(self.clone())
     }
}
*/
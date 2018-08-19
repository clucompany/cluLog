
use log_write::LogWrite;
use log_addition::empty::empty_write::EmptyWrite;

impl<'a> LogWrite<'a, EmptyWrite> for EmptyWrite {
     #[inline(always)]
     fn lock(&'a self) -> EmptyWrite {
          self.clone()
     }
}

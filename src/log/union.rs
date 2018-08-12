
use std::ops::Add;
use log::cluLogFlushIO;
use log::empty::total::LogTotalEmpty;
use log::empty::LogEmpty;
use std::marker::PhantomData;
use log::cluLog;
use std::io;

pub struct LogUnion<'a, A: 'a + cluLog<'a>, B: 'a + cluLog<'a>>(A, B, PhantomData<&'a ()>);

impl<'a, A: cluLog<'a>, B: cluLog<'a>> LogUnion<'a, A, B> {

     pub fn new(a: A, b: B) -> Self {
          LogUnion(
               a, b, PhantomData,
          )
     }

     pub fn boxed(a: A, b: B) -> Box<Self> {
          Box::new(Self::new(a, b))
     }
}

impl<'a> LogUnion<'a, LogEmpty, LogEmpty> {
     #[inline]
     pub fn empty() -> Self {
          Self::new(LogEmpty, LogEmpty)
     }
}

impl<'a> LogUnion<'a, LogTotalEmpty, LogTotalEmpty> {
     #[inline]
     pub fn total_empty() -> Self {
          Self::new(LogTotalEmpty, LogTotalEmpty)
     }
}


impl<'a, A: cluLog<'a>, B: cluLog<'a>> cluLogFlushIO for LogUnion<'a, A, B> {
     #[inline(always)]
     fn flush_out(&mut self) -> io::Result<()> {
          if let Err(e) = self.0.flush_out() {
               return Err(e);
          }
          if let Err(e) = self.1.flush_out() {
               return Err(e);
          }

          Ok( () )
     }

     #[inline(always)]
	fn flush_err(&mut self) -> io::Result<()> {
          if let Err(e) = self.0.flush_err() {
               return Err(e);
          }
          if let Err(e) = self.1.flush_err() {
               return Err(e);
          }

          Ok( () )
     }
}






impl<'a, A: cluLog<'a>, B: cluLog<'a>> From< (A, B) > for LogUnion<'a, A, B> {
     #[inline]
     fn from((a, b): (A, B)) -> Self {
          Self::new(a, b)
     }
}
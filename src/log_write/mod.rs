

mod std;
mod empty;
mod mutex;


pub use self::std::*;
pub use self::empty::*;
pub use self::mutex::*;

use std::io::Write;
use std::io;

pub trait LogWrite<'a, W: Write + 'a>: Write + 'a {
     ///Blocking the output stream
	fn lock(&'a self) -> W;

     ///The method of purification without the use of mutating
     fn un_flush(&self) -> io::Result<()>;
}


impl<'a, A: LogWrite<'a, W> + 'a, W: Write + 'a> LogWrite<'a, W> for &'a A where Self: Write {
     ///Blocking the output stream
     #[inline(always)]
	fn lock(&'a self) -> W {
          (**self).lock()
     }

     ///The method of purification without the use of mutating
     fn un_flush(&self) -> io::Result<()> {
          (**self).un_flush()
     }
}

impl<'a, A: LogWrite<'a, W> + 'a, W: Write + 'a> LogWrite<'a, W> for &'a mut A where Self: Write {
     ///Blocking the output stream
     #[inline(always)]
	fn lock(&'a self) -> W {
          (**self).lock()
     }

     ///The method of purification without the use of mutating
     fn un_flush(&self) -> io::Result<()> {
          (**self).un_flush()
     }
}
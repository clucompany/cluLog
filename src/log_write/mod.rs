

mod std;
mod empty;
mod mutex;
mod union;
mod flush;


pub use self::std::*;
pub use self::empty::*;
pub use self::mutex::*;
pub use self::union::*;
pub use self::flush::*;

use std::io::Write;

///The trait extends Write and allows to use in systems of logging.
pub trait LogWrite<'a, W: Write + 'a>: Write {
     ///Blocking the output stream
	fn lock(&'a self) -> W;
}

impl<'a, L: LogWrite<'a, W>, W: Write + 'a> LogWrite<'a, W> for &'a L where Self: Write + 'a {
     ///Blocking the output stream
	fn lock(&'a self) -> W {
          (**self).lock()
     }
}

impl<'a, L: LogWrite<'a, W>, W: Write + 'a> LogWrite<'a, W> for &'a mut L where Self: Write + 'a {
     ///Blocking the output stream
	fn lock(&'a self) -> W {
          (**self).lock()
     }
}


mod std;
mod empty;
mod mutex;
mod union;
mod flush;
mod autoclose;


pub use self::std::*;
pub use self::empty::*;
pub use self::mutex::*;
pub use self::union::*;
pub use self::flush::*;
pub use self::autoclose::*;

use std::io::Write;

///The trait extends Write and allows to use in systems of logging.
pub trait LogWrite<'a>: Write {
     type Lock: Write + 'a;

     ///Blocking the output stream
	fn lock(&'a self) -> Self::Lock;
}

impl<'a, 'l, L: LogWrite<'a, Lock = W>, W: 'a +  Write> LogWrite<'a> for &'l L where Self: Write + 'a {
     type Lock = W;
     ///Blocking the output stream
	fn lock(&'a self) -> Self::Lock {
          (**self).lock()
     }
}

impl<'a, 'l, L: LogWrite<'a, Lock = W>, W: 'a +  Write> LogWrite<'a> for &'l mut L where Self: Write + 'a {
     type Lock = W;
     ///Blocking the output stream
	fn lock(&'a self) -> Self::Lock {
          (**self).lock()
     }
}
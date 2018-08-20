

pub mod std;
pub mod empty;
pub mod mutex;

use std::io::Write;
use std::io;

pub trait LogWrite<'a, W: Write + 'a>: Write + 'a {
     ///Blocking the output stream
	fn lock(&'a self) -> W;

     ///The method of purification without the use of mutating
     fn un_flush(&self) -> io::Result<()>;
}


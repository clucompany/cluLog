

pub mod std;
pub mod empty;
pub mod file;

use std::io::Write;

pub trait LogWrite<'a, W: Write + 'a>: Write + 'a {
     ///Internal method
	fn lock(&'a self) -> W;
}


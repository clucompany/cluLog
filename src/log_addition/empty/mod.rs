
use std::io::Write;

pub mod empty_write;
pub mod total;
pub mod default;


///The constructor of empty structures
pub trait LogEmptyConst {
     ///Use an empty lock
     fn empty() -> Self;

     #[inline]
     fn empty_boxed<'a>() -> Box<Write + 'a> where Self: Sized + 'a + Write {
		Box::new(Self::empty())
	}
}



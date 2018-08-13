
use std::io::Write;
use std::ops::DerefMut;

pub mod empty_write;
pub mod total;
pub mod default;


///The constructor of empty structures
pub trait LogEmptyConst {
     ///Use an empty lock
     fn empty() -> Self;

     #[inline]
     fn empty_boxed<'a>() -> Box<'a + DerefMut<Target = Write + 'a>> where Self: 'a + Sized + DerefMut<Target = Write + 'a> {
		Box::new(Self::empty())
	}
}



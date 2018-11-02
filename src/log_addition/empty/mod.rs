

use log_lock::LogSafeLock;

pub mod empty_write;
pub mod total;
pub mod default;


///The constructor of empty structures
pub trait LogEmptyConst {
     ///Use an empty lock
     fn empty() -> Self;

     #[inline]
     fn empty_boxed<'a>() -> Box<Self> where Self: Sized + 'a + LogSafeLock<'a> {
		Box::new(Self::empty())
	}
}



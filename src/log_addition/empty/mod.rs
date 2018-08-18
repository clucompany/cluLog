

use log_lock::LogLock;

pub mod empty_write;
pub mod total;
pub mod default;


///The constructor of empty structures
pub trait LogEmptyConst {
     ///Use an empty lock
     fn empty() -> Self;

     #[inline]
     fn empty_boxed<'a>() -> Box<LogLock<'a> + 'a> where Self: Sized + 'a + LogLock<'a> {
		Box::new(Self::empty())
	}
}



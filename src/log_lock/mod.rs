

use std::io::Write;
use log_lock::union_nf::UnionNFLock;

pub mod default;
pub mod default_nf;
pub mod union;
pub mod union_nf;
pub mod mutex;
pub mod mutex_nf;


///The constructor of empty structures
pub trait LogSafeLock<'a>: Write + 'a {
     #[inline]
     fn union<B: LogSafeLock<'a> + Sized + 'a>(self, b: B) -> UnionNFLock<'a, Self, B> where Self: Sized { 
          UnionNFLock::new(self, b)
     }

	/*#[inline]
	fn union_nf<B: LogLockUnionConst<'a> + Sized + 'a>(self, b: B) -> UnionNFLock<'a, Self, B> where Self: Sized { 
          UnionNFLock::new(self, b)
     }*/
}

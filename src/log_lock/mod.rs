

use std::io::Write;

mod default;
mod default_nf;
mod union;
mod union_nf;
mod mutex;
mod mutex_nf;


pub use self::default::*;
pub use self::default_nf::*;
pub use self::union::*;
pub use self::union_nf::*;
pub use self::mutex::*;
pub use self::mutex_nf::*;



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


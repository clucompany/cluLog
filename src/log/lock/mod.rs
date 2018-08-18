
use log::lock::union_nf::UnionNFLock;
use log::lock::union::UnionLock;
use std::io::Write;

pub mod default;
pub mod default_nf;
pub mod union;
pub mod union_nf;

pub trait LogLock<'a>: LogLockUnionConst<'a> + Write + 'a {

}


///The constructor of empty structures
pub trait LogLockUnionConst<'a>: Write {
     #[inline]
     fn union<B: LogLockUnionConst<'a> + Sized + 'a>(self, b: B) -> UnionLock<'a, Self, B> where Self: Sized { 
          UnionLock::new(self, b)
     }

	#[inline]
	fn union_nf<B: LogLockUnionConst<'a> + Sized + 'a>(self, b: B) -> UnionNFLock<'a, Self, B> where Self: Sized { 
          UnionNFLock::new(self, b)
     }
}

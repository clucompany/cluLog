


mod total;
mod default;

pub use self::default::*;
pub use self::total::*;

///The constructor of empty structures
pub trait LogEmptyConst {
     ///Use an empty lock
     fn empty() -> Self;

     #[inline]
     fn empty_boxed<'a>() -> Box<Self> where Self: Sized {
		Box::new(Self::empty())
	}
}



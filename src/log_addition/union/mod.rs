
//!Combining several log systems into one.

use log::LogExtend;
use log_panic::LogPanic;
use log_addition::union::default::LogUnion;

pub mod default;
pub mod lock;

///The constructor of empty structures
pub trait LogUnionConst<'a>  {
     #[inline]
     fn union<P: LogPanic, B: Sized + LogExtend<'a>>(self, b: B) -> LogUnion<'a, Self, B, P> where Self: Sized + LogExtend<'a> {
          LogUnion::new(self, b)
     }
}

/*
impl<'a, C: 'a + cluLog<'a>, T: 'a + cluLog<'a>> Add for T {
     type Output = LogUnion<'a, Self, C>;

     fn add(self, other: C) -> Self::Output {
          LogUnion::new(self, other)
     }
}
*/
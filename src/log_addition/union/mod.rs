
//!Combining several log systems into one.

use log_core::LogExtend;
use DefLogPanic;
use log_panic::LogPanic;
use log_addition::union::default::LogUnion;

pub mod default;
//pub mod lock;

///The constructor of empty structures
pub trait LogUnionConst<'a> {
     #[inline]
     fn union<P: LogPanic, B: Sized + LogExtend<'a>>(self, b: B) -> LogUnion<'a, Self, B, P> where Self: Sized + LogExtend<'a> {
          LogUnion::new(self, b)
     }
     #[inline]
     fn default_union<B: Sized + LogExtend<'a>>(self, b: B) -> LogUnion<'a, Self, B, DefLogPanic> where Self: Sized + LogExtend<'a> {
          self.union(b)
     }
}

/*
impl<'a, A: LogUnionConst<'a>> LogUnionConst<'a> for &'a A {
     #[inline(always)]
     fn union<P: LogPanic, B: Sized + LogExtend<'a>>(self, b: B) -> LogUnion<'a, Self, B, P> where Self: Sized + LogExtend<'a> {
          (**self).union(b)
     }
     #[inline(always)]
     fn default_union<B: Sized + LogExtend<'a>>(self, b: B) -> LogUnion<'a, Self, B, DefLogPanic> where Self: Sized + LogExtend<'a> {
          (**self).default_union(b)
     }
}
*/



//!Combining several log systems into one.

use log_core::LogExtend;
use DefLogPanic;
use log_core::LogPanic;

mod default;
pub use self::default::*;
//pub mod lock;

///The constructor of empty structures
pub trait LogUnionConst {
     #[inline]
     fn union<'a, P: LogPanic, B: LogExtend<'a>>(self, b: B) -> LogUnion<'a, Self, B, P> where Self: Sized + LogExtend<'a> {
          LogUnion::new(self, b)
     }
     #[inline]
     fn default_union<'a, B: LogExtend<'a>>(self, b: B) -> LogUnion<'a, Self, B, DefLogPanic> where Self: Sized + LogExtend<'a> {
          self.union(b)
     }
}


impl<'a, T: LogExtend<'a>> LogUnionConst for T { }



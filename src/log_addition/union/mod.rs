

mod default;
pub use self::default::*;

use crate::log_core::LogExtend;



///The constructor of empty structures
pub trait LogUnionConst {
     #[inline]
     fn union<'a, B: LogExtend<'a>>(self, b: B) -> LogUnion<'a, Self, B> where Self: Sized + LogExtend<'a> {
          LogUnion::new(self, b)
     }
}


impl<'a, T: LogExtend<'a> + Sized> LogUnionConst for T { }


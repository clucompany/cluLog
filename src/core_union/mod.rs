
#[macro_use]
mod log;
pub use self::log::*;

use crate::core::LogExtend;



///The constructor of empty structures
pub trait LogUnionConst {
     #[inline]
     fn union<'a, B: LogExtend<'a>>(self, b: B) -> LogUnion<'a, Self, B> where Self: Sized + LogExtend<'a> {
          LogUnion::new(self, b)
     }
}

impl<'a, T: LogExtend<'a> + Sized> LogUnionConst for T { }


impl<'a, A: LogExtend<'a>, B: LogExtend<'a>> From<(A, B)> for LogUnion<'a, A, B> {
     #[inline(always)]
     fn from((a, b): (A, B)) -> Self {
          a.union(b)
     }
}



use log_addition::union::default::LogUnion;
use log::cluLog;

pub mod default;

///The constructor of empty structures
pub trait LogUnionConst<'a>: where Self: Sized + cluLog<'a> {
     #[inline]
     fn union<B: cluLog<'a>>(self, b: B) -> LogUnion<'a, Self, B> {
          LogUnion::new(self, b)
     }
}

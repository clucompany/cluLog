
//use std::ops::Add;
use log_addition::union::default::LogUnion;
use log::cluLog;

pub mod default;
pub mod lock;

///The constructor of empty structures
pub trait LogUnionConst<'a>: where Self: Sized + cluLog<'a> {
     #[inline]
     fn union<B: cluLog<'a>>(self, b: B) -> LogUnion<'a, Self, B> {
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
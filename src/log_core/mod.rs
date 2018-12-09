

mod flush;
mod out;
mod lock;
mod shape;
//mod panic;

use crate::log_addition::LogUnionConst;
pub use self::flush::*;
pub use self::out::*;
pub use self::lock::*;
pub use self::shape::*;

///An empty implementation allows you to use the current log system as the main
pub trait LogStatic<'a>: LogBase<'a> + LogLockIO<'a> + LogFlush<'a> {

}

impl<'a, A: LogStatic<'a>> LogStatic<'a> for &'a A {}
impl<'a, A: LogStatic<'a>> LogStatic<'a> for &'a mut A {}




///Empty implementation allows you to fully manipulate the current system of journals
pub trait LogExtend<'a>: LogBase<'a> + LogLockIO<'a> + LogFlush<'a> + LogUnionConst {

}



impl<'a, A: LogExtend<'a>> LogExtend<'a> for &'a A {}
impl<'a, A: LogExtend<'a>> LogExtend<'a> for &'a mut A {}



pub trait Log<'a>: LogStatic<'a> + LogExtend<'a> {}

impl<'a, A: LogStatic<'a> + LogExtend<'a>> Log<'a> for A { }


/*
impl<'a, A: Log<'a>> Log<'a> for &'a A {}
impl<'a, A: Log<'a>> Log<'a> for &'a mut A {}
*/


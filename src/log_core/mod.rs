

mod flush;
mod base_out;
mod lock;

use log_addition::union::LogUnionConst;
pub use self::flush::*;
pub use self::base_out::*;
pub use self::lock::*;



///An empty implementation allows you to use the current log system as the main
pub trait LogStatic<'a>: LogBase<'a> + LogLockIO<'a> + LogFlush {

}

impl<'a, A: LogStatic<'a>> LogStatic<'a> for &'a A {}
impl<'a, A: LogStatic<'a>> LogStatic<'a> for &'a mut A {}




///Empty implementation allows you to fully manipulate the current system of journals
pub trait LogExtend<'a>: LogBase<'a> + LogLockIO<'a> + LogFlush + LogUnionConst<'a> /*+ LogEmptyConst*/ {

}



//impl<'a, A: LogExtend<'a>> LogExtend<'a> for &'a A {}
//impl<'a, A: LogExtend<'a>> LogExtend<'a> for &'a mut A {}





#[macro_use]
extern crate cluLog;
use cluLog::log_addition::union::LogUnionConst;
use cluLog::log_addition::empty::LogEmptyConst;
use cluLog::log::default::LogDefault;
use cluLog::DefLogPanic;


fn main() {
     init_clulog!(union, DefLogPanic, LogDefault::default(), LogDefault::empty());

     trace!("1");
     trace!("2");
}
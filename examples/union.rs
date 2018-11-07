
#[macro_use]
extern crate cluLog;


use cluLog::log_addition::LogEmptyConst;
use cluLog::log_addition::LogUnionConst;
use cluLog::LogDefault;


fn main() {
     init_clulog!(union, LogDefault::default(), LogDefault::empty());

     trace!("1");
     trace!("2");
}
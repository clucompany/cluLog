
#[macro_use]
extern crate cluLog;


use cluLog::LogDefault;


fn main() {
     init_clulog!(union, LogDefault::default(), LogDefault::default());

     trace!("1");
     trace!("2");
}
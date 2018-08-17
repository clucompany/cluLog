
#[macro_use]
extern crate cluLog;
use cluLog::log_addition::union::LogUnionConst;


fn main() {
     {
          let union = cluLog::log::default::LogDefault::default(
               
          ).default_union(
               cluLog::log::default::LogDefault::default()

          ).default_union(
               cluLog::log::default::LogDefault::default()
               
          ).to_box();

          cluLog::set_boxed_logger(union);
     }
     

     trace!("This record is output in several logs of systems. 1");
     trace!("This record is output in several logs of systems. 2");
}
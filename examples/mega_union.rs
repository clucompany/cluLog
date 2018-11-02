
#[macro_use]
extern crate cluLog;
use std::io::Write;
use cluLog::log_addition::union::LogUnionConst;


fn main() {
     {
          let union = cluLog::LogDefault::default(
               
          ).default_union(
               cluLog::LogDefault::default()
          );

          cluLog::set_boxed_logger({
               match cluLog::log_addition::file::default_create_file("/tmp/clulog.out") {
                    Ok(file) => {
                         union.default_union(file).to_box()
                    },
                    _ => {
                         union.to_box()
                    },
               }
          });
     }
     

     trace!("This record is output in several logs of systems. 1");
     trace!("This record is output in several logs of systems. 2");

     let mut lock = lock_out!();
     write!(lock, "OK\n");
}
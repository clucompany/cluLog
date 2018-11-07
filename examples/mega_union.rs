
#[macro_use]
extern crate cluLog;
use cluLog::log_addition::LogUnionConst;
use std::io::Write;
use cluLog::LogFile;


fn main() {
     {
          let log_system = cluLog::LogDefault::default(
               
          ).union(
               cluLog::LogDefault::default()
          );

          cluLog::set_boxed_logger({
               match LogFile::default_create_path("/tmp/clulog.out") {
                    Ok(file) => {
                         log_system.union(file).to_box()
                    },
                    _ => {
                         log_system.to_box()
                    },
               }
          });
     }
     

     trace!("This record is output in several logs of systems. 1");
     trace!("This record is output in several logs of systems. 2");

     let mut lock = lock_out!();
     let _e = write!(lock, "OK\n");
}
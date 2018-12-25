
#[macro_use]
extern crate cluLog;

use cluLog::log_shape::DefLogShape;
use cluLog::log_union::LogUnionConst;
use std::io::Write;
use std::io;
use cluLog::LogFile;
use cluLog::LogDefault;


fn main() {
     {
          let log_system = LogDefault::default(
               
          ).union(
               cluLog::LogDefault::default()
          ).union(
               cluLog::LogDefaultExp::<DefLogShape, _, _, _, _>::new(io::stdout(), io::stderr())
          );

          cluLog::set_boxed_logger({
               match LogFile::default_create_path("/tmp/clulog.out") {
                    Ok(file0) => {
                         match LogFile::default_create_path("/tmp/clulog2.out") {
                              Ok(file2) => {
                                   log_system.union(file0).union(file2).to_box()
                                   //динамический размер
                                   //dynamic size
                              },
                              _ => {
                                   log_system.union(file0).to_box()
                                   //динамический размер
                                   //dynamic size
                              },
                         }
                    },
                    _ => {
                         log_system.to_box()
                         //динамический размер
                         //dynamic size
                    },
               }
          });
     }
     

     trace!("This record is output in several logs of systems. 1");
     trace!("This record is output in several logs of systems. 2");

     let mut lock = lock_out!();
     let _e = write!(lock, "OK\n");
}
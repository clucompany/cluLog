#[macro_use]
extern crate cluLog;
use cluLog::log_shape::DefNoColorShape;
use cluLog::DefLogPanic;
use cluLog::log_addition::union::LogUnionConst;
use std::io;

fn main() {
     {
          let union = cluLog::log::default::LogDefault::default(
               
          ).default_union(
               cluLog::log::default::LogDefault::default()
          ).default_union(
               cluLog::log::default::LogDefault::<DefNoColorShape, DefLogPanic, _, _, _, _>::new(io::stdout(), io::stderr())
          );

          cluLog::set_boxed_logger({
               match cluLog::log_addition::file::default_create_file("/tmp/clulog.out") {
                    Ok(file0) => {
                         match cluLog::log_addition::file::default_create_file("/tmp/clulog2.out") {
                              Ok(file2) => {
                                   union.default_union(file0).default_union(file2).to_box()
                              },
                              _ => {
                                   union.default_union(file0).to_box()
                              },
                         }
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
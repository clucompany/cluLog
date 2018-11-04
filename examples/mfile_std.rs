#[macro_use]
extern crate cluLog;
use std::io::Write;
use cluLog::DefLogPanic;
use cluLog::log_addition::union::LogUnionConst;
use std::io;
use cluLog::DefLogShape;

fn main() {
     {
          let union = cluLog::LogDefault::default(
               
          ).default_union(
               cluLog::LogDefault::default()
          ).default_union(
               cluLog::LogDefault::<DefLogShape, DefLogPanic, _, _, _, _>::new(io::stdout(), io::stderr())
          );

          cluLog::set_boxed_logger({
               match cluLog::log_addition::file::default_create_path("/tmp/clulog.out") {
                    Ok(file0) => {
                         match cluLog::log_addition::file::default_create_path("/tmp/clulog2.out") {
                              Ok(file2) => {
                                   union.default_union(file0).default_union(file2).to_box()
                                   //динамический размер
                                   //dynamic size
                              },
                              _ => {
                                   union.default_union(file0).to_box()
                                   //динамический размер
                                   //dynamic size
                              },
                         }
                    },
                    _ => {
                         union.to_box()
                         //динамический размер
                         //dynamic size
                    },
               }
          });
     }
     

     trace!("This record is output in several logs of systems. 1");
     trace!("This record is output in several logs of systems. 2");

     let mut lock = lock_out!();
     write!(lock, "OK\n");
}
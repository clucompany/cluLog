
#[macro_use]
extern crate cluLog;
use cluLog::DefColorShape;
use cluLog::DefLogPanic;

fn main() {
     {
          //Colored mode!
          match cluLog::log_addition::file::open_path::<DefColorShape, DefLogPanic, _>("/tmp/test") {
               Ok(file_log) => {cluLog::set_logger(file_log);} ,
               Err(e) => {
                    panic!("Err open file output, {:?}", e);
                    return;
               },
          }
     }
     
     trace!("Test 1");
     println!("Test 2");
     inf!("Test 3");
     err!("Test 4");

}


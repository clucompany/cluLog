
#[macro_use]
extern crate cluLog;

use cluLog::LogFile;

fn main() {
     match LogFile::default_create_path("/tmp/test") {
          Ok(file_log) => cluLog::set_logger(file_log),
          Err(e) => panic!("Err open file output, {:?}", e),
     };
     
     trace!("Test 1");
     println!("Test 2");
     inf!("Test 3");
     err!("Test 4");
}


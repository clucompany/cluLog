
#[macro_use]
extern crate cluLog;


fn main() {
	//init_clulog!();	
	
     {
          //Colored mode!
          match cluLog::log_addition::file::default_create_file("/tmp/test") {
               Ok(file_log) => cluLog::set_boxed_logger(Box::new(file_log)),
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

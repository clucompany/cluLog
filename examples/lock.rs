
#[macro_use]
extern crate cluLog;


use std::io::Write;

fn main() {	
     inf!("Test");
	
     {//Lock out thread
          let mut lock = lock_out!();

          for _a in 0..10 {
               let _e = lock.write(b"Test");
          }
          let _e = lock.write(b"\n");
     }
     let mut lock = lock_out!();
     let _e = lock.write(b"S\n");
}


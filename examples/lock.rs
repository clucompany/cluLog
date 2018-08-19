
#[macro_use]
extern crate cluLog;


fn main() {
	init_clulog!(none);
	
     inf!("Test");
	
     {//Lock out thread
          let mut lock = lock_out!();

          for _a in 0..10 {
               let _e = lock.write(b"Test");
          }
          let _e = lock.write(b"\n");
     }
}


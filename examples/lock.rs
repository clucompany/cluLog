
#[macro_use]
extern crate clulog;

fn main() {
	init_clulog!();
	
     inf!("Test");
	
     {
          let mut lock = lock_out!();

          for _a in 0..10 {
               let _e = lock.write(b"Test");
          }
          let _e = lock.write(b"\n");
     }
}

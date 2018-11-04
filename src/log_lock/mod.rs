

use log_write::UnionWrite;
use std::marker::PhantomData;
use std::io::Write;

//mod default;
mod mutex;

pub use self::default::*;
pub use self::mutex::*;





mod default;
mod default_one;

use crate::DefLogShape;


use std::io::StderrLock;
use std::io::StdoutLock;
use std::io::Stderr;
use std::io::Stdout;
pub use self::default::*;
pub use self::default_one::*;



pub type DefLog<'a> = LogDefault<'a, DefLogShape, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>>;
pub type DefOneLog<'a> = LogOneDefault<'a, DefLogShape, Stdout, StdoutLock<'a>>;



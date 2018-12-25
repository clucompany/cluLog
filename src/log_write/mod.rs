
mod union_write;
mod write;

use crate::log_shape::DefLogNoColorShape;
use crate::log_shape::DefLogShape;
use std::io::StderrLock;
use std::io::Stderr;
use std::io::StdoutLock;
use std::io::Stdout;
pub use self::union_write::*;
pub use self::write::*;



pub type LogDefault<'a> = LogTwoWrite<'a, DefLogShape, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>>;
pub type LogDefaultNoColor<'a> = LogTwoWrite<'a, DefLogNoColorShape, Stdout, Stderr, StdoutLock<'a>, StderrLock<'a>>;

pub type LogDefaultExp<'a, W, O, E, OL, EL> = LogTwoWrite<'a, W, O, E, OL, EL>;
pub type LogDefaultNoColorExp<'a, O, E, OL, EL> = LogTwoWrite<'a, DefLogNoColorShape, O, E, OL, EL>;



pub type LogDefaultOne<'a> = LogWrite<'a, DefLogShape, Stdout, StdoutLock<'a>>;
pub type LogDefaultOneNoColor<'a> = LogWrite<'a, DefLogNoColorShape, Stdout, StdoutLock<'a>>;

pub type LogDefaultOneExp<'a, W, O, OL> = LogWrite<'a, W, O, OL>;
pub type LogDefaultOneNoColorExp<'a, O, OL> = LogWrite<'a, DefLogNoColorShape, O, OL>;


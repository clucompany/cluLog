

use log_write::GuardWrite;
use log_shape::DefLogShape;
use log_panic::DefLogPanic;
use log_core::LogShape;
use std::path::Path;
use std::fs::File;
use log::LogOneDefault;
use log_core::LogPanic;
use log_write::MutexWrite;
use std::io;
use std::io::BufWriter;


pub fn file<'a, W: LogShape, P: LogPanic>(f: File) -> 
LogOneDefault<'a, W, P, MutexWrite<BufWriter<File>>, GuardWrite<'a, BufWriter<File>>> {
     
     LogOneDefault::new(
          MutexWrite::new(BufWriter::new(f))
     )
}


#[inline]
pub fn default_file<'a, PA: AsRef<Path>>(f: File) -> 
LogOneDefault<'a, DefLogShape, DefLogPanic, MutexWrite<BufWriter<File>>, GuardWrite<'a, BufWriter<File>>> {

     file(f)
}


#[inline]
pub fn open_path<'a, W: LogShape, P: LogPanic, PA: AsRef<Path>>(path: PA) -> io::Result< LogOneDefault<'a, W, P, MutexWrite<BufWriter<File>>, GuardWrite<'a, BufWriter<File>>> > {
     match File::open(path) {
          Ok(a) => Ok( file(a) ),
          Err(e) => Err(e),
     }
}

#[inline]
pub fn default_open_path<'a, P: AsRef<Path>>(path: P) -> io::Result< LogOneDefault<'a, DefLogShape, DefLogPanic, MutexWrite<BufWriter<File>>, GuardWrite<'a, BufWriter<File>>> > {
     open_path(path)
}


#[inline]
pub fn create_path<'a, W: LogShape, P: LogPanic, PA: AsRef<Path>>(path: PA) -> io::Result< LogOneDefault<'a, W, P, MutexWrite<BufWriter<File>>, GuardWrite<'a, BufWriter<File>>> > {
     match File::open(path) {
          Ok(a) => Ok( file(a) ),
          Err(e) => Err(e),
     }
}

#[inline]
pub fn default_create_path<'a, P: AsRef<Path>>(path: P) -> io::Result< LogOneDefault<'a, DefLogShape, DefLogPanic, MutexWrite<BufWriter<File>>, GuardWrite<'a, BufWriter<File>>> > {
     create_path(path)
}
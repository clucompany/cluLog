

use log_write::WriteGuard;
use log_shape::DefLogShape;
use log_panic::DefLogPanic;
use log_shape::LogShape;
use std::path::Path;
use std::fs::File;
use log::LogOneDefault;
use log_panic::LogPanic;
use log_write::MutexWrite;
use std::io;
use std::io::BufWriter;


pub fn file<'a, W: LogShape, P: LogPanic<W>>(f: File) -> 
LogOneDefault<'a, W, P, MutexWrite<'a, BufWriter<File>>, WriteGuard<'a, BufWriter<File>>> {
     
     LogOneDefault::new(
          MutexWrite::new(BufWriter::new(f))
     )
}


#[inline]
pub fn default_file<'a, PA: AsRef<Path>>(f: File) -> 
LogOneDefault<'a, DefLogShape, DefLogPanic, MutexWrite<'a, BufWriter<File>>, WriteGuard<'a, BufWriter<File>>> {

     file(f)
}


#[inline]
pub fn open_path<'a, W: LogShape, P: LogPanic<W>, PA: AsRef<Path>>(path: PA) -> io::Result< LogOneDefault<'a, W, P, MutexWrite<'a, BufWriter<File>>, WriteGuard<'a, BufWriter<File>>> > {
     match File::open(path) {
          Ok(a) => Ok( file(a) ),
          Err(e) => Err(e),
     }
}

#[inline]
pub fn default_open_path<'a, P: AsRef<Path>>(path: P) -> io::Result< LogOneDefault<'a, DefLogShape, DefLogPanic, MutexWrite<'a, BufWriter<File>>, WriteGuard<'a, BufWriter<File>>> > {
     open_path(path)
}


#[inline]
pub fn create_path<'a, W: LogShape, P: LogPanic<W>, PA: AsRef<Path>>(path: PA) -> io::Result< LogOneDefault<'a, W, P, MutexWrite<'a, BufWriter<File>>, WriteGuard<'a, BufWriter<File>>> > {
     match File::open(path) {
          Ok(a) => Ok( file(a) ),
          Err(e) => Err(e),
     }
}

#[inline]
pub fn default_create_path<'a, P: AsRef<Path>>(path: P) -> io::Result< LogOneDefault<'a, DefLogShape, DefLogPanic, MutexWrite<'a, BufWriter<File>>, WriteGuard<'a, BufWriter<File>>> > {
     create_path(path)
}
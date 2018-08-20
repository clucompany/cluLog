

use log_shape::LogShape;
use DefLogPanic;
use log_lock::mutex::LogSafeMutexLock;
use std::path::Path;
use std::fs::File;
use log::default_one::LogOneDefault;
use log_panic::LogPanic;
use log_write::mutex::MutexWrite;
use std::io;
use log_shape::DefNoColorShape;

pub mod buf;


pub fn new_file<'a, W: LogShape, P: LogPanic<W>>(f: File) -> LogOneDefault<'a, W, P, MutexWrite<'a, File>, LogSafeMutexLock<'a, File>> {
     LogOneDefault::new(MutexWrite::new(f))
}

#[inline]
pub fn default_new_file<'a, PA: AsRef<Path>>(f: File) -> LogOneDefault<'a, DefNoColorShape, DefLogPanic, MutexWrite<'a, File>, LogSafeMutexLock<'a, File>> {
     new_file(f)
}

#[inline]
pub fn open_file<'a, W: LogShape, P: LogPanic<W>, PA: AsRef<Path>>(path: PA) -> io::Result< LogOneDefault<'a, W, P, MutexWrite<'a, File>, LogSafeMutexLock<'a, File>> > {
     let f = match File::open(path) {
          Ok(a) => a,
          Err(e) => return Err(e),
     };

     Ok( new_file(f) )
}

#[inline]
pub fn default_open_file<'a, PA: AsRef<Path>>(path: PA) -> io::Result< LogOneDefault<'a, DefNoColorShape, DefLogPanic, MutexWrite<'a, File>, LogSafeMutexLock<'a, File>> > {
     open_file(path)
}



#[inline]
pub fn create_file<'a, W: LogShape, P: LogPanic<W>, PA: AsRef<Path>>(path: PA) -> io::Result< LogOneDefault<'a, W, P, MutexWrite<'a, File>, LogSafeMutexLock<'a, File>> > {
     let f = match File::create(path) {
          Ok(a) => a,
          Err(e) => return Err(e),
     };

     Ok( new_file(f) )
}

#[inline]
pub fn default_create_file<'a, PA: AsRef<Path>>(path: PA) -> io::Result< LogOneDefault<'a, DefNoColorShape, DefLogPanic, MutexWrite<'a, File>, LogSafeMutexLock<'a, File>> > {
     create_file(path)
}
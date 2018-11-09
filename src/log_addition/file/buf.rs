

use cluExtIO::MutexWrite;
use cluExtIO::GuardWrite;
use log_addition::LogOneDefault;
use log_shape::DefLogShape;
use log_core::LogShape;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::BufWriter;


pub type LogFile<'a, W> = LogOneDefault<'a, W, MutexWrite<BufWriter<File>>, GuardWrite<'a, BufWriter<File>>>;


impl<'a> LogFile<'a, DefLogShape> {
     #[inline]
     pub fn default_file(f: File) -> Self {
          Self::file(f)
     }
     
     #[inline]
     pub fn default_open_path<PA: AsRef<Path>>(path: PA) -> io::Result< Self > {
          Self::open_path(path)
     }

     #[inline]
     pub fn default_create_path<PA: AsRef<Path>>(path: PA) -> io::Result< Self > {
          Self::create_path(path)
     }
}


impl<'a, W: LogShape> LogFile<'a, W> {
     pub fn file(f: File) -> Self {
          LogOneDefault::new(
               MutexWrite::new(BufWriter::new(f))
          )
     }


     #[inline]
     pub fn open_path<PA: AsRef<Path>>(path: PA) -> io::Result< Self > {
          match File::open(path) {
               Ok(a) => Ok( Self::file(a) ),
               Err(e) => Err(e),
          }
     }


     #[inline]
     pub fn create_path<PA: AsRef<Path>>(path: PA) -> io::Result< Self > {
          match File::open(path) {
               Ok(a) => Ok( Self::file(a) ),
               Err(e) => Err(e),
          }
     }

}




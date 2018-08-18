

use std::fs::File;
use log::default_one::LogOneDefault;
use std::sync::Mutex;
use log::raw_lock::LogLockRawIO;
use std::marker::PhantomData;
use log_panic::LogPanic;
use log_write::LogWrite;

/*
pub fn default_open_file<'a, PA: AsRef<Path>>(path: PA) -> LogOneDefault<'a, W, P, LogLockRawIO<'a, Mutex<File>> + 'a, Mutex<File>> {
     
}
*/

/*
pub fn open_file<'a, W: LogWrite, P: LogPanic<W>, PA: AsRef<Path>>(path: PA) -> LogOneDefault<'a, W, P, LogLockRawIO<'a, Mutex<File>>, Mutex<File>> {

}
*/



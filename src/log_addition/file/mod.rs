

use std::fs::File;
use log::default_one::LogOneDefault;
use std::sync::Mutex;
use std::marker::PhantomData;
use log_panic::LogPanic;
use log_shape::LogShape;

/*
pub fn default_open_file<'a, PA: AsRef<Path>>(path: PA) -> LogOneDefault<'a, W, P, LogWrite<'a, Mutex<File>> + 'a, Mutex<File>> {
     
}
*/

/*
pub fn open_file<'a, W: LogShape, P: LogPanic<W>, PA: AsRef<Path>>(path: PA) -> LogOneDefault<'a, W, P, LogWrite<'a, Mutex<File>>, Mutex<File>> {

}
*/





use std::io::Write;
use log_lock::mutex_nf::LogSafeMutexLockNF;
use std::sync::Mutex;
use std::fs::File;
use log_lock::mutex::LogSafeMutexLock;
use log_write::LogWrite;


    

/*
impl<'a> LogWrite<'a, LogSafeMutexLock<'a, File>> for Mutex<File> {
     #[inline]
     fn lock(&'a self) -> LogSafeMutexLock<'a, File> {
          LogSafeMutexLock::mutex(self)
     }
}

impl<'a> LogWrite<'a, LogSafeMutexLockNF<'a, File>> for Mutex<File> {
     #[inline]
     fn lock(&'a self) -> LogSafeMutexLockNF<'a, File> {
          LogSafeMutexLockNF::mutex(self)
     }
}
*/
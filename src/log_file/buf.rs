
use crate::log_core::LogExtend;
use crate::log_core::LogStatic;
use crate::log_core::LogLockIO;
use crate::log_core::LogFlush;
use crate::log_core::LogBase;
use crate::log_core::LogShape;
use crate::log_shape::DefLogNoColorShape;
use std::io::Error;
use cluExtIO::GuardWrite;
use cluExtIO::MutexWrite;
use std::fs::File;
use cluExtIO::ExtWrite;

use std::marker::PhantomData;
use std::fmt::Arguments;
use std::io::Write;
use std::io;
use std::path::Path;
use std::fs::OpenOptions;

#[derive(Debug)]
pub struct LogFile<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: 'a +  Write> {
	_w:	PhantomData<W>,
	_ln: PhantomData<&'a ()>,
	_ol: PhantomData<OL>,
	
	out: O,
}



impl<'a, S: LogShape> LogFile< 'a, S, MutexWrite<File>, GuardWrite<'a, File> > {
     pub fn file(f: File) -> Self {
          Self::new(
               MutexWrite::new(f)
          )
     }

     pub fn open_path<P: AsRef<Path>>(p: P) -> Result<Self, Error> {
		let file = OpenOptions::new().read(false).write(true).create(false).append(true).open(p)?;
          /*match File::open(p) {
               Ok(a) => Ok( Self::file(a) ),
               Err(e) => Err(e),
          }*/

		Ok( Self::file(file) )
     }

     pub fn create_path<P: AsRef<Path>>(p: P) -> Result<Self, Error> {
          let file = OpenOptions::new().read(false).write(true).create(true).append(false).truncate(true).open(p)?;
          /*match File::open(p) {
               Ok(a) => Ok( Self::file(a) ),
               Err(e) => Err(e),
          }*/

		Ok( Self::file(file) )
     }
}


impl<'a> LogFile< 'a, DefLogNoColorShape, MutexWrite<File>, GuardWrite<'a, File> > {
     #[inline(always)]
     pub fn default_file(f: File) -> Self {
          Self::file(f)
     }

     #[inline(always)]
     pub fn default_open_path<P: AsRef<Path>>(p: P) -> Result<Self, Error> {
          Self::open_path(p)
     }

     #[inline(always)]
     pub fn default_create_path<P: AsRef<Path>>(p: P) -> Result<Self, Error> {
          Self::create_path(p)
     }
}

impl<'a> From<File> for LogFile< 'a, DefLogNoColorShape, MutexWrite<File>, GuardWrite<'a, File> > {
     #[inline(always)]
     fn from(f: File) -> Self {
          Self::default_file(f)
     }
}



impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogFile<'a, W, O, OL> {
	#[inline]
	pub fn new(out: O) -> Self {		
		Self {
			_w:	PhantomData,
			_ln: PhantomData,
			_ol: PhantomData,
			
			out: out,
		}
	}
}

impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogBase<'a> for LogFile<'a, W, O, OL> {
	fn warning<'l>(&'a self, args: Arguments) -> io::Result<()> {
		W::warning(self.out.lock(), args)
	}
	
	fn info<'l>(&'a self, args: Arguments) -> io::Result<()> {
		W::info(self.out.lock(), args)
	}
	
	fn error<'l>(&'a self, args: Arguments) -> io::Result<()> {
		W::error(self.out.lock(), args)
	}
	
	fn panic<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		W::panic(self.out.lock(), args)
	}
	
	fn unknown<'l>(&'a self, name: &'static str, args: Arguments<'l>) -> io::Result<()> {
		W::unknown(self.out.lock(), name, args)
	}

	fn trace<'l>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'l>) -> io::Result<()> {
		W::trace(self.out.lock(), line, pos, file, args)
	}
	
	fn print<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		W::print(self.out.lock(), args)
	}
	
	fn eprint<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		W::eprint(self.out.lock(), args)
	}
}

impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogFlush<'a> for LogFile<'a, W, O, OL> {
	fn flush_out(&'a self) -> io::Result<()> {
		self.out.lock().flush()
	}
	
	fn flush_err(&'a self) -> io::Result<()> {
		self.out.lock().flush()
	}
	#[inline]
	fn flush(&'a self) -> io::Result<()> {
		self.flush_out()
	}
}
	
impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogLockIO<'a> for LogFile<'a, W, O, OL> {
	fn raw_lock_out(&'a self) -> Box<Write + 'a> {
		//GuardWrite::boxed(self.out.lock())
		Box::new(self.out.lock())
	}

	fn raw_lock_err(&'a self) -> Box<Write + 'a> {
		//GuardWrite::boxed(self.out.lock())
		Box::new(self.out.lock())
	}
}


impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogStatic<'a> for LogFile<'a, W, O, OL> {}
impl<'a, W: LogShape, O: ExtWrite<'a, LockWrite = OL>, OL: Write> LogExtend<'a> for LogFile<'a, W, O, OL> {}


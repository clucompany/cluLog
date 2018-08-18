

use std::sync::Mutex;
use std::sync::Arc;
use std::path::Path;
use std::fs::File;
use log_addition::union::LogUnionConst;
use std::fmt::Arguments;
use log::LogBase;
use log::LogFlush;
use log::raw_lock::LogLockRawIO;
use std::io::Write;
use std::marker::PhantomData;
use log_write::LogWrite;
use log_panic::LogPanic;
use std::io;
use log::lock::LogLockIO;
use log::lock::default::LogSafeLock;
use log::lock::default_nf::LogSafeLockNF;
use log_addition::empty::LogEmptyConst;
use log::LogStatic;
use log::LogExtend;


pub struct LogFile<'a, WRITER: LogWrite, Panic: LogPanic> {
     _a: PhantomData<&'a ()>,
     _w2: PhantomData<WRITER>,
     _p: PhantomData<Panic>,

     out: Arc<Mutex<File>>,
}

impl<'a, WRITER: LogWrite, Panic: LogPanic> LogFile<'a, WRITER, Panic> {
     #[inline]
     pub fn new(out: Arc<Mutex<File>>) -> Self {
          LogFile {
               _a: PhantomData,
               _w2: PhantomData,
               _p: PhantomData,

               out: out,
          }
     }
     
	pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
		match File::open(path) {
			Ok(a) => Ok( Self::new(Arc::new(Mutex::new(a)))),
			Err(e) => Err( e ),
		}
	}
}

impl<'a, W: 'a + Write, OUT: LogLockRawIO<'a, W>, WRITER: LogWrite, Panic: LogPanic> LogFlush for LogFile<'a, W, OUT, WRITER, Panic> {
	fn flush_out(&mut self) -> io::Result<()> {
		self.out.flush()
	}

     #[inline(always)]
	fn flush_err(&mut self) -> io::Result<()> {
		Ok( () )
	}
}
	
impl<'a, W: 'a + Write, OUT: LogLockRawIO<'a, W>, WRITER: LogWrite, Panic: LogPanic>  LogLockIO<'a> for LogFile<'a, W, OUT, WRITER, Panic> {
	fn lock_out(&'a self) -> Box<'a + Write> {
		LogSafeLock::boxed(self.out.lock())
	}
	
	fn lock_err(&'a self) -> Box<'a + Write> {
		LogSafeLock::empty_boxed()
	}

	fn no_flush_lock_out(&'a self) -> Box<'a + Write> {
		LogSafeLockNF::boxed(self.out.lock())
	}

	fn no_flush_lock_err(&'a self) -> Box<'a + Write> {
		LogSafeLockNF::empty_boxed()
	}
}



impl<'a, W: 'a + Write, OUT: LogLockRawIO<'a, W>, WRITER: LogWrite, Panic: LogPanic> LogBase<'a> for LogFile<'a, W, OUT, WRITER, Panic> {
	fn warning<'l>(&'a self, args: Arguments) -> io::Result<()> {
		WRITER::warning(self.out.lock(), args)
	}
	
	fn info<'l>(&'a self, args: Arguments) -> io::Result<()> {
		WRITER::info(self.out.lock(), args)
	}
	
	fn error<'l>(&'a self, args: Arguments) -> io::Result<()> {
		WRITER::error(self.out.lock(), args)
	}
	
	fn panic<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		Panic::panic(self.out.lock(), args)
	}
	
	fn unknown<'l>(&'a self, name: &'static str, args: Arguments<'l>) -> io::Result<()> {
		WRITER::unknown(self.out.lock(), name, args)
	}

	fn trace<'l>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'l>) -> io::Result<()> {
		WRITER::trace(self.out.lock(), line, pos, file, args)
	}
	
	fn print<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		WRITER::print(self.out.lock(), args)
	}
	
	fn eprint<'l>(&'a self, args: Arguments<'l>) -> io::Result<()> {
		WRITER::eprint(self.out.lock(), args)
	}
}



impl<'a, W: 'a + Write, OUT: LogLockRawIO<'a, W>, WRITER: LogWrite, Panic: LogPanic> LogUnionConst<'a> for LogFile<'a, W, OUT, WRITER, Panic> {}
impl<'a, W: 'a + Write, OUT: LogLockRawIO<'a, W>, WRITER: LogWrite, Panic: LogPanic> LogStatic<'a> for LogFile<'a, W, OUT, WRITER, Panic> {}
impl<'a, W: 'a + Write, OUT: LogLockRawIO<'a, W>, WRITER: LogWrite, Panic: LogPanic> LogExtend<'a> for LogFile<'a, W, OUT, WRITER, Panic> {}


use log_panic::DefaultPanic;
use log_write::DefNoColorWrite;
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
use log::lock::default::LogLock;
use log::lock::default_no_flush::LogLockNoFlush;
use log_addition::empty::LogEmptyConst;
use log::LogStatic;
use log::LogExtend;


pub struct LogFile<'a, W: 'a + Write, OUT: LogLockRawIO<'a, W> + Sized, WRITER: LogWrite, Panic: LogPanic> {
     _a: PhantomData<&'a ()>,
     _w: PhantomData<W>,
     _o: PhantomData<OUT>,
     _w2: PhantomData<WRITER>,
     _p: PhantomData<Panic>,

     out: OUT,
}

impl<'a, W: 'a + Write, OUT: LogLockRawIO<'a, W> + Sized, WRITER: LogWrite, Panic: LogPanic> LogFile<'a, W, OUT, WRITER, Panic> {
     #[inline]
     pub fn new(out: OUT) -> Self {
          LogFile {
               _a: PhantomData,
               _w: PhantomData,
               _o: PhantomData,
               _w2: PhantomData,
               _p: PhantomData,

               out: out,
          }
     }
     
}

/*
impl<'a, WRITER: LogWrite, Panic: LogPanic> LogFile<'a, File, LogLockRawIO<'a, File>, WRITER, Panic> {
     
}
*/

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
		LogLock::boxed(self.out.lock())
	}
	
	fn lock_err(&'a self) -> Box<'a + Write> {
		LogLock::empty_boxed()
	}

	fn no_flush_lock_out(&'a self) -> Box<'a + Write> {
		LogLockNoFlush::boxed(self.out.lock())
	}

	fn no_flush_lock_err(&'a self) -> Box<'a + Write> {
		LogLockNoFlush::empty_boxed()
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
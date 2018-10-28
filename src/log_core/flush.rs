

use std::io;


///Generalization for cleaning output streams
pub trait LogFlush {
	///Flush the output stream
	fn flush_out(&self) -> io::Result<()>;

	///Flush the err-output stream
	fn flush_err(&self) -> io::Result<()>;
	
	///Flush Out stream and Err stream
	#[inline]
	fn flush(&self) -> io::Result<()> {
		if let Err(e) = self.flush_out() {
			return Err(e);
		}
		self.flush_err()
	}
}

impl<'a, A: LogFlush> LogFlush for &'a A {
	///Flush the output stream
	#[inline(always)]
	fn flush_out(&self) -> io::Result<()> {
		(**self).flush_out()
	}

	///Flush the err-output stream
	#[inline(always)]
	fn flush_err(&self) -> io::Result<()> {
		(**self).flush_err()
	}
	
	///Flush Out stream and Err stream
	#[inline(always)]
	fn flush(&self) -> io::Result<()>  {
		(**self).flush()
	}
}

impl<'a, A: LogFlush> LogFlush for &'a mut A {
	///Flush the output stream
	#[inline(always)]
	fn flush_out(&self) -> io::Result<()> {
		(**self).flush_out()
	}

	///Flush the err-output stream
	#[inline(always)]
	fn flush_err(&self) -> io::Result<()> {
		(**self).flush_err()
	}
	
	///Flush Out stream and Err stream
	#[inline(always)]
	fn flush(&self) -> io::Result<()>  {
		(**self).flush()
	}
}
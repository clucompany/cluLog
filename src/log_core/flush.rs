
use std::io;


///Generalization for cleaning output streams
pub trait LogFlush<'a> {
	///Flush the output stream
	fn flush_out(&'a self) -> io::Result<()>;

	///Flush the err-output stream
	fn flush_err(&'a self) -> io::Result<()>;
	
	///Flush Out stream and Err stream
	#[inline]
	fn flush(&'a self) -> io::Result<()> {
		let e = self.flush_out();
		let e2 = self.flush_err();

		if let Err(_) = e {
			return e;
		}
		e2
	}
}

impl<'a, 'l, A: LogFlush<'a>> LogFlush<'a> for &'l A {
	///Flush the output stream
	#[inline(always)]
	fn flush_out(&'a self) -> io::Result<()> {
		A::flush_out(self)
	}

	///Flush the err-output stream
	#[inline(always)]
	fn flush_err(&'a self) -> io::Result<()> {
		A::flush_err(self)
	}
	
	///Flush Out stream and Err stream
	#[inline(always)]
	fn flush(&'a self) -> io::Result<()>  {
		A::flush(self)
	}
}

impl<'a, 'l, A: LogFlush<'a>> LogFlush<'a> for &'l mut A {
	///Flush the output stream
	#[inline(always)]
	fn flush_out(&'a self) -> io::Result<()> {
		A::flush_out(self)
	}

	///Flush the err-output stream
	#[inline(always)]
	fn flush_err(&'a self) -> io::Result<()> {
		A::flush_err(self)
	}
	
	///Flush Out stream and Err stream
	#[inline(always)]
	fn flush(&'a self) -> io::Result<()>  {
		A::flush(self)
	}
}
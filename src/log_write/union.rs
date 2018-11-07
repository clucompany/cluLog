

use log_addition::LogEmptyConst;
use log_write::EmptyWrite;
use std::fmt::Debug;
use std::io::Write;
use std::io;


pub trait UnionWriteConst: Write {
     #[inline]
     fn union<B: UnionWriteConst>(self, b: B) -> UnionWrite<Self, B> where Self: Sized { 
          UnionWrite::new(self, b)
     }
}

impl<T: Write> UnionWriteConst for T {}





#[allow(non_camel_case_types)]
pub struct UnionWrite<W: Write, W2: Write>(W,W2);

impl<W: Write, W2: Write> UnionWrite<W, W2> {
	#[inline]
	pub fn new(out: W, out2: W2) -> Self {
		UnionWrite(out, out2)
	}
     #[inline]
	pub fn boxed(out: W, out2: W2) -> Box<Self> {
		Box::new(Self::new(out, out2))
	}
}

impl LogEmptyConst for UnionWrite<EmptyWrite, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		UnionWrite::new(EmptyWrite::new(), EmptyWrite::new())
	}
}



impl<W: Write, W2: Write> Debug for UnionWrite<W, W2> {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("UnionWrite { .. }")
	}
}
/*
impl<'a, W: Write + 'a, W2: Write + 'a> Drop for UnionWrite<'a, W, W2> {
	#[inline(always)]
	fn drop(&mut self) {
		let _e = self.flush();
	}
}
*/

impl<W: Write, W2: Write> Write for UnionWrite<W, W2> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          let e = self.0.write(buf);
          let e2 = self.1.write(buf);
          match e {
               Ok(s) => {
                    match e2 {
                         Ok(s2) => return Ok({
                              if s2 >= s {
                                   s
                              }else {
                                   s2
                              }
                         }),
                         a => return a,
                    }
               },
               a => return a,
          }
     }
     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          let e = self.0.flush();
          let e2 = self.1.flush();
		if let Err(e) = e {
               return Err(e);
          }
          e2
     }

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()> {
		let e = self.0.write_all(buf);
          let e2 = self.1.write_all(buf);
		if let Err(e) = e {
               return Err(e);
          }
          e2
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: ::std::fmt::Arguments) -> ::std::io::Result<()> {
          let e = self.0.write_fmt(fmt);
          let e2 = self.1.write_fmt(fmt);
		if let Err(e) = e {
               return Err(e);
          }
          e2
	}
}

//impl<'a, W: Write + 'a, W2: Write + 'a> LogSafeLock<'a> for UnionWrite<'a, W, W2> {}


impl<W: Write + Clone, W2: Write + Clone> Clone for UnionWrite<W, W2> {
     fn clone(&self) -> Self {
          Self::new(self.0.clone(), self.1.clone())
     }

}
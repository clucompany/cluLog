

use log_write::EmptyWrite;
use log_addition::empty::LogEmptyConst;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::io::Write;
use std::io;


pub trait UnionWriteConst<'a>: Write + 'a {
     #[inline]
     fn union<B: UnionWriteConst<'a> + Sized + 'a>(self, b: B) -> UnionWrite<'a, Self, B> where Self: Sized { 
          UnionWrite::new(self, b)
     }
}

impl<'a, T: Write + 'a> UnionWriteConst<'a> for T {}





#[allow(non_camel_case_types)]
pub struct UnionWrite<'a, W: Write + 'a, W2: Write + 'a>(W,W2, PhantomData<&'a ()>);

impl<'a, W: Write + 'a, W2: Write + 'a> UnionWrite<'a, W, W2> {
	#[inline]
	pub fn new(out: W, out2: W2) -> Self {
		UnionWrite(out, out2, PhantomData)
	}
     #[inline]
	pub fn boxed(out: W, out2: W2) -> Box<Write + 'a>{
		Box::new(Self::new(out, out2))
	}
}

impl<'a> LogEmptyConst for UnionWrite<'a, EmptyWrite, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite, EmptyWrite)
	}
}



impl<'a, W: Write + 'a, W2: Write + 'a> Debug for UnionWrite<'a, W, W2> {
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

impl<'a, W: Write + 'a, W2: Write + 'a> Write for UnionWrite<'a, W, W2> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          match self.0.write(buf) {
               Ok(s) => {
                    match self.1.write(buf) {
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
          if let Err(e) = self.0.flush() {
               return Err(e);
          }
          self.1.flush()
     }

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()> {
		if let Err(e) = self.0.write_all(buf) {
               return Err(e);
          }
          self.1.write_all(buf)
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: ::std::fmt::Arguments) -> ::std::io::Result<()> {
		if let Err(e) = self.0.write_fmt(fmt) {
               return Err(e);
          }
          self.1.write_fmt(fmt)
	}
}

//impl<'a, W: Write + 'a, W2: Write + 'a> LogSafeLock<'a> for UnionWrite<'a, W, W2> {}


impl<'a, W: Write + 'a + Clone, W2: Write + 'a + Clone> Clone for UnionWrite<'a, W, W2> {

     fn clone(&self) -> Self {
          Self::new(self.0.clone(), self.1.clone())
     }

}
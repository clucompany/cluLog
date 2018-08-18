

use log_lock::LogLockUnionConst;
use log_lock::LogLock;
use log_addition::empty::LogEmptyConst;
use std::fmt::Debug;
use log_addition::empty::empty_write::EmptyWrite;
use std::marker::PhantomData;
use std::io::Write;
use std::io;

#[allow(non_camel_case_types)]
pub struct UnionLock<'a, W: Write + 'a, W2: Write + 'a>(W,W2, PhantomData<&'a ()>);

impl<'a, W: Write + 'a, W2: Write + 'a> UnionLock<'a, W, W2> {
	#[inline]
	pub fn new(out: W, out2: W2) -> Self {
		UnionLock(out, out2, PhantomData)
	}
     #[inline]
	pub fn boxed(out: W, out2: W2) -> Box<LogLock<'a> + 'a>{
		Box::new(Self::new(out, out2))
	}
}

impl<'a> LogEmptyConst for UnionLock<'a, EmptyWrite, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite, EmptyWrite)
	}
}



impl<'a, W: Write + 'a, W2: Write + 'a> Debug for UnionLock<'a, W, W2> {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("UnionLock { .. }")
	}
}

impl<'a, W: Write + 'a, W2: Write + 'a> Drop for UnionLock<'a, W, W2> {
	#[inline(always)]
	fn drop(&mut self) {
		let _e = self.flush();
	}
}

impl<'a, W: Write + 'a, W2: Write + 'a> Write for UnionLock<'a, W, W2> {
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
}

impl<'a, W: Write + 'a, W2: Write + 'a> LogLock<'a> for UnionLock<'a, W, W2> {}
impl<'a, W: Write + 'a, W2: Write + 'a> LogLockUnionConst<'a> for UnionLock<'a, W, W2> {}
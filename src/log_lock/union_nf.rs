
use log_lock::LogSafeLock;
use log_addition::empty::LogEmptyConst;
use std::fmt::Debug;
use log_addition::empty::empty_write::EmptyWrite;
use std::marker::PhantomData;
use std::io::Write;
use std::io;

#[allow(non_camel_case_types)]
pub struct UnionNFLock<'a, W: Write + 'a, W2: Write + 'a>(W,W2, PhantomData<&'a ()>);

impl<'a, W: Write + 'a, W2: Write + 'a> UnionNFLock<'a, W, W2> {
	#[inline]
	pub fn new(out: W, out2: W2) -> Self {
		UnionNFLock(out, out2, PhantomData)
	}
     #[inline]
	pub fn boxed(out: W, out2: W2) -> Box<LogSafeLock<'a> + 'a> {
		Box::new(Self::new(out, out2))
	}
}

impl<'a> LogEmptyConst for UnionNFLock<'a, EmptyWrite, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite, EmptyWrite)
	}
}



impl<'a, W: Write + 'a, W2: Write + 'a> Debug for UnionNFLock<'a, W, W2> {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("UnionNFLock { .. }")
	}
}

impl<'a, W: Write + 'a, W2: Write + 'a> Write for UnionNFLock<'a, W, W2> {
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
impl<'a, W: Write + 'a, W2: Write + 'a> LogSafeLock<'a> for UnionNFLock<'a, W, W2> {}

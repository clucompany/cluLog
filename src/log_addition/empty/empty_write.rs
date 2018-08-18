
use std::io::Write;
use std::io;

#[derive(Debug, Clone)]
pub struct EmptyWrite;

impl EmptyWrite {
	#[inline]
	pub fn new() -> Self {
		EmptyWrite
	}
}

impl Write for EmptyWrite {
	#[inline(always)]
	fn write<'a>(&mut self, _buf: &'a [u8]) -> io::Result<usize> {
		Ok( 0 )
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()> {
		Ok( () )
	}
}

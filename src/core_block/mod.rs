
use crate::core::Log;

mod block_debug;
mod block_out;
mod block_err;
mod block_warning;
mod block_aout;


pub use self::block_debug::*;
pub use self::block_out::*;
pub use self::block_err::*;
pub use self::block_warning::*;
pub use self::block_out::*;
pub use self::block_aout::*;

pub trait BlockLogTrait<'a>: 
	  BlockLogDebugTrait<'a>
	+ BlockLogOutTrait<'a> 
	+ BlockLogErrTrait<'a> 
	+ BlockLogWarningTrait<'a> {

}

impl<'a, L: Log<'a> + Sized> BlockLogTrait<'a> for L {}



pub trait BlockLogDebugTrait<'a>: Sized + Log<'a> {
	#[inline(always)]
	fn block_debug(self) -> BlockLogDebug<'a, Self> {
		BlockLogDebug::new(self)
	}
}

impl<'a, L: Log<'a> + Sized> BlockLogDebugTrait<'a> for L {}

pub trait BlockLogOutTrait<'a>: Sized + Log<'a> {
	#[inline(always)]
	fn block_out(self) -> BlockLogOut<'a, Self> {
		BlockLogOut::new(self)
	}

	#[inline(always)]
	fn block_err_out(self) -> BlockLogErrOut<'a, Self> {
		BlockLogErrOut::new(self)
	}
}

impl<'a, L: Log<'a> + Sized> BlockLogOutTrait<'a> for L {}

pub trait BlockLogErrTrait<'a>: Sized + Log<'a> {
	#[inline(always)]
	fn block_err(self) -> BlockLogErr<'a, Self> {
		BlockLogErr::new(self)
	}
}

impl<'a, L: Log<'a> + Sized> BlockLogErrTrait<'a> for L {}

pub trait BlockLogWarningTrait<'a>: Sized + Log<'a> {
	#[inline(always)]
	fn block_warning(self) -> BlockLogWarning<'a, Self> {
		BlockLogWarning::new(self)
	}
}

impl<'a, L: Log<'a> + Sized> BlockLogWarningTrait<'a> for L {}

/*

War: (block_warning)
	warning



Panic: (block_panic)
	panic


Err: (block_err)
	eprint
	error


Out: (block_out)
	print


Debug: (block_debug)
	info
	trace
	unknown

*/
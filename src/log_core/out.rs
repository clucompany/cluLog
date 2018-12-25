

use std::fmt::Arguments;
use std::io;

///Generalization of the basic methods of information output
pub trait LogBase<'a> {
	fn unknown<'s>(&'a self, name: &'static str, args: Arguments<'s>) -> io::Result<()>;

	fn trace<'s>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()>;

	fn warning<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn info<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn error<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn panic<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn print<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;

	fn eprint<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
}

///Generalization of the basic methods of information output
impl<'a, 'l, A: LogBase<'a>> LogBase<'a> for &'l A {
	#[inline(always)]
	fn unknown<'s>(&'a self, name: &'static str, args: Arguments<'s>) -> io::Result<()> {
		A::unknown(self, name, args)
	}

	#[inline(always)]
	fn trace<'s>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()> {
		A::trace(self, line, pos, file, args)
	}


	#[inline(always)]
	fn warning<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::warning(self, args)
	}
	
	#[inline(always)]
	fn info<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::info(self, args)
	}
	
	#[inline(always)]
	fn error<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::error(self, args)
	}
	
	#[inline(always)]
	fn panic<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::panic(self, args)
	}
	
	#[inline(always)]
	fn print<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::print(self, args)
	}

	#[inline(always)]
	fn eprint<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::eprint(self, args)
	}
}


impl<'a, 'l, A: LogBase<'a>> LogBase<'a> for &'l mut A {
	#[inline(always)]
	fn unknown<'s>(&'a self, name: &'static str, args: Arguments<'s>) -> io::Result<()> {
		A::unknown(self, name, args)
	}

	#[inline(always)]
	fn trace<'s>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()> {
		A::trace(self, line, pos, file, args)
	}


	#[inline(always)]
	fn warning<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::warning(self, args)
	}
	
	#[inline(always)]
	fn info<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::info(self, args)
	}
	
	#[inline(always)]
	fn error<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::error(self, args)
	}
	
	#[inline(always)]
	fn panic<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::panic(self, args)
	}
	
	#[inline(always)]
	fn print<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::print(self, args)
	}

	#[inline(always)]
	fn eprint<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
		A::eprint(self, args)
	}
}


/*
pub struct TestLog<'a, A: Write>(A, PhantomData<&'a ()>);

macro_rules! Log_base {
	[
		$ns: ident < $($life:tt),* + $($n_t: ident  =  $v_t: tt <$life_t:tt>),* >:

		trace[line, pos, file, args] => $trace_function: block;
		unknown[name, args] => $unknown_function: block;

		$($n_function: ident [args] => $v_function: block;)*
	] => {
		impl< $($life),* , $($n_t:$v_t<$life_t>),* > LogBase<'a> for $ns < $($life),* , $( $n_t ),* > {
			
			#[allow(unused_variables)]
			fn trace<'s>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()> {
				let trace = self;
				$trace_function
			}

			#[allow(unused_variables)]
			fn unknown<'s>(&'a self, name: &'static str, args: Arguments<'s>) -> io::Result<()> {
				$unknown_function
			}

			
			$(
				#[allow(unused_variables)]
				fn $n_function<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
					$v_function
				}

			)*

		}
	};


	[
		$ns: ident < $($life:tt),* + $($n_t: ident  =  $v_t: tt),* >:

		trace[line, pos, file, args] => $trace_function: block;
		unknown[name, args] => $unknown_function: block;

		$($n_function: ident [args] => $v_function: block;)*
	] => {
		impl< $($life),* , $($n_t:$v_t),* > LogBase<'a> for $ns < $($life),* , $($n_t),* > {
			
			#[allow(unused_variables)]
			fn trace<'s>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()> {
				$trace_function
			}

			#[allow(unused_variables)]
			fn unknown<'s>(&'a self, name: &'static str, args: Arguments<'s>) -> io::Result<()> {
				$unknown_function
			}

			
			$(
				#[allow(unused_variables)]
				fn $n_function<'s>(&'a self, args: Arguments<'s>) -> io::Result<()> {
					$v_function
				}

			)*

		}
	};
}


Log_base![TestLog< 'a + A = Write >:
	trace[line, pos, file, args] => {
		Ok( () )
	};
	unknown[name, args] => {
		Ok( () )
	};

	warning[args] => {
		Ok( () )
	};
	info[args] => {
		Ok( () )
	};
	error[args] => {
		Ok( () )
	};
	panic[args] => {
		Ok( () )
	};
	
	print[args] => {
		Ok( () )
	};
	eprint[args] => {
		Ok( () )
	};
];

*/
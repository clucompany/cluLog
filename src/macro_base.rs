#[macro_use]



macro_rules! Log_base {
	[$ns: ident < $($life:tt),* + $($n_t: ident  =  $v_t: tt),* >; 

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


     /*[$ns: ident < $($life:tt),* >; $($args: ty),*] => {
          #[derive(Debug)]
          pub struct $ns<$($life),*>( ($($args),*) );
          impl<$($life),*> $ns<$($life),*> {
               #[inline]
               pub fn new(a: ($($args),* )) -> Self {
                    $ns(a)
               }
          }
     }*/
}



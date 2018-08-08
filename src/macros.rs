
/*
#[macro_export]
macro_rules! warning {
	($($arg:tt)*) => (
		::clulog::as_log().warning(format_args!( $($arg)* ));		
	)
}*/

#[macro_export]
macro_rules! warn {
	($($arg:tt)*) => (
		::clulog::as_log().warning(format_args!( $($arg)* ));		
	)
}



#[macro_export]
macro_rules! info {
	($($arg:tt)*) => (
		::clulog::as_log().info(format_args!( $($arg)* ));		
	)
}

#[macro_export]
macro_rules! inf {
	($($arg:tt)*) => (
		::clulog::as_log().info(format_args!( $($arg)* ));		
	)
}


/*
#[macro_export]
macro_rules! error {
	($($arg:tt)*) => (
		::clulog::as_log().error(format_args!( $($arg)* ));	
	)
}*/
#[macro_export]
macro_rules! err {
	($($arg:tt)*) => (
		::clulog::as_log().error(format_args!( $($arg)* ));	
	)
}




#[macro_export]
macro_rules! panic {
	($($arg:tt)*) => (
		::clulog::as_log().panic(format_args!( $($arg)* ));
	)
}

/*
#[macro_export]
macro_rules! unknown {
	($name:expr, $($arg:tt)*) => (
		::clulog::as_log().unknown($name, format_args!( $($arg)* ));
	);
	
	( $($arg:tt)* ) => {
		unknown!("UNK", $($arg)*)
	}
}*/



#[macro_export]
macro_rules! unk {
	($name:expr, $($arg:tt)*) => (
		::clulog::as_log().unknown($name, format_args!( $($arg)* ));
	);
	
	( $($arg:tt)* ) => {
		unk!("UNK", $($arg)*)
	}
}


#[macro_export]
macro_rules! lock {
	(out) => (
		::clulog::as_log().lock_out()
	);
	(err) => (
		::clulog::as_log().lock_err()
	);
}

///Blocking threads with automatic cleaning
#[macro_export]
macro_rules! lock_out {
	() => {
		lock!(out)
	};
}

///Blocking threads with automatic cleaning
#[macro_export]
macro_rules! lock_err {
	() => {
		lock!(err)
	};
}



#[macro_export]
macro_rules! flush {
	() => (
		::clulog::as_log().flush();
	);
	
	(out) => (
		::clulog::as_log().flush_out();
	);
	(err) => (
		::clulog::as_log().flush_err();
	);
}
#[macro_export]
macro_rules! flush_out {
	() => (
		flush!(out);
	);
}
#[macro_export]
macro_rules! flush_err {
	() => (
		flush!(err);
	);
}

//PRINT

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => {
		::clulog::as_log().print(format_args!($($arg)*));
	}
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}


#[macro_export]
macro_rules! eprint {
	($($arg:tt)*) => {
		::clulog::as_log().eprint(format_args!($($arg)*));
	}
}

#[macro_export]
macro_rules! eprintln {
    () => (eprint!("\n"));
    ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
}



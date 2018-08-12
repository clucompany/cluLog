
#[macro_export]
macro_rules! warn {
	($($arg:tt)*) => (
		::clulog::as_log().warning(format_args!( $($arg)* ));		
	)
}


#[macro_export]
macro_rules! inf {
	($($arg:tt)*) => (
		::clulog::as_log().info(format_args!( $($arg)* ));		
	)
}


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

#[macro_export]
macro_rules! log {
	(WARN: $($arg:tt)*) => (
		warn!($($arg)*);
	);
	(INF: $($arg:tt)*) => (
		inf!($($arg)*);
	);
	(ERR: $($arg:tt)*) => (
		err!($($arg)*);
	);
	(UNK: $($arg:tt)*) => (
		unk!($($arg)*);
	);

	(OUT: $($arg:tt)*) => (
		print!($($arg)*);
	);
	(OUTn: $($arg:tt)*) => (
		println!($($arg)*);
	);
	(ERR: $($arg:tt)*) => (
		eprint!($($arg)*);
	);
	(ERRn: $($arg:tt)*) => (
		eprintln!($($arg)*);
	);
}


#[macro_export]
macro_rules! unk {
	(?, $($arg:tt)*) => (
		::clulog::as_log().unknown("UNK", format_args!( $($arg)* ));
	);
	($name:expr, $($arg:tt)*) => (
		::clulog::as_log().unknown($name, format_args!( $($arg)* ));
	);
}

///Blocking out threads
#[macro_export]
macro_rules! lock {
	(out) => (
		::clulog::as_log().lock_out()
	);
	(err) => (
		::clulog::as_log().lock_err()
	);
	(no_flush_out) => (
		::clulog::as_log().no_flush_lock_out()
	);
	(no_flush_err) => (
		::clulog::as_log().no_flush_lock_err()
	);
}

///Blocking out threads with automatic cleaning
#[macro_export]
macro_rules! lock_out {
	() => {
		lock!(out)
	};
	(no_flush) => {
		lock!(no_flush_out)
	};
}

///Blocking out-err threads with automatic cleaning
#[macro_export]
macro_rules! lock_err {
	() => {
		lock!(err)
	};
	(no_flush) => {
		lock!(no_flush_err)
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
	() => (
		print!("\n")
	);
	($fmt:expr) => (
		print!(concat!($fmt, "\n"))
	);
	($fmt:expr, $($arg:tt)*) => (
		print!(concat!($fmt, "\n"), $($arg)*)
	);
}


#[macro_export]
macro_rules! eprint {
	($($arg:tt)*) => {
		::clulog::as_log().eprint( format_args!( $($arg)* ) );
	}
}

#[macro_export]
macro_rules! eprintln {
	() => (
	    eprint!("\n")
	);
	($fmt:expr) => (
		eprint!(concat!($fmt, "\n"))
	);
	($fmt:expr, $($arg:tt)*) => (
		eprint!(concat!($fmt, "\n"), $($arg)*)
	);
}


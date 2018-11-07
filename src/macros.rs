
#[macro_export]
macro_rules! init_clulog {
	(null) => {
		$crate::set_slice_logger(&$crate::LogTotalEmpty)
	};
	
	(none) => {
		$crate::set_logger($crate::LogEmpty::default())
	};
	(total_none) => {
		$crate::set_slice_logger(&$crate::LogTotalEmpty);
	};

	(one) => {
		$crate::set_logger($crate::LogOneDefault::default());
	};
	(one, $e:expr) => {
		$crate::set_logger($crate::LogOneDefault::new($e));
	};

	(union, $a:expr, $b:expr) => {
		$crate::set_logger($a.union($b));
	};

	() => {
		$crate::set_logger($crate::LogDefault::default());
	};
	($e: expr) => {
		$crate::set_logger($crate::LogOneDefault::new($e));
	};
	($e: expr, $e2: expr) => {
		$crate::set_logger($crate::LogDefault::new($e, $e2));
	};
}


///Obtaining a link to active logging
#[macro_export]
macro_rules! as_log {
	() => {
		$crate::as_log()
	};
}


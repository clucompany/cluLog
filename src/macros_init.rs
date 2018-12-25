#[macro_export]
macro_rules! init_clulog {
	(one) => {
		$crate::set_logger($crate::LogDefaultOne::default());
	};
	(one, $e:expr) => {
		$crate::set_logger($crate::LogDefaultOne::new($e));
	};

	(union, $a:expr, $b:expr) => {
          $crate::set_logger($crate::log_union::LogUnionConst::union($a, $b));
		//$crate::set_logger($a.union($b));
	};

	() => {
		$crate::set_logger($crate::LogDefault::default());
	};
	($e: expr) => {
		$crate::set_logger($crate::LogDefaultOne::new($e));
	};
	($e: expr, $e2: expr) => {
		$crate::set_logger($crate::LogDefault::new($e, $e2));
	};
}
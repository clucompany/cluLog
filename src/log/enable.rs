/*
use std::io::Write;
use std::fmt::Debug;
use std::fmt::Arguments;

pub type cluLEResult<T> = Result<T, ::std::io::Error>;

pub trait cluLogEnabled: Debug {
     #[inline(always)]
     fn is_warning_enable() -> bool;

     #[inline(always)]
     fn is_info_enable() -> bool;

     #[inline(always)]
     fn is_error_enable() -> bool;

     #[inline(always)]
     fn is_unknown_enable() -> bool;

     #[inline(always)]
     fn is_print_enable() -> bool;

     #[inline(always)]
     fn is_eprint_enable() -> bool;


     #[inline(always)]
	fn warning<'a>(&self, write: &mut Write, args: Arguments<'a>) -> cluLEResult<()>;
	
	#[inline(always)]
	fn info<'a>(&self, write: &mut Write, args: Arguments<'a>) -> cluLEResult<()>;
	
	#[inline(always)]
	fn error<'a>(&self, write: &mut Write, args: Arguments<'a>) -> cluLEResult<()>;
	
	#[inline(always)]
	fn unknown<'a>(&self, write: &mut Write, name: &'a str, args: Arguments<'a>) -> cluLEResult<()>;
	
	#[inline(always)]
	fn print<'a>(&self, write: &mut Write, args: Arguments<'a>) -> cluLEResult<()>;
	
	#[inline(always)]
	fn eprint<'a>(&self, write: &mut Write, args: Arguments<'a>) -> cluLEResult<()>;
}

#[macro_export]
macro_rules! log_enabled {
     ($name:ident, 
          $WAR:expr      ,    $WAR2:expr     ,
          $INF:expr      ,    $INF2:expr     ,
          $ERR:expr      ,    $ERR2:expr     ,
          $UNK:expr      ,    $UNK2:expr     ,
          $PRINT:expr    ,    $PRINT2:expr   ,
          $EPRINT:expr   ,    $EPRINT2:expr
     ) => {
          #[derive(Debug)]
          pub enum $name;

          impl cluLogEnabled for $name {
               #[inline(always)]
               fn is_warning_enable() -> bool { $WAR }

               #[inline(always)]
               fn is_info_enable() -> bool { $INF }

               #[inline(always)]
               fn is_error_enable() -> bool { $ERR }

               #[inline(always)]
               fn is_unknown_enable() -> bool { $UNK }

               #[inline(always)]
               fn is_print_enable() -> bool { $PRINT }

               #[inline(always)]
               fn is_eprint_enable() -> bool { $EPRINT }


               #[inline(always)]
               fn warning<'a>(&self, write: &mut Write, args: Arguments<'a>) -> cluLEResult<()> {
                    $WAR2
               }
               
               #[inline(always)]
               fn info<'a>(&self, write: &mut Write, args: Arguments<'a>) -> cluLEResult<()> {
                    $INF2
               }
               
               #[inline(always)]
               fn error<'a>(&self, write: &mut Write, args: Arguments<'a>) -> cluLEResult<()> {
                    $ERR2
               }
               
               #[inline(always)]
               fn unknown<'a>(&self, write: &mut Write, name: &'a str, args: Arguments<'a>) -> cluLEResult<()> {
                    $UNK2
               }
               
               #[inline(always)]
               fn print<'a>(&self, write: &mut Write, args: Arguments<'a>) -> cluLEResult<()> {
                    $PRINT2
               }
               
               #[inline(always)]
               fn eprint<'a>(&self, write: &mut Write, args: Arguments<'a>) -> cluLEResult<()> {
                    $EPRINT2
               }
          }
     };
}

log_enabled!(
     All,
     
     true      ,    
     true      ,    
     true      ,    
     true      ,    
     true      ,    
     true      ,    { write.write_fmt(write) }
);

*/
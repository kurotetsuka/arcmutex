use std::sync::{ Arc, Mutex};

pub type ArcMutex<T> = Arc< Mutex<T>>;

#[macro_export]
macro_rules! arcmutex {
	( $x:expr ) => {
		std::sync::Arc::new( std::sync::Mutex::new( $x))}
}

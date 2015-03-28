use std::sync::{ Arc, Mutex};


pub type ArcMutex<T> = Arc< Mutex<T>>;

/// Macro for creating an `ArcMutex`.
///
/// # Examples
/// ```
/// let a : ArcMutex< u8> = arcmutex!( 0);
/// let mut b = a.lock().unwrap();
/// *b += 1;
/// println!( "a: {}", *b);
/// ```

#[macro_export]
macro_rules! arcmutex {
	( $x:expr ) => {
		std::sync::Arc::new( std::sync::Mutex::new( $x))}
}

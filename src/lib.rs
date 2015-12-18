use std::sync::{ Arc, Mutex};

pub type ArcMutex<T> = Arc< Mutex<T>>;

/// Function for creating an `ArcMutex`.
///
/// # Examples
/// ```
/// extern crate arcmutex;
/// use arcmutex::{ arcmutex, ArcMutex};
/// 
/// let a : ArcMutex<u8> = arcmutex( 0);
/// let mut b = a.lock().unwrap();
/// *b += 1;
/// println!( "a: {}", *b);
/// ```

pub fn arcmutex<T>( x : T) -> ArcMutex<T> {
	return Arc::new( Mutex::new( x));}

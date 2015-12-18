extern crate arcmutex;
use arcmutex::{ arcmutex, ArcMutex};

#[test]
fn it_works() {
	let a : ArcMutex< u8> = arcmutex( 0);
	let mut b = a.lock().unwrap();
	*b += 1;
	assert!( *b == 1);
}

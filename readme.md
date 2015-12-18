## ArcMutex
A rust library for making creating `Arc<Mutex<T>>`s more convenient.  
Defines the `ArcMutex<T>` type alias and the `arcmutex( expr)` function.

Contributions welcome! :)

Author: [Kurotetsuka](https://github.com/kurotetsuka)  
License: [MIT](legal/mit.md)  

### Example
```rust
extern crate arcmutex;
use arcmutex::{ arcmutex, ArcMutex};

let a : ArcMutex<u8> = arcmutex( 0);
let mut b = a.lock().unwrap();
*b += 1;
println!( "a: {}", *b);
```

## ArcMutex
A rust library for making working with `Arc<Mutex<T>>`s more convenient.  
Defines the `ArcMutex<T>` type alias and the `arcmutex!( expr)` macro.

Contributions welcome! :)

Author: [Kurotetsuka](https://github.com/kurotetsuka)  
License: [MIT](
	https://github.com/kurotetsuka/hyphaelia/blob/master/legal/mit.md)

### Example
```rust
let a : ArcMutex< u8> = arcmutex!( 0);
let mut b = a.lock().unwrap();
*b += 1;
println!( "a: {}", *b);
```

# select-rs

A Select Async IO with Windows&Linux support Rust library.

[select-rs]: https://github.com/b23r0/select-rs

# Get started

```toml
# Cargo.toml
[dependencies]
select-rs = {git = "https://github.com/b23r0/select-rs"}
```

# Example

```rust
use select_rs::*;

fn main(){
	let fds : FdSet;
	FD_ZERO(&mut fds);
	FD_SET(1 , &mut fds);
	select(2 , std::ptr::null_mut(), &mut fds ,std::ptr::null_mut());
	assert!(FD_ISSET(1, &mut fds));
}
```
# select-rs

A POSIX select async IO with Windows&Linux support Rust library.

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
	let mut fds : FdSet = unsafe {std::mem::zeroed()};
	FD_ZERO(&mut fds);
	FD_SET(0 , &mut fds);
	assert!(select(1, std::ptr::null_mut() , &mut fds ,std::ptr::null_mut()) > 0);
	assert!(FD_ISSET(0, &mut fds));
}
```
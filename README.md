# select-rs [![Build Status](https://app.travis-ci.com/b23r0/select-rs.svg?branch=main)](https://app.travis-ci.com/b23r0/select-rs) [![ChatOnDiscord](https://img.shields.io/badge/chat-on%20discord-blue)](https://discord.gg/ZKtYMvDFN4) [![Crate](https://img.shields.io/crates/v/select-rs)](https://crates.io/crates/select-rs)

A POSIX select I/O Multiplexing Rust library.

[select-rs]: https://github.com/b23r0/select-rs

# Get started

```toml
# Cargo.toml
[dependencies]
select-rs = "0.1.0"
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

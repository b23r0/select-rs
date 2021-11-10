#![allow(non_snake_case)]

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "linux")]
use std::os::unix::prelude::{AsRawFd};
use std::ptr;
#[cfg(target_os = "linux")]
use errno::{Errno, errno};
#[cfg(target_os = "linux")]
use libc::*;

#[cfg(target_os = "windows")]
pub type FdSet = winapi::um::winsock2::fd_set;

#[cfg(target_os = "windows")]
pub unsafe fn FD_ISSET(a : u64 , set : &mut FdSet) -> bool{
	let mut i = 0 ;
	while i < set.fd_count {
		
		if set.fd_array[i as usize] == a as usize {
			return true;
		}

		i += 1;
	}
	return false
}

#[cfg(target_os = "windows")]
pub unsafe fn FD_SET(a : u64 , set : &mut FdSet){
    use winapi::um::winsock2::FD_SETSIZE;

	let mut i  = 0 ;
	while i < set.fd_count {
		
		if set.fd_array[i as usize] == a as usize {
			return
		}
		i += 1;
	}
	if (i as usize) < FD_SETSIZE {
		set.fd_array[i as usize] = a as usize;
		set.fd_count += 1;
	}
}

#[cfg(target_os = "windows")]
pub unsafe fn FD_ZERO(set : &mut FdSet){
	let mut i = 0 ;
	while i < set.fd_count {
		set.fd_array[i as usize] =0;
		i += 1;
	}
	set.fd_count = 0;
}

pub fn select(readfds: *mut FdSet, writefds: *mut FdSet,exceptfds: *mut FdSet) -> u32 {

    #[cfg(target_os = "windows")]
    return unsafe { winapi::um::winsock2::select(
        0 , 
        readfds , 
        writefds, 
        exceptfds , 
        ptr::null_mut()) as u32 };
}
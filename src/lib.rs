#![allow(non_snake_case)]

#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(not(target_os = "windows"))]
extern crate libc;

#[cfg(target_os = "windows")]
pub type FdSet = winapi::um::winsock2::fd_set;
#[cfg(target_os = "windows")]
pub type TimeVal = winapi::um::winsock2::timeval;
#[cfg(not(target_os = "windows"))]
pub type FdSet = libc::fd_set;
#[cfg(not(target_os = "windows"))]
pub type TimeVal = libc::timeval;

#[cfg(not(target_os = "windows"))]
pub fn FD_ISSET(fd: libc::c_int, set: *const libc::fd_set) -> bool {
	return unsafe { libc::FD_ISSET(fd, set) };
}

#[cfg(not(target_os = "windows"))]
pub fn FD_SET(fd: libc::c_int, set: *mut libc::fd_set) -> () {
	return unsafe { libc::FD_SET(fd, set) };
}

#[cfg(not(target_os = "windows"))]
pub fn FD_ZERO(set: *mut libc::fd_set) -> () {
	return unsafe { libc::FD_ZERO(set) };
}

#[cfg(target_os = "windows")]
pub fn FD_ISSET(a : u64 , set : &mut FdSet) -> bool{
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
pub fn FD_SET(a : u64 , set : &mut FdSet){
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
pub fn FD_ZERO(set : &mut FdSet){
	let mut i = 0 ;
	while i < set.fd_count {
		set.fd_array[i as usize] =0;
		i += 1;
	}
	set.fd_count = 0;
}

pub fn select( maxfd : i32, readfds: *mut FdSet, writefds: *mut FdSet,exceptfds: *mut FdSet , timeout: *mut TimeVal) -> i32 {

    #[cfg(target_os = "windows")]
    return unsafe { winapi::um::winsock2::select(
        maxfd , 
        readfds , 
        writefds, 
        exceptfds , 
        timeout) as i32 };

	#[cfg(not(target_os = "windows"))]
	return unsafe { libc::select(
		maxfd , 
		readfds , 
		writefds, 
		exceptfds , 
		timeout) as i32 };
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_select(){
	let mut fds : FdSet = unsafe {std::mem::zeroed()};
	FD_ZERO(&mut fds);
	FD_SET(0 , &mut fds);
	assert!(select(1, std::ptr::null_mut() , &mut fds ,std::ptr::null_mut() , std::ptr::null_mut()) > 0);
	assert!(FD_ISSET(0, &mut fds));
}
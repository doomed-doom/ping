use std::ffi::c_int;

pub const ICMP_ECHO_REQUEST_TYPE: u8 = 8;
pub const ICMP_ECHO_ANSWER_TYPE: u8 = 0;

pub const LIBC_EINPROGRESS: c_int = 155;

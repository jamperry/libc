pub type fsblkcnt_t = ::c_uint;
pub type fsfilcnt_t = ::c_uint;

pub const PTHREAD_STACK_MIN: ::size_t = 1024;
pub const KERN_PROC_PATHNAME: ::c_int = 9;
pub const SIGSTKSZ: ::size_t = 40960;
pub const MADV_INVAL: ::c_int = 10;

pub const HW_AVAILCPU: ::c_int = 25;

extern {
    pub fn __dfly_error() -> *const ::c_int;
}

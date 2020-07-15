/* automatically generated by rust-bindgen 0.54.1 */

pub type uint = cty::c_uint;
pub type ushort = cty::c_ushort;
pub type uchar = cty::c_uchar;
pub type pde_t = uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rtcdate {
    _unused: [u8; 0],
}
extern "C" {
    pub fn fork() -> cty::c_int;
}
extern "C" {
    pub fn exit() -> cty::c_int;
}
extern "C" {
    pub fn wait() -> cty::c_int;
}
extern "C" {
    pub fn pipe(arg1: *mut cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn write(arg1: cty::c_int, arg2: *const cty::c_void, arg3: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn read(arg1: cty::c_int, arg2: *mut cty::c_void, arg3: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn close(arg1: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn kill(arg1: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn exec(arg1: *mut cty::c_char, arg2: *mut *mut cty::c_char) -> cty::c_int;
}
extern "C" {
    pub fn open(arg1: *const cty::c_char, arg2: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn mknod(arg1: *const cty::c_char, arg2: cty::c_short, arg3: cty::c_short) -> cty::c_int;
}
extern "C" {
    pub fn unlink(arg1: *const cty::c_char) -> cty::c_int;
}
extern "C" {
    pub fn fstat(fd: cty::c_int, arg1: *mut stat) -> cty::c_int;
}
extern "C" {
    pub fn link(arg1: *const cty::c_char, arg2: *const cty::c_char) -> cty::c_int;
}
extern "C" {
    pub fn mkdir(arg1: *const cty::c_char) -> cty::c_int;
}
extern "C" {
    pub fn chdir(arg1: *const cty::c_char) -> cty::c_int;
}
extern "C" {
    pub fn dup(arg1: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn getpid() -> cty::c_int;
}
extern "C" {
    pub fn sbrk(arg1: cty::c_int) -> *mut cty::c_char;
}
extern "C" {
    pub fn sleep(arg1: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn uptime() -> cty::c_int;
}
extern "C" {
    pub fn getreadcount() -> cty::c_int;
}
extern "C" {
    pub fn stat(arg1: *const cty::c_char, arg2: *mut stat) -> cty::c_int;
}
extern "C" {
    pub fn strcpy(arg1: *mut cty::c_char, arg2: *const cty::c_char) -> *mut cty::c_char;
}
extern "C" {
    pub fn memmove(
        arg1: *mut cty::c_void,
        arg2: *const cty::c_void,
        arg3: cty::c_int,
    ) -> *mut cty::c_void;
}
extern "C" {
    pub fn strchr(arg1: *const cty::c_char, c: cty::c_char) -> *mut cty::c_char;
}
extern "C" {
    pub fn strcmp(arg1: *const cty::c_char, arg2: *const cty::c_char) -> cty::c_int;
}
extern "C" {
    pub fn printf(arg1: cty::c_int, arg2: *const cty::c_char, ...);
}
extern "C" {
    pub fn gets(arg1: *mut cty::c_char, max: cty::c_int) -> *mut cty::c_char;
}
extern "C" {
    pub fn strlen(arg1: *const cty::c_char) -> uint;
}
extern "C" {
    pub fn memset(arg1: *mut cty::c_void, arg2: cty::c_int, arg3: uint) -> *mut cty::c_void;
}
extern "C" {
    pub fn malloc(arg1: uint) -> *mut cty::c_void;
}
extern "C" {
    pub fn free(arg1: *mut cty::c_void);
}
extern "C" {
    pub fn atoi(arg1: *const cty::c_char) -> cty::c_int;
}

use libc::c_void;

extern "C" { pub fn cv_delete_Mat(ptr : *mut c_void); }

#[repr(C)]
pub struct Mat { ptr: *mut c_void }
impl Drop for Mat {
    fn drop(&mut self) {
        unsafe { cv_delete_Mat(self.ptr); }
    }
}

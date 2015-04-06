extern "C" { pub fn cv_delete_Mat(ptr : *mut i8); }

#[repr(C)]
pub struct Mat { ptr: *mut i8 }
impl Drop for Mat {
    fn drop(&mut self) {
        unsafe { cv_delete_Mat(self.ptr); }
    }
}

#[doc(hidden)]
#[repr(C)]
/// needed because layout of () in repr(C) is not guaranteed
pub struct Unit([u8; 0]);

impl From<Unit> for () {
    fn from(_: Unit) -> Self {
        ()
    }
}

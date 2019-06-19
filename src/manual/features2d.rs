use crate::{
    features2d::{self, ORB},
    Result,
    types::PtrOfORB,
};

impl dyn ORB {
    pub fn default() -> Result<PtrOfORB> {
        ORB::create(500, 1.2, 8, 31, 0, 2, features2d::ORB_HARRIS_SCORE, 31, 20)
    }
}

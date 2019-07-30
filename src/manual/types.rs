use crate::{core, Result, types};

#[doc(hidden)]
#[repr(C)]
/// needed because layout of () in repr(C) is not guaranteed
pub struct Unit([u8; 0]);

impl From<Unit> for () {
    fn from(_: Unit) -> Self {
        ()
    }
}

impl core::ToInputArray for types::VectorOfMat {
    fn input_array(&self) -> Result<core::InputArray> {
        core::InputArray::from_mat_vec(self)
    }
}

impl core::ToOutputArray for types::VectorOfMat {
    fn output_array(&mut self) -> Result<core::OutputArray> {
        core::OutputArray::from_mat_vec(self)
    }
}

impl core::ToInputOutputArray for types::VectorOfMat {
    fn input_output_array(&mut self) -> Result<core::InputOutputArray> {
        core::InputOutputArray::from_mat_vec(self)
    }
}

impl core::ToInputArray for types::VectorOfUMat {
    fn input_array(&self) -> Result<core::InputArray> {
        core::InputArray::from_umat_vec(self)
    }
}

impl core::ToOutputArray for types::VectorOfUMat {
    fn output_array(&mut self) -> Result<core::OutputArray> {
        core::OutputArray::from_umat_vec(self)
    }
}

impl core::ToInputOutputArray for types::VectorOfUMat {
    fn input_output_array(&mut self) -> Result<core::InputOutputArray> {
        core::InputOutputArray::from_umat_vec(self)
    }
}

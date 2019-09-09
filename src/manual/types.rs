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
    #[inline]
    fn input_array(&self) -> Result<core::InputArray> {
        core::InputArray::from_mat_vec(self)
    }
}

impl core::ToInputArray for &types::VectorOfMat {
    #[inline]
    fn input_array(&self) -> Result<core::InputArray> {
        (*self).input_array()
    }
}

impl core::ToOutputArray for types::VectorOfMat {
    #[inline]
    fn output_array(&mut self) -> Result<core::OutputArray> {
        core::OutputArray::from_mat_vec(self)
    }
}

impl core::ToOutputArray for &mut types::VectorOfMat {
    #[inline]
    fn output_array(&mut self) -> Result<core::OutputArray> {
        (*self).output_array()
    }
}

impl core::ToInputOutputArray for types::VectorOfMat {
    #[inline]
    fn input_output_array(&mut self) -> Result<core::InputOutputArray> {
        core::InputOutputArray::from_mat_vec(self)
    }
}

impl core::ToInputOutputArray for &mut types::VectorOfMat {
    #[inline]
    fn input_output_array(&mut self) -> Result<core::InputOutputArray> {
        (*self).input_output_array()
    }
}

impl core::ToInputArray for types::VectorOfUMat {
    #[inline]
    fn input_array(&self) -> Result<core::InputArray> {
        core::InputArray::from_umat_vec(self)
    }
}

impl core::ToInputArray for &types::VectorOfUMat {
    #[inline]
    fn input_array(&self) -> Result<core::InputArray> {
        (*self).input_array()
    }
}

impl core::ToOutputArray for types::VectorOfUMat {
    #[inline]
    fn output_array(&mut self) -> Result<core::OutputArray> {
        core::OutputArray::from_umat_vec(self)
    }
}

impl core::ToOutputArray for &mut types::VectorOfUMat {
    #[inline]
    fn output_array(&mut self) -> Result<core::OutputArray> {
        (*self).output_array()
    }
}

impl core::ToInputOutputArray for types::VectorOfUMat {
    #[inline]
    fn input_output_array(&mut self) -> Result<core::InputOutputArray> {
        core::InputOutputArray::from_umat_vec(self)
    }
}

impl core::ToInputOutputArray for &mut types::VectorOfUMat {
    #[inline]
    fn input_output_array(&mut self) -> Result<core::InputOutputArray> {
        (*self).input_output_array()
    }
}

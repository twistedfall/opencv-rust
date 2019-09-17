use crate::{
    core::{_InputArray, _InputOutputArray, _OutputArray, ToInputArray, ToInputOutputArray, ToOutputArray},
    Result,
    types,
};

#[doc(hidden)]
#[repr(C)]
/// needed because layout of () in repr(C) is not guaranteed
pub struct Unit([u8; 0]);

impl From<Unit> for () {
    fn from(_: Unit) -> Self {
        ()
    }
}

impl ToInputArray for types::VectorOfMat {
    #[inline]
    fn input_array(&self) -> Result<_InputArray> {
        _InputArray::from_mat_vec(self)
    }
}

impl ToInputArray for &types::VectorOfMat {
    #[inline]
    fn input_array(&self) -> Result<_InputArray> {
        (*self).input_array()
    }
}

impl ToOutputArray for types::VectorOfMat {
    #[inline]
    fn output_array(&mut self) -> Result<_OutputArray> {
        _OutputArray::from_mat_vec(self)
    }
}

impl ToOutputArray for &mut types::VectorOfMat {
    #[inline]
    fn output_array(&mut self) -> Result<_OutputArray> {
        (*self).output_array()
    }
}

impl ToInputOutputArray for types::VectorOfMat {
    #[inline]
    fn input_output_array(&mut self) -> Result<_InputOutputArray> {
        _InputOutputArray::from_mat_vec(self)
    }
}

impl ToInputOutputArray for &mut types::VectorOfMat {
    #[inline]
    fn input_output_array(&mut self) -> Result<_InputOutputArray> {
        (*self).input_output_array()
    }
}

impl ToInputArray for types::VectorOfUMat {
    #[inline]
    fn input_array(&self) -> Result<_InputArray> {
        _InputArray::from_umat_vec(self)
    }
}

impl ToInputArray for &types::VectorOfUMat {
    #[inline]
    fn input_array(&self) -> Result<_InputArray> {
        (*self).input_array()
    }
}

impl ToOutputArray for types::VectorOfUMat {
    #[inline]
    fn output_array(&mut self) -> Result<_OutputArray> {
        _OutputArray::from_umat_vec(self)
    }
}

impl ToOutputArray for &mut types::VectorOfUMat {
    #[inline]
    fn output_array(&mut self) -> Result<_OutputArray> {
        (*self).output_array()
    }
}

impl ToInputOutputArray for types::VectorOfUMat {
    #[inline]
    fn input_output_array(&mut self) -> Result<_InputOutputArray> {
        _InputOutputArray::from_umat_vec(self)
    }
}

impl ToInputOutputArray for &mut types::VectorOfUMat {
    #[inline]
    fn input_output_array(&mut self) -> Result<_InputOutputArray> {
        (*self).input_output_array()
    }
}

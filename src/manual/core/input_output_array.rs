use std::ffi::c_void;

use crate::{
    core::{_InputArray, _InputOutputArray, _OutputArray, Mat, MatExpr, UMat},
    Result,
    sys,
    types::{VectorOfMat, VectorOfUMat},
};

pub struct InputArray {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl InputArray {
    #[inline]
    pub fn from_mat(m: &Mat) -> Result<Self> {
        let m = m.as_raw_Mat();
        cpp!(unsafe [m as "const Mat*"] -> sys::cv_return_value_const_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new _InputArray(*m) };
            } CVRS_CATCH(cv_return_value_void_X)
        }).into_result()
            .map(|ptr| Self { ptr })
    }

    #[inline]
    pub fn from_mat_expr(m: &MatExpr) -> Result<Self> {
        let m = m.as_raw_MatExpr();
        cpp!(unsafe [m as "const MatExpr*"] -> sys::cv_return_value_const_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new _InputArray(*m) };
            } CVRS_CATCH(cv_return_value_void_X)
        }).into_result()
            .map(|ptr| Self { ptr })
    }

    #[inline]
    pub fn from_mat_vec(m: &VectorOfMat) -> Result<Self> {
        let m = m.as_raw_VectorOfMat();
        cpp!(unsafe [m as "const std::vector<Mat>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new _InputArray(*m) };
            } CVRS_CATCH(cv_return_value_void_X)
        }).into_result()
            .map(|ptr| Self { ptr })
    }

    #[inline]
    pub fn from_f64(val: &f64) -> Result<Self> {
        cpp!(unsafe [val as "const double*"] -> sys::cv_return_value_const_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new _InputArray(*val) };
            } CVRS_CATCH(cv_return_value_void_X)
        }).into_result()
            .map(|ptr| Self { ptr })
    }

    #[inline]
    pub fn from_umat(m: &UMat) -> Result<Self> {
        let m = m.as_raw_UMat();
        cpp!(unsafe [m as "const UMat*"] -> sys::cv_return_value_const_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new _InputArray(*m) };
            } CVRS_CATCH(cv_return_value_void_X)
        }).into_result()
            .map(|ptr| Self { ptr })
    }

    #[inline]
    pub fn from_umat_vec(m: &VectorOfUMat) -> Result<Self> {
        let m = m.as_raw_VectorOfUMat();
        cpp!(unsafe [m as "const std::vector<UMat>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new _InputArray(*m) };
            } CVRS_CATCH(cv_return_value_void_X)
        }).into_result()
            .map(|ptr| Self { ptr })
    }
}

impl _InputArray for InputArray {
    fn as_raw__InputArray(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for InputArray {
    fn drop(&mut self) {
        let me = self.as_raw__InputArray();
        cpp!(unsafe [me as "cv::_InputArray*"] {
            delete me;
        })
    }
}

pub trait ToInputArray {
    fn input_array(&self) -> Result<InputArray>;
}

impl ToInputArray for f64 {
    #[inline]
    fn input_array(&self) -> Result<InputArray> {
        InputArray::from_f64(self)
    }
}

pub struct OutputArray {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl OutputArray {
    #[inline]
    pub fn from_mat(m: &mut Mat) -> Result<Self> {
        let m = m.as_raw_Mat();
        cpp!(unsafe [m as "Mat*"] -> sys::cv_return_value_const_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new _OutputArray(*m) };
            } CVRS_CATCH(cv_return_value_void_X)
        }).into_result()
            .map(|ptr| Self { ptr })
    }

    #[inline]
    pub fn from_mat_vec(m: &mut VectorOfMat) -> Result<Self> {
        let m = m.as_raw_VectorOfMat();
        cpp!(unsafe [m as "std::vector<Mat>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new _OutputArray(*m) };
            } CVRS_CATCH(cv_return_value_void_X)
        }).into_result()
            .map(|ptr| Self { ptr })
    }

    #[inline]
    pub fn from_umat(m: &mut UMat) -> Result<Self> {
        let m = m.as_raw_UMat();
        cpp!(unsafe [m as "UMat*"] -> sys::cv_return_value_const_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new _OutputArray(*m) };
            } CVRS_CATCH(cv_return_value_void_X)
        }).into_result()
            .map(|ptr| Self { ptr })
    }

    #[inline]
    pub fn from_umat_vec(m: &mut VectorOfUMat) -> Result<Self> {
        let m = m.as_raw_VectorOfUMat();
        cpp!(unsafe [m as "std::vector<UMat>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new _OutputArray(*m) };
            } CVRS_CATCH(cv_return_value_void_X)
        }).into_result()
            .map(|ptr| Self { ptr })
    }
}

impl _InputArray for OutputArray {
    fn as_raw__InputArray(&self) -> *mut c_void {
        self.ptr
    }
}

impl _OutputArray for OutputArray {
    fn as_raw__OutputArray(&self) -> *mut c_void {
        self.as_raw__InputArray()
    }
}

impl Drop for OutputArray {
    fn drop(&mut self) {
        let me = self.as_raw__OutputArray();
        cpp!(unsafe [me as "cv::_OutputArray*"] {
            delete me;
        })
    }
}

pub trait ToOutputArray {
    fn output_array(&mut self) -> Result<OutputArray>;
}

pub type InputOutputArray = _InputOutputArray;

pub trait ToInputOutputArray {
    fn input_output_array(&mut self) -> Result<InputOutputArray>;
}

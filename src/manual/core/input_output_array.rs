use std::ffi::c_void;

use crate::{
    core::{_InputArrayTrait, _InputOutputArray, _OutputArrayTrait, Mat, MatExpr, UMat},
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

impl _InputArrayTrait for InputArray {
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

/// Trait to serve as a replacement for `InputArray` in C++ OpenCV
///
/// You can pass references to the types implementing this trait everywhere where OpenCV API expects
/// `InputArray` or `InputArrayOfArrays`.
///
/// More info in [OpenCV docs](https://docs.opencv.org/master/d4/d32/classcv_1_1__InputArray.html#details).
pub trait ToInputArray {
    fn input_array(&self) -> Result<InputArray>;
}

impl ToInputArray for f64 {
    #[inline]
    fn input_array(&self) -> Result<InputArray> {
        InputArray::from_f64(self)
    }
}

impl ToInputArray for &f64 {
    #[inline]
    fn input_array(&self) -> Result<InputArray> {
        (*self).input_array()
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

impl _InputArrayTrait for OutputArray {
    fn as_raw__InputArray(&self) -> *mut c_void {
        self.ptr
    }
}

impl _OutputArrayTrait for OutputArray {
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

/// Trait to serve as a replacement for `OutputArray` in C++ OpenCV
///
/// You can pass reference to the type implementing this trait everywhere where OpenCV API expects
/// `OutputArray` or `OutputArrayOfArrays`.
///
/// More info in [OpenCV docs](https://docs.opencv.org/master/d2/d9e/classcv_1_1__OutputArray.html#details).
pub trait ToOutputArray {
    fn output_array(&mut self) -> Result<OutputArray>;
}

pub type InputOutputArray = _InputOutputArray;

/// Trait to serve as a replacement for `InputOutputArray` in C++ OpenCV
///
/// You can pass reference to the type implementing this trait everywhere where OpenCV API expects
/// `InputOutputArray` or `InputOutputArrayOfArrays`.
///
/// For more info see comments for `ToInputArray` and `ToOutputArray`.
pub trait ToInputOutputArray {
    fn input_output_array(&mut self) -> Result<InputOutputArray>;
}

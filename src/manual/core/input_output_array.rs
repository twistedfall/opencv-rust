use crate::{
    core::{_InputArray, _InputOutputArray, _InputOutputArrayTrait, _OutputArray},
    Result,
    sys,
};

/// Trait to serve as a replacement for `InputArray` in C++ OpenCV
///
/// You can pass references to the types implementing this trait everywhere where OpenCV API expects
/// `InputArray` or `InputArrayOfArrays`.
///
/// More info in [OpenCV docs](https://docs.opencv.org/master/d4/d32/classcv_1_1__InputArray.html#details).
pub trait ToInputArray {
    fn input_array(&self) -> Result<_InputArray>;
}

impl ToInputArray for f64 {
    #[inline]
    fn input_array(&self) -> Result<_InputArray> {
        _InputArray::from_f64(self)
    }
}

impl ToInputArray for &f64 {
    #[inline]
    fn input_array(&self) -> Result<_InputArray> {
        (*self).input_array()
    }
}

/// Trait to serve as a replacement for `OutputArray` in C++ OpenCV
///
/// You can pass reference to the type implementing this trait everywhere where OpenCV API expects
/// `OutputArray` or `OutputArrayOfArrays`.
///
/// More info in [OpenCV docs](https://docs.opencv.org/master/d2/d9e/classcv_1_1__OutputArray.html#details).
pub trait ToOutputArray {
    fn output_array(&mut self) -> Result<_OutputArray>;
}

/// Trait to serve as a replacement for `InputOutputArray` in C++ OpenCV
///
/// You can pass reference to the type implementing this trait everywhere where OpenCV API expects
/// `InputOutputArray` or `InputOutputArrayOfArrays`.
///
/// For more info see comments for `ToInputArray` and `ToOutputArray`.
pub trait ToInputOutputArray {
    fn input_output_array(&mut self) -> Result<_InputOutputArray>;
}

impl<T: _InputOutputArrayTrait> ToInputArray for T {
    fn input_array(&self) -> Result<_InputArray> {
        let me = self.as_raw__InputOutputArray();
        cpp!(unsafe [me as "cv::_InputOutputArray*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
           .map(|ptr| _InputArray { ptr })
    }
}

impl ToOutputArray for _InputOutputArray {
    fn output_array(&mut self) -> Result<_OutputArray> {
        let me = self.as_raw__InputOutputArray();
        cpp!(unsafe [me as "cv::_InputOutputArray*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
           .map(|ptr| _OutputArray { ptr })
    }
}

impl ToInputOutputArray for _InputOutputArray {
    fn input_output_array(&mut self) -> Result<_InputOutputArray> {
        let me = self.as_raw__InputOutputArray();
        cpp!(unsafe [me as "cv::_InputOutputArray*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
           .map(|ptr| _InputOutputArray { ptr })
    }
}

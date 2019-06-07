use std::{
    ffi::c_void,
    fmt,
    ops::Deref,
    slice,
};

use libc::size_t;

use crate::{
    Error,
    hub::core::{self, Mat, MatSize, MatStep},
    Result,
    sys,
};

/// This sealed trait is implemented for types that are valid to use as Mat elements
pub trait DataType: Copy + private::Sealed {
    fn depth() -> i32;
    fn channels() -> i32;
    fn typ() -> i32;
}

macro_rules! data_type {
    ($rust_type: ty, $mat_depth: expr, $channels: expr) => {
        impl $crate::core::DataType for $rust_type {
            #[inline(always)]
            fn depth() -> i32 { $mat_depth }

            #[inline(always)]
            fn channels() -> i32 { $channels }

            #[inline(always)]
            fn typ() -> i32 { $crate::hub::core::MAKETYPE($mat_depth, $channels) }
        }

        impl private::Sealed for $rust_type {}
    };
}

// int
data_type!(u8, core::CV_8U, 1);
data_type!(i8, core::CV_8S, 1);
data_type!(u16, core::CV_16U, 1);
data_type!(i16, core::CV_16S, 1);
data_type!(i32, core::CV_32S, 1);

// float
data_type!(f32, core::CV_32F, 1);
data_type!(f64, core::CV_64F, 1);

// vec int
data_type!(core::Vec2b, core::CV_8U, 2);
data_type!(core::Vec3b, core::CV_8U, 3);
data_type!(core::Vec4b, core::CV_8U, 4);
data_type!(core::Vec2<i8>, core::CV_8S, 2);
data_type!(core::Vec3<i8>, core::CV_8S, 3);
data_type!(core::Vec4<i8>, core::CV_8S, 4);
data_type!(core::Vec2<u16>, core::CV_16U, 2);
data_type!(core::Vec3<u16>, core::CV_16U, 3);
data_type!(core::Vec4<u16>, core::CV_16U, 4);
data_type!(core::Vec2s, core::CV_16S, 2);
data_type!(core::Vec3s, core::CV_16S, 3);
data_type!(core::Vec4s, core::CV_16S, 4);
data_type!(core::Vec2i, core::CV_32S, 2);
data_type!(core::Vec3i, core::CV_32S, 3);
data_type!(core::Vec4i, core::CV_32S, 4);

// vec float
data_type!(core::Vec2f, core::CV_32F, 2);
data_type!(core::Vec3f, core::CV_32F, 3);
data_type!(core::Vec4f, core::CV_32F, 4);
data_type!(core::Vec2d, core::CV_64F, 2);
data_type!(core::Vec3d, core::CV_64F, 3);
data_type!(core::Vec4d, core::CV_64F, 4);

// scalar
data_type!(core::Scalar, core::CV_64F, 4);

impl Mat {
    #[inline(always)]
    fn match_format<T: DataType>(&self) -> Result<()> {
        let mat_type = self.typ()?;
        let out_type = T::typ();
        if mat_type == out_type {
            Ok(())
        } else {
            Err(Error::new(core::StsUnmatchedFormats, format!("Mat type is: {}, but requested type is: {}", core::type_to_string(mat_type)?, core::type_to_string(out_type)?)))
        }
    }

    #[inline(always)]
    pub(crate) fn _at<T: DataType>(&self, row: i32) -> Result<&T> {
        self.match_format::<T>()?;
        let size = self.size()?;
        if row >= size.height {
            return Err(Error::new(
                core::StsOutOfRange,
                format!("Given coordinate is out of bounds. \
                         Mat has size {:?} but was given row: {}", size, row)));
        }
        unsafe { self.at_unchecked(row) }
    }

    /// Like `Mat::at()` but performs no bounds or type checks
    ///
    /// Note that this (and `Mat::at()`) differs from the behavior of OpenCV's
    /// `_Tp& cv::Mat::at(int i0 = 0)`, in that it only looks up by row, and
    /// does not look up by column in the case of single-row Mats, as OpenCV's
    /// `Mat::at()` does.
    /// (This upstream behavior is documented at https://docs.opencv.org/3.4/d3/d63/classcv_1_1Mat.html#aa5d20fc86d41d59e4d71ae93daee9726)
    pub unsafe fn at_unchecked<T: DataType>(&self, row: i32) -> Result<&T> {
        self.ptr(row).map(|x| &*(x as *const _ as *const T))
    }

    #[inline(always)]
    pub(crate) fn _at_mut<T: DataType>(&mut self, i0: i32) -> Result<&mut T> {
        self.match_format::<T>()?;
        let size = self.size()?;
        if i0 >= size.height {
            return Err(Error::new(
                core::StsOutOfRange,
                format!("Given coordinate is out of bounds. \
                         Mat has size {:?} but was given i0: {}", size, i0)));
        }
        unsafe { self.at_mut_unchecked(i0) }
    }

    /// Like `Mat::at_mut()` but performs no bounds or type checks
    pub unsafe fn at_mut_unchecked<T: DataType>(&mut self, i0: i32) -> Result<&mut T> {
        self.ptr_mut(i0).map(|x| &mut *(x as *mut _ as *mut T))
    }

    #[inline(always)]
    pub(crate) fn _at_2d<T: DataType>(&self, row: i32, col: i32) -> Result<&T> {
        self.match_format::<T>()?;
        let size = self.size()?;
        if row >= size.height || col >= size.width {
            return Err(Error::new(
                core::StsOutOfRange,
                format!("Given coordinate is out of bounds. \
                         Mat has size {:?} but was given row: {}, col: {}", size, row, col)));
        }
        unsafe { self.at_2d_unchecked(row, col) }
    }

    /// Like `Mat::at_2d()` but performs no bounds or type checks
    pub unsafe fn at_2d_unchecked<T: DataType>(&self, row: i32, col: i32) -> Result<&T> {
        self.ptr_2d(row, col).map(|x| &*(x as *const _ as *const T))
    }

    #[inline(always)]
    pub(crate) fn _at_2d_mut<T: DataType>(&mut self, row: i32, col: i32) -> Result<&mut T> {
        self.match_format::<T>()?;
        let size = self.size()?;
        if row >= size.height || col >= size.width {
            return Err(Error::new(
                core::StsOutOfRange,
                format!("Given coordinate is out of bounds. \
                         Mat has size {:?} but was given row: {}, col: {}", size, row, col)));
        }
        unsafe { self.at_2d_mut_unchecked(row, col) }
    }

    /// Like `Mat::at_2d_mut()` but performs no bounds or type checks
    pub unsafe fn at_2d_mut_unchecked<T: DataType>(&mut self, row: i32, col: i32) -> Result<&mut T> {
        self.ptr_2d_mut(row, col).map(|x| &mut *(x as *mut _ as *mut T))
    }

    /// Note that since the bindings are set up to wrap all Mat sizes in our custom `Size` struct
    /// instead of a `MatSize` smart pointer, we don't have a good way to check bounds on Mats with
    /// more than 2 dimensions, so we can't perform a real bounds check here.
    #[inline(always)]
    pub(crate) fn _at_3d<T: DataType>(&self, i0: i32, i1: i32, i2: i32) -> Result<&T> {
        self.match_format::<T>().and_then(|_| unsafe { self.ptr_3d(i0, i1, i2) }.map(|x| unsafe { &*(x as *const _ as *const T) }))
    }

    /// See safety caveats of `Mat::_at_3d()`
    #[inline(always)]
    pub(crate) fn _at_3d_mut<T: DataType>(&mut self, i0: i32, i1: i32, i2: i32) -> Result<&mut T> {
        self.match_format::<T>().and_then(|_| unsafe { self.ptr_3d_mut(i0, i1, i2) }.map(|x| unsafe { &mut *(x as *mut _ as *mut T) }))
    }

    /// Return a complete read-only row
    pub fn at_row<T: DataType>(&self, row: i32) -> Result<&[T]> {
        self.match_format::<T>()?;
        let size = self.size()?;
        if row >= size.height {
            return Err(Error::new(
                core::StsOutOfRange,
                format!("Given coordinate is out of bounds. \
                         Mat has size {:?} but was given row: {}", size, row)));
        }
        unsafe { self.at_row_unchecked(row) }
    }

    /// Like `Mat::at_row()` but performs no bounds or type checks
    pub unsafe fn at_row_unchecked<T: DataType>(&self, row: i32) -> Result<&[T]> {
        let width = self.size()?.width as usize;
        self.at_unchecked(row).map(|x| slice::from_raw_parts(x, width))
    }

    /// Return a complete writeable row
    pub fn at_row_mut<T: DataType>(&mut self, row: i32) -> Result<&mut [T]> {
        self.match_format::<T>()?;
        let size = self.size()?;
        if row >= size.height {
            return Err(Error::new(
                core::StsOutOfRange,
                format!("Given coordinate is out of bounds. \
                         Mat has size {:?} but was given row: {}", size, row)));
        }
        unsafe { self.at_row_mut_unchecked(row) }
    }

    /// Like `Mat::at_row_mut()` but performs no bounds or type checks
    pub unsafe fn at_row_mut_unchecked<T: DataType>(&mut self, row: i32) -> Result<&mut [T]> {
        let width = self.size()?.width as usize;
        self.at_mut_unchecked(row).map(|x| slice::from_raw_parts_mut(x, width))
    }

    pub fn size(&self) -> Result<core::Size> {
        let me = self.as_raw_Mat();
        cpp!(unsafe [me as "const cv::Mat*"] -> sys::cv_return_value_SizeWrapper as "cv_return_value_SizeWrapper" {
            try {
                cv::Size ret = me->size();
                return { Error::Code::StsOk, NULL, *reinterpret_cast<SizeWrapper*>(&ret) };
            } CVRS_CATCH(cv_return_value_SizeWrapper)
        }).into_result()
    }

    pub fn data_typed<T: DataType>(&self) -> Result<&[T]> {
        let total = self.total()?;
        self._at(0).map(|x| unsafe { slice::from_raw_parts(x, total) })
    }

    pub fn data_typed_mut<T: DataType>(&mut self) -> Result<&mut [T]> {
        let total = self.total()?;
        self.data().map(|x| unsafe { slice::from_raw_parts_mut(x as *mut _ as *mut _, total) })
    }

    #[inline]
    pub fn from_slice<T: DataType>(s: &[T]) -> Result<Mat> {
        Self::from_slice_2d(&[s])
    }

    pub fn from_slice_2d<T: DataType>(s: &[impl AsRef<[T]>]) -> Result<Mat> {
        let row_count: i32 = s.len() as _;
        let col_count: i32 = if row_count > 0 {
            s[0].as_ref().len() as _
        } else {
            0
        };
        let mut out = unsafe { Mat::new_rows_cols(row_count, col_count, T::typ()) }?;
        for (row_n, row) in s.into_iter().enumerate() {
            let trg = out.at_row_mut(row_n as _)?;
            let src = row.as_ref();
            if trg.len() != src.len() {
                return Err(Error::new(core::StsUnmatchedSizes, format!("Unexpected number of items: {} in a row index: {}, expected: {}", trg.len(), row_n, src.len())));
            }
            trg.copy_from_slice(src);
        }
        Ok(out)
    }

    pub fn to_vec_2d<T: DataType>(&self) -> Result<Vec<Vec<T>>> {
        self.match_format::<T>().and_then(|_| {
            let dims = self.dims()?;
            if dims > 2 {
                return Err(Error::new(core::StsUnmatchedSizes, format!("Cannot convert Mat with dims: {} to 2D Vec", dims)));
            }
            let size = self.size()?;
            let width = size.width as usize;
            if self.is_continuous()? {
                let data = self.data_typed()?;
                Ok((0..size.height)
                    .map(|row_n| {
                        let row_n = row_n as usize;
                        let mut row = Vec::with_capacity(width);
                        row.extend_from_slice(&data[row_n * width..(row_n + 1) * width]);
                        row
                    })
                    .collect()
                )
            } else {
                Ok((0..size.height)
                    .map(|row_n| {
                        self.at_row(row_n).map(|src_row| {
                            let mut row = Vec::with_capacity(width);
                            row.extend_from_slice(src_row);
                            row
                        })
                    })
                    .collect::<Result<_>>()?
                )
            }
        })
    }
}

impl fmt::Debug for Mat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Mat")
            .field("type", &self.typ().and_then(core::type_to_string).map_err(|_| fmt::Error)?)
            .field("flags", &self.flags().map_err(|_| fmt::Error)?)
            .field("channels", &self.channels().map_err(|_| fmt::Error)?)
            .field("depth", &self.depth().and_then(core::depth_to_string).map_err(|_| fmt::Error)?)
            .field("dims", &self.dims().map_err(|_| fmt::Error)?)
            .field("size", &self.size().map_err(|_| fmt::Error)?)
            .field("rows", &self.rows().map_err(|_| fmt::Error)?)
            .field("cols", &self.cols().map_err(|_| fmt::Error)?)
            .field("elem_size", &self.elem_size().map_err(|_| fmt::Error)?)
            .field("elem_size1", &self.elem_size1().map_err(|_| fmt::Error)?)
            .field("total", &self.total().map_err(|_| fmt::Error)?)
            .field("is_continuous", &self.is_continuous().map_err(|_| fmt::Error)?)
            .field("is_submatrix", &self.is_submatrix().map_err(|_| fmt::Error)?)
            .finish()
    }
}

impl Default for Mat {
    fn default() -> Self {
        Mat::new().unwrap()
    }
}

impl Deref for MatSize {
    type Target = [i32];

    fn deref(&self) -> &Self::Target {
        let me = self.as_raw_MatSize();
        let ptr = cpp!(unsafe [me as "const MatSize*"] -> &i32 as "const int*" {
            return me->p;
        });
        unsafe { slice::from_raw_parts(ptr, self.dims().expect("Cannot get dims") as usize) }
    }
}

impl fmt::Debug for MatSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:#?}", self.deref())
    }
}

impl Deref for MatStep {
    type Target = [size_t];

    fn deref(&self) -> &Self::Target {
        let me = self.as_raw_MatStep();
        let ptr = cpp!(unsafe [me as "const MatStep*"] -> &size_t as "const size_t*" {
            return me->p;
        });
        unsafe { slice::from_raw_parts(ptr, 2) }
    }
}

impl fmt::Debug for MatStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:#?}", self.deref())
    }
}


mod private {
    pub trait Sealed {}
}

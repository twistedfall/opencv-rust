use std::{fmt, slice};
use std::intrinsics::transmute;

use crate::{
    Error,
    hub::core::{self, Mat},
    Result,
    sys,
};

// todo, make sealed
pub trait ValidMatElement {
    fn typ() -> i32;
}

macro_rules! valid_mat_element {
    ($rust_type: ty, $mat_type: expr) => {
        impl ValidMatElement for $rust_type {
            fn typ() -> i32 { $mat_type }
        }
    };
}

// int
valid_mat_element!(u8, core::CV_8U);
valid_mat_element!(i8, core::CV_8S);
valid_mat_element!(u16, core::CV_16U);
valid_mat_element!(i16, core::CV_16S);
valid_mat_element!(i32, core::CV_32S);

// float
valid_mat_element!(f32, core::CV_32F);
valid_mat_element!(f64, core::CV_64F);

// vec int
valid_mat_element!(core::Vec2b, core::CV_8UC2);
valid_mat_element!(core::Vec3b, core::CV_8UC3);
valid_mat_element!(core::Vec4b, core::CV_8UC4);
valid_mat_element!(core::Vec2s, core::CV_16SC2);
valid_mat_element!(core::Vec3s, core::CV_16SC3);
valid_mat_element!(core::Vec4s, core::CV_16SC4);
valid_mat_element!(core::Vec2i, core::CV_32SC2);
valid_mat_element!(core::Vec3i, core::CV_32SC3);
valid_mat_element!(core::Vec4i, core::CV_32SC4);

// vec float
valid_mat_element!(core::Vec2f, core::CV_32FC2);
valid_mat_element!(core::Vec3f, core::CV_32FC3);
valid_mat_element!(core::Vec4f, core::CV_32FC4);
valid_mat_element!(core::Vec2d, core::CV_64FC2);
valid_mat_element!(core::Vec3d, core::CV_64FC3);
valid_mat_element!(core::Vec4d, core::CV_64FC4);

// scalar
valid_mat_element!(core::Scalar, core::CV_64FC4);


//struct Mat_<T> {
//    mat: Mat,
//    _d: PhantomData<T>,
//}

//impl<T: MatElement> Mat_<T> {
//    pub fn try_from_mat(mat: Mat) -> Result<Self> {
//        if mat.typ()? == T::typ() {
//            Ok(Self { mat, _d: PhantomData })
//        } else {
//            Err(Error { code: -1, message: "".into() }) // fixme
//        }
//    }
//}

impl Mat {
    #[inline(always)]
    fn match_format<T: ValidMatElement>(&self) -> Result<()> {
        let mat_type = self.typ()?;
        let out_type = T::typ();
        if mat_type == out_type {
            Ok(())
        } else {
            Err(Error::new(core::StsUnmatchedFormats, format!("Mat type is: {}, but requested type is: {}", mat_type, out_type)))
        }
    }

    #[inline(always)]
    pub(crate) fn _at<T: ValidMatElement>(&self, i0: i32) -> Result<&T> {
        self.match_format::<T>().and_then(|_| self.ptr(i0).map(|x| unsafe { &*(x as *const _ as *const T) }))
    }

    #[inline(always)]
    pub(crate) fn _at_mut<T: ValidMatElement>(&mut self, i0: i32) -> Result<&mut T> {
        self.match_format::<T>().and_then(|_| self.ptr_mut(i0).map(|x| unsafe { &mut *(x as *mut _ as *mut T) }))
    }

    #[inline(always)]
    pub(crate) fn _at_2d<T: ValidMatElement>(&self, row: i32, col: i32) -> Result<&T> {
        self.match_format::<T>().and_then(|_| self.ptr_2d(row, col).map(|x| unsafe { &*(x as *const _ as *const T) }))
    }

    #[inline(always)]
    pub(crate) fn _at_2d_mut<T: ValidMatElement>(&mut self, row: i32, col: i32) -> Result<&mut T> {
        self.match_format::<T>().and_then(|_| self.ptr_2d_mut(row, col).map(|x| unsafe { &mut *(x as *mut _ as *mut T) }))
    }

    #[inline(always)]
    pub(crate) fn _at_3d<T: ValidMatElement>(&self, i0: i32, i1: i32, i2: i32) -> Result<&T> {
        self.match_format::<T>().and_then(|_| self.ptr_3d(i0, i1, i2).map(|x| unsafe { &*(x as *const _ as *const T) }))
    }

    #[inline(always)]
    pub(crate) fn _at_3d_mut<T: ValidMatElement>(&mut self, i0: i32, i1: i32, i2: i32) -> Result<&mut T> {
        self.match_format::<T>().and_then(|_| self.ptr_3d_mut(i0, i1, i2).map(|x| unsafe { &mut *(x as *mut _ as *mut T) }))
    }

    pub fn at_row<T: ValidMatElement>(&self, i0: i32) -> Result<&[T]> {
        let width = self.size()?.width as usize;
        self._at(i0).map(|x| unsafe { slice::from_raw_parts(x, width) })
    }

    pub fn at_row_mut<T: ValidMatElement>(&mut self, i0: i32) -> Result<&mut [T]> {
        let width = self.size()?.width as usize;
        self._at_mut(i0).map(|x| unsafe { slice::from_raw_parts_mut(x, width) })
    }

    pub fn data<T: ValidMatElement>(&self) -> Result<&[T]> {
        let total = self.total()?;
        self._at(0).map(|x| unsafe { slice::from_raw_parts(x, total) })
    }

    pub fn data_mut<T: ValidMatElement>(&mut self) -> Result<&mut [T]> {
        let total = self.total()?;
        self._at_mut(0).map(|x| unsafe { slice::from_raw_parts_mut(x, total) })
    }

    #[inline]
    pub fn from_slice<T: ValidMatElement + Copy>(s: &[T]) -> Result<Mat> {
        Self::from_slice_2d(&[s])
    }

    pub fn from_slice_2d<T: ValidMatElement + Copy>(s: &[impl AsRef<[T]>]) -> Result<Mat> {
        let row_count: i32 = s.len() as _;
        let col_count: i32 = if row_count > 0 {
            s[0].as_ref().len() as _
        } else {
            0
        };
        let mut out = Mat::new_rows_cols(row_count, col_count, T::typ())?;
        for (row_n, row) in s.into_iter().enumerate() {
            let trg = out.at_row_mut(row_n as _)?;
            let src = row.as_ref();
            if trg.len() != src.len() {
                return Err(Error::new(core::StsBadArg, format!("Unexpected number of items: {} in a row index: {}, expected: {}", trg.len(), row_n, src.len())));
            }
            trg.copy_from_slice(src);
        }
        Ok(out)
    }

    pub fn to_vec_2d<T: ValidMatElement + Copy>(&self) -> Result<Vec<Vec<T>>> {
        self.match_format::<T>().and_then(|_| {
            let size = self.size()?;
            let width = size.width as usize;
            let data = self.data()?;
            Ok((0..size.height)
                .map(|row_n| {
                    let row_n = row_n as usize;
                    let mut row = Vec::with_capacity(width);
                    unsafe { row.set_len(width); }
                    row[0..width].copy_from_slice(&data[row_n * width..(row_n + 1) * width]);
                    row
                })
                .collect()
            )
        })
    }
}

impl fmt::Debug for Mat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Mat")
            .field("channels", &self.channels().unwrap())
            .field("depth", &self.depth().unwrap())
            .field("elem_size", &self.elem_size().unwrap())
            .field("elem_size1", &self.elem_size1().unwrap())
            .field("is_continuous", &self.is_continuous().unwrap())
            .field("is_submatrix", &self.is_submatrix().unwrap())
            .field("total", &self.total().unwrap())
            .field("type", &self.typ().unwrap())
            .field("size", &self.size().unwrap())
            .finish()
    }
}

impl Default for Mat {
    fn default() -> Self {
        Mat::new().unwrap()
    }
}


mod private {
    pub trait Sealed {}
}

//! # The module brings implementations of different image hashing algorithms.
//!
//! Provide algorithms to extract the hash of images and fast way to figure out most similar images in
//! huge data set.
//!
//! Namespace for all functions is cv::img_hash.
//!
//! ### Supported Algorithms
//!
//! - Average hash (also called Different hash)
//! - PHash (also called Perceptual hash)
//! - Marr Hildreth Hash
//! - Radial Variance Hash
//! - Block Mean Hash (modes 0 and 1)
//! - Color Moment Hash (this is the one and only hash algorithm resist to rotation attack(-90~90 degree))
//!
//! You can study more about image hashing from following paper and websites:
//!
//! - "Implementation and benchmarking of perceptual image hash functions" [zauner2010implementation](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_zauner2010implementation)
//! - "Looks Like It" [lookslikeit](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_lookslikeit)
//!
//! ### Code Example
//!
//! @include samples/hash_samples.cpp
//!
//! ### Performance under different attacks
//!
//! ![Performance chart](https://docs.opencv.org/4.2.0/img_hash/doc/attack_performance.JPG)
//!
//! ### Speed comparison with PHash library (100 images from ukbench)
//!
//! ![Hash Computation chart](https://docs.opencv.org/4.2.0/img_hash/doc/hash_computation_chart.JPG)
//! ![Hash comparison chart](https://docs.opencv.org/4.2.0/img_hash/doc/hash_comparison_chart.JPG)
//!
//! As you can see, hash computation speed of img_hash module outperform [PHash library](http://www.phash.org/) a lot.
//!
//! PS : I do not list out the comparison of Average hash, PHash and Color Moment hash, because I cannot
//! find them in PHash.
//!
//! ### Motivation
//!
//! Collects useful image hash algorithms into opencv, so we do not need to rewrite them by ourselves
//! again and again or rely on another 3rd party library(ex : PHash library). BOVW or correlation
//! matching are good and robust, but they are very slow compare with image hash, if you need to deal
//! with large scale CBIR(content based image retrieval) problem, image hash is a more reasonable
//! solution.
//!
//! ### More info
//!
//! You can learn more about img_hash modules from following links, these links show you how to find
//! similar image from ukbench dataset, provide thorough benchmark of different attacks(contrast, blur,
//! noise(gaussion,pepper and salt), jpeg compression, watermark, resize).
//!
//! [Introduction to image hash module of opencv](http://qtandopencv.blogspot.my/2016/06/introduction-to-image-hash-module-of.html)
//! [Speed up image hashing of opencv(img_hash) and introduce color moment hash](http://qtandopencv.blogspot.my/2016/06/speed-up-image-hashing-of-opencvimghash.html)
//!
//! ### Contributors
//!
//! Tham Ngap Wei, thamngapwei@gmail.com
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

/// use fewer block and generate 16*16/8 uchar hash value
pub const BLOCK_MEAN_HASH_MODE_0: i32 = 0;
/// use block blocks(step sizes/2), generate 31*31/8 + 1 uchar hash value
pub const BLOCK_MEAN_HASH_MODE_1: i32 = 1;

// boxed class cv::img_hash::AverageHash
/// Computes average hash value of the input image
///
/// This is a fast image hashing algorithm, but only work on simple case. For more details, please
/// refer to [lookslikeit](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_lookslikeit)
pub struct AverageHash {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for AverageHash {
    fn drop(&mut self) {
        unsafe { sys::cv_AverageHash_delete(self.ptr) };
    }
}

impl AverageHash {
    #[inline(always)] pub fn as_raw_AverageHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for AverageHash {}

impl core::AlgorithmTrait for AverageHash {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::img_hash::ImgHashBaseTrait for AverageHash {
    #[inline(always)] fn as_raw_ImgHashBase(&self) -> *mut c_void { self.ptr }
}

impl AverageHash {
    pub fn create() -> Result<types::PtrOfAverageHash> {
        unsafe { sys::cv_img_hash_AverageHash_create() }.into_result().map(|ptr| types::PtrOfAverageHash { ptr })
    }
    
}

// boxed class cv::img_hash::BlockMeanHash
/// Image hash based on block mean.
///
/// See [zauner2010implementation](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_zauner2010implementation) for details.
pub struct BlockMeanHash {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BlockMeanHash {
    fn drop(&mut self) {
        unsafe { sys::cv_BlockMeanHash_delete(self.ptr) };
    }
}

impl BlockMeanHash {
    #[inline(always)] pub fn as_raw_BlockMeanHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BlockMeanHash {}

impl core::AlgorithmTrait for BlockMeanHash {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::img_hash::ImgHashBaseTrait for BlockMeanHash {
    #[inline(always)] fn as_raw_ImgHashBase(&self) -> *mut c_void { self.ptr }
}

impl BlockMeanHash {
    /// Create BlockMeanHash object
    /// ## Parameters
    /// * mode: the mode
    pub fn set_mode(&mut self, mode: i32) -> Result<()> {
        unsafe { sys::cv_img_hash_BlockMeanHash_setMode_int(self.as_raw_BlockMeanHash(), mode) }.into_result()
    }
    
    pub fn get_mean(&self) -> Result<types::VectorOfdouble> {
        unsafe { sys::cv_img_hash_BlockMeanHash_getMean_const(self.as_raw_BlockMeanHash()) }.into_result().map(|ptr| types::VectorOfdouble { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * mode: BLOCK_MEAN_HASH_MODE_0
    pub fn create(mode: i32) -> Result<types::PtrOfBlockMeanHash> {
        unsafe { sys::cv_img_hash_BlockMeanHash_create_int(mode) }.into_result().map(|ptr| types::PtrOfBlockMeanHash { ptr })
    }
    
}

// boxed class cv::img_hash::ColorMomentHash
/// Image hash based on color moments.
///
/// See [tang2012perceptual](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_tang2012perceptual) for details.
pub struct ColorMomentHash {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ColorMomentHash {
    fn drop(&mut self) {
        unsafe { sys::cv_ColorMomentHash_delete(self.ptr) };
    }
}

impl ColorMomentHash {
    #[inline(always)] pub fn as_raw_ColorMomentHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ColorMomentHash {}

impl core::AlgorithmTrait for ColorMomentHash {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::img_hash::ImgHashBaseTrait for ColorMomentHash {
    #[inline(always)] fn as_raw_ImgHashBase(&self) -> *mut c_void { self.ptr }
}

impl ColorMomentHash {
    pub fn create() -> Result<types::PtrOfColorMomentHash> {
        unsafe { sys::cv_img_hash_ColorMomentHash_create() }.into_result().map(|ptr| types::PtrOfColorMomentHash { ptr })
    }
    
}

// Generating impl for trait crate::img_hash::ImgHashBase
/// The base class for image hash algorithms
pub trait ImgHashBaseTrait: core::AlgorithmTrait {
    fn as_raw_ImgHashBase(&self) -> *mut c_void;
}

// boxed class cv::img_hash::ImgHashBase
/// The base class for image hash algorithms
pub struct ImgHashBase {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ImgHashBase {
    fn drop(&mut self) {
        unsafe { sys::cv_ImgHashBase_delete(self.ptr) };
    }
}

impl ImgHashBase {
    #[inline(always)] pub fn as_raw_ImgHashBase(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ImgHashBase {}

impl core::AlgorithmTrait for ImgHashBase {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::img_hash::ImgHashBaseTrait for ImgHashBase {
    #[inline(always)] fn as_raw_ImgHashBase(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::img_hash::MarrHildrethHash
/// Marr-Hildreth Operator Based Hash, slowest but more discriminative.
///
/// See [zauner2010implementation](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_zauner2010implementation) for details.
pub struct MarrHildrethHash {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for MarrHildrethHash {
    fn drop(&mut self) {
        unsafe { sys::cv_MarrHildrethHash_delete(self.ptr) };
    }
}

impl MarrHildrethHash {
    #[inline(always)] pub fn as_raw_MarrHildrethHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MarrHildrethHash {}

impl core::AlgorithmTrait for MarrHildrethHash {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::img_hash::ImgHashBaseTrait for MarrHildrethHash {
    #[inline(always)] fn as_raw_ImgHashBase(&self) -> *mut c_void { self.ptr }
}

impl MarrHildrethHash {
    /// self explain
    pub fn get_alpha(&self) -> Result<f32> {
        unsafe { sys::cv_img_hash_MarrHildrethHash_getAlpha_const(self.as_raw_MarrHildrethHash()) }.into_result()
    }
    
    /// self explain
    pub fn get_scale(&self) -> Result<f32> {
        unsafe { sys::cv_img_hash_MarrHildrethHash_getScale_const(self.as_raw_MarrHildrethHash()) }.into_result()
    }
    
    /// Set Mh kernel parameters
    /// ## Parameters
    /// * alpha: int scale factor for marr wavelet (default=2).
    /// * scale: int level of scale factor (default = 1)
    pub fn set_kernel_param(&mut self, alpha: f32, scale: f32) -> Result<()> {
        unsafe { sys::cv_img_hash_MarrHildrethHash_setKernelParam_float_float(self.as_raw_MarrHildrethHash(), alpha, scale) }.into_result()
    }
    
    /// ## Parameters
    /// * alpha: int scale factor for marr wavelet (default=2).
    /// * scale: int level of scale factor (default = 1)
    ///
    /// ## C++ default parameters
    /// * alpha: 2.0f
    /// * scale: 1.0f
    pub fn create(alpha: f32, scale: f32) -> Result<types::PtrOfMarrHildrethHash> {
        unsafe { sys::cv_img_hash_MarrHildrethHash_create_float_float(alpha, scale) }.into_result().map(|ptr| types::PtrOfMarrHildrethHash { ptr })
    }
    
}

// boxed class cv::img_hash::PHash
/// pHash
///
/// Slower than average_hash, but tolerant of minor modifications
///
/// This algorithm can combat more variation than averageHash, for more details please refer to [lookslikeit](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_lookslikeit)
pub struct PHash {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for PHash {
    fn drop(&mut self) {
        unsafe { sys::cv_PHash_delete(self.ptr) };
    }
}

impl PHash {
    #[inline(always)] pub fn as_raw_PHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PHash {}

impl core::AlgorithmTrait for PHash {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::img_hash::ImgHashBaseTrait for PHash {
    #[inline(always)] fn as_raw_ImgHashBase(&self) -> *mut c_void { self.ptr }
}

impl PHash {
    pub fn create() -> Result<types::PtrOfPHash> {
        unsafe { sys::cv_img_hash_PHash_create() }.into_result().map(|ptr| types::PtrOfPHash { ptr })
    }
    
}

// boxed class cv::img_hash::RadialVarianceHash
/// Image hash based on Radon transform.
///
/// See [tang2012perceptual](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_tang2012perceptual) for details.
pub struct RadialVarianceHash {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for RadialVarianceHash {
    fn drop(&mut self) {
        unsafe { sys::cv_RadialVarianceHash_delete(self.ptr) };
    }
}

impl RadialVarianceHash {
    #[inline(always)] pub fn as_raw_RadialVarianceHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for RadialVarianceHash {}

impl core::AlgorithmTrait for RadialVarianceHash {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::img_hash::ImgHashBaseTrait for RadialVarianceHash {
    #[inline(always)] fn as_raw_ImgHashBase(&self) -> *mut c_void { self.ptr }
}

impl RadialVarianceHash {
    ///
    /// ## C++ default parameters
    /// * sigma: 1
    /// * num_of_angle_line: 180
    pub fn create(sigma: f64, num_of_angle_line: i32) -> Result<types::PtrOfRadialVarianceHash> {
        unsafe { sys::cv_img_hash_RadialVarianceHash_create_double_int(sigma, num_of_angle_line) }.into_result().map(|ptr| types::PtrOfRadialVarianceHash { ptr })
    }
    
    pub fn get_num_of_angle_line(&self) -> Result<i32> {
        unsafe { sys::cv_img_hash_RadialVarianceHash_getNumOfAngleLine_const(self.as_raw_RadialVarianceHash()) }.into_result()
    }
    
    pub fn get_sigma(&self) -> Result<f64> {
        unsafe { sys::cv_img_hash_RadialVarianceHash_getSigma_const(self.as_raw_RadialVarianceHash()) }.into_result()
    }
    
    pub fn set_num_of_angle_line(&mut self, value: i32) -> Result<()> {
        unsafe { sys::cv_img_hash_RadialVarianceHash_setNumOfAngleLine_int(self.as_raw_RadialVarianceHash(), value) }.into_result()
    }
    
    pub fn set_sigma(&mut self, value: f64) -> Result<()> {
        unsafe { sys::cv_img_hash_RadialVarianceHash_setSigma_double(self.as_raw_RadialVarianceHash(), value) }.into_result()
    }
    
    pub fn get_features(&mut self) -> Result<types::VectorOfdouble> {
        unsafe { sys::cv_img_hash_RadialVarianceHash_getFeatures(self.as_raw_RadialVarianceHash()) }.into_result().map(|ptr| types::VectorOfdouble { ptr })
    }
    
    pub fn get_hash(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_img_hash_RadialVarianceHash_getHash(self.as_raw_RadialVarianceHash()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    pub fn get_projection(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_img_hash_RadialVarianceHash_getProjection(self.as_raw_RadialVarianceHash()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}


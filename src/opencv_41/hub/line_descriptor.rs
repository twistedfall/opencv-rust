//! # Binary descriptors for lines extracted from an image
//!
//! Introduction
//! ------------
//!
//! One of the most challenging activities in computer vision is the extraction of useful information
//! from a given image. Such information, usually comes in the form of points that preserve some kind of
//! property (for instance, they are scale-invariant) and are actually representative of input image.
//!
//! The goal of this module is seeking a new kind of representative information inside an image and
//! providing the functionalities for its extraction and representation. In particular, differently from
//! previous methods for detection of relevant elements inside an image, lines are extracted in place of
//! points; a new class is defined ad hoc to summarize a line's properties, for reuse and plotting
//! purposes.
//!
//! Computation of binary descriptors
//! ---------------------------------
//!
//! To obtatin a binary descriptor representing a certain line detected from a certain octave of an
//! image, we first compute a non-binary descriptor as described in [LBD](https://docs.opencv.org/4.1.2/d0/de3/citelist.html#CITEREF_LBD) . Such algorithm works on
//! lines extracted using EDLine detector, as explained in [EDL](https://docs.opencv.org/4.1.2/d0/de3/citelist.html#CITEREF_EDL) . Given a line, we consider a
//! rectangular region centered at it and called *line support region (LSR)*. Such region is divided
//! into a set of bands ![inline formula](https://latex.codecogs.com/png.latex?%5C%7BB_1%2C%20B_2%2C%20...%2C%20B_m%5C%7D), whose length equals the one of line.
//!
//! If we indicate with ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bd%7D_L) the direction of line, the orthogonal and clockwise direction to line
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bd%7D_%7B%5Cperp%7D) can be determined; these two directions, are used to construct a reference frame
//! centered in the middle point of line. The gradients of pixels ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bg%27%7D) inside LSR can be projected
//! to the newly determined frame, obtaining their local equivalent
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bg%27%7D%20%3D%20%28%5Cbf%7Bg%7D%5ET%20%5Ccdot%20%5Cbf%7Bd%7D_%7B%5Cperp%7D%2C%20%5Cbf%7Bg%7D%5ET%20%5Ccdot%20%5Cbf%7Bd%7D_L%29%5ET%20%5Ctriangleq%20%28%5Cbf%7Bg%27%7D_%7Bd_%7B%5Cperp%7D%7D%2C%20%5Cbf%7Bg%27%7D_%7Bd_L%7D%29%5ET).
//!
//! Later on, a Gaussian function is applied to all LSR's pixels along ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bd%7D_%5Cperp) direction; first,
//! we assign a global weighting coefficient ![inline formula](https://latex.codecogs.com/png.latex?f_g%28i%29%20%3D%20%281%2F%5Csqrt%7B2%5Cpi%7D%5Csigma_g%29e%5E%7B-d%5E2_i%2F2%5Csigma%5E2_g%7D) to
//! *i*-th row in LSR, where ![inline formula](https://latex.codecogs.com/png.latex?d_i) is the distance of *i*-th row from the center row in LSR,
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Csigma_g%20%3D%200.5%28m%20%5Ccdot%20w%20-%201%29) and ![inline formula](https://latex.codecogs.com/png.latex?w) is the width of bands (the same for every band). Secondly,
//! considering a band ![inline formula](https://latex.codecogs.com/png.latex?B_j) and its neighbor bands ![inline formula](https://latex.codecogs.com/png.latex?B_%7Bj-1%7D%2C%20B_%7Bj%2B1%7D), we assign a local weighting
//! ![inline formula](https://latex.codecogs.com/png.latex?F_l%28k%29%20%3D%20%281%2F%5Csqrt%7B2%5Cpi%7D%5Csigma_l%29e%5E%7B-d%27%5E2_k%2F2%5Csigma_l%5E2%7D), where ![inline formula](https://latex.codecogs.com/png.latex?d%27_k) is the distance of *k*-th
//! row from the center row in ![inline formula](https://latex.codecogs.com/png.latex?B_j) and ![inline formula](https://latex.codecogs.com/png.latex?%5Csigma_l%20%3D%20w). Using the global and local weights, we obtain,
//! at the same time, the reduction of role played by gradients far from line and of boundary effect,
//! respectively.
//!
//! Each band ![inline formula](https://latex.codecogs.com/png.latex?B_j) in LSR has an associated *band descriptor(BD)* which is computed considering
//! previous and next band (top and bottom bands are ignored when computing descriptor for first and
//! last band). Once each band has been assignen its BD, the LBD descriptor of line is simply given by
//!
//! ![block formula](https://latex.codecogs.com/png.latex?LBD%20%3D%20%28BD_1%5ET%2C%20BD_2%5ET%2C%20...%20%2C%20BD%5ET_m%29%5ET.)
//!
//! To compute a band descriptor ![inline formula](https://latex.codecogs.com/png.latex?B_j), each *k*-th row in it is considered and the gradients in such
//! row are accumulated:
//!
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bmatrix%7D%20%5Cbf%7BV1%7D%5Ek_j%20%3D%20%5Clambda%20%5Csum%5Climits_%7B%5Cbf%7Bg%7D%27_%7Bd_%5Cperp%7D%3E0%7D%5Cbf%7Bg%7D%27_%7Bd_%5Cperp%7D%2C%20%26%20%20%5Cbf%7BV2%7D%5Ek_j%20%3D%20%5Clambda%20%5Csum%5Climits_%7B%5Cbf%7Bg%7D%27_%7Bd_%5Cperp%7D%3C0%7D%20-%5Cbf%7Bg%7D%27_%7Bd_%5Cperp%7D%2C%20%5C%5C%20%5Cbf%7BV3%7D%5Ek_j%20%3D%20%5Clambda%20%5Csum%5Climits_%7B%5Cbf%7Bg%7D%27_%7Bd_L%7D%3E0%7D%5Cbf%7Bg%7D%27_%7Bd_L%7D%2C%20%26%20%5Cbf%7BV4%7D%5Ek_j%20%3D%20%5Clambda%20%5Csum%5Climits_%7B%5Cbf%7Bg%7D%27_%7Bd_L%7D%3C0%7D%20-%5Cbf%7Bg%7D%27_%7Bd_L%7D%5Cend%7Bmatrix%7D.)
//!
//! with ![inline formula](https://latex.codecogs.com/png.latex?%5Clambda%20%3D%20f_g%28k%29f_l%28k%29).
//!
//! By stacking previous results, we obtain the *band description matrix (BDM)*
//!
//! ![block formula](https://latex.codecogs.com/png.latex?BDM_j%20%3D%20%5Cleft%28%5Cbegin%7Bmatrix%7D%20%5Cbf%7BV1%7D_j%5E1%20%26%20%5Cbf%7BV1%7D_j%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV1%7D_j%5En%20%5C%5C%20%5Cbf%7BV2%7D_j%5E1%20%26%20%5Cbf%7BV2%7D_j%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV2%7D_j%5En%20%5C%5C%20%5Cbf%7BV3%7D_j%5E1%20%26%20%5Cbf%7BV3%7D_j%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV3%7D_j%5En%20%5C%5C%20%5Cbf%7BV4%7D_j%5E1%20%26%20%5Cbf%7BV4%7D_j%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV4%7D_j%5En%20%5Cend%7Bmatrix%7D%20%5Cright%29%20%5Cin%20%5Cmathbb%7BR%7D%5E%7B4%5Ctimes%20n%7D%2C)
//!
//! with ![inline formula](https://latex.codecogs.com/png.latex?n) the number of rows in band ![inline formula](https://latex.codecogs.com/png.latex?B_j):
//!
//! ![block formula](https://latex.codecogs.com/png.latex?n%20%3D%20%5Cbegin%7Bcases%7D%202w%2C%20%26%20j%20%3D%201%7C%7Cm%3B%20%5C%5C%203w%2C%20%26%20%5Cmbox%7Belse%7D.%20%5Cend%7Bcases%7D)
//!
//! Each ![inline formula](https://latex.codecogs.com/png.latex?BD_j) can be obtained using the standard deviation vector ![inline formula](https://latex.codecogs.com/png.latex?S_j) and mean vector ![inline formula](https://latex.codecogs.com/png.latex?M_j) of
//! ![inline formula](https://latex.codecogs.com/png.latex?BDM_J). Thus, finally:
//!
//! ![block formula](https://latex.codecogs.com/png.latex?LBD%20%3D%20%28M_1%5ET%2C%20S_1%5ET%2C%20M_2%5ET%2C%20S_2%5ET%2C%20%5Cldots%2C%20M_m%5ET%2C%20S_m%5ET%29%5ET%20%5Cin%20%5Cmathbb%7BR%7D%5E%7B8m%7D)
//!
//! Once the LBD has been obtained, it must be converted into a binary form. For such purpose, we
//! consider 32 possible pairs of BD inside it; each couple of BD is compared bit by bit and comparison
//! generates an 8 bit string. Concatenating 32 comparison strings, we get the 256-bit final binary
//! representation of a single LBD.
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

pub const MLN10: f64 = 2.30258509299404568402;
pub const RELATIVE_ERROR_FACTOR: f64 = 100.0;

/// A class to represent a line
///
/// As aformentioned, it is been necessary to design a class that fully stores the information needed to
/// characterize completely a line and plot it on image it was extracted from, when required.
///
/// *KeyLine* class has been created for such goal; it is mainly inspired to Feature2d's KeyPoint class,
/// since KeyLine shares some of *KeyPoint*'s fields, even if a part of them assumes a different
/// meaning, when speaking about lines. In particular:
///
/// *   the *class_id* field is used to gather lines extracted from different octaves which refer to
/// same line inside original image (such lines and the one they represent in original image share
/// the same *class_id* value)
/// *   the *angle* field represents line's slope with respect to (positive) X axis
/// *   the *pt* field represents line's midpoint
/// *   the *response* field is computed as the ratio between the line's length and maximum between
/// image's width and height
/// *   the *size* field is the area of the smallest rectangle containing line
///
/// Apart from fields inspired to KeyPoint class, KeyLines stores information about extremes of line in
/// original image and in octave it was extracted from, about line's length and number of pixels it
/// covers.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyLine {
    pub angle: f32,
    pub class_id: i32,
    pub octave: i32,
    pub pt: core::Point2f,
    pub response: f32,
    pub size: f32,
    pub start_point_x: f32,
    pub start_point_y: f32,
    pub end_point_x: f32,
    pub end_point_y: f32,
    pub s_point_in_octave_x: f32,
    pub s_point_in_octave_y: f32,
    pub e_point_in_octave_x: f32,
    pub e_point_in_octave_y: f32,
    pub line_length: f32,
    pub num_of_pixels: i32,
}

/// Lines extraction methodology
/// ----------------------------
///
/// The lines extraction methodology described in the following is mainly based on [EDL](https://docs.opencv.org/4.1.2/d0/de3/citelist.html#CITEREF_EDL) . The
/// extraction starts with a Gaussian pyramid generated from an original image, downsampled N-1 times,
/// blurred N times, to obtain N layers (one for each octave), with layer 0 corresponding to input
/// image. Then, from each layer (octave) in the pyramid, lines are extracted using LSD algorithm.
///
/// Differently from EDLine lines extractor used in original article, LSD furnishes information only
/// about lines extremes; thus, additional information regarding slope and equation of line are computed
/// via analytic methods. The number of pixels is obtained using *LineIterator*. Extracted lines are
/// returned in the form of KeyLine objects, but since extraction is based on a method different from
/// the one used in *BinaryDescriptor* class, data associated to a line's extremes in original image and
/// in octave it was extracted from, coincide. KeyLine's field *class_id* is used as an index to
/// indicate the order of extraction of a line inside a single octave.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LSDParam {
    pub scale: f64,
    pub sigma_scale: f64,
    pub quant: f64,
    pub ang_th: f64,
    pub log_eps: f64,
    pub density_th: f64,
    pub n_bins: i32,
}

/// struct for drawing options
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DrawLinesMatchesFlags {
    __rust_private: [u8; 0],
}

/// Draws keylines.
///
/// ## Parameters
/// * image: input image
/// * keylines: keylines to be drawn
/// * outImage: output image to draw on
/// * color: color of lines to be drawn (if set to defaul value, color is chosen randomly)
/// * flags: drawing flags
///
/// ## C++ default parameters
/// * color: Scalar::all( -1 )
/// * flags: DrawLinesMatchesFlags::DEFAULT
pub fn draw_keylines(image: &core::Mat, keylines: &types::VectorOfKeyLine, out_image: &mut core::Mat, color: core::Scalar, flags: i32) -> Result<()> {
    unsafe { sys::cv_line_descriptor_drawKeylines_Mat_VectorOfKeyLine_Mat_Scalar_int(image.as_raw_Mat(), keylines.as_raw_VectorOfKeyLine(), out_image.as_raw_Mat(), color, flags) }.into_result()
}

/// Draws the found matches of keylines from two images.
///
/// ## Parameters
/// * img1: first image
/// * keylines1: keylines extracted from first image
/// * img2: second image
/// * keylines2: keylines extracted from second image
/// * matches1to2: vector of matches
/// * outImg: output matrix to draw on
/// * matchColor: drawing color for matches (chosen randomly in case of default value)
/// * singleLineColor: drawing color for keylines (chosen randomly in case of default value)
/// * matchesMask: mask to indicate which matches must be drawn
/// * flags: drawing flags, see DrawLinesMatchesFlags
///
///
/// Note: If both *matchColor* and *singleLineColor* are set to their default values, function draws
/// matched lines and line connecting them with same color
///
/// ## C++ default parameters
/// * match_color: Scalar::all( -1 )
/// * single_line_color: Scalar::all( -1 )
/// * matches_mask: std::vector<char>()
/// * flags: DrawLinesMatchesFlags::DEFAULT
pub fn draw_line_matches(img1: &core::Mat, keylines1: &types::VectorOfKeyLine, img2: &core::Mat, keylines2: &types::VectorOfKeyLine, matches1to2: &types::VectorOfDMatch, out_img: &mut core::Mat, match_color: core::Scalar, single_line_color: core::Scalar, matches_mask: &types::VectorOfchar, flags: i32) -> Result<()> {
    unsafe { sys::cv_line_descriptor_drawLineMatches_Mat_VectorOfKeyLine_Mat_VectorOfKeyLine_VectorOfDMatch_Mat_Scalar_Scalar_VectorOfchar_int(img1.as_raw_Mat(), keylines1.as_raw_VectorOfKeyLine(), img2.as_raw_Mat(), keylines2.as_raw_VectorOfKeyLine(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw_Mat(), match_color, single_line_color, matches_mask.as_raw_VectorOfchar(), flags) }.into_result()
}

// boxed class cv::line_descriptor::BinaryDescriptor
/// Class implements both functionalities for detection of lines and computation of their
/// binary descriptor.
///
/// Class' interface is mainly based on the ones of classical detectors and extractors, such as
/// Feature2d's @ref features2d_main and @ref features2d_match. Retrieved information about lines is
/// stored in line_descriptor::KeyLine objects.
pub struct BinaryDescriptor {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BinaryDescriptor {
    fn drop(&mut self) {
        unsafe { sys::cv_BinaryDescriptor_delete(self.ptr) };
    }
}

impl BinaryDescriptor {
    #[inline(always)] pub fn as_raw_BinaryDescriptor(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BinaryDescriptor {}

impl core::AlgorithmTrait for BinaryDescriptor {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl BinaryDescriptor {
    /// Constructor
    ///
    /// ## Parameters
    /// * parameters: configuration parameters BinaryDescriptor::Params
    ///
    /// If no argument is provided, constructor sets default values (see comments in the code snippet in
    /// previous section). Default values are strongly reccomended.
    ///
    /// ## C++ default parameters
    /// * parameters: BinaryDescriptor::Params()
    pub fn new(parameters: &crate::line_descriptor::BinaryDescriptor_Params) -> Result<crate::line_descriptor::BinaryDescriptor> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_BinaryDescriptor_Params(parameters.as_raw_BinaryDescriptor_Params()) }.into_result().map(|ptr| crate::line_descriptor::BinaryDescriptor { ptr })
    }
    
    /// Create a BinaryDescriptor object with default parameters (or with the ones provided)
    /// and return a smart pointer to it
    pub fn create_binary_descriptor() -> Result<types::PtrOfBinaryDescriptor> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor() }.into_result().map(|ptr| types::PtrOfBinaryDescriptor { ptr })
    }
    
    pub fn create_binary_descriptor_1(parameters: &crate::line_descriptor::BinaryDescriptor_Params) -> Result<types::PtrOfBinaryDescriptor> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor_Params(parameters.as_raw_BinaryDescriptor_Params()) }.into_result().map(|ptr| types::PtrOfBinaryDescriptor { ptr })
    }
    
    /// Get current number of octaves
    pub fn get_num_of_octaves(&mut self) -> Result<i32> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_getNumOfOctaves(self.as_raw_BinaryDescriptor()) }.into_result()
    }
    
    /// Set number of octaves
    /// ## Parameters
    /// * octaves: number of octaves
    pub fn set_num_of_octaves(&mut self, octaves: i32) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_setNumOfOctaves_int(self.as_raw_BinaryDescriptor(), octaves) }.into_result()
    }
    
    /// Get current width of bands
    pub fn get_width_of_band(&mut self) -> Result<i32> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_getWidthOfBand(self.as_raw_BinaryDescriptor()) }.into_result()
    }
    
    /// Set width of bands
    /// ## Parameters
    /// * width: width of bands
    pub fn set_width_of_band(&mut self, width: i32) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_setWidthOfBand_int(self.as_raw_BinaryDescriptor(), width) }.into_result()
    }
    
    /// Get current reduction ratio (used in Gaussian pyramids)
    pub fn get_reduction_ratio(&mut self) -> Result<i32> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_getReductionRatio(self.as_raw_BinaryDescriptor()) }.into_result()
    }
    
    /// Set reduction ratio (used in Gaussian pyramids)
    /// ## Parameters
    /// * rRatio: reduction ratio
    pub fn set_reduction_ratio(&mut self, r_ratio: i32) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_setReductionRatio_int(self.as_raw_BinaryDescriptor(), r_ratio) }.into_result()
    }
    
    /// Read parameters from a FileNode object and store them
    ///
    /// ## Parameters
    /// * fn: source FileNode file
    pub fn read(&mut self, _fn: &core::FileNode) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_read_FileNode(self.as_raw_BinaryDescriptor(), _fn.as_raw_FileNode()) }.into_result()
    }
    
    /// Store parameters to a FileStorage object
    ///
    /// ## Parameters
    /// * fs: output FileStorage file
    pub fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_write_const_FileStorage(self.as_raw_BinaryDescriptor(), fs.as_raw_FileStorage()) }.into_result()
    }
    
    /// Requires line detection
    ///
    /// ## Parameters
    /// * image: input image
    /// * keypoints: vector that will store extracted lines for one or more images
    /// * mask: mask matrix to detect only KeyLines of interest
    ///
    /// ## C++ default parameters
    /// * mask: Mat()
    pub fn detect(&mut self, image: &core::Mat, keypoints: &mut types::VectorOfKeyLine, mask: &core::Mat) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_detect_Mat_VectorOfKeyLine_Mat(self.as_raw_BinaryDescriptor(), image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyLine(), mask.as_raw_Mat()) }.into_result()
    }
    
    /// ## Parameters
    /// * images: input images
    /// * keylines: set of vectors that will store extracted lines for one or more images
    /// * masks: vector of mask matrices to detect only KeyLines of interest from each input image
    ///
    /// ## C++ default parameters
    /// * masks: std::vector<Mat>()
    pub fn detect_1(&self, images: &types::VectorOfMat, keylines: &mut types::VectorOfVectorOfKeyLine, masks: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_detect_const_VectorOfMat_VectorOfVectorOfKeyLine_VectorOfMat(self.as_raw_BinaryDescriptor(), images.as_raw_VectorOfMat(), keylines.as_raw_VectorOfVectorOfKeyLine(), masks.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// Requires descriptors computation
    ///
    /// ## Parameters
    /// * image: input image
    /// * keylines: vector containing lines for which descriptors must be computed
    /// * descriptors:
    /// * returnFloatDescr: flag (when set to true, original non-binary descriptors are returned)
    ///
    /// ## C++ default parameters
    /// * return_float_descr: false
    pub fn compute(&self, image: &core::Mat, keylines: &mut types::VectorOfKeyLine, descriptors: &mut core::Mat, return_float_descr: bool) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_compute_const_Mat_VectorOfKeyLine_Mat_bool(self.as_raw_BinaryDescriptor(), image.as_raw_Mat(), keylines.as_raw_VectorOfKeyLine(), descriptors.as_raw_Mat(), return_float_descr) }.into_result()
    }
    
    /// ## Parameters
    /// * images: input images
    /// * keylines: set of vectors containing lines for which descriptors must be computed
    /// * descriptors:
    /// * returnFloatDescr: flag (when set to true, original non-binary descriptors are returned)
    ///
    /// ## C++ default parameters
    /// * return_float_descr: false
    pub fn compute_1(&self, images: &types::VectorOfMat, keylines: &mut types::VectorOfVectorOfKeyLine, descriptors: &mut types::VectorOfMat, return_float_descr: bool) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_compute_const_VectorOfMat_VectorOfVectorOfKeyLine_VectorOfMat_bool(self.as_raw_BinaryDescriptor(), images.as_raw_VectorOfMat(), keylines.as_raw_VectorOfVectorOfKeyLine(), descriptors.as_raw_VectorOfMat(), return_float_descr) }.into_result()
    }
    
    /// Return descriptor size
    pub fn descriptor_size(&self) -> Result<i32> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_descriptorSize_const(self.as_raw_BinaryDescriptor()) }.into_result()
    }
    
    /// Return data type
    pub fn descriptor_type(&self) -> Result<i32> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_descriptorType_const(self.as_raw_BinaryDescriptor()) }.into_result()
    }
    
    /// returns norm mode
    pub fn default_norm(&self) -> Result<i32> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_defaultNorm_const(self.as_raw_BinaryDescriptor()) }.into_result()
    }
    
}

// boxed class cv::line_descriptor::BinaryDescriptor::Params
/// List of BinaryDescriptor parameters:
pub struct BinaryDescriptor_Params {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BinaryDescriptor_Params {
    fn drop(&mut self) {
        unsafe { sys::cv_BinaryDescriptor_Params_delete(self.ptr) };
    }
}

impl BinaryDescriptor_Params {
    #[inline(always)] pub fn as_raw_BinaryDescriptor_Params(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BinaryDescriptor_Params {}

impl BinaryDescriptor_Params {
    pub fn default() -> Result<crate::line_descriptor::BinaryDescriptor_Params> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_Params() }.into_result().map(|ptr| crate::line_descriptor::BinaryDescriptor_Params { ptr })
    }
    
    /// read parameters from a FileNode object and store them (struct function)
    pub fn read(&mut self, _fn: &core::FileNode) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_read_FileNode(self.as_raw_BinaryDescriptor_Params(), _fn.as_raw_FileNode()) }.into_result()
    }
    
    /// store parameters to a FileStorage object (struct function)
    pub fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_write_const_FileStorage(self.as_raw_BinaryDescriptor_Params(), fs.as_raw_FileStorage()) }.into_result()
    }
    
}

// boxed class cv::line_descriptor::BinaryDescriptorMatcher
/// furnishes all functionalities for querying a dataset provided by user or internal to
/// class (that user must, anyway, populate) on the model of @ref features2d_match
///
///
/// Once descriptors have been extracted from an image (both they represent lines and points), it
/// becomes interesting to be able to match a descriptor with another one extracted from a different
/// image and representing the same line or point, seen from a differente perspective or on a different
/// scale. In reaching such goal, the main headache is designing an efficient search algorithm to
/// associate a query descriptor to one extracted from a dataset. In the following, a matching modality
/// based on *Multi-Index Hashing (MiHashing)* will be described.
///
/// Multi-Index Hashing
/// -------------------
///
/// The theory described in this section is based on [MIH](https://docs.opencv.org/4.1.2/d0/de3/citelist.html#CITEREF_MIH) . Given a dataset populated with binary
/// codes, each code is indexed *m* times into *m* different hash tables, according to *m* substrings it
/// has been divided into. Thus, given a query code, all the entries close to it at least in one
/// substring are returned by search as *neighbor candidates*. Returned entries are then checked for
/// validity by verifying that their full codes are not distant (in Hamming space) more than *r* bits
/// from query code. In details, each binary code **h** composed of *b* bits is divided into *m*
/// disjoint substrings ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7Bh%7D%5E%7B%281%29%7D%2C%20...%2C%20%5Cmathbf%7Bh%7D%5E%7B%28m%29%7D), each with length
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Clfloor%20b%2Fm%20%5Crfloor) or ![inline formula](https://latex.codecogs.com/png.latex?%5Clceil%20b%2Fm%20%5Crceil) bits. Formally, when two codes **h** and **g** differ
/// by at the most *r* bits, in at the least one of their *m* substrings they differ by at the most
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Clfloor%20r%2Fm%20%5Crfloor) bits. In particular, when ![inline formula](https://latex.codecogs.com/png.latex?%7C%7C%5Cmathbf%7Bh%7D-%5Cmathbf%7Bg%7D%7C%7C_H%20%5Cle%20r) (where ![inline formula](https://latex.codecogs.com/png.latex?%7C%7C.%7C%7C_H)
/// is the Hamming norm), there must exist a substring *k* (with ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Cle%20k%20%5Cle%20m)) such that
///
/// ![block formula](https://latex.codecogs.com/png.latex?%7C%7C%5Cmathbf%7Bh%7D%5E%7B%28k%29%7D%20-%20%5Cmathbf%7Bg%7D%5E%7B%28k%29%7D%7C%7C_H%20%5Cle%20%5Cleft%5Clfloor%20%5Cfrac%7Br%7D%7Bm%7D%20%5Cright%5Crfloor%20.)
///
/// That means that if Hamming distance between each of the *m* substring is strictly greater than
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Clfloor%20r%2Fm%20%5Crfloor), then ![inline formula](https://latex.codecogs.com/png.latex?%7C%7C%5Cmathbf%7Bh%7D-%5Cmathbf%7Bg%7D%7C%7C_H) must be larger that *r* and that is a
/// contradiction. If the codes in dataset are divided into *m* substrings, then *m* tables will be
/// built. Given a query **q** with substrings ![inline formula](https://latex.codecogs.com/png.latex?%5C%7B%5Cmathbf%7Bq%7D%5E%7B%28i%29%7D%5C%7D%5Em_%7Bi%3D1%7D), *i*-th hash table is
/// searched for entries distant at the most ![inline formula](https://latex.codecogs.com/png.latex?%5Clfloor%20r%2Fm%20%5Crfloor) from ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7Bq%7D%5E%7B%28i%29%7D) and a set of
/// candidates ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathcal%7BN%7D_i%28%5Cmathbf%7Bq%7D%29) is obtained. The union of sets
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathcal%7BN%7D%28%5Cmathbf%7Bq%7D%29%20%3D%20%5Cbigcup_i%20%5Cmathcal%7BN%7D_i%28%5Cmathbf%7Bq%7D%29) is a superset of the *r*-neighbors
/// of **q**. Then, last step of algorithm is computing the Hamming distance between **q** and each
/// element in ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathcal%7BN%7D%28%5Cmathbf%7Bq%7D%29), deleting the codes that are distant more that *r* from **q**.
pub struct BinaryDescriptorMatcher {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BinaryDescriptorMatcher {
    fn drop(&mut self) {
        unsafe { sys::cv_BinaryDescriptorMatcher_delete(self.ptr) };
    }
}

impl BinaryDescriptorMatcher {
    #[inline(always)] pub fn as_raw_BinaryDescriptorMatcher(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BinaryDescriptorMatcher {}

impl core::AlgorithmTrait for BinaryDescriptorMatcher {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl BinaryDescriptorMatcher {
    /// For every input query descriptor, retrieve the best matching one from a dataset provided from user
    /// or from the one internal to class
    ///
    /// ## Parameters
    /// * queryDescriptors: query descriptors
    /// * trainDescriptors: dataset of descriptors furnished by user
    /// * matches: vector to host retrieved matches
    /// * mask: mask to select which input descriptors must be matched to one in dataset
    ///
    /// ## C++ default parameters
    /// * mask: Mat()
    pub fn _match(&self, query_descriptors: &core::Mat, train_descriptors: &core::Mat, matches: &mut types::VectorOfDMatch, mask: &core::Mat) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_match_const_Mat_Mat_VectorOfDMatch_Mat(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_VectorOfDMatch(), mask.as_raw_Mat()) }.into_result()
    }
    
    /// ## Parameters
    /// * queryDescriptors: query descriptors
    /// * matches: vector to host retrieved matches
    /// * masks: vector of masks to select which input descriptors must be matched to one in dataset
    /// (the *i*-th mask in vector indicates whether each input query can be matched with descriptors in
    /// dataset relative to *i*-th image)
    ///
    /// ## C++ default parameters
    /// * masks: std::vector<Mat>()
    pub fn _match_1(&mut self, query_descriptors: &core::Mat, matches: &mut types::VectorOfDMatch, masks: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_match_Mat_VectorOfDMatch_VectorOfMat(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_VectorOfDMatch(), masks.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// For every input query descriptor, retrieve the best *k* matching ones from a dataset provided from
    /// user or from the one internal to class
    ///
    /// ## Parameters
    /// * queryDescriptors: query descriptors
    /// * trainDescriptors: dataset of descriptors furnished by user
    /// * matches: vector to host retrieved matches
    /// * k: number of the closest descriptors to be returned for every input query
    /// * mask: mask to select which input descriptors must be matched to ones in dataset
    /// * compactResult: flag to obtain a compact result (if true, a vector that doesn't contain any
    /// matches for a given query is not inserted in final result)
    ///
    /// ## C++ default parameters
    /// * mask: Mat()
    /// * compact_result: false
    pub fn knn_match(&self, query_descriptors: &core::Mat, train_descriptors: &core::Mat, matches: &mut types::VectorOfVectorOfDMatch, k: i32, mask: &core::Mat, compact_result: bool) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_Mat_Mat_VectorOfVectorOfDMatch_int_Mat_bool(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_VectorOfVectorOfDMatch(), k, mask.as_raw_Mat(), compact_result) }.into_result()
    }
    
    /// ## Parameters
    /// * queryDescriptors: query descriptors
    /// * matches: vector to host retrieved matches
    /// * k: number of the closest descriptors to be returned for every input query
    /// * masks: vector of masks to select which input descriptors must be matched to ones in dataset
    /// (the *i*-th mask in vector indicates whether each input query can be matched with descriptors in
    /// dataset relative to *i*-th image)
    /// * compactResult: flag to obtain a compact result (if true, a vector that doesn't contain any
    /// matches for a given query is not inserted in final result)
    ///
    /// ## C++ default parameters
    /// * masks: std::vector<Mat>()
    /// * compact_result: false
    pub fn knn_match_1(&mut self, query_descriptors: &core::Mat, matches: &mut types::VectorOfVectorOfDMatch, k: i32, masks: &types::VectorOfMat, compact_result: bool) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_Mat_VectorOfVectorOfDMatch_int_VectorOfMat_bool(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_VectorOfVectorOfDMatch(), k, masks.as_raw_VectorOfMat(), compact_result) }.into_result()
    }
    
    /// For every input query descriptor, retrieve, from a dataset provided from user or from the one
    /// internal to class, all the descriptors that are not further than *maxDist* from input query
    ///
    /// ## Parameters
    /// * queryDescriptors: query descriptors
    /// * trainDescriptors: dataset of descriptors furnished by user
    /// * matches: vector to host retrieved matches
    /// * maxDistance: search radius
    /// * mask: mask to select which input descriptors must be matched to ones in dataset
    /// * compactResult: flag to obtain a compact result (if true, a vector that doesn't contain any
    /// matches for a given query is not inserted in final result)
    ///
    /// ## C++ default parameters
    /// * mask: Mat()
    /// * compact_result: false
    pub fn radius_match(&self, query_descriptors: &core::Mat, train_descriptors: &core::Mat, matches: &mut types::VectorOfVectorOfDMatch, max_distance: f32, mask: &core::Mat, compact_result: bool) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_Mat_Mat_VectorOfVectorOfDMatch_float_Mat_bool(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_VectorOfVectorOfDMatch(), max_distance, mask.as_raw_Mat(), compact_result) }.into_result()
    }
    
    /// ## Parameters
    /// * queryDescriptors: query descriptors
    /// * matches: vector to host retrieved matches
    /// * maxDistance: search radius
    /// * masks: vector of masks to select which input descriptors must be matched to ones in dataset
    /// (the *i*-th mask in vector indicates whether each input query can be matched with descriptors in
    /// dataset relative to *i*-th image)
    /// * compactResult: flag to obtain a compact result (if true, a vector that doesn't contain any
    /// matches for a given query is not inserted in final result)
    ///
    /// ## C++ default parameters
    /// * masks: std::vector<Mat>()
    /// * compact_result: false
    pub fn radius_match_1(&mut self, query_descriptors: &core::Mat, matches: &mut types::VectorOfVectorOfDMatch, max_distance: f32, masks: &types::VectorOfMat, compact_result: bool) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_Mat_VectorOfVectorOfDMatch_float_VectorOfMat_bool(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_VectorOfVectorOfDMatch(), max_distance, masks.as_raw_VectorOfMat(), compact_result) }.into_result()
    }
    
    /// Store locally new descriptors to be inserted in dataset, without updating dataset.
    ///
    /// ## Parameters
    /// * descriptors: matrices containing descriptors to be inserted into dataset
    ///
    ///
    /// Note: Each matrix *i* in **descriptors** should contain descriptors relative to lines extracted from
    /// *i*-th image.
    pub fn add(&mut self, descriptors: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_add_VectorOfMat(self.as_raw_BinaryDescriptorMatcher(), descriptors.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// Update dataset by inserting into it all descriptors that were stored locally by *add* function.
    ///
    ///
    /// Note: Every time this function is invoked, current dataset is deleted and locally stored descriptors
    /// are inserted into dataset. The locally stored copy of just inserted descriptors is then removed.
    pub fn train(&mut self) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_train(self.as_raw_BinaryDescriptorMatcher()) }.into_result()
    }
    
    /// Create a BinaryDescriptorMatcher object and return a smart pointer to it.
    pub fn create_binary_descriptor_matcher() -> Result<types::PtrOfBinaryDescriptorMatcher> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_createBinaryDescriptorMatcher() }.into_result().map(|ptr| types::PtrOfBinaryDescriptorMatcher { ptr })
    }
    
    /// Clear dataset and internal data
    pub fn clear(&mut self) -> Result<()> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_clear(self.as_raw_BinaryDescriptorMatcher()) }.into_result()
    }
    
    /// Constructor.
    ///
    /// The BinaryDescriptorMatcher constructed is able to store and manage 256-bits long entries.
    pub fn default() -> Result<crate::line_descriptor::BinaryDescriptorMatcher> {
        unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_BinaryDescriptorMatcher() }.into_result().map(|ptr| crate::line_descriptor::BinaryDescriptorMatcher { ptr })
    }
    
}

impl KeyLine {
    /// Returns the start point of the line in the original image
    pub fn get_start_point(self) -> Result<core::Point2f> {
        unsafe { sys::cv_line_descriptor_KeyLine_getStartPoint_const(self) }.into_result()
    }
    
    /// Returns the end point of the line in the original image
    pub fn get_end_point(self) -> Result<core::Point2f> {
        unsafe { sys::cv_line_descriptor_KeyLine_getEndPoint_const(self) }.into_result()
    }
    
    /// Returns the start point of the line in the octave it was extracted from
    pub fn get_start_point_in_octave(self) -> Result<core::Point2f> {
        unsafe { sys::cv_line_descriptor_KeyLine_getStartPointInOctave_const(self) }.into_result()
    }
    
    /// Returns the end point of the line in the octave it was extracted from
    pub fn get_end_point_in_octave(self) -> Result<core::Point2f> {
        unsafe { sys::cv_line_descriptor_KeyLine_getEndPointInOctave_const(self) }.into_result()
    }
    
    /// constructor
    pub fn default() -> Result<crate::line_descriptor::KeyLine> {
        unsafe { sys::cv_line_descriptor_KeyLine_KeyLine() }.into_result()
    }
    
}

// boxed class cv::line_descriptor::LSDDetector
pub struct LSDDetector {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for LSDDetector {
    fn drop(&mut self) {
        unsafe { sys::cv_LSDDetector_delete(self.ptr) };
    }
}

impl LSDDetector {
    #[inline(always)] pub fn as_raw_LSDDetector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LSDDetector {}

impl core::AlgorithmTrait for LSDDetector {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl LSDDetector {
    pub fn default() -> Result<crate::line_descriptor::LSDDetector> {
        unsafe { sys::cv_line_descriptor_LSDDetector_LSDDetector() }.into_result().map(|ptr| crate::line_descriptor::LSDDetector { ptr })
    }
    
    pub fn new(_params: crate::line_descriptor::LSDParam) -> Result<crate::line_descriptor::LSDDetector> {
        unsafe { sys::cv_line_descriptor_LSDDetector_LSDDetector_LSDParam(_params) }.into_result().map(|ptr| crate::line_descriptor::LSDDetector { ptr })
    }
    
    /// Creates ad LSDDetector object, using smart pointers.
    pub fn create_lsd_detector() -> Result<types::PtrOfLSDDetector> {
        unsafe { sys::cv_line_descriptor_LSDDetector_createLSDDetector() }.into_result().map(|ptr| types::PtrOfLSDDetector { ptr })
    }
    
    pub fn create_lsd_detector_1(params: crate::line_descriptor::LSDParam) -> Result<types::PtrOfLSDDetector> {
        unsafe { sys::cv_line_descriptor_LSDDetector_createLSDDetector_LSDParam(params) }.into_result().map(|ptr| types::PtrOfLSDDetector { ptr })
    }
    
    /// Detect lines inside an image.
    ///
    /// ## Parameters
    /// * image: input image
    /// * keypoints: vector that will store extracted lines for one or more images
    /// * scale: scale factor used in pyramids generation
    /// * numOctaves: number of octaves inside pyramid
    /// * mask: mask matrix to detect only KeyLines of interest
    ///
    /// ## C++ default parameters
    /// * mask: Mat()
    pub fn detect(&mut self, image: &core::Mat, keypoints: &mut types::VectorOfKeyLine, scale: i32, num_octaves: i32, mask: &core::Mat) -> Result<()> {
        unsafe { sys::cv_line_descriptor_LSDDetector_detect_Mat_VectorOfKeyLine_int_int_Mat(self.as_raw_LSDDetector(), image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyLine(), scale, num_octaves, mask.as_raw_Mat()) }.into_result()
    }
    
    /// ## Parameters
    /// * images: input images
    /// * keylines: set of vectors that will store extracted lines for one or more images
    /// * scale: scale factor used in pyramids generation
    /// * numOctaves: number of octaves inside pyramid
    /// * masks: vector of mask matrices to detect only KeyLines of interest from each input image
    ///
    /// ## C++ default parameters
    /// * masks: std::vector<Mat>()
    pub fn detect_multiple(&self, images: &types::VectorOfMat, keylines: &mut types::VectorOfVectorOfKeyLine, scale: i32, num_octaves: i32, masks: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_line_descriptor_LSDDetector_detect_const_VectorOfMat_VectorOfVectorOfKeyLine_int_int_VectorOfMat(self.as_raw_LSDDetector(), images.as_raw_VectorOfMat(), keylines.as_raw_VectorOfVectorOfKeyLine(), scale, num_octaves, masks.as_raw_VectorOfMat()) }.into_result()
    }
    
}

impl LSDParam {
    pub fn default() -> Result<crate::line_descriptor::LSDParam> {
        unsafe { sys::cv_line_descriptor_LSDParam_LSDParam() }.into_result()
    }
    
}


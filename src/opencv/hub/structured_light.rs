//! # Structured Light API
//!
//! Structured light is considered one of the most effective techniques to acquire 3D models.
//! This technique is based on projecting a light pattern and capturing the illuminated scene
//! from one or more points of view. Since the pattern is coded, correspondences between image
//! points and points of the projected pattern can be quickly found and 3D information easily
//! retrieved.
//!
//! One of the most commonly exploited coding strategies is based on trmatime-multiplexing. In this
//! case, a set of patterns  are successively projected onto the measuring surface.
//! The codeword for a given pixel is usually formed by  the sequence of illuminance values for that
//! pixel across the projected patterns. Thus, the codification is called  temporal because the bits
//! of the codewords are multiplexed in time [pattern](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_pattern) .
//!
//! In this module a time-multiplexing coding strategy based on Gray encoding is implemented following the
//! (stereo) approach described in 3DUNDERWORLD algorithm [UNDERWORLD](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_UNDERWORLD) .
//! For more details, see @ref tutorial_structured_light.
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

/// Kyriakos Herakleous, Charalambos Poullis. "3DUNDERWORLD-SLS: An Open-Source Structured-Light Scanning System for Rapid Geometry Acquisition", arXiv preprint arXiv:1406.6595 (2014).
pub const DECODE_3D_UNDERWORLD: i32 = 0;
pub const FAPS: i32 = 2;
pub const FTP: i32 = 0;
pub const PSP: i32 = 1;

// Generating impl for trait crate::structured_light::GrayCodePattern
/// Class implementing the Gray-code pattern, based on [UNDERWORLD](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_UNDERWORLD).
///
///  The generation of the pattern images is performed with Gray encoding using the traditional white and black colors.
///
///  The information about the two image axes x, y is encoded separately into two different pattern sequences.
///  A projector P with resolution (P_res_x, P_res_y) will result in Ncols = log 2 (P_res_x) encoded pattern images representing the columns, and
///  in Nrows = log 2 (P_res_y) encoded pattern images representing the rows.
///  For example a projector with resolution 1024x768 will result in Ncols = 10 and Nrows = 10.
///
///  However, the generated pattern sequence consists of both regular color and color-inverted images: inverted pattern images are images
///  with the same structure as the original but with inverted colors.
///  This provides an effective method for easily determining the intensity value of each pixel when it is lit (highest value) and
///  when it is not lit (lowest value). So for a a projector with resolution 1024x768, the number of pattern images will be Ncols * 2 + Nrows * 2 = 40.
pub trait GrayCodePattern: crate::structured_light::StructuredLightPattern {
    fn as_raw_GrayCodePattern(&self) -> *mut c_void;
    /// Get the number of pattern images needed for the graycode pattern.
    ///
    /// ## Returns
    /// The number of pattern images needed for the graycode pattern.
    fn get_number_of_pattern_images(&self) -> Result<size_t> {
        unsafe { sys::cv_structured_light_GrayCodePattern_getNumberOfPatternImages_const(self.as_raw_GrayCodePattern()) }.into_result()
    }
    
    /// Sets the value for white threshold, needed for decoding.
    ///
    ///  White threshold is a number between 0-255 that represents the minimum brightness difference required for valid pixels, between the graycode pattern and its inverse images; used in getProjPixel method.
    ///
    /// ## Parameters
    /// * value: The desired white threshold value.
    fn set_white_threshold(&mut self, value: size_t) -> Result<()> {
        unsafe { sys::cv_structured_light_GrayCodePattern_setWhiteThreshold_size_t(self.as_raw_GrayCodePattern(), value) }.into_result()
    }
    
    /// Sets the value for black threshold, needed for decoding (shadowsmasks computation).
    ///
    ///  Black threshold is a number between 0-255 that represents the minimum brightness difference required for valid pixels, between the fully illuminated (white) and the not illuminated images (black); used in computeShadowMasks method.
    ///
    /// ## Parameters
    /// * value: The desired black threshold value.
    fn set_black_threshold(&mut self, value: size_t) -> Result<()> {
        unsafe { sys::cv_structured_light_GrayCodePattern_setBlackThreshold_size_t(self.as_raw_GrayCodePattern(), value) }.into_result()
    }
    
    /// Generates the all-black and all-white images needed for shadowMasks computation.
    ///
    ///  To identify shadow regions, the regions of two images where the pixels are not lit by projector's light and thus where there is not coded information,
    ///  the 3DUNDERWORLD algorithm computes a shadow mask for the two cameras views, starting from a white and a black images captured by each camera.
    ///  This method generates these two additional images to project.
    ///
    /// ## Parameters
    /// * blackImage: The generated all-black CV_8U image, at projector's resolution.
    /// * whiteImage: The generated all-white CV_8U image, at projector's resolution.
    fn get_images_for_shadow_masks(&self, black_image: &mut dyn core::ToInputOutputArray, white_image: &mut dyn core::ToInputOutputArray) -> Result<()> {
        input_output_array_arg!(black_image);
        input_output_array_arg!(white_image);
        unsafe { sys::cv_structured_light_GrayCodePattern_getImagesForShadowMasks_const__InputOutputArray__InputOutputArray(self.as_raw_GrayCodePattern(), black_image.as_raw__InputOutputArray(), white_image.as_raw__InputOutputArray()) }.into_result()
    }
    
    /// For a (x,y) pixel of a camera returns the corresponding projector pixel.
    ///
    ///  The function decodes each pixel in the pattern images acquired by a camera into their corresponding decimal numbers representing the projector's column and row,
    ///  providing a mapping between camera's and projector's pixel.
    ///
    /// ## Parameters
    /// * patternImages: The pattern images acquired by the camera, stored in a grayscale vector < Mat >.
    /// * x: x coordinate of the image pixel.
    /// * y: y coordinate of the image pixel.
    /// * projPix: Projector's pixel corresponding to the camera's pixel: projPix.x and projPix.y are the image coordinates of the projector's pixel corresponding to the pixel being decoded in a camera.
    fn get_proj_pixel(&self, pattern_images: &dyn core::ToInputArray, x: i32, y: i32, proj_pix: &mut core::Point) -> Result<bool> {
        input_array_arg!(pattern_images);
        unsafe { sys::cv_structured_light_GrayCodePattern_getProjPixel_const__InputArray_int_int_Point(self.as_raw_GrayCodePattern(), pattern_images.as_raw__InputArray(), x, y, proj_pix) }.into_result()
    }
    
}

impl dyn GrayCodePattern + '_ {
    /// Constructor
    /// ## Parameters
    /// * parameters: GrayCodePattern parameters GrayCodePattern::Params: the width and the height of the projector.
    ///
    /// ## C++ default parameters
    /// * parameters: GrayCodePattern::Params()
    pub fn create(parameters: &crate::structured_light::GrayCodePattern_Params) -> Result<types::PtrOfGrayCodePattern> {
        unsafe { sys::cv_structured_light_GrayCodePattern_create_Params(parameters.as_raw_GrayCodePattern_Params()) }.into_result().map(|ptr| types::PtrOfGrayCodePattern { ptr })
    }
    
    pub fn create_1(width: i32, height: i32) -> Result<types::PtrOfGrayCodePattern> {
        unsafe { sys::cv_structured_light_GrayCodePattern_create_int_int(width, height) }.into_result().map(|ptr| types::PtrOfGrayCodePattern { ptr })
    }
    
}

// boxed class cv::structured_light::GrayCodePattern::Params
/// Parameters of StructuredLightPattern constructor.
/// ## Parameters
/// * width: Projector's width. Default value is 1024.
/// * height: Projector's height. Default value is 768.
pub struct GrayCodePattern_Params {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for GrayCodePattern_Params {
    fn drop(&mut self) {
        unsafe { sys::cv_GrayCodePattern_Params_delete(self.ptr) };
    }
}

impl GrayCodePattern_Params {
    #[inline(always)] pub fn as_raw_GrayCodePattern_Params(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for GrayCodePattern_Params {}

impl GrayCodePattern_Params {
    pub fn default() -> Result<crate::structured_light::GrayCodePattern_Params> {
        unsafe { sys::cv_structured_light_GrayCodePattern_Params_Params() }.into_result().map(|ptr| crate::structured_light::GrayCodePattern_Params { ptr })
    }
    
}

// Generating impl for trait crate::structured_light::SinusoidalPattern
/// Class implementing Fourier transform profilometry (FTP) , phase-shifting profilometry (PSP)
/// and Fourier-assisted phase-shifting profilometry (FAPS) based on [faps](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_faps).
///
/// This class generates sinusoidal patterns that can be used with FTP, PSP and FAPS.
pub trait SinusoidalPattern: crate::structured_light::StructuredLightPattern {
    fn as_raw_SinusoidalPattern(&self) -> *mut c_void;
    /// Compute a wrapped phase map from sinusoidal patterns.
    /// ## Parameters
    /// * patternImages: Input data to compute the wrapped phase map.
    /// * wrappedPhaseMap: Wrapped phase map obtained through one of the three methods.
    /// * shadowMask: Mask used to discard shadow regions.
    /// * fundamental: Fundamental matrix used to compute epipolar lines and ease the matching step.
    ///
    /// ## C++ default parameters
    /// * shadow_mask: noArray()
    /// * fundamental: noArray()
    fn compute_phase_map(&mut self, pattern_images: &dyn core::ToInputArray, wrapped_phase_map: &mut dyn core::ToOutputArray, shadow_mask: &mut dyn core::ToOutputArray, fundamental: &dyn core::ToInputArray) -> Result<()> {
        input_array_arg!(pattern_images);
        output_array_arg!(wrapped_phase_map);
        output_array_arg!(shadow_mask);
        input_array_arg!(fundamental);
        unsafe { sys::cv_structured_light_SinusoidalPattern_computePhaseMap__InputArray__OutputArray__OutputArray__InputArray(self.as_raw_SinusoidalPattern(), pattern_images.as_raw__InputArray(), wrapped_phase_map.as_raw__OutputArray(), shadow_mask.as_raw__OutputArray(), fundamental.as_raw__InputArray()) }.into_result()
    }
    
    /// Unwrap the wrapped phase map to remove phase ambiguities.
    /// ## Parameters
    /// * wrappedPhaseMap: The wrapped phase map computed from the pattern.
    /// * unwrappedPhaseMap: The unwrapped phase map used to find correspondences between the two devices.
    /// * camSize: Resolution of the camera.
    /// * shadowMask: Mask used to discard shadow regions.
    ///
    /// ## C++ default parameters
    /// * shadow_mask: noArray()
    fn unwrap_phase_map(&mut self, wrapped_phase_map: &dyn core::ToInputArray, unwrapped_phase_map: &mut dyn core::ToOutputArray, cam_size: core::Size, shadow_mask: &dyn core::ToInputArray) -> Result<()> {
        input_array_arg!(wrapped_phase_map);
        output_array_arg!(unwrapped_phase_map);
        input_array_arg!(shadow_mask);
        unsafe { sys::cv_structured_light_SinusoidalPattern_unwrapPhaseMap__InputArray__OutputArray_Size__InputArray(self.as_raw_SinusoidalPattern(), wrapped_phase_map.as_raw__InputArray(), unwrapped_phase_map.as_raw__OutputArray(), cam_size, shadow_mask.as_raw__InputArray()) }.into_result()
    }
    
    /// Find correspondences between the two devices thanks to unwrapped phase maps.
    /// ## Parameters
    /// * projUnwrappedPhaseMap: Projector's unwrapped phase map.
    /// * camUnwrappedPhaseMap: Camera's unwrapped phase map.
    /// * matches: Images used to display correspondences map.
    fn find_pro_cam_matches(&mut self, proj_unwrapped_phase_map: &dyn core::ToInputArray, cam_unwrapped_phase_map: &dyn core::ToInputArray, matches: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(proj_unwrapped_phase_map);
        input_array_arg!(cam_unwrapped_phase_map);
        output_array_arg!(matches);
        unsafe { sys::cv_structured_light_SinusoidalPattern_findProCamMatches__InputArray__InputArray__OutputArray(self.as_raw_SinusoidalPattern(), proj_unwrapped_phase_map.as_raw__InputArray(), cam_unwrapped_phase_map.as_raw__InputArray(), matches.as_raw__OutputArray()) }.into_result()
    }
    
    /// compute the data modulation term.
    /// ## Parameters
    /// * patternImages: captured images with projected patterns.
    /// * dataModulationTerm: Mat where the data modulation term is saved.
    /// * shadowMask: Mask used to discard shadow regions.
    fn compute_data_modulation_term(&mut self, pattern_images: &dyn core::ToInputArray, data_modulation_term: &mut dyn core::ToOutputArray, shadow_mask: &dyn core::ToInputArray) -> Result<()> {
        input_array_arg!(pattern_images);
        output_array_arg!(data_modulation_term);
        input_array_arg!(shadow_mask);
        unsafe { sys::cv_structured_light_SinusoidalPattern_computeDataModulationTerm__InputArray__OutputArray__InputArray(self.as_raw_SinusoidalPattern(), pattern_images.as_raw__InputArray(), data_modulation_term.as_raw__OutputArray(), shadow_mask.as_raw__InputArray()) }.into_result()
    }
    
}

impl dyn SinusoidalPattern + '_ {
    /// Constructor.
    /// ## Parameters
    /// * parameters: SinusoidalPattern parameters SinusoidalPattern::Params: width, height of the projector and patterns parameters.
    ///
    /// ## C++ default parameters
    /// * parameters: makePtr<SinusoidalPattern::Params>()
    pub fn create(parameters: &types::PtrOfParams) -> Result<types::PtrOfSinusoidalPattern> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_create_PtrOfParams(parameters.as_raw_PtrOfParams()) }.into_result().map(|ptr| types::PtrOfSinusoidalPattern { ptr })
    }
    
}

// boxed class cv::structured_light::SinusoidalPattern::Params
/// Parameters of SinusoidalPattern constructor
/// ## Parameters
/// * width: Projector's width.
/// * height: Projector's height.
/// * nbrOfPeriods: Number of period along the patterns direction.
/// * shiftValue: Phase shift between two consecutive patterns.
/// * methodId: Allow to choose between FTP, PSP and FAPS.
/// * nbrOfPixelsBetweenMarkers: Number of pixels between two consecutive markers on the same row.
/// * setMarkers: Allow to set markers on the patterns.
/// * markersLocation: vector used to store markers location on the patterns.
pub struct SinusoidalPattern_Params {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SinusoidalPattern_Params {
    fn drop(&mut self) {
        unsafe { sys::cv_SinusoidalPattern_Params_delete(self.ptr) };
    }
}

impl SinusoidalPattern_Params {
    #[inline(always)] pub fn as_raw_SinusoidalPattern_Params(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SinusoidalPattern_Params {}

impl SinusoidalPattern_Params {
    pub fn width(&self) -> Result<i32> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_width_const(self.as_raw_SinusoidalPattern_Params()) }.into_result()
    }
    
    pub fn set_width(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_set_width_int(self.as_raw_SinusoidalPattern_Params(), val) }.into_result()
    }
    
    pub fn height(&self) -> Result<i32> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_height_const(self.as_raw_SinusoidalPattern_Params()) }.into_result()
    }
    
    pub fn set_height(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_set_height_int(self.as_raw_SinusoidalPattern_Params(), val) }.into_result()
    }
    
    pub fn nbr_of_periods(&self) -> Result<i32> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_nbrOfPeriods_const(self.as_raw_SinusoidalPattern_Params()) }.into_result()
    }
    
    pub fn set_nbr_of_periods(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_set_nbrOfPeriods_int(self.as_raw_SinusoidalPattern_Params(), val) }.into_result()
    }
    
    pub fn shift_value(&self) -> Result<f32> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_shiftValue_const(self.as_raw_SinusoidalPattern_Params()) }.into_result()
    }
    
    pub fn set_shift_value(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_set_shiftValue_float(self.as_raw_SinusoidalPattern_Params(), val) }.into_result()
    }
    
    pub fn method_id(&self) -> Result<i32> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_methodId_const(self.as_raw_SinusoidalPattern_Params()) }.into_result()
    }
    
    pub fn set_method_id(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_set_methodId_int(self.as_raw_SinusoidalPattern_Params(), val) }.into_result()
    }
    
    pub fn nbr_of_pixels_between_markers(&self) -> Result<i32> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_nbrOfPixelsBetweenMarkers_const(self.as_raw_SinusoidalPattern_Params()) }.into_result()
    }
    
    pub fn set_nbr_of_pixels_between_markers(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_set_nbrOfPixelsBetweenMarkers_int(self.as_raw_SinusoidalPattern_Params(), val) }.into_result()
    }
    
    pub fn horizontal(&self) -> Result<bool> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_horizontal_const(self.as_raw_SinusoidalPattern_Params()) }.into_result()
    }
    
    pub fn set_horizontal(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_set_horizontal_bool(self.as_raw_SinusoidalPattern_Params(), val) }.into_result()
    }
    
    pub fn set_markers(&self) -> Result<bool> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_setMarkers_const(self.as_raw_SinusoidalPattern_Params()) }.into_result()
    }
    
    pub fn set_set_markers(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_set_setMarkers_bool(self.as_raw_SinusoidalPattern_Params(), val) }.into_result()
    }
    
    pub fn default() -> Result<crate::structured_light::SinusoidalPattern_Params> {
        unsafe { sys::cv_structured_light_SinusoidalPattern_Params_Params() }.into_result().map(|ptr| crate::structured_light::SinusoidalPattern_Params { ptr })
    }
    
}

// Generating impl for trait crate::structured_light::StructuredLightPattern
/// Abstract base class for generating and decoding structured light patterns.
pub trait StructuredLightPattern: core::AlgorithmTrait {
    fn as_raw_StructuredLightPattern(&self) -> *mut c_void;
    /// Generates the structured light pattern to project.
    ///
    /// ## Parameters
    /// * patternImages: The generated pattern: a vector<Mat>, in which each image is a CV_8U Mat at projector's resolution.
    fn generate(&mut self, pattern_images: &mut dyn core::ToOutputArray) -> Result<bool> {
        output_array_arg!(pattern_images);
        unsafe { sys::cv_structured_light_StructuredLightPattern_generate__OutputArray(self.as_raw_StructuredLightPattern(), pattern_images.as_raw__OutputArray()) }.into_result()
    }
    
    /// Decodes the structured light pattern, generating a disparity map
    ///
    /// ## Parameters
    /// * patternImages: The acquired pattern images to decode (vector<vector<Mat>>), loaded as grayscale and previously rectified.
    /// * disparityMap: The decoding result: a CV_64F Mat at image resolution, storing the computed disparity map.
    /// * blackImages: The all-black images needed for shadowMasks computation.
    /// * whiteImages: The all-white images needed for shadowMasks computation.
    /// * flags: Flags setting decoding algorithms. Default: DECODE_3D_UNDERWORLD.
    ///
    /// Note: All the images must be at the same resolution.
    ///
    /// ## C++ default parameters
    /// * black_images: noArray()
    /// * white_images: noArray()
    /// * flags: DECODE_3D_UNDERWORLD
    fn decode(&self, pattern_images: &types::VectorOfVectorOfMat, disparity_map: &mut dyn core::ToOutputArray, black_images: &dyn core::ToInputArray, white_images: &dyn core::ToInputArray, flags: i32) -> Result<bool> {
        output_array_arg!(disparity_map);
        input_array_arg!(black_images);
        input_array_arg!(white_images);
        unsafe { sys::cv_structured_light_StructuredLightPattern_decode_const_VectorOfVectorOfMat__OutputArray__InputArray__InputArray_int(self.as_raw_StructuredLightPattern(), pattern_images.as_raw_VectorOfVectorOfMat(), disparity_map.as_raw__OutputArray(), black_images.as_raw__InputArray(), white_images.as_raw__InputArray(), flags) }.into_result()
    }
    
}


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
//! image, we first compute a non-binary descriptor as described in [LBD](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_LBD) . Such algorithm works on
//! lines extracted using EDLine detector, as explained in [EDL](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_EDL) . Given a line, we consider a
//! rectangular region centered at it and called *line support region (LSR)*. Such region is divided
//! into a set of bands ![inline formula](https://latex.codecogs.com/png.latex?%5C%7BB%5F1%2C%20B%5F2%2C%20%2E%2E%2E%2C%20B%5Fm%5C%7D), whose length equals the one of line.
//!
//! If we indicate with ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bd%7D%5FL) the direction of line, the orthogonal and clockwise direction to line
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bd%7D%5F%7B%5Cperp%7D) can be determined; these two directions, are used to construct a reference frame
//! centered in the middle point of line. The gradients of pixels ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bg%27%7D) inside LSR can be projected
//! to the newly determined frame, obtaining their local equivalent
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bg%27%7D%20%3D%20%28%5Cbf%7Bg%7D%5ET%20%5Ccdot%20%5Cbf%7Bd%7D%5F%7B%5Cperp%7D%2C%20%5Cbf%7Bg%7D%5ET%20%5Ccdot%20%5Cbf%7Bd%7D%5FL%29%5ET%20%5Ctriangleq%20%28%5Cbf%7Bg%27%7D%5F%7Bd%5F%7B%5Cperp%7D%7D%2C%20%5Cbf%7Bg%27%7D%5F%7Bd%5FL%7D%29%5ET).
//!
//! Later on, a Gaussian function is applied to all LSR's pixels along ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bd%7D%5F%5Cperp) direction; first,
//! we assign a global weighting coefficient ![inline formula](https://latex.codecogs.com/png.latex?f%5Fg%28i%29%20%3D%20%281%2F%5Csqrt%7B2%5Cpi%7D%5Csigma%5Fg%29e%5E%7B%2Dd%5E2%5Fi%2F2%5Csigma%5E2%5Fg%7D) to
//! *i*-th row in LSR, where ![inline formula](https://latex.codecogs.com/png.latex?d%5Fi) is the distance of *i*-th row from the center row in LSR,
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Csigma%5Fg%20%3D%200%2E5%28m%20%5Ccdot%20w%20%2D%201%29) and ![inline formula](https://latex.codecogs.com/png.latex?w) is the width of bands (the same for every band). Secondly,
//! considering a band ![inline formula](https://latex.codecogs.com/png.latex?B%5Fj) and its neighbor bands ![inline formula](https://latex.codecogs.com/png.latex?B%5F%7Bj%2D1%7D%2C%20B%5F%7Bj%2B1%7D), we assign a local weighting
//! ![inline formula](https://latex.codecogs.com/png.latex?F%5Fl%28k%29%20%3D%20%281%2F%5Csqrt%7B2%5Cpi%7D%5Csigma%5Fl%29e%5E%7B%2Dd%27%5E2%5Fk%2F2%5Csigma%5Fl%5E2%7D), where ![inline formula](https://latex.codecogs.com/png.latex?d%27%5Fk) is the distance of *k*-th
//! row from the center row in ![inline formula](https://latex.codecogs.com/png.latex?B%5Fj) and ![inline formula](https://latex.codecogs.com/png.latex?%5Csigma%5Fl%20%3D%20w). Using the global and local weights, we obtain,
//! at the same time, the reduction of role played by gradients far from line and of boundary effect,
//! respectively.
//!
//! Each band ![inline formula](https://latex.codecogs.com/png.latex?B%5Fj) in LSR has an associated *band descriptor(BD)* which is computed considering
//! previous and next band (top and bottom bands are ignored when computing descriptor for first and
//! last band). Once each band has been assignen its BD, the LBD descriptor of line is simply given by
//!
//! ![block formula](https://latex.codecogs.com/png.latex?LBD%20%3D%20%28BD%5F1%5ET%2C%20BD%5F2%5ET%2C%20%2E%2E%2E%20%2C%20BD%5ET%5Fm%29%5ET%2E)
//!
//! To compute a band descriptor ![inline formula](https://latex.codecogs.com/png.latex?B%5Fj), each *k*-th row in it is considered and the gradients in such
//! row are accumulated:
//!
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bmatrix%7D%20%5Cbf%7BV1%7D%5Ek%5Fj%20%3D%20%5Clambda%20%5Csum%5Climits%5F%7B%5Cbf%7Bg%7D%27%5F%7Bd%5F%5Cperp%7D%3E0%7D%5Cbf%7Bg%7D%27%5F%7Bd%5F%5Cperp%7D%2C%20%26%20%20%5Cbf%7BV2%7D%5Ek%5Fj%20%3D%20%5Clambda%20%5Csum%5Climits%5F%7B%5Cbf%7Bg%7D%27%5F%7Bd%5F%5Cperp%7D%3C0%7D%20%2D%5Cbf%7Bg%7D%27%5F%7Bd%5F%5Cperp%7D%2C%20%5C%5C%20%5Cbf%7BV3%7D%5Ek%5Fj%20%3D%20%5Clambda%20%5Csum%5Climits%5F%7B%5Cbf%7Bg%7D%27%5F%7Bd%5FL%7D%3E0%7D%5Cbf%7Bg%7D%27%5F%7Bd%5FL%7D%2C%20%26%20%5Cbf%7BV4%7D%5Ek%5Fj%20%3D%20%5Clambda%20%5Csum%5Climits%5F%7B%5Cbf%7Bg%7D%27%5F%7Bd%5FL%7D%3C0%7D%20%2D%5Cbf%7Bg%7D%27%5F%7Bd%5FL%7D%5Cend%7Bmatrix%7D%2E)
//!
//! with ![inline formula](https://latex.codecogs.com/png.latex?%5Clambda%20%3D%20f%5Fg%28k%29f%5Fl%28k%29).
//!
//! By stacking previous results, we obtain the *band description matrix (BDM)*
//!
//! ![block formula](https://latex.codecogs.com/png.latex?BDM%5Fj%20%3D%20%5Cleft%28%5Cbegin%7Bmatrix%7D%20%5Cbf%7BV1%7D%5Fj%5E1%20%26%20%5Cbf%7BV1%7D%5Fj%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV1%7D%5Fj%5En%20%5C%5C%20%5Cbf%7BV2%7D%5Fj%5E1%20%26%20%5Cbf%7BV2%7D%5Fj%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV2%7D%5Fj%5En%20%5C%5C%20%5Cbf%7BV3%7D%5Fj%5E1%20%26%20%5Cbf%7BV3%7D%5Fj%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV3%7D%5Fj%5En%20%5C%5C%20%5Cbf%7BV4%7D%5Fj%5E1%20%26%20%5Cbf%7BV4%7D%5Fj%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV4%7D%5Fj%5En%20%5Cend%7Bmatrix%7D%20%5Cright%29%20%5Cin%20%5Cmathbb%7BR%7D%5E%7B4%5Ctimes%20n%7D%2C)
//!
//! with ![inline formula](https://latex.codecogs.com/png.latex?n) the number of rows in band ![inline formula](https://latex.codecogs.com/png.latex?B%5Fj):
//!
//! ![block formula](https://latex.codecogs.com/png.latex?n%20%3D%20%5Cbegin%7Bcases%7D%202w%2C%20%26%20j%20%3D%201%7C%7Cm%3B%20%5C%5C%203w%2C%20%26%20%5Cmbox%7Belse%7D%2E%20%5Cend%7Bcases%7D)
//!
//! Each ![inline formula](https://latex.codecogs.com/png.latex?BD%5Fj) can be obtained using the standard deviation vector ![inline formula](https://latex.codecogs.com/png.latex?S%5Fj) and mean vector ![inline formula](https://latex.codecogs.com/png.latex?M%5Fj) of
//! ![inline formula](https://latex.codecogs.com/png.latex?BDM%5FJ). Thus, finally:
//!
//! ![block formula](https://latex.codecogs.com/png.latex?LBD%20%3D%20%28M%5F1%5ET%2C%20S%5F1%5ET%2C%20M%5F2%5ET%2C%20S%5F2%5ET%2C%20%5Cldots%2C%20M%5Fm%5ET%2C%20S%5Fm%5ET%29%5ET%20%5Cin%20%5Cmathbb%7BR%7D%5E%7B8m%7D)
//!
//! Once the LBD has been obtained, it must be converted into a binary form. For such purpose, we
//! consider 32 possible pairs of BD inside it; each couple of BD is compared bit by bit and comparison
//! generates an 8 bit string. Concatenating 32 comparison strings, we get the 256-bit final binary
//! representation of a single LBD.
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{BinaryDescriptorMatcherTrait, BinaryDescriptorMatcherTraitConst, BinaryDescriptorTrait, BinaryDescriptorTraitConst, BinaryDescriptor_ParamsTrait, BinaryDescriptor_ParamsTraitConst, LSDDetectorTrait, LSDDetectorTraitConst};
}

/// Output image matrix will be created (Mat::create),
/// i.e. existing memory of output image may be reused.
/// Two source images, matches, and single keylines
/// will be drawn.
// DEFAULT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1361
pub const DrawLinesMatchesFlags_DEFAULT: i32 = 0;
/// Output image matrix will not be
/// created (using Mat::create). Matches will be drawn
/// on existing content of output image.
// DRAW_OVER_OUTIMG /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1365
pub const DrawLinesMatchesFlags_DRAW_OVER_OUTIMG: i32 = 1;
/// Single keylines will not be drawn.
// NOT_DRAW_SINGLE_LINES /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1368
pub const DrawLinesMatchesFlags_NOT_DRAW_SINGLE_LINES: i32 = 2;
// MLN10 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:449
pub const MLN10: f64 = 2.30258509299404568402;
// RELATIVE_ERROR_FACTOR /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:448
pub const RELATIVE_ERROR_FACTOR: f64 = 100.0;
// (unnamed enum at /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1359:12) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1359
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DrawLinesMatchesFlags {
	/// Output image matrix will be created (Mat::create),
	/// i.e. existing memory of output image may be reused.
	/// Two source images, matches, and single keylines
	/// will be drawn.
	DEFAULT = 0,
	/// Output image matrix will not be
	/// created (using Mat::create). Matches will be drawn
	/// on existing content of output image.
	DRAW_OVER_OUTIMG = 1,
	/// Single keylines will not be drawn.
	NOT_DRAW_SINGLE_LINES = 2,
}

impl TryFrom<i32> for DrawLinesMatchesFlags {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::DEFAULT),
			1 => Ok(Self::DRAW_OVER_OUTIMG),
			2 => Ok(Self::NOT_DRAW_SINGLE_LINES),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::line_descriptor::DrawLinesMatchesFlags"))),
		}
	}
}

opencv_type_enum! { crate::line_descriptor::DrawLinesMatchesFlags }

// UINT16 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:67
pub type UINT16 = u16;
// UINT32 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:66
pub type UINT32 = u32;
// UINT64 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:65
pub type UINT64 = u64;
// UINT8 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:68
pub type UINT8 = u8;
/// Draws keylines.
///
/// ## Parameters
/// * image: input image
/// * keylines: keylines to be drawn
/// * outImage: output image to draw on
/// * color: color of lines to be drawn (if set to defaul value, color is chosen randomly)
/// * flags: drawing flags
///
/// ## Note
/// This alternative version of [draw_keylines] function uses the following default values for its arguments:
/// * color: Scalar::all(-1)
/// * flags: DrawLinesMatchesFlags::DEFAULT
// cv::line_descriptor::drawKeylines(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1401
// ("cv::line_descriptor::drawKeylines", vec![(pred!(mut, ["image", "keylines", "outImage"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*"]), _)]),
#[inline]
pub fn draw_keylines_def(image: &impl core::MatTraitConst, keylines: &core::Vector<crate::line_descriptor::KeyLine>, out_image: &mut impl core::MatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_line_descriptor_drawKeylines_const_MatR_const_vectorLKeyLineGR_MatR(image.as_raw_Mat(), keylines.as_raw_VectorOfKeyLine(), out_image.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// * color: Scalar::all(-1)
/// * flags: DrawLinesMatchesFlags::DEFAULT
// drawKeylines(const Mat &, const std::vector<KeyLine> &, Mat &, const Scalar &, int)(TraitClass, CppPassByVoidPtr, TraitClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1401
// ("cv::line_descriptor::drawKeylines", vec![(pred!(mut, ["image", "keylines", "outImage", "color", "flags"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*", "const cv::Scalar*", "int"]), _)]),
#[inline]
pub fn draw_keylines(image: &impl core::MatTraitConst, keylines: &core::Vector<crate::line_descriptor::KeyLine>, out_image: &mut impl core::MatTrait, color: core::Scalar, flags: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_line_descriptor_drawKeylines_const_MatR_const_vectorLKeyLineGR_MatR_const_ScalarR_int(image.as_raw_Mat(), keylines.as_raw_VectorOfKeyLine(), out_image.as_raw_mut_Mat(), &color, flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// ## Note
/// This alternative version of [draw_line_matches] function uses the following default values for its arguments:
/// * match_color: Scalar::all(-1)
/// * single_line_color: Scalar::all(-1)
/// * matches_mask: std::vector<char>()
/// * flags: DrawLinesMatchesFlags::DEFAULT
// cv::line_descriptor::drawLineMatches(TraitClass, CppPassByVoidPtr, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1388
// ("cv::line_descriptor::drawLineMatches", vec![(pred!(mut, ["img1", "keylines1", "img2", "keylines2", "matches1to2", "outImg"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const std::vector<cv::DMatch>*", "cv::Mat*"]), _)]),
#[inline]
pub fn draw_line_matches_def(img1: &impl core::MatTraitConst, keylines1: &core::Vector<crate::line_descriptor::KeyLine>, img2: &impl core::MatTraitConst, keylines2: &core::Vector<crate::line_descriptor::KeyLine>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl core::MatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_line_descriptor_drawLineMatches_const_MatR_const_vectorLKeyLineGR_const_MatR_const_vectorLKeyLineGR_const_vectorLDMatchGR_MatR(img1.as_raw_Mat(), keylines1.as_raw_VectorOfKeyLine(), img2.as_raw_Mat(), keylines2.as_raw_VectorOfKeyLine(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// * match_color: Scalar::all(-1)
/// * single_line_color: Scalar::all(-1)
/// * matches_mask: std::vector<char>()
/// * flags: DrawLinesMatchesFlags::DEFAULT
// drawLineMatches(const Mat &, const std::vector<KeyLine> &, const Mat &, const std::vector<KeyLine> &, const std::vector<DMatch> &, Mat &, const Scalar &, const Scalar &, const std::vector<char> &, int)(TraitClass, CppPassByVoidPtr, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass, SimpleClass, SimpleClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1388
// ("cv::line_descriptor::drawLineMatches", vec![(pred!(mut, ["img1", "keylines1", "img2", "keylines2", "matches1to2", "outImg", "matchColor", "singleLineColor", "matchesMask", "flags"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const std::vector<cv::DMatch>*", "cv::Mat*", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<char>*", "int"]), _)]),
#[inline]
pub fn draw_line_matches(img1: &impl core::MatTraitConst, keylines1: &core::Vector<crate::line_descriptor::KeyLine>, img2: &impl core::MatTraitConst, keylines2: &core::Vector<crate::line_descriptor::KeyLine>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl core::MatTrait, match_color: core::Scalar, single_line_color: core::Scalar, matches_mask: &core::Vector<c_char>, flags: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_line_descriptor_drawLineMatches_const_MatR_const_vectorLKeyLineGR_const_MatR_const_vectorLKeyLineGR_const_vectorLDMatchGR_MatR_const_ScalarR_const_ScalarR_const_vectorLcharGR_int(img1.as_raw_Mat(), keylines1.as_raw_VectorOfKeyLine(), img2.as_raw_Mat(), keylines2.as_raw_VectorOfKeyLine(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw_mut_Mat(), &match_color, &single_line_color, matches_mask.as_raw_VectorOfc_char(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::line_descriptor::BinaryDescriptor]
// BinaryDescriptor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:182
pub trait BinaryDescriptorTraitConst: core::AlgorithmTraitConst {
	fn as_raw_BinaryDescriptor(&self) -> *const c_void;

	/// Store parameters to a FileStorage object
	///
	/// ## Parameters
	/// * fs: output FileStorage file
	// write(cv::FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:264
	// ("cv::line_descriptor::BinaryDescriptor::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_write_const_FileStorageR(self.as_raw_BinaryDescriptor(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Requires line detection
	///
	/// ## Parameters
	/// * image: input image
	/// * keypoints: vector that will store extracted lines for one or more images
	/// * mask: mask matrix to detect only KeyLines of interest
	///
	/// ## Overloaded parameters
	///
	///
	/// * images: input images
	/// * keylines: set of vectors that will store extracted lines for one or more images
	/// * masks: vector of mask matrices to detect only KeyLines of interest from each input image
	///
	/// ## C++ default parameters
	/// * masks: std::vector<Mat>()
	// detect(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, const std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:280
	// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(const, ["images", "keylines", "masks"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn detect(&self, images: &core::Vector<core::Mat>, keylines: &mut core::Vector<core::Vector<crate::line_descriptor::KeyLine>>, masks: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_const_vectorLMatGR(self.as_raw_BinaryDescriptor(), images.as_raw_VectorOfMat(), keylines.as_raw_mut_VectorOfVectorOfKeyLine(), masks.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
	///
	/// ## Parameters
	/// * images: input images
	/// * keylines: set of vectors that will store extracted lines for one or more images
	/// * masks: vector of mask matrices to detect only KeyLines of interest from each input image
	///
	/// ## Note
	/// This alternative version of [BinaryDescriptorTraitConst::detect] function uses the following default values for its arguments:
	/// * masks: std::vector<Mat>()
	// cv::line_descriptor::BinaryDescriptor::detect(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:280
	// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(const, ["images", "keylines"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*"]), _)]),
	#[inline]
	fn detect_def(&self, images: &core::Vector<core::Mat>, keylines: &mut core::Vector<core::Vector<crate::line_descriptor::KeyLine>>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR(self.as_raw_BinaryDescriptor(), images.as_raw_VectorOfMat(), keylines.as_raw_mut_VectorOfVectorOfKeyLine(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// compute(const Mat &, std::vector<KeyLine> &, Mat &, bool)(TraitClass, CppPassByVoidPtr, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:290
	// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["image", "keylines", "descriptors", "returnFloatDescr"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*", "bool"]), _)]),
	#[inline]
	fn compute(&self, image: &impl core::MatTraitConst, keylines: &mut core::Vector<crate::line_descriptor::KeyLine>, descriptors: &mut impl core::MatTrait, return_float_descr: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_compute_const_const_MatR_vectorLKeyLineGR_MatR_bool(self.as_raw_BinaryDescriptor(), image.as_raw_Mat(), keylines.as_raw_mut_VectorOfKeyLine(), descriptors.as_raw_mut_Mat(), return_float_descr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Requires descriptors computation
	///
	/// ## Parameters
	/// * image: input image
	/// * keylines: vector containing lines for which descriptors must be computed
	/// * descriptors: 
	/// * returnFloatDescr: flag (when set to true, original non-binary descriptors are returned)
	///
	/// ## Note
	/// This alternative version of [BinaryDescriptorTraitConst::compute] function uses the following default values for its arguments:
	/// * return_float_descr: false
	// cv::line_descriptor::BinaryDescriptor::compute(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:290
	// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["image", "keylines", "descriptors"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*"]), _)]),
	#[inline]
	fn compute_def(&self, image: &impl core::MatTraitConst, keylines: &mut core::Vector<crate::line_descriptor::KeyLine>, descriptors: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_compute_const_const_MatR_vectorLKeyLineGR_MatR(self.as_raw_BinaryDescriptor(), image.as_raw_Mat(), keylines.as_raw_mut_VectorOfKeyLine(), descriptors.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Requires descriptors computation
	///
	/// ## Parameters
	/// * image: input image
	/// * keylines: vector containing lines for which descriptors must be computed
	/// * descriptors: 
	/// * returnFloatDescr: flag (when set to true, original non-binary descriptors are returned)
	///
	/// ## Overloaded parameters
	///
	///
	/// * images: input images
	/// * keylines: set of vectors containing lines for which descriptors must be computed
	/// * descriptors: 
	/// * returnFloatDescr: flag (when set to true, original non-binary descriptors are returned)
	///
	/// ## C++ default parameters
	/// * return_float_descr: false
	// compute(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, std::vector<Mat> &, bool)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:299
	// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["images", "keylines", "descriptors", "returnFloatDescr"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "std::vector<cv::Mat>*", "bool"]), _)]),
	#[inline]
	fn compute_1(&self, images: &core::Vector<core::Mat>, keylines: &mut core::Vector<core::Vector<crate::line_descriptor::KeyLine>>, descriptors: &mut core::Vector<core::Mat>, return_float_descr: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_compute_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_vectorLMatGR_bool(self.as_raw_BinaryDescriptor(), images.as_raw_VectorOfMat(), keylines.as_raw_mut_VectorOfVectorOfKeyLine(), descriptors.as_raw_mut_VectorOfMat(), return_float_descr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
	///
	/// ## Parameters
	/// * images: input images
	/// * keylines: set of vectors containing lines for which descriptors must be computed
	/// * descriptors: 
	/// * returnFloatDescr: flag (when set to true, original non-binary descriptors are returned)
	///
	/// ## Note
	/// This alternative version of [BinaryDescriptorTraitConst::compute] function uses the following default values for its arguments:
	/// * return_float_descr: false
	// cv::line_descriptor::BinaryDescriptor::compute(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:299
	// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["images", "keylines", "descriptors"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn compute_def_1(&self, images: &core::Vector<core::Mat>, keylines: &mut core::Vector<core::Vector<crate::line_descriptor::KeyLine>>, descriptors: &mut core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_compute_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_vectorLMatGR(self.as_raw_BinaryDescriptor(), images.as_raw_VectorOfMat(), keylines.as_raw_mut_VectorOfVectorOfKeyLine(), descriptors.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Return descriptor size
	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:304
	// ("cv::line_descriptor::BinaryDescriptor::descriptorSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_descriptorSize_const(self.as_raw_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Return data type
	// descriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:308
	// ("cv::line_descriptor::BinaryDescriptor::descriptorType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptor_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_descriptorType_const(self.as_raw_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// returns norm mode
	// defaultNorm()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:312
	// ("cv::line_descriptor::BinaryDescriptor::defaultNorm", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn default_norm(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_defaultNorm_const(self.as_raw_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Define operator '()' to perform detection of KeyLines and computation of descriptors in a row.
	///
	/// ## Parameters
	/// * image: input image
	/// * mask: mask matrix to select which lines in KeyLines must be accepted among the ones
	/// extracted (used when *keylines* is not empty)
	/// * keylines: vector that contains input lines (when filled, the detection part will be skipped
	/// and input lines will be passed as input to the algorithm computing descriptors)
	/// * descriptors: matrix that will store final descriptors
	/// * useProvidedKeyLines: flag (when set to true, detection phase will be skipped and only
	/// computation of descriptors will be executed, using lines provided in *keylines*)
	/// * returnFloatDescr: flag (when set to true, original non-binary descriptors are returned)
	///
	/// ## C++ default parameters
	/// * use_provided_key_lines: false
	/// * return_float_descr: false
	// operator()(InputArray, InputArray, std::vector<KeyLine> &, OutputArray, bool, bool)(InputArray, InputArray, CppPassByVoidPtr, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:326
	// ("cv::line_descriptor::BinaryDescriptor::operator()", vec![(pred!(const, ["image", "mask", "keylines", "descriptors", "useProvidedKeyLines", "returnFloatDescr"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::line_descriptor::KeyLine>*", "const cv::_OutputArray*", "bool", "bool"]), _)]),
	#[inline]
	fn apply(&self, image: &impl ToInputArray, mask: &impl ToInputArray, keylines: &mut core::Vector<crate::line_descriptor::KeyLine>, descriptors: &mut impl ToOutputArray, use_provided_key_lines: bool, return_float_descr: bool) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_operator___const_const__InputArrayR_const__InputArrayR_vectorLKeyLineGR_const__OutputArrayR_bool_bool(self.as_raw_BinaryDescriptor(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keylines.as_raw_mut_VectorOfKeyLine(), descriptors.as_raw__OutputArray(), use_provided_key_lines, return_float_descr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Define operator '()' to perform detection of KeyLines and computation of descriptors in a row.
	///
	/// ## Parameters
	/// * image: input image
	/// * mask: mask matrix to select which lines in KeyLines must be accepted among the ones
	/// extracted (used when *keylines* is not empty)
	/// * keylines: vector that contains input lines (when filled, the detection part will be skipped
	/// and input lines will be passed as input to the algorithm computing descriptors)
	/// * descriptors: matrix that will store final descriptors
	/// * useProvidedKeyLines: flag (when set to true, detection phase will be skipped and only
	/// computation of descriptors will be executed, using lines provided in *keylines*)
	/// * returnFloatDescr: flag (when set to true, original non-binary descriptors are returned)
	///
	/// ## Note
	/// This alternative version of [BinaryDescriptorTraitConst::apply] function uses the following default values for its arguments:
	/// * use_provided_key_lines: false
	/// * return_float_descr: false
	// cv::line_descriptor::BinaryDescriptor::operator()(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:326
	// ("cv::line_descriptor::BinaryDescriptor::operator()", vec![(pred!(const, ["image", "mask", "keylines", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::line_descriptor::KeyLine>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn apply_def(&self, image: &impl ToInputArray, mask: &impl ToInputArray, keylines: &mut core::Vector<crate::line_descriptor::KeyLine>, descriptors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_operator___const_const__InputArrayR_const__InputArrayR_vectorLKeyLineGR_const__OutputArrayR(self.as_raw_BinaryDescriptor(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keylines.as_raw_mut_VectorOfKeyLine(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::line_descriptor::BinaryDescriptor]
pub trait BinaryDescriptorTrait: core::AlgorithmTrait + crate::line_descriptor::BinaryDescriptorTraitConst {
	fn as_raw_mut_BinaryDescriptor(&mut self) -> *mut c_void;

	/// Get current number of octaves
	// getNumOfOctaves()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:234
	// ("cv::line_descriptor::BinaryDescriptor::getNumOfOctaves", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_num_of_octaves(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_getNumOfOctaves(self.as_raw_mut_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set number of octaves
	/// ## Parameters
	/// * octaves: number of octaves
	// setNumOfOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:238
	// ("cv::line_descriptor::BinaryDescriptor::setNumOfOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	#[inline]
	fn set_num_of_octaves(&mut self, octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_setNumOfOctaves_int(self.as_raw_mut_BinaryDescriptor(), octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get current width of bands
	// getWidthOfBand()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:241
	// ("cv::line_descriptor::BinaryDescriptor::getWidthOfBand", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_width_of_band(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_getWidthOfBand(self.as_raw_mut_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set width of bands
	/// ## Parameters
	/// * width: width of bands
	// setWidthOfBand(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:245
	// ("cv::line_descriptor::BinaryDescriptor::setWidthOfBand", vec![(pred!(mut, ["width"], ["int"]), _)]),
	#[inline]
	fn set_width_of_band(&mut self, width: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_setWidthOfBand_int(self.as_raw_mut_BinaryDescriptor(), width, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get current reduction ratio (used in Gaussian pyramids)
	// getReductionRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:248
	// ("cv::line_descriptor::BinaryDescriptor::getReductionRatio", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_reduction_ratio(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_getReductionRatio(self.as_raw_mut_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set reduction ratio (used in Gaussian pyramids)
	/// ## Parameters
	/// * rRatio: reduction ratio
	// setReductionRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:252
	// ("cv::line_descriptor::BinaryDescriptor::setReductionRatio", vec![(pred!(mut, ["rRatio"], ["int"]), _)]),
	#[inline]
	fn set_reduction_ratio(&mut self, r_ratio: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_setReductionRatio_int(self.as_raw_mut_BinaryDescriptor(), r_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Read parameters from a FileNode object and store them
	///
	/// ## Parameters
	/// * fn: source FileNode file
	// read(const cv::FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:258
	// ("cv::line_descriptor::BinaryDescriptor::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_read_const_FileNodeR(self.as_raw_mut_BinaryDescriptor(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// detect(const Mat &, std::vector<KeyLine> &, const Mat &)(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:272
	// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(mut, ["image", "keypoints", "mask"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "const cv::Mat*"]), _)]),
	#[inline]
	fn detect_1(&mut self, image: &impl core::MatTraitConst, keypoints: &mut core::Vector<crate::line_descriptor::KeyLine>, mask: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_detect_const_MatR_vectorLKeyLineGR_const_MatR(self.as_raw_mut_BinaryDescriptor(), image.as_raw_Mat(), keypoints.as_raw_mut_VectorOfKeyLine(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Requires line detection
	///
	/// ## Parameters
	/// * image: input image
	/// * keypoints: vector that will store extracted lines for one or more images
	/// * mask: mask matrix to detect only KeyLines of interest
	///
	/// ## Note
	/// This alternative version of [BinaryDescriptorTrait::detect] function uses the following default values for its arguments:
	/// * mask: Mat()
	// cv::line_descriptor::BinaryDescriptor::detect(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:272
	// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(mut, ["image", "keypoints"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*"]), _)]),
	#[inline]
	fn detect_def_1(&mut self, image: &impl core::MatTraitConst, keypoints: &mut core::Vector<crate::line_descriptor::KeyLine>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_detect_const_MatR_vectorLKeyLineGR(self.as_raw_mut_BinaryDescriptor(), image.as_raw_Mat(), keypoints.as_raw_mut_VectorOfKeyLine(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implements both functionalities for detection of lines and computation of their
/// binary descriptor.
///
/// Class' interface is mainly based on the ones of classical detectors and extractors, such as
/// Feature2d's [features2d_main] and [features2d_match]. Retrieved information about lines is
/// stored in line_descriptor::KeyLine objects.
// BinaryDescriptor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:182
pub struct BinaryDescriptor {
	ptr: *mut c_void,
}

opencv_type_boxed! { BinaryDescriptor }

impl Drop for BinaryDescriptor {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_delete(self.as_raw_mut_BinaryDescriptor()) };
	}
}

unsafe impl Send for BinaryDescriptor {}

impl core::AlgorithmTraitConst for BinaryDescriptor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BinaryDescriptor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BinaryDescriptor, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::line_descriptor::BinaryDescriptorTraitConst for BinaryDescriptor {
	#[inline] fn as_raw_BinaryDescriptor(&self) -> *const c_void { self.as_raw() }
}

impl crate::line_descriptor::BinaryDescriptorTrait for BinaryDescriptor {
	#[inline] fn as_raw_mut_BinaryDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BinaryDescriptor, crate::line_descriptor::BinaryDescriptorTraitConst, as_raw_BinaryDescriptor, crate::line_descriptor::BinaryDescriptorTrait, as_raw_mut_BinaryDescriptor }

impl BinaryDescriptor {
	/// Constructor
	///
	/// ## Parameters
	/// * parameters: configuration parameters BinaryDescriptor::Params
	///
	/// If no argument is provided, constructor sets default values (see comments in the code snippet in
	/// previous section). Default values are strongly recommended.
	///
	/// ## C++ default parameters
	/// * parameters: BinaryDescriptor::Params()
	// BinaryDescriptor(const BinaryDescriptor::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:221
	// ("cv::line_descriptor::BinaryDescriptor::BinaryDescriptor", vec![(pred!(mut, ["parameters"], ["const cv::line_descriptor::BinaryDescriptor::Params*"]), _)]),
	#[inline]
	pub fn new(parameters: &impl crate::line_descriptor::BinaryDescriptor_ParamsTraitConst) -> Result<crate::line_descriptor::BinaryDescriptor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_BinaryDescriptor_const_ParamsR(parameters.as_raw_BinaryDescriptor_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::line_descriptor::BinaryDescriptor::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	///
	/// ## Parameters
	/// * parameters: configuration parameters BinaryDescriptor::Params
	///
	/// If no argument is provided, constructor sets default values (see comments in the code snippet in
	/// previous section). Default values are strongly recommended.
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * parameters: BinaryDescriptor::Params()
	// cv::line_descriptor::BinaryDescriptor::BinaryDescriptor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:221
	// ("cv::line_descriptor::BinaryDescriptor::BinaryDescriptor", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::line_descriptor::BinaryDescriptor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_BinaryDescriptor(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::line_descriptor::BinaryDescriptor::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Create a BinaryDescriptor object with default parameters (or with the ones provided)
	/// and return a smart pointer to it
	// createBinaryDescriptor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:226
	// ("cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_binary_descriptor() -> Result<core::Ptr<crate::line_descriptor::BinaryDescriptor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::line_descriptor::BinaryDescriptor>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// createBinaryDescriptor(Params)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:227
	// ("cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor", vec![(pred!(mut, ["parameters"], ["cv::line_descriptor::BinaryDescriptor::Params"]), _)]),
	#[inline]
	pub fn create_binary_descriptor_1(mut parameters: impl crate::line_descriptor::BinaryDescriptor_ParamsTrait) -> Result<core::Ptr<crate::line_descriptor::BinaryDescriptor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor_Params(parameters.as_raw_mut_BinaryDescriptor_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::line_descriptor::BinaryDescriptor>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BinaryDescriptor, core::Algorithm, cv_line_descriptor_BinaryDescriptor_to_Algorithm }

impl std::fmt::Debug for BinaryDescriptor {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BinaryDescriptor")
			.finish()
	}
}

/// Constant methods for [crate::line_descriptor::BinaryDescriptor_Params]
// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:188
pub trait BinaryDescriptor_ParamsTraitConst {
	fn as_raw_BinaryDescriptor_Params(&self) -> *const c_void;

	/// the number of image octaves (default = 1)
	// cv::line_descriptor::BinaryDescriptor::Params::numOfOctave_() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:195
	// ("cv::line_descriptor::BinaryDescriptor::Params::numOfOctave_", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_of_octave_(&self) -> i32 {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_propNumOfOctave__const(self.as_raw_BinaryDescriptor_Params()) };
		ret
	}

	/// the width of band; (default: 7)
	// cv::line_descriptor::BinaryDescriptor::Params::widthOfBand_() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:199
	// ("cv::line_descriptor::BinaryDescriptor::Params::widthOfBand_", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn width_of_band_(&self) -> i32 {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_propWidthOfBand__const(self.as_raw_BinaryDescriptor_Params()) };
		ret
	}

	/// image's reduction ratio in construction of Gaussian pyramids
	// cv::line_descriptor::BinaryDescriptor::Params::reductionRatio() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:202
	// ("cv::line_descriptor::BinaryDescriptor::Params::reductionRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn reduction_ratio(&self) -> i32 {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_propReductionRatio_const(self.as_raw_BinaryDescriptor_Params()) };
		ret
	}

	// cv::line_descriptor::BinaryDescriptor::Params::ksize_() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:204
	// ("cv::line_descriptor::BinaryDescriptor::Params::ksize_", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ksize_(&self) -> i32 {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_propKsize__const(self.as_raw_BinaryDescriptor_Params()) };
		ret
	}

	/// store parameters to a FileStorage object (struct function)
	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:210
	// ("cv::line_descriptor::BinaryDescriptor::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_write_const_FileStorageR(self.as_raw_BinaryDescriptor_Params(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::line_descriptor::BinaryDescriptor_Params]
pub trait BinaryDescriptor_ParamsTrait: crate::line_descriptor::BinaryDescriptor_ParamsTraitConst {
	fn as_raw_mut_BinaryDescriptor_Params(&mut self) -> *mut c_void;

	/// the number of image octaves (default = 1)
	// cv::line_descriptor::BinaryDescriptor::Params::setNumOfOctave_(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:195
	// ("cv::line_descriptor::BinaryDescriptor::Params::setNumOfOctave_", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_num_of_octave_(&mut self, val: i32) {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_propNumOfOctave__const_int(self.as_raw_mut_BinaryDescriptor_Params(), val) };
		ret
	}

	/// the width of band; (default: 7)
	// cv::line_descriptor::BinaryDescriptor::Params::setWidthOfBand_(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:199
	// ("cv::line_descriptor::BinaryDescriptor::Params::setWidthOfBand_", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_width_of_band_(&mut self, val: i32) {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_propWidthOfBand__const_int(self.as_raw_mut_BinaryDescriptor_Params(), val) };
		ret
	}

	/// image's reduction ratio in construction of Gaussian pyramids
	// cv::line_descriptor::BinaryDescriptor::Params::setReductionRatio(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:202
	// ("cv::line_descriptor::BinaryDescriptor::Params::setReductionRatio", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_reduction_ratio(&mut self, val: i32) {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_propReductionRatio_const_int(self.as_raw_mut_BinaryDescriptor_Params(), val) };
		ret
	}

	// cv::line_descriptor::BinaryDescriptor::Params::setKsize_(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:204
	// ("cv::line_descriptor::BinaryDescriptor::Params::setKsize_", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_ksize_(&mut self, val: i32) {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_propKsize__const_int(self.as_raw_mut_BinaryDescriptor_Params(), val) };
		ret
	}

	/// read parameters from a FileNode object and store them (struct function)
	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:207
	// ("cv::line_descriptor::BinaryDescriptor::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_read_const_FileNodeR(self.as_raw_mut_BinaryDescriptor_Params(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// List of BinaryDescriptor parameters:
// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:188
pub struct BinaryDescriptor_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { BinaryDescriptor_Params }

impl Drop for BinaryDescriptor_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_delete(self.as_raw_mut_BinaryDescriptor_Params()) };
	}
}

unsafe impl Send for BinaryDescriptor_Params {}

impl crate::line_descriptor::BinaryDescriptor_ParamsTraitConst for BinaryDescriptor_Params {
	#[inline] fn as_raw_BinaryDescriptor_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::line_descriptor::BinaryDescriptor_ParamsTrait for BinaryDescriptor_Params {
	#[inline] fn as_raw_mut_BinaryDescriptor_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BinaryDescriptor_Params, crate::line_descriptor::BinaryDescriptor_ParamsTraitConst, as_raw_BinaryDescriptor_Params, crate::line_descriptor::BinaryDescriptor_ParamsTrait, as_raw_mut_BinaryDescriptor_Params }

impl BinaryDescriptor_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:191
	// ("cv::line_descriptor::BinaryDescriptor::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::line_descriptor::BinaryDescriptor_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::line_descriptor::BinaryDescriptor_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for BinaryDescriptor_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BinaryDescriptor_Params")
			.field("num_of_octave_", &crate::line_descriptor::BinaryDescriptor_ParamsTraitConst::num_of_octave_(self))
			.field("width_of_band_", &crate::line_descriptor::BinaryDescriptor_ParamsTraitConst::width_of_band_(self))
			.field("reduction_ratio", &crate::line_descriptor::BinaryDescriptor_ParamsTraitConst::reduction_ratio(self))
			.field("ksize_", &crate::line_descriptor::BinaryDescriptor_ParamsTraitConst::ksize_(self))
			.finish()
	}
}

/// Constant methods for [crate::line_descriptor::BinaryDescriptorMatcher]
// BinaryDescriptorMatcher /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1019
pub trait BinaryDescriptorMatcherTraitConst: core::AlgorithmTraitConst {
	fn as_raw_BinaryDescriptorMatcher(&self) -> *const c_void;

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
	// match(const Mat &, const Mat &, std::vector<DMatch> &, const Mat &)(TraitClass, TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1031
	// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "mask"], ["const cv::Mat*", "const cv::Mat*", "std::vector<cv::DMatch>*", "const cv::Mat*"]), _)]),
	#[inline]
	fn match_(&self, query_descriptors: &impl core::MatTraitConst, train_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::DMatch>, mask: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_match_const_const_MatR_const_MatR_vectorLDMatchGR_const_MatR(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfDMatch(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// For every input query descriptor, retrieve the best matching one from a dataset provided from user
	/// or from the one internal to class
	///
	/// ## Parameters
	/// * queryDescriptors: query descriptors
	/// * trainDescriptors: dataset of descriptors furnished by user
	/// * matches: vector to host retrieved matches
	/// * mask: mask to select which input descriptors must be matched to one in dataset
	///
	/// ## Note
	/// This alternative version of [BinaryDescriptorMatcherTraitConst::match_] function uses the following default values for its arguments:
	/// * mask: Mat()
	// cv::line_descriptor::BinaryDescriptorMatcher::match(TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1031
	// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches"], ["const cv::Mat*", "const cv::Mat*", "std::vector<cv::DMatch>*"]), _)]),
	#[inline]
	fn match__def(&self, query_descriptors: &impl core::MatTraitConst, train_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_match_const_const_MatR_const_MatR_vectorLDMatchGR(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// knnMatch(const Mat &, const Mat &, std::vector<std::vector<DMatch>> &, int, const Mat &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1053
	// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k", "mask", "compactResult"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int", "const cv::Mat*", "bool"]), _)]),
	#[inline]
	fn knn_match(&self, query_descriptors: &impl core::MatTraitConst, train_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, mask: &impl core::MatTraitConst, compact_result: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_int_const_MatR_bool(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, mask.as_raw_Mat(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [BinaryDescriptorMatcherTraitConst::knn_match] function uses the following default values for its arguments:
	/// * mask: Mat()
	/// * compact_result: false
	// cv::line_descriptor::BinaryDescriptorMatcher::knnMatch(TraitClass, TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1053
	// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
	#[inline]
	fn knn_match_def(&self, query_descriptors: &impl core::MatTraitConst, train_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_int(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// radiusMatch(const Mat &, const Mat &, std::vector<std::vector<DMatch>> &, float, const Mat &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1080
	// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance", "mask", "compactResult"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float", "const cv::Mat*", "bool"]), _)]),
	#[inline]
	fn radius_match(&self, query_descriptors: &impl core::MatTraitConst, train_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, mask: &impl core::MatTraitConst, compact_result: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_float_const_MatR_bool(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, mask.as_raw_Mat(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [BinaryDescriptorMatcherTraitConst::radius_match] function uses the following default values for its arguments:
	/// * mask: Mat()
	/// * compact_result: false
	// cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch(TraitClass, TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1080
	// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
	#[inline]
	fn radius_match_def(&self, query_descriptors: &impl core::MatTraitConst, train_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_float(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::line_descriptor::BinaryDescriptorMatcher]
pub trait BinaryDescriptorMatcherTrait: core::AlgorithmTrait + crate::line_descriptor::BinaryDescriptorMatcherTraitConst {
	fn as_raw_mut_BinaryDescriptorMatcher(&mut self) -> *mut c_void;

	/// For every input query descriptor, retrieve the best matching one from a dataset provided from user
	/// or from the one internal to class
	///
	/// ## Parameters
	/// * queryDescriptors: query descriptors
	/// * trainDescriptors: dataset of descriptors furnished by user
	/// * matches: vector to host retrieved matches
	/// * mask: mask to select which input descriptors must be matched to one in dataset
	///
	/// ## Overloaded parameters
	///
	/// * queryDescriptors: query descriptors
	/// * matches: vector to host retrieved matches
	/// * masks: vector of masks to select which input descriptors must be matched to one in dataset
	/// (the *i*-th mask in vector indicates whether each input query can be matched with descriptors in
	/// dataset relative to *i*-th image)
	///
	/// ## C++ default parameters
	/// * masks: std::vector<Mat>()
	// match(const Mat &, std::vector<DMatch> &, const std::vector<Mat> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1040
	// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches", "masks"], ["const cv::Mat*", "std::vector<cv::DMatch>*", "const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn match_query(&mut self, query_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::DMatch>, masks: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_match_const_MatR_vectorLDMatchGR_const_vectorLMatGR(self.as_raw_mut_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfDMatch(), masks.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
	/// ## Parameters
	/// * queryDescriptors: query descriptors
	/// * matches: vector to host retrieved matches
	/// * masks: vector of masks to select which input descriptors must be matched to one in dataset
	/// (the *i*-th mask in vector indicates whether each input query can be matched with descriptors in
	/// dataset relative to *i*-th image)
	///
	/// ## Note
	/// This alternative version of [BinaryDescriptorMatcherTrait::match_query] function uses the following default values for its arguments:
	/// * masks: std::vector<Mat>()
	// cv::line_descriptor::BinaryDescriptorMatcher::match(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1040
	// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches"], ["const cv::Mat*", "std::vector<cv::DMatch>*"]), _)]),
	#[inline]
	fn match_query_def(&mut self, query_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_match_const_MatR_vectorLDMatchGR(self.as_raw_mut_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Overloaded parameters
	///
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
	// knnMatch(const Mat &, std::vector<std::vector<DMatch>> &, int, const std::vector<Mat> &, bool)(TraitClass, CppPassByVoidPtr, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1066
	// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k", "masks", "compactResult"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int", "const std::vector<cv::Mat>*", "bool"]), _)]),
	#[inline]
	fn knn_match_query(&mut self, query_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, masks: &core::Vector<core::Mat>, compact_result: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_MatR_vectorLvectorLDMatchGGR_int_const_vectorLMatGR_bool(self.as_raw_mut_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, masks.as_raw_VectorOfMat(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
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
	/// ## Note
	/// This alternative version of [BinaryDescriptorMatcherTrait::knn_match_query] function uses the following default values for its arguments:
	/// * masks: std::vector<Mat>()
	/// * compact_result: false
	// cv::line_descriptor::BinaryDescriptorMatcher::knnMatch(TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1066
	// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
	#[inline]
	fn knn_match_query_def(&mut self, query_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_MatR_vectorLvectorLDMatchGGR_int(self.as_raw_mut_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Overloaded parameters
	///
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
	// radiusMatch(const Mat &, std::vector<std::vector<DMatch>> &, float, const std::vector<Mat> &, bool)(TraitClass, CppPassByVoidPtr, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1093
	// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance", "masks", "compactResult"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float", "const std::vector<cv::Mat>*", "bool"]), _)]),
	#[inline]
	fn radius_match_1(&mut self, query_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, masks: &core::Vector<core::Mat>, compact_result: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_MatR_vectorLvectorLDMatchGGR_float_const_vectorLMatGR_bool(self.as_raw_mut_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, masks.as_raw_VectorOfMat(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
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
	/// ## Note
	/// This alternative version of [BinaryDescriptorMatcherTrait::radius_match] function uses the following default values for its arguments:
	/// * masks: std::vector<Mat>()
	/// * compact_result: false
	// cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch(TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1093
	// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
	#[inline]
	fn radius_match_def_1(&mut self, query_descriptors: &impl core::MatTraitConst, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_MatR_vectorLvectorLDMatchGGR_float(self.as_raw_mut_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Store locally new descriptors to be inserted in dataset, without updating dataset.
	///
	/// ## Parameters
	/// * descriptors: matrices containing descriptors to be inserted into dataset
	///
	///
	/// Note: Each matrix *i* in **descriptors** should contain descriptors relative to lines extracted from
	/// *i*-th image.
	// add(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1104
	// ("cv::line_descriptor::BinaryDescriptorMatcher::add", vec![(pred!(mut, ["descriptors"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn add(&mut self, descriptors: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_add_const_vectorLMatGR(self.as_raw_mut_BinaryDescriptorMatcher(), descriptors.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Update dataset by inserting into it all descriptors that were stored locally by *add* function.
	///
	///
	/// Note: Every time this function is invoked, current dataset is deleted and locally stored descriptors
	/// are inserted into dataset. The locally stored copy of just inserted descriptors is then removed.
	// train()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1111
	// ("cv::line_descriptor::BinaryDescriptorMatcher::train", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn train(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_train(self.as_raw_mut_BinaryDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Clear dataset and internal data
	// clear()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1119
	// ("cv::line_descriptor::BinaryDescriptorMatcher::clear", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_clear(self.as_raw_mut_BinaryDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// furnishes all functionalities for querying a dataset provided by user or internal to
/// class (that user must, anyway, populate) on the model of [features2d_match]
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
/// The theory described in this section is based on [MIH](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_MIH) . Given a dataset populated with binary
/// codes, each code is indexed *m* times into *m* different hash tables, according to *m* substrings it
/// has been divided into. Thus, given a query code, all the entries close to it at least in one
/// substring are returned by search as *neighbor candidates*. Returned entries are then checked for
/// validity by verifying that their full codes are not distant (in Hamming space) more than *r* bits
/// from query code. In details, each binary code **h** composed of *b* bits is divided into *m*
/// disjoint substrings ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7Bh%7D%5E%7B%281%29%7D%2C%20%2E%2E%2E%2C%20%5Cmathbf%7Bh%7D%5E%7B%28m%29%7D), each with length
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Clfloor%20b%2Fm%20%5Crfloor) or ![inline formula](https://latex.codecogs.com/png.latex?%5Clceil%20b%2Fm%20%5Crceil) bits. Formally, when two codes **h** and **g** differ
/// by at the most *r* bits, in at the least one of their *m* substrings they differ by at the most
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Clfloor%20r%2Fm%20%5Crfloor) bits. In particular, when ![inline formula](https://latex.codecogs.com/png.latex?%7C%7C%5Cmathbf%7Bh%7D%2D%5Cmathbf%7Bg%7D%7C%7C%5FH%20%5Cle%20r) (where ![inline formula](https://latex.codecogs.com/png.latex?%7C%7C%2E%7C%7C%5FH)
/// is the Hamming norm), there must exist a substring *k* (with ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Cle%20k%20%5Cle%20m)) such that
///
/// ![block formula](https://latex.codecogs.com/png.latex?%7C%7C%5Cmathbf%7Bh%7D%5E%7B%28k%29%7D%20%2D%20%5Cmathbf%7Bg%7D%5E%7B%28k%29%7D%7C%7C%5FH%20%5Cle%20%5Cleft%5Clfloor%20%5Cfrac%7Br%7D%7Bm%7D%20%5Cright%5Crfloor%20%2E)
///
/// That means that if Hamming distance between each of the *m* substring is strictly greater than
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Clfloor%20r%2Fm%20%5Crfloor), then ![inline formula](https://latex.codecogs.com/png.latex?%7C%7C%5Cmathbf%7Bh%7D%2D%5Cmathbf%7Bg%7D%7C%7C%5FH) must be larger that *r* and that is a
/// contradiction. If the codes in dataset are divided into *m* substrings, then *m* tables will be
/// built. Given a query **q** with substrings ![inline formula](https://latex.codecogs.com/png.latex?%5C%7B%5Cmathbf%7Bq%7D%5E%7B%28i%29%7D%5C%7D%5Em%5F%7Bi%3D1%7D), *i*-th hash table is
/// searched for entries distant at the most ![inline formula](https://latex.codecogs.com/png.latex?%5Clfloor%20r%2Fm%20%5Crfloor) from ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7Bq%7D%5E%7B%28i%29%7D) and a set of
/// candidates ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathcal%7BN%7D%5Fi%28%5Cmathbf%7Bq%7D%29) is obtained. The union of sets
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathcal%7BN%7D%28%5Cmathbf%7Bq%7D%29%20%3D%20%5Cbigcup%5Fi%20%5Cmathcal%7BN%7D%5Fi%28%5Cmathbf%7Bq%7D%29) is a superset of the *r*-neighbors
/// of **q**. Then, last step of algorithm is computing the Hamming distance between **q** and each
/// element in ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathcal%7BN%7D%28%5Cmathbf%7Bq%7D%29), deleting the codes that are distant more that *r* from **q**.
// BinaryDescriptorMatcher /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1019
pub struct BinaryDescriptorMatcher {
	ptr: *mut c_void,
}

opencv_type_boxed! { BinaryDescriptorMatcher }

impl Drop for BinaryDescriptorMatcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_delete(self.as_raw_mut_BinaryDescriptorMatcher()) };
	}
}

unsafe impl Send for BinaryDescriptorMatcher {}

impl core::AlgorithmTraitConst for BinaryDescriptorMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BinaryDescriptorMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BinaryDescriptorMatcher, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::line_descriptor::BinaryDescriptorMatcherTraitConst for BinaryDescriptorMatcher {
	#[inline] fn as_raw_BinaryDescriptorMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::line_descriptor::BinaryDescriptorMatcherTrait for BinaryDescriptorMatcher {
	#[inline] fn as_raw_mut_BinaryDescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BinaryDescriptorMatcher, crate::line_descriptor::BinaryDescriptorMatcherTraitConst, as_raw_BinaryDescriptorMatcher, crate::line_descriptor::BinaryDescriptorMatcherTrait, as_raw_mut_BinaryDescriptorMatcher }

impl BinaryDescriptorMatcher {
	/// Create a BinaryDescriptorMatcher object and return a smart pointer to it.
	// createBinaryDescriptorMatcher()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1115
	// ("cv::line_descriptor::BinaryDescriptorMatcher::createBinaryDescriptorMatcher", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_binary_descriptor_matcher() -> Result<core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_createBinaryDescriptorMatcher(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::line_descriptor::BinaryDescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor.
	///
	/// The BinaryDescriptorMatcher constructed is able to store and manage 256-bits long entries.
	// BinaryDescriptorMatcher()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:1125
	// ("cv::line_descriptor::BinaryDescriptorMatcher::BinaryDescriptorMatcher", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::line_descriptor::BinaryDescriptorMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_BinaryDescriptorMatcher(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::line_descriptor::BinaryDescriptorMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BinaryDescriptorMatcher, core::Algorithm, cv_line_descriptor_BinaryDescriptorMatcher_to_Algorithm }

impl std::fmt::Debug for BinaryDescriptorMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BinaryDescriptorMatcher")
			.finish()
	}
}

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
///    same line inside original image (such lines and the one they represent in original image share
///    the same *class_id* value)
/// *   the *angle* field represents line's slope with respect to (positive) X axis
/// *   the *pt* field represents line's midpoint
/// *   the *response* field is computed as the ratio between the line's length and maximum between
///    image's width and height
/// *   the *size* field is the area of the smallest rectangle containing line
///
/// Apart from fields inspired to KeyPoint class, KeyLines stores information about extremes of line in
/// original image and in octave it was extracted from, about line's length and number of pixels it
/// covers.
// KeyLine /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:104
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyLine {
	/// orientation of the line
	pub angle: f32,
	/// object ID, that can be used to cluster keylines by the line they represent
	pub class_id: i32,
	/// octave (pyramid layer), from which the keyline has been extracted
	pub octave: i32,
	/// coordinates of the middlepoint
	pub pt: core::Point2f,
	/// the response, by which the strongest keylines have been selected.
	/// It's represented by the ratio between line's length and maximum between
	/// image's width and height
	pub response: f32,
	/// minimum area containing line
	pub size: f32,
	/// lines's extremes in original image
	pub start_point_x: f32,
	pub start_point_y: f32,
	pub end_point_x: f32,
	pub end_point_y: f32,
	/// line's extremes in image it was extracted from
	pub s_point_in_octave_x: f32,
	pub s_point_in_octave_y: f32,
	pub e_point_in_octave_x: f32,
	pub e_point_in_octave_y: f32,
	/// the length of line
	pub line_length: f32,
	/// number of pixels covered by the line
	pub num_of_pixels: i32,
}

opencv_type_simple! { crate::line_descriptor::KeyLine }

impl KeyLine {
	/// Returns the start point of the line in the original image
	// getStartPoint()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:146
	// ("cv::line_descriptor::KeyLine::getStartPoint", vec![(pred!(const, [], []), _)]),
	#[inline]
	pub fn get_start_point(self) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_KeyLine_getStartPoint_const(&self, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the end point of the line in the original image
	// getEndPoint()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:152
	// ("cv::line_descriptor::KeyLine::getEndPoint", vec![(pred!(const, [], []), _)]),
	#[inline]
	pub fn get_end_point(self) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_KeyLine_getEndPoint_const(&self, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the start point of the line in the octave it was extracted from
	// getStartPointInOctave()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:158
	// ("cv::line_descriptor::KeyLine::getStartPointInOctave", vec![(pred!(const, [], []), _)]),
	#[inline]
	pub fn get_start_point_in_octave(self) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_KeyLine_getStartPointInOctave_const(&self, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the end point of the line in the octave it was extracted from
	// getEndPointInOctave()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:164
	// ("cv::line_descriptor::KeyLine::getEndPointInOctave", vec![(pred!(const, [], []), _)]),
	#[inline]
	pub fn get_end_point_in_octave(self) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_KeyLine_getEndPointInOctave_const(&self, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// constructor
	// KeyLine()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:170
	// ("cv::line_descriptor::KeyLine::KeyLine", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::line_descriptor::KeyLine> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_KeyLine_KeyLine(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Constant methods for [crate::line_descriptor::LSDDetector]
// LSDDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:924
pub trait LSDDetectorTraitConst: core::AlgorithmTraitConst {
	fn as_raw_LSDDetector(&self) -> *const c_void;

	/// Detect lines inside an image.
	///
	/// ## Parameters
	/// * image: input image
	/// * keypoints: vector that will store extracted lines for one or more images
	/// * scale: scale factor used in pyramids generation
	/// * numOctaves: number of octaves inside pyramid
	/// * mask: mask matrix to detect only KeyLines of interest
	///
	/// ## Overloaded parameters
	///
	/// * images: input images
	/// * keylines: set of vectors that will store extracted lines for one or more images
	/// * scale: scale factor used in pyramids generation
	/// * numOctaves: number of octaves inside pyramid
	/// * masks: vector of mask matrices to detect only KeyLines of interest from each input image
	///
	/// ## C++ default parameters
	/// * masks: std::vector<Mat>()
	// detect(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, int, int, const std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:964
	// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(const, ["images", "keylines", "scale", "numOctaves", "masks"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "int", "int", "const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn detect_multiple(&self, images: &core::Vector<core::Mat>, keylines: &mut core::Vector<core::Vector<crate::line_descriptor::KeyLine>>, scale: i32, num_octaves: i32, masks: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_int_int_const_vectorLMatGR(self.as_raw_LSDDetector(), images.as_raw_VectorOfMat(), keylines.as_raw_mut_VectorOfVectorOfKeyLine(), scale, num_octaves, masks.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
	/// ## Parameters
	/// * images: input images
	/// * keylines: set of vectors that will store extracted lines for one or more images
	/// * scale: scale factor used in pyramids generation
	/// * numOctaves: number of octaves inside pyramid
	/// * masks: vector of mask matrices to detect only KeyLines of interest from each input image
	///
	/// ## Note
	/// This alternative version of [LSDDetectorTraitConst::detect_multiple] function uses the following default values for its arguments:
	/// * masks: std::vector<Mat>()
	// cv::line_descriptor::LSDDetector::detect(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:964
	// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(const, ["images", "keylines", "scale", "numOctaves"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "int", "int"]), _)]),
	#[inline]
	fn detect_multiple_def(&self, images: &core::Vector<core::Mat>, keylines: &mut core::Vector<core::Vector<crate::line_descriptor::KeyLine>>, scale: i32, num_octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_int_int(self.as_raw_LSDDetector(), images.as_raw_VectorOfMat(), keylines.as_raw_mut_VectorOfVectorOfKeyLine(), scale, num_octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::line_descriptor::LSDDetector]
pub trait LSDDetectorTrait: core::AlgorithmTrait + crate::line_descriptor::LSDDetectorTraitConst {
	fn as_raw_mut_LSDDetector(&mut self) -> *mut c_void;

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
	// detect(const Mat &, std::vector<KeyLine> &, int, int, const Mat &)(TraitClass, CppPassByVoidPtr, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:955
	// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(mut, ["image", "keypoints", "scale", "numOctaves", "mask"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "int", "int", "const cv::Mat*"]), _)]),
	#[inline]
	fn detect(&mut self, image: &impl core::MatTraitConst, keypoints: &mut core::Vector<crate::line_descriptor::KeyLine>, scale: i32, num_octaves: i32, mask: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_detect_const_MatR_vectorLKeyLineGR_int_int_const_MatR(self.as_raw_mut_LSDDetector(), image.as_raw_Mat(), keypoints.as_raw_mut_VectorOfKeyLine(), scale, num_octaves, mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [LSDDetectorTrait::detect] function uses the following default values for its arguments:
	/// * mask: Mat()
	// cv::line_descriptor::LSDDetector::detect(TraitClass, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:955
	// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(mut, ["image", "keypoints", "scale", "numOctaves"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "int", "int"]), _)]),
	#[inline]
	fn detect_def(&mut self, image: &impl core::MatTraitConst, keypoints: &mut core::Vector<crate::line_descriptor::KeyLine>, scale: i32, num_octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_detect_const_MatR_vectorLKeyLineGR_int_int(self.as_raw_mut_LSDDetector(), image.as_raw_Mat(), keypoints.as_raw_mut_VectorOfKeyLine(), scale, num_octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// LSDDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:924
pub struct LSDDetector {
	ptr: *mut c_void,
}

opencv_type_boxed! { LSDDetector }

impl Drop for LSDDetector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_line_descriptor_LSDDetector_delete(self.as_raw_mut_LSDDetector()) };
	}
}

unsafe impl Send for LSDDetector {}

impl core::AlgorithmTraitConst for LSDDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LSDDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LSDDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::line_descriptor::LSDDetectorTraitConst for LSDDetector {
	#[inline] fn as_raw_LSDDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::line_descriptor::LSDDetectorTrait for LSDDetector {
	#[inline] fn as_raw_mut_LSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LSDDetector, crate::line_descriptor::LSDDetectorTraitConst, as_raw_LSDDetector, crate::line_descriptor::LSDDetectorTrait, as_raw_mut_LSDDetector }

impl LSDDetector {
	// LSDDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:929
	// ("cv::line_descriptor::LSDDetector::LSDDetector", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::line_descriptor::LSDDetector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_LSDDetector(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::line_descriptor::LSDDetector::opencv_from_extern(ret) };
		Ok(ret)
	}

	// LSDDetector(LSDParam)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:934
	// ("cv::line_descriptor::LSDDetector::LSDDetector", vec![(pred!(mut, ["_params"], ["cv::line_descriptor::LSDParam"]), _)]),
	#[inline]
	pub fn new(_params: crate::line_descriptor::LSDParam) -> Result<crate::line_descriptor::LSDDetector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_LSDDetector_LSDParam(&_params, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::line_descriptor::LSDDetector::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates ad LSDDetector object, using smart pointers.
	// createLSDDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:941
	// ("cv::line_descriptor::LSDDetector::createLSDDetector", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_lsd_detector() -> Result<core::Ptr<crate::line_descriptor::LSDDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_createLSDDetector(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::line_descriptor::LSDDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// createLSDDetector(LSDParam)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:944
	// ("cv::line_descriptor::LSDDetector::createLSDDetector", vec![(pred!(mut, ["params"], ["cv::line_descriptor::LSDParam"]), _)]),
	#[inline]
	pub fn create_lsd_detector_with_params(params: crate::line_descriptor::LSDParam) -> Result<core::Ptr<crate::line_descriptor::LSDDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_createLSDDetector_LSDParam(&params, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::line_descriptor::LSDDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { LSDDetector, core::Algorithm, cv_line_descriptor_LSDDetector_to_Algorithm }

impl std::fmt::Debug for LSDDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LSDDetector")
			.finish()
	}
}

/// Lines extraction methodology
/// ----------------------------
///
/// The lines extraction methodology described in the following is mainly based on [EDL](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_EDL) . The
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
// LSDParam /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:903
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

opencv_type_simple! { crate::line_descriptor::LSDParam }

impl LSDParam {
	// LSDParam()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/line_descriptor/descriptor.hpp:914
	// ("cv::line_descriptor::LSDParam::LSDParam", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::line_descriptor::LSDParam> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDParam_LSDParam(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

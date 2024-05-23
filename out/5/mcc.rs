//! # Macbeth Chart module
//!    # Color Correction Model
//!
//!
//!
//! Introduction
//! ------------
//!
//! ColorCharts are a tool for calibrating the color profile of camera, which not
//! only depends on the intrinsic and extrinsic parameters of camera but also on the
//! lighting conditions. This is done by taking the image of a chart, such that the
//! value of its colors present in it known, in the image the color values changes
//! depeding on many variables, this gives us the colors initially present and the
//! colors that are present in the image, based on this information we can apply any
//! suitable algorithm to find the actual color of all the objects present in the
//! image.
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{ColorCorrectionModelTrait, ColorCorrectionModelTraitConst, MCC_CCheckerDetectorTrait, MCC_CCheckerDetectorTraitConst, MCC_CCheckerDrawTrait, MCC_CCheckerDrawTraitConst, MCC_CCheckerTrait, MCC_CCheckerTraitConst, MCC_DetectorParametersTrait, MCC_DetectorParametersTraitConst};
}

/// The CCM with the shape ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) performs linear transformation on color values.
// CCM_3x3 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:61
pub const CCM_3x3: i32 = 0;
/// The CCM with the shape ![inline formula](https://latex.codecogs.com/png.latex?4%5Ctimes3) performs affine transformation.
// CCM_4x3 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:62
pub const CCM_4x3: i32 = 1;
/// DigitalSG ColorChecker with 140 squares
// COLORCHECKER_DigitalSG /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:91
pub const COLORCHECKER_DigitalSG: i32 = 2;
/// Macbeth ColorChecker
// COLORCHECKER_Macbeth /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:89
pub const COLORCHECKER_Macbeth: i32 = 0;
/// DKK ColorChecker
// COLORCHECKER_Vinyl /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:90
pub const COLORCHECKER_Vinyl: i32 = 1;
/// <https://en.wikipedia.org/wiki/Adobe_RGB_color_space> , RGB color space
// COLOR_SPACE_AdobeRGB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:96
pub const COLOR_SPACE_AdobeRGB: i32 = 2;
/// <https://en.wikipedia.org/wiki/Adobe_RGB_color_space> , linear RGB color space
// COLOR_SPACE_AdobeRGBL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:97
pub const COLOR_SPACE_AdobeRGBL: i32 = 3;
/// <https://en.wikipedia.org/wiki/RGB_color_space> , RGB color space
// COLOR_SPACE_AppleRGB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:104
pub const COLOR_SPACE_AppleRGB: i32 = 10;
/// <https://en.wikipedia.org/wiki/RGB_color_space> , linear RGB color space
// COLOR_SPACE_AppleRGBL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:105
pub const COLOR_SPACE_AppleRGBL: i32 = 11;
/// <https://en.wikipedia.org/wiki/DCI-P3> , RGB color space
// COLOR_SPACE_DCI_P3_RGB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:102
pub const COLOR_SPACE_DCI_P3_RGB: i32 = 8;
/// <https://en.wikipedia.org/wiki/DCI-P3> , linear RGB color space
// COLOR_SPACE_DCI_P3_RGBL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:103
pub const COLOR_SPACE_DCI_P3_RGBL: i32 = 9;
/// non-RGB color space
// COLOR_SPACE_Lab_A_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:127
pub const COLOR_SPACE_Lab_A_10: i32 = 33;
/// non-RGB color space
// COLOR_SPACE_Lab_A_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:126
pub const COLOR_SPACE_Lab_A_2: i32 = 32;
/// non-RGB color space
// COLOR_SPACE_Lab_D50_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:125
pub const COLOR_SPACE_Lab_D50_10: i32 = 31;
/// non-RGB color space
// COLOR_SPACE_Lab_D50_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:124
pub const COLOR_SPACE_Lab_D50_2: i32 = 30;
/// non-RGB color space
// COLOR_SPACE_Lab_D55_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:129
pub const COLOR_SPACE_Lab_D55_10: i32 = 35;
/// non-RGB color space
// COLOR_SPACE_Lab_D55_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:128
pub const COLOR_SPACE_Lab_D55_2: i32 = 34;
/// non-RGB color space
// COLOR_SPACE_Lab_D65_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:123
pub const COLOR_SPACE_Lab_D65_10: i32 = 29;
/// <https://en.wikipedia.org/wiki/CIELAB_color_space> , non-RGB color space
// COLOR_SPACE_Lab_D65_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:122
pub const COLOR_SPACE_Lab_D65_2: i32 = 28;
/// non-RGB color space
// COLOR_SPACE_Lab_D75_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:131
pub const COLOR_SPACE_Lab_D75_10: i32 = 37;
/// non-RGB color space
// COLOR_SPACE_Lab_D75_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:130
pub const COLOR_SPACE_Lab_D75_2: i32 = 36;
/// non-RGB color space
// COLOR_SPACE_Lab_E_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:133
pub const COLOR_SPACE_Lab_E_10: i32 = 39;
/// non-RGB color space
// COLOR_SPACE_Lab_E_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:132
pub const COLOR_SPACE_Lab_E_2: i32 = 38;
/// <https://en.wikipedia.org/wiki/ProPhoto_RGB_color_space> , RGB color space
// COLOR_SPACE_ProPhotoRGB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:100
pub const COLOR_SPACE_ProPhotoRGB: i32 = 6;
/// <https://en.wikipedia.org/wiki/ProPhoto_RGB_color_space> , linear RGB color space
// COLOR_SPACE_ProPhotoRGBL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:101
pub const COLOR_SPACE_ProPhotoRGBL: i32 = 7;
/// <https://en.wikipedia.org/wiki/Rec._2020> , RGB color space
// COLOR_SPACE_REC_2020_RGB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:108
pub const COLOR_SPACE_REC_2020_RGB: i32 = 14;
/// <https://en.wikipedia.org/wiki/Rec._2020> , linear RGB color space
// COLOR_SPACE_REC_2020_RGBL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:109
pub const COLOR_SPACE_REC_2020_RGBL: i32 = 15;
/// <https://en.wikipedia.org/wiki/Rec._709> , RGB color space
// COLOR_SPACE_REC_709_RGB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:106
pub const COLOR_SPACE_REC_709_RGB: i32 = 12;
/// <https://en.wikipedia.org/wiki/Rec._709> , linear RGB color space
// COLOR_SPACE_REC_709_RGBL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:107
pub const COLOR_SPACE_REC_709_RGBL: i32 = 13;
/// <https://en.wikipedia.org/wiki/Wide-gamut_RGB_color_space> , RGB color space
// COLOR_SPACE_WideGamutRGB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:98
pub const COLOR_SPACE_WideGamutRGB: i32 = 4;
/// <https://en.wikipedia.org/wiki/Wide-gamut_RGB_color_space> , linear RGB color space
// COLOR_SPACE_WideGamutRGBL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:99
pub const COLOR_SPACE_WideGamutRGBL: i32 = 5;
/// non-RGB color space
// COLOR_SPACE_XYZ_A_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:115
pub const COLOR_SPACE_XYZ_A_10: i32 = 21;
/// non-RGB color space
// COLOR_SPACE_XYZ_A_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:114
pub const COLOR_SPACE_XYZ_A_2: i32 = 20;
/// non-RGB color space
// COLOR_SPACE_XYZ_D50_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:113
pub const COLOR_SPACE_XYZ_D50_10: i32 = 19;
/// non-RGB color space
// COLOR_SPACE_XYZ_D50_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:112
pub const COLOR_SPACE_XYZ_D50_2: i32 = 18;
/// non-RGB color space
// COLOR_SPACE_XYZ_D55_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:117
pub const COLOR_SPACE_XYZ_D55_10: i32 = 23;
/// non-RGB color space
// COLOR_SPACE_XYZ_D55_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:116
pub const COLOR_SPACE_XYZ_D55_2: i32 = 22;
/// non-RGB color space
// COLOR_SPACE_XYZ_D65_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:111
pub const COLOR_SPACE_XYZ_D65_10: i32 = 17;
/// <https://en.wikipedia.org/wiki/CIE_1931_color_space> , non-RGB color space
// COLOR_SPACE_XYZ_D65_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:110
pub const COLOR_SPACE_XYZ_D65_2: i32 = 16;
/// non-RGB color space
// COLOR_SPACE_XYZ_D75_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:119
pub const COLOR_SPACE_XYZ_D75_10: i32 = 25;
/// non-RGB color space
// COLOR_SPACE_XYZ_D75_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:118
pub const COLOR_SPACE_XYZ_D75_2: i32 = 24;
/// non-RGB color space
// COLOR_SPACE_XYZ_E_10 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:121
pub const COLOR_SPACE_XYZ_E_10: i32 = 27;
/// non-RGB color space
// COLOR_SPACE_XYZ_E_2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:120
pub const COLOR_SPACE_XYZ_E_2: i32 = 26;
/// <https://en.wikipedia.org/wiki/SRGB> , RGB color space
// COLOR_SPACE_sRGB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:94
pub const COLOR_SPACE_sRGB: i32 = 0;
/// <https://en.wikipedia.org/wiki/SRGB> , linear RGB color space
// COLOR_SPACE_sRGBL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:95
pub const COLOR_SPACE_sRGBL: i32 = 1;
// DISTANCE_CIE2000 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:349
pub const DISTANCE_CIE2000: i32 = 3;
/// The 1976 formula is the first formula that related a measured color difference to a known set of CIELAB coordinates.
// DISTANCE_CIE76 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:346
pub const DISTANCE_CIE76: i32 = 0;
/// The 1976 definition was extended to address perceptual non-uniformities.
// DISTANCE_CIE94_GRAPHIC_ARTS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:347
pub const DISTANCE_CIE94_GRAPHIC_ARTS: i32 = 1;
// DISTANCE_CIE94_TEXTILES /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:348
pub const DISTANCE_CIE94_TEXTILES: i32 = 2;
/// In 1984, the Colour Measurement Committee of the Society of Dyers and Colourists defined a difference measure, also based on the L*C*h color model.
// DISTANCE_CMC_1TO1 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:350
pub const DISTANCE_CMC_1TO1: i32 = 4;
// DISTANCE_CMC_2TO1 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:351
pub const DISTANCE_CMC_2TO1: i32 = 5;
/// Euclidean distance of rgb color space
// DISTANCE_RGB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:352
pub const DISTANCE_RGB: i32 = 6;
/// Euclidean distance of rgbl color space
// DISTANCE_RGBL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:353
pub const DISTANCE_RGBL: i32 = 7;
/// the least square method is an optimal solution under the linear RGB distance function
// INITIAL_METHOD_LEAST_SQUARE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:84
pub const INITIAL_METHOD_LEAST_SQUARE: i32 = 1;
/// The white balance method. The initial value is:
///
// INITIAL_METHOD_WHITE_BALANCE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:69
pub const INITIAL_METHOD_WHITE_BALANCE: i32 = 0;
/// logarithmic polynomial fitting channels respectively; Need assign a value to deg simultaneously
// LINEARIZATION_COLORLOGPOLYFIT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:335
pub const LINEARIZATION_COLORLOGPOLYFIT: i32 = 3;
/// polynomial fitting channels respectively; Need assign a value to deg simultaneously
// LINEARIZATION_COLORPOLYFIT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:334
pub const LINEARIZATION_COLORPOLYFIT: i32 = 2;
/// gamma correction; Need assign a value to gamma simultaneously
// LINEARIZATION_GAMMA /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:333
pub const LINEARIZATION_GAMMA: i32 = 1;
/// grayscale Logarithmic polynomial fitting;  Need assign a value to deg and dst_whites simultaneously
// LINEARIZATION_GRAYLOGPOLYFIT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:337
pub const LINEARIZATION_GRAYLOGPOLYFIT: i32 = 5;
/// grayscale polynomial fitting; Need assign a value to deg and dst_whites simultaneously
// LINEARIZATION_GRAYPOLYFIT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:336
pub const LINEARIZATION_GRAYPOLYFIT: i32 = 4;
/// no change is made
// LINEARIZATION_IDENTITY /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:332
pub const LINEARIZATION_IDENTITY: i32 = 0;
/// Standard Macbeth Chart with 24 squares
// MCC24 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:48
pub const MCC_MCC24: i32 = 0;
/// DigitalSG with 140 squares
// SG140 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:49
pub const MCC_SG140: i32 = 1;
/// DKK color chart with 12 squares and 6 rectangle
// VINYL18 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:50
pub const MCC_VINYL18: i32 = 2;
/// Enum of the possible types of ccm.
// CCM_TYPE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:59
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CCM_TYPE {
	/// The CCM with the shape ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) performs linear transformation on color values.
	CCM_3x3 = 0,
	/// The CCM with the shape ![inline formula](https://latex.codecogs.com/png.latex?4%5Ctimes3) performs affine transformation.
	CCM_4x3 = 1,
}

impl TryFrom<i32> for CCM_TYPE {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::CCM_3x3),
			1 => Ok(Self::CCM_4x3),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mcc::CCM_TYPE"))),
		}
	}
}

opencv_type_enum! { crate::mcc::CCM_TYPE }

// COLOR_SPACE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:93
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum COLOR_SPACE {
	/// <https://en.wikipedia.org/wiki/SRGB> , RGB color space
	COLOR_SPACE_sRGB = 0,
	/// <https://en.wikipedia.org/wiki/SRGB> , linear RGB color space
	COLOR_SPACE_sRGBL = 1,
	/// <https://en.wikipedia.org/wiki/Adobe_RGB_color_space> , RGB color space
	COLOR_SPACE_AdobeRGB = 2,
	/// <https://en.wikipedia.org/wiki/Adobe_RGB_color_space> , linear RGB color space
	COLOR_SPACE_AdobeRGBL = 3,
	/// <https://en.wikipedia.org/wiki/Wide-gamut_RGB_color_space> , RGB color space
	COLOR_SPACE_WideGamutRGB = 4,
	/// <https://en.wikipedia.org/wiki/Wide-gamut_RGB_color_space> , linear RGB color space
	COLOR_SPACE_WideGamutRGBL = 5,
	/// <https://en.wikipedia.org/wiki/ProPhoto_RGB_color_space> , RGB color space
	COLOR_SPACE_ProPhotoRGB = 6,
	/// <https://en.wikipedia.org/wiki/ProPhoto_RGB_color_space> , linear RGB color space
	COLOR_SPACE_ProPhotoRGBL = 7,
	/// <https://en.wikipedia.org/wiki/DCI-P3> , RGB color space
	COLOR_SPACE_DCI_P3_RGB = 8,
	/// <https://en.wikipedia.org/wiki/DCI-P3> , linear RGB color space
	COLOR_SPACE_DCI_P3_RGBL = 9,
	/// <https://en.wikipedia.org/wiki/RGB_color_space> , RGB color space
	COLOR_SPACE_AppleRGB = 10,
	/// <https://en.wikipedia.org/wiki/RGB_color_space> , linear RGB color space
	COLOR_SPACE_AppleRGBL = 11,
	/// <https://en.wikipedia.org/wiki/Rec._709> , RGB color space
	COLOR_SPACE_REC_709_RGB = 12,
	/// <https://en.wikipedia.org/wiki/Rec._709> , linear RGB color space
	COLOR_SPACE_REC_709_RGBL = 13,
	/// <https://en.wikipedia.org/wiki/Rec._2020> , RGB color space
	COLOR_SPACE_REC_2020_RGB = 14,
	/// <https://en.wikipedia.org/wiki/Rec._2020> , linear RGB color space
	COLOR_SPACE_REC_2020_RGBL = 15,
	/// <https://en.wikipedia.org/wiki/CIE_1931_color_space> , non-RGB color space
	COLOR_SPACE_XYZ_D65_2 = 16,
	/// non-RGB color space
	COLOR_SPACE_XYZ_D65_10 = 17,
	/// non-RGB color space
	COLOR_SPACE_XYZ_D50_2 = 18,
	/// non-RGB color space
	COLOR_SPACE_XYZ_D50_10 = 19,
	/// non-RGB color space
	COLOR_SPACE_XYZ_A_2 = 20,
	/// non-RGB color space
	COLOR_SPACE_XYZ_A_10 = 21,
	/// non-RGB color space
	COLOR_SPACE_XYZ_D55_2 = 22,
	/// non-RGB color space
	COLOR_SPACE_XYZ_D55_10 = 23,
	/// non-RGB color space
	COLOR_SPACE_XYZ_D75_2 = 24,
	/// non-RGB color space
	COLOR_SPACE_XYZ_D75_10 = 25,
	/// non-RGB color space
	COLOR_SPACE_XYZ_E_2 = 26,
	/// non-RGB color space
	COLOR_SPACE_XYZ_E_10 = 27,
	/// <https://en.wikipedia.org/wiki/CIELAB_color_space> , non-RGB color space
	COLOR_SPACE_Lab_D65_2 = 28,
	/// non-RGB color space
	COLOR_SPACE_Lab_D65_10 = 29,
	/// non-RGB color space
	COLOR_SPACE_Lab_D50_2 = 30,
	/// non-RGB color space
	COLOR_SPACE_Lab_D50_10 = 31,
	/// non-RGB color space
	COLOR_SPACE_Lab_A_2 = 32,
	/// non-RGB color space
	COLOR_SPACE_Lab_A_10 = 33,
	/// non-RGB color space
	COLOR_SPACE_Lab_D55_2 = 34,
	/// non-RGB color space
	COLOR_SPACE_Lab_D55_10 = 35,
	/// non-RGB color space
	COLOR_SPACE_Lab_D75_2 = 36,
	/// non-RGB color space
	COLOR_SPACE_Lab_D75_10 = 37,
	/// non-RGB color space
	COLOR_SPACE_Lab_E_2 = 38,
	/// non-RGB color space
	COLOR_SPACE_Lab_E_10 = 39,
}

impl TryFrom<i32> for COLOR_SPACE {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::COLOR_SPACE_sRGB),
			1 => Ok(Self::COLOR_SPACE_sRGBL),
			2 => Ok(Self::COLOR_SPACE_AdobeRGB),
			3 => Ok(Self::COLOR_SPACE_AdobeRGBL),
			4 => Ok(Self::COLOR_SPACE_WideGamutRGB),
			5 => Ok(Self::COLOR_SPACE_WideGamutRGBL),
			6 => Ok(Self::COLOR_SPACE_ProPhotoRGB),
			7 => Ok(Self::COLOR_SPACE_ProPhotoRGBL),
			8 => Ok(Self::COLOR_SPACE_DCI_P3_RGB),
			9 => Ok(Self::COLOR_SPACE_DCI_P3_RGBL),
			10 => Ok(Self::COLOR_SPACE_AppleRGB),
			11 => Ok(Self::COLOR_SPACE_AppleRGBL),
			12 => Ok(Self::COLOR_SPACE_REC_709_RGB),
			13 => Ok(Self::COLOR_SPACE_REC_709_RGBL),
			14 => Ok(Self::COLOR_SPACE_REC_2020_RGB),
			15 => Ok(Self::COLOR_SPACE_REC_2020_RGBL),
			16 => Ok(Self::COLOR_SPACE_XYZ_D65_2),
			17 => Ok(Self::COLOR_SPACE_XYZ_D65_10),
			18 => Ok(Self::COLOR_SPACE_XYZ_D50_2),
			19 => Ok(Self::COLOR_SPACE_XYZ_D50_10),
			20 => Ok(Self::COLOR_SPACE_XYZ_A_2),
			21 => Ok(Self::COLOR_SPACE_XYZ_A_10),
			22 => Ok(Self::COLOR_SPACE_XYZ_D55_2),
			23 => Ok(Self::COLOR_SPACE_XYZ_D55_10),
			24 => Ok(Self::COLOR_SPACE_XYZ_D75_2),
			25 => Ok(Self::COLOR_SPACE_XYZ_D75_10),
			26 => Ok(Self::COLOR_SPACE_XYZ_E_2),
			27 => Ok(Self::COLOR_SPACE_XYZ_E_10),
			28 => Ok(Self::COLOR_SPACE_Lab_D65_2),
			29 => Ok(Self::COLOR_SPACE_Lab_D65_10),
			30 => Ok(Self::COLOR_SPACE_Lab_D50_2),
			31 => Ok(Self::COLOR_SPACE_Lab_D50_10),
			32 => Ok(Self::COLOR_SPACE_Lab_A_2),
			33 => Ok(Self::COLOR_SPACE_Lab_A_10),
			34 => Ok(Self::COLOR_SPACE_Lab_D55_2),
			35 => Ok(Self::COLOR_SPACE_Lab_D55_10),
			36 => Ok(Self::COLOR_SPACE_Lab_D75_2),
			37 => Ok(Self::COLOR_SPACE_Lab_D75_10),
			38 => Ok(Self::COLOR_SPACE_Lab_E_2),
			39 => Ok(Self::COLOR_SPACE_Lab_E_10),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mcc::COLOR_SPACE"))),
		}
	}
}

opencv_type_enum! { crate::mcc::COLOR_SPACE }

/// Macbeth and Vinyl ColorChecker with 2deg D50
// CONST_COLOR /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:88
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CONST_COLOR {
	/// Macbeth ColorChecker
	COLORCHECKER_Macbeth = 0,
	/// DKK ColorChecker
	COLORCHECKER_Vinyl = 1,
	/// DigitalSG ColorChecker with 140 squares
	COLORCHECKER_DigitalSG = 2,
}

impl TryFrom<i32> for CONST_COLOR {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::COLORCHECKER_Macbeth),
			1 => Ok(Self::COLORCHECKER_Vinyl),
			2 => Ok(Self::COLORCHECKER_DigitalSG),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mcc::CONST_COLOR"))),
		}
	}
}

opencv_type_enum! { crate::mcc::CONST_COLOR }

/// Enum of possible functions to calculate the distance between colors.
///
/// See <https://en.wikipedia.org/wiki/Color_difference> for details
// DISTANCE_TYPE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:344
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DISTANCE_TYPE {
	/// The 1976 formula is the first formula that related a measured color difference to a known set of CIELAB coordinates.
	DISTANCE_CIE76 = 0,
	/// The 1976 definition was extended to address perceptual non-uniformities.
	DISTANCE_CIE94_GRAPHIC_ARTS = 1,
	DISTANCE_CIE94_TEXTILES = 2,
	DISTANCE_CIE2000 = 3,
	/// In 1984, the Colour Measurement Committee of the Society of Dyers and Colourists defined a difference measure, also based on the L*C*h color model.
	DISTANCE_CMC_1TO1 = 4,
	DISTANCE_CMC_2TO1 = 5,
	/// Euclidean distance of rgb color space
	DISTANCE_RGB = 6,
	/// Euclidean distance of rgbl color space
	DISTANCE_RGBL = 7,
}

impl TryFrom<i32> for DISTANCE_TYPE {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::DISTANCE_CIE76),
			1 => Ok(Self::DISTANCE_CIE94_GRAPHIC_ARTS),
			2 => Ok(Self::DISTANCE_CIE94_TEXTILES),
			3 => Ok(Self::DISTANCE_CIE2000),
			4 => Ok(Self::DISTANCE_CMC_1TO1),
			5 => Ok(Self::DISTANCE_CMC_2TO1),
			6 => Ok(Self::DISTANCE_RGB),
			7 => Ok(Self::DISTANCE_RGBL),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mcc::DISTANCE_TYPE"))),
		}
	}
}

opencv_type_enum! { crate::mcc::DISTANCE_TYPE }

/// Enum of the possible types of initial method.
// INITIAL_METHOD_TYPE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:67
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum INITIAL_METHOD_TYPE {
	/// The white balance method. The initial value is:
	///
	INITIAL_METHOD_WHITE_BALANCE = 0,
	/// the least square method is an optimal solution under the linear RGB distance function
	INITIAL_METHOD_LEAST_SQUARE = 1,
}

impl TryFrom<i32> for INITIAL_METHOD_TYPE {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::INITIAL_METHOD_WHITE_BALANCE),
			1 => Ok(Self::INITIAL_METHOD_LEAST_SQUARE),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mcc::INITIAL_METHOD_TYPE"))),
		}
	}
}

opencv_type_enum! { crate::mcc::INITIAL_METHOD_TYPE }

/// Linearization transformation type
///
/// The first step in color correction is to linearize the detected colors.
/// Because the input color space has not been calibrated, we usually use some empirical methods to linearize.
/// There are several common linearization methods.
/// The first is identical transformation, the second is gamma correction, and the third is polynomial fitting.
///
/// Linearization is generally an elementwise function. The mathematical symbols are as follows:
///
/// ![inline formula](https://latex.codecogs.com/png.latex?C): any channel of a color, could be ![inline formula](https://latex.codecogs.com/png.latex?R%2C%20G) or ![inline formula](https://latex.codecogs.com/png.latex?B).
///
/// ![inline formula](https://latex.codecogs.com/png.latex?R%2C%20G%2C%20%20B):  ![inline formula](https://latex.codecogs.com/png.latex?R%2C%20G%2C%20B) channels respectively.
///
/// ![inline formula](https://latex.codecogs.com/png.latex?G): grayscale;
///
/// ![inline formula](https://latex.codecogs.com/png.latex?s%2Csl): subscript, which represents the detected data and its linearized value, the former is the input and the latter is the output;
///
/// ![inline formula](https://latex.codecogs.com/png.latex?d%2Cdl): subscript, which represents the reference data and its linearized value
///
///
///
/// ### Identical Transformation
///
/// No change is made during the Identical transformation linearization, usually because the tristimulus values of the input RGB image is already proportional to the luminance.
/// For example, if the input measurement data is in RAW format, the measurement data is already linear, so no linearization is required.
///
/// The identity transformation formula is as follows:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%0AC%5F%7Bsl%7D%3DC%5Fs%0A)
///
/// ### Gamma Correction
///
/// Gamma correction is a means of performing nonlinearity in RGB space, see the Color Space documentation for details.
/// In the linearization part, the value of ![inline formula](https://latex.codecogs.com/png.latex?%5Cgamma) is usually set to 2.2.
/// You can also customize the value.
///
/// The formula for gamma correction linearization is as follows:
/// ![block formula](https://latex.codecogs.com/png.latex?%0AC%5F%7Bsl%7D%3DC%5Fs%5E%7B%5Cgamma%7D%2C%5Cqquad%20C%5Fs%5Cge0%5C%5C%0AC%5F%7Bsl%7D%3D%2D%28%2DC%5Fs%29%5E%7B%5Cgamma%7D%2C%5Cqquad%20C%5Fs%3C0%5C%5C%5C%5C%0A)
///
/// ### Polynomial Fitting
///
/// Polynomial fitting uses polynomials to linearize.
/// Provided the polynomial is:
/// ![block formula](https://latex.codecogs.com/png.latex?%0Af%28x%29%3Da%5Fnx%5En%2Ba%5F%7Bn%2D1%7Dx%5E%7Bn%2D1%7D%2B%2E%2E%2E%20%2Ba%5F0%0A)
/// Then:
/// ![block formula](https://latex.codecogs.com/png.latex?%0AC%5F%7Bsl%7D%3Df%28C%5Fs%29%0A)
/// In practice, ![inline formula](https://latex.codecogs.com/png.latex?n%5Cle3) is used to prevent overfitting.
///
/// There are many variants of polynomial fitting, the difference lies in the way of generating ![inline formula](https://latex.codecogs.com/png.latex?f%28x%29).
/// It is usually necessary to use linearized reference colors and corresponding detected colors to calculate the polynomial parameters.
/// However, not all colors can participate in the calculation. The saturation detected colors needs to be removed. See the algorithm introduction document for details.
///
/// #### Fitting Channels Respectively
///
/// Use three polynomials, ![inline formula](https://latex.codecogs.com/png.latex?r%28x%29%2C%20g%28x%29%2C%20b%28x%29),  to linearize each channel of the RGB color space[1-3]:
/// ![block formula](https://latex.codecogs.com/png.latex?%0AR%5F%7Bsl%7D%3Dr%28R%5Fs%29%5C%5C%0AG%5F%7Bsl%7D%3Dg%28G%5Fs%29%5C%5C%0AB%5F%7Bsl%7D%3Db%28B%5Fs%29%5C%5C%0A)
/// The polynomial is generated by minimizing the residual sum of squares between the detected data and the linearized reference data.
/// Take the R-channel as an example:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%0AR%3D%5Carg%20min%5F%7Bf%7D%28%5CSigma%28R%5F%7Bdl%7D%2Df%28R%5FS%29%5E2%29%0A)
///
/// It's equivalent to finding the least square regression for below equations:
/// ![block formula](https://latex.codecogs.com/png.latex?%0Af%28R%5F%7Bs1%7D%29%3DR%5F%7Bdl1%7D%5C%5C%0Af%28R%5F%7Bs2%7D%29%3DR%5F%7Bdl2%7D%5C%5C%0A%2E%2E%2E%0A)
///
/// With a polynomial, the above equations becomes:
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0AR%5F%7Bs1%7D%5E%7Bn%7D%20%26%20R%5F%7Bs1%7D%5E%7Bn%2D1%7D%20%26%20%2E%2E%2E%20%26%201%5C%5C%0AR%5F%7Bs2%7D%5E%7Bn%7D%20%26%20R%5F%7Bs2%7D%5E%7Bn%2D1%7D%20%26%20%2E%2E%2E%20%26%201%5C%5C%0A%2E%2E%2E%20%26%20%2E%2E%2E%20%26%20%2E%2E%2E%20%26%20%2E%2E%2E%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7Bn%7D%5C%5C%0Aa%5F%7Bn%2D1%7D%5C%5C%0A%2E%2E%2E%20%5C%5C%0Aa%5F0%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0AR%5F%7Bdl1%7D%5C%5C%0AR%5F%7Bdl2%7D%5C%5C%0A%2E%2E%2E%0A%5Cend%7Bbmatrix%7D%0A)
/// It can be expressed as a system of linear equations:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%0AAX%3DB%0A)
///
/// When the number of reference colors is not less than the degree of the polynomial, the linear system has a least-squares solution:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%0AX%3D%28A%5ETA%29%5E%7B%2D1%7DA%5ETB%0A)
///
/// Once we get the polynomial coefficients, we can get the polynomial r.
///
/// This method of finding polynomial coefficients can be implemented by numpy.polyfit in numpy, expressed here as:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%0AR%3Dpolyfit%28R%5FS%2C%20R%5F%7Bdl%7D%29%0A)
///
/// Note that, in general, the polynomial that we want to obtain is guaranteed to monotonically increase in the interval [0,1] ,
/// but this means that nonlinear method is needed to generate the polynomials(see [4] for detail).
/// This would greatly increases the complexity of the program.
/// Considering that the monotonicity does not affect the correct operation of the color correction program, polyfit is still used to implement the program.
///
/// Parameters for other channels can also be derived in a similar way.
///
/// #### Grayscale Polynomial Fitting
///
/// In this method[2], single polynomial is used for all channels.
/// The polynomial is still a polyfit result from the detected colors to the linear reference colors.
/// However, only the gray of the reference colors can participate in the calculation.
///
/// Since the detected colors corresponding to the gray of reference colors is not necessarily gray, it needs to be grayed.
/// Grayscale refers to the Y channel of the XYZ color space.
/// The color space of the detected data is not determined and cannot be converted into the XYZ space.
/// Therefore, the sRGB formula is used to approximate[5].
/// ![block formula](https://latex.codecogs.com/png.latex?%0AG%5F%7Bs%7D%3D0%2E2126R%5F%7Bs%7D%2B0%2E7152G%5F%7Bs%7D%2B0%2E0722B%5F%7Bs%7D%0A)
/// Then the polynomial parameters can be obtained by using the polyfit.
/// ![block formula](https://latex.codecogs.com/png.latex?%0Af%3Dpolyfit%28G%5F%7Bs%7D%2C%20G%5F%7Bdl%7D%29%0A)
/// After ![inline formula](https://latex.codecogs.com/png.latex?f) is obtained, linearization can be performed.
///
/// #### Logarithmic Polynomial Fitting
///
/// For gamma correction formula, we take the logarithm:
/// ![block formula](https://latex.codecogs.com/png.latex?%0Aln%28C%5F%7Bsl%7D%29%3D%7B%5Cgamma%7Dln%28C%5Fs%29%2C%5Cqquad%20C%5Fs%5Cge0%5C%0A)
/// It can be seen that there is a linear relationship between ![inline formula](https://latex.codecogs.com/png.latex?ln%28C%5Fs%29) and ![inline formula](https://latex.codecogs.com/png.latex?ln%28C%5F%7Bsl%7D%29). It can be considered that the formula is an approximation of a polynomial relationship, that is, there exists a polynomial ![inline formula](https://latex.codecogs.com/png.latex?f), which makes[2]:
/// ![block formula](https://latex.codecogs.com/png.latex?%0Aln%28C%5F%7Bsl%7D%29%3Df%28ln%28C%5Fs%29%29%2C%20%5Cqquad%20C%5Fs%3E0%5C%5C%0AC%5F%7Bsl%7D%3D0%2C%20%5Cqquad%20C%5Fs%3D0%0A)
///
/// Because ![inline formula](https://latex.codecogs.com/png.latex?exp%28ln%280%29%29%5Cto%5Cinfty%20), the channel whose component is 0 is directly mapped to 0 in the formula above.
///
/// For fitting channels respectively, we have:
/// ![block formula](https://latex.codecogs.com/png.latex?%0Ar%3Dpolyfit%28ln%28R%5Fs%29%2Cln%28R%5F%7Bdl%7D%29%29%5C%5C%0Ag%3Dpolyfit%28ln%28G%5Fs%29%2Cln%28G%5F%7Bdl%7D%29%29%5C%5C%0Ab%3Dpolyfit%28ln%28B%5Fs%29%2Cln%28B%5F%7Bdl%7D%29%29%5C%5C%0A)
/// Note that the parameter of ![inline formula](https://latex.codecogs.com/png.latex?ln%28%2A%29%20) cannot be 0.
/// Therefore, we need to delete the channels whose values are 0 from ![inline formula](https://latex.codecogs.com/png.latex?R%5Fs%20) and ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7Bdl%7D%20), ![inline formula](https://latex.codecogs.com/png.latex?G%5Fs) and ![inline formula](https://latex.codecogs.com/png.latex?G%5F%7Bdl%7D), ![inline formula](https://latex.codecogs.com/png.latex?B%5Fs) and ![inline formula](https://latex.codecogs.com/png.latex?B%5F%7Bdl%7D).
///
/// Therefore:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%0Aln%28R%5F%7Bsl%7D%29%3Dr%28ln%28R%5Fs%29%29%2C%20%5Cqquad%20R%5Fs%3E0%5C%5C%0AR%5F%7Bsl%7D%3D0%2C%20%5Cqquad%20R%5Fs%3D0%5C%5C%0Aln%28G%5F%7Bsl%7D%29%3Dg%28ln%28G%5Fs%29%29%2C%5Cqquad%20G%5Fs%3E0%5C%5C%0AG%5F%7Bsl%7D%3D0%2C%20%5Cqquad%20G%5Fs%3D0%5C%5C%0Aln%28B%5F%7Bsl%7D%29%3Db%28ln%28B%5Fs%29%29%2C%5Cqquad%20B%5Fs%3E0%5C%5C%0AB%5F%7Bsl%7D%3D0%2C%20%5Cqquad%20B%5Fs%3D0%5C%5C%0A)
///
/// For grayscale polynomials, there are also:
/// ![block formula](https://latex.codecogs.com/png.latex?%0Af%3Dpolyfit%28ln%28G%5F%7Bsl%7D%29%2Cln%28G%5F%7Bdl%7D%29%29%0A)
/// and:
/// ![block formula](https://latex.codecogs.com/png.latex?%0Aln%28C%5F%7Bsl%7D%29%3Df%28ln%28C%5Fs%29%29%2C%20%5Cqquad%20C%5Fs%3E0%5C%5C%0AC%5Fsl%3D0%2C%20%5Cqquad%20C%5Fs%3D0%0A)
// LINEAR_TYPE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:329
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LINEAR_TYPE {
	/// no change is made
	LINEARIZATION_IDENTITY = 0,
	/// gamma correction; Need assign a value to gamma simultaneously
	LINEARIZATION_GAMMA = 1,
	/// polynomial fitting channels respectively; Need assign a value to deg simultaneously
	LINEARIZATION_COLORPOLYFIT = 2,
	/// logarithmic polynomial fitting channels respectively; Need assign a value to deg simultaneously
	LINEARIZATION_COLORLOGPOLYFIT = 3,
	/// grayscale polynomial fitting; Need assign a value to deg and dst_whites simultaneously
	LINEARIZATION_GRAYPOLYFIT = 4,
	/// grayscale Logarithmic polynomial fitting;  Need assign a value to deg and dst_whites simultaneously
	LINEARIZATION_GRAYLOGPOLYFIT = 5,
}

impl TryFrom<i32> for LINEAR_TYPE {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::LINEARIZATION_IDENTITY),
			1 => Ok(Self::LINEARIZATION_GAMMA),
			2 => Ok(Self::LINEARIZATION_COLORPOLYFIT),
			3 => Ok(Self::LINEARIZATION_COLORLOGPOLYFIT),
			4 => Ok(Self::LINEARIZATION_GRAYPOLYFIT),
			5 => Ok(Self::LINEARIZATION_GRAYLOGPOLYFIT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mcc::LINEAR_TYPE"))),
		}
	}
}

opencv_type_enum! { crate::mcc::LINEAR_TYPE }

/// TYPECHART
///
/// \brief enum to hold the type of the checker
// TYPECHART /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:46
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MCC_TYPECHART {
	/// Standard Macbeth Chart with 24 squares
	MCC24 = 0,
	/// DigitalSG with 140 squares
	SG140 = 1,
	/// DKK color chart with 12 squares and 6 rectangle
	VINYL18 = 2,
}

impl TryFrom<i32> for MCC_TYPECHART {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::MCC24),
			1 => Ok(Self::SG140),
			2 => Ok(Self::VINYL18),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mcc::MCC_TYPECHART"))),
		}
	}
}

opencv_type_enum! { crate::mcc::MCC_TYPECHART }

/// Constant methods for [crate::mcc::ColorCorrectionModel]
// ColorCorrectionModel /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:360
pub trait ColorCorrectionModelTraitConst {
	fn as_raw_ColorCorrectionModel(&self) -> *const c_void;

	// getCCM()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:496
	// ("cv::ccm::ColorCorrectionModel::getCCM", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_ccm(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_getCCM_const(self.as_raw_ColorCorrectionModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getLoss()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:497
	// ("cv::ccm::ColorCorrectionModel::getLoss", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_loss(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_getLoss_const(self.as_raw_ColorCorrectionModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// get_src_rgbl()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:498
	// ("cv::ccm::ColorCorrectionModel::get_src_rgbl", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_src_rgbl(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_get_src_rgbl_const(self.as_raw_ColorCorrectionModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// get_dst_rgbl()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:499
	// ("cv::ccm::ColorCorrectionModel::get_dst_rgbl", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_dst_rgbl(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_get_dst_rgbl_const(self.as_raw_ColorCorrectionModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMask()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:500
	// ("cv::ccm::ColorCorrectionModel::getMask", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_mask(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_getMask_const(self.as_raw_ColorCorrectionModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getWeights()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:501
	// ("cv::ccm::ColorCorrectionModel::getWeights", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_weights(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_getWeights_const(self.as_raw_ColorCorrectionModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::mcc::ColorCorrectionModel]
pub trait ColorCorrectionModelTrait: crate::mcc::ColorCorrectionModelTraitConst {
	fn as_raw_mut_ColorCorrectionModel(&mut self) -> *mut c_void;

	/// set ColorSpace
	///
	/// Note: It should be some RGB color space;
	/// Supported list of color cards:
	/// - [COLOR_SPACE_sRGB]
	/// - [COLOR_SPACE_AdobeRGB]
	/// - [COLOR_SPACE_WideGamutRGB]
	/// - [COLOR_SPACE_ProPhotoRGB]
	/// - [COLOR_SPACE_DCI_P3_RGB]
	/// - [COLOR_SPACE_AppleRGB]
	/// - [COLOR_SPACE_REC_709_RGB]
	/// - [COLOR_SPACE_REC_2020_RGB]
	/// ## Parameters
	/// * cs: the absolute color space that detected colors convert to;
	///
	///           default: [COLOR_SPACE_sRGB]
	// setColorSpace(COLOR_SPACE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:409
	// ("cv::ccm::ColorCorrectionModel::setColorSpace", vec![(pred!(mut, ["cs"], ["cv::ccm::COLOR_SPACE"]), _)]),
	#[inline]
	fn set_color_space(&mut self, cs: crate::mcc::COLOR_SPACE) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setColorSpace_COLOR_SPACE(self.as_raw_mut_ColorCorrectionModel(), cs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set ccm_type
	/// ## Parameters
	/// * ccm_type: the shape of color correction matrix(CCM);
	///
	///                default: [CCM_3x3]
	// setCCM_TYPE(CCM_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:415
	// ("cv::ccm::ColorCorrectionModel::setCCM_TYPE", vec![(pred!(mut, ["ccm_type"], ["cv::ccm::CCM_TYPE"]), _)]),
	#[inline]
	fn set_ccm_type(&mut self, ccm_type: crate::mcc::CCM_TYPE) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setCCM_TYPE_CCM_TYPE(self.as_raw_mut_ColorCorrectionModel(), ccm_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set Distance
	/// ## Parameters
	/// * distance: the type of color distance;
	///
	///                default: [DISTANCE_CIE2000]
	// setDistance(DISTANCE_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:421
	// ("cv::ccm::ColorCorrectionModel::setDistance", vec![(pred!(mut, ["distance"], ["cv::ccm::DISTANCE_TYPE"]), _)]),
	#[inline]
	fn set_distance(&mut self, distance: crate::mcc::DISTANCE_TYPE) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setDistance_DISTANCE_TYPE(self.as_raw_mut_ColorCorrectionModel(), distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set Linear
	/// ## Parameters
	/// * linear_type: the method of linearization;
	///
	///                   default: [LINEARIZATION_GAMMA]
	// setLinear(LINEAR_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:427
	// ("cv::ccm::ColorCorrectionModel::setLinear", vec![(pred!(mut, ["linear_type"], ["cv::ccm::LINEAR_TYPE"]), _)]),
	#[inline]
	fn set_linear(&mut self, linear_type: crate::mcc::LINEAR_TYPE) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setLinear_LINEAR_TYPE(self.as_raw_mut_ColorCorrectionModel(), linear_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set Gamma
	///
	///
	/// Note: only valid when linear is set to "gamma";
	///
	///
	/// ## Parameters
	/// * gamma: the gamma value of gamma correction;
	///
	///              default: 2.2;
	// setLinearGamma(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:436
	// ("cv::ccm::ColorCorrectionModel::setLinearGamma", vec![(pred!(mut, ["gamma"], ["const double*"]), _)]),
	#[inline]
	fn set_linear_gamma(&mut self, gamma: &f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setLinearGamma_const_doubleR(self.as_raw_mut_ColorCorrectionModel(), gamma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set degree
	///     
	/// Note: only valid when linear is set to
	///    - [LINEARIZATION_COLORPOLYFIT]
	///    - [LINEARIZATION_GRAYPOLYFIT]
	///    - [LINEARIZATION_COLORLOGPOLYFIT]
	///    - [LINEARIZATION_GRAYLOGPOLYFIT]
	///
	/// ## Parameters
	/// * deg: the degree of linearization polynomial;
	///
	///        default: 3
	// setLinearDegree(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:449
	// ("cv::ccm::ColorCorrectionModel::setLinearDegree", vec![(pred!(mut, ["deg"], ["const int*"]), _)]),
	#[inline]
	fn set_linear_degree(&mut self, deg: &i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setLinearDegree_const_intR(self.as_raw_mut_ColorCorrectionModel(), deg, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set SaturatedThreshold.
	///            The colors in the closed interval [lower, upper] are reserved to participate
	///            in the calculation of the loss function and initialization parameters
	/// ## Parameters
	/// * lower: the lower threshold to determine saturation;
	///
	///        default: 0;
	/// * upper: the upper threshold to determine saturation;
	///
	///              default: 0
	// setSaturatedThreshold(const double &, const double &)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:459
	// ("cv::ccm::ColorCorrectionModel::setSaturatedThreshold", vec![(pred!(mut, ["lower", "upper"], ["const double*", "const double*"]), _)]),
	#[inline]
	fn set_saturated_threshold(&mut self, lower: &f64, upper: &f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setSaturatedThreshold_const_doubleR_const_doubleR(self.as_raw_mut_ColorCorrectionModel(), lower, upper, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set WeightsList
	/// ## Parameters
	/// * weights_list: the list of weight of each color;
	///
	///                    default: empty array
	// setWeightsList(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:465
	// ("cv::ccm::ColorCorrectionModel::setWeightsList", vec![(pred!(mut, ["weights_list"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_weights_list(&mut self, weights_list: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setWeightsList_const_MatR(self.as_raw_mut_ColorCorrectionModel(), weights_list.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set WeightCoeff
	/// ## Parameters
	/// * weights_coeff: the exponent number of L* component of the reference color in CIE Lab color space;
	///
	///                      default: 0
	// setWeightCoeff(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:471
	// ("cv::ccm::ColorCorrectionModel::setWeightCoeff", vec![(pred!(mut, ["weights_coeff"], ["const double*"]), _)]),
	#[inline]
	fn set_weight_coeff(&mut self, weights_coeff: &f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setWeightCoeff_const_doubleR(self.as_raw_mut_ColorCorrectionModel(), weights_coeff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set InitialMethod
	/// ## Parameters
	/// * initial_method_type: the method of calculating CCM initial value;
	///
	///        default: INITIAL_METHOD_LEAST_SQUARE
	// setInitialMethod(INITIAL_METHOD_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:477
	// ("cv::ccm::ColorCorrectionModel::setInitialMethod", vec![(pred!(mut, ["initial_method_type"], ["cv::ccm::INITIAL_METHOD_TYPE"]), _)]),
	#[inline]
	fn set_initial_method(&mut self, initial_method_type: crate::mcc::INITIAL_METHOD_TYPE) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setInitialMethod_INITIAL_METHOD_TYPE(self.as_raw_mut_ColorCorrectionModel(), initial_method_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set MaxCount
	/// ## Parameters
	/// * max_count: used in MinProblemSolver-DownhillSolver;
	///
	///    Terminal criteria to the algorithm;
	///
	///                  default: 5000;
	// setMaxCount(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:484
	// ("cv::ccm::ColorCorrectionModel::setMaxCount", vec![(pred!(mut, ["max_count"], ["const int*"]), _)]),
	#[inline]
	fn set_max_count(&mut self, max_count: &i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setMaxCount_const_intR(self.as_raw_mut_ColorCorrectionModel(), max_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set Epsilon
	/// ## Parameters
	/// * epsilon: used in MinProblemSolver-DownhillSolver;
	///
	///    Terminal criteria to the algorithm;
	///
	///               default: 1e-4;
	// setEpsilon(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:491
	// ("cv::ccm::ColorCorrectionModel::setEpsilon", vec![(pred!(mut, ["epsilon"], ["const double*"]), _)]),
	#[inline]
	fn set_epsilon(&mut self, epsilon: &f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_setEpsilon_const_doubleR(self.as_raw_mut_ColorCorrectionModel(), epsilon, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// make color correction
	// run()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:494
	// ("cv::ccm::ColorCorrectionModel::run", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn run(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_run(self.as_raw_mut_ColorCorrectionModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Infer using fitting ccm.
	/// ## Parameters
	/// * img: the input image.
	/// * islinear: default false.
	/// ## Returns
	/// the output array.
	///
	/// ## C++ default parameters
	/// * islinear: false
	// infer(const Mat &, bool)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:508
	// ("cv::ccm::ColorCorrectionModel::infer", vec![(pred!(mut, ["img", "islinear"], ["const cv::Mat*", "bool"]), _)]),
	#[inline]
	fn infer(&mut self, img: &impl core::MatTraitConst, islinear: bool) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_infer_const_MatR_bool(self.as_raw_mut_ColorCorrectionModel(), img.as_raw_Mat(), islinear, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Infer using fitting ccm.
	/// ## Parameters
	/// * img: the input image.
	/// * islinear: default false.
	/// ## Returns
	/// the output array.
	///
	/// ## Note
	/// This alternative version of [ColorCorrectionModelTrait::infer] function uses the following default values for its arguments:
	/// * islinear: false
	// cv::ccm::ColorCorrectionModel::infer(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:508
	// ("cv::ccm::ColorCorrectionModel::infer", vec![(pred!(mut, ["img"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn infer_def(&mut self, img: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_infer_const_MatR(self.as_raw_mut_ColorCorrectionModel(), img.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Core class of ccm model
///
/// Produce a ColorCorrectionModel instance for inference
// ColorCorrectionModel /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:360
pub struct ColorCorrectionModel {
	ptr: *mut c_void,
}

opencv_type_boxed! { ColorCorrectionModel }

impl Drop for ColorCorrectionModel {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_ccm_ColorCorrectionModel_delete(self.as_raw_mut_ColorCorrectionModel()) };
	}
}

unsafe impl Send for ColorCorrectionModel {}

impl crate::mcc::ColorCorrectionModelTraitConst for ColorCorrectionModel {
	#[inline] fn as_raw_ColorCorrectionModel(&self) -> *const c_void { self.as_raw() }
}

impl crate::mcc::ColorCorrectionModelTrait for ColorCorrectionModel {
	#[inline] fn as_raw_mut_ColorCorrectionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ColorCorrectionModel, crate::mcc::ColorCorrectionModelTraitConst, as_raw_ColorCorrectionModel, crate::mcc::ColorCorrectionModelTrait, as_raw_mut_ColorCorrectionModel }

impl ColorCorrectionModel {
	/// Color Correction Model
	///
	/// Supported list of color cards:
	/// - [COLORCHECKER_Macbeth], the Macbeth ColorChecker
	/// - [COLORCHECKER_Vinyl], the DKK ColorChecker
	/// - [COLORCHECKER_DigitalSG], the DigitalSG ColorChecker with 140 squares
	///
	/// ## Parameters
	/// * src: detected colors of ColorChecker patches;
	///
	///            the color type is RGB not BGR, and the color values are in [0, 1];
	/// * constcolor: the Built-in color card
	// ColorCorrectionModel(const Mat &, CONST_COLOR)(TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:374
	// ("cv::ccm::ColorCorrectionModel::ColorCorrectionModel", vec![(pred!(mut, ["src", "constcolor"], ["const cv::Mat*", "cv::ccm::CONST_COLOR"]), _)]),
	#[inline]
	pub fn new(src: &impl core::MatTraitConst, constcolor: crate::mcc::CONST_COLOR) -> Result<crate::mcc::ColorCorrectionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_CONST_COLOR(src.as_raw_Mat(), constcolor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mcc::ColorCorrectionModel::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Color Correction Model
	/// ## Parameters
	/// * src: detected colors of ColorChecker patches;
	///
	///           the color type is RGB not BGR, and the color values are in [0, 1];
	/// * colors: the reference color values, the color values are in [0, 1].
	///
	/// * ref_cs: the corresponding color space
	///            If the color type is some RGB, the format is RGB not BGR;
	///
	// ColorCorrectionModel(const Mat &, Mat, COLOR_SPACE)(TraitClass, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:383
	// ("cv::ccm::ColorCorrectionModel::ColorCorrectionModel", vec![(pred!(mut, ["src", "colors", "ref_cs"], ["const cv::Mat*", "cv::Mat", "cv::ccm::COLOR_SPACE"]), _)]),
	#[inline]
	pub fn new_1(src: &impl core::MatTraitConst, mut colors: impl core::MatTrait, ref_cs: crate::mcc::COLOR_SPACE) -> Result<crate::mcc::ColorCorrectionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE(src.as_raw_Mat(), colors.as_raw_mut_Mat(), ref_cs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mcc::ColorCorrectionModel::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Color Correction Model
	/// ## Parameters
	/// * src: detected colors of ColorChecker patches;
	///
	///            the color type is RGB not BGR, and the color values are in [0, 1];
	/// * colors: the reference color values, the color values are in [0, 1].
	/// * ref_cs: the corresponding color space
	///            If the color type is some RGB, the format is RGB not BGR;
	/// * colored: mask of colored color
	// ColorCorrectionModel(const Mat &, Mat, COLOR_SPACE, Mat)(TraitClass, TraitClass, Enum, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:393
	// ("cv::ccm::ColorCorrectionModel::ColorCorrectionModel", vec![(pred!(mut, ["src", "colors", "ref_cs", "colored"], ["const cv::Mat*", "cv::Mat", "cv::ccm::COLOR_SPACE", "cv::Mat"]), _)]),
	#[inline]
	pub fn new_2(src: &impl core::MatTraitConst, mut colors: impl core::MatTrait, ref_cs: crate::mcc::COLOR_SPACE, mut colored: impl core::MatTrait) -> Result<crate::mcc::ColorCorrectionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE_Mat(src.as_raw_Mat(), colors.as_raw_mut_Mat(), ref_cs, colored.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mcc::ColorCorrectionModel::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for ColorCorrectionModel {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ColorCorrectionModel")
			.finish()
	}
}

/// Constant methods for [crate::mcc::MCC_CChecker]
// CChecker /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:64
pub trait MCC_CCheckerTraitConst {
	fn as_raw_MCC_CChecker(&self) -> *const c_void;

}

/// Mutable methods for [crate::mcc::MCC_CChecker]
pub trait MCC_CCheckerTrait: crate::mcc::MCC_CCheckerTraitConst {
	fn as_raw_mut_MCC_CChecker(&mut self) -> *mut c_void;

	// setTarget(TYPECHART)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:83
	// ("cv::mcc::CChecker::setTarget", vec![(pred!(mut, ["_target"], ["cv::mcc::TYPECHART"]), _)]),
	#[inline]
	fn set_target(&mut self, _target: crate::mcc::MCC_TYPECHART) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_setTarget_TYPECHART(self.as_raw_mut_MCC_CChecker(), _target, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBox(std::vector<Point2f>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:84
	// ("cv::mcc::CChecker::setBox", vec![(pred!(mut, ["_box"], ["std::vector<cv::Point2f>"]), _)]),
	#[inline]
	fn set_box(&mut self, mut _box: core::Vector<core::Point2f>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_setBox_vectorLPoint2fG(self.as_raw_mut_MCC_CChecker(), _box.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setChartsRGB(Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:85
	// ("cv::mcc::CChecker::setChartsRGB", vec![(pred!(mut, ["_chartsRGB"], ["cv::Mat"]), _)]),
	#[inline]
	fn set_charts_rgb(&mut self, mut _charts_rgb: impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_setChartsRGB_Mat(self.as_raw_mut_MCC_CChecker(), _charts_rgb.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setChartsYCbCr(Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:86
	// ("cv::mcc::CChecker::setChartsYCbCr", vec![(pred!(mut, ["_chartsYCbCr"], ["cv::Mat"]), _)]),
	#[inline]
	fn set_charts_y_cb_cr(&mut self, mut _charts_y_cb_cr: impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_setChartsYCbCr_Mat(self.as_raw_mut_MCC_CChecker(), _charts_y_cb_cr.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCost(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:87
	// ("cv::mcc::CChecker::setCost", vec![(pred!(mut, ["_cost"], ["float"]), _)]),
	#[inline]
	fn set_cost(&mut self, _cost: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_setCost_float(self.as_raw_mut_MCC_CChecker(), _cost, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCenter(Point2f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:88
	// ("cv::mcc::CChecker::setCenter", vec![(pred!(mut, ["_center"], ["cv::Point2f"]), _)]),
	#[inline]
	fn set_center(&mut self, _center: core::Point2f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_setCenter_Point2f(self.as_raw_mut_MCC_CChecker(), &_center, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getTarget()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:90
	// ("cv::mcc::CChecker::getTarget", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_target(&mut self) -> Result<crate::mcc::MCC_TYPECHART> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_getTarget(self.as_raw_mut_MCC_CChecker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBox()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:91
	// ("cv::mcc::CChecker::getBox", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_box(&mut self) -> Result<core::Vector<core::Point2f>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_getBox(self.as_raw_mut_MCC_CChecker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Computes and returns the coordinates of the central parts of the charts modules.
	///
	/// This method computes transformation matrix from the checkers's coordinates (`cv::mcc::CChecker::getBox()`)
	/// and find by this the coordinates of the central parts of the charts modules.
	/// It is used in `cv::mcc::CCheckerDraw::draw()` and in `ChartsRGB` calculation.
	// getColorCharts()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:99
	// ("cv::mcc::CChecker::getColorCharts", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_color_charts(&mut self) -> Result<core::Vector<core::Point2f>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_getColorCharts(self.as_raw_mut_MCC_CChecker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getChartsRGB()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:101
	// ("cv::mcc::CChecker::getChartsRGB", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_charts_rgb(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_getChartsRGB(self.as_raw_mut_MCC_CChecker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getChartsYCbCr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:102
	// ("cv::mcc::CChecker::getChartsYCbCr", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_charts_y_cb_cr(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_getChartsYCbCr(self.as_raw_mut_MCC_CChecker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getCost()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:103
	// ("cv::mcc::CChecker::getCost", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_cost(&mut self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_getCost(self.as_raw_mut_MCC_CChecker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCenter()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:104
	// ("cv::mcc::CChecker::getCenter", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_center(&mut self) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_getCenter(self.as_raw_mut_MCC_CChecker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// CChecker
///
/// \brief checker object
///
///    This class contains the information about the detected checkers,i.e, their
///    type, the corners of the chart, the color profile, the cost, centers chart,
///    etc.
// CChecker /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:64
pub struct MCC_CChecker {
	ptr: *mut c_void,
}

opencv_type_boxed! { MCC_CChecker }

impl Drop for MCC_CChecker {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_mcc_CChecker_delete(self.as_raw_mut_MCC_CChecker()) };
	}
}

unsafe impl Send for MCC_CChecker {}

impl crate::mcc::MCC_CCheckerTraitConst for MCC_CChecker {
	#[inline] fn as_raw_MCC_CChecker(&self) -> *const c_void { self.as_raw() }
}

impl crate::mcc::MCC_CCheckerTrait for MCC_CChecker {
	#[inline] fn as_raw_mut_MCC_CChecker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MCC_CChecker, crate::mcc::MCC_CCheckerTraitConst, as_raw_MCC_CChecker, crate::mcc::MCC_CCheckerTrait, as_raw_mut_MCC_CChecker }

impl MCC_CChecker {
	/// \brief Create a new CChecker object.
	/// \return A pointer to the implementation of the CChecker
	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:73
	// ("cv::mcc::CChecker::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::mcc::MCC_CChecker>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CChecker_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mcc::MCC_CChecker>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for MCC_CChecker {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MCC_CChecker")
			.finish()
	}
}

/// Constant methods for [crate::mcc::MCC_CCheckerDetector]
// CCheckerDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:129
pub trait MCC_CCheckerDetectorTraitConst: core::AlgorithmTraitConst {
	fn as_raw_MCC_CCheckerDetector(&self) -> *const c_void;

}

/// Mutable methods for [crate::mcc::MCC_CCheckerDetector]
pub trait MCC_CCheckerDetectorTrait: core::AlgorithmTrait + crate::mcc::MCC_CCheckerDetectorTraitConst {
	fn as_raw_mut_MCC_CCheckerDetector(&mut self) -> *mut c_void;

	/// \brief Set the net which will be used to find the approximate
	///        bounding boxes for the color charts.
	///
	/// It is not necessary to use this, but this usually results in
	/// better detection rate.
	///
	/// \param net the neural network, if the network in empty, then
	///            the function will return false.
	/// \return true if it was able to set the detector's network,
	///        false otherwise.
	// setNet(dnn::Net)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:144
	// ("cv::mcc::CCheckerDetector::setNet", vec![(pred!(mut, ["net"], ["cv::dnn::Net"]), _)]),
	#[inline]
	fn set_net(&mut self, mut net: impl crate::dnn::NetTrait) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CCheckerDetector_setNet_Net(self.as_raw_mut_MCC_CCheckerDetector(), net.as_raw_mut_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Find the ColorCharts in the given image.
	///
	/// The found charts are not returned but instead stored in the
	/// detector, these can be accessed later on using getBestColorChecker()
	/// and getListColorChecker()
	/// \param image image in color space BGR
	/// \param chartType type of the chart to detect
	/// \param regionsOfInterest regions of image to look for the chart, if
	///                          it is empty, charts are looked for in the
	///                          entire image
	/// \param nc number of charts in the image, if you don't know the exact
	///           then keeping this number high helps.
	/// \param useNet if it is true the network provided using the setNet()
	///               is used for preliminary search for regions where chart
	///               could be present, inside the regionsOfInterest provied.
	/// \param params parameters of the detection system. More information
	///               about them can be found in the struct DetectorParameters.
	/// \return true if atleast one chart is detected otherwise false
	///
	/// ## C++ default parameters
	/// * nc: 1
	/// * use_net: false
	/// * params: DetectorParameters::create()
	// process(InputArray, const TYPECHART, const std::vector<Rect> &, const int, bool, const Ptr<DetectorParameters> &)(InputArray, Enum, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:167
	// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType", "regionsOfInterest", "nc", "useNet", "params"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART", "const std::vector<cv::Rect>*", "const int", "bool", "const cv::Ptr<cv::mcc::DetectorParameters>*"]), _)]),
	#[inline]
	fn process_with_roi(&mut self, image: &impl ToInputArray, chart_type: crate::mcc::MCC_TYPECHART, regions_of_interest: &core::Vector<core::Rect>, nc: i32, use_net: bool, params: &core::Ptr<crate::mcc::MCC_DetectorParameters>) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_vectorLRectGR_const_int_bool_const_PtrLDetectorParametersGR(self.as_raw_mut_MCC_CCheckerDetector(), image.as_raw__InputArray(), chart_type, regions_of_interest.as_raw_VectorOfRect(), nc, use_net, params.as_raw_PtrOfMCC_DetectorParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Find the ColorCharts in the given image.
	///
	/// The found charts are not returned but instead stored in the
	/// detector, these can be accessed later on using getBestColorChecker()
	/// and getListColorChecker()
	/// \param image image in color space BGR
	/// \param chartType type of the chart to detect
	/// \param regionsOfInterest regions of image to look for the chart, if
	///                          it is empty, charts are looked for in the
	///                          entire image
	/// \param nc number of charts in the image, if you don't know the exact
	///           then keeping this number high helps.
	/// \param useNet if it is true the network provided using the setNet()
	///               is used for preliminary search for regions where chart
	///               could be present, inside the regionsOfInterest provied.
	/// \param params parameters of the detection system. More information
	///               about them can be found in the struct DetectorParameters.
	/// \return true if atleast one chart is detected otherwise false
	///
	/// ## Note
	/// This alternative version of [MCC_CCheckerDetectorTrait::process_with_roi] function uses the following default values for its arguments:
	/// * nc: 1
	/// * use_net: false
	/// * params: DetectorParameters::create()
	// cv::mcc::CCheckerDetector::process(InputArray, Enum, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:167
	// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType", "regionsOfInterest"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART", "const std::vector<cv::Rect>*"]), _)]),
	#[inline]
	fn process_with_roi_def(&mut self, image: &impl ToInputArray, chart_type: crate::mcc::MCC_TYPECHART, regions_of_interest: &core::Vector<core::Rect>) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_vectorLRectGR(self.as_raw_mut_MCC_CCheckerDetector(), image.as_raw__InputArray(), chart_type, regions_of_interest.as_raw_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Find the ColorCharts in the given image.
	///
	/// Differs from the above one only in the arguments.
	///
	/// This version searches for the chart in the full image.
	///
	/// The found charts are not returned but instead stored in the
	/// detector, these can be accessed later on using getBestColorChecker()
	/// and getListColorChecker()
	/// \param image image in color space BGR
	/// \param chartType type of the chart to detect
	/// \param nc number of charts in the image, if you don't know the exact
	///           then keeping this number high helps.
	/// \param useNet if it is true the network provided using the setNet()
	///               is used for preliminary search for regions where chart
	///               could be present, inside the regionsOfInterest provied.
	/// \param params parameters of the detection system. More information
	///               about them can be found in the struct DetectorParameters.
	/// \return true if atleast one chart is detected otherwise false
	///
	/// ## C++ default parameters
	/// * nc: 1
	/// * use_net: false
	/// * params: DetectorParameters::create()
	// process(InputArray, const TYPECHART, const int, bool, const Ptr<DetectorParameters> &)(InputArray, Enum, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:195
	// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType", "nc", "useNet", "params"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART", "const int", "bool", "const cv::Ptr<cv::mcc::DetectorParameters>*"]), _)]),
	#[inline]
	fn process(&mut self, image: &impl ToInputArray, chart_type: crate::mcc::MCC_TYPECHART, nc: i32, use_net: bool, params: &core::Ptr<crate::mcc::MCC_DetectorParameters>) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_int_bool_const_PtrLDetectorParametersGR(self.as_raw_mut_MCC_CCheckerDetector(), image.as_raw__InputArray(), chart_type, nc, use_net, params.as_raw_PtrOfMCC_DetectorParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Find the ColorCharts in the given image.
	///
	/// Differs from the above one only in the arguments.
	///
	/// This version searches for the chart in the full image.
	///
	/// The found charts are not returned but instead stored in the
	/// detector, these can be accessed later on using getBestColorChecker()
	/// and getListColorChecker()
	/// \param image image in color space BGR
	/// \param chartType type of the chart to detect
	/// \param nc number of charts in the image, if you don't know the exact
	///           then keeping this number high helps.
	/// \param useNet if it is true the network provided using the setNet()
	///               is used for preliminary search for regions where chart
	///               could be present, inside the regionsOfInterest provied.
	/// \param params parameters of the detection system. More information
	///               about them can be found in the struct DetectorParameters.
	/// \return true if atleast one chart is detected otherwise false
	///
	/// ## Note
	/// This alternative version of [MCC_CCheckerDetectorTrait::process] function uses the following default values for its arguments:
	/// * nc: 1
	/// * use_net: false
	/// * params: DetectorParameters::create()
	// cv::mcc::CCheckerDetector::process(InputArray, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:195
	// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART"]), _)]),
	#[inline]
	fn process_def(&mut self, image: &impl ToInputArray, chart_type: crate::mcc::MCC_TYPECHART) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART(self.as_raw_mut_MCC_CCheckerDetector(), image.as_raw__InputArray(), chart_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Get the best color checker. By the best it means the one
	///        detected with the highest confidence.
	/// \return checker A single colorchecker, if atleast one colorchecker
	///                was detected, 'nullptr' otherwise.
	// getBestColorChecker()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:204
	// ("cv::mcc::CCheckerDetector::getBestColorChecker", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_best_color_checker(&mut self) -> Result<core::Ptr<crate::mcc::MCC_CChecker>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CCheckerDetector_getBestColorChecker(self.as_raw_mut_MCC_CCheckerDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mcc::MCC_CChecker>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// \brief Get the list of all detected colorcheckers
	/// \return checkers vector of colorcheckers
	// getListColorChecker()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:209
	// ("cv::mcc::CCheckerDetector::getListColorChecker", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_list_color_checker(&mut self) -> Result<core::Vector<core::Ptr<crate::mcc::MCC_CChecker>>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CCheckerDetector_getListColorChecker(self.as_raw_mut_MCC_CCheckerDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Ptr<crate::mcc::MCC_CChecker>>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// A class to find the positions of the ColorCharts in the image.
// CCheckerDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:129
pub struct MCC_CCheckerDetector {
	ptr: *mut c_void,
}

opencv_type_boxed! { MCC_CCheckerDetector }

impl Drop for MCC_CCheckerDetector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_mcc_CCheckerDetector_delete(self.as_raw_mut_MCC_CCheckerDetector()) };
	}
}

unsafe impl Send for MCC_CCheckerDetector {}

impl core::AlgorithmTraitConst for MCC_CCheckerDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MCC_CCheckerDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MCC_CCheckerDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::mcc::MCC_CCheckerDetectorTraitConst for MCC_CCheckerDetector {
	#[inline] fn as_raw_MCC_CCheckerDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::mcc::MCC_CCheckerDetectorTrait for MCC_CCheckerDetector {
	#[inline] fn as_raw_mut_MCC_CCheckerDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MCC_CCheckerDetector, crate::mcc::MCC_CCheckerDetectorTraitConst, as_raw_MCC_CCheckerDetector, crate::mcc::MCC_CCheckerDetectorTrait, as_raw_mut_MCC_CCheckerDetector }

impl MCC_CCheckerDetector {
	/// \brief Returns the implementation of the CCheckerDetector.
	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:214
	// ("cv::mcc::CCheckerDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::mcc::MCC_CCheckerDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CCheckerDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mcc::MCC_CCheckerDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MCC_CCheckerDetector, core::Algorithm, cv_mcc_CCheckerDetector_to_Algorithm }

impl std::fmt::Debug for MCC_CCheckerDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MCC_CCheckerDetector")
			.finish()
	}
}

/// Constant methods for [crate::mcc::MCC_CCheckerDraw]
// CCheckerDraw /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:121
pub trait MCC_CCheckerDrawTraitConst {
	fn as_raw_MCC_CCheckerDraw(&self) -> *const c_void;

}

/// Mutable methods for [crate::mcc::MCC_CCheckerDraw]
pub trait MCC_CCheckerDrawTrait: crate::mcc::MCC_CCheckerDrawTraitConst {
	fn as_raw_mut_MCC_CCheckerDraw(&mut self) -> *mut c_void;

	/// \brief Draws the checker to the given image.
	/// \param img image in color space BGR
	// draw(InputOutputArray)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:129
	// ("cv::mcc::CCheckerDraw::draw", vec![(pred!(mut, ["img"], ["const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn draw(&mut self, img: &mut impl ToInputOutputArray) -> Result<()> {
		input_output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CCheckerDraw_draw_const__InputOutputArrayR(self.as_raw_mut_MCC_CCheckerDraw(), img.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// \brief checker draw
///
/// This class contains the functions for drawing a detected chart.  This class
/// expects a pointer to the checker which will be drawn by this object in the
/// constructor and then later on whenever the draw function is called the
/// checker will be drawn. Remember that it is not possible to change the
/// checkers which will be draw by a given object, as it is decided in the
/// constructor itself. If you want to draw some other object you can create a
/// new CCheckerDraw instance.
///
/// The reason for this type of design is that in some videos we can assume that
/// the checker is always in the same position, even if the image changes, so
/// the drawing will always take place at the same position.
// CCheckerDraw /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:121
pub struct MCC_CCheckerDraw {
	ptr: *mut c_void,
}

opencv_type_boxed! { MCC_CCheckerDraw }

impl Drop for MCC_CCheckerDraw {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_mcc_CCheckerDraw_delete(self.as_raw_mut_MCC_CCheckerDraw()) };
	}
}

unsafe impl Send for MCC_CCheckerDraw {}

impl crate::mcc::MCC_CCheckerDrawTraitConst for MCC_CCheckerDraw {
	#[inline] fn as_raw_MCC_CCheckerDraw(&self) -> *const c_void { self.as_raw() }
}

impl crate::mcc::MCC_CCheckerDrawTrait for MCC_CCheckerDraw {
	#[inline] fn as_raw_mut_MCC_CCheckerDraw(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MCC_CCheckerDraw, crate::mcc::MCC_CCheckerDrawTraitConst, as_raw_MCC_CCheckerDraw, crate::mcc::MCC_CCheckerDrawTrait, as_raw_mut_MCC_CCheckerDraw }

impl MCC_CCheckerDraw {
	/// \brief Create a new CCheckerDraw object.
	/// \param pChecker The checker which will be drawn by this object.
	/// \param color The color by with which the squares of the checker
	///              will be drawn
	/// \param thickness The thickness with which the sqaures will be
	///                  drawn
	/// \return A pointer to the implementation of the CCheckerDraw
	///
	/// ## C++ default parameters
	/// * color: CV_RGB(0,250,0)
	/// * thickness: 2
	// create(Ptr<CChecker>, cv::Scalar, int)(CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:138
	// ("cv::mcc::CCheckerDraw::create", vec![(pred!(mut, ["pChecker", "color", "thickness"], ["cv::Ptr<cv::mcc::CChecker>", "cv::Scalar", "int"]), _)]),
	#[inline]
	pub fn create(mut p_checker: core::Ptr<crate::mcc::MCC_CChecker>, color: core::Scalar, thickness: i32) -> Result<core::Ptr<crate::mcc::MCC_CCheckerDraw>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CCheckerDraw_create_PtrLCCheckerG_Scalar_int(p_checker.as_raw_mut_PtrOfMCC_CChecker(), &color, thickness, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mcc::MCC_CCheckerDraw>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// \brief Create a new CCheckerDraw object.
	/// \param pChecker The checker which will be drawn by this object.
	/// \param color The color by with which the squares of the checker
	///              will be drawn
	/// \param thickness The thickness with which the sqaures will be
	///                  drawn
	/// \return A pointer to the implementation of the CCheckerDraw
	///
	/// ## Note
	/// This alternative version of [MCC_CCheckerDraw::create] function uses the following default values for its arguments:
	/// * color: CV_RGB(0,250,0)
	/// * thickness: 2
	// cv::mcc::CCheckerDraw::create(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:138
	// ("cv::mcc::CCheckerDraw::create", vec![(pred!(mut, ["pChecker"], ["cv::Ptr<cv::mcc::CChecker>"]), _)]),
	#[inline]
	pub fn create_def(mut p_checker: core::Ptr<crate::mcc::MCC_CChecker>) -> Result<core::Ptr<crate::mcc::MCC_CCheckerDraw>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_CCheckerDraw_create_PtrLCCheckerG(p_checker.as_raw_mut_PtrOfMCC_CChecker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mcc::MCC_CCheckerDraw>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for MCC_CCheckerDraw {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MCC_CCheckerDraw")
			.finish()
	}
}

/// Constant methods for [crate::mcc::MCC_DetectorParameters]
// DetectorParameters /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:99
pub trait MCC_DetectorParametersTraitConst {
	fn as_raw_MCC_DetectorParameters(&self) -> *const c_void;

	// cv::mcc::DetectorParameters::adaptiveThreshWinSizeMin() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:106
	// ("cv::mcc::DetectorParameters::adaptiveThreshWinSizeMin", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn adaptive_thresh_win_size_min(&self) -> i32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMin_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::adaptiveThreshWinSizeMax() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:107
	// ("cv::mcc::DetectorParameters::adaptiveThreshWinSizeMax", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn adaptive_thresh_win_size_max(&self) -> i32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMax_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::adaptiveThreshWinSizeStep() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:108
	// ("cv::mcc::DetectorParameters::adaptiveThreshWinSizeStep", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn adaptive_thresh_win_size_step(&self) -> i32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeStep_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::adaptiveThreshConstant() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:109
	// ("cv::mcc::DetectorParameters::adaptiveThreshConstant", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn adaptive_thresh_constant(&self) -> f64 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propAdaptiveThreshConstant_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::minContoursAreaRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:110
	// ("cv::mcc::DetectorParameters::minContoursAreaRate", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_contours_area_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinContoursAreaRate_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::minContoursArea() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:111
	// ("cv::mcc::DetectorParameters::minContoursArea", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_contours_area(&self) -> f64 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinContoursArea_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::confidenceThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:112
	// ("cv::mcc::DetectorParameters::confidenceThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn confidence_threshold(&self) -> f64 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propConfidenceThreshold_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::minContourSolidity() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:113
	// ("cv::mcc::DetectorParameters::minContourSolidity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_contour_solidity(&self) -> f64 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinContourSolidity_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::findCandidatesApproxPolyDPEpsMultiplier() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:114
	// ("cv::mcc::DetectorParameters::findCandidatesApproxPolyDPEpsMultiplier", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn find_candidates_approx_poly_dp_eps_multiplier(&self) -> f64 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propFindCandidatesApproxPolyDPEpsMultiplier_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::borderWidth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:115
	// ("cv::mcc::DetectorParameters::borderWidth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn border_width(&self) -> i32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propBorderWidth_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::B0factor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:116
	// ("cv::mcc::DetectorParameters::B0factor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn b0factor(&self) -> f32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propB0factor_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::maxError() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:117
	// ("cv::mcc::DetectorParameters::maxError", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_error(&self) -> f32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMaxError_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::minContourPointsAllowed() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:118
	// ("cv::mcc::DetectorParameters::minContourPointsAllowed", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_contour_points_allowed(&self) -> i32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinContourPointsAllowed_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::minContourLengthAllowed() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:119
	// ("cv::mcc::DetectorParameters::minContourLengthAllowed", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_contour_length_allowed(&self) -> i32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinContourLengthAllowed_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::minInterContourDistance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:120
	// ("cv::mcc::DetectorParameters::minInterContourDistance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_inter_contour_distance(&self) -> i32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinInterContourDistance_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::minInterCheckerDistance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:121
	// ("cv::mcc::DetectorParameters::minInterCheckerDistance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_inter_checker_distance(&self) -> i32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinInterCheckerDistance_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::minImageSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:122
	// ("cv::mcc::DetectorParameters::minImageSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_image_size(&self) -> i32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinImageSize_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

	// cv::mcc::DetectorParameters::minGroupSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:123
	// ("cv::mcc::DetectorParameters::minGroupSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_group_size(&self) -> u32 {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinGroupSize_const(self.as_raw_MCC_DetectorParameters()) };
		ret
	}

}

/// Mutable methods for [crate::mcc::MCC_DetectorParameters]
pub trait MCC_DetectorParametersTrait: crate::mcc::MCC_DetectorParametersTraitConst {
	fn as_raw_mut_MCC_DetectorParameters(&mut self) -> *mut c_void;

	// cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMin(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:106
	// ("cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMin", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_adaptive_thresh_win_size_min(&mut self, val: i32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMin_const_int(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMax(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:107
	// ("cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMax", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_adaptive_thresh_win_size_max(&mut self, val: i32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMax_const_int(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeStep(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:108
	// ("cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_adaptive_thresh_win_size_step(&mut self, val: i32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeStep_const_int(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setAdaptiveThreshConstant(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:109
	// ("cv::mcc::DetectorParameters::setAdaptiveThreshConstant", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_adaptive_thresh_constant(&mut self, val: f64) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propAdaptiveThreshConstant_const_double(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setMinContoursAreaRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:110
	// ("cv::mcc::DetectorParameters::setMinContoursAreaRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_min_contours_area_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinContoursAreaRate_const_double(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setMinContoursArea(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:111
	// ("cv::mcc::DetectorParameters::setMinContoursArea", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_min_contours_area(&mut self, val: f64) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinContoursArea_const_double(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setConfidenceThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:112
	// ("cv::mcc::DetectorParameters::setConfidenceThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_confidence_threshold(&mut self, val: f64) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propConfidenceThreshold_const_double(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setMinContourSolidity(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:113
	// ("cv::mcc::DetectorParameters::setMinContourSolidity", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_min_contour_solidity(&mut self, val: f64) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinContourSolidity_const_double(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setFindCandidatesApproxPolyDPEpsMultiplier(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:114
	// ("cv::mcc::DetectorParameters::setFindCandidatesApproxPolyDPEpsMultiplier", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_find_candidates_approx_poly_dp_eps_multiplier(&mut self, val: f64) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propFindCandidatesApproxPolyDPEpsMultiplier_const_double(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setBorderWidth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:115
	// ("cv::mcc::DetectorParameters::setBorderWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_border_width(&mut self, val: i32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propBorderWidth_const_int(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setB0factor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:116
	// ("cv::mcc::DetectorParameters::setB0factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_b0factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propB0factor_const_float(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setMaxError(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:117
	// ("cv::mcc::DetectorParameters::setMaxError", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_max_error(&mut self, val: f32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMaxError_const_float(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setMinContourPointsAllowed(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:118
	// ("cv::mcc::DetectorParameters::setMinContourPointsAllowed", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_min_contour_points_allowed(&mut self, val: i32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinContourPointsAllowed_const_int(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setMinContourLengthAllowed(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:119
	// ("cv::mcc::DetectorParameters::setMinContourLengthAllowed", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_min_contour_length_allowed(&mut self, val: i32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinContourLengthAllowed_const_int(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setMinInterContourDistance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:120
	// ("cv::mcc::DetectorParameters::setMinInterContourDistance", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_min_inter_contour_distance(&mut self, val: i32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinInterContourDistance_const_int(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setMinInterCheckerDistance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:121
	// ("cv::mcc::DetectorParameters::setMinInterCheckerDistance", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_min_inter_checker_distance(&mut self, val: i32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinInterCheckerDistance_const_int(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setMinImageSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:122
	// ("cv::mcc::DetectorParameters::setMinImageSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_min_image_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinImageSize_const_int(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

	// cv::mcc::DetectorParameters::setMinGroupSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:123
	// ("cv::mcc::DetectorParameters::setMinGroupSize", vec![(pred!(mut, ["val"], ["const unsigned int"]), _)]),
	#[inline]
	fn set_min_group_size(&mut self, val: u32) {
		let ret = unsafe { sys::cv_mcc_DetectorParameters_propMinGroupSize_const_unsigned_int(self.as_raw_mut_MCC_DetectorParameters(), val) };
		ret
	}

}

/// Parameters for the detectMarker process:
/// - int adaptiveThreshWinSizeMin : minimum window size for adaptive
///                                  thresholding before finding contours
///                                  (default 23).
/// - int adaptiveThreshWinSizeMax : maximum window size for adaptive
///                                  thresholding before finding contours
///                                  (default 153).
/// - int adaptiveThreshWinSizeStep : increments from adaptiveThreshWinSizeMin to
///                                   adaptiveThreshWinSizeMax during the
///                                   thresholding (default 16).
/// - double adaptiveThreshConstant : constant for adaptive thresholding before
///                                   finding contours (default 7)
/// - double minContoursAreaRate : determine minimum area for marker contour to
///                                be detected. This is defined as a rate respect
///                                to the area of the input image. Used only if
///                                neural network is used (default 0.003).
/// - double minContoursArea : determine minimum area for marker contour to be
///                            detected. This is defined as the actual area. Used
///                            only if neural network is not used (default 100).
/// - double confidenceThreshold : minimum confidence for a bounding box detected
///                                by neural network to classify as
///                                detection.(default 0.5)
///                                (0<=confidenceThreshold<=1)
/// - double minContourSolidity : minimum solidity of a contour for it be
///                               detected as a square in the chart. (default
///                               0.9).
/// - double findCandidatesApproxPolyDPEpsMultiplier : multipler to be used in
///                                                    cv::ApproxPolyDP function
///                                                    (default 0.05)
/// - int borderWidth : width of the padding used to pass the inital neural
///                    network detection in the succeeding system.(default 0)
/// - float B0factor : distance between two neighbours squares of the same chart.
///                    Defined as the ratio between distance and large dimension
///                    of square (default 1.25)
/// - float maxError : maximum allowed error in the detection of a chart.
///                    default(0.1)
/// - int minContourPointsAllowed : minium points in a detected contour.
///                                 default(4)
/// - int minContourLengthAllowed : minimum length of a countour. default(100)
/// - int minInterContourDistance : minimum distance between two contours.
///                                 default(100)
/// - int minInterCheckerDistance : minimum distance between two checkers.
///                                 default(10000)
/// - int minImageSize : minimum size of the smaller dimension of the image.
///                      default(1000)
/// - unsigned minGroupSize : minimum number of a squared of a chart that must be
///                           detected. default(4)
// DetectorParameters /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:99
pub struct MCC_DetectorParameters {
	ptr: *mut c_void,
}

opencv_type_boxed! { MCC_DetectorParameters }

impl Drop for MCC_DetectorParameters {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_mcc_DetectorParameters_delete(self.as_raw_mut_MCC_DetectorParameters()) };
	}
}

unsafe impl Send for MCC_DetectorParameters {}

impl crate::mcc::MCC_DetectorParametersTraitConst for MCC_DetectorParameters {
	#[inline] fn as_raw_MCC_DetectorParameters(&self) -> *const c_void { self.as_raw() }
}

impl crate::mcc::MCC_DetectorParametersTrait for MCC_DetectorParameters {
	#[inline] fn as_raw_mut_MCC_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MCC_DetectorParameters, crate::mcc::MCC_DetectorParametersTraitConst, as_raw_MCC_DetectorParameters, crate::mcc::MCC_DetectorParametersTrait, as_raw_mut_MCC_DetectorParameters }

impl MCC_DetectorParameters {
	// DetectorParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:102
	// ("cv::mcc::DetectorParameters::DetectorParameters", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::mcc::MCC_DetectorParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_DetectorParameters_DetectorParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mcc::MCC_DetectorParameters::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:104
	// ("cv::mcc::DetectorParameters::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::mcc::MCC_DetectorParameters>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_mcc_DetectorParameters_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mcc::MCC_DetectorParameters>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for MCC_DetectorParameters {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MCC_DetectorParameters")
			.field("adaptive_thresh_win_size_min", &crate::mcc::MCC_DetectorParametersTraitConst::adaptive_thresh_win_size_min(self))
			.field("adaptive_thresh_win_size_max", &crate::mcc::MCC_DetectorParametersTraitConst::adaptive_thresh_win_size_max(self))
			.field("adaptive_thresh_win_size_step", &crate::mcc::MCC_DetectorParametersTraitConst::adaptive_thresh_win_size_step(self))
			.field("adaptive_thresh_constant", &crate::mcc::MCC_DetectorParametersTraitConst::adaptive_thresh_constant(self))
			.field("min_contours_area_rate", &crate::mcc::MCC_DetectorParametersTraitConst::min_contours_area_rate(self))
			.field("min_contours_area", &crate::mcc::MCC_DetectorParametersTraitConst::min_contours_area(self))
			.field("confidence_threshold", &crate::mcc::MCC_DetectorParametersTraitConst::confidence_threshold(self))
			.field("min_contour_solidity", &crate::mcc::MCC_DetectorParametersTraitConst::min_contour_solidity(self))
			.field("find_candidates_approx_poly_dp_eps_multiplier", &crate::mcc::MCC_DetectorParametersTraitConst::find_candidates_approx_poly_dp_eps_multiplier(self))
			.field("border_width", &crate::mcc::MCC_DetectorParametersTraitConst::border_width(self))
			.field("b0factor", &crate::mcc::MCC_DetectorParametersTraitConst::b0factor(self))
			.field("max_error", &crate::mcc::MCC_DetectorParametersTraitConst::max_error(self))
			.field("min_contour_points_allowed", &crate::mcc::MCC_DetectorParametersTraitConst::min_contour_points_allowed(self))
			.field("min_contour_length_allowed", &crate::mcc::MCC_DetectorParametersTraitConst::min_contour_length_allowed(self))
			.field("min_inter_contour_distance", &crate::mcc::MCC_DetectorParametersTraitConst::min_inter_contour_distance(self))
			.field("min_inter_checker_distance", &crate::mcc::MCC_DetectorParametersTraitConst::min_inter_checker_distance(self))
			.field("min_image_size", &crate::mcc::MCC_DetectorParametersTraitConst::min_image_size(self))
			.field("min_group_size", &crate::mcc::MCC_DetectorParametersTraitConst::min_group_size(self))
			.finish()
	}
}

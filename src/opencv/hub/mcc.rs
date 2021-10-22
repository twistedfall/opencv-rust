#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
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
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::MCC_CCheckerConst, super::MCC_CChecker, super::MCC_CCheckerDrawConst, super::MCC_CCheckerDraw, super::MCC_DetectorParametersTraitConst, super::MCC_DetectorParametersTrait, super::MCC_CCheckerDetectorConst, super::MCC_CCheckerDetector, super::ColorCorrectionModelTraitConst, super::ColorCorrectionModelTrait };
}

/// The CCM with the shape ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) performs linear transformation on color values.
pub const CCM_3x3: i32 = 0;
/// The CCM with the shape ![inline formula](https://latex.codecogs.com/png.latex?4%5Ctimes3) performs affine transformation.
pub const CCM_4x3: i32 = 1;
/// DigitalSG ColorChecker with 140 squares
pub const COLORCHECKER_DigitalSG: i32 = 2;
/// Macbeth ColorChecker
pub const COLORCHECKER_Macbeth: i32 = 0;
/// DKK ColorChecker
pub const COLORCHECKER_Vinyl: i32 = 1;
/// https://en.wikipedia.org/wiki/Adobe_RGB_color_space , RGB color space
pub const COLOR_SPACE_AdobeRGB: i32 = 2;
/// https://en.wikipedia.org/wiki/Adobe_RGB_color_space , linear RGB color space
pub const COLOR_SPACE_AdobeRGBL: i32 = 3;
/// https://en.wikipedia.org/wiki/RGB_color_space , RGB color space
pub const COLOR_SPACE_AppleRGB: i32 = 10;
/// https://en.wikipedia.org/wiki/RGB_color_space , linear RGB color space
pub const COLOR_SPACE_AppleRGBL: i32 = 11;
/// https://en.wikipedia.org/wiki/DCI-P3 , RGB color space
pub const COLOR_SPACE_DCI_P3_RGB: i32 = 8;
/// https://en.wikipedia.org/wiki/DCI-P3 , linear RGB color space
pub const COLOR_SPACE_DCI_P3_RGBL: i32 = 9;
/// non-RGB color space
pub const COLOR_SPACE_Lab_A_10: i32 = 33;
/// non-RGB color space
pub const COLOR_SPACE_Lab_A_2: i32 = 32;
/// non-RGB color space
pub const COLOR_SPACE_Lab_D50_10: i32 = 31;
/// non-RGB color space
pub const COLOR_SPACE_Lab_D50_2: i32 = 30;
/// non-RGB color space
pub const COLOR_SPACE_Lab_D55_10: i32 = 35;
/// non-RGB color space
pub const COLOR_SPACE_Lab_D55_2: i32 = 34;
/// non-RGB color space
pub const COLOR_SPACE_Lab_D65_10: i32 = 29;
/// https://en.wikipedia.org/wiki/CIELAB_color_space , non-RGB color space
pub const COLOR_SPACE_Lab_D65_2: i32 = 28;
/// non-RGB color space
pub const COLOR_SPACE_Lab_D75_10: i32 = 37;
/// non-RGB color space
pub const COLOR_SPACE_Lab_D75_2: i32 = 36;
/// non-RGB color space
pub const COLOR_SPACE_Lab_E_10: i32 = 39;
/// non-RGB color space
pub const COLOR_SPACE_Lab_E_2: i32 = 38;
/// https://en.wikipedia.org/wiki/ProPhoto_RGB_color_space , RGB color space
pub const COLOR_SPACE_ProPhotoRGB: i32 = 6;
/// https://en.wikipedia.org/wiki/ProPhoto_RGB_color_space , linear RGB color space
pub const COLOR_SPACE_ProPhotoRGBL: i32 = 7;
/// https://en.wikipedia.org/wiki/Rec._2020 , RGB color space
pub const COLOR_SPACE_REC_2020_RGB: i32 = 14;
/// https://en.wikipedia.org/wiki/Rec._2020 , linear RGB color space
pub const COLOR_SPACE_REC_2020_RGBL: i32 = 15;
/// https://en.wikipedia.org/wiki/Rec._709 , RGB color space
pub const COLOR_SPACE_REC_709_RGB: i32 = 12;
/// https://en.wikipedia.org/wiki/Rec._709 , linear RGB color space
pub const COLOR_SPACE_REC_709_RGBL: i32 = 13;
/// https://en.wikipedia.org/wiki/Wide-gamut_RGB_color_space , RGB color space
pub const COLOR_SPACE_WideGamutRGB: i32 = 4;
/// https://en.wikipedia.org/wiki/Wide-gamut_RGB_color_space , linear RGB color space
pub const COLOR_SPACE_WideGamutRGBL: i32 = 5;
/// non-RGB color space
pub const COLOR_SPACE_XYZ_A_10: i32 = 21;
/// non-RGB color space
pub const COLOR_SPACE_XYZ_A_2: i32 = 20;
/// non-RGB color space
pub const COLOR_SPACE_XYZ_D50_10: i32 = 19;
/// non-RGB color space
pub const COLOR_SPACE_XYZ_D50_2: i32 = 18;
/// non-RGB color space
pub const COLOR_SPACE_XYZ_D55_10: i32 = 23;
/// non-RGB color space
pub const COLOR_SPACE_XYZ_D55_2: i32 = 22;
/// non-RGB color space
pub const COLOR_SPACE_XYZ_D65_10: i32 = 17;
/// https://en.wikipedia.org/wiki/CIE_1931_color_space , non-RGB color space
pub const COLOR_SPACE_XYZ_D65_2: i32 = 16;
/// non-RGB color space
pub const COLOR_SPACE_XYZ_D75_10: i32 = 25;
/// non-RGB color space
pub const COLOR_SPACE_XYZ_D75_2: i32 = 24;
/// non-RGB color space
pub const COLOR_SPACE_XYZ_E_10: i32 = 27;
/// non-RGB color space
pub const COLOR_SPACE_XYZ_E_2: i32 = 26;
/// https://en.wikipedia.org/wiki/SRGB , RGB color space
pub const COLOR_SPACE_sRGB: i32 = 0;
/// https://en.wikipedia.org/wiki/SRGB , linear RGB color space
pub const COLOR_SPACE_sRGBL: i32 = 1;
pub const DISTANCE_CIE2000: i32 = 3;
/// The 1976 formula is the first formula that related a measured color difference to a known set of CIELAB coordinates.
pub const DISTANCE_CIE76: i32 = 0;
/// The 1976 definition was extended to address perceptual non-uniformities.
pub const DISTANCE_CIE94_GRAPHIC_ARTS: i32 = 1;
pub const DISTANCE_CIE94_TEXTILES: i32 = 2;
/// In 1984, the Colour Measurement Committee of the Society of Dyers and Colourists defined a difference measure, also based on the L*C*h color model.
pub const DISTANCE_CMC_1TO1: i32 = 4;
pub const DISTANCE_CMC_2TO1: i32 = 5;
/// Euclidean distance of rgb color space
pub const DISTANCE_RGB: i32 = 6;
/// Euclidean distance of rgbl color space
pub const DISTANCE_RGBL: i32 = 7;
/// the least square method is an optimal solution under the linear RGB distance function
pub const INITIAL_METHOD_LEAST_SQUARE: i32 = 1;
/// The white balance method. The initial value is:
/// 
pub const INITIAL_METHOD_WHITE_BALANCE: i32 = 0;
/// logarithmic polynomial fitting channels respectively; Need assign a value to deg simultaneously
pub const LINEARIZATION_COLORLOGPOLYFIT: i32 = 3;
/// polynomial fitting channels respectively; Need assign a value to deg simultaneously
pub const LINEARIZATION_COLORPOLYFIT: i32 = 2;
/// gamma correction; Need assign a value to gamma simultaneously
pub const LINEARIZATION_GAMMA: i32 = 1;
/// grayscale Logarithmic polynomial fitting;  Need assign a value to deg and dst_whites simultaneously
pub const LINEARIZATION_GRAYLOGPOLYFIT: i32 = 5;
/// grayscale polynomial fitting; Need assign a value to deg and dst_whites simultaneously
pub const LINEARIZATION_GRAYPOLYFIT: i32 = 4;
/// no change is made
pub const LINEARIZATION_IDENTITY: i32 = 0;
/// Standard Macbeth Chart with 24 squares
pub const MCC_MCC24: i32 = 0;
/// DigitalSG with 140 squares
pub const MCC_SG140: i32 = 1;
/// DKK color chart with 12 squares and 6 rectangle
pub const MCC_VINYL18: i32 = 2;
/// Enum of the possible types of ccm.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CCM_TYPE {
	/// The CCM with the shape ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) performs linear transformation on color values.
	CCM_3x3 = 0,
	/// The CCM with the shape ![inline formula](https://latex.codecogs.com/png.latex?4%5Ctimes3) performs affine transformation.
	CCM_4x3 = 1,
}

opencv_type_enum! { crate::mcc::CCM_TYPE }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum COLOR_SPACE {
	/// https://en.wikipedia.org/wiki/SRGB , RGB color space
	COLOR_SPACE_sRGB = 0,
	/// https://en.wikipedia.org/wiki/SRGB , linear RGB color space
	COLOR_SPACE_sRGBL = 1,
	/// https://en.wikipedia.org/wiki/Adobe_RGB_color_space , RGB color space
	COLOR_SPACE_AdobeRGB = 2,
	/// https://en.wikipedia.org/wiki/Adobe_RGB_color_space , linear RGB color space
	COLOR_SPACE_AdobeRGBL = 3,
	/// https://en.wikipedia.org/wiki/Wide-gamut_RGB_color_space , RGB color space
	COLOR_SPACE_WideGamutRGB = 4,
	/// https://en.wikipedia.org/wiki/Wide-gamut_RGB_color_space , linear RGB color space
	COLOR_SPACE_WideGamutRGBL = 5,
	/// https://en.wikipedia.org/wiki/ProPhoto_RGB_color_space , RGB color space
	COLOR_SPACE_ProPhotoRGB = 6,
	/// https://en.wikipedia.org/wiki/ProPhoto_RGB_color_space , linear RGB color space
	COLOR_SPACE_ProPhotoRGBL = 7,
	/// https://en.wikipedia.org/wiki/DCI-P3 , RGB color space
	COLOR_SPACE_DCI_P3_RGB = 8,
	/// https://en.wikipedia.org/wiki/DCI-P3 , linear RGB color space
	COLOR_SPACE_DCI_P3_RGBL = 9,
	/// https://en.wikipedia.org/wiki/RGB_color_space , RGB color space
	COLOR_SPACE_AppleRGB = 10,
	/// https://en.wikipedia.org/wiki/RGB_color_space , linear RGB color space
	COLOR_SPACE_AppleRGBL = 11,
	/// https://en.wikipedia.org/wiki/Rec._709 , RGB color space
	COLOR_SPACE_REC_709_RGB = 12,
	/// https://en.wikipedia.org/wiki/Rec._709 , linear RGB color space
	COLOR_SPACE_REC_709_RGBL = 13,
	/// https://en.wikipedia.org/wiki/Rec._2020 , RGB color space
	COLOR_SPACE_REC_2020_RGB = 14,
	/// https://en.wikipedia.org/wiki/Rec._2020 , linear RGB color space
	COLOR_SPACE_REC_2020_RGBL = 15,
	/// https://en.wikipedia.org/wiki/CIE_1931_color_space , non-RGB color space
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
	/// https://en.wikipedia.org/wiki/CIELAB_color_space , non-RGB color space
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

opencv_type_enum! { crate::mcc::COLOR_SPACE }

///  Macbeth and Vinyl ColorChecker with 2deg D50
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CONST_COLOR {
	/// Macbeth ColorChecker
	COLORCHECKER_Macbeth = 0,
	/// DKK ColorChecker
	COLORCHECKER_Vinyl = 1,
	/// DigitalSG ColorChecker with 140 squares
	COLORCHECKER_DigitalSG = 2,
}

opencv_type_enum! { crate::mcc::CONST_COLOR }

/// Enum of possible functions to calculate the distance between colors.
/// 
/// See https://en.wikipedia.org/wiki/Color_difference for details
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
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

opencv_type_enum! { crate::mcc::DISTANCE_TYPE }

/// Enum of the possible types of initial method.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum INITIAL_METHOD_TYPE {
	/// The white balance method. The initial value is:
	/// 
	INITIAL_METHOD_WHITE_BALANCE = 0,
	/// the least square method is an optimal solution under the linear RGB distance function
	INITIAL_METHOD_LEAST_SQUARE = 1,
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
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
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

opencv_type_enum! { crate::mcc::LINEAR_TYPE }

/// TYPECHART
/// 
/// \brief enum to hold the type of the checker
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MCC_TYPECHART {
	/// Standard Macbeth Chart with 24 squares
	MCC24 = 0,
	/// DigitalSG with 140 squares
	SG140 = 1,
	/// DKK color chart with 12 squares and 6 rectangle
	VINYL18 = 2,
}

opencv_type_enum! { crate::mcc::MCC_TYPECHART }

/// Core class of ccm model
/// 
/// Produce a ColorCorrectionModel instance for inference
pub trait ColorCorrectionModelTraitConst {
	fn as_raw_ColorCorrectionModel(&self) -> *const c_void;

	#[inline]
	fn get_ccm(&self) -> Result<core::Mat> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_getCCM_const(self.as_raw_ColorCorrectionModel()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	#[inline]
	fn get_loss(&self) -> Result<f64> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_getLoss_const(self.as_raw_ColorCorrectionModel()) }.into_result()
	}
	
	#[inline]
	fn get_src_rgbl(&self) -> Result<core::Mat> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_get_src_rgbl_const(self.as_raw_ColorCorrectionModel()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	#[inline]
	fn get_dst_rgbl(&self) -> Result<core::Mat> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_get_dst_rgbl_const(self.as_raw_ColorCorrectionModel()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	#[inline]
	fn get_mask(&self) -> Result<core::Mat> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_getMask_const(self.as_raw_ColorCorrectionModel()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	#[inline]
	fn get_weights(&self) -> Result<core::Mat> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_getWeights_const(self.as_raw_ColorCorrectionModel()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

pub trait ColorCorrectionModelTrait: crate::mcc::ColorCorrectionModelTraitConst {
	fn as_raw_mut_ColorCorrectionModel(&mut self) -> *mut c_void;

	/// set ColorSpace
	/// 
	/// Note: It should be some RGB color space;
	/// Supported list of color cards:
	/// - @ref COLOR_SPACE_sRGB
	/// - @ref COLOR_SPACE_AdobeRGB
	/// - @ref COLOR_SPACE_WideGamutRGB
	/// - @ref COLOR_SPACE_ProPhotoRGB
	/// - @ref COLOR_SPACE_DCI_P3_RGB
	/// - @ref COLOR_SPACE_AppleRGB
	/// - @ref COLOR_SPACE_REC_709_RGB
	/// - @ref COLOR_SPACE_REC_2020_RGB
	/// ## Parameters
	/// * cs: the absolute color space that detected colors convert to;
	/// 
	///           default: @ref COLOR_SPACE_sRGB
	#[inline]
	fn set_color_space(&mut self, cs: crate::mcc::COLOR_SPACE) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setColorSpace_COLOR_SPACE(self.as_raw_mut_ColorCorrectionModel(), cs) }.into_result()
	}
	
	/// set ccm_type
	/// ## Parameters
	/// * ccm_type: the shape of color correction matrix(CCM);
	/// 
	///                default: @ref CCM_3x3
	#[inline]
	fn set_ccm_type(&mut self, ccm_type: crate::mcc::CCM_TYPE) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setCCM_TYPE_CCM_TYPE(self.as_raw_mut_ColorCorrectionModel(), ccm_type) }.into_result()
	}
	
	/// set Distance
	/// ## Parameters
	/// * distance: the type of color distance;
	/// 
	///                default: @ref DISTANCE_CIE2000
	#[inline]
	fn set_distance(&mut self, distance: crate::mcc::DISTANCE_TYPE) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setDistance_DISTANCE_TYPE(self.as_raw_mut_ColorCorrectionModel(), distance) }.into_result()
	}
	
	/// set Linear
	/// ## Parameters
	/// * linear_type: the method of linearization;
	/// 
	///                   default: @ref LINEARIZATION_GAMMA
	#[inline]
	fn set_linear(&mut self, linear_type: crate::mcc::LINEAR_TYPE) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setLinear_LINEAR_TYPE(self.as_raw_mut_ColorCorrectionModel(), linear_type) }.into_result()
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
	#[inline]
	fn set_linear_gamma(&mut self, gamma: &f64) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setLinearGamma_const_doubleR(self.as_raw_mut_ColorCorrectionModel(), gamma) }.into_result()
	}
	
	/// set degree
	///     
	/// Note: only valid when linear is set to
	///    - @ref LINEARIZATION_COLORPOLYFIT
	///    - @ref LINEARIZATION_GRAYPOLYFIT
	///    - @ref LINEARIZATION_COLORLOGPOLYFIT
	///    - @ref LINEARIZATION_GRAYLOGPOLYFIT
	/// 
	/// ## Parameters
	/// * deg: the degree of linearization polynomial;
	/// 
	///        default: 3
	#[inline]
	fn set_linear_degree(&mut self, deg: &i32) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setLinearDegree_const_intR(self.as_raw_mut_ColorCorrectionModel(), deg) }.into_result()
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
	#[inline]
	fn set_saturated_threshold(&mut self, lower: &f64, upper: &f64) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setSaturatedThreshold_const_doubleR_const_doubleR(self.as_raw_mut_ColorCorrectionModel(), lower, upper) }.into_result()
	}
	
	/// set WeightsList
	/// ## Parameters
	/// * weights_list: the list of weight of each color;
	/// 
	///                    default: empty array
	#[inline]
	fn set_weights_list(&mut self, weights_list: &core::Mat) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setWeightsList_const_MatR(self.as_raw_mut_ColorCorrectionModel(), weights_list.as_raw_Mat()) }.into_result()
	}
	
	/// set WeightCoeff
	/// ## Parameters
	/// * weights_coeff: the exponent number of L* component of the reference color in CIE Lab color space;
	/// 
	///                      default: 0
	#[inline]
	fn set_weight_coeff(&mut self, weights_coeff: &f64) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setWeightCoeff_const_doubleR(self.as_raw_mut_ColorCorrectionModel(), weights_coeff) }.into_result()
	}
	
	/// set InitialMethod
	/// ## Parameters
	/// * initial_method_type: the method of calculating CCM initial value;
	/// 
	///        default: INITIAL_METHOD_LEAST_SQUARE
	#[inline]
	fn set_initial_method(&mut self, initial_method_type: crate::mcc::INITIAL_METHOD_TYPE) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setInitialMethod_INITIAL_METHOD_TYPE(self.as_raw_mut_ColorCorrectionModel(), initial_method_type) }.into_result()
	}
	
	/// set MaxCount
	/// ## Parameters
	/// * max_count: used in MinProblemSolver-DownhillSolver;
	/// 
	///    Terminal criteria to the algorithm;
	/// 
	///                  default: 5000;
	#[inline]
	fn set_max_count(&mut self, max_count: &i32) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setMaxCount_const_intR(self.as_raw_mut_ColorCorrectionModel(), max_count) }.into_result()
	}
	
	/// set Epsilon
	/// ## Parameters
	/// * epsilon: used in MinProblemSolver-DownhillSolver;
	/// 
	///    Terminal criteria to the algorithm;
	/// 
	///               default: 1e-4;
	#[inline]
	fn set_epsilon(&mut self, epsilon: &f64) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_setEpsilon_const_doubleR(self.as_raw_mut_ColorCorrectionModel(), epsilon) }.into_result()
	}
	
	/// make color correction
	#[inline]
	fn run(&mut self) -> Result<()> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_run(self.as_raw_mut_ColorCorrectionModel()) }.into_result()
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
	#[inline]
	fn infer(&mut self, img: &core::Mat, islinear: bool) -> Result<core::Mat> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_infer_const_MatR_bool(self.as_raw_mut_ColorCorrectionModel(), img.as_raw_Mat(), islinear) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Core class of ccm model
/// 
/// Produce a ColorCorrectionModel instance for inference
pub struct ColorCorrectionModel {
	ptr: *mut c_void
}

opencv_type_boxed! { ColorCorrectionModel }

impl Drop for ColorCorrectionModel {
	fn drop(&mut self) {
		extern "C" { fn cv_ColorCorrectionModel_delete(instance: *mut c_void); }
		unsafe { cv_ColorCorrectionModel_delete(self.as_raw_mut_ColorCorrectionModel()) };
	}
}

unsafe impl Send for ColorCorrectionModel {}

impl crate::mcc::ColorCorrectionModelTraitConst for ColorCorrectionModel {
	#[inline] fn as_raw_ColorCorrectionModel(&self) -> *const c_void { self.as_raw() }
}

impl crate::mcc::ColorCorrectionModelTrait for ColorCorrectionModel {
	#[inline] fn as_raw_mut_ColorCorrectionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ColorCorrectionModel {
	/// Color Correction Model
	/// 
	/// Supported list of color cards:
	/// - @ref COLORCHECKER_Macbeth, the Macbeth ColorChecker
	/// - @ref COLORCHECKER_Vinyl, the DKK ColorChecker
	/// - @ref COLORCHECKER_DigitalSG, the DigitalSG ColorChecker with 140 squares
	/// 
	/// ## Parameters
	/// * src: detected colors of ColorChecker patches;
	/// 
	///            the color type is RGB not BGR, and the color values are in [0, 1];
	/// * constcolor: the Built-in color card
	#[inline]
	pub fn new(src: &core::Mat, constcolor: crate::mcc::CONST_COLOR) -> Result<crate::mcc::ColorCorrectionModel> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_CONST_COLOR(src.as_raw_Mat(), constcolor) }.into_result().map(|r| unsafe { crate::mcc::ColorCorrectionModel::opencv_from_extern(r) } )
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
	#[inline]
	pub fn new_1(src: &core::Mat, mut colors: core::Mat, ref_cs: crate::mcc::COLOR_SPACE) -> Result<crate::mcc::ColorCorrectionModel> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE(src.as_raw_Mat(), colors.as_raw_mut_Mat(), ref_cs) }.into_result().map(|r| unsafe { crate::mcc::ColorCorrectionModel::opencv_from_extern(r) } )
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
	#[inline]
	pub fn new_2(src: &core::Mat, mut colors: core::Mat, ref_cs: crate::mcc::COLOR_SPACE, mut colored: core::Mat) -> Result<crate::mcc::ColorCorrectionModel> {
		unsafe { sys::cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE_Mat(src.as_raw_Mat(), colors.as_raw_mut_Mat(), ref_cs, colored.as_raw_mut_Mat()) }.into_result().map(|r| unsafe { crate::mcc::ColorCorrectionModel::opencv_from_extern(r) } )
	}
	
}

/// CChecker
/// 
/// \brief checker object
/// 
///    This class contains the information about the detected checkers,i.e, their
///    type, the corners of the chart, the color profile, the cost, centers chart,
///    etc.
pub trait MCC_CCheckerConst {
	fn as_raw_MCC_CChecker(&self) -> *const c_void;

}

pub trait MCC_CChecker: crate::mcc::MCC_CCheckerConst {
	fn as_raw_mut_MCC_CChecker(&mut self) -> *mut c_void;

	#[inline]
	fn set_target(&mut self, _target: crate::mcc::MCC_TYPECHART) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setTarget_TYPECHART(self.as_raw_mut_MCC_CChecker(), _target) }.into_result()
	}
	
	#[inline]
	fn set_box(&mut self, mut _box: core::Vector<core::Point2f>) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setBox_vector_Point2f_(self.as_raw_mut_MCC_CChecker(), _box.as_raw_mut_VectorOfPoint2f()) }.into_result()
	}
	
	#[inline]
	fn set_charts_rgb(&mut self, mut _charts_rgb: core::Mat) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setChartsRGB_Mat(self.as_raw_mut_MCC_CChecker(), _charts_rgb.as_raw_mut_Mat()) }.into_result()
	}
	
	#[inline]
	fn set_charts_y_cb_cr(&mut self, mut _charts_y_cb_cr: core::Mat) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setChartsYCbCr_Mat(self.as_raw_mut_MCC_CChecker(), _charts_y_cb_cr.as_raw_mut_Mat()) }.into_result()
	}
	
	#[inline]
	fn set_cost(&mut self, _cost: f32) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setCost_float(self.as_raw_mut_MCC_CChecker(), _cost) }.into_result()
	}
	
	#[inline]
	fn set_center(&mut self, _center: core::Point2f) -> Result<()> {
		unsafe { sys::cv_mcc_CChecker_setCenter_Point2f(self.as_raw_mut_MCC_CChecker(), _center.opencv_as_extern()) }.into_result()
	}
	
	#[inline]
	fn get_target(&mut self) -> Result<crate::mcc::MCC_TYPECHART> {
		unsafe { sys::cv_mcc_CChecker_getTarget(self.as_raw_mut_MCC_CChecker()) }.into_result()
	}
	
	#[inline]
	fn get_box(&mut self) -> Result<core::Vector<core::Point2f>> {
		unsafe { sys::cv_mcc_CChecker_getBox(self.as_raw_mut_MCC_CChecker()) }.into_result().map(|r| unsafe { core::Vector::<core::Point2f>::opencv_from_extern(r) } )
	}
	
	#[inline]
	fn get_charts_rgb(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_mcc_CChecker_getChartsRGB(self.as_raw_mut_MCC_CChecker()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	#[inline]
	fn get_charts_y_cb_cr(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_mcc_CChecker_getChartsYCbCr(self.as_raw_mut_MCC_CChecker()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	#[inline]
	fn get_cost(&mut self) -> Result<f32> {
		unsafe { sys::cv_mcc_CChecker_getCost(self.as_raw_mut_MCC_CChecker()) }.into_result()
	}
	
	#[inline]
	fn get_center(&mut self) -> Result<core::Point2f> {
		unsafe { sys::cv_mcc_CChecker_getCenter(self.as_raw_mut_MCC_CChecker()) }.into_result()
	}
	
}

impl dyn MCC_CChecker + '_ {
	/// \brief Create a new CChecker object.
	/// \return A pointer to the implementation of the CChecker
	#[inline]
	pub fn create() -> Result<core::Ptr<dyn crate::mcc::MCC_CChecker>> {
		unsafe { sys::cv_mcc_CChecker_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::mcc::MCC_CChecker>::opencv_from_extern(r) } )
	}
	
}
/// A class to find the positions of the ColorCharts in the image.
pub trait MCC_CCheckerDetectorConst: core::AlgorithmTraitConst {
	fn as_raw_MCC_CCheckerDetector(&self) -> *const c_void;

}

pub trait MCC_CCheckerDetector: core::AlgorithmTrait + crate::mcc::MCC_CCheckerDetectorConst {
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
	#[inline]
	fn set_net(&mut self, mut net: crate::dnn::Net) -> Result<bool> {
		unsafe { sys::cv_mcc_CCheckerDetector_setNet_Net(self.as_raw_mut_MCC_CCheckerDetector(), net.as_raw_mut_Net()) }.into_result()
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
	#[inline]
	fn process_with_roi(&mut self, image: &dyn core::ToInputArray, chart_type: crate::mcc::MCC_TYPECHART, regions_of_interest: &core::Vector<core::Rect>, nc: i32, use_net: bool, params: &core::Ptr<crate::mcc::MCC_DetectorParameters>) -> Result<bool> {
		input_array_arg!(image);
		unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_vector_Rect_R_const_int_bool_const_Ptr_DetectorParameters_R(self.as_raw_mut_MCC_CCheckerDetector(), image.as_raw__InputArray(), chart_type, regions_of_interest.as_raw_VectorOfRect(), nc, use_net, params.as_raw_PtrOfMCC_DetectorParameters()) }.into_result()
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
	#[inline]
	fn process(&mut self, image: &dyn core::ToInputArray, chart_type: crate::mcc::MCC_TYPECHART, nc: i32, use_net: bool, params: &core::Ptr<crate::mcc::MCC_DetectorParameters>) -> Result<bool> {
		input_array_arg!(image);
		unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_int_bool_const_Ptr_DetectorParameters_R(self.as_raw_mut_MCC_CCheckerDetector(), image.as_raw__InputArray(), chart_type, nc, use_net, params.as_raw_PtrOfMCC_DetectorParameters()) }.into_result()
	}
	
	/// \brief Get the best color checker. By the best it means the one
	///        detected with the highest confidence.
	/// \return checker A single colorchecker, if atleast one colorchecker
	///                was detected, 'nullptr' otherwise.
	#[inline]
	fn get_best_color_checker(&mut self) -> Result<core::Ptr<dyn crate::mcc::MCC_CChecker>> {
		unsafe { sys::cv_mcc_CCheckerDetector_getBestColorChecker(self.as_raw_mut_MCC_CCheckerDetector()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::mcc::MCC_CChecker>::opencv_from_extern(r) } )
	}
	
	/// \brief Get the list of all detected colorcheckers
	/// \return checkers vector of colorcheckers
	#[inline]
	fn get_list_color_checker(&mut self) -> Result<core::Vector<core::Ptr<dyn crate::mcc::MCC_CChecker>>> {
		unsafe { sys::cv_mcc_CCheckerDetector_getListColorChecker(self.as_raw_mut_MCC_CCheckerDetector()) }.into_result().map(|r| unsafe { core::Vector::<core::Ptr<dyn crate::mcc::MCC_CChecker>>::opencv_from_extern(r) } )
	}
	
}

impl dyn MCC_CCheckerDetector + '_ {
	/// \brief Returns the implementation of the CCheckerDetector.
	#[inline]
	pub fn create() -> Result<core::Ptr<dyn crate::mcc::MCC_CCheckerDetector>> {
		unsafe { sys::cv_mcc_CCheckerDetector_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::mcc::MCC_CCheckerDetector>::opencv_from_extern(r) } )
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
pub trait MCC_CCheckerDrawConst {
	fn as_raw_MCC_CCheckerDraw(&self) -> *const c_void;

}

pub trait MCC_CCheckerDraw: crate::mcc::MCC_CCheckerDrawConst {
	fn as_raw_mut_MCC_CCheckerDraw(&mut self) -> *mut c_void;

	/// \brief Draws the checker to the given image.
	/// \param img image in color space BGR
	/// \return void
	#[inline]
	fn draw(&mut self, img: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_output_array_arg!(img);
		unsafe { sys::cv_mcc_CCheckerDraw_draw_const__InputOutputArrayR(self.as_raw_mut_MCC_CCheckerDraw(), img.as_raw__InputOutputArray()) }.into_result()
	}
	
}

impl dyn MCC_CCheckerDraw + '_ {
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
	#[inline]
	pub fn create(mut p_checker: core::Ptr<dyn crate::mcc::MCC_CChecker>, color: core::Scalar, thickness: i32) -> Result<core::Ptr<dyn crate::mcc::MCC_CCheckerDraw>> {
		unsafe { sys::cv_mcc_CCheckerDraw_create_Ptr_CChecker__Scalar_int(p_checker.as_raw_mut_PtrOfMCC_CChecker(), color.opencv_as_extern(), thickness) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::mcc::MCC_CCheckerDraw>::opencv_from_extern(r) } )
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
pub trait MCC_DetectorParametersTraitConst {
	fn as_raw_MCC_DetectorParameters(&self) -> *const c_void;

	#[inline]
	fn adaptive_thresh_win_size_min(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeMin_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_win_size_min")
	}
	
	#[inline]
	fn adaptive_thresh_win_size_max(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeMax_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_win_size_max")
	}
	
	#[inline]
	fn adaptive_thresh_win_size_step(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeStep_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_win_size_step")
	}
	
	#[inline]
	fn adaptive_thresh_constant(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropAdaptiveThreshConstant_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_constant")
	}
	
	#[inline]
	fn min_contours_area_rate(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinContoursAreaRate_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_contours_area_rate")
	}
	
	#[inline]
	fn min_contours_area(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinContoursArea_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_contours_area")
	}
	
	#[inline]
	fn confidence_threshold(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropConfidenceThreshold_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: confidence_threshold")
	}
	
	#[inline]
	fn min_contour_solidity(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinContourSolidity_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_contour_solidity")
	}
	
	#[inline]
	fn find_candidates_approx_poly_dp_eps_multiplier(&self) -> f64 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropFindCandidatesApproxPolyDPEpsMultiplier_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: find_candidates_approx_poly_dp_eps_multiplier")
	}
	
	#[inline]
	fn border_width(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropBorderWidth_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: border_width")
	}
	
	#[inline]
	fn b0factor(&self) -> f32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropB0factor_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: b0factor")
	}
	
	#[inline]
	fn max_error(&self) -> f32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMaxError_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: max_error")
	}
	
	#[inline]
	fn min_contour_points_allowed(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinContourPointsAllowed_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_contour_points_allowed")
	}
	
	#[inline]
	fn min_contour_length_allowed(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinContourLengthAllowed_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_contour_length_allowed")
	}
	
	#[inline]
	fn min_inter_contour_distance(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinInterContourDistance_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_inter_contour_distance")
	}
	
	#[inline]
	fn min_inter_checker_distance(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinInterCheckerDistance_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_inter_checker_distance")
	}
	
	#[inline]
	fn min_image_size(&self) -> i32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinImageSize_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_image_size")
	}
	
	#[inline]
	fn min_group_size(&self) -> u32 {
		unsafe { sys::cv_mcc_DetectorParameters_getPropMinGroupSize_const(self.as_raw_MCC_DetectorParameters()) }.into_result().expect("Infallible function failed: min_group_size")
	}
	
}

pub trait MCC_DetectorParametersTrait: crate::mcc::MCC_DetectorParametersTraitConst {
	fn as_raw_mut_MCC_DetectorParameters(&mut self) -> *mut c_void;

	#[inline]
	fn set_adaptive_thresh_win_size_min(&mut self, val: i32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeMin_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_win_size_min")
	}
	
	#[inline]
	fn set_adaptive_thresh_win_size_max(&mut self, val: i32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeMax_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_win_size_max")
	}
	
	#[inline]
	fn set_adaptive_thresh_win_size_step(&mut self, val: i32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeStep_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_win_size_step")
	}
	
	#[inline]
	fn set_adaptive_thresh_constant(&mut self, val: f64) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropAdaptiveThreshConstant_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_constant")
	}
	
	#[inline]
	fn set_min_contours_area_rate(&mut self, val: f64) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinContoursAreaRate_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_contours_area_rate")
	}
	
	#[inline]
	fn set_min_contours_area(&mut self, val: f64) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinContoursArea_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_contours_area")
	}
	
	#[inline]
	fn set_confidence_threshold(&mut self, val: f64) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropConfidenceThreshold_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_confidence_threshold")
	}
	
	#[inline]
	fn set_min_contour_solidity(&mut self, val: f64) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinContourSolidity_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_contour_solidity")
	}
	
	#[inline]
	fn set_find_candidates_approx_poly_dp_eps_multiplier(&mut self, val: f64) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropFindCandidatesApproxPolyDPEpsMultiplier_double(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_find_candidates_approx_poly_dp_eps_multiplier")
	}
	
	#[inline]
	fn set_border_width(&mut self, val: i32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropBorderWidth_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_border_width")
	}
	
	#[inline]
	fn set_b0factor(&mut self, val: f32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropB0factor_float(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_b0factor")
	}
	
	#[inline]
	fn set_max_error(&mut self, val: f32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMaxError_float(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_max_error")
	}
	
	#[inline]
	fn set_min_contour_points_allowed(&mut self, val: i32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinContourPointsAllowed_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_contour_points_allowed")
	}
	
	#[inline]
	fn set_min_contour_length_allowed(&mut self, val: i32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinContourLengthAllowed_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_contour_length_allowed")
	}
	
	#[inline]
	fn set_min_inter_contour_distance(&mut self, val: i32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinInterContourDistance_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_inter_contour_distance")
	}
	
	#[inline]
	fn set_min_inter_checker_distance(&mut self, val: i32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinInterCheckerDistance_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_inter_checker_distance")
	}
	
	#[inline]
	fn set_min_image_size(&mut self, val: i32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinImageSize_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_image_size")
	}
	
	#[inline]
	fn set_min_group_size(&mut self, val: u32) {
		unsafe { sys::cv_mcc_DetectorParameters_setPropMinGroupSize_unsigned_int(self.as_raw_mut_MCC_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_group_size")
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
pub struct MCC_DetectorParameters {
	ptr: *mut c_void
}

opencv_type_boxed! { MCC_DetectorParameters }

impl Drop for MCC_DetectorParameters {
	fn drop(&mut self) {
		extern "C" { fn cv_MCC_DetectorParameters_delete(instance: *mut c_void); }
		unsafe { cv_MCC_DetectorParameters_delete(self.as_raw_mut_MCC_DetectorParameters()) };
	}
}

unsafe impl Send for MCC_DetectorParameters {}

impl crate::mcc::MCC_DetectorParametersTraitConst for MCC_DetectorParameters {
	#[inline] fn as_raw_MCC_DetectorParameters(&self) -> *const c_void { self.as_raw() }
}

impl crate::mcc::MCC_DetectorParametersTrait for MCC_DetectorParameters {
	#[inline] fn as_raw_mut_MCC_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MCC_DetectorParameters {
	#[inline]
	pub fn default() -> Result<crate::mcc::MCC_DetectorParameters> {
		unsafe { sys::cv_mcc_DetectorParameters_DetectorParameters() }.into_result().map(|r| unsafe { crate::mcc::MCC_DetectorParameters::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::mcc::MCC_DetectorParameters>> {
		unsafe { sys::cv_mcc_DetectorParameters_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::mcc::MCC_DetectorParameters>::opencv_from_extern(r) } )
	}
	
}

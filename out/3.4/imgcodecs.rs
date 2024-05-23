//! # Image file reading and writing
//!   # C API
//!   # Flags used for image file reading and writing
//!   # iOS glue
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
}

/// If set, the image is read in any possible color format.
// IMREAD_ANYCOLOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:73
pub const IMREAD_ANYCOLOR: i32 = 4;
/// If set, return 16-bit/32-bit image when the input has the corresponding depth, otherwise convert it to 8-bit.
// IMREAD_ANYDEPTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:72
pub const IMREAD_ANYDEPTH: i32 = 2;
/// If set, always convert image to the 3 channel BGR color image.
// IMREAD_COLOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:71
pub const IMREAD_COLOR: i32 = 1;
/// If set, always convert image to the single channel grayscale image (codec internal conversion).
// IMREAD_GRAYSCALE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:70
pub const IMREAD_GRAYSCALE: i32 = 0;
/// If set, do not rotate the image according to EXIF's orientation flag.
// IMREAD_IGNORE_ORIENTATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:81
pub const IMREAD_IGNORE_ORIENTATION: i32 = 128;
/// If set, use the gdal driver for loading the image.
// IMREAD_LOAD_GDAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:74
pub const IMREAD_LOAD_GDAL: i32 = 8;
/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/2.
// IMREAD_REDUCED_COLOR_2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:76
pub const IMREAD_REDUCED_COLOR_2: i32 = 17;
/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/4.
// IMREAD_REDUCED_COLOR_4 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:78
pub const IMREAD_REDUCED_COLOR_4: i32 = 33;
/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/8.
// IMREAD_REDUCED_COLOR_8 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:80
pub const IMREAD_REDUCED_COLOR_8: i32 = 65;
/// If set, always convert image to the single channel grayscale image and the image size reduced 1/2.
// IMREAD_REDUCED_GRAYSCALE_2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:75
pub const IMREAD_REDUCED_GRAYSCALE_2: i32 = 16;
/// If set, always convert image to the single channel grayscale image and the image size reduced 1/4.
// IMREAD_REDUCED_GRAYSCALE_4 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:77
pub const IMREAD_REDUCED_GRAYSCALE_4: i32 = 32;
/// If set, always convert image to the single channel grayscale image and the image size reduced 1/8.
// IMREAD_REDUCED_GRAYSCALE_8 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:79
pub const IMREAD_REDUCED_GRAYSCALE_8: i32 = 64;
/// If set, return the loaded image as is (with alpha channel, otherwise it gets cropped). Ignore EXIF orientation.
// IMREAD_UNCHANGED /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:69
pub const IMREAD_UNCHANGED: i32 = -1;
/// override EXR storage type (FLOAT (FP32) is default)
// IMWRITE_EXR_TYPE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:97
pub const IMWRITE_EXR_TYPE: i32 = 48;
/// store as FP32 (default)
// IMWRITE_EXR_TYPE_FLOAT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:119
pub const IMWRITE_EXR_TYPE_FLOAT: i32 = 2;
/// store as HALF (FP16)
// IMWRITE_EXR_TYPE_HALF /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:118
pub const IMWRITE_EXR_TYPE_HALF: i32 = 1;
/// specify HDR compression
// IMWRITE_HDR_COMPRESSION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:99
pub const IMWRITE_HDR_COMPRESSION: i32 = 80;
// IMWRITE_HDR_COMPRESSION_NONE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:151
pub const IMWRITE_HDR_COMPRESSION_NONE: i32 = 0;
// IMWRITE_HDR_COMPRESSION_RLE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:152
pub const IMWRITE_HDR_COMPRESSION_RLE: i32 = 1;
/// Separate chroma quality level, 0 - 100, default is -1 - don't use.
// IMWRITE_JPEG_CHROMA_QUALITY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:91
pub const IMWRITE_JPEG_CHROMA_QUALITY: i32 = 6;
/// Separate luma quality level, 0 - 100, default is -1 - don't use.
// IMWRITE_JPEG_LUMA_QUALITY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:90
pub const IMWRITE_JPEG_LUMA_QUALITY: i32 = 5;
/// Enable JPEG features, 0 or 1, default is False.
// IMWRITE_JPEG_OPTIMIZE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:88
pub const IMWRITE_JPEG_OPTIMIZE: i32 = 3;
/// Enable JPEG features, 0 or 1, default is False.
// IMWRITE_JPEG_PROGRESSIVE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:87
pub const IMWRITE_JPEG_PROGRESSIVE: i32 = 2;
/// For JPEG, it can be a quality from 0 to 100 (the higher is the better). Default value is 95.
// IMWRITE_JPEG_QUALITY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:86
pub const IMWRITE_JPEG_QUALITY: i32 = 1;
/// JPEG restart interval, 0 - 65535, default is 0 - no restart.
// IMWRITE_JPEG_RST_INTERVAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:89
pub const IMWRITE_JPEG_RST_INTERVAL: i32 = 4;
/// For JPEG, set sampling factor. See cv::ImwriteJPEGSamplingFactorParams.
// IMWRITE_JPEG_SAMPLING_FACTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:92
pub const IMWRITE_JPEG_SAMPLING_FACTOR: i32 = 7;
/// 4x1,1x1,1x1
// IMWRITE_JPEG_SAMPLING_FACTOR_411 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:108
pub const IMWRITE_JPEG_SAMPLING_FACTOR_411: i32 = 4264209;
/// 2x2,1x1,1x1(Default)
// IMWRITE_JPEG_SAMPLING_FACTOR_420 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:109
pub const IMWRITE_JPEG_SAMPLING_FACTOR_420: i32 = 2232593;
/// 2x1,1x1,1x1
// IMWRITE_JPEG_SAMPLING_FACTOR_422 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:110
pub const IMWRITE_JPEG_SAMPLING_FACTOR_422: i32 = 2167057;
/// 1x2,1x1,1x1
// IMWRITE_JPEG_SAMPLING_FACTOR_440 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:111
pub const IMWRITE_JPEG_SAMPLING_FACTOR_440: i32 = 1184017;
/// 1x1,1x1,1x1(No subsampling)
// IMWRITE_JPEG_SAMPLING_FACTOR_444 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:112
pub const IMWRITE_JPEG_SAMPLING_FACTOR_444: i32 = 1118481;
// IMWRITE_PAM_FORMAT_BLACKANDWHITE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:142
pub const IMWRITE_PAM_FORMAT_BLACKANDWHITE: i32 = 1;
// IMWRITE_PAM_FORMAT_GRAYSCALE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:143
pub const IMWRITE_PAM_FORMAT_GRAYSCALE: i32 = 2;
// IMWRITE_PAM_FORMAT_GRAYSCALE_ALPHA /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:144
pub const IMWRITE_PAM_FORMAT_GRAYSCALE_ALPHA: i32 = 3;
// IMWRITE_PAM_FORMAT_NULL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:141
pub const IMWRITE_PAM_FORMAT_NULL: i32 = 0;
// IMWRITE_PAM_FORMAT_RGB /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:145
pub const IMWRITE_PAM_FORMAT_RGB: i32 = 4;
// IMWRITE_PAM_FORMAT_RGB_ALPHA /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:146
pub const IMWRITE_PAM_FORMAT_RGB_ALPHA: i32 = 5;
/// For PAM, sets the TUPLETYPE field to the corresponding string value that is defined for the format
// IMWRITE_PAM_TUPLETYPE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:100
pub const IMWRITE_PAM_TUPLETYPE: i32 = 128;
/// Binary level PNG, 0 or 1, default is 0.
// IMWRITE_PNG_BILEVEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:95
pub const IMWRITE_PNG_BILEVEL: i32 = 18;
/// For PNG, it can be the compression level from 0 to 9. A higher value means a smaller size and longer compression time. If specified, strategy is changed to IMWRITE_PNG_STRATEGY_DEFAULT (Z_DEFAULT_STRATEGY). Default value is 1 (best speed setting).
// IMWRITE_PNG_COMPRESSION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:93
pub const IMWRITE_PNG_COMPRESSION: i32 = 16;
/// One of cv::ImwritePNGFlags, default is IMWRITE_PNG_STRATEGY_RLE.
// IMWRITE_PNG_STRATEGY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:94
pub const IMWRITE_PNG_STRATEGY: i32 = 17;
/// Use this value for normal data.
// IMWRITE_PNG_STRATEGY_DEFAULT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:132
pub const IMWRITE_PNG_STRATEGY_DEFAULT: i32 = 0;
/// Use this value for data produced by a filter (or predictor).Filtered data consists mostly of small values with a somewhat random distribution. In this case, the compression algorithm is tuned to compress them better.
// IMWRITE_PNG_STRATEGY_FILTERED /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:133
pub const IMWRITE_PNG_STRATEGY_FILTERED: i32 = 1;
/// Using this value prevents the use of dynamic Huffman codes, allowing for a simpler decoder for special applications.
// IMWRITE_PNG_STRATEGY_FIXED /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:136
pub const IMWRITE_PNG_STRATEGY_FIXED: i32 = 4;
/// Use this value to force Huffman encoding only (no string match).
// IMWRITE_PNG_STRATEGY_HUFFMAN_ONLY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:134
pub const IMWRITE_PNG_STRATEGY_HUFFMAN_ONLY: i32 = 2;
/// Use this value to limit match distances to one (run-length encoding).
// IMWRITE_PNG_STRATEGY_RLE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:135
pub const IMWRITE_PNG_STRATEGY_RLE: i32 = 3;
/// For PPM, PGM, or PBM, it can be a binary format flag, 0 or 1. Default value is 1.
// IMWRITE_PXM_BINARY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:96
pub const IMWRITE_PXM_BINARY: i32 = 32;
/// For TIFF, use to specify the image compression scheme. See libtiff for integer constants corresponding to compression formats. Note, for images whose depth is CV_32F, only libtiff's SGILOG compression scheme is used. For other supported depths, the compression scheme can be specified by this flag; LZW compression is the default.
// IMWRITE_TIFF_COMPRESSION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:104
pub const IMWRITE_TIFF_COMPRESSION: i32 = 259;
/// For TIFF, use to specify which DPI resolution unit to set; see libtiff documentation for valid values.
// IMWRITE_TIFF_RESUNIT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:101
pub const IMWRITE_TIFF_RESUNIT: i32 = 256;
/// For TIFF, use to specify the X direction DPI.
// IMWRITE_TIFF_XDPI /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:102
pub const IMWRITE_TIFF_XDPI: i32 = 257;
/// For TIFF, use to specify the Y direction DPI.
// IMWRITE_TIFF_YDPI /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:103
pub const IMWRITE_TIFF_YDPI: i32 = 258;
/// For WEBP, it can be a quality from 1 to 100 (the higher is the better). By default (without any parameter) and for quality above 100 the lossless compression is used.
// IMWRITE_WEBP_QUALITY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:98
pub const IMWRITE_WEBP_QUALITY: i32 = 64;
/// Imread flags
// ImreadModes /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:68
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImreadModes {
	/// If set, return the loaded image as is (with alpha channel, otherwise it gets cropped). Ignore EXIF orientation.
	IMREAD_UNCHANGED = -1,
	/// If set, always convert image to the single channel grayscale image (codec internal conversion).
	IMREAD_GRAYSCALE = 0,
	/// If set, always convert image to the 3 channel BGR color image.
	IMREAD_COLOR = 1,
	/// If set, return 16-bit/32-bit image when the input has the corresponding depth, otherwise convert it to 8-bit.
	IMREAD_ANYDEPTH = 2,
	/// If set, the image is read in any possible color format.
	IMREAD_ANYCOLOR = 4,
	/// If set, use the gdal driver for loading the image.
	IMREAD_LOAD_GDAL = 8,
	/// If set, always convert image to the single channel grayscale image and the image size reduced 1/2.
	IMREAD_REDUCED_GRAYSCALE_2 = 16,
	/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/2.
	IMREAD_REDUCED_COLOR_2 = 17,
	/// If set, always convert image to the single channel grayscale image and the image size reduced 1/4.
	IMREAD_REDUCED_GRAYSCALE_4 = 32,
	/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/4.
	IMREAD_REDUCED_COLOR_4 = 33,
	/// If set, always convert image to the single channel grayscale image and the image size reduced 1/8.
	IMREAD_REDUCED_GRAYSCALE_8 = 64,
	/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/8.
	IMREAD_REDUCED_COLOR_8 = 65,
	/// If set, do not rotate the image according to EXIF's orientation flag.
	IMREAD_IGNORE_ORIENTATION = 128,
}

impl TryFrom<i32> for ImreadModes {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			-1 => Ok(Self::IMREAD_UNCHANGED),
			0 => Ok(Self::IMREAD_GRAYSCALE),
			1 => Ok(Self::IMREAD_COLOR),
			2 => Ok(Self::IMREAD_ANYDEPTH),
			4 => Ok(Self::IMREAD_ANYCOLOR),
			8 => Ok(Self::IMREAD_LOAD_GDAL),
			16 => Ok(Self::IMREAD_REDUCED_GRAYSCALE_2),
			17 => Ok(Self::IMREAD_REDUCED_COLOR_2),
			32 => Ok(Self::IMREAD_REDUCED_GRAYSCALE_4),
			33 => Ok(Self::IMREAD_REDUCED_COLOR_4),
			64 => Ok(Self::IMREAD_REDUCED_GRAYSCALE_8),
			65 => Ok(Self::IMREAD_REDUCED_COLOR_8),
			128 => Ok(Self::IMREAD_IGNORE_ORIENTATION),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImreadModes"))),
		}
	}
}

opencv_type_enum! { crate::imgcodecs::ImreadModes }

// ImwriteEXRTypeFlags /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:116
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImwriteEXRTypeFlags {
	/// store as HALF (FP16)
	IMWRITE_EXR_TYPE_HALF = 1,
	/// store as FP32 (default)
	IMWRITE_EXR_TYPE_FLOAT = 2,
}

impl TryFrom<i32> for ImwriteEXRTypeFlags {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::IMWRITE_EXR_TYPE_HALF),
			2 => Ok(Self::IMWRITE_EXR_TYPE_FLOAT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImwriteEXRTypeFlags"))),
		}
	}
}

opencv_type_enum! { crate::imgcodecs::ImwriteEXRTypeFlags }

/// Imwrite flags
// ImwriteFlags /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:85
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImwriteFlags {
	/// For JPEG, it can be a quality from 0 to 100 (the higher is the better). Default value is 95.
	IMWRITE_JPEG_QUALITY = 1,
	/// Enable JPEG features, 0 or 1, default is False.
	IMWRITE_JPEG_PROGRESSIVE = 2,
	/// Enable JPEG features, 0 or 1, default is False.
	IMWRITE_JPEG_OPTIMIZE = 3,
	/// JPEG restart interval, 0 - 65535, default is 0 - no restart.
	IMWRITE_JPEG_RST_INTERVAL = 4,
	/// Separate luma quality level, 0 - 100, default is -1 - don't use.
	IMWRITE_JPEG_LUMA_QUALITY = 5,
	/// Separate chroma quality level, 0 - 100, default is -1 - don't use.
	IMWRITE_JPEG_CHROMA_QUALITY = 6,
	/// For JPEG, set sampling factor. See cv::ImwriteJPEGSamplingFactorParams.
	IMWRITE_JPEG_SAMPLING_FACTOR = 7,
	/// For PNG, it can be the compression level from 0 to 9. A higher value means a smaller size and longer compression time. If specified, strategy is changed to IMWRITE_PNG_STRATEGY_DEFAULT (Z_DEFAULT_STRATEGY). Default value is 1 (best speed setting).
	IMWRITE_PNG_COMPRESSION = 16,
	/// One of cv::ImwritePNGFlags, default is IMWRITE_PNG_STRATEGY_RLE.
	IMWRITE_PNG_STRATEGY = 17,
	/// Binary level PNG, 0 or 1, default is 0.
	IMWRITE_PNG_BILEVEL = 18,
	/// For PPM, PGM, or PBM, it can be a binary format flag, 0 or 1. Default value is 1.
	IMWRITE_PXM_BINARY = 32,
	/// override EXR storage type (FLOAT (FP32) is default)
	IMWRITE_EXR_TYPE = 48,
	/// For WEBP, it can be a quality from 1 to 100 (the higher is the better). By default (without any parameter) and for quality above 100 the lossless compression is used.
	IMWRITE_WEBP_QUALITY = 64,
	/// specify HDR compression
	IMWRITE_HDR_COMPRESSION = 80,
	/// For PAM, sets the TUPLETYPE field to the corresponding string value that is defined for the format
	IMWRITE_PAM_TUPLETYPE = 128,
	/// For TIFF, use to specify which DPI resolution unit to set; see libtiff documentation for valid values.
	IMWRITE_TIFF_RESUNIT = 256,
	/// For TIFF, use to specify the X direction DPI.
	IMWRITE_TIFF_XDPI = 257,
	/// For TIFF, use to specify the Y direction DPI.
	IMWRITE_TIFF_YDPI = 258,
	/// For TIFF, use to specify the image compression scheme. See libtiff for integer constants corresponding to compression formats. Note, for images whose depth is CV_32F, only libtiff's SGILOG compression scheme is used. For other supported depths, the compression scheme can be specified by this flag; LZW compression is the default.
	IMWRITE_TIFF_COMPRESSION = 259,
}

impl TryFrom<i32> for ImwriteFlags {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::IMWRITE_JPEG_QUALITY),
			2 => Ok(Self::IMWRITE_JPEG_PROGRESSIVE),
			3 => Ok(Self::IMWRITE_JPEG_OPTIMIZE),
			4 => Ok(Self::IMWRITE_JPEG_RST_INTERVAL),
			5 => Ok(Self::IMWRITE_JPEG_LUMA_QUALITY),
			6 => Ok(Self::IMWRITE_JPEG_CHROMA_QUALITY),
			7 => Ok(Self::IMWRITE_JPEG_SAMPLING_FACTOR),
			16 => Ok(Self::IMWRITE_PNG_COMPRESSION),
			17 => Ok(Self::IMWRITE_PNG_STRATEGY),
			18 => Ok(Self::IMWRITE_PNG_BILEVEL),
			32 => Ok(Self::IMWRITE_PXM_BINARY),
			48 => Ok(Self::IMWRITE_EXR_TYPE),
			64 => Ok(Self::IMWRITE_WEBP_QUALITY),
			80 => Ok(Self::IMWRITE_HDR_COMPRESSION),
			128 => Ok(Self::IMWRITE_PAM_TUPLETYPE),
			256 => Ok(Self::IMWRITE_TIFF_RESUNIT),
			257 => Ok(Self::IMWRITE_TIFF_XDPI),
			258 => Ok(Self::IMWRITE_TIFF_YDPI),
			259 => Ok(Self::IMWRITE_TIFF_COMPRESSION),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImwriteFlags"))),
		}
	}
}

opencv_type_enum! { crate::imgcodecs::ImwriteFlags }

/// Imwrite HDR specific values for IMWRITE_HDR_COMPRESSION parameter key
// ImwriteHDRCompressionFlags /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:150
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImwriteHDRCompressionFlags {
	IMWRITE_HDR_COMPRESSION_NONE = 0,
	IMWRITE_HDR_COMPRESSION_RLE = 1,
}

impl TryFrom<i32> for ImwriteHDRCompressionFlags {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::IMWRITE_HDR_COMPRESSION_NONE),
			1 => Ok(Self::IMWRITE_HDR_COMPRESSION_RLE),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImwriteHDRCompressionFlags"))),
		}
	}
}

opencv_type_enum! { crate::imgcodecs::ImwriteHDRCompressionFlags }

// ImwriteJPEGSamplingFactorParams /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:107
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImwriteJPEGSamplingFactorParams {
	/// 4x1,1x1,1x1
	IMWRITE_JPEG_SAMPLING_FACTOR_411 = 4264209,
	/// 2x2,1x1,1x1(Default)
	IMWRITE_JPEG_SAMPLING_FACTOR_420 = 2232593,
	/// 2x1,1x1,1x1
	IMWRITE_JPEG_SAMPLING_FACTOR_422 = 2167057,
	/// 1x2,1x1,1x1
	IMWRITE_JPEG_SAMPLING_FACTOR_440 = 1184017,
	/// 1x1,1x1,1x1(No subsampling)
	IMWRITE_JPEG_SAMPLING_FACTOR_444 = 1118481,
}

impl TryFrom<i32> for ImwriteJPEGSamplingFactorParams {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			4264209 => Ok(Self::IMWRITE_JPEG_SAMPLING_FACTOR_411),
			2232593 => Ok(Self::IMWRITE_JPEG_SAMPLING_FACTOR_420),
			2167057 => Ok(Self::IMWRITE_JPEG_SAMPLING_FACTOR_422),
			1184017 => Ok(Self::IMWRITE_JPEG_SAMPLING_FACTOR_440),
			1118481 => Ok(Self::IMWRITE_JPEG_SAMPLING_FACTOR_444),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImwriteJPEGSamplingFactorParams"))),
		}
	}
}

opencv_type_enum! { crate::imgcodecs::ImwriteJPEGSamplingFactorParams }

/// Imwrite PAM specific tupletype flags used to define the 'TUPLETYPE' field of a PAM file.
// ImwritePAMFlags /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:140
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImwritePAMFlags {
	IMWRITE_PAM_FORMAT_NULL = 0,
	IMWRITE_PAM_FORMAT_BLACKANDWHITE = 1,
	IMWRITE_PAM_FORMAT_GRAYSCALE = 2,
	IMWRITE_PAM_FORMAT_GRAYSCALE_ALPHA = 3,
	IMWRITE_PAM_FORMAT_RGB = 4,
	IMWRITE_PAM_FORMAT_RGB_ALPHA = 5,
}

impl TryFrom<i32> for ImwritePAMFlags {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::IMWRITE_PAM_FORMAT_NULL),
			1 => Ok(Self::IMWRITE_PAM_FORMAT_BLACKANDWHITE),
			2 => Ok(Self::IMWRITE_PAM_FORMAT_GRAYSCALE),
			3 => Ok(Self::IMWRITE_PAM_FORMAT_GRAYSCALE_ALPHA),
			4 => Ok(Self::IMWRITE_PAM_FORMAT_RGB),
			5 => Ok(Self::IMWRITE_PAM_FORMAT_RGB_ALPHA),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImwritePAMFlags"))),
		}
	}
}

opencv_type_enum! { crate::imgcodecs::ImwritePAMFlags }

/// Imwrite PNG specific flags used to tune the compression algorithm.
/// These flags will be modify the way of PNG image compression and will be passed to the underlying zlib processing stage.
///
/// *   The effect of IMWRITE_PNG_STRATEGY_FILTERED is to force more Huffman coding and less string matching; it is somewhat intermediate between IMWRITE_PNG_STRATEGY_DEFAULT and IMWRITE_PNG_STRATEGY_HUFFMAN_ONLY.
/// *   IMWRITE_PNG_STRATEGY_RLE is designed to be almost as fast as IMWRITE_PNG_STRATEGY_HUFFMAN_ONLY, but give better compression for PNG image data.
/// *   The strategy parameter only affects the compression ratio but not the correctness of the compressed output even if it is not set appropriately.
/// *   IMWRITE_PNG_STRATEGY_FIXED prevents the use of dynamic Huffman codes, allowing for a simpler decoder for special applications.
// ImwritePNGFlags /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:131
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImwritePNGFlags {
	/// Use this value for normal data.
	IMWRITE_PNG_STRATEGY_DEFAULT = 0,
	/// Use this value for data produced by a filter (or predictor).Filtered data consists mostly of small values with a somewhat random distribution. In this case, the compression algorithm is tuned to compress them better.
	IMWRITE_PNG_STRATEGY_FILTERED = 1,
	/// Use this value to force Huffman encoding only (no string match).
	IMWRITE_PNG_STRATEGY_HUFFMAN_ONLY = 2,
	/// Use this value to limit match distances to one (run-length encoding).
	IMWRITE_PNG_STRATEGY_RLE = 3,
	/// Using this value prevents the use of dynamic Huffman codes, allowing for a simpler decoder for special applications.
	IMWRITE_PNG_STRATEGY_FIXED = 4,
}

impl TryFrom<i32> for ImwritePNGFlags {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::IMWRITE_PNG_STRATEGY_DEFAULT),
			1 => Ok(Self::IMWRITE_PNG_STRATEGY_FILTERED),
			2 => Ok(Self::IMWRITE_PNG_STRATEGY_HUFFMAN_ONLY),
			3 => Ok(Self::IMWRITE_PNG_STRATEGY_RLE),
			4 => Ok(Self::IMWRITE_PNG_STRATEGY_FIXED),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImwritePNGFlags"))),
		}
	}
}

opencv_type_enum! { crate::imgcodecs::ImwritePNGFlags }

/// Reads an image from a buffer in memory.
///
/// The function imdecode reads an image from the specified buffer in the memory. If the buffer is too short or
/// contains invalid data, the function returns an empty matrix ( Mat::data==NULL ).
///
/// See cv::imread for the list of supported formats and flags description.
///
///
/// Note: In the case of color images, the decoded images will have the channels stored in **B G R** order.
/// ## Parameters
/// * buf: Input array or vector of bytes.
/// * flags: The same flags as in cv::imread, see cv::ImreadModes.
// imdecode(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:277
// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags"], ["const cv::_InputArray*", "int"]), _)]),
#[inline]
pub fn imdecode(buf: &impl ToInputArray, flags: i32) -> Result<core::Mat> {
	input_array_arg!(buf);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imdecode_const__InputArrayR_int(buf.as_raw__InputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads an image from a buffer in memory.
///
/// The function imdecode reads an image from the specified buffer in the memory. If the buffer is too short or
/// contains invalid data, the function returns an empty matrix ( Mat::data==NULL ).
///
/// See cv::imread for the list of supported formats and flags description.
///
///
/// Note: In the case of color images, the decoded images will have the channels stored in **B G R** order.
/// ## Parameters
/// * buf: Input array or vector of bytes.
/// * flags: The same flags as in cv::imread, see cv::ImreadModes.
///
/// ## Overloaded parameters
///
/// * buf: 
/// * flags: 
/// * dst: The optional output placeholder for the decoded matrix. It can save the image
/// reallocations when the function is called repeatedly for images of the same size.
// imdecode(InputArray, int, Mat *)(InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:285
// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags", "dst"], ["const cv::_InputArray*", "int", "cv::Mat*"]), _)]),
#[inline]
pub fn imdecode_to(buf: &impl ToInputArray, flags: i32, dst: &mut impl core::MatTrait) -> Result<core::Mat> {
	input_array_arg!(buf);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imdecode_const__InputArrayR_int_MatX(buf.as_raw__InputArray(), flags, dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Encodes an image into a memory buffer.
///
/// The function imencode compresses the image and stores it in the memory buffer that is resized to fit the
/// result. See cv::imwrite for the list of supported formats and flags description.
///
/// ## Parameters
/// * ext: File extension that defines the output format. Must include a leading period.
/// * img: Image to be written.
/// * buf: Output buffer resized to fit the compressed image.
/// * params: Format-specific parameters. See cv::imwrite and cv::ImwriteFlags.
///
/// ## Note
/// This alternative version of [imencode] function uses the following default values for its arguments:
/// * params: std::vector<int>()
// cv::imencode(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:297
// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*"]), _)]),
#[inline]
pub fn imencode_def(ext: &str, img: &impl ToInputArray, buf: &mut core::Vector<u8>) -> Result<bool> {
	extern_container_arg!(ext);
	input_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR(ext.opencv_as_extern(), img.as_raw__InputArray(), buf.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Encodes an image into a memory buffer.
///
/// The function imencode compresses the image and stores it in the memory buffer that is resized to fit the
/// result. See cv::imwrite for the list of supported formats and flags description.
///
/// ## Parameters
/// * ext: File extension that defines the output format. Must include a leading period.
/// * img: Image to be written.
/// * buf: Output buffer resized to fit the compressed image.
/// * params: Format-specific parameters. See cv::imwrite and cv::ImwriteFlags.
///
/// ## C++ default parameters
/// * params: std::vector<int>()
// imencode(const String &, InputArray, std::vector<uchar> &, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:297
// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf", "params"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*", "const std::vector<int>*"]), _)]),
#[inline]
pub fn imencode(ext: &str, img: &impl ToInputArray, buf: &mut core::Vector<u8>, params: &core::Vector<i32>) -> Result<bool> {
	extern_container_arg!(ext);
	input_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(ext.opencv_as_extern(), img.as_raw__InputArray(), buf.as_raw_mut_VectorOfu8(), params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Loads an image from a file.
///
/// @anchor imread
///
/// The function imread loads an image from the specified file and returns it. If the image cannot be
/// read (because of missing file, improper permissions, unsupported or invalid format), the function
/// returns an empty matrix ( Mat::data==NULL ).
///
/// Currently, the following file formats are supported:
///
/// *   Windows bitmaps - \*.bmp, \*.dib (always supported)
/// *   JPEG files - \*.jpeg, \*.jpg, \*.jpe (see the *Note* section)
/// *   JPEG 2000 files - \*.jp2 (see the *Note* section)
/// *   Portable Network Graphics - \*.png (see the *Note* section)
/// *   WebP - \*.webp (see the *Note* section)
/// *   Portable image format - \*.pbm, \*.pgm, \*.ppm \*.pxm, \*.pnm (always supported)
/// *   Sun rasters - \*.sr, \*.ras (always supported)
/// *   TIFF files - \*.tiff, \*.tif (see the *Note* section)
/// *   OpenEXR Image files - \*.exr (see the *Note* section)
/// *   Radiance HDR - \*.hdr, \*.pic (always supported)
/// *   Raster and Vector geospatial data supported by GDAL (see the *Note* section)
///
///
/// Note:
/// *   The function determines the type of an image by the content, not by the file extension.
/// *   In the case of color images, the decoded images will have the channels stored in **B G R** order.
/// *   When using IMREAD_GRAYSCALE, the codec's internal grayscale conversion will be used, if available.
///    Results may differ to the output of cvtColor()
/// *   On Microsoft Windows\* OS and MacOSX\*, the codecs shipped with an OpenCV image (libjpeg,
///    libpng, libtiff, and libjasper) are used by default. So, OpenCV can always read JPEGs, PNGs,
///    and TIFFs. On MacOSX, there is also an option to use native MacOSX image readers. But beware
///    that currently these native image loaders give images with different pixel values because of
///    the color management embedded into MacOSX.
/// *   On Linux\*, BSD flavors and other Unix-like open-source operating systems, OpenCV looks for
///    codecs supplied with an OS image. Install the relevant packages (do not forget the development
///    files, for example, "libjpeg-dev", in Debian\* and Ubuntu\*) to get the codec support or turn
///    on the OPENCV_BUILD_3RDPARTY_LIBS flag in CMake.
/// *   In the case you set *WITH_GDAL* flag to true in CMake and [IMREAD_LOAD_GDAL] to load the image,
///    then the [GDAL](http://www.gdal.org) driver will be used in order to decode the image, supporting
///    the following formats: [Raster](http://www.gdal.org/formats_list.html),
///    [Vector](http://www.gdal.org/ogr_formats.html).
/// *   If EXIF information is embedded in the image file, the EXIF orientation will be taken into account
///    and thus the image will be rotated accordingly except if the flags [IMREAD_IGNORE_ORIENTATION]
///    or [IMREAD_UNCHANGED] are passed.
/// *   By default number of pixels must be less than 2^30. Limit can be set using system
///    variable OPENCV_IO_MAX_IMAGE_PIXELS
///
/// ## Parameters
/// * filename: Name of file to be loaded.
/// * flags: Flag that can take values of cv::ImreadModes
///
/// ## Note
/// This alternative version of [imread] function uses the following default values for its arguments:
/// * flags: IMREAD_COLOR
// cv::imread(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:206
// ("cv::imread", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
#[inline]
pub fn imread_def(filename: &str) -> Result<core::Mat> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imread_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Loads an image from a file.
///
/// @anchor imread
///
/// The function imread loads an image from the specified file and returns it. If the image cannot be
/// read (because of missing file, improper permissions, unsupported or invalid format), the function
/// returns an empty matrix ( Mat::data==NULL ).
///
/// Currently, the following file formats are supported:
///
/// *   Windows bitmaps - \*.bmp, \*.dib (always supported)
/// *   JPEG files - \*.jpeg, \*.jpg, \*.jpe (see the *Note* section)
/// *   JPEG 2000 files - \*.jp2 (see the *Note* section)
/// *   Portable Network Graphics - \*.png (see the *Note* section)
/// *   WebP - \*.webp (see the *Note* section)
/// *   Portable image format - \*.pbm, \*.pgm, \*.ppm \*.pxm, \*.pnm (always supported)
/// *   Sun rasters - \*.sr, \*.ras (always supported)
/// *   TIFF files - \*.tiff, \*.tif (see the *Note* section)
/// *   OpenEXR Image files - \*.exr (see the *Note* section)
/// *   Radiance HDR - \*.hdr, \*.pic (always supported)
/// *   Raster and Vector geospatial data supported by GDAL (see the *Note* section)
///
///
/// Note:
/// *   The function determines the type of an image by the content, not by the file extension.
/// *   In the case of color images, the decoded images will have the channels stored in **B G R** order.
/// *   When using IMREAD_GRAYSCALE, the codec's internal grayscale conversion will be used, if available.
///    Results may differ to the output of cvtColor()
/// *   On Microsoft Windows\* OS and MacOSX\*, the codecs shipped with an OpenCV image (libjpeg,
///    libpng, libtiff, and libjasper) are used by default. So, OpenCV can always read JPEGs, PNGs,
///    and TIFFs. On MacOSX, there is also an option to use native MacOSX image readers. But beware
///    that currently these native image loaders give images with different pixel values because of
///    the color management embedded into MacOSX.
/// *   On Linux\*, BSD flavors and other Unix-like open-source operating systems, OpenCV looks for
///    codecs supplied with an OS image. Install the relevant packages (do not forget the development
///    files, for example, "libjpeg-dev", in Debian\* and Ubuntu\*) to get the codec support or turn
///    on the OPENCV_BUILD_3RDPARTY_LIBS flag in CMake.
/// *   In the case you set *WITH_GDAL* flag to true in CMake and [IMREAD_LOAD_GDAL] to load the image,
///    then the [GDAL](http://www.gdal.org) driver will be used in order to decode the image, supporting
///    the following formats: [Raster](http://www.gdal.org/formats_list.html),
///    [Vector](http://www.gdal.org/ogr_formats.html).
/// *   If EXIF information is embedded in the image file, the EXIF orientation will be taken into account
///    and thus the image will be rotated accordingly except if the flags [IMREAD_IGNORE_ORIENTATION]
///    or [IMREAD_UNCHANGED] are passed.
/// *   By default number of pixels must be less than 2^30. Limit can be set using system
///    variable OPENCV_IO_MAX_IMAGE_PIXELS
///
/// ## Parameters
/// * filename: Name of file to be loaded.
/// * flags: Flag that can take values of cv::ImreadModes
///
/// ## C++ default parameters
/// * flags: IMREAD_COLOR
// imread(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:206
// ("cv::imread", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
#[inline]
pub fn imread(filename: &str, flags: i32) -> Result<core::Mat> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imread_const_StringR_int(filename.opencv_as_extern(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Loads a multi-page image from a file.
///
/// The function imreadmulti loads a multi-page image from the specified file into a vector of Mat objects.
/// ## Parameters
/// * filename: Name of file to be loaded.
/// * mats: A vector of Mat objects holding each page.
/// * flags: Flag that can take values of cv::ImreadModes, default with cv::IMREAD_ANYCOLOR.
/// ## See also
/// cv::imread
///
/// ## Note
/// This alternative version of [imreadmulti] function uses the following default values for its arguments:
/// * flags: IMREAD_ANYCOLOR
// cv::imreadmulti(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:216
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats"], ["const cv::String*", "std::vector<cv::Mat>*"]), _)]),
#[inline]
pub fn imreadmulti_def(filename: &str, mats: &mut core::Vector<core::Mat>) -> Result<bool> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imreadmulti_const_StringR_vectorLMatGR(filename.opencv_as_extern(), mats.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Loads a multi-page image from a file.
///
/// The function imreadmulti loads a multi-page image from the specified file into a vector of Mat objects.
/// ## Parameters
/// * filename: Name of file to be loaded.
/// * mats: A vector of Mat objects holding each page.
/// * flags: Flag that can take values of cv::ImreadModes, default with cv::IMREAD_ANYCOLOR.
/// ## See also
/// cv::imread
///
/// ## C++ default parameters
/// * flags: IMREAD_ANYCOLOR
// imreadmulti(const String &, std::vector<Mat> &, int)(InString, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:216
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "flags"], ["const cv::String*", "std::vector<cv::Mat>*", "int"]), _)]),
#[inline]
pub fn imreadmulti(filename: &str, mats: &mut core::Vector<core::Mat>, flags: i32) -> Result<bool> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imreadmulti_const_StringR_vectorLMatGR_int(filename.opencv_as_extern(), mats.as_raw_mut_VectorOfMat(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Saves an image to a specified file.
///
/// The function imwrite saves the image to the specified file. The image format is chosen based on the
/// filename extension (see cv::imread for the list of extensions). In general, only 8-bit unsigned (CV_8U)
/// single-channel or 3-channel (with 'BGR' channel order) images
/// can be saved using this function, with these exceptions:
///
/// - With OpenEXR encoder, only 32-bit float (CV_32F) images can be saved.
///   - 8-bit unsigned (CV_8U) images are not supported.
/// - With Radiance HDR encoder, non 64-bit float (CV_64F) images can be saved.
///   - All images will be converted to 32-bit float (CV_32F).
/// - With JPEG 2000 encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
/// - With PAM encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
/// - With PNG encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
///   - PNG images with an alpha channel can be saved using this function. To do this, create
///    8-bit (or 16-bit) 4-channel image BGRA, where the alpha channel goes last. Fully transparent pixels
///    should have alpha set to 0, fully opaque pixels should have alpha set to 255/65535 (see the code sample below).
/// - With PGM/PPM encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
/// - With TIFF encoder, 8-bit unsigned (CV_8U), 16-bit unsigned (CV_16U),
///                      32-bit float (CV_32F) and 64-bit float (CV_64F) images can be saved.
///   - Multiple images (vector of Mat) can be saved in TIFF format (see the code sample below).
///   - 32-bit float 3-channel (CV_32FC3) TIFF images will be saved
///    using the LogLuv high dynamic range encoding (4 bytes per pixel)
///
/// If the image format is not supported, the image will be converted to 8-bit unsigned (CV_8U) and saved that way.
///
/// If the format, depth or channel order is different, use
/// Mat::convertTo and cv::cvtColor to convert it before saving. Or, use the universal FileStorage I/O
/// functions to save the image to XML or YAML format.
///
/// The sample below shows how to create a BGRA image, how to set custom compression parameters and save it to a PNG file.
/// It also demonstrates how to save multiple images in a TIFF file:
/// @include snippets/imgcodecs_imwrite.cpp
/// ## Parameters
/// * filename: Name of the file.
/// * img: (Mat or vector of Mat) Image or Images to be saved.
/// * params: Format-specific parameters encoded as pairs (paramId_1, paramValue_1, paramId_2, paramValue_2, ... .) see cv::ImwriteFlags
///
/// ## Note
/// This alternative version of [imwrite] function uses the following default values for its arguments:
/// * params: std::vector<int>()
// cv::imwrite(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:255
// ("cv::imwrite", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn imwrite_def(filename: &str, img: &impl ToInputArray) -> Result<bool> {
	extern_container_arg!(filename);
	input_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imwrite_const_StringR_const__InputArrayR(filename.opencv_as_extern(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Saves an image to a specified file.
///
/// The function imwrite saves the image to the specified file. The image format is chosen based on the
/// filename extension (see cv::imread for the list of extensions). In general, only 8-bit unsigned (CV_8U)
/// single-channel or 3-channel (with 'BGR' channel order) images
/// can be saved using this function, with these exceptions:
///
/// - With OpenEXR encoder, only 32-bit float (CV_32F) images can be saved.
///   - 8-bit unsigned (CV_8U) images are not supported.
/// - With Radiance HDR encoder, non 64-bit float (CV_64F) images can be saved.
///   - All images will be converted to 32-bit float (CV_32F).
/// - With JPEG 2000 encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
/// - With PAM encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
/// - With PNG encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
///   - PNG images with an alpha channel can be saved using this function. To do this, create
///    8-bit (or 16-bit) 4-channel image BGRA, where the alpha channel goes last. Fully transparent pixels
///    should have alpha set to 0, fully opaque pixels should have alpha set to 255/65535 (see the code sample below).
/// - With PGM/PPM encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
/// - With TIFF encoder, 8-bit unsigned (CV_8U), 16-bit unsigned (CV_16U),
///                      32-bit float (CV_32F) and 64-bit float (CV_64F) images can be saved.
///   - Multiple images (vector of Mat) can be saved in TIFF format (see the code sample below).
///   - 32-bit float 3-channel (CV_32FC3) TIFF images will be saved
///    using the LogLuv high dynamic range encoding (4 bytes per pixel)
///
/// If the image format is not supported, the image will be converted to 8-bit unsigned (CV_8U) and saved that way.
///
/// If the format, depth or channel order is different, use
/// Mat::convertTo and cv::cvtColor to convert it before saving. Or, use the universal FileStorage I/O
/// functions to save the image to XML or YAML format.
///
/// The sample below shows how to create a BGRA image, how to set custom compression parameters and save it to a PNG file.
/// It also demonstrates how to save multiple images in a TIFF file:
/// @include snippets/imgcodecs_imwrite.cpp
/// ## Parameters
/// * filename: Name of the file.
/// * img: (Mat or vector of Mat) Image or Images to be saved.
/// * params: Format-specific parameters encoded as pairs (paramId_1, paramValue_1, paramId_2, paramValue_2, ... .) see cv::ImwriteFlags
///
/// ## C++ default parameters
/// * params: std::vector<int>()
// imwrite(const String &, InputArray, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:255
// ("cv::imwrite", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
#[inline]
pub fn imwrite(filename: &str, img: &impl ToInputArray, params: &core::Vector<i32>) -> Result<bool> {
	extern_container_arg!(filename);
	input_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imwrite_const_StringR_const__InputArrayR_const_vectorLintGR(filename.opencv_as_extern(), img.as_raw__InputArray(), params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload multi-image overload for bindings
///
/// ## Note
/// This alternative version of [imwritemulti] function uses the following default values for its arguments:
/// * params: std::vector<int>()
// cv::imwritemulti(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:260
// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn imwritemulti_def(filename: &str, img: &impl ToInputArray) -> Result<bool> {
	extern_container_arg!(filename);
	input_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imwritemulti_const_StringR_const__InputArrayR(filename.opencv_as_extern(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts. multi-image overload for bindings
///
/// ## C++ default parameters
/// * params: std::vector<int>()
// imwritemulti(const String &, InputArrayOfArrays, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:260
// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
#[inline]
pub fn imwritemulti(filename: &str, img: &impl ToInputArray, params: &core::Vector<i32>) -> Result<bool> {
	extern_container_arg!(filename);
	input_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_imwritemulti_const_StringR_const__InputArrayR_const_vectorLintGR(filename.opencv_as_extern(), img.as_raw__InputArray(), params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

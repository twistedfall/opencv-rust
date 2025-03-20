pub mod imgcodecs {
	//! # Image file reading and writing
	//!   # Flags used for image file reading and writing
	//!   # iOS glue
	//!   # MacOS(OSX) glue
	use crate::mod_prelude::*;
	use crate::{core, sys, types};
	pub mod prelude {
		pub use super::{AnimationTrait, AnimationTraitConst, ImageCollectionTrait, ImageCollectionTraitConst, ImageCollection_iteratorTrait, ImageCollection_iteratorTraitConst};
	}

	/// If set, the image is read in any possible color format.
	pub const IMREAD_ANYCOLOR: i32 = 4;
	/// If set, return 16-bit/32-bit image when the input has the corresponding depth, otherwise convert it to 8-bit.
	pub const IMREAD_ANYDEPTH: i32 = 2;
	/// Same as IMREAD_COLOR_BGR.
	pub const IMREAD_COLOR: i32 = 1;
	/// If set, always convert image to the 3 channel BGR color image.
	pub const IMREAD_COLOR_BGR: i32 = 1;
	/// If set, always convert image to the 3 channel RGB color image.
	pub const IMREAD_COLOR_RGB: i32 = 256;
	/// If set, always convert image to the single channel grayscale image (codec internal conversion).
	pub const IMREAD_GRAYSCALE: i32 = 0;
	/// If set, do not rotate the image according to EXIF's orientation flag.
	pub const IMREAD_IGNORE_ORIENTATION: i32 = 128;
	/// If set, use the gdal driver for loading the image.
	pub const IMREAD_LOAD_GDAL: i32 = 8;
	/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/2.
	pub const IMREAD_REDUCED_COLOR_2: i32 = 17;
	/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/4.
	pub const IMREAD_REDUCED_COLOR_4: i32 = 33;
	/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/8.
	pub const IMREAD_REDUCED_COLOR_8: i32 = 65;
	/// If set, always convert image to the single channel grayscale image and the image size reduced 1/2.
	pub const IMREAD_REDUCED_GRAYSCALE_2: i32 = 16;
	/// If set, always convert image to the single channel grayscale image and the image size reduced 1/4.
	pub const IMREAD_REDUCED_GRAYSCALE_4: i32 = 32;
	/// If set, always convert image to the single channel grayscale image and the image size reduced 1/8.
	pub const IMREAD_REDUCED_GRAYSCALE_8: i32 = 64;
	/// If set, return the loaded image as is (with alpha channel, otherwise it gets cropped). Ignore EXIF orientation.
	pub const IMREAD_UNCHANGED: i32 = -1;
	/// For AVIF, it can be 8, 10 or 12. If >8, it is stored/read as CV_32F. Default is 8.
	pub const IMWRITE_AVIF_DEPTH: i32 = 513;
	/// For AVIF, it can be a quality between 0 and 100 (the higher the better). Default is 95.
	pub const IMWRITE_AVIF_QUALITY: i32 = 512;
	/// For AVIF, it is between 0 (slowest) and (fastest). Default is 9.
	pub const IMWRITE_AVIF_SPEED: i32 = 514;
	/// override EXR compression type (ZIP_COMPRESSION = 3 is default)
	pub const IMWRITE_EXR_COMPRESSION: i32 = 49;
	/// lossy 4-by-4 pixel block compression, fixed compression rate
	pub const IMWRITE_EXR_COMPRESSION_B44: i32 = 6;
	/// lossy 4-by-4 pixel block compression, flat fields are compressed more
	pub const IMWRITE_EXR_COMPRESSION_B44A: i32 = 7;
	/// lossy DCT based compression, in blocks of 32 scanlines. More efficient for partial buffer access. Supported since OpenEXR 2.2.0.
	pub const IMWRITE_EXR_COMPRESSION_DWAA: i32 = 8;
	/// lossy DCT based compression, in blocks of 256 scanlines. More efficient space wise and faster to decode full frames than DWAA_COMPRESSION. Supported since OpenEXR 2.2.0.
	pub const IMWRITE_EXR_COMPRESSION_DWAB: i32 = 9;
	/// no compression
	pub const IMWRITE_EXR_COMPRESSION_NO: i32 = 0;
	/// piz-based wavelet compression
	pub const IMWRITE_EXR_COMPRESSION_PIZ: i32 = 4;
	/// lossy 24-bit float compression
	pub const IMWRITE_EXR_COMPRESSION_PXR24: i32 = 5;
	/// run length encoding
	pub const IMWRITE_EXR_COMPRESSION_RLE: i32 = 1;
	/// zlib compression, in blocks of 16 scan lines
	pub const IMWRITE_EXR_COMPRESSION_ZIP: i32 = 3;
	/// zlib compression, one scan line at a time
	pub const IMWRITE_EXR_COMPRESSION_ZIPS: i32 = 2;
	/// override EXR DWA compression level (45 is default)
	pub const IMWRITE_EXR_DWA_COMPRESSION_LEVEL: i32 = 50;
	/// override EXR storage type (FLOAT (FP32) is default)
	pub const IMWRITE_EXR_TYPE: i32 = 48;
	/// store as FP32 (default)
	pub const IMWRITE_EXR_TYPE_FLOAT: i32 = 2;
	/// store as HALF (FP16)
	pub const IMWRITE_EXR_TYPE_HALF: i32 = 1;
	/// For GIF, 0 means global color table is used, 1 means local color table is used. Default is 0.
	pub const IMWRITE_GIF_COLORTABLE: i32 = 1029;
	pub const IMWRITE_GIF_COLORTABLE_SIZE_128: i32 = 7;
	pub const IMWRITE_GIF_COLORTABLE_SIZE_16: i32 = 4;
	pub const IMWRITE_GIF_COLORTABLE_SIZE_256: i32 = 8;
	pub const IMWRITE_GIF_COLORTABLE_SIZE_32: i32 = 5;
	pub const IMWRITE_GIF_COLORTABLE_SIZE_64: i32 = 6;
	pub const IMWRITE_GIF_COLORTABLE_SIZE_8: i32 = 3;
	/// For GIF, it can be a quality from -1(most dither) to 3(no dither). Default is 0.
	pub const IMWRITE_GIF_DITHER: i32 = 1027;
	pub const IMWRITE_GIF_FAST_FLOYD_DITHER: i32 = 2;
	pub const IMWRITE_GIF_FAST_NO_DITHER: i32 = 1;
	/// For GIF, it can be a loop flag from 0 to 65535. Default is 0 - loop forever.
	pub const IMWRITE_GIF_LOOP: i32 = 1024;
	/// For GIF, it can be a quality from 1 to 8. Default is 2. See cv::ImwriteGifCompressionFlags.
	pub const IMWRITE_GIF_QUALITY: i32 = 1026;
	/// For GIF, it is between 1 (slowest) and 100 (fastest). Default is 96.
	pub const IMWRITE_GIF_SPEED: i32 = 1025;
	/// For GIF, the alpha channel lower than this will be set to transparent. Default is 1.
	pub const IMWRITE_GIF_TRANSPARENCY: i32 = 1028;
	/// specify HDR compression
	pub const IMWRITE_HDR_COMPRESSION: i32 = 80;
	pub const IMWRITE_HDR_COMPRESSION_NONE: i32 = 0;
	pub const IMWRITE_HDR_COMPRESSION_RLE: i32 = 1;
	/// For JPEG2000, use to specify the target compression rate (multiplied by 1000). The value can be from 0 to 1000. Default is 1000.
	pub const IMWRITE_JPEG2000_COMPRESSION_X1000: i32 = 272;
	/// For JPEG XL, decoding speed tier for the provided options; minimum is 0 (slowest to decode, best quality/density), and maximum is 4 (fastest to decode, at the cost of some quality/density). Default is 0.
	pub const IMWRITE_JPEGXL_DECODING_SPEED: i32 = 643;
	/// For JPEG XL, distance level for lossy compression: target max butteraugli distance, lower = higher quality, 0 = lossless; range: 0 .. 25. Default is 1.
	pub const IMWRITE_JPEGXL_DISTANCE: i32 = 642;
	/// For JPEG XL, encoder effort/speed level without affecting decoding speed; it is between 1 (fastest) and 10 (slowest). Default is 7.
	pub const IMWRITE_JPEGXL_EFFORT: i32 = 641;
	/// For JPEG XL, it can be a quality from 0 to 100 (the higher is the better). Default value is 95. If set, distance parameter is re-calicurated from quality level automatically. This parameter request libjxl v0.10 or later.
	pub const IMWRITE_JPEGXL_QUALITY: i32 = 640;
	/// Separate chroma quality level, 0 - 100, default is -1 - don't use. If JPEG_LIB_VERSION < 70, Not supported.
	pub const IMWRITE_JPEG_CHROMA_QUALITY: i32 = 6;
	/// Separate luma quality level, 0 - 100, default is -1 - don't use. If JPEG_LIB_VERSION < 70, Not supported.
	pub const IMWRITE_JPEG_LUMA_QUALITY: i32 = 5;
	/// Enable JPEG features, 0 or 1, default is False.
	pub const IMWRITE_JPEG_OPTIMIZE: i32 = 3;
	/// Enable JPEG features, 0 or 1, default is False.
	pub const IMWRITE_JPEG_PROGRESSIVE: i32 = 2;
	/// For JPEG, it can be a quality from 0 to 100 (the higher is the better). Default value is 95.
	pub const IMWRITE_JPEG_QUALITY: i32 = 1;
	/// JPEG restart interval, 0 - 65535, default is 0 - no restart.
	pub const IMWRITE_JPEG_RST_INTERVAL: i32 = 4;
	/// For JPEG, set sampling factor. See cv::ImwriteJPEGSamplingFactorParams.
	pub const IMWRITE_JPEG_SAMPLING_FACTOR: i32 = 7;
	/// 4x1,1x1,1x1
	pub const IMWRITE_JPEG_SAMPLING_FACTOR_411: i32 = 4264209;
	/// 2x2,1x1,1x1(Default)
	pub const IMWRITE_JPEG_SAMPLING_FACTOR_420: i32 = 2232593;
	/// 2x1,1x1,1x1
	pub const IMWRITE_JPEG_SAMPLING_FACTOR_422: i32 = 2167057;
	/// 1x2,1x1,1x1
	pub const IMWRITE_JPEG_SAMPLING_FACTOR_440: i32 = 1184017;
	/// 1x1,1x1,1x1(No subsampling)
	pub const IMWRITE_JPEG_SAMPLING_FACTOR_444: i32 = 1118481;
	pub const IMWRITE_PAM_FORMAT_BLACKANDWHITE: i32 = 1;
	pub const IMWRITE_PAM_FORMAT_GRAYSCALE: i32 = 2;
	pub const IMWRITE_PAM_FORMAT_GRAYSCALE_ALPHA: i32 = 3;
	pub const IMWRITE_PAM_FORMAT_NULL: i32 = 0;
	pub const IMWRITE_PAM_FORMAT_RGB: i32 = 4;
	pub const IMWRITE_PAM_FORMAT_RGB_ALPHA: i32 = 5;
	/// For PAM, sets the TUPLETYPE field to the corresponding string value that is defined for the format
	pub const IMWRITE_PAM_TUPLETYPE: i32 = 128;
	/// Binary level PNG, 0 or 1, default is 0.
	pub const IMWRITE_PNG_BILEVEL: i32 = 18;
	/// For PNG, it can be the compression level from 0 to 9. A higher value means a smaller size and longer compression time. If specified, strategy is changed to IMWRITE_PNG_STRATEGY_DEFAULT (Z_DEFAULT_STRATEGY). Default value is 1 (best speed setting).
	pub const IMWRITE_PNG_COMPRESSION: i32 = 16;
	/// One of cv::ImwritePNGFlags, default is IMWRITE_PNG_STRATEGY_RLE.
	pub const IMWRITE_PNG_STRATEGY: i32 = 17;
	/// Use this value for normal data.
	pub const IMWRITE_PNG_STRATEGY_DEFAULT: i32 = 0;
	/// Use this value for data produced by a filter (or predictor).Filtered data consists mostly of small values with a somewhat random distribution. In this case, the compression algorithm is tuned to compress them better.
	pub const IMWRITE_PNG_STRATEGY_FILTERED: i32 = 1;
	/// Using this value prevents the use of dynamic Huffman codes, allowing for a simpler decoder for special applications.
	pub const IMWRITE_PNG_STRATEGY_FIXED: i32 = 4;
	/// Use this value to force Huffman encoding only (no string match).
	pub const IMWRITE_PNG_STRATEGY_HUFFMAN_ONLY: i32 = 2;
	/// Use this value to limit match distances to one (run-length encoding).
	pub const IMWRITE_PNG_STRATEGY_RLE: i32 = 3;
	/// For PPM, PGM, or PBM, it can be a binary format flag, 0 or 1. Default value is 1.
	pub const IMWRITE_PXM_BINARY: i32 = 32;
	/// For TIFF, use to specify the image compression scheme. See cv::ImwriteTiffCompressionFlags. Note, for images whose depth is CV_32F, only libtiff's SGILOG compression scheme is used. For other supported depths, the compression scheme can be specified by this flag; LZW compression is the default.
	pub const IMWRITE_TIFF_COMPRESSION: i32 = 259;
	/// Deflate compression, as recognized by Adobe
	pub const IMWRITE_TIFF_COMPRESSION_ADOBE_DEFLATE: i32 = 8;
	/// CCITT Group 3 fax encoding
	pub const IMWRITE_TIFF_COMPRESSION_CCITTFAX3: i32 = 3;
	/// CCITT Group 4 fax encoding
	pub const IMWRITE_TIFF_COMPRESSION_CCITTFAX4: i32 = 4;
	/// CCITT modified Huffman RLE
	pub const IMWRITE_TIFF_COMPRESSION_CCITTRLE: i32 = 2;
	/// [1] w/ word alignment
	pub const IMWRITE_TIFF_COMPRESSION_CCITTRLEW: i32 = 32771;
	/// CCITT T.4 (TIFF 6 name)
	pub const IMWRITE_TIFF_COMPRESSION_CCITT_T4: i32 = 3;
	/// CCITT T.6 (TIFF 6 name)
	pub const IMWRITE_TIFF_COMPRESSION_CCITT_T6: i32 = 4;
	/// Kodak DCS encoding
	pub const IMWRITE_TIFF_COMPRESSION_DCS: i32 = 32947;
	/// Deflate compression, legacy tag
	pub const IMWRITE_TIFF_COMPRESSION_DEFLATE: i32 = 32946;
	/// IT8 Binary line art
	pub const IMWRITE_TIFF_COMPRESSION_IT8BL: i32 = 32898;
	/// IT8 CT w/padding
	pub const IMWRITE_TIFF_COMPRESSION_IT8CTPAD: i32 = 32895;
	/// IT8 Linework RLE
	pub const IMWRITE_TIFF_COMPRESSION_IT8LW: i32 = 32896;
	/// IT8 Monochrome picture
	pub const IMWRITE_TIFF_COMPRESSION_IT8MP: i32 = 32897;
	/// ISO JBIG
	pub const IMWRITE_TIFF_COMPRESSION_JBIG: i32 = 34661;
	/// Leadtools JPEG2000
	pub const IMWRITE_TIFF_COMPRESSION_JP2000: i32 = 34712;
	/// %JPEG DCT compression
	pub const IMWRITE_TIFF_COMPRESSION_JPEG: i32 = 7;
	/// JPEGXL: WARNING not registered in Adobe-maintained registry
	pub const IMWRITE_TIFF_COMPRESSION_JXL: i32 = 50002;
	/// ESRI Lerc codec: <https://github.com/Esri/lerc>
	pub const IMWRITE_TIFF_COMPRESSION_LERC: i32 = 34887;
	/// LZMA2
	pub const IMWRITE_TIFF_COMPRESSION_LZMA: i32 = 34925;
	/// Lempel-Ziv  & Welch
	pub const IMWRITE_TIFF_COMPRESSION_LZW: i32 = 5;
	/// NeXT 2-bit RLE
	pub const IMWRITE_TIFF_COMPRESSION_NEXT: i32 = 32766;
	/// dump mode
	pub const IMWRITE_TIFF_COMPRESSION_NONE: i32 = 1;
	/// !6.0 JPEG
	pub const IMWRITE_TIFF_COMPRESSION_OJPEG: i32 = 6;
	/// Macintosh RLE
	pub const IMWRITE_TIFF_COMPRESSION_PACKBITS: i32 = 32773;
	/// Pixar companded 10bit LZW
	pub const IMWRITE_TIFF_COMPRESSION_PIXARFILM: i32 = 32908;
	/// Pixar companded 11bit ZIP
	pub const IMWRITE_TIFF_COMPRESSION_PIXARLOG: i32 = 32909;
	/// SGI Log Luminance RLE
	pub const IMWRITE_TIFF_COMPRESSION_SGILOG: i32 = 34676;
	/// SGI Log 24-bit packed
	pub const IMWRITE_TIFF_COMPRESSION_SGILOG24: i32 = 34677;
	/// !TIFF/FX T.43 colour by layered JBIG compression
	pub const IMWRITE_TIFF_COMPRESSION_T43: i32 = 10;
	/// !TIFF/FX T.85 JBIG compression
	pub const IMWRITE_TIFF_COMPRESSION_T85: i32 = 9;
	/// ThunderScan RLE
	pub const IMWRITE_TIFF_COMPRESSION_THUNDERSCAN: i32 = 32809;
	/// WEBP: WARNING not registered in Adobe-maintained registry
	pub const IMWRITE_TIFF_COMPRESSION_WEBP: i32 = 50001;
	/// ZSTD: WARNING not registered in Adobe-maintained registry
	pub const IMWRITE_TIFF_COMPRESSION_ZSTD: i32 = 50000;
	/// For TIFF, use to specify predictor. See cv::ImwriteTiffPredictorFlags.
	pub const IMWRITE_TIFF_PREDICTOR: i32 = 317;
	/// floating point predictor
	pub const IMWRITE_TIFF_PREDICTOR_FLOATINGPOINT: i32 = 3;
	/// horizontal differencing
	pub const IMWRITE_TIFF_PREDICTOR_HORIZONTAL: i32 = 2;
	/// no prediction scheme used
	pub const IMWRITE_TIFF_PREDICTOR_NONE: i32 = 1;
	/// For TIFF, use to specify which DPI resolution unit to set; see libtiff documentation for valid values
	pub const IMWRITE_TIFF_RESUNIT: i32 = 256;
	/// For TIFF, use to specify the number of rows per strip.
	pub const IMWRITE_TIFF_ROWSPERSTRIP: i32 = 278;
	/// For TIFF, use to specify the X direction DPI
	pub const IMWRITE_TIFF_XDPI: i32 = 257;
	/// For TIFF, use to specify the Y direction DPI
	pub const IMWRITE_TIFF_YDPI: i32 = 258;
	/// For WEBP, it can be a quality from 1 to 100 (the higher is the better). By default (without any parameter) and for quality above 100 the lossless compression is used.
	pub const IMWRITE_WEBP_QUALITY: i32 = 64;
	/// Imread flags
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ImreadModes {
		/// If set, return the loaded image as is (with alpha channel, otherwise it gets cropped). Ignore EXIF orientation.
		IMREAD_UNCHANGED = -1,
		/// If set, always convert image to the single channel grayscale image (codec internal conversion).
		IMREAD_GRAYSCALE = 0,
		/// If set, always convert image to the 3 channel BGR color image.
		IMREAD_COLOR_BGR = 1,
		// Same as IMREAD_COLOR_BGR.
		// Duplicate, use IMREAD_COLOR_BGR instead
		// IMREAD_COLOR = 1,
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
		/// If set, always convert image to the 3 channel RGB color image.
		IMREAD_COLOR_RGB = 256,
	}

	impl TryFrom<i32> for ImreadModes {
		type Error = crate::Error;

		fn try_from(value: i32) -> Result<Self, Self::Error> {
			match value {
				-1 => Ok(Self::IMREAD_UNCHANGED),
				0 => Ok(Self::IMREAD_GRAYSCALE),
				1 => Ok(Self::IMREAD_COLOR_BGR),
				// Duplicate of IMREAD_COLOR_BGR
				// 1 => Ok(Self::IMREAD_COLOR),
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
				256 => Ok(Self::IMREAD_COLOR_RGB),
				_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImreadModes"))),
			}
		}
	}

	opencv_type_enum! { crate::imgcodecs::ImreadModes }

	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ImwriteEXRCompressionFlags {
		/// no compression
		IMWRITE_EXR_COMPRESSION_NO = 0,
		/// run length encoding
		IMWRITE_EXR_COMPRESSION_RLE = 1,
		/// zlib compression, one scan line at a time
		IMWRITE_EXR_COMPRESSION_ZIPS = 2,
		/// zlib compression, in blocks of 16 scan lines
		IMWRITE_EXR_COMPRESSION_ZIP = 3,
		/// piz-based wavelet compression
		IMWRITE_EXR_COMPRESSION_PIZ = 4,
		/// lossy 24-bit float compression
		IMWRITE_EXR_COMPRESSION_PXR24 = 5,
		/// lossy 4-by-4 pixel block compression, fixed compression rate
		IMWRITE_EXR_COMPRESSION_B44 = 6,
		/// lossy 4-by-4 pixel block compression, flat fields are compressed more
		IMWRITE_EXR_COMPRESSION_B44A = 7,
		/// lossy DCT based compression, in blocks of 32 scanlines. More efficient for partial buffer access. Supported since OpenEXR 2.2.0.
		IMWRITE_EXR_COMPRESSION_DWAA = 8,
		/// lossy DCT based compression, in blocks of 256 scanlines. More efficient space wise and faster to decode full frames than DWAA_COMPRESSION. Supported since OpenEXR 2.2.0.
		IMWRITE_EXR_COMPRESSION_DWAB = 9,
	}

	impl TryFrom<i32> for ImwriteEXRCompressionFlags {
		type Error = crate::Error;

		fn try_from(value: i32) -> Result<Self, Self::Error> {
			match value {
				0 => Ok(Self::IMWRITE_EXR_COMPRESSION_NO),
				1 => Ok(Self::IMWRITE_EXR_COMPRESSION_RLE),
				2 => Ok(Self::IMWRITE_EXR_COMPRESSION_ZIPS),
				3 => Ok(Self::IMWRITE_EXR_COMPRESSION_ZIP),
				4 => Ok(Self::IMWRITE_EXR_COMPRESSION_PIZ),
				5 => Ok(Self::IMWRITE_EXR_COMPRESSION_PXR24),
				6 => Ok(Self::IMWRITE_EXR_COMPRESSION_B44),
				7 => Ok(Self::IMWRITE_EXR_COMPRESSION_B44A),
				8 => Ok(Self::IMWRITE_EXR_COMPRESSION_DWAA),
				9 => Ok(Self::IMWRITE_EXR_COMPRESSION_DWAB),
				_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImwriteEXRCompressionFlags"))),
			}
		}
	}

	opencv_type_enum! { crate::imgcodecs::ImwriteEXRCompressionFlags }

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
		/// Separate luma quality level, 0 - 100, default is -1 - don't use. If JPEG_LIB_VERSION < 70, Not supported.
		IMWRITE_JPEG_LUMA_QUALITY = 5,
		/// Separate chroma quality level, 0 - 100, default is -1 - don't use. If JPEG_LIB_VERSION < 70, Not supported.
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
		/// override EXR compression type (ZIP_COMPRESSION = 3 is default)
		IMWRITE_EXR_COMPRESSION = 49,
		/// override EXR DWA compression level (45 is default)
		IMWRITE_EXR_DWA_COMPRESSION_LEVEL = 50,
		/// For WEBP, it can be a quality from 1 to 100 (the higher is the better). By default (without any parameter) and for quality above 100 the lossless compression is used.
		IMWRITE_WEBP_QUALITY = 64,
		/// specify HDR compression
		IMWRITE_HDR_COMPRESSION = 80,
		/// For PAM, sets the TUPLETYPE field to the corresponding string value that is defined for the format
		IMWRITE_PAM_TUPLETYPE = 128,
		/// For TIFF, use to specify which DPI resolution unit to set; see libtiff documentation for valid values
		IMWRITE_TIFF_RESUNIT = 256,
		/// For TIFF, use to specify the X direction DPI
		IMWRITE_TIFF_XDPI = 257,
		/// For TIFF, use to specify the Y direction DPI
		IMWRITE_TIFF_YDPI = 258,
		/// For TIFF, use to specify the image compression scheme. See cv::ImwriteTiffCompressionFlags. Note, for images whose depth is CV_32F, only libtiff's SGILOG compression scheme is used. For other supported depths, the compression scheme can be specified by this flag; LZW compression is the default.
		IMWRITE_TIFF_COMPRESSION = 259,
		/// For TIFF, use to specify the number of rows per strip.
		IMWRITE_TIFF_ROWSPERSTRIP = 278,
		/// For TIFF, use to specify predictor. See cv::ImwriteTiffPredictorFlags.
		IMWRITE_TIFF_PREDICTOR = 317,
		/// For JPEG2000, use to specify the target compression rate (multiplied by 1000). The value can be from 0 to 1000. Default is 1000.
		IMWRITE_JPEG2000_COMPRESSION_X1000 = 272,
		/// For AVIF, it can be a quality between 0 and 100 (the higher the better). Default is 95.
		IMWRITE_AVIF_QUALITY = 512,
		/// For AVIF, it can be 8, 10 or 12. If >8, it is stored/read as CV_32F. Default is 8.
		IMWRITE_AVIF_DEPTH = 513,
		/// For AVIF, it is between 0 (slowest) and (fastest). Default is 9.
		IMWRITE_AVIF_SPEED = 514,
		/// For JPEG XL, it can be a quality from 0 to 100 (the higher is the better). Default value is 95. If set, distance parameter is re-calicurated from quality level automatically. This parameter request libjxl v0.10 or later.
		IMWRITE_JPEGXL_QUALITY = 640,
		/// For JPEG XL, encoder effort/speed level without affecting decoding speed; it is between 1 (fastest) and 10 (slowest). Default is 7.
		IMWRITE_JPEGXL_EFFORT = 641,
		/// For JPEG XL, distance level for lossy compression: target max butteraugli distance, lower = higher quality, 0 = lossless; range: 0 .. 25. Default is 1.
		IMWRITE_JPEGXL_DISTANCE = 642,
		/// For JPEG XL, decoding speed tier for the provided options; minimum is 0 (slowest to decode, best quality/density), and maximum is 4 (fastest to decode, at the cost of some quality/density). Default is 0.
		IMWRITE_JPEGXL_DECODING_SPEED = 643,
		/// For GIF, it can be a loop flag from 0 to 65535. Default is 0 - loop forever.
		IMWRITE_GIF_LOOP = 1024,
		/// For GIF, it is between 1 (slowest) and 100 (fastest). Default is 96.
		IMWRITE_GIF_SPEED = 1025,
		/// For GIF, it can be a quality from 1 to 8. Default is 2. See cv::ImwriteGifCompressionFlags.
		IMWRITE_GIF_QUALITY = 1026,
		/// For GIF, it can be a quality from -1(most dither) to 3(no dither). Default is 0.
		IMWRITE_GIF_DITHER = 1027,
		/// For GIF, the alpha channel lower than this will be set to transparent. Default is 1.
		IMWRITE_GIF_TRANSPARENCY = 1028,
		/// For GIF, 0 means global color table is used, 1 means local color table is used. Default is 0.
		IMWRITE_GIF_COLORTABLE = 1029,
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
				49 => Ok(Self::IMWRITE_EXR_COMPRESSION),
				50 => Ok(Self::IMWRITE_EXR_DWA_COMPRESSION_LEVEL),
				64 => Ok(Self::IMWRITE_WEBP_QUALITY),
				80 => Ok(Self::IMWRITE_HDR_COMPRESSION),
				128 => Ok(Self::IMWRITE_PAM_TUPLETYPE),
				256 => Ok(Self::IMWRITE_TIFF_RESUNIT),
				257 => Ok(Self::IMWRITE_TIFF_XDPI),
				258 => Ok(Self::IMWRITE_TIFF_YDPI),
				259 => Ok(Self::IMWRITE_TIFF_COMPRESSION),
				278 => Ok(Self::IMWRITE_TIFF_ROWSPERSTRIP),
				317 => Ok(Self::IMWRITE_TIFF_PREDICTOR),
				272 => Ok(Self::IMWRITE_JPEG2000_COMPRESSION_X1000),
				512 => Ok(Self::IMWRITE_AVIF_QUALITY),
				513 => Ok(Self::IMWRITE_AVIF_DEPTH),
				514 => Ok(Self::IMWRITE_AVIF_SPEED),
				640 => Ok(Self::IMWRITE_JPEGXL_QUALITY),
				641 => Ok(Self::IMWRITE_JPEGXL_EFFORT),
				642 => Ok(Self::IMWRITE_JPEGXL_DISTANCE),
				643 => Ok(Self::IMWRITE_JPEGXL_DECODING_SPEED),
				1024 => Ok(Self::IMWRITE_GIF_LOOP),
				1025 => Ok(Self::IMWRITE_GIF_SPEED),
				1026 => Ok(Self::IMWRITE_GIF_QUALITY),
				1027 => Ok(Self::IMWRITE_GIF_DITHER),
				1028 => Ok(Self::IMWRITE_GIF_TRANSPARENCY),
				1029 => Ok(Self::IMWRITE_GIF_COLORTABLE),
				_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImwriteFlags"))),
			}
		}
	}

	opencv_type_enum! { crate::imgcodecs::ImwriteFlags }

	/// Imwrite GIF specific values for IMWRITE_GIF_QUALITY parameter key, if larger than 3, then its related to the size of the color table.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ImwriteGIFCompressionFlags {
		IMWRITE_GIF_FAST_NO_DITHER = 1,
		IMWRITE_GIF_FAST_FLOYD_DITHER = 2,
		IMWRITE_GIF_COLORTABLE_SIZE_8 = 3,
		IMWRITE_GIF_COLORTABLE_SIZE_16 = 4,
		IMWRITE_GIF_COLORTABLE_SIZE_32 = 5,
		IMWRITE_GIF_COLORTABLE_SIZE_64 = 6,
		IMWRITE_GIF_COLORTABLE_SIZE_128 = 7,
		IMWRITE_GIF_COLORTABLE_SIZE_256 = 8,
	}

	impl TryFrom<i32> for ImwriteGIFCompressionFlags {
		type Error = crate::Error;

		fn try_from(value: i32) -> Result<Self, Self::Error> {
			match value {
				1 => Ok(Self::IMWRITE_GIF_FAST_NO_DITHER),
				2 => Ok(Self::IMWRITE_GIF_FAST_FLOYD_DITHER),
				3 => Ok(Self::IMWRITE_GIF_COLORTABLE_SIZE_8),
				4 => Ok(Self::IMWRITE_GIF_COLORTABLE_SIZE_16),
				5 => Ok(Self::IMWRITE_GIF_COLORTABLE_SIZE_32),
				6 => Ok(Self::IMWRITE_GIF_COLORTABLE_SIZE_64),
				7 => Ok(Self::IMWRITE_GIF_COLORTABLE_SIZE_128),
				8 => Ok(Self::IMWRITE_GIF_COLORTABLE_SIZE_256),
				_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImwriteGIFCompressionFlags"))),
			}
		}
	}

	opencv_type_enum! { crate::imgcodecs::ImwriteGIFCompressionFlags }

	/// Imwrite HDR specific values for IMWRITE_HDR_COMPRESSION parameter key
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

	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ImwriteTiffCompressionFlags {
		/// dump mode
		IMWRITE_TIFF_COMPRESSION_NONE = 1,
		/// CCITT modified Huffman RLE
		IMWRITE_TIFF_COMPRESSION_CCITTRLE = 2,
		/// CCITT Group 3 fax encoding
		IMWRITE_TIFF_COMPRESSION_CCITTFAX3 = 3,
		// CCITT T.4 (TIFF 6 name)
		// Duplicate, use IMWRITE_TIFF_COMPRESSION_CCITTFAX3 instead
		// IMWRITE_TIFF_COMPRESSION_CCITT_T4 = 3,
		/// CCITT Group 4 fax encoding
		IMWRITE_TIFF_COMPRESSION_CCITTFAX4 = 4,
		// CCITT T.6 (TIFF 6 name)
		// Duplicate, use IMWRITE_TIFF_COMPRESSION_CCITTFAX4 instead
		// IMWRITE_TIFF_COMPRESSION_CCITT_T6 = 4,
		/// Lempel-Ziv  & Welch
		IMWRITE_TIFF_COMPRESSION_LZW = 5,
		/// !6.0 JPEG
		IMWRITE_TIFF_COMPRESSION_OJPEG = 6,
		/// %JPEG DCT compression
		IMWRITE_TIFF_COMPRESSION_JPEG = 7,
		/// !TIFF/FX T.85 JBIG compression
		IMWRITE_TIFF_COMPRESSION_T85 = 9,
		/// !TIFF/FX T.43 colour by layered JBIG compression
		IMWRITE_TIFF_COMPRESSION_T43 = 10,
		/// NeXT 2-bit RLE
		IMWRITE_TIFF_COMPRESSION_NEXT = 32766,
		/// [1] w/ word alignment
		IMWRITE_TIFF_COMPRESSION_CCITTRLEW = 32771,
		/// Macintosh RLE
		IMWRITE_TIFF_COMPRESSION_PACKBITS = 32773,
		/// ThunderScan RLE
		IMWRITE_TIFF_COMPRESSION_THUNDERSCAN = 32809,
		/// IT8 CT w/padding
		IMWRITE_TIFF_COMPRESSION_IT8CTPAD = 32895,
		/// IT8 Linework RLE
		IMWRITE_TIFF_COMPRESSION_IT8LW = 32896,
		/// IT8 Monochrome picture
		IMWRITE_TIFF_COMPRESSION_IT8MP = 32897,
		/// IT8 Binary line art
		IMWRITE_TIFF_COMPRESSION_IT8BL = 32898,
		/// Pixar companded 10bit LZW
		IMWRITE_TIFF_COMPRESSION_PIXARFILM = 32908,
		/// Pixar companded 11bit ZIP
		IMWRITE_TIFF_COMPRESSION_PIXARLOG = 32909,
		/// Deflate compression, legacy tag
		IMWRITE_TIFF_COMPRESSION_DEFLATE = 32946,
		/// Deflate compression, as recognized by Adobe
		IMWRITE_TIFF_COMPRESSION_ADOBE_DEFLATE = 8,
		/// Kodak DCS encoding
		IMWRITE_TIFF_COMPRESSION_DCS = 32947,
		/// ISO JBIG
		IMWRITE_TIFF_COMPRESSION_JBIG = 34661,
		/// SGI Log Luminance RLE
		IMWRITE_TIFF_COMPRESSION_SGILOG = 34676,
		/// SGI Log 24-bit packed
		IMWRITE_TIFF_COMPRESSION_SGILOG24 = 34677,
		/// Leadtools JPEG2000
		IMWRITE_TIFF_COMPRESSION_JP2000 = 34712,
		/// ESRI Lerc codec: <https://github.com/Esri/lerc>
		IMWRITE_TIFF_COMPRESSION_LERC = 34887,
		/// LZMA2
		IMWRITE_TIFF_COMPRESSION_LZMA = 34925,
		/// ZSTD: WARNING not registered in Adobe-maintained registry
		IMWRITE_TIFF_COMPRESSION_ZSTD = 50000,
		/// WEBP: WARNING not registered in Adobe-maintained registry
		IMWRITE_TIFF_COMPRESSION_WEBP = 50001,
		/// JPEGXL: WARNING not registered in Adobe-maintained registry
		IMWRITE_TIFF_COMPRESSION_JXL = 50002,
	}

	impl TryFrom<i32> for ImwriteTiffCompressionFlags {
		type Error = crate::Error;

		fn try_from(value: i32) -> Result<Self, Self::Error> {
			match value {
				1 => Ok(Self::IMWRITE_TIFF_COMPRESSION_NONE),
				2 => Ok(Self::IMWRITE_TIFF_COMPRESSION_CCITTRLE),
				3 => Ok(Self::IMWRITE_TIFF_COMPRESSION_CCITTFAX3),
				// Duplicate of IMWRITE_TIFF_COMPRESSION_CCITTFAX3
				// 3 => Ok(Self::IMWRITE_TIFF_COMPRESSION_CCITT_T4),
				4 => Ok(Self::IMWRITE_TIFF_COMPRESSION_CCITTFAX4),
				// Duplicate of IMWRITE_TIFF_COMPRESSION_CCITTFAX4
				// 4 => Ok(Self::IMWRITE_TIFF_COMPRESSION_CCITT_T6),
				5 => Ok(Self::IMWRITE_TIFF_COMPRESSION_LZW),
				6 => Ok(Self::IMWRITE_TIFF_COMPRESSION_OJPEG),
				7 => Ok(Self::IMWRITE_TIFF_COMPRESSION_JPEG),
				9 => Ok(Self::IMWRITE_TIFF_COMPRESSION_T85),
				10 => Ok(Self::IMWRITE_TIFF_COMPRESSION_T43),
				32766 => Ok(Self::IMWRITE_TIFF_COMPRESSION_NEXT),
				32771 => Ok(Self::IMWRITE_TIFF_COMPRESSION_CCITTRLEW),
				32773 => Ok(Self::IMWRITE_TIFF_COMPRESSION_PACKBITS),
				32809 => Ok(Self::IMWRITE_TIFF_COMPRESSION_THUNDERSCAN),
				32895 => Ok(Self::IMWRITE_TIFF_COMPRESSION_IT8CTPAD),
				32896 => Ok(Self::IMWRITE_TIFF_COMPRESSION_IT8LW),
				32897 => Ok(Self::IMWRITE_TIFF_COMPRESSION_IT8MP),
				32898 => Ok(Self::IMWRITE_TIFF_COMPRESSION_IT8BL),
				32908 => Ok(Self::IMWRITE_TIFF_COMPRESSION_PIXARFILM),
				32909 => Ok(Self::IMWRITE_TIFF_COMPRESSION_PIXARLOG),
				32946 => Ok(Self::IMWRITE_TIFF_COMPRESSION_DEFLATE),
				8 => Ok(Self::IMWRITE_TIFF_COMPRESSION_ADOBE_DEFLATE),
				32947 => Ok(Self::IMWRITE_TIFF_COMPRESSION_DCS),
				34661 => Ok(Self::IMWRITE_TIFF_COMPRESSION_JBIG),
				34676 => Ok(Self::IMWRITE_TIFF_COMPRESSION_SGILOG),
				34677 => Ok(Self::IMWRITE_TIFF_COMPRESSION_SGILOG24),
				34712 => Ok(Self::IMWRITE_TIFF_COMPRESSION_JP2000),
				34887 => Ok(Self::IMWRITE_TIFF_COMPRESSION_LERC),
				34925 => Ok(Self::IMWRITE_TIFF_COMPRESSION_LZMA),
				50000 => Ok(Self::IMWRITE_TIFF_COMPRESSION_ZSTD),
				50001 => Ok(Self::IMWRITE_TIFF_COMPRESSION_WEBP),
				50002 => Ok(Self::IMWRITE_TIFF_COMPRESSION_JXL),
				_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImwriteTiffCompressionFlags"))),
			}
		}
	}

	opencv_type_enum! { crate::imgcodecs::ImwriteTiffCompressionFlags }

	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ImwriteTiffPredictorFlags {
		/// no prediction scheme used
		IMWRITE_TIFF_PREDICTOR_NONE = 1,
		/// horizontal differencing
		IMWRITE_TIFF_PREDICTOR_HORIZONTAL = 2,
		/// floating point predictor
		IMWRITE_TIFF_PREDICTOR_FLOATINGPOINT = 3,
	}

	impl TryFrom<i32> for ImwriteTiffPredictorFlags {
		type Error = crate::Error;

		fn try_from(value: i32) -> Result<Self, Self::Error> {
			match value {
				1 => Ok(Self::IMWRITE_TIFF_PREDICTOR_NONE),
				2 => Ok(Self::IMWRITE_TIFF_PREDICTOR_HORIZONTAL),
				3 => Ok(Self::IMWRITE_TIFF_PREDICTOR_FLOATINGPOINT),
				_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::imgcodecs::ImwriteTiffPredictorFlags"))),
			}
		}
	}

	opencv_type_enum! { crate::imgcodecs::ImwriteTiffPredictorFlags }

	/// Checks if the specified image file can be decoded by OpenCV.
	///
	/// The function haveImageReader checks if OpenCV is capable of reading the specified file.
	/// This can be useful for verifying support for a given image format before attempting to load an image.
	///
	/// ## Parameters
	/// * filename: The name of the file to be checked.
	/// ## Returns
	/// true if an image reader for the specified file is available and the file can be opened, false otherwise.
	///
	///
	/// Note: The function checks the availability of image codecs that are either built into OpenCV or dynamically loaded.
	/// It does not check for the actual existence of the file but rather the ability to read the specified file type.
	/// If the file cannot be opened or the format is unsupported, the function will return false.
	/// ## See also
	/// cv::haveImageWriter, cv::imread, cv::imdecode
	#[inline]
	pub fn have_image_reader(filename: &str) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_haveImageReader_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Checks if the specified image file or specified file extension can be encoded by OpenCV.
	///
	/// The function haveImageWriter checks if OpenCV is capable of writing images with the specified file extension.
	/// This can be useful for verifying support for a given image format before attempting to save an image.
	///
	/// ## Parameters
	/// * filename: The name of the file or the file extension (e.g., ".jpg", ".png").
	/// It is recommended to provide the file extension rather than the full file name.
	/// ## Returns
	/// true if an image writer for the specified extension is available, false otherwise.
	///
	///
	/// Note: The function checks the availability of image codecs that are either built into OpenCV or dynamically loaded.
	/// It does not check for the actual existence of the file but rather the ability to write files of the given type.
	/// ## See also
	/// cv::haveImageReader, cv::imwrite, cv::imencode
	#[inline]
	pub fn have_image_writer(filename: &str) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_haveImageWriter_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the number of images inside the given file
	///
	/// The function imcount returns the number of pages in a multi-page image (e.g. TIFF), the number of frames in an animation (e.g. AVIF), and 1 otherwise.
	/// If the image cannot be decoded, 0 is returned.
	/// ## Parameters
	/// * filename: Name of file to be loaded.
	/// * flags: Flag that can take values of cv::ImreadModes, default with cv::IMREAD_ANYCOLOR.
	/// @todo when cv::IMREAD_LOAD_GDAL flag used the return value will be 0 or 1 because OpenCV's GDAL decoder doesn't support multi-page reading yet.
	///
	/// ## Note
	/// This alternative version of [imcount] function uses the following default values for its arguments:
	/// * flags: IMREAD_ANYCOLOR
	#[inline]
	pub fn imcount_def(filename: &str) -> Result<size_t> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imcount_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the number of images inside the given file
	///
	/// The function imcount returns the number of pages in a multi-page image (e.g. TIFF), the number of frames in an animation (e.g. AVIF), and 1 otherwise.
	/// If the image cannot be decoded, 0 is returned.
	/// ## Parameters
	/// * filename: Name of file to be loaded.
	/// * flags: Flag that can take values of cv::ImreadModes, default with cv::IMREAD_ANYCOLOR.
	/// @todo when cv::IMREAD_LOAD_GDAL flag used the return value will be 0 or 1 because OpenCV's GDAL decoder doesn't support multi-page reading yet.
	///
	/// ## C++ default parameters
	/// * flags: IMREAD_ANYCOLOR
	#[inline]
	pub fn imcount(filename: &str, flags: i32) -> Result<size_t> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imcount_const_StringR_int(filename.opencv_as_extern(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
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
	#[inline]
	pub fn imdecode(buf: &impl ToInputArray, flags: i32) -> Result<core::Mat> {
		input_array_arg!(buf);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imdecode_const__InputArrayR_int(buf.as_raw__InputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	/// * buf: Input array or vector of bytes.
	/// * flags: The same flags as in cv::imread, see cv::ImreadModes.
	/// * dst: The optional output placeholder for the decoded matrix. It can save the image
	/// reallocations when the function is called repeatedly for images of the same size. In case of decoder
	/// failure the function returns empty cv::Mat object, but does not release user-provided dst buffer.
	#[inline]
	pub fn imdecode_to(buf: &impl ToInputArray, flags: i32, dst: &mut impl core::MatTrait) -> Result<core::Mat> {
		input_array_arg!(buf);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imdecode_const__InputArrayR_int_MatX(buf.as_raw__InputArray(), flags, dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Reads a multi-page image from a buffer in memory.
	///
	/// The function imdecodemulti reads a multi-page image from the specified buffer in the memory. If the buffer is too short or
	/// contains invalid data, the function returns false.
	///
	/// See cv::imreadmulti for the list of supported formats and flags description.
	///
	///
	/// Note: In the case of color images, the decoded images will have the channels stored in **B G R** order.
	/// ## Parameters
	/// * buf: Input array or vector of bytes.
	/// * flags: The same flags as in cv::imread, see cv::ImreadModes.
	/// * mats: A vector of Mat objects holding each page, if more than one.
	/// * range: A continuous selection of pages.
	///
	/// ## Note
	/// This alternative version of [imdecodemulti] function uses the following default values for its arguments:
	/// * range: Range::all()
	#[inline]
	pub fn imdecodemulti_def(buf: &impl ToInputArray, flags: i32, mats: &mut core::Vector<core::Mat>) -> Result<bool> {
		input_array_arg!(buf);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imdecodemulti_const__InputArrayR_int_vectorLMatGR(buf.as_raw__InputArray(), flags, mats.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Reads a multi-page image from a buffer in memory.
	///
	/// The function imdecodemulti reads a multi-page image from the specified buffer in the memory. If the buffer is too short or
	/// contains invalid data, the function returns false.
	///
	/// See cv::imreadmulti for the list of supported formats and flags description.
	///
	///
	/// Note: In the case of color images, the decoded images will have the channels stored in **B G R** order.
	/// ## Parameters
	/// * buf: Input array or vector of bytes.
	/// * flags: The same flags as in cv::imread, see cv::ImreadModes.
	/// * mats: A vector of Mat objects holding each page, if more than one.
	/// * range: A continuous selection of pages.
	///
	/// ## C++ default parameters
	/// * range: Range::all()
	#[inline]
	pub fn imdecodemulti(buf: &impl ToInputArray, flags: i32, mats: &mut core::Vector<core::Mat>, range: &impl core::RangeTraitConst) -> Result<bool> {
		input_array_arg!(buf);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imdecodemulti_const__InputArrayR_int_vectorLMatGR_const_RangeR(buf.as_raw__InputArray(), flags, mats.as_raw_mut_VectorOfMat(), range.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	/// * img: Image to be compressed.
	/// * buf: Output buffer resized to fit the compressed image.
	/// * params: Format-specific parameters. See cv::imwrite and cv::ImwriteFlags.
	///
	/// ## Note
	/// This alternative version of [imencode] function uses the following default values for its arguments:
	/// * params: std::vector<int>()
	#[inline]
	pub fn imencode_def(ext: &str, img: &impl ToInputArray, buf: &mut core::Vector<u8>) -> Result<bool> {
		extern_container_arg!(ext);
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR(ext.opencv_as_extern(), img.as_raw__InputArray(), buf.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	/// * img: Image to be compressed.
	/// * buf: Output buffer resized to fit the compressed image.
	/// * params: Format-specific parameters. See cv::imwrite and cv::ImwriteFlags.
	///
	/// ## C++ default parameters
	/// * params: std::vector<int>()
	#[inline]
	pub fn imencode(ext: &str, img: &impl ToInputArray, buf: &mut core::Vector<u8>, params: &core::Vector<i32>) -> Result<bool> {
		extern_container_arg!(ext);
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(ext.opencv_as_extern(), img.as_raw__InputArray(), buf.as_raw_mut_VectorOfu8(), params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Encodes array of images into a memory buffer.
	///
	/// The function is analog to cv::imencode for in-memory multi-page image compression.
	/// See cv::imwrite for the list of supported formats and flags description.
	///
	/// ## Parameters
	/// * ext: File extension that defines the output format. Must include a leading period.
	/// * imgs: Vector of images to be written.
	/// * buf: Output buffer resized to fit the compressed data.
	/// * params: Format-specific parameters. See cv::imwrite and cv::ImwriteFlags.
	///
	/// ## Note
	/// This alternative version of [imencodemulti] function uses the following default values for its arguments:
	/// * params: std::vector<int>()
	#[inline]
	pub fn imencodemulti_def(ext: &str, imgs: &impl ToInputArray, buf: &mut core::Vector<u8>) -> Result<bool> {
		extern_container_arg!(ext);
		input_array_arg!(imgs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imencodemulti_const_StringR_const__InputArrayR_vectorLunsigned_charGR(ext.opencv_as_extern(), imgs.as_raw__InputArray(), buf.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Encodes array of images into a memory buffer.
	///
	/// The function is analog to cv::imencode for in-memory multi-page image compression.
	/// See cv::imwrite for the list of supported formats and flags description.
	///
	/// ## Parameters
	/// * ext: File extension that defines the output format. Must include a leading period.
	/// * imgs: Vector of images to be written.
	/// * buf: Output buffer resized to fit the compressed data.
	/// * params: Format-specific parameters. See cv::imwrite and cv::ImwriteFlags.
	///
	/// ## C++ default parameters
	/// * params: std::vector<int>()
	#[inline]
	pub fn imencodemulti(ext: &str, imgs: &impl ToInputArray, buf: &mut core::Vector<u8>, params: &core::Vector<i32>) -> Result<bool> {
		extern_container_arg!(ext);
		input_array_arg!(imgs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imencodemulti_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(ext.opencv_as_extern(), imgs.as_raw__InputArray(), buf.as_raw_mut_VectorOfu8(), params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads an image from a file.
	///
	/// @anchor imread
	///
	/// The `imread` function loads an image from the specified file and returns OpenCV matrix. If the image cannot be
	/// read (because of a missing file, improper permissions, or unsupported/invalid format), the function
	/// returns an empty matrix.
	///
	/// Currently, the following file formats are supported:
	///
	/// *   Windows bitmaps - \*.bmp, \*.dib (always supported)
	/// *   GIF files - \*.gif (always supported)
	/// *   JPEG files - \*.jpeg, \*.jpg, \*.jpe (see the *Note* section)
	/// *   JPEG 2000 files - \*.jp2 (see the *Note* section)
	/// *   Portable Network Graphics - \*.png (see the *Note* section)
	/// *   WebP - \*.webp (see the *Note* section)
	/// *   AVIF - \*.avif (see the *Note* section)
	/// *   Portable image format - \*.pbm, \*.pgm, \*.ppm, \*.pxm, \*.pnm (always supported)
	/// *   PFM files - \*.pfm (see the *Note* section)
	/// *   Sun rasters - \*.sr, \*.ras (always supported)
	/// *   TIFF files - \*.tiff, \*.tif (see the *Note* section)
	/// *   OpenEXR Image files - \*.exr (see the *Note* section)
	/// *   Radiance HDR - \*.hdr, \*.pic (always supported)
	/// *   Raster and Vector geospatial data supported by GDAL (see the *Note* section)
	///
	///
	/// Note:
	/// *   The function determines the type of an image by its content, not by the file extension.
	/// *   In the case of color images, the decoded images will have the channels stored in **B G R** order.
	/// *   When using IMREAD_GRAYSCALE, the codec's internal grayscale conversion will be used, if available.
	///    Results may differ from the output of cvtColor().
	/// *   On Microsoft Windows\* and Mac OS\*, the codecs shipped with OpenCV (libjpeg, libpng, libtiff,
	///    and libjasper) are used by default. So, OpenCV can always read JPEGs, PNGs, and TIFFs. On Mac OS,
	///    there is also an option to use native Mac OS image readers. However, beware that currently these
	///    native image loaders give images with different pixel values because of the color management embedded
	///    into Mac OS.
	/// *   On Linux\*, BSD flavors, and other Unix-like open-source operating systems, OpenCV looks for
	///    codecs supplied with the OS. Ensure the relevant packages are installed (including development
	///    files, such as "libjpeg-dev" in Debian\* and Ubuntu\*) to get codec support, or turn
	///    on the OPENCV_BUILD_3RDPARTY_LIBS flag in CMake.
	/// *   If the *WITH_GDAL* flag is set to true in CMake and [IMREAD_LOAD_GDAL] is used to load the image,
	///    the [GDAL](http://www.gdal.org) driver will be used to decode the image, supporting
	///    [Raster](http://www.gdal.org/formats_list.html) and [Vector](http://www.gdal.org/ogr_formats.html) formats.
	/// *   If EXIF information is embedded in the image file, the EXIF orientation will be taken into account,
	///    and thus the image will be rotated accordingly unless the flags [IMREAD_IGNORE_ORIENTATION]
	///    or [IMREAD_UNCHANGED] are passed.
	/// *   Use the IMREAD_UNCHANGED flag to preserve the floating-point values from PFM images.
	/// *   By default, the number of pixels must be less than 2^30. This limit can be changed by setting
	///    the environment variable `OPENCV_IO_MAX_IMAGE_PIXELS`. See [tutorial_env_reference].
	///
	/// ## Parameters
	/// * filename: Name of the file to be loaded.
	/// * flags: Flag that can take values of `cv::ImreadModes`.
	///
	/// ## Note
	/// This alternative version of [imread] function uses the following default values for its arguments:
	/// * flags: IMREAD_COLOR_BGR
	#[inline]
	pub fn imread_def(filename: &str) -> Result<core::Mat> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imread_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Loads an image from a file.
	///
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts and the return value.
	/// ## Parameters
	/// * filename: Name of file to be loaded.
	/// * dst: object in which the image will be loaded.
	/// * flags: Flag that can take values of cv::ImreadModes
	///
	/// Note:
	/// The image passing through the img parameter can be pre-allocated. The memory is reused if the shape and the type match with the load image.
	///
	/// ## Note
	/// This alternative version of [imread_1] function uses the following default values for its arguments:
	/// * flags: IMREAD_COLOR_BGR
	#[inline]
	pub fn imread_1_def(filename: &str, dst: &mut impl ToOutputArray) -> Result<()> {
		extern_container_arg!(filename);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imread_const_StringR_const__OutputArrayR(filename.opencv_as_extern(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads an image from a file.
	///
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts and the return value.
	/// ## Parameters
	/// * filename: Name of file to be loaded.
	/// * dst: object in which the image will be loaded.
	/// * flags: Flag that can take values of cv::ImreadModes
	///
	/// Note:
	/// The image passing through the img parameter can be pre-allocated. The memory is reused if the shape and the type match with the load image.
	///
	/// ## C++ default parameters
	/// * flags: IMREAD_COLOR_BGR
	#[inline]
	pub fn imread_1(filename: &str, dst: &mut impl ToOutputArray, flags: i32) -> Result<()> {
		extern_container_arg!(filename);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imread_const_StringR_const__OutputArrayR_int(filename.opencv_as_extern(), dst.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads an image from a file.
	///
	/// @anchor imread
	///
	/// The `imread` function loads an image from the specified file and returns OpenCV matrix. If the image cannot be
	/// read (because of a missing file, improper permissions, or unsupported/invalid format), the function
	/// returns an empty matrix.
	///
	/// Currently, the following file formats are supported:
	///
	/// *   Windows bitmaps - \*.bmp, \*.dib (always supported)
	/// *   GIF files - \*.gif (always supported)
	/// *   JPEG files - \*.jpeg, \*.jpg, \*.jpe (see the *Note* section)
	/// *   JPEG 2000 files - \*.jp2 (see the *Note* section)
	/// *   Portable Network Graphics - \*.png (see the *Note* section)
	/// *   WebP - \*.webp (see the *Note* section)
	/// *   AVIF - \*.avif (see the *Note* section)
	/// *   Portable image format - \*.pbm, \*.pgm, \*.ppm, \*.pxm, \*.pnm (always supported)
	/// *   PFM files - \*.pfm (see the *Note* section)
	/// *   Sun rasters - \*.sr, \*.ras (always supported)
	/// *   TIFF files - \*.tiff, \*.tif (see the *Note* section)
	/// *   OpenEXR Image files - \*.exr (see the *Note* section)
	/// *   Radiance HDR - \*.hdr, \*.pic (always supported)
	/// *   Raster and Vector geospatial data supported by GDAL (see the *Note* section)
	///
	///
	/// Note:
	/// *   The function determines the type of an image by its content, not by the file extension.
	/// *   In the case of color images, the decoded images will have the channels stored in **B G R** order.
	/// *   When using IMREAD_GRAYSCALE, the codec's internal grayscale conversion will be used, if available.
	///    Results may differ from the output of cvtColor().
	/// *   On Microsoft Windows\* and Mac OS\*, the codecs shipped with OpenCV (libjpeg, libpng, libtiff,
	///    and libjasper) are used by default. So, OpenCV can always read JPEGs, PNGs, and TIFFs. On Mac OS,
	///    there is also an option to use native Mac OS image readers. However, beware that currently these
	///    native image loaders give images with different pixel values because of the color management embedded
	///    into Mac OS.
	/// *   On Linux\*, BSD flavors, and other Unix-like open-source operating systems, OpenCV looks for
	///    codecs supplied with the OS. Ensure the relevant packages are installed (including development
	///    files, such as "libjpeg-dev" in Debian\* and Ubuntu\*) to get codec support, or turn
	///    on the OPENCV_BUILD_3RDPARTY_LIBS flag in CMake.
	/// *   If the *WITH_GDAL* flag is set to true in CMake and [IMREAD_LOAD_GDAL] is used to load the image,
	///    the [GDAL](http://www.gdal.org) driver will be used to decode the image, supporting
	///    [Raster](http://www.gdal.org/formats_list.html) and [Vector](http://www.gdal.org/ogr_formats.html) formats.
	/// *   If EXIF information is embedded in the image file, the EXIF orientation will be taken into account,
	///    and thus the image will be rotated accordingly unless the flags [IMREAD_IGNORE_ORIENTATION]
	///    or [IMREAD_UNCHANGED] are passed.
	/// *   Use the IMREAD_UNCHANGED flag to preserve the floating-point values from PFM images.
	/// *   By default, the number of pixels must be less than 2^30. This limit can be changed by setting
	///    the environment variable `OPENCV_IO_MAX_IMAGE_PIXELS`. See [tutorial_env_reference].
	///
	/// ## Parameters
	/// * filename: Name of the file to be loaded.
	/// * flags: Flag that can take values of `cv::ImreadModes`.
	///
	/// ## C++ default parameters
	/// * flags: IMREAD_COLOR_BGR
	#[inline]
	pub fn imread(filename: &str, flags: i32) -> Result<core::Mat> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imread_const_StringR_int(filename.opencv_as_extern(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Loads frames from an animated image file into an Animation structure.
	///
	/// The function imreadanimation loads frames from an animated image file (e.g., GIF, AVIF, APNG, WEBP) into the provided Animation struct.
	///
	/// ## Parameters
	/// * filename: A string containing the path to the file.
	/// * animation: A reference to an Animation structure where the loaded frames will be stored. It should be initialized before the function is called.
	/// * start: The index of the first frame to load. This is optional and defaults to 0.
	/// * count: The number of frames to load. This is optional and defaults to 32767.
	///
	/// ## Returns
	/// Returns true if the file was successfully loaded and frames were extracted; returns false otherwise.
	///
	/// ## Note
	/// This alternative version of [imreadanimation] function uses the following default values for its arguments:
	/// * start: 0
	/// * count: INT16_MAX
	#[inline]
	pub fn imreadanimation_def(filename: &str, animation: &mut impl crate::imgcodecs::AnimationTrait) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imreadanimation_const_StringR_AnimationR(filename.opencv_as_extern(), animation.as_raw_mut_Animation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads frames from an animated image file into an Animation structure.
	///
	/// The function imreadanimation loads frames from an animated image file (e.g., GIF, AVIF, APNG, WEBP) into the provided Animation struct.
	///
	/// ## Parameters
	/// * filename: A string containing the path to the file.
	/// * animation: A reference to an Animation structure where the loaded frames will be stored. It should be initialized before the function is called.
	/// * start: The index of the first frame to load. This is optional and defaults to 0.
	/// * count: The number of frames to load. This is optional and defaults to 32767.
	///
	/// ## Returns
	/// Returns true if the file was successfully loaded and frames were extracted; returns false otherwise.
	///
	/// ## C++ default parameters
	/// * start: 0
	/// * count: INT16_MAX
	#[inline]
	pub fn imreadanimation(filename: &str, animation: &mut impl crate::imgcodecs::AnimationTrait, start: i32, count: i32) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imreadanimation_const_StringR_AnimationR_int_int(filename.opencv_as_extern(), animation.as_raw_mut_Animation(), start, count, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	/// ## Note
	/// This alternative version of [imreadmulti] function uses the following default values for its arguments:
	/// * flags: IMREAD_ANYCOLOR
	#[inline]
	pub fn imreadmulti_def(filename: &str, mats: &mut core::Vector<core::Mat>) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imreadmulti_const_StringR_vectorLMatGR(filename.opencv_as_extern(), mats.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	#[inline]
	pub fn imreadmulti(filename: &str, mats: &mut core::Vector<core::Mat>, flags: i32) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imreadmulti_const_StringR_vectorLMatGR_int(filename.opencv_as_extern(), mats.as_raw_mut_VectorOfMat(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads images of a multi-page image from a file.
	///
	/// The function imreadmulti loads a specified range from a multi-page image from the specified file into a vector of Mat objects.
	/// ## Parameters
	/// * filename: Name of file to be loaded.
	/// * mats: A vector of Mat objects holding each page.
	/// * start: Start index of the image to load
	/// * count: Count number of images to load
	/// * flags: Flag that can take values of cv::ImreadModes, default with cv::IMREAD_ANYCOLOR.
	/// ## See also
	/// cv::imread
	///
	/// ## Note
	/// This alternative version of [imreadmulti_range] function uses the following default values for its arguments:
	/// * flags: IMREAD_ANYCOLOR
	#[inline]
	pub fn imreadmulti_range_def(filename: &str, mats: &mut core::Vector<core::Mat>, start: i32, count: i32) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imreadmulti_const_StringR_vectorLMatGR_int_int(filename.opencv_as_extern(), mats.as_raw_mut_VectorOfMat(), start, count, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads images of a multi-page image from a file.
	///
	/// The function imreadmulti loads a specified range from a multi-page image from the specified file into a vector of Mat objects.
	/// ## Parameters
	/// * filename: Name of file to be loaded.
	/// * mats: A vector of Mat objects holding each page.
	/// * start: Start index of the image to load
	/// * count: Count number of images to load
	/// * flags: Flag that can take values of cv::ImreadModes, default with cv::IMREAD_ANYCOLOR.
	/// ## See also
	/// cv::imread
	///
	/// ## C++ default parameters
	/// * flags: IMREAD_ANYCOLOR
	#[inline]
	pub fn imreadmulti_range(filename: &str, mats: &mut core::Vector<core::Mat>, start: i32, count: i32, flags: i32) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imreadmulti_const_StringR_vectorLMatGR_int_int_int(filename.opencv_as_extern(), mats.as_raw_mut_VectorOfMat(), start, count, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	/// - With JPEG XL encoder, 8-bit unsigned (CV_8U), 16-bit unsigned (CV_16U) and 32-bit float(CV_32F) images can be saved.
	///   - JPEG XL images with an alpha channel can be saved using this function.
	///    To do this, create 8-bit (or 16-bit, 32-bit float) 4-channel image BGRA, where the alpha channel goes last.
	///    Fully transparent pixels should have alpha set to 0, fully opaque pixels should have alpha set to 255/65535/1.0.
	/// - With PAM encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
	/// - With PNG encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
	///   - PNG images with an alpha channel can be saved using this function. To do this, create
	///    8-bit (or 16-bit) 4-channel image BGRA, where the alpha channel goes last. Fully transparent pixels
	///    should have alpha set to 0, fully opaque pixels should have alpha set to 255/65535 (see the code sample below).
	/// - With PGM/PPM encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
	/// - With TIFF encoder, 8-bit unsigned (CV_8U), 8-bit signed (CV_8S),
	///                      16-bit unsigned (CV_16U), 16-bit signed (CV_16S),
	///                      32-bit signed (CV_32S),
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
	#[inline]
	pub fn imwrite_def(filename: &str, img: &impl ToInputArray) -> Result<bool> {
		extern_container_arg!(filename);
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imwrite_const_StringR_const__InputArrayR(filename.opencv_as_extern(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	/// - With JPEG XL encoder, 8-bit unsigned (CV_8U), 16-bit unsigned (CV_16U) and 32-bit float(CV_32F) images can be saved.
	///   - JPEG XL images with an alpha channel can be saved using this function.
	///    To do this, create 8-bit (or 16-bit, 32-bit float) 4-channel image BGRA, where the alpha channel goes last.
	///    Fully transparent pixels should have alpha set to 0, fully opaque pixels should have alpha set to 255/65535/1.0.
	/// - With PAM encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
	/// - With PNG encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
	///   - PNG images with an alpha channel can be saved using this function. To do this, create
	///    8-bit (or 16-bit) 4-channel image BGRA, where the alpha channel goes last. Fully transparent pixels
	///    should have alpha set to 0, fully opaque pixels should have alpha set to 255/65535 (see the code sample below).
	/// - With PGM/PPM encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
	/// - With TIFF encoder, 8-bit unsigned (CV_8U), 8-bit signed (CV_8S),
	///                      16-bit unsigned (CV_16U), 16-bit signed (CV_16S),
	///                      32-bit signed (CV_32S),
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
	#[inline]
	pub fn imwrite(filename: &str, img: &impl ToInputArray, params: &core::Vector<i32>) -> Result<bool> {
		extern_container_arg!(filename);
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imwrite_const_StringR_const__InputArrayR_const_vectorLintGR(filename.opencv_as_extern(), img.as_raw__InputArray(), params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Saves an Animation to a specified file.
	///
	/// The function imwriteanimation saves the provided Animation data to the specified file in an animated format.
	/// Supported formats depend on the implementation and may include formats like GIF, AVIF, APNG, or WEBP.
	///
	/// ## Parameters
	/// * filename: The name of the file where the animation will be saved. The file extension determines the format.
	/// * animation: A constant reference to an Animation struct containing the frames and metadata to be saved.
	/// * params: Optional format-specific parameters encoded as pairs (paramId_1, paramValue_1, paramId_2, paramValue_2, ...).
	/// These parameters are used to specify additional options for the encoding process. Refer to `cv::ImwriteFlags` for details on possible parameters.
	///
	/// ## Returns
	/// Returns true if the animation was successfully saved; returns false otherwise.
	///
	/// ## Note
	/// This alternative version of [imwriteanimation] function uses the following default values for its arguments:
	/// * params: std::vector<int>()
	#[inline]
	pub fn imwriteanimation_def(filename: &str, animation: &impl crate::imgcodecs::AnimationTraitConst) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imwriteanimation_const_StringR_const_AnimationR(filename.opencv_as_extern(), animation.as_raw_Animation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Saves an Animation to a specified file.
	///
	/// The function imwriteanimation saves the provided Animation data to the specified file in an animated format.
	/// Supported formats depend on the implementation and may include formats like GIF, AVIF, APNG, or WEBP.
	///
	/// ## Parameters
	/// * filename: The name of the file where the animation will be saved. The file extension determines the format.
	/// * animation: A constant reference to an Animation struct containing the frames and metadata to be saved.
	/// * params: Optional format-specific parameters encoded as pairs (paramId_1, paramValue_1, paramId_2, paramValue_2, ...).
	/// These parameters are used to specify additional options for the encoding process. Refer to `cv::ImwriteFlags` for details on possible parameters.
	///
	/// ## Returns
	/// Returns true if the animation was successfully saved; returns false otherwise.
	///
	/// ## C++ default parameters
	/// * params: std::vector<int>()
	#[inline]
	pub fn imwriteanimation(filename: &str, animation: &impl crate::imgcodecs::AnimationTraitConst, params: &core::Vector<i32>) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imwriteanimation_const_StringR_const_AnimationR_const_vectorLintGR(filename.opencv_as_extern(), animation.as_raw_Animation(), params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// multi-image overload for bindings
	///
	/// ## Note
	/// This alternative version of [imwritemulti] function uses the following default values for its arguments:
	/// * params: std::vector<int>()
	#[inline]
	pub fn imwritemulti_def(filename: &str, img: &impl ToInputArray) -> Result<bool> {
		extern_container_arg!(filename);
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imwritemulti_const_StringR_const__InputArrayR(filename.opencv_as_extern(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// multi-image overload for bindings
	///
	/// ## C++ default parameters
	/// * params: std::vector<int>()
	#[inline]
	pub fn imwritemulti(filename: &str, img: &impl ToInputArray, params: &core::Vector<i32>) -> Result<bool> {
		extern_container_arg!(filename);
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imwritemulti_const_StringR_const__InputArrayR_const_vectorLintGR(filename.opencv_as_extern(), img.as_raw__InputArray(), params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Represents an animation with multiple frames.
	/// The `Animation` struct is designed to store and manage data for animated sequences such as those from animated formats (e.g., GIF, AVIF, APNG, WebP).
	/// It provides support for looping, background color settings, frame timing, and frame storage.
	pub struct Animation {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Animation }

	impl Drop for Animation {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_Animation_delete(self.as_raw_mut_Animation()) };
		}
	}

	unsafe impl Send for Animation {}

	impl Animation {
		/// Constructs an Animation object with optional loop count and background color.
		///
		/// ## Parameters
		/// * loopCount: An integer representing the number of times the animation should loop:
		/// - `0` (default) indicates infinite looping, meaning the animation will replay continuously.
		/// - Positive values denote finite repeat counts, allowing the animation to play a limited number of times.
		/// - If a negative value or a value beyond the maximum of `0xffff` (65535) is provided, it is reset to `0`
		/// (infinite looping) to maintain valid bounds.
		///
		/// * bgColor: A `Scalar` object representing the background color in BGRA format:
		/// - Defaults to `Scalar()`, indicating an empty color (usually transparent if supported).
		/// - This background color provides a solid fill behind frames that have transparency, ensuring a consistent display appearance.
		///
		/// ## C++ default parameters
		/// * loop_count: 0
		/// * bg_color: Scalar()
		#[inline]
		pub fn new(loop_count: i32, bg_color: core::Scalar) -> Result<crate::imgcodecs::Animation> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Animation_Animation_int_Scalar(loop_count, &bg_color, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::imgcodecs::Animation::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Constructs an Animation object with optional loop count and background color.
		///
		/// ## Parameters
		/// * loopCount: An integer representing the number of times the animation should loop:
		/// - `0` (default) indicates infinite looping, meaning the animation will replay continuously.
		/// - Positive values denote finite repeat counts, allowing the animation to play a limited number of times.
		/// - If a negative value or a value beyond the maximum of `0xffff` (65535) is provided, it is reset to `0`
		/// (infinite looping) to maintain valid bounds.
		///
		/// * bgColor: A `Scalar` object representing the background color in BGRA format:
		/// - Defaults to `Scalar()`, indicating an empty color (usually transparent if supported).
		/// - This background color provides a solid fill behind frames that have transparency, ensuring a consistent display appearance.
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * loop_count: 0
		/// * bg_color: Scalar()
		#[inline]
		pub fn new_def() -> Result<crate::imgcodecs::Animation> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Animation_Animation(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::imgcodecs::Animation::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::imgcodecs::Animation]
	pub trait AnimationTraitConst {
		fn as_raw_Animation(&self) -> *const c_void;

		/// Number of times the animation should loop. 0 means infinite looping.
		#[inline]
		fn loop_count(&self) -> i32 {
			let ret = unsafe { sys::cv_Animation_propLoop_count_const(self.as_raw_Animation()) };
			ret
		}

		/// Background color of the animation in BGRA format.
		#[inline]
		fn bgcolor(&self) -> core::Scalar {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Animation_propBgcolor_const(self.as_raw_Animation(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// Duration for each frame in milliseconds.
		#[inline]
		fn durations(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_Animation_propDurations_const(self.as_raw_Animation()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}

		/// Vector of frames, where each Mat represents a single frame.
		#[inline]
		fn frames(&self) -> core::Vector<core::Mat> {
			let ret = unsafe { sys::cv_Animation_propFrames_const(self.as_raw_Animation()) };
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			ret
		}

	}

	/// Mutable methods for [crate::imgcodecs::Animation]
	pub trait AnimationTrait: crate::imgcodecs::AnimationTraitConst {
		fn as_raw_mut_Animation(&mut self) -> *mut c_void;

		/// Number of times the animation should loop. 0 means infinite looping.
		#[inline]
		fn set_loop_count(&mut self, val: i32) {
			let ret = unsafe { sys::cv_Animation_propLoop_count_const_int(self.as_raw_mut_Animation(), val) };
			ret
		}

		/// Background color of the animation in BGRA format.
		#[inline]
		fn set_bgcolor(&mut self, val: core::Scalar) {
			let ret = unsafe { sys::cv_Animation_propBgcolor_const_Scalar(self.as_raw_mut_Animation(), &val) };
			ret
		}

		/// Duration for each frame in milliseconds.
		#[inline]
		fn set_durations(&mut self, val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_Animation_propDurations_const_vectorLintG(self.as_raw_mut_Animation(), val.as_raw_VectorOfi32()) };
			ret
		}

		/// Vector of frames, where each Mat represents a single frame.
		#[inline]
		fn set_frames(&mut self, val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_Animation_propFrames_const_vectorLMatG(self.as_raw_mut_Animation(), val.as_raw_VectorOfMat()) };
			ret
		}

	}

	impl Clone for Animation {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_Animation_implicitClone_const(self.as_raw_Animation())) }
		}
	}

	impl std::fmt::Debug for Animation {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Animation")
				.field("loop_count", &crate::imgcodecs::AnimationTraitConst::loop_count(self))
				.field("bgcolor", &crate::imgcodecs::AnimationTraitConst::bgcolor(self))
				.field("durations", &crate::imgcodecs::AnimationTraitConst::durations(self))
				.field("frames", &crate::imgcodecs::AnimationTraitConst::frames(self))
				.finish()
		}
	}

	impl crate::imgcodecs::AnimationTraitConst for Animation {
		#[inline] fn as_raw_Animation(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::imgcodecs::AnimationTrait for Animation {
		#[inline] fn as_raw_mut_Animation(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Animation, crate::imgcodecs::AnimationTraitConst, as_raw_Animation, crate::imgcodecs::AnimationTrait, as_raw_mut_Animation }

	/// To read multi-page images on demand
	///
	/// The ImageCollection class provides iterator API to read multi-page images on demand. Create iterator
	/// to the collection of the images and iterate over the collection. Decode the necessary page with operator*.
	///
	/// The performance of page decoding is O(1) if collection is increment sequentially. If the user wants to access random page,
	/// then the time Complexity is O(n) because the collection has to be reinitialized every time in order to go to the correct page.
	/// However, the intermediate pages are not decoded during the process, so typically it's quite fast.
	/// This is required because multi-page codecs does not support going backwards.
	/// After decoding the one page, it is stored inside the collection cache. Hence, trying to get Mat object from already decoded page is O(1).
	/// If you need memory, you can use .releaseCache() method to release cached index.
	/// The space complexity is O(n) if all pages are decoded into memory. The user is able to decode and release images on demand.
	pub struct ImageCollection {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { ImageCollection }

	impl Drop for ImageCollection {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_ImageCollection_delete(self.as_raw_mut_ImageCollection()) };
		}
	}

	unsafe impl Send for ImageCollection {}

	impl ImageCollection {
		#[inline]
		pub fn default() -> Result<crate::imgcodecs::ImageCollection> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_ImageCollection(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::imgcodecs::ImageCollection::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn new(filename: &str, flags: i32) -> Result<crate::imgcodecs::ImageCollection> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_ImageCollection_const_StringR_int(filename.opencv_as_extern(), flags, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::imgcodecs::ImageCollection::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::imgcodecs::ImageCollection]
	pub trait ImageCollectionTraitConst {
		fn as_raw_ImageCollection(&self) -> *const c_void;

		#[inline]
		fn size(&self) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_size_const(self.as_raw_ImageCollection(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::imgcodecs::ImageCollection]
	pub trait ImageCollectionTrait: crate::imgcodecs::ImageCollectionTraitConst {
		fn as_raw_mut_ImageCollection(&mut self) -> *mut c_void;

		#[inline]
		fn init(&mut self, img: &str, flags: i32) -> Result<()> {
			extern_container_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_init_const_StringR_int(self.as_raw_mut_ImageCollection(), img.opencv_as_extern(), flags, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn at(&mut self, index: i32) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_at_int(self.as_raw_mut_ImageCollection(), index, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn get_mut(&mut self, index: i32) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_operator___int(self.as_raw_mut_ImageCollection(), index, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn release_cache(&mut self, index: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_releaseCache_int(self.as_raw_mut_ImageCollection(), index, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn begin(&mut self) -> Result<crate::imgcodecs::ImageCollection_iterator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_begin(self.as_raw_mut_ImageCollection(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::imgcodecs::ImageCollection_iterator::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn end(&mut self) -> Result<crate::imgcodecs::ImageCollection_iterator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_end(self.as_raw_mut_ImageCollection(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::imgcodecs::ImageCollection_iterator::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl std::fmt::Debug for ImageCollection {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ImageCollection")
				.finish()
		}
	}

	impl crate::imgcodecs::ImageCollectionTraitConst for ImageCollection {
		#[inline] fn as_raw_ImageCollection(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::imgcodecs::ImageCollectionTrait for ImageCollection {
		#[inline] fn as_raw_mut_ImageCollection(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { ImageCollection, crate::imgcodecs::ImageCollectionTraitConst, as_raw_ImageCollection, crate::imgcodecs::ImageCollectionTrait, as_raw_mut_ImageCollection }

	pub struct ImageCollection_iterator {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { ImageCollection_iterator }

	impl Drop for ImageCollection_iterator {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_ImageCollection_iterator_delete(self.as_raw_mut_ImageCollection_iterator()) };
		}
	}

	unsafe impl Send for ImageCollection_iterator {}

	impl ImageCollection_iterator {
		#[inline]
		pub fn new(col: &mut impl crate::imgcodecs::ImageCollectionTrait) -> Result<crate::imgcodecs::ImageCollection_iterator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_iterator_iterator_ImageCollectionX(col.as_raw_mut_ImageCollection(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::imgcodecs::ImageCollection_iterator::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn new_1(col: &mut impl crate::imgcodecs::ImageCollectionTrait, end: i32) -> Result<crate::imgcodecs::ImageCollection_iterator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_iterator_iterator_ImageCollectionX_int(col.as_raw_mut_ImageCollection(), end, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::imgcodecs::ImageCollection_iterator::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::imgcodecs::ImageCollection_iterator]
	pub trait ImageCollection_iteratorTraitConst {
		fn as_raw_ImageCollection_iterator(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::imgcodecs::ImageCollection_iterator]
	pub trait ImageCollection_iteratorTrait: crate::imgcodecs::ImageCollection_iteratorTraitConst {
		fn as_raw_mut_ImageCollection_iterator(&mut self) -> *mut c_void;

		#[inline]
		fn try_deref_mut(&mut self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_iterator_operatorX(self.as_raw_mut_ImageCollection_iterator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn incr(&mut self) -> Result<crate::imgcodecs::ImageCollection_iterator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ImageCollection_iterator_operatorAA(self.as_raw_mut_ImageCollection_iterator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::imgcodecs::ImageCollection_iterator::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl std::fmt::Debug for ImageCollection_iterator {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ImageCollection_iterator")
				.finish()
		}
	}

	impl crate::imgcodecs::ImageCollection_iteratorTraitConst for ImageCollection_iterator {
		#[inline] fn as_raw_ImageCollection_iterator(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::imgcodecs::ImageCollection_iteratorTrait for ImageCollection_iterator {
		#[inline] fn as_raw_mut_ImageCollection_iterator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { ImageCollection_iterator, crate::imgcodecs::ImageCollection_iteratorTraitConst, as_raw_ImageCollection_iterator, crate::imgcodecs::ImageCollection_iteratorTrait, as_raw_mut_ImageCollection_iterator }

}

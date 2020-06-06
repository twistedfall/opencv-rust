#![allow(unused_parens)]
//! # Image file reading and writing
//!   # C API
//!   # iOS glue
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use {  };
}

/// If set, the image is read in any possible color format.
pub const IMREAD_ANYCOLOR: i32 = 4;
/// If set, return 16-bit/32-bit image when the input has the corresponding depth, otherwise convert it to 8-bit.
pub const IMREAD_ANYDEPTH: i32 = 2;
/// If set, always convert image to the 3 channel BGR color image.
pub const IMREAD_COLOR: i32 = 1;
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
/// override EXR storage type (FLOAT (FP32) is default)
pub const IMWRITE_EXR_TYPE: i32 = 48;
/// store as FP32 (default)
pub const IMWRITE_EXR_TYPE_FLOAT: i32 = 2;
/// store as HALF (FP16)
pub const IMWRITE_EXR_TYPE_HALF: i32 = 1;
/// Separate chroma quality level, 0 - 100, default is 0 - don't use.
pub const IMWRITE_JPEG_CHROMA_QUALITY: i32 = 6;
/// Separate luma quality level, 0 - 100, default is 0 - don't use.
pub const IMWRITE_JPEG_LUMA_QUALITY: i32 = 5;
/// Enable JPEG features, 0 or 1, default is False.
pub const IMWRITE_JPEG_OPTIMIZE: i32 = 3;
/// Enable JPEG features, 0 or 1, default is False.
pub const IMWRITE_JPEG_PROGRESSIVE: i32 = 2;
/// For JPEG, it can be a quality from 0 to 100 (the higher is the better). Default value is 95.
pub const IMWRITE_JPEG_QUALITY: i32 = 1;
/// JPEG restart interval, 0 - 65535, default is 0 - no restart.
pub const IMWRITE_JPEG_RST_INTERVAL: i32 = 4;
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
/// For TIFF, use to specify the image compression scheme. See libtiff for integer constants corresponding to compression formats. Note, for images whose depth is CV_32F, only libtiff's SGILOG compression scheme is used. For other supported depths, the compression scheme can be specified by this flag; LZW compression is the default.
pub const IMWRITE_TIFF_COMPRESSION: i32 = 259;
/// For TIFF, use to specify which DPI resolution unit to set; see libtiff documentation for valid values.
pub const IMWRITE_TIFF_RESUNIT: i32 = 256;
/// For TIFF, use to specify the X direction DPI.
pub const IMWRITE_TIFF_XDPI: i32 = 257;
/// For TIFF, use to specify the Y direction DPI.
pub const IMWRITE_TIFF_YDPI: i32 = 258;
/// For WEBP, it can be a quality from 1 to 100 (the higher is the better). By default (without any parameter) and for quality above 100 the lossless compression is used.
pub const IMWRITE_WEBP_QUALITY: i32 = 64;
/// Imread flags
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ImreadModes {
	/// If set, return the loaded image as is (with alpha channel, otherwise it gets cropped). Ignore EXIF orientation.
	IMREAD_UNCHANGED = -1 as isize,
	/// If set, always convert image to the single channel grayscale image (codec internal conversion).
	IMREAD_GRAYSCALE = 0 as isize,
	/// If set, always convert image to the 3 channel BGR color image.
	IMREAD_COLOR = 1 as isize,
	/// If set, return 16-bit/32-bit image when the input has the corresponding depth, otherwise convert it to 8-bit.
	IMREAD_ANYDEPTH = 2 as isize,
	/// If set, the image is read in any possible color format.
	IMREAD_ANYCOLOR = 4 as isize,
	/// If set, use the gdal driver for loading the image.
	IMREAD_LOAD_GDAL = 8 as isize,
	/// If set, always convert image to the single channel grayscale image and the image size reduced 1/2.
	IMREAD_REDUCED_GRAYSCALE_2 = 16 as isize,
	/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/2.
	IMREAD_REDUCED_COLOR_2 = 17 as isize,
	/// If set, always convert image to the single channel grayscale image and the image size reduced 1/4.
	IMREAD_REDUCED_GRAYSCALE_4 = 32 as isize,
	/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/4.
	IMREAD_REDUCED_COLOR_4 = 33 as isize,
	/// If set, always convert image to the single channel grayscale image and the image size reduced 1/8.
	IMREAD_REDUCED_GRAYSCALE_8 = 64 as isize,
	/// If set, always convert image to the 3 channel BGR color image and the image size reduced 1/8.
	IMREAD_REDUCED_COLOR_8 = 65 as isize,
	/// If set, do not rotate the image according to EXIF's orientation flag.
	IMREAD_IGNORE_ORIENTATION = 128 as isize,
}

opencv_type_enum! { crate::imgcodecs::ImreadModes }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ImwriteEXRTypeFlags {
	/// store as HALF (FP16)
	IMWRITE_EXR_TYPE_HALF = 1 as isize,
	/// store as FP32 (default)
	IMWRITE_EXR_TYPE_FLOAT = 2 as isize,
}

opencv_type_enum! { crate::imgcodecs::ImwriteEXRTypeFlags }

/// Imwrite flags
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ImwriteFlags {
	/// For JPEG, it can be a quality from 0 to 100 (the higher is the better). Default value is 95.
	IMWRITE_JPEG_QUALITY = 1 as isize,
	/// Enable JPEG features, 0 or 1, default is False.
	IMWRITE_JPEG_PROGRESSIVE = 2 as isize,
	/// Enable JPEG features, 0 or 1, default is False.
	IMWRITE_JPEG_OPTIMIZE = 3 as isize,
	/// JPEG restart interval, 0 - 65535, default is 0 - no restart.
	IMWRITE_JPEG_RST_INTERVAL = 4 as isize,
	/// Separate luma quality level, 0 - 100, default is 0 - don't use.
	IMWRITE_JPEG_LUMA_QUALITY = 5 as isize,
	/// Separate chroma quality level, 0 - 100, default is 0 - don't use.
	IMWRITE_JPEG_CHROMA_QUALITY = 6 as isize,
	/// For PNG, it can be the compression level from 0 to 9. A higher value means a smaller size and longer compression time. If specified, strategy is changed to IMWRITE_PNG_STRATEGY_DEFAULT (Z_DEFAULT_STRATEGY). Default value is 1 (best speed setting).
	IMWRITE_PNG_COMPRESSION = 16 as isize,
	/// One of cv::ImwritePNGFlags, default is IMWRITE_PNG_STRATEGY_RLE.
	IMWRITE_PNG_STRATEGY = 17 as isize,
	/// Binary level PNG, 0 or 1, default is 0.
	IMWRITE_PNG_BILEVEL = 18 as isize,
	/// For PPM, PGM, or PBM, it can be a binary format flag, 0 or 1. Default value is 1.
	IMWRITE_PXM_BINARY = 32 as isize,
	/// override EXR storage type (FLOAT (FP32) is default)
	IMWRITE_EXR_TYPE = 48 as isize,
	/// For WEBP, it can be a quality from 1 to 100 (the higher is the better). By default (without any parameter) and for quality above 100 the lossless compression is used.
	IMWRITE_WEBP_QUALITY = 64 as isize,
	/// For PAM, sets the TUPLETYPE field to the corresponding string value that is defined for the format
	IMWRITE_PAM_TUPLETYPE = 128 as isize,
	/// For TIFF, use to specify which DPI resolution unit to set; see libtiff documentation for valid values.
	IMWRITE_TIFF_RESUNIT = 256 as isize,
	/// For TIFF, use to specify the X direction DPI.
	IMWRITE_TIFF_XDPI = 257 as isize,
	/// For TIFF, use to specify the Y direction DPI.
	IMWRITE_TIFF_YDPI = 258 as isize,
	/// For TIFF, use to specify the image compression scheme. See libtiff for integer constants corresponding to compression formats. Note, for images whose depth is CV_32F, only libtiff's SGILOG compression scheme is used. For other supported depths, the compression scheme can be specified by this flag; LZW compression is the default.
	IMWRITE_TIFF_COMPRESSION = 259 as isize,
}

opencv_type_enum! { crate::imgcodecs::ImwriteFlags }

/// Imwrite PAM specific tupletype flags used to define the 'TUPETYPE' field of a PAM file.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ImwritePAMFlags {
	IMWRITE_PAM_FORMAT_NULL = 0 as isize,
	IMWRITE_PAM_FORMAT_BLACKANDWHITE = 1 as isize,
	IMWRITE_PAM_FORMAT_GRAYSCALE = 2 as isize,
	IMWRITE_PAM_FORMAT_GRAYSCALE_ALPHA = 3 as isize,
	IMWRITE_PAM_FORMAT_RGB = 4 as isize,
	IMWRITE_PAM_FORMAT_RGB_ALPHA = 5 as isize,
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ImwritePNGFlags {
	/// Use this value for normal data.
	IMWRITE_PNG_STRATEGY_DEFAULT = 0 as isize,
	/// Use this value for data produced by a filter (or predictor).Filtered data consists mostly of small values with a somewhat random distribution. In this case, the compression algorithm is tuned to compress them better.
	IMWRITE_PNG_STRATEGY_FILTERED = 1 as isize,
	/// Use this value to force Huffman encoding only (no string match).
	IMWRITE_PNG_STRATEGY_HUFFMAN_ONLY = 2 as isize,
	/// Use this value to limit match distances to one (run-length encoding).
	IMWRITE_PNG_STRATEGY_RLE = 3 as isize,
	/// Using this value prevents the use of dynamic Huffman codes, allowing for a simpler decoder for special applications.
	IMWRITE_PNG_STRATEGY_FIXED = 4 as isize,
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
pub fn imdecode(buf: &dyn core::ToInputArray, flags: i32) -> Result<core::Mat> {
	input_array_arg!(buf);
	unsafe { sys::cv_imdecode_const__InputArrayX_int(buf.as_raw__InputArray(), flags) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
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
pub fn imdecode_to(buf: &dyn core::ToInputArray, flags: i32, dst: &mut core::Mat) -> Result<core::Mat> {
	input_array_arg!(buf);
	unsafe { sys::cv_imdecode_const__InputArrayX_int_MatX(buf.as_raw__InputArray(), flags, dst.as_raw_mut_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

/// Encodes an image into a memory buffer.
/// 
/// The function imencode compresses the image and stores it in the memory buffer that is resized to fit the
/// result. See cv::imwrite for the list of supported formats and flags description.
/// 
/// ## Parameters
/// * ext: File extension that defines the output format.
/// * img: Image to be written.
/// * buf: Output buffer resized to fit the compressed image.
/// * params: Format-specific parameters. See cv::imwrite and cv::ImwriteFlags.
/// 
/// ## C++ default parameters
/// * params: std::vector<int>()
pub fn imencode(ext: &str, img: &dyn core::ToInputArray, buf: &mut core::Vector::<u8>, params: &core::Vector::<i32>) -> Result<bool> {
	extern_container_arg!(ext);
	input_array_arg!(img);
	unsafe { sys::cv_imencode_const_StringX_const__InputArrayX_vector_unsigned_char_X_const_vector_int_X(ext.opencv_to_extern(), img.as_raw__InputArray(), buf.as_raw_mut_VectorOfu8(), params.as_raw_VectorOfi32()) }.into_result()
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
/// *   In the case you set *WITH_GDAL* flag to true in CMake and @ref IMREAD_LOAD_GDAL to load the image,
///    then the [GDAL](http://www.gdal.org) driver will be used in order to decode the image, supporting
///    the following formats: [Raster](http://www.gdal.org/formats_list.html),
///    [Vector](http://www.gdal.org/ogr_formats.html).
/// *   If EXIF information is embedded in the image file, the EXIF orientation will be taken into account
///    and thus the image will be rotated accordingly except if the flags @ref IMREAD_IGNORE_ORIENTATION
///    or @ref IMREAD_UNCHANGED are passed.
/// *   By default number of pixels must be less than 2^30. Limit can be set using system
///    variable OPENCV_IO_MAX_IMAGE_PIXELS
/// 
/// ## Parameters
/// * filename: Name of file to be loaded.
/// * flags: Flag that can take values of cv::ImreadModes
/// 
/// ## C++ default parameters
/// * flags: IMREAD_COLOR
pub fn imread(filename: &str, flags: i32) -> Result<core::Mat> {
	extern_container_arg!(filename);
	unsafe { sys::cv_imread_const_StringX_int(filename.opencv_to_extern(), flags) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

/// Loads a multi-page image from a file.
/// 
/// The function imreadmulti loads a multi-page image from the specified file into a vector of Mat objects.
/// ## Parameters
/// * filename: Name of file to be loaded.
/// * flags: Flag that can take values of cv::ImreadModes, default with cv::IMREAD_ANYCOLOR.
/// * mats: A vector of Mat objects holding each page, if more than one.
/// ## See also
/// cv::imread
/// 
/// ## C++ default parameters
/// * flags: IMREAD_ANYCOLOR
pub fn imreadmulti(filename: &str, mats: &mut core::Vector::<core::Mat>, flags: i32) -> Result<bool> {
	extern_container_arg!(filename);
	unsafe { sys::cv_imreadmulti_const_StringX_vector_Mat_X_int(filename.opencv_to_extern(), mats.as_raw_mut_VectorOfMat(), flags) }.into_result()
}

/// Saves an image to a specified file.
/// 
/// The function imwrite saves the image to the specified file. The image format is chosen based on the
/// filename extension (see cv::imread for the list of extensions). In general, only 8-bit
/// single-channel or 3-channel (with 'BGR' channel order) images
/// can be saved using this function, with these exceptions:
/// 
/// - 16-bit unsigned (CV_16U) images can be saved in the case of PNG, JPEG 2000, and TIFF formats
/// - 32-bit float (CV_32F) images can be saved in TIFF, OpenEXR, and Radiance HDR formats; 3-channel
/// (CV_32FC3) TIFF images will be saved using the LogLuv high dynamic range encoding (4 bytes per pixel)
/// - PNG images with an alpha channel can be saved using this function. To do this, create
/// 8-bit (or 16-bit) 4-channel image BGRA, where the alpha channel goes last. Fully transparent pixels
/// should have alpha set to 0, fully opaque pixels should have alpha set to 255/65535 (see the code sample below).
/// 
/// If the format, depth or channel order is different, use
/// Mat::convertTo and cv::cvtColor to convert it before saving. Or, use the universal FileStorage I/O
/// functions to save the image to XML or YAML format.
/// 
/// The sample below shows how to create a BGRA image and save it to a PNG file. It also demonstrates how to set custom
/// compression parameters:
/// @include snippets/imgcodecs_imwrite.cpp
/// ## Parameters
/// * filename: Name of the file.
/// * img: Image to be saved.
/// * params: Format-specific parameters encoded as pairs (paramId_1, paramValue_1, paramId_2, paramValue_2, ... .) see cv::ImwriteFlags
/// 
/// ## C++ default parameters
/// * params: std::vector<int>()
pub fn imwrite(filename: &str, img: &dyn core::ToInputArray, params: &core::Vector::<i32>) -> Result<bool> {
	extern_container_arg!(filename);
	input_array_arg!(img);
	unsafe { sys::cv_imwrite_const_StringX_const__InputArrayX_const_vector_int_X(filename.opencv_to_extern(), img.as_raw__InputArray(), params.as_raw_VectorOfi32()) }.into_result()
}

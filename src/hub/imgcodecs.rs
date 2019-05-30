//! # Image file reading and writing
//! # C API
//! # iOS glue
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const IMREAD_ANYCOLOR: i32 = 4;
pub const IMREAD_ANYDEPTH: i32 = 2;
pub const IMREAD_COLOR: i32 = 1;
pub const IMREAD_GRAYSCALE: i32 = 0;
pub const IMREAD_IGNORE_ORIENTATION: i32 = 128;
pub const IMREAD_LOAD_GDAL: i32 = 8;
pub const IMREAD_REDUCED_COLOR_2: i32 = 17;
pub const IMREAD_REDUCED_COLOR_4: i32 = 33;
pub const IMREAD_REDUCED_COLOR_8: i32 = 65;
pub const IMREAD_REDUCED_GRAYSCALE_2: i32 = 16;
pub const IMREAD_REDUCED_GRAYSCALE_4: i32 = 32;
pub const IMREAD_REDUCED_GRAYSCALE_8: i32 = 64;
pub const IMREAD_UNCHANGED: i32 = -1;
pub const IMWRITE_EXR_TYPE_FLOAT: i32 = 2;
pub const IMWRITE_EXR_TYPE_HALF: i32 = 1;
pub const IMWRITE_JPEG_CHROMA_QUALITY: i32 = 6;
pub const IMWRITE_JPEG_LUMA_QUALITY: i32 = 5;
pub const IMWRITE_JPEG_OPTIMIZE: i32 = 3;
pub const IMWRITE_JPEG_PROGRESSIVE: i32 = 2;
pub const IMWRITE_JPEG_QUALITY: i32 = 1;
pub const IMWRITE_JPEG_RST_INTERVAL: i32 = 4;
pub const IMWRITE_PAM_FORMAT_BLACKANDWHITE: i32 = 1;
pub const IMWRITE_PAM_FORMAT_GRAYSCALE: i32 = 2;
pub const IMWRITE_PAM_FORMAT_GRAYSCALE_ALPHA: i32 = 3;
pub const IMWRITE_PAM_FORMAT_NULL: i32 = 0;
pub const IMWRITE_PAM_FORMAT_RGB: i32 = 4;
pub const IMWRITE_PAM_FORMAT_RGB_ALPHA: i32 = 5;
pub const IMWRITE_PAM_TUPLETYPE: i32 = 128;
pub const IMWRITE_PNG_BILEVEL: i32 = 18;
pub const IMWRITE_PNG_COMPRESSION: i32 = 16;
pub const IMWRITE_PNG_STRATEGY: i32 = 17;
pub const IMWRITE_PNG_STRATEGY_DEFAULT: i32 = 0;
pub const IMWRITE_PNG_STRATEGY_FILTERED: i32 = 1;
pub const IMWRITE_PNG_STRATEGY_FIXED: i32 = 4;
pub const IMWRITE_PNG_STRATEGY_HUFFMAN_ONLY: i32 = 2;
pub const IMWRITE_PNG_STRATEGY_RLE: i32 = 3;
pub const IMWRITE_PXM_BINARY: i32 = 32;
pub const IMWRITE_TIFF_COMPRESSION: i32 = 259;
pub const IMWRITE_TIFF_RESUNIT: i32 = 256;
pub const IMWRITE_TIFF_XDPI: i32 = 257;
pub const IMWRITE_TIFF_YDPI: i32 = 258;
pub const IMWRITE_WEBP_QUALITY: i32 = 64;

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
pub fn imdecode(buf: &core::Mat, flags: i32) -> Result<core::Mat> {
    unsafe { sys::cv_imdecode_Mat_int(buf.as_raw_Mat(), flags) }.into_result().map(|ptr| core::Mat { ptr })
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
pub fn imdecode_to(buf: &core::Mat, flags: i32, dst: &mut core::Mat) -> Result<core::Mat> {
    unsafe { sys::cv_imdecode_Mat_int_Mat(buf.as_raw_Mat(), flags, dst.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
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
pub fn imencode(ext: &str, img: &core::Mat, buf: &mut types::VectorOfuchar, params: &types::VectorOfint) -> Result<bool> {
    string_arg!(ext);
    unsafe { sys::cv_imencode_String_Mat_VectorOfuchar_VectorOfint(ext.as_ptr(), img.as_raw_Mat(), buf.as_raw_VectorOfuchar(), params.as_raw_VectorOfint()) }.into_result()
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
/// Results may differ to the output of cvtColor()
/// *   On Microsoft Windows\* OS and MacOSX\*, the codecs shipped with an OpenCV image (libjpeg,
/// libpng, libtiff, and libjasper) are used by default. So, OpenCV can always read JPEGs, PNGs,
/// and TIFFs. On MacOSX, there is also an option to use native MacOSX image readers. But beware
/// that currently these native image loaders give images with different pixel values because of
/// the color management embedded into MacOSX.
/// *   On Linux\*, BSD flavors and other Unix-like open-source operating systems, OpenCV looks for
/// codecs supplied with an OS image. Install the relevant packages (do not forget the development
/// files, for example, "libjpeg-dev", in Debian\* and Ubuntu\*) to get the codec support or turn
/// on the OPENCV_BUILD_3RDPARTY_LIBS flag in CMake.
/// *   In the case you set *WITH_GDAL* flag to true in CMake and @ref IMREAD_LOAD_GDAL to load the image,
/// then the [GDAL](http://www.gdal.org) driver will be used in order to decode the image, supporting
/// the following formats: [Raster](http://www.gdal.org/formats_list.html),
/// [Vector](http://www.gdal.org/ogr_formats.html).
/// *   If EXIF information are embedded in the image file, the EXIF orientation will be taken into account
/// and thus the image will be rotated accordingly except if the flag @ref IMREAD_IGNORE_ORIENTATION is passed.
/// *   By default number of pixels must be less than 2^30. Limit can be set using system
/// variable OPENCV_IO_MAX_IMAGE_PIXELS
/// 
/// ## Parameters
/// * filename: Name of file to be loaded.
/// * flags: Flag that can take values of cv::ImreadModes
///
/// ## C++ default parameters
/// * flags: IMREAD_COLOR
pub fn imread(filename: &str, flags: i32) -> Result<core::Mat> {
    string_arg!(filename);
    unsafe { sys::cv_imread_String_int(filename.as_ptr(), flags) }.into_result().map(|ptr| core::Mat { ptr })
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
pub fn imreadmulti(filename: &str, mats: &mut types::VectorOfMat, flags: i32) -> Result<bool> {
    string_arg!(filename);
    unsafe { sys::cv_imreadmulti_String_VectorOfMat_int(filename.as_ptr(), mats.as_raw_VectorOfMat(), flags) }.into_result()
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
pub fn imwrite(filename: &str, img: &core::Mat, params: &types::VectorOfint) -> Result<bool> {
    string_arg!(filename);
    unsafe { sys::cv_imwrite_String_Mat_VectorOfint(filename.as_ptr(), img.as_raw_Mat(), params.as_raw_VectorOfint()) }.into_result()
}

pub const IMWRITE_EXR_TYPE: i32 = 0x30; // 48

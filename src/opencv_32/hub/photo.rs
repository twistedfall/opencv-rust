//! # Computational Photography
//! # Denoising
//! # HDR imaging
//!
//! This section describes high dynamic range imaging algorithms namely tonemapping, exposure alignment,
//! camera calibration with multiple exposures and exposure fusion.
//!
//! # Seamless Cloning
//! # Non-Photorealistic Rendering
//! # C API
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const INPAINT_NS: i32 = 0;
pub const INPAINT_TELEA: i32 = 1;
pub const LDR_SIZE: i32 = 256;
pub const MIXED_CLONE: i32 = 2;
pub const MONOCHROME_TRANSFER: i32 = 3;
pub const NORMAL_CLONE: i32 = 1;
pub const NORMCONV_FILTER: i32 = 2;
pub const RECURS_FILTER: i32 = 1;

/// Given an original color image, two differently colored versions of this image can be mixed
/// seamlessly.
///
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * mask: Input 8-bit 1 or 3-channel image.
/// * dst: Output image with the same size and type as src .
/// * red_mul: R-channel multiply factor.
/// * green_mul: G-channel multiply factor.
/// * blue_mul: B-channel multiply factor.
///
/// Multiplication factor is between .5 to 2.5.
///
/// ## C++ default parameters
/// * red_mul: 1.0f
/// * green_mul: 1.0f
/// * blue_mul: 1.0f
pub fn color_change(src: &core::Mat, mask: &core::Mat, dst: &mut core::Mat, red_mul: f32, green_mul: f32, blue_mul: f32) -> Result<()> {
    unsafe { sys::cv_colorChange_Mat_Mat_Mat_float_float_float(src.as_raw_Mat(), mask.as_raw_Mat(), dst.as_raw_Mat(), red_mul, green_mul, blue_mul) }.into_result()
}

/// Creates AlignMTB object
///
/// ## Parameters
/// * max_bits: logarithm to the base 2 of maximal shift in each dimension. Values of 5 and 6 are
/// usually good enough (31 and 63 pixels shift respectively).
/// * exclude_range: range for exclusion bitmap that is constructed to suppress noise around the
/// median value.
/// * cut: if true cuts images, otherwise fills the new regions with zeros.
///
/// ## C++ default parameters
/// * max_bits: 6
/// * exclude_range: 4
/// * cut: true
pub fn create_align_mtb(max_bits: i32, exclude_range: i32, cut: bool) -> Result<types::PtrOfAlignMTB> {
    unsafe { sys::cv_createAlignMTB_int_int_bool(max_bits, exclude_range, cut) }.into_result().map(|ptr| types::PtrOfAlignMTB { ptr })
}

/// Creates CalibrateDebevec object
///
/// ## Parameters
/// * samples: number of pixel locations to use
/// * lambda: smoothness term weight. Greater values produce smoother results, but can alter the
/// response.
/// * random: if true sample pixel locations are chosen at random, otherwise the form a
/// rectangular grid.
///
/// ## C++ default parameters
/// * samples: 70
/// * lambda: 10.0f
/// * random: false
pub fn create_calibrate_debevec(samples: i32, lambda: f32, random: bool) -> Result<types::PtrOfCalibrateDebevec> {
    unsafe { sys::cv_createCalibrateDebevec_int_float_bool(samples, lambda, random) }.into_result().map(|ptr| types::PtrOfCalibrateDebevec { ptr })
}

/// Creates CalibrateRobertson object
///
/// ## Parameters
/// * max_iter: maximal number of Gauss-Seidel solver iterations.
/// * threshold: target difference between results of two successive steps of the minimization.
///
/// ## C++ default parameters
/// * max_iter: 30
/// * threshold: 0.01f
pub fn create_calibrate_robertson(max_iter: i32, threshold: f32) -> Result<types::PtrOfCalibrateRobertson> {
    unsafe { sys::cv_createCalibrateRobertson_int_float(max_iter, threshold) }.into_result().map(|ptr| types::PtrOfCalibrateRobertson { ptr })
}

/// Creates MergeDebevec object
pub fn create_merge_debevec() -> Result<types::PtrOfMergeDebevec> {
    unsafe { sys::cv_createMergeDebevec() }.into_result().map(|ptr| types::PtrOfMergeDebevec { ptr })
}

/// Creates MergeMertens object
///
/// ## Parameters
/// * contrast_weight: contrast measure weight. See MergeMertens.
/// * saturation_weight: saturation measure weight
/// * exposure_weight: well-exposedness measure weight
///
/// ## C++ default parameters
/// * contrast_weight: 1.0f
/// * saturation_weight: 1.0f
/// * exposure_weight: 0.0f
pub fn create_merge_mertens(contrast_weight: f32, saturation_weight: f32, exposure_weight: f32) -> Result<types::PtrOfMergeMertens> {
    unsafe { sys::cv_createMergeMertens_float_float_float(contrast_weight, saturation_weight, exposure_weight) }.into_result().map(|ptr| types::PtrOfMergeMertens { ptr })
}

/// Creates MergeRobertson object
pub fn create_merge_robertson() -> Result<types::PtrOfMergeRobertson> {
    unsafe { sys::cv_createMergeRobertson() }.into_result().map(|ptr| types::PtrOfMergeRobertson { ptr })
}

/// Creates TonemapDrago object
///
/// ## Parameters
/// * gamma: gamma value for gamma correction. See createTonemap
/// * saturation: positive saturation enhancement value. 1.0 preserves saturation, values greater
/// than 1 increase saturation and values less than 1 decrease it.
/// * bias: value for bias function in [0, 1] range. Values from 0.7 to 0.9 usually give best
/// results, default value is 0.85.
///
/// ## C++ default parameters
/// * gamma: 1.0f
/// * saturation: 1.0f
/// * bias: 0.85f
pub fn create_tonemap_drago(gamma: f32, saturation: f32, bias: f32) -> Result<types::PtrOfTonemapDrago> {
    unsafe { sys::cv_createTonemapDrago_float_float_float(gamma, saturation, bias) }.into_result().map(|ptr| types::PtrOfTonemapDrago { ptr })
}

/// Creates TonemapDurand object
///
/// ## Parameters
/// * gamma: gamma value for gamma correction. See createTonemap
/// * contrast: resulting contrast on logarithmic scale, i. e. log(max / min), where max and min
/// are maximum and minimum luminance values of the resulting image.
/// * saturation: saturation enhancement value. See createTonemapDrago
/// * sigma_space: bilateral filter sigma in color space
/// * sigma_color: bilateral filter sigma in coordinate space
///
/// ## C++ default parameters
/// * gamma: 1.0f
/// * contrast: 4.0f
/// * saturation: 1.0f
/// * sigma_space: 2.0f
/// * sigma_color: 2.0f
pub fn create_tonemap_durand(gamma: f32, contrast: f32, saturation: f32, sigma_space: f32, sigma_color: f32) -> Result<types::PtrOfTonemapDurand> {
    unsafe { sys::cv_createTonemapDurand_float_float_float_float_float(gamma, contrast, saturation, sigma_space, sigma_color) }.into_result().map(|ptr| types::PtrOfTonemapDurand { ptr })
}

/// Creates TonemapMantiuk object
///
/// ## Parameters
/// * gamma: gamma value for gamma correction. See createTonemap
/// * scale: contrast scale factor. HVS response is multiplied by this parameter, thus compressing
/// dynamic range. Values from 0.6 to 0.9 produce best results.
/// * saturation: saturation enhancement value. See createTonemapDrago
///
/// ## C++ default parameters
/// * gamma: 1.0f
/// * scale: 0.7f
/// * saturation: 1.0f
pub fn create_tonemap_mantiuk(gamma: f32, scale: f32, saturation: f32) -> Result<types::PtrOfTonemapMantiuk> {
    unsafe { sys::cv_createTonemapMantiuk_float_float_float(gamma, scale, saturation) }.into_result().map(|ptr| types::PtrOfTonemapMantiuk { ptr })
}

/// Creates TonemapReinhard object
///
/// ## Parameters
/// * gamma: gamma value for gamma correction. See createTonemap
/// * intensity: result intensity in [-8, 8] range. Greater intensity produces brighter results.
/// * light_adapt: light adaptation in [0, 1] range. If 1 adaptation is based only on pixel
/// value, if 0 it's global, otherwise it's a weighted mean of this two cases.
/// * color_adapt: chromatic adaptation in [0, 1] range. If 1 channels are treated independently,
/// if 0 adaptation level is the same for each channel.
///
/// ## C++ default parameters
/// * gamma: 1.0f
/// * intensity: 0.0f
/// * light_adapt: 1.0f
/// * color_adapt: 0.0f
pub fn create_tonemap_reinhard(gamma: f32, intensity: f32, light_adapt: f32, color_adapt: f32) -> Result<types::PtrOfTonemapReinhard> {
    unsafe { sys::cv_createTonemapReinhard_float_float_float_float(gamma, intensity, light_adapt, color_adapt) }.into_result().map(|ptr| types::PtrOfTonemapReinhard { ptr })
}

/// Creates simple linear mapper with gamma correction
///
/// ## Parameters
/// * gamma: positive value for gamma correction. Gamma value of 1.0 implies no correction, gamma
/// equal to 2.2f is suitable for most displays.
/// Generally gamma \> 1 brightens the image and gamma \< 1 darkens it.
///
/// ## C++ default parameters
/// * gamma: 1.0f
pub fn create_tonemap(gamma: f32) -> Result<types::PtrOfTonemap> {
    unsafe { sys::cv_createTonemap_float(gamma) }.into_result().map(|ptr| types::PtrOfTonemap { ptr })
}

/// Transforms a color image to a grayscale image. It is a basic tool in digital printing, stylized
/// black-and-white photograph rendering, and in many single channel image processing applications
/// [CL12](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_CL12) .
///
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * grayscale: Output 8-bit 1-channel image.
/// * color_boost: Output 8-bit 3-channel image.
///
/// This function is to be applied on color images.
pub fn decolor(src: &core::Mat, grayscale: &mut core::Mat, color_boost: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_decolor_Mat_Mat_Mat(src.as_raw_Mat(), grayscale.as_raw_Mat(), color_boost.as_raw_Mat()) }.into_result()
}

/// Primal-dual algorithm is an algorithm for solving special types of variational problems (that is,
/// finding a function to minimize some functional). As the image denoising, in particular, may be seen
/// as the variational problem, primal-dual algorithm then can be used to perform denoising and this is
/// exactly what is implemented.
///
/// It should be noted, that this implementation was taken from the July 2013 blog entry
/// [MA13](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_MA13) , which also contained (slightly more general) ready-to-use source code on Python.
/// Subsequently, that code was rewritten on C++ with the usage of openCV by Vadim Pisarevsky at the end
/// of July 2013 and finally it was slightly adapted by later authors.
///
/// Although the thorough discussion and justification of the algorithm involved may be found in
/// [ChambolleEtAl](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_ChambolleEtAl), it might make sense to skim over it here, following [MA13](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_MA13) . To begin
/// with, we consider the 1-byte gray-level images as the functions from the rectangular domain of
/// pixels (it may be seen as set
/// <span lang='latex'>\left\{(x,y)\in\mathbb{N}\times\mathbb{N}\mid 1\leq x\leq n,\;1\leq y\leq m\right\}</span> for some
/// <span lang='latex'>m,\;n\in\mathbb{N}</span>) into <span lang='latex'>\{0,1,\dots,255\}</span>. We shall denote the noised images as <span lang='latex'>f_i</span> and with
/// this view, given some image <span lang='latex'>x</span> of the same size, we may measure how bad it is by the formula
///
/// <div lang='latex'>\left\|\left\|\nabla x\right\|\right\| + \lambda\sum_i\left\|\left\|x-f_i\right\|\right\|</div>
///
/// <span lang='latex'>\|\|\cdot\|\|</span> here denotes <span lang='latex'>L_2</span>-norm and as you see, the first addend states that we want our
/// image to be smooth (ideally, having zero gradient, thus being constant) and the second states that
/// we want our result to be close to the observations we've got. If we treat <span lang='latex'>x</span> as a function, this is
/// exactly the functional what we seek to minimize and here the Primal-Dual algorithm comes into play.
///
/// ## Parameters
/// * observations: This array should contain one or more noised versions of the image that is to
/// be restored.
/// * result: Here the denoised image will be stored. There is no need to do pre-allocation of
/// storage space, as it will be automatically allocated, if necessary.
/// * lambda: Corresponds to <span lang='latex'>\lambda</span> in the formulas above. As it is enlarged, the smooth
/// (blurred) images are treated more favorably than detailed (but maybe more noised) ones. Roughly
/// speaking, as it becomes smaller, the result will be more blur but more sever outliers will be
/// removed.
/// * niters: Number of iterations that the algorithm will run. Of course, as more iterations as
/// better, but it is hard to quantitatively refine this statement, so just use the default and
/// increase it if the results are poor.
///
/// ## C++ default parameters
/// * lambda: 1.0
/// * niters: 30
pub fn denoise_tvl1(observations: &types::VectorOfMat, result: &mut core::Mat, lambda: f64, niters: i32) -> Result<()> {
    unsafe { sys::cv_denoise_TVL1_VectorOfMat_Mat_double_int(observations.as_raw_VectorOfMat(), result.as_raw_Mat(), lambda, niters) }.into_result()
}

/// This filter enhances the details of a particular image.
///
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst: Output image with the same size and type as src.
/// * sigma_s: Range between 0 to 200.
/// * sigma_r: Range between 0 to 1.
///
/// ## C++ default parameters
/// * sigma_s: 10
/// * sigma_r: 0.15f
pub fn detail_enhance(src: &core::Mat, dst: &mut core::Mat, sigma_s: f32, sigma_r: f32) -> Result<()> {
    unsafe { sys::cv_detailEnhance_Mat_Mat_float_float(src.as_raw_Mat(), dst.as_raw_Mat(), sigma_s, sigma_r) }.into_result()
}

/// Filtering is the fundamental operation in image and video processing. Edge-preserving smoothing
/// filters are used in many different applications [EM11](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_EM11) .
///
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst: Output 8-bit 3-channel image.
/// * flags: Edge preserving filters:
/// *   **RECURS_FILTER** = 1
/// *   **NORMCONV_FILTER** = 2
/// * sigma_s: Range between 0 to 200.
/// * sigma_r: Range between 0 to 1.
///
/// ## C++ default parameters
/// * flags: 1
/// * sigma_s: 60
/// * sigma_r: 0.4f
pub fn edge_preserving_filter(src: &core::Mat, dst: &mut core::Mat, flags: i32, sigma_s: f32, sigma_r: f32) -> Result<()> {
    unsafe { sys::cv_edgePreservingFilter_Mat_Mat_int_float_float(src.as_raw_Mat(), dst.as_raw_Mat(), flags, sigma_s, sigma_r) }.into_result()
}

/// Modification of fastNlMeansDenoisingMulti function for colored images sequences
///
/// ## Parameters
/// * srcImgs: Input 8-bit 3-channel images sequence. All images should have the same type and
/// size.
/// * imgToDenoiseIndex: Target image to denoise index in srcImgs sequence
/// * temporalWindowSize: Number of surrounding images to use for target image denoising. Should
/// be odd. Images from imgToDenoiseIndex - temporalWindowSize / 2 to
/// imgToDenoiseIndex - temporalWindowSize / 2 from srcImgs will be used to denoise
/// srcImgs[imgToDenoiseIndex] image.
/// * dst: Output image with the same size and type as srcImgs images.
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Parameter regulating filter strength for luminance component. Bigger h value perfectly
/// removes noise but also removes image details, smaller h value preserves details but also preserves
/// some noise.
/// * hColor: The same as h but for color components.
///
/// The function converts images to CIELAB colorspace and then separately denoise L and AB components
/// with given h parameters using fastNlMeansDenoisingMulti function.
///
/// ## C++ default parameters
/// * h: 3
/// * h_color: 3
/// * template_window_size: 7
/// * search_window_size: 21
pub fn fast_nl_means_denoising_colored_multi(src_imgs: &types::VectorOfMat, dst: &mut core::Mat, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
    unsafe { sys::cv_fastNlMeansDenoisingColoredMulti_VectorOfMat_Mat_int_int_float_float_int_int(src_imgs.as_raw_VectorOfMat(), dst.as_raw_Mat(), img_to_denoise_index, temporal_window_size, h, h_color, template_window_size, search_window_size) }.into_result()
}

/// Modification of fastNlMeansDenoising function for colored images
///
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst: Output image with the same size and type as src .
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Parameter regulating filter strength for luminance component. Bigger h value perfectly
/// removes noise but also removes image details, smaller h value preserves details but also preserves
/// some noise
/// * hColor: The same as h but for color components. For most images value equals 10
/// will be enough to remove colored noise and do not distort colors
///
/// The function converts image to CIELAB colorspace and then separately denoise L and AB components
/// with given h parameters using fastNlMeansDenoising function.
///
/// ## C++ default parameters
/// * h: 3
/// * h_color: 3
/// * template_window_size: 7
/// * search_window_size: 21
pub fn fast_nl_means_denoising_color(src: &core::Mat, dst: &mut core::Mat, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
    unsafe { sys::cv_fastNlMeansDenoisingColored_Mat_Mat_float_float_int_int(src.as_raw_Mat(), dst.as_raw_Mat(), h, h_color, template_window_size, search_window_size) }.into_result()
}

/// Modification of fastNlMeansDenoising function for images sequence where consequtive images have been
/// captured in small period of time. For example video. This version of the function is for grayscale
/// images or for manual manipulation with colorspaces. For more details see
/// <http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.131.6394>
///
/// ## Parameters
/// * srcImgs: Input 8-bit or 16-bit (only with NORM_L1) 1-channel,
/// 2-channel, 3-channel or 4-channel images sequence. All images should
/// have the same type and size.
/// * imgToDenoiseIndex: Target image to denoise index in srcImgs sequence
/// * temporalWindowSize: Number of surrounding images to use for target image denoising. Should
/// be odd. Images from imgToDenoiseIndex - temporalWindowSize / 2 to
/// imgToDenoiseIndex - temporalWindowSize / 2 from srcImgs will be used to denoise
/// srcImgs[imgToDenoiseIndex] image.
/// * dst: Output image with the same size and type as srcImgs images.
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Array of parameters regulating filter strength, either one
/// parameter applied to all channels or one per channel in dst. Big h value
/// perfectly removes noise but also removes image details, smaller h
/// value preserves details but also preserves some noise
/// * normType: Type of norm used for weight calculation. Can be either NORM_L2 or NORM_L1
///
/// ## C++ default parameters
/// * template_window_size: 7
/// * search_window_size: 21
/// * norm_type: NORM_L2
pub fn fast_nl_means_denoising_multi(src_imgs: &types::VectorOfMat, dst: &mut core::Mat, img_to_denoise_index: i32, temporal_window_size: i32, h: &types::VectorOffloat, template_window_size: i32, search_window_size: i32, norm_type: i32) -> Result<()> {
    unsafe { sys::cv_fastNlMeansDenoisingMulti_VectorOfMat_Mat_int_int_VectorOffloat_int_int_int(src_imgs.as_raw_VectorOfMat(), dst.as_raw_Mat(), img_to_denoise_index, temporal_window_size, h.as_raw_VectorOffloat(), template_window_size, search_window_size, norm_type) }.into_result()
}

/// Modification of fastNlMeansDenoising function for images sequence where consequtive images have been
/// captured in small period of time. For example video. This version of the function is for grayscale
/// images or for manual manipulation with colorspaces. For more details see
/// <http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.131.6394>
///
/// ## Parameters
/// * srcImgs: Input 8-bit 1-channel, 2-channel, 3-channel or
/// 4-channel images sequence. All images should have the same type and
/// size.
/// * imgToDenoiseIndex: Target image to denoise index in srcImgs sequence
/// * temporalWindowSize: Number of surrounding images to use for target image denoising. Should
/// be odd. Images from imgToDenoiseIndex - temporalWindowSize / 2 to
/// imgToDenoiseIndex - temporalWindowSize / 2 from srcImgs will be used to denoise
/// srcImgs[imgToDenoiseIndex] image.
/// * dst: Output image with the same size and type as srcImgs images.
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Parameter regulating filter strength. Bigger h value
/// perfectly removes noise but also removes image details, smaller h
/// value preserves details but also preserves some noise
///
/// ## C++ default parameters
/// * h: 3
/// * template_window_size: 7
/// * search_window_size: 21
pub fn fast_nl_means_denoising_multi_1(src_imgs: &types::VectorOfMat, dst: &mut core::Mat, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
    unsafe { sys::cv_fastNlMeansDenoisingMulti_VectorOfMat_Mat_int_int_float_int_int(src_imgs.as_raw_VectorOfMat(), dst.as_raw_Mat(), img_to_denoise_index, temporal_window_size, h, template_window_size, search_window_size) }.into_result()
}

/// Perform image denoising using Non-local Means Denoising algorithm
/// <http://www.ipol.im/pub/algo/bcm_non_local_means_denoising/> with several computational
/// optimizations. Noise expected to be a gaussian white noise
///
/// ## Parameters
/// * src: Input 8-bit or 16-bit (only with NORM_L1) 1-channel,
/// 2-channel, 3-channel or 4-channel image.
/// * dst: Output image with the same size and type as src .
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Array of parameters regulating filter strength, either one
/// parameter applied to all channels or one per channel in dst. Big h value
/// perfectly removes noise but also removes image details, smaller h
/// value preserves details but also preserves some noise
/// * normType: Type of norm used for weight calculation. Can be either NORM_L2 or NORM_L1
///
/// This function expected to be applied to grayscale images. For colored images look at
/// fastNlMeansDenoisingColored. Advanced usage of this functions can be manual denoising of colored
/// image in different colorspaces. Such approach is used in fastNlMeansDenoisingColored by converting
/// image to CIELAB colorspace and then separately denoise L and AB components with different h
/// parameter.
///
/// ## C++ default parameters
/// * template_window_size: 7
/// * search_window_size: 21
/// * norm_type: NORM_L2
pub fn fast_nl_means_denoising_vec(src: &core::Mat, dst: &mut core::Mat, h: &types::VectorOffloat, template_window_size: i32, search_window_size: i32, norm_type: i32) -> Result<()> {
    unsafe { sys::cv_fastNlMeansDenoising_Mat_Mat_VectorOffloat_int_int_int(src.as_raw_Mat(), dst.as_raw_Mat(), h.as_raw_VectorOffloat(), template_window_size, search_window_size, norm_type) }.into_result()
}

/// Perform image denoising using Non-local Means Denoising algorithm
/// <http://www.ipol.im/pub/algo/bcm_non_local_means_denoising/> with several computational
/// optimizations. Noise expected to be a gaussian white noise
///
/// ## Parameters
/// * src: Input 8-bit 1-channel, 2-channel, 3-channel or 4-channel image.
/// * dst: Output image with the same size and type as src .
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Parameter regulating filter strength. Big h value perfectly removes noise but also
/// removes image details, smaller h value preserves details but also preserves some noise
///
/// This function expected to be applied to grayscale images. For colored images look at
/// fastNlMeansDenoisingColored. Advanced usage of this functions can be manual denoising of colored
/// image in different colorspaces. Such approach is used in fastNlMeansDenoisingColored by converting
/// image to CIELAB colorspace and then separately denoise L and AB components with different h
/// parameter.
///
/// ## C++ default parameters
/// * h: 3
/// * template_window_size: 7
/// * search_window_size: 21
pub fn fast_nl_means_denoising_window(src: &core::Mat, dst: &mut core::Mat, h: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
    unsafe { sys::cv_fastNlMeansDenoising_Mat_Mat_float_int_int(src.as_raw_Mat(), dst.as_raw_Mat(), h, template_window_size, search_window_size) }.into_result()
}

/// Applying an appropriate non-linear transformation to the gradient field inside the selection and
/// then integrating back with a Poisson solver, modifies locally the apparent illumination of an image.
///
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * mask: Input 8-bit 1 or 3-channel image.
/// * dst: Output image with the same size and type as src.
/// * alpha: Value ranges between 0-2.
/// * beta: Value ranges between 0-2.
///
/// This is useful to highlight under-exposed foreground objects or to reduce specular reflections.
///
/// ## C++ default parameters
/// * alpha: 0.2f
/// * beta: 0.4f
pub fn illumination_change(src: &core::Mat, mask: &core::Mat, dst: &mut core::Mat, alpha: f32, beta: f32) -> Result<()> {
    unsafe { sys::cv_illuminationChange_Mat_Mat_Mat_float_float(src.as_raw_Mat(), mask.as_raw_Mat(), dst.as_raw_Mat(), alpha, beta) }.into_result()
}

/// Restores the selected region in an image using the region neighborhood.
///
/// ## Parameters
/// * src: Input 8-bit 1-channel or 3-channel image.
/// * inpaintMask: Inpainting mask, 8-bit 1-channel image. Non-zero pixels indicate the area that
/// needs to be inpainted.
/// * dst: Output image with the same size and type as src .
/// * inpaintRadius: Radius of a circular neighborhood of each point inpainted that is considered
/// by the algorithm.
/// * flags: Inpainting method that could be one of the following:
/// *   **INPAINT_NS** Navier-Stokes based method [Navier01]
/// *   **INPAINT_TELEA** Method by Alexandru Telea [Telea04](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Telea04) .
///
/// The function reconstructs the selected image area from the pixel near the area boundary. The
/// function may be used to remove dust and scratches from a scanned photo, or to remove undesirable
/// objects from still images or video. See <http://en.wikipedia.org/wiki/Inpainting> for more details.
///
///
/// Note:
/// *   An example using the inpainting technique can be found at
/// opencv_source_code/samples/cpp/inpaint.cpp
/// *   (Python) An example using the inpainting technique can be found at
/// opencv_source_code/samples/python/inpaint.py
pub fn inpaint(src: &core::Mat, inpaint_mask: &core::Mat, dst: &mut core::Mat, inpaint_radius: f64, flags: i32) -> Result<()> {
    unsafe { sys::cv_inpaint_Mat_Mat_Mat_double_int(src.as_raw_Mat(), inpaint_mask.as_raw_Mat(), dst.as_raw_Mat(), inpaint_radius, flags) }.into_result()
}

/// Pencil-like non-photorealistic line drawing
///
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst1: Output 8-bit 1-channel image.
/// * dst2: Output image with the same size and type as src.
/// * sigma_s: Range between 0 to 200.
/// * sigma_r: Range between 0 to 1.
/// * shade_factor: Range between 0 to 0.1.
///
/// ## C++ default parameters
/// * sigma_s: 60
/// * sigma_r: 0.07f
/// * shade_factor: 0.02f
pub fn pencil_sketch(src: &core::Mat, dst1: &mut core::Mat, dst2: &mut core::Mat, sigma_s: f32, sigma_r: f32, shade_factor: f32) -> Result<()> {
    unsafe { sys::cv_pencilSketch_Mat_Mat_Mat_float_float_float(src.as_raw_Mat(), dst1.as_raw_Mat(), dst2.as_raw_Mat(), sigma_s, sigma_r, shade_factor) }.into_result()
}

/// Image editing tasks concern either global changes (color/intensity corrections, filters,
/// deformations) or local changes concerned to a selection. Here we are interested in achieving local
/// changes, ones that are restricted to a region manually selected (ROI), in a seamless and effortless
/// manner. The extent of the changes ranges from slight distortions to complete replacement by novel
/// content [PM03](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_PM03) .
///
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst: Input 8-bit 3-channel image.
/// * mask: Input 8-bit 1 or 3-channel image.
/// * p: Point in dst image where object is placed.
/// * blend: Output image with the same size and type as dst.
/// * flags: Cloning method that could be one of the following:
/// *   **NORMAL_CLONE** The power of the method is fully expressed when inserting objects with
/// complex outlines into a new background
/// *   **MIXED_CLONE** The classic method, color-based selection and alpha masking might be time
/// consuming and often leaves an undesirable halo. Seamless cloning, even averaged with the
/// original image, is not effective. Mixed seamless cloning based on a loose selection proves
/// effective.
/// *   **FEATURE_EXCHANGE** Feature exchange allows the user to easily replace certain features of
/// one object by alternative features.
pub fn seamless_clone(src: &core::Mat, dst: &core::Mat, mask: &core::Mat, p: core::Point, blend: &mut core::Mat, flags: i32) -> Result<()> {
    unsafe { sys::cv_seamlessClone_Mat_Mat_Mat_Point_Mat_int(src.as_raw_Mat(), dst.as_raw_Mat(), mask.as_raw_Mat(), p, blend.as_raw_Mat(), flags) }.into_result()
}

/// Stylization aims to produce digital imagery with a wide variety of effects not focused on
/// photorealism. Edge-aware filters are ideal for stylization, as they can abstract regions of low
/// contrast while preserving, or enhancing, high-contrast features.
///
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst: Output image with the same size and type as src.
/// * sigma_s: Range between 0 to 200.
/// * sigma_r: Range between 0 to 1.
///
/// ## C++ default parameters
/// * sigma_s: 60
/// * sigma_r: 0.45f
pub fn stylization(src: &core::Mat, dst: &mut core::Mat, sigma_s: f32, sigma_r: f32) -> Result<()> {
    unsafe { sys::cv_stylization_Mat_Mat_float_float(src.as_raw_Mat(), dst.as_raw_Mat(), sigma_s, sigma_r) }.into_result()
}

/// By retaining only the gradients at edge locations, before integrating with the Poisson solver, one
/// washes out the texture of the selected region, giving its contents a flat aspect. Here Canny Edge
/// Detector is used.
///
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * mask: Input 8-bit 1 or 3-channel image.
/// * dst: Output image with the same size and type as src.
/// * low_threshold: Range from 0 to 100.
/// * high_threshold: Value \> 100.
/// * kernel_size: The size of the Sobel kernel to be used.
///
/// **NOTE:**
///
/// The algorithm assumes that the color of the source image is close to that of the destination. This
/// assumption means that when the colors don't match, the source image color gets tinted toward the
/// color of the destination image.
///
/// ## C++ default parameters
/// * low_threshold: 30
/// * high_threshold: 45
/// * kernel_size: 3
pub fn texture_flattening(src: &core::Mat, mask: &core::Mat, dst: &mut core::Mat, low_threshold: f32, high_threshold: f32, kernel_size: i32) -> Result<()> {
    unsafe { sys::cv_textureFlattening_Mat_Mat_Mat_float_float_int(src.as_raw_Mat(), mask.as_raw_Mat(), dst.as_raw_Mat(), low_threshold, high_threshold, kernel_size) }.into_result()
}

// Generating impl for trait cv::AlignExposures (trait)
/// The base class for algorithms that align images of the same scene with different exposures
pub trait AlignExposures: core::Algorithm {
    #[inline(always)] fn as_raw_AlignExposures(&self) -> *mut c_void;
    /// Aligns images
    ///
    /// ## Parameters
    /// * src: vector of input images
    /// * dst: vector of aligned images
    /// * times: vector of exposure time values for each image
    /// * response: 256x1 matrix with inverse camera response function for each pixel value, it should
    /// have the same number of channels as images.
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut types::VectorOfMat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_AlignExposures_process_VectorOfMat_VectorOfMat_Mat_Mat(self.as_raw_AlignExposures(), src.as_raw_VectorOfMat(), dst.as_raw_VectorOfMat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::AlignMTB (trait)
/// This algorithm converts images to median threshold bitmaps (1 for pixels brighter than median
/// luminance and 0 otherwise) and than aligns the resulting bitmaps using bit operations.
///
/// It is invariant to exposure, so exposure values and camera response are not necessary.
///
/// In this implementation new image regions are filled with zeros.
///
/// For more information see [GW03](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_GW03) .
pub trait AlignMTB: crate::photo::AlignExposures {
    #[inline(always)] fn as_raw_AlignMTB(&self) -> *mut c_void;
    fn process_with_response(&mut self, src: &types::VectorOfMat, dst: &mut types::VectorOfMat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_AlignMTB_process_VectorOfMat_VectorOfMat_Mat_Mat(self.as_raw_AlignMTB(), src.as_raw_VectorOfMat(), dst.as_raw_VectorOfMat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
    /// Short version of process, that doesn't take extra arguments.
    ///
    /// ## Parameters
    /// * src: vector of input images
    /// * dst: vector of aligned images
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_AlignMTB_process_VectorOfMat_VectorOfMat(self.as_raw_AlignMTB(), src.as_raw_VectorOfMat(), dst.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// Calculates shift between two images, i. e. how to shift the second image to correspond it with the
    /// first.
    ///
    /// ## Parameters
    /// * img0: first image
    /// * img1: second image
    fn calculate_shift(&mut self, img0: &core::Mat, img1: &core::Mat) -> Result<core::Point> {
        unsafe { sys::cv_AlignMTB_calculateShift_Mat_Mat(self.as_raw_AlignMTB(), img0.as_raw_Mat(), img1.as_raw_Mat()) }.into_result()
    }
    
    /// Helper function, that shift Mat filling new regions with zeros.
    ///
    /// ## Parameters
    /// * src: input image
    /// * dst: result image
    /// * shift: shift value
    fn shift_mat(&mut self, src: &core::Mat, dst: &mut core::Mat, shift: core::Point) -> Result<()> {
        unsafe { sys::cv_AlignMTB_shiftMat_Mat_Mat_Point(self.as_raw_AlignMTB(), src.as_raw_Mat(), dst.as_raw_Mat(), shift) }.into_result()
    }
    
    /// Computes median threshold and exclude bitmaps of given image.
    ///
    /// ## Parameters
    /// * img: input image
    /// * tb: median threshold bitmap
    /// * eb: exclude bitmap
    fn compute_bitmaps(&mut self, img: &core::Mat, tb: &mut core::Mat, eb: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_AlignMTB_computeBitmaps_Mat_Mat_Mat(self.as_raw_AlignMTB(), img.as_raw_Mat(), tb.as_raw_Mat(), eb.as_raw_Mat()) }.into_result()
    }
    
    fn get_max_bits(&self) -> Result<i32> {
        unsafe { sys::cv_AlignMTB_getMaxBits_const(self.as_raw_AlignMTB()) }.into_result()
    }
    
    fn set_max_bits(&mut self, max_bits: i32) -> Result<()> {
        unsafe { sys::cv_AlignMTB_setMaxBits_int(self.as_raw_AlignMTB(), max_bits) }.into_result()
    }
    
    fn get_exclude_range(&self) -> Result<i32> {
        unsafe { sys::cv_AlignMTB_getExcludeRange_const(self.as_raw_AlignMTB()) }.into_result()
    }
    
    fn set_exclude_range(&mut self, exclude_range: i32) -> Result<()> {
        unsafe { sys::cv_AlignMTB_setExcludeRange_int(self.as_raw_AlignMTB(), exclude_range) }.into_result()
    }
    
    fn get_cut(&self) -> Result<bool> {
        unsafe { sys::cv_AlignMTB_getCut_const(self.as_raw_AlignMTB()) }.into_result()
    }
    
    fn set_cut(&mut self, value: bool) -> Result<()> {
        unsafe { sys::cv_AlignMTB_setCut_bool(self.as_raw_AlignMTB(), value) }.into_result()
    }
    
}

// Generating impl for trait cv::CalibrateCRF (trait)
/// The base class for camera response calibration algorithms.
pub trait CalibrateCRF: core::Algorithm {
    #[inline(always)] fn as_raw_CalibrateCRF(&self) -> *mut c_void;
    /// Recovers inverse camera response.
    ///
    /// ## Parameters
    /// * src: vector of input images
    /// * dst: 256x1 matrix with inverse camera response function
    /// * times: vector of exposure time values for each image
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat) -> Result<()> {
        unsafe { sys::cv_CalibrateCRF_process_VectorOfMat_Mat_Mat(self.as_raw_CalibrateCRF(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::CalibrateDebevec (trait)
/// Inverse camera response function is extracted for each brightness value by minimizing an objective
/// function as linear system. Objective function is constructed using pixel values on the same position
/// in all images, extra term is added to make the result smoother.
///
/// For more information see [DM97](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_DM97) .
pub trait CalibrateDebevec: crate::photo::CalibrateCRF {
    #[inline(always)] fn as_raw_CalibrateDebevec(&self) -> *mut c_void;
    fn get_lambda(&self) -> Result<f32> {
        unsafe { sys::cv_CalibrateDebevec_getLambda_const(self.as_raw_CalibrateDebevec()) }.into_result()
    }
    
    fn set_lambda(&mut self, lambda: f32) -> Result<()> {
        unsafe { sys::cv_CalibrateDebevec_setLambda_float(self.as_raw_CalibrateDebevec(), lambda) }.into_result()
    }
    
    fn get_samples(&self) -> Result<i32> {
        unsafe { sys::cv_CalibrateDebevec_getSamples_const(self.as_raw_CalibrateDebevec()) }.into_result()
    }
    
    fn set_samples(&mut self, samples: i32) -> Result<()> {
        unsafe { sys::cv_CalibrateDebevec_setSamples_int(self.as_raw_CalibrateDebevec(), samples) }.into_result()
    }
    
    fn get_random(&self) -> Result<bool> {
        unsafe { sys::cv_CalibrateDebevec_getRandom_const(self.as_raw_CalibrateDebevec()) }.into_result()
    }
    
    fn set_random(&mut self, random: bool) -> Result<()> {
        unsafe { sys::cv_CalibrateDebevec_setRandom_bool(self.as_raw_CalibrateDebevec(), random) }.into_result()
    }
    
}

// Generating impl for trait cv::CalibrateRobertson (trait)
/// Inverse camera response function is extracted for each brightness value by minimizing an objective
/// function as linear system. This algorithm uses all image pixels.
///
/// For more information see [RB99](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_RB99) .
pub trait CalibrateRobertson: crate::photo::CalibrateCRF {
    #[inline(always)] fn as_raw_CalibrateRobertson(&self) -> *mut c_void;
    fn get_max_iter(&self) -> Result<i32> {
        unsafe { sys::cv_CalibrateRobertson_getMaxIter_const(self.as_raw_CalibrateRobertson()) }.into_result()
    }
    
    fn set_max_iter(&mut self, max_iter: i32) -> Result<()> {
        unsafe { sys::cv_CalibrateRobertson_setMaxIter_int(self.as_raw_CalibrateRobertson(), max_iter) }.into_result()
    }
    
    fn get_threshold(&self) -> Result<f32> {
        unsafe { sys::cv_CalibrateRobertson_getThreshold_const(self.as_raw_CalibrateRobertson()) }.into_result()
    }
    
    fn set_threshold(&mut self, threshold: f32) -> Result<()> {
        unsafe { sys::cv_CalibrateRobertson_setThreshold_float(self.as_raw_CalibrateRobertson(), threshold) }.into_result()
    }
    
    fn get_radiance(&self) -> Result<core::Mat> {
        unsafe { sys::cv_CalibrateRobertson_getRadiance_const(self.as_raw_CalibrateRobertson()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// Generating impl for trait cv::MergeDebevec (trait)
/// The resulting HDR image is calculated as weighted average of the exposures considering exposure
/// values and camera response.
///
/// For more information see [DM97](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_DM97) .
pub trait MergeDebevec: crate::photo::MergeExposures {
    #[inline(always)] fn as_raw_MergeDebevec(&self) -> *mut c_void;
    fn process_with_response(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_MergeDebevec_process_VectorOfMat_Mat_Mat_Mat(self.as_raw_MergeDebevec(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat) -> Result<()> {
        unsafe { sys::cv_MergeDebevec_process_VectorOfMat_Mat_Mat(self.as_raw_MergeDebevec(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::MergeExposures (trait)
/// The base class algorithms that can merge exposure sequence to a single image.
pub trait MergeExposures: core::Algorithm {
    #[inline(always)] fn as_raw_MergeExposures(&self) -> *mut c_void;
    /// Merges images.
    ///
    /// ## Parameters
    /// * src: vector of input images
    /// * dst: result image
    /// * times: vector of exposure time values for each image
    /// * response: 256x1 matrix with inverse camera response function for each pixel value, it should
    /// have the same number of channels as images.
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_MergeExposures_process_VectorOfMat_Mat_Mat_Mat(self.as_raw_MergeExposures(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::MergeMertens (trait)
/// Pixels are weighted using contrast, saturation and well-exposedness measures, than images are
/// combined using laplacian pyramids.
///
/// The resulting image weight is constructed as weighted average of contrast, saturation and
/// well-exposedness measures.
///
/// The resulting image doesn't require tonemapping and can be converted to 8-bit image by multiplying
/// by 255, but it's recommended to apply gamma correction and/or linear tonemapping.
///
/// For more information see [MK07](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_MK07) .
pub trait MergeMertens: crate::photo::MergeExposures {
    #[inline(always)] fn as_raw_MergeMertens(&self) -> *mut c_void;
    fn process_with_response(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_MergeMertens_process_VectorOfMat_Mat_Mat_Mat(self.as_raw_MergeMertens(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
    /// Short version of process, that doesn't take extra arguments.
    ///
    /// ## Parameters
    /// * src: vector of input images
    /// * dst: result image
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_MergeMertens_process_VectorOfMat_Mat(self.as_raw_MergeMertens(), src.as_raw_VectorOfMat(), dst.as_raw_Mat()) }.into_result()
    }
    
    fn get_contrast_weight(&self) -> Result<f32> {
        unsafe { sys::cv_MergeMertens_getContrastWeight_const(self.as_raw_MergeMertens()) }.into_result()
    }
    
    fn set_contrast_weight(&mut self, contrast_weiht: f32) -> Result<()> {
        unsafe { sys::cv_MergeMertens_setContrastWeight_float(self.as_raw_MergeMertens(), contrast_weiht) }.into_result()
    }
    
    fn get_saturation_weight(&self) -> Result<f32> {
        unsafe { sys::cv_MergeMertens_getSaturationWeight_const(self.as_raw_MergeMertens()) }.into_result()
    }
    
    fn set_saturation_weight(&mut self, saturation_weight: f32) -> Result<()> {
        unsafe { sys::cv_MergeMertens_setSaturationWeight_float(self.as_raw_MergeMertens(), saturation_weight) }.into_result()
    }
    
    fn get_exposure_weight(&self) -> Result<f32> {
        unsafe { sys::cv_MergeMertens_getExposureWeight_const(self.as_raw_MergeMertens()) }.into_result()
    }
    
    fn set_exposure_weight(&mut self, exposure_weight: f32) -> Result<()> {
        unsafe { sys::cv_MergeMertens_setExposureWeight_float(self.as_raw_MergeMertens(), exposure_weight) }.into_result()
    }
    
}

// Generating impl for trait cv::MergeRobertson (trait)
/// The resulting HDR image is calculated as weighted average of the exposures considering exposure
/// values and camera response.
///
/// For more information see [RB99](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_RB99) .
pub trait MergeRobertson: crate::photo::MergeExposures {
    #[inline(always)] fn as_raw_MergeRobertson(&self) -> *mut c_void;
    fn process_with_response(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_MergeRobertson_process_VectorOfMat_Mat_Mat_Mat(self.as_raw_MergeRobertson(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat) -> Result<()> {
        unsafe { sys::cv_MergeRobertson_process_VectorOfMat_Mat_Mat(self.as_raw_MergeRobertson(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::Tonemap (trait)
/// Base class for tonemapping algorithms - tools that are used to map HDR image to 8-bit range.
pub trait Tonemap: core::Algorithm {
    #[inline(always)] fn as_raw_Tonemap(&self) -> *mut c_void;
    /// Tonemaps image
    ///
    /// ## Parameters
    /// * src: source image - 32-bit 3-channel Mat
    /// * dst: destination image - 32-bit 3-channel Mat with values in [0, 1] range
    fn process(&mut self, src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_Tonemap_process_Mat_Mat(self.as_raw_Tonemap(), src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
    }
    
    fn get_gamma(&self) -> Result<f32> {
        unsafe { sys::cv_Tonemap_getGamma_const(self.as_raw_Tonemap()) }.into_result()
    }
    
    fn set_gamma(&mut self, gamma: f32) -> Result<()> {
        unsafe { sys::cv_Tonemap_setGamma_float(self.as_raw_Tonemap(), gamma) }.into_result()
    }
    
}

// Generating impl for trait cv::TonemapDrago (trait)
/// Adaptive logarithmic mapping is a fast global tonemapping algorithm that scales the image in
/// logarithmic domain.
///
/// Since it's a global operator the same function is applied to all the pixels, it is controlled by the
/// bias parameter.
///
/// Optional saturation enhancement is possible as described in [FL02](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_FL02) .
///
/// For more information see [DM03](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_DM03) .
pub trait TonemapDrago: crate::photo::Tonemap {
    #[inline(always)] fn as_raw_TonemapDrago(&self) -> *mut c_void;
    fn get_saturation(&self) -> Result<f32> {
        unsafe { sys::cv_TonemapDrago_getSaturation_const(self.as_raw_TonemapDrago()) }.into_result()
    }
    
    fn set_saturation(&mut self, saturation: f32) -> Result<()> {
        unsafe { sys::cv_TonemapDrago_setSaturation_float(self.as_raw_TonemapDrago(), saturation) }.into_result()
    }
    
    fn get_bias(&self) -> Result<f32> {
        unsafe { sys::cv_TonemapDrago_getBias_const(self.as_raw_TonemapDrago()) }.into_result()
    }
    
    fn set_bias(&mut self, bias: f32) -> Result<()> {
        unsafe { sys::cv_TonemapDrago_setBias_float(self.as_raw_TonemapDrago(), bias) }.into_result()
    }
    
}

// Generating impl for trait cv::TonemapDurand (trait)
/// This algorithm decomposes image into two layers: base layer and detail layer using bilateral filter
/// and compresses contrast of the base layer thus preserving all the details.
///
/// This implementation uses regular bilateral filter from opencv.
///
/// Saturation enhancement is possible as in ocvTonemapDrago.
///
/// For more information see [DD02](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_DD02) .
pub trait TonemapDurand: crate::photo::Tonemap {
    #[inline(always)] fn as_raw_TonemapDurand(&self) -> *mut c_void;
    fn get_saturation(&self) -> Result<f32> {
        unsafe { sys::cv_TonemapDurand_getSaturation_const(self.as_raw_TonemapDurand()) }.into_result()
    }
    
    fn set_saturation(&mut self, saturation: f32) -> Result<()> {
        unsafe { sys::cv_TonemapDurand_setSaturation_float(self.as_raw_TonemapDurand(), saturation) }.into_result()
    }
    
    fn get_contrast(&self) -> Result<f32> {
        unsafe { sys::cv_TonemapDurand_getContrast_const(self.as_raw_TonemapDurand()) }.into_result()
    }
    
    fn set_contrast(&mut self, contrast: f32) -> Result<()> {
        unsafe { sys::cv_TonemapDurand_setContrast_float(self.as_raw_TonemapDurand(), contrast) }.into_result()
    }
    
    fn get_sigma_space(&self) -> Result<f32> {
        unsafe { sys::cv_TonemapDurand_getSigmaSpace_const(self.as_raw_TonemapDurand()) }.into_result()
    }
    
    fn set_sigma_space(&mut self, sigma_space: f32) -> Result<()> {
        unsafe { sys::cv_TonemapDurand_setSigmaSpace_float(self.as_raw_TonemapDurand(), sigma_space) }.into_result()
    }
    
    fn get_sigma_color(&self) -> Result<f32> {
        unsafe { sys::cv_TonemapDurand_getSigmaColor_const(self.as_raw_TonemapDurand()) }.into_result()
    }
    
    fn set_sigma_color(&mut self, sigma_color: f32) -> Result<()> {
        unsafe { sys::cv_TonemapDurand_setSigmaColor_float(self.as_raw_TonemapDurand(), sigma_color) }.into_result()
    }
    
}

// Generating impl for trait cv::TonemapMantiuk (trait)
/// This algorithm transforms image to contrast using gradients on all levels of gaussian pyramid,
/// transforms contrast values to HVS response and scales the response. After this the image is
/// reconstructed from new contrast values.
///
/// For more information see [MM06](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_MM06) .
pub trait TonemapMantiuk: crate::photo::Tonemap {
    #[inline(always)] fn as_raw_TonemapMantiuk(&self) -> *mut c_void;
    fn get_scale(&self) -> Result<f32> {
        unsafe { sys::cv_TonemapMantiuk_getScale_const(self.as_raw_TonemapMantiuk()) }.into_result()
    }
    
    fn set_scale(&mut self, scale: f32) -> Result<()> {
        unsafe { sys::cv_TonemapMantiuk_setScale_float(self.as_raw_TonemapMantiuk(), scale) }.into_result()
    }
    
    fn get_saturation(&self) -> Result<f32> {
        unsafe { sys::cv_TonemapMantiuk_getSaturation_const(self.as_raw_TonemapMantiuk()) }.into_result()
    }
    
    fn set_saturation(&mut self, saturation: f32) -> Result<()> {
        unsafe { sys::cv_TonemapMantiuk_setSaturation_float(self.as_raw_TonemapMantiuk(), saturation) }.into_result()
    }
    
}

// Generating impl for trait cv::TonemapReinhard (trait)
/// This is a global tonemapping operator that models human visual system.
///
/// Mapping function is controlled by adaptation parameter, that is computed using light adaptation and
/// color adaptation.
///
/// For more information see [RD05](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_RD05) .
pub trait TonemapReinhard: crate::photo::Tonemap {
    #[inline(always)] fn as_raw_TonemapReinhard(&self) -> *mut c_void;
    fn get_intensity(&self) -> Result<f32> {
        unsafe { sys::cv_TonemapReinhard_getIntensity_const(self.as_raw_TonemapReinhard()) }.into_result()
    }
    
    fn set_intensity(&mut self, intensity: f32) -> Result<()> {
        unsafe { sys::cv_TonemapReinhard_setIntensity_float(self.as_raw_TonemapReinhard(), intensity) }.into_result()
    }
    
    fn get_light_adaptation(&self) -> Result<f32> {
        unsafe { sys::cv_TonemapReinhard_getLightAdaptation_const(self.as_raw_TonemapReinhard()) }.into_result()
    }
    
    fn set_light_adaptation(&mut self, light_adapt: f32) -> Result<()> {
        unsafe { sys::cv_TonemapReinhard_setLightAdaptation_float(self.as_raw_TonemapReinhard(), light_adapt) }.into_result()
    }
    
    fn get_color_adaptation(&self) -> Result<f32> {
        unsafe { sys::cv_TonemapReinhard_getColorAdaptation_const(self.as_raw_TonemapReinhard()) }.into_result()
    }
    
    fn set_color_adaptation(&mut self, color_adapt: f32) -> Result<()> {
        unsafe { sys::cv_TonemapReinhard_setColorAdaptation_float(self.as_raw_TonemapReinhard(), color_adapt) }.into_result()
    }
    
}


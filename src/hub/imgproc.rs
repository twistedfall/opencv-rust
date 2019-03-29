//! <script type="text/javascript" src="http://latex.codecogs.com/latexit.js"></script>
//! Image Processing
//! 
//! # Image Processing
//! 
//! This module includes image-processing functions.
//! 
//! @{
//! Image Filtering
//! 
//! # Image Filtering
//! 
//! Functions and classes described in this section are used to perform various linear or non-linear
//! filtering operations on 2D images (represented as Mat's). It means that for each pixel location
//! <span lang='latex'>(x,y)</span> in the source image (normally, rectangular), its neighborhood is considered and used to
//! compute the response. In case of a linear filter, it is a weighted sum of pixel values. In case of
//! morphological operations, it is the minimum or maximum values, and so on. The computed response is
//! stored in the destination image at the same location <span lang='latex'>(x,y)</span>. It means that the output image
//! will be of the same size as the input image. Normally, the functions support multi-channel arrays,
//! in which case every channel is processed independently. Therefore, the output image will also have
//! the same number of channels as the input one.
//! 
//! Another common feature of the functions and classes described in this section is that, unlike
//! simple arithmetic functions, they need to extrapolate values of some non-existing pixels. For
//! example, if you want to smooth an image using a Gaussian <span lang='latex'>3 \times 3</span> filter, then, when
//! processing the left-most pixels in each row, you need pixels to the left of them, that is, outside
//! of the image. You can let these pixels be the same as the left-most image pixels ("replicated
//! border" extrapolation method), or assume that all the non-existing pixels are zeros ("constant
//! border" extrapolation method), and so on. OpenCV enables you to specify the extrapolation method.
//! For details, see #BorderTypes
//! 
//! @anchor filter_depths
//! ### Depth combinations
//! Input depth (src.depth()) | Output depth (ddepth)
//! --------------------------|----------------------
//! CV_8U                     | -1/CV_16S/CV_32F/CV_64F
//! CV_16U/CV_16S             | -1/CV_32F/CV_64F
//! CV_32F                    | -1/CV_32F/CV_64F
//! CV_64F                    | -1/CV_64F
//! 
//! 
//! Note: when ddepth=-1, the output image will have the same depth as the source.
//! 
//! Geometric Image Transformations
//! 
//! # Geometric Image Transformations
//! 
//! The functions in this section perform various geometrical transformations of 2D images. They do not
//! change the image content but deform the pixel grid and map this deformed grid to the destination
//! image. In fact, to avoid sampling artifacts, the mapping is done in the reverse order, from
//! destination to the source. That is, for each pixel <span lang='latex'>(x, y)</span> of the destination image, the
//! functions compute coordinates of the corresponding "donor" pixel in the source image and copy the
//! pixel value:
//! 
//! <div lang='latex'>\texttt{dst} (x,y)= \texttt{src} (f_x(x,y), f_y(x,y))</div>
//! 
//! In case when you specify the forward mapping \f$\left<g_x, g_y\right>: \texttt{src} \rightarrow
//! \texttt{dst}\f$, the OpenCV functions first compute the corresponding inverse mapping
//! <span lang='latex'>\left<f_x, f_y\right>: \texttt{dst} \rightarrow \texttt{src}</span> and then use the above formula.
//! 
//! The actual implementations of the geometrical transformations, from the most generic remap and to
//! the simplest and the fastest resize, need to solve two main problems with the above formula:
//! 
//! - Extrapolation of non-existing pixels. Similarly to the filtering functions described in the
//! previous section, for some <span lang='latex'>(x,y)</span>, either one of <span lang='latex'>f_x(x,y)</span>, or <span lang='latex'>f_y(x,y)</span>, or both
//! of them may fall outside of the image. In this case, an extrapolation method needs to be used.
//! OpenCV provides the same selection of extrapolation methods as in the filtering functions. In
//! addition, it provides the method #BORDER_TRANSPARENT. This means that the corresponding pixels in
//! the destination image will not be modified at all.
//! 
//! - Interpolation of pixel values. Usually <span lang='latex'>f_x(x,y)</span> and <span lang='latex'>f_y(x,y)</span> are floating-point
//! numbers. This means that <span lang='latex'>\left<f_x, f_y\right></span> can be either an affine or perspective
//! transformation, or radial lens distortion correction, and so on. So, a pixel value at fractional
//! coordinates needs to be retrieved. In the simplest case, the coordinates can be just rounded to the
//! nearest integer coordinates and the corresponding pixel can be used. This is called a
//! nearest-neighbor interpolation. However, a better result can be achieved by using more
//! sophisticated [interpolation methods](http://en.wikipedia.org/wiki/Multivariate_interpolation) ,
//! where a polynomial function is fit into some neighborhood of the computed pixel \f$(f_x(x,y),
//! f_y(x,y))<span lang='latex'>, and then the value of the polynomial at </span>(f_x(x,y), f_y(x,y))\f$ is taken as the
//! interpolated pixel value. In OpenCV, you can choose between several interpolation methods. See
//! resize for details.
//! 
//! 
//! Note: The geometrical transformations do not work with `CV_8S` or `CV_32S` images.
//! 
//! Miscellaneous Image Transformations
//! 
//! # Miscellaneous Image Transformations
//! Drawing Functions
//! 
//! # Drawing Functions
//! 
//! Drawing functions work with matrices/images of arbitrary depth. The boundaries of the shapes can be
//! rendered with antialiasing (implemented only for 8-bit images for now). All the functions include
//! the parameter color that uses an RGB value (that may be constructed with the Scalar constructor )
//! for color images and brightness for grayscale images. For color images, the channel ordering is
//! normally *Blue, Green, Red*. This is what imshow, imread, and imwrite expect. So, if you form a
//! color using the Scalar constructor, it should look like:
//! 
//! <div lang='latex'>\texttt{Scalar} (blue \_ component, green \_ component, red \_ component[, alpha \_ component])</div>
//! 
//! If you are using your own image rendering and I/O functions, you can use any channel ordering. The
//! drawing functions process each channel independently and do not depend on the channel order or even
//! on the used color space. The whole image can be converted from BGR to RGB or to a different color
//! space using cvtColor .
//! 
//! If a drawn figure is partially or completely outside the image, the drawing functions clip it. Also,
//! many drawing functions can handle pixel coordinates specified with sub-pixel accuracy. This means
//! that the coordinates can be passed as fixed-point numbers encoded as integers. The number of
//! fractional bits is specified by the shift parameter and the real point coordinates are calculated as
//! <span lang='latex'>\texttt{Point}(x,y)\rightarrow\texttt{Point2f}(x*2^{-shift},y*2^{-shift})</span> . This feature is
//! especially effective when rendering antialiased shapes.
//! 
//! 
//! Note: The functions do not support alpha-transparency when the target image is 4-channel. In this
//! case, the color[3] is simply copied to the repainted pixels. Thus, if you want to paint
//! semi-transparent shapes, you can paint them in a separate buffer and then blend it with the main
//! image.
//! 
//! Color Space Conversions
//! 
//! # Color Space Conversions
//! ColorMaps in OpenCV
//! 
//! # ColorMaps in OpenCV
//! 
//! The human perception isn't built for observing fine changes in grayscale images. Human eyes are more
//! sensitive to observing changes between colors, so you often need to recolor your grayscale images to
//! get a clue about them. OpenCV now comes with various colormaps to enhance the visualization in your
//! computer vision application.
//! 
//! In OpenCV you only need applyColorMap to apply a colormap on a given image. The following sample
//! code reads the path to an image from command line, applies a Jet colormap on it and shows the
//! result:
//! 
//! @include snippets/imgproc_applyColorMap.cpp
//! 
//! @see #ColormapTypes
//! 
//! Planar Subdivision
//! 
//! # Planar Subdivision
//! 
//! The Subdiv2D class described in this section is used to perform various planar subdivision on
//! a set of 2D points (represented as vector of Point2f). OpenCV subdivides a plane into triangles
//! using the Delaunay's algorithm, which corresponds to the dual graph of the Voronoi diagram.
//! In the figure below, the Delaunay's triangulation is marked with black lines and the Voronoi
//! diagram with red lines.
//! 
//! ![Delaunay triangulation (black) and Voronoi (red)](pics/delaunay_voronoi.png)
//! 
//! The subdivisions can be used for the 3D piece-wise transformation of a plane, morphing, fast
//! location of points on the plane, building special graphs (such as NNG,RNG), and so forth.
//! 
//! Histograms
//! 
//! # Histograms
//! Structural Analysis and Shape Descriptors
//! 
//! # Structural Analysis and Shape Descriptors
//! Motion Analysis and Object Tracking
//! 
//! # Motion Analysis and Object Tracking
//! Feature Detection
//! 
//! # Feature Detection
//! Object Detection
//! 
//! # Object Detection
//! C API
//! 
//! # C API
//! Hardware Acceleration Layer
//! 
//! # Hardware Acceleration Layer
//! @{
//! Functions
//! 
//! # Functions
//! Interface
//! 
//! # Interface
//! @}
//! @}

use libc::{c_void, c_char, size_t};
use std::ffi::{CStr, CString};
use crate::{core, sys, types};
use crate::{Error, Result};
pub const ADAPTIVE_THRESH_GAUSSIAN_C: i32 = 1;
pub const ADAPTIVE_THRESH_MEAN_C: i32 = 0;
pub const CCL_DEFAULT: i32 = -1;
pub const CCL_GRANA: i32 = 1;
pub const CCL_WU: i32 = 0;
pub const CC_STAT_AREA: i32 = 4;
pub const CC_STAT_HEIGHT: i32 = 3;
pub const CC_STAT_LEFT: i32 = 0;
pub const CC_STAT_MAX: i32 = 5;
pub const CC_STAT_TOP: i32 = 1;
pub const CC_STAT_WIDTH: i32 = 2;
pub const CHAIN_APPROX_NONE: i32 = 1;
pub const CHAIN_APPROX_SIMPLE: i32 = 2;
pub const CHAIN_APPROX_TC89_KCOS: i32 = 4;
pub const CHAIN_APPROX_TC89_L1: i32 = 3;
pub const COLORMAP_AUTUMN: i32 = 0;
pub const COLORMAP_BONE: i32 = 1;
pub const COLORMAP_COOL: i32 = 8;
pub const COLORMAP_HOT: i32 = 11;
pub const COLORMAP_HSV: i32 = 9;
pub const COLORMAP_JET: i32 = 2;
pub const COLORMAP_OCEAN: i32 = 5;
pub const COLORMAP_PARULA: i32 = 12;
pub const COLORMAP_PINK: i32 = 10;
pub const COLORMAP_RAINBOW: i32 = 4;
pub const COLORMAP_SPRING: i32 = 7;
pub const COLORMAP_SUMMER: i32 = 6;
pub const COLORMAP_WINTER: i32 = 3;
pub const COLOR_BGR2BGR555: i32 = 22;
pub const COLOR_BGR2BGR565: i32 = 12;
pub const COLOR_BGR2BGRA: i32 = 0;
pub const COLOR_BGR2GRAY: i32 = 6;
pub const COLOR_BGR2HLS: i32 = 52;
pub const COLOR_BGR2HLS_FULL: i32 = 68;
pub const COLOR_BGR2HSV: i32 = 40;
pub const COLOR_BGR2HSV_FULL: i32 = 66;
pub const COLOR_BGR2Lab: i32 = 44;
pub const COLOR_BGR2Luv: i32 = 50;
pub const COLOR_BGR2RGB: i32 = 4;
pub const COLOR_BGR2RGBA: i32 = 2;
pub const COLOR_BGR2XYZ: i32 = 32;
pub const COLOR_BGR2YCrCb: i32 = 36;
pub const COLOR_BGR2YUV: i32 = 82;
pub const COLOR_BGR2YUV_I420: i32 = 128;
pub const COLOR_BGR2YUV_YV12: i32 = 132;
pub const COLOR_BGR5552BGR: i32 = 24;
pub const COLOR_BGR5552BGRA: i32 = 28;
pub const COLOR_BGR5552GRAY: i32 = 31;
pub const COLOR_BGR5552RGB: i32 = 25;
pub const COLOR_BGR5552RGBA: i32 = 29;
pub const COLOR_BGR5652BGR: i32 = 14;
pub const COLOR_BGR5652BGRA: i32 = 18;
pub const COLOR_BGR5652GRAY: i32 = 21;
pub const COLOR_BGR5652RGB: i32 = 15;
pub const COLOR_BGR5652RGBA: i32 = 19;
pub const COLOR_BGRA2BGR: i32 = 1;
pub const COLOR_BGRA2BGR555: i32 = 26;
pub const COLOR_BGRA2BGR565: i32 = 16;
pub const COLOR_BGRA2GRAY: i32 = 10;
pub const COLOR_BGRA2RGBA: i32 = 5;
pub const COLOR_BGRA2YUV_I420: i32 = 130;
pub const COLOR_BGRA2YUV_YV12: i32 = 134;
pub const COLOR_BayerBG2BGR: i32 = 46;
pub const COLOR_BayerBG2BGRA: i32 = 139;
pub const COLOR_BayerBG2BGR_EA: i32 = 135;
pub const COLOR_BayerBG2BGR_VNG: i32 = 62;
pub const COLOR_BayerBG2GRAY: i32 = 86;
pub const COLOR_BayerGB2BGR: i32 = 47;
pub const COLOR_BayerGB2BGRA: i32 = 140;
pub const COLOR_BayerGB2BGR_EA: i32 = 136;
pub const COLOR_BayerGB2BGR_VNG: i32 = 63;
pub const COLOR_BayerGB2GRAY: i32 = 87;
pub const COLOR_BayerGR2BGR: i32 = 49;
pub const COLOR_BayerGR2BGRA: i32 = 142;
pub const COLOR_BayerGR2BGR_EA: i32 = 138;
pub const COLOR_BayerGR2BGR_VNG: i32 = 65;
pub const COLOR_BayerGR2GRAY: i32 = 89;
pub const COLOR_BayerRG2BGR: i32 = 48;
pub const COLOR_BayerRG2BGRA: i32 = 141;
pub const COLOR_BayerRG2BGR_EA: i32 = 137;
pub const COLOR_BayerRG2BGR_VNG: i32 = 64;
pub const COLOR_BayerRG2GRAY: i32 = 88;
pub const COLOR_COLORCVT_MAX: i32 = 143;
pub const COLOR_GRAY2BGR: i32 = 8;
pub const COLOR_GRAY2BGR555: i32 = 30;
pub const COLOR_GRAY2BGR565: i32 = 20;
pub const COLOR_GRAY2BGRA: i32 = 9;
pub const COLOR_HLS2BGR: i32 = 60;
pub const COLOR_HLS2BGR_FULL: i32 = 72;
pub const COLOR_HLS2RGB: i32 = 61;
pub const COLOR_HLS2RGB_FULL: i32 = 73;
pub const COLOR_HSV2BGR: i32 = 54;
pub const COLOR_HSV2BGR_FULL: i32 = 70;
pub const COLOR_HSV2RGB: i32 = 55;
pub const COLOR_HSV2RGB_FULL: i32 = 71;
pub const COLOR_LBGR2Lab: i32 = 74;
pub const COLOR_LBGR2Luv: i32 = 76;
pub const COLOR_LRGB2Lab: i32 = 75;
pub const COLOR_LRGB2Luv: i32 = 77;
pub const COLOR_Lab2BGR: i32 = 56;
pub const COLOR_Lab2LBGR: i32 = 78;
pub const COLOR_Lab2LRGB: i32 = 79;
pub const COLOR_Lab2RGB: i32 = 57;
pub const COLOR_Luv2BGR: i32 = 58;
pub const COLOR_Luv2LBGR: i32 = 80;
pub const COLOR_Luv2LRGB: i32 = 81;
pub const COLOR_Luv2RGB: i32 = 59;
pub const COLOR_RGB2BGR555: i32 = 23;
pub const COLOR_RGB2BGR565: i32 = 13;
pub const COLOR_RGB2GRAY: i32 = 7;
pub const COLOR_RGB2HLS: i32 = 53;
pub const COLOR_RGB2HLS_FULL: i32 = 69;
pub const COLOR_RGB2HSV: i32 = 41;
pub const COLOR_RGB2HSV_FULL: i32 = 67;
pub const COLOR_RGB2Lab: i32 = 45;
pub const COLOR_RGB2Luv: i32 = 51;
pub const COLOR_RGB2XYZ: i32 = 33;
pub const COLOR_RGB2YCrCb: i32 = 37;
pub const COLOR_RGB2YUV: i32 = 83;
pub const COLOR_RGB2YUV_I420: i32 = 127;
pub const COLOR_RGB2YUV_YV12: i32 = 131;
pub const COLOR_RGBA2BGR: i32 = 3;
pub const COLOR_RGBA2BGR555: i32 = 27;
pub const COLOR_RGBA2BGR565: i32 = 17;
pub const COLOR_RGBA2GRAY: i32 = 11;
pub const COLOR_RGBA2YUV_I420: i32 = 129;
pub const COLOR_RGBA2YUV_YV12: i32 = 133;
pub const COLOR_RGBA2mRGBA: i32 = 125;
pub const COLOR_XYZ2BGR: i32 = 34;
pub const COLOR_XYZ2RGB: i32 = 35;
pub const COLOR_YCrCb2BGR: i32 = 38;
pub const COLOR_YCrCb2RGB: i32 = 39;
pub const COLOR_YUV2BGR: i32 = 84;
pub const COLOR_YUV2BGRA_IYUV: i32 = 105;
pub const COLOR_YUV2BGRA_NV12: i32 = 95;
pub const COLOR_YUV2BGRA_NV21: i32 = 97;
pub const COLOR_YUV2BGRA_UYVY: i32 = 112;
pub const COLOR_YUV2BGRA_YUY2: i32 = 120;
pub const COLOR_YUV2BGRA_YV12: i32 = 103;
pub const COLOR_YUV2BGRA_YVYU: i32 = 122;
pub const COLOR_YUV2BGR_IYUV: i32 = 101;
pub const COLOR_YUV2BGR_NV12: i32 = 91;
pub const COLOR_YUV2BGR_NV21: i32 = 93;
pub const COLOR_YUV2BGR_UYVY: i32 = 108;
pub const COLOR_YUV2BGR_YUY2: i32 = 116;
pub const COLOR_YUV2BGR_YV12: i32 = 99;
pub const COLOR_YUV2BGR_YVYU: i32 = 118;
pub const COLOR_YUV2GRAY_420: i32 = 106;
pub const COLOR_YUV2GRAY_UYVY: i32 = 123;
pub const COLOR_YUV2GRAY_YUY2: i32 = 124;
pub const COLOR_YUV2RGB: i32 = 85;
pub const COLOR_YUV2RGBA_IYUV: i32 = 104;
pub const COLOR_YUV2RGBA_NV12: i32 = 94;
pub const COLOR_YUV2RGBA_NV21: i32 = 96;
pub const COLOR_YUV2RGBA_UYVY: i32 = 111;
pub const COLOR_YUV2RGBA_YUY2: i32 = 119;
pub const COLOR_YUV2RGBA_YV12: i32 = 102;
pub const COLOR_YUV2RGBA_YVYU: i32 = 121;
pub const COLOR_YUV2RGB_IYUV: i32 = 100;
pub const COLOR_YUV2RGB_NV12: i32 = 90;
pub const COLOR_YUV2RGB_NV21: i32 = 92;
pub const COLOR_YUV2RGB_UYVY: i32 = 107;
pub const COLOR_YUV2RGB_YUY2: i32 = 115;
pub const COLOR_YUV2RGB_YV12: i32 = 98;
pub const COLOR_YUV2RGB_YVYU: i32 = 117;
pub const COLOR_mRGBA2RGBA: i32 = 126;
pub const CONTOURS_MATCH_I1: i32 = 1;
pub const CONTOURS_MATCH_I2: i32 = 2;
pub const CONTOURS_MATCH_I3: i32 = 3;
pub const CV_AA: i32 = 16;
pub const CV_ADAPTIVE_THRESH_GAUSSIAN_C: i32 = 1;
pub const CV_ADAPTIVE_THRESH_MEAN_C: i32 = 0;
pub const CV_BGR2BGR555: i32 = 22;
pub const CV_BGR2BGR565: i32 = 12;
pub const CV_BGR2BGRA: i32 = 0;
pub const CV_BGR2GRAY: i32 = 6;
pub const CV_BGR2HLS: i32 = 52;
pub const CV_BGR2HLS_FULL: i32 = 68;
pub const CV_BGR2HSV: i32 = 40;
pub const CV_BGR2HSV_FULL: i32 = 66;
pub const CV_BGR2Lab: i32 = 44;
pub const CV_BGR2Luv: i32 = 50;
pub const CV_BGR2RGB: i32 = 4;
pub const CV_BGR2RGBA: i32 = 2;
pub const CV_BGR2XYZ: i32 = 32;
pub const CV_BGR2YCrCb: i32 = 36;
pub const CV_BGR2YUV: i32 = 82;
pub const CV_BGR2YUV_I420: i32 = 128;
pub const CV_BGR2YUV_YV12: i32 = 132;
pub const CV_BGR5552BGR: i32 = 24;
pub const CV_BGR5552BGRA: i32 = 28;
pub const CV_BGR5552GRAY: i32 = 31;
pub const CV_BGR5552RGB: i32 = 25;
pub const CV_BGR5552RGBA: i32 = 29;
pub const CV_BGR5652BGR: i32 = 14;
pub const CV_BGR5652BGRA: i32 = 18;
pub const CV_BGR5652GRAY: i32 = 21;
pub const CV_BGR5652RGB: i32 = 15;
pub const CV_BGR5652RGBA: i32 = 19;
pub const CV_BGRA2BGR: i32 = 1;
pub const CV_BGRA2BGR555: i32 = 26;
pub const CV_BGRA2BGR565: i32 = 16;
pub const CV_BGRA2GRAY: i32 = 10;
pub const CV_BGRA2RGBA: i32 = 5;
pub const CV_BGRA2YUV_I420: i32 = 130;
pub const CV_BGRA2YUV_YV12: i32 = 134;
pub const CV_BILATERAL: i32 = 4;
pub const CV_BLUR: i32 = 1;
pub const CV_BLUR_NO_SCALE: i32 = 0;
pub const CV_BayerBG2BGR: i32 = 46;
pub const CV_BayerBG2BGRA: i32 = 139;
pub const CV_BayerBG2BGR_EA: i32 = 135;
pub const CV_BayerBG2BGR_VNG: i32 = 62;
pub const CV_BayerBG2GRAY: i32 = 86;
pub const CV_BayerGB2BGR: i32 = 47;
pub const CV_BayerGB2BGRA: i32 = 140;
pub const CV_BayerGB2BGR_EA: i32 = 136;
pub const CV_BayerGB2BGR_VNG: i32 = 63;
pub const CV_BayerGB2GRAY: i32 = 87;
pub const CV_BayerGR2BGR: i32 = 49;
pub const CV_BayerGR2BGRA: i32 = 142;
pub const CV_BayerGR2BGR_EA: i32 = 138;
pub const CV_BayerGR2BGR_VNG: i32 = 65;
pub const CV_BayerGR2GRAY: i32 = 89;
pub const CV_BayerRG2BGR: i32 = 48;
pub const CV_BayerRG2BGRA: i32 = 141;
pub const CV_BayerRG2BGR_EA: i32 = 137;
pub const CV_BayerRG2BGR_VNG: i32 = 64;
pub const CV_BayerRG2GRAY: i32 = 88;
pub const CV_CHAIN_APPROX_NONE: i32 = 1;
pub const CV_CHAIN_APPROX_SIMPLE: i32 = 2;
pub const CV_CHAIN_APPROX_TC89_KCOS: i32 = 4;
pub const CV_CHAIN_APPROX_TC89_L1: i32 = 3;
pub const CV_CHAIN_CODE: i32 = 0;
pub const CV_CLOCKWISE: i32 = 1;
pub const CV_COLORCVT_MAX: i32 = 143;
pub const CV_COMP_BHATTACHARYYA: i32 = 3;
pub const CV_COMP_CHISQR: i32 = 1;
pub const CV_COMP_CHISQR_ALT: i32 = 4;
pub const CV_COMP_CORREL: i32 = 0;
pub const CV_COMP_INTERSECT: i32 = 2;
pub const CV_COMP_KL_DIV: i32 = 5;
pub const CV_CONTOURS_MATCH_I1: i32 = 1;
pub const CV_CONTOURS_MATCH_I2: i32 = 2;
pub const CV_CONTOURS_MATCH_I3: i32 = 3;
pub const CV_COUNTER_CLOCKWISE: i32 = 2;
pub const CV_DIST_C: i32 = 3;
pub const CV_DIST_FAIR: i32 = 5;
pub const CV_DIST_HUBER: i32 = 7;
pub const CV_DIST_L1: i32 = 1;
pub const CV_DIST_L12: i32 = 4;
pub const CV_DIST_L2: i32 = 2;
pub const CV_DIST_LABEL_CCOMP: i32 = 0;
pub const CV_DIST_LABEL_PIXEL: i32 = 1;
pub const CV_DIST_MASK_3: i32 = 3;
pub const CV_DIST_MASK_5: i32 = 5;
pub const CV_DIST_MASK_PRECISE: i32 = 0;
pub const CV_DIST_USER: i32 = -1;
pub const CV_DIST_WELSCH: i32 = 6;
pub const CV_FILLED: i32 = -1;
pub const CV_FONT_HERSHEY_COMPLEX: i32 = 3;
pub const CV_FONT_HERSHEY_COMPLEX_SMALL: i32 = 5;
pub const CV_FONT_HERSHEY_DUPLEX: i32 = 2;
pub const CV_FONT_HERSHEY_PLAIN: i32 = 1;
pub const CV_FONT_HERSHEY_SCRIPT_COMPLEX: i32 = 7;
pub const CV_FONT_HERSHEY_SCRIPT_SIMPLEX: i32 = 6;
pub const CV_FONT_HERSHEY_SIMPLEX: i32 = 0;
pub const CV_FONT_HERSHEY_TRIPLEX: i32 = 4;
pub const CV_FONT_ITALIC: i32 = 16;
pub const CV_GAUSSIAN: i32 = 2;
pub const CV_GAUSSIAN_5x5: i32 = 7;
pub const CV_GRAY2BGR: i32 = 8;
pub const CV_GRAY2BGR555: i32 = 30;
pub const CV_GRAY2BGR565: i32 = 20;
pub const CV_GRAY2BGRA: i32 = 9;
pub const CV_HAL_ADAPTIVE_THRESH_GAUSSIAN_C: i32 = 1;
pub const CV_HAL_ADAPTIVE_THRESH_MEAN_C: i32 = 0;
pub const CV_HAL_INTER_AREA: i32 = 3;
pub const CV_HAL_INTER_CUBIC: i32 = 2;
pub const CV_HAL_INTER_LANCZOS4: i32 = 4;
pub const CV_HAL_INTER_LINEAR: i32 = 1;
pub const CV_HAL_INTER_NEAREST: i32 = 0;
pub const CV_HAL_MORPH_DILATE: i32 = 1;
pub const CV_HAL_MORPH_ERODE: i32 = 0;
pub const CV_HAL_THRESH_BINARY: i32 = 0;
pub const CV_HAL_THRESH_BINARY_INV: i32 = 1;
pub const CV_HAL_THRESH_MASK: i32 = 7;
pub const CV_HAL_THRESH_OTSU: i32 = 8;
pub const CV_HAL_THRESH_TOZERO: i32 = 3;
pub const CV_HAL_THRESH_TOZERO_INV: i32 = 4;
pub const CV_HAL_THRESH_TRIANGLE: i32 = 16;
pub const CV_HAL_THRESH_TRUNC: i32 = 2;
pub const CV_HLS2BGR: i32 = 60;
pub const CV_HLS2BGR_FULL: i32 = 72;
pub const CV_HLS2RGB: i32 = 61;
pub const CV_HLS2RGB_FULL: i32 = 73;
pub const CV_HOUGH_GRADIENT: i32 = 3;
pub const CV_HOUGH_MULTI_SCALE: i32 = 2;
pub const CV_HOUGH_PROBABILISTIC: i32 = 1;
pub const CV_HOUGH_STANDARD: i32 = 0;
pub const CV_HSV2BGR: i32 = 54;
pub const CV_HSV2BGR_FULL: i32 = 70;
pub const CV_HSV2RGB: i32 = 55;
pub const CV_HSV2RGB_FULL: i32 = 71;
pub const CV_INTER_AREA: i32 = 3;
pub const CV_INTER_CUBIC: i32 = 2;
pub const CV_INTER_LANCZOS4: i32 = 4;
pub const CV_INTER_LINEAR: i32 = 1;
pub const CV_INTER_NN: i32 = 0;
pub const CV_LBGR2Lab: i32 = 74;
pub const CV_LBGR2Luv: i32 = 76;
pub const CV_LINK_RUNS: i32 = 5;
pub const CV_LRGB2Lab: i32 = 75;
pub const CV_LRGB2Luv: i32 = 77;
pub const CV_Lab2BGR: i32 = 56;
pub const CV_Lab2LBGR: i32 = 78;
pub const CV_Lab2LRGB: i32 = 79;
pub const CV_Lab2RGB: i32 = 57;
pub const CV_Luv2BGR: i32 = 58;
pub const CV_Luv2LBGR: i32 = 80;
pub const CV_Luv2LRGB: i32 = 81;
pub const CV_Luv2RGB: i32 = 59;
pub const CV_MAX_SOBEL_KSIZE: i32 = 7;
pub const CV_MEDIAN: i32 = 3;
pub const CV_MOP_BLACKHAT: i32 = 6;
pub const CV_MOP_CLOSE: i32 = 3;
pub const CV_MOP_DILATE: i32 = 1;
pub const CV_MOP_ERODE: i32 = 0;
pub const CV_MOP_GRADIENT: i32 = 4;
pub const CV_MOP_OPEN: i32 = 2;
pub const CV_MOP_TOPHAT: i32 = 5;
pub const CV_POLY_APPROX_DP: i32 = 0;
pub const CV_RETR_CCOMP: i32 = 2;
pub const CV_RETR_EXTERNAL: i32 = 0;
pub const CV_RETR_FLOODFILL: i32 = 4;
pub const CV_RETR_LIST: i32 = 1;
pub const CV_RETR_TREE: i32 = 3;
pub const CV_RGB2BGR555: i32 = 23;
pub const CV_RGB2BGR565: i32 = 13;
pub const CV_RGB2GRAY: i32 = 7;
pub const CV_RGB2HLS: i32 = 53;
pub const CV_RGB2HLS_FULL: i32 = 69;
pub const CV_RGB2HSV: i32 = 41;
pub const CV_RGB2HSV_FULL: i32 = 67;
pub const CV_RGB2Lab: i32 = 45;
pub const CV_RGB2Luv: i32 = 51;
pub const CV_RGB2XYZ: i32 = 33;
pub const CV_RGB2YCrCb: i32 = 37;
pub const CV_RGB2YUV: i32 = 83;
pub const CV_RGB2YUV_I420: i32 = 127;
pub const CV_RGB2YUV_YV12: i32 = 131;
pub const CV_RGBA2BGR: i32 = 3;
pub const CV_RGBA2BGR555: i32 = 27;
pub const CV_RGBA2BGR565: i32 = 17;
pub const CV_RGBA2GRAY: i32 = 11;
pub const CV_RGBA2YUV_I420: i32 = 129;
pub const CV_RGBA2YUV_YV12: i32 = 133;
pub const CV_RGBA2mRGBA: i32 = 125;
pub const CV_SCHARR: i32 = -1;
pub const CV_SHAPE_CROSS: i32 = 1;
pub const CV_SHAPE_CUSTOM: i32 = 100;
pub const CV_SHAPE_ELLIPSE: i32 = 2;
pub const CV_SHAPE_RECT: i32 = 0;
pub const CV_THRESH_BINARY: i32 = 0;
pub const CV_THRESH_BINARY_INV: i32 = 1;
pub const CV_THRESH_MASK: i32 = 7;
pub const CV_THRESH_OTSU: i32 = 8;
pub const CV_THRESH_TOZERO: i32 = 3;
pub const CV_THRESH_TOZERO_INV: i32 = 4;
pub const CV_THRESH_TRIANGLE: i32 = 16;
pub const CV_THRESH_TRUNC: i32 = 2;
pub const CV_TM_CCOEFF: i32 = 4;
pub const CV_TM_CCOEFF_NORMED: i32 = 5;
pub const CV_TM_CCORR: i32 = 2;
pub const CV_TM_CCORR_NORMED: i32 = 3;
pub const CV_TM_SQDIFF: i32 = 0;
pub const CV_TM_SQDIFF_NORMED: i32 = 1;
pub const CV_WARP_FILL_OUTLIERS: i32 = 8;
pub const CV_WARP_INVERSE_MAP: i32 = 16;
pub const CV_XYZ2BGR: i32 = 34;
pub const CV_XYZ2RGB: i32 = 35;
pub const CV_YCrCb2BGR: i32 = 38;
pub const CV_YCrCb2RGB: i32 = 39;
pub const CV_YUV2BGR: i32 = 84;
pub const CV_YUV2BGRA_IYUV: i32 = 105;
pub const CV_YUV2BGRA_NV12: i32 = 95;
pub const CV_YUV2BGRA_NV21: i32 = 97;
pub const CV_YUV2BGRA_UYVY: i32 = 112;
pub const CV_YUV2BGRA_YUY2: i32 = 120;
pub const CV_YUV2BGRA_YV12: i32 = 103;
pub const CV_YUV2BGRA_YVYU: i32 = 122;
pub const CV_YUV2BGR_IYUV: i32 = 101;
pub const CV_YUV2BGR_NV12: i32 = 91;
pub const CV_YUV2BGR_NV21: i32 = 93;
pub const CV_YUV2BGR_UYVY: i32 = 108;
pub const CV_YUV2BGR_YUY2: i32 = 116;
pub const CV_YUV2BGR_YV12: i32 = 99;
pub const CV_YUV2BGR_YVYU: i32 = 118;
pub const CV_YUV2GRAY_420: i32 = 106;
pub const CV_YUV2GRAY_UYVY: i32 = 123;
pub const CV_YUV2GRAY_YUY2: i32 = 124;
pub const CV_YUV2RGB: i32 = 85;
pub const CV_YUV2RGBA_IYUV: i32 = 104;
pub const CV_YUV2RGBA_NV12: i32 = 94;
pub const CV_YUV2RGBA_NV21: i32 = 96;
pub const CV_YUV2RGBA_UYVY: i32 = 111;
pub const CV_YUV2RGBA_YUY2: i32 = 119;
pub const CV_YUV2RGBA_YV12: i32 = 102;
pub const CV_YUV2RGBA_YVYU: i32 = 121;
pub const CV_YUV2RGB_IYUV: i32 = 100;
pub const CV_YUV2RGB_NV12: i32 = 90;
pub const CV_YUV2RGB_NV21: i32 = 92;
pub const CV_YUV2RGB_UYVY: i32 = 107;
pub const CV_YUV2RGB_YUY2: i32 = 115;
pub const CV_YUV2RGB_YV12: i32 = 98;
pub const CV_YUV2RGB_YVYU: i32 = 117;
pub const CV_mRGBA2RGBA: i32 = 126;
pub const DIST_C: i32 = 3;
pub const DIST_FAIR: i32 = 5;
pub const DIST_HUBER: i32 = 7;
pub const DIST_L1: i32 = 1;
pub const DIST_L12: i32 = 4;
pub const DIST_L2: i32 = 2;
pub const DIST_LABEL_CCOMP: i32 = 0;
pub const DIST_LABEL_PIXEL: i32 = 1;
pub const DIST_MASK_3: i32 = 3;
pub const DIST_MASK_5: i32 = 5;
pub const DIST_MASK_PRECISE: i32 = 0;
pub const DIST_USER: i32 = -1;
pub const DIST_WELSCH: i32 = 6;
pub const GC_BGD: i32 = 0;
pub const GC_EVAL: i32 = 2;
pub const GC_EVAL_FREEZE_MODEL: i32 = 3;
pub const GC_FGD: i32 = 1;
pub const GC_INIT_WITH_MASK: i32 = 1;
pub const GC_INIT_WITH_RECT: i32 = 0;
pub const GC_PR_BGD: i32 = 2;
pub const GC_PR_FGD: i32 = 3;
pub const HISTCMP_BHATTACHARYYA: i32 = 3;
pub const HISTCMP_CHISQR: i32 = 1;
pub const HISTCMP_CHISQR_ALT: i32 = 4;
pub const HISTCMP_CORREL: i32 = 0;
pub const HISTCMP_INTERSECT: i32 = 2;
pub const HISTCMP_KL_DIV: i32 = 5;
pub const HOUGH_GRADIENT: i32 = 3;
pub const HOUGH_MULTI_SCALE: i32 = 2;
pub const HOUGH_PROBABILISTIC: i32 = 1;
pub const HOUGH_STANDARD: i32 = 0;
pub const INTERSECT_FULL: i32 = 2;
pub const INTERSECT_NONE: i32 = 0;
pub const INTERSECT_PARTIAL: i32 = 1;
pub const INTER_AREA: i32 = 3;
pub const INTER_BITS: i32 = 5;
pub const INTER_CUBIC: i32 = 2;
pub const INTER_LANCZOS4: i32 = 4;
pub const INTER_LINEAR: i32 = 1;
pub const INTER_LINEAR_EXACT: i32 = 5;
pub const INTER_MAX: i32 = 7;
pub const INTER_NEAREST: i32 = 0;
pub const LSD_REFINE_ADV: i32 = 2;
pub const LSD_REFINE_NONE: i32 = 0;
pub const LSD_REFINE_STD: i32 = 1;
pub const MARKER_CROSS: i32 = 0;
pub const MARKER_DIAMOND: i32 = 3;
pub const MARKER_SQUARE: i32 = 4;
pub const MARKER_STAR: i32 = 2;
pub const MARKER_TILTED_CROSS: i32 = 1;
pub const MARKER_TRIANGLE_DOWN: i32 = 6;
pub const MARKER_TRIANGLE_UP: i32 = 5;
pub const MORPH_BLACKHAT: i32 = 6;
pub const MORPH_CLOSE: i32 = 3;
pub const MORPH_CROSS: i32 = 1;
pub const MORPH_DILATE: i32 = 1;
pub const MORPH_ELLIPSE: i32 = 2;
pub const MORPH_ERODE: i32 = 0;
pub const MORPH_GRADIENT: i32 = 4;
pub const MORPH_HITMISS: i32 = 7;
pub const MORPH_OPEN: i32 = 2;
pub const MORPH_RECT: i32 = 0;
pub const MORPH_TOPHAT: i32 = 5;
pub const PROJ_SPHERICAL_EQRECT: i32 = 1;
pub const PROJ_SPHERICAL_ORTHO: i32 = 0;
pub const RETR_CCOMP: i32 = 2;
pub const RETR_EXTERNAL: i32 = 0;
pub const RETR_FLOODFILL: i32 = 4;
pub const RETR_LIST: i32 = 1;
pub const RETR_TREE: i32 = 3;
pub const Subdiv2D_NEXT_AROUND_DST: i32 = 0x22;
pub const Subdiv2D_NEXT_AROUND_LEFT: i32 = 0x13;
pub const Subdiv2D_NEXT_AROUND_ORG: i32 = 0x00;
pub const Subdiv2D_NEXT_AROUND_RIGHT: i32 = 0x31;
pub const Subdiv2D_PREV_AROUND_DST: i32 = 0x33;
pub const Subdiv2D_PREV_AROUND_LEFT: i32 = 0x20;
pub const Subdiv2D_PREV_AROUND_ORG: i32 = 0x11;
pub const Subdiv2D_PREV_AROUND_RIGHT: i32 = 0x02;
pub const Subdiv2D_PTLOC_ERROR: i32 = -2;
pub const Subdiv2D_PTLOC_INSIDE: i32 = 0;
pub const Subdiv2D_PTLOC_ON_EDGE: i32 = 2;
pub const Subdiv2D_PTLOC_OUTSIDE_RECT: i32 = -1;
pub const Subdiv2D_PTLOC_VERTEX: i32 = 1;
pub const THRESH_BINARY: i32 = 0;
pub const THRESH_BINARY_INV: i32 = 1;
pub const THRESH_MASK: i32 = 7;
pub const THRESH_OTSU: i32 = 8;
pub const THRESH_TOZERO: i32 = 3;
pub const THRESH_TOZERO_INV: i32 = 4;
pub const THRESH_TRIANGLE: i32 = 16;
pub const THRESH_TRUNC: i32 = 2;
pub const TM_CCOEFF: i32 = 4;
pub const TM_CCOEFF_NORMED: i32 = 5;
pub const TM_CCORR: i32 = 2;
pub const TM_CCORR_NORMED: i32 = 3;
pub const TM_SQDIFF: i32 = 0;
pub const TM_SQDIFF_NORMED: i32 = 1;
pub const WARP_FILL_OUTLIERS: i32 = 8;
pub const WARP_INVERSE_MAP: i32 = 16;
pub const WARP_POLAR_LINEAR: i32 = 0;
pub const WARP_POLAR_LOG: i32 = 256;

/// \overload
/// 
/// Finds edges in an image using the Canny algorithm with custom image gradient.
/// 
/// ## Parameters
/// * dx: 16-bit x derivative of input image (CV_16SC1 or CV_16SC3).
/// * dy: 16-bit y derivative of input image (same type as dx).
/// * edges: output edge map; single channels 8-bit image, which has the same size as image .
/// * threshold1: first threshold for the hysteresis procedure.
/// * threshold2: second threshold for the hysteresis procedure.
/// * L2gradient: a flag, indicating whether a more accurate <span lang='latex'>L_2</span> norm
/// <span lang='latex'>=\sqrt{(dI/dx)^2 + (dI/dy)^2}</span> should be used to calculate the image gradient magnitude (
/// L2gradient=true ), or whether the default <span lang='latex'>L_1</span> norm <span lang='latex'>=|dI/dx|+|dI/dy|</span> is enough (
/// L2gradient=false ).
///
/// ## C++ default parameters:
/// * l2gradient: false
pub fn canny_derivative(dx: &core::Mat, dy: &core::Mat, edges: &mut core::Mat, threshold1: f64, threshold2: f64, l2gradient: bool) -> Result<()> {
// identifier: cv_Canny_Mat_dx_Mat_dy_Mat_edges_double_threshold1_double_threshold2_bool_L2gradient
  unsafe {
    let rv = sys::cv_imgproc_cv_Canny_Mat_dx_Mat_dy_Mat_edges_double_threshold1_double_threshold2_bool_L2gradient(dx.as_raw_Mat(), dy.as_raw_Mat(), edges.as_raw_Mat(), threshold1, threshold2, l2gradient);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds edges in an image using the Canny algorithm @cite Canny86 .
/// 
/// The function finds edges in the input image and marks them in the output map edges using the
/// Canny algorithm. The smallest value between threshold1 and threshold2 is used for edge linking. The
/// largest value is used to find initial segments of strong edges. See
/// <http://en.wikipedia.org/wiki/Canny_edge_detector>
/// 
/// ## Parameters
/// * image: 8-bit input image.
/// * edges: output edge map; single channels 8-bit image, which has the same size as image .
/// * threshold1: first threshold for the hysteresis procedure.
/// * threshold2: second threshold for the hysteresis procedure.
/// * apertureSize: aperture size for the Sobel operator.
/// * L2gradient: a flag, indicating whether a more accurate <span lang='latex'>L_2</span> norm
/// <span lang='latex'>=\sqrt{(dI/dx)^2 + (dI/dy)^2}</span> should be used to calculate the image gradient magnitude (
/// L2gradient=true ), or whether the default <span lang='latex'>L_1</span> norm <span lang='latex'>=|dI/dx|+|dI/dy|</span> is enough (
/// L2gradient=false ).
///
/// ## C++ default parameters:
/// * aperture_size: 3
/// * l2gradient: false
pub fn canny(image: &core::Mat, edges: &mut core::Mat, threshold1: f64, threshold2: f64, aperture_size: i32, l2gradient: bool) -> Result<()> {
// identifier: cv_Canny_Mat_image_Mat_edges_double_threshold1_double_threshold2_int_apertureSize_bool_L2gradient
  unsafe {
    let rv = sys::cv_imgproc_cv_Canny_Mat_image_Mat_edges_double_threshold1_double_threshold2_int_apertureSize_bool_L2gradient(image.as_raw_Mat(), edges.as_raw_Mat(), threshold1, threshold2, aperture_size, l2gradient);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Blurs an image using a Gaussian filter.
/// 
/// The function convolves the source image with the specified Gaussian kernel. In-place filtering is
/// supported.
/// 
/// ## Parameters
/// * src: input image; the image can have any number of channels, which are processed
/// independently, but the depth should be CV_8U, CV_16U, CV_16S, CV_32F or CV_64F.
/// * dst: output image of the same size and type as src.
/// * ksize: Gaussian kernel size. ksize.width and ksize.height can differ but they both must be
/// positive and odd. Or, they can be zero's and then they are computed from sigma.
/// * sigmaX: Gaussian kernel standard deviation in X direction.
/// * sigmaY: Gaussian kernel standard deviation in Y direction; if sigmaY is zero, it is set to be
/// equal to sigmaX, if both sigmas are zeros, they are computed from ksize.width and ksize.height,
/// respectively (see #getGaussianKernel for details); to fully control the result regardless of
/// possible future modifications of all this semantics, it is recommended to specify all of ksize,
/// sigmaX, and sigmaY.
/// * borderType: pixel extrapolation method, see #BorderTypes
/// 
/// @sa  sepFilter2D, filter2D, blur, boxFilter, bilateralFilter, medianBlur
///
/// ## C++ default parameters:
/// * sigma_y: 0
/// * border_type: BORDER_DEFAULT
pub fn gaussian_blur(src: &core::Mat, dst: &mut core::Mat, ksize: core::Size, sigma_x: f64, sigma_y: f64, border_type: i32) -> Result<()> {
// identifier: cv_GaussianBlur_Mat_src_Mat_dst_Size_ksize_double_sigmaX_double_sigmaY_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_GaussianBlur_Mat_src_Mat_dst_Size_ksize_double_sigmaX_double_sigmaY_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), ksize, sigma_x, sigma_y, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds circles in a grayscale image using the Hough transform.
/// 
/// The function finds circles in a grayscale image using a modification of the Hough transform.
/// 
/// Example: :
/// @include snippets/imgproc_HoughLinesCircles.cpp
/// 
/// 
/// Note: Usually the function detects the centers of circles well. However, it may fail to find correct
/// radii. You can assist to the function by specifying the radius range ( minRadius and maxRadius ) if
/// you know it. Or, you may set maxRadius to a negative number to return centers only without radius
/// search, and find the correct radius using an additional procedure.
/// 
/// ## Parameters
/// * image: 8-bit, single-channel, grayscale input image.
/// * circles: Output vector of found circles. Each vector is encoded as  3 or 4 element
/// floating-point vector <span lang='latex'>(x, y, radius)</span> or <span lang='latex'>(x, y, radius, votes)</span> .
/// * method: Detection method, see #HoughModes. Currently, the only implemented method is #HOUGH_GRADIENT
/// * dp: Inverse ratio of the accumulator resolution to the image resolution. For example, if
/// dp=1 , the accumulator has the same resolution as the input image. If dp=2 , the accumulator has
/// half as big width and height.
/// * minDist: Minimum distance between the centers of the detected circles. If the parameter is
/// too small, multiple neighbor circles may be falsely detected in addition to a true one. If it is
/// too large, some circles may be missed.
/// * param1: First method-specific parameter. In case of #HOUGH_GRADIENT , it is the higher
/// threshold of the two passed to the Canny edge detector (the lower one is twice smaller).
/// * param2: Second method-specific parameter. In case of #HOUGH_GRADIENT , it is the
/// accumulator threshold for the circle centers at the detection stage. The smaller it is, the more
/// false circles may be detected. Circles, corresponding to the larger accumulator values, will be
/// returned first.
/// * minRadius: Minimum circle radius.
/// * maxRadius: Maximum circle radius. If <= 0, uses the maximum image dimension. If < 0, returns
/// centers without finding the radius.
/// 
/// @sa fitEllipse, minEnclosingCircle
///
/// ## C++ default parameters:
/// * param1: 100
/// * param2: 100
/// * min_radius: 0
/// * max_radius: 0
pub fn hough_circles(image: &core::Mat, circles: &mut core::Mat, method: i32, dp: f64, min_dist: f64, param1: f64, param2: f64, min_radius: i32, max_radius: i32) -> Result<()> {
// identifier: cv_HoughCircles_Mat_image_Mat_circles_int_method_double_dp_double_minDist_double_param1_double_param2_int_minRadius_int_maxRadius
  unsafe {
    let rv = sys::cv_imgproc_cv_HoughCircles_Mat_image_Mat_circles_int_method_double_dp_double_minDist_double_param1_double_param2_int_minRadius_int_maxRadius(image.as_raw_Mat(), circles.as_raw_Mat(), method, dp, min_dist, param1, param2, min_radius, max_radius);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds line segments in a binary image using the probabilistic Hough transform.
/// 
/// The function implements the probabilistic Hough transform algorithm for line detection, described
/// in @cite Matas00
/// 
/// See the line detection example below:
/// @include snippets/imgproc_HoughLinesP.cpp
/// This is a sample picture the function parameters have been tuned for:
/// 
/// ![image](pics/building.jpg)
/// 
/// And this is the output of the above program in case of the probabilistic Hough transform:
/// 
/// ![image](pics/houghp.png)
/// 
/// ## Parameters
/// * image: 8-bit, single-channel binary source image. The image may be modified by the function.
/// * lines: Output vector of lines. Each line is represented by a 4-element vector
/// <span lang='latex'>(x_1, y_1, x_2, y_2)</span> , where <span lang='latex'>(x_1,y_1)</span> and <span lang='latex'>(x_2, y_2)</span> are the ending points of each detected
/// line segment.
/// * rho: Distance resolution of the accumulator in pixels.
/// * theta: Angle resolution of the accumulator in radians.
/// * threshold: Accumulator threshold parameter. Only those lines are returned that get enough
/// votes ( <span lang='latex'>>\texttt{threshold}</span> ).
/// * minLineLength: Minimum line length. Line segments shorter than that are rejected.
/// * maxLineGap: Maximum allowed gap between points on the same line to link them.
/// 
/// @sa LineSegmentDetector
///
/// ## C++ default parameters:
/// * min_line_length: 0
/// * max_line_gap: 0
pub fn hough_lines_p(image: &core::Mat, lines: &mut core::Mat, rho: f64, theta: f64, threshold: i32, min_line_length: f64, max_line_gap: f64) -> Result<()> {
// identifier: cv_HoughLinesP_Mat_image_Mat_lines_double_rho_double_theta_int_threshold_double_minLineLength_double_maxLineGap
  unsafe {
    let rv = sys::cv_imgproc_cv_HoughLinesP_Mat_image_Mat_lines_double_rho_double_theta_int_threshold_double_minLineLength_double_maxLineGap(image.as_raw_Mat(), lines.as_raw_Mat(), rho, theta, threshold, min_line_length, max_line_gap);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds lines in a set of points using the standard Hough transform.
/// 
/// The function finds lines in a set of points using a modification of the Hough transform.
/// @include snippets/imgproc_HoughLinesPointSet.cpp
/// ## Parameters
/// * _point: Input vector of points. Each vector must be encoded as a Point vector <span lang='latex'>(x,y)</span>. Type must be CV_32FC2 or CV_32SC2.
/// * _lines: Output vector of found lines. Each vector is encoded as a vector<Vec3d> <span lang='latex'>(votes, rho, theta)</span>.
/// The larger the value of 'votes', the higher the reliability of the Hough line.
/// * lines_max: Max count of hough lines.
/// * threshold: Accumulator threshold parameter. Only those lines are returned that get enough
/// votes ( <span lang='latex'>>\texttt{threshold}</span> )
/// * min_rho: Minimum Distance value of the accumulator in pixels.
/// * max_rho: Maximum Distance value of the accumulator in pixels.
/// * rho_step: Distance resolution of the accumulator in pixels.
/// * min_theta: Minimum angle value of the accumulator in radians.
/// * max_theta: Maximum angle value of the accumulator in radians.
/// * theta_step: Angle resolution of the accumulator in radians.
pub fn hough_lines_point_set(_point: &core::Mat, _lines: &mut core::Mat, lines_max: i32, threshold: i32, min_rho: f64, max_rho: f64, rho_step: f64, min_theta: f64, max_theta: f64, theta_step: f64) -> Result<()> {
// identifier: cv_HoughLinesPointSet_Mat__point_Mat__lines_int_lines_max_int_threshold_double_min_rho_double_max_rho_double_rho_step_double_min_theta_double_max_theta_double_theta_step
  unsafe {
    let rv = sys::cv_imgproc_cv_HoughLinesPointSet_Mat__point_Mat__lines_int_lines_max_int_threshold_double_min_rho_double_max_rho_double_rho_step_double_min_theta_double_max_theta_double_theta_step(_point.as_raw_Mat(), _lines.as_raw_Mat(), lines_max, threshold, min_rho, max_rho, rho_step, min_theta, max_theta, theta_step);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds lines in a binary image using the standard Hough transform.
/// 
/// The function implements the standard or standard multi-scale Hough transform algorithm for line
/// detection. See <http://homepages.inf.ed.ac.uk/rbf/HIPR2/hough.htm> for a good explanation of Hough
/// transform.
/// 
/// ## Parameters
/// * image: 8-bit, single-channel binary source image. The image may be modified by the function.
/// * lines: Output vector of lines. Each line is represented by a 2 or 3 element vector
/// <span lang='latex'>(\rho, \theta)</span> or <span lang='latex'>(\rho, \theta, \textrm{votes})</span> . <span lang='latex'>\rho</span> is the distance from the coordinate origin <span lang='latex'>(0,0)</span> (top-left corner of
/// the image). <span lang='latex'>\theta</span> is the line rotation angle in radians (
/// <span lang='latex'>0 \sim \textrm{vertical line}, \pi/2 \sim \textrm{horizontal line}</span> ).
/// <span lang='latex'>\textrm{votes}</span> is the value of accumulator.
/// * rho: Distance resolution of the accumulator in pixels.
/// * theta: Angle resolution of the accumulator in radians.
/// * threshold: Accumulator threshold parameter. Only those lines are returned that get enough
/// votes ( <span lang='latex'>>\texttt{threshold}</span> ).
/// * srn: For the multi-scale Hough transform, it is a divisor for the distance resolution rho .
/// The coarse accumulator distance resolution is rho and the accurate accumulator resolution is
/// rho/srn . If both srn=0 and stn=0 , the classical Hough transform is used. Otherwise, both these
/// parameters should be positive.
/// * stn: For the multi-scale Hough transform, it is a divisor for the distance resolution theta.
/// * min_theta: For standard and multi-scale Hough transform, minimum angle to check for lines.
/// Must fall between 0 and max_theta.
/// * max_theta: For standard and multi-scale Hough transform, maximum angle to check for lines.
/// Must fall between min_theta and CV_PI.
///
/// ## C++ default parameters:
/// * srn: 0
/// * stn: 0
/// * min_theta: 0
/// * max_theta: CV_PI
pub fn hough_lines(image: &core::Mat, lines: &mut core::Mat, rho: f64, theta: f64, threshold: i32, srn: f64, stn: f64, min_theta: f64, max_theta: f64) -> Result<()> {
// identifier: cv_HoughLines_Mat_image_Mat_lines_double_rho_double_theta_int_threshold_double_srn_double_stn_double_min_theta_double_max_theta
  unsafe {
    let rv = sys::cv_imgproc_cv_HoughLines_Mat_image_Mat_lines_double_rho_double_theta_int_threshold_double_srn_double_stn_double_min_theta_double_max_theta(image.as_raw_Mat(), lines.as_raw_Mat(), rho, theta, threshold, srn, stn, min_theta, max_theta);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates the Laplacian of an image.
/// 
/// The function calculates the Laplacian of the source image by adding up the second x and y
/// derivatives calculated using the Sobel operator:
/// 
/// <div lang='latex'>\texttt{dst} =  \Delta \texttt{src} =  \frac{\partial^2 \texttt{src}}{\partial x^2} +  \frac{\partial^2 \texttt{src}}{\partial y^2}</div>
/// 
/// This is done when `ksize > 1`. When `ksize == 1`, the Laplacian is computed by filtering the image
/// with the following <span lang='latex'>3 \times 3</span> aperture:
/// 
/// <div lang='latex'>\vecthreethree {0}{1}{0}{1}{-4}{1}{0}{1}{0}</div>
/// 
/// ## Parameters
/// * src: Source image.
/// * dst: Destination image of the same size and the same number of channels as src .
/// * ddepth: Desired depth of the destination image.
/// * ksize: Aperture size used to compute the second-derivative filters. See #getDerivKernels for
/// details. The size must be positive and odd.
/// * scale: Optional scale factor for the computed Laplacian values. By default, no scaling is
/// applied. See #getDerivKernels for details.
/// * delta: Optional delta value that is added to the results prior to storing them in dst .
/// * borderType: Pixel extrapolation method, see #BorderTypes
/// @sa  Sobel, Scharr
///
/// ## C++ default parameters:
/// * ksize: 1
/// * scale: 1
/// * delta: 0
/// * border_type: BORDER_DEFAULT
pub fn laplacian(src: &core::Mat, dst: &mut core::Mat, ddepth: i32, ksize: i32, scale: f64, delta: f64, border_type: i32) -> Result<()> {
// identifier: cv_Laplacian_Mat_src_Mat_dst_int_ddepth_int_ksize_double_scale_double_delta_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_Laplacian_Mat_src_Mat_dst_int_ddepth_int_ksize_double_scale_double_delta_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), ddepth, ksize, scale, delta, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates the first x- or y- image derivative using Scharr operator.
/// 
/// The function computes the first x- or y- spatial image derivative using the Scharr operator. The
/// call
/// 
/// <div lang='latex'>\texttt{Scharr(src, dst, ddepth, dx, dy, scale, delta, borderType)}</div>
/// 
/// is equivalent to
/// 
/// <div lang='latex'>\texttt{Sobel(src, dst, ddepth, dx, dy, CV_SCHARR, scale, delta, borderType)} .</div>
/// 
/// ## Parameters
/// * src: input image.
/// * dst: output image of the same size and the same number of channels as src.
/// * ddepth: output image depth, see @ref filter_depths "combinations"
/// * dx: order of the derivative x.
/// * dy: order of the derivative y.
/// * scale: optional scale factor for the computed derivative values; by default, no scaling is
/// applied (see #getDerivKernels for details).
/// * delta: optional delta value that is added to the results prior to storing them in dst.
/// * borderType: pixel extrapolation method, see #BorderTypes
/// @sa  cartToPolar
///
/// ## C++ default parameters:
/// * scale: 1
/// * delta: 0
/// * border_type: BORDER_DEFAULT
pub fn scharr(src: &core::Mat, dst: &mut core::Mat, ddepth: i32, dx: i32, dy: i32, scale: f64, delta: f64, border_type: i32) -> Result<()> {
// identifier: cv_Scharr_Mat_src_Mat_dst_int_ddepth_int_dx_int_dy_double_scale_double_delta_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_Scharr_Mat_src_Mat_dst_int_ddepth_int_dx_int_dy_double_scale_double_delta_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), ddepth, dx, dy, scale, delta, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates the first, second, third, or mixed image derivatives using an extended Sobel operator.
/// 
/// In all cases except one, the <span lang='latex'>\texttt{ksize} \times \texttt{ksize}</span> separable kernel is used to
/// calculate the derivative. When <span lang='latex'>\texttt{ksize = 1}</span>, the <span lang='latex'>3 \times 1</span> or <span lang='latex'>1 \times 3</span>
/// kernel is used (that is, no Gaussian smoothing is done). `ksize = 1` can only be used for the first
/// or the second x- or y- derivatives.
/// 
/// There is also the special value `ksize = #CV_SCHARR (-1)` that corresponds to the <span lang='latex'>3\times3</span> Scharr
/// filter that may give more accurate results than the <span lang='latex'>3\times3</span> Sobel. The Scharr aperture is
/// 
/// <div lang='latex'>\vecthreethree{-3}{0}{3}{-10}{0}{10}{-3}{0}{3}</div>
/// 
/// for the x-derivative, or transposed for the y-derivative.
/// 
/// The function calculates an image derivative by convolving the image with the appropriate kernel:
/// 
/// <div lang='latex'>\texttt{dst} =  \frac{\partial^{xorder+yorder} \texttt{src}}{\partial x^{xorder} \partial y^{yorder}}</div>
/// 
/// The Sobel operators combine Gaussian smoothing and differentiation, so the result is more or less
/// resistant to the noise. Most often, the function is called with ( xorder = 1, yorder = 0, ksize = 3)
/// or ( xorder = 0, yorder = 1, ksize = 3) to calculate the first x- or y- image derivative. The first
/// case corresponds to a kernel of:
/// 
/// <div lang='latex'>\vecthreethree{-1}{0}{1}{-2}{0}{2}{-1}{0}{1}</div>
/// 
/// The second case corresponds to a kernel of:
/// 
/// <div lang='latex'>\vecthreethree{-1}{-2}{-1}{0}{0}{0}{1}{2}{1}</div>
/// 
/// ## Parameters
/// * src: input image.
/// * dst: output image of the same size and the same number of channels as src .
/// * ddepth: output image depth, see @ref filter_depths "combinations"; in the case of
/// 8-bit input images it will result in truncated derivatives.
/// * dx: order of the derivative x.
/// * dy: order of the derivative y.
/// * ksize: size of the extended Sobel kernel; it must be 1, 3, 5, or 7.
/// * scale: optional scale factor for the computed derivative values; by default, no scaling is
/// applied (see #getDerivKernels for details).
/// * delta: optional delta value that is added to the results prior to storing them in dst.
/// * borderType: pixel extrapolation method, see #BorderTypes
/// @sa  Scharr, Laplacian, sepFilter2D, filter2D, GaussianBlur, cartToPolar
///
/// ## C++ default parameters:
/// * ksize: 3
/// * scale: 1
/// * delta: 0
/// * border_type: BORDER_DEFAULT
pub fn sobel(src: &core::Mat, dst: &mut core::Mat, ddepth: i32, dx: i32, dy: i32, ksize: i32, scale: f64, delta: f64, border_type: i32) -> Result<()> {
// identifier: cv_Sobel_Mat_src_Mat_dst_int_ddepth_int_dx_int_dy_int_ksize_double_scale_double_delta_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_Sobel_Mat_src_Mat_dst_int_ddepth_int_dx_int_dy_int_ksize_double_scale_double_delta_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), ddepth, dx, dy, ksize, scale, delta, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Adds the per-element product of two input images to the accumulator image.
/// 
/// The function adds the product of two images or their selected regions to the accumulator dst :
/// 
/// <div lang='latex'>\texttt{dst} (x,y)  \leftarrow \texttt{dst} (x,y) +  \texttt{src1} (x,y)  \cdot \texttt{src2} (x,y)  \quad \text{if} \quad \texttt{mask} (x,y)  \ne 0</div>
/// 
/// The function supports multi-channel images. Each channel is processed independently.
/// 
/// ## Parameters
/// * src1: First input image, 1- or 3-channel, 8-bit or 32-bit floating point.
/// * src2: Second input image of the same type and the same size as src1 .
/// * dst: %Accumulator image with the same number of channels as input images, 32-bit or 64-bit
/// floating-point.
/// * mask: Optional operation mask.
/// 
/// @sa  accumulate, accumulateSquare, accumulateWeighted
///
/// ## C++ default parameters:
/// * mask: noArray()
pub fn accumulate_product(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat, mask: &core::Mat) -> Result<()> {
// identifier: cv_accumulateProduct_Mat_src1_Mat_src2_Mat_dst_Mat_mask
  unsafe {
    let rv = sys::cv_imgproc_cv_accumulateProduct_Mat_src1_Mat_src2_Mat_dst_Mat_mask(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat(), mask.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Adds the square of a source image to the accumulator image.
/// 
/// The function adds the input image src or its selected region, raised to a power of 2, to the
/// accumulator dst :
/// 
/// <div lang='latex'>\texttt{dst} (x,y)  \leftarrow \texttt{dst} (x,y) +  \texttt{src} (x,y)^2  \quad \text{if} \quad \texttt{mask} (x,y)  \ne 0</div>
/// 
/// The function supports multi-channel images. Each channel is processed independently.
/// 
/// ## Parameters
/// * src: Input image as 1- or 3-channel, 8-bit or 32-bit floating point.
/// * dst: %Accumulator image with the same number of channels as input image, 32-bit or 64-bit
/// floating-point.
/// * mask: Optional operation mask.
/// 
/// @sa  accumulateSquare, accumulateProduct, accumulateWeighted
///
/// ## C++ default parameters:
/// * mask: noArray()
pub fn accumulate_square(src: &core::Mat, dst: &mut core::Mat, mask: &core::Mat) -> Result<()> {
// identifier: cv_accumulateSquare_Mat_src_Mat_dst_Mat_mask
  unsafe {
    let rv = sys::cv_imgproc_cv_accumulateSquare_Mat_src_Mat_dst_Mat_mask(src.as_raw_Mat(), dst.as_raw_Mat(), mask.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Updates a running average.
/// 
/// The function calculates the weighted sum of the input image src and the accumulator dst so that dst
/// becomes a running average of a frame sequence:
/// 
/// <div lang='latex'>\texttt{dst} (x,y)  \leftarrow (1- \texttt{alpha} )  \cdot \texttt{dst} (x,y) +  \texttt{alpha} \cdot \texttt{src} (x,y)  \quad \text{if} \quad \texttt{mask} (x,y)  \ne 0</div>
/// 
/// That is, alpha regulates the update speed (how fast the accumulator "forgets" about earlier images).
/// The function supports multi-channel images. Each channel is processed independently.
/// 
/// ## Parameters
/// * src: Input image as 1- or 3-channel, 8-bit or 32-bit floating point.
/// * dst: %Accumulator image with the same number of channels as input image, 32-bit or 64-bit
/// floating-point.
/// * alpha: Weight of the input image.
/// * mask: Optional operation mask.
/// 
/// @sa  accumulate, accumulateSquare, accumulateProduct
///
/// ## C++ default parameters:
/// * mask: noArray()
pub fn accumulate_weighted(src: &core::Mat, dst: &mut core::Mat, alpha: f64, mask: &core::Mat) -> Result<()> {
// identifier: cv_accumulateWeighted_Mat_src_Mat_dst_double_alpha_Mat_mask
  unsafe {
    let rv = sys::cv_imgproc_cv_accumulateWeighted_Mat_src_Mat_dst_double_alpha_Mat_mask(src.as_raw_Mat(), dst.as_raw_Mat(), alpha, mask.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Adds an image to the accumulator image.
/// 
/// The function adds src or some of its elements to dst :
/// 
/// <div lang='latex'>\texttt{dst} (x,y)  \leftarrow \texttt{dst} (x,y) +  \texttt{src} (x,y)  \quad \text{if} \quad \texttt{mask} (x,y)  \ne 0</div>
/// 
/// The function supports multi-channel images. Each channel is processed independently.
/// 
/// The function cv::accumulate can be used, for example, to collect statistics of a scene background
/// viewed by a still camera and for the further foreground-background segmentation.
/// 
/// ## Parameters
/// * src: Input image of type CV_8UC(n), CV_16UC(n), CV_32FC(n) or CV_64FC(n), where n is a positive integer.
/// * dst: %Accumulator image with the same number of channels as input image, and a depth of CV_32F or CV_64F.
/// * mask: Optional operation mask.
/// 
/// @sa  accumulateSquare, accumulateProduct, accumulateWeighted
///
/// ## C++ default parameters:
/// * mask: noArray()
pub fn accumulate(src: &core::Mat, dst: &mut core::Mat, mask: &core::Mat) -> Result<()> {
// identifier: cv_accumulate_Mat_src_Mat_dst_Mat_mask
  unsafe {
    let rv = sys::cv_imgproc_cv_accumulate_Mat_src_Mat_dst_Mat_mask(src.as_raw_Mat(), dst.as_raw_Mat(), mask.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Applies an adaptive threshold to an array.
/// 
/// The function transforms a grayscale image to a binary image according to the formulae:
/// *   **THRESH_BINARY**
/// <div lang='latex'>dst(x,y) =  \fork{\texttt{maxValue}}{if \(src(x,y) > T(x,y)\)}{0}{otherwise}</div>
/// *   **THRESH_BINARY_INV**
/// <div lang='latex'>dst(x,y) =  \fork{0}{if \(src(x,y) > T(x,y)\)}{\texttt{maxValue}}{otherwise}</div>
/// where <span lang='latex'>T(x,y)</span> is a threshold calculated individually for each pixel (see adaptiveMethod parameter).
/// 
/// The function can process the image in-place.
/// 
/// ## Parameters
/// * src: Source 8-bit single-channel image.
/// * dst: Destination image of the same size and the same type as src.
/// * maxValue: Non-zero value assigned to the pixels for which the condition is satisfied
/// * adaptiveMethod: Adaptive thresholding algorithm to use, see #AdaptiveThresholdTypes.
/// The #BORDER_REPLICATE | #BORDER_ISOLATED is used to process boundaries.
/// * thresholdType: Thresholding type that must be either #THRESH_BINARY or #THRESH_BINARY_INV,
/// see #ThresholdTypes.
/// * blockSize: Size of a pixel neighborhood that is used to calculate a threshold value for the
/// pixel: 3, 5, 7, and so on.
/// * C: Constant subtracted from the mean or weighted mean (see the details below). Normally, it
/// is positive but may be zero or negative as well.
/// 
/// @sa  threshold, blur, GaussianBlur
pub fn adaptive_threshold(src: &core::Mat, dst: &mut core::Mat, max_value: f64, adaptive_method: i32, threshold_type: i32, block_size: i32, c: f64) -> Result<()> {
// identifier: cv_adaptiveThreshold_Mat_src_Mat_dst_double_maxValue_int_adaptiveMethod_int_thresholdType_int_blockSize_double_C
  unsafe {
    let rv = sys::cv_imgproc_cv_adaptiveThreshold_Mat_src_Mat_dst_double_maxValue_int_adaptiveMethod_int_thresholdType_int_blockSize_double_C(src.as_raw_Mat(), dst.as_raw_Mat(), max_value, adaptive_method, threshold_type, block_size, c);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Applies a user colormap on a given image.
/// 
/// ## Parameters
/// * src: The source image, grayscale or colored of type CV_8UC1 or CV_8UC3.
/// * dst: The result is the colormapped source image. Note: Mat::create is called on dst.
/// * userColor: The colormap to apply of type CV_8UC1 or CV_8UC3 and size 256
pub fn apply_color_map(src: &core::Mat, dst: &mut core::Mat, user_color: &core::Mat) -> Result<()> {
// identifier: cv_applyColorMap_Mat_src_Mat_dst_Mat_userColor
  unsafe {
    let rv = sys::cv_imgproc_cv_applyColorMap_Mat_src_Mat_dst_Mat_userColor(src.as_raw_Mat(), dst.as_raw_Mat(), user_color.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Applies a GNU Octave/MATLAB equivalent colormap on a given image.
/// 
/// ## Parameters
/// * src: The source image, grayscale or colored of type CV_8UC1 or CV_8UC3.
/// * dst: The result is the colormapped source image. Note: Mat::create is called on dst.
/// * colormap: The colormap to apply, see #ColormapTypes
pub fn apply_color_map_v0(src: &core::Mat, dst: &mut core::Mat, colormap: i32) -> Result<()> {
// identifier: cv_applyColorMap_Mat_src_Mat_dst_int_colormap
  unsafe {
    let rv = sys::cv_imgproc_cv_applyColorMap_Mat_src_Mat_dst_int_colormap(src.as_raw_Mat(), dst.as_raw_Mat(), colormap);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Approximates a polygonal curve(s) with the specified precision.
/// 
/// The function cv::approxPolyDP approximates a curve or a polygon with another curve/polygon with less
/// vertices so that the distance between them is less or equal to the specified precision. It uses the
/// Douglas-Peucker algorithm <http://en.wikipedia.org/wiki/Ramer-Douglas-Peucker_algorithm>
/// 
/// ## Parameters
/// * curve: Input vector of a 2D point stored in std::vector or Mat
/// * approxCurve: Result of the approximation. The type should match the type of the input curve.
/// * epsilon: Parameter specifying the approximation accuracy. This is the maximum distance
/// between the original curve and its approximation.
/// * closed: If true, the approximated curve is closed (its first and last vertices are
/// connected). Otherwise, it is not closed.
pub fn approx_poly_dp(curve: &core::Mat, approx_curve: &mut core::Mat, epsilon: f64, closed: bool) -> Result<()> {
// identifier: cv_approxPolyDP_Mat_curve_Mat_approxCurve_double_epsilon_bool_closed
  unsafe {
    let rv = sys::cv_imgproc_cv_approxPolyDP_Mat_curve_Mat_approxCurve_double_epsilon_bool_closed(curve.as_raw_Mat(), approx_curve.as_raw_Mat(), epsilon, closed);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates a contour perimeter or a curve length.
/// 
/// The function computes a curve length or a closed contour perimeter.
/// 
/// ## Parameters
/// * curve: Input vector of 2D points, stored in std::vector or Mat.
/// * closed: Flag indicating whether the curve is closed or not.
pub fn arc_length(curve: &core::Mat, closed: bool) -> Result<f64> {
// identifier: cv_arcLength_Mat_curve_bool_closed
  unsafe {
    let rv = sys::cv_imgproc_cv_arcLength_Mat_curve_bool_closed(curve.as_raw_Mat(), closed);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Draws a arrow segment pointing from the first point to the second one.
/// 
/// The function cv::arrowedLine draws an arrow between pt1 and pt2 points in the image. See also #line.
/// 
/// ## Parameters
/// * img: Image.
/// * pt1: The point the arrow starts from.
/// * pt2: The point the arrow points to.
/// * color: Line color.
/// * thickness: Line thickness.
/// * line_type: Type of the line. See #LineTypes
/// * shift: Number of fractional bits in the point coordinates.
/// * tipLength: The length of the arrow tip in relation to the arrow length
///
/// ## C++ default parameters:
/// * thickness: 1
/// * line_type: 8
/// * shift: 0
/// * tip_length: 0.1
pub fn arrowed_line(img: &mut core::Mat, pt1: core::Point, pt2: core::Point, color: core::Scalar, thickness: i32, line_type: i32, shift: i32, tip_length: f64) -> Result<()> {
// identifier: cv_arrowedLine_Mat_img_Point_pt1_Point_pt2_Scalar_color_int_thickness_int_line_type_int_shift_double_tipLength
  unsafe {
    let rv = sys::cv_imgproc_cv_arrowedLine_Mat_img_Point_pt1_Point_pt2_Scalar_color_int_thickness_int_line_type_int_shift_double_tipLength(img.as_raw_Mat(), pt1, pt2, color, thickness, line_type, shift, tip_length);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Applies the bilateral filter to an image.
/// 
/// The function applies bilateral filtering to the input image, as described in
/// http://www.dai.ed.ac.uk/CVonline/LOCAL_COPIES/MANDUCHI1/Bilateral_Filtering.html
/// bilateralFilter can reduce unwanted noise very well while keeping edges fairly sharp. However, it is
/// very slow compared to most filters.
/// 
/// _Sigma values_: For simplicity, you can set the 2 sigma values to be the same. If they are small (\<
/// 10), the filter will not have much effect, whereas if they are large (\> 150), they will have a very
/// strong effect, making the image look "cartoonish".
/// 
/// _Filter size_: Large filters (d \> 5) are very slow, so it is recommended to use d=5 for real-time
/// applications, and perhaps d=9 for offline applications that need heavy noise filtering.
/// 
/// This filter does not work inplace.
/// ## Parameters
/// * src: Source 8-bit or floating-point, 1-channel or 3-channel image.
/// * dst: Destination image of the same size and type as src .
/// * d: Diameter of each pixel neighborhood that is used during filtering. If it is non-positive,
/// it is computed from sigmaSpace.
/// * sigmaColor: Filter sigma in the color space. A larger value of the parameter means that
/// farther colors within the pixel neighborhood (see sigmaSpace) will be mixed together, resulting
/// in larger areas of semi-equal color.
/// * sigmaSpace: Filter sigma in the coordinate space. A larger value of the parameter means that
/// farther pixels will influence each other as long as their colors are close enough (see sigmaColor
/// ). When d\>0, it specifies the neighborhood size regardless of sigmaSpace. Otherwise, d is
/// proportional to sigmaSpace.
/// * borderType: border mode used to extrapolate pixels outside of the image, see #BorderTypes
///
/// ## C++ default parameters:
/// * border_type: BORDER_DEFAULT
pub fn bilateral_filter(src: &core::Mat, dst: &mut core::Mat, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32) -> Result<()> {
// identifier: cv_bilateralFilter_Mat_src_Mat_dst_int_d_double_sigmaColor_double_sigmaSpace_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_bilateralFilter_Mat_src_Mat_dst_int_d_double_sigmaColor_double_sigmaSpace_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), d, sigma_color, sigma_space, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

pub fn blend_linear(src1: &core::Mat, src2: &core::Mat, weights1: &core::Mat, weights2: &core::Mat, dst: &mut core::Mat) -> Result<()> {
// identifier: cv_blendLinear_Mat_src1_Mat_src2_Mat_weights1_Mat_weights2_Mat_dst
  unsafe {
    let rv = sys::cv_imgproc_cv_blendLinear_Mat_src1_Mat_src2_Mat_weights1_Mat_weights2_Mat_dst(src1.as_raw_Mat(), src2.as_raw_Mat(), weights1.as_raw_Mat(), weights2.as_raw_Mat(), dst.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Blurs an image using the normalized box filter.
/// 
/// The function smooths an image using the kernel:
/// 
/// <div lang='latex'>\texttt{K} =  \frac{1}{\texttt{ksize.width*ksize.height}} \begin{bmatrix} 1 & 1 & 1 &  \cdots & 1 & 1  \\ 1 & 1 & 1 &  \cdots & 1 & 1  \\ \hdotsfor{6} \\ 1 & 1 & 1 &  \cdots & 1 & 1  \\ \end{bmatrix}</div>
/// 
/// The call `blur(src, dst, ksize, anchor, borderType)` is equivalent to `boxFilter(src, dst, src.type(),
/// anchor, true, borderType)`.
/// 
/// ## Parameters
/// * src: input image; it can have any number of channels, which are processed independently, but
/// the depth should be CV_8U, CV_16U, CV_16S, CV_32F or CV_64F.
/// * dst: output image of the same size and type as src.
/// * ksize: blurring kernel size.
/// * anchor: anchor point; default value Point(-1,-1) means that the anchor is at the kernel
/// center.
/// * borderType: border mode used to extrapolate pixels outside of the image, see #BorderTypes
/// @sa  boxFilter, bilateralFilter, GaussianBlur, medianBlur
///
/// ## C++ default parameters:
/// * anchor: Point(-1,-1)
/// * border_type: BORDER_DEFAULT
pub fn blur(src: &core::Mat, dst: &mut core::Mat, ksize: core::Size, anchor: core::Point, border_type: i32) -> Result<()> {
// identifier: cv_blur_Mat_src_Mat_dst_Size_ksize_Point_anchor_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_blur_Mat_src_Mat_dst_Size_ksize_Point_anchor_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), ksize, anchor, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates the up-right bounding rectangle of a point set or non-zero pixels of gray-scale image.
/// 
/// The function calculates and returns the minimal up-right bounding rectangle for the specified point set or
/// non-zero pixels of gray-scale image.
/// 
/// ## Parameters
/// * array: Input gray-scale image or 2D point set, stored in std::vector or Mat.
pub fn bounding_rect(array: &core::Mat) -> Result<core::Rect> {
// identifier: cv_boundingRect_Mat_array
  unsafe {
    let rv = sys::cv_imgproc_cv_boundingRect_Mat_array(array.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Blurs an image using the box filter.
/// 
/// The function smooths an image using the kernel:
/// 
/// <div lang='latex'>\texttt{K} =  \alpha \begin{bmatrix} 1 & 1 & 1 &  \cdots & 1 & 1  \\ 1 & 1 & 1 &  \cdots & 1 & 1  \\ \hdotsfor{6} \\ 1 & 1 & 1 &  \cdots & 1 & 1 \end{bmatrix}</div>
/// 
/// where
/// 
/// <div lang='latex'>\alpha = \fork{\frac{1}{\texttt{ksize.width*ksize.height}}}{when \texttt{normalize=true}}{1}{otherwise}</div>
/// 
/// Unnormalized box filter is useful for computing various integral characteristics over each pixel
/// neighborhood, such as covariance matrices of image derivatives (used in dense optical flow
/// algorithms, and so on). If you need to compute pixel sums over variable-size windows, use #integral.
/// 
/// ## Parameters
/// * src: input image.
/// * dst: output image of the same size and type as src.
/// * ddepth: the output image depth (-1 to use src.depth()).
/// * ksize: blurring kernel size.
/// * anchor: anchor point; default value Point(-1,-1) means that the anchor is at the kernel
/// center.
/// * normalize: flag, specifying whether the kernel is normalized by its area or not.
/// * borderType: border mode used to extrapolate pixels outside of the image, see #BorderTypes
/// @sa  blur, bilateralFilter, GaussianBlur, medianBlur, integral
///
/// ## C++ default parameters:
/// * anchor: Point(-1,-1)
/// * normalize: true
/// * border_type: BORDER_DEFAULT
pub fn box_filter(src: &core::Mat, dst: &mut core::Mat, ddepth: i32, ksize: core::Size, anchor: core::Point, normalize: bool, border_type: i32) -> Result<()> {
// identifier: cv_boxFilter_Mat_src_Mat_dst_int_ddepth_Size_ksize_Point_anchor_bool_normalize_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_boxFilter_Mat_src_Mat_dst_int_ddepth_Size_ksize_Point_anchor_bool_normalize_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), ddepth, ksize, anchor, normalize, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds the four vertices of a rotated rect. Useful to draw the rotated rectangle.
/// 
/// The function finds the four vertices of a rotated rectangle. This function is useful to draw the
/// rectangle. In C++, instead of using this function, you can directly use RotatedRect::points method. Please
/// visit the @ref tutorial_bounding_rotated_ellipses "tutorial on Creating Bounding rotated boxes and ellipses for contours" for more information.
/// 
/// ## Parameters
/// * box: The input rotated rectangle. It may be the output of
/// * points: The output array of four vertices of rectangles.
pub fn box_points(_box: &core::RotatedRect, points: &mut core::Mat) -> Result<()> {
// identifier: cv_boxPoints_RotatedRect_box_Mat_points
  unsafe {
    let rv = sys::cv_imgproc_cv_boxPoints_RotatedRect_box_Mat_points(_box.as_raw_RotatedRect(), points.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Constructs the Gaussian pyramid for an image.
/// 
/// The function constructs a vector of images and builds the Gaussian pyramid by recursively applying
/// pyrDown to the previously built pyramid layers, starting from `dst[0]==src`.
/// 
/// ## Parameters
/// * src: Source image. Check pyrDown for the list of supported types.
/// * dst: Destination vector of maxlevel+1 images of the same type as src. dst[0] will be the
/// same as src. dst[1] is the next pyramid layer, a smoothed and down-sized src, and so on.
/// * maxlevel: 0-based index of the last (the smallest) pyramid layer. It must be non-negative.
/// * borderType: Pixel extrapolation method, see #BorderTypes (#BORDER_CONSTANT isn't supported)
///
/// ## C++ default parameters:
/// * border_type: BORDER_DEFAULT
pub fn build_pyramid(src: &core::Mat, dst: &mut types::VectorOfMat, maxlevel: i32, border_type: i32) -> Result<()> {
// identifier: cv_buildPyramid_Mat_src_VectorOfMat_dst_int_maxlevel_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_buildPyramid_Mat_src_VectorOfMat_dst_int_maxlevel_int_borderType(src.as_raw_Mat(), dst.as_raw_VectorOfMat(), maxlevel, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
pub fn calc_back_project(images: &types::VectorOfMat, channels: &types::VectorOfint, hist: &core::Mat, dst: &mut core::Mat, ranges: &types::VectorOffloat, scale: f64) -> Result<()> {
// identifier: cv_calcBackProject_VectorOfMat_images_VectorOfint_channels_Mat_hist_Mat_dst_VectorOffloat_ranges_double_scale
  unsafe {
    let rv = sys::cv_imgproc_cv_calcBackProject_VectorOfMat_images_VectorOfint_channels_Mat_hist_Mat_dst_VectorOffloat_ranges_double_scale(images.as_raw_VectorOfMat(), channels.as_raw_VectorOfint(), hist.as_raw_Mat(), dst.as_raw_Mat(), ranges.as_raw_VectorOffloat(), scale);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
///
/// ## C++ default parameters:
/// * accumulate: false
pub fn calc_hist(images: &types::VectorOfMat, channels: &types::VectorOfint, mask: &core::Mat, hist: &mut core::Mat, hist_size: &types::VectorOfint, ranges: &types::VectorOffloat, accumulate: bool) -> Result<()> {
// identifier: cv_calcHist_VectorOfMat_images_VectorOfint_channels_Mat_mask_Mat_hist_VectorOfint_histSize_VectorOffloat_ranges_bool_accumulate
  unsafe {
    let rv = sys::cv_imgproc_cv_calcHist_VectorOfMat_images_VectorOfint_channels_Mat_mask_Mat_hist_VectorOfint_histSize_VectorOffloat_ranges_bool_accumulate(images.as_raw_VectorOfMat(), channels.as_raw_VectorOfint(), mask.as_raw_Mat(), hist.as_raw_Mat(), hist_size.as_raw_VectorOfint(), ranges.as_raw_VectorOffloat(), accumulate);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Draws a circle.
/// 
/// The function cv::circle draws a simple or filled circle with a given center and radius.
/// ## Parameters
/// * img: Image where the circle is drawn.
/// * center: Center of the circle.
/// * radius: Radius of the circle.
/// * color: Circle color.
/// * thickness: Thickness of the circle outline, if positive. Negative values, like #FILLED,
/// mean that a filled circle is to be drawn.
/// * lineType: Type of the circle boundary. See #LineTypes
/// * shift: Number of fractional bits in the coordinates of the center and in the radius value.
///
/// ## C++ default parameters:
/// * thickness: 1
/// * line_type: LINE_8
/// * shift: 0
pub fn circle(img: &mut core::Mat, center: core::Point, radius: i32, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> Result<()> {
// identifier: cv_circle_Mat_img_Point_center_int_radius_Scalar_color_int_thickness_int_lineType_int_shift
  unsafe {
    let rv = sys::cv_imgproc_cv_circle_Mat_img_Point_center_int_radius_Scalar_color_int_thickness_int_lineType_int_shift(img.as_raw_Mat(), center, radius, color, thickness, line_type, shift);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
/// ## Parameters
/// * imgRect: Image rectangle.
/// * pt1: First line point.
/// * pt2: Second line point.
pub fn clip_line(img_rect: core::Rect, pt1: core::Point, pt2: core::Point) -> Result<bool> {
// identifier: cv_clipLine_Rect_imgRect_Point_pt1_Point_pt2
  unsafe {
    let rv = sys::cv_imgproc_cv_clipLine_Rect_imgRect_Point_pt1_Point_pt2(img_rect, pt1, pt2);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// @overload
/// ## Parameters
/// * imgSize: Image size. The image rectangle is Rect(0, 0, imgSize.width, imgSize.height) .
/// * pt1: First line point.
/// * pt2: Second line point.
pub fn clip_line_size_i64(img_size: core::Size2l, pt1: core::Point2l, pt2: core::Point2l) -> Result<bool> {
// identifier: cv_clipLine_Size2l_imgSize_Point2l_pt1_Point2l_pt2
  unsafe {
    let rv = sys::cv_imgproc_cv_clipLine_Size2l_imgSize_Point2l_pt1_Point2l_pt2(img_size, pt1, pt2);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Clips the line against the image rectangle.
/// 
/// The function cv::clipLine calculates a part of the line segment that is entirely within the specified
/// rectangle. it returns false if the line segment is completely outside the rectangle. Otherwise,
/// it returns true .
/// ## Parameters
/// * imgSize: Image size. The image rectangle is Rect(0, 0, imgSize.width, imgSize.height) .
/// * pt1: First line point.
/// * pt2: Second line point.
pub fn clip_line_size(img_size: core::Size, pt1: core::Point, pt2: core::Point) -> Result<bool> {
// identifier: cv_clipLine_Size_imgSize_Point_pt1_Point_pt2
  unsafe {
    let rv = sys::cv_imgproc_cv_clipLine_Size_imgSize_Point_pt1_Point_pt2(img_size, pt1, pt2);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Compares two histograms.
/// 
/// The function cv::compareHist compares two dense or two sparse histograms using the specified method.
/// 
/// The function returns <span lang='latex'>d(H_1, H_2)</span> .
/// 
/// While the function works well with 1-, 2-, 3-dimensional dense histograms, it may not be suitable
/// for high-dimensional sparse histograms. In such histograms, because of aliasing and sampling
/// problems, the coordinates of non-zero histogram bins can slightly shift. To compare such histograms
/// or more general sparse configurations of weighted points, consider using the #EMD function.
/// 
/// ## Parameters
/// * H1: First compared histogram.
/// * H2: Second compared histogram of the same size as H1 .
/// * method: Comparison method, see #HistCompMethods
pub fn compare_hist(h1: &core::Mat, h2: &core::Mat, method: i32) -> Result<f64> {
// identifier: cv_compareHist_Mat_H1_Mat_H2_int_method
  unsafe {
    let rv = sys::cv_imgproc_cv_compareHist_Mat_H1_Mat_H2_int_method(h1.as_raw_Mat(), h2.as_raw_Mat(), method);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// @overload
/// ## Parameters
/// * image: the 8-bit single-channel image to be labeled
/// * labels: destination labeled image
/// * stats: statistics output for each label, including the background label, see below for
/// available statistics. Statistics are accessed via stats(label, COLUMN) where COLUMN is one of
/// #ConnectedComponentsTypes. The data type is CV_32S.
/// * centroids: centroid output for each label, including the background label. Centroids are
/// accessed via centroids(label, 0) for x and centroids(label, 1) for y. The data type CV_64F.
/// * connectivity: 8 or 4 for 8-way or 4-way connectivity respectively
/// * ltype: output image label type. Currently CV_32S and CV_16U are supported.
///
/// ## C++ default parameters:
/// * connectivity: 8
/// * ltype: CV_32S
pub fn connected_components_with_stats(image: &core::Mat, labels: &mut core::Mat, stats: &mut core::Mat, centroids: &mut core::Mat, connectivity: i32, ltype: i32) -> Result<i32> {
// identifier: cv_connectedComponentsWithStats_Mat_image_Mat_labels_Mat_stats_Mat_centroids_int_connectivity_int_ltype
  unsafe {
    let rv = sys::cv_imgproc_cv_connectedComponentsWithStats_Mat_image_Mat_labels_Mat_stats_Mat_centroids_int_connectivity_int_ltype(image.as_raw_Mat(), labels.as_raw_Mat(), stats.as_raw_Mat(), centroids.as_raw_Mat(), connectivity, ltype);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// computes the connected components labeled image of boolean image and also produces a statistics output for each label
/// 
/// image with 4 or 8 way connectivity - returns N, the total number of labels [0, N-1] where 0
/// represents the background label. ltype specifies the output label image type, an important
/// consideration based on the total number of labels or alternatively the total number of pixels in
/// the source image. ccltype specifies the connected components labeling algorithm to use, currently
/// Grana's (BBDT) and Wu's (SAUF) algorithms are supported, see the #ConnectedComponentsAlgorithmsTypes
/// for details. Note that SAUF algorithm forces a row major ordering of labels while BBDT does not.
/// This function uses parallel version of both Grana and Wu's algorithms (statistics included) if at least one allowed
/// parallel framework is enabled and if the rows of the image are at least twice the number returned by #getNumberOfCPUs.
/// 
/// ## Parameters
/// * image: the 8-bit single-channel image to be labeled
/// * labels: destination labeled image
/// * stats: statistics output for each label, including the background label, see below for
/// available statistics. Statistics are accessed via stats(label, COLUMN) where COLUMN is one of
/// #ConnectedComponentsTypes. The data type is CV_32S.
/// * centroids: centroid output for each label, including the background label. Centroids are
/// accessed via centroids(label, 0) for x and centroids(label, 1) for y. The data type CV_64F.
/// * connectivity: 8 or 4 for 8-way or 4-way connectivity respectively
/// * ltype: output image label type. Currently CV_32S and CV_16U are supported.
/// * ccltype: connected components algorithm type (see #ConnectedComponentsAlgorithmsTypes).
pub fn connected_components_with_stats_v0(image: &core::Mat, labels: &mut core::Mat, stats: &mut core::Mat, centroids: &mut core::Mat, connectivity: i32, ltype: i32, ccltype: i32) -> Result<i32> {
// identifier: cv_connectedComponentsWithStats_Mat_image_Mat_labels_Mat_stats_Mat_centroids_int_connectivity_int_ltype_int_ccltype
  unsafe {
    let rv = sys::cv_imgproc_cv_connectedComponentsWithStats_Mat_image_Mat_labels_Mat_stats_Mat_centroids_int_connectivity_int_ltype_int_ccltype(image.as_raw_Mat(), labels.as_raw_Mat(), stats.as_raw_Mat(), centroids.as_raw_Mat(), connectivity, ltype, ccltype);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// @overload
/// 
/// ## Parameters
/// * image: the 8-bit single-channel image to be labeled
/// * labels: destination labeled image
/// * connectivity: 8 or 4 for 8-way or 4-way connectivity respectively
/// * ltype: output image label type. Currently CV_32S and CV_16U are supported.
///
/// ## C++ default parameters:
/// * connectivity: 8
/// * ltype: CV_32S
pub fn connected_components(image: &core::Mat, labels: &mut core::Mat, connectivity: i32, ltype: i32) -> Result<i32> {
// identifier: cv_connectedComponents_Mat_image_Mat_labels_int_connectivity_int_ltype
  unsafe {
    let rv = sys::cv_imgproc_cv_connectedComponents_Mat_image_Mat_labels_int_connectivity_int_ltype(image.as_raw_Mat(), labels.as_raw_Mat(), connectivity, ltype);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// computes the connected components labeled image of boolean image
/// 
/// image with 4 or 8 way connectivity - returns N, the total number of labels [0, N-1] where 0
/// represents the background label. ltype specifies the output label image type, an important
/// consideration based on the total number of labels or alternatively the total number of pixels in
/// the source image. ccltype specifies the connected components labeling algorithm to use, currently
/// Grana (BBDT) and Wu's (SAUF) algorithms are supported, see the #ConnectedComponentsAlgorithmsTypes
/// for details. Note that SAUF algorithm forces a row major ordering of labels while BBDT does not.
/// This function uses parallel version of both Grana and Wu's algorithms if at least one allowed
/// parallel framework is enabled and if the rows of the image are at least twice the number returned by #getNumberOfCPUs.
/// 
/// ## Parameters
/// * image: the 8-bit single-channel image to be labeled
/// * labels: destination labeled image
/// * connectivity: 8 or 4 for 8-way or 4-way connectivity respectively
/// * ltype: output image label type. Currently CV_32S and CV_16U are supported.
/// * ccltype: connected components algorithm type (see the #ConnectedComponentsAlgorithmsTypes).
pub fn connected_components_v0(image: &core::Mat, labels: &mut core::Mat, connectivity: i32, ltype: i32, ccltype: i32) -> Result<i32> {
// identifier: cv_connectedComponents_Mat_image_Mat_labels_int_connectivity_int_ltype_int_ccltype
  unsafe {
    let rv = sys::cv_imgproc_cv_connectedComponents_Mat_image_Mat_labels_int_connectivity_int_ltype_int_ccltype(image.as_raw_Mat(), labels.as_raw_Mat(), connectivity, ltype, ccltype);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Calculates a contour area.
/// 
/// The function computes a contour area. Similarly to moments , the area is computed using the Green
/// formula. Thus, the returned area and the number of non-zero pixels, if you draw the contour using
/// #drawContours or #fillPoly , can be different. Also, the function will most certainly give a wrong
/// results for contours with self-intersections.
/// 
/// Example:
/// ```ignore
/// vector<Point> contour;
/// contour.push_back(Point2f(0, 0));
/// contour.push_back(Point2f(10, 0));
/// contour.push_back(Point2f(10, 10));
/// contour.push_back(Point2f(5, 4));
/// 
/// double area0 = contourArea(contour);
/// vector<Point> approx;
/// approxPolyDP(contour, approx, 5, true);
/// double area1 = contourArea(approx);
/// 
/// cout << "area0 =" << area0 << endl <<
/// "area1 =" << area1 << endl <<
/// "approx poly vertices" << approx.size() << endl;
/// ```
/// 
/// ## Parameters
/// * contour: Input vector of 2D points (contour vertices), stored in std::vector or Mat.
/// * oriented: Oriented area flag. If it is true, the function returns a signed area value,
/// depending on the contour orientation (clockwise or counter-clockwise). Using this feature you can
/// determine orientation of a contour by taking the sign of an area. By default, the parameter is
/// false, which means that the absolute value is returned.
///
/// ## C++ default parameters:
/// * oriented: false
pub fn contour_area(contour: &core::Mat, oriented: bool) -> Result<f64> {
// identifier: cv_contourArea_Mat_contour_bool_oriented
  unsafe {
    let rv = sys::cv_imgproc_cv_contourArea_Mat_contour_bool_oriented(contour.as_raw_Mat(), oriented);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Converts image transformation maps from one representation to another.
/// 
/// The function converts a pair of maps for remap from one representation to another. The following
/// options ( (map1.type(), map2.type()) <span lang='latex'>\rightarrow</span> (dstmap1.type(), dstmap2.type()) ) are
/// supported:
/// 
/// - <span lang='latex'>\texttt{(CV_32FC1, CV_32FC1)} \rightarrow \texttt{(CV_16SC2, CV_16UC1)}</span>. This is the
/// most frequently used conversion operation, in which the original floating-point maps (see remap )
/// are converted to a more compact and much faster fixed-point representation. The first output array
/// contains the rounded coordinates and the second array (created only when nninterpolation=false )
/// contains indices in the interpolation tables.
/// 
/// - <span lang='latex'>\texttt{(CV_32FC2)} \rightarrow \texttt{(CV_16SC2, CV_16UC1)}</span>. The same as above but
/// the original maps are stored in one 2-channel matrix.
/// 
/// - Reverse conversion. Obviously, the reconstructed floating-point maps will not be exactly the same
/// as the originals.
/// 
/// ## Parameters
/// * map1: The first input map of type CV_16SC2, CV_32FC1, or CV_32FC2 .
/// * map2: The second input map of type CV_16UC1, CV_32FC1, or none (empty matrix),
/// respectively.
/// * dstmap1: The first output map that has the type dstmap1type and the same size as src .
/// * dstmap2: The second output map.
/// * dstmap1type: Type of the first output map that should be CV_16SC2, CV_32FC1, or
/// CV_32FC2 .
/// * nninterpolation: Flag indicating whether the fixed-point maps are used for the
/// nearest-neighbor or for a more complex interpolation.
/// 
/// @sa  remap, undistort, initUndistortRectifyMap
///
/// ## C++ default parameters:
/// * nninterpolation: false
pub fn convert_maps(map1: &core::Mat, map2: &core::Mat, dstmap1: &mut core::Mat, dstmap2: &mut core::Mat, dstmap1type: i32, nninterpolation: bool) -> Result<()> {
// identifier: cv_convertMaps_Mat_map1_Mat_map2_Mat_dstmap1_Mat_dstmap2_int_dstmap1type_bool_nninterpolation
  unsafe {
    let rv = sys::cv_imgproc_cv_convertMaps_Mat_map1_Mat_map2_Mat_dstmap1_Mat_dstmap2_int_dstmap1type_bool_nninterpolation(map1.as_raw_Mat(), map2.as_raw_Mat(), dstmap1.as_raw_Mat(), dstmap2.as_raw_Mat(), dstmap1type, nninterpolation);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds the convex hull of a point set.
/// 
/// The function cv::convexHull finds the convex hull of a 2D point set using the Sklansky's algorithm @cite Sklansky82
/// that has *O(N logN)* complexity in the current implementation.
/// 
/// ## Parameters
/// * points: Input 2D point set, stored in std::vector or Mat.
/// * hull: Output convex hull. It is either an integer vector of indices or vector of points. In
/// the first case, the hull elements are 0-based indices of the convex hull points in the original
/// array (since the set of convex hull points is a subset of the original point set). In the second
/// case, hull elements are the convex hull points themselves.
/// * clockwise: Orientation flag. If it is true, the output convex hull is oriented clockwise.
/// Otherwise, it is oriented counter-clockwise. The assumed coordinate system has its X axis pointing
/// to the right, and its Y axis pointing upwards.
/// * returnPoints: Operation flag. In case of a matrix, when the flag is true, the function
/// returns convex hull points. Otherwise, it returns indices of the convex hull points. When the
/// output array is std::vector, the flag is ignored, and the output depends on the type of the
/// vector: std::vector\<int\> implies returnPoints=false, std::vector\<Point\> implies
/// returnPoints=true.
/// 
/// 
/// Note: `points` and `hull` should be different arrays, inplace processing isn't supported.
/// 
/// Check @ref tutorial_hull "the corresponding tutorial" for more details.
/// 
/// useful links:
/// 
/// https://www.learnopencv.com/convex-hull-using-opencv-in-python-and-c/
///
/// ## C++ default parameters:
/// * clockwise: false
/// * return_points: true
pub fn convex_hull(points: &core::Mat, hull: &mut core::Mat, clockwise: bool, return_points: bool) -> Result<()> {
// identifier: cv_convexHull_Mat_points_Mat_hull_bool_clockwise_bool_returnPoints
  unsafe {
    let rv = sys::cv_imgproc_cv_convexHull_Mat_points_Mat_hull_bool_clockwise_bool_returnPoints(points.as_raw_Mat(), hull.as_raw_Mat(), clockwise, return_points);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds the convexity defects of a contour.
/// 
/// The figure below displays convexity defects of a hand contour:
/// 
/// ![image](pics/defects.png)
/// 
/// ## Parameters
/// * contour: Input contour.
/// * convexhull: Convex hull obtained using convexHull that should contain indices of the contour
/// points that make the hull.
/// * convexityDefects: The output vector of convexity defects. In C++ and the new Python/Java
/// interface each convexity defect is represented as 4-element integer vector (a.k.a. #Vec4i):
/// (start_index, end_index, farthest_pt_index, fixpt_depth), where indices are 0-based indices
/// in the original contour of the convexity defect beginning, end and the farthest point, and
/// fixpt_depth is fixed-point approximation (with 8 fractional bits) of the distance between the
/// farthest contour point and the hull. That is, to get the floating-point value of the depth will be
/// fixpt_depth/256.0.
pub fn convexity_defects(contour: &core::Mat, convexhull: &core::Mat, convexity_defects: &mut core::Mat) -> Result<()> {
// identifier: cv_convexityDefects_Mat_contour_Mat_convexhull_Mat_convexityDefects
  unsafe {
    let rv = sys::cv_imgproc_cv_convexityDefects_Mat_contour_Mat_convexhull_Mat_convexityDefects(contour.as_raw_Mat(), convexhull.as_raw_Mat(), convexity_defects.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates eigenvalues and eigenvectors of image blocks for corner detection.
/// 
/// For every pixel <span lang='latex'>p</span> , the function cornerEigenValsAndVecs considers a blockSize <span lang='latex'>\times</span> blockSize
/// neighborhood <span lang='latex'>S(p)</span> . It calculates the covariation matrix of derivatives over the neighborhood as:
/// 
/// <div lang='latex'>M =  \begin{bmatrix} \sum _{S(p)}(dI/dx)^2 &  \sum _{S(p)}dI/dx dI/dy  \\ \sum _{S(p)}dI/dx dI/dy &  \sum _{S(p)}(dI/dy)^2 \end{bmatrix}</div>
/// 
/// where the derivatives are computed using the Sobel operator.
/// 
/// After that, it finds eigenvectors and eigenvalues of <span lang='latex'>M</span> and stores them in the destination image as
/// <span lang='latex'>(\lambda_1, \lambda_2, x_1, y_1, x_2, y_2)</span> where
/// 
/// *   <span lang='latex'>\lambda_1, \lambda_2</span> are the non-sorted eigenvalues of <span lang='latex'>M</span>
/// *   <span lang='latex'>x_1, y_1</span> are the eigenvectors corresponding to <span lang='latex'>\lambda_1</span>
/// *   <span lang='latex'>x_2, y_2</span> are the eigenvectors corresponding to <span lang='latex'>\lambda_2</span>
/// 
/// The output of the function can be used for robust edge or corner detection.
/// 
/// ## Parameters
/// * src: Input single-channel 8-bit or floating-point image.
/// * dst: Image to store the results. It has the same size as src and the type CV_32FC(6) .
/// * blockSize: Neighborhood size (see details below).
/// * ksize: Aperture parameter for the Sobel operator.
/// * borderType: Pixel extrapolation method. See #BorderTypes.
/// 
/// @sa  cornerMinEigenVal, cornerHarris, preCornerDetect
///
/// ## C++ default parameters:
/// * border_type: BORDER_DEFAULT
pub fn corner_eigen_vals_and_vecs(src: &core::Mat, dst: &mut core::Mat, block_size: i32, ksize: i32, border_type: i32) -> Result<()> {
// identifier: cv_cornerEigenValsAndVecs_Mat_src_Mat_dst_int_blockSize_int_ksize_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_cornerEigenValsAndVecs_Mat_src_Mat_dst_int_blockSize_int_ksize_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), block_size, ksize, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Harris corner detector.
/// 
/// The function runs the Harris corner detector on the image. Similarly to cornerMinEigenVal and
/// cornerEigenValsAndVecs , for each pixel <span lang='latex'>(x, y)</span> it calculates a <span lang='latex'>2\times2</span> gradient covariance
/// matrix <span lang='latex'>M^{(x,y)}</span> over a <span lang='latex'>\texttt{blockSize} \times \texttt{blockSize}</span> neighborhood. Then, it
/// computes the following characteristic:
/// 
/// <div lang='latex'>\texttt{dst} (x,y) =  \mathrm{det} M^{(x,y)} - k  \cdot \left ( \mathrm{tr} M^{(x,y)} \right )^2</div>
/// 
/// Corners in the image can be found as the local maxima of this response map.
/// 
/// ## Parameters
/// * src: Input single-channel 8-bit or floating-point image.
/// * dst: Image to store the Harris detector responses. It has the type CV_32FC1 and the same
/// size as src .
/// * blockSize: Neighborhood size (see the details on #cornerEigenValsAndVecs ).
/// * ksize: Aperture parameter for the Sobel operator.
/// * k: Harris detector free parameter. See the formula above.
/// * borderType: Pixel extrapolation method. See #BorderTypes.
///
/// ## C++ default parameters:
/// * border_type: BORDER_DEFAULT
pub fn corner_harris(src: &core::Mat, dst: &mut core::Mat, block_size: i32, ksize: i32, k: f64, border_type: i32) -> Result<()> {
// identifier: cv_cornerHarris_Mat_src_Mat_dst_int_blockSize_int_ksize_double_k_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_cornerHarris_Mat_src_Mat_dst_int_blockSize_int_ksize_double_k_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), block_size, ksize, k, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates the minimal eigenvalue of gradient matrices for corner detection.
/// 
/// The function is similar to cornerEigenValsAndVecs but it calculates and stores only the minimal
/// eigenvalue of the covariance matrix of derivatives, that is, <span lang='latex'>\min(\lambda_1, \lambda_2)</span> in terms
/// of the formulae in the cornerEigenValsAndVecs description.
/// 
/// ## Parameters
/// * src: Input single-channel 8-bit or floating-point image.
/// * dst: Image to store the minimal eigenvalues. It has the type CV_32FC1 and the same size as
/// src .
/// * blockSize: Neighborhood size (see the details on #cornerEigenValsAndVecs ).
/// * ksize: Aperture parameter for the Sobel operator.
/// * borderType: Pixel extrapolation method. See #BorderTypes.
///
/// ## C++ default parameters:
/// * ksize: 3
/// * border_type: BORDER_DEFAULT
pub fn corner_min_eigen_val(src: &core::Mat, dst: &mut core::Mat, block_size: i32, ksize: i32, border_type: i32) -> Result<()> {
// identifier: cv_cornerMinEigenVal_Mat_src_Mat_dst_int_blockSize_int_ksize_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_cornerMinEigenVal_Mat_src_Mat_dst_int_blockSize_int_ksize_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), block_size, ksize, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Refines the corner locations.
/// 
/// The function iterates to find the sub-pixel accurate location of corners or radial saddle points, as
/// shown on the figure below.
/// 
/// ![image](pics/cornersubpix.png)
/// 
/// Sub-pixel accurate corner locator is based on the observation that every vector from the center <span lang='latex'>q</span>
/// to a point <span lang='latex'>p</span> located within a neighborhood of <span lang='latex'>q</span> is orthogonal to the image gradient at <span lang='latex'>p</span>
/// subject to image and measurement noise. Consider the expression:
/// 
/// <div lang='latex'>\epsilon _i = {DI_{p_i}}^T  \cdot (q - p_i)</div>
/// 
/// where <span lang='latex'>{DI_{p_i}}</span> is an image gradient at one of the points <span lang='latex'>p_i</span> in a neighborhood of <span lang='latex'>q</span> . The
/// value of <span lang='latex'>q</span> is to be found so that <span lang='latex'>\epsilon_i</span> is minimized. A system of equations may be set up
/// with <span lang='latex'>\epsilon_i</span> set to zero:
/// 
/// <div lang='latex'>\sum _i(DI_{p_i}  \cdot {DI_{p_i}}^T) \cdot q -  \sum _i(DI_{p_i}  \cdot {DI_{p_i}}^T  \cdot p_i)</div>
/// 
/// where the gradients are summed within a neighborhood ("search window") of <span lang='latex'>q</span> . Calling the first
/// gradient term <span lang='latex'>G</span> and the second gradient term <span lang='latex'>b</span> gives:
/// 
/// <div lang='latex'>q = G^{-1}  \cdot b</div>
/// 
/// The algorithm sets the center of the neighborhood window at this new center <span lang='latex'>q</span> and then iterates
/// until the center stays within a set threshold.
/// 
/// ## Parameters
/// * image: Input single-channel, 8-bit or float image.
/// * corners: Initial coordinates of the input corners and refined coordinates provided for
/// output.
/// * winSize: Half of the side length of the search window. For example, if winSize=Size(5,5) ,
/// then a <span lang='latex'>(5*2+1) \times (5*2+1) = 11 \times 11</span> search window is used.
/// * zeroZone: Half of the size of the dead region in the middle of the search zone over which
/// the summation in the formula below is not done. It is used sometimes to avoid possible
/// singularities of the autocorrelation matrix. The value of (-1,-1) indicates that there is no such
/// a size.
/// * criteria: Criteria for termination of the iterative process of corner refinement. That is,
/// the process of corner position refinement stops either after criteria.maxCount iterations or when
/// the corner position moves by less than criteria.epsilon on some iteration.
pub fn corner_sub_pix(image: &core::Mat, corners: &mut core::Mat, win_size: core::Size, zero_zone: core::Size, criteria: &core::TermCriteria) -> Result<()> {
// identifier: cv_cornerSubPix_Mat_image_Mat_corners_Size_winSize_Size_zeroZone_TermCriteria_criteria
  unsafe {
    let rv = sys::cv_imgproc_cv_cornerSubPix_Mat_image_Mat_corners_Size_winSize_Size_zeroZone_TermCriteria_criteria(image.as_raw_Mat(), corners.as_raw_Mat(), win_size, zero_zone, criteria.as_raw_TermCriteria());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Creates a smart pointer to a cv::CLAHE class and initializes it.
/// 
/// ## Parameters
/// * clipLimit: Threshold for contrast limiting.
/// * tileGridSize: Size of grid for histogram equalization. Input image will be divided into
/// equally sized rectangular tiles. tileGridSize defines the number of tiles in row and column.
///
/// ## C++ default parameters:
/// * clip_limit: 40.0
/// * tile_grid_size: Size(8, 8)
pub fn create_clahe(clip_limit: f64, tile_grid_size: core::Size) -> Result<types::PtrOfCLAHE> {
// identifier: cv_createCLAHE_double_clipLimit_Size_tileGridSize
  unsafe {
    let rv = sys::cv_imgproc_cv_createCLAHE_double_clipLimit_Size_tileGridSize(clip_limit, tile_grid_size);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfCLAHE { ptr: rv.result })
    }
  }
}

/// Creates a smart pointer to a cv::GeneralizedHoughBallard class and initializes it.
pub fn create_generalized_hough_ballard() -> Result<types::PtrOfGeneralizedHoughBallard> {
// identifier: cv_createGeneralizedHoughBallard
  unsafe {
    let rv = sys::cv_imgproc_cv_createGeneralizedHoughBallard();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfGeneralizedHoughBallard { ptr: rv.result })
    }
  }
}

/// Creates a smart pointer to a cv::GeneralizedHoughGuil class and initializes it.
pub fn create_generalized_hough_guil() -> Result<types::PtrOfGeneralizedHoughGuil> {
// identifier: cv_createGeneralizedHoughGuil
  unsafe {
    let rv = sys::cv_imgproc_cv_createGeneralizedHoughGuil();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfGeneralizedHoughGuil { ptr: rv.result })
    }
  }
}

/// This function computes a Hanning window coefficients in two dimensions.
/// 
/// See (http://en.wikipedia.org/wiki/Hann_function) and (http://en.wikipedia.org/wiki/Window_function)
/// for more information.
/// 
/// An example is shown below:
/// ```ignore
/// // create hanning window of size 100x100 and type CV_32F
/// Mat hann;
/// createHanningWindow(hann, Size(100, 100), CV_32F);
/// ```
/// 
/// ## Parameters
/// * dst: Destination array to place Hann coefficients in
/// * winSize: The window size specifications (both width and height must be > 1)
/// * type: Created array type
pub fn create_hanning_window(dst: &mut core::Mat, win_size: core::Size, _type: i32) -> Result<()> {
// identifier: cv_createHanningWindow_Mat_dst_Size_winSize_int_type
  unsafe {
    let rv = sys::cv_imgproc_cv_createHanningWindow_Mat_dst_Size_winSize_int_type(dst.as_raw_Mat(), win_size, _type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Creates a smart pointer to a LineSegmentDetector object and initializes it.
/// 
/// The LineSegmentDetector algorithm is defined using the standard values. Only advanced users may want
/// to edit those, as to tailor it for their own application.
/// 
/// ## Parameters
/// * _refine: The way found lines will be refined, see #LineSegmentDetectorModes
/// * _scale: The scale of the image that will be used to find the lines. Range (0..1].
/// * _sigma_scale: Sigma for Gaussian filter. It is computed as sigma = _sigma_scale/_scale.
/// * _quant: Bound to the quantization error on the gradient norm.
/// * _ang_th: Gradient angle tolerance in degrees.
/// * _log_eps: Detection threshold: -log10(NFA) \> log_eps. Used only when advance refinement
/// is chosen.
/// * _density_th: Minimal density of aligned region points in the enclosing rectangle.
/// * _n_bins: Number of bins in pseudo-ordering of gradient modulus.
///
/// ## C++ default parameters:
/// * _refine: LSD_REFINE_STD
/// * _scale: 0.8
/// * _sigma_scale: 0.6
/// * _quant: 2.0
/// * _ang_th: 22.5
/// * _log_eps: 0
/// * _density_th: 0.7
/// * _n_bins: 1024
pub fn create_line_segment_detector(_refine: i32, _scale: f64, _sigma_scale: f64, _quant: f64, _ang_th: f64, _log_eps: f64, _density_th: f64, _n_bins: i32) -> Result<types::PtrOfLineSegmentDetector> {
// identifier: cv_createLineSegmentDetector_int__refine_double__scale_double__sigma_scale_double__quant_double__ang_th_double__log_eps_double__density_th_int__n_bins
  unsafe {
    let rv = sys::cv_imgproc_cv_createLineSegmentDetector_int__refine_double__scale_double__sigma_scale_double__quant_double__ang_th_double__log_eps_double__density_th_int__n_bins(_refine, _scale, _sigma_scale, _quant, _ang_th, _log_eps, _density_th, _n_bins);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfLineSegmentDetector { ptr: rv.result })
    }
  }
}

/// Converts an image from one color space to another where the source image is
/// stored in two planes.
/// 
/// This function only supports YUV420 to RGB conversion as of now.
/// 
/// ## Parameters
/// * src1:: 8-bit image (#CV_8U) of the Y plane.
/// * src2:: image containing interleaved U/V plane.
/// * dst:: output image.
/// * code:: Specifies the type of conversion. It can take any of the following values:
/// - #COLOR_YUV2BGR_NV12
/// - #COLOR_YUV2RGB_NV12
/// - #COLOR_YUV2BGRA_NV12
/// - #COLOR_YUV2RGBA_NV12
/// - #COLOR_YUV2BGR_NV21
/// - #COLOR_YUV2RGB_NV21
/// - #COLOR_YUV2BGRA_NV21
/// - #COLOR_YUV2RGBA_NV21
pub fn cvt_color_two_plane(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat, code: i32) -> Result<()> {
// identifier: cv_cvtColorTwoPlane_Mat_src1_Mat_src2_Mat_dst_int_code
  unsafe {
    let rv = sys::cv_imgproc_cv_cvtColorTwoPlane_Mat_src1_Mat_src2_Mat_dst_int_code(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat(), code);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Converts an image from one color space to another.
/// 
/// The function converts an input image from one color space to another. In case of a transformation
/// to-from RGB color space, the order of the channels should be specified explicitly (RGB or BGR). Note
/// that the default color format in OpenCV is often referred to as RGB but it is actually BGR (the
/// bytes are reversed). So the first byte in a standard (24-bit) color image will be an 8-bit Blue
/// component, the second byte will be Green, and the third byte will be Red. The fourth, fifth, and
/// sixth bytes would then be the second pixel (Blue, then Green, then Red), and so on.
/// 
/// The conventional ranges for R, G, and B channel values are:
/// *   0 to 255 for CV_8U images
/// *   0 to 65535 for CV_16U images
/// *   0 to 1 for CV_32F images
/// 
/// In case of linear transformations, the range does not matter. But in case of a non-linear
/// transformation, an input RGB image should be normalized to the proper value range to get the correct
/// results, for example, for RGB <span lang='latex'>\rightarrow</span> L\*u\*v\* transformation. For example, if you have a
/// 32-bit floating-point image directly converted from an 8-bit image without any scaling, then it will
/// have the 0..255 value range instead of 0..1 assumed by the function. So, before calling #cvtColor ,
/// you need first to scale the image down:
/// ```ignore
/// img *= 1./255;
/// cvtColor(img, img, COLOR_BGR2Luv);
/// ```
/// 
/// If you use #cvtColor with 8-bit images, the conversion will have some information lost. For many
/// applications, this will not be noticeable but it is recommended to use 32-bit images in applications
/// that need the full range of colors or that convert an image before an operation and then convert
/// back.
/// 
/// If conversion adds the alpha channel, its value will set to the maximum of corresponding channel
/// range: 255 for CV_8U, 65535 for CV_16U, 1 for CV_32F.
/// 
/// ## Parameters
/// * src: input image: 8-bit unsigned, 16-bit unsigned ( CV_16UC... ), or single-precision
/// floating-point.
/// * dst: output image of the same size and depth as src.
/// * code: color space conversion code (see #ColorConversionCodes).
/// * dstCn: number of channels in the destination image; if the parameter is 0, the number of the
/// channels is derived automatically from src and code.
/// 
/// @see @ref imgproc_color_conversions
///
/// ## C++ default parameters:
/// * dst_cn: 0
pub fn cvt_color(src: &core::Mat, dst: &mut core::Mat, code: i32, dst_cn: i32) -> Result<()> {
// identifier: cv_cvtColor_Mat_src_Mat_dst_int_code_int_dstCn
  unsafe {
    let rv = sys::cv_imgproc_cv_cvtColor_Mat_src_Mat_dst_int_code_int_dstCn(src.as_raw_Mat(), dst.as_raw_Mat(), code, dst_cn);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// main function for all demosaicing processes
/// 
/// ## Parameters
/// * src: input image: 8-bit unsigned or 16-bit unsigned.
/// * dst: output image of the same size and depth as src.
/// * code: Color space conversion code (see the description below).
/// * dstCn: number of channels in the destination image; if the parameter is 0, the number of the
/// channels is derived automatically from src and code.
/// 
/// The function can do the following transformations:
/// 
/// *   Demosaicing using bilinear interpolation
/// 
/// #COLOR_BayerBG2BGR , #COLOR_BayerGB2BGR , #COLOR_BayerRG2BGR , #COLOR_BayerGR2BGR
/// 
/// #COLOR_BayerBG2GRAY , #COLOR_BayerGB2GRAY , #COLOR_BayerRG2GRAY , #COLOR_BayerGR2GRAY
/// 
/// *   Demosaicing using Variable Number of Gradients.
/// 
/// #COLOR_BayerBG2BGR_VNG , #COLOR_BayerGB2BGR_VNG , #COLOR_BayerRG2BGR_VNG , #COLOR_BayerGR2BGR_VNG
/// 
/// *   Edge-Aware Demosaicing.
/// 
/// #COLOR_BayerBG2BGR_EA , #COLOR_BayerGB2BGR_EA , #COLOR_BayerRG2BGR_EA , #COLOR_BayerGR2BGR_EA
/// 
/// *   Demosaicing with alpha channel
/// 
/// #COLOR_BayerBG2BGRA , #COLOR_BayerGB2BGRA , #COLOR_BayerRG2BGRA , #COLOR_BayerGR2BGRA
/// 
/// @sa cvtColor
///
/// ## C++ default parameters:
/// * dst_cn: 0
pub fn demosaicing(src: &core::Mat, dst: &mut core::Mat, code: i32, dst_cn: i32) -> Result<()> {
// identifier: cv_demosaicing_Mat_src_Mat_dst_int_code_int_dstCn
  unsafe {
    let rv = sys::cv_imgproc_cv_demosaicing_Mat_src_Mat_dst_int_code_int_dstCn(src.as_raw_Mat(), dst.as_raw_Mat(), code, dst_cn);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Dilates an image by using a specific structuring element.
/// 
/// The function dilates the source image using the specified structuring element that determines the
/// shape of a pixel neighborhood over which the maximum is taken:
/// <div lang='latex'>\texttt{dst} (x,y) =  \max _{(x',y'):  \, \texttt{element} (x',y') \ne0 } \texttt{src} (x+x',y+y')</div>
/// 
/// The function supports the in-place mode. Dilation can be applied several ( iterations ) times. In
/// case of multi-channel images, each channel is processed independently.
/// 
/// ## Parameters
/// * src: input image; the number of channels can be arbitrary, but the depth should be one of
/// CV_8U, CV_16U, CV_16S, CV_32F or CV_64F.
/// * dst: output image of the same size and type as src.
/// * kernel: structuring element used for dilation; if elemenat=Mat(), a 3 x 3 rectangular
/// structuring element is used. Kernel can be created using #getStructuringElement
/// * anchor: position of the anchor within the element; default value (-1, -1) means that the
/// anchor is at the element center.
/// * iterations: number of times dilation is applied.
/// * borderType: pixel extrapolation method, see #BorderTypes
/// * borderValue: border value in case of a constant border
/// @sa  erode, morphologyEx, getStructuringElement
///
/// ## C++ default parameters:
/// * anchor: Point(-1,-1)
/// * iterations: 1
/// * border_type: BORDER_CONSTANT
/// * border_value: morphologyDefaultBorderValue()
pub fn dilate(src: &core::Mat, dst: &mut core::Mat, kernel: &core::Mat, anchor: core::Point, iterations: i32, border_type: i32, border_value: core::Scalar) -> Result<()> {
// identifier: cv_dilate_Mat_src_Mat_dst_Mat_kernel_Point_anchor_int_iterations_int_borderType_Scalar_borderValue
  unsafe {
    let rv = sys::cv_imgproc_cv_dilate_Mat_src_Mat_dst_Mat_kernel_Point_anchor_int_iterations_int_borderType_Scalar_borderValue(src.as_raw_Mat(), dst.as_raw_Mat(), kernel.as_raw_Mat(), anchor, iterations, border_type, border_value);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates the distance to the closest zero pixel for each pixel of the source image.
/// 
/// The function cv::distanceTransform calculates the approximate or precise distance from every binary
/// image pixel to the nearest zero pixel. For zero image pixels, the distance will obviously be zero.
/// 
/// When maskSize == #DIST_MASK_PRECISE and distanceType == #DIST_L2 , the function runs the
/// algorithm described in @cite Felzenszwalb04 . This algorithm is parallelized with the TBB library.
/// 
/// In other cases, the algorithm @cite Borgefors86 is used. This means that for a pixel the function
/// finds the shortest path to the nearest zero pixel consisting of basic shifts: horizontal, vertical,
/// diagonal, or knight's move (the latest is available for a <span lang='latex'>5\times 5</span> mask). The overall
/// distance is calculated as a sum of these basic distances. Since the distance function should be
/// symmetric, all of the horizontal and vertical shifts must have the same cost (denoted as a ), all
/// the diagonal shifts must have the same cost (denoted as `b`), and all knight's moves must have the
/// same cost (denoted as `c`). For the #DIST_C and #DIST_L1 types, the distance is calculated
/// precisely, whereas for #DIST_L2 (Euclidean distance) the distance can be calculated only with a
/// relative error (a <span lang='latex'>5\times 5</span> mask gives more accurate results). For `a`,`b`, and `c`, OpenCV
/// uses the values suggested in the original paper:
/// - DIST_L1: `a = 1, b = 2`
/// - DIST_L2:
/// - `3 x 3`: `a=0.955, b=1.3693`
/// - `5 x 5`: `a=1, b=1.4, c=2.1969`
/// - DIST_C: `a = 1, b = 1`
/// 
/// Typically, for a fast, coarse distance estimation #DIST_L2, a <span lang='latex'>3\times 3</span> mask is used. For a
/// more accurate distance estimation #DIST_L2, a <span lang='latex'>5\times 5</span> mask or the precise algorithm is used.
/// Note that both the precise and the approximate algorithms are linear on the number of pixels.
/// 
/// This variant of the function does not only compute the minimum distance for each pixel <span lang='latex'>(x, y)</span>
/// but also identifies the nearest connected component consisting of zero pixels
/// (labelType==#DIST_LABEL_CCOMP) or the nearest zero pixel (labelType==#DIST_LABEL_PIXEL). Index of the
/// component/pixel is stored in `labels(x, y)`. When labelType==#DIST_LABEL_CCOMP, the function
/// automatically finds connected components of zero pixels in the input image and marks them with
/// distinct labels. When labelType==#DIST_LABEL_CCOMP, the function scans through the input image and
/// marks all the zero pixels with distinct labels.
/// 
/// In this mode, the complexity is still linear. That is, the function provides a very fast way to
/// compute the Voronoi diagram for a binary image. Currently, the second variant can use only the
/// approximate distance transform algorithm, i.e. maskSize=#DIST_MASK_PRECISE is not supported
/// yet.
/// 
/// ## Parameters
/// * src: 8-bit, single-channel (binary) source image.
/// * dst: Output image with calculated distances. It is a 8-bit or 32-bit floating-point,
/// single-channel image of the same size as src.
/// * labels: Output 2D array of labels (the discrete Voronoi diagram). It has the type
/// CV_32SC1 and the same size as src.
/// * distanceType: Type of distance, see #DistanceTypes
/// * maskSize: Size of the distance transform mask, see #DistanceTransformMasks.
/// #DIST_MASK_PRECISE is not supported by this variant. In case of the #DIST_L1 or #DIST_C distance type,
/// the parameter is forced to 3 because a <span lang='latex'>3\times 3</span> mask gives the same result as \f$5\times
/// 5\f$ or any larger aperture.
/// * labelType: Type of the label array to build, see #DistanceTransformLabelTypes.
///
/// ## C++ default parameters:
/// * label_type: DIST_LABEL_CCOMP
pub fn distance_transform_labels(src: &core::Mat, dst: &mut core::Mat, labels: &mut core::Mat, distance_type: i32, mask_size: i32, label_type: i32) -> Result<()> {
// identifier: cv_distanceTransform_Mat_src_Mat_dst_Mat_labels_int_distanceType_int_maskSize_int_labelType
  unsafe {
    let rv = sys::cv_imgproc_cv_distanceTransform_Mat_src_Mat_dst_Mat_labels_int_distanceType_int_maskSize_int_labelType(src.as_raw_Mat(), dst.as_raw_Mat(), labels.as_raw_Mat(), distance_type, mask_size, label_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
/// ## Parameters
/// * src: 8-bit, single-channel (binary) source image.
/// * dst: Output image with calculated distances. It is a 8-bit or 32-bit floating-point,
/// single-channel image of the same size as src .
/// * distanceType: Type of distance, see #DistanceTypes
/// * maskSize: Size of the distance transform mask, see #DistanceTransformMasks. In case of the
/// #DIST_L1 or #DIST_C distance type, the parameter is forced to 3 because a <span lang='latex'>3\times 3</span> mask gives
/// the same result as <span lang='latex'>5\times 5</span> or any larger aperture.
/// * dstType: Type of output image. It can be CV_8U or CV_32F. Type CV_8U can be used only for
/// the first variant of the function and distanceType == #DIST_L1.
///
/// ## C++ default parameters:
/// * dst_type: CV_32F
pub fn distance_transform(src: &core::Mat, dst: &mut core::Mat, distance_type: i32, mask_size: i32, dst_type: i32) -> Result<()> {
// identifier: cv_distanceTransform_Mat_src_Mat_dst_int_distanceType_int_maskSize_int_dstType
  unsafe {
    let rv = sys::cv_imgproc_cv_distanceTransform_Mat_src_Mat_dst_int_distanceType_int_maskSize_int_dstType(src.as_raw_Mat(), dst.as_raw_Mat(), distance_type, mask_size, dst_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Draws contours outlines or filled contours.
/// 
/// The function draws contour outlines in the image if <span lang='latex'>\texttt{thickness} \ge 0</span> or fills the area
/// bounded by the contours if <span lang='latex'>\texttt{thickness}<0</span> . The example below shows how to retrieve
/// connected components from the binary image and label them: :
/// @include snippets/imgproc_drawContours.cpp
/// 
/// ## Parameters
/// * image: Destination image.
/// * contours: All the input contours. Each contour is stored as a point vector.
/// * contourIdx: Parameter indicating a contour to draw. If it is negative, all the contours are drawn.
/// * color: Color of the contours.
/// * thickness: Thickness of lines the contours are drawn with. If it is negative (for example,
/// thickness=#FILLED ), the contour interiors are drawn.
/// * lineType: Line connectivity. See #LineTypes
/// * hierarchy: Optional information about hierarchy. It is only needed if you want to draw only
/// some of the contours (see maxLevel ).
/// * maxLevel: Maximal level for drawn contours. If it is 0, only the specified contour is drawn.
/// If it is 1, the function draws the contour(s) and all the nested contours. If it is 2, the function
/// draws the contours, all the nested contours, all the nested-to-nested contours, and so on. This
/// parameter is only taken into account when there is hierarchy available.
/// * offset: Optional contour shift parameter. Shift all the drawn contours by the specified
/// <span lang='latex'>\texttt{offset}=(dx,dy)</span> .
/// 
/// Note: When thickness=#FILLED, the function is designed to handle connected components with holes correctly
/// even when no hierarchy date is provided. This is done by analyzing all the outlines together
/// using even-odd rule. This may give incorrect results if you have a joint collection of separately retrieved
/// contours. In order to solve this problem, you need to call #drawContours separately for each sub-group
/// of contours, or iterate over the collection using contourIdx parameter.
///
/// ## C++ default parameters:
/// * thickness: 1
/// * line_type: LINE_8
/// * hierarchy: noArray()
/// * max_level: INT_MAX
/// * offset: Point()
pub fn draw_contours(image: &mut core::Mat, contours: &types::VectorOfMat, contour_idx: i32, color: core::Scalar, thickness: i32, line_type: i32, hierarchy: &core::Mat, max_level: i32, offset: core::Point) -> Result<()> {
// identifier: cv_drawContours_Mat_image_VectorOfMat_contours_int_contourIdx_Scalar_color_int_thickness_int_lineType_Mat_hierarchy_int_maxLevel_Point_offset
  unsafe {
    let rv = sys::cv_imgproc_cv_drawContours_Mat_image_VectorOfMat_contours_int_contourIdx_Scalar_color_int_thickness_int_lineType_Mat_hierarchy_int_maxLevel_Point_offset(image.as_raw_Mat(), contours.as_raw_VectorOfMat(), contour_idx, color, thickness, line_type, hierarchy.as_raw_Mat(), max_level, offset);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Draws a marker on a predefined position in an image.
/// 
/// The function cv::drawMarker draws a marker on a given position in the image. For the moment several
/// marker types are supported, see #MarkerTypes for more information.
/// 
/// ## Parameters
/// * img: Image.
/// * position: The point where the crosshair is positioned.
/// * color: Line color.
/// * markerType: The specific type of marker you want to use, see #MarkerTypes
/// * thickness: Line thickness.
/// * line_type: Type of the line, See #LineTypes
/// * markerSize: The length of the marker axis [default = 20 pixels]
///
/// ## C++ default parameters:
/// * marker_type: MARKER_CROSS
/// * marker_size: 20
/// * thickness: 1
/// * line_type: 8
pub fn draw_marker(img: &core::Mat, position: core::Point, color: core::Scalar, marker_type: i32, marker_size: i32, thickness: i32, line_type: i32) -> Result<()> {
// identifier: cv_drawMarker_Mat_img_Point_position_Scalar_color_int_markerType_int_markerSize_int_thickness_int_line_type
  unsafe {
    let rv = sys::cv_imgproc_cv_drawMarker_Mat_img_Point_position_Scalar_color_int_markerType_int_markerSize_int_thickness_int_line_type(img.as_raw_Mat(), position, color, marker_type, marker_size, thickness, line_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
/// ## Parameters
/// * center: Center of the arc.
/// * axes: Half of the size of the ellipse main axes. See #ellipse for details.
/// * angle: Rotation angle of the ellipse in degrees. See #ellipse for details.
/// * arcStart: Starting angle of the elliptic arc in degrees.
/// * arcEnd: Ending angle of the elliptic arc in degrees.
/// * delta: Angle between the subsequent polyline vertices. It defines the approximation accuracy.
/// * pts: Output vector of polyline vertices.
pub fn ellipse2_poly_f64(center: core::Point2d, axes: core::Size2d, angle: i32, arc_start: i32, arc_end: i32, delta: i32, pts: &types::VectorOfPoint2d) -> Result<()> {
// identifier: cv_ellipse2Poly_Point2d_center_Size2d_axes_int_angle_int_arcStart_int_arcEnd_int_delta_VectorOfPoint2d_pts
  unsafe {
    let rv = sys::cv_imgproc_cv_ellipse2Poly_Point2d_center_Size2d_axes_int_angle_int_arcStart_int_arcEnd_int_delta_VectorOfPoint2d_pts(center, axes, angle, arc_start, arc_end, delta, pts.as_raw_VectorOfPoint2d());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Approximates an elliptic arc with a polyline.
/// 
/// The function ellipse2Poly computes the vertices of a polyline that approximates the specified
/// elliptic arc. It is used by #ellipse. If `arcStart` is greater than `arcEnd`, they are swapped.
/// 
/// ## Parameters
/// * center: Center of the arc.
/// * axes: Half of the size of the ellipse main axes. See #ellipse for details.
/// * angle: Rotation angle of the ellipse in degrees. See #ellipse for details.
/// * arcStart: Starting angle of the elliptic arc in degrees.
/// * arcEnd: Ending angle of the elliptic arc in degrees.
/// * delta: Angle between the subsequent polyline vertices. It defines the approximation
/// accuracy.
/// * pts: Output vector of polyline vertices.
pub fn ellipse2_poly(center: core::Point, axes: core::Size, angle: i32, arc_start: i32, arc_end: i32, delta: i32, pts: &types::VectorOfPoint) -> Result<()> {
// identifier: cv_ellipse2Poly_Point_center_Size_axes_int_angle_int_arcStart_int_arcEnd_int_delta_VectorOfPoint_pts
  unsafe {
    let rv = sys::cv_imgproc_cv_ellipse2Poly_Point_center_Size_axes_int_angle_int_arcStart_int_arcEnd_int_delta_VectorOfPoint_pts(center, axes, angle, arc_start, arc_end, delta, pts.as_raw_VectorOfPoint());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Draws a simple or thick elliptic arc or fills an ellipse sector.
/// 
/// The function cv::ellipse with more parameters draws an ellipse outline, a filled ellipse, an elliptic
/// arc, or a filled ellipse sector. The drawing code uses general parametric form.
/// A piecewise-linear curve is used to approximate the elliptic arc
/// boundary. If you need more control of the ellipse rendering, you can retrieve the curve using
/// #ellipse2Poly and then render it with #polylines or fill it with #fillPoly. If you use the first
/// variant of the function and want to draw the whole ellipse, not an arc, pass `startAngle=0` and
/// `endAngle=360`. If `startAngle` is greater than `endAngle`, they are swapped. The figure below explains
/// the meaning of the parameters to draw the blue arc.
/// 
/// ![Parameters of Elliptic Arc](pics/ellipse.svg)
/// 
/// ## Parameters
/// * img: Image.
/// * center: Center of the ellipse.
/// * axes: Half of the size of the ellipse main axes.
/// * angle: Ellipse rotation angle in degrees.
/// * startAngle: Starting angle of the elliptic arc in degrees.
/// * endAngle: Ending angle of the elliptic arc in degrees.
/// * color: Ellipse color.
/// * thickness: Thickness of the ellipse arc outline, if positive. Otherwise, this indicates that
/// a filled ellipse sector is to be drawn.
/// * lineType: Type of the ellipse boundary. See #LineTypes
/// * shift: Number of fractional bits in the coordinates of the center and values of axes.
///
/// ## C++ default parameters:
/// * thickness: 1
/// * line_type: LINE_8
/// * shift: 0
pub fn ellipse(img: &mut core::Mat, center: core::Point, axes: core::Size, angle: f64, start_angle: f64, end_angle: f64, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> Result<()> {
// identifier: cv_ellipse_Mat_img_Point_center_Size_axes_double_angle_double_startAngle_double_endAngle_Scalar_color_int_thickness_int_lineType_int_shift
  unsafe {
    let rv = sys::cv_imgproc_cv_ellipse_Mat_img_Point_center_Size_axes_double_angle_double_startAngle_double_endAngle_Scalar_color_int_thickness_int_lineType_int_shift(img.as_raw_Mat(), center, axes, angle, start_angle, end_angle, color, thickness, line_type, shift);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
/// ## Parameters
/// * img: Image.
/// * box: Alternative ellipse representation via RotatedRect. This means that the function draws
/// an ellipse inscribed in the rotated rectangle.
/// * color: Ellipse color.
/// * thickness: Thickness of the ellipse arc outline, if positive. Otherwise, this indicates that
/// a filled ellipse sector is to be drawn.
/// * lineType: Type of the ellipse boundary. See #LineTypes
///
/// ## C++ default parameters:
/// * thickness: 1
/// * line_type: LINE_8
pub fn ellipse_new_rotated_rect(img: &mut core::Mat, _box: &core::RotatedRect, color: core::Scalar, thickness: i32, line_type: i32) -> Result<()> {
// identifier: cv_ellipse_Mat_img_RotatedRect_box_Scalar_color_int_thickness_int_lineType
  unsafe {
    let rv = sys::cv_imgproc_cv_ellipse_Mat_img_RotatedRect_box_Scalar_color_int_thickness_int_lineType(img.as_raw_Mat(), _box.as_raw_RotatedRect(), color, thickness, line_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Equalizes the histogram of a grayscale image.
/// 
/// The function equalizes the histogram of the input image using the following algorithm:
/// 
/// - Calculate the histogram <span lang='latex'>H</span> for src .
/// - Normalize the histogram so that the sum of histogram bins is 255.
/// - Compute the integral of the histogram:
/// <div lang='latex'>H'_i =  \sum _{0  \le j < i} H(j)</div>
/// - Transform the image using <span lang='latex'>H'</span> as a look-up table: <span lang='latex'>\texttt{dst}(x,y) = H'(\texttt{src}(x,y))</span>
/// 
/// The algorithm normalizes the brightness and increases the contrast of the image.
/// 
/// ## Parameters
/// * src: Source 8-bit single channel image.
/// * dst: Destination image of the same size and type as src .
pub fn equalize_hist(src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
// identifier: cv_equalizeHist_Mat_src_Mat_dst
  unsafe {
    let rv = sys::cv_imgproc_cv_equalizeHist_Mat_src_Mat_dst(src.as_raw_Mat(), dst.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Erodes an image by using a specific structuring element.
/// 
/// The function erodes the source image using the specified structuring element that determines the
/// shape of a pixel neighborhood over which the minimum is taken:
/// 
/// <div lang='latex'>\texttt{dst} (x,y) =  \min _{(x',y'):  \, \texttt{element} (x',y') \ne0 } \texttt{src} (x+x',y+y')</div>
/// 
/// The function supports the in-place mode. Erosion can be applied several ( iterations ) times. In
/// case of multi-channel images, each channel is processed independently.
/// 
/// ## Parameters
/// * src: input image; the number of channels can be arbitrary, but the depth should be one of
/// CV_8U, CV_16U, CV_16S, CV_32F or CV_64F.
/// * dst: output image of the same size and type as src.
/// * kernel: structuring element used for erosion; if `element=Mat()`, a `3 x 3` rectangular
/// structuring element is used. Kernel can be created using #getStructuringElement.
/// * anchor: position of the anchor within the element; default value (-1, -1) means that the
/// anchor is at the element center.
/// * iterations: number of times erosion is applied.
/// * borderType: pixel extrapolation method, see #BorderTypes
/// * borderValue: border value in case of a constant border
/// @sa  dilate, morphologyEx, getStructuringElement
///
/// ## C++ default parameters:
/// * anchor: Point(-1,-1)
/// * iterations: 1
/// * border_type: BORDER_CONSTANT
/// * border_value: morphologyDefaultBorderValue()
pub fn erode(src: &core::Mat, dst: &mut core::Mat, kernel: &core::Mat, anchor: core::Point, iterations: i32, border_type: i32, border_value: core::Scalar) -> Result<()> {
// identifier: cv_erode_Mat_src_Mat_dst_Mat_kernel_Point_anchor_int_iterations_int_borderType_Scalar_borderValue
  unsafe {
    let rv = sys::cv_imgproc_cv_erode_Mat_src_Mat_dst_Mat_kernel_Point_anchor_int_iterations_int_borderType_Scalar_borderValue(src.as_raw_Mat(), dst.as_raw_Mat(), kernel.as_raw_Mat(), anchor, iterations, border_type, border_value);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Fills a convex polygon.
/// 
/// The function cv::fillConvexPoly draws a filled convex polygon. This function is much faster than the
/// function #fillPoly . It can fill not only convex polygons but any monotonic polygon without
/// self-intersections, that is, a polygon whose contour intersects every horizontal line (scan line)
/// twice at the most (though, its top-most and/or the bottom edge could be horizontal).
/// 
/// ## Parameters
/// * img: Image.
/// * points: Polygon vertices.
/// * color: Polygon color.
/// * lineType: Type of the polygon boundaries. See #LineTypes
/// * shift: Number of fractional bits in the vertex coordinates.
///
/// ## C++ default parameters:
/// * line_type: LINE_8
/// * shift: 0
pub fn fill_convex_poly(img: &mut core::Mat, points: &core::Mat, color: core::Scalar, line_type: i32, shift: i32) -> Result<()> {
// identifier: cv_fillConvexPoly_Mat_img_Mat_points_Scalar_color_int_lineType_int_shift
  unsafe {
    let rv = sys::cv_imgproc_cv_fillConvexPoly_Mat_img_Mat_points_Scalar_color_int_lineType_int_shift(img.as_raw_Mat(), points.as_raw_Mat(), color, line_type, shift);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Fills the area bounded by one or more polygons.
/// 
/// The function cv::fillPoly fills an area bounded by several polygonal contours. The function can fill
/// complex areas, for example, areas with holes, contours with self-intersections (some of their
/// parts), and so forth.
/// 
/// ## Parameters
/// * img: Image.
/// * pts: Array of polygons where each polygon is represented as an array of points.
/// * color: Polygon color.
/// * lineType: Type of the polygon boundaries. See #LineTypes
/// * shift: Number of fractional bits in the vertex coordinates.
/// * offset: Optional offset of all points of the contours.
///
/// ## C++ default parameters:
/// * line_type: LINE_8
/// * shift: 0
/// * offset: Point()
pub fn fill_poly(img: &mut core::Mat, pts: &types::VectorOfMat, color: core::Scalar, line_type: i32, shift: i32, offset: core::Point) -> Result<()> {
// identifier: cv_fillPoly_Mat_img_VectorOfMat_pts_Scalar_color_int_lineType_int_shift_Point_offset
  unsafe {
    let rv = sys::cv_imgproc_cv_fillPoly_Mat_img_VectorOfMat_pts_Scalar_color_int_lineType_int_shift_Point_offset(img.as_raw_Mat(), pts.as_raw_VectorOfMat(), color, line_type, shift, offset);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Convolves an image with the kernel.
/// 
/// The function applies an arbitrary linear filter to an image. In-place operation is supported. When
/// the aperture is partially outside the image, the function interpolates outlier pixel values
/// according to the specified border mode.
/// 
/// The function does actually compute correlation, not the convolution:
/// 
/// <div lang='latex'>\texttt{dst} (x,y) =  \sum _{ \stackrel{0\leq x' < \texttt{kernel.cols},}{0\leq y' < \texttt{kernel.rows}} }  \texttt{kernel} (x',y')* \texttt{src} (x+x'- \texttt{anchor.x} ,y+y'- \texttt{anchor.y} )</div>
/// 
/// That is, the kernel is not mirrored around the anchor point. If you need a real convolution, flip
/// the kernel using #flip and set the new anchor to `(kernel.cols - anchor.x - 1, kernel.rows -
/// anchor.y - 1)`.
/// 
/// The function uses the DFT-based algorithm in case of sufficiently large kernels (~`11 x 11` or
/// larger) and the direct algorithm for small kernels.
/// 
/// ## Parameters
/// * src: input image.
/// * dst: output image of the same size and the same number of channels as src.
/// * ddepth: desired depth of the destination image, see @ref filter_depths "combinations"
/// * kernel: convolution kernel (or rather a correlation kernel), a single-channel floating point
/// matrix; if you want to apply different kernels to different channels, split the image into
/// separate color planes using split and process them individually.
/// * anchor: anchor of the kernel that indicates the relative position of a filtered point within
/// the kernel; the anchor should lie within the kernel; default value (-1,-1) means that the anchor
/// is at the kernel center.
/// * delta: optional value added to the filtered pixels before storing them in dst.
/// * borderType: pixel extrapolation method, see #BorderTypes
/// @sa  sepFilter2D, dft, matchTemplate
///
/// ## C++ default parameters:
/// * anchor: Point(-1,-1)
/// * delta: 0
/// * border_type: BORDER_DEFAULT
pub fn filter2_d(src: &core::Mat, dst: &mut core::Mat, ddepth: i32, kernel: &core::Mat, anchor: core::Point, delta: f64, border_type: i32) -> Result<()> {
// identifier: cv_filter2D_Mat_src_Mat_dst_int_ddepth_Mat_kernel_Point_anchor_double_delta_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_filter2D_Mat_src_Mat_dst_int_ddepth_Mat_kernel_Point_anchor_double_delta_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), ddepth, kernel.as_raw_Mat(), anchor, delta, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds contours in a binary image.
/// 
/// The function retrieves contours from the binary image using the algorithm @cite Suzuki85 . The contours
/// are a useful tool for shape analysis and object detection and recognition. See squares.cpp in the
/// OpenCV sample directory.
/// 
/// Note: Since opencv 3.2 source image is not modified by this function.
/// 
/// ## Parameters
/// * image: Source, an 8-bit single-channel image. Non-zero pixels are treated as 1's. Zero
/// pixels remain 0's, so the image is treated as binary . You can use #compare, #inRange, #threshold ,
/// #adaptiveThreshold, #Canny, and others to create a binary image out of a grayscale or color one.
/// If mode equals to #RETR_CCOMP or #RETR_FLOODFILL, the input can also be a 32-bit integer image of labels (CV_32SC1).
/// * contours: Detected contours. Each contour is stored as a vector of points (e.g.
/// std::vector<std::vector<cv::Point> >).
/// * hierarchy: Optional output vector (e.g. std::vector<cv::Vec4i>), containing information about the image topology. It has
/// as many elements as the number of contours. For each i-th contour contours[i], the elements
/// hierarchy[i][0] , hierarchy[i][1] , hierarchy[i][2] , and hierarchy[i][3] are set to 0-based indices
/// in contours of the next and previous contours at the same hierarchical level, the first child
/// contour and the parent contour, respectively. If for the contour i there are no next, previous,
/// parent, or nested contours, the corresponding elements of hierarchy[i] will be negative.
/// * mode: Contour retrieval mode, see #RetrievalModes
/// * method: Contour approximation method, see #ContourApproximationModes
/// * offset: Optional offset by which every contour point is shifted. This is useful if the
/// contours are extracted from the image ROI and then they should be analyzed in the whole image
/// context.
///
/// ## C++ default parameters:
/// * offset: Point()
pub fn find_contours_with_hierarchy(image: &mut core::Mat, contours: &mut types::VectorOfMat, hierarchy: &mut core::Mat, mode: i32, method: i32, offset: core::Point) -> Result<()> {
// identifier: cv_findContours_Mat_image_VectorOfMat_contours_Mat_hierarchy_int_mode_int_method_Point_offset
  unsafe {
    let rv = sys::cv_imgproc_cv_findContours_Mat_image_VectorOfMat_contours_Mat_hierarchy_int_mode_int_method_Point_offset(image.as_raw_Mat(), contours.as_raw_VectorOfMat(), hierarchy.as_raw_Mat(), mode, method, offset);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
///
/// ## C++ default parameters:
/// * offset: Point()
pub fn find_contours(image: &mut core::Mat, contours: &mut types::VectorOfMat, mode: i32, method: i32, offset: core::Point) -> Result<()> {
// identifier: cv_findContours_Mat_image_VectorOfMat_contours_int_mode_int_method_Point_offset
  unsafe {
    let rv = sys::cv_imgproc_cv_findContours_Mat_image_VectorOfMat_contours_int_mode_int_method_Point_offset(image.as_raw_Mat(), contours.as_raw_VectorOfMat(), mode, method, offset);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Fits an ellipse around a set of 2D points.
/// 
/// The function calculates the ellipse that fits a set of 2D points.
/// It returns the rotated rectangle in which the ellipse is inscribed.
/// The Approximate Mean Square (AMS) proposed by @cite Taubin1991 is used.
/// 
/// For an ellipse, this basis set is <span lang='latex'> \chi= \left(x^2, x y, y^2, x, y, 1\right) </span>,
/// which is a set of six free coefficients <span lang='latex'> A^T=\left\{A_{\text{xx}},A_{\text{xy}},A_{\text{yy}},A_x,A_y,A_0\right\} </span>.
/// However, to specify an ellipse, all that is needed is five numbers; the major and minor axes lengths <span lang='latex'> (a,b) </span>,
/// the position <span lang='latex'> (x_0,y_0) </span>, and the orientation <span lang='latex'> \theta </span>. This is because the basis set includes lines,
/// quadratics, parabolic and hyperbolic functions as well as elliptical functions as possible fits.
/// If the fit is found to be a parabolic or hyperbolic function then the standard #fitEllipse method is used.
/// The AMS method restricts the fit to parabolic, hyperbolic and elliptical curves
/// by imposing the condition that <span lang='latex'> A^T ( D_x^T D_x  +   D_y^T D_y) A = 1 </span> where
/// the matrices <span lang='latex'> Dx </span> and <span lang='latex'> Dy </span> are the partial derivatives of the design matrix <span lang='latex'> D </span> with
/// respect to x and y. The matrices are formed row by row applying the following to
/// each of the points in the set:
/// \f{align*}{
/// D(i,:)&=\left\{x_i^2, x_i y_i, y_i^2, x_i, y_i, 1\right\} &
/// D_x(i,:)&=\left\{2 x_i,y_i,0,1,0,0\right\} &
/// D_y(i,:)&=\left\{0,x_i,2 y_i,0,1,0\right\}
/// \f}
/// The AMS method minimizes the cost function
/// \f{equation*}{
/// \epsilon ^2=\frac{ A^T D^T D A }{ A^T (D_x^T D_x +  D_y^T D_y) A^T }
/// \f}
/// 
/// The minimum cost is found by solving the generalized eigenvalue problem.
/// 
/// \f{equation*}{
/// D^T D A = \lambda  \left( D_x^T D_x +  D_y^T D_y\right) A
/// \f}
/// 
/// ## Parameters
/// * points: Input 2D point set, stored in std::vector\<\> or Mat
pub fn fit_ellipse_ams(points: &core::Mat) -> Result<core::RotatedRect> {
// identifier: cv_fitEllipseAMS_Mat_points
  unsafe {
    let rv = sys::cv_imgproc_cv_fitEllipseAMS_Mat_points(points.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::RotatedRect { ptr: rv.result })
    }
  }
}

/// Fits an ellipse around a set of 2D points.
/// 
/// The function calculates the ellipse that fits a set of 2D points.
/// It returns the rotated rectangle in which the ellipse is inscribed.
/// The Direct least square (Direct) method by @cite Fitzgibbon1999 is used.
/// 
/// For an ellipse, this basis set is <span lang='latex'> \chi= \left(x^2, x y, y^2, x, y, 1\right) </span>,
/// which is a set of six free coefficients <span lang='latex'> A^T=\left\{A_{\text{xx}},A_{\text{xy}},A_{\text{yy}},A_x,A_y,A_0\right\} </span>.
/// However, to specify an ellipse, all that is needed is five numbers; the major and minor axes lengths <span lang='latex'> (a,b) </span>,
/// the position <span lang='latex'> (x_0,y_0) </span>, and the orientation <span lang='latex'> \theta </span>. This is because the basis set includes lines,
/// quadratics, parabolic and hyperbolic functions as well as elliptical functions as possible fits.
/// The Direct method confines the fit to ellipses by ensuring that <span lang='latex'> 4 A_{xx} A_{yy}- A_{xy}^2 > 0 </span>.
/// The condition imposed is that <span lang='latex'> 4 A_{xx} A_{yy}- A_{xy}^2=1 </span> which satisfies the inequality
/// and as the coefficients can be arbitrarily scaled is not overly restrictive.
/// 
/// \f{equation*}{
/// \epsilon ^2= A^T D^T D A \quad \text{with} \quad A^T C A =1 \quad \text{and} \quad C=\left(\begin{matrix}
/// 0 & 0  & 2  & 0  & 0  &  0  \\
/// 0 & -1  & 0  & 0  & 0  &  0 \\
/// 2 & 0  & 0  & 0  & 0  &  0 \\
/// 0 & 0  & 0  & 0  & 0  &  0 \\
/// 0 & 0  & 0  & 0  & 0  &  0 \\
/// 0 & 0  & 0  & 0  & 0  &  0
/// \end{matrix} \right)
/// \f}
/// 
/// The minimum cost is found by solving the generalized eigenvalue problem.
/// 
/// \f{equation*}{
/// D^T D A = \lambda  \left( C\right) A
/// \f}
/// 
/// The system produces only one positive eigenvalue <span lang='latex'> \lambda</span> which is chosen as the solution
/// with its eigenvector <span lang='latex'>\mathbf{u}</span>. These are used to find the coefficients
/// 
/// \f{equation*}{
/// A = \sqrt{\frac{1}{\mathbf{u}^T C \mathbf{u}}}  \mathbf{u}
/// \f}
/// The scaling factor guarantees that  <span lang='latex'>A^T C A =1</span>.
/// 
/// ## Parameters
/// * points: Input 2D point set, stored in std::vector\<\> or Mat
pub fn fit_ellipse_direct(points: &core::Mat) -> Result<core::RotatedRect> {
// identifier: cv_fitEllipseDirect_Mat_points
  unsafe {
    let rv = sys::cv_imgproc_cv_fitEllipseDirect_Mat_points(points.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::RotatedRect { ptr: rv.result })
    }
  }
}

/// Fits an ellipse around a set of 2D points.
/// 
/// The function calculates the ellipse that fits (in a least-squares sense) a set of 2D points best of
/// all. It returns the rotated rectangle in which the ellipse is inscribed. The first algorithm described by @cite Fitzgibbon95
/// is used. Developer should keep in mind that it is possible that the returned
/// ellipse/rotatedRect data contains negative indices, due to the data points being close to the
/// border of the containing Mat element.
/// 
/// ## Parameters
/// * points: Input 2D point set, stored in std::vector\<\> or Mat
pub fn fit_ellipse(points: &core::Mat) -> Result<core::RotatedRect> {
// identifier: cv_fitEllipse_Mat_points
  unsafe {
    let rv = sys::cv_imgproc_cv_fitEllipse_Mat_points(points.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::RotatedRect { ptr: rv.result })
    }
  }
}

/// Fits a line to a 2D or 3D point set.
/// 
/// The function fitLine fits a line to a 2D or 3D point set by minimizing <span lang='latex'>\sum_i \rho(r_i)</span> where
/// <span lang='latex'>r_i</span> is a distance between the <span lang='latex'>i^{th}</span> point, the line and <span lang='latex'>\rho(r)</span> is a distance function, one
/// of the following:
/// *  DIST_L2
/// <div lang='latex'>\rho (r) = r^2/2  \quad \text{(the simplest and the fastest least-squares method)}</div>
/// - DIST_L1
/// <div lang='latex'>\rho (r) = r</div>
/// - DIST_L12
/// <div lang='latex'>\rho (r) = 2  \cdot ( \sqrt{1 + \frac{r^2}{2}} - 1)</div>
/// - DIST_FAIR
/// <div lang='latex'>\rho \left (r \right ) = C^2  \cdot \left (  \frac{r}{C} -  \log{\left(1 + \frac{r}{C}\right)} \right )  \quad \text{where} \quad C=1.3998</div>
/// - DIST_WELSCH
/// <div lang='latex'>\rho \left (r \right ) =  \frac{C^2}{2} \cdot \left ( 1 -  \exp{\left(-\left(\frac{r}{C}\right)^2\right)} \right )  \quad \text{where} \quad C=2.9846</div>
/// - DIST_HUBER
/// <div lang='latex'>\rho (r) =  \fork{r^2/2}{if \(r < C\)}{C \cdot (r-C/2)}{otherwise} \quad \text{where} \quad C=1.345</div>
/// 
/// The algorithm is based on the M-estimator ( <http://en.wikipedia.org/wiki/M-estimator> ) technique
/// that iteratively fits the line using the weighted least-squares algorithm. After each iteration the
/// weights <span lang='latex'>w_i</span> are adjusted to be inversely proportional to <span lang='latex'>\rho(r_i)</span> .
/// 
/// ## Parameters
/// * points: Input vector of 2D or 3D points, stored in std::vector\<\> or Mat.
/// * line: Output line parameters. In case of 2D fitting, it should be a vector of 4 elements
/// (like Vec4f) - (vx, vy, x0, y0), where (vx, vy) is a normalized vector collinear to the line and
/// (x0, y0) is a point on the line. In case of 3D fitting, it should be a vector of 6 elements (like
/// Vec6f) - (vx, vy, vz, x0, y0, z0), where (vx, vy, vz) is a normalized vector collinear to the line
/// and (x0, y0, z0) is a point on the line.
/// * distType: Distance used by the M-estimator, see #DistanceTypes
/// * param: Numerical parameter ( C ) for some types of distances. If it is 0, an optimal value
/// is chosen.
/// * reps: Sufficient accuracy for the radius (distance between the coordinate origin and the line).
/// * aeps: Sufficient accuracy for the angle. 0.01 would be a good default value for reps and aeps.
pub fn fit_line(points: &core::Mat, line: &mut core::Mat, dist_type: i32, param: f64, reps: f64, aeps: f64) -> Result<()> {
// identifier: cv_fitLine_Mat_points_Mat_line_int_distType_double_param_double_reps_double_aeps
  unsafe {
    let rv = sys::cv_imgproc_cv_fitLine_Mat_points_Mat_line_int_distType_double_param_double_reps_double_aeps(points.as_raw_Mat(), line.as_raw_Mat(), dist_type, param, reps, aeps);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

pub fn get_affine_transform(src: &core::Mat, dst: &core::Mat) -> Result<core::Mat> {
// identifier: cv_getAffineTransform_Mat_src_Mat_dst
  unsafe {
    let rv = sys::cv_imgproc_cv_getAffineTransform_Mat_src_Mat_dst(src.as_raw_Mat(), dst.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Returns the default new camera matrix.
/// 
/// The function returns the camera matrix that is either an exact copy of the input cameraMatrix (when
/// centerPrinicipalPoint=false ), or the modified one (when centerPrincipalPoint=true).
/// 
/// In the latter case, the new camera matrix will be:
/// 
/// <div lang='latex'>\begin{bmatrix} f_x && 0 && ( \texttt{imgSize.width} -1)*0.5  \\ 0 && f_y && ( \texttt{imgSize.height} -1)*0.5  \\ 0 && 0 && 1 \end{bmatrix} ,</div>
/// 
/// where <span lang='latex'>f_x</span> and <span lang='latex'>f_y</span> are <span lang='latex'>(0,0)</span> and <span lang='latex'>(1,1)</span> elements of cameraMatrix, respectively.
/// 
/// By default, the undistortion functions in OpenCV (see #initUndistortRectifyMap, #undistort) do not
/// move the principal point. However, when you work with stereo, it is important to move the principal
/// points in both views to the same y-coordinate (which is required by most of stereo correspondence
/// algorithms), and may be to the same x-coordinate too. So, you can form the new camera matrix for
/// each view where the principal points are located at the center.
/// 
/// ## Parameters
/// * cameraMatrix: Input camera matrix.
/// * imgsize: Camera view image size in pixels.
/// * centerPrincipalPoint: Location of the principal point in the new camera matrix. The
/// parameter indicates whether this location should be at the image center or not.
///
/// ## C++ default parameters:
/// * imgsize: Size()
/// * center_principal_point: false
pub fn get_default_new_camera_matrix(camera_matrix: &core::Mat, imgsize: core::Size, center_principal_point: bool) -> Result<core::Mat> {
// identifier: cv_getDefaultNewCameraMatrix_Mat_cameraMatrix_Size_imgsize_bool_centerPrincipalPoint
  unsafe {
    let rv = sys::cv_imgproc_cv_getDefaultNewCameraMatrix_Mat_cameraMatrix_Size_imgsize_bool_centerPrincipalPoint(camera_matrix.as_raw_Mat(), imgsize, center_principal_point);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Returns filter coefficients for computing spatial image derivatives.
/// 
/// The function computes and returns the filter coefficients for spatial image derivatives. When
/// `ksize=CV_SCHARR`, the Scharr <span lang='latex'>3 \times 3</span> kernels are generated (see #Scharr). Otherwise, Sobel
/// kernels are generated (see #Sobel). The filters are normally passed to #sepFilter2D or to
/// 
/// ## Parameters
/// * kx: Output matrix of row filter coefficients. It has the type ktype .
/// * ky: Output matrix of column filter coefficients. It has the type ktype .
/// * dx: Derivative order in respect of x.
/// * dy: Derivative order in respect of y.
/// * ksize: Aperture size. It can be CV_SCHARR, 1, 3, 5, or 7.
/// * normalize: Flag indicating whether to normalize (scale down) the filter coefficients or not.
/// Theoretically, the coefficients should have the denominator <span lang='latex'>=2^{ksize*2-dx-dy-2}</span>. If you are
/// going to filter floating-point images, you are likely to use the normalized kernels. But if you
/// compute derivatives of an 8-bit image, store the results in a 16-bit image, and wish to preserve
/// all the fractional bits, you may want to set normalize=false .
/// * ktype: Type of filter coefficients. It can be CV_32f or CV_64F .
///
/// ## C++ default parameters:
/// * normalize: false
/// * ktype: CV_32F
pub fn get_deriv_kernels(kx: &mut core::Mat, ky: &mut core::Mat, dx: i32, dy: i32, ksize: i32, normalize: bool, ktype: i32) -> Result<()> {
// identifier: cv_getDerivKernels_Mat_kx_Mat_ky_int_dx_int_dy_int_ksize_bool_normalize_int_ktype
  unsafe {
    let rv = sys::cv_imgproc_cv_getDerivKernels_Mat_kx_Mat_ky_int_dx_int_dy_int_ksize_bool_normalize_int_ktype(kx.as_raw_Mat(), ky.as_raw_Mat(), dx, dy, ksize, normalize, ktype);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates the font-specific size to use to achieve a given height in pixels.
/// 
/// ## Parameters
/// * fontFace: Font to use, see cv::HersheyFonts.
/// * pixelHeight: Pixel height to compute the fontScale for
/// * thickness: Thickness of lines used to render the text.See putText for details.
/// @return The fontSize to use for cv::putText
/// 
/// @see cv::putText
///
/// ## C++ default parameters:
/// * thickness: 1
pub fn get_font_scale_from_height(font_face: i32, pixel_height: i32, thickness: i32) -> Result<f64> {
// identifier: cv_getFontScaleFromHeight_int_fontFace_int_pixelHeight_int_thickness
  unsafe {
    let rv = sys::cv_imgproc_cv_getFontScaleFromHeight_int_fontFace_int_pixelHeight_int_thickness(font_face, pixel_height, thickness);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Returns Gabor filter coefficients.
/// 
/// For more details about gabor filter equations and parameters, see: [Gabor
/// Filter](http://en.wikipedia.org/wiki/Gabor_filter).
/// 
/// ## Parameters
/// * ksize: Size of the filter returned.
/// * sigma: Standard deviation of the gaussian envelope.
/// * theta: Orientation of the normal to the parallel stripes of a Gabor function.
/// * lambd: Wavelength of the sinusoidal factor.
/// * gamma: Spatial aspect ratio.
/// * psi: Phase offset.
/// * ktype: Type of filter coefficients. It can be CV_32F or CV_64F .
///
/// ## C++ default parameters:
/// * psi: CV_PI*0.5
/// * ktype: CV_64F
pub fn get_gabor_kernel(ksize: core::Size, sigma: f64, theta: f64, lambd: f64, gamma: f64, psi: f64, ktype: i32) -> Result<core::Mat> {
// identifier: cv_getGaborKernel_Size_ksize_double_sigma_double_theta_double_lambd_double_gamma_double_psi_int_ktype
  unsafe {
    let rv = sys::cv_imgproc_cv_getGaborKernel_Size_ksize_double_sigma_double_theta_double_lambd_double_gamma_double_psi_int_ktype(ksize, sigma, theta, lambd, gamma, psi, ktype);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Returns Gaussian filter coefficients.
/// 
/// The function computes and returns the <span lang='latex'>\texttt{ksize} \times 1</span> matrix of Gaussian filter
/// coefficients:
/// 
/// <div lang='latex'>G_i= \alpha *e^{-(i-( \texttt{ksize} -1)/2)^2/(2* \texttt{sigma}^2)},</div>
/// 
/// where <span lang='latex'>i=0..\texttt{ksize}-1</span> and <span lang='latex'>\alpha</span> is the scale factor chosen so that <span lang='latex'>\sum_i G_i=1</span>.
/// 
/// Two of such generated kernels can be passed to sepFilter2D. Those functions automatically recognize
/// smoothing kernels (a symmetrical kernel with sum of weights equal to 1) and handle them accordingly.
/// You may also use the higher-level GaussianBlur.
/// ## Parameters
/// * ksize: Aperture size. It should be odd ( <span lang='latex'>\texttt{ksize} \mod 2 = 1</span> ) and positive.
/// * sigma: Gaussian standard deviation. If it is non-positive, it is computed from ksize as
/// `sigma = 0.3*((ksize-1)*0.5 - 1) + 0.8`.
/// * ktype: Type of filter coefficients. It can be CV_32F or CV_64F .
/// @sa  sepFilter2D, getDerivKernels, getStructuringElement, GaussianBlur
///
/// ## C++ default parameters:
/// * ktype: CV_64F
pub fn get_gaussian_kernel(ksize: i32, sigma: f64, ktype: i32) -> Result<core::Mat> {
// identifier: cv_getGaussianKernel_int_ksize_double_sigma_int_ktype
  unsafe {
    let rv = sys::cv_imgproc_cv_getGaussianKernel_int_ksize_double_sigma_int_ktype(ksize, sigma, ktype);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Calculates a perspective transform from four pairs of the corresponding points.
/// 
/// The function calculates the <span lang='latex'>3 \times 3</span> matrix of a perspective transform so that:
/// 
/// <div lang='latex'>\begin{bmatrix} t_i x'_i \\ t_i y'_i \\ t_i \end{bmatrix} = \texttt{map_matrix} \cdot \begin{bmatrix} x_i \\ y_i \\ 1 \end{bmatrix}</div>
/// 
/// where
/// 
/// <div lang='latex'>dst(i)=(x'_i,y'_i), src(i)=(x_i, y_i), i=0,1,2,3</div>
/// 
/// ## Parameters
/// * src: Coordinates of quadrangle vertices in the source image.
/// * dst: Coordinates of the corresponding quadrangle vertices in the destination image.
/// 
/// @sa  findHomography, warpPerspective, perspectiveTransform
pub fn get_perspective_transform(src: &core::Mat, dst: &core::Mat) -> Result<core::Mat> {
// identifier: cv_getPerspectiveTransform_Mat_src_Mat_dst
  unsafe {
    let rv = sys::cv_imgproc_cv_getPerspectiveTransform_Mat_src_Mat_dst(src.as_raw_Mat(), dst.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Retrieves a pixel rectangle from an image with sub-pixel accuracy.
/// 
/// The function getRectSubPix extracts pixels from src:
/// 
/// <div lang='latex'>patch(x, y) = src(x +  \texttt{center.x} - ( \texttt{dst.cols} -1)*0.5, y +  \texttt{center.y} - ( \texttt{dst.rows} -1)*0.5)</div>
/// 
/// where the values of the pixels at non-integer coordinates are retrieved using bilinear
/// interpolation. Every channel of multi-channel images is processed independently. Also
/// the image should be a single channel or three channel image. While the center of the
/// rectangle must be inside the image, parts of the rectangle may be outside.
/// 
/// ## Parameters
/// * image: Source image.
/// * patchSize: Size of the extracted patch.
/// * center: Floating point coordinates of the center of the extracted rectangle within the
/// source image. The center must be inside the image.
/// * patch: Extracted patch that has the size patchSize and the same number of channels as src .
/// * patchType: Depth of the extracted pixels. By default, they have the same depth as src .
/// 
/// @sa  warpAffine, warpPerspective
///
/// ## C++ default parameters:
/// * patch_type: -1
pub fn get_rect_sub_pix(image: &core::Mat, patch_size: core::Size, center: core::Point2f, patch: &mut core::Mat, patch_type: i32) -> Result<()> {
// identifier: cv_getRectSubPix_Mat_image_Size_patchSize_Point2f_center_Mat_patch_int_patchType
  unsafe {
    let rv = sys::cv_imgproc_cv_getRectSubPix_Mat_image_Size_patchSize_Point2f_center_Mat_patch_int_patchType(image.as_raw_Mat(), patch_size, center, patch.as_raw_Mat(), patch_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates an affine matrix of 2D rotation.
/// 
/// The function calculates the following matrix:
/// 
/// <div lang='latex'>\begin{bmatrix} \alpha &  \beta & (1- \alpha )  \cdot \texttt{center.x} -  \beta \cdot \texttt{center.y} \\ - \beta &  \alpha &  \beta \cdot \texttt{center.x} + (1- \alpha )  \cdot \texttt{center.y} \end{bmatrix}</div>
/// 
/// where
/// 
/// <div lang='latex'>\begin{array}{l} \alpha =  \texttt{scale} \cdot \cos \texttt{angle} , \\ \beta =  \texttt{scale} \cdot \sin \texttt{angle} \end{array}</div>
/// 
/// The transformation maps the rotation center to itself. If this is not the target, adjust the shift.
/// 
/// ## Parameters
/// * center: Center of the rotation in the source image.
/// * angle: Rotation angle in degrees. Positive values mean counter-clockwise rotation (the
/// coordinate origin is assumed to be the top-left corner).
/// * scale: Isotropic scale factor.
/// 
/// @sa  getAffineTransform, warpAffine, transform
pub fn get_rotation_matrix2_d(center: core::Point2f, angle: f64, scale: f64) -> Result<core::Mat> {
// identifier: cv_getRotationMatrix2D_Point2f_center_double_angle_double_scale
  unsafe {
    let rv = sys::cv_imgproc_cv_getRotationMatrix2D_Point2f_center_double_angle_double_scale(center, angle, scale);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Returns a structuring element of the specified size and shape for morphological operations.
/// 
/// The function constructs and returns the structuring element that can be further passed to #erode,
/// #dilate or #morphologyEx. But you can also construct an arbitrary binary mask yourself and use it as
/// the structuring element.
/// 
/// ## Parameters
/// * shape: Element shape that could be one of #MorphShapes
/// * ksize: Size of the structuring element.
/// * anchor: Anchor position within the element. The default value <span lang='latex'>(-1, -1)</span> means that the
/// anchor is at the center. Note that only the shape of a cross-shaped element depends on the anchor
/// position. In other cases the anchor just regulates how much the result of the morphological
/// operation is shifted.
///
/// ## C++ default parameters:
/// * anchor: Point(-1,-1)
pub fn get_structuring_element(shape: i32, ksize: core::Size, anchor: core::Point) -> Result<core::Mat> {
// identifier: cv_getStructuringElement_int_shape_Size_ksize_Point_anchor
  unsafe {
    let rv = sys::cv_imgproc_cv_getStructuringElement_int_shape_Size_ksize_Point_anchor(shape, ksize, anchor);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Determines strong corners on an image.
/// 
/// The function finds the most prominent corners in the image or in the specified image region, as
/// described in @cite Shi94
/// 
/// *   Function calculates the corner quality measure at every source image pixel using the
/// #cornerMinEigenVal or #cornerHarris .
/// *   Function performs a non-maximum suppression (the local maximums in *3 x 3* neighborhood are
/// retained).
/// *   The corners with the minimal eigenvalue less than
/// <span lang='latex'>\texttt{qualityLevel} \cdot \max_{x,y} qualityMeasureMap(x,y)</span> are rejected.
/// *   The remaining corners are sorted by the quality measure in the descending order.
/// *   Function throws away each corner for which there is a stronger corner at a distance less than
/// maxDistance.
/// 
/// The function can be used to initialize a point-based tracker of an object.
/// 
/// 
/// Note: If the function is called with different values A and B of the parameter qualityLevel , and
/// A \> B, the vector of returned corners with qualityLevel=A will be the prefix of the output vector
/// with qualityLevel=B .
/// 
/// ## Parameters
/// * image: Input 8-bit or floating-point 32-bit, single-channel image.
/// * corners: Output vector of detected corners.
/// * maxCorners: Maximum number of corners to return. If there are more corners than are found,
/// the strongest of them is returned. `maxCorners <= 0` implies that no limit on the maximum is set
/// and all detected corners are returned.
/// * qualityLevel: Parameter characterizing the minimal accepted quality of image corners. The
/// parameter value is multiplied by the best corner quality measure, which is the minimal eigenvalue
/// (see #cornerMinEigenVal ) or the Harris function response (see #cornerHarris ). The corners with the
/// quality measure less than the product are rejected. For example, if the best corner has the
/// quality measure = 1500, and the qualityLevel=0.01 , then all the corners with the quality measure
/// less than 15 are rejected.
/// * minDistance: Minimum possible Euclidean distance between the returned corners.
/// * mask: Optional region of interest. If the image is not empty (it needs to have the type
/// CV_8UC1 and the same size as image ), it specifies the region in which the corners are detected.
/// * blockSize: Size of an average block for computing a derivative covariation matrix over each
/// pixel neighborhood. See cornerEigenValsAndVecs .
/// * useHarrisDetector: Parameter indicating whether to use a Harris detector (see #cornerHarris)
/// or #cornerMinEigenVal.
/// * k: Free parameter of the Harris detector.
/// 
/// @sa  cornerMinEigenVal, cornerHarris, calcOpticalFlowPyrLK, estimateRigidTransform,
///
/// ## C++ default parameters:
/// * mask: noArray()
/// * block_size: 3
/// * use_harris_detector: false
/// * k: 0.04
pub fn good_features_to_track(image: &core::Mat, corners: &mut core::Mat, max_corners: i32, quality_level: f64, min_distance: f64, mask: &core::Mat, block_size: i32, use_harris_detector: bool, k: f64) -> Result<()> {
// identifier: cv_goodFeaturesToTrack_Mat_image_Mat_corners_int_maxCorners_double_qualityLevel_double_minDistance_Mat_mask_int_blockSize_bool_useHarrisDetector_double_k
  unsafe {
    let rv = sys::cv_imgproc_cv_goodFeaturesToTrack_Mat_image_Mat_corners_int_maxCorners_double_qualityLevel_double_minDistance_Mat_mask_int_blockSize_bool_useHarrisDetector_double_k(image.as_raw_Mat(), corners.as_raw_Mat(), max_corners, quality_level, min_distance, mask.as_raw_Mat(), block_size, use_harris_detector, k);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

///
/// ## C++ default parameters:
/// * use_harris_detector: false
/// * k: 0.04
pub fn good_features_to_track_v0(image: &core::Mat, corners: &mut core::Mat, max_corners: i32, quality_level: f64, min_distance: f64, mask: &core::Mat, block_size: i32, gradient_size: i32, use_harris_detector: bool, k: f64) -> Result<()> {
// identifier: cv_goodFeaturesToTrack_Mat_image_Mat_corners_int_maxCorners_double_qualityLevel_double_minDistance_Mat_mask_int_blockSize_int_gradientSize_bool_useHarrisDetector_double_k
  unsafe {
    let rv = sys::cv_imgproc_cv_goodFeaturesToTrack_Mat_image_Mat_corners_int_maxCorners_double_qualityLevel_double_minDistance_Mat_mask_int_blockSize_int_gradientSize_bool_useHarrisDetector_double_k(image.as_raw_Mat(), corners.as_raw_Mat(), max_corners, quality_level, min_distance, mask.as_raw_Mat(), block_size, gradient_size, use_harris_detector, k);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Runs the GrabCut algorithm.
/// 
/// The function implements the [GrabCut image segmentation algorithm](http://en.wikipedia.org/wiki/GrabCut).
/// 
/// ## Parameters
/// * img: Input 8-bit 3-channel image.
/// * mask: Input/output 8-bit single-channel mask. The mask is initialized by the function when
/// mode is set to #GC_INIT_WITH_RECT. Its elements may have one of the #GrabCutClasses.
/// * rect: ROI containing a segmented object. The pixels outside of the ROI are marked as
/// "obvious background". The parameter is only used when mode==#GC_INIT_WITH_RECT .
/// * bgdModel: Temporary array for the background model. Do not modify it while you are
/// processing the same image.
/// * fgdModel: Temporary arrays for the foreground model. Do not modify it while you are
/// processing the same image.
/// * iterCount: Number of iterations the algorithm should make before returning the result. Note
/// that the result can be refined with further calls with mode==#GC_INIT_WITH_MASK or
/// mode==GC_EVAL .
/// * mode: Operation mode that could be one of the #GrabCutModes
///
/// ## C++ default parameters:
/// * mode: GC_EVAL
pub fn grab_cut(img: &core::Mat, mask: &mut core::Mat, rect: core::Rect, bgd_model: &mut core::Mat, fgd_model: &mut core::Mat, iter_count: i32, mode: i32) -> Result<()> {
// identifier: cv_grabCut_Mat_img_Mat_mask_Rect_rect_Mat_bgdModel_Mat_fgdModel_int_iterCount_int_mode
  unsafe {
    let rv = sys::cv_imgproc_cv_grabCut_Mat_img_Mat_mask_Rect_rect_Mat_bgdModel_Mat_fgdModel_int_iterCount_int_mode(img.as_raw_Mat(), mask.as_raw_Mat(), rect, bgd_model.as_raw_Mat(), fgd_model.as_raw_Mat(), iter_count, mode);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Computes the undistortion and rectification transformation map.
/// 
/// The function computes the joint undistortion and rectification transformation and represents the
/// result in the form of maps for remap. The undistorted image looks like original, as if it is
/// captured with a camera using the camera matrix =newCameraMatrix and zero distortion. In case of a
/// monocular camera, newCameraMatrix is usually equal to cameraMatrix, or it can be computed by
/// #getOptimalNewCameraMatrix for a better control over scaling. In case of a stereo camera,
/// newCameraMatrix is normally set to P1 or P2 computed by #stereoRectify .
/// 
/// Also, this new camera is oriented differently in the coordinate space, according to R. That, for
/// example, helps to align two heads of a stereo camera so that the epipolar lines on both images
/// become horizontal and have the same y- coordinate (in case of a horizontally aligned stereo camera).
/// 
/// The function actually builds the maps for the inverse mapping algorithm that is used by remap. That
/// is, for each pixel <span lang='latex'>(u, v)</span> in the destination (corrected and rectified) image, the function
/// computes the corresponding coordinates in the source image (that is, in the original image from
/// camera). The following process is applied:
/// <div lang='latex'>
/// \begin{array}{l}
/// x  \leftarrow (u - {c'}_x)/{f'}_x  \\
/// y  \leftarrow (v - {c'}_y)/{f'}_y  \\
/// {[X\,Y\,W]} ^T  \leftarrow R^{-1}*[x \, y \, 1]^T  \\
/// x'  \leftarrow X/W  \\
/// y'  \leftarrow Y/W  \\
/// r^2  \leftarrow x'^2 + y'^2 \\
/// x''  \leftarrow x' \frac{1 + k_1 r^2 + k_2 r^4 + k_3 r^6}{1 + k_4 r^2 + k_5 r^4 + k_6 r^6}
/// + 2p_1 x' y' + p_2(r^2 + 2 x'^2)  + s_1 r^2 + s_2 r^4\\
/// y''  \leftarrow y' \frac{1 + k_1 r^2 + k_2 r^4 + k_3 r^6}{1 + k_4 r^2 + k_5 r^4 + k_6 r^6}
/// + p_1 (r^2 + 2 y'^2) + 2 p_2 x' y' + s_3 r^2 + s_4 r^4 \\
/// s\vecthree{x'''}{y'''}{1} =
/// \vecthreethree{R_{33}(\tau_x, \tau_y)}{0}{-R_{13}((\tau_x, \tau_y)}
/// {0}{R_{33}(\tau_x, \tau_y)}{-R_{23}(\tau_x, \tau_y)}
/// {0}{0}{1} R(\tau_x, \tau_y) \vecthree{x''}{y''}{1}\\
/// map_x(u,v)  \leftarrow x''' f_x + c_x  \\
/// map_y(u,v)  \leftarrow y''' f_y + c_y
/// \end{array}
/// </div>
/// where <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6[, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span>
/// are the distortion coefficients.
/// 
/// In case of a stereo camera, this function is called twice: once for each camera head, after
/// stereoRectify, which in its turn is called after #stereoCalibrate. But if the stereo camera
/// was not calibrated, it is still possible to compute the rectification transformations directly from
/// the fundamental matrix using #stereoRectifyUncalibrated. For each camera, the function computes
/// homography H as the rectification transformation in a pixel domain, not a rotation matrix R in 3D
/// space. R can be computed from H as
/// <div lang='latex'>\texttt{R} = \texttt{cameraMatrix} ^{-1} \cdot \texttt{H} \cdot \texttt{cameraMatrix}</div>
/// where cameraMatrix can be chosen arbitrarily.
/// 
/// ## Parameters
/// * cameraMatrix: Input camera matrix <span lang='latex'>A=\vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> .
/// * distCoeffs: Input vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6[, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span>
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * R: Optional rectification transformation in the object space (3x3 matrix). R1 or R2 ,
/// computed by #stereoRectify can be passed here. If the matrix is empty, the identity transformation
/// is assumed. In cvInitUndistortMap R assumed to be an identity matrix.
/// * newCameraMatrix: New camera matrix <span lang='latex'>A'=\vecthreethree{f_x'}{0}{c_x'}{0}{f_y'}{c_y'}{0}{0}{1}</span>.
/// * size: Undistorted image size.
/// * m1type: Type of the first output map that can be CV_32FC1, CV_32FC2 or CV_16SC2, see #convertMaps
/// * map1: The first output map.
/// * map2: The second output map.
pub fn init_undistort_rectify_map(camera_matrix: &core::Mat, dist_coeffs: &core::Mat, r: &core::Mat, new_camera_matrix: &core::Mat, size: core::Size, m1type: i32, map1: &mut core::Mat, map2: &mut core::Mat) -> Result<()> {
// identifier: cv_initUndistortRectifyMap_Mat_cameraMatrix_Mat_distCoeffs_Mat_R_Mat_newCameraMatrix_Size_size_int_m1type_Mat_map1_Mat_map2
  unsafe {
    let rv = sys::cv_imgproc_cv_initUndistortRectifyMap_Mat_cameraMatrix_Mat_distCoeffs_Mat_R_Mat_newCameraMatrix_Size_size_int_m1type_Mat_map1_Mat_map2(camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), r.as_raw_Mat(), new_camera_matrix.as_raw_Mat(), size, m1type, map1.as_raw_Mat(), map2.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

///
/// ## C++ default parameters:
/// * proj_type: PROJ_SPHERICAL_EQRECT
/// * alpha: 0
pub fn init_wide_angle_proj_map(camera_matrix: &core::Mat, dist_coeffs: &core::Mat, image_size: core::Size, dest_image_width: i32, m1type: i32, map1: &mut core::Mat, map2: &mut core::Mat, proj_type: i32, alpha: f64) -> Result<f32> {
// identifier: cv_initWideAngleProjMap_Mat_cameraMatrix_Mat_distCoeffs_Size_imageSize_int_destImageWidth_int_m1type_Mat_map1_Mat_map2_int_projType_double_alpha
  unsafe {
    let rv = sys::cv_imgproc_cv_initWideAngleProjMap_Mat_cameraMatrix_Mat_distCoeffs_Size_imageSize_int_destImageWidth_int_m1type_Mat_map1_Mat_map2_int_projType_double_alpha(camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), image_size, dest_image_width, m1type, map1.as_raw_Mat(), map2.as_raw_Mat(), proj_type, alpha);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Calculates the integral of an image.
/// 
/// The function calculates one or more integral images for the source image as follows:
/// 
/// <div lang='latex'>\texttt{sum} (X,Y) =  \sum _{x<X,y<Y}  \texttt{image} (x,y)</div>
/// 
/// <div lang='latex'>\texttt{sqsum} (X,Y) =  \sum _{x<X,y<Y}  \texttt{image} (x,y)^2</div>
/// 
/// <div lang='latex'>\texttt{tilted} (X,Y) =  \sum _{y<Y,abs(x-X+1) \leq Y-y-1}  \texttt{image} (x,y)</div>
/// 
/// Using these integral images, you can calculate sum, mean, and standard deviation over a specific
/// up-right or rotated rectangular region of the image in a constant time, for example:
/// 
/// <div lang='latex'>\sum _{x_1 \leq x < x_2,  \, y_1  \leq y < y_2}  \texttt{image} (x,y) =  \texttt{sum} (x_2,y_2)- \texttt{sum} (x_1,y_2)- \texttt{sum} (x_2,y_1)+ \texttt{sum} (x_1,y_1)</div>
/// 
/// It makes possible to do a fast blurring or fast block correlation with a variable window size, for
/// example. In case of multi-channel images, sums for each channel are accumulated independently.
/// 
/// As a practical example, the next figure shows the calculation of the integral of a straight
/// rectangle Rect(3,3,3,2) and of a tilted rectangle Rect(5,1,2,3) . The selected pixels in the
/// original image are shown, as well as the relative pixels in the integral images sum and tilted .
/// 
/// ![integral calculation example](pics/integral.png)
/// 
/// ## Parameters
/// * src: input image as <span lang='latex'>W \times H</span>, 8-bit or floating-point (32f or 64f).
/// * sum: integral image as <span lang='latex'>(W+1)\times (H+1)</span> , 32-bit integer or floating-point (32f or 64f).
/// * sqsum: integral image for squared pixel values; it is <span lang='latex'>(W+1)\times (H+1)</span>, double-precision
/// floating-point (64f) array.
/// * tilted: integral for the image rotated by 45 degrees; it is <span lang='latex'>(W+1)\times (H+1)</span> array with
/// the same data type as sum.
/// * sdepth: desired depth of the integral and the tilted integral images, CV_32S, CV_32F, or
/// CV_64F.
/// * sqdepth: desired depth of the integral image of squared pixel values, CV_32F or CV_64F.
///
/// ## C++ default parameters:
/// * sdepth: -1
/// * sqdepth: -1
pub fn integral_titled_sq(src: &core::Mat, sum: &mut core::Mat, sqsum: &mut core::Mat, tilted: &mut core::Mat, sdepth: i32, sqdepth: i32) -> Result<()> {
// identifier: cv_integral_Mat_src_Mat_sum_Mat_sqsum_Mat_tilted_int_sdepth_int_sqdepth
  unsafe {
    let rv = sys::cv_imgproc_cv_integral_Mat_src_Mat_sum_Mat_sqsum_Mat_tilted_int_sdepth_int_sqdepth(src.as_raw_Mat(), sum.as_raw_Mat(), sqsum.as_raw_Mat(), tilted.as_raw_Mat(), sdepth, sqdepth);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
///
/// ## C++ default parameters:
/// * sdepth: -1
/// * sqdepth: -1
pub fn integral_sq_depth(src: &core::Mat, sum: &mut core::Mat, sqsum: &mut core::Mat, sdepth: i32, sqdepth: i32) -> Result<()> {
// identifier: cv_integral_Mat_src_Mat_sum_Mat_sqsum_int_sdepth_int_sqdepth
  unsafe {
    let rv = sys::cv_imgproc_cv_integral_Mat_src_Mat_sum_Mat_sqsum_int_sdepth_int_sqdepth(src.as_raw_Mat(), sum.as_raw_Mat(), sqsum.as_raw_Mat(), sdepth, sqdepth);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
///
/// ## C++ default parameters:
/// * sdepth: -1
pub fn integral(src: &core::Mat, sum: &mut core::Mat, sdepth: i32) -> Result<()> {
// identifier: cv_integral_Mat_src_Mat_sum_int_sdepth
  unsafe {
    let rv = sys::cv_imgproc_cv_integral_Mat_src_Mat_sum_int_sdepth(src.as_raw_Mat(), sum.as_raw_Mat(), sdepth);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

///
/// ## C++ default parameters:
/// * handle_nested: true
pub fn intersect_convex_convex(_p1: &core::Mat, _p2: &core::Mat, _p12: &mut core::Mat, handle_nested: bool) -> Result<f32> {
// identifier: cv_intersectConvexConvex_Mat__p1_Mat__p2_Mat__p12_bool_handleNested
  unsafe {
    let rv = sys::cv_imgproc_cv_intersectConvexConvex_Mat__p1_Mat__p2_Mat__p12_bool_handleNested(_p1.as_raw_Mat(), _p2.as_raw_Mat(), _p12.as_raw_Mat(), handle_nested);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Inverts an affine transformation.
/// 
/// The function computes an inverse affine transformation represented by <span lang='latex'>2 \times 3</span> matrix M:
/// 
/// <div lang='latex'>\begin{bmatrix} a_{11} & a_{12} & b_1  \\ a_{21} & a_{22} & b_2 \end{bmatrix}</div>
/// 
/// The result is also a <span lang='latex'>2 \times 3</span> matrix of the same type as M.
/// 
/// ## Parameters
/// * M: Original affine transformation.
/// * iM: Output reverse affine transformation.
pub fn invert_affine_transform(m: &core::Mat, i_m: &mut core::Mat) -> Result<()> {
// identifier: cv_invertAffineTransform_Mat_M_Mat_iM
  unsafe {
    let rv = sys::cv_imgproc_cv_invertAffineTransform_Mat_M_Mat_iM(m.as_raw_Mat(), i_m.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Tests a contour convexity.
/// 
/// The function tests whether the input contour is convex or not. The contour must be simple, that is,
/// without self-intersections. Otherwise, the function output is undefined.
/// 
/// ## Parameters
/// * contour: Input vector of 2D points, stored in std::vector\<\> or Mat
pub fn is_contour_convex(contour: &core::Mat) -> Result<bool> {
// identifier: cv_isContourConvex_Mat_contour
  unsafe {
    let rv = sys::cv_imgproc_cv_isContourConvex_Mat_contour(contour.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Draws a line segment connecting two points.
/// 
/// The function line draws the line segment between pt1 and pt2 points in the image. The line is
/// clipped by the image boundaries. For non-antialiased lines with integer coordinates, the 8-connected
/// or 4-connected Bresenham algorithm is used. Thick lines are drawn with rounding endings. Antialiased
/// lines are drawn using Gaussian filtering.
/// 
/// ## Parameters
/// * img: Image.
/// * pt1: First point of the line segment.
/// * pt2: Second point of the line segment.
/// * color: Line color.
/// * thickness: Line thickness.
/// * lineType: Type of the line. See #LineTypes.
/// * shift: Number of fractional bits in the point coordinates.
///
/// ## C++ default parameters:
/// * thickness: 1
/// * line_type: LINE_8
/// * shift: 0
pub fn line(img: &mut core::Mat, pt1: core::Point, pt2: core::Point, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> Result<()> {
// identifier: cv_line_Mat_img_Point_pt1_Point_pt2_Scalar_color_int_thickness_int_lineType_int_shift
  unsafe {
    let rv = sys::cv_imgproc_cv_line_Mat_img_Point_pt1_Point_pt2_Scalar_color_int_thickness_int_lineType_int_shift(img.as_raw_Mat(), pt1, pt2, color, thickness, line_type, shift);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Remaps an image to polar coordinates space.
/// 
/// @deprecated This function produces same result as cv::warpPolar(src, dst, src.size(), center, maxRadius, flags)
/// 
/// @internal
/// Transform the source image using the following transformation (See @ref polar_remaps_reference_image "Polar remaps reference image c)"):
/// <div lang='latex'>\begin{array}{l}
/// dst( \rho , \phi ) = src(x,y) \\
/// dst.size() \leftarrow src.size()
/// \end{array}</div>
/// 
/// where
/// <div lang='latex'>\begin{array}{l}
/// I = (dx,dy) = (x - center.x,y - center.y) \\
/// \rho = Kmag \cdot \texttt{magnitude} (I) ,\\
/// \phi = angle \cdot \texttt{angle} (I)
/// \end{array}</div>
/// 
/// and
/// <div lang='latex'>\begin{array}{l}
/// Kx = src.cols / maxRadius \\
/// Ky = src.rows / 2\Pi
/// \end{array}</div>
/// 
/// 
/// ## Parameters
/// * src: Source image
/// * dst: Destination image. It will have same size and type as src.
/// * center: The transformation center;
/// * maxRadius: The radius of the bounding circle to transform. It determines the inverse magnitude scale parameter too.
/// * flags: A combination of interpolation methods, see #InterpolationFlags
/// 
/// 
/// Note:
/// *   The function can not operate in-place.
/// *   To calculate magnitude and angle in degrees #cartToPolar is used internally thus angles are measured from 0 to 360 with accuracy about 0.3 degrees.
/// 
/// @sa cv::logPolar
/// @endinternal
pub fn linear_polar(src: &core::Mat, dst: &mut core::Mat, center: core::Point2f, max_radius: f64, flags: i32) -> Result<()> {
// identifier: cv_linearPolar_Mat_src_Mat_dst_Point2f_center_double_maxRadius_int_flags
  unsafe {
    let rv = sys::cv_imgproc_cv_linearPolar_Mat_src_Mat_dst_Point2f_center_double_maxRadius_int_flags(src.as_raw_Mat(), dst.as_raw_Mat(), center, max_radius, flags);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Remaps an image to semilog-polar coordinates space.
/// 
/// @deprecated This function produces same result as cv::warpPolar(src, dst, src.size(), center, maxRadius, flags+WARP_POLAR_LOG);
/// 
/// @internal
/// Transform the source image using the following transformation (See @ref polar_remaps_reference_image "Polar remaps reference image d)"):
/// <div lang='latex'>\begin{array}{l}
/// dst( \rho , \phi ) = src(x,y) \\
/// dst.size() \leftarrow src.size()
/// \end{array}</div>
/// 
/// where
/// <div lang='latex'>\begin{array}{l}
/// I = (dx,dy) = (x - center.x,y - center.y) \\
/// \rho = M \cdot log_e(\texttt{magnitude} (I)) ,\\
/// \phi = Kangle \cdot \texttt{angle} (I) \\
/// \end{array}</div>
/// 
/// and
/// <div lang='latex'>\begin{array}{l}
/// M = src.cols / log_e(maxRadius) \\
/// Kangle = src.rows / 2\Pi \\
/// \end{array}</div>
/// 
/// The function emulates the human "foveal" vision and can be used for fast scale and
/// rotation-invariant template matching, for object tracking and so forth.
/// ## Parameters
/// * src: Source image
/// * dst: Destination image. It will have same size and type as src.
/// * center: The transformation center; where the output precision is maximal
/// * M: Magnitude scale parameter. It determines the radius of the bounding circle to transform too.
/// * flags: A combination of interpolation methods, see #InterpolationFlags
/// 
/// 
/// Note:
/// *   The function can not operate in-place.
/// *   To calculate magnitude and angle in degrees #cartToPolar is used internally thus angles are measured from 0 to 360 with accuracy about 0.3 degrees.
/// 
/// @sa cv::linearPolar
/// @endinternal
pub fn log_polar(src: &core::Mat, dst: &mut core::Mat, center: core::Point2f, m: f64, flags: i32) -> Result<()> {
// identifier: cv_logPolar_Mat_src_Mat_dst_Point2f_center_double_M_int_flags
  unsafe {
    let rv = sys::cv_imgproc_cv_logPolar_Mat_src_Mat_dst_Point2f_center_double_M_int_flags(src.as_raw_Mat(), dst.as_raw_Mat(), center, m, flags);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Compares two shapes.
/// 
/// The function compares two shapes. All three implemented methods use the Hu invariants (see #HuMoments)
/// 
/// ## Parameters
/// * contour1: First contour or grayscale image.
/// * contour2: Second contour or grayscale image.
/// * method: Comparison method, see #ShapeMatchModes
/// * parameter: Method-specific parameter (not supported now).
pub fn match_shapes(contour1: &core::Mat, contour2: &core::Mat, method: i32, parameter: f64) -> Result<f64> {
// identifier: cv_matchShapes_Mat_contour1_Mat_contour2_int_method_double_parameter
  unsafe {
    let rv = sys::cv_imgproc_cv_matchShapes_Mat_contour1_Mat_contour2_int_method_double_parameter(contour1.as_raw_Mat(), contour2.as_raw_Mat(), method, parameter);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Compares a template against overlapped image regions.
/// 
/// The function slides through image , compares the overlapped patches of size <span lang='latex'>w \times h</span> against
/// templ using the specified method and stores the comparison results in result . Here are the formulae
/// for the available comparison methods ( <span lang='latex'>I</span> denotes image, <span lang='latex'>T</span> template, <span lang='latex'>R</span> result ). The summation
/// is done over template and/or the image patch: <span lang='latex'>x' = 0...w-1, y' = 0...h-1</span>
/// 
/// After the function finishes the comparison, the best matches can be found as global minimums (when
/// #TM_SQDIFF was used) or maximums (when #TM_CCORR or #TM_CCOEFF was used) using the
/// #minMaxLoc function. In case of a color image, template summation in the numerator and each sum in
/// the denominator is done over all of the channels and separate mean values are used for each channel.
/// That is, the function can take a color template and a color image. The result will still be a
/// single-channel image, which is easier to analyze.
/// 
/// ## Parameters
/// * image: Image where the search is running. It must be 8-bit or 32-bit floating-point.
/// * templ: Searched template. It must be not greater than the source image and have the same
/// data type.
/// * result: Map of comparison results. It must be single-channel 32-bit floating-point. If image
/// is <span lang='latex'>W \times H</span> and templ is <span lang='latex'>w \times h</span> , then result is <span lang='latex'>(W-w+1) \times (H-h+1)</span> .
/// * method: Parameter specifying the comparison method, see #TemplateMatchModes
/// * mask: Mask of searched template. It must have the same datatype and size with templ. It is
/// not set by default. Currently, only the #TM_SQDIFF and #TM_CCORR_NORMED methods are supported.
///
/// ## C++ default parameters:
/// * mask: noArray()
pub fn match_template(image: &core::Mat, templ: &core::Mat, result: &mut core::Mat, method: i32, mask: &core::Mat) -> Result<()> {
// identifier: cv_matchTemplate_Mat_image_Mat_templ_Mat_result_int_method_Mat_mask
  unsafe {
    let rv = sys::cv_imgproc_cv_matchTemplate_Mat_image_Mat_templ_Mat_result_int_method_Mat_mask(image.as_raw_Mat(), templ.as_raw_Mat(), result.as_raw_Mat(), method, mask.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Blurs an image using the median filter.
/// 
/// The function smoothes an image using the median filter with the \f$\texttt{ksize} \times
/// \texttt{ksize}\f$ aperture. Each channel of a multi-channel image is processed independently.
/// In-place operation is supported.
/// 
/// 
/// Note: The median filter uses #BORDER_REPLICATE internally to cope with border pixels, see #BorderTypes
/// 
/// ## Parameters
/// * src: input 1-, 3-, or 4-channel image; when ksize is 3 or 5, the image depth should be
/// CV_8U, CV_16U, or CV_32F, for larger aperture sizes, it can only be CV_8U.
/// * dst: destination array of the same size and type as src.
/// * ksize: aperture linear size; it must be odd and greater than 1, for example: 3, 5, 7 ...
/// @sa  bilateralFilter, blur, boxFilter, GaussianBlur
pub fn median_blur(src: &core::Mat, dst: &mut core::Mat, ksize: i32) -> Result<()> {
// identifier: cv_medianBlur_Mat_src_Mat_dst_int_ksize
  unsafe {
    let rv = sys::cv_imgproc_cv_medianBlur_Mat_src_Mat_dst_int_ksize(src.as_raw_Mat(), dst.as_raw_Mat(), ksize);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds a rotated rectangle of the minimum area enclosing the input 2D point set.
/// 
/// The function calculates and returns the minimum-area bounding rectangle (possibly rotated) for a
/// specified point set. Developer should keep in mind that the returned RotatedRect can contain negative
/// indices when data is close to the containing Mat element boundary.
/// 
/// ## Parameters
/// * points: Input vector of 2D points, stored in std::vector\<\> or Mat
pub fn min_area_rect(points: &core::Mat) -> Result<core::RotatedRect> {
// identifier: cv_minAreaRect_Mat_points
  unsafe {
    let rv = sys::cv_imgproc_cv_minAreaRect_Mat_points(points.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::RotatedRect { ptr: rv.result })
    }
  }
}

/// Finds a circle of the minimum area enclosing a 2D point set.
/// 
/// The function finds the minimal enclosing circle of a 2D point set using an iterative algorithm.
/// 
/// ## Parameters
/// * points: Input vector of 2D points, stored in std::vector\<\> or Mat
/// * center: Output center of the circle.
/// * radius: Output radius of the circle.
pub fn min_enclosing_circle(points: &core::Mat, center: core::Point2f, radius: f32) -> Result<()> {
// identifier: cv_minEnclosingCircle_Mat_points_Point2f_center_float_radius
  unsafe {
    let rv = sys::cv_imgproc_cv_minEnclosingCircle_Mat_points_Point2f_center_float_radius(points.as_raw_Mat(), center, radius);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds a triangle of minimum area enclosing a 2D point set and returns its area.
/// 
/// The function finds a triangle of minimum area enclosing the given set of 2D points and returns its
/// area. The output for a given 2D point set is shown in the image below. 2D points are depicted in
/// *red* and the enclosing triangle in *yellow*.
/// 
/// ![Sample output of the minimum enclosing triangle function](pics/minenclosingtriangle.png)
/// 
/// The implementation of the algorithm is based on O'Rourke's @cite ORourke86 and Klee and Laskowski's
/// @cite KleeLaskowski85 papers. O'Rourke provides a <span lang='latex'>\theta(n)</span> algorithm for finding the minimal
/// enclosing triangle of a 2D convex polygon with n vertices. Since the #minEnclosingTriangle function
/// takes a 2D point set as input an additional preprocessing step of computing the convex hull of the
/// 2D point set is required. The complexity of the #convexHull function is <span lang='latex'>O(n log(n))</span> which is higher
/// than <span lang='latex'>\theta(n)</span>. Thus the overall complexity of the function is <span lang='latex'>O(n log(n))</span>.
/// 
/// ## Parameters
/// * points: Input vector of 2D points with depth CV_32S or CV_32F, stored in std::vector\<\> or Mat
/// * triangle: Output vector of three 2D points defining the vertices of the triangle. The depth
/// of the OutputArray must be CV_32F.
pub fn min_enclosing_triangle(points: &core::Mat, triangle: &mut core::Mat) -> Result<f64> {
// identifier: cv_minEnclosingTriangle_Mat_points_Mat_triangle
  unsafe {
    let rv = sys::cv_imgproc_cv_minEnclosingTriangle_Mat_points_Mat_triangle(points.as_raw_Mat(), triangle.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

pub fn morphology_default_border_value() -> Result<core::Scalar> {
// identifier: cv_morphologyDefaultBorderValue
  unsafe {
    let rv = sys::cv_imgproc_cv_morphologyDefaultBorderValue();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Performs advanced morphological transformations.
/// 
/// The function cv::morphologyEx can perform advanced morphological transformations using an erosion and dilation as
/// basic operations.
/// 
/// Any of the operations can be done in-place. In case of multi-channel images, each channel is
/// processed independently.
/// 
/// ## Parameters
/// * src: Source image. The number of channels can be arbitrary. The depth should be one of
/// CV_8U, CV_16U, CV_16S, CV_32F or CV_64F.
/// * dst: Destination image of the same size and type as source image.
/// * op: Type of a morphological operation, see #MorphTypes
/// * kernel: Structuring element. It can be created using #getStructuringElement.
/// * anchor: Anchor position with the kernel. Negative values mean that the anchor is at the
/// kernel center.
/// * iterations: Number of times erosion and dilation are applied.
/// * borderType: Pixel extrapolation method, see #BorderTypes
/// * borderValue: Border value in case of a constant border. The default value has a special
/// meaning.
/// @sa  dilate, erode, getStructuringElement
/// 
/// Note: The number of iterations is the number of times erosion or dilatation operation will be applied.
/// For instance, an opening operation (#MORPH_OPEN) with two iterations is equivalent to apply
/// successively: erode -> erode -> dilate -> dilate (and not erode -> dilate -> erode -> dilate).
///
/// ## C++ default parameters:
/// * anchor: Point(-1,-1)
/// * iterations: 1
/// * border_type: BORDER_CONSTANT
/// * border_value: morphologyDefaultBorderValue()
pub fn morphology_ex(src: &core::Mat, dst: &mut core::Mat, op: i32, kernel: &core::Mat, anchor: core::Point, iterations: i32, border_type: i32, border_value: core::Scalar) -> Result<()> {
// identifier: cv_morphologyEx_Mat_src_Mat_dst_int_op_Mat_kernel_Point_anchor_int_iterations_int_borderType_Scalar_borderValue
  unsafe {
    let rv = sys::cv_imgproc_cv_morphologyEx_Mat_src_Mat_dst_int_op_Mat_kernel_Point_anchor_int_iterations_int_borderType_Scalar_borderValue(src.as_raw_Mat(), dst.as_raw_Mat(), op, kernel.as_raw_Mat(), anchor, iterations, border_type, border_value);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Performs a point-in-contour test.
/// 
/// The function determines whether the point is inside a contour, outside, or lies on an edge (or
/// coincides with a vertex). It returns positive (inside), negative (outside), or zero (on an edge)
/// value, correspondingly. When measureDist=false , the return value is +1, -1, and 0, respectively.
/// Otherwise, the return value is a signed distance between the point and the nearest contour edge.
/// 
/// See below a sample output of the function where each image pixel is tested against the contour:
/// 
/// ![sample output](pics/pointpolygon.png)
/// 
/// ## Parameters
/// * contour: Input contour.
/// * pt: Point tested against the contour.
/// * measureDist: If true, the function estimates the signed distance from the point to the
/// nearest contour edge. Otherwise, the function only checks if the point is inside a contour or not.
pub fn point_polygon_test(contour: &core::Mat, pt: core::Point2f, measure_dist: bool) -> Result<f64> {
// identifier: cv_pointPolygonTest_Mat_contour_Point2f_pt_bool_measureDist
  unsafe {
    let rv = sys::cv_imgproc_cv_pointPolygonTest_Mat_contour_Point2f_pt_bool_measureDist(contour.as_raw_Mat(), pt, measure_dist);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Draws several polygonal curves.
/// 
/// ## Parameters
/// * img: Image.
/// * pts: Array of polygonal curves.
/// * isClosed: Flag indicating whether the drawn polylines are closed or not. If they are closed,
/// the function draws a line from the last vertex of each curve to its first vertex.
/// * color: Polyline color.
/// * thickness: Thickness of the polyline edges.
/// * lineType: Type of the line segments. See #LineTypes
/// * shift: Number of fractional bits in the vertex coordinates.
/// 
/// The function cv::polylines draws one or more polygonal curves.
///
/// ## C++ default parameters:
/// * thickness: 1
/// * line_type: LINE_8
/// * shift: 0
pub fn polylines(img: &mut core::Mat, pts: &types::VectorOfMat, is_closed: bool, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> Result<()> {
// identifier: cv_polylines_Mat_img_VectorOfMat_pts_bool_isClosed_Scalar_color_int_thickness_int_lineType_int_shift
  unsafe {
    let rv = sys::cv_imgproc_cv_polylines_Mat_img_VectorOfMat_pts_bool_isClosed_Scalar_color_int_thickness_int_lineType_int_shift(img.as_raw_Mat(), pts.as_raw_VectorOfMat(), is_closed, color, thickness, line_type, shift);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates a feature map for corner detection.
/// 
/// The function calculates the complex spatial derivative-based function of the source image
/// 
/// <div lang='latex'>\texttt{dst} = (D_x  \texttt{src} )^2  \cdot D_{yy}  \texttt{src} + (D_y  \texttt{src} )^2  \cdot D_{xx}  \texttt{src} - 2 D_x  \texttt{src} \cdot D_y  \texttt{src} \cdot D_{xy}  \texttt{src}</div>
/// 
/// where <span lang='latex'>D_x</span>,<span lang='latex'>D_y</span> are the first image derivatives, <span lang='latex'>D_{xx}</span>,<span lang='latex'>D_{yy}</span> are the second image
/// derivatives, and <span lang='latex'>D_{xy}</span> is the mixed derivative.
/// 
/// The corners can be found as local maximums of the functions, as shown below:
/// ```ignore
/// Mat corners, dilated_corners;
/// preCornerDetect(image, corners, 3);
/// // dilation with 3x3 rectangular structuring element
/// dilate(corners, dilated_corners, Mat(), 1);
/// Mat corner_mask = corners == dilated_corners;
/// ```
/// 
/// 
/// ## Parameters
/// * src: Source single-channel 8-bit of floating-point image.
/// * dst: Output image that has the type CV_32F and the same size as src .
/// * ksize: %Aperture size of the Sobel .
/// * borderType: Pixel extrapolation method. See #BorderTypes.
///
/// ## C++ default parameters:
/// * border_type: BORDER_DEFAULT
pub fn pre_corner_detect(src: &core::Mat, dst: &mut core::Mat, ksize: i32, border_type: i32) -> Result<()> {
// identifier: cv_preCornerDetect_Mat_src_Mat_dst_int_ksize_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_preCornerDetect_Mat_src_Mat_dst_int_ksize_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), ksize, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Draws a text string.
/// 
/// The function cv::putText renders the specified text string in the image. Symbols that cannot be rendered
/// using the specified font are replaced by question marks. See #getTextSize for a text rendering code
/// example.
/// 
/// ## Parameters
/// * img: Image.
/// * text: Text string to be drawn.
/// * org: Bottom-left corner of the text string in the image.
/// * fontFace: Font type, see #HersheyFonts.
/// * fontScale: Font scale factor that is multiplied by the font-specific base size.
/// * color: Text color.
/// * thickness: Thickness of the lines used to draw a text.
/// * lineType: Line type. See #LineTypes
/// * bottomLeftOrigin: When true, the image data origin is at the bottom-left corner. Otherwise,
/// it is at the top-left corner.
///
/// ## C++ default parameters:
/// * thickness: 1
/// * line_type: LINE_8
/// * bottom_left_origin: false
pub fn put_text(img: &mut core::Mat, text:&str, org: core::Point, font_face: i32, font_scale: f64, color: core::Scalar, thickness: i32, line_type: i32, bottom_left_origin: bool) -> Result<()> {
// identifier: cv_putText_Mat_img_String_text_Point_org_int_fontFace_double_fontScale_Scalar_color_int_thickness_int_lineType_bool_bottomLeftOrigin
  unsafe {
    let text = CString::new(text).unwrap();
    let rv = sys::cv_imgproc_cv_putText_Mat_img_String_text_Point_org_int_fontFace_double_fontScale_Scalar_color_int_thickness_int_lineType_bool_bottomLeftOrigin(img.as_raw_Mat(), text.as_ptr() as _, org, font_face, font_scale, color, thickness, line_type, bottom_left_origin);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Blurs an image and downsamples it.
/// 
/// By default, size of the output image is computed as `Size((src.cols+1)/2, (src.rows+1)/2)`, but in
/// any case, the following conditions should be satisfied:
/// 
/// <div lang='latex'>\begin{array}{l} | \texttt{dstsize.width} *2-src.cols| \leq 2 \\ | \texttt{dstsize.height} *2-src.rows| \leq 2 \end{array}</div>
/// 
/// The function performs the downsampling step of the Gaussian pyramid construction. First, it
/// convolves the source image with the kernel:
/// 
/// <div lang='latex'>\frac{1}{256} \begin{bmatrix} 1 & 4 & 6 & 4 & 1  \\ 4 & 16 & 24 & 16 & 4  \\ 6 & 24 & 36 & 24 & 6  \\ 4 & 16 & 24 & 16 & 4  \\ 1 & 4 & 6 & 4 & 1 \end{bmatrix}</div>
/// 
/// Then, it downsamples the image by rejecting even rows and columns.
/// 
/// ## Parameters
/// * src: input image.
/// * dst: output image; it has the specified size and the same type as src.
/// * dstsize: size of the output image.
/// * borderType: Pixel extrapolation method, see #BorderTypes (#BORDER_CONSTANT isn't supported)
///
/// ## C++ default parameters:
/// * dstsize: Size()
/// * border_type: BORDER_DEFAULT
pub fn pyr_down(src: &core::Mat, dst: &mut core::Mat, dstsize: core::Size, border_type: i32) -> Result<()> {
// identifier: cv_pyrDown_Mat_src_Mat_dst_Size_dstsize_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_pyrDown_Mat_src_Mat_dst_Size_dstsize_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), dstsize, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Performs initial step of meanshift segmentation of an image.
/// 
/// The function implements the filtering stage of meanshift segmentation, that is, the output of the
/// function is the filtered "posterized" image with color gradients and fine-grain texture flattened.
/// At every pixel (X,Y) of the input image (or down-sized input image, see below) the function executes
/// meanshift iterations, that is, the pixel (X,Y) neighborhood in the joint space-color hyperspace is
/// considered:
/// 
/// <div lang='latex'>(x,y): X- \texttt{sp} \le x  \le X+ \texttt{sp} , Y- \texttt{sp} \le y  \le Y+ \texttt{sp} , ||(R,G,B)-(r,g,b)||   \le \texttt{sr}</div>
/// 
/// where (R,G,B) and (r,g,b) are the vectors of color components at (X,Y) and (x,y), respectively
/// (though, the algorithm does not depend on the color space used, so any 3-component color space can
/// be used instead). Over the neighborhood the average spatial value (X',Y') and average color vector
/// (R',G',B') are found and they act as the neighborhood center on the next iteration:
/// 
/// <div lang='latex'>(X,Y)~(X',Y'), (R,G,B)~(R',G',B').</div>
/// 
/// After the iterations over, the color components of the initial pixel (that is, the pixel from where
/// the iterations started) are set to the final value (average color at the last iteration):
/// 
/// <div lang='latex'>I(X,Y) <- (R*,G*,B*)</div>
/// 
/// When maxLevel \> 0, the gaussian pyramid of maxLevel+1 levels is built, and the above procedure is
/// run on the smallest layer first. After that, the results are propagated to the larger layer and the
/// iterations are run again only on those pixels where the layer colors differ by more than sr from the
/// lower-resolution layer of the pyramid. That makes boundaries of color regions sharper. Note that the
/// results will be actually different from the ones obtained by running the meanshift procedure on the
/// whole original image (i.e. when maxLevel==0).
/// 
/// ## Parameters
/// * src: The source 8-bit, 3-channel image.
/// * dst: The destination image of the same format and the same size as the source.
/// * sp: The spatial window radius.
/// * sr: The color window radius.
/// * maxLevel: Maximum level of the pyramid for the segmentation.
/// * termcrit: Termination criteria: when to stop meanshift iterations.
///
/// ## C++ default parameters:
/// * max_level: 1
/// * termcrit: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5,1)
pub fn pyr_mean_shift_filtering(src: &core::Mat, dst: &mut core::Mat, sp: f64, sr: f64, max_level: i32, termcrit: &core::TermCriteria) -> Result<()> {
// identifier: cv_pyrMeanShiftFiltering_Mat_src_Mat_dst_double_sp_double_sr_int_maxLevel_TermCriteria_termcrit
  unsafe {
    let rv = sys::cv_imgproc_cv_pyrMeanShiftFiltering_Mat_src_Mat_dst_double_sp_double_sr_int_maxLevel_TermCriteria_termcrit(src.as_raw_Mat(), dst.as_raw_Mat(), sp, sr, max_level, termcrit.as_raw_TermCriteria());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Upsamples an image and then blurs it.
/// 
/// By default, size of the output image is computed as `Size(src.cols\*2, (src.rows\*2)`, but in any
/// case, the following conditions should be satisfied:
/// 
/// <div lang='latex'>\begin{array}{l} | \texttt{dstsize.width} -src.cols*2| \leq  ( \texttt{dstsize.width}   \mod  2)  \\ | \texttt{dstsize.height} -src.rows*2| \leq  ( \texttt{dstsize.height}   \mod  2) \end{array}</div>
/// 
/// The function performs the upsampling step of the Gaussian pyramid construction, though it can
/// actually be used to construct the Laplacian pyramid. First, it upsamples the source image by
/// injecting even zero rows and columns and then convolves the result with the same kernel as in
/// pyrDown multiplied by 4.
/// 
/// ## Parameters
/// * src: input image.
/// * dst: output image. It has the specified size and the same type as src .
/// * dstsize: size of the output image.
/// * borderType: Pixel extrapolation method, see #BorderTypes (only #BORDER_DEFAULT is supported)
///
/// ## C++ default parameters:
/// * dstsize: Size()
/// * border_type: BORDER_DEFAULT
pub fn pyr_up(src: &core::Mat, dst: &mut core::Mat, dstsize: core::Size, border_type: i32) -> Result<()> {
// identifier: cv_pyrUp_Mat_src_Mat_dst_Size_dstsize_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_pyrUp_Mat_src_Mat_dst_Size_dstsize_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), dstsize, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Draws a simple, thick, or filled up-right rectangle.
/// 
/// The function cv::rectangle draws a rectangle outline or a filled rectangle whose two opposite corners
/// are pt1 and pt2.
/// 
/// ## Parameters
/// * img: Image.
/// * pt1: Vertex of the rectangle.
/// * pt2: Vertex of the rectangle opposite to pt1 .
/// * color: Rectangle color or brightness (grayscale image).
/// * thickness: Thickness of lines that make up the rectangle. Negative values, like #FILLED,
/// mean that the function has to draw a filled rectangle.
/// * lineType: Type of the line. See #LineTypes
/// * shift: Number of fractional bits in the point coordinates.
///
/// ## C++ default parameters:
/// * thickness: 1
/// * line_type: LINE_8
/// * shift: 0
pub fn rectangle_points(img: &mut core::Mat, pt1: core::Point, pt2: core::Point, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> Result<()> {
// identifier: cv_rectangle_Mat_img_Point_pt1_Point_pt2_Scalar_color_int_thickness_int_lineType_int_shift
  unsafe {
    let rv = sys::cv_imgproc_cv_rectangle_Mat_img_Point_pt1_Point_pt2_Scalar_color_int_thickness_int_lineType_int_shift(img.as_raw_Mat(), pt1, pt2, color, thickness, line_type, shift);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
/// 
/// use `rec` parameter as alternative specification of the drawn rectangle: `r.tl() and
/// r.br()-Point(1,1)` are opposite corners
///
/// ## C++ default parameters:
/// * thickness: 1
/// * line_type: LINE_8
/// * shift: 0
pub fn rectangle(img: &core::Mat, rec: core::Rect, color: core::Scalar, thickness: i32, line_type: i32, shift: i32) -> Result<()> {
// identifier: cv_rectangle_Mat_img_Rect_rec_Scalar_color_int_thickness_int_lineType_int_shift
  unsafe {
    let rv = sys::cv_imgproc_cv_rectangle_Mat_img_Rect_rec_Scalar_color_int_thickness_int_lineType_int_shift(img.as_raw_Mat(), rec, color, thickness, line_type, shift);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Applies a generic geometrical transformation to an image.
/// 
/// The function remap transforms the source image using the specified map:
/// 
/// <div lang='latex'>\texttt{dst} (x,y) =  \texttt{src} (map_x(x,y),map_y(x,y))</div>
/// 
/// where values of pixels with non-integer coordinates are computed using one of available
/// interpolation methods. <span lang='latex'>map_x</span> and <span lang='latex'>map_y</span> can be encoded as separate floating-point maps
/// in <span lang='latex'>map_1</span> and <span lang='latex'>map_2</span> respectively, or interleaved floating-point maps of <span lang='latex'>(x,y)</span> in
/// <span lang='latex'>map_1</span>, or fixed-point maps created by using convertMaps. The reason you might want to
/// convert from floating to fixed-point representations of a map is that they can yield much faster
/// (\~2x) remapping operations. In the converted case, <span lang='latex'>map_1</span> contains pairs (cvFloor(x),
/// cvFloor(y)) and <span lang='latex'>map_2</span> contains indices in a table of interpolation coefficients.
/// 
/// This function cannot operate in-place.
/// 
/// ## Parameters
/// * src: Source image.
/// * dst: Destination image. It has the same size as map1 and the same type as src .
/// * map1: The first map of either (x,y) points or just x values having the type CV_16SC2 ,
/// CV_32FC1, or CV_32FC2. See convertMaps for details on converting a floating point
/// representation to fixed-point for speed.
/// * map2: The second map of y values having the type CV_16UC1, CV_32FC1, or none (empty map
/// if map1 is (x,y) points), respectively.
/// * interpolation: Interpolation method (see #InterpolationFlags). The method #INTER_AREA is
/// not supported by this function.
/// * borderMode: Pixel extrapolation method (see #BorderTypes). When
/// borderMode=#BORDER_TRANSPARENT, it means that the pixels in the destination image that
/// corresponds to the "outliers" in the source image are not modified by the function.
/// * borderValue: Value used in case of a constant border. By default, it is 0.
/// 
/// Note:
/// Due to current implementation limitations the size of an input and output images should be less than 32767x32767.
///
/// ## C++ default parameters:
/// * border_mode: BORDER_CONSTANT
/// * border_value: Scalar()
pub fn remap(src: &core::Mat, dst: &mut core::Mat, map1: &core::Mat, map2: &core::Mat, interpolation: i32, border_mode: i32, border_value: core::Scalar) -> Result<()> {
// identifier: cv_remap_Mat_src_Mat_dst_Mat_map1_Mat_map2_int_interpolation_int_borderMode_Scalar_borderValue
  unsafe {
    let rv = sys::cv_imgproc_cv_remap_Mat_src_Mat_dst_Mat_map1_Mat_map2_int_interpolation_int_borderMode_Scalar_borderValue(src.as_raw_Mat(), dst.as_raw_Mat(), map1.as_raw_Mat(), map2.as_raw_Mat(), interpolation, border_mode, border_value);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Resizes an image.
/// 
/// The function resize resizes the image src down to or up to the specified size. Note that the
/// initial dst type or size are not taken into account. Instead, the size and type are derived from
/// the `src`,`dsize`,`fx`, and `fy`. If you want to resize src so that it fits the pre-created dst,
/// you may call the function as follows:
/// ```ignore
/// // explicitly specify dsize=dst.size(); fx and fy will be computed from that.
/// resize(src, dst, dst.size(), 0, 0, interpolation);
/// ```
/// 
/// If you want to decimate the image by factor of 2 in each direction, you can call the function this
/// way:
/// ```ignore
/// // specify fx and fy and let the function compute the destination image size.
/// resize(src, dst, Size(), 0.5, 0.5, interpolation);
/// ```
/// 
/// To shrink an image, it will generally look best with #INTER_AREA interpolation, whereas to
/// enlarge an image, it will generally look best with c#INTER_CUBIC (slow) or #INTER_LINEAR
/// (faster but still looks OK).
/// 
/// ## Parameters
/// * src: input image.
/// * dst: output image; it has the size dsize (when it is non-zero) or the size computed from
/// src.size(), fx, and fy; the type of dst is the same as of src.
/// * dsize: output image size; if it equals zero, it is computed as:
/// <div lang='latex'>\texttt{dsize = Size(round(fx*src.cols), round(fy*src.rows))}</div>
/// Either dsize or both fx and fy must be non-zero.
/// * fx: scale factor along the horizontal axis; when it equals 0, it is computed as
/// <div lang='latex'>\texttt{(double)dsize.width/src.cols}</div>
/// * fy: scale factor along the vertical axis; when it equals 0, it is computed as
/// <div lang='latex'>\texttt{(double)dsize.height/src.rows}</div>
/// * interpolation: interpolation method, see #InterpolationFlags
/// 
/// @sa  warpAffine, warpPerspective, remap
///
/// ## C++ default parameters:
/// * fx: 0
/// * fy: 0
/// * interpolation: INTER_LINEAR
pub fn resize(src: &core::Mat, dst: &mut core::Mat, dsize: core::Size, fx: f64, fy: f64, interpolation: i32) -> Result<()> {
// identifier: cv_resize_Mat_src_Mat_dst_Size_dsize_double_fx_double_fy_int_interpolation
  unsafe {
    let rv = sys::cv_imgproc_cv_resize_Mat_src_Mat_dst_Size_dsize_double_fx_double_fy_int_interpolation(src.as_raw_Mat(), dst.as_raw_Mat(), dsize, fx, fy, interpolation);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Finds out if there is any intersection between two rotated rectangles.
/// 
/// If there is then the vertices of the intersecting region are returned as well.
/// 
/// Below are some examples of intersection configurations. The hatched pattern indicates the
/// intersecting region and the red vertices are returned by the function.
/// 
/// ![intersection examples](pics/intersection.png)
/// 
/// ## Parameters
/// * rect1: First rectangle
/// * rect2: Second rectangle
/// * intersectingRegion: The output array of the vertices of the intersecting region. It returns
/// at most 8 vertices. Stored as std::vector\<cv::Point2f\> or cv::Mat as Mx1 of type CV_32FC2.
/// @returns One of #RectanglesIntersectTypes
pub fn rotated_rectangle_intersection(rect1: &core::RotatedRect, rect2: &core::RotatedRect, intersecting_region: &mut core::Mat) -> Result<i32> {
// identifier: cv_rotatedRectangleIntersection_RotatedRect_rect1_RotatedRect_rect2_Mat_intersectingRegion
  unsafe {
    let rv = sys::cv_imgproc_cv_rotatedRectangleIntersection_RotatedRect_rect1_RotatedRect_rect2_Mat_intersectingRegion(rect1.as_raw_RotatedRect(), rect2.as_raw_RotatedRect(), intersecting_region.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Applies a separable linear filter to an image.
/// 
/// The function applies a separable linear filter to the image. That is, first, every row of src is
/// filtered with the 1D kernel kernelX. Then, every column of the result is filtered with the 1D
/// kernel kernelY. The final result shifted by delta is stored in dst .
/// 
/// ## Parameters
/// * src: Source image.
/// * dst: Destination image of the same size and the same number of channels as src .
/// * ddepth: Destination image depth, see @ref filter_depths "combinations"
/// * kernelX: Coefficients for filtering each row.
/// * kernelY: Coefficients for filtering each column.
/// * anchor: Anchor position within the kernel. The default value <span lang='latex'>(-1,-1)</span> means that the anchor
/// is at the kernel center.
/// * delta: Value added to the filtered results before storing them.
/// * borderType: Pixel extrapolation method, see #BorderTypes
/// @sa  filter2D, Sobel, GaussianBlur, boxFilter, blur
///
/// ## C++ default parameters:
/// * anchor: Point(-1,-1)
/// * delta: 0
/// * border_type: BORDER_DEFAULT
pub fn sep_filter2_d(src: &core::Mat, dst: &mut core::Mat, ddepth: i32, kernel_x: &core::Mat, kernel_y: &core::Mat, anchor: core::Point, delta: f64, border_type: i32) -> Result<()> {
// identifier: cv_sepFilter2D_Mat_src_Mat_dst_int_ddepth_Mat_kernelX_Mat_kernelY_Point_anchor_double_delta_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_sepFilter2D_Mat_src_Mat_dst_int_ddepth_Mat_kernelX_Mat_kernelY_Point_anchor_double_delta_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), ddepth, kernel_x.as_raw_Mat(), kernel_y.as_raw_Mat(), anchor, delta, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates the first order image derivative in both x and y using a Sobel operator
/// 
/// Equivalent to calling:
/// 
/// ```ignore
/// Sobel( src, dx, CV_16SC1, 1, 0, 3 );
/// Sobel( src, dy, CV_16SC1, 0, 1, 3 );
/// ```
/// 
/// 
/// ## Parameters
/// * src: input image.
/// * dx: output image with first-order derivative in x.
/// * dy: output image with first-order derivative in y.
/// * ksize: size of Sobel kernel. It must be 3.
/// * borderType: pixel extrapolation method, see #BorderTypes
/// 
/// @sa Sobel
///
/// ## C++ default parameters:
/// * ksize: 3
/// * border_type: BORDER_DEFAULT
pub fn spatial_gradient(src: &core::Mat, dx: &mut core::Mat, dy: &mut core::Mat, ksize: i32, border_type: i32) -> Result<()> {
// identifier: cv_spatialGradient_Mat_src_Mat_dx_Mat_dy_int_ksize_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_spatialGradient_Mat_src_Mat_dx_Mat_dy_int_ksize_int_borderType(src.as_raw_Mat(), dx.as_raw_Mat(), dy.as_raw_Mat(), ksize, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Calculates the normalized sum of squares of the pixel values overlapping the filter.
/// 
/// For every pixel <span lang='latex'> (x, y) </span> in the source image, the function calculates the sum of squares of those neighboring
/// pixel values which overlap the filter placed over the pixel <span lang='latex'> (x, y) </span>.
/// 
/// The unnormalized square box filter can be useful in computing local image statistics such as the the local
/// variance and standard deviation around the neighborhood of a pixel.
/// 
/// ## Parameters
/// * src: input image
/// * dst: output image of the same size and type as _src
/// * ddepth: the output image depth (-1 to use src.depth())
/// * ksize: kernel size
/// * anchor: kernel anchor point. The default value of Point(-1, -1) denotes that the anchor is at the kernel
/// center.
/// * normalize: flag, specifying whether the kernel is to be normalized by it's area or not.
/// * borderType: border mode used to extrapolate pixels outside of the image, see #BorderTypes
/// @sa boxFilter
///
/// ## C++ default parameters:
/// * anchor: Point(-1, -1)
/// * normalize: true
/// * border_type: BORDER_DEFAULT
pub fn sqr_box_filter(src: &core::Mat, dst: &mut core::Mat, ddepth: i32, ksize: core::Size, anchor: core::Point, normalize: bool, border_type: i32) -> Result<()> {
// identifier: cv_sqrBoxFilter_Mat_src_Mat_dst_int_ddepth_Size_ksize_Point_anchor_bool_normalize_int_borderType
  unsafe {
    let rv = sys::cv_imgproc_cv_sqrBoxFilter_Mat_src_Mat_dst_int_ddepth_Size_ksize_Point_anchor_bool_normalize_int_borderType(src.as_raw_Mat(), dst.as_raw_Mat(), ddepth, ksize, anchor, normalize, border_type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Applies a fixed-level threshold to each array element.
/// 
/// The function applies fixed-level thresholding to a multiple-channel array. The function is typically
/// used to get a bi-level (binary) image out of a grayscale image ( #compare could be also used for
/// this purpose) or for removing a noise, that is, filtering out pixels with too small or too large
/// values. There are several types of thresholding supported by the function. They are determined by
/// type parameter.
/// 
/// Also, the special values #THRESH_OTSU or #THRESH_TRIANGLE may be combined with one of the
/// above values. In these cases, the function determines the optimal threshold value using the Otsu's
/// or Triangle algorithm and uses it instead of the specified thresh.
/// 
/// 
/// Note: Currently, the Otsu's and Triangle methods are implemented only for 8-bit single-channel images.
/// 
/// ## Parameters
/// * src: input array (multiple-channel, 8-bit or 32-bit floating point).
/// * dst: output array of the same size  and type and the same number of channels as src.
/// * thresh: threshold value.
/// * maxval: maximum value to use with the #THRESH_BINARY and #THRESH_BINARY_INV thresholding
/// types.
/// * type: thresholding type (see #ThresholdTypes).
/// @return the computed threshold value if Otsu's or Triangle methods used.
/// 
/// @sa  adaptiveThreshold, findContours, compare, min, max
pub fn threshold(src: &core::Mat, dst: &mut core::Mat, thresh: f64, maxval: f64, _type: i32) -> Result<f64> {
// identifier: cv_threshold_Mat_src_Mat_dst_double_thresh_double_maxval_int_type
  unsafe {
    let rv = sys::cv_imgproc_cv_threshold_Mat_src_Mat_dst_double_thresh_double_maxval_int_type(src.as_raw_Mat(), dst.as_raw_Mat(), thresh, maxval, _type);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Computes the ideal point coordinates from the observed point coordinates.
/// 
/// The function is similar to #undistort and #initUndistortRectifyMap but it operates on a
/// sparse set of points instead of a raster image. Also the function performs a reverse transformation
/// to projectPoints. In case of a 3D object, it does not reconstruct its 3D coordinates, but for a
/// planar object, it does, up to a translation vector, if the proper R is specified.
/// 
/// For each observed point coordinate <span lang='latex'>(u, v)</span> the function computes:
/// <div lang='latex'>
/// \begin{array}{l}
/// x^{"}  \leftarrow (u - c_x)/f_x  \\
/// y^{"}  \leftarrow (v - c_y)/f_y  \\
/// (x',y') = undistort(x^{"},y^{"}, \texttt{distCoeffs}) \\
/// {[X\,Y\,W]} ^T  \leftarrow R*[x' \, y' \, 1]^T  \\
/// x  \leftarrow X/W  \\
/// y  \leftarrow Y/W  \\
/// \text{only performed if P is specified:} \\
/// u'  \leftarrow x {f'}_x + {c'}_x  \\
/// v'  \leftarrow y {f'}_y + {c'}_y
/// \end{array}
/// </div>
/// 
/// where *undistort* is an approximate iterative algorithm that estimates the normalized original
/// point coordinates out of the normalized distorted point coordinates ("normalized" means that the
/// coordinates do not depend on the camera matrix).
/// 
/// The function can be used for both a stereo camera head or a monocular camera (when R is empty).
/// 
/// ## Parameters
/// * src: Observed point coordinates, 1xN or Nx1 2-channel (CV_32FC2 or CV_64FC2).
/// * dst: Output ideal point coordinates after undistortion and reverse perspective
/// transformation. If matrix P is identity or omitted, dst will contain normalized point coordinates.
/// * cameraMatrix: Camera matrix <span lang='latex'>\vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> .
/// * distCoeffs: Input vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6[, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span>
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * R: Rectification transformation in the object space (3x3 matrix). R1 or R2 computed by
/// #stereoRectify can be passed here. If the matrix is empty, the identity transformation is used.
/// * P: New camera matrix (3x3) or new projection matrix (3x4) <span lang='latex'>\begin{bmatrix} {f'}_x & 0 & {c'}_x & t_x \\ 0 & {f'}_y & {c'}_y & t_y \\ 0 & 0 & 1 & t_z \end{bmatrix}</span>. P1 or P2 computed by
/// #stereoRectify can be passed here. If the matrix is empty, the identity new camera matrix is used.
///
/// ## C++ default parameters:
/// * r: noArray()
/// * p: noArray()
pub fn undistort_points(src: &core::Mat, dst: &mut core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, r: &core::Mat, p: &core::Mat) -> Result<()> {
// identifier: cv_undistortPoints_Mat_src_Mat_dst_Mat_cameraMatrix_Mat_distCoeffs_Mat_R_Mat_P
  unsafe {
    let rv = sys::cv_imgproc_cv_undistortPoints_Mat_src_Mat_dst_Mat_cameraMatrix_Mat_distCoeffs_Mat_R_Mat_P(src.as_raw_Mat(), dst.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), r.as_raw_Mat(), p.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
/// 
/// Note: Default version of #undistortPoints does 5 iterations to compute undistorted points.
pub fn undistort_points_v0(src: &core::Mat, dst: &mut core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, r: &core::Mat, p: &core::Mat, criteria: &core::TermCriteria) -> Result<()> {
// identifier: cv_undistortPoints_Mat_src_Mat_dst_Mat_cameraMatrix_Mat_distCoeffs_Mat_R_Mat_P_TermCriteria_criteria
  unsafe {
    let rv = sys::cv_imgproc_cv_undistortPoints_Mat_src_Mat_dst_Mat_cameraMatrix_Mat_distCoeffs_Mat_R_Mat_P_TermCriteria_criteria(src.as_raw_Mat(), dst.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), r.as_raw_Mat(), p.as_raw_Mat(), criteria.as_raw_TermCriteria());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Transforms an image to compensate for lens distortion.
/// 
/// The function transforms an image to compensate radial and tangential lens distortion.
/// 
/// The function is simply a combination of #initUndistortRectifyMap (with unity R ) and #remap
/// (with bilinear interpolation). See the former function for details of the transformation being
/// performed.
/// 
/// Those pixels in the destination image, for which there is no correspondent pixels in the source
/// image, are filled with zeros (black color).
/// 
/// A particular subset of the source image that will be visible in the corrected image can be regulated
/// by newCameraMatrix. You can use #getOptimalNewCameraMatrix to compute the appropriate
/// newCameraMatrix depending on your requirements.
/// 
/// The camera matrix and the distortion parameters can be determined using #calibrateCamera. If
/// the resolution of images is different from the resolution used at the calibration stage, \f$f_x,
/// f_y, c_x<span lang='latex'> and </span>c_y\f$ need to be scaled accordingly, while the distortion coefficients remain
/// the same.
/// 
/// ## Parameters
/// * src: Input (distorted) image.
/// * dst: Output (corrected) image that has the same size and type as src .
/// * cameraMatrix: Input camera matrix <span lang='latex'>A = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> .
/// * distCoeffs: Input vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6[, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span>
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * newCameraMatrix: Camera matrix of the distorted image. By default, it is the same as
/// cameraMatrix but you may additionally scale and shift the result by using a different matrix.
///
/// ## C++ default parameters:
/// * new_camera_matrix: noArray()
pub fn undistort(src: &core::Mat, dst: &mut core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, new_camera_matrix: &core::Mat) -> Result<()> {
// identifier: cv_undistort_Mat_src_Mat_dst_Mat_cameraMatrix_Mat_distCoeffs_Mat_newCameraMatrix
  unsafe {
    let rv = sys::cv_imgproc_cv_undistort_Mat_src_Mat_dst_Mat_cameraMatrix_Mat_distCoeffs_Mat_newCameraMatrix(src.as_raw_Mat(), dst.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), new_camera_matrix.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Applies an affine transformation to an image.
/// 
/// The function warpAffine transforms the source image using the specified matrix:
/// 
/// <div lang='latex'>\texttt{dst} (x,y) =  \texttt{src} ( \texttt{M} _{11} x +  \texttt{M} _{12} y +  \texttt{M} _{13}, \texttt{M} _{21} x +  \texttt{M} _{22} y +  \texttt{M} _{23})</div>
/// 
/// when the flag #WARP_INVERSE_MAP is set. Otherwise, the transformation is first inverted
/// with #invertAffineTransform and then put in the formula above instead of M. The function cannot
/// operate in-place.
/// 
/// ## Parameters
/// * src: input image.
/// * dst: output image that has the size dsize and the same type as src .
/// * M: <span lang='latex'>2\times 3</span> transformation matrix.
/// * dsize: size of the output image.
/// * flags: combination of interpolation methods (see #InterpolationFlags) and the optional
/// flag #WARP_INVERSE_MAP that means that M is the inverse transformation (
/// <span lang='latex'>\texttt{dst}\rightarrow\texttt{src}</span> ).
/// * borderMode: pixel extrapolation method (see #BorderTypes); when
/// borderMode=#BORDER_TRANSPARENT, it means that the pixels in the destination image corresponding to
/// the "outliers" in the source image are not modified by the function.
/// * borderValue: value used in case of a constant border; by default, it is 0.
/// 
/// @sa  warpPerspective, resize, remap, getRectSubPix, transform
///
/// ## C++ default parameters:
/// * flags: INTER_LINEAR
/// * border_mode: BORDER_CONSTANT
/// * border_value: Scalar()
pub fn warp_affine(src: &core::Mat, dst: &mut core::Mat, m: &core::Mat, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar) -> Result<()> {
// identifier: cv_warpAffine_Mat_src_Mat_dst_Mat_M_Size_dsize_int_flags_int_borderMode_Scalar_borderValue
  unsafe {
    let rv = sys::cv_imgproc_cv_warpAffine_Mat_src_Mat_dst_Mat_M_Size_dsize_int_flags_int_borderMode_Scalar_borderValue(src.as_raw_Mat(), dst.as_raw_Mat(), m.as_raw_Mat(), dsize, flags, border_mode, border_value);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Applies a perspective transformation to an image.
/// 
/// The function warpPerspective transforms the source image using the specified matrix:
/// 
/// <div lang='latex'>\texttt{dst} (x,y) =  \texttt{src} \left ( \frac{M_{11} x + M_{12} y + M_{13}}{M_{31} x + M_{32} y + M_{33}} ,
/// \frac{M_{21} x + M_{22} y + M_{23}}{M_{31} x + M_{32} y + M_{33}} \right )</div>
/// 
/// when the flag #WARP_INVERSE_MAP is set. Otherwise, the transformation is first inverted with invert
/// and then put in the formula above instead of M. The function cannot operate in-place.
/// 
/// ## Parameters
/// * src: input image.
/// * dst: output image that has the size dsize and the same type as src .
/// * M: <span lang='latex'>3\times 3</span> transformation matrix.
/// * dsize: size of the output image.
/// * flags: combination of interpolation methods (#INTER_LINEAR or #INTER_NEAREST) and the
/// optional flag #WARP_INVERSE_MAP, that sets M as the inverse transformation (
/// <span lang='latex'>\texttt{dst}\rightarrow\texttt{src}</span> ).
/// * borderMode: pixel extrapolation method (#BORDER_CONSTANT or #BORDER_REPLICATE).
/// * borderValue: value used in case of a constant border; by default, it equals 0.
/// 
/// @sa  warpAffine, resize, remap, getRectSubPix, perspectiveTransform
///
/// ## C++ default parameters:
/// * flags: INTER_LINEAR
/// * border_mode: BORDER_CONSTANT
/// * border_value: Scalar()
pub fn warp_perspective(src: &core::Mat, dst: &mut core::Mat, m: &core::Mat, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar) -> Result<()> {
// identifier: cv_warpPerspective_Mat_src_Mat_dst_Mat_M_Size_dsize_int_flags_int_borderMode_Scalar_borderValue
  unsafe {
    let rv = sys::cv_imgproc_cv_warpPerspective_Mat_src_Mat_dst_Mat_M_Size_dsize_int_flags_int_borderMode_Scalar_borderValue(src.as_raw_Mat(), dst.as_raw_Mat(), m.as_raw_Mat(), dsize, flags, border_mode, border_value);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// \brief Remaps an image to polar or semilog-polar coordinates space
/// 
/// @anchor polar_remaps_reference_image
/// ![Polar remaps reference](pics/polar_remap_doc.png)
/// 
/// Transform the source image using the following transformation:
/// <div lang='latex'>
/// dst(\rho , \phi ) = src(x,y)
/// </div>
/// 
/// where
/// <div lang='latex'>
/// \begin{array}{l}
/// \vec{I} = (x - center.x, \;y - center.y) \\
/// \phi = Kangle \cdot \texttt{angle} (\vec{I}) \\
/// \rho = \left\{\begin{matrix}
/// Klin \cdot \texttt{magnitude} (\vec{I}) & default \\
/// Klog \cdot log_e(\texttt{magnitude} (\vec{I})) & if \; semilog \\
/// \end{matrix}\right.
/// \end{array}
/// </div>
/// 
/// and
/// <div lang='latex'>
/// \begin{array}{l}
/// Kangle = dsize.height / 2\Pi \\
/// Klin = dsize.width / maxRadius \\
/// Klog = dsize.width / log_e(maxRadius) \\
/// \end{array}
/// </div>
/// 
/// 
/// \par Linear vs semilog mapping
/// 
/// Polar mapping can be linear or semi-log. Add one of #WarpPolarMode to `flags` to specify the polar mapping mode.
/// 
/// Linear is the default mode.
/// 
/// The semilog mapping emulates the human "foveal" vision that permit very high acuity on the line of sight (central vision)
/// in contrast to peripheral vision where acuity is minor.
/// 
/// \par Option on `dsize`:
/// 
/// - if both values in `dsize <=0 ` (default),
/// the destination image will have (almost) same area of source bounding circle:
/// <div lang='latex'>\begin{array}{l}
/// dsize.area  \leftarrow (maxRadius^2 \cdot \Pi) \\
/// dsize.width = \texttt{cvRound}(maxRadius) \\
/// dsize.height = \texttt{cvRound}(maxRadius \cdot \Pi) \\
/// \end{array}</div>
/// 
/// 
/// - if only `dsize.height <= 0`,
/// the destination image area will be proportional to the bounding circle area but scaled by `Kx * Kx`:
/// <div lang='latex'>\begin{array}{l}
/// dsize.height = \texttt{cvRound}(dsize.width \cdot \Pi) \\
/// \end{array}
/// </div>
/// 
/// - if both values in `dsize > 0 `,
/// the destination image will have the given size therefore the area of the bounding circle will be scaled to `dsize`.
/// 
/// 
/// \par Reverse mapping
/// 
/// You can get reverse mapping adding #WARP_INVERSE_MAP to `flags`
/// \snippet polar_transforms.cpp InverseMap
/// 
/// In addiction, to calculate the original coordinate from a polar mapped coordinate <span lang='latex'>(rho, phi)->(x, y)</span>:
/// \snippet polar_transforms.cpp InverseCoordinate
/// 
/// ## Parameters
/// * src: Source image.
/// * dst: Destination image. It will have same type as src.
/// * dsize: The destination image size (see description for valid options).
/// * center: The transformation center.
/// * maxRadius: The radius of the bounding circle to transform. It determines the inverse magnitude scale parameter too.
/// * flags: A combination of interpolation methods, #InterpolationFlags + #WarpPolarMode.
/// - Add #WARP_POLAR_LINEAR to select linear polar mapping (default)
/// - Add #WARP_POLAR_LOG to select semilog polar mapping
/// - Add #WARP_INVERSE_MAP for reverse mapping.
/// 
/// Note:
/// *  The function can not operate in-place.
/// *  To calculate magnitude and angle in degrees #cartToPolar is used internally thus angles are measured from 0 to 360 with accuracy about 0.3 degrees.
/// *  This function uses #remap. Due to current implementation limitations the size of an input and output images should be less than 32767x32767.
/// 
/// @sa cv::remap
pub fn warp_polar(src: &core::Mat, dst: &mut core::Mat, dsize: core::Size, center: core::Point2f, max_radius: f64, flags: i32) -> Result<()> {
// identifier: cv_warpPolar_Mat_src_Mat_dst_Size_dsize_Point2f_center_double_maxRadius_int_flags
  unsafe {
    let rv = sys::cv_imgproc_cv_warpPolar_Mat_src_Mat_dst_Size_dsize_Point2f_center_double_maxRadius_int_flags(src.as_raw_Mat(), dst.as_raw_Mat(), dsize, center, max_radius, flags);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Performs a marker-based image segmentation using the watershed algorithm.
/// 
/// The function implements one of the variants of watershed, non-parametric marker-based segmentation
/// algorithm, described in @cite Meyer92 .
/// 
/// Before passing the image to the function, you have to roughly outline the desired regions in the
/// image markers with positive (\>0) indices. So, every region is represented as one or more connected
/// components with the pixel values 1, 2, 3, and so on. Such markers can be retrieved from a binary
/// mask using #findContours and #drawContours (see the watershed.cpp demo). The markers are "seeds" of
/// the future image regions. All the other pixels in markers , whose relation to the outlined regions
/// is not known and should be defined by the algorithm, should be set to 0's. In the function output,
/// each pixel in markers is set to a value of the "seed" components or to -1 at boundaries between the
/// regions.
/// 
/// 
/// Note: Any two neighbor connected components are not necessarily separated by a watershed boundary
/// (-1's pixels); for example, they can touch each other in the initial marker image passed to the
/// function.
/// 
/// ## Parameters
/// * image: Input 8-bit 3-channel image.
/// * markers: Input/output 32-bit single-channel image (map) of markers. It should have the same
/// size as image .
/// 
/// @sa findContours
/// 
/// @ingroup imgproc_misc
pub fn watershed(image: &core::Mat, markers: &mut core::Mat) -> Result<()> {
// identifier: cv_watershed_Mat_image_Mat_markers
  unsafe {
    let rv = sys::cv_imgproc_cv_watershed_Mat_image_Mat_markers(image.as_raw_Mat(), markers.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

///
/// ## C++ default parameters:
/// * cost: noArray()
/// * lower_bound: Ptr<float>()
/// * flow: noArray()
pub fn wrapper_emd(signature1: &core::Mat, signature2: &core::Mat, dist_type: i32, cost: &core::Mat, lower_bound: &types::PtrOffloat, flow: &mut core::Mat) -> Result<f32> {
// identifier: cv_wrapperEMD_Mat_signature1_Mat_signature2_int_distType_Mat_cost_PtrOffloat_lowerBound_Mat_flow
  unsafe {
    let rv = sys::cv_imgproc_cv_wrapperEMD_Mat_signature1_Mat_signature2_int_distType_Mat_cost_PtrOffloat_lowerBound_Mat_flow(signature1.as_raw_Mat(), signature2.as_raw_Mat(), dist_type, cost.as_raw_Mat(), lower_bound.as_raw_PtrOffloat(), flow.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

// Generating impl for trait cv::CLAHE (trait)
/// Base class for Contrast Limited Adaptive Histogram Equalization.
pub trait CLAHE : core::Algorithm {
  #[doc(hidden)] fn as_raw_CLAHE(&self) -> *mut c_void;
  /// Equalizes the histogram of a grayscale image using Contrast Limited Adaptive Histogram Equalization.
  /// 
  /// ## Parameters
  /// * src: Source image of type CV_8UC1 or CV_16UC1.
  /// * dst: Destination image.
  fn apply(&mut self, src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
  // identifier: cv_CLAHE_apply_Mat_src_Mat_dst
    unsafe {
      let rv = sys::cv_imgproc_cv_CLAHE_apply_Mat_src_Mat_dst(self.as_raw_CLAHE(), src.as_raw_Mat(), dst.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Sets threshold for contrast limiting.
  /// 
  /// ## Parameters
  /// * clipLimit: threshold value.
  fn set_clip_limit(&mut self, clip_limit: f64) -> Result<()> {
  // identifier: cv_CLAHE_setClipLimit_double_clipLimit
    unsafe {
      let rv = sys::cv_imgproc_cv_CLAHE_setClipLimit_double_clipLimit(self.as_raw_CLAHE(), clip_limit);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_clip_limit(&self) -> Result<f64> {
  // identifier: cv_CLAHE_getClipLimit
    unsafe {
      let rv = sys::cv_imgproc_cv_CLAHE_getClipLimit(self.as_raw_CLAHE());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Sets size of grid for histogram equalization. Input image will be divided into
  /// equally sized rectangular tiles.
  /// 
  /// ## Parameters
  /// * tileGridSize: defines the number of tiles in row and column.
  fn set_tiles_grid_size(&mut self, tile_grid_size: core::Size) -> Result<()> {
  // identifier: cv_CLAHE_setTilesGridSize_Size_tileGridSize
    unsafe {
      let rv = sys::cv_imgproc_cv_CLAHE_setTilesGridSize_Size_tileGridSize(self.as_raw_CLAHE(), tile_grid_size);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_tiles_grid_size(&self) -> Result<core::Size> {
  // identifier: cv_CLAHE_getTilesGridSize
    unsafe {
      let rv = sys::cv_imgproc_cv_CLAHE_getTilesGridSize(self.as_raw_CLAHE());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn collect_garbage(&mut self) -> Result<()> {
  // identifier: cv_CLAHE_collectGarbage
    unsafe {
      let rv = sys::cv_imgproc_cv_CLAHE_collectGarbage(self.as_raw_CLAHE());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

}
impl<'a> CLAHE + 'a {

}

// Generating impl for trait cv::GeneralizedHough (trait)
/// finds arbitrary template in the grayscale image using Generalized Hough Transform
pub trait GeneralizedHough : core::Algorithm {
  #[doc(hidden)] fn as_raw_GeneralizedHough(&self) -> *mut c_void;
  ///
  /// ## C++ default parameters:
  /// * templ_center: Point(-1, -1)
  fn set_template(&mut self, templ: &core::Mat, templ_center: core::Point) -> Result<()> {
  // identifier: cv_GeneralizedHough_setTemplate_Mat_templ_Point_templCenter
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_setTemplate_Mat_templ_Point_templCenter(self.as_raw_GeneralizedHough(), templ.as_raw_Mat(), templ_center);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  ///
  /// ## C++ default parameters:
  /// * templ_center: Point(-1, -1)
  fn set_template_v0(&mut self, edges: &core::Mat, dx: &core::Mat, dy: &core::Mat, templ_center: core::Point) -> Result<()> {
  // identifier: cv_GeneralizedHough_setTemplate_Mat_edges_Mat_dx_Mat_dy_Point_templCenter
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_setTemplate_Mat_edges_Mat_dx_Mat_dy_Point_templCenter(self.as_raw_GeneralizedHough(), edges.as_raw_Mat(), dx.as_raw_Mat(), dy.as_raw_Mat(), templ_center);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  ///
  /// ## C++ default parameters:
  /// * votes: noArray()
  fn detect(&mut self, image: &core::Mat, positions: &mut core::Mat, votes: &mut core::Mat) -> Result<()> {
  // identifier: cv_GeneralizedHough_detect_Mat_image_Mat_positions_Mat_votes
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_detect_Mat_image_Mat_positions_Mat_votes(self.as_raw_GeneralizedHough(), image.as_raw_Mat(), positions.as_raw_Mat(), votes.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  ///
  /// ## C++ default parameters:
  /// * votes: noArray()
  fn detect_v0(&mut self, edges: &core::Mat, dx: &core::Mat, dy: &core::Mat, positions: &mut core::Mat, votes: &mut core::Mat) -> Result<()> {
  // identifier: cv_GeneralizedHough_detect_Mat_edges_Mat_dx_Mat_dy_Mat_positions_Mat_votes
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_detect_Mat_edges_Mat_dx_Mat_dy_Mat_positions_Mat_votes(self.as_raw_GeneralizedHough(), edges.as_raw_Mat(), dx.as_raw_Mat(), dy.as_raw_Mat(), positions.as_raw_Mat(), votes.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn set_canny_low_thresh(&mut self, canny_low_thresh: i32) -> Result<()> {
  // identifier: cv_GeneralizedHough_setCannyLowThresh_int_cannyLowThresh
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_setCannyLowThresh_int_cannyLowThresh(self.as_raw_GeneralizedHough(), canny_low_thresh);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_canny_low_thresh(&self) -> Result<i32> {
  // identifier: cv_GeneralizedHough_getCannyLowThresh
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_getCannyLowThresh(self.as_raw_GeneralizedHough());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_canny_high_thresh(&mut self, canny_high_thresh: i32) -> Result<()> {
  // identifier: cv_GeneralizedHough_setCannyHighThresh_int_cannyHighThresh
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_setCannyHighThresh_int_cannyHighThresh(self.as_raw_GeneralizedHough(), canny_high_thresh);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_canny_high_thresh(&self) -> Result<i32> {
  // identifier: cv_GeneralizedHough_getCannyHighThresh
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_getCannyHighThresh(self.as_raw_GeneralizedHough());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_min_dist(&mut self, min_dist: f64) -> Result<()> {
  // identifier: cv_GeneralizedHough_setMinDist_double_minDist
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_setMinDist_double_minDist(self.as_raw_GeneralizedHough(), min_dist);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_min_dist(&self) -> Result<f64> {
  // identifier: cv_GeneralizedHough_getMinDist
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_getMinDist(self.as_raw_GeneralizedHough());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_dp(&mut self, dp: f64) -> Result<()> {
  // identifier: cv_GeneralizedHough_setDp_double_dp
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_setDp_double_dp(self.as_raw_GeneralizedHough(), dp);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_dp(&self) -> Result<f64> {
  // identifier: cv_GeneralizedHough_getDp
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_getDp(self.as_raw_GeneralizedHough());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_max_buffer_size(&mut self, max_buffer_size: i32) -> Result<()> {
  // identifier: cv_GeneralizedHough_setMaxBufferSize_int_maxBufferSize
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_setMaxBufferSize_int_maxBufferSize(self.as_raw_GeneralizedHough(), max_buffer_size);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_max_buffer_size(&self) -> Result<i32> {
  // identifier: cv_GeneralizedHough_getMaxBufferSize
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHough_getMaxBufferSize(self.as_raw_GeneralizedHough());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> GeneralizedHough + 'a {

}

// Generating impl for trait cv::GeneralizedHoughBallard (trait)
/// finds arbitrary template in the grayscale image using Generalized Hough Transform
/// 
/// Detects position only without translation and rotation @cite Ballard1981 .
pub trait GeneralizedHoughBallard : super::imgproc::GeneralizedHough {
  #[doc(hidden)] fn as_raw_GeneralizedHoughBallard(&self) -> *mut c_void;
  fn set_levels(&mut self, levels: i32) -> Result<()> {
  // identifier: cv_GeneralizedHoughBallard_setLevels_int_levels
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughBallard_setLevels_int_levels(self.as_raw_GeneralizedHoughBallard(), levels);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_levels(&self) -> Result<i32> {
  // identifier: cv_GeneralizedHoughBallard_getLevels
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughBallard_getLevels(self.as_raw_GeneralizedHoughBallard());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_votes_threshold(&mut self, votes_threshold: i32) -> Result<()> {
  // identifier: cv_GeneralizedHoughBallard_setVotesThreshold_int_votesThreshold
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughBallard_setVotesThreshold_int_votesThreshold(self.as_raw_GeneralizedHoughBallard(), votes_threshold);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_votes_threshold(&self) -> Result<i32> {
  // identifier: cv_GeneralizedHoughBallard_getVotesThreshold
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughBallard_getVotesThreshold(self.as_raw_GeneralizedHoughBallard());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> GeneralizedHoughBallard + 'a {

}

// Generating impl for trait cv::GeneralizedHoughGuil (trait)
/// finds arbitrary template in the grayscale image using Generalized Hough Transform
/// 
/// Detects position, translation and rotation @cite Guil1999 .
pub trait GeneralizedHoughGuil : super::imgproc::GeneralizedHough {
  #[doc(hidden)] fn as_raw_GeneralizedHoughGuil(&self) -> *mut c_void;
  fn set_xi(&mut self, xi: f64) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setXi_double_xi
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setXi_double_xi(self.as_raw_GeneralizedHoughGuil(), xi);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_xi(&self) -> Result<f64> {
  // identifier: cv_GeneralizedHoughGuil_getXi
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getXi(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_levels(&mut self, levels: i32) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setLevels_int_levels
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setLevels_int_levels(self.as_raw_GeneralizedHoughGuil(), levels);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_levels(&self) -> Result<i32> {
  // identifier: cv_GeneralizedHoughGuil_getLevels
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getLevels(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_angle_epsilon(&mut self, angle_epsilon: f64) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setAngleEpsilon_double_angleEpsilon
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setAngleEpsilon_double_angleEpsilon(self.as_raw_GeneralizedHoughGuil(), angle_epsilon);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_angle_epsilon(&self) -> Result<f64> {
  // identifier: cv_GeneralizedHoughGuil_getAngleEpsilon
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getAngleEpsilon(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_min_angle(&mut self, min_angle: f64) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setMinAngle_double_minAngle
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setMinAngle_double_minAngle(self.as_raw_GeneralizedHoughGuil(), min_angle);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_min_angle(&self) -> Result<f64> {
  // identifier: cv_GeneralizedHoughGuil_getMinAngle
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getMinAngle(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_max_angle(&mut self, max_angle: f64) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setMaxAngle_double_maxAngle
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setMaxAngle_double_maxAngle(self.as_raw_GeneralizedHoughGuil(), max_angle);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_max_angle(&self) -> Result<f64> {
  // identifier: cv_GeneralizedHoughGuil_getMaxAngle
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getMaxAngle(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_angle_step(&mut self, angle_step: f64) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setAngleStep_double_angleStep
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setAngleStep_double_angleStep(self.as_raw_GeneralizedHoughGuil(), angle_step);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_angle_step(&self) -> Result<f64> {
  // identifier: cv_GeneralizedHoughGuil_getAngleStep
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getAngleStep(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_angle_thresh(&mut self, angle_thresh: i32) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setAngleThresh_int_angleThresh
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setAngleThresh_int_angleThresh(self.as_raw_GeneralizedHoughGuil(), angle_thresh);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_angle_thresh(&self) -> Result<i32> {
  // identifier: cv_GeneralizedHoughGuil_getAngleThresh
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getAngleThresh(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_min_scale(&mut self, min_scale: f64) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setMinScale_double_minScale
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setMinScale_double_minScale(self.as_raw_GeneralizedHoughGuil(), min_scale);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_min_scale(&self) -> Result<f64> {
  // identifier: cv_GeneralizedHoughGuil_getMinScale
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getMinScale(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_max_scale(&mut self, max_scale: f64) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setMaxScale_double_maxScale
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setMaxScale_double_maxScale(self.as_raw_GeneralizedHoughGuil(), max_scale);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_max_scale(&self) -> Result<f64> {
  // identifier: cv_GeneralizedHoughGuil_getMaxScale
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getMaxScale(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_scale_step(&mut self, scale_step: f64) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setScaleStep_double_scaleStep
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setScaleStep_double_scaleStep(self.as_raw_GeneralizedHoughGuil(), scale_step);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_scale_step(&self) -> Result<f64> {
  // identifier: cv_GeneralizedHoughGuil_getScaleStep
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getScaleStep(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_scale_thresh(&mut self, scale_thresh: i32) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setScaleThresh_int_scaleThresh
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setScaleThresh_int_scaleThresh(self.as_raw_GeneralizedHoughGuil(), scale_thresh);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_scale_thresh(&self) -> Result<i32> {
  // identifier: cv_GeneralizedHoughGuil_getScaleThresh
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getScaleThresh(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_pos_thresh(&mut self, pos_thresh: i32) -> Result<()> {
  // identifier: cv_GeneralizedHoughGuil_setPosThresh_int_posThresh
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_setPosThresh_int_posThresh(self.as_raw_GeneralizedHoughGuil(), pos_thresh);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_pos_thresh(&self) -> Result<i32> {
  // identifier: cv_GeneralizedHoughGuil_getPosThresh
    unsafe {
      let rv = sys::cv_imgproc_cv_GeneralizedHoughGuil_getPosThresh(self.as_raw_GeneralizedHoughGuil());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> GeneralizedHoughGuil + 'a {

}

// boxed class cv::LineIterator
/// Line iterator
/// 
/// The class is used to iterate over all the pixels on the raster line
/// segment connecting two specified points.
/// 
/// The class LineIterator is used to get each pixel of a raster line. It
/// can be treated as versatile implementation of the Bresenham algorithm
/// where you can stop at each pixel and do some extra processing, for
/// example, grab pixel values along the line or draw a line with an effect
/// (for example, with XOR operation).
/// 
/// The number of pixels along the line is stored in LineIterator::count.
/// The method LineIterator::pos returns the current position in the image:
/// 
/// ```ignore{.cpp}
/// // grabs pixels along the line (pt1, pt2)
/// // from 8-bit 3-channel image to the buffer
/// LineIterator it(img, pt1, pt2, 8);
/// LineIterator it2 = it;
/// vector<Vec3b> buf(it.count);
/// 
/// for(int i = 0; i < it.count; i++, ++it)
/// buf[i] = *(const Vec3b*)*it;
/// 
/// // alternative way of iterating through the line
/// for(int i = 0; i < it2.count; i++, ++it2)
/// {
/// Vec3b val = img.at<Vec3b>(it2.pos());
/// CV_Assert(buf[i] == val);
/// }
/// ```

#[allow(dead_code)]
pub struct LineIterator {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::imgproc::LineIterator {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_LineIterator(self.ptr) };
    }
}
impl super::imgproc::LineIterator {
    #[doc(hidden)] pub fn as_raw_LineIterator(&self) -> *mut c_void { self.ptr }
}
impl LineIterator {

  /// initializes the iterator
  /// 
  /// creates iterators for the line connecting pt1 and pt2
  /// the line will be clipped on the image boundaries
  /// the line is 8-connected or 4-connected
  /// If leftToRight=true, then the iteration is always done
  /// from the left-most point to the right most,
  /// not to depend on the ordering of pt1 and pt2 parameters
  ///
  /// ## C++ default parameters:
  /// * connectivity: 8
  /// * left_to_right: false
  pub fn new(img: &core::Mat, pt1: core::Point, pt2: core::Point, connectivity: i32, left_to_right: bool) -> Result<super::imgproc::LineIterator> {
  // identifier: cv_LineIterator_LineIterator_Mat_img_Point_pt1_Point_pt2_int_connectivity_bool_leftToRight
    unsafe {
      let rv = sys::cv_imgproc_cv_LineIterator_LineIterator_Mat_img_Point_pt1_Point_pt2_int_connectivity_bool_leftToRight(img.as_raw_Mat(), pt1, pt2, connectivity, left_to_right);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::imgproc::LineIterator { ptr: rv.result })
      }
    }
  }

  /// returns coordinates of the current pixel
  pub fn pos(&self) -> Result<core::Point> {
  // identifier: cv_LineIterator_pos
    unsafe {
      let rv = sys::cv_imgproc_cv_LineIterator_pos(self.as_raw_LineIterator());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
// Generating impl for trait cv::LineSegmentDetector (trait)
/// Line segment detector class
/// 
/// following the algorithm described at @cite Rafael12 .
pub trait LineSegmentDetector : core::Algorithm {
  #[doc(hidden)] fn as_raw_LineSegmentDetector(&self) -> *mut c_void;
  /// Finds lines in the input image.
  /// 
  /// This is the output of the default parameters of the algorithm on the above shown image.
  /// 
  /// ![image](pics/building_lsd.png)
  /// 
  /// ## Parameters
  /// * _image: A grayscale (CV_8UC1) input image. If only a roi needs to be selected, use:
  /// `lsd_ptr-\>detect(image(roi), lines, ...); lines += Scalar(roi.x, roi.y, roi.x, roi.y);`
  /// * _lines: A vector of Vec4i or Vec4f elements specifying the beginning and ending point of a line. Where
  /// Vec4i/Vec4f is (x1, y1, x2, y2), point 1 is the start, point 2 - end. Returned lines are strictly
  /// oriented depending on the gradient.
  /// * width: Vector of widths of the regions, where the lines are found. E.g. Width of line.
  /// * prec: Vector of precisions with which the lines are found.
  /// * nfa: Vector containing number of false alarms in the line region, with precision of 10%. The
  /// bigger the value, logarithmically better the detection.
  /// - -1 corresponds to 10 mean false alarms
  /// - 0 corresponds to 1 mean false alarm
  /// - 1 corresponds to 0.1 mean false alarms
  /// This vector will be calculated only when the objects type is #LSD_REFINE_ADV.
  ///
  /// ## C++ default parameters:
  /// * width: noArray()
  /// * prec: noArray()
  /// * nfa: noArray()
  fn detect(&mut self, _image: &core::Mat, _lines: &mut core::Mat, width: &mut core::Mat, prec: &mut core::Mat, nfa: &mut core::Mat) -> Result<()> {
  // identifier: cv_LineSegmentDetector_detect_Mat__image_Mat__lines_Mat_width_Mat_prec_Mat_nfa
    unsafe {
      let rv = sys::cv_imgproc_cv_LineSegmentDetector_detect_Mat__image_Mat__lines_Mat_width_Mat_prec_Mat_nfa(self.as_raw_LineSegmentDetector(), _image.as_raw_Mat(), _lines.as_raw_Mat(), width.as_raw_Mat(), prec.as_raw_Mat(), nfa.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Draws the line segments on a given image.
  /// ## Parameters
  /// * _image: The image, where the lines will be drawn. Should be bigger or equal to the image,
  /// where the lines were found.
  /// * lines: A vector of the lines that needed to be drawn.
  fn draw_segments(&mut self, _image: &mut core::Mat, lines: &core::Mat) -> Result<()> {
  // identifier: cv_LineSegmentDetector_drawSegments_Mat__image_Mat_lines
    unsafe {
      let rv = sys::cv_imgproc_cv_LineSegmentDetector_drawSegments_Mat__image_Mat_lines(self.as_raw_LineSegmentDetector(), _image.as_raw_Mat(), lines.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Draws two groups of lines in blue and red, counting the non overlapping (mismatching) pixels.
  /// 
  /// ## Parameters
  /// * size: The size of the image, where lines1 and lines2 were found.
  /// * lines1: The first group of lines that needs to be drawn. It is visualized in blue color.
  /// * lines2: The second group of lines. They visualized in red color.
  /// * _image: Optional image, where the lines will be drawn. The image should be color(3-channel)
  /// in order for lines1 and lines2 to be drawn in the above mentioned colors.
  ///
  /// ## C++ default parameters:
  /// * _image: noArray()
  fn compare_segments(&mut self, size: core::Size, lines1: &core::Mat, lines2: &core::Mat, _image: &mut core::Mat) -> Result<i32> {
  // identifier: cv_LineSegmentDetector_compareSegments_Size_size_Mat_lines1_Mat_lines2_Mat__image
    unsafe {
      let rv = sys::cv_imgproc_cv_LineSegmentDetector_compareSegments_Size_size_Mat_lines1_Mat_lines2_Mat__image(self.as_raw_LineSegmentDetector(), size, lines1.as_raw_Mat(), lines2.as_raw_Mat(), _image.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> LineSegmentDetector + 'a {

}

// boxed class cv::Subdiv2D

#[allow(dead_code)]
pub struct Subdiv2D {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::imgproc::Subdiv2D {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_Subdiv2D(self.ptr) };
    }
}
impl super::imgproc::Subdiv2D {
    #[doc(hidden)] pub fn as_raw_Subdiv2D(&self) -> *mut c_void { self.ptr }
}
impl Subdiv2D {

  /// creates an empty Subdiv2D object.
  /// To create a new empty Delaunay subdivision you need to use the #initDelaunay function.
  pub fn default() -> Result<super::imgproc::Subdiv2D> {
  // identifier: cv_Subdiv2D_Subdiv2D
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_Subdiv2D();
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::imgproc::Subdiv2D { ptr: rv.result })
      }
    }
  }

  /// @overload
  /// 
  /// ## Parameters
  /// * rect: Rectangle that includes all of the 2D points that are to be added to the subdivision.
  /// 
  /// The function creates an empty Delaunay subdivision where 2D points can be added using the function
  /// insert() . All of the points to be added must be within the specified rectangle, otherwise a runtime
  /// error is raised.
  pub fn new(rect: core::Rect) -> Result<super::imgproc::Subdiv2D> {
  // identifier: cv_Subdiv2D_Subdiv2D_Rect_rect
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_Subdiv2D_Rect_rect(rect);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::imgproc::Subdiv2D { ptr: rv.result })
      }
    }
  }

  /// Creates a new empty Delaunay subdivision
  /// 
  /// ## Parameters
  /// * rect: Rectangle that includes all of the 2D points that are to be added to the subdivision.
  pub fn init_delaunay(&mut self, rect: core::Rect) -> Result<()> {
  // identifier: cv_Subdiv2D_initDelaunay_Rect_rect
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_initDelaunay_Rect_rect(self.as_raw_Subdiv2D(), rect);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Insert a single point into a Delaunay triangulation.
  /// 
  /// ## Parameters
  /// * pt: Point to insert.
  /// 
  /// The function inserts a single point into a subdivision and modifies the subdivision topology
  /// appropriately. If a point with the same coordinates exists already, no new point is added.
  /// @returns the ID of the point.
  /// 
  /// 
  /// Note: If the point is outside of the triangulation specified rect a runtime error is raised.
  pub fn insert(&mut self, pt: core::Point2f) -> Result<i32> {
  // identifier: cv_Subdiv2D_insert_Point2f_pt
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_insert_Point2f_pt(self.as_raw_Subdiv2D(), pt);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Insert multiple points into a Delaunay triangulation.
  /// 
  /// ## Parameters
  /// * ptvec: Points to insert.
  /// 
  /// The function inserts a vector of points into a subdivision and modifies the subdivision topology
  /// appropriately.
  pub fn insert_n(&mut self, ptvec: &types::VectorOfPoint2f) -> Result<()> {
  // identifier: cv_Subdiv2D_insert_VectorOfPoint2f_ptvec
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_insert_VectorOfPoint2f_ptvec(self.as_raw_Subdiv2D(), ptvec.as_raw_VectorOfPoint2f());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Returns the location of a point within a Delaunay triangulation.
  /// 
  /// ## Parameters
  /// * pt: Point to locate.
  /// * edge: Output edge that the point belongs to or is located to the right of it.
  /// * vertex: Optional output vertex the input point coincides with.
  /// 
  /// The function locates the input point within the subdivision and gives one of the triangle edges
  /// or vertices.
  /// 
  /// @returns an integer which specify one of the following five cases for point location:
  /// *  The point falls into some facet. The function returns #PTLOC_INSIDE and edge will contain one of
  /// edges of the facet.
  /// *  The point falls onto the edge. The function returns #PTLOC_ON_EDGE and edge will contain this edge.
  /// *  The point coincides with one of the subdivision vertices. The function returns #PTLOC_VERTEX and
  /// vertex will contain a pointer to the vertex.
  /// *  The point is outside the subdivision reference rectangle. The function returns #PTLOC_OUTSIDE_RECT
  /// and no pointers are filled.
  /// *  One of input arguments is invalid. A runtime error is raised or, if silent or "parent" error
  /// processing mode is selected, #PTLOC_ERROR is returned.
  pub fn locate(&mut self, pt: core::Point2f, edge: i32, vertex: i32) -> Result<i32> {
  // identifier: cv_Subdiv2D_locate_Point2f_pt_int_edge_int_vertex
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_locate_Point2f_pt_int_edge_int_vertex(self.as_raw_Subdiv2D(), pt, edge, vertex);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Returns a list of all edges.
  /// 
  /// ## Parameters
  /// * edgeList: Output vector.
  /// 
  /// The function gives each edge as a 4 numbers vector, where each two are one of the edge
  /// vertices. i.e. org_x = v[0], org_y = v[1], dst_x = v[2], dst_y = v[3].
  pub fn get_edge_list(&self, edge_list: &types::VectorOfVec4f) -> Result<()> {
  // identifier: cv_Subdiv2D_getEdgeList_VectorOfVec4f_edgeList
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_getEdgeList_VectorOfVec4f_edgeList(self.as_raw_Subdiv2D(), edge_list.as_raw_VectorOfVec4f());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Returns a list of the leading edge ID connected to each triangle.
  /// 
  /// ## Parameters
  /// * leadingEdgeList: Output vector.
  /// 
  /// The function gives one edge ID for each triangle.
  pub fn get_leading_edge_list(&self, leading_edge_list: &types::VectorOfint) -> Result<()> {
  // identifier: cv_Subdiv2D_getLeadingEdgeList_VectorOfint_leadingEdgeList
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_getLeadingEdgeList_VectorOfint_leadingEdgeList(self.as_raw_Subdiv2D(), leading_edge_list.as_raw_VectorOfint());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Returns a list of all triangles.
  /// 
  /// ## Parameters
  /// * triangleList: Output vector.
  /// 
  /// The function gives each triangle as a 6 numbers vector, where each two are one of the triangle
  /// vertices. i.e. p1_x = v[0], p1_y = v[1], p2_x = v[2], p2_y = v[3], p3_x = v[4], p3_y = v[5].
  pub fn get_triangle_list(&self, triangle_list: &types::VectorOfVec6f) -> Result<()> {
  // identifier: cv_Subdiv2D_getTriangleList_VectorOfVec6f_triangleList
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_getTriangleList_VectorOfVec6f_triangleList(self.as_raw_Subdiv2D(), triangle_list.as_raw_VectorOfVec6f());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Returns a list of all Voroni facets.
  /// 
  /// ## Parameters
  /// * idx: Vector of vertices IDs to consider. For all vertices you can pass empty vector.
  /// * facetList: Output vector of the Voroni facets.
  /// * facetCenters: Output vector of the Voroni facets center points.
  pub fn get_voronoi_facet_list(&mut self, idx: &types::VectorOfint, facet_list: &types::VectorOfVectorOfPoint2f, facet_centers: &types::VectorOfPoint2f) -> Result<()> {
  // identifier: cv_Subdiv2D_getVoronoiFacetList_VectorOfint_idx_VectorOfVectorOfPoint2f_facetList_VectorOfPoint2f_facetCenters
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_getVoronoiFacetList_VectorOfint_idx_VectorOfVectorOfPoint2f_facetList_VectorOfPoint2f_facetCenters(self.as_raw_Subdiv2D(), idx.as_raw_VectorOfint(), facet_list.as_raw_VectorOfVectorOfPoint2f(), facet_centers.as_raw_VectorOfPoint2f());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Returns one of the edges related to the given edge.
  /// 
  /// ## Parameters
  /// * edge: Subdivision edge ID.
  /// * nextEdgeType: Parameter specifying which of the related edges to return.
  /// The following values are possible:
  /// *   NEXT_AROUND_ORG next around the edge origin ( eOnext on the picture below if e is the input edge)
  /// *   NEXT_AROUND_DST next around the edge vertex ( eDnext )
  /// *   PREV_AROUND_ORG previous around the edge origin (reversed eRnext )
  /// *   PREV_AROUND_DST previous around the edge destination (reversed eLnext )
  /// *   NEXT_AROUND_LEFT next around the left facet ( eLnext )
  /// *   NEXT_AROUND_RIGHT next around the right facet ( eRnext )
  /// *   PREV_AROUND_LEFT previous around the left facet (reversed eOnext )
  /// *   PREV_AROUND_RIGHT previous around the right facet (reversed eDnext )
  /// 
  /// ![sample output](pics/quadedge.png)
  /// 
  /// @returns edge ID related to the input edge.
  pub fn get_edge(&self, edge: i32, next_edge_type: i32) -> Result<i32> {
  // identifier: cv_Subdiv2D_getEdge_int_edge_int_nextEdgeType
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_getEdge_int_edge_int_nextEdgeType(self.as_raw_Subdiv2D(), edge, next_edge_type);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Returns next edge around the edge origin.
  /// 
  /// ## Parameters
  /// * edge: Subdivision edge ID.
  /// 
  /// @returns an integer which is next edge ID around the edge origin: eOnext on the
  /// picture above if e is the input edge).
  pub fn next_edge(&self, edge: i32) -> Result<i32> {
  // identifier: cv_Subdiv2D_nextEdge_int_edge
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_nextEdge_int_edge(self.as_raw_Subdiv2D(), edge);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Returns another edge of the same quad-edge.
  /// 
  /// ## Parameters
  /// * edge: Subdivision edge ID.
  /// * rotate: Parameter specifying which of the edges of the same quad-edge as the input
  /// one to return. The following values are possible:
  /// *   0 - the input edge ( e on the picture below if e is the input edge)
  /// *   1 - the rotated edge ( eRot )
  /// *   2 - the reversed edge (reversed e (in green))
  /// *   3 - the reversed rotated edge (reversed eRot (in green))
  /// 
  /// @returns one of the edges ID of the same quad-edge as the input edge.
  pub fn rotate_edge(&self, edge: i32, rotate: i32) -> Result<i32> {
  // identifier: cv_Subdiv2D_rotateEdge_int_edge_int_rotate
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_rotateEdge_int_edge_int_rotate(self.as_raw_Subdiv2D(), edge, rotate);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn sym_edge(&self, edge: i32) -> Result<i32> {
  // identifier: cv_Subdiv2D_symEdge_int_edge
    unsafe {
      let rv = sys::cv_imgproc_cv_Subdiv2D_symEdge_int_edge(self.as_raw_Subdiv2D(), edge);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
pub const COLOR_BGR2YUV_IYUV: i32 = 0x80;
pub const COLOR_BGRA2RGB: i32 = 0x3;
pub const COLOR_BGRA2YUV_IYUV: i32 = 0x82;
pub const COLOR_BayerBG2RGB: i32 = 0x30;
pub const COLOR_BayerBG2RGBA: i32 = 0x8d;
pub const COLOR_BayerBG2RGB_EA: i32 = 0x89;
pub const COLOR_BayerBG2RGB_VNG: i32 = 0x40;
pub const COLOR_BayerGB2RGB: i32 = 0x31;
pub const COLOR_BayerGB2RGBA: i32 = 0x8e;
pub const COLOR_BayerGB2RGB_EA: i32 = 0x8a;
pub const COLOR_BayerGB2RGB_VNG: i32 = 0x41;
pub const COLOR_BayerGR2RGB: i32 = 0x2f;
pub const COLOR_BayerGR2RGBA: i32 = 0x8c;
pub const COLOR_BayerGR2RGB_EA: i32 = 0x88;
pub const COLOR_BayerGR2RGB_VNG: i32 = 0x3f;
pub const COLOR_BayerRG2RGB: i32 = 0x2e;
pub const COLOR_BayerRG2RGBA: i32 = 0x8b;
pub const COLOR_BayerRG2RGB_EA: i32 = 0x87;
pub const COLOR_BayerRG2RGB_VNG: i32 = 0x3e;
pub const COLOR_GRAY2RGB: i32 = 0x8;
pub const COLOR_GRAY2RGBA: i32 = 0x9;
pub const COLOR_RGB2BGR: i32 = 0x4;
pub const COLOR_RGB2BGRA: i32 = 0x2;
pub const COLOR_RGB2RGBA: i32 = 0x0;
pub const COLOR_RGB2YUV_IYUV: i32 = 0x7f;
pub const COLOR_RGBA2BGRA: i32 = 0x5;
pub const COLOR_RGBA2RGB: i32 = 0x1;
pub const COLOR_RGBA2YUV_IYUV: i32 = 0x81;
pub const COLOR_YUV2BGRA_I420: i32 = 0x69;
pub const COLOR_YUV2BGRA_UYNV: i32 = 0x70;
pub const COLOR_YUV2BGRA_Y422: i32 = 0x70;
pub const COLOR_YUV2BGRA_YUNV: i32 = 0x78;
pub const COLOR_YUV2BGRA_YUYV: i32 = 0x78;
pub const COLOR_YUV2BGR_I420: i32 = 0x65;
pub const COLOR_YUV2BGR_UYNV: i32 = 0x6c;
pub const COLOR_YUV2BGR_Y422: i32 = 0x6c;
pub const COLOR_YUV2BGR_YUNV: i32 = 0x74;
pub const COLOR_YUV2BGR_YUYV: i32 = 0x74;
pub const COLOR_YUV2GRAY_I420: i32 = 0x6a;
pub const COLOR_YUV2GRAY_IYUV: i32 = 0x6a;
pub const COLOR_YUV2GRAY_NV12: i32 = 0x6a;
pub const COLOR_YUV2GRAY_NV21: i32 = 0x6a;
pub const COLOR_YUV2GRAY_UYNV: i32 = 0x7b;
pub const COLOR_YUV2GRAY_Y422: i32 = 0x7b;
pub const COLOR_YUV2GRAY_YUNV: i32 = 0x7c;
pub const COLOR_YUV2GRAY_YUYV: i32 = 0x7c;
pub const COLOR_YUV2GRAY_YV12: i32 = 0x6a;
pub const COLOR_YUV2GRAY_YVYU: i32 = 0x7c;
pub const COLOR_YUV2RGBA_I420: i32 = 0x68;
pub const COLOR_YUV2RGBA_UYNV: i32 = 0x6f;
pub const COLOR_YUV2RGBA_Y422: i32 = 0x6f;
pub const COLOR_YUV2RGBA_YUNV: i32 = 0x77;
pub const COLOR_YUV2RGBA_YUYV: i32 = 0x77;
pub const COLOR_YUV2RGB_I420: i32 = 0x64;
pub const COLOR_YUV2RGB_UYNV: i32 = 0x6b;
pub const COLOR_YUV2RGB_Y422: i32 = 0x6b;
pub const COLOR_YUV2RGB_YUNV: i32 = 0x73;
pub const COLOR_YUV2RGB_YUYV: i32 = 0x73;
pub const COLOR_YUV420p2BGR: i32 = 0x63;
pub const COLOR_YUV420p2BGRA: i32 = 0x67;
pub const COLOR_YUV420p2GRAY: i32 = 0x6a;
pub const COLOR_YUV420p2RGB: i32 = 0x62;
pub const COLOR_YUV420p2RGBA: i32 = 0x66;
pub const COLOR_YUV420sp2BGR: i32 = 0x5d;
pub const COLOR_YUV420sp2BGRA: i32 = 0x61;
pub const COLOR_YUV420sp2GRAY: i32 = 0x6a;
pub const COLOR_YUV420sp2RGB: i32 = 0x5c;
pub const COLOR_YUV420sp2RGBA: i32 = 0x60;
pub const CV_BGR2YUV_IYUV: i32 = 0x80;
pub const CV_BGRA2RGB: i32 = 0x3;
pub const CV_BGRA2YUV_IYUV: i32 = 0x82;
pub const CV_BayerBG2RGB: i32 = 0x30;
pub const CV_BayerBG2RGBA: i32 = 0x8d;
pub const CV_BayerBG2RGB_EA: i32 = 0x89;
pub const CV_BayerBG2RGB_VNG: i32 = 0x40;
pub const CV_BayerGB2RGB: i32 = 0x31;
pub const CV_BayerGB2RGBA: i32 = 0x8e;
pub const CV_BayerGB2RGB_EA: i32 = 0x8a;
pub const CV_BayerGB2RGB_VNG: i32 = 0x41;
pub const CV_BayerGR2RGB: i32 = 0x2f;
pub const CV_BayerGR2RGBA: i32 = 0x8c;
pub const CV_BayerGR2RGB_EA: i32 = 0x88;
pub const CV_BayerGR2RGB_VNG: i32 = 0x3f;
pub const CV_BayerRG2RGB: i32 = 0x2e;
pub const CV_BayerRG2RGBA: i32 = 0x8b;
pub const CV_BayerRG2RGB_EA: i32 = 0x87;
pub const CV_BayerRG2RGB_VNG: i32 = 0x3e;
pub const CV_CANNY_L2_GRADIENT: i32 = 0x80000000;
pub const CV_COMP_HELLINGER: i32 = 0x3;
pub const CV_FLOODFILL_FIXED_RANGE: i32 = 0x10000;
pub const CV_FLOODFILL_MASK_ONLY: i32 = 0x20000;
pub const CV_FONT_VECTOR0: i32 = 0x0;
pub const CV_GRAY2RGB: i32 = 0x8;
pub const CV_GRAY2RGBA: i32 = 0x9;
pub const CV_RGB2BGR: i32 = 0x4;
pub const CV_RGB2BGRA: i32 = 0x2;
pub const CV_RGB2RGBA: i32 = 0x0;
pub const CV_RGB2YUV_IYUV: i32 = 0x7f;
pub const CV_RGBA2BGRA: i32 = 0x5;
pub const CV_RGBA2RGB: i32 = 0x1;
pub const CV_RGBA2YUV_IYUV: i32 = 0x81;
pub const CV_YUV2BGRA_I420: i32 = 0x69;
pub const CV_YUV2BGRA_UYNV: i32 = 0x70;
pub const CV_YUV2BGRA_Y422: i32 = 0x70;
pub const CV_YUV2BGRA_YUNV: i32 = 0x78;
pub const CV_YUV2BGRA_YUYV: i32 = 0x78;
pub const CV_YUV2BGR_I420: i32 = 0x65;
pub const CV_YUV2BGR_UYNV: i32 = 0x6c;
pub const CV_YUV2BGR_Y422: i32 = 0x6c;
pub const CV_YUV2BGR_YUNV: i32 = 0x74;
pub const CV_YUV2BGR_YUYV: i32 = 0x74;
pub const CV_YUV2GRAY_I420: i32 = 0x6a;
pub const CV_YUV2GRAY_IYUV: i32 = 0x6a;
pub const CV_YUV2GRAY_NV12: i32 = 0x6a;
pub const CV_YUV2GRAY_NV21: i32 = 0x6a;
pub const CV_YUV2GRAY_UYNV: i32 = 0x7b;
pub const CV_YUV2GRAY_Y422: i32 = 0x7b;
pub const CV_YUV2GRAY_YUNV: i32 = 0x7c;
pub const CV_YUV2GRAY_YUYV: i32 = 0x7c;
pub const CV_YUV2GRAY_YV12: i32 = 0x6a;
pub const CV_YUV2GRAY_YVYU: i32 = 0x7c;
pub const CV_YUV2RGBA_I420: i32 = 0x68;
pub const CV_YUV2RGBA_UYNV: i32 = 0x6f;
pub const CV_YUV2RGBA_Y422: i32 = 0x6f;
pub const CV_YUV2RGBA_YUNV: i32 = 0x77;
pub const CV_YUV2RGBA_YUYV: i32 = 0x77;
pub const CV_YUV2RGB_I420: i32 = 0x64;
pub const CV_YUV2RGB_UYNV: i32 = 0x6b;
pub const CV_YUV2RGB_Y422: i32 = 0x6b;
pub const CV_YUV2RGB_YUNV: i32 = 0x73;
pub const CV_YUV2RGB_YUYV: i32 = 0x73;
pub const CV_YUV420p2BGR: i32 = 0x63;
pub const CV_YUV420p2BGRA: i32 = 0x67;
pub const CV_YUV420p2GRAY: i32 = 0x6a;
pub const CV_YUV420p2RGB: i32 = 0x62;
pub const CV_YUV420p2RGBA: i32 = 0x66;
pub const CV_YUV420sp2BGR: i32 = 0x5d;
pub const CV_YUV420sp2BGRA: i32 = 0x61;
pub const CV_YUV420sp2GRAY: i32 = 0x6a;
pub const CV_YUV420sp2RGB: i32 = 0x5c;
pub const CV_YUV420sp2RGBA: i32 = 0x60;
pub const FLOODFILL_FIXED_RANGE: i32 = 0x10000;
pub const FLOODFILL_MASK_ONLY: i32 = 0x20000;
pub const HISTCMP_HELLINGER: i32 = 0x3;
pub const INTER_BITS2: i32 = 0xa;
pub const INTER_TAB_SIZE: i32 = 0x20;
pub const INTER_TAB_SIZE2: i32 = 0x400;

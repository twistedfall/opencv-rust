//! # Video I/O
//!
//! Read and write video or images sequence with OpenCV
//!
//! ### See also:
//! - [videoio_overview]
//! - Tutorials: [tutorial_table_of_content_videoio]
//!   # Flags for video I/O
//!   # Additional flags for video I/O API backends
//!   # C API for video I/O
//!   # iOS glue for video I/O
//!   # WinRT glue for video I/O
//!   # Query I/O API backends registry
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{VideoCaptureTrait, VideoCaptureTraitConst, VideoWriterTrait, VideoWriterTraitConst};
}

/// Android - not used
// CAP_ANDROID /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:105
pub const CAP_ANDROID: i32 = 1000;
/// Auto detect == 0
// CAP_ANY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:90
pub const CAP_ANY: i32 = 0;
/// Aravis SDK
// CAP_ARAVIS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:118
pub const CAP_ARAVIS: i32 = 2100;
/// AVFoundation framework for iOS (OS X Lion will have the same API)
// CAP_AVFOUNDATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:107
pub const CAP_AVFOUNDATION: i32 = 1200;
/// Same as CAP_FIREWIRE
// CAP_CMU1394 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:98
pub const CAP_CMU1394: i32 = 300;
/// Same as CAP_FIREWIRE
// CAP_DC1394 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:97
pub const CAP_DC1394: i32 = 300;
/// DirectShow (via videoInput)
// CAP_DSHOW /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:101
pub const CAP_DSHOW: i32 = 700;
/// Open and record video file or stream using the FFMPEG library
// CAP_FFMPEG /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:116
pub const CAP_FFMPEG: i32 = 1900;
/// Same as CAP_FIREWIRE
// CAP_FIREWARE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:95
pub const CAP_FIREWARE: i32 = 300;
/// IEEE 1394 drivers
// CAP_FIREWIRE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:94
pub const CAP_FIREWIRE: i32 = 300;
/// Smartek Giganetix GigEVisionSDK
// CAP_GIGANETIX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:108
pub const CAP_GIGANETIX: i32 = 1300;
/// gPhoto2 connection
// CAP_GPHOTO2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:114
pub const CAP_GPHOTO2: i32 = 1700;
/// GStreamer
// CAP_GSTREAMER /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:115
pub const CAP_GSTREAMER: i32 = 1800;
/// Same as CAP_FIREWIRE
// CAP_IEEE1394 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:96
pub const CAP_IEEE1394: i32 = 300;
/// OpenCV Image Sequence (e.g. img_%02d.jpg)
// CAP_IMAGES /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:117
pub const CAP_IMAGES: i32 = 2000;
/// Intel Perceptual Computing SDK
// CAP_INTELPERC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:111
pub const CAP_INTELPERC: i32 = 1500;
// CAP_INTELPERC_DEPTH_GENERATOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:548
pub const CAP_INTELPERC_DEPTH_GENERATOR: i32 = 536870912;
/// Each pixel is a 16-bit integer. The value indicates the distance from an object to the camera's XY plane or the Cartesian depth.
// CAP_INTELPERC_DEPTH_MAP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:553
pub const CAP_INTELPERC_DEPTH_MAP: i32 = 0;
// CAP_INTELPERC_GENERATORS_MASK /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:550
pub const CAP_INTELPERC_GENERATORS_MASK: i32 = 805306368;
// CAP_INTELPERC_IMAGE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:556
pub const CAP_INTELPERC_IMAGE: i32 = 3;
// CAP_INTELPERC_IMAGE_GENERATOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:549
pub const CAP_INTELPERC_IMAGE_GENERATOR: i32 = 268435456;
/// Each pixel is a 16-bit integer. The value indicates the intensity of the reflected laser beam.
// CAP_INTELPERC_IR_MAP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:555
pub const CAP_INTELPERC_IR_MAP: i32 = 2;
/// Each pixel contains two 32-bit floating point values in the range of 0-1, representing the mapping of depth coordinates to the color coordinates.
// CAP_INTELPERC_UVDEPTH_MAP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:554
pub const CAP_INTELPERC_UVDEPTH_MAP: i32 = 1;
/// Intel MediaSDK
// CAP_INTEL_MFX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:120
pub const CAP_INTEL_MFX: i32 = 2300;
/// BGR24 (default)
// CAP_MODE_BGR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:194
pub const CAP_MODE_BGR: i32 = 0;
/// Y8
// CAP_MODE_GRAY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:196
pub const CAP_MODE_GRAY: i32 = 2;
/// RGB24
// CAP_MODE_RGB /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:195
pub const CAP_MODE_RGB: i32 = 1;
/// YUYV
// CAP_MODE_YUYV /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:197
pub const CAP_MODE_YUYV: i32 = 3;
/// Microsoft Media Foundation (via videoInput)
// CAP_MSMF /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:109
pub const CAP_MSMF: i32 = 1400;
/// Built-in OpenCV MotionJPEG codec
// CAP_OPENCV_MJPEG /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:119
pub const CAP_OPENCV_MJPEG: i32 = 2200;
/// OpenNI (for Kinect)
// CAP_OPENNI /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:103
pub const CAP_OPENNI: i32 = 900;
/// OpenNI2 (for Kinect)
// CAP_OPENNI2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:112
pub const CAP_OPENNI2: i32 = 1600;
/// OpenNI2 (for Asus Xtion and Occipital Structure sensors)
// CAP_OPENNI2_ASUS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:113
pub const CAP_OPENNI2_ASUS: i32 = 1610;
/// OpenNI (for Asus Xtion)
// CAP_OPENNI_ASUS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:104
pub const CAP_OPENNI_ASUS: i32 = 910;
/// Data given from RGB image generator
// CAP_OPENNI_BGR_IMAGE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:278
pub const CAP_OPENNI_BGR_IMAGE: i32 = 5;
// CAP_OPENNI_DEPTH_GENERATOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:236
pub const CAP_OPENNI_DEPTH_GENERATOR: i32 = -2147483648;
// CAP_OPENNI_DEPTH_GENERATOR_BASELINE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:264
pub const CAP_OPENNI_DEPTH_GENERATOR_BASELINE: i32 = -2147483546;
// CAP_OPENNI_DEPTH_GENERATOR_FOCAL_LENGTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:265
pub const CAP_OPENNI_DEPTH_GENERATOR_FOCAL_LENGTH: i32 = -2147483545;
// CAP_OPENNI_DEPTH_GENERATOR_PRESENT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:263
pub const CAP_OPENNI_DEPTH_GENERATOR_PRESENT: i32 = -2147483539;
// CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:266
pub const CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION: i32 = -2147483544;
// CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION_ON /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:267
pub const CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION_ON: i32 = -2147483544;
/// Depth values in mm (CV_16UC1)
// CAP_OPENNI_DEPTH_MAP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:272
pub const CAP_OPENNI_DEPTH_MAP: i32 = 0;
/// Disparity in pixels (CV_8UC1)
// CAP_OPENNI_DISPARITY_MAP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:274
pub const CAP_OPENNI_DISPARITY_MAP: i32 = 2;
/// Disparity in pixels (CV_32FC1)
// CAP_OPENNI_DISPARITY_MAP_32F /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:275
pub const CAP_OPENNI_DISPARITY_MAP_32F: i32 = 3;
// CAP_OPENNI_GENERATORS_MASK /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:239
pub const CAP_OPENNI_GENERATORS_MASK: i32 = -536870912;
/// Data given from RGB image generator
// CAP_OPENNI_GRAY_IMAGE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:279
pub const CAP_OPENNI_GRAY_IMAGE: i32 = 6;
// CAP_OPENNI_IMAGE_GENERATOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:237
pub const CAP_OPENNI_IMAGE_GENERATOR: i32 = 1073741824;
// CAP_OPENNI_IMAGE_GENERATOR_OUTPUT_MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:262
pub const CAP_OPENNI_IMAGE_GENERATOR_OUTPUT_MODE: i32 = 1073741924;
// CAP_OPENNI_IMAGE_GENERATOR_PRESENT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:261
pub const CAP_OPENNI_IMAGE_GENERATOR_PRESENT: i32 = 1073741933;
// CAP_OPENNI_IR_GENERATOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:238
pub const CAP_OPENNI_IR_GENERATOR: i32 = 536870912;
// CAP_OPENNI_IR_GENERATOR_PRESENT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:268
pub const CAP_OPENNI_IR_GENERATOR_PRESENT: i32 = 536871021;
/// Data given from IR image generator
// CAP_OPENNI_IR_IMAGE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:281
pub const CAP_OPENNI_IR_IMAGE: i32 = 7;
/// XYZ in meters (CV_32FC3)
// CAP_OPENNI_POINT_CLOUD_MAP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:273
pub const CAP_OPENNI_POINT_CLOUD_MAP: i32 = 1;
// CAP_OPENNI_QVGA_30HZ /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:288
pub const CAP_OPENNI_QVGA_30HZ: i32 = 3;
// CAP_OPENNI_QVGA_60HZ /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:289
pub const CAP_OPENNI_QVGA_60HZ: i32 = 4;
// CAP_OPENNI_SXGA_15HZ /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:286
pub const CAP_OPENNI_SXGA_15HZ: i32 = 1;
// CAP_OPENNI_SXGA_30HZ /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:287
pub const CAP_OPENNI_SXGA_30HZ: i32 = 2;
/// CV_8UC1
// CAP_OPENNI_VALID_DEPTH_MASK /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:276
pub const CAP_OPENNI_VALID_DEPTH_MASK: i32 = 4;
// CAP_OPENNI_VGA_30HZ /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:285
pub const CAP_OPENNI_VGA_30HZ: i32 = 0;
/// Aperture. Can be readonly, depends on camera program.
// CAP_PROP_APERTURE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:578
pub const CAP_PROP_APERTURE: i32 = 17008;
// CAP_PROP_AUTOFOCUS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:171
pub const CAP_PROP_AUTOFOCUS: i32 = 39;
/// DC1394: exposure control done by camera, user can adjust reference level using this feature.
// CAP_PROP_AUTO_EXPOSURE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:154
pub const CAP_PROP_AUTO_EXPOSURE: i32 = 21;
/// enable/ disable auto white-balance
// CAP_PROP_AUTO_WB /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:176
pub const CAP_PROP_AUTO_WB: i32 = 44;
/// Current backend (enum VideoCaptureAPIs). Read-only property
// CAP_PROP_BACKEND /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:174
pub const CAP_PROP_BACKEND: i32 = 42;
// CAP_PROP_BACKLIGHT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:164
pub const CAP_PROP_BACKLIGHT: i32 = 32;
/// (read-only) Video bitrate in kbits/s
// CAP_PROP_BITRATE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:179
pub const CAP_PROP_BITRATE: i32 = 47;
/// Brightness of the image (only for those cameras that support).
// CAP_PROP_BRIGHTNESS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:142
pub const CAP_PROP_BRIGHTNESS: i32 = 10;
// CAP_PROP_BUFFERSIZE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:170
pub const CAP_PROP_BUFFERSIZE: i32 = 38;
/// Video input or Channel Number (only for those cameras that support)
// CAP_PROP_CHANNEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:175
pub const CAP_PROP_CHANNEL: i32 = 43;
/// (read-only) codec's pixel format. 4-character code - see VideoWriter::fourcc . Subset of [AV_PIX_FMT_*](https://github.com/FFmpeg/FFmpeg/blob/master/libavcodec/raw.c) or -1 if unknown
// CAP_PROP_CODEC_PIXEL_FORMAT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:178
pub const CAP_PROP_CODEC_PIXEL_FORMAT: i32 = 46;
/// Contrast of the image (only for cameras).
// CAP_PROP_CONTRAST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:143
pub const CAP_PROP_CONTRAST: i32 = 11;
/// Boolean flags indicating whether images should be converted to RGB. <br/>
/// *GStreamer note*: The flag is ignored in case if custom pipeline is used. It's user responsibility to interpret pipeline output.
// CAP_PROP_CONVERT_RGB /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:148
pub const CAP_PROP_CONVERT_RGB: i32 = 16;
// CAP_PROP_DC1394_MAX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:226
pub const CAP_PROP_DC1394_MAX: i32 = 31;
// CAP_PROP_DC1394_MODE_AUTO /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:224
pub const CAP_PROP_DC1394_MODE_AUTO: i32 = -2;
/// set automatically when a value of the feature is set by the user.
// CAP_PROP_DC1394_MODE_MANUAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:223
pub const CAP_PROP_DC1394_MODE_MANUAL: i32 = -3;
// CAP_PROP_DC1394_MODE_ONE_PUSH_AUTO /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:225
pub const CAP_PROP_DC1394_MODE_ONE_PUSH_AUTO: i32 = -1;
/// turn the feature off (not controlled manually nor automatically).
// CAP_PROP_DC1394_OFF /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:222
pub const CAP_PROP_DC1394_OFF: i32 = -4;
/// Exposure (only for those cameras that support).
// CAP_PROP_EXPOSURE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:147
pub const CAP_PROP_EXPOSURE: i32 = 15;
/// Camera exposure program.
// CAP_PROP_EXPOSUREPROGRAM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:579
pub const CAP_PROP_EXPOSUREPROGRAM: i32 = 17009;
// CAP_PROP_FOCUS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:161
pub const CAP_PROP_FOCUS: i32 = 28;
/// Format of the %Mat objects (see Mat::type()) returned by VideoCapture::retrieve().
/// Set value -1 to fetch undecoded RAW video streams (as Mat 8UC1).
// CAP_PROP_FORMAT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:139
pub const CAP_PROP_FORMAT: i32 = 8;
/// 4-character code of codec. see VideoWriter::fourcc .
// CAP_PROP_FOURCC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:137
pub const CAP_PROP_FOURCC: i32 = 6;
/// Frame rate.
// CAP_PROP_FPS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:136
pub const CAP_PROP_FPS: i32 = 5;
/// Number of frames in the video file.
// CAP_PROP_FRAME_COUNT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:138
pub const CAP_PROP_FRAME_COUNT: i32 = 7;
/// Height of the frames in the video stream.
// CAP_PROP_FRAME_HEIGHT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:135
pub const CAP_PROP_FRAME_HEIGHT: i32 = 4;
/// Width of the frames in the video stream.
// CAP_PROP_FRAME_WIDTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:134
pub const CAP_PROP_FRAME_WIDTH: i32 = 3;
/// Gain of the image (only for those cameras that support).
// CAP_PROP_GAIN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:146
pub const CAP_PROP_GAIN: i32 = 14;
// CAP_PROP_GAMMA /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:155
pub const CAP_PROP_GAMMA: i32 = 22;
// CAP_PROP_GIGA_FRAME_HEIGH_MAX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:528
pub const CAP_PROP_GIGA_FRAME_HEIGH_MAX: i32 = 10004;
// CAP_PROP_GIGA_FRAME_OFFSET_X /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:525
pub const CAP_PROP_GIGA_FRAME_OFFSET_X: i32 = 10001;
// CAP_PROP_GIGA_FRAME_OFFSET_Y /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:526
pub const CAP_PROP_GIGA_FRAME_OFFSET_Y: i32 = 10002;
// CAP_PROP_GIGA_FRAME_SENS_HEIGH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:530
pub const CAP_PROP_GIGA_FRAME_SENS_HEIGH: i32 = 10006;
// CAP_PROP_GIGA_FRAME_SENS_WIDTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:529
pub const CAP_PROP_GIGA_FRAME_SENS_WIDTH: i32 = 10005;
// CAP_PROP_GIGA_FRAME_WIDTH_MAX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:527
pub const CAP_PROP_GIGA_FRAME_WIDTH_MAX: i32 = 10003;
/// Collect messages with details.
// CAP_PROP_GPHOTO2_COLLECT_MSGS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:575
pub const CAP_PROP_GPHOTO2_COLLECT_MSGS: i32 = 17005;
/// Readonly, returns (const char *).
// CAP_PROP_GPHOTO2_FLUSH_MSGS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:576
pub const CAP_PROP_GPHOTO2_FLUSH_MSGS: i32 = 17006;
/// Capture only preview from liveview mode.
// CAP_PROP_GPHOTO2_PREVIEW /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:571
pub const CAP_PROP_GPHOTO2_PREVIEW: i32 = 17001;
/// Trigger, only by set. Reload camera settings.
// CAP_PROP_GPHOTO2_RELOAD_CONFIG /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:573
pub const CAP_PROP_GPHOTO2_RELOAD_CONFIG: i32 = 17003;
/// Reload all settings on set.
// CAP_PROP_GPHOTO2_RELOAD_ON_CHANGE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:574
pub const CAP_PROP_GPHOTO2_RELOAD_ON_CHANGE: i32 = 17004;
/// Readonly, returns (const char *).
// CAP_PROP_GPHOTO2_WIDGET_ENUMERATE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:572
pub const CAP_PROP_GPHOTO2_WIDGET_ENUMERATE: i32 = 17002;
/// Default is 1
// CAP_PROP_GSTREAMER_QUEUE_LENGTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:298
pub const CAP_PROP_GSTREAMER_QUEUE_LENGTH: i32 = 200;
// CAP_PROP_GUID /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:162
pub const CAP_PROP_GUID: i32 = 29;
/// Hue of the image (only for cameras).
// CAP_PROP_HUE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:145
pub const CAP_PROP_HUE: i32 = 13;
// CAP_PROP_IMAGES_BASE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:593
pub const CAP_PROP_IMAGES_BASE: i32 = 18000;
// CAP_PROP_IMAGES_LAST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:594
pub const CAP_PROP_IMAGES_LAST: i32 = 19000;
// CAP_PROP_INTELPERC_DEPTH_CONFIDENCE_THRESHOLD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:542
pub const CAP_PROP_INTELPERC_DEPTH_CONFIDENCE_THRESHOLD: i32 = 11005;
// CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_HORZ /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:543
pub const CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_HORZ: i32 = 11006;
// CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_VERT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:544
pub const CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_VERT: i32 = 11007;
// CAP_PROP_INTELPERC_DEPTH_LOW_CONFIDENCE_VALUE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:540
pub const CAP_PROP_INTELPERC_DEPTH_LOW_CONFIDENCE_VALUE: i32 = 11003;
// CAP_PROP_INTELPERC_DEPTH_SATURATION_VALUE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:541
pub const CAP_PROP_INTELPERC_DEPTH_SATURATION_VALUE: i32 = 11004;
// CAP_PROP_INTELPERC_PROFILE_COUNT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:538
pub const CAP_PROP_INTELPERC_PROFILE_COUNT: i32 = 11001;
// CAP_PROP_INTELPERC_PROFILE_IDX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:539
pub const CAP_PROP_INTELPERC_PROFILE_IDX: i32 = 11002;
// CAP_PROP_IOS_DEVICE_EXPOSURE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:510
pub const CAP_PROP_IOS_DEVICE_EXPOSURE: i32 = 9002;
// CAP_PROP_IOS_DEVICE_FLASH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:511
pub const CAP_PROP_IOS_DEVICE_FLASH: i32 = 9003;
// CAP_PROP_IOS_DEVICE_FOCUS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:509
pub const CAP_PROP_IOS_DEVICE_FOCUS: i32 = 9001;
// CAP_PROP_IOS_DEVICE_TORCH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:513
pub const CAP_PROP_IOS_DEVICE_TORCH: i32 = 9005;
// CAP_PROP_IOS_DEVICE_WHITEBALANCE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:512
pub const CAP_PROP_IOS_DEVICE_WHITEBALANCE: i32 = 9004;
// CAP_PROP_IRIS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:168
pub const CAP_PROP_IRIS: i32 = 36;
// CAP_PROP_ISO_SPEED /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:163
pub const CAP_PROP_ISO_SPEED: i32 = 30;
/// Backend-specific value indicating the current capture mode.
// CAP_PROP_MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:141
pub const CAP_PROP_MODE: i32 = 9;
// CAP_PROP_MONOCHROME /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:152
pub const CAP_PROP_MONOCHROME: i32 = 19;
// CAP_PROP_OPENNI2_MIRROR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:257
pub const CAP_PROP_OPENNI2_MIRROR: i32 = 111;
// CAP_PROP_OPENNI2_SYNC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:256
pub const CAP_PROP_OPENNI2_SYNC: i32 = 110;
// CAP_PROP_OPENNI_APPROX_FRAME_SYNC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:251
pub const CAP_PROP_OPENNI_APPROX_FRAME_SYNC: i32 = 105;
/// In mm
// CAP_PROP_OPENNI_BASELINE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:245
pub const CAP_PROP_OPENNI_BASELINE: i32 = 102;
// CAP_PROP_OPENNI_CIRCLE_BUFFER /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:253
pub const CAP_PROP_OPENNI_CIRCLE_BUFFER: i32 = 107;
/// In pixels
// CAP_PROP_OPENNI_FOCAL_LENGTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:246
pub const CAP_PROP_OPENNI_FOCAL_LENGTH: i32 = 103;
/// In mm
// CAP_PROP_OPENNI_FRAME_MAX_DEPTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:244
pub const CAP_PROP_OPENNI_FRAME_MAX_DEPTH: i32 = 101;
// CAP_PROP_OPENNI_GENERATOR_PRESENT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:255
pub const CAP_PROP_OPENNI_GENERATOR_PRESENT: i32 = 109;
// CAP_PROP_OPENNI_MAX_BUFFER_SIZE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:252
pub const CAP_PROP_OPENNI_MAX_BUFFER_SIZE: i32 = 106;
// CAP_PROP_OPENNI_MAX_TIME_DURATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:254
pub const CAP_PROP_OPENNI_MAX_TIME_DURATION: i32 = 108;
// CAP_PROP_OPENNI_OUTPUT_MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:243
pub const CAP_PROP_OPENNI_OUTPUT_MODE: i32 = 100;
/// Flag that synchronizes the remapping depth map to image map
/// by changing depth generator's view point (if the flag is "on") or
/// sets this view point to its normal one (if the flag is "off").
// CAP_PROP_OPENNI_REGISTRATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:247
pub const CAP_PROP_OPENNI_REGISTRATION: i32 = 104;
// CAP_PROP_OPENNI_REGISTRATION_ON /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:250
pub const CAP_PROP_OPENNI_REGISTRATION_ON: i32 = 104;
// CAP_PROP_OPEN_TIMEOUT_MSEC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:182
pub const CAP_PROP_OPEN_TIMEOUT_MSEC: i32 = 53;
/// if true - rotates output frames of CvCapture considering video file's metadata  (applicable for FFmpeg back-end only) (<https://github.com/opencv/opencv/issues/15499>)
// CAP_PROP_ORIENTATION_AUTO /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:181
pub const CAP_PROP_ORIENTATION_AUTO: i32 = 49;
/// (read-only) Frame rotation defined by stream meta (applicable for FFmpeg back-end only)
// CAP_PROP_ORIENTATION_META /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:180
pub const CAP_PROP_ORIENTATION_META: i32 = 48;
// CAP_PROP_PAN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:165
pub const CAP_PROP_PAN: i32 = 33;
/// Relative position of the video file: 0=start of the film, 1=end of the film.
// CAP_PROP_POS_AVI_RATIO /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:133
pub const CAP_PROP_POS_AVI_RATIO: i32 = 2;
/// 0-based index of the frame to be decoded/captured next.
// CAP_PROP_POS_FRAMES /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:132
pub const CAP_PROP_POS_FRAMES: i32 = 1;
/// Current position of the video file in milliseconds.
// CAP_PROP_POS_MSEC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:131
pub const CAP_PROP_POS_MSEC: i32 = 0;
/// Horizontal binning factor.
// CAP_PROP_PVAPI_BINNINGX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:312
pub const CAP_PROP_PVAPI_BINNINGX: i32 = 304;
/// Vertical binning factor.
// CAP_PROP_PVAPI_BINNINGY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:313
pub const CAP_PROP_PVAPI_BINNINGY: i32 = 305;
/// Horizontal sub-sampling of the image.
// CAP_PROP_PVAPI_DECIMATIONHORIZONTAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:310
pub const CAP_PROP_PVAPI_DECIMATIONHORIZONTAL: i32 = 302;
/// Vertical sub-sampling of the image.
// CAP_PROP_PVAPI_DECIMATIONVERTICAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:311
pub const CAP_PROP_PVAPI_DECIMATIONVERTICAL: i32 = 303;
/// FrameStartTriggerMode: Determines how a frame is initiated.
// CAP_PROP_PVAPI_FRAMESTARTTRIGGERMODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:309
pub const CAP_PROP_PVAPI_FRAMESTARTTRIGGERMODE: i32 = 301;
/// IP for enable multicast master mode. 0 for disable multicast.
// CAP_PROP_PVAPI_MULTICASTIP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:308
pub const CAP_PROP_PVAPI_MULTICASTIP: i32 = 300;
/// Pixel format.
// CAP_PROP_PVAPI_PIXELFORMAT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:314
pub const CAP_PROP_PVAPI_PIXELFORMAT: i32 = 306;
// CAP_PROP_READ_TIMEOUT_MSEC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:183
pub const CAP_PROP_READ_TIMEOUT_MSEC: i32 = 54;
/// Rectification flag for stereo cameras (note: only supported by DC1394 v 2.x backend currently).
// CAP_PROP_RECTIFICATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:151
pub const CAP_PROP_RECTIFICATION: i32 = 18;
// CAP_PROP_ROLL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:167
pub const CAP_PROP_ROLL: i32 = 35;
/// Sample aspect ratio: num/den (den)
// CAP_PROP_SAR_DEN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:173
pub const CAP_PROP_SAR_DEN: i32 = 41;
/// Sample aspect ratio: num/den (num)
// CAP_PROP_SAR_NUM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:172
pub const CAP_PROP_SAR_NUM: i32 = 40;
/// Saturation of the image (only for cameras).
// CAP_PROP_SATURATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:144
pub const CAP_PROP_SATURATION: i32 = 12;
/// Pop up video/camera filter dialog (note: only supported by DSHOW backend currently. The property value is ignored)
// CAP_PROP_SETTINGS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:169
pub const CAP_PROP_SETTINGS: i32 = 37;
// CAP_PROP_SHARPNESS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:153
pub const CAP_PROP_SHARPNESS: i32 = 20;
/// Exposure speed. Can be readonly, depends on camera program.
// CAP_PROP_SPEED /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:577
pub const CAP_PROP_SPEED: i32 = 17007;
// CAP_PROP_TEMPERATURE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:156
pub const CAP_PROP_TEMPERATURE: i32 = 23;
// CAP_PROP_TILT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:166
pub const CAP_PROP_TILT: i32 = 34;
// CAP_PROP_TRIGGER /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:157
pub const CAP_PROP_TRIGGER: i32 = 24;
// CAP_PROP_TRIGGER_DELAY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:158
pub const CAP_PROP_TRIGGER_DELAY: i32 = 25;
/// Enter liveview mode.
// CAP_PROP_VIEWFINDER /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:580
pub const CAP_PROP_VIEWFINDER: i32 = 17010;
/// white-balance color temperature
// CAP_PROP_WB_TEMPERATURE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:177
pub const CAP_PROP_WB_TEMPERATURE: i32 = 45;
/// Currently unsupported.
// CAP_PROP_WHITE_BALANCE_BLUE_U /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:150
pub const CAP_PROP_WHITE_BALANCE_BLUE_U: i32 = 17;
// CAP_PROP_WHITE_BALANCE_RED_V /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:159
pub const CAP_PROP_WHITE_BALANCE_RED_V: i32 = 26;
/// Acquisition buffer size in buffer_size_unit. Default bytes.
// CAP_PROP_XI_ACQ_BUFFER_SIZE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:473
pub const CAP_PROP_XI_ACQ_BUFFER_SIZE: i32 = 548;
/// Acquisition buffer size unit in bytes. Default 1. E.g. Value 1024 means that buffer_size is in KiBytes.
// CAP_PROP_XI_ACQ_BUFFER_SIZE_UNIT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:474
pub const CAP_PROP_XI_ACQ_BUFFER_SIZE_UNIT: i32 = 549;
/// Sets number of frames acquired by burst. This burst is used only if trigger is set to FrameBurstStart.
// CAP_PROP_XI_ACQ_FRAME_BURST_COUNT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:439
pub const CAP_PROP_XI_ACQ_FRAME_BURST_COUNT: i32 = 499;
/// Type of sensor frames timing.
// CAP_PROP_XI_ACQ_TIMING_MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:463
pub const CAP_PROP_XI_ACQ_TIMING_MODE: i32 = 538;
/// Number of buffers to commit to low level.
// CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_COMMIT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:477
pub const CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_COMMIT: i32 = 552;
/// Acquisition transport buffer size in bytes.
// CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_SIZE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:475
pub const CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_SIZE: i32 = 550;
/// Automatic exposure/gain.
// CAP_PROP_XI_AEAG /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:365
pub const CAP_PROP_XI_AEAG: i32 = 415;
/// Average intensity of output signal AEAG should achieve(in %).
// CAP_PROP_XI_AEAG_LEVEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:369
pub const CAP_PROP_XI_AEAG_LEVEL: i32 = 419;
/// Automatic exposure/gain ROI Height.
// CAP_PROP_XI_AEAG_ROI_HEIGHT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:392
pub const CAP_PROP_XI_AEAG_ROI_HEIGHT: i32 = 442;
/// Automatic exposure/gain ROI offset X.
// CAP_PROP_XI_AEAG_ROI_OFFSET_X /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:389
pub const CAP_PROP_XI_AEAG_ROI_OFFSET_X: i32 = 439;
/// Automatic exposure/gain ROI offset Y.
// CAP_PROP_XI_AEAG_ROI_OFFSET_Y /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:390
pub const CAP_PROP_XI_AEAG_ROI_OFFSET_Y: i32 = 440;
/// Automatic exposure/gain ROI Width.
// CAP_PROP_XI_AEAG_ROI_WIDTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:391
pub const CAP_PROP_XI_AEAG_ROI_WIDTH: i32 = 441;
/// Maximum limit of exposure in AEAG procedure.
// CAP_PROP_XI_AE_MAX_LIMIT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:367
pub const CAP_PROP_XI_AE_MAX_LIMIT: i32 = 417;
/// Maximum limit of gain in AEAG procedure.
// CAP_PROP_XI_AG_MAX_LIMIT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:368
pub const CAP_PROP_XI_AG_MAX_LIMIT: i32 = 418;
/// Enable applying of CMS profiles to xiGetImage (see XI_PRM_INPUT_CMS_PROFILE, XI_PRM_OUTPUT_CMS_PROFILE).
// CAP_PROP_XI_APPLY_CMS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:415
pub const CAP_PROP_XI_APPLY_CMS: i32 = 471;
/// Automatic bandwidth calculation.
// CAP_PROP_XI_AUTO_BANDWIDTH_CALCULATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:492
pub const CAP_PROP_XI_AUTO_BANDWIDTH_CALCULATION: i32 = 573;
/// Automatic white balance.
// CAP_PROP_XI_AUTO_WB /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:364
pub const CAP_PROP_XI_AUTO_WB: i32 = 414;
/// Calculate and returns available interface bandwidth(int Megabits).
// CAP_PROP_XI_AVAILABLE_BANDWIDTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:464
pub const CAP_PROP_XI_AVAILABLE_BANDWIDTH: i32 = 539;
/// Horizontal Binning - number of horizontal photo-sensitive cells to combine together.
// CAP_PROP_XI_BINNING_HORIZONTAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:378
pub const CAP_PROP_XI_BINNING_HORIZONTAL: i32 = 429;
/// Binning pattern type.
// CAP_PROP_XI_BINNING_PATTERN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:379
pub const CAP_PROP_XI_BINNING_PATTERN: i32 = 430;
/// Binning engine selector.
// CAP_PROP_XI_BINNING_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:376
pub const CAP_PROP_XI_BINNING_SELECTOR: i32 = 427;
/// Vertical Binning - number of vertical photo-sensitive cells to combine together.
// CAP_PROP_XI_BINNING_VERTICAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:377
pub const CAP_PROP_XI_BINNING_VERTICAL: i32 = 428;
/// Correction of bad pixels.
// CAP_PROP_XI_BPC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:393
pub const CAP_PROP_XI_BPC: i32 = 445;
/// Queue of field/frame buffers.
// CAP_PROP_XI_BUFFERS_QUEUE_SIZE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:476
pub const CAP_PROP_XI_BUFFERS_QUEUE_SIZE: i32 = 551;
/// Data move policy.
// CAP_PROP_XI_BUFFER_POLICY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:465
pub const CAP_PROP_XI_BUFFER_POLICY: i32 = 540;
/// Color Correction Matrix element [0][0].
// CAP_PROP_XI_CC_MATRIX_00 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:421
pub const CAP_PROP_XI_CC_MATRIX_00: i32 = 479;
/// Color Correction Matrix element [0][1].
// CAP_PROP_XI_CC_MATRIX_01 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:422
pub const CAP_PROP_XI_CC_MATRIX_01: i32 = 480;
/// Color Correction Matrix element [0][2].
// CAP_PROP_XI_CC_MATRIX_02 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:423
pub const CAP_PROP_XI_CC_MATRIX_02: i32 = 481;
/// Color Correction Matrix element [0][3].
// CAP_PROP_XI_CC_MATRIX_03 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:424
pub const CAP_PROP_XI_CC_MATRIX_03: i32 = 482;
/// Color Correction Matrix element [1][0].
// CAP_PROP_XI_CC_MATRIX_10 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:425
pub const CAP_PROP_XI_CC_MATRIX_10: i32 = 483;
/// Color Correction Matrix element [1][1].
// CAP_PROP_XI_CC_MATRIX_11 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:426
pub const CAP_PROP_XI_CC_MATRIX_11: i32 = 484;
/// Color Correction Matrix element [1][2].
// CAP_PROP_XI_CC_MATRIX_12 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:427
pub const CAP_PROP_XI_CC_MATRIX_12: i32 = 485;
/// Color Correction Matrix element [1][3].
// CAP_PROP_XI_CC_MATRIX_13 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:428
pub const CAP_PROP_XI_CC_MATRIX_13: i32 = 486;
/// Color Correction Matrix element [2][0].
// CAP_PROP_XI_CC_MATRIX_20 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:429
pub const CAP_PROP_XI_CC_MATRIX_20: i32 = 487;
/// Color Correction Matrix element [2][1].
// CAP_PROP_XI_CC_MATRIX_21 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:430
pub const CAP_PROP_XI_CC_MATRIX_21: i32 = 488;
/// Color Correction Matrix element [2][2].
// CAP_PROP_XI_CC_MATRIX_22 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:431
pub const CAP_PROP_XI_CC_MATRIX_22: i32 = 489;
/// Color Correction Matrix element [2][3].
// CAP_PROP_XI_CC_MATRIX_23 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:432
pub const CAP_PROP_XI_CC_MATRIX_23: i32 = 490;
/// Color Correction Matrix element [3][0].
// CAP_PROP_XI_CC_MATRIX_30 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:433
pub const CAP_PROP_XI_CC_MATRIX_30: i32 = 491;
/// Color Correction Matrix element [3][1].
// CAP_PROP_XI_CC_MATRIX_31 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:434
pub const CAP_PROP_XI_CC_MATRIX_31: i32 = 492;
/// Color Correction Matrix element [3][2].
// CAP_PROP_XI_CC_MATRIX_32 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:435
pub const CAP_PROP_XI_CC_MATRIX_32: i32 = 493;
/// Color Correction Matrix element [3][3].
// CAP_PROP_XI_CC_MATRIX_33 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:436
pub const CAP_PROP_XI_CC_MATRIX_33: i32 = 494;
/// Camera sensor temperature.
// CAP_PROP_XI_CHIP_TEMP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:410
pub const CAP_PROP_XI_CHIP_TEMP: i32 = 468;
/// Mode of color management system.
// CAP_PROP_XI_CMS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:414
pub const CAP_PROP_XI_CMS: i32 = 470;
/// Returns color filter array type of RAW data.
// CAP_PROP_XI_COLOR_FILTER_ARRAY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:417
pub const CAP_PROP_XI_COLOR_FILTER_ARRAY: i32 = 475;
/// Correction of column FPN.
// CAP_PROP_XI_COLUMN_FPN_CORRECTION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:480
pub const CAP_PROP_XI_COLUMN_FPN_CORRECTION: i32 = 555;
/// Start camera cooling.
// CAP_PROP_XI_COOLING /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:408
pub const CAP_PROP_XI_COOLING: i32 = 466;
/// Select counter.
// CAP_PROP_XI_COUNTER_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:461
pub const CAP_PROP_XI_COUNTER_SELECTOR: i32 = 536;
/// Counter status.
// CAP_PROP_XI_COUNTER_VALUE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:462
pub const CAP_PROP_XI_COUNTER_VALUE: i32 = 537;
/// Output data format.
// CAP_PROP_XI_DATA_FORMAT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:351
pub const CAP_PROP_XI_DATA_FORMAT: i32 = 401;
/// Enable/Disable debounce to selected GPI.
// CAP_PROP_XI_DEBOUNCE_EN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:440
pub const CAP_PROP_XI_DEBOUNCE_EN: i32 = 507;
/// Debounce polarity (pol = 1 t0 - falling edge, t1 - rising edge).
// CAP_PROP_XI_DEBOUNCE_POL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:443
pub const CAP_PROP_XI_DEBOUNCE_POL: i32 = 510;
/// Debounce time (x * 10us).
// CAP_PROP_XI_DEBOUNCE_T0 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:441
pub const CAP_PROP_XI_DEBOUNCE_T0: i32 = 508;
/// Debounce time (x * 10us).
// CAP_PROP_XI_DEBOUNCE_T1 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:442
pub const CAP_PROP_XI_DEBOUNCE_T1: i32 = 509;
/// Set debug level.
// CAP_PROP_XI_DEBUG_LEVEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:491
pub const CAP_PROP_XI_DEBUG_LEVEL: i32 = 572;
/// Horizontal Decimation - horizontal sub-sampling of the image - reduces the horizontal resolution of the image by the specified vertical decimation factor.
// CAP_PROP_XI_DECIMATION_HORIZONTAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:382
pub const CAP_PROP_XI_DECIMATION_HORIZONTAL: i32 = 433;
/// Decimation pattern type.
// CAP_PROP_XI_DECIMATION_PATTERN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:383
pub const CAP_PROP_XI_DECIMATION_PATTERN: i32 = 434;
/// Decimation engine selector.
// CAP_PROP_XI_DECIMATION_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:380
pub const CAP_PROP_XI_DECIMATION_SELECTOR: i32 = 431;
/// Vertical Decimation - vertical sub-sampling of the image - reduces the vertical resolution of the image by the specified vertical decimation factor.
// CAP_PROP_XI_DECIMATION_VERTICAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:381
pub const CAP_PROP_XI_DECIMATION_VERTICAL: i32 = 432;
/// Set default Color Correction Matrix.
// CAP_PROP_XI_DEFAULT_CC_MATRIX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:437
pub const CAP_PROP_XI_DEFAULT_CC_MATRIX: i32 = 495;
/// Returns device model id.
// CAP_PROP_XI_DEVICE_MODEL_ID /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:452
pub const CAP_PROP_XI_DEVICE_MODEL_ID: i32 = 521;
/// Resets the camera to default state.
// CAP_PROP_XI_DEVICE_RESET /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:479
pub const CAP_PROP_XI_DEVICE_RESET: i32 = 554;
/// Returns device serial number.
// CAP_PROP_XI_DEVICE_SN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:453
pub const CAP_PROP_XI_DEVICE_SN: i32 = 522;
/// Change image resolution by binning or skipping.
// CAP_PROP_XI_DOWNSAMPLING /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:350
pub const CAP_PROP_XI_DOWNSAMPLING: i32 = 400;
/// Change image downsampling type.
// CAP_PROP_XI_DOWNSAMPLING_TYPE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:375
pub const CAP_PROP_XI_DOWNSAMPLING_TYPE: i32 = 426;
/// Exposure time in microseconds.
// CAP_PROP_XI_EXPOSURE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:371
pub const CAP_PROP_XI_EXPOSURE: i32 = 421;
/// Sets the number of times of exposure in one frame.
// CAP_PROP_XI_EXPOSURE_BURST_COUNT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:372
pub const CAP_PROP_XI_EXPOSURE_BURST_COUNT: i32 = 422;
/// Exposure priority (0.5 - exposure 50%, gain 50%).
// CAP_PROP_XI_EXP_PRIORITY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:366
pub const CAP_PROP_XI_EXP_PRIORITY: i32 = 416;
/// Setting of key enables file operations on some cameras.
// CAP_PROP_XI_FFS_ACCESS_KEY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:497
pub const CAP_PROP_XI_FFS_ACCESS_KEY: i32 = 583;
/// File number.
// CAP_PROP_XI_FFS_FILE_ID /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:493
pub const CAP_PROP_XI_FFS_FILE_ID: i32 = 594;
/// Size of file.
// CAP_PROP_XI_FFS_FILE_SIZE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:494
pub const CAP_PROP_XI_FFS_FILE_SIZE: i32 = 580;
/// Define framerate in Hz.
// CAP_PROP_XI_FRAMERATE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:460
pub const CAP_PROP_XI_FRAMERATE: i32 = 535;
/// Size of free camera FFS.
// CAP_PROP_XI_FREE_FFS_SIZE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:495
pub const CAP_PROP_XI_FREE_FFS_SIZE: i32 = 581;
/// Gain in dB.
// CAP_PROP_XI_GAIN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:374
pub const CAP_PROP_XI_GAIN: i32 = 424;
/// Gain selector for parameter Gain allows to select different type of gains.
// CAP_PROP_XI_GAIN_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:373
pub const CAP_PROP_XI_GAIN_SELECTOR: i32 = 423;
/// Chromaticity gamma.
// CAP_PROP_XI_GAMMAC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:419
pub const CAP_PROP_XI_GAMMAC: i32 = 477;
/// Luminosity gamma.
// CAP_PROP_XI_GAMMAY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:418
pub const CAP_PROP_XI_GAMMAY: i32 = 476;
/// Get general purpose level.
// CAP_PROP_XI_GPI_LEVEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:358
pub const CAP_PROP_XI_GPI_LEVEL: i32 = 408;
/// Set general purpose input mode.
// CAP_PROP_XI_GPI_MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:357
pub const CAP_PROP_XI_GPI_MODE: i32 = 407;
/// Selects general purpose input.
// CAP_PROP_XI_GPI_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:356
pub const CAP_PROP_XI_GPI_SELECTOR: i32 = 406;
/// Set general purpose output mode.
// CAP_PROP_XI_GPO_MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:360
pub const CAP_PROP_XI_GPO_MODE: i32 = 410;
/// Selects general purpose output.
// CAP_PROP_XI_GPO_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:359
pub const CAP_PROP_XI_GPO_SELECTOR: i32 = 409;
/// Enable High Dynamic Range feature.
// CAP_PROP_XI_HDR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:483
pub const CAP_PROP_XI_HDR: i32 = 559;
/// The number of kneepoints in the PWLR.
// CAP_PROP_XI_HDR_KNEEPOINT_COUNT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:484
pub const CAP_PROP_XI_HDR_KNEEPOINT_COUNT: i32 = 560;
/// Position of first kneepoint(in % of XI_PRM_EXPOSURE).
// CAP_PROP_XI_HDR_T1 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:485
pub const CAP_PROP_XI_HDR_T1: i32 = 561;
/// Position of second kneepoint (in % of XI_PRM_EXPOSURE).
// CAP_PROP_XI_HDR_T2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:486
pub const CAP_PROP_XI_HDR_T2: i32 = 562;
/// Height of the Image provided by the device (in pixels).
// CAP_PROP_XI_HEIGHT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:398
pub const CAP_PROP_XI_HEIGHT: i32 = 452;
/// Camera housing back side temperature.
// CAP_PROP_XI_HOUS_BACK_SIDE_TEMP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:412
pub const CAP_PROP_XI_HOUS_BACK_SIDE_TEMP: i32 = 590;
/// Camera housing temperature.
// CAP_PROP_XI_HOUS_TEMP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:411
pub const CAP_PROP_XI_HOUS_TEMP: i32 = 469;
/// Returns hardware revision number.
// CAP_PROP_XI_HW_REVISION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:490
pub const CAP_PROP_XI_HW_REVISION: i32 = 571;
/// Last image black level counts. Can be used for Offline processing to recall it.
// CAP_PROP_XI_IMAGE_BLACK_LEVEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:489
pub const CAP_PROP_XI_IMAGE_BLACK_LEVEL: i32 = 565;
/// bitdepth of data returned by function xiGetImage.
// CAP_PROP_XI_IMAGE_DATA_BIT_DEPTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:404
pub const CAP_PROP_XI_IMAGE_DATA_BIT_DEPTH: i32 = 462;
/// Output data format.
// CAP_PROP_XI_IMAGE_DATA_FORMAT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:386
pub const CAP_PROP_XI_IMAGE_DATA_FORMAT: i32 = 435;
/// The alpha channel of RGB32 output image format.
// CAP_PROP_XI_IMAGE_DATA_FORMAT_RGB32_ALPHA /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:454
pub const CAP_PROP_XI_IMAGE_DATA_FORMAT_RGB32_ALPHA: i32 = 529;
/// Returns 1 for color cameras.
// CAP_PROP_XI_IMAGE_IS_COLOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:416
pub const CAP_PROP_XI_IMAGE_IS_COLOR: i32 = 474;
/// Buffer size in bytes sufficient for output image returned by xiGetImage.
// CAP_PROP_XI_IMAGE_PAYLOAD_SIZE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:455
pub const CAP_PROP_XI_IMAGE_PAYLOAD_SIZE: i32 = 530;
/// Returns 1 for cameras that support cooling.
// CAP_PROP_XI_IS_COOLED /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:407
pub const CAP_PROP_XI_IS_COOLED: i32 = 465;
/// Returns 1 if camera connected and works properly.
// CAP_PROP_XI_IS_DEVICE_EXIST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:472
pub const CAP_PROP_XI_IS_DEVICE_EXIST: i32 = 547;
/// Value of first kneepoint (% of sensor saturation).
// CAP_PROP_XI_KNEEPOINT1 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:487
pub const CAP_PROP_XI_KNEEPOINT1: i32 = 563;
/// Value of second kneepoint (% of sensor saturation).
// CAP_PROP_XI_KNEEPOINT2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:488
pub const CAP_PROP_XI_KNEEPOINT2: i32 = 564;
/// Define camera signalling LED functionality.
// CAP_PROP_XI_LED_MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:362
pub const CAP_PROP_XI_LED_MODE: i32 = 412;
/// Selects camera signalling LED.
// CAP_PROP_XI_LED_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:361
pub const CAP_PROP_XI_LED_SELECTOR: i32 = 411;
/// Current lens aperture value in stops. Examples: 2.8, 4, 5.6, 8, 11.
// CAP_PROP_XI_LENS_APERTURE_VALUE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:445
pub const CAP_PROP_XI_LENS_APERTURE_VALUE: i32 = 512;
/// Allows access to lens feature value currently selected by XI_PRM_LENS_FEATURE_SELECTOR.
// CAP_PROP_XI_LENS_FEATURE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:451
pub const CAP_PROP_XI_LENS_FEATURE: i32 = 518;
/// Selects the current feature which is accessible by XI_PRM_LENS_FEATURE.
// CAP_PROP_XI_LENS_FEATURE_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:450
pub const CAP_PROP_XI_LENS_FEATURE_SELECTOR: i32 = 517;
/// Lens focal distance in mm.
// CAP_PROP_XI_LENS_FOCAL_LENGTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:449
pub const CAP_PROP_XI_LENS_FOCAL_LENGTH: i32 = 516;
/// Lens focus distance in cm.
// CAP_PROP_XI_LENS_FOCUS_DISTANCE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:448
pub const CAP_PROP_XI_LENS_FOCUS_DISTANCE: i32 = 515;
/// Moves lens focus motor by steps set in XI_PRM_LENS_FOCUS_MOVEMENT_VALUE.
// CAP_PROP_XI_LENS_FOCUS_MOVE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:447
pub const CAP_PROP_XI_LENS_FOCUS_MOVE: i32 = 514;
/// Lens current focus movement value to be used by XI_PRM_LENS_FOCUS_MOVE in motor steps.
// CAP_PROP_XI_LENS_FOCUS_MOVEMENT_VALUE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:446
pub const CAP_PROP_XI_LENS_FOCUS_MOVEMENT_VALUE: i32 = 513;
/// Status of lens control interface. This shall be set to XI_ON before any Lens operations.
// CAP_PROP_XI_LENS_MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:444
pub const CAP_PROP_XI_LENS_MODE: i32 = 511;
/// Set/get bandwidth(datarate)(in Megabits).
// CAP_PROP_XI_LIMIT_BANDWIDTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:401
pub const CAP_PROP_XI_LIMIT_BANDWIDTH: i32 = 459;
/// Activates LUT.
// CAP_PROP_XI_LUT_EN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:466
pub const CAP_PROP_XI_LUT_EN: i32 = 541;
/// Control the index (offset) of the coefficient to access in the LUT.
// CAP_PROP_XI_LUT_INDEX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:467
pub const CAP_PROP_XI_LUT_INDEX: i32 = 542;
/// Value at entry LUTIndex of the LUT.
// CAP_PROP_XI_LUT_VALUE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:468
pub const CAP_PROP_XI_LUT_VALUE: i32 = 543;
/// Calculates White Balance(must be called during acquisition).
// CAP_PROP_XI_MANUAL_WB /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:363
pub const CAP_PROP_XI_MANUAL_WB: i32 = 413;
/// Horizontal offset from the origin to the area of interest (in pixels).
// CAP_PROP_XI_OFFSET_X /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:352
pub const CAP_PROP_XI_OFFSET_X: i32 = 402;
/// Vertical offset from the origin to the area of interest (in pixels).
// CAP_PROP_XI_OFFSET_Y /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:353
pub const CAP_PROP_XI_OFFSET_Y: i32 = 403;
/// Device output data bit depth.
// CAP_PROP_XI_OUTPUT_DATA_BIT_DEPTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:403
pub const CAP_PROP_XI_OUTPUT_DATA_BIT_DEPTH: i32 = 461;
/// Device output data packing (or grouping) enabled. Packing could be enabled if output_data_bit_depth > 8 and packing capability is available.
// CAP_PROP_XI_OUTPUT_DATA_PACKING /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:405
pub const CAP_PROP_XI_OUTPUT_DATA_PACKING: i32 = 463;
/// Data packing type. Some cameras supports only specific packing type.
// CAP_PROP_XI_OUTPUT_DATA_PACKING_TYPE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:406
pub const CAP_PROP_XI_OUTPUT_DATA_PACKING_TYPE: i32 = 464;
/// GetImage returns most recent frame.
// CAP_PROP_XI_RECENT_FRAME /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:478
pub const CAP_PROP_XI_RECENT_FRAME: i32 = 553;
/// Activates/deactivates Region selected by Region Selector.
// CAP_PROP_XI_REGION_MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:400
pub const CAP_PROP_XI_REGION_MODE: i32 = 595;
/// Selects Region in Multiple ROI which parameters are set by width, height, ... ,region mode.
// CAP_PROP_XI_REGION_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:399
pub const CAP_PROP_XI_REGION_SELECTOR: i32 = 589;
/// Correction of row FPN.
// CAP_PROP_XI_ROW_FPN_CORRECTION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:481
pub const CAP_PROP_XI_ROW_FPN_CORRECTION: i32 = 591;
/// Camera sensor board temperature.
// CAP_PROP_XI_SENSOR_BOARD_TEMP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:413
pub const CAP_PROP_XI_SENSOR_BOARD_TEMP: i32 = 596;
/// Sensor clock frequency in Hz.
// CAP_PROP_XI_SENSOR_CLOCK_FREQ_HZ /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:457
pub const CAP_PROP_XI_SENSOR_CLOCK_FREQ_HZ: i32 = 532;
/// Sensor clock frequency index. Sensor with selected frequencies have possibility to set the frequency only by this index.
// CAP_PROP_XI_SENSOR_CLOCK_FREQ_INDEX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:458
pub const CAP_PROP_XI_SENSOR_CLOCK_FREQ_INDEX: i32 = 533;
/// Sensor output data bit depth.
// CAP_PROP_XI_SENSOR_DATA_BIT_DEPTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:402
pub const CAP_PROP_XI_SENSOR_DATA_BIT_DEPTH: i32 = 460;
/// Selects the current feature which is accessible by XI_PRM_SENSOR_FEATURE_VALUE.
// CAP_PROP_XI_SENSOR_FEATURE_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:498
pub const CAP_PROP_XI_SENSOR_FEATURE_SELECTOR: i32 = 585;
/// Allows access to sensor feature value currently selected by XI_PRM_SENSOR_FEATURE_SELECTOR.
// CAP_PROP_XI_SENSOR_FEATURE_VALUE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:499
pub const CAP_PROP_XI_SENSOR_FEATURE_VALUE: i32 = 586;
/// Current sensor mode. Allows to select sensor mode by one integer. Setting of this parameter affects: image dimensions and downsampling.
// CAP_PROP_XI_SENSOR_MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:482
pub const CAP_PROP_XI_SENSOR_MODE: i32 = 558;
/// Number of output channels from sensor used for data transfer.
// CAP_PROP_XI_SENSOR_OUTPUT_CHANNEL_COUNT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:459
pub const CAP_PROP_XI_SENSOR_OUTPUT_CHANNEL_COUNT: i32 = 534;
/// Number of taps.
// CAP_PROP_XI_SENSOR_TAPS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:388
pub const CAP_PROP_XI_SENSOR_TAPS: i32 = 437;
/// Sharpness Strength.
// CAP_PROP_XI_SHARPNESS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:420
pub const CAP_PROP_XI_SHARPNESS: i32 = 478;
/// Change sensor shutter type(CMOS sensor).
// CAP_PROP_XI_SHUTTER_TYPE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:387
pub const CAP_PROP_XI_SHUTTER_TYPE: i32 = 436;
/// Set sensor target temperature for cooling.
// CAP_PROP_XI_TARGET_TEMP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:409
pub const CAP_PROP_XI_TARGET_TEMP: i32 = 467;
/// Selects which test pattern type is generated by the selected generator.
// CAP_PROP_XI_TEST_PATTERN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:385
pub const CAP_PROP_XI_TEST_PATTERN: i32 = 588;
/// Selects which test pattern generator is controlled by the TestPattern feature.
// CAP_PROP_XI_TEST_PATTERN_GENERATOR_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:384
pub const CAP_PROP_XI_TEST_PATTERN_GENERATOR_SELECTOR: i32 = 587;
/// Image capture timeout in milliseconds.
// CAP_PROP_XI_TIMEOUT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:370
pub const CAP_PROP_XI_TIMEOUT: i32 = 420;
/// Current format of pixels on transport layer.
// CAP_PROP_XI_TRANSPORT_PIXEL_FORMAT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:456
pub const CAP_PROP_XI_TRANSPORT_PIXEL_FORMAT: i32 = 531;
/// Specifies the delay in microseconds (us) to apply after the trigger reception before activating it.
// CAP_PROP_XI_TRG_DELAY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:469
pub const CAP_PROP_XI_TRG_DELAY: i32 = 544;
/// Selects the type of trigger.
// CAP_PROP_XI_TRG_SELECTOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:438
pub const CAP_PROP_XI_TRG_SELECTOR: i32 = 498;
/// Generates an internal trigger. PRM_TRG_SOURCE must be set to TRG_SOFTWARE.
// CAP_PROP_XI_TRG_SOFTWARE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:355
pub const CAP_PROP_XI_TRG_SOFTWARE: i32 = 405;
/// Defines source of trigger.
// CAP_PROP_XI_TRG_SOURCE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:354
pub const CAP_PROP_XI_TRG_SOURCE: i32 = 404;
/// Defines how time stamp reset engine will be armed.
// CAP_PROP_XI_TS_RST_MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:470
pub const CAP_PROP_XI_TS_RST_MODE: i32 = 545;
/// Defines which source will be used for timestamp reset. Writing this parameter will trigger settings of engine (arming).
// CAP_PROP_XI_TS_RST_SOURCE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:471
pub const CAP_PROP_XI_TS_RST_SOURCE: i32 = 546;
/// Size of used camera FFS.
// CAP_PROP_XI_USED_FFS_SIZE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:496
pub const CAP_PROP_XI_USED_FFS_SIZE: i32 = 582;
/// White balance blue coefficient.
// CAP_PROP_XI_WB_KB /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:396
pub const CAP_PROP_XI_WB_KB: i32 = 450;
/// White balance green coefficient.
// CAP_PROP_XI_WB_KG /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:395
pub const CAP_PROP_XI_WB_KG: i32 = 449;
/// White balance red coefficient.
// CAP_PROP_XI_WB_KR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:394
pub const CAP_PROP_XI_WB_KR: i32 = 448;
/// Width of the Image provided by the device (in pixels).
// CAP_PROP_XI_WIDTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:397
pub const CAP_PROP_XI_WIDTH: i32 = 451;
// CAP_PROP_ZOOM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:160
pub const CAP_PROP_ZOOM: i32 = 27;
/// PvAPI, Prosilica GigE SDK
// CAP_PVAPI /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:102
pub const CAP_PVAPI: i32 = 800;
/// 2 out of 16 decimation
// CAP_PVAPI_DECIMATION_2OUTOF16 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:329
pub const CAP_PVAPI_DECIMATION_2OUTOF16: i32 = 8;
/// 2 out of 4 decimation
// CAP_PVAPI_DECIMATION_2OUTOF4 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:327
pub const CAP_PVAPI_DECIMATION_2OUTOF4: i32 = 2;
/// 2 out of 8 decimation
// CAP_PVAPI_DECIMATION_2OUTOF8 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:328
pub const CAP_PVAPI_DECIMATION_2OUTOF8: i32 = 4;
/// Off
// CAP_PVAPI_DECIMATION_OFF /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:326
pub const CAP_PVAPI_DECIMATION_OFF: i32 = 1;
/// FixedRate
// CAP_PVAPI_FSTRIGMODE_FIXEDRATE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:321
pub const CAP_PVAPI_FSTRIGMODE_FIXEDRATE: i32 = 3;
/// Freerun
// CAP_PVAPI_FSTRIGMODE_FREERUN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:318
pub const CAP_PVAPI_FSTRIGMODE_FREERUN: i32 = 0;
/// Software
// CAP_PVAPI_FSTRIGMODE_SOFTWARE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:322
pub const CAP_PVAPI_FSTRIGMODE_SOFTWARE: i32 = 4;
/// SyncIn1
// CAP_PVAPI_FSTRIGMODE_SYNCIN1 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:319
pub const CAP_PVAPI_FSTRIGMODE_SYNCIN1: i32 = 1;
/// SyncIn2
// CAP_PVAPI_FSTRIGMODE_SYNCIN2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:320
pub const CAP_PVAPI_FSTRIGMODE_SYNCIN2: i32 = 2;
/// Bayer16
// CAP_PVAPI_PIXELFORMAT_BAYER16 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:336
pub const CAP_PVAPI_PIXELFORMAT_BAYER16: i32 = 4;
/// Bayer8
// CAP_PVAPI_PIXELFORMAT_BAYER8 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:335
pub const CAP_PVAPI_PIXELFORMAT_BAYER8: i32 = 3;
/// Bgr24
// CAP_PVAPI_PIXELFORMAT_BGR24 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:338
pub const CAP_PVAPI_PIXELFORMAT_BGR24: i32 = 6;
/// Bgra32
// CAP_PVAPI_PIXELFORMAT_BGRA32 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:340
pub const CAP_PVAPI_PIXELFORMAT_BGRA32: i32 = 8;
/// Mono16
// CAP_PVAPI_PIXELFORMAT_MONO16 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:334
pub const CAP_PVAPI_PIXELFORMAT_MONO16: i32 = 2;
/// Mono8
// CAP_PVAPI_PIXELFORMAT_MONO8 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:333
pub const CAP_PVAPI_PIXELFORMAT_MONO8: i32 = 1;
/// Rgb24
// CAP_PVAPI_PIXELFORMAT_RGB24 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:337
pub const CAP_PVAPI_PIXELFORMAT_RGB24: i32 = 5;
/// Rgba32
// CAP_PVAPI_PIXELFORMAT_RGBA32 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:339
pub const CAP_PVAPI_PIXELFORMAT_RGBA32: i32 = 7;
/// QuickTime
// CAP_QT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:99
pub const CAP_QT: i32 = 500;
/// Unicap drivers
// CAP_UNICAP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:100
pub const CAP_UNICAP: i32 = 600;
/// V4L/V4L2 capturing support via libv4l
// CAP_V4L /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:92
pub const CAP_V4L: i32 = 200;
/// Same as CAP_V4L
// CAP_V4L2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:93
pub const CAP_V4L2: i32 = 200;
/// Video For Windows (platform native)
// CAP_VFW /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:91
pub const CAP_VFW: i32 = 200;
/// Microsoft Windows Runtime using Media Foundation
// CAP_WINRT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:110
pub const CAP_WINRT: i32 = 1410;
/// XIMEA Camera API
// CAP_XIAPI /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:106
pub const CAP_XIAPI: i32 = 1100;
/// XINE engine (Linux)
// CAP_XINE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:121
pub const CAP_XINE: i32 = 2400;
// CV__CAP_PROP_LATEST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:185
pub const CV__CAP_PROP_LATEST: i32 = 55;
/// (Read-only): Size of just encoded video frame. Note that the encoding order may be different from representation order.
// VIDEOWRITER_PROP_FRAMEBYTES /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:205
pub const VIDEOWRITER_PROP_FRAMEBYTES: i32 = 2;
/// Number of stripes for parallel encoding. -1 for auto detection.
// VIDEOWRITER_PROP_NSTRIPES /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:206
pub const VIDEOWRITER_PROP_NSTRIPES: i32 = 3;
/// Current quality (0..100%) of the encoded videostream. Can be adjusted dynamically in some codecs.
// VIDEOWRITER_PROP_QUALITY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:204
pub const VIDEOWRITER_PROP_QUALITY: i32 = 1;
/// %VideoCapture API backends identifier.
///
/// Select preferred API for a capture object.
/// To be used in the VideoCapture::VideoCapture() constructor or VideoCapture::open()
///
///
/// Note: Backends are available only if they have been built with your OpenCV binaries.
/// See [videoio_overview] for more information.
// VideoCaptureAPIs /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:89
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VideoCaptureAPIs {
	/// Auto detect == 0
	CAP_ANY = 0,
	/// Video For Windows (platform native)
	CAP_VFW = 200,
	// V4L/V4L2 capturing support via libv4l
	// Duplicate, use CAP_VFW instead
	// CAP_V4L = 200,
	// Same as CAP_V4L
	// Duplicate, use CAP_V4L instead
	// CAP_V4L2 = 200,
	/// IEEE 1394 drivers
	CAP_FIREWIRE = 300,
	// Same as CAP_FIREWIRE
	// Duplicate, use CAP_FIREWIRE instead
	// CAP_FIREWARE = 300,
	// Same as CAP_FIREWIRE
	// Duplicate, use CAP_FIREWARE instead
	// CAP_IEEE1394 = 300,
	// Same as CAP_FIREWIRE
	// Duplicate, use CAP_IEEE1394 instead
	// CAP_DC1394 = 300,
	// Same as CAP_FIREWIRE
	// Duplicate, use CAP_DC1394 instead
	// CAP_CMU1394 = 300,
	/// QuickTime
	CAP_QT = 500,
	/// Unicap drivers
	CAP_UNICAP = 600,
	/// DirectShow (via videoInput)
	CAP_DSHOW = 700,
	/// PvAPI, Prosilica GigE SDK
	CAP_PVAPI = 800,
	/// OpenNI (for Kinect)
	CAP_OPENNI = 900,
	/// OpenNI (for Asus Xtion)
	CAP_OPENNI_ASUS = 910,
	/// Android - not used
	CAP_ANDROID = 1000,
	/// XIMEA Camera API
	CAP_XIAPI = 1100,
	/// AVFoundation framework for iOS (OS X Lion will have the same API)
	CAP_AVFOUNDATION = 1200,
	/// Smartek Giganetix GigEVisionSDK
	CAP_GIGANETIX = 1300,
	/// Microsoft Media Foundation (via videoInput)
	CAP_MSMF = 1400,
	/// Microsoft Windows Runtime using Media Foundation
	CAP_WINRT = 1410,
	/// Intel Perceptual Computing SDK
	CAP_INTELPERC = 1500,
	/// OpenNI2 (for Kinect)
	CAP_OPENNI2 = 1600,
	/// OpenNI2 (for Asus Xtion and Occipital Structure sensors)
	CAP_OPENNI2_ASUS = 1610,
	/// gPhoto2 connection
	CAP_GPHOTO2 = 1700,
	/// GStreamer
	CAP_GSTREAMER = 1800,
	/// Open and record video file or stream using the FFMPEG library
	CAP_FFMPEG = 1900,
	/// OpenCV Image Sequence (e.g. img_%02d.jpg)
	CAP_IMAGES = 2000,
	/// Aravis SDK
	CAP_ARAVIS = 2100,
	/// Built-in OpenCV MotionJPEG codec
	CAP_OPENCV_MJPEG = 2200,
	/// Intel MediaSDK
	CAP_INTEL_MFX = 2300,
	/// XINE engine (Linux)
	CAP_XINE = 2400,
}

impl TryFrom<i32> for VideoCaptureAPIs {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::CAP_ANY),
			200 => Ok(Self::CAP_VFW),
			// Duplicate of CAP_VFW
			// 200 => Ok(Self::CAP_V4L),
			// Duplicate of CAP_V4L
			// 200 => Ok(Self::CAP_V4L2),
			300 => Ok(Self::CAP_FIREWIRE),
			// Duplicate of CAP_FIREWIRE
			// 300 => Ok(Self::CAP_FIREWARE),
			// Duplicate of CAP_FIREWARE
			// 300 => Ok(Self::CAP_IEEE1394),
			// Duplicate of CAP_IEEE1394
			// 300 => Ok(Self::CAP_DC1394),
			// Duplicate of CAP_DC1394
			// 300 => Ok(Self::CAP_CMU1394),
			500 => Ok(Self::CAP_QT),
			600 => Ok(Self::CAP_UNICAP),
			700 => Ok(Self::CAP_DSHOW),
			800 => Ok(Self::CAP_PVAPI),
			900 => Ok(Self::CAP_OPENNI),
			910 => Ok(Self::CAP_OPENNI_ASUS),
			1000 => Ok(Self::CAP_ANDROID),
			1100 => Ok(Self::CAP_XIAPI),
			1200 => Ok(Self::CAP_AVFOUNDATION),
			1300 => Ok(Self::CAP_GIGANETIX),
			1400 => Ok(Self::CAP_MSMF),
			1410 => Ok(Self::CAP_WINRT),
			1500 => Ok(Self::CAP_INTELPERC),
			1600 => Ok(Self::CAP_OPENNI2),
			1610 => Ok(Self::CAP_OPENNI2_ASUS),
			1700 => Ok(Self::CAP_GPHOTO2),
			1800 => Ok(Self::CAP_GSTREAMER),
			1900 => Ok(Self::CAP_FFMPEG),
			2000 => Ok(Self::CAP_IMAGES),
			2100 => Ok(Self::CAP_ARAVIS),
			2200 => Ok(Self::CAP_OPENCV_MJPEG),
			2300 => Ok(Self::CAP_INTEL_MFX),
			2400 => Ok(Self::CAP_XINE),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::videoio::VideoCaptureAPIs"))),
		}
	}
}

opencv_type_enum! { crate::videoio::VideoCaptureAPIs }

/// Generic camera output modes identifier.
///
/// Note: Currently, these are supported through the libv4l backend only.
// VideoCaptureModes /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:193
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VideoCaptureModes {
	/// BGR24 (default)
	CAP_MODE_BGR = 0,
	/// RGB24
	CAP_MODE_RGB = 1,
	/// Y8
	CAP_MODE_GRAY = 2,
	/// YUYV
	CAP_MODE_YUYV = 3,
}

impl TryFrom<i32> for VideoCaptureModes {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::CAP_MODE_BGR),
			1 => Ok(Self::CAP_MODE_RGB),
			2 => Ok(Self::CAP_MODE_GRAY),
			3 => Ok(Self::CAP_MODE_YUYV),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::videoio::VideoCaptureModes"))),
		}
	}
}

opencv_type_enum! { crate::videoio::VideoCaptureModes }

/// %VideoCapture generic properties identifier.
///
/// Reading / writing properties involves many layers. Some unexpected result might happens along this chain.
/// Effective behaviour depends from device hardware, driver and API Backend.
/// ## See also
/// videoio_flags_others, VideoCapture::get(), VideoCapture::set()
// VideoCaptureProperties /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:130
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VideoCaptureProperties {
	/// Current position of the video file in milliseconds.
	CAP_PROP_POS_MSEC = 0,
	/// 0-based index of the frame to be decoded/captured next.
	CAP_PROP_POS_FRAMES = 1,
	/// Relative position of the video file: 0=start of the film, 1=end of the film.
	CAP_PROP_POS_AVI_RATIO = 2,
	/// Width of the frames in the video stream.
	CAP_PROP_FRAME_WIDTH = 3,
	/// Height of the frames in the video stream.
	CAP_PROP_FRAME_HEIGHT = 4,
	/// Frame rate.
	CAP_PROP_FPS = 5,
	/// 4-character code of codec. see VideoWriter::fourcc .
	CAP_PROP_FOURCC = 6,
	/// Number of frames in the video file.
	CAP_PROP_FRAME_COUNT = 7,
	/// Format of the %Mat objects (see Mat::type()) returned by VideoCapture::retrieve().
	/// Set value -1 to fetch undecoded RAW video streams (as Mat 8UC1).
	CAP_PROP_FORMAT = 8,
	/// Backend-specific value indicating the current capture mode.
	CAP_PROP_MODE = 9,
	/// Brightness of the image (only for those cameras that support).
	CAP_PROP_BRIGHTNESS = 10,
	/// Contrast of the image (only for cameras).
	CAP_PROP_CONTRAST = 11,
	/// Saturation of the image (only for cameras).
	CAP_PROP_SATURATION = 12,
	/// Hue of the image (only for cameras).
	CAP_PROP_HUE = 13,
	/// Gain of the image (only for those cameras that support).
	CAP_PROP_GAIN = 14,
	/// Exposure (only for those cameras that support).
	CAP_PROP_EXPOSURE = 15,
	/// Boolean flags indicating whether images should be converted to RGB. <br/>
	/// *GStreamer note*: The flag is ignored in case if custom pipeline is used. It's user responsibility to interpret pipeline output.
	CAP_PROP_CONVERT_RGB = 16,
	/// Currently unsupported.
	CAP_PROP_WHITE_BALANCE_BLUE_U = 17,
	/// Rectification flag for stereo cameras (note: only supported by DC1394 v 2.x backend currently).
	CAP_PROP_RECTIFICATION = 18,
	CAP_PROP_MONOCHROME = 19,
	CAP_PROP_SHARPNESS = 20,
	/// DC1394: exposure control done by camera, user can adjust reference level using this feature.
	CAP_PROP_AUTO_EXPOSURE = 21,
	CAP_PROP_GAMMA = 22,
	CAP_PROP_TEMPERATURE = 23,
	CAP_PROP_TRIGGER = 24,
	CAP_PROP_TRIGGER_DELAY = 25,
	CAP_PROP_WHITE_BALANCE_RED_V = 26,
	CAP_PROP_ZOOM = 27,
	CAP_PROP_FOCUS = 28,
	CAP_PROP_GUID = 29,
	CAP_PROP_ISO_SPEED = 30,
	CAP_PROP_BACKLIGHT = 32,
	CAP_PROP_PAN = 33,
	CAP_PROP_TILT = 34,
	CAP_PROP_ROLL = 35,
	CAP_PROP_IRIS = 36,
	/// Pop up video/camera filter dialog (note: only supported by DSHOW backend currently. The property value is ignored)
	CAP_PROP_SETTINGS = 37,
	CAP_PROP_BUFFERSIZE = 38,
	CAP_PROP_AUTOFOCUS = 39,
	/// Sample aspect ratio: num/den (num)
	CAP_PROP_SAR_NUM = 40,
	/// Sample aspect ratio: num/den (den)
	CAP_PROP_SAR_DEN = 41,
	/// Current backend (enum VideoCaptureAPIs). Read-only property
	CAP_PROP_BACKEND = 42,
	/// Video input or Channel Number (only for those cameras that support)
	CAP_PROP_CHANNEL = 43,
	/// enable/ disable auto white-balance
	CAP_PROP_AUTO_WB = 44,
	/// white-balance color temperature
	CAP_PROP_WB_TEMPERATURE = 45,
	/// (read-only) codec's pixel format. 4-character code - see VideoWriter::fourcc . Subset of [AV_PIX_FMT_*](https://github.com/FFmpeg/FFmpeg/blob/master/libavcodec/raw.c) or -1 if unknown
	CAP_PROP_CODEC_PIXEL_FORMAT = 46,
	/// (read-only) Video bitrate in kbits/s
	CAP_PROP_BITRATE = 47,
	/// (read-only) Frame rotation defined by stream meta (applicable for FFmpeg back-end only)
	CAP_PROP_ORIENTATION_META = 48,
	/// if true - rotates output frames of CvCapture considering video file's metadata  (applicable for FFmpeg back-end only) (<https://github.com/opencv/opencv/issues/15499>)
	CAP_PROP_ORIENTATION_AUTO = 49,
	CAP_PROP_OPEN_TIMEOUT_MSEC = 53,
	CAP_PROP_READ_TIMEOUT_MSEC = 54,
	CV__CAP_PROP_LATEST = 55,
}

impl TryFrom<i32> for VideoCaptureProperties {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::CAP_PROP_POS_MSEC),
			1 => Ok(Self::CAP_PROP_POS_FRAMES),
			2 => Ok(Self::CAP_PROP_POS_AVI_RATIO),
			3 => Ok(Self::CAP_PROP_FRAME_WIDTH),
			4 => Ok(Self::CAP_PROP_FRAME_HEIGHT),
			5 => Ok(Self::CAP_PROP_FPS),
			6 => Ok(Self::CAP_PROP_FOURCC),
			7 => Ok(Self::CAP_PROP_FRAME_COUNT),
			8 => Ok(Self::CAP_PROP_FORMAT),
			9 => Ok(Self::CAP_PROP_MODE),
			10 => Ok(Self::CAP_PROP_BRIGHTNESS),
			11 => Ok(Self::CAP_PROP_CONTRAST),
			12 => Ok(Self::CAP_PROP_SATURATION),
			13 => Ok(Self::CAP_PROP_HUE),
			14 => Ok(Self::CAP_PROP_GAIN),
			15 => Ok(Self::CAP_PROP_EXPOSURE),
			16 => Ok(Self::CAP_PROP_CONVERT_RGB),
			17 => Ok(Self::CAP_PROP_WHITE_BALANCE_BLUE_U),
			18 => Ok(Self::CAP_PROP_RECTIFICATION),
			19 => Ok(Self::CAP_PROP_MONOCHROME),
			20 => Ok(Self::CAP_PROP_SHARPNESS),
			21 => Ok(Self::CAP_PROP_AUTO_EXPOSURE),
			22 => Ok(Self::CAP_PROP_GAMMA),
			23 => Ok(Self::CAP_PROP_TEMPERATURE),
			24 => Ok(Self::CAP_PROP_TRIGGER),
			25 => Ok(Self::CAP_PROP_TRIGGER_DELAY),
			26 => Ok(Self::CAP_PROP_WHITE_BALANCE_RED_V),
			27 => Ok(Self::CAP_PROP_ZOOM),
			28 => Ok(Self::CAP_PROP_FOCUS),
			29 => Ok(Self::CAP_PROP_GUID),
			30 => Ok(Self::CAP_PROP_ISO_SPEED),
			32 => Ok(Self::CAP_PROP_BACKLIGHT),
			33 => Ok(Self::CAP_PROP_PAN),
			34 => Ok(Self::CAP_PROP_TILT),
			35 => Ok(Self::CAP_PROP_ROLL),
			36 => Ok(Self::CAP_PROP_IRIS),
			37 => Ok(Self::CAP_PROP_SETTINGS),
			38 => Ok(Self::CAP_PROP_BUFFERSIZE),
			39 => Ok(Self::CAP_PROP_AUTOFOCUS),
			40 => Ok(Self::CAP_PROP_SAR_NUM),
			41 => Ok(Self::CAP_PROP_SAR_DEN),
			42 => Ok(Self::CAP_PROP_BACKEND),
			43 => Ok(Self::CAP_PROP_CHANNEL),
			44 => Ok(Self::CAP_PROP_AUTO_WB),
			45 => Ok(Self::CAP_PROP_WB_TEMPERATURE),
			46 => Ok(Self::CAP_PROP_CODEC_PIXEL_FORMAT),
			47 => Ok(Self::CAP_PROP_BITRATE),
			48 => Ok(Self::CAP_PROP_ORIENTATION_META),
			49 => Ok(Self::CAP_PROP_ORIENTATION_AUTO),
			53 => Ok(Self::CAP_PROP_OPEN_TIMEOUT_MSEC),
			54 => Ok(Self::CAP_PROP_READ_TIMEOUT_MSEC),
			55 => Ok(Self::CV__CAP_PROP_LATEST),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::videoio::VideoCaptureProperties"))),
		}
	}
}

opencv_type_enum! { crate::videoio::VideoCaptureProperties }

/// %VideoWriter generic properties identifier.
/// ## See also
/// VideoWriter::get(), VideoWriter::set()
// VideoWriterProperties /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:203
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VideoWriterProperties {
	/// Current quality (0..100%) of the encoded videostream. Can be adjusted dynamically in some codecs.
	VIDEOWRITER_PROP_QUALITY = 1,
	/// (Read-only): Size of just encoded video frame. Note that the encoding order may be different from representation order.
	VIDEOWRITER_PROP_FRAMEBYTES = 2,
	/// Number of stripes for parallel encoding. -1 for auto detection.
	VIDEOWRITER_PROP_NSTRIPES = 3,
}

impl TryFrom<i32> for VideoWriterProperties {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::VIDEOWRITER_PROP_QUALITY),
			2 => Ok(Self::VIDEOWRITER_PROP_FRAMEBYTES),
			3 => Ok(Self::VIDEOWRITER_PROP_NSTRIPES),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::videoio::VideoWriterProperties"))),
		}
	}
}

opencv_type_enum! { crate::videoio::VideoWriterProperties }

/// Returns backend API name or "unknown"
/// ## Parameters
/// * api: backend ID (#VideoCaptureAPIs)
// getBackendName(VideoCaptureAPIs)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:27
// ("cv::videoio_registry::getBackendName", vec![(pred!(mut, ["api"], ["cv::VideoCaptureAPIs"]), _)]),
#[inline]
pub fn get_backend_name(api: crate::videoio::VideoCaptureAPIs) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getBackendName_VideoCaptureAPIs(api, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns list of all builtin backends
// getBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:30
// ("cv::videoio_registry::getBackends", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn get_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns list of available backends which works via `cv::VideoCapture(int index)`
// getCameraBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:33
// ("cv::videoio_registry::getCameraBackends", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn get_camera_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getCameraBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns list of available backends which works via `cv::VideoCapture(filename)`
// getStreamBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:36
// ("cv::videoio_registry::getStreamBackends", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn get_stream_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getStreamBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns list of available backends which works via `cv::VideoWriter()`
// getWriterBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:39
// ("cv::videoio_registry::getWriterBackends", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn get_writer_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videoio_registry_getWriterBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Constant methods for [crate::videoio::VideoCapture]
// VideoCapture /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:622
pub trait VideoCaptureTraitConst {
	fn as_raw_VideoCapture(&self) -> *const c_void;

	/// Returns true if video capturing has been initialized already.
	///
	/// If the previous call to VideoCapture constructor or VideoCapture::open() succeeded, the method returns
	/// true.
	// isOpened()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:722
	// ("cv::VideoCapture::isOpened", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_opened(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_isOpened_const(self.as_raw_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the specified VideoCapture property
	///
	/// ## Parameters
	/// * propId: Property identifier from cv::VideoCaptureProperties (eg. cv::CAP_PROP_POS_MSEC, cv::CAP_PROP_POS_FRAMES, ...)
	/// or one from [videoio_flags_others]
	/// ## Returns
	/// Value for the specified property. Value 0 is returned when querying a property that is
	/// not supported by the backend used by the VideoCapture instance.
	///
	///
	/// Note: Reading / writing properties involves many layers. Some unexpected result might happens
	/// along this chain.
	/// ```C++
	/// VideoCapture -> API Backend -> Operating System -> Device Driver -> Device Hardware
	/// ```
	///
	/// The returned value might be different from what really used by the device or it could be encoded
	/// using device dependent rules (eg. steps or percentage). Effective behaviour depends from device
	/// driver and API Backend
	// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:826
	// ("cv::VideoCapture::get", vec![(pred!(const, ["propId"], ["int"]), _)]),
	#[inline]
	fn get(&self, prop_id: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_get_const_int(self.as_raw_VideoCapture(), prop_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns used backend API name
	///
	///
	/// Note: Stream should be opened.
	// getBackendName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:843
	// ("cv::VideoCapture::getBackendName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_backend_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_getBackendName_const(self.as_raw_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::videoio::VideoCapture]
pub trait VideoCaptureTrait: crate::videoio::VideoCaptureTraitConst {
	fn as_raw_mut_VideoCapture(&mut self) -> *mut c_void;

	/// Open video file or a capturing device or a IP video stream for video capturing
	///
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	///
	/// Parameters are same as the constructor VideoCapture(const String& filename)
	/// ## Returns
	/// `true` if the file has been successfully opened
	///
	/// The method first calls VideoCapture::release to close the already opened file or camera.
	// open(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:694
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	#[inline]
	fn open_file_default(&mut self, filename: &str) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_open_const_StringR(self.as_raw_mut_VideoCapture(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Open a camera for video capturing
	///
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	///
	/// Parameters are same as the constructor VideoCapture(int index)
	/// ## Returns
	/// `true` if the camera has been successfully opened.
	///
	/// The method first calls VideoCapture::release to close the already opened file or camera.
	// open(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:705
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["index"], ["int"]), _)]),
	#[inline]
	fn open_default(&mut self, index: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_open_int(self.as_raw_mut_VideoCapture(), index, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Open a camera for video capturing
	///
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	///
	/// Parameters are similar as the constructor VideoCapture(int index),except it takes an additional argument apiPreference.
	/// Definitely, is same as open(int index) where `index=cameraNum + apiPreference`
	/// ## Returns
	/// `true` if the camera has been successfully opened.
	// open(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:715
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["cameraNum", "apiPreference"], ["int", "int"]), _)]),
	#[inline]
	fn open(&mut self, camera_num: i32, api_preference: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_open_int_int(self.as_raw_mut_VideoCapture(), camera_num, api_preference, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Closes video file or capturing device.
	///
	/// The method is automatically called by subsequent VideoCapture::open and by VideoCapture
	/// destructor.
	///
	/// The C function also deallocates memory and clears \*capture pointer.
	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:731
	// ("cv::VideoCapture::release", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_release(self.as_raw_mut_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Grabs the next frame from video file or capturing device.
	///
	/// ## Returns
	/// `true` (non-zero) in the case of success.
	///
	/// The method/function grabs the next frame from video file or camera and returns true (non-zero) in
	/// the case of success.
	///
	/// The primary use of the function is in multi-camera environments, especially when the cameras do not
	/// have hardware synchronization. That is, you call VideoCapture::grab() for each camera and after that
	/// call the slower method VideoCapture::retrieve() to decode and get frame from each camera. This way
	/// the overhead on demosaicing or motion jpeg decompression etc. is eliminated and the retrieved frames
	/// from different cameras will be closer in time.
	///
	/// Also, when a connected camera is multi-head (for example, a stereo camera or a Kinect device), the
	/// correct way of retrieving data from it is to call VideoCapture::grab() first and then call
	/// VideoCapture::retrieve() one or more times with different values of the channel parameter.
	///
	/// [tutorial_kinect_openni]
	// grab()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:752
	// ("cv::VideoCapture::grab", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn grab(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_grab(self.as_raw_mut_VideoCapture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Decodes and returns the grabbed video frame.
	///
	/// ## Parameters
	/// * image:[out] the video frame is returned here. If no frames has been grabbed the image will be empty.
	/// * flag: it could be a frame index or a driver specific flag
	/// ## Returns
	/// `false` if no frames has been grabbed
	///
	/// The method decodes and returns the just grabbed frame. If no frames has been grabbed
	/// (camera has been disconnected, or there are no more frames in video file), the method returns false
	/// and the function returns an empty image (with %cv::Mat, test it with Mat::empty()).
	/// ## See also
	/// read()
	///
	///
	/// Note: In [videoio_c] "C API", functions cvRetrieveFrame() and cv.RetrieveFrame() return image stored inside the video
	/// capturing structure. It is not allowed to modify or release the image! You can copy the frame using
	/// cvCloneImage and then do whatever you want with the copy.
	///
	/// ## C++ default parameters
	/// * flag: 0
	// retrieve(OutputArray, int)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:770
	// ("cv::VideoCapture::retrieve", vec![(pred!(mut, ["image", "flag"], ["const cv::_OutputArray*", "int"]), _)]),
	#[inline]
	fn retrieve(&mut self, image: &mut impl ToOutputArray, flag: i32) -> Result<bool> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_retrieve_const__OutputArrayR_int(self.as_raw_mut_VideoCapture(), image.as_raw__OutputArray(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Decodes and returns the grabbed video frame.
	///
	/// ## Parameters
	/// * image:[out] the video frame is returned here. If no frames has been grabbed the image will be empty.
	/// * flag: it could be a frame index or a driver specific flag
	/// ## Returns
	/// `false` if no frames has been grabbed
	///
	/// The method decodes and returns the just grabbed frame. If no frames has been grabbed
	/// (camera has been disconnected, or there are no more frames in video file), the method returns false
	/// and the function returns an empty image (with %cv::Mat, test it with Mat::empty()).
	/// ## See also
	/// read()
	///
	///
	/// Note: In [videoio_c] "C API", functions cvRetrieveFrame() and cv.RetrieveFrame() return image stored inside the video
	/// capturing structure. It is not allowed to modify or release the image! You can copy the frame using
	/// cvCloneImage and then do whatever you want with the copy.
	///
	/// ## Note
	/// This alternative version of [VideoCaptureTrait::retrieve] function uses the following default values for its arguments:
	/// * flag: 0
	// cv::VideoCapture::retrieve(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:770
	// ("cv::VideoCapture::retrieve", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn retrieve_def(&mut self, image: &mut impl ToOutputArray) -> Result<bool> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_retrieve_const__OutputArrayR(self.as_raw_mut_VideoCapture(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Grabs, decodes and returns the next video frame.
	///
	/// ## Parameters
	/// * image:[out] the video frame is returned here. If no frames has been grabbed the image will be empty.
	/// ## Returns
	/// `false` if no frames has been grabbed
	///
	/// The method/function combines VideoCapture::grab() and VideoCapture::retrieve() in one call. This is the
	/// most convenient method for reading video files or capturing data from decode and returns the just
	/// grabbed frame. If no frames has been grabbed (camera has been disconnected, or there are no more
	/// frames in video file), the method returns false and the function returns empty image (with %cv::Mat, test it with Mat::empty()).
	///
	///
	/// Note: In [videoio_c] "C API", functions cvRetrieveFrame() and cv.RetrieveFrame() return image stored inside the video
	/// capturing structure. It is not allowed to modify or release the image! You can copy the frame using
	/// cvCloneImage and then do whatever you want with the copy.
	// read(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:796
	// ("cv::VideoCapture::read", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn read(&mut self, image: &mut impl ToOutputArray) -> Result<bool> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_read_const__OutputArrayR(self.as_raw_mut_VideoCapture(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets a property in the VideoCapture.
	///
	/// ## Parameters
	/// * propId: Property identifier from cv::VideoCaptureProperties (eg. cv::CAP_PROP_POS_MSEC, cv::CAP_PROP_POS_FRAMES, ...)
	/// or one from [videoio_flags_others]
	/// * value: Value of the property.
	/// ## Returns
	/// `true` if the property is supported by backend used by the VideoCapture instance.
	///
	/// Note: Even if it returns `true` this doesn't ensure that the property
	/// value has been accepted by the capture device. See note in VideoCapture::get()
	// set(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:807
	// ("cv::VideoCapture::set", vec![(pred!(mut, ["propId", "value"], ["int", "double"]), _)]),
	#[inline]
	fn set(&mut self, prop_id: i32, value: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_set_int_double(self.as_raw_mut_VideoCapture(), prop_id, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Open video file or a capturing device or a IP video stream for video capturing with API Preference
	///
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	///
	/// Parameters are same as the constructor VideoCapture(const String& filename, int apiPreference)
	/// ## Returns
	/// `true` if the file has been successfully opened
	///
	/// The method first calls VideoCapture::release to close the already opened file or camera.
	// open(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:837
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename", "apiPreference"], ["const cv::String*", "int"]), _)]),
	#[inline]
	fn open_file(&mut self, filename: &str, api_preference: i32) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_open_const_StringR_int(self.as_raw_mut_VideoCapture(), filename.opencv_as_extern(), api_preference, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class for video capturing from video files, image sequences or cameras.
///
/// The class provides C++ API for capturing video from cameras or for reading video files and image sequences.
///
/// Here is how the class can be used:
/// @include samples/cpp/videocapture_basic.cpp
///
///
/// Note: In [videoio_c] "C API" the black-box structure `CvCapture` is used instead of %VideoCapture.
///
/// Note:
/// *   (C++) A basic sample on using the %VideoCapture interface can be found at
///    `OPENCV_SOURCE_CODE/samples/cpp/videocapture_starter.cpp`
/// *   (Python) A basic sample on using the %VideoCapture interface can be found at
///    `OPENCV_SOURCE_CODE/samples/python/video.py`
/// *   (Python) A multi threaded video processing sample can be found at
///    `OPENCV_SOURCE_CODE/samples/python/video_threaded.py`
/// *   (Python) %VideoCapture sample showcasing some features of the Video4Linux2 backend
///    `OPENCV_SOURCE_CODE/samples/python/video_v4l2.py`
// VideoCapture /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:622
pub struct VideoCapture {
	ptr: *mut c_void,
}

opencv_type_boxed! { VideoCapture }

impl Drop for VideoCapture {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_VideoCapture_delete(self.as_raw_mut_VideoCapture()) };
	}
}

unsafe impl Send for VideoCapture {}

impl crate::videoio::VideoCaptureTraitConst for VideoCapture {
	#[inline] fn as_raw_VideoCapture(&self) -> *const c_void { self.as_raw() }
}

impl crate::videoio::VideoCaptureTrait for VideoCapture {
	#[inline] fn as_raw_mut_VideoCapture(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VideoCapture, crate::videoio::VideoCaptureTraitConst, as_raw_VideoCapture, crate::videoio::VideoCaptureTrait, as_raw_mut_VideoCapture }

impl VideoCapture {
	/// Default constructor
	///
	/// Note: In [videoio_c] "C API", when you finished working with video, release CvCapture structure with
	/// cvReleaseCapture(), or use Ptr\<CvCapture\> that calls cvReleaseCapture() automatically in the
	/// destructor.
	// VideoCapture()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:630
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videoio::VideoCapture> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_VideoCapture(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Default constructor
	///
	/// Note: In [videoio_c] "C API", when you finished working with video, release CvCapture structure with
	/// cvReleaseCapture(), or use Ptr\<CvCapture\> that calls cvReleaseCapture() automatically in the
	/// destructor.
	///
	/// ## Overloaded parameters
	///
	/// Open video file or image file sequence or a capturing device or a IP video stream for video capturing
	///
	/// Same as VideoCapture(const String& filename, int apiPreference) but using default Capture API backends
	// VideoCapture(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:637
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn from_file_default(filename: &str) -> Result<crate::videoio::VideoCapture> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_VideoCapture_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Default constructor
	///
	/// Note: In [videoio_c] "C API", when you finished working with video, release CvCapture structure with
	/// cvReleaseCapture(), or use Ptr\<CvCapture\> that calls cvReleaseCapture() automatically in the
	/// destructor.
	///
	/// ## Overloaded parameters
	///
	/// Open video file or a capturing device or a IP video stream for video capturing with API Preference
	///
	/// ## Parameters
	/// * filename: it can be:
	/// - name of video file (eg. `video.avi`)
	/// - or image sequence (eg. `img_%02d.jpg`, which will read samples like `img_00.jpg, img_01.jpg, img_02.jpg, ...`)
	/// - or URL of video stream (eg. `protocol://host:port/script_name?script_params|auth`)
	/// - or GStreamer pipeline string in gst-launch tool format in case if GStreamer is used as backend
	///   Note that each video stream or IP camera feed has its own URL scheme. Please refer to the
	///   documentation of source stream to know the right URL.
	/// * apiPreference: preferred Capture API backends to use. Can be used to enforce a specific reader
	/// implementation if multiple are available: e.g. cv::CAP_FFMPEG or cv::CAP_IMAGES or cv::CAP_DSHOW.
	/// ## See also
	/// cv::VideoCaptureAPIs
	// VideoCapture(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:654
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename", "apiPreference"], ["const cv::String*", "int"]), _)]),
	#[inline]
	pub fn from_file(filename: &str, api_preference: i32) -> Result<crate::videoio::VideoCapture> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_VideoCapture_const_StringR_int(filename.opencv_as_extern(), api_preference, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Default constructor
	///
	/// Note: In [videoio_c] "C API", when you finished working with video, release CvCapture structure with
	/// cvReleaseCapture(), or use Ptr\<CvCapture\> that calls cvReleaseCapture() automatically in the
	/// destructor.
	///
	/// ## Overloaded parameters
	///
	/// Open a camera for video capturing
	///
	/// ## Parameters
	/// * index: camera_id + domain_offset (CAP_*) id of the video capturing device to open. To open default camera using default backend just pass 0.
	/// Use a `domain_offset` to enforce a specific reader implementation if multiple are available like cv::CAP_FFMPEG or cv::CAP_IMAGES or cv::CAP_DSHOW.
	/// e.g. to open Camera 1 using the MS Media Foundation API use `index = 1 + cv::CAP_MSMF`
	/// ## See also
	/// cv::VideoCaptureAPIs
	// VideoCapture(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:665
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index"], ["int"]), _)]),
	#[inline]
	pub fn new_default(index: i32) -> Result<crate::videoio::VideoCapture> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_VideoCapture_int(index, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Default constructor
	///
	/// Note: In [videoio_c] "C API", when you finished working with video, release CvCapture structure with
	/// cvReleaseCapture(), or use Ptr\<CvCapture\> that calls cvReleaseCapture() automatically in the
	/// destructor.
	///
	/// ## Overloaded parameters
	///
	/// Opens a camera for video capturing
	///
	/// ## Parameters
	/// * index: id of the video capturing device to open. To open default camera using default backend just pass 0.
	/// (to backward compatibility usage of camera_id + domain_offset (CAP_*) is valid when apiPreference is CAP_ANY)
	/// * apiPreference: preferred Capture API backends to use. Can be used to enforce a specific reader
	/// implementation if multiple are available: e.g. cv::CAP_DSHOW or cv::CAP_MSMF or cv::CAP_V4L2.
	/// ## See also
	/// cv::VideoCaptureAPIs
	// VideoCapture(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:677
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index", "apiPreference"], ["int", "int"]), _)]),
	#[inline]
	pub fn new(index: i32, api_preference: i32) -> Result<crate::videoio::VideoCapture> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoCapture_VideoCapture_int_int(index, api_preference, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for VideoCapture {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("VideoCapture")
			.finish()
	}
}

/// Constant methods for [crate::videoio::VideoWriter]
// VideoWriter /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:864
pub trait VideoWriterTraitConst {
	fn as_raw_VideoWriter(&self) -> *const c_void;

	/// Returns true if video writer has been successfully initialized.
	// isOpened()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:933
	// ("cv::VideoWriter::isOpened", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_opened(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_isOpened_const(self.as_raw_VideoWriter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the specified VideoWriter property
	///
	/// ## Parameters
	/// * propId: Property identifier from cv::VideoWriterProperties (eg. cv::VIDEOWRITER_PROP_QUALITY)
	/// or one of [videoio_flags_others]
	///
	/// ## Returns
	/// Value for the specified property. Value 0 is returned when querying a property that is
	/// not supported by the backend used by the VideoWriter instance.
	// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:974
	// ("cv::VideoWriter::get", vec![(pred!(const, ["propId"], ["int"]), _)]),
	#[inline]
	fn get(&self, prop_id: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_get_const_int(self.as_raw_VideoWriter(), prop_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns used backend API name
	///
	///
	/// Note: Stream should be opened.
	// getBackendName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:989
	// ("cv::VideoWriter::getBackendName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_backend_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_getBackendName_const(self.as_raw_VideoWriter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::videoio::VideoWriter]
pub trait VideoWriterTrait: crate::videoio::VideoWriterTraitConst {
	fn as_raw_mut_VideoWriter(&mut self) -> *mut c_void;

	/// Initializes or reinitializes video writer.
	///
	/// The method opens video writer. Parameters are the same as in the constructor
	/// VideoWriter::VideoWriter.
	/// ## Returns
	/// `true` if video writer has been successfully initialized
	///
	/// The method first calls VideoWriter::release to close the already opened file.
	///
	/// ## C++ default parameters
	/// * is_color: true
	// open(const String &, int, double, Size, bool)(InString, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:923
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "double", "cv::Size", "bool"]), _)]),
	#[inline]
	fn open(&mut self, filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_open_const_StringR_int_double_Size_bool(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), fourcc, fps, &frame_size, is_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Initializes or reinitializes video writer.
	///
	/// The method opens video writer. Parameters are the same as in the constructor
	/// VideoWriter::VideoWriter.
	/// ## Returns
	/// `true` if video writer has been successfully initialized
	///
	/// The method first calls VideoWriter::release to close the already opened file.
	///
	/// ## Note
	/// This alternative version of [VideoWriterTrait::open] function uses the following default values for its arguments:
	/// * is_color: true
	// cv::VideoWriter::open(InString, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:923
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "double", "cv::Size"]), _)]),
	#[inline]
	fn open_def(&mut self, filename: &str, fourcc: i32, fps: f64, frame_size: core::Size) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_open_const_StringR_int_double_Size(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), fourcc, fps, &frame_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Initializes or reinitializes video writer.
	///
	/// The method opens video writer. Parameters are the same as in the constructor
	/// VideoWriter::VideoWriter.
	/// ## Returns
	/// `true` if video writer has been successfully initialized
	///
	/// The method first calls VideoWriter::release to close the already opened file.
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * is_color: true
	// open(const String &, int, int, double, Size, bool)(InString, Primitive, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:928
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "int", "double", "cv::Size", "bool"]), _)]),
	#[inline]
	fn open_with_backend(&mut self, filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_open_const_StringR_int_int_double_Size_bool(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), api_preference, fourcc, fps, &frame_size, is_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
	///
	/// ## Note
	/// This alternative version of [VideoWriterTrait::open_with_backend] function uses the following default values for its arguments:
	/// * is_color: true
	// cv::VideoWriter::open(InString, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:928
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "int", "double", "cv::Size"]), _)]),
	#[inline]
	fn open_with_backend_def(&mut self, filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_open_const_StringR_int_int_double_Size(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), api_preference, fourcc, fps, &frame_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Closes the video writer.
	///
	/// The method is automatically called by subsequent VideoWriter::open and by the VideoWriter
	/// destructor.
	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:940
	// ("cv::VideoWriter::release", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_release(self.as_raw_mut_VideoWriter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Writes the next video frame
	///
	/// ## Parameters
	/// * image: The written frame. In general, color images are expected in BGR format.
	///
	/// The function/method writes the specified image to video file. It must have the same size as has
	/// been specified when opening the video writer.
	// write(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:954
	// ("cv::VideoWriter::write", vec![(pred!(mut, ["image"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn write(&mut self, image: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_write_const_MatR(self.as_raw_mut_VideoWriter(), image.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets a property in the VideoWriter.
	///
	/// ## Parameters
	/// * propId: Property identifier from cv::VideoWriterProperties (eg. cv::VIDEOWRITER_PROP_QUALITY)
	/// or one of [videoio_flags_others]
	///
	/// * value: Value of the property.
	/// ## Returns
	/// `true` if the property is supported by the backend used by the VideoWriter instance.
	// set(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:964
	// ("cv::VideoWriter::set", vec![(pred!(mut, ["propId", "value"], ["int", "double"]), _)]),
	#[inline]
	fn set(&mut self, prop_id: i32, value: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_set_int_double(self.as_raw_mut_VideoWriter(), prop_id, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Video writer class.
///
/// The class provides C++ API for writing video files or image sequences.
// VideoWriter /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:864
pub struct VideoWriter {
	ptr: *mut c_void,
}

opencv_type_boxed! { VideoWriter }

impl Drop for VideoWriter {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_VideoWriter_delete(self.as_raw_mut_VideoWriter()) };
	}
}

unsafe impl Send for VideoWriter {}

impl crate::videoio::VideoWriterTraitConst for VideoWriter {
	#[inline] fn as_raw_VideoWriter(&self) -> *const c_void { self.as_raw() }
}

impl crate::videoio::VideoWriterTrait for VideoWriter {
	#[inline] fn as_raw_mut_VideoWriter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VideoWriter, crate::videoio::VideoWriterTraitConst, as_raw_VideoWriter, crate::videoio::VideoWriterTrait, as_raw_mut_VideoWriter }

impl VideoWriter {
	/// Default constructors
	///
	/// The constructors/functions initialize video writers.
	/// *   On Linux FFMPEG is used to write videos;
	/// *   On Windows FFMPEG or VFW is used;
	/// *   On MacOSX QTKit is used.
	// VideoWriter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:874
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videoio::VideoWriter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_VideoWriter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Default constructors
	///
	/// The constructors/functions initialize video writers.
	/// *   On Linux FFMPEG is used to write videos;
	/// *   On Windows FFMPEG or VFW is used;
	/// *   On MacOSX QTKit is used.
	///
	/// ## Overloaded parameters
	///
	/// ## Parameters
	/// * filename: Name of the output video file.
	/// * fourcc: 4-character code of codec used to compress the frames. For example,
	/// VideoWriter::fourcc('P','I','M','1') is a MPEG-1 codec, VideoWriter::fourcc('M','J','P','G')
	/// is a motion-jpeg codec etc. List of codes can be obtained at
	/// [MSDN](https://docs.microsoft.com/en-us/windows/win32/medfound/video-fourccs) page
	/// or with this [archived page](https://web.archive.org/web/20220316062600/http://www.fourcc.org/codecs.php)
	/// of the fourcc site for a more complete list). FFMPEG backend with MP4 container natively uses
	/// other values as fourcc code: see [ObjectType](http://mp4ra.org/#/codecs),
	/// so you may receive a warning message from OpenCV about fourcc code conversion.
	/// * fps: Framerate of the created video stream.
	/// * frameSize: Size of the video frames.
	/// * isColor: If it is not zero, the encoder will expect and encode color frames, otherwise it
	/// will work with grayscale frames (the flag is currently supported on Windows only).
	///
	/// @b Tips:
	/// - With some backends `fourcc=-1` pops up the codec selection dialog from the system.
	/// - To save image sequence use a proper filename (eg. `img_%02d.jpg`) and `fourcc=0`
	///   OR `fps=0`. Use uncompressed image format (eg. `img_%02d.BMP`) to save raw frames.
	/// - Most codecs are lossy. If you want lossless video file you need to use a lossless codecs
	///   (eg. FFMPEG FFV1, Huffman HFYU, Lagarith LAGS, etc...)
	/// - If FFMPEG is enabled, using `codec=0; fps=0;` you can create an uncompressed (raw) video file.
	///
	/// ## C++ default parameters
	/// * is_color: true
	// VideoWriter(const String &, int, double, Size, bool)(InString, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:899
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "double", "cv::Size", "bool"]), _)]),
	#[inline]
	pub fn new(filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<crate::videoio::VideoWriter> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_double_Size_bool(filename.opencv_as_extern(), fourcc, fps, &frame_size, is_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @overload
	/// ## Parameters
	/// * filename: Name of the output video file.
	/// * fourcc: 4-character code of codec used to compress the frames. For example,
	/// VideoWriter::fourcc('P','I','M','1') is a MPEG-1 codec, VideoWriter::fourcc('M','J','P','G')
	/// is a motion-jpeg codec etc. List of codes can be obtained at
	/// [MSDN](https://docs.microsoft.com/en-us/windows/win32/medfound/video-fourccs) page
	/// or with this [archived page](https://web.archive.org/web/20220316062600/http://www.fourcc.org/codecs.php)
	/// of the fourcc site for a more complete list). FFMPEG backend with MP4 container natively uses
	/// other values as fourcc code: see [ObjectType](http://mp4ra.org/#/codecs),
	/// so you may receive a warning message from OpenCV about fourcc code conversion.
	/// * fps: Framerate of the created video stream.
	/// * frameSize: Size of the video frames.
	/// * isColor: If it is not zero, the encoder will expect and encode color frames, otherwise it
	/// will work with grayscale frames (the flag is currently supported on Windows only).
	///
	/// @b Tips:
	/// - With some backends `fourcc=-1` pops up the codec selection dialog from the system.
	/// - To save image sequence use a proper filename (eg. `img_%02d.jpg`) and `fourcc=0`
	///   OR `fps=0`. Use uncompressed image format (eg. `img_%02d.BMP`) to save raw frames.
	/// - Most codecs are lossy. If you want lossless video file you need to use a lossless codecs
	///   (eg. FFMPEG FFV1, Huffman HFYU, Lagarith LAGS, etc...)
	/// - If FFMPEG is enabled, using `codec=0; fps=0;` you can create an uncompressed (raw) video file.
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * is_color: true
	// cv::VideoWriter::VideoWriter(InString, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:899
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "double", "cv::Size"]), _)]),
	#[inline]
	pub fn new_def(filename: &str, fourcc: i32, fps: f64, frame_size: core::Size) -> Result<crate::videoio::VideoWriter> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_double_Size(filename.opencv_as_extern(), fourcc, fps, &frame_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Default constructors
	///
	/// The constructors/functions initialize video writers.
	/// *   On Linux FFMPEG is used to write videos;
	/// *   On Windows FFMPEG or VFW is used;
	/// *   On MacOSX QTKit is used.
	///
	/// ## Overloaded parameters
	///
	/// The `apiPreference` parameter allows to specify API backends to use. Can be used to enforce a specific reader implementation
	/// if multiple are available: e.g. cv::CAP_FFMPEG or cv::CAP_GSTREAMER.
	///
	/// ## C++ default parameters
	/// * is_color: true
	// VideoWriter(const String &, int, int, double, Size, bool)(InString, Primitive, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:906
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "int", "double", "cv::Size", "bool"]), _)]),
	#[inline]
	pub fn new_with_backend(filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<crate::videoio::VideoWriter> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size_bool(filename.opencv_as_extern(), api_preference, fourcc, fps, &frame_size, is_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @overload
	/// The `apiPreference` parameter allows to specify API backends to use. Can be used to enforce a specific reader implementation
	/// if multiple are available: e.g. cv::CAP_FFMPEG or cv::CAP_GSTREAMER.
	///
	/// ## Note
	/// This alternative version of [new_with_backend] function uses the following default values for its arguments:
	/// * is_color: true
	// cv::VideoWriter::VideoWriter(InString, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:906
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "int", "double", "cv::Size"]), _)]),
	#[inline]
	pub fn new_with_backend_def(filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size) -> Result<crate::videoio::VideoWriter> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size(filename.opencv_as_extern(), api_preference, fourcc, fps, &frame_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Concatenates 4 chars to a fourcc code
	///
	/// ## Returns
	/// a fourcc code
	///
	/// This static method constructs the fourcc code of the codec to be used in the constructor
	/// VideoWriter::VideoWriter or VideoWriter::open.
	// fourcc(char, char, char, char)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:983
	// ("cv::VideoWriter::fourcc", vec![(pred!(mut, ["c1", "c2", "c3", "c4"], ["char", "char", "char", "char"]), _)]),
	#[inline]
	pub fn fourcc(c1: char, c2: char, c3: char, c4: char) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VideoWriter_fourcc_char_char_char_char(u8::try_from(c1)? as c_char, u8::try_from(c2)? as c_char, u8::try_from(c3)? as c_char, u8::try_from(c4)? as c_char, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

impl std::fmt::Debug for VideoWriter {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("VideoWriter")
			.finish()
	}
}

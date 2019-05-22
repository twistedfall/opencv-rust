//! # Video I/O
//! 
//!  Read and write video or images sequence with OpenCV
//! 
//! ### See also:
//! - @ref videoio_overview
//! - Tutorials: @ref tutorial_table_of_content_videoio
//! # Flags for video I/O
//! # Additional flags for video I/O API backends
//! # C API for video I/O
//! # iOS glue for video I/O
//! # WinRT glue for video I/O
//! # Query I/O API backends registry
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const CAP_ANDROID: i32 = 1000;
pub const CAP_ANY: i32 = 0;
pub const CAP_ARAVIS: i32 = 2100;
pub const CAP_AVFOUNDATION: i32 = 1200;
pub const CAP_CMU1394: i32 = 300;
pub const CAP_DC1394: i32 = 300;
pub const CAP_DSHOW: i32 = 700;
pub const CAP_FFMPEG: i32 = 1900;
pub const CAP_FIREWARE: i32 = 300;
pub const CAP_FIREWIRE: i32 = 300;
pub const CAP_GIGANETIX: i32 = 1300;
pub const CAP_GPHOTO2: i32 = 1700;
pub const CAP_GSTREAMER: i32 = 1800;
pub const CAP_IEEE1394: i32 = 300;
pub const CAP_IMAGES: i32 = 2000;
pub const CAP_INTELPERC: i32 = 1500;
pub const CAP_INTELPERC_DEPTH_GENERATOR: i32 = 1 << 29;
pub const CAP_INTELPERC_DEPTH_MAP: i32 = 0;
pub const CAP_INTELPERC_IMAGE: i32 = 3;
pub const CAP_INTELPERC_IMAGE_GENERATOR: i32 = 1 << 28;
pub const CAP_INTELPERC_IR_MAP: i32 = 2;
pub const CAP_INTELPERC_UVDEPTH_MAP: i32 = 1;
pub const CAP_INTEL_MFX: i32 = 2300;
pub const CAP_MODE_BGR: i32 = 0;
pub const CAP_MODE_GRAY: i32 = 2;
pub const CAP_MODE_RGB: i32 = 1;
pub const CAP_MODE_YUYV: i32 = 3;
pub const CAP_MSMF: i32 = 1400;
pub const CAP_OPENCV_MJPEG: i32 = 2200;
pub const CAP_OPENNI: i32 = 900;
pub const CAP_OPENNI2: i32 = 1600;
pub const CAP_OPENNI2_ASUS: i32 = 1610;
pub const CAP_OPENNI_ASUS: i32 = 910;
pub const CAP_OPENNI_BGR_IMAGE: i32 = 5;
pub const CAP_OPENNI_DEPTH_GENERATOR: i32 = 1 << 31;
pub const CAP_OPENNI_DEPTH_MAP: i32 = 0;
pub const CAP_OPENNI_DISPARITY_MAP: i32 = 2;
pub const CAP_OPENNI_DISPARITY_MAP_32F: i32 = 3;
pub const CAP_OPENNI_GRAY_IMAGE: i32 = 6;
pub const CAP_OPENNI_IMAGE_GENERATOR: i32 = 1 << 30;
pub const CAP_OPENNI_IR_GENERATOR: i32 = 1 << 29;
pub const CAP_OPENNI_IR_IMAGE: i32 = 7;
pub const CAP_OPENNI_POINT_CLOUD_MAP: i32 = 1;
pub const CAP_OPENNI_QVGA_30HZ: i32 = 3;
pub const CAP_OPENNI_QVGA_60HZ: i32 = 4;
pub const CAP_OPENNI_SXGA_15HZ: i32 = 1;
pub const CAP_OPENNI_SXGA_30HZ: i32 = 2;
pub const CAP_OPENNI_VALID_DEPTH_MASK: i32 = 4;
pub const CAP_OPENNI_VGA_30HZ: i32 = 0;
pub const CAP_PROP_APERTURE: i32 = 17008;
pub const CAP_PROP_AUTOFOCUS: i32 = 39;
pub const CAP_PROP_AUTO_EXPOSURE: i32 = 21;
pub const CAP_PROP_AUTO_WB: i32 = 44;
pub const CAP_PROP_BACKEND: i32 = 42;
pub const CAP_PROP_BACKLIGHT: i32 = 32;
pub const CAP_PROP_BRIGHTNESS: i32 = 10;
pub const CAP_PROP_BUFFERSIZE: i32 = 38;
pub const CAP_PROP_CHANNEL: i32 = 43;
pub const CAP_PROP_CONTRAST: i32 = 11;
pub const CAP_PROP_CONVERT_RGB: i32 = 16;
pub const CAP_PROP_DC1394_MAX: i32 = 31;
pub const CAP_PROP_DC1394_MODE_AUTO: i32 = -2;
pub const CAP_PROP_DC1394_MODE_MANUAL: i32 = -3;
pub const CAP_PROP_DC1394_MODE_ONE_PUSH_AUTO: i32 = -1;
pub const CAP_PROP_DC1394_OFF: i32 = -4;
pub const CAP_PROP_EXPOSURE: i32 = 15;
pub const CAP_PROP_EXPOSUREPROGRAM: i32 = 17009;
pub const CAP_PROP_FOCUS: i32 = 28;
pub const CAP_PROP_FORMAT: i32 = 8;
pub const CAP_PROP_FOURCC: i32 = 6;
pub const CAP_PROP_FPS: i32 = 5;
pub const CAP_PROP_FRAME_COUNT: i32 = 7;
pub const CAP_PROP_FRAME_HEIGHT: i32 = 4;
pub const CAP_PROP_FRAME_WIDTH: i32 = 3;
pub const CAP_PROP_GAIN: i32 = 14;
pub const CAP_PROP_GAMMA: i32 = 22;
pub const CAP_PROP_GIGA_FRAME_HEIGH_MAX: i32 = 10004;
pub const CAP_PROP_GIGA_FRAME_OFFSET_X: i32 = 10001;
pub const CAP_PROP_GIGA_FRAME_OFFSET_Y: i32 = 10002;
pub const CAP_PROP_GIGA_FRAME_SENS_HEIGH: i32 = 10006;
pub const CAP_PROP_GIGA_FRAME_SENS_WIDTH: i32 = 10005;
pub const CAP_PROP_GIGA_FRAME_WIDTH_MAX: i32 = 10003;
pub const CAP_PROP_GPHOTO2_COLLECT_MSGS: i32 = 17005;
pub const CAP_PROP_GPHOTO2_FLUSH_MSGS: i32 = 17006;
pub const CAP_PROP_GPHOTO2_PREVIEW: i32 = 17001;
pub const CAP_PROP_GPHOTO2_RELOAD_CONFIG: i32 = 17003;
pub const CAP_PROP_GPHOTO2_RELOAD_ON_CHANGE: i32 = 17004;
pub const CAP_PROP_GPHOTO2_WIDGET_ENUMERATE: i32 = 17002;
pub const CAP_PROP_GSTREAMER_QUEUE_LENGTH: i32 = 200;
pub const CAP_PROP_GUID: i32 = 29;
pub const CAP_PROP_HUE: i32 = 13;
pub const CAP_PROP_IMAGES_BASE: i32 = 18000;
pub const CAP_PROP_IMAGES_LAST: i32 = 19000;
pub const CAP_PROP_INTELPERC_DEPTH_CONFIDENCE_THRESHOLD: i32 = 11005;
pub const CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_HORZ: i32 = 11006;
pub const CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_VERT: i32 = 11007;
pub const CAP_PROP_INTELPERC_DEPTH_LOW_CONFIDENCE_VALUE: i32 = 11003;
pub const CAP_PROP_INTELPERC_DEPTH_SATURATION_VALUE: i32 = 11004;
pub const CAP_PROP_INTELPERC_PROFILE_COUNT: i32 = 11001;
pub const CAP_PROP_INTELPERC_PROFILE_IDX: i32 = 11002;
pub const CAP_PROP_IOS_DEVICE_EXPOSURE: i32 = 9002;
pub const CAP_PROP_IOS_DEVICE_FLASH: i32 = 9003;
pub const CAP_PROP_IOS_DEVICE_FOCUS: i32 = 9001;
pub const CAP_PROP_IOS_DEVICE_TORCH: i32 = 9005;
pub const CAP_PROP_IOS_DEVICE_WHITEBALANCE: i32 = 9004;
pub const CAP_PROP_IRIS: i32 = 36;
pub const CAP_PROP_ISO_SPEED: i32 = 30;
pub const CAP_PROP_MODE: i32 = 9;
pub const CAP_PROP_MONOCHROME: i32 = 19;
pub const CAP_PROP_OPENNI2_MIRROR: i32 = 111;
pub const CAP_PROP_OPENNI2_SYNC: i32 = 110;
pub const CAP_PROP_OPENNI_APPROX_FRAME_SYNC: i32 = 105;
pub const CAP_PROP_OPENNI_BASELINE: i32 = 102;
pub const CAP_PROP_OPENNI_CIRCLE_BUFFER: i32 = 107;
pub const CAP_PROP_OPENNI_FOCAL_LENGTH: i32 = 103;
pub const CAP_PROP_OPENNI_FRAME_MAX_DEPTH: i32 = 101;
pub const CAP_PROP_OPENNI_GENERATOR_PRESENT: i32 = 109;
pub const CAP_PROP_OPENNI_MAX_BUFFER_SIZE: i32 = 106;
pub const CAP_PROP_OPENNI_MAX_TIME_DURATION: i32 = 108;
pub const CAP_PROP_OPENNI_OUTPUT_MODE: i32 = 100;
pub const CAP_PROP_OPENNI_REGISTRATION: i32 = 104;
pub const CAP_PROP_OPENNI_REGISTRATION_ON: i32 = 104;
pub const CAP_PROP_PAN: i32 = 33;
pub const CAP_PROP_POS_AVI_RATIO: i32 = 2;
pub const CAP_PROP_POS_FRAMES: i32 = 1;
pub const CAP_PROP_POS_MSEC: i32 = 0;
pub const CAP_PROP_PVAPI_BINNINGX: i32 = 304;
pub const CAP_PROP_PVAPI_BINNINGY: i32 = 305;
pub const CAP_PROP_PVAPI_DECIMATIONHORIZONTAL: i32 = 302;
pub const CAP_PROP_PVAPI_DECIMATIONVERTICAL: i32 = 303;
pub const CAP_PROP_PVAPI_FRAMESTARTTRIGGERMODE: i32 = 301;
pub const CAP_PROP_PVAPI_MULTICASTIP: i32 = 300;
pub const CAP_PROP_PVAPI_PIXELFORMAT: i32 = 306;
pub const CAP_PROP_RECTIFICATION: i32 = 18;
pub const CAP_PROP_ROLL: i32 = 35;
pub const CAP_PROP_SAR_DEN: i32 = 41;
pub const CAP_PROP_SAR_NUM: i32 = 40;
pub const CAP_PROP_SATURATION: i32 = 12;
pub const CAP_PROP_SETTINGS: i32 = 37;
pub const CAP_PROP_SHARPNESS: i32 = 20;
pub const CAP_PROP_SPEED: i32 = 17007;
pub const CAP_PROP_TEMPERATURE: i32 = 23;
pub const CAP_PROP_TILT: i32 = 34;
pub const CAP_PROP_TRIGGER: i32 = 24;
pub const CAP_PROP_TRIGGER_DELAY: i32 = 25;
pub const CAP_PROP_VIEWFINDER: i32 = 17010;
pub const CAP_PROP_WB_TEMPERATURE: i32 = 45;
pub const CAP_PROP_WHITE_BALANCE_BLUE_U: i32 = 17;
pub const CAP_PROP_WHITE_BALANCE_RED_V: i32 = 26;
pub const CAP_PROP_XI_ACQ_BUFFER_SIZE: i32 = 548;
pub const CAP_PROP_XI_ACQ_BUFFER_SIZE_UNIT: i32 = 549;
pub const CAP_PROP_XI_ACQ_FRAME_BURST_COUNT: i32 = 499;
pub const CAP_PROP_XI_ACQ_TIMING_MODE: i32 = 538;
pub const CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_COMMIT: i32 = 552;
pub const CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_SIZE: i32 = 550;
pub const CAP_PROP_XI_AEAG: i32 = 415;
pub const CAP_PROP_XI_AEAG_LEVEL: i32 = 419;
pub const CAP_PROP_XI_AEAG_ROI_HEIGHT: i32 = 442;
pub const CAP_PROP_XI_AEAG_ROI_OFFSET_X: i32 = 439;
pub const CAP_PROP_XI_AEAG_ROI_OFFSET_Y: i32 = 440;
pub const CAP_PROP_XI_AEAG_ROI_WIDTH: i32 = 441;
pub const CAP_PROP_XI_AE_MAX_LIMIT: i32 = 417;
pub const CAP_PROP_XI_AG_MAX_LIMIT: i32 = 418;
pub const CAP_PROP_XI_APPLY_CMS: i32 = 471;
pub const CAP_PROP_XI_AUTO_BANDWIDTH_CALCULATION: i32 = 573;
pub const CAP_PROP_XI_AUTO_WB: i32 = 414;
pub const CAP_PROP_XI_AVAILABLE_BANDWIDTH: i32 = 539;
pub const CAP_PROP_XI_BINNING_HORIZONTAL: i32 = 429;
pub const CAP_PROP_XI_BINNING_PATTERN: i32 = 430;
pub const CAP_PROP_XI_BINNING_SELECTOR: i32 = 427;
pub const CAP_PROP_XI_BINNING_VERTICAL: i32 = 428;
pub const CAP_PROP_XI_BPC: i32 = 445;
pub const CAP_PROP_XI_BUFFERS_QUEUE_SIZE: i32 = 551;
pub const CAP_PROP_XI_BUFFER_POLICY: i32 = 540;
pub const CAP_PROP_XI_CC_MATRIX_00: i32 = 479;
pub const CAP_PROP_XI_CC_MATRIX_01: i32 = 480;
pub const CAP_PROP_XI_CC_MATRIX_02: i32 = 481;
pub const CAP_PROP_XI_CC_MATRIX_03: i32 = 482;
pub const CAP_PROP_XI_CC_MATRIX_10: i32 = 483;
pub const CAP_PROP_XI_CC_MATRIX_11: i32 = 484;
pub const CAP_PROP_XI_CC_MATRIX_12: i32 = 485;
pub const CAP_PROP_XI_CC_MATRIX_13: i32 = 486;
pub const CAP_PROP_XI_CC_MATRIX_20: i32 = 487;
pub const CAP_PROP_XI_CC_MATRIX_21: i32 = 488;
pub const CAP_PROP_XI_CC_MATRIX_22: i32 = 489;
pub const CAP_PROP_XI_CC_MATRIX_23: i32 = 490;
pub const CAP_PROP_XI_CC_MATRIX_30: i32 = 491;
pub const CAP_PROP_XI_CC_MATRIX_31: i32 = 492;
pub const CAP_PROP_XI_CC_MATRIX_32: i32 = 493;
pub const CAP_PROP_XI_CC_MATRIX_33: i32 = 494;
pub const CAP_PROP_XI_CHIP_TEMP: i32 = 468;
pub const CAP_PROP_XI_CMS: i32 = 470;
pub const CAP_PROP_XI_COLOR_FILTER_ARRAY: i32 = 475;
pub const CAP_PROP_XI_COLUMN_FPN_CORRECTION: i32 = 555;
pub const CAP_PROP_XI_COOLING: i32 = 466;
pub const CAP_PROP_XI_COUNTER_SELECTOR: i32 = 536;
pub const CAP_PROP_XI_COUNTER_VALUE: i32 = 537;
pub const CAP_PROP_XI_DATA_FORMAT: i32 = 401;
pub const CAP_PROP_XI_DEBOUNCE_EN: i32 = 507;
pub const CAP_PROP_XI_DEBOUNCE_POL: i32 = 510;
pub const CAP_PROP_XI_DEBOUNCE_T0: i32 = 508;
pub const CAP_PROP_XI_DEBOUNCE_T1: i32 = 509;
pub const CAP_PROP_XI_DEBUG_LEVEL: i32 = 572;
pub const CAP_PROP_XI_DECIMATION_HORIZONTAL: i32 = 433;
pub const CAP_PROP_XI_DECIMATION_PATTERN: i32 = 434;
pub const CAP_PROP_XI_DECIMATION_SELECTOR: i32 = 431;
pub const CAP_PROP_XI_DECIMATION_VERTICAL: i32 = 432;
pub const CAP_PROP_XI_DEFAULT_CC_MATRIX: i32 = 495;
pub const CAP_PROP_XI_DEVICE_MODEL_ID: i32 = 521;
pub const CAP_PROP_XI_DEVICE_RESET: i32 = 554;
pub const CAP_PROP_XI_DEVICE_SN: i32 = 522;
pub const CAP_PROP_XI_DOWNSAMPLING: i32 = 400;
pub const CAP_PROP_XI_DOWNSAMPLING_TYPE: i32 = 426;
pub const CAP_PROP_XI_EXPOSURE: i32 = 421;
pub const CAP_PROP_XI_EXPOSURE_BURST_COUNT: i32 = 422;
pub const CAP_PROP_XI_EXP_PRIORITY: i32 = 416;
pub const CAP_PROP_XI_FFS_ACCESS_KEY: i32 = 583;
pub const CAP_PROP_XI_FFS_FILE_ID: i32 = 594;
pub const CAP_PROP_XI_FFS_FILE_SIZE: i32 = 580;
pub const CAP_PROP_XI_FRAMERATE: i32 = 535;
pub const CAP_PROP_XI_FREE_FFS_SIZE: i32 = 581;
pub const CAP_PROP_XI_GAIN: i32 = 424;
pub const CAP_PROP_XI_GAIN_SELECTOR: i32 = 423;
pub const CAP_PROP_XI_GAMMAC: i32 = 477;
pub const CAP_PROP_XI_GAMMAY: i32 = 476;
pub const CAP_PROP_XI_GPI_LEVEL: i32 = 408;
pub const CAP_PROP_XI_GPI_MODE: i32 = 407;
pub const CAP_PROP_XI_GPI_SELECTOR: i32 = 406;
pub const CAP_PROP_XI_GPO_MODE: i32 = 410;
pub const CAP_PROP_XI_GPO_SELECTOR: i32 = 409;
pub const CAP_PROP_XI_HDR: i32 = 559;
pub const CAP_PROP_XI_HDR_KNEEPOINT_COUNT: i32 = 560;
pub const CAP_PROP_XI_HDR_T1: i32 = 561;
pub const CAP_PROP_XI_HDR_T2: i32 = 562;
pub const CAP_PROP_XI_HEIGHT: i32 = 452;
pub const CAP_PROP_XI_HOUS_BACK_SIDE_TEMP: i32 = 590;
pub const CAP_PROP_XI_HOUS_TEMP: i32 = 469;
pub const CAP_PROP_XI_HW_REVISION: i32 = 571;
pub const CAP_PROP_XI_IMAGE_BLACK_LEVEL: i32 = 565;
pub const CAP_PROP_XI_IMAGE_DATA_BIT_DEPTH: i32 = 462;
pub const CAP_PROP_XI_IMAGE_DATA_FORMAT: i32 = 435;
pub const CAP_PROP_XI_IMAGE_DATA_FORMAT_RGB32_ALPHA: i32 = 529;
pub const CAP_PROP_XI_IMAGE_IS_COLOR: i32 = 474;
pub const CAP_PROP_XI_IMAGE_PAYLOAD_SIZE: i32 = 530;
pub const CAP_PROP_XI_IS_COOLED: i32 = 465;
pub const CAP_PROP_XI_IS_DEVICE_EXIST: i32 = 547;
pub const CAP_PROP_XI_KNEEPOINT1: i32 = 563;
pub const CAP_PROP_XI_KNEEPOINT2: i32 = 564;
pub const CAP_PROP_XI_LED_MODE: i32 = 412;
pub const CAP_PROP_XI_LED_SELECTOR: i32 = 411;
pub const CAP_PROP_XI_LENS_APERTURE_VALUE: i32 = 512;
pub const CAP_PROP_XI_LENS_FEATURE: i32 = 518;
pub const CAP_PROP_XI_LENS_FEATURE_SELECTOR: i32 = 517;
pub const CAP_PROP_XI_LENS_FOCAL_LENGTH: i32 = 516;
pub const CAP_PROP_XI_LENS_FOCUS_DISTANCE: i32 = 515;
pub const CAP_PROP_XI_LENS_FOCUS_MOVE: i32 = 514;
pub const CAP_PROP_XI_LENS_FOCUS_MOVEMENT_VALUE: i32 = 513;
pub const CAP_PROP_XI_LENS_MODE: i32 = 511;
pub const CAP_PROP_XI_LIMIT_BANDWIDTH: i32 = 459;
pub const CAP_PROP_XI_LUT_EN: i32 = 541;
pub const CAP_PROP_XI_LUT_INDEX: i32 = 542;
pub const CAP_PROP_XI_LUT_VALUE: i32 = 543;
pub const CAP_PROP_XI_MANUAL_WB: i32 = 413;
pub const CAP_PROP_XI_OFFSET_X: i32 = 402;
pub const CAP_PROP_XI_OFFSET_Y: i32 = 403;
pub const CAP_PROP_XI_OUTPUT_DATA_BIT_DEPTH: i32 = 461;
pub const CAP_PROP_XI_OUTPUT_DATA_PACKING: i32 = 463;
pub const CAP_PROP_XI_OUTPUT_DATA_PACKING_TYPE: i32 = 464;
pub const CAP_PROP_XI_RECENT_FRAME: i32 = 553;
pub const CAP_PROP_XI_REGION_MODE: i32 = 595;
pub const CAP_PROP_XI_REGION_SELECTOR: i32 = 589;
pub const CAP_PROP_XI_ROW_FPN_CORRECTION: i32 = 591;
pub const CAP_PROP_XI_SENSOR_BOARD_TEMP: i32 = 596;
pub const CAP_PROP_XI_SENSOR_CLOCK_FREQ_HZ: i32 = 532;
pub const CAP_PROP_XI_SENSOR_CLOCK_FREQ_INDEX: i32 = 533;
pub const CAP_PROP_XI_SENSOR_DATA_BIT_DEPTH: i32 = 460;
pub const CAP_PROP_XI_SENSOR_FEATURE_SELECTOR: i32 = 585;
pub const CAP_PROP_XI_SENSOR_FEATURE_VALUE: i32 = 586;
pub const CAP_PROP_XI_SENSOR_MODE: i32 = 558;
pub const CAP_PROP_XI_SENSOR_OUTPUT_CHANNEL_COUNT: i32 = 534;
pub const CAP_PROP_XI_SENSOR_TAPS: i32 = 437;
pub const CAP_PROP_XI_SHARPNESS: i32 = 478;
pub const CAP_PROP_XI_SHUTTER_TYPE: i32 = 436;
pub const CAP_PROP_XI_TARGET_TEMP: i32 = 467;
pub const CAP_PROP_XI_TEST_PATTERN: i32 = 588;
pub const CAP_PROP_XI_TEST_PATTERN_GENERATOR_SELECTOR: i32 = 587;
pub const CAP_PROP_XI_TIMEOUT: i32 = 420;
pub const CAP_PROP_XI_TRANSPORT_PIXEL_FORMAT: i32 = 531;
pub const CAP_PROP_XI_TRG_DELAY: i32 = 544;
pub const CAP_PROP_XI_TRG_SELECTOR: i32 = 498;
pub const CAP_PROP_XI_TRG_SOFTWARE: i32 = 405;
pub const CAP_PROP_XI_TRG_SOURCE: i32 = 404;
pub const CAP_PROP_XI_TS_RST_MODE: i32 = 545;
pub const CAP_PROP_XI_TS_RST_SOURCE: i32 = 546;
pub const CAP_PROP_XI_USED_FFS_SIZE: i32 = 582;
pub const CAP_PROP_XI_WB_KB: i32 = 450;
pub const CAP_PROP_XI_WB_KG: i32 = 449;
pub const CAP_PROP_XI_WB_KR: i32 = 448;
pub const CAP_PROP_XI_WIDTH: i32 = 451;
pub const CAP_PROP_ZOOM: i32 = 27;
pub const CAP_PVAPI: i32 = 800;
pub const CAP_PVAPI_DECIMATION_2OUTOF16: i32 = 8;
pub const CAP_PVAPI_DECIMATION_2OUTOF4: i32 = 2;
pub const CAP_PVAPI_DECIMATION_2OUTOF8: i32 = 4;
pub const CAP_PVAPI_DECIMATION_OFF: i32 = 1;
pub const CAP_PVAPI_FSTRIGMODE_FIXEDRATE: i32 = 3;
pub const CAP_PVAPI_FSTRIGMODE_FREERUN: i32 = 0;
pub const CAP_PVAPI_FSTRIGMODE_SOFTWARE: i32 = 4;
pub const CAP_PVAPI_FSTRIGMODE_SYNCIN1: i32 = 1;
pub const CAP_PVAPI_FSTRIGMODE_SYNCIN2: i32 = 2;
pub const CAP_PVAPI_PIXELFORMAT_BAYER16: i32 = 4;
pub const CAP_PVAPI_PIXELFORMAT_BAYER8: i32 = 3;
pub const CAP_PVAPI_PIXELFORMAT_BGR24: i32 = 6;
pub const CAP_PVAPI_PIXELFORMAT_BGRA32: i32 = 8;
pub const CAP_PVAPI_PIXELFORMAT_MONO16: i32 = 2;
pub const CAP_PVAPI_PIXELFORMAT_MONO8: i32 = 1;
pub const CAP_PVAPI_PIXELFORMAT_RGB24: i32 = 5;
pub const CAP_PVAPI_PIXELFORMAT_RGBA32: i32 = 7;
pub const CAP_QT: i32 = 500;
pub const CAP_UNICAP: i32 = 600;
pub const CAP_V4L: i32 = 200;
pub const CAP_V4L2: i32 = 200;
pub const CAP_VFW: i32 = 200;
pub const CAP_WINRT: i32 = 1410;
pub const CAP_XIAPI: i32 = 1100;
pub const CAP_XINE: i32 = 2400;
pub const VIDEOWRITER_PROP_FRAMEBYTES: i32 = 2;
pub const VIDEOWRITER_PROP_NSTRIPES: i32 = 3;
pub const VIDEOWRITER_PROP_QUALITY: i32 = 1;

// boxed class cv::VideoCapture
/// Class for video capturing from video files, image sequences or cameras.
/// 
/// The class provides C++ API for capturing video from cameras or for reading video files and image sequences.
/// 
/// Here is how the class can be used:
/// @include samples/cpp/videocapture_basic.cpp
/// 
/// 
/// Note: In @ref videoio_c "C API" the black-box structure `CvCapture` is used instead of %VideoCapture.
/// 
/// Note:
/// *   (C++) A basic sample on using the %VideoCapture interface can be found at
/// `OPENCV_SOURCE_CODE/samples/cpp/videocapture_starter.cpp`
/// *   (Python) A basic sample on using the %VideoCapture interface can be found at
/// `OPENCV_SOURCE_CODE/samples/python/video.py`
/// *   (Python) A multi threaded video processing sample can be found at
/// `OPENCV_SOURCE_CODE/samples/python/video_threaded.py`
/// *   (Python) %VideoCapture sample showcasing some features of the Video4Linux2 backend
/// `OPENCV_SOURCE_CODE/samples/python/video_v4l2.py`
#[allow(dead_code)]
pub struct VideoCapture {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::videoio::VideoCapture {
    fn drop(&mut self) {
        unsafe { sys::cv_VideoCapture_delete(self.ptr) };
    }
}
impl crate::videoio::VideoCapture {
    #[doc(hidden)] pub fn as_raw_VideoCapture(&self) -> *mut c_void { self.ptr }
}

impl VideoCapture {

    /// Default constructor
    /// 
    /// Note: In @ref videoio_c "C API", when you finished working with video, release CvCapture structure with
    /// cvReleaseCapture(), or use Ptr\<CvCapture\> that calls cvReleaseCapture() automatically in the
    /// destructor.
    pub fn default() -> Result<crate::videoio::VideoCapture> {
        unsafe { sys::cv_VideoCapture_VideoCapture() }.into_result().map(|x| crate::videoio::VideoCapture { ptr: x })
    }
    
    /// Open video file or a capturing device or a IP video stream for video capturing
    /// 
    /// Same as VideoCapture(const String& filename, int apiPreference) but using default Capture API backends
    pub fn new_from_file(filename: &str) -> Result<crate::videoio::VideoCapture> {
        string_arg!(filename);
        unsafe { sys::cv_VideoCapture_VideoCapture_String(filename.as_ptr()) }.into_result().map(|x| crate::videoio::VideoCapture { ptr: x })
    }
    
    /// Open video file or a capturing device or a IP video stream for video capturing with API Preference
    /// 
    /// ## Parameters
    /// * filename: it can be:
    /// - name of video file (eg. `video.avi`)
    /// - or image sequence (eg. `img_%02d.jpg`, which will read samples like `img_00.jpg, img_01.jpg, img_02.jpg, ...`)
    /// - or URL of video stream (eg. `protocol://host:port/script_name?script_params|auth`).
    /// Note that each video stream or IP camera feed has its own URL scheme. Please refer to the
    /// documentation of source stream to know the right URL.
    /// * apiPreference: preferred Capture API backends to use. Can be used to enforce a specific reader
    /// implementation if multiple are available: e.g. cv::CAP_FFMPEG or cv::CAP_IMAGES or cv::CAP_DSHOW.
    /// ## See also
    /// The list of supported API backends cv::VideoCaptureAPIs
    pub fn new_from_file_with_backend(filename: &str, api_preference: i32) -> Result<crate::videoio::VideoCapture> {
        string_arg!(filename);
        unsafe { sys::cv_VideoCapture_VideoCapture_String_int(filename.as_ptr(), api_preference) }.into_result().map(|x| crate::videoio::VideoCapture { ptr: x })
    }
    
    /// Open a camera for video capturing
    /// 
    /// ## Parameters
    /// * index: camera_id + domain_offset (CAP_*) id of the video capturing device to open. To open default camera using default backend just pass 0.
    /// Use a `domain_offset` to enforce a specific reader implementation if multiple are available like cv::CAP_FFMPEG or cv::CAP_IMAGES or cv::CAP_DSHOW.
    /// e.g. to open Camera 1 using the MS Media Foundation API use `index = 1 + cv::CAP_MSMF`
    /// 
    /// ## See also
    /// The list of supported API backends cv::VideoCaptureAPIs
    pub fn new(index: i32) -> Result<crate::videoio::VideoCapture> {
        unsafe { sys::cv_VideoCapture_VideoCapture_int(index) }.into_result().map(|x| crate::videoio::VideoCapture { ptr: x })
    }
    
    /// Opens a camera for video capturing
    /// 
    /// ## Parameters
    /// * index: id of the video capturing device to open. To open default camera using default backend just pass 0.
    /// (to backward compatibility usage of camera_id + domain_offset (CAP_*) is valid when apiPreference is CAP_ANY)
    /// * apiPreference: preferred Capture API backends to use. Can be used to enforce a specific reader
    /// implementation if multiple are available: e.g. cv::CAP_DSHOW or cv::CAP_MSMF or cv::CAP_V4L2.
    /// 
    /// ## See also
    /// The list of supported API backends cv::VideoCaptureAPIs
    pub fn new_with_backend(index: i32, api_preference: i32) -> Result<crate::videoio::VideoCapture> {
        unsafe { sys::cv_VideoCapture_VideoCapture_int_int(index, api_preference) }.into_result().map(|x| crate::videoio::VideoCapture { ptr: x })
    }
    
    /// Open video file or a capturing device or a IP video stream for video capturing
    /// 
    /// 
    /// 
    /// Parameters are same as the constructor VideoCapture(const String& filename)
    /// ## Returns
    /// `true` if the file has been successfully opened
    /// 
    /// The method first calls VideoCapture::release to close the already opened file or camera.
    pub fn open_file(&mut self, filename: &str) -> Result<bool> {
        string_arg!(filename);
        unsafe { sys::cv_VideoCapture_open_String(self.as_raw_VideoCapture(), filename.as_ptr()) }.into_result()
    }
    
    /// Open a camera for video capturing
    /// 
    /// 
    /// 
    /// Parameters are same as the constructor VideoCapture(int index)
    /// ## Returns
    /// `true` if the camera has been successfully opened.
    /// 
    /// The method first calls VideoCapture::release to close the already opened file or camera.
    pub fn open(&mut self, index: i32) -> Result<bool> {
        unsafe { sys::cv_VideoCapture_open_int(self.as_raw_VideoCapture(), index) }.into_result()
    }
    
    /// Open a camera for video capturing
    /// 
    /// 
    /// 
    /// Parameters are similar as the constructor VideoCapture(int index),except it takes an additional argument apiPreference.
    /// Definitely, is same as open(int index) where `index=cameraNum + apiPreference`
    /// ## Returns
    /// `true` if the camera has been successfully opened.
    pub fn open_with_backend(&mut self, camera_num: i32, api_preference: i32) -> Result<bool> {
        unsafe { sys::cv_VideoCapture_open_int_int(self.as_raw_VideoCapture(), camera_num, api_preference) }.into_result()
    }
    
    /// Returns true if video capturing has been initialized already.
    /// 
    /// If the previous call to VideoCapture constructor or VideoCapture::open() succeeded, the method returns
    /// true.
    pub fn is_opened(&self) -> Result<bool> {
        unsafe { sys::cv_VideoCapture_isOpened_const(self.as_raw_VideoCapture()) }.into_result()
    }
    
    /// Closes video file or capturing device.
    /// 
    /// The method is automatically called by subsequent VideoCapture::open and by VideoCapture
    /// destructor.
    /// 
    /// The C function also deallocates memory and clears \*capture pointer.
    pub fn release(&mut self) -> Result<()> {
        unsafe { sys::cv_VideoCapture_release(self.as_raw_VideoCapture()) }.into_result()
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
    /// @ref tutorial_kinect_openni
    pub fn grab(&mut self) -> Result<bool> {
        unsafe { sys::cv_VideoCapture_grab(self.as_raw_VideoCapture()) }.into_result()
    }
    
    /// Decodes and returns the grabbed video frame.
    /// 
    /// ## Parameters
    /// * image: [out] the video frame is returned here. If no frames has been grabbed the image will be empty.
    /// * flag: it could be a frame index or a driver specific flag
    /// ## Returns
    /// `false` if no frames has been grabbed
    /// 
    /// The method decodes and returns the just grabbed frame. If no frames has been grabbed
    /// (camera has been disconnected, or there are no more frames in video file), the method returns false
    /// and the function returns an empty image (with %cv::Mat, test it with Mat::empty()).
    /// 
    /// ## See also
    /// read()
    /// 
    /// 
    /// Note: In @ref videoio_c "C API", functions cvRetrieveFrame() and cv.RetrieveFrame() return image stored inside the video
    /// capturing structure. It is not allowed to modify or release the image! You can copy the frame using
    /// cvCloneImage and then do whatever you want with the copy.
    ///
    /// ## C++ default parameters
    /// * flag: 0
    pub fn retrieve(&mut self, image: &mut core::Mat, flag: i32) -> Result<bool> {
        unsafe { sys::cv_VideoCapture_retrieve_Mat_int(self.as_raw_VideoCapture(), image.as_raw_Mat(), flag) }.into_result()
    }
    
    /// Grabs, decodes and returns the next video frame.
    /// 
    /// ## Parameters
    /// * image: [out] the video frame is returned here. If no frames has been grabbed the image will be empty.
    /// ## Returns
    /// `false` if no frames has been grabbed
    /// 
    /// The method/function combines VideoCapture::grab() and VideoCapture::retrieve() in one call. This is the
    /// most convenient method for reading video files or capturing data from decode and returns the just
    /// grabbed frame. If no frames has been grabbed (camera has been disconnected, or there are no more
    /// frames in video file), the method returns false and the function returns empty image (with %cv::Mat, test it with Mat::empty()).
    /// 
    /// 
    /// Note: In @ref videoio_c "C API", functions cvRetrieveFrame() and cv.RetrieveFrame() return image stored inside the video
    /// capturing structure. It is not allowed to modify or release the image! You can copy the frame using
    /// cvCloneImage and then do whatever you want with the copy.
    pub fn read(&mut self, image: &mut core::Mat) -> Result<bool> {
        unsafe { sys::cv_VideoCapture_read_Mat(self.as_raw_VideoCapture(), image.as_raw_Mat()) }.into_result()
    }
    
    /// Sets a property in the VideoCapture.
    /// 
    /// ## Parameters
    /// * propId: Property identifier from cv::VideoCaptureProperties (eg. cv::CAP_PROP_POS_MSEC, cv::CAP_PROP_POS_FRAMES, ...)
    /// or one from @ref videoio_flags_others
    /// * value: Value of the property.
    /// ## Returns
    /// `true` if the property is supported by backend used by the VideoCapture instance.
    /// 
    /// Note: Even if it returns `true` this doesn't ensure that the property
    /// value has been accepted by the capture device. See note in VideoCapture::get()
    pub fn set(&mut self, prop_id: i32, value: f64) -> Result<bool> {
        unsafe { sys::cv_VideoCapture_set_int_double(self.as_raw_VideoCapture(), prop_id, value) }.into_result()
    }
    
    /// Returns the specified VideoCapture property
    /// 
    /// ## Parameters
    /// * propId: Property identifier from cv::VideoCaptureProperties (eg. cv::CAP_PROP_POS_MSEC, cv::CAP_PROP_POS_FRAMES, ...)
    /// or one from @ref videoio_flags_others
    /// ## Returns
    /// Value for the specified property. Value 0 is returned when querying a property that is
    /// not supported by the backend used by the VideoCapture instance.
    /// 
    /// 
    /// Note: Reading / writing properties involves many layers. Some unexpected result might happens
    /// along this chain.
    /// ```ignore {.txt}
    /// `VideoCapture -> API Backend -> Operating System -> Device Driver -> Device Hardware`
    /// ```
    /// 
    /// The returned value might be different from what really used by the device or it could be encoded
    /// using device dependent rules (eg. steps or percentage). Effective behaviour depends from device
    /// driver and API Backend
    pub fn get(&self, prop_id: i32) -> Result<f64> {
        unsafe { sys::cv_VideoCapture_get_const_int(self.as_raw_VideoCapture(), prop_id) }.into_result()
    }
    
    /// Open video file or a capturing device or a IP video stream for video capturing with API Preference
    /// 
    /// 
    /// 
    /// Parameters are same as the constructor VideoCapture(const String& filename, int apiPreference)
    /// ## Returns
    /// `true` if the file has been successfully opened
    /// 
    /// The method first calls VideoCapture::release to close the already opened file or camera.
    pub fn open_file_with_backend(&mut self, filename: &str, api_preference: i32) -> Result<bool> {
        string_arg!(filename);
        unsafe { sys::cv_VideoCapture_open_String_int(self.as_raw_VideoCapture(), filename.as_ptr(), api_preference) }.into_result()
    }
    
    /// Returns used backend API name
    /// 
    /// 
    /// Note: Stream should be opened.
    pub fn get_backend_name(&self) -> Result<String> {
        unsafe { sys::cv_VideoCapture_getBackendName_const(self.as_raw_VideoCapture()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

// boxed class cv::VideoWriter
/// Video writer class.
/// 
/// The class provides C++ API for writing video files or image sequences.
#[allow(dead_code)]
pub struct VideoWriter {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::videoio::VideoWriter {
    fn drop(&mut self) {
        unsafe { sys::cv_VideoWriter_delete(self.ptr) };
    }
}
impl crate::videoio::VideoWriter {
    #[doc(hidden)] pub fn as_raw_VideoWriter(&self) -> *mut c_void { self.ptr }
}

impl VideoWriter {

    /// Default constructors
    /// 
    /// The constructors/functions initialize video writers.
    /// *   On Linux FFMPEG is used to write videos;
    /// *   On Windows FFMPEG or VFW is used;
    /// *   On MacOSX QTKit is used.
    pub fn default() -> Result<crate::videoio::VideoWriter> {
        unsafe { sys::cv_VideoWriter_VideoWriter() }.into_result().map(|x| crate::videoio::VideoWriter { ptr: x })
    }
    
    /// ## Parameters
    /// * filename: Name of the output video file.
    /// * fourcc: 4-character code of codec used to compress the frames. For example,
    /// VideoWriter::fourcc('P','I','M','1') is a MPEG-1 codec, VideoWriter::fourcc('M','J','P','G') is a
    /// motion-jpeg codec etc. List of codes can be obtained at [Video Codecs by
    /// FOURCC](http://www.fourcc.org/codecs.php) page. FFMPEG backend with MP4 container natively uses
    /// other values as fourcc code: see [ObjectType](http://www.mp4ra.org/codecs.html),
    /// so you may receive a warning message from OpenCV about fourcc code conversion.
    /// * fps: Framerate of the created video stream.
    /// * frameSize: Size of the video frames.
    /// * isColor: If it is not zero, the encoder will expect and encode color frames, otherwise it
    /// will work with grayscale frames (the flag is currently supported on Windows only).
    /// 
    /// @b Tips:
    /// - With some backends `fourcc=-1` pops up the codec selection dialog from the system.
    /// - To save image sequence use a proper filename (eg. `img_%02d.jpg`) and `fourcc=0`
    /// OR `fps=0`. Use uncompressed image format (eg. `img_%02d.BMP`) to save raw frames.
    /// - Most codecs are lossy. If you want lossless video file you need to use a lossless codecs
    /// (eg. FFMPEG FFV1, Huffman HFYU, Lagarith LAGS, etc...)
    /// - If FFMPEG is enabled, using `codec=0; fps=0;` you can create an uncompressed (raw) video file.
    ///
    /// ## C++ default parameters
    /// * is_color: true
    pub fn new(filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<crate::videoio::VideoWriter> {
        string_arg!(filename);
        unsafe { sys::cv_VideoWriter_VideoWriter_String_int_double_Size_bool(filename.as_ptr(), fourcc, fps, frame_size, is_color) }.into_result().map(|x| crate::videoio::VideoWriter { ptr: x })
    }
    
    /// The `apiPreference` parameter allows to specify API backends to use. Can be used to enforce a specific reader implementation
    /// if multiple are available: e.g. cv::CAP_FFMPEG or cv::CAP_GSTREAMER.
    ///
    /// ## C++ default parameters
    /// * is_color: true
    pub fn new_with_backend(filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<crate::videoio::VideoWriter> {
        string_arg!(filename);
        unsafe { sys::cv_VideoWriter_VideoWriter_String_int_int_double_Size_bool(filename.as_ptr(), api_preference, fourcc, fps, frame_size, is_color) }.into_result().map(|x| crate::videoio::VideoWriter { ptr: x })
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
    /// ## C++ default parameters
    /// * is_color: true
    pub fn open(&mut self, filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<bool> {
        string_arg!(filename);
        unsafe { sys::cv_VideoWriter_open_String_int_double_Size_bool(self.as_raw_VideoWriter(), filename.as_ptr(), fourcc, fps, frame_size, is_color) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * is_color: true
    pub fn open_with_backend(&mut self, filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<bool> {
        string_arg!(filename);
        unsafe { sys::cv_VideoWriter_open_String_int_int_double_Size_bool(self.as_raw_VideoWriter(), filename.as_ptr(), api_preference, fourcc, fps, frame_size, is_color) }.into_result()
    }
    
    /// Returns true if video writer has been successfully initialized.
    pub fn is_opened(&self) -> Result<bool> {
        unsafe { sys::cv_VideoWriter_isOpened_const(self.as_raw_VideoWriter()) }.into_result()
    }
    
    /// Closes the video writer.
    /// 
    /// The method is automatically called by subsequent VideoWriter::open and by the VideoWriter
    /// destructor.
    pub fn release(&mut self) -> Result<()> {
        unsafe { sys::cv_VideoWriter_release(self.as_raw_VideoWriter()) }.into_result()
    }
    
    /// Writes the next video frame
    /// 
    /// ## Parameters
    /// * image: The written frame. In general, color images are expected in BGR format.
    /// 
    /// The function/method writes the specified image to video file. It must have the same size as has
    /// been specified when opening the video writer.
    pub fn write(&mut self, image: &core::Mat) -> Result<()> {
        unsafe { sys::cv_VideoWriter_write_Mat(self.as_raw_VideoWriter(), image.as_raw_Mat()) }.into_result()
    }
    
    /// Sets a property in the VideoWriter.
    /// 
    /// ## Parameters
    /// * propId: Property identifier from cv::VideoWriterProperties (eg. cv::VIDEOWRITER_PROP_QUALITY)
    /// or one of @ref videoio_flags_others
    /// 
    /// * value: Value of the property.
    /// ## Returns
    /// `true` if the property is supported by the backend used by the VideoWriter instance.
    pub fn set(&mut self, prop_id: i32, value: f64) -> Result<bool> {
        unsafe { sys::cv_VideoWriter_set_int_double(self.as_raw_VideoWriter(), prop_id, value) }.into_result()
    }
    
    /// Returns the specified VideoWriter property
    /// 
    /// ## Parameters
    /// * propId: Property identifier from cv::VideoWriterProperties (eg. cv::VIDEOWRITER_PROP_QUALITY)
    /// or one of @ref videoio_flags_others
    /// 
    /// ## Returns
    /// Value for the specified property. Value 0 is returned when querying a property that is
    /// not supported by the backend used by the VideoWriter instance.
    pub fn get(&self, prop_id: i32) -> Result<f64> {
        unsafe { sys::cv_VideoWriter_get_const_int(self.as_raw_VideoWriter(), prop_id) }.into_result()
    }
    
    /// Concatenates 4 chars to a fourcc code
    /// 
    /// ## Returns
    /// a fourcc code
    /// 
    /// This static method constructs the fourcc code of the codec to be used in the constructor
    /// VideoWriter::VideoWriter or VideoWriter::open.
    pub fn fourcc(c1: i8, c2: i8, c3: i8, c4: i8) -> Result<i32> {
        unsafe { sys::cv_VideoWriter_fourcc_char_char_char_char(c1, c2, c3, c4) }.into_result()
    }
    
    /// Returns used backend API name
    /// 
    /// 
    /// Note: Stream should be opened.
    pub fn get_backend_name(&self) -> Result<String> {
        unsafe { sys::cv_VideoWriter_getBackendName_const(self.as_raw_VideoWriter()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

pub const CAP_INTELPERC_GENERATORS_MASK: i32 = 0x30000000; // 805306368
pub const CAP_OPENNI_DEPTH_GENERATOR_BASELINE: i32 = 0x80000066; // -2147483546
pub const CAP_OPENNI_DEPTH_GENERATOR_FOCAL_LENGTH: i32 = 0x80000067; // -2147483545
pub const CAP_OPENNI_DEPTH_GENERATOR_PRESENT: i32 = 0x8000006d; // -2147483539
pub const CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION: i32 = 0x80000068; // -2147483544
pub const CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION_ON: i32 = 0x80000068; // -2147483544
pub const CAP_OPENNI_GENERATORS_MASK: i32 = 0xe0000000; // -536870912
pub const CAP_OPENNI_IMAGE_GENERATOR_OUTPUT_MODE: i32 = 0x40000064; // 1073741924
pub const CAP_OPENNI_IMAGE_GENERATOR_PRESENT: i32 = 0x4000006d; // 1073741933
pub const CAP_OPENNI_IR_GENERATOR_PRESENT: i32 = 0x2000006d; // 536871021

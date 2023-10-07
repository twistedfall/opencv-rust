pub mod videoio {
	//! # Video I/O
	//! 
	//! Read and write video or images sequence with OpenCV
	//! 
	//! ### See also:
	//! - [videoio_overview]
	//! - Tutorials: [tutorial_table_of_content_app]
	//!   # Flags for video I/O
	//!   # Additional flags for video I/O API backends
	//!   # Hardware-accelerated video decoding and encoding
	//!   # C API for video I/O
	//!   # iOS glue for video I/O
	//!   # WinRT glue for video I/O
	//!   # Query I/O API backends registry
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::VideoCaptureTraitConst, super::VideoCaptureTrait, super::VideoWriterTraitConst, super::VideoWriterTrait };
	}
	
	/// Android - not used
	pub const CAP_ANDROID: i32 = 1000;
	/// Auto detect == 0
	pub const CAP_ANY: i32 = 0;
	/// Aravis SDK
	pub const CAP_ARAVIS: i32 = 2100;
	/// AVFoundation framework for iOS (OS X Lion will have the same API)
	pub const CAP_AVFOUNDATION: i32 = 1200;
	/// Same value as CAP_FIREWIRE
	pub const CAP_CMU1394: i32 = 300;
	/// Same value as CAP_FIREWIRE
	pub const CAP_DC1394: i32 = 300;
	/// DirectShow (via videoInput)
	pub const CAP_DSHOW: i32 = 700;
	/// Open and record video file or stream using the FFMPEG library
	pub const CAP_FFMPEG: i32 = 1900;
	/// Same value as CAP_FIREWIRE
	pub const CAP_FIREWARE: i32 = 300;
	/// IEEE 1394 drivers
	pub const CAP_FIREWIRE: i32 = 300;
	/// Smartek Giganetix GigEVisionSDK
	pub const CAP_GIGANETIX: i32 = 1300;
	/// gPhoto2 connection
	pub const CAP_GPHOTO2: i32 = 1700;
	/// GStreamer
	pub const CAP_GSTREAMER: i32 = 1800;
	/// Same value as CAP_FIREWIRE
	pub const CAP_IEEE1394: i32 = 300;
	/// OpenCV Image Sequence (e.g. img_%02d.jpg)
	pub const CAP_IMAGES: i32 = 2000;
	/// RealSense (former Intel Perceptual Computing SDK)
	pub const CAP_INTELPERC: i32 = 1500;
	pub const CAP_INTELPERC_DEPTH_GENERATOR: i32 = 536870912;
	/// Each pixel is a 16-bit integer. The value indicates the distance from an object to the camera's XY plane or the Cartesian depth.
	pub const CAP_INTELPERC_DEPTH_MAP: i32 = 0;
	pub const CAP_INTELPERC_GENERATORS_MASK: i32 = 939524096;
	pub const CAP_INTELPERC_IMAGE: i32 = 3;
	pub const CAP_INTELPERC_IMAGE_GENERATOR: i32 = 268435456;
	pub const CAP_INTELPERC_IR_GENERATOR: i32 = 134217728;
	/// Each pixel is a 16-bit integer. The value indicates the intensity of the reflected laser beam.
	pub const CAP_INTELPERC_IR_MAP: i32 = 2;
	/// Each pixel contains two 32-bit floating point values in the range of 0-1, representing the mapping of depth coordinates to the color coordinates.
	pub const CAP_INTELPERC_UVDEPTH_MAP: i32 = 1;
	/// Intel MediaSDK
	pub const CAP_INTEL_MFX: i32 = 2300;
	/// Microsoft Media Foundation (via videoInput). See platform specific notes above.
	pub const CAP_MSMF: i32 = 1400;
	/// For Orbbec 3D-Sensor device/module (Astra+, Femto)
	pub const CAP_OBSENSOR: i32 = 2600;
	/// Data given from BGR stream generator
	pub const CAP_OBSENSOR_BGR_IMAGE: i32 = 1;
	pub const CAP_OBSENSOR_DEPTH_GENERATOR: i32 = 536870912;
	/// Depth values in mm (CV_16UC1)
	pub const CAP_OBSENSOR_DEPTH_MAP: i32 = 0;
	pub const CAP_OBSENSOR_GENERATORS_MASK: i32 = 939524096;
	pub const CAP_OBSENSOR_IMAGE_GENERATOR: i32 = 268435456;
	pub const CAP_OBSENSOR_IR_GENERATOR: i32 = 134217728;
	/// Data given from IR stream generator(CV_16UC1)
	pub const CAP_OBSENSOR_IR_IMAGE: i32 = 2;
	/// Built-in OpenCV MotionJPEG codec
	pub const CAP_OPENCV_MJPEG: i32 = 2200;
	/// OpenNI (for Kinect)
	pub const CAP_OPENNI: i32 = 900;
	/// OpenNI2 (for Kinect)
	pub const CAP_OPENNI2: i32 = 1600;
	/// OpenNI2 (for Orbbec Astra)
	pub const CAP_OPENNI2_ASTRA: i32 = 1620;
	/// OpenNI2 (for Asus Xtion and Occipital Structure sensors)
	pub const CAP_OPENNI2_ASUS: i32 = 1610;
	/// OpenNI (for Asus Xtion)
	pub const CAP_OPENNI_ASUS: i32 = 910;
	/// Data given from RGB image generator
	pub const CAP_OPENNI_BGR_IMAGE: i32 = 5;
	pub const CAP_OPENNI_DEPTH_GENERATOR: i32 = -2147483648;
	pub const CAP_OPENNI_DEPTH_GENERATOR_BASELINE: i32 = -2147483546;
	pub const CAP_OPENNI_DEPTH_GENERATOR_FOCAL_LENGTH: i32 = -2147483545;
	pub const CAP_OPENNI_DEPTH_GENERATOR_PRESENT: i32 = -2147483539;
	pub const CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION: i32 = -2147483544;
	pub const CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION_ON: i32 = -2147483544;
	/// Depth values in mm (CV_16UC1)
	pub const CAP_OPENNI_DEPTH_MAP: i32 = 0;
	/// Disparity in pixels (CV_8UC1)
	pub const CAP_OPENNI_DISPARITY_MAP: i32 = 2;
	/// Disparity in pixels (CV_32FC1)
	pub const CAP_OPENNI_DISPARITY_MAP_32F: i32 = 3;
	pub const CAP_OPENNI_GENERATORS_MASK: i32 = -536870912;
	/// Data given from RGB image generator
	pub const CAP_OPENNI_GRAY_IMAGE: i32 = 6;
	pub const CAP_OPENNI_IMAGE_GENERATOR: i32 = 1073741824;
	pub const CAP_OPENNI_IMAGE_GENERATOR_OUTPUT_MODE: i32 = 1073741924;
	pub const CAP_OPENNI_IMAGE_GENERATOR_PRESENT: i32 = 1073741933;
	pub const CAP_OPENNI_IR_GENERATOR: i32 = 536870912;
	pub const CAP_OPENNI_IR_GENERATOR_PRESENT: i32 = 536871021;
	/// Data given from IR image generator
	pub const CAP_OPENNI_IR_IMAGE: i32 = 7;
	/// XYZ in meters (CV_32FC3)
	pub const CAP_OPENNI_POINT_CLOUD_MAP: i32 = 1;
	pub const CAP_OPENNI_QVGA_30HZ: i32 = 3;
	pub const CAP_OPENNI_QVGA_60HZ: i32 = 4;
	pub const CAP_OPENNI_SXGA_15HZ: i32 = 1;
	pub const CAP_OPENNI_SXGA_30HZ: i32 = 2;
	/// CV_8UC1
	pub const CAP_OPENNI_VALID_DEPTH_MASK: i32 = 4;
	pub const CAP_OPENNI_VGA_30HZ: i32 = 0;
	/// Aperture. Can be readonly, depends on camera program.
	pub const CAP_PROP_APERTURE: i32 = 17008;
	/// Automatically trigger frame capture if camera is configured with software trigger
	pub const CAP_PROP_ARAVIS_AUTOTRIGGER: i32 = 600;
	/// (read-only) Index of the first audio channel for .retrieve() calls. That audio channel number continues enumeration after video channels.
	pub const CAP_PROP_AUDIO_BASE_INDEX: i32 = 63;
	/// (open, read) Alternative definition to bits-per-sample, but with clear handling of 32F / 32S
	pub const CAP_PROP_AUDIO_DATA_DEPTH: i32 = 61;
	/// (read-only) Audio position is measured in samples. Accurate audio sample timestamp of previous grabbed fragment. See CAP_PROP_AUDIO_SAMPLES_PER_SECOND and CAP_PROP_AUDIO_SHIFT_NSEC.
	pub const CAP_PROP_AUDIO_POS: i32 = 59;
	/// (open, read) determined from file/codec input. If not specified, then selected audio sample rate is 44100
	pub const CAP_PROP_AUDIO_SAMPLES_PER_SECOND: i32 = 62;
	/// (read only) Contains the time difference between the start of the audio stream and the video stream in nanoseconds. Positive value means that audio is started after the first video frame. Negative value means that audio is started before the first video frame.
	pub const CAP_PROP_AUDIO_SHIFT_NSEC: i32 = 60;
	/// (**open-only**) Specify stream in multi-language media files, -1 - disable audio processing or microphone. Default value is -1.
	pub const CAP_PROP_AUDIO_STREAM: i32 = 58;
	/// (open, read) Enables audio synchronization.
	pub const CAP_PROP_AUDIO_SYNCHRONIZE: i32 = 66;
	/// (read-only) Number of audio channels in the selected audio stream (mono, stereo, etc)
	pub const CAP_PROP_AUDIO_TOTAL_CHANNELS: i32 = 64;
	/// (read-only) Number of audio streams.
	pub const CAP_PROP_AUDIO_TOTAL_STREAMS: i32 = 65;
	pub const CAP_PROP_AUTOFOCUS: i32 = 39;
	/// DC1394: exposure control done by camera, user can adjust reference level using this feature.
	pub const CAP_PROP_AUTO_EXPOSURE: i32 = 21;
	/// enable/ disable auto white-balance
	pub const CAP_PROP_AUTO_WB: i32 = 44;
	/// Current backend (enum VideoCaptureAPIs). Read-only property
	pub const CAP_PROP_BACKEND: i32 = 42;
	pub const CAP_PROP_BACKLIGHT: i32 = 32;
	/// (read-only) Video bitrate in kbits/s
	pub const CAP_PROP_BITRATE: i32 = 47;
	/// Brightness of the image (only for those cameras that support).
	pub const CAP_PROP_BRIGHTNESS: i32 = 10;
	pub const CAP_PROP_BUFFERSIZE: i32 = 38;
	/// Video input or Channel Number (only for those cameras that support)
	pub const CAP_PROP_CHANNEL: i32 = 43;
	/// Positive index indicates that returning extra data is supported by the video back end.  This can be retrieved as cap.retrieve(data, <returned index>).  E.g. When reading from a h264 encoded RTSP stream, the FFmpeg backend could return the SPS and/or PPS if available (if sent in reply to a DESCRIBE request), from calls to cap.retrieve(data, <returned index>).
	pub const CAP_PROP_CODEC_EXTRADATA_INDEX: i32 = 68;
	/// (read-only) codec's pixel format. 4-character code - see VideoWriter::fourcc . Subset of [AV_PIX_FMT_*](https://github.com/FFmpeg/FFmpeg/blob/master/libavcodec/raw.c) or -1 if unknown
	pub const CAP_PROP_CODEC_PIXEL_FORMAT: i32 = 46;
	/// Contrast of the image (only for cameras).
	pub const CAP_PROP_CONTRAST: i32 = 11;
	/// Boolean flags indicating whether images should be converted to RGB. <br/>
	/// *GStreamer note*: The flag is ignored in case if custom pipeline is used. It's user responsibility to interpret pipeline output.
	pub const CAP_PROP_CONVERT_RGB: i32 = 16;
	pub const CAP_PROP_DC1394_MAX: i32 = 31;
	pub const CAP_PROP_DC1394_MODE_AUTO: i32 = -2;
	/// set automatically when a value of the feature is set by the user.
	pub const CAP_PROP_DC1394_MODE_MANUAL: i32 = -3;
	pub const CAP_PROP_DC1394_MODE_ONE_PUSH_AUTO: i32 = -1;
	/// turn the feature off (not controlled manually nor automatically).
	pub const CAP_PROP_DC1394_OFF: i32 = -4;
	/// Exposure (only for those cameras that support).
	pub const CAP_PROP_EXPOSURE: i32 = 15;
	/// Camera exposure program.
	pub const CAP_PROP_EXPOSUREPROGRAM: i32 = 17009;
	pub const CAP_PROP_FOCUS: i32 = 28;
	/// Format of the %Mat objects (see Mat::type()) returned by VideoCapture::retrieve().
	/// Set value -1 to fetch undecoded RAW video streams (as Mat 8UC1).
	pub const CAP_PROP_FORMAT: i32 = 8;
	/// 4-character code of codec. see VideoWriter::fourcc .
	pub const CAP_PROP_FOURCC: i32 = 6;
	/// Frame rate.
	pub const CAP_PROP_FPS: i32 = 5;
	/// Number of frames in the video file.
	pub const CAP_PROP_FRAME_COUNT: i32 = 7;
	/// Height of the frames in the video stream.
	pub const CAP_PROP_FRAME_HEIGHT: i32 = 4;
	/// (read-only) FFmpeg back-end only - Frame type ascii code (73 = 'I', 80 = 'P', 66 = 'B' or 63 = '?' if unknown) of the most recently read frame.
	pub const CAP_PROP_FRAME_TYPE: i32 = 69;
	/// Width of the frames in the video stream.
	pub const CAP_PROP_FRAME_WIDTH: i32 = 3;
	/// Gain of the image (only for those cameras that support).
	pub const CAP_PROP_GAIN: i32 = 14;
	pub const CAP_PROP_GAMMA: i32 = 22;
	pub const CAP_PROP_GIGA_FRAME_HEIGH_MAX: i32 = 10004;
	pub const CAP_PROP_GIGA_FRAME_OFFSET_X: i32 = 10001;
	pub const CAP_PROP_GIGA_FRAME_OFFSET_Y: i32 = 10002;
	pub const CAP_PROP_GIGA_FRAME_SENS_HEIGH: i32 = 10006;
	pub const CAP_PROP_GIGA_FRAME_SENS_WIDTH: i32 = 10005;
	pub const CAP_PROP_GIGA_FRAME_WIDTH_MAX: i32 = 10003;
	/// Collect messages with details.
	pub const CAP_PROP_GPHOTO2_COLLECT_MSGS: i32 = 17005;
	/// Readonly, returns (const char *).
	pub const CAP_PROP_GPHOTO2_FLUSH_MSGS: i32 = 17006;
	/// Capture only preview from liveview mode.
	pub const CAP_PROP_GPHOTO2_PREVIEW: i32 = 17001;
	/// Trigger, only by set. Reload camera settings.
	pub const CAP_PROP_GPHOTO2_RELOAD_CONFIG: i32 = 17003;
	/// Reload all settings on set.
	pub const CAP_PROP_GPHOTO2_RELOAD_ON_CHANGE: i32 = 17004;
	/// Readonly, returns (const char *).
	pub const CAP_PROP_GPHOTO2_WIDGET_ENUMERATE: i32 = 17002;
	/// Default is 1
	pub const CAP_PROP_GSTREAMER_QUEUE_LENGTH: i32 = 200;
	pub const CAP_PROP_GUID: i32 = 29;
	/// Hue of the image (only for cameras).
	pub const CAP_PROP_HUE: i32 = 13;
	/// (**open-only**) Hardware acceleration type (see #VideoAccelerationType). Setting supported only via `params` parameter in cv::VideoCapture constructor / .open() method. Default value is backend-specific.
	pub const CAP_PROP_HW_ACCELERATION: i32 = 50;
	/// (**open-only**) If non-zero, create new OpenCL context and bind it to current thread. The OpenCL context created with Video Acceleration context attached it (if not attached yet) for optimized GPU data copy between HW accelerated decoder and cv::UMat.
	pub const CAP_PROP_HW_ACCELERATION_USE_OPENCL: i32 = 52;
	/// (**open-only**) Hardware device index (select GPU if multiple available). Device enumeration is acceleration type specific.
	pub const CAP_PROP_HW_DEVICE: i32 = 51;
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
	/// FFmpeg back-end only - Indicates whether the Last Raw Frame (LRF), output from VideoCapture::read() when VideoCapture is initialized with VideoCapture::open(CAP_FFMPEG, {CAP_PROP_FORMAT, -1}) or VideoCapture::set(CAP_PROP_FORMAT,-1) is called before the first call to VideoCapture::read(), contains encoded data for a key frame.
	pub const CAP_PROP_LRF_HAS_KEY_FRAME: i32 = 67;
	/// Backend-specific value indicating the current capture mode.
	pub const CAP_PROP_MODE: i32 = 9;
	pub const CAP_PROP_MONOCHROME: i32 = 19;
	/// (**open-only**) Set the maximum number of threads to use. Use 0 to use as many threads as CPU cores (applicable for FFmpeg back-end only).
	pub const CAP_PROP_N_THREADS: i32 = 70;
	pub const CAP_PROP_OBSENSOR_INTRINSIC_CX: i32 = 26003;
	pub const CAP_PROP_OBSENSOR_INTRINSIC_CY: i32 = 26004;
	pub const CAP_PROP_OBSENSOR_INTRINSIC_FX: i32 = 26001;
	pub const CAP_PROP_OBSENSOR_INTRINSIC_FY: i32 = 26002;
	pub const CAP_PROP_OPENNI2_MIRROR: i32 = 111;
	pub const CAP_PROP_OPENNI2_SYNC: i32 = 110;
	pub const CAP_PROP_OPENNI_APPROX_FRAME_SYNC: i32 = 105;
	/// In mm
	pub const CAP_PROP_OPENNI_BASELINE: i32 = 102;
	pub const CAP_PROP_OPENNI_CIRCLE_BUFFER: i32 = 107;
	/// In pixels
	pub const CAP_PROP_OPENNI_FOCAL_LENGTH: i32 = 103;
	/// In mm
	pub const CAP_PROP_OPENNI_FRAME_MAX_DEPTH: i32 = 101;
	pub const CAP_PROP_OPENNI_GENERATOR_PRESENT: i32 = 109;
	pub const CAP_PROP_OPENNI_MAX_BUFFER_SIZE: i32 = 106;
	pub const CAP_PROP_OPENNI_MAX_TIME_DURATION: i32 = 108;
	pub const CAP_PROP_OPENNI_OUTPUT_MODE: i32 = 100;
	/// Flag that synchronizes the remapping depth map to image map
	/// by changing depth generator's view point (if the flag is "on") or
	/// sets this view point to its normal one (if the flag is "off").
	pub const CAP_PROP_OPENNI_REGISTRATION: i32 = 104;
	pub const CAP_PROP_OPENNI_REGISTRATION_ON: i32 = 104;
	/// (**open-only**) timeout in milliseconds for opening a video capture (applicable for FFmpeg and GStreamer back-ends only)
	pub const CAP_PROP_OPEN_TIMEOUT_MSEC: i32 = 53;
	/// if true - rotates output frames of CvCapture considering video file's metadata  (applicable for FFmpeg and AVFoundation back-ends only) (<https://github.com/opencv/opencv/issues/15499>)
	pub const CAP_PROP_ORIENTATION_AUTO: i32 = 49;
	/// (read-only) Frame rotation defined by stream meta (applicable for FFmpeg and AVFoundation back-ends only)
	pub const CAP_PROP_ORIENTATION_META: i32 = 48;
	pub const CAP_PROP_PAN: i32 = 33;
	/// Relative position of the video file: 0=start of the film, 1=end of the film.
	pub const CAP_PROP_POS_AVI_RATIO: i32 = 2;
	/// 0-based index of the frame to be decoded/captured next.
	pub const CAP_PROP_POS_FRAMES: i32 = 1;
	/// Current position of the video file in milliseconds.
	pub const CAP_PROP_POS_MSEC: i32 = 0;
	/// Horizontal binning factor.
	pub const CAP_PROP_PVAPI_BINNINGX: i32 = 304;
	/// Vertical binning factor.
	pub const CAP_PROP_PVAPI_BINNINGY: i32 = 305;
	/// Horizontal sub-sampling of the image.
	pub const CAP_PROP_PVAPI_DECIMATIONHORIZONTAL: i32 = 302;
	/// Vertical sub-sampling of the image.
	pub const CAP_PROP_PVAPI_DECIMATIONVERTICAL: i32 = 303;
	/// FrameStartTriggerMode: Determines how a frame is initiated.
	pub const CAP_PROP_PVAPI_FRAMESTARTTRIGGERMODE: i32 = 301;
	/// IP for enable multicast master mode. 0 for disable multicast.
	pub const CAP_PROP_PVAPI_MULTICASTIP: i32 = 300;
	/// Pixel format.
	pub const CAP_PROP_PVAPI_PIXELFORMAT: i32 = 306;
	/// (**open-only**) timeout in milliseconds for reading from a video capture (applicable for FFmpeg and GStreamer back-ends only)
	pub const CAP_PROP_READ_TIMEOUT_MSEC: i32 = 54;
	/// Rectification flag for stereo cameras (note: only supported by DC1394 v 2.x backend currently).
	pub const CAP_PROP_RECTIFICATION: i32 = 18;
	pub const CAP_PROP_ROLL: i32 = 35;
	/// Sample aspect ratio: num/den (den)
	pub const CAP_PROP_SAR_DEN: i32 = 41;
	/// Sample aspect ratio: num/den (num)
	pub const CAP_PROP_SAR_NUM: i32 = 40;
	/// Saturation of the image (only for cameras).
	pub const CAP_PROP_SATURATION: i32 = 12;
	/// Pop up video/camera filter dialog (note: only supported by DSHOW backend currently. The property value is ignored)
	pub const CAP_PROP_SETTINGS: i32 = 37;
	pub const CAP_PROP_SHARPNESS: i32 = 20;
	/// Exposure speed. Can be readonly, depends on camera program.
	pub const CAP_PROP_SPEED: i32 = 17007;
	pub const CAP_PROP_STREAM_OPEN_TIME_USEC: i32 = 55;
	pub const CAP_PROP_TEMPERATURE: i32 = 23;
	pub const CAP_PROP_TILT: i32 = 34;
	pub const CAP_PROP_TRIGGER: i32 = 24;
	pub const CAP_PROP_TRIGGER_DELAY: i32 = 25;
	/// (**open-only**) Specify video stream, 0-based index. Use -1 to disable video stream from file or IP cameras. Default value is 0.
	pub const CAP_PROP_VIDEO_STREAM: i32 = 57;
	/// (read-only) Number of video channels
	pub const CAP_PROP_VIDEO_TOTAL_CHANNELS: i32 = 56;
	/// Enter liveview mode.
	pub const CAP_PROP_VIEWFINDER: i32 = 17010;
	/// white-balance color temperature
	pub const CAP_PROP_WB_TEMPERATURE: i32 = 45;
	/// Currently unsupported.
	pub const CAP_PROP_WHITE_BALANCE_BLUE_U: i32 = 17;
	pub const CAP_PROP_WHITE_BALANCE_RED_V: i32 = 26;
	/// Acquisition buffer size in buffer_size_unit. Default bytes.
	pub const CAP_PROP_XI_ACQ_BUFFER_SIZE: i32 = 548;
	/// Acquisition buffer size unit in bytes. Default 1. E.g. Value 1024 means that buffer_size is in KiBytes.
	pub const CAP_PROP_XI_ACQ_BUFFER_SIZE_UNIT: i32 = 549;
	/// Sets number of frames acquired by burst. This burst is used only if trigger is set to FrameBurstStart.
	pub const CAP_PROP_XI_ACQ_FRAME_BURST_COUNT: i32 = 499;
	/// Type of sensor frames timing.
	pub const CAP_PROP_XI_ACQ_TIMING_MODE: i32 = 538;
	/// Number of buffers to commit to low level.
	pub const CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_COMMIT: i32 = 552;
	/// Acquisition transport buffer size in bytes.
	pub const CAP_PROP_XI_ACQ_TRANSPORT_BUFFER_SIZE: i32 = 550;
	/// Automatic exposure/gain.
	pub const CAP_PROP_XI_AEAG: i32 = 415;
	/// Average intensity of output signal AEAG should achieve(in %).
	pub const CAP_PROP_XI_AEAG_LEVEL: i32 = 419;
	/// Automatic exposure/gain ROI Height.
	pub const CAP_PROP_XI_AEAG_ROI_HEIGHT: i32 = 442;
	/// Automatic exposure/gain ROI offset X.
	pub const CAP_PROP_XI_AEAG_ROI_OFFSET_X: i32 = 439;
	/// Automatic exposure/gain ROI offset Y.
	pub const CAP_PROP_XI_AEAG_ROI_OFFSET_Y: i32 = 440;
	/// Automatic exposure/gain ROI Width.
	pub const CAP_PROP_XI_AEAG_ROI_WIDTH: i32 = 441;
	/// Maximum limit of exposure in AEAG procedure.
	pub const CAP_PROP_XI_AE_MAX_LIMIT: i32 = 417;
	/// Maximum limit of gain in AEAG procedure.
	pub const CAP_PROP_XI_AG_MAX_LIMIT: i32 = 418;
	/// Enable applying of CMS profiles to xiGetImage (see XI_PRM_INPUT_CMS_PROFILE, XI_PRM_OUTPUT_CMS_PROFILE).
	pub const CAP_PROP_XI_APPLY_CMS: i32 = 471;
	/// Automatic bandwidth calculation.
	pub const CAP_PROP_XI_AUTO_BANDWIDTH_CALCULATION: i32 = 573;
	/// Automatic white balance.
	pub const CAP_PROP_XI_AUTO_WB: i32 = 414;
	/// Calculate and returns available interface bandwidth(int Megabits).
	pub const CAP_PROP_XI_AVAILABLE_BANDWIDTH: i32 = 539;
	/// Horizontal Binning - number of horizontal photo-sensitive cells to combine together.
	pub const CAP_PROP_XI_BINNING_HORIZONTAL: i32 = 429;
	/// Binning pattern type.
	pub const CAP_PROP_XI_BINNING_PATTERN: i32 = 430;
	/// Binning engine selector.
	pub const CAP_PROP_XI_BINNING_SELECTOR: i32 = 427;
	/// Vertical Binning - number of vertical photo-sensitive cells to combine together.
	pub const CAP_PROP_XI_BINNING_VERTICAL: i32 = 428;
	/// Correction of bad pixels.
	pub const CAP_PROP_XI_BPC: i32 = 445;
	/// Queue of field/frame buffers.
	pub const CAP_PROP_XI_BUFFERS_QUEUE_SIZE: i32 = 551;
	/// Data move policy.
	pub const CAP_PROP_XI_BUFFER_POLICY: i32 = 540;
	/// Color Correction Matrix element [0][0].
	pub const CAP_PROP_XI_CC_MATRIX_00: i32 = 479;
	/// Color Correction Matrix element [0][1].
	pub const CAP_PROP_XI_CC_MATRIX_01: i32 = 480;
	/// Color Correction Matrix element [0][2].
	pub const CAP_PROP_XI_CC_MATRIX_02: i32 = 481;
	/// Color Correction Matrix element [0][3].
	pub const CAP_PROP_XI_CC_MATRIX_03: i32 = 482;
	/// Color Correction Matrix element [1][0].
	pub const CAP_PROP_XI_CC_MATRIX_10: i32 = 483;
	/// Color Correction Matrix element [1][1].
	pub const CAP_PROP_XI_CC_MATRIX_11: i32 = 484;
	/// Color Correction Matrix element [1][2].
	pub const CAP_PROP_XI_CC_MATRIX_12: i32 = 485;
	/// Color Correction Matrix element [1][3].
	pub const CAP_PROP_XI_CC_MATRIX_13: i32 = 486;
	/// Color Correction Matrix element [2][0].
	pub const CAP_PROP_XI_CC_MATRIX_20: i32 = 487;
	/// Color Correction Matrix element [2][1].
	pub const CAP_PROP_XI_CC_MATRIX_21: i32 = 488;
	/// Color Correction Matrix element [2][2].
	pub const CAP_PROP_XI_CC_MATRIX_22: i32 = 489;
	/// Color Correction Matrix element [2][3].
	pub const CAP_PROP_XI_CC_MATRIX_23: i32 = 490;
	/// Color Correction Matrix element [3][0].
	pub const CAP_PROP_XI_CC_MATRIX_30: i32 = 491;
	/// Color Correction Matrix element [3][1].
	pub const CAP_PROP_XI_CC_MATRIX_31: i32 = 492;
	/// Color Correction Matrix element [3][2].
	pub const CAP_PROP_XI_CC_MATRIX_32: i32 = 493;
	/// Color Correction Matrix element [3][3].
	pub const CAP_PROP_XI_CC_MATRIX_33: i32 = 494;
	/// Camera sensor temperature.
	pub const CAP_PROP_XI_CHIP_TEMP: i32 = 468;
	/// Mode of color management system.
	pub const CAP_PROP_XI_CMS: i32 = 470;
	/// Returns color filter array type of RAW data.
	pub const CAP_PROP_XI_COLOR_FILTER_ARRAY: i32 = 475;
	/// Correction of column FPN.
	pub const CAP_PROP_XI_COLUMN_FPN_CORRECTION: i32 = 555;
	/// Start camera cooling.
	pub const CAP_PROP_XI_COOLING: i32 = 466;
	/// Select counter.
	pub const CAP_PROP_XI_COUNTER_SELECTOR: i32 = 536;
	/// Counter status.
	pub const CAP_PROP_XI_COUNTER_VALUE: i32 = 537;
	/// Output data format.
	pub const CAP_PROP_XI_DATA_FORMAT: i32 = 401;
	/// Enable/Disable debounce to selected GPI.
	pub const CAP_PROP_XI_DEBOUNCE_EN: i32 = 507;
	/// Debounce polarity (pol = 1 t0 - falling edge, t1 - rising edge).
	pub const CAP_PROP_XI_DEBOUNCE_POL: i32 = 510;
	/// Debounce time (x * 10us).
	pub const CAP_PROP_XI_DEBOUNCE_T0: i32 = 508;
	/// Debounce time (x * 10us).
	pub const CAP_PROP_XI_DEBOUNCE_T1: i32 = 509;
	/// Set debug level.
	pub const CAP_PROP_XI_DEBUG_LEVEL: i32 = 572;
	/// Horizontal Decimation - horizontal sub-sampling of the image - reduces the horizontal resolution of the image by the specified vertical decimation factor.
	pub const CAP_PROP_XI_DECIMATION_HORIZONTAL: i32 = 433;
	/// Decimation pattern type.
	pub const CAP_PROP_XI_DECIMATION_PATTERN: i32 = 434;
	/// Decimation engine selector.
	pub const CAP_PROP_XI_DECIMATION_SELECTOR: i32 = 431;
	/// Vertical Decimation - vertical sub-sampling of the image - reduces the vertical resolution of the image by the specified vertical decimation factor.
	pub const CAP_PROP_XI_DECIMATION_VERTICAL: i32 = 432;
	/// Set default Color Correction Matrix.
	pub const CAP_PROP_XI_DEFAULT_CC_MATRIX: i32 = 495;
	/// Returns device model id.
	pub const CAP_PROP_XI_DEVICE_MODEL_ID: i32 = 521;
	/// Resets the camera to default state.
	pub const CAP_PROP_XI_DEVICE_RESET: i32 = 554;
	/// Returns device serial number.
	pub const CAP_PROP_XI_DEVICE_SN: i32 = 522;
	/// Change image resolution by binning or skipping.
	pub const CAP_PROP_XI_DOWNSAMPLING: i32 = 400;
	/// Change image downsampling type.
	pub const CAP_PROP_XI_DOWNSAMPLING_TYPE: i32 = 426;
	/// Exposure time in microseconds.
	pub const CAP_PROP_XI_EXPOSURE: i32 = 421;
	/// Sets the number of times of exposure in one frame.
	pub const CAP_PROP_XI_EXPOSURE_BURST_COUNT: i32 = 422;
	/// Exposure priority (0.5 - exposure 50%, gain 50%).
	pub const CAP_PROP_XI_EXP_PRIORITY: i32 = 416;
	/// Setting of key enables file operations on some cameras.
	pub const CAP_PROP_XI_FFS_ACCESS_KEY: i32 = 583;
	/// File number.
	pub const CAP_PROP_XI_FFS_FILE_ID: i32 = 594;
	/// Size of file.
	pub const CAP_PROP_XI_FFS_FILE_SIZE: i32 = 580;
	/// Define framerate in Hz.
	pub const CAP_PROP_XI_FRAMERATE: i32 = 535;
	/// Size of free camera FFS.
	pub const CAP_PROP_XI_FREE_FFS_SIZE: i32 = 581;
	/// Gain in dB.
	pub const CAP_PROP_XI_GAIN: i32 = 424;
	/// Gain selector for parameter Gain allows to select different type of gains.
	pub const CAP_PROP_XI_GAIN_SELECTOR: i32 = 423;
	/// Chromaticity gamma.
	pub const CAP_PROP_XI_GAMMAC: i32 = 477;
	/// Luminosity gamma.
	pub const CAP_PROP_XI_GAMMAY: i32 = 476;
	/// Get general purpose level.
	pub const CAP_PROP_XI_GPI_LEVEL: i32 = 408;
	/// Set general purpose input mode.
	pub const CAP_PROP_XI_GPI_MODE: i32 = 407;
	/// Selects general purpose input.
	pub const CAP_PROP_XI_GPI_SELECTOR: i32 = 406;
	/// Set general purpose output mode.
	pub const CAP_PROP_XI_GPO_MODE: i32 = 410;
	/// Selects general purpose output.
	pub const CAP_PROP_XI_GPO_SELECTOR: i32 = 409;
	/// Enable High Dynamic Range feature.
	pub const CAP_PROP_XI_HDR: i32 = 559;
	/// The number of kneepoints in the PWLR.
	pub const CAP_PROP_XI_HDR_KNEEPOINT_COUNT: i32 = 560;
	/// Position of first kneepoint(in % of XI_PRM_EXPOSURE).
	pub const CAP_PROP_XI_HDR_T1: i32 = 561;
	/// Position of second kneepoint (in % of XI_PRM_EXPOSURE).
	pub const CAP_PROP_XI_HDR_T2: i32 = 562;
	/// Height of the Image provided by the device (in pixels).
	pub const CAP_PROP_XI_HEIGHT: i32 = 452;
	/// Camera housing back side temperature.
	pub const CAP_PROP_XI_HOUS_BACK_SIDE_TEMP: i32 = 590;
	/// Camera housing temperature.
	pub const CAP_PROP_XI_HOUS_TEMP: i32 = 469;
	/// Returns hardware revision number.
	pub const CAP_PROP_XI_HW_REVISION: i32 = 571;
	/// Last image black level counts. Can be used for Offline processing to recall it.
	pub const CAP_PROP_XI_IMAGE_BLACK_LEVEL: i32 = 565;
	/// bitdepth of data returned by function xiGetImage.
	pub const CAP_PROP_XI_IMAGE_DATA_BIT_DEPTH: i32 = 462;
	/// Output data format.
	pub const CAP_PROP_XI_IMAGE_DATA_FORMAT: i32 = 435;
	/// The alpha channel of RGB32 output image format.
	pub const CAP_PROP_XI_IMAGE_DATA_FORMAT_RGB32_ALPHA: i32 = 529;
	/// Returns 1 for color cameras.
	pub const CAP_PROP_XI_IMAGE_IS_COLOR: i32 = 474;
	/// Buffer size in bytes sufficient for output image returned by xiGetImage.
	pub const CAP_PROP_XI_IMAGE_PAYLOAD_SIZE: i32 = 530;
	/// Returns 1 for cameras that support cooling.
	pub const CAP_PROP_XI_IS_COOLED: i32 = 465;
	/// Returns 1 if camera connected and works properly.
	pub const CAP_PROP_XI_IS_DEVICE_EXIST: i32 = 547;
	/// Value of first kneepoint (% of sensor saturation).
	pub const CAP_PROP_XI_KNEEPOINT1: i32 = 563;
	/// Value of second kneepoint (% of sensor saturation).
	pub const CAP_PROP_XI_KNEEPOINT2: i32 = 564;
	/// Define camera signalling LED functionality.
	pub const CAP_PROP_XI_LED_MODE: i32 = 412;
	/// Selects camera signalling LED.
	pub const CAP_PROP_XI_LED_SELECTOR: i32 = 411;
	/// Current lens aperture value in stops. Examples: 2.8, 4, 5.6, 8, 11.
	pub const CAP_PROP_XI_LENS_APERTURE_VALUE: i32 = 512;
	/// Allows access to lens feature value currently selected by XI_PRM_LENS_FEATURE_SELECTOR.
	pub const CAP_PROP_XI_LENS_FEATURE: i32 = 518;
	/// Selects the current feature which is accessible by XI_PRM_LENS_FEATURE.
	pub const CAP_PROP_XI_LENS_FEATURE_SELECTOR: i32 = 517;
	/// Lens focal distance in mm.
	pub const CAP_PROP_XI_LENS_FOCAL_LENGTH: i32 = 516;
	/// Lens focus distance in cm.
	pub const CAP_PROP_XI_LENS_FOCUS_DISTANCE: i32 = 515;
	/// Moves lens focus motor by steps set in XI_PRM_LENS_FOCUS_MOVEMENT_VALUE.
	pub const CAP_PROP_XI_LENS_FOCUS_MOVE: i32 = 514;
	/// Lens current focus movement value to be used by XI_PRM_LENS_FOCUS_MOVE in motor steps.
	pub const CAP_PROP_XI_LENS_FOCUS_MOVEMENT_VALUE: i32 = 513;
	/// Status of lens control interface. This shall be set to XI_ON before any Lens operations.
	pub const CAP_PROP_XI_LENS_MODE: i32 = 511;
	/// Set/get bandwidth(datarate)(in Megabits).
	pub const CAP_PROP_XI_LIMIT_BANDWIDTH: i32 = 459;
	/// Activates LUT.
	pub const CAP_PROP_XI_LUT_EN: i32 = 541;
	/// Control the index (offset) of the coefficient to access in the LUT.
	pub const CAP_PROP_XI_LUT_INDEX: i32 = 542;
	/// Value at entry LUTIndex of the LUT.
	pub const CAP_PROP_XI_LUT_VALUE: i32 = 543;
	/// Calculates White Balance(must be called during acquisition).
	pub const CAP_PROP_XI_MANUAL_WB: i32 = 413;
	/// Horizontal offset from the origin to the area of interest (in pixels).
	pub const CAP_PROP_XI_OFFSET_X: i32 = 402;
	/// Vertical offset from the origin to the area of interest (in pixels).
	pub const CAP_PROP_XI_OFFSET_Y: i32 = 403;
	/// Device output data bit depth.
	pub const CAP_PROP_XI_OUTPUT_DATA_BIT_DEPTH: i32 = 461;
	/// Device output data packing (or grouping) enabled. Packing could be enabled if output_data_bit_depth > 8 and packing capability is available.
	pub const CAP_PROP_XI_OUTPUT_DATA_PACKING: i32 = 463;
	/// Data packing type. Some cameras supports only specific packing type.
	pub const CAP_PROP_XI_OUTPUT_DATA_PACKING_TYPE: i32 = 464;
	/// GetImage returns most recent frame.
	pub const CAP_PROP_XI_RECENT_FRAME: i32 = 553;
	/// Activates/deactivates Region selected by Region Selector.
	pub const CAP_PROP_XI_REGION_MODE: i32 = 595;
	/// Selects Region in Multiple ROI which parameters are set by width, height, ... ,region mode.
	pub const CAP_PROP_XI_REGION_SELECTOR: i32 = 589;
	/// Correction of row FPN.
	pub const CAP_PROP_XI_ROW_FPN_CORRECTION: i32 = 591;
	/// Camera sensor board temperature.
	pub const CAP_PROP_XI_SENSOR_BOARD_TEMP: i32 = 596;
	/// Sensor clock frequency in Hz.
	pub const CAP_PROP_XI_SENSOR_CLOCK_FREQ_HZ: i32 = 532;
	/// Sensor clock frequency index. Sensor with selected frequencies have possibility to set the frequency only by this index.
	pub const CAP_PROP_XI_SENSOR_CLOCK_FREQ_INDEX: i32 = 533;
	/// Sensor output data bit depth.
	pub const CAP_PROP_XI_SENSOR_DATA_BIT_DEPTH: i32 = 460;
	/// Selects the current feature which is accessible by XI_PRM_SENSOR_FEATURE_VALUE.
	pub const CAP_PROP_XI_SENSOR_FEATURE_SELECTOR: i32 = 585;
	/// Allows access to sensor feature value currently selected by XI_PRM_SENSOR_FEATURE_SELECTOR.
	pub const CAP_PROP_XI_SENSOR_FEATURE_VALUE: i32 = 586;
	/// Current sensor mode. Allows to select sensor mode by one integer. Setting of this parameter affects: image dimensions and downsampling.
	pub const CAP_PROP_XI_SENSOR_MODE: i32 = 558;
	/// Number of output channels from sensor used for data transfer.
	pub const CAP_PROP_XI_SENSOR_OUTPUT_CHANNEL_COUNT: i32 = 534;
	/// Number of taps.
	pub const CAP_PROP_XI_SENSOR_TAPS: i32 = 437;
	/// Sharpness Strength.
	pub const CAP_PROP_XI_SHARPNESS: i32 = 478;
	/// Change sensor shutter type(CMOS sensor).
	pub const CAP_PROP_XI_SHUTTER_TYPE: i32 = 436;
	/// Set sensor target temperature for cooling.
	pub const CAP_PROP_XI_TARGET_TEMP: i32 = 467;
	/// Selects which test pattern type is generated by the selected generator.
	pub const CAP_PROP_XI_TEST_PATTERN: i32 = 588;
	/// Selects which test pattern generator is controlled by the TestPattern feature.
	pub const CAP_PROP_XI_TEST_PATTERN_GENERATOR_SELECTOR: i32 = 587;
	/// Image capture timeout in milliseconds.
	pub const CAP_PROP_XI_TIMEOUT: i32 = 420;
	/// Current format of pixels on transport layer.
	pub const CAP_PROP_XI_TRANSPORT_PIXEL_FORMAT: i32 = 531;
	/// Specifies the delay in microseconds (us) to apply after the trigger reception before activating it.
	pub const CAP_PROP_XI_TRG_DELAY: i32 = 544;
	/// Selects the type of trigger.
	pub const CAP_PROP_XI_TRG_SELECTOR: i32 = 498;
	/// Generates an internal trigger. PRM_TRG_SOURCE must be set to TRG_SOFTWARE.
	pub const CAP_PROP_XI_TRG_SOFTWARE: i32 = 405;
	/// Defines source of trigger.
	pub const CAP_PROP_XI_TRG_SOURCE: i32 = 404;
	/// Defines how time stamp reset engine will be armed.
	pub const CAP_PROP_XI_TS_RST_MODE: i32 = 545;
	/// Defines which source will be used for timestamp reset. Writing this parameter will trigger settings of engine (arming).
	pub const CAP_PROP_XI_TS_RST_SOURCE: i32 = 546;
	/// Size of used camera FFS.
	pub const CAP_PROP_XI_USED_FFS_SIZE: i32 = 582;
	/// White balance blue coefficient.
	pub const CAP_PROP_XI_WB_KB: i32 = 450;
	/// White balance green coefficient.
	pub const CAP_PROP_XI_WB_KG: i32 = 449;
	/// White balance red coefficient.
	pub const CAP_PROP_XI_WB_KR: i32 = 448;
	/// Width of the Image provided by the device (in pixels).
	pub const CAP_PROP_XI_WIDTH: i32 = 451;
	pub const CAP_PROP_ZOOM: i32 = 27;
	/// PvAPI, Prosilica GigE SDK
	pub const CAP_PVAPI: i32 = 800;
	/// 2 out of 16 decimation
	pub const CAP_PVAPI_DECIMATION_2OUTOF16: i32 = 8;
	/// 2 out of 4 decimation
	pub const CAP_PVAPI_DECIMATION_2OUTOF4: i32 = 2;
	/// 2 out of 8 decimation
	pub const CAP_PVAPI_DECIMATION_2OUTOF8: i32 = 4;
	/// Off
	pub const CAP_PVAPI_DECIMATION_OFF: i32 = 1;
	/// FixedRate
	pub const CAP_PVAPI_FSTRIGMODE_FIXEDRATE: i32 = 3;
	/// Freerun
	pub const CAP_PVAPI_FSTRIGMODE_FREERUN: i32 = 0;
	/// Software
	pub const CAP_PVAPI_FSTRIGMODE_SOFTWARE: i32 = 4;
	/// SyncIn1
	pub const CAP_PVAPI_FSTRIGMODE_SYNCIN1: i32 = 1;
	/// SyncIn2
	pub const CAP_PVAPI_FSTRIGMODE_SYNCIN2: i32 = 2;
	/// Bayer16
	pub const CAP_PVAPI_PIXELFORMAT_BAYER16: i32 = 4;
	/// Bayer8
	pub const CAP_PVAPI_PIXELFORMAT_BAYER8: i32 = 3;
	/// Bgr24
	pub const CAP_PVAPI_PIXELFORMAT_BGR24: i32 = 6;
	/// Bgra32
	pub const CAP_PVAPI_PIXELFORMAT_BGRA32: i32 = 8;
	/// Mono16
	pub const CAP_PVAPI_PIXELFORMAT_MONO16: i32 = 2;
	/// Mono8
	pub const CAP_PVAPI_PIXELFORMAT_MONO8: i32 = 1;
	/// Rgb24
	pub const CAP_PVAPI_PIXELFORMAT_RGB24: i32 = 5;
	/// Rgba32
	pub const CAP_PVAPI_PIXELFORMAT_RGBA32: i32 = 7;
	/// QuickTime (obsolete, removed)
	pub const CAP_QT: i32 = 500;
	/// Synonym for CAP_INTELPERC
	pub const CAP_REALSENSE: i32 = 1500;
	/// uEye Camera API
	pub const CAP_UEYE: i32 = 2500;
	/// Unicap drivers (obsolete, removed)
	pub const CAP_UNICAP: i32 = 600;
	/// V4L/V4L2 capturing support
	pub const CAP_V4L: i32 = 200;
	/// Same as CAP_V4L
	pub const CAP_V4L2: i32 = 200;
	/// Video For Windows (obsolete, removed)
	pub const CAP_VFW: i32 = 200;
	/// Microsoft Windows Runtime using Media Foundation
	pub const CAP_WINRT: i32 = 1410;
	/// XIMEA Camera API
	pub const CAP_XIAPI: i32 = 1100;
	/// XINE engine (Linux)
	pub const CAP_XINE: i32 = 2400;
	pub const CV__CAP_PROP_LATEST: i32 = 71;
	pub const CV__VIDEOWRITER_PROP_LATEST: i32 = 9;
	/// Defaults to CV_8U.
	pub const VIDEOWRITER_PROP_DEPTH: i32 = 5;
	/// (Read-only): Size of just encoded video frame. Note that the encoding order may be different from representation order.
	pub const VIDEOWRITER_PROP_FRAMEBYTES: i32 = 2;
	/// (**open-only**) Hardware acceleration type (see #VideoAccelerationType). Setting supported only via `params` parameter in VideoWriter constructor / .open() method. Default value is backend-specific.
	pub const VIDEOWRITER_PROP_HW_ACCELERATION: i32 = 6;
	/// (**open-only**) If non-zero, create new OpenCL context and bind it to current thread. The OpenCL context created with Video Acceleration context attached it (if not attached yet) for optimized GPU data copy between cv::UMat and HW accelerated encoder.
	pub const VIDEOWRITER_PROP_HW_ACCELERATION_USE_OPENCL: i32 = 8;
	/// (**open-only**) Hardware device index (select GPU if multiple available). Device enumeration is acceleration type specific.
	pub const VIDEOWRITER_PROP_HW_DEVICE: i32 = 7;
	/// If it is not zero, the encoder will expect and encode color frames, otherwise it
	/// will work with grayscale frames.
	pub const VIDEOWRITER_PROP_IS_COLOR: i32 = 4;
	/// Number of stripes for parallel encoding. -1 for auto detection.
	pub const VIDEOWRITER_PROP_NSTRIPES: i32 = 3;
	/// Current quality (0..100%) of the encoded videostream. Can be adjusted dynamically in some codecs.
	pub const VIDEOWRITER_PROP_QUALITY: i32 = 1;
	/// Prefer to use H/W acceleration. If no one supported, then fallback to software processing.
	/// 
	/// Note: H/W acceleration may require special configuration of used environment.
	/// 
	/// Note: Results in encoding scenario may differ between software and hardware accelerated encoders.
	pub const VIDEO_ACCELERATION_ANY: i32 = 1;
	/// DirectX 11
	pub const VIDEO_ACCELERATION_D3D11: i32 = 2;
	/// libmfx (Intel MediaSDK/oneVPL)
	pub const VIDEO_ACCELERATION_MFX: i32 = 4;
	/// Do not require any specific H/W acceleration, prefer software processing.
	/// Reading of this value means that special H/W accelerated handling is not added or not detected by OpenCV.
	pub const VIDEO_ACCELERATION_NONE: i32 = 0;
	/// VAAPI
	pub const VIDEO_ACCELERATION_VAAPI: i32 = 3;
	/// Video Acceleration type
	/// 
	/// Used as value in [CAP_PROP_HW_ACCELERATION] and [VIDEOWRITER_PROP_HW_ACCELERATION]
	/// 
	/// 
	/// Note: In case of FFmpeg backend, it translated to enum AVHWDeviceType (<https://github.com/FFmpeg/FFmpeg/blob/master/libavutil/hwcontext.h>)
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum VideoAccelerationType {
		/// Do not require any specific H/W acceleration, prefer software processing.
		/// Reading of this value means that special H/W accelerated handling is not added or not detected by OpenCV.
		VIDEO_ACCELERATION_NONE = 0,
		/// Prefer to use H/W acceleration. If no one supported, then fallback to software processing.
		/// 
		/// Note: H/W acceleration may require special configuration of used environment.
		/// 
		/// Note: Results in encoding scenario may differ between software and hardware accelerated encoders.
		VIDEO_ACCELERATION_ANY = 1,
		/// DirectX 11
		VIDEO_ACCELERATION_D3D11 = 2,
		/// VAAPI
		VIDEO_ACCELERATION_VAAPI = 3,
		/// libmfx (Intel MediaSDK/oneVPL)
		VIDEO_ACCELERATION_MFX = 4,
	}
	
	opencv_type_enum! { crate::videoio::VideoAccelerationType }
	
	/// cv::VideoCapture API backends identifier.
	/// 
	/// Select preferred API for a capture object.
	/// To be used in the VideoCapture::VideoCapture() constructor or VideoCapture::open()
	/// 
	/// 
	/// Note:
	/// *   Backends are available only if they have been built with your OpenCV binaries.
	/// See [videoio_overview] for more information.
	/// *   Microsoft Media Foundation backend tries to use hardware accelerated transformations
	/// if possible. Environment flag "OPENCV_VIDEOIO_MSMF_ENABLE_HW_TRANSFORMS" set to 0
	/// disables it and may improve initialization time. More details:
	/// <https://learn.microsoft.com/en-us/windows/win32/medfound/mf-readwrite-enable-hardware-transforms>
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum VideoCaptureAPIs {
		/// Auto detect == 0
		CAP_ANY = 0,
		/// Video For Windows (obsolete, removed)
		CAP_VFW = 200,
		// V4L/V4L2 capturing support
		// Duplicate, use CAP_VFW instead
		// CAP_V4L = 200,
		// Same as CAP_V4L
		// Duplicate, use CAP_V4L instead
		// CAP_V4L2 = 200,
		/// IEEE 1394 drivers
		CAP_FIREWIRE = 300,
		// Same value as CAP_FIREWIRE
		// Duplicate, use CAP_FIREWIRE instead
		// CAP_FIREWARE = 300,
		// Same value as CAP_FIREWIRE
		// Duplicate, use CAP_FIREWARE instead
		// CAP_IEEE1394 = 300,
		// Same value as CAP_FIREWIRE
		// Duplicate, use CAP_IEEE1394 instead
		// CAP_DC1394 = 300,
		// Same value as CAP_FIREWIRE
		// Duplicate, use CAP_DC1394 instead
		// CAP_CMU1394 = 300,
		/// QuickTime (obsolete, removed)
		CAP_QT = 500,
		/// Unicap drivers (obsolete, removed)
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
		/// Microsoft Media Foundation (via videoInput). See platform specific notes above.
		CAP_MSMF = 1400,
		/// Microsoft Windows Runtime using Media Foundation
		CAP_WINRT = 1410,
		/// RealSense (former Intel Perceptual Computing SDK)
		CAP_INTELPERC = 1500,
		// Synonym for CAP_INTELPERC
		// Duplicate, use CAP_INTELPERC instead
		// CAP_REALSENSE = 1500,
		/// OpenNI2 (for Kinect)
		CAP_OPENNI2 = 1600,
		/// OpenNI2 (for Asus Xtion and Occipital Structure sensors)
		CAP_OPENNI2_ASUS = 1610,
		/// OpenNI2 (for Orbbec Astra)
		CAP_OPENNI2_ASTRA = 1620,
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
		/// uEye Camera API
		CAP_UEYE = 2500,
		/// For Orbbec 3D-Sensor device/module (Astra+, Femto)
		CAP_OBSENSOR = 2600,
	}
	
	opencv_type_enum! { crate::videoio::VideoCaptureAPIs }
	
	/// OBSENSOR (for Orbbec 3D-Sensor device/module )
	/// 
	/// //! OBSENSOR data given from image generator
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum VideoCaptureOBSensorDataType {
		/// Depth values in mm (CV_16UC1)
		CAP_OBSENSOR_DEPTH_MAP = 0,
		/// Data given from BGR stream generator
		CAP_OBSENSOR_BGR_IMAGE = 1,
		/// Data given from IR stream generator(CV_16UC1)
		CAP_OBSENSOR_IR_IMAGE = 2,
	}
	
	opencv_type_enum! { crate::videoio::VideoCaptureOBSensorDataType }
	
	/// OBSENSOR stream generator
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum VideoCaptureOBSensorGenerators {
		CAP_OBSENSOR_DEPTH_GENERATOR = 536870912,
		CAP_OBSENSOR_IMAGE_GENERATOR = 268435456,
		CAP_OBSENSOR_IR_GENERATOR = 134217728,
		CAP_OBSENSOR_GENERATORS_MASK = 939524096,
	}
	
	opencv_type_enum! { crate::videoio::VideoCaptureOBSensorGenerators }
	
	/// OBSENSOR properties
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum VideoCaptureOBSensorProperties {
		CAP_PROP_OBSENSOR_INTRINSIC_FX = 26001,
		CAP_PROP_OBSENSOR_INTRINSIC_FY = 26002,
		CAP_PROP_OBSENSOR_INTRINSIC_CX = 26003,
		CAP_PROP_OBSENSOR_INTRINSIC_CY = 26004,
	}
	
	opencv_type_enum! { crate::videoio::VideoCaptureOBSensorProperties }
	
	/// cv::VideoCapture generic properties identifier.
	/// 
	/// Reading / writing properties involves many layers. Some unexpected result might happens along this chain.
	/// Effective behaviour depends from device hardware, driver and API Backend.
	/// ## See also
	/// videoio_flags_others, VideoCapture::get(), VideoCapture::set()
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
		/// (read-only) Frame rotation defined by stream meta (applicable for FFmpeg and AVFoundation back-ends only)
		CAP_PROP_ORIENTATION_META = 48,
		/// if true - rotates output frames of CvCapture considering video file's metadata  (applicable for FFmpeg and AVFoundation back-ends only) (<https://github.com/opencv/opencv/issues/15499>)
		CAP_PROP_ORIENTATION_AUTO = 49,
		/// (**open-only**) Hardware acceleration type (see #VideoAccelerationType). Setting supported only via `params` parameter in cv::VideoCapture constructor / .open() method. Default value is backend-specific.
		CAP_PROP_HW_ACCELERATION = 50,
		/// (**open-only**) Hardware device index (select GPU if multiple available). Device enumeration is acceleration type specific.
		CAP_PROP_HW_DEVICE = 51,
		/// (**open-only**) If non-zero, create new OpenCL context and bind it to current thread. The OpenCL context created with Video Acceleration context attached it (if not attached yet) for optimized GPU data copy between HW accelerated decoder and cv::UMat.
		CAP_PROP_HW_ACCELERATION_USE_OPENCL = 52,
		/// (**open-only**) timeout in milliseconds for opening a video capture (applicable for FFmpeg and GStreamer back-ends only)
		CAP_PROP_OPEN_TIMEOUT_MSEC = 53,
		/// (**open-only**) timeout in milliseconds for reading from a video capture (applicable for FFmpeg and GStreamer back-ends only)
		CAP_PROP_READ_TIMEOUT_MSEC = 54,
		CAP_PROP_STREAM_OPEN_TIME_USEC = 55,
		/// (read-only) Number of video channels
		CAP_PROP_VIDEO_TOTAL_CHANNELS = 56,
		/// (**open-only**) Specify video stream, 0-based index. Use -1 to disable video stream from file or IP cameras. Default value is 0.
		CAP_PROP_VIDEO_STREAM = 57,
		/// (**open-only**) Specify stream in multi-language media files, -1 - disable audio processing or microphone. Default value is -1.
		CAP_PROP_AUDIO_STREAM = 58,
		/// (read-only) Audio position is measured in samples. Accurate audio sample timestamp of previous grabbed fragment. See CAP_PROP_AUDIO_SAMPLES_PER_SECOND and CAP_PROP_AUDIO_SHIFT_NSEC.
		CAP_PROP_AUDIO_POS = 59,
		/// (read only) Contains the time difference between the start of the audio stream and the video stream in nanoseconds. Positive value means that audio is started after the first video frame. Negative value means that audio is started before the first video frame.
		CAP_PROP_AUDIO_SHIFT_NSEC = 60,
		/// (open, read) Alternative definition to bits-per-sample, but with clear handling of 32F / 32S
		CAP_PROP_AUDIO_DATA_DEPTH = 61,
		/// (open, read) determined from file/codec input. If not specified, then selected audio sample rate is 44100
		CAP_PROP_AUDIO_SAMPLES_PER_SECOND = 62,
		/// (read-only) Index of the first audio channel for .retrieve() calls. That audio channel number continues enumeration after video channels.
		CAP_PROP_AUDIO_BASE_INDEX = 63,
		/// (read-only) Number of audio channels in the selected audio stream (mono, stereo, etc)
		CAP_PROP_AUDIO_TOTAL_CHANNELS = 64,
		/// (read-only) Number of audio streams.
		CAP_PROP_AUDIO_TOTAL_STREAMS = 65,
		/// (open, read) Enables audio synchronization.
		CAP_PROP_AUDIO_SYNCHRONIZE = 66,
		/// FFmpeg back-end only - Indicates whether the Last Raw Frame (LRF), output from VideoCapture::read() when VideoCapture is initialized with VideoCapture::open(CAP_FFMPEG, {CAP_PROP_FORMAT, -1}) or VideoCapture::set(CAP_PROP_FORMAT,-1) is called before the first call to VideoCapture::read(), contains encoded data for a key frame.
		CAP_PROP_LRF_HAS_KEY_FRAME = 67,
		/// Positive index indicates that returning extra data is supported by the video back end.  This can be retrieved as cap.retrieve(data, <returned index>).  E.g. When reading from a h264 encoded RTSP stream, the FFmpeg backend could return the SPS and/or PPS if available (if sent in reply to a DESCRIBE request), from calls to cap.retrieve(data, <returned index>).
		CAP_PROP_CODEC_EXTRADATA_INDEX = 68,
		/// (read-only) FFmpeg back-end only - Frame type ascii code (73 = 'I', 80 = 'P', 66 = 'B' or 63 = '?' if unknown) of the most recently read frame.
		CAP_PROP_FRAME_TYPE = 69,
		/// (**open-only**) Set the maximum number of threads to use. Use 0 to use as many threads as CPU cores (applicable for FFmpeg back-end only).
		CAP_PROP_N_THREADS = 70,
		CV__CAP_PROP_LATEST = 71,
	}
	
	opencv_type_enum! { crate::videoio::VideoCaptureProperties }
	
	/// cv::VideoWriter generic properties identifier.
	/// ## See also
	/// VideoWriter::get(), VideoWriter::set()
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum VideoWriterProperties {
		/// Current quality (0..100%) of the encoded videostream. Can be adjusted dynamically in some codecs.
		VIDEOWRITER_PROP_QUALITY = 1,
		/// (Read-only): Size of just encoded video frame. Note that the encoding order may be different from representation order.
		VIDEOWRITER_PROP_FRAMEBYTES = 2,
		/// Number of stripes for parallel encoding. -1 for auto detection.
		VIDEOWRITER_PROP_NSTRIPES = 3,
		/// If it is not zero, the encoder will expect and encode color frames, otherwise it
		/// will work with grayscale frames.
		VIDEOWRITER_PROP_IS_COLOR = 4,
		/// Defaults to CV_8U.
		VIDEOWRITER_PROP_DEPTH = 5,
		/// (**open-only**) Hardware acceleration type (see #VideoAccelerationType). Setting supported only via `params` parameter in VideoWriter constructor / .open() method. Default value is backend-specific.
		VIDEOWRITER_PROP_HW_ACCELERATION = 6,
		/// (**open-only**) Hardware device index (select GPU if multiple available). Device enumeration is acceleration type specific.
		VIDEOWRITER_PROP_HW_DEVICE = 7,
		/// (**open-only**) If non-zero, create new OpenCL context and bind it to current thread. The OpenCL context created with Video Acceleration context attached it (if not attached yet) for optimized GPU data copy between cv::UMat and HW accelerated encoder.
		VIDEOWRITER_PROP_HW_ACCELERATION_USE_OPENCL = 8,
		CV__VIDEOWRITER_PROP_LATEST = 9,
	}
	
	opencv_type_enum! { crate::videoio::VideoWriterProperties }
	
	/// Returns backend API name or "UnknownVideoAPI(xxx)"
	/// ## Parameters
	/// * api: backend ID (#VideoCaptureAPIs)
	#[inline]
	pub fn get_backend_name(api: crate::videoio::VideoCaptureAPIs) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videoio_registry_getBackendName_VideoCaptureAPIs(api, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Returns list of all available backends
	#[inline]
	pub fn get_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videoio_registry_getBackends(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Returns description and ABI/API version of videoio plugin's camera interface
	#[inline]
	pub fn get_camera_backend_plugin_version(api: crate::videoio::VideoCaptureAPIs, version_abi: &mut i32, version_api: &mut i32) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videoio_registry_getCameraBackendPluginVersion_VideoCaptureAPIs_intR_intR(api, version_abi, version_api, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Returns list of available backends which works via `cv::VideoCapture(int index)`
	#[inline]
	pub fn get_camera_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videoio_registry_getCameraBackends(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Returns description and ABI/API version of videoio plugin's stream capture interface
	#[inline]
	pub fn get_stream_backend_plugin_version(api: crate::videoio::VideoCaptureAPIs, version_abi: &mut i32, version_api: &mut i32) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videoio_registry_getStreamBackendPluginVersion_VideoCaptureAPIs_intR_intR(api, version_abi, version_api, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Returns list of available backends which works via `cv::VideoCapture(filename)`
	#[inline]
	pub fn get_stream_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videoio_registry_getStreamBackends(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Returns description and ABI/API version of videoio plugin's writer interface
	#[inline]
	pub fn get_writer_backend_plugin_version(api: crate::videoio::VideoCaptureAPIs, version_abi: &mut i32, version_api: &mut i32) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videoio_registry_getWriterBackendPluginVersion_VideoCaptureAPIs_intR_intR(api, version_abi, version_api, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Returns list of available backends which works via `cv::VideoWriter()`
	#[inline]
	pub fn get_writer_backends() -> Result<core::Vector<crate::videoio::VideoCaptureAPIs>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videoio_registry_getWriterBackends(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<crate::videoio::VideoCaptureAPIs>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Returns true if backend is available
	#[inline]
	pub fn has_backend(api: crate::videoio::VideoCaptureAPIs) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videoio_registry_hasBackend_VideoCaptureAPIs(api, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns true if backend is built in (false if backend is used as plugin)
	#[inline]
	pub fn is_backend_built_in(api: crate::videoio::VideoCaptureAPIs) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videoio_registry_isBackendBuiltIn_VideoCaptureAPIs(api, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::videoio::VideoCapture]
	pub trait VideoCaptureTraitConst {
		fn as_raw_VideoCapture(&self) -> *const c_void;
	
		/// Returns true if video capturing has been initialized already.
		/// 
		/// If the previous call to VideoCapture constructor or VideoCapture::open() succeeded, the method returns
		/// true.
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
	
		/// Opens a video file or a capturing device or an IP video stream for video capturing.
		/// 
		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// 
		/// Parameters are same as the constructor VideoCapture(const String& filename, int apiPreference = CAP_ANY)
		/// ## Returns
		/// `true` if the file has been successfully opened
		/// 
		/// The method first calls VideoCapture::release to close the already opened file or camera.
		/// 
		/// ## C++ default parameters
		/// * api_preference: CAP_ANY
		#[inline]
		fn open_file(&mut self, filename: &str, api_preference: i32) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_open_const_StringR_int(self.as_raw_mut_VideoCapture(), filename.opencv_as_extern(), api_preference, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Opens a video file or a capturing device or an IP video stream for video capturing.
		/// 
		/// @overload
		/// 
		/// Parameters are same as the constructor VideoCapture(const String& filename, int apiPreference = CAP_ANY)
		/// ## Returns
		/// `true` if the file has been successfully opened
		/// 
		/// The method first calls VideoCapture::release to close the already opened file or camera.
		/// 
		/// ## Note
		/// This alternative version of [open_file] function uses the following default values for its arguments:
		/// * api_preference: CAP_ANY
		#[inline]
		fn open_file_def(&mut self, filename: &str) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_open_const_StringR(self.as_raw_mut_VideoCapture(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Opens a video file or a capturing device or an IP video stream for video capturing with API Preference and parameters
		/// 
		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// 
		/// The `params` parameter allows to specify extra parameters encoded as pairs `(paramId_1, paramValue_1, paramId_2, paramValue_2, ...)`.
		/// See cv::VideoCaptureProperties
		/// 
		/// ## Returns
		/// `true` if the file has been successfully opened
		/// 
		/// The method first calls VideoCapture::release to close the already opened file or camera.
		#[inline]
		fn open_file_with_params(&mut self, filename: &str, api_preference: i32, params: &core::Vector<i32>) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_open_const_StringR_int_const_vectorLintGR(self.as_raw_mut_VideoCapture(), filename.opencv_as_extern(), api_preference, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Opens a camera for video capturing
		/// 
		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// 
		/// Parameters are same as the constructor VideoCapture(int index, int apiPreference = CAP_ANY)
		/// ## Returns
		/// `true` if the camera has been successfully opened.
		/// 
		/// The method first calls VideoCapture::release to close the already opened file or camera.
		/// 
		/// ## C++ default parameters
		/// * api_preference: CAP_ANY
		#[inline]
		fn open(&mut self, index: i32, api_preference: i32) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_open_int_int(self.as_raw_mut_VideoCapture(), index, api_preference, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Opens a camera for video capturing
		/// 
		/// @overload
		/// 
		/// Parameters are same as the constructor VideoCapture(int index, int apiPreference = CAP_ANY)
		/// ## Returns
		/// `true` if the camera has been successfully opened.
		/// 
		/// The method first calls VideoCapture::release to close the already opened file or camera.
		/// 
		/// ## Note
		/// This alternative version of [open] function uses the following default values for its arguments:
		/// * api_preference: CAP_ANY
		#[inline]
		fn open_def(&mut self, index: i32) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_open_int(self.as_raw_mut_VideoCapture(), index, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Opens a camera for video capturing with API Preference and parameters
		/// 
		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// 
		/// The `params` parameter allows to specify extra parameters encoded as pairs `(paramId_1, paramValue_1, paramId_2, paramValue_2, ...)`.
		/// See cv::VideoCaptureProperties
		/// 
		/// ## Returns
		/// `true` if the camera has been successfully opened.
		/// 
		/// The method first calls VideoCapture::release to close the already opened file or camera.
		#[inline]
		fn open_with_params(&mut self, index: i32, api_preference: i32, params: &core::Vector<i32>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_open_int_int_const_vectorLintGR(self.as_raw_mut_VideoCapture(), index, api_preference, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn retrieve(&mut self, image: &mut impl core::ToOutputArray, flag: i32) -> Result<bool> {
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
		/// This alternative version of [retrieve] function uses the following default values for its arguments:
		/// * flag: 0
		#[inline]
		fn retrieve_def(&mut self, image: &mut impl core::ToOutputArray) -> Result<bool> {
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
		#[inline]
		fn read(&mut self, image: &mut impl core::ToOutputArray) -> Result<bool> {
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
		#[inline]
		fn set(&mut self, prop_id: i32, value: f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_set_int_double(self.as_raw_mut_VideoCapture(), prop_id, value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Switches exceptions mode
		/// 
		/// methods raise exceptions if not successful instead of returning an error code
		#[inline]
		fn set_exception_mode(&mut self, enable: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_setExceptionMode_bool(self.as_raw_mut_VideoCapture(), enable, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// query if exception mode is active
		#[inline]
		fn get_exception_mode(&mut self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_getExceptionMode(self.as_raw_mut_VideoCapture(), ocvrs_return.as_mut_ptr()) };
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
	pub struct VideoCapture {
		ptr: *mut c_void
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
	
	impl VideoCapture {
		/// Default constructor
		/// 
		/// Note: In [videoio_c] "C API", when you finished working with video, release CvCapture structure with
		/// cvReleaseCapture(), or use Ptr\<CvCapture\> that calls cvReleaseCapture() automatically in the
		/// destructor.
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
		///    Opens a video file or a capturing device or an IP video stream for video capturing with API Preference
		/// 
		/// ## Parameters
		/// * filename: it can be:
		///    - name of video file (eg. `video.avi`)
		///    - or image sequence (eg. `img_%02d.jpg`, which will read samples like `img_00.jpg, img_01.jpg, img_02.jpg, ...`)
		///    - or URL of video stream (eg. `protocol://host:port/script_name?script_params|auth`)
		///    - or GStreamer pipeline string in gst-launch tool format in case if GStreamer is used as backend
		///       Note that each video stream or IP camera feed has its own URL scheme. Please refer to the
		///       documentation of source stream to know the right URL.
		/// * apiPreference: preferred Capture API backends to use. Can be used to enforce a specific reader
		///    implementation if multiple are available: e.g. cv::CAP_FFMPEG or cv::CAP_IMAGES or cv::CAP_DSHOW.
		/// ## See also
		/// cv::VideoCaptureAPIs
		/// 
		/// ## C++ default parameters
		/// * api_preference: CAP_ANY
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
		
		/// @overload
		/// Opens a video file or a capturing device or an IP video stream for video capturing with API Preference
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
		/// 
		/// ## Note
		/// This alternative version of [from_file] function uses the following default values for its arguments:
		/// * api_preference: CAP_ANY
		#[inline]
		pub fn from_file_def(filename: &str) -> Result<crate::videoio::VideoCapture> {
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
		///    Opens a video file or a capturing device or an IP video stream for video capturing with API Preference and parameters
		/// 
		///    The `params` parameter allows to specify extra parameters encoded as pairs `(paramId_1, paramValue_1, paramId_2, paramValue_2, ...)`.
		///    See cv::VideoCaptureProperties
		#[inline]
		pub fn from_file_with_params(filename: &str, api_preference: i32, params: &core::Vector<i32>) -> Result<crate::videoio::VideoCapture> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_VideoCapture_const_StringR_int_const_vectorLintGR(filename.opencv_as_extern(), api_preference, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
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
		///    Opens a camera for video capturing
		/// 
		/// ## Parameters
		/// * index: id of the video capturing device to open. To open default camera using default backend just pass 0.
		///    (to backward compatibility usage of camera_id + domain_offset (CAP_*) is valid when apiPreference is CAP_ANY)
		/// * apiPreference: preferred Capture API backends to use. Can be used to enforce a specific reader
		///    implementation if multiple are available: e.g. cv::CAP_DSHOW or cv::CAP_MSMF or cv::CAP_V4L.
		/// ## See also
		/// cv::VideoCaptureAPIs
		/// 
		/// ## C++ default parameters
		/// * api_preference: CAP_ANY
		#[inline]
		pub fn new(index: i32, api_preference: i32) -> Result<crate::videoio::VideoCapture> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_VideoCapture_int_int(index, api_preference, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// @overload
		/// Opens a camera for video capturing
		/// 
		/// ## Parameters
		/// * index: id of the video capturing device to open. To open default camera using default backend just pass 0.
		/// (to backward compatibility usage of camera_id + domain_offset (CAP_*) is valid when apiPreference is CAP_ANY)
		/// * apiPreference: preferred Capture API backends to use. Can be used to enforce a specific reader
		/// implementation if multiple are available: e.g. cv::CAP_DSHOW or cv::CAP_MSMF or cv::CAP_V4L.
		/// ## See also
		/// cv::VideoCaptureAPIs
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * api_preference: CAP_ANY
		#[inline]
		pub fn new_def(index: i32) -> Result<crate::videoio::VideoCapture> {
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
		///    Opens a camera for video capturing with API Preference and parameters
		/// 
		///    The `params` parameter allows to specify extra parameters encoded as pairs `(paramId_1, paramValue_1, paramId_2, paramValue_2, ...)`.
		///    See cv::VideoCaptureProperties
		#[inline]
		pub fn new_with_params(index: i32, api_preference: i32, params: &core::Vector<i32>) -> Result<crate::videoio::VideoCapture> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_VideoCapture_int_int_const_vectorLintGR(index, api_preference, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::videoio::VideoCapture::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Wait for ready frames from VideoCapture.
		/// 
		/// ## Parameters
		/// * streams: input video streams
		/// * readyIndex: stream indexes with grabbed frames (ready to use .retrieve() to fetch actual frame)
		/// * timeoutNs: number of nanoseconds (0 - infinite)
		/// ## Returns
		/// `true` if streamReady is not empty
		/// 
		/// @throws Exception %Exception on stream errors (check .isOpened() to filter out malformed streams) or VideoCapture type is not supported
		/// 
		/// The primary use of the function is in multi-camera environments.
		/// The method fills the ready state vector, grabs video frame, if camera is ready.
		/// 
		/// After this call use VideoCapture::retrieve() to decode and fetch frame data.
		/// 
		/// ## C++ default parameters
		/// * timeout_ns: 0
		#[inline]
		pub fn wait_any(streams: &core::Vector<crate::videoio::VideoCapture>, ready_index: &mut core::Vector<i32>, timeout_ns: i64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_waitAny_const_vectorLVideoCaptureGR_vectorLintGR_int64_t(streams.as_raw_VectorOfVideoCapture(), ready_index.as_raw_mut_VectorOfi32(), timeout_ns, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Wait for ready frames from VideoCapture.
		/// 
		/// ## Parameters
		/// * streams: input video streams
		/// * readyIndex: stream indexes with grabbed frames (ready to use .retrieve() to fetch actual frame)
		/// * timeoutNs: number of nanoseconds (0 - infinite)
		/// ## Returns
		/// `true` if streamReady is not empty
		/// 
		/// @throws Exception %Exception on stream errors (check .isOpened() to filter out malformed streams) or VideoCapture type is not supported
		/// 
		/// The primary use of the function is in multi-camera environments.
		/// The method fills the ready state vector, grabs video frame, if camera is ready.
		/// 
		/// After this call use VideoCapture::retrieve() to decode and fetch frame data.
		/// 
		/// ## Note
		/// This alternative version of [wait_any] function uses the following default values for its arguments:
		/// * timeout_ns: 0
		#[inline]
		pub fn wait_any_def(streams: &core::Vector<crate::videoio::VideoCapture>, ready_index: &mut core::Vector<i32>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoCapture_waitAny_const_vectorLVideoCaptureGR_vectorLintGR(streams.as_raw_VectorOfVideoCapture(), ready_index.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
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
	pub trait VideoWriterTraitConst {
		fn as_raw_VideoWriter(&self) -> *const c_void;
	
		/// Returns true if video writer has been successfully initialized.
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
		#[inline]
		fn open(&mut self, filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_open_const_StringR_int_double_Size_bool(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), fourcc, fps, frame_size.opencv_as_extern(), is_color, ocvrs_return.as_mut_ptr()) };
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
		/// This alternative version of [open] function uses the following default values for its arguments:
		/// * is_color: true
		#[inline]
		fn open_def(&mut self, filename: &str, fourcc: i32, fps: f64, frame_size: core::Size) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_open_const_StringR_int_double_Size(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), fourcc, fps, frame_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn open_with_backend(&mut self, filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_open_const_StringR_int_int_double_Size_bool(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), api_preference, fourcc, fps, frame_size.opencv_as_extern(), is_color, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @overload
		/// 
		/// ## Note
		/// This alternative version of [open_with_backend] function uses the following default values for its arguments:
		/// * is_color: true
		#[inline]
		fn open_with_backend_def(&mut self, filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_open_const_StringR_int_int_double_Size(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), api_preference, fourcc, fps, frame_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn open_1(&mut self, filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, params: &core::Vector<i32>) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_open_const_StringR_int_double_const_SizeR_const_vectorLintGR(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), fourcc, fps, &frame_size, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn open_2(&mut self, filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, params: &core::Vector<i32>) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_open_const_StringR_int_int_double_const_SizeR_const_vectorLintGR(self.as_raw_mut_VideoWriter(), filename.opencv_as_extern(), api_preference, fourcc, fps, &frame_size, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Closes the video writer.
		/// 
		/// The method is automatically called by subsequent VideoWriter::open and by the VideoWriter
		/// destructor.
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
		#[inline]
		fn write(&mut self, image: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_write_const__InputArrayR(self.as_raw_mut_VideoWriter(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
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
	pub struct VideoWriter {
		ptr: *mut c_void
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
	
	impl VideoWriter {
		/// Default constructors
		/// 
		/// The constructors/functions initialize video writers.
		/// *   On Linux FFMPEG is used to write videos;
		/// *   On Windows FFMPEG or MSWF or DSHOW is used;
		/// *   On MacOSX AVFoundation is used.
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
		/// *   On Windows FFMPEG or MSWF or DSHOW is used;
		/// *   On MacOSX AVFoundation is used.
		/// 
		/// ## Overloaded parameters
		/// 
		/// ## Parameters
		/// * filename: Name of the output video file.
		/// * fourcc: 4-character code of codec used to compress the frames. For example,
		///    VideoWriter::fourcc('P','I','M','1') is a MPEG-1 codec, VideoWriter::fourcc('M','J','P','G')
		///    is a motion-jpeg codec etc. List of codes can be obtained at
		///    [MSDN](https://docs.microsoft.com/en-us/windows/win32/medfound/video-fourccs) page
		///    or with this [archived page](https://web.archive.org/web/20220316062600/http://www.fourcc.org/codecs.php)
		///    of the fourcc site for a more complete list). FFMPEG backend with MP4 container natively uses
		///    other values as fourcc code: see [ObjectType](http://mp4ra.org/#/codecs),
		///    so you may receive a warning message from OpenCV about fourcc code conversion.
		/// * fps: Framerate of the created video stream.
		/// * frameSize: Size of the video frames.
		/// * isColor: If it is not zero, the encoder will expect and encode color frames, otherwise it
		///    will work with grayscale frames.
		/// 
		///    @b Tips:
		///    - With some backends `fourcc=-1` pops up the codec selection dialog from the system.
		///    - To save image sequence use a proper filename (eg. `img_%02d.jpg`) and `fourcc=0`
		///       OR `fps=0`. Use uncompressed image format (eg. `img_%02d.BMP`) to save raw frames.
		///    - Most codecs are lossy. If you want lossless video file you need to use a lossless codecs
		///       (eg. FFMPEG FFV1, Huffman HFYU, Lagarith LAGS, etc...)
		///    - If FFMPEG is enabled, using `codec=0; fps=0;` you can create an uncompressed (raw) video file.
		/// 
		/// ## C++ default parameters
		/// * is_color: true
		#[inline]
		pub fn new(filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<crate::videoio::VideoWriter> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_double_Size_bool(filename.opencv_as_extern(), fourcc, fps, frame_size.opencv_as_extern(), is_color, ocvrs_return.as_mut_ptr()) };
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
		/// will work with grayscale frames.
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
		#[inline]
		pub fn new_def(filename: &str, fourcc: i32, fps: f64, frame_size: core::Size) -> Result<crate::videoio::VideoWriter> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_double_Size(filename.opencv_as_extern(), fourcc, fps, frame_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Default constructors
		/// 
		/// The constructors/functions initialize video writers.
		/// *   On Linux FFMPEG is used to write videos;
		/// *   On Windows FFMPEG or MSWF or DSHOW is used;
		/// *   On MacOSX AVFoundation is used.
		/// 
		/// ## Overloaded parameters
		/// 
		///    The `apiPreference` parameter allows to specify API backends to use. Can be used to enforce a specific reader implementation
		///    if multiple are available: e.g. cv::CAP_FFMPEG or cv::CAP_GSTREAMER.
		/// 
		/// ## C++ default parameters
		/// * is_color: true
		#[inline]
		pub fn new_with_backend(filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, is_color: bool) -> Result<crate::videoio::VideoWriter> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size_bool(filename.opencv_as_extern(), api_preference, fourcc, fps, frame_size.opencv_as_extern(), is_color, ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		pub fn new_with_backend_def(filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size) -> Result<crate::videoio::VideoWriter> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size(filename.opencv_as_extern(), api_preference, fourcc, fps, frame_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Default constructors
		/// 
		/// The constructors/functions initialize video writers.
		/// *   On Linux FFMPEG is used to write videos;
		/// *   On Windows FFMPEG or MSWF or DSHOW is used;
		/// *   On MacOSX AVFoundation is used.
		/// 
		/// ## Overloaded parameters
		/// 
		///      * The `params` parameter allows to specify extra encoder parameters encoded as pairs (paramId_1, paramValue_1, paramId_2, paramValue_2, ... .)
		///      * see cv::VideoWriterProperties
		#[inline]
		pub fn new_1(filename: &str, fourcc: i32, fps: f64, frame_size: core::Size, params: &core::Vector<i32>) -> Result<crate::videoio::VideoWriter> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_double_const_SizeR_const_vectorLintGR(filename.opencv_as_extern(), fourcc, fps, &frame_size, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::videoio::VideoWriter::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Default constructors
		/// 
		/// The constructors/functions initialize video writers.
		/// *   On Linux FFMPEG is used to write videos;
		/// *   On Windows FFMPEG or MSWF or DSHOW is used;
		/// *   On MacOSX AVFoundation is used.
		/// 
		/// ## Overloaded parameters
		#[inline]
		pub fn new_2(filename: &str, api_preference: i32, fourcc: i32, fps: f64, frame_size: core::Size, params: &core::Vector<i32>) -> Result<crate::videoio::VideoWriter> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VideoWriter_VideoWriter_const_StringR_int_int_double_const_SizeR_const_vectorLintGR(filename.opencv_as_extern(), api_preference, fourcc, fps, &frame_size, params.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
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
}

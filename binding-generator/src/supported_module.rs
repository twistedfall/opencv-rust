// !! Update `pub static SUPPORTED_MODULES` in `enums.rs` when changing this
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SupportedModule {
	/// alphamat
	AlphaMat,
	/// aruco
	Aruco,
	/// aruco_detector
	ArucoDetector,
	/// barcode
	Barcode,
	/// bgsegm
	BgSegm,
	/// bioinspired
	Bioinspired,
	/// calib
	Calib,
	/// calib3d
	Calib3d,
	/// cannops
	///
	/// Not supported
	CannOps,
	/// ccalib
	CCalib,
	/// core
	///
	/// Always present
	Core,
	/// cudaarithm
	CudaArithm,
	/// cudabgsegm
	CudaBgSegm,
	/// cudacodec
	CudaCodec,
	/// cudafeatures2d
	CudaFeatures2d,
	/// cudafilters
	CudaFilters,
	/// cudaimgproc
	CudaImgProc,
	/// cudalegacy
	CudaLegacy,
	/// cudaobjdetect
	CudaObjDetect,
	/// cudaoptflow
	CudaOptFlow,
	/// cudastereo
	CudaStereo,
	/// cudawarping
	CudaWarping,
	/// cudev
	///
	/// Not supported
	CuDev,
	/// cvv
	Cvv,
	/// dnn
	Dnn,
	/// dnn_superres
	DnnSuperRes,
	/// dpm
	Dpm,
	/// face
	Face,
	/// features
	Features,
	/// features2d
	Features2d,
	/// flann
	Flann,
	/// freetype
	Freetype,
	/// fuzzy
	Fuzzy,
	/// gapi
	Gapi,
	/// geometry
	Geometry,
	/// hdf
	Hdf,
	/// hfs
	Hfs,
	/// highgui
	HighGui,
	/// img_hash
	ImgHash,
	/// imgcodecs
	ImgCodecs,
	/// imgproc
	ImgProc,
	/// intensity_transform
	IntensityTransform,
	/// line_descriptor
	LineDescriptor,
	/// mcc
	Mcc,
	/// ml
	Ml,
	/// objdetect
	ObjDetect,
	/// optflow
	OptFlow,
	/// ovis
	Ovis,
	/// phase_unwrapping
	PhaseUnwrapping,
	/// photo
	Photo,
	/// plot
	Plot,
	/// ptcloud
	PtCloud,
	/// quality
	Quality,
	/// rapid
	Rapid,
	/// rgbd
	Rgbd,
	/// saliency
	Saliency,
	/// sfm
	Sfm,
	/// shape
	Shape,
	/// signal
	Signal,
	/// stereo
	Stereo,
	/// stitching
	Stitching,
	/// structured_light
	StructuredLight,
	/// superres
	SuperRes,
	/// surface_matching
	SurfaceMatching,
	/// text
	Text,
	/// tracking
	Tracking,
	/// video
	Video,
	/// videoio
	VideoIo,
	/// videostab
	VideoStab,
	/// viz
	Viz,
	/// wechat_qrcode
	WechatQrCode,
	/// xfeatures2d
	XFeatures2d,
	/// ximgproc
	XImgProc,
	/// xobjdetect
	XObjDetect,
	/// xphoto
	XPhoto,
	/// xstereo
	XStereo,
}

impl SupportedModule {
	pub fn try_from_opencv_name(name: &str) -> Option<Self> {
		let name_is = |check_name| name.eq_ignore_ascii_case(check_name);
		match () {
			_ if name_is("alphamat") => Some(Self::AlphaMat),
			_ if name_is("aruco") => Some(Self::Aruco),
			_ if name_is("aruco_detector") => Some(Self::ArucoDetector),
			_ if name_is("barcode") => Some(Self::Barcode),
			_ if name_is("bgsegm") => Some(Self::BgSegm),
			_ if name_is("bioinspired") => Some(Self::Bioinspired),
			_ if name_is("calib") => Some(Self::Calib),
			_ if name_is("calib3d") => Some(Self::Calib3d),
			_ if name_is("cannops") => Some(Self::CannOps),
			_ if name_is("ccalib") => Some(Self::CCalib),
			_ if name_is("core") => Some(Self::Core),
			_ if name_is("cudaarithm") => Some(Self::CudaArithm),
			_ if name_is("cudabgsegm") => Some(Self::CudaBgSegm),
			_ if name_is("cudacodec") => Some(Self::CudaCodec),
			_ if name_is("cudafeatures2d") => Some(Self::CudaFeatures2d),
			_ if name_is("cudafilters") => Some(Self::CudaFilters),
			_ if name_is("cudaimgproc") => Some(Self::CudaImgProc),
			_ if name_is("cudalegacy") => Some(Self::CudaLegacy),
			_ if name_is("cudaobjdetect") => Some(Self::CudaObjDetect),
			_ if name_is("cudaoptflow") => Some(Self::CudaOptFlow),
			_ if name_is("cudastereo") => Some(Self::CudaStereo),
			_ if name_is("cudawarping") => Some(Self::CudaWarping),
			_ if name_is("cudev") => Some(Self::CuDev),
			_ if name_is("cvv") => Some(Self::Cvv),
			_ if name_is("dnn") => Some(Self::Dnn),
			_ if name_is("dnn_superres") => Some(Self::DnnSuperRes),
			_ if name_is("dpm") => Some(Self::Dpm),
			_ if name_is("face") => Some(Self::Face),
			_ if name_is("features") => Some(Self::Features),
			_ if name_is("features2d") => Some(Self::Features2d),
			_ if name_is("flann") => Some(Self::Flann),
			_ if name_is("freetype") => Some(Self::Freetype),
			_ if name_is("fuzzy") => Some(Self::Fuzzy),
			_ if name_is("gapi") => Some(Self::Gapi),
			_ if name_is("geometry") => Some(Self::Geometry),
			_ if name_is("hdf") => Some(Self::Hdf),
			_ if name_is("hfs") => Some(Self::Hfs),
			_ if name_is("highgui") => Some(Self::HighGui),
			_ if name_is("img_hash") => Some(Self::ImgHash),
			_ if name_is("imgcodecs") => Some(Self::ImgCodecs),
			_ if name_is("imgproc") => Some(Self::ImgProc),
			_ if name_is("intensity_transform") => Some(Self::IntensityTransform),
			_ if name_is("line_descriptor") => Some(Self::LineDescriptor),
			_ if name_is("mcc") => Some(Self::Mcc),
			_ if name_is("ml") => Some(Self::Ml),
			_ if name_is("objdetect") => Some(Self::ObjDetect),
			_ if name_is("optflow") => Some(Self::OptFlow),
			_ if name_is("ovis") => Some(Self::Ovis),
			_ if name_is("phase_unwrapping") => Some(Self::PhaseUnwrapping),
			_ if name_is("photo") => Some(Self::Photo),
			_ if name_is("plot") => Some(Self::Plot),
			_ if name_is("ptcloud") => Some(Self::PtCloud),
			_ if name_is("quality") => Some(Self::Quality),
			_ if name_is("rapid") => Some(Self::Rapid),
			_ if name_is("rgbd") => Some(Self::Rgbd),
			_ if name_is("saliency") => Some(Self::Saliency),
			_ if name_is("sfm") => Some(Self::Sfm),
			_ if name_is("shape") => Some(Self::Shape),
			_ if name_is("signal") => Some(Self::Signal),
			_ if name_is("stereo") => Some(Self::Stereo),
			_ if name_is("stitching") => Some(Self::Stitching),
			_ if name_is("structured_light") => Some(Self::StructuredLight),
			_ if name_is("superres") => Some(Self::SuperRes),
			_ if name_is("surface_matching") => Some(Self::SurfaceMatching),
			_ if name_is("text") => Some(Self::Text),
			_ if name_is("tracking") => Some(Self::Tracking),
			_ if name_is("video") => Some(Self::Video),
			_ if name_is("videoio") => Some(Self::VideoIo),
			_ if name_is("videostab") => Some(Self::VideoStab),
			_ if name_is("viz") => Some(Self::Viz),
			_ if name_is("wechat_qrcode") => Some(Self::WechatQrCode),
			_ if name_is("xfeatures2d") => Some(Self::XFeatures2d),
			_ if name_is("ximgproc") => Some(Self::XImgProc),
			_ if name_is("xobjdetect") => Some(Self::XObjDetect),
			_ if name_is("xphoto") => Some(Self::XPhoto),
			_ if name_is("xstereo") => Some(Self::XStereo),
			_ => None,
		}
	}

	pub fn opencv_name(self) -> &'static str {
		match self {
			Self::AlphaMat => "alphamat",
			Self::Aruco => "aruco",
			Self::ArucoDetector => "aruco_detector",
			Self::Barcode => "barcode",
			Self::BgSegm => "bgsegm",
			Self::Bioinspired => "bioinspired",
			Self::Calib => "calib",
			Self::Calib3d => "calib3d",
			Self::CannOps => "cannops",
			Self::CCalib => "ccalib",
			Self::Core => "core",
			Self::CudaArithm => "cudaarithm",
			Self::CudaBgSegm => "cudabgsegm",
			Self::CudaCodec => "cudacodec",
			Self::CudaFeatures2d => "cudafeatures2d",
			Self::CudaFilters => "cudafilters",
			Self::CudaImgProc => "cudaimgproc",
			Self::CudaLegacy => "cudalegacy",
			Self::CudaObjDetect => "cudaobjdetect",
			Self::CudaOptFlow => "cudaoptflow",
			Self::CudaStereo => "cudastereo",
			Self::CuDev => "cudev",
			Self::CudaWarping => "cudawarping",
			Self::Cvv => "cvv",
			Self::Dnn => "dnn",
			Self::DnnSuperRes => "dnn_superres",
			Self::Dpm => "dpm",
			Self::Face => "face",
			Self::Features => "features",
			Self::Features2d => "features2d",
			Self::Flann => "flann",
			Self::Freetype => "freetype",
			Self::Fuzzy => "fuzzy",
			Self::Gapi => "gapi",
			Self::Geometry => "geometry",
			Self::Hdf => "hdf",
			Self::Hfs => "hfs",
			Self::HighGui => "highgui",
			Self::ImgCodecs => "imgcodecs",
			Self::ImgHash => "img_hash",
			Self::ImgProc => "imgproc",
			Self::IntensityTransform => "intensity_transform",
			Self::LineDescriptor => "line_descriptor",
			Self::Mcc => "mcc",
			Self::Ml => "ml",
			Self::ObjDetect => "objdetect",
			Self::OptFlow => "optflow",
			Self::Ovis => "ovis",
			Self::PhaseUnwrapping => "phase_unwrapping",
			Self::Photo => "photo",
			Self::Plot => "plot",
			Self::PtCloud => "ptcloud",
			Self::Quality => "quality",
			Self::Rapid => "rapid",
			Self::Rgbd => "rgbd",
			Self::Saliency => "saliency",
			Self::Sfm => "sfm",
			Self::Shape => "shape",
			Self::Signal => "signal",
			Self::Stereo => "stereo",
			Self::Stitching => "stitching",
			Self::StructuredLight => "structured_light",
			Self::SuperRes => "superres",
			Self::SurfaceMatching => "surface_matching",
			Self::Text => "text",
			Self::Tracking => "tracking",
			Self::Video => "video",
			Self::VideoIo => "videoio",
			Self::VideoStab => "videostab",
			Self::Viz => "viz",
			Self::WechatQrCode => "wechat_qrcode",
			Self::XFeatures2d => "xfeatures2d",
			Self::XImgProc => "ximgproc",
			Self::XObjDetect => "xobjdetect",
			Self::XPhoto => "xphoto",
			Self::XStereo => "xstereo",
		}
	}

	pub fn rust_name(self) -> &'static str {
		self.opencv_name()
	}
}

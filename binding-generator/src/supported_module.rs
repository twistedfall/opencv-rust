#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SupportedModule {
	/// 3d
	ThreeD,
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
		match name {
			"3d" => Some(Self::ThreeD),
			"alphamat" => Some(Self::AlphaMat),
			"aruco" => Some(Self::Aruco),
			"aruco_detector" => Some(Self::ArucoDetector),
			"barcode" => Some(Self::Barcode),
			"bgsegm" => Some(Self::BgSegm),
			"bioinspired" => Some(Self::Bioinspired),
			"calib" => Some(Self::Calib),
			"calib3d" => Some(Self::Calib3d),
			"cannops" => Some(Self::CannOps),
			"ccalib" => Some(Self::CCalib),
			"core" => Some(Self::Core),
			"cudaarithm" => Some(Self::CudaArithm),
			"cudabgsegm" => Some(Self::CudaBgSegm),
			"cudacodec" => Some(Self::CudaCodec),
			"cudafeatures2d" => Some(Self::CudaFeatures2d),
			"cudafilters" => Some(Self::CudaFilters),
			"cudaimgproc" => Some(Self::CudaImgProc),
			"cudalegacy" => Some(Self::CudaLegacy),
			"cudaobjdetect" => Some(Self::CudaObjDetect),
			"cudaoptflow" => Some(Self::CudaOptFlow),
			"cudastereo" => Some(Self::CudaStereo),
			"cudawarping" => Some(Self::CudaWarping),
			"cudev" => Some(Self::CuDev),
			"cvv" => Some(Self::Cvv),
			"dnn" => Some(Self::Dnn),
			"dnn_superres" => Some(Self::DnnSuperRes),
			"dpm" => Some(Self::Dpm),
			"face" => Some(Self::Face),
			"features" => Some(Self::Features),
			"features2d" => Some(Self::Features2d),
			"flann" => Some(Self::Flann),
			"freetype" => Some(Self::Freetype),
			"fuzzy" => Some(Self::Fuzzy),
			"gapi" => Some(Self::Gapi),
			"hdf" => Some(Self::Hdf),
			"hfs" => Some(Self::Hfs),
			"highgui" => Some(Self::HighGui),
			"img_hash" => Some(Self::ImgHash),
			"imgcodecs" => Some(Self::ImgCodecs),
			"imgproc" => Some(Self::ImgProc),
			"intensity_transform" => Some(Self::IntensityTransform),
			"line_descriptor" => Some(Self::LineDescriptor),
			"mcc" => Some(Self::Mcc),
			"ml" => Some(Self::Ml),
			"objdetect" => Some(Self::ObjDetect),
			"optflow" => Some(Self::OptFlow),
			"ovis" => Some(Self::Ovis),
			"phase_unwrapping" => Some(Self::PhaseUnwrapping),
			"photo" => Some(Self::Photo),
			"plot" => Some(Self::Plot),
			"quality" => Some(Self::Quality),
			"rapid" => Some(Self::Rapid),
			"rgbd" => Some(Self::Rgbd),
			"saliency" => Some(Self::Saliency),
			"sfm" => Some(Self::Sfm),
			"shape" => Some(Self::Shape),
			"signal" => Some(Self::Signal),
			"stereo" => Some(Self::Stereo),
			"stitching" => Some(Self::Stitching),
			"structured_light" => Some(Self::StructuredLight),
			"superres" => Some(Self::SuperRes),
			"surface_matching" => Some(Self::SurfaceMatching),
			"text" => Some(Self::Text),
			"tracking" => Some(Self::Tracking),
			"video" => Some(Self::Video),
			"videoio" => Some(Self::VideoIo),
			"videostab" => Some(Self::VideoStab),
			"viz" => Some(Self::Viz),
			"wechat_qrcode" => Some(Self::WechatQrCode),
			"xfeatures2d" => Some(Self::XFeatures2d),
			"ximgproc" => Some(Self::XImgProc),
			"xobjdetect" => Some(Self::XObjDetect),
			"xphoto" => Some(Self::XPhoto),
			"xstereo" => Some(Self::XStereo),
			_ => None,
		}
	}

	pub fn opencv_name(self) -> &'static str {
		match self {
			Self::ThreeD => "3d",
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
			Self::CudaWarping => "cudawarping",
			Self::CuDev => "cudev",
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
			Self::Hdf => "hdf",
			Self::Hfs => "hfs",
			Self::HighGui => "highgui",
			Self::ImgHash => "img_hash",
			Self::ImgCodecs => "imgcodecs",
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

	pub fn rust_safe_name(self) -> &'static str {
		match self {
			Self::ThreeD => "mod_3d",
			_ => self.opencv_name(),
		}
	}
}

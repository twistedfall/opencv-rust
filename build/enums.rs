use opencv_binding_generator::SupportedModule;

pub static SUPPORTED_MODULES: [SupportedModule; 73] = [
	SupportedModule::ThreeD,
	SupportedModule::AlphaMat,
	SupportedModule::Aruco,
	SupportedModule::ArucoDetector,
	SupportedModule::Barcode,
	SupportedModule::BgSegm,
	SupportedModule::Bioinspired,
	SupportedModule::Calib,
	SupportedModule::Calib3d,
	SupportedModule::CCalib,
	SupportedModule::Core,
	SupportedModule::CudaArithm,
	SupportedModule::CudaBgSegm,
	SupportedModule::CudaCodec,
	SupportedModule::CudaFeatures2d,
	SupportedModule::CudaFilters,
	SupportedModule::CudaImgProc,
	SupportedModule::CudaLegacy,
	SupportedModule::CudaObjDetect,
	SupportedModule::CudaOptFlow,
	SupportedModule::CudaStereo,
	SupportedModule::CudaWarping,
	SupportedModule::Cvv,
	SupportedModule::Dnn,
	SupportedModule::DnnSuperRes,
	SupportedModule::Dpm,
	SupportedModule::Face,
	SupportedModule::Features,
	SupportedModule::Features2d,
	SupportedModule::Flann,
	SupportedModule::Freetype,
	SupportedModule::Fuzzy,
	SupportedModule::Gapi,
	SupportedModule::Hdf,
	SupportedModule::Hfs,
	SupportedModule::HighGui,
	SupportedModule::ImgHash,
	SupportedModule::ImgCodecs,
	SupportedModule::ImgProc,
	SupportedModule::IntensityTransform,
	SupportedModule::LineDescriptor,
	SupportedModule::Mcc,
	SupportedModule::Ml,
	SupportedModule::ObjDetect,
	SupportedModule::OptFlow,
	SupportedModule::Ovis,
	SupportedModule::PhaseUnwrapping,
	SupportedModule::Photo,
	SupportedModule::Plot,
	SupportedModule::Quality,
	SupportedModule::Rapid,
	SupportedModule::Rgbd,
	SupportedModule::Saliency,
	SupportedModule::Sfm,
	SupportedModule::Shape,
	SupportedModule::Signal,
	SupportedModule::Stereo,
	SupportedModule::Stitching,
	SupportedModule::StructuredLight,
	SupportedModule::SuperRes,
	SupportedModule::SurfaceMatching,
	SupportedModule::Text,
	SupportedModule::Tracking,
	SupportedModule::Video,
	SupportedModule::VideoIo,
	SupportedModule::VideoStab,
	SupportedModule::Viz,
	SupportedModule::WechatQrCode,
	SupportedModule::XFeatures2d,
	SupportedModule::XImgProc,
	SupportedModule::XObjDetect,
	SupportedModule::XPhoto,
	SupportedModule::XStereo,
];

pub static SUPPORTED_INHERENT_FEATURES: [InherentFeature; 4] = [
	InherentFeature::Opencl,
	InherentFeature::Cuda,
	InherentFeature::Hfloat,
	InherentFeature::AlgorithmHint,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InherentFeature {
	/// OpenCL is enabled (HAVE_OPENCL, WITH_OPENCL=ON)
	Opencl,
	/// CUDA is enabled (HAVE_CUDA, WITH_CUDA=ON)
	Cuda,
	/// not an actual OpenCV feature, half-float support was added in 4.10+
	Hfloat,
	/// not an actual OpenCV feature, a considerable API update with the introduction of AlgorithmHint argument in 4.11+
	AlgorithmHint,
}

impl InherentFeature {
	pub fn try_from_str(s: &str) -> Option<Self> {
		match s {
			"opencl" => Some(Self::Opencl),
			"cuda" => Some(Self::Cuda),
			"hfloat" => Some(Self::Hfloat),
			"algorithm_hint" => Some(Self::AlgorithmHint),
			_ => None,
		}
	}

	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Opencl => "opencl",
			Self::Cuda => "cuda",
			Self::Hfloat => "hfloat",
			Self::AlgorithmHint => "algorithm_hint",
		}
	}
}

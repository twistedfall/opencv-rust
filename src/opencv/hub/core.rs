#![allow(unused_parens)]
//! # Core functionality
//!    # Basic structures
//!    # C structures and operations
//!        # Connections with C++
//!    # Operations on arrays
//!    # Asynchronous API
//!    # XML/YAML Persistence
//!    # Clustering
//!    # Utility and system functions and macros
//!        # Logging facilities
//!        # SSE utilities
//!        # NEON utilities
//!        # VSX utilities
//!        # Softfloat support
//!        # Utility functions for OpenCV samples
//!    # OpenGL interoperability
//!    # Intel IPP Asynchronous C/C++ Converters
//!    # Optimization Algorithms
//!    # DirectX interoperability
//!    # Eigen support
//!    # OpenCL support
//!    # Intel VA-API/OpenCL (CL-VA) interoperability
//!    # Hardware Acceleration Layer
//!        # Functions
//!        # Interface
//!        # Universal intrinsics
//!            # Private implementation helpers
//!        # Low-level API for external libraries / plugins
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::HammingTrait, super::Detail_CheckContextTrait, super::Matx_AddOpTrait, super::Matx_SubOpTrait, super::Matx_ScaleOpTrait, super::Matx_MulOpTrait, super::Matx_DivOpTrait, super::Matx_MatMulOpTrait, super::Matx_TOpTrait, super::RotatedRectTrait, super::RangeTrait, super::_InputArrayTrait, super::_OutputArrayTrait, super::_InputOutputArrayTrait, super::UMatDataTrait, super::MatSizeTrait, super::MatStepTrait, super::MatTrait, super::UMatTrait, super::SparseMat_HdrTrait, super::SparseMat_NodeTrait, super::SparseMatTrait, super::MatConstIteratorTrait, super::SparseMatConstIteratorTrait, super::SparseMatIteratorTrait, super::MatOp, super::MatExprTrait, super::FileStorageTrait, super::FileNodeTrait, super::FileNodeIteratorTrait, super::WriteStructContextTrait, super::ExceptionTrait, super::PCATrait, super::LDATrait, super::SVDTrait, super::RNGTrait, super::RNG_MT19937Trait, super::Formatted, super::Formatter, super::AlgorithmTrait, super::TickMeterTrait, super::ParallelLoopBody, super::CommandLineParserTrait, super::TLSDataContainer, super::NodeDataTrait, super::MinProblemSolver_Function, super::MinProblemSolver, super::DownhillSolver, super::ConjGradSolver, super::DeviceTrait, super::ContextTrait, super::PlatformTrait, super::QueueTrait, super::KernelArgTrait, super::KernelTrait, super::ProgramTrait, super::ProgramSourceTrait, super::PlatformInfoTrait, super::Image2DTrait, super::TimerTrait, super::GpuMat_Allocator, super::GpuMatTrait, super::BufferPoolTrait, super::HostMemTrait, super::StreamTrait, super::EventTrait, super::TargetArchsTrait, super::DeviceInfoTrait, super::AsyncArrayTrait, super::AsyncPromiseTrait, super::LogTagTrait };
}

pub const ACCESS_FAST: i32 = 67108864;
pub const ACCESS_MASK: i32 = 50331648;
pub const ACCESS_READ: i32 = 16777216;
pub const ACCESS_RW: i32 = 50331648;
pub const ACCESS_WRITE: i32 = 33554432;
/// `iiiiii|abcdefgh|iiiiiii`  with some specified `i`
pub const BORDER_CONSTANT: i32 = 0;
/// same as BORDER_REFLECT_101
pub const BORDER_DEFAULT: i32 = 4;
/// do not look outside of ROI
pub const BORDER_ISOLATED: i32 = 16;
/// `fedcba|abcdefgh|hgfedcb`
pub const BORDER_REFLECT: i32 = 2;
/// same as BORDER_REFLECT_101
pub const BORDER_REFLECT101: i32 = 4;
/// `gfedcb|abcdefgh|gfedcba`
pub const BORDER_REFLECT_101: i32 = 4;
/// `aaaaaa|abcdefgh|hhhhhhh`
pub const BORDER_REPLICATE: i32 = 1;
/// `uvwxyz|abcdefgh|ijklmno`
pub const BORDER_TRANSPARENT: i32 = 5;
/// `cdefgh|abcdefgh|abcdefg`
pub const BORDER_WRAP: i32 = 3;
/// incorrect input align
pub const BadAlign: i32 = -21;
pub const BadAlphaChannel: i32 = -18;
/// input COI is not supported
pub const BadCOI: i32 = -24;
pub const BadCallBack: i32 = -22;
pub const BadDataPtr: i32 = -12;
/// input image depth is not supported by the function
pub const BadDepth: i32 = -17;
/// image size is invalid
pub const BadImageSize: i32 = -10;
pub const BadModelOrChSeq: i32 = -14;
pub const BadNumChannel1U: i32 = -16;
/// bad number of channels, for example, some functions accept only single channel matrices.
pub const BadNumChannels: i32 = -15;
/// offset is invalid
pub const BadOffset: i32 = -11;
/// number of dimensions is out of range
pub const BadOrder: i32 = -19;
/// incorrect input origin
pub const BadOrigin: i32 = -20;
/// incorrect input roi
pub const BadROISize: i32 = -25;
/// image step is wrong, this may happen for a non-continuous matrix.
pub const BadStep: i32 = -13;
pub const BadTileSize: i32 = -23;
/// src1 is equal to src2.
pub const CMP_EQ: i32 = 0;
/// src1 is greater than or equal to src2.
pub const CMP_GE: i32 = 2;
/// src1 is greater than src2.
pub const CMP_GT: i32 = 1;
/// src1 is less than or equal to src2.
pub const CMP_LE: i32 = 4;
/// src1 is less than src2.
pub const CMP_LT: i32 = 3;
/// src1 is unequal to src2.
pub const CMP_NE: i32 = 5;
/// If the flag is
/// specified, all the input vectors are stored as columns of the samples matrix. mean should be a
/// single-column vector in this case.
pub const COVAR_COLS: i32 = 16;
/// The output covariance matrix is calculated as:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bscale%7D%20%20%20%5Ccdot%20%20%5B%20%20%5Ctexttt%7Bvects%7D%20%20%5B0%5D%2D%20%20%5Ctexttt%7Bmean%7D%20%20%2C%20%5Ctexttt%7Bvects%7D%20%20%5B1%5D%2D%20%20%5Ctexttt%7Bmean%7D%20%20%2C%2E%2E%2E%5D%20%20%5Ccdot%20%20%5B%20%5Ctexttt%7Bvects%7D%20%20%5B0%5D%2D%20%5Ctexttt%7Bmean%7D%20%20%2C%20%5Ctexttt%7Bvects%7D%20%20%5B1%5D%2D%20%5Ctexttt%7Bmean%7D%20%20%2C%2E%2E%2E%5D%5ET%2C)
/// covar will be a square matrix of the same size as the total number of elements in each input
/// vector. One and only one of #COVAR_SCRAMBLED and #COVAR_NORMAL must be specified.
pub const COVAR_NORMAL: i32 = 1;
/// If the flag is
/// specified, all the input vectors are stored as rows of the samples matrix. mean should be a
/// single-row vector in this case.
pub const COVAR_ROWS: i32 = 8;
/// If the flag is specified, the covariance matrix is scaled. In the
/// "normal" mode, scale is 1./nsamples . In the "scrambled" mode, scale is the reciprocal of the
/// total number of elements in each input vector. By default (if the flag is not specified), the
/// covariance matrix is not scaled ( scale=1 ).
pub const COVAR_SCALE: i32 = 4;
/// The output covariance matrix is calculated as:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bscale%7D%20%20%20%5Ccdot%20%20%5B%20%20%5Ctexttt%7Bvects%7D%20%20%5B0%5D%2D%20%20%5Ctexttt%7Bmean%7D%20%20%2C%20%5Ctexttt%7Bvects%7D%20%20%5B1%5D%2D%20%20%5Ctexttt%7Bmean%7D%20%20%2C%2E%2E%2E%5D%5ET%20%20%5Ccdot%20%20%5B%20%5Ctexttt%7Bvects%7D%20%20%5B0%5D%2D%20%5Ctexttt%7Bmean%7D%20%20%2C%20%5Ctexttt%7Bvects%7D%20%20%5B1%5D%2D%20%5Ctexttt%7Bmean%7D%20%20%2C%2E%2E%2E%5D%2C)
/// The covariance matrix will be nsamples x nsamples. Such an unusual covariance matrix is used
/// for fast PCA of a set of very large vectors (see, for example, the EigenFaces technique for
/// face recognition). Eigenvalues of this "scrambled" matrix match the eigenvalues of the true
/// covariance matrix. The "true" eigenvectors can be easily calculated from the eigenvectors of
/// the "scrambled" covariance matrix.
pub const COVAR_SCRAMBLED: i32 = 0;
/// If the flag is specified, the function does not calculate mean from
/// the input vectors but, instead, uses the passed mean vector. This is useful if mean has been
/// pre-calculated or known in advance, or if the covariance matrix is calculated by parts. In
/// this case, mean is not a mean vector of the input sub-set of vectors but rather the mean
/// vector of the whole set.
pub const COVAR_USE_AVG: i32 = 2;
pub const CPU_AVX: i32 = 10;
pub const CPU_AVX2: i32 = 11;
/// Cascade Lake with AVX-512F/CD/BW/DQ/VL/VNNI
pub const CPU_AVX512_CLX: i32 = 261;
/// Cannon Lake with AVX-512F/CD/BW/DQ/VL/IFMA/VBMI
pub const CPU_AVX512_CNL: i32 = 260;
/// Common instructions AVX-512F/CD for all CPUs that support AVX-512
pub const CPU_AVX512_COMMON: i32 = 257;
/// Ice Lake with AVX-512F/CD/BW/DQ/VL/IFMA/VBMI/VNNI/VBMI2/BITALG/VPOPCNTDQ
pub const CPU_AVX512_ICL: i32 = 262;
/// Knights Landing with AVX-512F/CD/ER/PF
pub const CPU_AVX512_KNL: i32 = 258;
/// Knights Mill with AVX-512F/CD/ER/PF/4FMAPS/4VNNIW/VPOPCNTDQ
pub const CPU_AVX512_KNM: i32 = 259;
/// Skylake-X with AVX-512F/CD/BW/DQ/VL
pub const CPU_AVX512_SKX: i32 = 256;
pub const CPU_AVX_5124FMAPS: i32 = 27;
pub const CPU_AVX_5124VNNIW: i32 = 26;
pub const CPU_AVX_512BITALG: i32 = 24;
pub const CPU_AVX_512BW: i32 = 14;
pub const CPU_AVX_512CD: i32 = 15;
pub const CPU_AVX_512DQ: i32 = 16;
pub const CPU_AVX_512ER: i32 = 17;
pub const CPU_AVX_512F: i32 = 13;
pub const CPU_AVX_512IFMA: i32 = 18;
pub const CPU_AVX_512IFMA512: i32 = 18;
pub const CPU_AVX_512PF: i32 = 19;
pub const CPU_AVX_512VBMI: i32 = 20;
pub const CPU_AVX_512VBMI2: i32 = 22;
pub const CPU_AVX_512VL: i32 = 21;
pub const CPU_AVX_512VNNI: i32 = 23;
pub const CPU_AVX_512VPOPCNTDQ: i32 = 25;
pub const CPU_FMA3: i32 = 12;
pub const CPU_FP16: i32 = 9;
pub const CPU_MAX_FEATURE: i32 = 512;
pub const CPU_MMX: i32 = 1;
pub const CPU_MSA: i32 = 150;
pub const CPU_NEON: i32 = 100;
pub const CPU_POPCNT: i32 = 8;
pub const CPU_SSE: i32 = 2;
pub const CPU_SSE2: i32 = 3;
pub const CPU_SSE3: i32 = 4;
pub const CPU_SSE4_1: i32 = 6;
pub const CPU_SSE4_2: i32 = 7;
pub const CPU_SSSE3: i32 = 5;
pub const CPU_VSX: i32 = 200;
pub const CPU_VSX3: i32 = 201;
pub const CV_16F: i32 = 7;
pub const CV_16FC1: i32 = CV_MAKETYPE(CV_16F,1);
pub const CV_16FC2: i32 = CV_MAKETYPE(CV_16F,2);
pub const CV_16FC3: i32 = CV_MAKETYPE(CV_16F,3);
pub const CV_16FC4: i32 = CV_MAKETYPE(CV_16F,4);
pub const CV_16S: i32 = 3;
pub const CV_16SC1: i32 = CV_MAKETYPE(CV_16S,1);
pub const CV_16SC2: i32 = CV_MAKETYPE(CV_16S,2);
pub const CV_16SC3: i32 = CV_MAKETYPE(CV_16S,3);
pub const CV_16SC4: i32 = CV_MAKETYPE(CV_16S,4);
pub const CV_16U: i32 = 2;
pub const CV_16UC1: i32 = CV_MAKETYPE(CV_16U,1);
pub const CV_16UC2: i32 = CV_MAKETYPE(CV_16U,2);
pub const CV_16UC3: i32 = CV_MAKETYPE(CV_16U,3);
pub const CV_16UC4: i32 = CV_MAKETYPE(CV_16U,4);
pub const CV_2PI: f64 = 6.283185307179586476925286766559;
pub const CV_32F: i32 = 5;
pub const CV_32FC1: i32 = CV_MAKETYPE(CV_32F,1);
pub const CV_32FC2: i32 = CV_MAKETYPE(CV_32F,2);
pub const CV_32FC3: i32 = CV_MAKETYPE(CV_32F,3);
pub const CV_32FC4: i32 = CV_MAKETYPE(CV_32F,4);
pub const CV_32S: i32 = 4;
pub const CV_32SC1: i32 = CV_MAKETYPE(CV_32S,1);
pub const CV_32SC2: i32 = CV_MAKETYPE(CV_32S,2);
pub const CV_32SC3: i32 = CV_MAKETYPE(CV_32S,3);
pub const CV_32SC4: i32 = CV_MAKETYPE(CV_32S,4);
pub const CV_64F: i32 = 6;
pub const CV_64FC1: i32 = CV_MAKETYPE(CV_64F,1);
pub const CV_64FC2: i32 = CV_MAKETYPE(CV_64F,2);
pub const CV_64FC3: i32 = CV_MAKETYPE(CV_64F,3);
pub const CV_64FC4: i32 = CV_MAKETYPE(CV_64F,4);
pub const CV_8S: i32 = 1;
pub const CV_8SC1: i32 = CV_MAKETYPE(CV_8S,1);
pub const CV_8SC2: i32 = CV_MAKETYPE(CV_8S,2);
pub const CV_8SC3: i32 = CV_MAKETYPE(CV_8S,3);
pub const CV_8SC4: i32 = CV_MAKETYPE(CV_8S,4);
pub const CV_8U: i32 = 0;
pub const CV_8UC1: i32 = CV_MAKETYPE(CV_8U,1);
pub const CV_8UC2: i32 = CV_MAKETYPE(CV_8U,2);
pub const CV_8UC3: i32 = CV_MAKETYPE(CV_8U,3);
pub const CV_8UC4: i32 = CV_MAKETYPE(CV_8U,4);
pub const CV_AVX: i32 = 0;
pub const CV_AVX2: i32 = 0;
pub const CV_AVX512_CLX: i32 = 0;
pub const CV_AVX512_CNL: i32 = 0;
pub const CV_AVX512_COMMON: i32 = 0;
pub const CV_AVX512_ICL: i32 = 0;
pub const CV_AVX512_KNL: i32 = 0;
pub const CV_AVX512_KNM: i32 = 0;
pub const CV_AVX512_SKX: i32 = 0;
pub const CV_AVX_5124FMAPS: i32 = 0;
pub const CV_AVX_5124VNNIW: i32 = 0;
pub const CV_AVX_512BITALG: i32 = 0;
pub const CV_AVX_512BW: i32 = 0;
pub const CV_AVX_512CD: i32 = 0;
pub const CV_AVX_512DQ: i32 = 0;
pub const CV_AVX_512ER: i32 = 0;
pub const CV_AVX_512F: i32 = 0;
pub const CV_AVX_512IFMA: i32 = 0;
pub const CV_AVX_512IFMA512: i32 = CV_AVX_512IFMA;
pub const CV_AVX_512PF: i32 = 0;
pub const CV_AVX_512VBMI: i32 = 0;
pub const CV_AVX_512VBMI2: i32 = 0;
pub const CV_AVX_512VL: i32 = 0;
pub const CV_AVX_512VNNI: i32 = 0;
pub const CV_AVX_512VPOPCNTDQ: i32 = 0;
pub const CV_CN_MAX: i32 = 512;
pub const CV_CN_SHIFT: i32 = 3;
pub const CV_CPU_AVX: i32 = 10;
pub const CV_CPU_AVX2: i32 = 11;
pub const CV_CPU_AVX512_CLX: i32 = 261;
pub const CV_CPU_AVX512_CNL: i32 = 260;
pub const CV_CPU_AVX512_COMMON: i32 = 257;
pub const CV_CPU_AVX512_ICL: i32 = 262;
pub const CV_CPU_AVX512_KNL: i32 = 258;
pub const CV_CPU_AVX512_KNM: i32 = 259;
pub const CV_CPU_AVX512_SKX: i32 = 256;
pub const CV_CPU_AVX_5124FMAPS: i32 = 27;
pub const CV_CPU_AVX_5124VNNIW: i32 = 26;
pub const CV_CPU_AVX_512BITALG: i32 = 24;
pub const CV_CPU_AVX_512BW: i32 = 14;
pub const CV_CPU_AVX_512CD: i32 = 15;
pub const CV_CPU_AVX_512DQ: i32 = 16;
pub const CV_CPU_AVX_512ER: i32 = 17;
pub const CV_CPU_AVX_512F: i32 = 13;
pub const CV_CPU_AVX_512IFMA: i32 = 18;
pub const CV_CPU_AVX_512IFMA512: i32 = 18;
pub const CV_CPU_AVX_512PF: i32 = 19;
pub const CV_CPU_AVX_512VBMI: i32 = 20;
pub const CV_CPU_AVX_512VBMI2: i32 = 22;
pub const CV_CPU_AVX_512VL: i32 = 21;
pub const CV_CPU_AVX_512VNNI: i32 = 23;
pub const CV_CPU_AVX_512VPOPCNTDQ: i32 = 25;
pub const CV_CPU_FMA3: i32 = 12;
pub const CV_CPU_FP16: i32 = 9;
pub const CV_CPU_MMX: i32 = 1;
pub const CV_CPU_MSA: i32 = 150;
pub const CV_CPU_NEON: i32 = 100;
pub const CV_CPU_NONE: i32 = 0;
pub const CV_CPU_POPCNT: i32 = 8;
pub const CV_CPU_SSE: i32 = 2;
pub const CV_CPU_SSE2: i32 = 3;
pub const CV_CPU_SSE3: i32 = 4;
pub const CV_CPU_SSE4_1: i32 = 6;
pub const CV_CPU_SSE4_2: i32 = 7;
pub const CV_CPU_SSSE3: i32 = 5;
pub const CV_CPU_VSX: i32 = 200;
pub const CV_CPU_VSX3: i32 = 201;
pub const CV_CXX11: i32 = 1;
pub const CV_CXX_MOVE_SEMANTICS: i32 = 1;
pub const CV_CXX_STD_ARRAY: i32 = 1;
pub const CV_DEPTH_MAX: i32 = (1<<CV_CN_SHIFT);
pub const CV_ENABLE_UNROLLED: i32 = 1;
pub const CV_FMA3: i32 = 0;
pub const CV_FP16: i32 = 0;
pub const CV_FP16_TYPE: i32 = 0;
pub const CV_HAL_BORDER_CONSTANT: i32 = 0;
pub const CV_HAL_BORDER_ISOLATED: i32 = 16;
pub const CV_HAL_BORDER_REFLECT: i32 = 2;
pub const CV_HAL_BORDER_REFLECT_101: i32 = 4;
pub const CV_HAL_BORDER_REPLICATE: i32 = 1;
pub const CV_HAL_BORDER_TRANSPARENT: i32 = 5;
pub const CV_HAL_BORDER_WRAP: i32 = 3;
pub const CV_HAL_CMP_EQ: i32 = 0;
pub const CV_HAL_CMP_GE: i32 = 2;
pub const CV_HAL_CMP_GT: i32 = 1;
pub const CV_HAL_CMP_LE: i32 = 4;
pub const CV_HAL_CMP_LT: i32 = 3;
pub const CV_HAL_CMP_NE: i32 = 5;
pub const CV_HAL_DFT_COMPLEX_OUTPUT: i32 = 16;
pub const CV_HAL_DFT_INVERSE: i32 = 1;
pub const CV_HAL_DFT_IS_CONTINUOUS: i32 = 512;
pub const CV_HAL_DFT_IS_INPLACE: i32 = 1024;
pub const CV_HAL_DFT_REAL_OUTPUT: i32 = 32;
pub const CV_HAL_DFT_ROWS: i32 = 4;
pub const CV_HAL_DFT_SCALE: i32 = 2;
pub const CV_HAL_DFT_STAGE_COLS: i32 = 128;
pub const CV_HAL_DFT_TWO_STAGE: i32 = 64;
pub const CV_HAL_ERROR_NOT_IMPLEMENTED: i32 = 1;
pub const CV_HAL_ERROR_OK: i32 = 0;
pub const CV_HAL_ERROR_UNKNOWN: i32 = -1;
pub const CV_HAL_GEMM_1_T: i32 = 1;
pub const CV_HAL_GEMM_2_T: i32 = 2;
pub const CV_HAL_GEMM_3_T: i32 = 4;
pub const CV_HAL_SVD_FULL_UV: i32 = 8;
pub const CV_HAL_SVD_MODIFY_A: i32 = 4;
pub const CV_HAL_SVD_NO_UV: i32 = 1;
pub const CV_HAL_SVD_SHORT_UV: i32 = 2;
pub const CV_HARDWARE_MAX_FEATURE: i32 = 512;
pub const CV_IMPL_IPP: i32 = 0x04;
pub const CV_IMPL_MT: i32 = 0x10;
pub const CV_IMPL_OCL: i32 = 0x02;
pub const CV_IMPL_PLAIN: i32 = 0x01;
pub const CV_LOG2: f64 = 0.69314718055994530941723212145818;
pub const CV_LOG_LEVEL_DEBUG: i32 = 5;
pub const CV_LOG_LEVEL_ERROR: i32 = 2;
pub const CV_LOG_LEVEL_FATAL: i32 = 1;
pub const CV_LOG_LEVEL_INFO: i32 = 4;
pub const CV_LOG_LEVEL_SILENT: i32 = 0;
pub const CV_LOG_LEVEL_VERBOSE: i32 = 6;
pub const CV_LOG_LEVEL_WARN: i32 = 3;
pub const CV_LOG_STRIP_LEVEL: i32 = CV_LOG_LEVEL_VERBOSE;
pub const CV_MAJOR_VERSION: i32 = CV_VERSION_MAJOR;
pub const CV_MAT_CN_MASK: i32 = ((CV_CN_MAX-1)<<CV_CN_SHIFT);
pub const CV_MAT_CONT_FLAG: i32 = (1<<CV_MAT_CONT_FLAG_SHIFT);
pub const CV_MAT_CONT_FLAG_SHIFT: i32 = 14;
pub const CV_MAT_DEPTH_MASK: i32 = (CV_DEPTH_MAX-1);
pub const CV_MAT_TYPE_MASK: i32 = (CV_DEPTH_MAX*CV_CN_MAX-1);
pub const CV_MINOR_VERSION: i32 = CV_VERSION_MINOR;
pub const CV_MMX: i32 = 1;
pub const CV_MSA: i32 = 0;
pub const CV_NEON: i32 = 0;
pub const CV_PI: f64 = 3.1415926535897932384626433832795;
pub const CV_POPCNT: i32 = 0;
pub const CV_SSE: i32 = 1;
pub const CV_SSE2: i32 = 1;
pub const CV_SSE3: i32 = 0;
pub const CV_SSE4_1: i32 = 0;
pub const CV_SSE4_2: i32 = 0;
pub const CV_SSSE3: i32 = 0;
pub const CV_STRONG_ALIGNMENT: i32 = 0;
pub const CV_SUBMAT_FLAG: i32 = (1<<CV_SUBMAT_FLAG_SHIFT);
pub const CV_SUBMAT_FLAG_SHIFT: i32 = 15;
pub const CV_SUBMINOR_VERSION: i32 = CV_VERSION_REVISION;
pub const CV_VERSION: &'static str = "4.3.0";
pub const CV_VERSION_MAJOR: i32 = 4;
pub const CV_VERSION_MINOR: i32 = 3;
pub const CV_VERSION_REVISION: i32 = 0;
pub const CV_VERSION_STATUS: &'static str = "";
pub const CV_VSX: i32 = 0;
pub const CV_VSX3: i32 = 0;
pub const CV_WASM_SIMD: i32 = 0;
pub const CV__EXCEPTION_PTR: i32 = 1;
/// performs an inverse 1D or 2D transform instead of the default forward transform.
pub const DCT_INVERSE: i32 = 1;
/// performs a forward or inverse transform of every individual row of the input
/// matrix. This flag enables you to transform multiple vectors simultaneously and can be used to
/// decrease the overhead (which is sometimes several times larger than the processing itself) to
/// perform 3D and higher-dimensional transforms and so forth.
pub const DCT_ROWS: i32 = 4;
/// Cholesky ![inline formula](https://latex.codecogs.com/png.latex?LL%5ET) factorization; the matrix src1 must be symmetrical and positively
/// defined
pub const DECOMP_CHOLESKY: i32 = 3;
/// eigenvalue decomposition; the matrix src1 must be symmetrical
pub const DECOMP_EIG: i32 = 2;
/// Gaussian elimination with the optimal pivot element chosen.
pub const DECOMP_LU: i32 = 0;
/// while all the previous flags are mutually exclusive, this flag can be used together with
/// any of the previous; it means that the normal equations
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsrc1%7D%5ET%5Ccdot%5Ctexttt%7Bsrc1%7D%5Ccdot%5Ctexttt%7Bdst%7D%3D%5Ctexttt%7Bsrc1%7D%5ET%5Ctexttt%7Bsrc2%7D) are
/// solved instead of the original system
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsrc1%7D%5Ccdot%5Ctexttt%7Bdst%7D%3D%5Ctexttt%7Bsrc2%7D)
pub const DECOMP_NORMAL: i32 = 16;
/// QR factorization; the system can be over-defined and/or the matrix src1 can be singular
pub const DECOMP_QR: i32 = 4;
/// singular value decomposition (SVD) method; the system can be over-defined and/or the matrix
/// src1 can be singular
pub const DECOMP_SVD: i32 = 1;
/// specifies that input is complex input. If this flag is set, the input must have 2 channels.
/// On the other hand, for backwards compatibility reason, if input has 2 channels, input is
/// already considered complex.
pub const DFT_COMPLEX_INPUT: i32 = 64;
/// performs a forward transformation of 1D or 2D real array; the result,
/// though being a complex array, has complex-conjugate symmetry (*CCS*, see the function
/// description below for details), and such an array can be packed into a real array of the same
/// size as input, which is the fastest option and which is what the function does by default;
/// however, you may wish to get a full complex array (for simpler spectrum analysis, and so on) -
/// pass the flag to enable the function to produce a full-size complex output array.
pub const DFT_COMPLEX_OUTPUT: i32 = 16;
/// performs an inverse 1D or 2D transform instead of the default forward
/// transform.
pub const DFT_INVERSE: i32 = 1;
/// performs an inverse transformation of a 1D or 2D complex array; the
/// result is normally a complex array of the same size, however, if the input array has
/// conjugate-complex symmetry (for example, it is a result of forward transformation with
/// DFT_COMPLEX_OUTPUT flag), the output is a real array; while the function itself does not
/// check whether the input is symmetrical or not, you can pass the flag and then the function
/// will assume the symmetry and produce the real output array (note that when the input is packed
/// into a real array and inverse transformation is executed, the function treats the input as a
/// packed complex-conjugate symmetrical array, and the output will also be a real array).
pub const DFT_REAL_OUTPUT: i32 = 32;
/// performs a forward or inverse transform of every individual row of the input
/// matrix; this flag enables you to transform multiple vectors simultaneously and can be used to
/// decrease the overhead (which is sometimes several times larger than the processing itself) to
/// perform 3D and higher-dimensional transformations and so forth.
pub const DFT_ROWS: i32 = 4;
/// scales the result: divide it by the number of array elements. Normally, it is
/// combined with DFT_INVERSE.
pub const DFT_SCALE: i32 = 2;
pub const DYNAMIC_PARALLELISM: i32 = 35;
pub const Detail_CV__LAST_TEST_OP: i32 = 7;
pub const Detail_TEST_CUSTOM: i32 = 0;
pub const Detail_TEST_EQ: i32 = 1;
pub const Detail_TEST_GE: i32 = 5;
pub const Detail_TEST_GT: i32 = 6;
pub const Detail_TEST_LE: i32 = 3;
pub const Detail_TEST_LT: i32 = 4;
pub const Detail_TEST_NE: i32 = 2;
pub const Device_EXEC_KERNEL: i32 = 1;
pub const Device_EXEC_NATIVE_KERNEL: i32 = 2;
pub const Device_FP_CORRECTLY_ROUNDED_DIVIDE_SQRT: i32 = 128;
pub const Device_FP_DENORM: i32 = 1;
pub const Device_FP_FMA: i32 = 32;
pub const Device_FP_INF_NAN: i32 = 2;
pub const Device_FP_ROUND_TO_INF: i32 = 16;
pub const Device_FP_ROUND_TO_NEAREST: i32 = 4;
pub const Device_FP_ROUND_TO_ZERO: i32 = 8;
pub const Device_FP_SOFT_FLOAT: i32 = 64;
pub const Device_LOCAL_IS_GLOBAL: i32 = 2;
pub const Device_LOCAL_IS_LOCAL: i32 = 1;
pub const Device_NO_CACHE: i32 = 0;
pub const Device_NO_LOCAL_MEM: i32 = 0;
pub const Device_READ_ONLY_CACHE: i32 = 1;
pub const Device_READ_WRITE_CACHE: i32 = 2;
pub const Device_TYPE_ACCELERATOR: i32 = 8;
pub const Device_TYPE_ALL: i32 = -1;
pub const Device_TYPE_CPU: i32 = 2;
pub const Device_TYPE_DEFAULT: i32 = 1;
pub const Device_TYPE_DGPU: i32 = 65540;
pub const Device_TYPE_GPU: i32 = 4;
pub const Device_TYPE_IGPU: i32 = 131076;
pub const Device_UNKNOWN_VENDOR: i32 = 0;
pub const Device_VENDOR_AMD: i32 = 1;
pub const Device_VENDOR_INTEL: i32 = 2;
pub const Device_VENDOR_NVIDIA: i32 = 3;
pub const ENUM_LOG_LEVEL_FORCE_INT: i32 = 2147483647;
pub const FEATURE_SET_COMPUTE_10: i32 = 10;
pub const FEATURE_SET_COMPUTE_11: i32 = 11;
pub const FEATURE_SET_COMPUTE_12: i32 = 12;
pub const FEATURE_SET_COMPUTE_13: i32 = 13;
pub const FEATURE_SET_COMPUTE_20: i32 = 20;
pub const FEATURE_SET_COMPUTE_21: i32 = 21;
pub const FEATURE_SET_COMPUTE_30: i32 = 30;
pub const FEATURE_SET_COMPUTE_32: i32 = 32;
pub const FEATURE_SET_COMPUTE_35: i32 = 35;
pub const FEATURE_SET_COMPUTE_50: i32 = 50;
pub const FLAGS_EXPAND_SAME_NAMES: i32 = 2;
pub const FLAGS_MAPPING: i32 = 1;
pub const FLAGS_NONE: i32 = 0;
/// empty structure (sequence or mapping)
pub const FileNode_EMPTY: i32 = 16;
/// synonym or REAL
pub const FileNode_FLOAT: i32 = 2;
/// compact representation of a sequence or mapping. Used only by YAML writer
pub const FileNode_FLOW: i32 = 8;
/// an integer
pub const FileNode_INT: i32 = 1;
/// mapping
pub const FileNode_MAP: i32 = 5;
/// the node has a name (i.e. it is element of a mapping).
pub const FileNode_NAMED: i32 = 32;
/// empty node
pub const FileNode_NONE: i32 = 0;
/// floating-point number
pub const FileNode_REAL: i32 = 2;
/// sequence
pub const FileNode_SEQ: i32 = 4;
/// text string in UTF-8 encoding
pub const FileNode_STR: i32 = 3;
/// synonym for STR
pub const FileNode_STRING: i32 = 3;
pub const FileNode_TYPE_MASK: i32 = 7;
/// if set, means that all the collection elements are numbers of the same type (real's or int's).
/// UNIFORM is used only when reading FileStorage; FLOW is used only when writing. So they share the same bit
pub const FileNode_UNIFORM: i32 = 8;
/// transposes src1
pub const GEMM_1_T: i32 = 1;
/// transposes src2
pub const GEMM_2_T: i32 = 2;
/// transposes src3
pub const GEMM_3_T: i32 = 4;
pub const GLOBAL_ATOMICS: i32 = 11;
/// GPU API call error
pub const GpuApiCallError: i32 = -217;
/// no CUDA support
pub const GpuNotSupported: i32 = -216;
/// image header is NULL
pub const HeaderIsNull: i32 = -9;
pub const IMPL_IPP: i32 = 1;
pub const IMPL_OPENCL: i32 = 2;
pub const IMPL_PLAIN: i32 = 0;
/// Use kmeans++ center initialization by Arthur and Vassilvitskii [Arthur2007].
pub const KMEANS_PP_CENTERS: i32 = 2;
/// Select random initial centers in each attempt.
pub const KMEANS_RANDOM_CENTERS: i32 = 0;
/// During the first (and possibly the only) attempt, use the
/// user-supplied labels instead of computing them from the initial centers. For the second and
/// further attempts, use the random or semi-random centers. Use one of KMEANS_\*_CENTERS flag
/// to specify the exact method.
pub const KMEANS_USE_INITIAL_LABELS: i32 = 1;
pub const KernelArg_CONSTANT: i32 = 8;
pub const KernelArg_LOCAL: i32 = 1;
pub const KernelArg_NO_SIZE: i32 = 256;
pub const KernelArg_PTR_ONLY: i32 = 16;
pub const KernelArg_READ_ONLY: i32 = 2;
pub const KernelArg_READ_WRITE: i32 = 6;
pub const KernelArg_WRITE_ONLY: i32 = 4;
/// Debug message. Disabled in the "Release" build.
pub const LOG_LEVEL_DEBUG: i32 = 5;
/// Error message
pub const LOG_LEVEL_ERROR: i32 = 2;
/// Fatal (critical) error (unrecoverable internal error)
pub const LOG_LEVEL_FATAL: i32 = 1;
/// Info message
pub const LOG_LEVEL_INFO: i32 = 4;
/// for using in setLogVevel() call
pub const LOG_LEVEL_SILENT: i32 = 0;
/// Verbose (trace) messages. Requires verbosity level. Disabled in the "Release" build.
pub const LOG_LEVEL_VERBOSE: i32 = 6;
/// Warning message
pub const LOG_LEVEL_WARNING: i32 = 3;
pub const MaskIsTiled: i32 = -26;
pub const Mat_AUTO_STEP: usize = 0;
pub const Mat_CONTINUOUS_FLAG: i32 = 16384;
pub const Mat_DEPTH_MASK: i32 = 7;
pub const Mat_MAGIC_MASK: i32 = -65536;
pub const Mat_MAGIC_VAL: i32 = 1124007936;
pub const Mat_SUBMATRIX_FLAG: i32 = 32768;
pub const Mat_TYPE_MASK: i32 = 4095;
pub const NATIVE_DOUBLE: i32 = 13;
/// In the case of one input array, calculates the Hamming distance of the array from zero,
/// In the case of two input arrays, calculates the Hamming distance between the arrays.
pub const NORM_HAMMING: i32 = 6;
/// Similar to NORM_HAMMING, but in the calculation, each two bits of the input sequence will
/// be added and treated as a single bit to be used in the same calculation as NORM_HAMMING.
pub const NORM_HAMMING2: i32 = 7;
/// ![block formula](https://latex.codecogs.com/png.latex?%0Anorm%20%3D%20%20%5Cforkthree%0A%7B%5C%7C%5Ctexttt%7Bsrc1%7D%5C%7C%5F%7BL%5F%7B%5Cinfty%7D%7D%20%3D%20%20%5Cmax%20%5FI%20%7C%20%5Ctexttt%7Bsrc1%7D%20%28I%29%7C%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FINF%7D%5C%29%20%7D%0A%7B%5C%7C%5Ctexttt%7Bsrc1%7D%2D%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F%7B%5Cinfty%7D%7D%20%3D%20%20%5Cmax%20%5FI%20%7C%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%2D%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29%7C%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FINF%7D%5C%29%20%7D%0A%7B%5Cfrac%7B%5C%7C%5Ctexttt%7Bsrc1%7D%2D%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F%7B%5Cinfty%7D%7D%20%20%20%20%7D%7B%5C%7C%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F%7B%5Cinfty%7D%7D%20%7D%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FRELATIVE%20%7C%20NORM%5FINF%7D%5C%29%20%7D%0A)
pub const NORM_INF: i32 = 1;
/// ![block formula](https://latex.codecogs.com/png.latex?%0Anorm%20%3D%20%20%5Cforkthree%0A%7B%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%5C%7C%20%5F%7BL%5F1%7D%20%3D%20%20%5Csum%20%5FI%20%7C%20%5Ctexttt%7Bsrc1%7D%20%28I%29%7C%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL1%7D%5C%29%7D%0A%7B%20%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%2D%20%5Ctexttt%7Bsrc2%7D%20%5C%7C%20%5F%7BL%5F1%7D%20%3D%20%20%5Csum%20%5FI%20%7C%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%2D%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29%7C%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL1%7D%5C%29%20%7D%0A%7B%20%5Cfrac%7B%5C%7C%5Ctexttt%7Bsrc1%7D%2D%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F1%7D%20%7D%7B%5C%7C%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F1%7D%7D%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FRELATIVE%20%7C%20NORM%5FL1%7D%5C%29%20%7D%0A)
pub const NORM_L1: i32 = 2;
/// ![block formula](https://latex.codecogs.com/png.latex?%0Anorm%20%3D%20%20%5Cforkthree%0A%7B%20%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%5C%7C%20%5F%7BL%5F2%7D%20%3D%20%20%5Csqrt%7B%5Csum%5FI%20%5Ctexttt%7Bsrc1%7D%28I%29%5E2%7D%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL2%7D%5C%29%20%7D%0A%7B%20%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%2D%20%5Ctexttt%7Bsrc2%7D%20%5C%7C%20%5F%7BL%5F2%7D%20%3D%20%20%5Csqrt%7B%5Csum%5FI%20%28%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%5Ctexttt%7Bsrc2%7D%28I%29%29%5E2%7D%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL2%7D%5C%29%20%7D%0A%7B%20%5Cfrac%7B%5C%7C%5Ctexttt%7Bsrc1%7D%2D%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F2%7D%20%7D%7B%5C%7C%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F2%7D%7D%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FRELATIVE%20%7C%20NORM%5FL2%7D%5C%29%20%7D%0A)
pub const NORM_L2: i32 = 4;
/// ![block formula](https://latex.codecogs.com/png.latex?%0Anorm%20%3D%20%20%5Cforkthree%0A%7B%20%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%5C%7C%20%5F%7BL%5F2%7D%20%5E%7B2%7D%20%3D%20%5Csum%5FI%20%5Ctexttt%7Bsrc1%7D%28I%29%5E2%7D%20%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL2SQR%7D%5C%29%7D%0A%7B%20%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%2D%20%5Ctexttt%7Bsrc2%7D%20%5C%7C%20%5F%7BL%5F2%7D%20%5E%7B2%7D%20%3D%20%20%5Csum%5FI%20%28%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%5Ctexttt%7Bsrc2%7D%28I%29%29%5E2%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL2SQR%7D%5C%29%20%7D%0A%7B%20%5Cleft%28%5Cfrac%7B%5C%7C%5Ctexttt%7Bsrc1%7D%2D%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F2%7D%20%7D%7B%5C%7C%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F2%7D%7D%5Cright%29%5E2%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FRELATIVE%20%7C%20NORM%5FL2SQR%7D%5C%29%20%7D%0A)
pub const NORM_L2SQR: i32 = 5;
/// flag
pub const NORM_MINMAX: i32 = 32;
/// flag
pub const NORM_RELATIVE: i32 = 8;
/// bit-mask which can be used to separate norm type from norm flags
pub const NORM_TYPE_MASK: i32 = 7;
pub const OCL_VECTOR_DEFAULT: i32 = 0;
pub const OCL_VECTOR_MAX: i32 = 1;
pub const OCL_VECTOR_OWN: i32 = 0;
pub const OPENCV_ABI_COMPATIBILITY: i32 = 400;
pub const OPENCV_USE_FASTMATH_BUILTINS: i32 = 1;
/// OpenCL API call error
pub const OpenCLApiCallError: i32 = -220;
pub const OpenCLDoubleNotSupported: i32 = -221;
/// OpenCL initialization error
pub const OpenCLInitError: i32 = -222;
pub const OpenCLNoAMDBlasFft: i32 = -223;
/// OpenGL API call error
pub const OpenGlApiCallError: i32 = -219;
/// no OpenGL support
pub const OpenGlNotSupported: i32 = -218;
pub const Param_ALGORITHM: i32 = 6;
pub const Param_BOOLEAN: i32 = 1;
pub const Param_FLOAT: i32 = 7;
pub const Param_INT: i32 = 0;
pub const Param_MAT: i32 = 4;
pub const Param_MAT_VECTOR: i32 = 5;
pub const Param_REAL: i32 = 2;
pub const Param_SCALAR: i32 = 12;
pub const Param_STRING: i32 = 3;
pub const Param_UCHAR: i32 = 11;
pub const Param_UINT64: i32 = 9;
pub const Param_UNSIGNED_INT: i32 = 8;
/// the output is the mean vector of all rows/columns of the matrix.
pub const REDUCE_AVG: i32 = 1;
/// the output is the maximum (column/row-wise) of all rows/columns of the matrix.
pub const REDUCE_MAX: i32 = 2;
/// the output is the minimum (column/row-wise) of all rows/columns of the matrix.
pub const REDUCE_MIN: i32 = 3;
/// the output is the sum of all rows/columns of the matrix.
pub const REDUCE_SUM: i32 = 0;
pub const RNG_NORMAL: i32 = 1;
pub const RNG_UNIFORM: i32 = 0;
/// Rotate 180 degrees clockwise
pub const ROTATE_180: i32 = 1;
/// Rotate 90 degrees clockwise
pub const ROTATE_90_CLOCKWISE: i32 = 0;
/// Rotate 270 degrees clockwise
pub const ROTATE_90_COUNTERCLOCKWISE: i32 = 2;
pub const SHARED_ATOMICS: i32 = 12;
/// there are multiple maxima for target function - the arbitrary one is returned
pub const SOLVELP_MULTI: i32 = 1;
/// there is only one maximum for target function
pub const SOLVELP_SINGLE: i32 = 0;
/// problem is unbounded (target function can achieve arbitrary high values)
pub const SOLVELP_UNBOUNDED: i32 = -2;
/// problem is unfeasible (there are no points that satisfy all the constraints imposed)
pub const SOLVELP_UNFEASIBLE: i32 = -1;
/// each matrix row is sorted in the ascending
/// order.
pub const SORT_ASCENDING: i32 = 0;
/// each matrix row is sorted in the
/// descending order; this flag and the previous one are also
/// mutually exclusive.
pub const SORT_DESCENDING: i32 = 16;
/// each matrix column is sorted
/// independently; this flag and the previous one are
/// mutually exclusive.
pub const SORT_EVERY_COLUMN: i32 = 1;
/// each matrix row is sorted independently
pub const SORT_EVERY_ROW: i32 = 0;
pub const SparseMat_HASH_BIT: i32 = -2147483648;
pub const SparseMat_HASH_SCALE: i32 = 1540483477;
pub const SparseMat_MAGIC_VAL: i32 = 1123876864;
pub const SparseMat_MAX_DIM: i32 = 32;
/// assertion failed
pub const StsAssert: i32 = -215;
/// tracing
pub const StsAutoTrace: i32 = -8;
/// pseudo error for back trace
pub const StsBackTrace: i32 = -1;
/// function arg/param is bad
pub const StsBadArg: i32 = -5;
/// flag is wrong or not supported
pub const StsBadFlag: i32 = -206;
/// unsupported function
pub const StsBadFunc: i32 = -6;
/// bad format of mask (neither 8uC1 nor 8sC1)
pub const StsBadMask: i32 = -208;
/// an allocated block has been corrupted
pub const StsBadMemBlock: i32 = -214;
/// bad CvPoint
pub const StsBadPoint: i32 = -207;
/// the input/output structure size is incorrect
pub const StsBadSize: i32 = -201;
/// division by zero
pub const StsDivByZero: i32 = -202;
/// unknown /unspecified error
pub const StsError: i32 = -2;
/// incorrect filter offset value
pub const StsFilterOffsetErr: i32 = -31;
/// incorrect filter structure content
pub const StsFilterStructContentErr: i32 = -29;
/// in-place operation is not supported
pub const StsInplaceNotSupported: i32 = -203;
/// internal error (bad state)
pub const StsInternal: i32 = -3;
/// incorrect transform kernel content
pub const StsKernelStructContentErr: i32 = -30;
/// iteration didn't converge
pub const StsNoConv: i32 = -7;
/// insufficient memory
pub const StsNoMem: i32 = -4;
/// the requested function/feature is not implemented
pub const StsNotImplemented: i32 = -213;
/// null pointer
pub const StsNullPtr: i32 = -27;
/// request can't be completed
pub const StsObjectNotFound: i32 = -204;
/// everything is ok
pub const StsOk: i32 = 0;
/// some of parameters are out of range
pub const StsOutOfRange: i32 = -211;
/// invalid syntax/structure of the parsed file
pub const StsParseError: i32 = -212;
/// formats of input/output arrays differ
pub const StsUnmatchedFormats: i32 = -205;
/// sizes of input/output structures do not match
pub const StsUnmatchedSizes: i32 = -209;
/// the data format/type is not supported by the function
pub const StsUnsupportedFormat: i32 = -210;
/// incorrect vector length
pub const StsVecLengthErr: i32 = -28;
pub const TYPE_FUN: i32 = 3;
pub const TYPE_GENERAL: i32 = 0;
pub const TYPE_MARKER: i32 = 1;
pub const TYPE_WRAPPER: i32 = 2;
pub const UMat_AUTO_STEP: i32 = 0;
pub const UMat_CONTINUOUS_FLAG: i32 = 16384;
pub const UMat_DEPTH_MASK: i32 = 7;
pub const UMat_MAGIC_MASK: i32 = -65536;
pub const UMat_MAGIC_VAL: i32 = 1124007936;
pub const UMat_SUBMATRIX_FLAG: i32 = 32768;
pub const UMat_TYPE_MASK: i32 = 4095;
pub const USAGE_ALLOCATE_DEVICE_MEMORY: i32 = 2;
pub const USAGE_ALLOCATE_HOST_MEMORY: i32 = 1;
pub const USAGE_ALLOCATE_SHARED_MEMORY: i32 = 4;
pub const USAGE_DEFAULT: i32 = 0;
pub const WARP_SHUFFLE_FUNCTIONS: i32 = 30;
pub const __UMAT_USAGE_FLAGS_32BIT: i32 = 2147483647;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AccessFlag {
	ACCESS_READ = 16777216,
	ACCESS_WRITE = 33554432,
	ACCESS_RW = 50331648,
	// ACCESS_MASK = 50331648 as isize, // duplicate discriminant
	ACCESS_FAST = 67108864,
}

opencv_type_enum! { core::AccessFlag }

/// Various border types, image boundaries are denoted with `|`
/// ## See also
/// borderInterpolate, copyMakeBorder
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BorderTypes {
	/// `iiiiii|abcdefgh|iiiiiii`  with some specified `i`
	BORDER_CONSTANT = 0,
	/// `aaaaaa|abcdefgh|hhhhhhh`
	BORDER_REPLICATE = 1,
	/// `fedcba|abcdefgh|hgfedcb`
	BORDER_REFLECT = 2,
	/// `cdefgh|abcdefgh|abcdefg`
	BORDER_WRAP = 3,
	/// `gfedcb|abcdefgh|gfedcba`
	BORDER_REFLECT_101 = 4,
	/// `uvwxyz|abcdefgh|ijklmno`
	BORDER_TRANSPARENT = 5,
	// same as BORDER_REFLECT_101
	// BORDER_REFLECT101 = 4 as isize, // duplicate discriminant
	// same as BORDER_REFLECT_101
	// BORDER_DEFAULT = 4 as isize, // duplicate discriminant
	/// do not look outside of ROI
	BORDER_ISOLATED = 16,
}

opencv_type_enum! { core::BorderTypes }

/// comparison types
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CmpTypes {
	/// src1 is equal to src2.
	CMP_EQ = 0,
	/// src1 is greater than src2.
	CMP_GT = 1,
	/// src1 is greater than or equal to src2.
	CMP_GE = 2,
	/// src1 is less than src2.
	CMP_LT = 3,
	/// src1 is less than or equal to src2.
	CMP_LE = 4,
	/// src1 is unequal to src2.
	CMP_NE = 5,
}

opencv_type_enum! { core::CmpTypes }

/// error codes
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Code {
	/// everything is ok
	StsOk = 0,
	/// pseudo error for back trace
	StsBackTrace = -1,
	/// unknown /unspecified error
	StsError = -2,
	/// internal error (bad state)
	StsInternal = -3,
	/// insufficient memory
	StsNoMem = -4,
	/// function arg/param is bad
	StsBadArg = -5,
	/// unsupported function
	StsBadFunc = -6,
	/// iteration didn't converge
	StsNoConv = -7,
	/// tracing
	StsAutoTrace = -8,
	/// image header is NULL
	HeaderIsNull = -9,
	/// image size is invalid
	BadImageSize = -10,
	/// offset is invalid
	BadOffset = -11,
	BadDataPtr = -12,
	/// image step is wrong, this may happen for a non-continuous matrix.
	BadStep = -13,
	BadModelOrChSeq = -14,
	/// bad number of channels, for example, some functions accept only single channel matrices.
	BadNumChannels = -15,
	BadNumChannel1U = -16,
	/// input image depth is not supported by the function
	BadDepth = -17,
	BadAlphaChannel = -18,
	/// number of dimensions is out of range
	BadOrder = -19,
	/// incorrect input origin
	BadOrigin = -20,
	/// incorrect input align
	BadAlign = -21,
	BadCallBack = -22,
	BadTileSize = -23,
	/// input COI is not supported
	BadCOI = -24,
	/// incorrect input roi
	BadROISize = -25,
	MaskIsTiled = -26,
	/// null pointer
	StsNullPtr = -27,
	/// incorrect vector length
	StsVecLengthErr = -28,
	/// incorrect filter structure content
	StsFilterStructContentErr = -29,
	/// incorrect transform kernel content
	StsKernelStructContentErr = -30,
	/// incorrect filter offset value
	StsFilterOffsetErr = -31,
	/// the input/output structure size is incorrect
	StsBadSize = -201,
	/// division by zero
	StsDivByZero = -202,
	/// in-place operation is not supported
	StsInplaceNotSupported = -203,
	/// request can't be completed
	StsObjectNotFound = -204,
	/// formats of input/output arrays differ
	StsUnmatchedFormats = -205,
	/// flag is wrong or not supported
	StsBadFlag = -206,
	/// bad CvPoint
	StsBadPoint = -207,
	/// bad format of mask (neither 8uC1 nor 8sC1)
	StsBadMask = -208,
	/// sizes of input/output structures do not match
	StsUnmatchedSizes = -209,
	/// the data format/type is not supported by the function
	StsUnsupportedFormat = -210,
	/// some of parameters are out of range
	StsOutOfRange = -211,
	/// invalid syntax/structure of the parsed file
	StsParseError = -212,
	/// the requested function/feature is not implemented
	StsNotImplemented = -213,
	/// an allocated block has been corrupted
	StsBadMemBlock = -214,
	/// assertion failed
	StsAssert = -215,
	/// no CUDA support
	GpuNotSupported = -216,
	/// GPU API call error
	GpuApiCallError = -217,
	/// no OpenGL support
	OpenGlNotSupported = -218,
	/// OpenGL API call error
	OpenGlApiCallError = -219,
	/// OpenCL API call error
	OpenCLApiCallError = -220,
	OpenCLDoubleNotSupported = -221,
	/// OpenCL initialization error
	OpenCLInitError = -222,
	OpenCLNoAMDBlasFft = -223,
}

opencv_type_enum! { core::Code }

/// Covariation flags
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CovarFlags {
	/// The output covariance matrix is calculated as:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bscale%7D%20%20%20%5Ccdot%20%20%5B%20%20%5Ctexttt%7Bvects%7D%20%20%5B0%5D%2D%20%20%5Ctexttt%7Bmean%7D%20%20%2C%20%5Ctexttt%7Bvects%7D%20%20%5B1%5D%2D%20%20%5Ctexttt%7Bmean%7D%20%20%2C%2E%2E%2E%5D%5ET%20%20%5Ccdot%20%20%5B%20%5Ctexttt%7Bvects%7D%20%20%5B0%5D%2D%20%5Ctexttt%7Bmean%7D%20%20%2C%20%5Ctexttt%7Bvects%7D%20%20%5B1%5D%2D%20%5Ctexttt%7Bmean%7D%20%20%2C%2E%2E%2E%5D%2C)
	/// The covariance matrix will be nsamples x nsamples. Such an unusual covariance matrix is used
	/// for fast PCA of a set of very large vectors (see, for example, the EigenFaces technique for
	/// face recognition). Eigenvalues of this "scrambled" matrix match the eigenvalues of the true
	/// covariance matrix. The "true" eigenvectors can be easily calculated from the eigenvectors of
	/// the "scrambled" covariance matrix.
	COVAR_SCRAMBLED = 0,
	/// The output covariance matrix is calculated as:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bscale%7D%20%20%20%5Ccdot%20%20%5B%20%20%5Ctexttt%7Bvects%7D%20%20%5B0%5D%2D%20%20%5Ctexttt%7Bmean%7D%20%20%2C%20%5Ctexttt%7Bvects%7D%20%20%5B1%5D%2D%20%20%5Ctexttt%7Bmean%7D%20%20%2C%2E%2E%2E%5D%20%20%5Ccdot%20%20%5B%20%5Ctexttt%7Bvects%7D%20%20%5B0%5D%2D%20%5Ctexttt%7Bmean%7D%20%20%2C%20%5Ctexttt%7Bvects%7D%20%20%5B1%5D%2D%20%5Ctexttt%7Bmean%7D%20%20%2C%2E%2E%2E%5D%5ET%2C)
	/// covar will be a square matrix of the same size as the total number of elements in each input
	/// vector. One and only one of #COVAR_SCRAMBLED and #COVAR_NORMAL must be specified.
	COVAR_NORMAL = 1,
	/// If the flag is specified, the function does not calculate mean from
	/// the input vectors but, instead, uses the passed mean vector. This is useful if mean has been
	/// pre-calculated or known in advance, or if the covariance matrix is calculated by parts. In
	/// this case, mean is not a mean vector of the input sub-set of vectors but rather the mean
	/// vector of the whole set.
	COVAR_USE_AVG = 2,
	/// If the flag is specified, the covariance matrix is scaled. In the
	/// "normal" mode, scale is 1./nsamples . In the "scrambled" mode, scale is the reciprocal of the
	/// total number of elements in each input vector. By default (if the flag is not specified), the
	/// covariance matrix is not scaled ( scale=1 ).
	COVAR_SCALE = 4,
	/// If the flag is
	/// specified, all the input vectors are stored as rows of the samples matrix. mean should be a
	/// single-row vector in this case.
	COVAR_ROWS = 8,
	/// If the flag is
	/// specified, all the input vectors are stored as columns of the samples matrix. mean should be a
	/// single-column vector in this case.
	COVAR_COLS = 16,
}

opencv_type_enum! { core::CovarFlags }

/// Available CPU features.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CpuFeatures {
	CPU_MMX = 1,
	CPU_SSE = 2,
	CPU_SSE2 = 3,
	CPU_SSE3 = 4,
	CPU_SSSE3 = 5,
	CPU_SSE4_1 = 6,
	CPU_SSE4_2 = 7,
	CPU_POPCNT = 8,
	CPU_FP16 = 9,
	CPU_AVX = 10,
	CPU_AVX2 = 11,
	CPU_FMA3 = 12,
	CPU_AVX_512F = 13,
	CPU_AVX_512BW = 14,
	CPU_AVX_512CD = 15,
	CPU_AVX_512DQ = 16,
	CPU_AVX_512ER = 17,
	CPU_AVX_512IFMA512 = 18,
	// CPU_AVX_512IFMA = 18 as isize, // duplicate discriminant
	CPU_AVX_512PF = 19,
	CPU_AVX_512VBMI = 20,
	CPU_AVX_512VL = 21,
	CPU_AVX_512VBMI2 = 22,
	CPU_AVX_512VNNI = 23,
	CPU_AVX_512BITALG = 24,
	CPU_AVX_512VPOPCNTDQ = 25,
	CPU_AVX_5124VNNIW = 26,
	CPU_AVX_5124FMAPS = 27,
	CPU_NEON = 100,
	CPU_MSA = 150,
	CPU_VSX = 200,
	CPU_VSX3 = 201,
	/// Skylake-X with AVX-512F/CD/BW/DQ/VL
	CPU_AVX512_SKX = 256,
	/// Common instructions AVX-512F/CD for all CPUs that support AVX-512
	CPU_AVX512_COMMON = 257,
	/// Knights Landing with AVX-512F/CD/ER/PF
	CPU_AVX512_KNL = 258,
	/// Knights Mill with AVX-512F/CD/ER/PF/4FMAPS/4VNNIW/VPOPCNTDQ
	CPU_AVX512_KNM = 259,
	/// Cannon Lake with AVX-512F/CD/BW/DQ/VL/IFMA/VBMI
	CPU_AVX512_CNL = 260,
	/// Cascade Lake with AVX-512F/CD/BW/DQ/VL/VNNI
	CPU_AVX512_CLX = 261,
	/// Ice Lake with AVX-512F/CD/BW/DQ/VL/IFMA/VBMI/VNNI/VBMI2/BITALG/VPOPCNTDQ
	CPU_AVX512_ICL = 262,
	CPU_MAX_FEATURE = 512,
}

opencv_type_enum! { core::CpuFeatures }

/// matrix decomposition types
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DecompTypes {
	/// Gaussian elimination with the optimal pivot element chosen.
	DECOMP_LU = 0,
	/// singular value decomposition (SVD) method; the system can be over-defined and/or the matrix
	/// src1 can be singular
	DECOMP_SVD = 1,
	/// eigenvalue decomposition; the matrix src1 must be symmetrical
	DECOMP_EIG = 2,
	/// Cholesky ![inline formula](https://latex.codecogs.com/png.latex?LL%5ET) factorization; the matrix src1 must be symmetrical and positively
	/// defined
	DECOMP_CHOLESKY = 3,
	/// QR factorization; the system can be over-defined and/or the matrix src1 can be singular
	DECOMP_QR = 4,
	/// while all the previous flags are mutually exclusive, this flag can be used together with
	/// any of the previous; it means that the normal equations
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsrc1%7D%5ET%5Ccdot%5Ctexttt%7Bsrc1%7D%5Ccdot%5Ctexttt%7Bdst%7D%3D%5Ctexttt%7Bsrc1%7D%5ET%5Ctexttt%7Bsrc2%7D) are
	/// solved instead of the original system
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsrc1%7D%5Ccdot%5Ctexttt%7Bdst%7D%3D%5Ctexttt%7Bsrc2%7D)
	DECOMP_NORMAL = 16,
}

opencv_type_enum! { core::DecompTypes }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Detail_TestOp {
	TEST_CUSTOM = 0,
	TEST_EQ = 1,
	TEST_NE = 2,
	TEST_LE = 3,
	TEST_LT = 4,
	TEST_GE = 5,
	TEST_GT = 6,
	CV__LAST_TEST_OP = 7,
}

opencv_type_enum! { core::Detail_TestOp }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DeviceInfo_ComputeMode {
	/// < default compute mode (Multiple threads can use cudaSetDevice with this device)
	ComputeModeDefault = 0,
	/// < compute-exclusive-thread mode (Only one thread in one process will be able to use cudaSetDevice with this device)
	ComputeModeExclusive = 1,
	/// < compute-prohibited mode (No threads can use cudaSetDevice with this device)
	ComputeModeProhibited = 2,
	/// < compute-exclusive-process mode (Many threads in one process will be able to use cudaSetDevice with this device)
	ComputeModeExclusiveProcess = 3,
}

opencv_type_enum! { core::DeviceInfo_ComputeMode }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DftFlags {
	/// performs an inverse 1D or 2D transform instead of the default forward
	/// transform.
	DFT_INVERSE = 1,
	/// scales the result: divide it by the number of array elements. Normally, it is
	/// combined with DFT_INVERSE.
	DFT_SCALE = 2,
	/// performs a forward or inverse transform of every individual row of the input
	/// matrix; this flag enables you to transform multiple vectors simultaneously and can be used to
	/// decrease the overhead (which is sometimes several times larger than the processing itself) to
	/// perform 3D and higher-dimensional transformations and so forth.
	DFT_ROWS = 4,
	/// performs a forward transformation of 1D or 2D real array; the result,
	/// though being a complex array, has complex-conjugate symmetry (*CCS*, see the function
	/// description below for details), and such an array can be packed into a real array of the same
	/// size as input, which is the fastest option and which is what the function does by default;
	/// however, you may wish to get a full complex array (for simpler spectrum analysis, and so on) -
	/// pass the flag to enable the function to produce a full-size complex output array.
	DFT_COMPLEX_OUTPUT = 16,
	/// performs an inverse transformation of a 1D or 2D complex array; the
	/// result is normally a complex array of the same size, however, if the input array has
	/// conjugate-complex symmetry (for example, it is a result of forward transformation with
	/// DFT_COMPLEX_OUTPUT flag), the output is a real array; while the function itself does not
	/// check whether the input is symmetrical or not, you can pass the flag and then the function
	/// will assume the symmetry and produce the real output array (note that when the input is packed
	/// into a real array and inverse transformation is executed, the function treats the input as a
	/// packed complex-conjugate symmetrical array, and the output will also be a real array).
	DFT_REAL_OUTPUT = 32,
	/// specifies that input is complex input. If this flag is set, the input must have 2 channels.
	/// On the other hand, for backwards compatibility reason, if input has 2 channels, input is
	/// already considered complex.
	DFT_COMPLEX_INPUT = 64,
	// performs an inverse 1D or 2D transform instead of the default forward transform.
	// DCT_INVERSE = 1 as isize, // duplicate discriminant
	// performs a forward or inverse transform of every individual row of the input
	// matrix. This flag enables you to transform multiple vectors simultaneously and can be used to
	// decrease the overhead (which is sometimes several times larger than the processing itself) to
	// perform 3D and higher-dimensional transforms and so forth.
	// DCT_ROWS = 4 as isize, // duplicate discriminant
}

opencv_type_enum! { core::DftFlags }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Event_CreateFlags {
	/// < Default event flag
	DEFAULT = 0,
	/// < Event uses blocking synchronization
	BLOCKING_SYNC = 1,
	/// < Event will not record timing data
	DISABLE_TIMING = 2,
	/// < Event is suitable for interprocess use. DisableTiming must be set
	INTERPROCESS = 4,
}

opencv_type_enum! { core::Event_CreateFlags }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FLAGS {
	FLAGS_NONE = 0,
	FLAGS_MAPPING = 1,
	FLAGS_EXPAND_SAME_NAMES = 2,
}

opencv_type_enum! { core::FLAGS }

/// Enumeration providing CUDA computing features.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FeatureSet {
	FEATURE_SET_COMPUTE_10 = 10,
	FEATURE_SET_COMPUTE_11 = 11,
	FEATURE_SET_COMPUTE_12 = 12,
	FEATURE_SET_COMPUTE_13 = 13,
	FEATURE_SET_COMPUTE_20 = 20,
	FEATURE_SET_COMPUTE_21 = 21,
	FEATURE_SET_COMPUTE_30 = 30,
	FEATURE_SET_COMPUTE_32 = 32,
	FEATURE_SET_COMPUTE_35 = 35,
	FEATURE_SET_COMPUTE_50 = 50,
	// GLOBAL_ATOMICS = 11 as isize, // duplicate discriminant
	// SHARED_ATOMICS = 12 as isize, // duplicate discriminant
	// NATIVE_DOUBLE = 13 as isize, // duplicate discriminant
	// WARP_SHUFFLE_FUNCTIONS = 30 as isize, // duplicate discriminant
	// DYNAMIC_PARALLELISM = 35 as isize, // duplicate discriminant
}

opencv_type_enum! { core::FeatureSet }

/// file storage mode
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FileStorage_Mode {
	/// value, open the file for reading
	READ = 0,
	/// value, open the file for writing
	WRITE = 1,
	/// value, open the file for appending
	APPEND = 2,
	/// flag, read data from source or write data to the internal buffer (which is
	/// returned by FileStorage::release)
	MEMORY = 4,
	/// mask for format flags
	FORMAT_MASK = 56,
	// flag, auto format
	// FORMAT_AUTO = 0 as isize, // duplicate discriminant
	/// flag, XML format
	FORMAT_XML = 8,
	/// flag, YAML format
	FORMAT_YAML = 16,
	/// flag, JSON format
	FORMAT_JSON = 24,
	/// flag, write rawdata in Base64 by default. (consider using WRITE_BASE64)
	BASE64 = 64,
	/// flag, enable both WRITE and BASE64
	WRITE_BASE64 = 65,
}

opencv_type_enum! { core::FileStorage_Mode }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FileStorage_State {
	UNDEFINED = 0,
	VALUE_EXPECTED = 1,
	NAME_EXPECTED = 2,
	INSIDE_MAP = 4,
}

opencv_type_enum! { core::FileStorage_State }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Formatter_FormatType {
	FMT_DEFAULT = 0,
	FMT_MATLAB = 1,
	FMT_CSV = 2,
	FMT_PYTHON = 3,
	FMT_NUMPY = 4,
	FMT_C = 5,
}

opencv_type_enum! { core::Formatter_FormatType }

/// generalized matrix multiplication flags
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GemmFlags {
	/// transposes src1
	GEMM_1_T = 1,
	/// transposes src2
	GEMM_2_T = 2,
	/// transposes src3
	GEMM_3_T = 4,
}

opencv_type_enum! { core::GemmFlags }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HostMem_AllocType {
	PAGE_LOCKED = 1,
	SHARED = 2,
	WRITE_COMBINED = 4,
}

opencv_type_enum! { core::HostMem_AllocType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IMPL {
	IMPL_PLAIN = 0,
	IMPL_IPP = 1,
	IMPL_OPENCL = 2,
}

opencv_type_enum! { core::IMPL }

/// k-Means flags
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KmeansFlags {
	/// Select random initial centers in each attempt.
	KMEANS_RANDOM_CENTERS = 0,
	/// Use kmeans++ center initialization by Arthur and Vassilvitskii [Arthur2007].
	KMEANS_PP_CENTERS = 2,
	/// During the first (and possibly the only) attempt, use the
	/// user-supplied labels instead of computing them from the initial centers. For the second and
	/// further attempts, use the random or semi-random centers. Use one of KMEANS_\*_CENTERS flag
	/// to specify the exact method.
	KMEANS_USE_INITIAL_LABELS = 1,
}

opencv_type_enum! { core::KmeansFlags }

/// Supported logging levels and their semantic
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogLevel {
	/// for using in setLogVevel() call
	LOG_LEVEL_SILENT = 0,
	/// Fatal (critical) error (unrecoverable internal error)
	LOG_LEVEL_FATAL = 1,
	/// Error message
	LOG_LEVEL_ERROR = 2,
	/// Warning message
	LOG_LEVEL_WARNING = 3,
	/// Info message
	LOG_LEVEL_INFO = 4,
	/// Debug message. Disabled in the "Release" build.
	LOG_LEVEL_DEBUG = 5,
	/// Verbose (trace) messages. Requires verbosity level. Disabled in the "Release" build.
	LOG_LEVEL_VERBOSE = 6,
	ENUM_LOG_LEVEL_FORCE_INT = 2147483647,
}

opencv_type_enum! { core::LogLevel }

/// norm types
/// 
/// src1 and src2 denote input arrays.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NormTypes {
	/// ![block formula](https://latex.codecogs.com/png.latex?%0Anorm%20%3D%20%20%5Cforkthree%0A%7B%5C%7C%5Ctexttt%7Bsrc1%7D%5C%7C%5F%7BL%5F%7B%5Cinfty%7D%7D%20%3D%20%20%5Cmax%20%5FI%20%7C%20%5Ctexttt%7Bsrc1%7D%20%28I%29%7C%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FINF%7D%5C%29%20%7D%0A%7B%5C%7C%5Ctexttt%7Bsrc1%7D%2D%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F%7B%5Cinfty%7D%7D%20%3D%20%20%5Cmax%20%5FI%20%7C%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%2D%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29%7C%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FINF%7D%5C%29%20%7D%0A%7B%5Cfrac%7B%5C%7C%5Ctexttt%7Bsrc1%7D%2D%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F%7B%5Cinfty%7D%7D%20%20%20%20%7D%7B%5C%7C%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F%7B%5Cinfty%7D%7D%20%7D%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FRELATIVE%20%7C%20NORM%5FINF%7D%5C%29%20%7D%0A)
	NORM_INF = 1,
	/// ![block formula](https://latex.codecogs.com/png.latex?%0Anorm%20%3D%20%20%5Cforkthree%0A%7B%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%5C%7C%20%5F%7BL%5F1%7D%20%3D%20%20%5Csum%20%5FI%20%7C%20%5Ctexttt%7Bsrc1%7D%20%28I%29%7C%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL1%7D%5C%29%7D%0A%7B%20%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%2D%20%5Ctexttt%7Bsrc2%7D%20%5C%7C%20%5F%7BL%5F1%7D%20%3D%20%20%5Csum%20%5FI%20%7C%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%2D%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29%7C%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL1%7D%5C%29%20%7D%0A%7B%20%5Cfrac%7B%5C%7C%5Ctexttt%7Bsrc1%7D%2D%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F1%7D%20%7D%7B%5C%7C%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F1%7D%7D%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FRELATIVE%20%7C%20NORM%5FL1%7D%5C%29%20%7D%0A)
	NORM_L1 = 2,
	/// ![block formula](https://latex.codecogs.com/png.latex?%0Anorm%20%3D%20%20%5Cforkthree%0A%7B%20%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%5C%7C%20%5F%7BL%5F2%7D%20%3D%20%20%5Csqrt%7B%5Csum%5FI%20%5Ctexttt%7Bsrc1%7D%28I%29%5E2%7D%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL2%7D%5C%29%20%7D%0A%7B%20%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%2D%20%5Ctexttt%7Bsrc2%7D%20%5C%7C%20%5F%7BL%5F2%7D%20%3D%20%20%5Csqrt%7B%5Csum%5FI%20%28%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%5Ctexttt%7Bsrc2%7D%28I%29%29%5E2%7D%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL2%7D%5C%29%20%7D%0A%7B%20%5Cfrac%7B%5C%7C%5Ctexttt%7Bsrc1%7D%2D%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F2%7D%20%7D%7B%5C%7C%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F2%7D%7D%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FRELATIVE%20%7C%20NORM%5FL2%7D%5C%29%20%7D%0A)
	NORM_L2 = 4,
	/// ![block formula](https://latex.codecogs.com/png.latex?%0Anorm%20%3D%20%20%5Cforkthree%0A%7B%20%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%5C%7C%20%5F%7BL%5F2%7D%20%5E%7B2%7D%20%3D%20%5Csum%5FI%20%5Ctexttt%7Bsrc1%7D%28I%29%5E2%7D%20%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL2SQR%7D%5C%29%7D%0A%7B%20%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%2D%20%5Ctexttt%7Bsrc2%7D%20%5C%7C%20%5F%7BL%5F2%7D%20%5E%7B2%7D%20%3D%20%20%5Csum%5FI%20%28%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%5Ctexttt%7Bsrc2%7D%28I%29%29%5E2%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FL2SQR%7D%5C%29%20%7D%0A%7B%20%5Cleft%28%5Cfrac%7B%5C%7C%5Ctexttt%7Bsrc1%7D%2D%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F2%7D%20%7D%7B%5C%7C%5Ctexttt%7Bsrc2%7D%5C%7C%5F%7BL%5F2%7D%7D%5Cright%29%5E2%20%7D%7Bif%20%20%5C%28%5Ctexttt%7BnormType%7D%20%3D%20%5Ctexttt%7BNORM%5FRELATIVE%20%7C%20NORM%5FL2SQR%7D%5C%29%20%7D%0A)
	NORM_L2SQR = 5,
	/// In the case of one input array, calculates the Hamming distance of the array from zero,
	/// In the case of two input arrays, calculates the Hamming distance between the arrays.
	NORM_HAMMING = 6,
	/// Similar to NORM_HAMMING, but in the calculation, each two bits of the input sequence will
	/// be added and treated as a single bit to be used in the same calculation as NORM_HAMMING.
	NORM_HAMMING2 = 7,
	// bit-mask which can be used to separate norm type from norm flags
	// NORM_TYPE_MASK = 7 as isize, // duplicate discriminant
	/// flag
	NORM_RELATIVE = 8,
	/// flag
	NORM_MINMAX = 32,
}

opencv_type_enum! { core::NormTypes }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OclVectorStrategy {
	OCL_VECTOR_OWN = 0,
	OCL_VECTOR_MAX = 1,
	// OCL_VECTOR_DEFAULT = 0 as isize, // duplicate discriminant
}

opencv_type_enum! { core::OclVectorStrategy }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PCA_Flags {
	/// indicates that the input samples are stored as matrix rows
	DATA_AS_ROW = 0,
	/// indicates that the input samples are stored as matrix columns
	DATA_AS_COL = 1,
	USE_AVG = 2,
}

opencv_type_enum! { core::PCA_Flags }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Param {
	INT = 0,
	BOOLEAN = 1,
	REAL = 2,
	STRING = 3,
	MAT = 4,
	MAT_VECTOR = 5,
	ALGORITHM = 6,
	FLOAT = 7,
	UNSIGNED_INT = 8,
	UINT64 = 9,
	UCHAR = 11,
	SCALAR = 12,
}

opencv_type_enum! { core::Param }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ReduceTypes {
	/// the output is the sum of all rows/columns of the matrix.
	REDUCE_SUM = 0,
	/// the output is the mean vector of all rows/columns of the matrix.
	REDUCE_AVG = 1,
	/// the output is the maximum (column/row-wise) of all rows/columns of the matrix.
	REDUCE_MAX = 2,
	/// the output is the minimum (column/row-wise) of all rows/columns of the matrix.
	REDUCE_MIN = 3,
}

opencv_type_enum! { core::ReduceTypes }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RotateFlags {
	/// Rotate 90 degrees clockwise
	ROTATE_90_CLOCKWISE = 0,
	/// Rotate 180 degrees clockwise
	ROTATE_180 = 1,
	/// Rotate 270 degrees clockwise
	ROTATE_90_COUNTERCLOCKWISE = 2,
}

opencv_type_enum! { core::RotateFlags }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SVD_Flags {
	/// allow the algorithm to modify the decomposed matrix; it can save space and speed up
	/// processing. currently ignored.
	MODIFY_A = 1,
	/// indicates that only a vector of singular values `w` is to be processed, while u and vt
	/// will be set to empty matrices
	NO_UV = 2,
	/// when the matrix is not square, by default the algorithm produces u and vt matrices of
	/// sufficiently large size for the further A reconstruction; if, however, FULL_UV flag is
	/// specified, u and vt will be full-size square orthogonal matrices.
	FULL_UV = 4,
}

opencv_type_enum! { core::SVD_Flags }

/// return codes for cv::solveLP() function
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SolveLPResult {
	/// problem is unbounded (target function can achieve arbitrary high values)
	SOLVELP_UNBOUNDED = -2,
	/// problem is unfeasible (there are no points that satisfy all the constraints imposed)
	SOLVELP_UNFEASIBLE = -1,
	/// there is only one maximum for target function
	SOLVELP_SINGLE = 0,
	/// there are multiple maxima for target function - the arbitrary one is returned
	SOLVELP_MULTI = 1,
}

opencv_type_enum! { core::SolveLPResult }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SortFlags {
	/// each matrix row is sorted independently
	SORT_EVERY_ROW = 0,
	/// each matrix column is sorted
	/// independently; this flag and the previous one are
	/// mutually exclusive.
	SORT_EVERY_COLUMN = 1,
	// each matrix row is sorted in the ascending
	// order.
	// SORT_ASCENDING = 0 as isize, // duplicate discriminant
	/// each matrix row is sorted in the
	/// descending order; this flag and the previous one are also
	/// mutually exclusive.
	SORT_DESCENDING = 16,
}

opencv_type_enum! { core::SortFlags }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TYPE {
	TYPE_GENERAL = 0,
	TYPE_MARKER = 1,
	TYPE_WRAPPER = 2,
	TYPE_FUN = 3,
}

opencv_type_enum! { core::TYPE }

/// Criteria type, can be one of: COUNT, EPS or COUNT + EPS
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TermCriteria_Type {
	/// the maximum number of iterations or elements to compute
	COUNT = 1,
	// ditto
	// MAX_ITER = 1 as isize, // duplicate discriminant
	/// the desired accuracy or change in parameters at which the iterative algorithm stops
	EPS = 2,
}

opencv_type_enum! { core::TermCriteria_Type }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UMatData_MemoryFlag {
	COPY_ON_MAP = 1,
	HOST_COPY_OBSOLETE = 2,
	DEVICE_COPY_OBSOLETE = 4,
	TEMP_UMAT = 8,
	TEMP_COPIED_UMAT = 24,
	USER_ALLOCATED = 32,
	DEVICE_MEM_MAPPED = 64,
	ASYNC_CLEANUP = 128,
}

opencv_type_enum! { core::UMatData_MemoryFlag }

/// Usage flags for allocator
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UMatUsageFlags {
	USAGE_DEFAULT = 0,
	USAGE_ALLOCATE_HOST_MEMORY = 1,
	USAGE_ALLOCATE_DEVICE_MEMORY = 2,
	USAGE_ALLOCATE_SHARED_MEMORY = 4,
	__UMAT_USAGE_FLAGS_32BIT = 2147483647,
}

opencv_type_enum! { core::UMatUsageFlags }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum _InputArray_KindFlag {
	KIND_SHIFT = 16,
	FIXED_TYPE = -2147483648,
	FIXED_SIZE = 1073741824,
	KIND_MASK = 2031616,
	NONE = 0,
	MAT = 65536,
	MATX = 131072,
	STD_VECTOR = 196608,
	STD_VECTOR_VECTOR = 262144,
	STD_VECTOR_MAT = 327680,
	EXPR = 393216,
	OPENGL_BUFFER = 458752,
	CUDA_HOST_MEM = 524288,
	CUDA_GPU_MAT = 589824,
	UMAT = 655360,
	STD_VECTOR_UMAT = 720896,
	STD_BOOL_VECTOR = 786432,
	STD_VECTOR_CUDA_GPU_MAT = 851968,
	STD_ARRAY = 917504,
	STD_ARRAY_MAT = 983040,
}

opencv_type_enum! { core::_InputArray_KindFlag }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum _OutputArray_DepthMask {
	DEPTH_MASK_8U = 1,
	DEPTH_MASK_8S = 2,
	DEPTH_MASK_16U = 4,
	DEPTH_MASK_16S = 8,
	DEPTH_MASK_32S = 16,
	DEPTH_MASK_32F = 32,
	DEPTH_MASK_64F = 64,
	DEPTH_MASK_16F = 128,
	DEPTH_MASK_ALL = 127,
	DEPTH_MASK_ALL_BUT_8S = 125,
	DEPTH_MASK_ALL_16F = 255,
	DEPTH_MASK_FLT = 96,
}

opencv_type_enum! { core::_OutputArray_DepthMask }

pub type va_display = *mut c_void;
pub type va_surface_id = u32;
pub type Affine3d = core::Affine3<f64>;
pub type Affine3f = core::Affine3<f32>;
pub type Hamming_result_type = i32;
pub type Hamming_value_type = u8;
pub type HammingLUT = core::Hamming;
pub type InputArray<'a> = &'a core::_InputArray;
pub type InputArrayOfArrays<'a> = core::InputArray<'a>;
pub type InputOutputArray<'a> = &'a core::_InputOutputArray;
pub type InputOutputArrayOfArrays<'a> = core::InputOutputArray<'a>;
pub type Mat1b = core::Mat_<u8>;
pub type Mat1d = core::Mat_<f64>;
pub type Mat1f = core::Mat_<f32>;
pub type Mat1i = core::Mat_<i32>;
pub type Mat1s = core::Mat_<i16>;
pub type Mat1w = core::Mat_<u16>;
pub type Mat2b = core::Mat_<core::Vec2b>;
pub type Mat2d = core::Mat_<core::Vec2d>;
pub type Mat2f = core::Mat_<core::Vec2f>;
pub type Mat2i = core::Mat_<core::Vec2i>;
pub type Mat2s = core::Mat_<core::Vec2s>;
pub type Mat2w = core::Mat_<core::Vec2w>;
pub type Mat3b = core::Mat_<core::Vec3b>;
pub type Mat3d = core::Mat_<core::Vec3d>;
pub type Mat3f = core::Mat_<core::Vec3f>;
pub type Mat3i = core::Mat_<core::Vec3i>;
pub type Mat3s = core::Mat_<core::Vec3s>;
pub type Mat3w = core::Mat_<core::Vec3w>;
pub type Mat4b = core::Mat_<core::Vec4b>;
pub type Mat4d = core::Mat_<core::Vec4d>;
pub type Mat4f = core::Mat_<core::Vec4f>;
pub type Mat4i = core::Mat_<core::Vec4i>;
pub type Mat4s = core::Mat_<core::Vec4s>;
pub type Mat4w = core::Mat_<core::Vec4w>;
pub type MatConstIterator_difference_type = ptrdiff_t;
pub type MatConstIterator_pointer<'a, 'b> = &'a &'b u8;
pub type MatConstIterator_reference<'a> = &'a mut u8;
pub type MatConstIterator_value_type<'a> = &'a mut u8;
pub type MatND = core::Mat;
pub type Matx12d = core::Matx12<f64>;
pub type Matx12f = core::Matx12<f32>;
pub type Matx13d = core::Matx13<f64>;
pub type Matx13f = core::Matx13<f32>;
pub type Matx14d = core::Matx14<f64>;
pub type Matx14f = core::Matx14<f32>;
pub type Matx16d = core::Matx16<f64>;
pub type Matx16f = core::Matx16<f32>;
pub type Matx21d = core::Matx21<f64>;
pub type Matx21f = core::Matx21<f32>;
pub type Matx22d = core::Matx22<f64>;
pub type Matx22f = core::Matx22<f32>;
pub type Matx23d = core::Matx23<f64>;
pub type Matx23f = core::Matx23<f32>;
pub type Matx31d = core::Matx31<f64>;
pub type Matx31f = core::Matx31<f32>;
pub type Matx32d = core::Matx32<f64>;
pub type Matx32f = core::Matx32<f32>;
pub type Matx33d = core::Matx33<f64>;
pub type Matx33f = core::Matx33<f32>;
pub type Matx34d = core::Matx34<f64>;
pub type Matx34f = core::Matx34<f32>;
pub type Matx41d = core::Matx41<f64>;
pub type Matx41f = core::Matx41<f32>;
pub type Matx43d = core::Matx43<f64>;
pub type Matx43f = core::Matx43<f32>;
pub type Matx44d = core::Matx44<f64>;
pub type Matx44f = core::Matx44<f32>;
pub type Matx61d = core::Matx61<f64>;
pub type Matx61f = core::Matx61<f32>;
pub type Matx66d = core::Matx66<f64>;
pub type Matx66f = core::Matx66<f32>;
pub type OutputArray<'a> = &'a core::_OutputArray;
pub type OutputArrayOfArrays<'a> = core::OutputArray<'a>;
pub type Point = core::Point2i;
pub type Point2d = core::Point_<f64>;
pub type Point2f = core::Point_<f32>;
pub type Point2i = core::Point_<i32>;
pub type Point2l = core::Point_<i64>;
pub type Point3d = core::Point3_<f64>;
pub type Point3f = core::Point3_<f32>;
pub type Point3i = core::Point3_<i32>;
pub type Rect = core::Rect2i;
pub type Rect2d = core::Rect_<f64>;
pub type Rect2f = core::Rect_<f32>;
pub type Rect2i = core::Rect_<i32>;
pub type Scalar = core::Scalar_<f64>;
pub type Size = core::Size2i;
pub type Size2d = core::Size_<f64>;
pub type Size2f = core::Size_<f32>;
pub type Size2i = core::Size_<i32>;
pub type Size2l = core::Size_<i64>;
pub type SparseMat_const_iterator = core::SparseMatConstIterator;
pub type SparseMat_iterator = core::SparseMatIterator;
/// @name Shorter aliases for the most popular specializations of Vec<T,n>
pub type Vec2b = core::Vec2<u8>;
pub type Vec2d = core::Vec2<f64>;
pub type Vec2f = core::Vec2<f32>;
pub type Vec2i = core::Vec2<i32>;
pub type Vec2s = core::Vec2<i16>;
pub type Vec2w = core::Vec2<u16>;
pub type Vec3b = core::Vec3<u8>;
pub type Vec3d = core::Vec3<f64>;
pub type Vec3f = core::Vec3<f32>;
pub type Vec3i = core::Vec3<i32>;
pub type Vec3s = core::Vec3<i16>;
pub type Vec3w = core::Vec3<u16>;
pub type Vec4b = core::Vec4<u8>;
pub type Vec4d = core::Vec4<f64>;
pub type Vec4f = core::Vec4<f32>;
pub type Vec4i = core::Vec4<i32>;
pub type Vec4s = core::Vec4<i16>;
pub type Vec4w = core::Vec4<u16>;
pub type Vec6d = core::Vec6<f64>;
pub type Vec6f = core::Vec6<f32>;
pub type Vec6i = core::Vec6<i32>;
pub type Vec8i = core::Vec8<i32>;
pub type Stream_StreamCallback = Option<Box<dyn FnMut(i32) -> () + Send + Sync + 'static>>;
pub type ProgramSource_hash_t = u64;
/// proxy for hal::Cholesky
pub fn cholesky(a: &mut f64, astep: size_t, m: i32, b: &mut f64, bstep: size_t, n: i32) -> Result<bool> {
	unsafe { sys::cv_Cholesky_doubleX_size_t_int_doubleX_size_t_int(a, astep, m, b, bstep, n) }.into_result()
}

/// proxy for hal::Cholesky
pub fn cholesky_f32(a: &mut f32, astep: size_t, m: i32, b: &mut f32, bstep: size_t, n: i32) -> Result<bool> {
	unsafe { sys::cv_Cholesky_floatX_size_t_int_floatX_size_t_int(a, astep, m, b, bstep, n) }.into_result()
}

/// Performs a look-up table transform of an array.
/// 
/// The function LUT fills the output array with values from the look-up table. Indices of the entries
/// are taken from the input array. That is, the function processes each element of src as follows:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%20%5Cleftarrow%20%5Ctexttt%7Blut%28src%28I%29%20%2B%20d%29%7D)
/// where
/// ![block formula](https://latex.codecogs.com/png.latex?d%20%3D%20%20%5Cfork%7B0%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%5C%29%20has%20depth%20%5C%28%5Ctexttt%7BCV%5F8U%7D%5C%29%7D%7B128%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%5C%29%20has%20depth%20%5C%28%5Ctexttt%7BCV%5F8S%7D%5C%29%7D)
/// ## Parameters
/// * src: input array of 8-bit elements.
/// * lut: look-up table of 256 elements; in case of multi-channel input array, the table should
/// either have a single channel (in this case the same table is used for all channels) or the same
/// number of channels as in the input array.
/// * dst: output array of the same size and number of channels as src, and the same depth as lut.
/// ## See also
/// convertScaleAbs, Mat::convertTo
pub fn lut(src: &dyn core::ToInputArray, lut: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(lut);
	output_array_arg!(dst);
	unsafe { sys::cv_LUT_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), lut.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// proxy for hal::LU
pub fn lu(a: &mut f64, astep: size_t, m: i32, b: &mut f64, bstep: size_t, n: i32) -> Result<i32> {
	unsafe { sys::cv_LU_doubleX_size_t_int_doubleX_size_t_int(a, astep, m, b, bstep, n) }.into_result()
}

/// proxy for hal::LU
pub fn lu_f32(a: &mut f32, astep: size_t, m: i32, b: &mut f32, bstep: size_t, n: i32) -> Result<i32> {
	unsafe { sys::cv_LU_floatX_size_t_int_floatX_size_t_int(a, astep, m, b, bstep, n) }.into_result()
}

/// Calculates the Mahalanobis distance between two vectors.
/// 
/// The function cv::Mahalanobis calculates and returns the weighted distance between two vectors:
/// ![block formula](https://latex.codecogs.com/png.latex?d%28%20%5Ctexttt%7Bvec1%7D%20%2C%20%5Ctexttt%7Bvec2%7D%20%29%3D%20%5Csqrt%7B%5Csum%5F%7Bi%2Cj%7D%7B%5Ctexttt%7Bicovar%28i%2Cj%29%7D%5Ccdot%28%5Ctexttt%7Bvec1%7D%28I%29%2D%5Ctexttt%7Bvec2%7D%28I%29%29%5Ccdot%28%5Ctexttt%7Bvec1%28j%29%7D%2D%5Ctexttt%7Bvec2%28j%29%7D%29%7D%20%7D)
/// The covariance matrix may be calculated using the #calcCovarMatrix function and then inverted using
/// the invert function (preferably using the #DECOMP_SVD method, as the most accurate).
/// ## Parameters
/// * v1: first 1D input vector.
/// * v2: second 1D input vector.
/// * icovar: inverse covariance matrix.
pub fn mahalanobis(v1: &dyn core::ToInputArray, v2: &dyn core::ToInputArray, icovar: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(v1);
	input_array_arg!(v2);
	input_array_arg!(icovar);
	unsafe { sys::cv_Mahalanobis_const__InputArrayR_const__InputArrayR_const__InputArrayR(v1.as_raw__InputArray(), v2.as_raw__InputArray(), icovar.as_raw__InputArray()) }.into_result()
}

/// wrap PCA::backProject
pub fn pca_back_project(data: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, eigenvectors: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(data);
	input_array_arg!(mean);
	input_array_arg!(eigenvectors);
	output_array_arg!(result);
	unsafe { sys::cv_PCABackProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(data.as_raw__InputArray(), mean.as_raw__InputArray(), eigenvectors.as_raw__InputArray(), result.as_raw__OutputArray()) }.into_result()
}

/// wrap PCA::operator() and add eigenvalues output parameter
pub fn pca_compute2_variance(data: &dyn core::ToInputArray, mean: &mut dyn core::ToInputOutputArray, eigenvectors: &mut dyn core::ToOutputArray, eigenvalues: &mut dyn core::ToOutputArray, retained_variance: f64) -> Result<()> {
	input_array_arg!(data);
	input_output_array_arg!(mean);
	output_array_arg!(eigenvectors);
	output_array_arg!(eigenvalues);
	unsafe { sys::cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_double(data.as_raw__InputArray(), mean.as_raw__InputOutputArray(), eigenvectors.as_raw__OutputArray(), eigenvalues.as_raw__OutputArray(), retained_variance) }.into_result()
}

/// wrap PCA::operator() and add eigenvalues output parameter
/// 
/// ## C++ default parameters
/// * max_components: 0
pub fn pca_compute2(data: &dyn core::ToInputArray, mean: &mut dyn core::ToInputOutputArray, eigenvectors: &mut dyn core::ToOutputArray, eigenvalues: &mut dyn core::ToOutputArray, max_components: i32) -> Result<()> {
	input_array_arg!(data);
	input_output_array_arg!(mean);
	output_array_arg!(eigenvectors);
	output_array_arg!(eigenvalues);
	unsafe { sys::cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int(data.as_raw__InputArray(), mean.as_raw__InputOutputArray(), eigenvectors.as_raw__OutputArray(), eigenvalues.as_raw__OutputArray(), max_components) }.into_result()
}

/// wrap PCA::operator()
pub fn pca_compute_variance(data: &dyn core::ToInputArray, mean: &mut dyn core::ToInputOutputArray, eigenvectors: &mut dyn core::ToOutputArray, retained_variance: f64) -> Result<()> {
	input_array_arg!(data);
	input_output_array_arg!(mean);
	output_array_arg!(eigenvectors);
	unsafe { sys::cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_double(data.as_raw__InputArray(), mean.as_raw__InputOutputArray(), eigenvectors.as_raw__OutputArray(), retained_variance) }.into_result()
}

/// wrap PCA::operator()
/// 
/// ## C++ default parameters
/// * max_components: 0
pub fn pca_compute(data: &dyn core::ToInputArray, mean: &mut dyn core::ToInputOutputArray, eigenvectors: &mut dyn core::ToOutputArray, max_components: i32) -> Result<()> {
	input_array_arg!(data);
	input_output_array_arg!(mean);
	output_array_arg!(eigenvectors);
	unsafe { sys::cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_int(data.as_raw__InputArray(), mean.as_raw__InputOutputArray(), eigenvectors.as_raw__OutputArray(), max_components) }.into_result()
}

/// wrap PCA::project
pub fn pca_project(data: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, eigenvectors: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(data);
	input_array_arg!(mean);
	input_array_arg!(eigenvectors);
	output_array_arg!(result);
	unsafe { sys::cv_PCAProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(data.as_raw__InputArray(), mean.as_raw__InputArray(), eigenvectors.as_raw__InputArray(), result.as_raw__OutputArray()) }.into_result()
}

/// Computes the Peak Signal-to-Noise Ratio (PSNR) image quality metric.
/// 
/// This function calculates the Peak Signal-to-Noise Ratio (PSNR) image quality metric in decibels (dB),
/// between two input arrays src1 and src2. The arrays must have the same type.
/// 
/// The PSNR is calculated as follows:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Ctexttt%7BPSNR%7D%20%3D%2010%20%5Ccdot%20%5Clog%5F%7B10%7D%7B%5Cleft%28%20%5Cfrac%7BR%5E2%7D%7BMSE%7D%20%5Cright%29%20%7D%0A)
/// 
/// where R is the maximum integer value of depth (e.g. 255 in the case of CV_8U data)
/// and MSE is the mean squared error between the two arrays.
/// 
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size as src1.
/// * R: the maximum pixel value (255 by default)
/// 
/// ## C++ default parameters
/// * r: 255.
pub fn psnr(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, r: f64) -> Result<f64> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	unsafe { sys::cv_PSNR_const__InputArrayR_const__InputArrayR_double(src1.as_raw__InputArray(), src2.as_raw__InputArray(), r) }.into_result()
}

/// wrap SVD::backSubst
pub fn sv_back_subst(w: &dyn core::ToInputArray, u: &dyn core::ToInputArray, vt: &dyn core::ToInputArray, rhs: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(w);
	input_array_arg!(u);
	input_array_arg!(vt);
	input_array_arg!(rhs);
	output_array_arg!(dst);
	unsafe { sys::cv_SVBackSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(w.as_raw__InputArray(), u.as_raw__InputArray(), vt.as_raw__InputArray(), rhs.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// wrap SVD::compute
/// 
/// ## C++ default parameters
/// * flags: 0
pub fn sv_decomp(src: &dyn core::ToInputArray, w: &mut dyn core::ToOutputArray, u: &mut dyn core::ToOutputArray, vt: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(w);
	output_array_arg!(u);
	output_array_arg!(vt);
	unsafe { sys::cv_SVDecomp_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), w.as_raw__OutputArray(), u.as_raw__OutputArray(), vt.as_raw__OutputArray(), flags) }.into_result()
}

/// Calculates an absolute value of each matrix element.
/// 
/// abs is a meta-function that is expanded to one of absdiff or convertScaleAbs forms:
/// - C = abs(A-B) is equivalent to `absdiff(A, B, C)`
/// - C = abs(A) is equivalent to `absdiff(A, Scalar::all(0), C)`
/// - C = `Mat_<Vec<uchar,n> >(abs(A*alpha + beta))` is equivalent to `convertScaleAbs(A, C, alpha,
/// beta)`
/// 
/// The output matrix has the same size and the same type as the input one except for the last case,
/// where C is depth=CV_8U .
/// ## Parameters
/// * m: matrix.
/// ## See also
/// @ref MatrixExpressions, absdiff, convertScaleAbs
/// 
/// ## Overloaded parameters
/// 
/// * e: matrix expression.
pub fn abs_matexpr(e: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_abs_const_MatExprR(e.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

/// Calculates an absolute value of each matrix element.
/// 
/// abs is a meta-function that is expanded to one of absdiff or convertScaleAbs forms:
/// - C = abs(A-B) is equivalent to `absdiff(A, B, C)`
/// - C = abs(A) is equivalent to `absdiff(A, Scalar::all(0), C)`
/// - C = `Mat_<Vec<uchar,n> >(abs(A*alpha + beta))` is equivalent to `convertScaleAbs(A, C, alpha,
/// beta)`
/// 
/// The output matrix has the same size and the same type as the input one except for the last case,
/// where C is depth=CV_8U .
/// ## Parameters
/// * m: matrix.
/// ## See also
/// @ref MatrixExpressions, absdiff, convertScaleAbs
pub fn abs(m: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_abs_const_MatR(m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

/// Calculates the per-element absolute difference between two arrays or between an array and a scalar.
/// 
/// The function cv::absdiff calculates:
/// *   Absolute difference between two arrays when they have the same
///    size and type:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%7C%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%20%5Ctexttt%7Bsrc2%7D%28I%29%7C%29)
/// *   Absolute difference between an array and a scalar when the second
///    array is constructed from Scalar or has as many elements as the
///    number of channels in `src1`:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%7C%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%20%5Ctexttt%7Bsrc2%7D%20%7C%29)
/// *   Absolute difference between a scalar and an array when the first
///    array is constructed from Scalar or has as many elements as the
///    number of channels in `src2`:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%7C%20%5Ctexttt%7Bsrc1%7D%20%2D%20%20%5Ctexttt%7Bsrc2%7D%28I%29%20%7C%29)
///    where I is a multi-dimensional index of array elements. In case of
///    multi-channel arrays, each channel is processed independently.
/// 
/// Note: Saturation is not applied when the arrays have the depth CV_32S.
/// You may even get a negative value in the case of overflow.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array that has the same size and type as input arrays.
/// ## See also
/// cv::abs(const Mat&)
pub fn absdiff(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Calculates the weighted sum of two arrays.
/// 
/// The function addWeighted calculates the weighted sum of two arrays as follows:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2A%20%5Ctexttt%7Balpha%7D%20%2B%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29%2A%20%5Ctexttt%7Bbeta%7D%20%2B%20%20%5Ctexttt%7Bgamma%7D%20%29)
/// where I is a multi-dimensional index of array elements. In case of multi-channel arrays, each
/// channel is processed independently.
/// The function can be replaced with a matrix expression:
/// ```ignore
///    dst = src1*alpha + src2*beta + gamma;
/// ```
/// 
/// 
/// Note: Saturation is not applied when the output array has the depth CV_32S. You may even get
/// result of an incorrect sign in the case of overflow.
/// ## Parameters
/// * src1: first input array.
/// * alpha: weight of the first array elements.
/// * src2: second input array of the same size and channel number as src1.
/// * beta: weight of the second array elements.
/// * gamma: scalar added to each sum.
/// * dst: output array that has the same size and number of channels as the input arrays.
/// * dtype: optional depth of the output array; when both input arrays have the same depth, dtype
/// can be set to -1, which will be equivalent to src1.depth().
/// ## See also
/// add, subtract, scaleAdd, Mat::convertTo
/// 
/// ## C++ default parameters
/// * dtype: -1
pub fn add_weighted(src1: &dyn core::ToInputArray, alpha: f64, src2: &dyn core::ToInputArray, beta: f64, gamma: f64, dst: &mut dyn core::ToOutputArray, dtype: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR_int(src1.as_raw__InputArray(), alpha, src2.as_raw__InputArray(), beta, gamma, dst.as_raw__OutputArray(), dtype) }.into_result()
}

/// Calculates the per-element sum of two arrays or an array and a scalar.
/// 
/// The function add calculates:
/// - Sum of two arrays when both input arrays have the same size and the same number of channels:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2B%20%20%5Ctexttt%7Bsrc2%7D%28I%29%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%28I%29%20%5Cne0)
/// - Sum of an array and a scalar when src2 is constructed from Scalar or has the same number of
/// elements as `src1.channels()`:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2B%20%20%5Ctexttt%7Bsrc2%7D%20%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%28I%29%20%5Cne0)
/// - Sum of a scalar and an array when src1 is constructed from Scalar or has the same number of
/// elements as `src2.channels()`:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%20%2B%20%20%5Ctexttt%7Bsrc2%7D%28I%29%20%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%28I%29%20%5Cne0)
/// where `I` is a multi-dimensional index of array elements. In case of multi-channel arrays, each
/// channel is processed independently.
/// 
/// The first function in the list above can be replaced with matrix expressions:
/// ```ignore
///    dst = src1 + src2;
///    dst += src1; // equivalent to add(dst, src1, dst);
/// ```
/// 
/// The input arrays and the output array can all have the same or different depths. For example, you
/// can add a 16-bit unsigned array to a 8-bit signed array and store the sum as a 32-bit
/// floating-point array. Depth of the output array is determined by the dtype parameter. In the second
/// and third cases above, as well as in the first case, when src1.depth() == src2.depth(), dtype can
/// be set to the default -1. In this case, the output array will have the same depth as the input
/// array, be it src1, src2 or both.
/// 
/// Note: Saturation is not applied when the output array has the depth CV_32S. You may even get
/// result of an incorrect sign in the case of overflow.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array that has the same size and number of channels as the input array(s); the
/// depth is defined by dtype or src1/src2.
/// * mask: optional operation mask - 8-bit single channel array, that specifies elements of the
/// output array to be changed.
/// * dtype: optional depth of the output array (see the discussion below).
/// ## See also
/// subtract, addWeighted, scaleAdd, Mat::convertTo
/// 
/// ## C++ default parameters
/// * mask: noArray()
/// * dtype: -1
pub fn add(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray, dtype: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	input_array_arg!(mask);
	unsafe { sys::cv_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), dtype) }.into_result()
}

/// naive nearest neighbor finder
/// 
/// see http://en.wikipedia.org/wiki/Nearest_neighbor_search
/// @todo document
/// 
/// ## C++ default parameters
/// * norm_type: NORM_L2
/// * k: 0
/// * mask: noArray()
/// * update: 0
/// * crosscheck: false
pub fn batch_distance(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dist: &mut dyn core::ToOutputArray, dtype: i32, nidx: &mut dyn core::ToOutputArray, norm_type: i32, k: i32, mask: &dyn core::ToInputArray, update: i32, crosscheck: bool) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dist);
	output_array_arg!(nidx);
	input_array_arg!(mask);
	unsafe { sys::cv_batchDistance_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__OutputArrayR_int_int_const__InputArrayR_int_bool(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dist.as_raw__OutputArray(), dtype, nidx.as_raw__OutputArray(), norm_type, k, mask.as_raw__InputArray(), update, crosscheck) }.into_result()
}

/// computes bitwise conjunction of the two arrays (dst = src1 & src2)
/// Calculates the per-element bit-wise conjunction of two arrays or an
/// array and a scalar.
/// 
/// The function cv::bitwise_and calculates the per-element bit-wise logical conjunction for:
/// *   Two arrays when src1 and src2 have the same size:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Cwedge%20%5Ctexttt%7Bsrc2%7D%20%28I%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%20%28I%29%20%5Cne0)
/// *   An array and a scalar when src2 is constructed from Scalar or has
///    the same number of elements as `src1.channels()`:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Cwedge%20%5Ctexttt%7Bsrc2%7D%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%20%28I%29%20%5Cne0)
/// *   A scalar and an array when src1 is constructed from Scalar or has
///    the same number of elements as `src2.channels()`:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%20%5Cwedge%20%5Ctexttt%7Bsrc2%7D%20%28I%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%20%28I%29%20%5Cne0)
/// In case of floating-point arrays, their machine-specific bit
/// representations (usually IEEE754-compliant) are used for the operation.
/// In case of multi-channel arrays, each channel is processed
/// independently. In the second and third cases above, the scalar is first
/// converted to the array type.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array that has the same size and type as the input
/// arrays.
/// * mask: optional operation mask, 8-bit single channel array, that
/// specifies elements of the output array to be changed.
/// 
/// ## C++ default parameters
/// * mask: noArray()
pub fn bitwise_and(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	input_array_arg!(mask);
	unsafe { sys::cv_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
}

///  Inverts every bit of an array.
/// 
/// The function cv::bitwise_not calculates per-element bit-wise inversion of the input
/// array:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Cneg%20%5Ctexttt%7Bsrc%7D%20%28I%29)
/// In case of a floating-point input array, its machine-specific bit
/// representation (usually IEEE754-compliant) is used for the operation. In
/// case of multi-channel arrays, each channel is processed independently.
/// ## Parameters
/// * src: input array.
/// * dst: output array that has the same size and type as the input
/// array.
/// * mask: optional operation mask, 8-bit single channel array, that
/// specifies elements of the output array to be changed.
/// 
/// ## C++ default parameters
/// * mask: noArray()
pub fn bitwise_not(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(mask);
	unsafe { sys::cv_bitwise_not_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
}

/// Calculates the per-element bit-wise disjunction of two arrays or an
/// array and a scalar.
/// 
/// The function cv::bitwise_or calculates the per-element bit-wise logical disjunction for:
/// *   Two arrays when src1 and src2 have the same size:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Cvee%20%5Ctexttt%7Bsrc2%7D%20%28I%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%20%28I%29%20%5Cne0)
/// *   An array and a scalar when src2 is constructed from Scalar or has
///    the same number of elements as `src1.channels()`:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Cvee%20%5Ctexttt%7Bsrc2%7D%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%20%28I%29%20%5Cne0)
/// *   A scalar and an array when src1 is constructed from Scalar or has
///    the same number of elements as `src2.channels()`:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%20%5Cvee%20%5Ctexttt%7Bsrc2%7D%20%28I%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%20%28I%29%20%5Cne0)
/// In case of floating-point arrays, their machine-specific bit
/// representations (usually IEEE754-compliant) are used for the operation.
/// In case of multi-channel arrays, each channel is processed
/// independently. In the second and third cases above, the scalar is first
/// converted to the array type.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array that has the same size and type as the input
/// arrays.
/// * mask: optional operation mask, 8-bit single channel array, that
/// specifies elements of the output array to be changed.
/// 
/// ## C++ default parameters
/// * mask: noArray()
pub fn bitwise_or(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	input_array_arg!(mask);
	unsafe { sys::cv_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
}

/// Calculates the per-element bit-wise "exclusive or" operation on two
/// arrays or an array and a scalar.
/// 
/// The function cv::bitwise_xor calculates the per-element bit-wise logical "exclusive-or"
/// operation for:
/// *   Two arrays when src1 and src2 have the same size:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Coplus%20%5Ctexttt%7Bsrc2%7D%20%28I%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%20%28I%29%20%5Cne0)
/// *   An array and a scalar when src2 is constructed from Scalar or has
///    the same number of elements as `src1.channels()`:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Coplus%20%5Ctexttt%7Bsrc2%7D%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%20%28I%29%20%5Cne0)
/// *   A scalar and an array when src1 is constructed from Scalar or has
///    the same number of elements as `src2.channels()`:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%20%5Coplus%20%5Ctexttt%7Bsrc2%7D%20%28I%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%20%28I%29%20%5Cne0)
/// In case of floating-point arrays, their machine-specific bit
/// representations (usually IEEE754-compliant) are used for the operation.
/// In case of multi-channel arrays, each channel is processed
/// independently. In the 2nd and 3rd cases above, the scalar is first
/// converted to the array type.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array that has the same size and type as the input
/// arrays.
/// * mask: optional operation mask, 8-bit single channel array, that
/// specifies elements of the output array to be changed.
/// 
/// ## C++ default parameters
/// * mask: noArray()
pub fn bitwise_xor(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	input_array_arg!(mask);
	unsafe { sys::cv_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
}

/// Computes the source location of an extrapolated pixel.
/// 
/// The function computes and returns the coordinate of a donor pixel corresponding to the specified
/// extrapolated pixel when using the specified extrapolation border mode. For example, if you use
/// cv::BORDER_WRAP mode in the horizontal direction, cv::BORDER_REFLECT_101 in the vertical direction and
/// want to compute value of the "virtual" pixel Point(-5, 100) in a floating-point image img , it
/// looks like:
/// ```ignore
///    float val = img.at<float>(borderInterpolate(100, img.rows, cv::BORDER_REFLECT_101),
///                               borderInterpolate(-5, img.cols, cv::BORDER_WRAP));
/// ```
/// 
/// Normally, the function is not called directly. It is used inside filtering functions and also in
/// copyMakeBorder.
/// ## Parameters
/// * p: 0-based coordinate of the extrapolated pixel along one of the axes, likely \<0 or \>= len
/// * len: Length of the array along the corresponding axis.
/// * borderType: Border type, one of the #BorderTypes, except for #BORDER_TRANSPARENT and
/// #BORDER_ISOLATED . When borderType==#BORDER_CONSTANT , the function always returns -1, regardless
/// of p and len.
/// ## See also
/// copyMakeBorder
pub fn border_interpolate(p: i32, len: i32, border_type: i32) -> Result<i32> {
	unsafe { sys::cv_borderInterpolate_int_int_int(p, len, border_type) }.into_result()
}

/// Calculates the covariance matrix of a set of vectors.
/// 
/// The function cv::calcCovarMatrix calculates the covariance matrix and, optionally, the mean vector of
/// the set of input vectors.
/// ## Parameters
/// * samples: samples stored as separate matrices
/// * nsamples: number of samples
/// * covar: output covariance matrix of the type ctype and square size.
/// * mean: input or output (depending on the flags) array as the average value of the input vectors.
/// * flags: operation flags as a combination of #CovarFlags
/// * ctype: type of the matrixl; it equals 'CV_64F' by default.
/// ## See also
/// PCA, mulTransposed, Mahalanobis
/// @todo InputArrayOfArrays
/// 
/// ## Overloaded parameters
/// 
/// 
/// Note: use #COVAR_ROWS or #COVAR_COLS flag
/// * samples: samples stored as rows/columns of a single matrix.
/// * covar: output covariance matrix of the type ctype and square size.
/// * mean: input or output (depending on the flags) array as the average value of the input vectors.
/// * flags: operation flags as a combination of #CovarFlags
/// * ctype: type of the matrixl; it equals 'CV_64F' by default.
/// 
/// ## C++ default parameters
/// * ctype: CV_64F
pub fn calc_covar_matrix(samples: &dyn core::ToInputArray, covar: &mut dyn core::ToOutputArray, mean: &mut dyn core::ToInputOutputArray, flags: i32, ctype: i32) -> Result<()> {
	input_array_arg!(samples);
	output_array_arg!(covar);
	input_output_array_arg!(mean);
	unsafe { sys::cv_calcCovarMatrix_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_int_int(samples.as_raw__InputArray(), covar.as_raw__OutputArray(), mean.as_raw__InputOutputArray(), flags, ctype) }.into_result()
}

/// Calculates the magnitude and angle of 2D vectors.
/// 
/// The function cv::cartToPolar calculates either the magnitude, angle, or both
/// for every 2D vector (x(I),y(I)):
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctexttt%7Bmagnitude%7D%20%28I%29%3D%20%5Csqrt%7B%5Ctexttt%7Bx%7D%28I%29%5E2%2B%5Ctexttt%7By%7D%28I%29%5E2%7D%20%2C%20%5C%5C%20%5Ctexttt%7Bangle%7D%20%28I%29%3D%20%5Ctexttt%7Batan2%7D%20%28%20%5Ctexttt%7By%7D%20%28I%29%2C%20%5Ctexttt%7Bx%7D%20%28I%29%29%5B%20%5Ccdot180%20%2F%20%5Cpi%20%5D%20%5Cend%7Barray%7D)
/// 
/// The angles are calculated with accuracy about 0.3 degrees. For the point
/// (0,0), the angle is set to 0.
/// ## Parameters
/// * x: array of x-coordinates; this must be a single-precision or
/// double-precision floating-point array.
/// * y: array of y-coordinates, that must have the same size and same type as x.
/// * magnitude: output array of magnitudes of the same size and type as x.
/// * angle: output array of angles that has the same size and type as
/// x; the angles are measured in radians (from 0 to 2\*Pi) or in degrees (0 to 360 degrees).
/// * angleInDegrees: a flag, indicating whether the angles are measured
/// in radians (which is by default), or in degrees.
/// ## See also
/// Sobel, Scharr
/// 
/// ## C++ default parameters
/// * angle_in_degrees: false
pub fn cart_to_polar(x: &dyn core::ToInputArray, y: &dyn core::ToInputArray, magnitude: &mut dyn core::ToOutputArray, angle: &mut dyn core::ToOutputArray, angle_in_degrees: bool) -> Result<()> {
	input_array_arg!(x);
	input_array_arg!(y);
	output_array_arg!(magnitude);
	output_array_arg!(angle);
	unsafe { sys::cv_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(x.as_raw__InputArray(), y.as_raw__InputArray(), magnitude.as_raw__OutputArray(), angle.as_raw__OutputArray(), angle_in_degrees) }.into_result()
}

/// Returns true if the specified feature is supported by the host hardware.
/// 
/// The function returns true if the host hardware supports the specified feature. When user calls
/// setUseOptimized(false), the subsequent calls to checkHardwareSupport() will return false until
/// setUseOptimized(true) is called. This way user can dynamically switch on and off the optimized code
/// in OpenCV.
/// ## Parameters
/// * feature: The feature of interest, one of cv::CpuFeatures
pub fn check_hardware_support(feature: i32) -> Result<bool> {
	unsafe { sys::cv_checkHardwareSupport_int(feature) }.into_result()
}

/// Checks every element of an input array for invalid values.
/// 
/// The function cv::checkRange checks that every array element is neither NaN nor infinite. When minVal \>
/// -DBL_MAX and maxVal \< DBL_MAX, the function also checks that each value is between minVal and
/// maxVal. In case of multi-channel arrays, each channel is processed independently. If some values
/// are out of range, position of the first outlier is stored in pos (when pos != NULL). Then, the
/// function either returns false (when quiet=true) or throws an exception.
/// ## Parameters
/// * a: input array.
/// * quiet: a flag, indicating whether the functions quietly return false when the array elements
/// are out of range or they throw an exception.
/// * pos: optional output parameter, when not NULL, must be a pointer to array of src.dims
/// elements.
/// * minVal: inclusive lower boundary of valid values range.
/// * maxVal: exclusive upper boundary of valid values range.
/// 
/// ## C++ default parameters
/// * quiet: true
/// * pos: 0
/// * min_val: -DBL_MAX
/// * max_val: DBL_MAX
pub fn check_range(a: &dyn core::ToInputArray, quiet: bool, pos: &mut core::Point, min_val: f64, max_val: f64) -> Result<bool> {
	input_array_arg!(a);
	unsafe { sys::cv_checkRange_const__InputArrayR_bool_PointX_double_double(a.as_raw__InputArray(), quiet, pos, min_val, max_val) }.into_result()
}

/// Performs the per-element comparison of two arrays or an array and scalar value.
/// 
/// The function compares:
/// *   Elements of two arrays when src1 and src2 have the same size:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5C%2C%5Ctexttt%7Bcmpop%7D%5C%2C%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
/// *   Elements of src1 with a scalar src2 when src2 is constructed from
///    Scalar or has a single element:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%28I%29%20%5C%2C%5Ctexttt%7Bcmpop%7D%5C%2C%20%20%5Ctexttt%7Bsrc2%7D)
/// *   src1 with elements of src2 when src1 is constructed from Scalar or
///    has a single element:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%20%5C%2C%5Ctexttt%7Bcmpop%7D%5C%2C%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
/// When the comparison result is true, the corresponding element of output
/// array is set to 255. The comparison operations can be replaced with the
/// equivalent matrix expressions:
/// ```ignore
///    Mat dst1 = src1 >= src2;
///    Mat dst2 = src1 < 8;
///    ...
/// ```
/// 
/// ## Parameters
/// * src1: first input array or a scalar; when it is an array, it must have a single channel.
/// * src2: second input array or a scalar; when it is an array, it must have a single channel.
/// * dst: output array of type ref CV_8U that has the same size and the same number of channels as
///    the input arrays.
/// * cmpop: a flag, that specifies correspondence between the arrays (cv::CmpTypes)
/// ## See also
/// checkRange, min, max, threshold
pub fn compare(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, cmpop: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), cmpop) }.into_result()
}

/// Copies the lower or the upper half of a square matrix to its another half.
/// 
/// The function cv::completeSymm copies the lower or the upper half of a square matrix to
/// its another half. The matrix diagonal remains unchanged:
///  - ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bm%7D%5F%7Bij%7D%3D%5Ctexttt%7Bm%7D%5F%7Bji%7D) for ![inline formula](https://latex.codecogs.com/png.latex?i%20%3E%20j) if
///    lowerToUpper=false
///  - ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bm%7D%5F%7Bij%7D%3D%5Ctexttt%7Bm%7D%5F%7Bji%7D) for ![inline formula](https://latex.codecogs.com/png.latex?i%20%3C%20j) if
///    lowerToUpper=true
/// 
/// ## Parameters
/// * m: input-output floating-point square matrix.
/// * lowerToUpper: operation flag; if true, the lower half is copied to
/// the upper half. Otherwise, the upper half is copied to the lower half.
/// ## See also
/// flip, transpose
/// 
/// ## C++ default parameters
/// * lower_to_upper: false
pub fn complete_symm(m: &mut dyn core::ToInputOutputArray, lower_to_upper: bool) -> Result<()> {
	input_output_array_arg!(m);
	unsafe { sys::cv_completeSymm_const__InputOutputArrayR_bool(m.as_raw__InputOutputArray(), lower_to_upper) }.into_result()
}

/// Converts an array to half precision floating number.
/// 
/// This function converts FP32 (single precision floating point) from/to FP16 (half precision floating point). CV_16S format is used to represent FP16 data.
/// There are two use modes (src -> dst): CV_32F -> CV_16S and CV_16S -> CV_32F. The input array has to have type of CV_32F or
/// CV_16S to represent the bit depth. If the input array is neither of them, the function will raise an error.
/// The format of half precision floating point is defined in IEEE 754-2008.
/// 
/// ## Parameters
/// * src: input array.
/// * dst: output array.
pub fn convert_fp16(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_convertFp16_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Scales, calculates absolute values, and converts the result to 8-bit.
/// 
/// On each element of the input array, the function convertScaleAbs
/// performs three operations sequentially: scaling, taking an absolute
/// value, conversion to an unsigned 8-bit type:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%5C%5Fcast%3Cuchar%3E%7D%20%28%7C%20%5Ctexttt%7Bsrc%7D%20%28I%29%2A%20%5Ctexttt%7Balpha%7D%20%2B%20%20%5Ctexttt%7Bbeta%7D%20%7C%29)
/// In case of multi-channel arrays, the function processes each channel
/// independently. When the output is not 8-bit, the operation can be
/// emulated by calling the Mat::convertTo method (or by using matrix
/// expressions) and then by calculating an absolute value of the result.
/// For example:
/// ```ignore
///    Mat_<float> A(30,30);
///    randu(A, Scalar(-100), Scalar(100));
///    Mat_<float> B = A*5 + 3;
///    B = abs(B);
///    // Mat_<float> B = abs(A*5+3) will also do the job,
///    // but it will allocate a temporary matrix
/// ```
/// 
/// ## Parameters
/// * src: input array.
/// * dst: output array.
/// * alpha: optional scale factor.
/// * beta: optional delta added to the scaled values.
/// ## See also
/// Mat::convertTo, cv::abs(const Mat&)
/// 
/// ## C++ default parameters
/// * alpha: 1
/// * beta: 0
pub fn convert_scale_abs(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, alpha: f64, beta: f64) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_convertScaleAbs_const__InputArrayR_const__OutputArrayR_double_double(src.as_raw__InputArray(), dst.as_raw__OutputArray(), alpha, beta) }.into_result()
}

/// Forms a border around an image.
/// 
/// The function copies the source image into the middle of the destination image. The areas to the
/// left, to the right, above and below the copied source image will be filled with extrapolated
/// pixels. This is not what filtering functions based on it do (they extrapolate pixels on-fly), but
/// what other more complex functions, including your own, may do to simplify image boundary handling.
/// 
/// The function supports the mode when src is already in the middle of dst . In this case, the
/// function does not copy src itself but simply constructs the border, for example:
/// 
/// ```ignore
///    // let border be the same in all directions
///    int border=2;
///    // constructs a larger image to fit both the image and the border
///    Mat gray_buf(rgb.rows + border*2, rgb.cols + border*2, rgb.depth());
///    // select the middle part of it w/o copying data
///    Mat gray(gray_canvas, Rect(border, border, rgb.cols, rgb.rows));
///    // convert image from RGB to grayscale
///    cvtColor(rgb, gray, COLOR_RGB2GRAY);
///    // form a border in-place
///    copyMakeBorder(gray, gray_buf, border, border,
///                    border, border, BORDER_REPLICATE);
///    // now do some custom filtering ...
///    ...
/// ```
/// 
/// 
/// Note: When the source image is a part (ROI) of a bigger image, the function will try to use the
/// pixels outside of the ROI to form a border. To disable this feature and always do extrapolation, as
/// if src was not a ROI, use borderType | #BORDER_ISOLATED.
/// 
/// ## Parameters
/// * src: Source image.
/// * dst: Destination image of the same type as src and the size Size(src.cols+left+right,
/// src.rows+top+bottom) .
/// * top: the top pixels
/// * bottom: the bottom pixels
/// * left: the left pixels
/// * right: Parameter specifying how many pixels in each direction from the source image rectangle
/// to extrapolate. For example, top=1, bottom=1, left=1, right=1 mean that 1 pixel-wide border needs
/// to be built.
/// * borderType: Border type. See borderInterpolate for details.
/// * value: Border value if borderType==BORDER_CONSTANT .
/// ## See also
/// borderInterpolate
/// 
/// ## C++ default parameters
/// * value: Scalar()
pub fn copy_make_border(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, top: i32, bottom: i32, left: i32, right: i32, border_type: i32, value: core::Scalar) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int_const_ScalarR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), top, bottom, left, right, border_type, &value) }.into_result()
}

///  This is an overloaded member function, provided for convenience (python)
/// Copies the matrix to another one.
/// When the operation mask is specified, if the Mat::create call shown above reallocates the matrix, the newly allocated matrix is initialized with all zeros before copying the data.
/// ## Parameters
/// * src: source matrix.
/// * dst: Destination matrix. If it does not have a proper size or type before the operation, it is
/// reallocated.
/// * mask: Operation mask of the same size as \*this. Its non-zero elements indicate which matrix
/// elements need to be copied. The mask has to be of type CV_8U and can have 1 or multiple channels.
pub fn copy_to(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(mask);
	unsafe { sys::cv_copyTo_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
}

/// Counts non-zero array elements.
/// 
/// The function returns the number of non-zero elements in src :
/// ![block formula](https://latex.codecogs.com/png.latex?%5Csum%20%5F%7BI%3A%20%5C%3B%20%5Ctexttt%7Bsrc%7D%20%28I%29%20%5Cne0%20%7D%201)
/// ## Parameters
/// * src: single-channel array.
/// ## See also
/// mean, meanStdDev, norm, minMaxLoc, calcCovarMatrix
pub fn count_non_zero(src: &dyn core::ToInputArray) -> Result<i32> {
	input_array_arg!(src);
	unsafe { sys::cv_countNonZero_const__InputArrayR(src.as_raw__InputArray()) }.into_result()
}

/// Computes the cube root of an argument.
/// 
/// The function cubeRoot computes ![inline formula](https://latex.codecogs.com/png.latex?%5Csqrt%5B3%5D%7B%5Ctexttt%7Bval%7D%7D). Negative arguments are handled correctly.
/// NaN and Inf are not handled. The accuracy approaches the maximum possible accuracy for
/// single-precision data.
/// ## Parameters
/// * val: A function argument.
pub fn cube_root(val: f32) -> Result<f32> {
	unsafe { sys::cv_cubeRoot_float(val) }.into_result()
}

#[cfg(not(target_os = "windows"))]
/// Converts an array to half precision floating number.
/// 
/// ## Parameters
/// * _src: input array.
/// * _dst: output array.
/// * stream: Stream for the asynchronous version.
/// ## See also
/// convertFp16
/// 
/// ## C++ default parameters
/// * stream: Stream::Null()
pub fn convert_fp16_1(_src: &dyn core::ToInputArray, _dst: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(_src);
	output_array_arg!(_dst);
	unsafe { sys::cv_cuda_convertFp16_const__InputArrayR_const__OutputArrayR_StreamR(_src.as_raw__InputArray(), _dst.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
}

/// Creates a continuous matrix.
/// 
/// ## Parameters
/// * rows: Row count.
/// * cols: Column count.
/// * type: Type of the matrix.
/// * arr: Destination matrix. This parameter changes only if it has a proper type and area (
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Brows%7D%20%5Ctimes%20%5Ctexttt%7Bcols%7D) ).
/// 
/// Matrix is called continuous if its elements are stored continuously, that is, without gaps at the
/// end of each row.
pub fn create_continuous(rows: i32, cols: i32, typ: i32, arr: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(arr);
	unsafe { sys::cv_cuda_createContinuous_int_int_int_const__OutputArrayR(rows, cols, typ, arr.as_raw__OutputArray()) }.into_result()
}

/// checks whether current device supports the given feature
pub fn device_supports(feature_set: core::FeatureSet) -> Result<bool> {
	unsafe { sys::cv_cuda_deviceSupports_FeatureSet(feature_set) }.into_result()
}

/// Ensures that the size of a matrix is big enough and the matrix has a proper type.
/// 
/// ## Parameters
/// * rows: Minimum desired number of rows.
/// * cols: Minimum desired number of columns.
/// * type: Desired matrix type.
/// * arr: Destination matrix.
/// 
/// The function does not reallocate memory if the matrix has proper attributes already.
pub fn ensure_size_is_enough(rows: i32, cols: i32, typ: i32, arr: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(arr);
	unsafe { sys::cv_cuda_ensureSizeIsEnough_int_int_int_const__OutputArrayR(rows, cols, typ, arr.as_raw__OutputArray()) }.into_result()
}

/// Returns the number of installed CUDA-enabled devices.
/// 
/// Use this function before any other CUDA functions calls. If OpenCV is compiled without CUDA support,
/// this function returns 0. If the CUDA driver is not installed, or is incompatible, this function
/// returns -1.
pub fn get_cuda_enabled_device_count() -> Result<i32> {
	unsafe { sys::cv_cuda_getCudaEnabledDeviceCount() }.into_result()
}

/// Returns the current device index set by cuda::setDevice or initialized by default.
pub fn get_device() -> Result<i32> {
	unsafe { sys::cv_cuda_getDevice() }.into_result()
}

pub fn print_cuda_device_info(device: i32) -> Result<()> {
	unsafe { sys::cv_cuda_printCudaDeviceInfo_int(device) }.into_result()
}

pub fn print_short_cuda_device_info(device: i32) -> Result<()> {
	unsafe { sys::cv_cuda_printShortCudaDeviceInfo_int(device) }.into_result()
}

/// Page-locks the memory of matrix and maps it for the device(s).
/// 
/// ## Parameters
/// * m: Input matrix.
pub fn register_page_locked(m: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_cuda_registerPageLocked_MatR(m.as_raw_mut_Mat()) }.into_result()
}

/// Explicitly destroys and cleans up all resources associated with the current device in the current
/// process.
/// 
/// Any subsequent API call to this device will reinitialize the device.
pub fn reset_device() -> Result<()> {
	unsafe { sys::cv_cuda_resetDevice() }.into_result()
}

pub fn set_buffer_pool_config(device_id: i32, stack_size: size_t, stack_count: i32) -> Result<()> {
	unsafe { sys::cv_cuda_setBufferPoolConfig_int_size_t_int(device_id, stack_size, stack_count) }.into_result()
}

/// BufferPool management (must be called before Stream creation)
pub fn set_buffer_pool_usage(on: bool) -> Result<()> {
	unsafe { sys::cv_cuda_setBufferPoolUsage_bool(on) }.into_result()
}

/// Sets a device and initializes it for the current thread.
/// 
/// ## Parameters
/// * device: System index of a CUDA device starting with 0.
/// 
/// If the call of this function is omitted, a default device is initialized at the fist CUDA usage.
pub fn set_device(device: i32) -> Result<()> {
	unsafe { sys::cv_cuda_setDevice_int(device) }.into_result()
}

/// Unmaps the memory of matrix and makes it pageable again.
/// 
/// ## Parameters
/// * m: Input matrix.
pub fn unregister_page_locked(m: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_cuda_unregisterPageLocked_MatR(m.as_raw_mut_Mat()) }.into_result()
}

/// Performs a forward or inverse discrete Cosine transform of 1D or 2D array.
/// 
/// The function cv::dct performs a forward or inverse discrete Cosine transform (DCT) of a 1D or 2D
/// floating-point array:
/// *   Forward Cosine transform of a 1D vector of N elements:
///    ![block formula](https://latex.codecogs.com/png.latex?Y%20%3D%20C%5E%7B%28N%29%7D%20%20%5Ccdot%20X)
///    where
///    ![block formula](https://latex.codecogs.com/png.latex?C%5E%7B%28N%29%7D%5F%7Bjk%7D%3D%20%5Csqrt%7B%5Calpha%5Fj%2FN%7D%20%5Ccos%20%5Cleft%20%28%20%5Cfrac%7B%5Cpi%282k%2B1%29j%7D%7B2N%7D%20%5Cright%20%29)
///    and
///    ![inline formula](https://latex.codecogs.com/png.latex?%5Calpha%5F0%3D1), ![inline formula](https://latex.codecogs.com/png.latex?%5Calpha%5Fj%3D2) for *j \> 0*.
/// *   Inverse Cosine transform of a 1D vector of N elements:
///    ![block formula](https://latex.codecogs.com/png.latex?X%20%3D%20%20%5Cleft%20%28C%5E%7B%28N%29%7D%20%5Cright%20%29%5E%7B%2D1%7D%20%20%5Ccdot%20Y%20%3D%20%20%5Cleft%20%28C%5E%7B%28N%29%7D%20%5Cright%20%29%5ET%20%20%5Ccdot%20Y)
///    (since ![inline formula](https://latex.codecogs.com/png.latex?C%5E%7B%28N%29%7D) is an orthogonal matrix, ![inline formula](https://latex.codecogs.com/png.latex?C%5E%7B%28N%29%7D%20%5Ccdot%20%5Cleft%28C%5E%7B%28N%29%7D%5Cright%29%5ET%20%3D%20I) )
/// *   Forward 2D Cosine transform of M x N matrix:
///    ![block formula](https://latex.codecogs.com/png.latex?Y%20%3D%20C%5E%7B%28N%29%7D%20%20%5Ccdot%20X%20%20%5Ccdot%20%5Cleft%20%28C%5E%7B%28N%29%7D%20%5Cright%20%29%5ET)
/// *   Inverse 2D Cosine transform of M x N matrix:
///    ![block formula](https://latex.codecogs.com/png.latex?X%20%3D%20%20%5Cleft%20%28C%5E%7B%28N%29%7D%20%5Cright%20%29%5ET%20%20%5Ccdot%20X%20%20%5Ccdot%20C%5E%7B%28N%29%7D)
/// 
/// The function chooses the mode of operation by looking at the flags and size of the input array:
/// *   If (flags & #DCT_INVERSE) == 0 , the function does a forward 1D or 2D transform. Otherwise, it
///    is an inverse 1D or 2D transform.
/// *   If (flags & #DCT_ROWS) != 0 , the function performs a 1D transform of each row.
/// *   If the array is a single column or a single row, the function performs a 1D transform.
/// *   If none of the above is true, the function performs a 2D transform.
/// 
/// 
/// Note: Currently dct supports even-size arrays (2, 4, 6 ...). For data analysis and approximation, you
/// can pad the array when necessary.
/// Also, the function performance depends very much, and not monotonically, on the array size (see
/// getOptimalDFTSize ). In the current implementation DCT of a vector of size N is calculated via DFT
/// of a vector of size N/2 . Thus, the optimal DCT size N1 \>= N can be calculated as:
/// ```ignore
///    size_t getOptimalDCTSize(size_t N) { return 2*getOptimalDFTSize((N+1)/2); }
///    N1 = getOptimalDCTSize(N);
/// ```
/// 
/// ## Parameters
/// * src: input floating-point array.
/// * dst: output array of the same size and type as src .
/// * flags: transformation flags as a combination of cv::DftFlags (DCT_*)
/// ## See also
/// dft , getOptimalDFTSize , idct
/// 
/// ## C++ default parameters
/// * flags: 0
pub fn dct(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_dct_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags) }.into_result()
}

/// Returns string of cv::Mat depth value: CV_8U -> "CV_8U" or "<invalid depth>"
pub fn depth_to_string(depth: i32) -> Result<String> {
	unsafe { sys::cv_depthToString_int(depth) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn check_failed_mat_channels_1(v: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_MatChannels_int_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_mat_channels(v1: i32, v2: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_MatChannels_int_int_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_mat_depth_1(v: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_MatDepth_int_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_mat_depth(v1: i32, v2: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_MatDepth_int_int_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_mat_type_1(v: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_MatType_int_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_mat_type(v1: i32, v2: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_MatType_int_int_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_auto_4(v1: core::Size_<i32>, v2: core::Size_<i32>, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_auto_Size__int__Size__int__const_CheckContextR(v1.opencv_to_extern(), v2.opencv_to_extern(), ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_auto_9(v: core::Size_<i32>, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_auto_Size__int__const_CheckContextR(v.opencv_to_extern(), ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_auto_10(v1: &str, ctx: &core::Detail_CheckContext) -> Result<()> {
	extern_container_arg!(v1);
	unsafe { sys::cv_detail_check_failed_auto_const_stringR_const_CheckContextR(v1.opencv_to_extern(), ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_auto_8(v: f64, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_auto_double_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_auto_3(v1: f64, v2: f64, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_auto_double_double_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_auto_7(v: f32, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_auto_float_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_auto_2(v1: f32, v2: f32, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_auto_float_float_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_auto_5(v: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_auto_int_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_auto(v1: i32, v2: i32, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_auto_int_int_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_auto_6(v: size_t, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_auto_size_t_const_CheckContextR(v, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

pub fn check_failed_auto_1(v1: size_t, v2: size_t, ctx: &core::Detail_CheckContext) -> Result<()> {
	unsafe { sys::cv_detail_check_failed_auto_size_t_size_t_const_CheckContextR(v1, v2, ctx.as_raw_Detail_CheckContext()) }.into_result()
}

/// Returns the determinant of a square floating-point matrix.
/// 
/// The function cv::determinant calculates and returns the determinant of the
/// specified matrix. For small matrices ( mtx.cols=mtx.rows\<=3 ), the
/// direct method is used. For larger matrices, the function uses LU
/// factorization with partial pivoting.
/// 
/// For symmetric positively-determined matrices, it is also possible to use
/// eigen decomposition to calculate the determinant.
/// ## Parameters
/// * mtx: input matrix that must have CV_32FC1 or CV_64FC1 type and
/// square size.
/// ## See also
/// trace, invert, solve, eigen, @ref MatrixExpressions
pub fn determinant(mtx: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(mtx);
	unsafe { sys::cv_determinant_const__InputArrayR(mtx.as_raw__InputArray()) }.into_result()
}

/// Performs a forward or inverse Discrete Fourier transform of a 1D or 2D floating-point array.
/// 
/// The function cv::dft performs one of the following:
/// *   Forward the Fourier transform of a 1D vector of N elements:
///    ![block formula](https://latex.codecogs.com/png.latex?Y%20%3D%20F%5E%7B%28N%29%7D%20%20%5Ccdot%20X%2C)
///    where ![inline formula](https://latex.codecogs.com/png.latex?F%5E%7B%28N%29%7D%5F%7Bjk%7D%3D%5Cexp%28%2D2%5Cpi%20i%20j%20k%2FN%29) and ![inline formula](https://latex.codecogs.com/png.latex?i%3D%5Csqrt%7B%2D1%7D)
/// *   Inverse the Fourier transform of a 1D vector of N elements:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20X%27%3D%20%20%5Cleft%20%28F%5E%7B%28N%29%7D%20%5Cright%20%29%5E%7B%2D1%7D%20%20%5Ccdot%20Y%20%3D%20%20%5Cleft%20%28F%5E%7B%28N%29%7D%20%5Cright%20%29%5E%2A%20%20%5Ccdot%20y%20%20%5C%5C%20X%20%3D%20%281%2FN%29%20%20%5Ccdot%20X%2C%20%5Cend%7Barray%7D)
///    where ![inline formula](https://latex.codecogs.com/png.latex?F%5E%2A%3D%5Cleft%28%5Ctextrm%7BRe%7D%28F%5E%7B%28N%29%7D%29%2D%5Ctextrm%7BIm%7D%28F%5E%7B%28N%29%7D%29%5Cright%29%5ET)
/// *   Forward the 2D Fourier transform of a M x N matrix:
///    ![block formula](https://latex.codecogs.com/png.latex?Y%20%3D%20F%5E%7B%28M%29%7D%20%20%5Ccdot%20X%20%20%5Ccdot%20F%5E%7B%28N%29%7D)
/// *   Inverse the 2D Fourier transform of a M x N matrix:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20X%27%3D%20%20%5Cleft%20%28F%5E%7B%28M%29%7D%20%5Cright%20%29%5E%2A%20%20%5Ccdot%20Y%20%20%5Ccdot%20%5Cleft%20%28F%5E%7B%28N%29%7D%20%5Cright%20%29%5E%2A%20%5C%5C%20X%20%3D%20%20%5Cfrac%7B1%7D%7BM%20%5Ccdot%20N%7D%20%5Ccdot%20X%27%20%5Cend%7Barray%7D)
/// 
/// In case of real (single-channel) data, the output spectrum of the forward Fourier transform or input
/// spectrum of the inverse Fourier transform can be represented in a packed format called *CCS*
/// (complex-conjugate-symmetrical). It was borrowed from IPL (Intel\* Image Processing Library). Here
/// is how 2D *CCS* spectrum looks:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20Re%20Y%5F%7B0%2C0%7D%20%26%20Re%20Y%5F%7B0%2C1%7D%20%26%20Im%20Y%5F%7B0%2C1%7D%20%26%20Re%20Y%5F%7B0%2C2%7D%20%26%20Im%20Y%5F%7B0%2C2%7D%20%26%20%20%5Ccdots%20%26%20Re%20Y%5F%7B0%2CN%2F2%2D1%7D%20%26%20Im%20Y%5F%7B0%2CN%2F2%2D1%7D%20%26%20Re%20Y%5F%7B0%2CN%2F2%7D%20%20%5C%5C%20Re%20Y%5F%7B1%2C0%7D%20%26%20Re%20Y%5F%7B1%2C1%7D%20%26%20Im%20Y%5F%7B1%2C1%7D%20%26%20Re%20Y%5F%7B1%2C2%7D%20%26%20Im%20Y%5F%7B1%2C2%7D%20%26%20%20%5Ccdots%20%26%20Re%20Y%5F%7B1%2CN%2F2%2D1%7D%20%26%20Im%20Y%5F%7B1%2CN%2F2%2D1%7D%20%26%20Re%20Y%5F%7B1%2CN%2F2%7D%20%20%5C%5C%20Im%20Y%5F%7B1%2C0%7D%20%26%20Re%20Y%5F%7B2%2C1%7D%20%26%20Im%20Y%5F%7B2%2C1%7D%20%26%20Re%20Y%5F%7B2%2C2%7D%20%26%20Im%20Y%5F%7B2%2C2%7D%20%26%20%20%5Ccdots%20%26%20Re%20Y%5F%7B2%2CN%2F2%2D1%7D%20%26%20Im%20Y%5F%7B2%2CN%2F2%2D1%7D%20%26%20Im%20Y%5F%7B1%2CN%2F2%7D%20%20%5C%5C%20%5Cdots%20%5C%5C%20Re%20Y%5F%7BM%2F2%2D1%2C0%7D%20%26%20%20Re%20Y%5F%7BM%2D3%2C1%7D%20%20%26%20Im%20Y%5F%7BM%2D3%2C1%7D%20%26%20%20%5Cdots%20%26%20Re%20Y%5F%7BM%2D3%2CN%2F2%2D1%7D%20%26%20Im%20Y%5F%7BM%2D3%2CN%2F2%2D1%7D%26%20Re%20Y%5F%7BM%2F2%2D1%2CN%2F2%7D%20%20%5C%5C%20Im%20Y%5F%7BM%2F2%2D1%2C0%7D%20%26%20%20Re%20Y%5F%7BM%2D2%2C1%7D%20%20%26%20Im%20Y%5F%7BM%2D2%2C1%7D%20%26%20%20%5Cdots%20%26%20Re%20Y%5F%7BM%2D2%2CN%2F2%2D1%7D%20%26%20Im%20Y%5F%7BM%2D2%2CN%2F2%2D1%7D%26%20Im%20Y%5F%7BM%2F2%2D1%2CN%2F2%7D%20%20%5C%5C%20Re%20Y%5F%7BM%2F2%2C0%7D%20%20%26%20%20Re%20Y%5F%7BM%2D1%2C1%7D%20%26%20%20Im%20Y%5F%7BM%2D1%2C1%7D%20%26%20%20%5Cdots%20%26%20Re%20Y%5F%7BM%2D1%2CN%2F2%2D1%7D%20%26%20Im%20Y%5F%7BM%2D1%2CN%2F2%2D1%7D%26%20Re%20Y%5F%7BM%2F2%2CN%2F2%7D%20%5Cend%7Bbmatrix%7D)
/// 
/// In case of 1D transform of a real vector, the output looks like the first row of the matrix above.
/// 
/// So, the function chooses an operation mode depending on the flags and size of the input array:
/// *   If #DFT_ROWS is set or the input array has a single row or single column, the function
///    performs a 1D forward or inverse transform of each row of a matrix when #DFT_ROWS is set.
///    Otherwise, it performs a 2D transform.
/// *   If the input array is real and #DFT_INVERSE is not set, the function performs a forward 1D or
///    2D transform:
///    *   When #DFT_COMPLEX_OUTPUT is set, the output is a complex matrix of the same size as
///        input.
///    *   When #DFT_COMPLEX_OUTPUT is not set, the output is a real matrix of the same size as
///        input. In case of 2D transform, it uses the packed format as shown above. In case of a
///        single 1D transform, it looks like the first row of the matrix above. In case of
///        multiple 1D transforms (when using the #DFT_ROWS flag), each row of the output matrix
///        looks like the first row of the matrix above.
/// *   If the input array is complex and either #DFT_INVERSE or #DFT_REAL_OUTPUT are not set, the
///    output is a complex array of the same size as input. The function performs a forward or
///    inverse 1D or 2D transform of the whole input array or each row of the input array
///    independently, depending on the flags DFT_INVERSE and DFT_ROWS.
/// *   When #DFT_INVERSE is set and the input array is real, or it is complex but #DFT_REAL_OUTPUT
///    is set, the output is a real array of the same size as input. The function performs a 1D or 2D
///    inverse transformation of the whole input array or each individual row, depending on the flags
///    #DFT_INVERSE and #DFT_ROWS.
/// 
/// If #DFT_SCALE is set, the scaling is done after the transformation.
/// 
/// Unlike dct , the function supports arrays of arbitrary size. But only those arrays are processed
/// efficiently, whose sizes can be factorized in a product of small prime numbers (2, 3, and 5 in the
/// current implementation). Such an efficient DFT size can be calculated using the getOptimalDFTSize
/// method.
/// 
/// The sample below illustrates how to calculate a DFT-based convolution of two 2D real arrays:
/// ```ignore
///    void convolveDFT(InputArray A, InputArray B, OutputArray C)
///    {
///        // reallocate the output array if needed
///        C.create(abs(A.rows - B.rows)+1, abs(A.cols - B.cols)+1, A.type());
///        Size dftSize;
///        // calculate the size of DFT transform
///        dftSize.width = getOptimalDFTSize(A.cols + B.cols - 1);
///        dftSize.height = getOptimalDFTSize(A.rows + B.rows - 1);
/// 
///        // allocate temporary buffers and initialize them with 0's
///        Mat tempA(dftSize, A.type(), Scalar::all(0));
///        Mat tempB(dftSize, B.type(), Scalar::all(0));
/// 
///        // copy A and B to the top-left corners of tempA and tempB, respectively
///        Mat roiA(tempA, Rect(0,0,A.cols,A.rows));
///        A.copyTo(roiA);
///        Mat roiB(tempB, Rect(0,0,B.cols,B.rows));
///        B.copyTo(roiB);
/// 
///        // now transform the padded A & B in-place;
///        // use "nonzeroRows" hint for faster processing
///        dft(tempA, tempA, 0, A.rows);
///        dft(tempB, tempB, 0, B.rows);
/// 
///        // multiply the spectrums;
///        // the function handles packed spectrum representations well
///        mulSpectrums(tempA, tempB, tempA);
/// 
///        // transform the product back from the frequency domain.
///        // Even though all the result rows will be non-zero,
///        // you need only the first C.rows of them, and thus you
///        // pass nonzeroRows == C.rows
///        dft(tempA, tempA, DFT_INVERSE + DFT_SCALE, C.rows);
/// 
///        // now copy the result back to C.
///        tempA(Rect(0, 0, C.cols, C.rows)).copyTo(C);
/// 
///        // all the temporary buffers will be deallocated automatically
///    }
/// ```
/// 
/// To optimize this sample, consider the following approaches:
/// *   Since nonzeroRows != 0 is passed to the forward transform calls and since A and B are copied to
///    the top-left corners of tempA and tempB, respectively, it is not necessary to clear the whole
///    tempA and tempB. It is only necessary to clear the tempA.cols - A.cols ( tempB.cols - B.cols)
///    rightmost columns of the matrices.
/// *   This DFT-based convolution does not have to be applied to the whole big arrays, especially if B
///    is significantly smaller than A or vice versa. Instead, you can calculate convolution by parts.
///    To do this, you need to split the output array C into multiple tiles. For each tile, estimate
///    which parts of A and B are required to calculate convolution in this tile. If the tiles in C are
///    too small, the speed will decrease a lot because of repeated work. In the ultimate case, when
///    each tile in C is a single pixel, the algorithm becomes equivalent to the naive convolution
///    algorithm. If the tiles are too big, the temporary arrays tempA and tempB become too big and
///    there is also a slowdown because of bad cache locality. So, there is an optimal tile size
///    somewhere in the middle.
/// *   If different tiles in C can be calculated in parallel and, thus, the convolution is done by
///    parts, the loop can be threaded.
/// 
/// All of the above improvements have been implemented in #matchTemplate and #filter2D . Therefore, by
/// using them, you can get the performance even better than with the above theoretically optimal
/// implementation. Though, those two functions actually calculate cross-correlation, not convolution,
/// so you need to "flip" the second convolution operand B vertically and horizontally using flip .
/// 
/// Note:
/// *   An example using the discrete fourier transform can be found at
///    opencv_source_code/samples/cpp/dft.cpp
/// *   (Python) An example using the dft functionality to perform Wiener deconvolution can be found
///    at opencv_source/samples/python/deconvolution.py
/// *   (Python) An example rearranging the quadrants of a Fourier image can be found at
///    opencv_source/samples/python/dft.py
/// ## Parameters
/// * src: input array that could be real or complex.
/// * dst: output array whose size and type depends on the flags .
/// * flags: transformation flags, representing a combination of the #DftFlags
/// * nonzeroRows: when the parameter is not zero, the function assumes that only the first
/// nonzeroRows rows of the input array (#DFT_INVERSE is not set) or only the first nonzeroRows of the
/// output array (#DFT_INVERSE is set) contain non-zeros, thus, the function can handle the rest of the
/// rows more efficiently and save some time; this technique is very useful for calculating array
/// cross-correlation or convolution using DFT.
/// ## See also
/// dct , getOptimalDFTSize , mulSpectrums, filter2D , matchTemplate , flip , cartToPolar ,
/// magnitude , phase
/// 
/// ## C++ default parameters
/// * flags: 0
/// * nonzero_rows: 0
pub fn dft(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32, nonzero_rows: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_dft_const__InputArrayR_const__OutputArrayR_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, nonzero_rows) }.into_result()
}

/// Get OpenCV type from DirectX type
/// ## Parameters
/// * iD3DFORMAT: - enum D3DTYPE for D3D9
/// ## Returns
/// OpenCV type or -1 if there is no equivalent
pub fn get_type_from_d3d_format(i_d3_dformat: i32) -> Result<i32> {
	unsafe { sys::cv_directx_getTypeFromD3DFORMAT_int(i_d3_dformat) }.into_result()
}

/// Get OpenCV type from DirectX type
/// ## Parameters
/// * iDXGI_FORMAT: - enum DXGI_FORMAT for D3D10/D3D11
/// ## Returns
/// OpenCV type or -1 if there is no equivalent
pub fn get_type_from_dxgi_format(i_dxgi_format: i32) -> Result<i32> {
	unsafe { sys::cv_directx_getTypeFromDXGI_FORMAT_int(i_dxgi_format) }.into_result()
}

/// Performs per-element division of two arrays or a scalar by an array.
/// 
/// The function cv::divide divides one array by another:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%28I%29%20%3D%20saturate%28src1%28I%29%2Ascale%2Fsrc2%28I%29%29%7D)
/// or a scalar by an array when there is no src1 :
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%28I%29%20%3D%20saturate%28scale%2Fsrc2%28I%29%29%7D)
/// 
/// Different channels of multi-channel arrays are processed independently.
/// 
/// For integer types when src2(I) is zero, dst(I) will also be zero.
/// 
/// 
/// Note: In case of floating point data there is no special defined behavior for zero src2(I) values.
/// Regular floating-point division is used.
/// Expect correct IEEE-754 behaviour for floating-point data (with NaN, Inf result values).
/// 
/// 
/// Note: Saturation is not applied when the output array has the depth CV_32S. You may even get
/// result of an incorrect sign in the case of overflow.
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1.
/// * scale: scalar factor.
/// * dst: output array of the same size and type as src2.
/// * dtype: optional depth of the output array; if -1, dst will have depth src2.depth(), but in
/// case of an array-by-array division, you can only pass -1 when src1.depth()==src2.depth().
/// ## See also
/// multiply, add, subtract
/// 
/// ## C++ default parameters
/// * scale: 1
/// * dtype: -1
pub fn divide2(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, scale: f64, dtype: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), scale, dtype) }.into_result()
}

/// Performs per-element division of two arrays or a scalar by an array.
/// 
/// The function cv::divide divides one array by another:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%28I%29%20%3D%20saturate%28src1%28I%29%2Ascale%2Fsrc2%28I%29%29%7D)
/// or a scalar by an array when there is no src1 :
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%28I%29%20%3D%20saturate%28scale%2Fsrc2%28I%29%29%7D)
/// 
/// Different channels of multi-channel arrays are processed independently.
/// 
/// For integer types when src2(I) is zero, dst(I) will also be zero.
/// 
/// 
/// Note: In case of floating point data there is no special defined behavior for zero src2(I) values.
/// Regular floating-point division is used.
/// Expect correct IEEE-754 behaviour for floating-point data (with NaN, Inf result values).
/// 
/// 
/// Note: Saturation is not applied when the output array has the depth CV_32S. You may even get
/// result of an incorrect sign in the case of overflow.
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1.
/// * scale: scalar factor.
/// * dst: output array of the same size and type as src2.
/// * dtype: optional depth of the output array; if -1, dst will have depth src2.depth(), but in
/// case of an array-by-array division, you can only pass -1 when src1.depth()==src2.depth().
/// ## See also
/// multiply, add, subtract
/// 
/// ## Overloaded parameters
/// 
/// ## C++ default parameters
/// * dtype: -1
pub fn divide(scale: f64, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, dtype: i32) -> Result<()> {
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_divide_double_const__InputArrayR_const__OutputArrayR_int(scale, src2.as_raw__InputArray(), dst.as_raw__OutputArray(), dtype) }.into_result()
}

/// Calculates eigenvalues and eigenvectors of a non-symmetric matrix (real eigenvalues only).
/// 
/// 
/// Note: Assumes real eigenvalues.
/// 
/// The function calculates eigenvalues and eigenvectors (optional) of the square matrix src:
/// ```ignore
///    src*eigenvectors.row(i).t() = eigenvalues.at<srcType>(i)*eigenvectors.row(i).t()
/// ```
/// 
/// 
/// ## Parameters
/// * src: input matrix (CV_32FC1 or CV_64FC1 type).
/// * eigenvalues: output vector of eigenvalues (type is the same type as src).
/// * eigenvectors: output matrix of eigenvectors (type is the same type as src). The eigenvectors are stored as subsequent matrix rows, in the same order as the corresponding eigenvalues.
/// ## See also
/// eigen
pub fn eigen_non_symmetric(src: &dyn core::ToInputArray, eigenvalues: &mut dyn core::ToOutputArray, eigenvectors: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(eigenvalues);
	output_array_arg!(eigenvectors);
	unsafe { sys::cv_eigenNonSymmetric_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), eigenvalues.as_raw__OutputArray(), eigenvectors.as_raw__OutputArray()) }.into_result()
}

/// Calculates eigenvalues and eigenvectors of a symmetric matrix.
/// 
/// The function cv::eigen calculates just eigenvalues, or eigenvalues and eigenvectors of the symmetric
/// matrix src:
/// ```ignore
///    src*eigenvectors.row(i).t() = eigenvalues.at<srcType>(i)*eigenvectors.row(i).t()
/// ```
/// 
/// 
/// 
/// Note: Use cv::eigenNonSymmetric for calculation of real eigenvalues and eigenvectors of non-symmetric matrix.
/// 
/// ## Parameters
/// * src: input matrix that must have CV_32FC1 or CV_64FC1 type, square size and be symmetrical
/// (src ^T^ == src).
/// * eigenvalues: output vector of eigenvalues of the same type as src; the eigenvalues are stored
/// in the descending order.
/// * eigenvectors: output matrix of eigenvectors; it has the same size and type as src; the
/// eigenvectors are stored as subsequent matrix rows, in the same order as the corresponding
/// eigenvalues.
/// ## See also
/// eigenNonSymmetric, completeSymm , PCA
/// 
/// ## C++ default parameters
/// * eigenvectors: noArray()
pub fn eigen(src: &dyn core::ToInputArray, eigenvalues: &mut dyn core::ToOutputArray, eigenvectors: &mut dyn core::ToOutputArray) -> Result<bool> {
	input_array_arg!(src);
	output_array_arg!(eigenvalues);
	output_array_arg!(eigenvectors);
	unsafe { sys::cv_eigen_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), eigenvalues.as_raw__OutputArray(), eigenvectors.as_raw__OutputArray()) }.into_result()
}

/// ! Signals an error and raises the exception.
/// 
/// By default the function prints information about the error to stderr,
/// then it either stops if cv::setBreakOnError() had been called before or raises the exception.
/// It is possible to alternate error processing by using #redirectError().
/// ## Parameters
/// * exc: the exception raisen.
/// 
/// **Deprecated**: drop this version
#[deprecated = "drop this version"]
pub fn error_1(exc: &core::Exception) -> Result<()> {
	unsafe { sys::cv_error_const_ExceptionR(exc.as_raw_Exception()) }.into_result()
}

/// ! Signals an error and raises the exception.
/// 
/// By default the function prints information about the error to stderr,
/// then it either stops if setBreakOnError() had been called before or raises the exception.
/// It is possible to alternate error processing by using redirectError().
/// ## Parameters
/// * _code: - error code (Error::Code)
/// * _err: - error description
/// * _func: - function name. Available only when the compiler supports getting it
/// * _file: - source file name where the error has occurred
/// * _line: - line number in the source file where the error has occurred
/// ## See also
/// CV_Error, CV_Error_, CV_Assert, CV_DbgAssert
pub fn error(_code: i32, _err: &str, _func: &str, _file: &str, _line: i32) -> Result<()> {
	extern_container_arg!(_err);
	extern_container_arg!(_func);
	extern_container_arg!(_file);
	unsafe { sys::cv_error_int_const_StringR_const_charX_const_charX_int(_code, _err.opencv_to_extern(), _func.opencv_to_extern(), _file.opencv_to_extern(), _line) }.into_result()
}

/// Calculates the exponent of every array element.
/// 
/// The function cv::exp calculates the exponent of every element of the input
/// array:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%5BI%5D%20%3D%20e%5E%7B%20src%28I%29%20%7D)
/// 
/// The maximum relative error is about 7e-6 for single-precision input and
/// less than 1e-10 for double-precision input. Currently, the function
/// converts denormalized values to zeros on output. Special values (NaN,
/// Inf) are not handled.
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same size and type as src.
/// ## See also
/// log , cartToPolar , polarToCart , phase , pow , sqrt , magnitude
pub fn exp(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_exp_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Extracts a single channel from src (coi is 0-based index)
/// ## Parameters
/// * src: input array
/// * dst: output array
/// * coi: index of channel to extract
/// ## See also
/// mixChannels, split
pub fn extract_channel(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, coi: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_extractChannel_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), coi) }.into_result()
}

/// Calculates the angle of a 2D vector in degrees.
/// 
/// The function fastAtan2 calculates the full-range angle of an input 2D vector. The angle is measured
/// in degrees and varies from 0 to 360 degrees. The accuracy is about 0.3 degrees.
/// ## Parameters
/// * x: x-coordinate of the vector.
/// * y: y-coordinate of the vector.
pub fn fast_atan2(y: f32, x: f32) -> Result<f32> {
	unsafe { sys::cv_fastAtan2_float_float(y, x) }.into_result()
}

/// Returns the list of locations of non-zero pixels
/// 
/// Given a binary matrix (likely returned from an operation such
/// as threshold(), compare(), >, ==, etc, return all of
/// the non-zero indices as a cv::Mat or std::vector<cv::Point> (x,y)
/// For example:
/// ```ignore
///    cv::Mat binaryImage; // input, binary image
///    cv::Mat locations;   // output, locations of non-zero pixels
///    cv::findNonZero(binaryImage, locations);
/// 
///    // access pixel coordinates
///    Point pnt = locations.at<Point>(i);
/// ```
/// 
/// or
/// ```ignore
///    cv::Mat binaryImage; // input, binary image
///    vector<Point> locations;   // output, locations of non-zero pixels
///    cv::findNonZero(binaryImage, locations);
/// 
///    // access pixel coordinates
///    Point pnt = locations[i];
/// ```
/// 
/// ## Parameters
/// * src: single-channel array
/// * idx: the output array, type of cv::Mat or std::vector<Point>, corresponding to non-zero indices in the input
pub fn find_non_zero(src: &dyn core::ToInputArray, idx: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(idx);
	unsafe { sys::cv_findNonZero_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), idx.as_raw__OutputArray()) }.into_result()
}

/// Flips a 2D array around vertical, horizontal, or both axes.
/// 
/// The function cv::flip flips the array in one of three different ways (row
/// and column indices are 0-based):
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%5F%7Bij%7D%20%3D%0A%5Cleft%5C%7B%0A%5Cbegin%7Barray%7D%7Bl%20l%7D%0A%5Ctexttt%7Bsrc%7D%20%5F%7B%5Ctexttt%7Bsrc%2Erows%7D%2Di%2D1%2Cj%7D%20%26%20if%5C%3B%20%20%5Ctexttt%7BflipCode%7D%20%3D%200%20%5C%5C%0A%5Ctexttt%7Bsrc%7D%20%5F%7Bi%2C%20%5Ctexttt%7Bsrc%2Ecols%7D%20%2Dj%2D1%7D%20%26%20if%5C%3B%20%20%5Ctexttt%7BflipCode%7D%20%3E%200%20%5C%5C%0A%5Ctexttt%7Bsrc%7D%20%5F%7B%20%5Ctexttt%7Bsrc%2Erows%7D%20%2Di%2D1%2C%20%5Ctexttt%7Bsrc%2Ecols%7D%20%2Dj%2D1%7D%20%26%20if%5C%3B%20%5Ctexttt%7BflipCode%7D%20%3C%200%20%5C%5C%0A%5Cend%7Barray%7D%0A%5Cright%2E)
/// The example scenarios of using the function are the following:
/// *   Vertical flipping of the image (flipCode == 0) to switch between
///    top-left and bottom-left image origin. This is a typical operation
///    in video processing on Microsoft Windows\* OS.
/// *   Horizontal flipping of the image with the subsequent horizontal
///    shift and absolute difference calculation to check for a
///    vertical-axis symmetry (flipCode \> 0).
/// *   Simultaneous horizontal and vertical flipping of the image with
///    the subsequent shift and absolute difference calculation to check
///    for a central symmetry (flipCode \< 0).
/// *   Reversing the order of point arrays (flipCode \> 0 or
///    flipCode == 0).
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same size and type as src.
/// * flipCode: a flag to specify how to flip the array; 0 means
/// flipping around the x-axis and positive value (for example, 1) means
/// flipping around y-axis. Negative value (for example, -1) means flipping
/// around both axes.
/// ## See also
/// transpose , repeat , completeSymm
pub fn flip(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flip_code: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_flip_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flip_code) }.into_result()
}

/// Performs generalized matrix multiplication.
/// 
/// The function cv::gemm performs generalized matrix multiplication similar to the
/// gemm functions in BLAS level 3. For example,
/// `gemm(src1, src2, alpha, src3, beta, dst, GEMM_1_T + GEMM_3_T)`
/// corresponds to
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Balpha%7D%20%5Ccdot%20%5Ctexttt%7Bsrc1%7D%20%5ET%20%20%5Ccdot%20%5Ctexttt%7Bsrc2%7D%20%2B%20%20%5Ctexttt%7Bbeta%7D%20%5Ccdot%20%5Ctexttt%7Bsrc3%7D%20%5ET)
/// 
/// In case of complex (two-channel) data, performed a complex matrix
/// multiplication.
/// 
/// The function can be replaced with a matrix expression. For example, the
/// above call can be replaced with:
/// ```ignore
///    dst = alpha*src1.t()*src2 + beta*src3.t();
/// ```
/// 
/// ## Parameters
/// * src1: first multiplied input matrix that could be real(CV_32FC1,
/// CV_64FC1) or complex(CV_32FC2, CV_64FC2).
/// * src2: second multiplied input matrix of the same type as src1.
/// * alpha: weight of the matrix product.
/// * src3: third optional delta matrix added to the matrix product; it
/// should have the same type as src1 and src2.
/// * beta: weight of src3.
/// * dst: output matrix; it has the proper size and the same type as
/// input matrices.
/// * flags: operation flags (cv::GemmFlags)
/// ## See also
/// mulTransposed , transform
/// 
/// ## C++ default parameters
/// * flags: 0
pub fn gemm(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, alpha: f64, src3: &dyn core::ToInputArray, beta: f64, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	input_array_arg!(src3);
	output_array_arg!(dst);
	unsafe { sys::cv_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), alpha, src3.as_raw__InputArray(), beta, dst.as_raw__OutputArray(), flags) }.into_result()
}

/// Returns full configuration time cmake output.
/// 
/// Returned value is raw cmake output including version control system revision, compiler version,
/// compiler flags, enabled modules and third party libraries, etc. Output format depends on target
/// architecture.
pub fn get_build_information() -> Result<String> {
	unsafe { sys::cv_getBuildInformation() }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

/// Returns list of CPU features enabled during compilation.
/// 
/// Returned value is a string containing space separated list of CPU features with following markers:
/// 
/// - no markers - baseline features
/// - prefix `*` - features enabled in dispatcher
/// - suffix `?` - features enabled but not available in HW
/// 
/// Example: `SSE SSE2 SSE3 *SSE4.1 *SSE4.2 *FP16 *AVX *AVX2 *AVX512-SKX?`
pub fn get_cpu_features_line() -> Result<String> {
	unsafe { sys::cv_getCPUFeaturesLine() }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

/// Returns the number of CPU ticks.
/// 
/// The function returns the current number of CPU ticks on some architectures (such as x86, x64,
/// PowerPC). On other platforms the function is equivalent to getTickCount. It can also be used for
/// very accurate time measurements, as well as for RNG initialization. Note that in case of multi-CPU
/// systems a thread, from which getCPUTickCount is called, can be suspended and resumed at another CPU
/// with its own counter. So, theoretically (and practically) the subsequent calls to the function do
/// not necessary return the monotonously increasing values. Also, since a modern CPU varies the CPU
/// frequency depending on the load, the number of CPU clocks spent in some code cannot be directly
/// converted to time units. Therefore, getTickCount is generally a preferable solution for measuring
/// execution time.
pub fn get_cpu_tick_count() -> Result<i64> {
	unsafe { sys::cv_getCPUTickCount() }.into_result()
}

pub fn get_elem_size(typ: i32) -> Result<size_t> {
	unsafe { sys::cv_getElemSize_int(typ) }.into_result()
}

/// Returns feature name by ID
/// 
/// Returns empty string if feature is not defined
pub fn get_hardware_feature_name(feature: i32) -> Result<String> {
	unsafe { sys::cv_getHardwareFeatureName_int(feature) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

#[cfg(not(target_os = "windows"))]
pub fn get_impl(impl_: &mut core::Vector::<i32>, fun_name: &mut core::Vector::<String>) -> Result<i32> {
	unsafe { sys::cv_getImpl_vector_int_R_vector_String_R(impl_.as_raw_mut_VectorOfi32(), fun_name.as_raw_mut_VectorOfString()) }.into_result()
}

/// Returns the number of threads used by OpenCV for parallel regions.
/// 
/// Always returns 1 if OpenCV is built without threading support.
/// 
/// The exact meaning of return value depends on the threading framework used by OpenCV library:
/// - `TBB` - The number of threads, that OpenCV will try to use for parallel regions. If there is
///   any tbb::thread_scheduler_init in user code conflicting with OpenCV, then function returns
///   default number of threads used by TBB library.
/// - `OpenMP` - An upper bound on the number of threads that could be used to form a new team.
/// - `Concurrency` - The number of threads, that OpenCV will try to use for parallel regions.
/// - `GCD` - Unsupported; returns the GCD thread pool limit (512) for compatibility.
/// - `C=` - The number of threads, that OpenCV will try to use for parallel regions, if before
///   called setNumThreads with threads \> 0, otherwise returns the number of logical CPUs,
///   available for the process.
/// ## See also
/// setNumThreads, getThreadNum
pub fn get_num_threads() -> Result<i32> {
	unsafe { sys::cv_getNumThreads() }.into_result()
}

/// Returns the number of logical CPUs available for the process.
pub fn get_number_of_cpus() -> Result<i32> {
	unsafe { sys::cv_getNumberOfCPUs() }.into_result()
}

/// Returns the optimal DFT size for a given vector size.
/// 
/// DFT performance is not a monotonic function of a vector size. Therefore, when you calculate
/// convolution of two arrays or perform the spectral analysis of an array, it usually makes sense to
/// pad the input data with zeros to get a bit larger array that can be transformed much faster than the
/// original one. Arrays whose size is a power-of-two (2, 4, 8, 16, 32, ...) are the fastest to process.
/// Though, the arrays whose size is a product of 2's, 3's, and 5's (for example, 300 = 5\*5\*3\*2\*2)
/// are also processed quite efficiently.
/// 
/// The function cv::getOptimalDFTSize returns the minimum number N that is greater than or equal to vecsize
/// so that the DFT of a vector of size N can be processed efficiently. In the current implementation N
/// = 2 ^p^ \* 3 ^q^ \* 5 ^r^ for some integer p, q, r.
/// 
/// The function returns a negative number if vecsize is too large (very close to INT_MAX ).
/// 
/// While the function cannot be used directly to estimate the optimal vector size for DCT transform
/// (since the current DCT implementation supports only even-size vectors), it can be easily processed
/// as getOptimalDFTSize((vecsize+1)/2)\*2.
/// ## Parameters
/// * vecsize: vector size.
/// ## See also
/// dft , dct , idft , idct , mulSpectrums
pub fn get_optimal_dft_size(vecsize: i32) -> Result<i32> {
	unsafe { sys::cv_getOptimalDFTSize_int(vecsize) }.into_result()
}

/// Returns the index of the currently executed thread within the current parallel region. Always
/// returns 0 if called outside of parallel region.
/// 
/// 
/// **Deprecated**: Current implementation doesn't corresponding to this documentation.
/// 
/// The exact meaning of the return value depends on the threading framework used by OpenCV library:
/// - `TBB` - Unsupported with current 4.1 TBB release. Maybe will be supported in future.
/// - `OpenMP` - The thread number, within the current team, of the calling thread.
/// - `Concurrency` - An ID for the virtual processor that the current context is executing on (0
///   for master thread and unique number for others, but not necessary 1,2,3,...).
/// - `GCD` - System calling thread's ID. Never returns 0 inside parallel region.
/// - `C=` - The index of the current parallel task.
/// ## See also
/// setNumThreads, getNumThreads
#[deprecated = "Current implementation doesn't corresponding to this documentation."]
pub fn get_thread_num() -> Result<i32> {
	unsafe { sys::cv_getThreadNum() }.into_result()
}

/// Returns the number of ticks.
/// 
/// The function returns the number of ticks after the certain event (for example, when the machine was
/// turned on). It can be used to initialize RNG or to measure a function execution time by reading the
/// tick count before and after the function call.
/// ## See also
/// getTickFrequency, TickMeter
pub fn get_tick_count() -> Result<i64> {
	unsafe { sys::cv_getTickCount() }.into_result()
}

/// Returns the number of ticks per second.
/// 
/// The function returns the number of ticks per second. That is, the following code computes the
/// execution time in seconds:
/// ```ignore
///    double t = (double)getTickCount();
///    // do something ...
///    t = ((double)getTickCount() - t)/getTickFrequency();
/// ```
/// ## See also
/// getTickCount, TickMeter
pub fn get_tick_frequency() -> Result<f64> {
	unsafe { sys::cv_getTickFrequency() }.into_result()
}

/// Returns major library version
pub fn get_version_major() -> Result<i32> {
	unsafe { sys::cv_getVersionMajor() }.into_result()
}

/// Returns minor library version
pub fn get_version_minor() -> Result<i32> {
	unsafe { sys::cv_getVersionMinor() }.into_result()
}

/// Returns revision field of the library version
pub fn get_version_revision() -> Result<i32> {
	unsafe { sys::cv_getVersionRevision() }.into_result()
}

/// Returns library version string
/// 
/// For example "3.4.1-dev".
/// ## See also
/// getMajorVersion, getMinorVersion, getRevisionVersion
pub fn get_version_string() -> Result<String> {
	unsafe { sys::cv_getVersionString() }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

/// ## C++ default parameters
/// * recursive: false
pub fn glob(pattern: &str, result: &mut core::Vector::<String>, recursive: bool) -> Result<()> {
	extern_container_arg!(mut pattern);
	unsafe { sys::cv_glob_String_vector_String_R_bool(pattern.opencv_to_extern_mut(), result.as_raw_mut_VectorOfString(), recursive) }.into_result()
}

/// Check if use of OpenVX is possible
pub fn have_openvx() -> Result<bool> {
	unsafe { sys::cv_haveOpenVX() }.into_result()
}

/// Applies horizontal concatenation to given matrices.
/// 
/// The function horizontally concatenates two or more cv::Mat matrices (with the same number of rows).
/// ```ignore
///    cv::Mat matArray[] = { cv::Mat(4, 1, CV_8UC1, cv::Scalar(1)),
///                            cv::Mat(4, 1, CV_8UC1, cv::Scalar(2)),
///                            cv::Mat(4, 1, CV_8UC1, cv::Scalar(3)),};
/// 
///    cv::Mat out;
///    cv::hconcat( matArray, 3, out );
///    //out:
///    //[1, 2, 3;
///    // 1, 2, 3;
///    // 1, 2, 3;
///    // 1, 2, 3]
/// ```
/// 
/// ## Parameters
/// * src: input array or vector of matrices. all of the matrices must have the same number of rows and the same depth.
/// * nsrc: number of matrices in src.
/// * dst: output array. It has the same number of rows and depth as the src, and the sum of cols of the src.
/// ## See also
/// cv::vconcat(const Mat*, size_t, OutputArray), cv::vconcat(InputArrayOfArrays, OutputArray) and cv::vconcat(InputArray, InputArray, OutputArray)
/// 
/// ## Overloaded parameters
/// 
///  ```ignore
///    cv::Mat_<float> A = (cv::Mat_<float>(3, 2) << 1, 4,
///                                                   2, 5,
///                                                   3, 6);
///    cv::Mat_<float> B = (cv::Mat_<float>(3, 2) << 7, 10,
///                                                   8, 11,
///                                                   9, 12);
/// 
///    cv::Mat C;
///    cv::hconcat(A, B, C);
///    //C:
///    //[1, 4, 7, 10;
///    // 2, 5, 8, 11;
///    // 3, 6, 9, 12]
///  ```
/// 
/// * src1: first input array to be considered for horizontal concatenation.
/// * src2: second input array to be considered for horizontal concatenation.
/// * dst: output array. It has the same number of rows and depth as the src1 and src2, and the sum of cols of the src1 and src2.
pub fn hconcat2(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_hconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Applies horizontal concatenation to given matrices.
/// 
/// The function horizontally concatenates two or more cv::Mat matrices (with the same number of rows).
/// ```ignore
///    cv::Mat matArray[] = { cv::Mat(4, 1, CV_8UC1, cv::Scalar(1)),
///                            cv::Mat(4, 1, CV_8UC1, cv::Scalar(2)),
///                            cv::Mat(4, 1, CV_8UC1, cv::Scalar(3)),};
/// 
///    cv::Mat out;
///    cv::hconcat( matArray, 3, out );
///    //out:
///    //[1, 2, 3;
///    // 1, 2, 3;
///    // 1, 2, 3;
///    // 1, 2, 3]
/// ```
/// 
/// ## Parameters
/// * src: input array or vector of matrices. all of the matrices must have the same number of rows and the same depth.
/// * nsrc: number of matrices in src.
/// * dst: output array. It has the same number of rows and depth as the src, and the sum of cols of the src.
/// ## See also
/// cv::vconcat(const Mat*, size_t, OutputArray), cv::vconcat(InputArrayOfArrays, OutputArray) and cv::vconcat(InputArray, InputArray, OutputArray)
/// 
/// ## Overloaded parameters
/// 
///  ```ignore
///    std::vector<cv::Mat> matrices = { cv::Mat(4, 1, CV_8UC1, cv::Scalar(1)),
///                                       cv::Mat(4, 1, CV_8UC1, cv::Scalar(2)),
///                                       cv::Mat(4, 1, CV_8UC1, cv::Scalar(3)),};
/// 
///    cv::Mat out;
///    cv::hconcat( matrices, out );
///    //out:
///    //[1, 2, 3;
///    // 1, 2, 3;
///    // 1, 2, 3;
///    // 1, 2, 3]
///  ```
/// 
/// * src: input array or vector of matrices. all of the matrices must have the same number of rows and the same depth.
/// * dst: output array. It has the same number of rows and depth as the src, and the sum of cols of the src.
/// same depth.
pub fn hconcat(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_hconcat_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Calculates the inverse Discrete Cosine Transform of a 1D or 2D array.
/// 
/// idct(src, dst, flags) is equivalent to dct(src, dst, flags | DCT_INVERSE).
/// ## Parameters
/// * src: input floating-point single-channel array.
/// * dst: output array of the same size and type as src.
/// * flags: operation flags.
/// ## See also
/// dct, dft, idft, getOptimalDFTSize
/// 
/// ## C++ default parameters
/// * flags: 0
pub fn idct(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_idct_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags) }.into_result()
}

/// Calculates the inverse Discrete Fourier Transform of a 1D or 2D array.
/// 
/// idft(src, dst, flags) is equivalent to dft(src, dst, flags | #DFT_INVERSE) .
/// 
/// Note: None of dft and idft scales the result by default. So, you should pass #DFT_SCALE to one of
/// dft or idft explicitly to make these transforms mutually inverse.
/// ## See also
/// dft, dct, idct, mulSpectrums, getOptimalDFTSize
/// ## Parameters
/// * src: input floating-point real or complex array.
/// * dst: output array whose size and type depend on the flags.
/// * flags: operation flags (see dft and #DftFlags).
/// * nonzeroRows: number of dst rows to process; the rest of the rows have undefined content (see
/// the convolution sample in dft description.
/// 
/// ## C++ default parameters
/// * flags: 0
/// * nonzero_rows: 0
pub fn idft(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32, nonzero_rows: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_idft_const__InputArrayR_const__OutputArrayR_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, nonzero_rows) }.into_result()
}

///  Checks if array elements lie between the elements of two other arrays.
/// 
/// The function checks the range as follows:
/// *   For every element of a single-channel input array:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Blowerb%7D%20%28I%29%5F0%20%20%5Cleq%20%5Ctexttt%7Bsrc%7D%20%28I%29%5F0%20%5Cleq%20%20%5Ctexttt%7Bupperb%7D%20%28I%29%5F0)
/// *   For two-channel arrays:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Blowerb%7D%20%28I%29%5F0%20%20%5Cleq%20%5Ctexttt%7Bsrc%7D%20%28I%29%5F0%20%5Cleq%20%20%5Ctexttt%7Bupperb%7D%20%28I%29%5F0%20%20%5Cland%20%5Ctexttt%7Blowerb%7D%20%28I%29%5F1%20%20%5Cleq%20%5Ctexttt%7Bsrc%7D%20%28I%29%5F1%20%5Cleq%20%20%5Ctexttt%7Bupperb%7D%20%28I%29%5F1)
/// *   and so forth.
/// 
/// That is, dst (I) is set to 255 (all 1 -bits) if src (I) is within the
/// specified 1D, 2D, 3D, ... box and 0 otherwise.
/// 
/// When the lower and/or upper boundary parameters are scalars, the indexes
/// (I) at lowerb and upperb in the above formulas should be omitted.
/// ## Parameters
/// * src: first input array.
/// * lowerb: inclusive lower boundary array or a scalar.
/// * upperb: inclusive upper boundary array or a scalar.
/// * dst: output array of the same size as src and CV_8U type.
pub fn in_range(src: &dyn core::ToInputArray, lowerb: &dyn core::ToInputArray, upperb: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(lowerb);
	input_array_arg!(upperb);
	output_array_arg!(dst);
	unsafe { sys::cv_inRange_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), lowerb.as_raw__InputArray(), upperb.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Inserts a single channel to dst (coi is 0-based index)
/// ## Parameters
/// * src: input array
/// * dst: output array
/// * coi: index of channel for insertion
/// ## See also
/// mixChannels, merge
pub fn insert_channel(src: &dyn core::ToInputArray, dst: &mut dyn core::ToInputOutputArray, coi: i32) -> Result<()> {
	input_array_arg!(src);
	input_output_array_arg!(dst);
	unsafe { sys::cv_insertChannel_const__InputArrayR_const__InputOutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__InputOutputArray(), coi) }.into_result()
}

pub fn get_flags() -> Result<core::FLAGS> {
	unsafe { sys::cv_instr_getFlags() }.into_result()
}

pub fn reset_trace() -> Result<()> {
	unsafe { sys::cv_instr_resetTrace() }.into_result()
}

pub fn set_flags(mode_flags: core::FLAGS) -> Result<()> {
	unsafe { sys::cv_instr_setFlags_FLAGS(mode_flags) }.into_result()
}

pub fn set_use_instrumentation(flag: bool) -> Result<()> {
	unsafe { sys::cv_instr_setUseInstrumentation_bool(flag) }.into_result()
}

pub fn use_instrumentation() -> Result<bool> {
	unsafe { sys::cv_instr_useInstrumentation() }.into_result()
}

/// Finds the inverse or pseudo-inverse of a matrix.
/// 
/// The function cv::invert inverts the matrix src and stores the result in dst
/// . When the matrix src is singular or non-square, the function calculates
/// the pseudo-inverse matrix (the dst matrix) so that norm(src\*dst - I) is
/// minimal, where I is an identity matrix.
/// 
/// In case of the #DECOMP_LU method, the function returns non-zero value if
/// the inverse has been successfully calculated and 0 if src is singular.
/// 
/// In case of the #DECOMP_SVD method, the function returns the inverse
/// condition number of src (the ratio of the smallest singular value to the
/// largest singular value) and 0 if src is singular. The SVD method
/// calculates a pseudo-inverse matrix if src is singular.
/// 
/// Similarly to #DECOMP_LU, the method #DECOMP_CHOLESKY works only with
/// non-singular square matrices that should also be symmetrical and
/// positively defined. In this case, the function stores the inverted
/// matrix in dst and returns non-zero. Otherwise, it returns 0.
/// 
/// ## Parameters
/// * src: input floating-point M x N matrix.
/// * dst: output matrix of N x M size and the same type as src.
/// * flags: inversion method (cv::DecompTypes)
/// ## See also
/// solve, SVD
/// 
/// ## C++ default parameters
/// * flags: DECOMP_LU
pub fn invert(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<f64> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_invert_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags) }.into_result()
}

pub fn get_ipp_error_location() -> Result<String> {
	unsafe { sys::cv_ipp_getIppErrorLocation() }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn get_ipp_features() -> Result<u64> {
	unsafe { sys::cv_ipp_getIppFeatures() }.into_result()
}

pub fn get_ipp_status() -> Result<i32> {
	unsafe { sys::cv_ipp_getIppStatus() }.into_result()
}

pub fn get_ipp_version() -> Result<String> {
	unsafe { sys::cv_ipp_getIppVersion() }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

/// ## C++ default parameters
/// * funcname: NULL
/// * filename: NULL
/// * line: 0
pub fn set_ipp_status(status: i32, funcname: &str, filename: &str, line: i32) -> Result<()> {
	extern_container_arg!(funcname);
	extern_container_arg!(filename);
	unsafe { sys::cv_ipp_setIppStatus_int_const_charX_const_charX_int(status, funcname.opencv_to_extern(), filename.opencv_to_extern(), line) }.into_result()
}

pub fn set_use_ipp_not_exact(flag: bool) -> Result<()> {
	unsafe { sys::cv_ipp_setUseIPP_NotExact_bool(flag) }.into_result()
}

pub fn set_use_ipp(flag: bool) -> Result<()> {
	unsafe { sys::cv_ipp_setUseIPP_bool(flag) }.into_result()
}

pub fn use_ipp() -> Result<bool> {
	unsafe { sys::cv_ipp_useIPP() }.into_result()
}

pub fn use_ipp_not_exact() -> Result<bool> {
	unsafe { sys::cv_ipp_useIPP_NotExact() }.into_result()
}

/// Finds centers of clusters and groups input samples around the clusters.
/// 
/// The function kmeans implements a k-means algorithm that finds the centers of cluster_count clusters
/// and groups the input samples around the clusters. As an output, ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BbestLabels%7D%5Fi) contains a
/// 0-based cluster index for the sample stored in the ![inline formula](https://latex.codecogs.com/png.latex?i%5E%7Bth%7D) row of the samples matrix.
/// 
/// 
/// Note:
/// *   (Python) An example on K-means clustering can be found at
///    opencv_source_code/samples/python/kmeans.py
/// ## Parameters
/// * data: Data for clustering. An array of N-Dimensional points with float coordinates is needed.
/// Examples of this array can be:
/// *   Mat points(count, 2, CV_32F);
/// *   Mat points(count, 1, CV_32FC2);
/// *   Mat points(1, count, CV_32FC2);
/// *   std::vector\<cv::Point2f\> points(sampleCount);
/// * K: Number of clusters to split the set by.
/// * bestLabels: Input/output integer array that stores the cluster indices for every sample.
/// * criteria: The algorithm termination criteria, that is, the maximum number of iterations and/or
/// the desired accuracy. The accuracy is specified as criteria.epsilon. As soon as each of the cluster
/// centers moves by less than criteria.epsilon on some iteration, the algorithm stops.
/// * attempts: Flag to specify the number of times the algorithm is executed using different
/// initial labellings. The algorithm returns the labels that yield the best compactness (see the last
/// function parameter).
/// * flags: Flag that can take values of cv::KmeansFlags
/// * centers: Output matrix of the cluster centers, one row per each cluster center.
/// ## Returns
/// The function returns the compactness measure that is computed as
/// ![block formula](https://latex.codecogs.com/png.latex?%5Csum%20%5Fi%20%20%5C%7C%20%5Ctexttt%7Bsamples%7D%20%5Fi%20%2D%20%20%5Ctexttt%7Bcenters%7D%20%5F%7B%20%5Ctexttt%7Blabels%7D%20%5Fi%7D%20%5C%7C%20%5E2)
/// after every attempt. The best (minimum) value is chosen and the corresponding labels and the
/// compactness value are returned by the function. Basically, you can use only the core of the
/// function, set the number of attempts to 1, initialize labels each time using a custom algorithm,
/// pass them with the ( flags = #KMEANS_USE_INITIAL_LABELS ) flag, and then choose the best
/// (most-compact) clustering.
/// 
/// ## C++ default parameters
/// * centers: noArray()
pub fn kmeans(data: &dyn core::ToInputArray, k: i32, best_labels: &mut dyn core::ToInputOutputArray, criteria: core::TermCriteria, attempts: i32, flags: i32, centers: &mut dyn core::ToOutputArray) -> Result<f64> {
	input_array_arg!(data);
	input_output_array_arg!(best_labels);
	output_array_arg!(centers);
	unsafe { sys::cv_kmeans_const__InputArrayR_int_const__InputOutputArrayR_TermCriteria_int_int_const__OutputArrayR(data.as_raw__InputArray(), k, best_labels.as_raw__InputOutputArray(), criteria.opencv_to_extern(), attempts, flags, centers.as_raw__OutputArray()) }.into_result()
}

/// Calculates the natural logarithm of every array element.
/// 
/// The function cv::log calculates the natural logarithm of every element of the input array:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Clog%20%28%5Ctexttt%7Bsrc%7D%28I%29%29%20)
/// 
/// Output on zero, negative and special (NaN, Inf) values is undefined.
/// 
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same size and type as src .
/// ## See also
/// exp, cartToPolar, polarToCart, phase, pow, sqrt, magnitude
pub fn log(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_log_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Calculates the magnitude of 2D vectors.
/// 
/// The function cv::magnitude calculates the magnitude of 2D vectors formed
/// from the corresponding elements of x and y arrays:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Csqrt%7B%5Ctexttt%7Bx%7D%28I%29%5E2%20%2B%20%5Ctexttt%7By%7D%28I%29%5E2%7D)
/// ## Parameters
/// * x: floating-point array of x-coordinates of the vectors.
/// * y: floating-point array of y-coordinates of the vectors; it must
/// have the same size as x.
/// * magnitude: output array of the same size and type as x.
/// ## See also
/// cartToPolar, polarToCart, phase, sqrt
pub fn magnitude(x: &dyn core::ToInputArray, y: &dyn core::ToInputArray, magnitude: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(x);
	input_array_arg!(y);
	output_array_arg!(magnitude);
	unsafe { sys::cv_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x.as_raw__InputArray(), y.as_raw__InputArray(), magnitude.as_raw__OutputArray()) }.into_result()
}

pub fn max_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_max_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

/// Calculates per-element maximum of two arrays or an array and a scalar.
/// 
/// The function cv::max calculates the per-element maximum of two arrays:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmax%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bsrc2%7D%20%28I%29%29)
/// or array and a scalar:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmax%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bvalue%7D%20%29)
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1 .
/// * dst: output array of the same size and type as src1.
/// ## See also
/// min, compare, inRange, minMaxLoc, @ref MatrixExpressions
/// 
/// ## Overloaded parameters
/// 
/// needed to avoid conflicts with const _Tp& std::min(const _Tp&, const _Tp&, _Compare)
pub fn max_mat_to(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_max_const_MatR_const_MatR_MatR(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_mut_Mat()) }.into_result()
}

pub fn max_mat_f64(a: &core::Mat, s: f64) -> Result<core::MatExpr> {
	unsafe { sys::cv_max_const_MatR_double(a.as_raw_Mat(), s) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

/// Calculates per-element maximum of two arrays or an array and a scalar.
/// 
/// The function cv::max calculates the per-element maximum of two arrays:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmax%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bsrc2%7D%20%28I%29%29)
/// or array and a scalar:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmax%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bvalue%7D%20%29)
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1 .
/// * dst: output array of the same size and type as src1.
/// ## See also
/// min, compare, inRange, minMaxLoc, @ref MatrixExpressions
/// 
/// ## Overloaded parameters
/// 
/// needed to avoid conflicts with const _Tp& std::min(const _Tp&, const _Tp&, _Compare)
pub fn max_umat_to(src1: &core::UMat, src2: &core::UMat, dst: &mut core::UMat) -> Result<()> {
	unsafe { sys::cv_max_const_UMatR_const_UMatR_UMatR(src1.as_raw_UMat(), src2.as_raw_UMat(), dst.as_raw_mut_UMat()) }.into_result()
}

/// Calculates per-element maximum of two arrays or an array and a scalar.
/// 
/// The function cv::max calculates the per-element maximum of two arrays:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmax%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bsrc2%7D%20%28I%29%29)
/// or array and a scalar:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmax%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bvalue%7D%20%29)
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1 .
/// * dst: output array of the same size and type as src1.
/// ## See also
/// min, compare, inRange, minMaxLoc, @ref MatrixExpressions
pub fn max(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

pub fn max_f64_mat(s: f64, a: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_max_double_const_MatR(s, a.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

/// Calculates a mean and standard deviation of array elements.
/// 
/// The function cv::meanStdDev calculates the mean and the standard deviation M
/// of array elements independently for each channel and returns it via the
/// output parameters:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20N%20%3D%20%20%5Csum%20%5F%7BI%2C%20%5Ctexttt%7Bmask%7D%20%28I%29%20%20%5Cne%200%7D%201%20%5C%5C%20%5Ctexttt%7Bmean%7D%20%5Fc%20%3D%20%20%5Cfrac%7B%5Csum%5F%7B%20I%3A%20%5C%3B%20%5Ctexttt%7Bmask%7D%28I%29%20%5Cne%200%7D%20%5Ctexttt%7Bsrc%7D%20%28I%29%5Fc%7D%7BN%7D%20%5C%5C%20%5Ctexttt%7Bstddev%7D%20%5Fc%20%3D%20%20%5Csqrt%7B%5Cfrac%7B%5Csum%5F%7B%20I%3A%20%5C%3B%20%5Ctexttt%7Bmask%7D%28I%29%20%5Cne%200%7D%20%5Cleft%20%28%20%5Ctexttt%7Bsrc%7D%20%28I%29%5Fc%20%2D%20%20%5Ctexttt%7Bmean%7D%20%5Fc%20%5Cright%20%29%5E2%7D%7BN%7D%7D%20%5Cend%7Barray%7D)
/// When all the mask elements are 0's, the function returns
/// mean=stddev=Scalar::all(0).
/// 
/// Note: The calculated standard deviation is only the diagonal of the
/// complete normalized covariance matrix. If the full matrix is needed, you
/// can reshape the multi-channel array M x N to the single-channel array
/// M\*N x mtx.channels() (only possible when the matrix is continuous) and
/// then pass the matrix to calcCovarMatrix .
/// ## Parameters
/// * src: input array that should have from 1 to 4 channels so that the results can be stored in
/// Scalar_ 's.
/// * mean: output parameter: calculated mean value.
/// * stddev: output parameter: calculated standard deviation.
/// * mask: optional operation mask.
/// ## See also
/// countNonZero, mean, norm, minMaxLoc, calcCovarMatrix
/// 
/// ## C++ default parameters
/// * mask: noArray()
pub fn mean_std_dev(src: &dyn core::ToInputArray, mean: &mut dyn core::ToOutputArray, stddev: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(mean);
	output_array_arg!(stddev);
	input_array_arg!(mask);
	unsafe { sys::cv_meanStdDev_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), mean.as_raw__OutputArray(), stddev.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
}

/// Calculates an average (mean) of array elements.
/// 
/// The function cv::mean calculates the mean value M of array elements,
/// independently for each channel, and return it:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20N%20%3D%20%20%5Csum%20%5F%7BI%3A%20%5C%3B%20%5Ctexttt%7Bmask%7D%20%28I%29%20%5Cne%200%7D%201%20%5C%5C%20M%5Fc%20%3D%20%20%5Cleft%20%28%20%5Csum%20%5F%7BI%3A%20%5C%3B%20%5Ctexttt%7Bmask%7D%20%28I%29%20%5Cne%200%7D%7B%20%5Ctexttt%7Bmtx%7D%20%28I%29%5Fc%7D%20%5Cright%20%29%2FN%20%5Cend%7Barray%7D)
/// When all the mask elements are 0's, the function returns Scalar::all(0)
/// ## Parameters
/// * src: input array that should have from 1 to 4 channels so that the result can be stored in
/// Scalar_ .
/// * mask: optional operation mask.
/// ## See also
/// countNonZero, meanStdDev, norm, minMaxLoc
/// 
/// ## C++ default parameters
/// * mask: noArray()
pub fn mean(src: &dyn core::ToInputArray, mask: &dyn core::ToInputArray) -> Result<core::Scalar> {
	input_array_arg!(src);
	input_array_arg!(mask);
	unsafe { sys::cv_mean_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), mask.as_raw__InputArray()) }.into_result()
}

/// Creates one multi-channel array out of several single-channel ones.
/// 
/// The function cv::merge merges several arrays to make a single multi-channel array. That is, each
/// element of the output array will be a concatenation of the elements of the input arrays, where
/// elements of i-th input array are treated as mv[i].channels()-element vectors.
/// 
/// The function cv::split does the reverse operation. If you need to shuffle channels in some other
/// advanced way, use cv::mixChannels.
/// 
/// The following example shows how to merge 3 single channel matrices into a single 3-channel matrix.
/// [example](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_merge.cpp#L1)
/// 
/// ## Parameters
/// * mv: input array of matrices to be merged; all the matrices in mv must have the same
/// size and the same depth.
/// * count: number of input matrices when mv is a plain C array; it must be greater than zero.
/// * dst: output array of the same size and the same depth as mv[0]; The number of channels will
/// be equal to the parameter count.
/// ## See also
/// mixChannels, split, Mat::reshape
pub fn merge_slice(mv: &core::Mat, count: size_t, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(dst);
	unsafe { sys::cv_merge_const_MatX_size_t_const__OutputArrayR(mv.as_raw_Mat(), count, dst.as_raw__OutputArray()) }.into_result()
}

/// Creates one multi-channel array out of several single-channel ones.
/// 
/// The function cv::merge merges several arrays to make a single multi-channel array. That is, each
/// element of the output array will be a concatenation of the elements of the input arrays, where
/// elements of i-th input array are treated as mv[i].channels()-element vectors.
/// 
/// The function cv::split does the reverse operation. If you need to shuffle channels in some other
/// advanced way, use cv::mixChannels.
/// 
/// The following example shows how to merge 3 single channel matrices into a single 3-channel matrix.
/// [example](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_merge.cpp#L1)
/// 
/// ## Parameters
/// * mv: input array of matrices to be merged; all the matrices in mv must have the same
/// size and the same depth.
/// * count: number of input matrices when mv is a plain C array; it must be greater than zero.
/// * dst: output array of the same size and the same depth as mv[0]; The number of channels will
/// be equal to the parameter count.
/// ## See also
/// mixChannels, split, Mat::reshape
/// 
/// ## Overloaded parameters
/// 
/// * mv: input vector of matrices to be merged; all the matrices in mv must have the same
/// size and the same depth.
/// * dst: output array of the same size and the same depth as mv[0]; The number of channels will
/// be the total number of channels in the matrix array.
pub fn merge(mv: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(mv);
	output_array_arg!(dst);
	unsafe { sys::cv_merge_const__InputArrayR_const__OutputArrayR(mv.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Finds the global minimum and maximum in an array
/// 
/// The function cv::minMaxIdx finds the minimum and maximum element values and their positions. The
/// extremums are searched across the whole array or, if mask is not an empty array, in the specified
/// array region. The function does not work with multi-channel arrays. If you need to find minimum or
/// maximum elements across all the channels, use Mat::reshape first to reinterpret the array as
/// single-channel. Or you may extract the particular channel using either extractImageCOI , or
/// mixChannels , or split . In case of a sparse matrix, the minimum is found among non-zero elements
/// only.
/// 
/// Note: When minIdx is not NULL, it must have at least 2 elements (as well as maxIdx), even if src is
/// a single-row or single-column matrix. In OpenCV (following MATLAB) each array has at least 2
/// dimensions, i.e. single-column matrix is Mx1 matrix (and therefore minIdx/maxIdx will be
/// (i1,0)/(i2,0)) and single-row matrix is 1xN matrix (and therefore minIdx/maxIdx will be
/// (0,j1)/(0,j2)).
/// ## Parameters
/// * src: input single-channel array.
/// * minVal: pointer to the returned minimum value; NULL is used if not required.
/// * maxVal: pointer to the returned maximum value; NULL is used if not required.
/// * minIdx: pointer to the returned minimum location (in nD case); NULL is used if not required;
/// Otherwise, it must point to an array of src.dims elements, the coordinates of the minimum element
/// in each dimension are stored there sequentially.
/// * maxIdx: pointer to the returned maximum location (in nD case). NULL is used if not required.
/// * mask: specified array region
/// 
/// ## C++ default parameters
/// * max_val: 0
/// * min_idx: 0
/// * max_idx: 0
/// * mask: noArray()
pub fn min_max_idx(src: &dyn core::ToInputArray, min_val: &mut f64, max_val: &mut f64, min_idx: &mut i32, max_idx: &mut i32, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(mask);
	unsafe { sys::cv_minMaxIdx_const__InputArrayR_doubleX_doubleX_intX_intX_const__InputArrayR(src.as_raw__InputArray(), min_val, max_val, min_idx, max_idx, mask.as_raw__InputArray()) }.into_result()
}

/// Finds the global minimum and maximum in an array.
/// 
/// The function cv::minMaxLoc finds the minimum and maximum element values and their positions. The
/// extremums are searched across the whole array or, if mask is not an empty array, in the specified
/// array region.
/// 
/// The function do not work with multi-channel arrays. If you need to find minimum or maximum
/// elements across all the channels, use Mat::reshape first to reinterpret the array as
/// single-channel. Or you may extract the particular channel using either extractImageCOI , or
/// mixChannels , or split .
/// ## Parameters
/// * src: input single-channel array.
/// * minVal: pointer to the returned minimum value; NULL is used if not required.
/// * maxVal: pointer to the returned maximum value; NULL is used if not required.
/// * minLoc: pointer to the returned minimum location (in 2D case); NULL is used if not required.
/// * maxLoc: pointer to the returned maximum location (in 2D case); NULL is used if not required.
/// * mask: optional mask used to select a sub-array.
/// ## See also
/// max, min, compare, inRange, extractImageCOI, mixChannels, split, Mat::reshape
/// 
/// ## Overloaded parameters
/// 
/// * a: input single-channel array.
/// * minVal: pointer to the returned minimum value; NULL is used if not required.
/// * maxVal: pointer to the returned maximum value; NULL is used if not required.
/// * minIdx: pointer to the returned minimum location (in nD case); NULL is used if not required;
/// Otherwise, it must point to an array of src.dims elements, the coordinates of the minimum element
/// in each dimension are stored there sequentially.
/// * maxIdx: pointer to the returned maximum location (in nD case). NULL is used if not required.
/// 
/// ## C++ default parameters
/// * min_idx: 0
/// * max_idx: 0
pub fn min_max_loc_sparse(a: &core::SparseMat, min_val: &mut f64, max_val: &mut f64, min_idx: &mut i32, max_idx: &mut i32) -> Result<()> {
	unsafe { sys::cv_minMaxLoc_const_SparseMatR_doubleX_doubleX_intX_intX(a.as_raw_SparseMat(), min_val, max_val, min_idx, max_idx) }.into_result()
}

/// Finds the global minimum and maximum in an array.
/// 
/// The function cv::minMaxLoc finds the minimum and maximum element values and their positions. The
/// extremums are searched across the whole array or, if mask is not an empty array, in the specified
/// array region.
/// 
/// The function do not work with multi-channel arrays. If you need to find minimum or maximum
/// elements across all the channels, use Mat::reshape first to reinterpret the array as
/// single-channel. Or you may extract the particular channel using either extractImageCOI , or
/// mixChannels , or split .
/// ## Parameters
/// * src: input single-channel array.
/// * minVal: pointer to the returned minimum value; NULL is used if not required.
/// * maxVal: pointer to the returned maximum value; NULL is used if not required.
/// * minLoc: pointer to the returned minimum location (in 2D case); NULL is used if not required.
/// * maxLoc: pointer to the returned maximum location (in 2D case); NULL is used if not required.
/// * mask: optional mask used to select a sub-array.
/// ## See also
/// max, min, compare, inRange, extractImageCOI, mixChannels, split, Mat::reshape
/// 
/// ## C++ default parameters
/// * max_val: 0
/// * min_loc: 0
/// * max_loc: 0
/// * mask: noArray()
pub fn min_max_loc(src: &dyn core::ToInputArray, min_val: &mut f64, max_val: &mut f64, min_loc: &mut core::Point, max_loc: &mut core::Point, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(mask);
	unsafe { sys::cv_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX_const__InputArrayR(src.as_raw__InputArray(), min_val, max_val, min_loc, max_loc, mask.as_raw__InputArray()) }.into_result()
}

pub fn min_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_min_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

/// Calculates per-element minimum of two arrays or an array and a scalar.
/// 
/// The function cv::min calculates the per-element minimum of two arrays:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmin%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bsrc2%7D%20%28I%29%29)
/// or array and a scalar:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmin%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bvalue%7D%20%29)
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1.
/// * dst: output array of the same size and type as src1.
/// ## See also
/// max, compare, inRange, minMaxLoc
/// 
/// ## Overloaded parameters
/// 
/// needed to avoid conflicts with const _Tp& std::min(const _Tp&, const _Tp&, _Compare)
pub fn min_mat_to(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_min_const_MatR_const_MatR_MatR(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_mut_Mat()) }.into_result()
}

pub fn min_mat_f64(a: &core::Mat, s: f64) -> Result<core::MatExpr> {
	unsafe { sys::cv_min_const_MatR_double(a.as_raw_Mat(), s) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

/// Calculates per-element minimum of two arrays or an array and a scalar.
/// 
/// The function cv::min calculates the per-element minimum of two arrays:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmin%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bsrc2%7D%20%28I%29%29)
/// or array and a scalar:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmin%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bvalue%7D%20%29)
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1.
/// * dst: output array of the same size and type as src1.
/// ## See also
/// max, compare, inRange, minMaxLoc
/// 
/// ## Overloaded parameters
/// 
/// needed to avoid conflicts with const _Tp& std::min(const _Tp&, const _Tp&, _Compare)
pub fn min_umat_to(src1: &core::UMat, src2: &core::UMat, dst: &mut core::UMat) -> Result<()> {
	unsafe { sys::cv_min_const_UMatR_const_UMatR_UMatR(src1.as_raw_UMat(), src2.as_raw_UMat(), dst.as_raw_mut_UMat()) }.into_result()
}

/// Calculates per-element minimum of two arrays or an array and a scalar.
/// 
/// The function cv::min calculates the per-element minimum of two arrays:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmin%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bsrc2%7D%20%28I%29%29)
/// or array and a scalar:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmin%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bvalue%7D%20%29)
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1.
/// * dst: output array of the same size and type as src1.
/// ## See also
/// max, compare, inRange, minMaxLoc
pub fn min(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

pub fn min_f64_mat(s: f64, a: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_min_double_const_MatR(s, a.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

/// Copies specified channels from input arrays to the specified channels of
/// output arrays.
/// 
/// The function cv::mixChannels provides an advanced mechanism for shuffling image channels.
/// 
/// cv::split,cv::merge,cv::extractChannel,cv::insertChannel and some forms of cv::cvtColor are partial cases of cv::mixChannels.
/// 
/// In the example below, the code splits a 4-channel BGRA image into a 3-channel BGR (with B and R
/// channels swapped) and a separate alpha-channel image:
/// ```ignore
///    Mat bgra( 100, 100, CV_8UC4, Scalar(255,0,0,255) );
///    Mat bgr( bgra.rows, bgra.cols, CV_8UC3 );
///    Mat alpha( bgra.rows, bgra.cols, CV_8UC1 );
/// 
///    // forming an array of matrices is a quite efficient operation,
///    // because the matrix data is not copied, only the headers
///    Mat out[] = { bgr, alpha };
///    // bgra[0] -> bgr[2], bgra[1] -> bgr[1],
///    // bgra[2] -> bgr[0], bgra[3] -> alpha[0]
///    int from_to[] = { 0,2, 1,1, 2,0, 3,3 };
///    mixChannels( &bgra, 1, out, 2, from_to, 4 );
/// ```
/// 
/// 
/// Note: Unlike many other new-style C++ functions in OpenCV (see the introduction section and
/// Mat::create ), cv::mixChannels requires the output arrays to be pre-allocated before calling the
/// function.
/// ## Parameters
/// * src: input array or vector of matrices; all of the matrices must have the same size and the
/// same depth.
/// * nsrcs: number of matrices in `src`.
/// * dst: output array or vector of matrices; all the matrices **must be allocated**; their size and
/// depth must be the same as in `src[0]`.
/// * ndsts: number of matrices in `dst`.
/// * fromTo: array of index pairs specifying which channels are copied and where; fromTo[k\*2] is
/// a 0-based index of the input channel in src, fromTo[k\*2+1] is an index of the output channel in
/// dst; the continuous channel numbering is used: the first input image channels are indexed from 0 to
/// src[0].channels()-1, the second input image channels are indexed from src[0].channels() to
/// src[0].channels() + src[1].channels()-1, and so on, the same scheme is used for the output image
/// channels; as a special case, when fromTo[k\*2] is negative, the corresponding output channel is
/// filled with zero .
/// * npairs: number of index pairs in `fromTo`.
/// ## See also
/// split, merge, extractChannel, insertChannel, cvtColor
/// 
/// ## Overloaded parameters
/// 
/// * src: input array or vector of matrices; all of the matrices must have the same size and the
/// same depth.
/// * dst: output array or vector of matrices; all the matrices **must be allocated**; their size and
/// depth must be the same as in src[0].
/// * fromTo: array of index pairs specifying which channels are copied and where; fromTo[k\*2] is
/// a 0-based index of the input channel in src, fromTo[k\*2+1] is an index of the output channel in
/// dst; the continuous channel numbering is used: the first input image channels are indexed from 0 to
/// src[0].channels()-1, the second input image channels are indexed from src[0].channels() to
/// src[0].channels() + src[1].channels()-1, and so on, the same scheme is used for the output image
/// channels; as a special case, when fromTo[k\*2] is negative, the corresponding output channel is
/// filled with zero .
/// * npairs: number of index pairs in fromTo.
pub fn mix_channels(src: &dyn core::ToInputArray, dst: &mut dyn core::ToInputOutputArray, from_to: &[i32]) -> Result<()> {
	input_array_arg!(src);
	input_output_array_arg!(dst);
	unsafe { sys::cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_intX_size_t(src.as_raw__InputArray(), dst.as_raw__InputOutputArray(), from_to.as_ptr(), (from_to.len() / 2) as _) }.into_result()
}

/// Copies specified channels from input arrays to the specified channels of
/// output arrays.
/// 
/// The function cv::mixChannels provides an advanced mechanism for shuffling image channels.
/// 
/// cv::split,cv::merge,cv::extractChannel,cv::insertChannel and some forms of cv::cvtColor are partial cases of cv::mixChannels.
/// 
/// In the example below, the code splits a 4-channel BGRA image into a 3-channel BGR (with B and R
/// channels swapped) and a separate alpha-channel image:
/// ```ignore
///    Mat bgra( 100, 100, CV_8UC4, Scalar(255,0,0,255) );
///    Mat bgr( bgra.rows, bgra.cols, CV_8UC3 );
///    Mat alpha( bgra.rows, bgra.cols, CV_8UC1 );
/// 
///    // forming an array of matrices is a quite efficient operation,
///    // because the matrix data is not copied, only the headers
///    Mat out[] = { bgr, alpha };
///    // bgra[0] -> bgr[2], bgra[1] -> bgr[1],
///    // bgra[2] -> bgr[0], bgra[3] -> alpha[0]
///    int from_to[] = { 0,2, 1,1, 2,0, 3,3 };
///    mixChannels( &bgra, 1, out, 2, from_to, 4 );
/// ```
/// 
/// 
/// Note: Unlike many other new-style C++ functions in OpenCV (see the introduction section and
/// Mat::create ), cv::mixChannels requires the output arrays to be pre-allocated before calling the
/// function.
/// ## Parameters
/// * src: input array or vector of matrices; all of the matrices must have the same size and the
/// same depth.
/// * nsrcs: number of matrices in `src`.
/// * dst: output array or vector of matrices; all the matrices **must be allocated**; their size and
/// depth must be the same as in `src[0]`.
/// * ndsts: number of matrices in `dst`.
/// * fromTo: array of index pairs specifying which channels are copied and where; fromTo[k\*2] is
/// a 0-based index of the input channel in src, fromTo[k\*2+1] is an index of the output channel in
/// dst; the continuous channel numbering is used: the first input image channels are indexed from 0 to
/// src[0].channels()-1, the second input image channels are indexed from src[0].channels() to
/// src[0].channels() + src[1].channels()-1, and so on, the same scheme is used for the output image
/// channels; as a special case, when fromTo[k\*2] is negative, the corresponding output channel is
/// filled with zero .
/// * npairs: number of index pairs in `fromTo`.
/// ## See also
/// split, merge, extractChannel, insertChannel, cvtColor
/// 
/// ## Overloaded parameters
/// 
/// * src: input array or vector of matrices; all of the matrices must have the same size and the
/// same depth.
/// * dst: output array or vector of matrices; all the matrices **must be allocated**; their size and
/// depth must be the same as in src[0].
/// * fromTo: array of index pairs specifying which channels are copied and where; fromTo[k\*2] is
/// a 0-based index of the input channel in src, fromTo[k\*2+1] is an index of the output channel in
/// dst; the continuous channel numbering is used: the first input image channels are indexed from 0 to
/// src[0].channels()-1, the second input image channels are indexed from src[0].channels() to
/// src[0].channels() + src[1].channels()-1, and so on, the same scheme is used for the output image
/// channels; as a special case, when fromTo[k\*2] is negative, the corresponding output channel is
/// filled with zero .
pub fn mix_channels_vec(src: &dyn core::ToInputArray, dst: &mut dyn core::ToInputOutputArray, from_to: &core::Vector::<i32>) -> Result<()> {
	input_array_arg!(src);
	input_output_array_arg!(dst);
	unsafe { sys::cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_vector_int_R(src.as_raw__InputArray(), dst.as_raw__InputOutputArray(), from_to.as_raw_VectorOfi32()) }.into_result()
}

/// Performs the per-element multiplication of two Fourier spectrums.
/// 
/// The function cv::mulSpectrums performs the per-element multiplication of the two CCS-packed or complex
/// matrices that are results of a real or complex Fourier transform.
/// 
/// The function, together with dft and idft , may be used to calculate convolution (pass conjB=false )
/// or correlation (pass conjB=true ) of two arrays rapidly. When the arrays are complex, they are
/// simply multiplied (per element) with an optional conjugation of the second-array elements. When the
/// arrays are real, they are assumed to be CCS-packed (see dft for details).
/// ## Parameters
/// * a: first input array.
/// * b: second input array of the same size and type as src1 .
/// * c: output array of the same size and type as src1 .
/// * flags: operation flags; currently, the only supported flag is cv::DFT_ROWS, which indicates that
/// each row of src1 and src2 is an independent 1D Fourier spectrum. If you do not want to use this flag, then simply add a `0` as value.
/// * conjB: optional flag that conjugates the second input array before the multiplication (true)
/// or not (false).
/// 
/// ## C++ default parameters
/// * conj_b: false
pub fn mul_spectrums(a: &dyn core::ToInputArray, b: &dyn core::ToInputArray, c: &mut dyn core::ToOutputArray, flags: i32, conj_b: bool) -> Result<()> {
	input_array_arg!(a);
	input_array_arg!(b);
	output_array_arg!(c);
	unsafe { sys::cv_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool(a.as_raw__InputArray(), b.as_raw__InputArray(), c.as_raw__OutputArray(), flags, conj_b) }.into_result()
}

/// Calculates the product of a matrix and its transposition.
/// 
/// The function cv::mulTransposed calculates the product of src and its
/// transposition:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Ctexttt%7Bscale%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2D%20%5Ctexttt%7Bdelta%7D%20%29%5ET%20%28%20%5Ctexttt%7Bsrc%7D%20%2D%20%5Ctexttt%7Bdelta%7D%20%29)
/// if aTa=true , and
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Ctexttt%7Bscale%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2D%20%5Ctexttt%7Bdelta%7D%20%29%20%28%20%5Ctexttt%7Bsrc%7D%20%2D%20%5Ctexttt%7Bdelta%7D%20%29%5ET)
/// otherwise. The function is used to calculate the covariance matrix. With
/// zero delta, it can be used as a faster substitute for general matrix
/// product A\*B when B=A'
/// ## Parameters
/// * src: input single-channel matrix. Note that unlike gemm, the
/// function can multiply not only floating-point matrices.
/// * dst: output square matrix.
/// * aTa: Flag specifying the multiplication ordering. See the
/// description below.
/// * delta: Optional delta matrix subtracted from src before the
/// multiplication. When the matrix is empty ( delta=noArray() ), it is
/// assumed to be zero, that is, nothing is subtracted. If it has the same
/// size as src , it is simply subtracted. Otherwise, it is "repeated" (see
/// repeat ) to cover the full src and then subtracted. Type of the delta
/// matrix, when it is not empty, must be the same as the type of created
/// output matrix. See the dtype parameter description below.
/// * scale: Optional scale factor for the matrix product.
/// * dtype: Optional type of the output matrix. When it is negative,
/// the output matrix will have the same type as src . Otherwise, it will be
/// type=CV_MAT_DEPTH(dtype) that should be either CV_32F or CV_64F .
/// ## See also
/// calcCovarMatrix, gemm, repeat, reduce
/// 
/// ## C++ default parameters
/// * delta: noArray()
/// * scale: 1
/// * dtype: -1
pub fn mul_transposed(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, a_ta: bool, delta: &dyn core::ToInputArray, scale: f64, dtype: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(delta);
	unsafe { sys::cv_mulTransposed_const__InputArrayR_const__OutputArrayR_bool_const__InputArrayR_double_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), a_ta, delta.as_raw__InputArray(), scale, dtype) }.into_result()
}

/// Calculates the per-element scaled product of two arrays.
/// 
/// The function multiply calculates the per-element product of two arrays:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bscale%7D%20%5Ccdot%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Ccdot%20%5Ctexttt%7Bsrc2%7D%20%28I%29%29)
/// 
/// There is also a @ref MatrixExpressions -friendly variant of the first function. See Mat::mul .
/// 
/// For a not-per-element matrix product, see gemm .
/// 
/// 
/// Note: Saturation is not applied when the output array has the depth
/// CV_32S. You may even get result of an incorrect sign in the case of
/// overflow.
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and the same type as src1.
/// * dst: output array of the same size and type as src1.
/// * scale: optional scale factor.
/// * dtype: optional depth of the output array
/// ## See also
/// add, subtract, divide, scaleAdd, addWeighted, accumulate, accumulateProduct, accumulateSquare,
/// Mat::convertTo
/// 
/// ## C++ default parameters
/// * scale: 1
/// * dtype: -1
pub fn multiply(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, scale: f64, dtype: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), scale, dtype) }.into_result()
}

pub fn no_array() -> Result<core::_InputOutputArray> {
	unsafe { sys::cv_noArray() }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
}

/// Calculates an absolute difference norm or a relative difference norm.
/// 
/// This version of cv::norm calculates the absolute difference norm
/// or the relative difference norm of arrays src1 and src2.
/// The type of norm to calculate is specified using #NormTypes.
/// 
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and the same type as src1.
/// * normType: type of the norm (see #NormTypes).
/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
/// 
/// ## Overloaded parameters
/// 
/// * src: first input array.
/// * normType: type of the norm (see #NormTypes).
pub fn norm_sparse(src: &core::SparseMat, norm_type: i32) -> Result<f64> {
	unsafe { sys::cv_norm_const_SparseMatR_int(src.as_raw_SparseMat(), norm_type) }.into_result()
}

/// Calculates an absolute difference norm or a relative difference norm.
/// 
/// This version of cv::norm calculates the absolute difference norm
/// or the relative difference norm of arrays src1 and src2.
/// The type of norm to calculate is specified using #NormTypes.
/// 
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and the same type as src1.
/// * normType: type of the norm (see #NormTypes).
/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
/// 
/// ## C++ default parameters
/// * norm_type: NORM_L2
/// * mask: noArray()
pub fn norm2(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, norm_type: i32, mask: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	input_array_arg!(mask);
	unsafe { sys::cv_norm_const__InputArrayR_const__InputArrayR_int_const__InputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), norm_type, mask.as_raw__InputArray()) }.into_result()
}

/// Calculates the  absolute norm of an array.
/// 
/// This version of #norm calculates the absolute norm of src1. The type of norm to calculate is specified using #NormTypes.
/// 
/// As example for one array consider the function ![inline formula](https://latex.codecogs.com/png.latex?r%28x%29%3D%20%5Cbegin%7Bpmatrix%7D%20x%20%5C%5C%201%2Dx%20%5Cend%7Bpmatrix%7D%2C%20x%20%5Cin%20%5B%2D1%3B1%5D).
/// The ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F%7B1%7D%2C%20L%5F%7B2%7D%20) and ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F%7B%5Cinfty%7D%20) norm for the sample value ![inline formula](https://latex.codecogs.com/png.latex?r%28%2D1%29%20%3D%20%5Cbegin%7Bpmatrix%7D%20%2D1%20%5C%5C%202%20%5Cend%7Bpmatrix%7D)
/// is calculated as follows
/// \f{align*}
///    \| r(-1) \|_{L_1} &= |-1| + |2| = 3 \\
///    \| r(-1) \|_{L_2} &= \sqrt{(-1)^{2} + (2)^{2}} = \sqrt{5} \\
///    \| r(-1) \|_{L_\infty} &= \max(|-1|,|2|) = 2
/// \f}
/// and for ![inline formula](https://latex.codecogs.com/png.latex?r%280%2E5%29%20%3D%20%5Cbegin%7Bpmatrix%7D%200%2E5%20%5C%5C%200%2E5%20%5Cend%7Bpmatrix%7D) the calculation is
/// \f{align*}
///    \| r(0.5) \|_{L_1} &= |0.5| + |0.5| = 1 \\
///    \| r(0.5) \|_{L_2} &= \sqrt{(0.5)^{2} + (0.5)^{2}} = \sqrt{0.5} \\
///    \| r(0.5) \|_{L_\infty} &= \max(|0.5|,|0.5|) = 0.5.
/// \f}
/// The following graphic shows all values for the three norm functions ![inline formula](https://latex.codecogs.com/png.latex?%5C%7C%20r%28x%29%20%5C%7C%5F%7BL%5F1%7D%2C%20%5C%7C%20r%28x%29%20%5C%7C%5F%7BL%5F2%7D) and ![inline formula](https://latex.codecogs.com/png.latex?%5C%7C%20r%28x%29%20%5C%7C%5F%7BL%5F%5Cinfty%7D).
/// It is notable that the ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F%7B1%7D%20) norm forms the upper and the ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F%7B%5Cinfty%7D%20) norm forms the lower border for the example function ![inline formula](https://latex.codecogs.com/png.latex?%20r%28x%29%20).
/// ![Graphs for the different norm functions from the above example](https://docs.opencv.org/4.3.0/NormTypes_OneArray_1-2-INF.png)
/// 
/// When the mask parameter is specified and it is not empty, the norm is
/// 
/// If normType is not specified, #NORM_L2 is used.
/// calculated only over the region specified by the mask.
/// 
/// Multi-channel input arrays are treated as single-channel arrays, that is,
/// the results for all channels are combined.
/// 
/// Hamming norms can only be calculated with CV_8U depth arrays.
/// 
/// ## Parameters
/// * src1: first input array.
/// * normType: type of the norm (see #NormTypes).
/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
/// 
/// ## C++ default parameters
/// * norm_type: NORM_L2
/// * mask: noArray()
pub fn norm(src1: &dyn core::ToInputArray, norm_type: i32, mask: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(src1);
	input_array_arg!(mask);
	unsafe { sys::cv_norm_const__InputArrayR_int_const__InputArrayR(src1.as_raw__InputArray(), norm_type, mask.as_raw__InputArray()) }.into_result()
}

/// Normalizes the norm or value range of an array.
/// 
/// The function cv::normalize normalizes scale and shift the input array elements so that
/// ![block formula](https://latex.codecogs.com/png.latex?%5C%7C%20%5Ctexttt%7Bdst%7D%20%5C%7C%20%5F%7BL%5Fp%7D%3D%20%5Ctexttt%7Balpha%7D)
/// (where p=Inf, 1 or 2) when normType=NORM_INF, NORM_L1, or NORM_L2, respectively; or so that
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cmin%20%5FI%20%20%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Balpha%7D%20%2C%20%5C%2C%20%5C%2C%20%5Cmax%20%5FI%20%20%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bbeta%7D)
/// 
/// when normType=NORM_MINMAX (for dense arrays only). The optional mask specifies a sub-array to be
/// normalized. This means that the norm or min-n-max are calculated over the sub-array, and then this
/// sub-array is modified to be normalized. If you want to only use the mask to calculate the norm or
/// min-max but modify the whole array, you can use norm and Mat::convertTo.
/// 
/// In case of sparse matrices, only the non-zero values are analyzed and transformed. Because of this,
/// the range transformation for sparse matrices is not allowed since it can shift the zero level.
/// 
/// Possible usage with some positive example data:
/// ```ignore
///    vector<double> positiveData = { 2.0, 8.0, 10.0 };
///    vector<double> normalizedData_l1, normalizedData_l2, normalizedData_inf, normalizedData_minmax;
/// 
///    // Norm to probability (total count)
///    // sum(numbers) = 20.0
///    // 2.0      0.1     (2.0/20.0)
///    // 8.0      0.4     (8.0/20.0)
///    // 10.0     0.5     (10.0/20.0)
///    normalize(positiveData, normalizedData_l1, 1.0, 0.0, NORM_L1);
/// 
///    // Norm to unit vector: ||positiveData|| = 1.0
///    // 2.0      0.15
///    // 8.0      0.62
///    // 10.0     0.77
///    normalize(positiveData, normalizedData_l2, 1.0, 0.0, NORM_L2);
/// 
///    // Norm to max element
///    // 2.0      0.2     (2.0/10.0)
///    // 8.0      0.8     (8.0/10.0)
///    // 10.0     1.0     (10.0/10.0)
///    normalize(positiveData, normalizedData_inf, 1.0, 0.0, NORM_INF);
/// 
///    // Norm to range [0.0;1.0]
///    // 2.0      0.0     (shift to left border)
///    // 8.0      0.75    (6.0/8.0)
///    // 10.0     1.0     (shift to right border)
///    normalize(positiveData, normalizedData_minmax, 1.0, 0.0, NORM_MINMAX);
/// ```
/// 
/// 
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same size as src .
/// * alpha: norm value to normalize to or the lower range boundary in case of the range
/// normalization.
/// * beta: upper range boundary in case of the range normalization; it is not used for the norm
/// normalization.
/// * norm_type: normalization type (see cv::NormTypes).
/// * dtype: when negative, the output array has the same type as src; otherwise, it has the same
/// number of channels as src and the depth =CV_MAT_DEPTH(dtype).
/// * mask: optional operation mask.
/// ## See also
/// norm, Mat::convertTo, SparseMat::convertTo
/// 
/// ## Overloaded parameters
/// 
/// * src: input array.
/// * dst: output array of the same size as src .
/// * alpha: norm value to normalize to or the lower range boundary in case of the range
/// normalization.
/// * normType: normalization type (see cv::NormTypes).
pub fn normalize_sparse(src: &core::SparseMat, dst: &mut core::SparseMat, alpha: f64, norm_type: i32) -> Result<()> {
	unsafe { sys::cv_normalize_const_SparseMatR_SparseMatR_double_int(src.as_raw_SparseMat(), dst.as_raw_mut_SparseMat(), alpha, norm_type) }.into_result()
}

/// Normalizes the norm or value range of an array.
/// 
/// The function cv::normalize normalizes scale and shift the input array elements so that
/// ![block formula](https://latex.codecogs.com/png.latex?%5C%7C%20%5Ctexttt%7Bdst%7D%20%5C%7C%20%5F%7BL%5Fp%7D%3D%20%5Ctexttt%7Balpha%7D)
/// (where p=Inf, 1 or 2) when normType=NORM_INF, NORM_L1, or NORM_L2, respectively; or so that
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cmin%20%5FI%20%20%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Balpha%7D%20%2C%20%5C%2C%20%5C%2C%20%5Cmax%20%5FI%20%20%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bbeta%7D)
/// 
/// when normType=NORM_MINMAX (for dense arrays only). The optional mask specifies a sub-array to be
/// normalized. This means that the norm or min-n-max are calculated over the sub-array, and then this
/// sub-array is modified to be normalized. If you want to only use the mask to calculate the norm or
/// min-max but modify the whole array, you can use norm and Mat::convertTo.
/// 
/// In case of sparse matrices, only the non-zero values are analyzed and transformed. Because of this,
/// the range transformation for sparse matrices is not allowed since it can shift the zero level.
/// 
/// Possible usage with some positive example data:
/// ```ignore
///    vector<double> positiveData = { 2.0, 8.0, 10.0 };
///    vector<double> normalizedData_l1, normalizedData_l2, normalizedData_inf, normalizedData_minmax;
/// 
///    // Norm to probability (total count)
///    // sum(numbers) = 20.0
///    // 2.0      0.1     (2.0/20.0)
///    // 8.0      0.4     (8.0/20.0)
///    // 10.0     0.5     (10.0/20.0)
///    normalize(positiveData, normalizedData_l1, 1.0, 0.0, NORM_L1);
/// 
///    // Norm to unit vector: ||positiveData|| = 1.0
///    // 2.0      0.15
///    // 8.0      0.62
///    // 10.0     0.77
///    normalize(positiveData, normalizedData_l2, 1.0, 0.0, NORM_L2);
/// 
///    // Norm to max element
///    // 2.0      0.2     (2.0/10.0)
///    // 8.0      0.8     (8.0/10.0)
///    // 10.0     1.0     (10.0/10.0)
///    normalize(positiveData, normalizedData_inf, 1.0, 0.0, NORM_INF);
/// 
///    // Norm to range [0.0;1.0]
///    // 2.0      0.0     (shift to left border)
///    // 8.0      0.75    (6.0/8.0)
///    // 10.0     1.0     (shift to right border)
///    normalize(positiveData, normalizedData_minmax, 1.0, 0.0, NORM_MINMAX);
/// ```
/// 
/// 
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same size as src .
/// * alpha: norm value to normalize to or the lower range boundary in case of the range
/// normalization.
/// * beta: upper range boundary in case of the range normalization; it is not used for the norm
/// normalization.
/// * norm_type: normalization type (see cv::NormTypes).
/// * dtype: when negative, the output array has the same type as src; otherwise, it has the same
/// number of channels as src and the depth =CV_MAT_DEPTH(dtype).
/// * mask: optional operation mask.
/// ## See also
/// norm, Mat::convertTo, SparseMat::convertTo
/// 
/// ## C++ default parameters
/// * alpha: 1
/// * beta: 0
/// * norm_type: NORM_L2
/// * dtype: -1
/// * mask: noArray()
pub fn normalize(src: &dyn core::ToInputArray, dst: &mut dyn core::ToInputOutputArray, alpha: f64, beta: f64, norm_type: i32, dtype: i32, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	input_output_array_arg!(dst);
	input_array_arg!(mask);
	unsafe { sys::cv_normalize_const__InputArrayR_const__InputOutputArrayR_double_double_int_int_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__InputOutputArray(), alpha, beta, norm_type, dtype, mask.as_raw__InputArray()) }.into_result()
}

/// Attaches OpenCL context to OpenCV
/// 
/// Note:
///   OpenCV will check if available OpenCL platform has platformName name, then assign context to
///   OpenCV and call `clRetainContext` function. The deviceID device will be used as target device and
///   new command queue will be created.
/// ## Parameters
/// * platformName: name of OpenCL platform to attach, this string is used to check if platform is available to OpenCV at runtime
/// * platformID: ID of platform attached context was created for
/// * context: OpenCL context to be attached to OpenCV
/// * deviceID: ID of device, must be created from attached context
pub fn attach_context(platform_name: &str, platform_id: *mut c_void, context: *mut c_void, device_id: *mut c_void) -> Result<()> {
	extern_container_arg!(platform_name);
	unsafe { sys::cv_ocl_attachContext_const_StringR_voidX_voidX_voidX(platform_name.opencv_to_extern(), platform_id, context, device_id) }.into_result()
}

pub fn build_options_add_matrix_description(build_options: &mut String, name: &str, _m: &dyn core::ToInputArray) -> Result<()> {
	string_arg_output_send!(via build_options_via);
	extern_container_arg!(name);
	input_array_arg!(_m);
	let out = unsafe { sys::cv_ocl_buildOptionsAddMatrixDescription_StringR_const_StringR_const__InputArrayR(&mut build_options_via, name.opencv_to_extern(), _m.as_raw__InputArray()) }.into_result();
	string_arg_output_receive!(out, build_options_via => build_options);
	out
}

/// ## C++ default parameters
/// * src2: noArray()
/// * src3: noArray()
/// * src4: noArray()
/// * src5: noArray()
/// * src6: noArray()
/// * src7: noArray()
/// * src8: noArray()
/// * src9: noArray()
/// * strat: OCL_VECTOR_DEFAULT
pub fn check_optimal_vector_width(vector_widths: &i32, src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, src3: &dyn core::ToInputArray, src4: &dyn core::ToInputArray, src5: &dyn core::ToInputArray, src6: &dyn core::ToInputArray, src7: &dyn core::ToInputArray, src8: &dyn core::ToInputArray, src9: &dyn core::ToInputArray, strat: core::OclVectorStrategy) -> Result<i32> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	input_array_arg!(src3);
	input_array_arg!(src4);
	input_array_arg!(src5);
	input_array_arg!(src6);
	input_array_arg!(src7);
	input_array_arg!(src8);
	input_array_arg!(src9);
	unsafe { sys::cv_ocl_checkOptimalVectorWidth_const_intX_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(vector_widths, src1.as_raw__InputArray(), src2.as_raw__InputArray(), src3.as_raw__InputArray(), src4.as_raw__InputArray(), src5.as_raw__InputArray(), src6.as_raw__InputArray(), src7.as_raw__InputArray(), src8.as_raw__InputArray(), src9.as_raw__InputArray(), strat) }.into_result()
}

/// Convert OpenCL buffer to UMat
/// 
/// Note:
///   OpenCL buffer (cl_mem_buffer) should contain 2D image data, compatible with OpenCV. Memory
///   content is not copied from `clBuffer` to UMat. Instead, buffer handle assigned to UMat and
///   `clRetainMemObject` is called.
/// ## Parameters
/// * cl_mem_buffer: source clBuffer handle
/// * step: num of bytes in single row
/// * rows: number of rows
/// * cols: number of cols
/// * type: OpenCV type of image
/// * dst: destination UMat
pub fn convert_from_buffer(cl_mem_buffer: *mut c_void, step: size_t, rows: i32, cols: i32, typ: i32, dst: &mut core::UMat) -> Result<()> {
	unsafe { sys::cv_ocl_convertFromBuffer_voidX_size_t_int_int_int_UMatR(cl_mem_buffer, step, rows, cols, typ, dst.as_raw_mut_UMat()) }.into_result()
}

/// Convert OpenCL image2d_t to UMat
/// 
/// Note:
///   OpenCL `image2d_t` (cl_mem_image), should be compatible with OpenCV UMat formats. Memory content
///   is copied from image to UMat with `clEnqueueCopyImageToBuffer` function.
/// ## Parameters
/// * cl_mem_image: source image2d_t handle
/// * dst: destination UMat
pub fn convert_from_image(cl_mem_image: *mut c_void, dst: &mut core::UMat) -> Result<()> {
	unsafe { sys::cv_ocl_convertFromImage_voidX_UMatR(cl_mem_image, dst.as_raw_mut_UMat()) }.into_result()
}

pub fn convert_type_str(sdepth: i32, ddepth: i32, cn: i32, buf: &str) -> Result<String> {
	extern_container_arg!(mut buf);
	unsafe { sys::cv_ocl_convertTypeStr_int_int_int_charX(sdepth, ddepth, cn, buf.opencv_to_extern_mut()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn finish() -> Result<()> {
	unsafe { sys::cv_ocl_finish() }.into_result()
}

pub fn get_opencl_error_string(error_code: i32) -> Result<String> {
	unsafe { sys::cv_ocl_getOpenCLErrorString_int(error_code) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn get_platfoms_info(platform_info: &mut core::Vector::<core::PlatformInfo>) -> Result<()> {
	unsafe { sys::cv_ocl_getPlatfomsInfo_vector_PlatformInfo_R(platform_info.as_raw_mut_VectorOfPlatformInfo()) }.into_result()
}

pub fn have_amd_blas() -> Result<bool> {
	unsafe { sys::cv_ocl_haveAmdBlas() }.into_result()
}

pub fn have_amd_fft() -> Result<bool> {
	unsafe { sys::cv_ocl_haveAmdFft() }.into_result()
}

pub fn have_opencl() -> Result<bool> {
	unsafe { sys::cv_ocl_haveOpenCL() }.into_result()
}

pub fn have_svm() -> Result<bool> {
	unsafe { sys::cv_ocl_haveSVM() }.into_result()
}

/// ## C++ default parameters
/// * ddepth: -1
/// * name: NULL
pub fn kernel_to_str(_kernel: &dyn core::ToInputArray, ddepth: i32, name: &str) -> Result<String> {
	input_array_arg!(_kernel);
	extern_container_arg!(name);
	unsafe { sys::cv_ocl_kernelToStr_const__InputArrayR_int_const_charX(_kernel.as_raw__InputArray(), ddepth, name.opencv_to_extern()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn memop_type_to_str(t: i32) -> Result<String> {
	unsafe { sys::cv_ocl_memopTypeToStr_int(t) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

/// ## C++ default parameters
/// * src2: noArray()
/// * src3: noArray()
/// * src4: noArray()
/// * src5: noArray()
/// * src6: noArray()
/// * src7: noArray()
/// * src8: noArray()
/// * src9: noArray()
pub fn predict_optimal_vector_width_max(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, src3: &dyn core::ToInputArray, src4: &dyn core::ToInputArray, src5: &dyn core::ToInputArray, src6: &dyn core::ToInputArray, src7: &dyn core::ToInputArray, src8: &dyn core::ToInputArray, src9: &dyn core::ToInputArray) -> Result<i32> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	input_array_arg!(src3);
	input_array_arg!(src4);
	input_array_arg!(src5);
	input_array_arg!(src6);
	input_array_arg!(src7);
	input_array_arg!(src8);
	input_array_arg!(src9);
	unsafe { sys::cv_ocl_predictOptimalVectorWidthMax_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), src3.as_raw__InputArray(), src4.as_raw__InputArray(), src5.as_raw__InputArray(), src6.as_raw__InputArray(), src7.as_raw__InputArray(), src8.as_raw__InputArray(), src9.as_raw__InputArray()) }.into_result()
}

/// ## C++ default parameters
/// * src2: noArray()
/// * src3: noArray()
/// * src4: noArray()
/// * src5: noArray()
/// * src6: noArray()
/// * src7: noArray()
/// * src8: noArray()
/// * src9: noArray()
/// * strat: OCL_VECTOR_DEFAULT
pub fn predict_optimal_vector_width(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, src3: &dyn core::ToInputArray, src4: &dyn core::ToInputArray, src5: &dyn core::ToInputArray, src6: &dyn core::ToInputArray, src7: &dyn core::ToInputArray, src8: &dyn core::ToInputArray, src9: &dyn core::ToInputArray, strat: core::OclVectorStrategy) -> Result<i32> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	input_array_arg!(src3);
	input_array_arg!(src4);
	input_array_arg!(src5);
	input_array_arg!(src6);
	input_array_arg!(src7);
	input_array_arg!(src8);
	input_array_arg!(src9);
	unsafe { sys::cv_ocl_predictOptimalVectorWidth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(src1.as_raw__InputArray(), src2.as_raw__InputArray(), src3.as_raw__InputArray(), src4.as_raw__InputArray(), src5.as_raw__InputArray(), src6.as_raw__InputArray(), src7.as_raw__InputArray(), src8.as_raw__InputArray(), src9.as_raw__InputArray(), strat) }.into_result()
}

pub fn set_use_opencl(flag: bool) -> Result<()> {
	unsafe { sys::cv_ocl_setUseOpenCL_bool(flag) }.into_result()
}

pub fn type_to_str(t: i32) -> Result<String> {
	unsafe { sys::cv_ocl_typeToStr_int(t) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn use_opencl() -> Result<bool> {
	unsafe { sys::cv_ocl_useOpenCL() }.into_result()
}

pub fn vecop_type_to_str(t: i32) -> Result<String> {
	unsafe { sys::cv_ocl_vecopTypeToStr_int(t) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn add_matexpr_matexpr(e1: &core::MatExpr, e2: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorA_const_MatExprR_const_MatExprR(e1.as_raw_MatExpr(), e2.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn add_matexpr_mat(e: &core::MatExpr, m: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorA_const_MatExprR_const_MatR(e.as_raw_MatExpr(), m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn add_matexpr_scalar(e: &core::MatExpr, s: core::Scalar) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorA_const_MatExprR_const_ScalarR(e.as_raw_MatExpr(), &s) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn add_mat_matexpr(m: &core::Mat, e: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorA_const_MatR_const_MatExprR(m.as_raw_Mat(), e.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

/// @relates cv::MatExpr
pub fn add_mat_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorA_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn add_mat_scalar(a: &core::Mat, s: core::Scalar) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorA_const_MatR_const_ScalarR(a.as_raw_Mat(), &s) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn add_scalar_matexpr(s: core::Scalar, e: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorA_const_ScalarR_const_MatExprR(&s, e.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn add_scalar_mat(s: core::Scalar, a: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorA_const_ScalarR_const_MatR(&s, a.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn div_matexpr_matexpr(e1: &core::MatExpr, e2: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorD_const_MatExprR_const_MatExprR(e1.as_raw_MatExpr(), e2.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn div_matexpr_mat(e: &core::MatExpr, m: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorD_const_MatExprR_const_MatR(e.as_raw_MatExpr(), m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn div_matexpr_f64(e: &core::MatExpr, s: f64) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorD_const_MatExprR_double(e.as_raw_MatExpr(), s) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn div_mat_matexpr(m: &core::Mat, e: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorD_const_MatR_const_MatExprR(m.as_raw_Mat(), e.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn div_mat_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorD_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn div_mat_f64(a: &core::Mat, s: f64) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorD_const_MatR_double(a.as_raw_Mat(), s) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn div_f64_matexpr(s: f64, e: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorD_double_const_MatExprR(s, e.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn div_f64_mat(s: f64, a: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorD_double_const_MatR(s, a.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn sub_matexpr(e: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorS_const_MatExprR(e.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn sub_matexpr_matexpr(e1: &core::MatExpr, e2: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorS_const_MatExprR_const_MatExprR(e1.as_raw_MatExpr(), e2.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn sub_matexpr_mat(e: &core::MatExpr, m: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorS_const_MatExprR_const_MatR(e.as_raw_MatExpr(), m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn sub_matexpr_scalar(e: &core::MatExpr, s: core::Scalar) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorS_const_MatExprR_const_ScalarR(e.as_raw_MatExpr(), &s) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn sub_mat(m: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorS_const_MatR(m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn sub_mat_matexpr(m: &core::Mat, e: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorS_const_MatR_const_MatExprR(m.as_raw_Mat(), e.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn sub_mat_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorS_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn sub_mat_scalar(a: &core::Mat, s: core::Scalar) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorS_const_MatR_const_ScalarR(a.as_raw_Mat(), &s) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn sub_scalar_matexpr(s: core::Scalar, e: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorS_const_ScalarR_const_MatExprR(&s, e.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn sub_scalar_mat(s: core::Scalar, a: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorS_const_ScalarR_const_MatR(&s, a.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn mul_matexpr_matexpr(e1: &core::MatExpr, e2: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorX_const_MatExprR_const_MatExprR(e1.as_raw_MatExpr(), e2.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn mul_matexpr_mat(e: &core::MatExpr, m: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorX_const_MatExprR_const_MatR(e.as_raw_MatExpr(), m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn mul_matexpr_f64(e: &core::MatExpr, s: f64) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorX_const_MatExprR_double(e.as_raw_MatExpr(), s) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn mul_mat_matexpr(m: &core::Mat, e: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorX_const_MatR_const_MatExprR(m.as_raw_Mat(), e.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn mul_mat_mat(a: &core::Mat, b: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorX_const_MatR_const_MatR(a.as_raw_Mat(), b.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn mul_mat_f64(a: &core::Mat, s: f64) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorX_const_MatR_double(a.as_raw_Mat(), s) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn mul_f64_matexpr(s: f64, e: &core::MatExpr) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorX_double_const_MatExprR(s, e.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

pub fn mul_f64_mat(s: f64, a: &core::Mat) -> Result<core::MatExpr> {
	unsafe { sys::cv_operatorX_double_const_MatR(s, a.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
}

/// Parallel data processor
/// 
/// ## C++ default parameters
/// * nstripes: -1.
pub fn parallel_for_(range: &core::Range, body: &dyn core::ParallelLoopBody, nstripes: f64) -> Result<()> {
	unsafe { sys::cv_parallel_for__const_RangeR_const_ParallelLoopBodyR_double(range.as_raw_Range(), body.as_raw_ParallelLoopBody(), nstripes) }.into_result()
}

/// converts NaN's to the given number
/// 
/// ## C++ default parameters
/// * val: 0
pub fn patch_na_ns(a: &mut dyn core::ToInputOutputArray, val: f64) -> Result<()> {
	input_output_array_arg!(a);
	unsafe { sys::cv_patchNaNs_const__InputOutputArrayR_double(a.as_raw__InputOutputArray(), val) }.into_result()
}

/// Performs the perspective matrix transformation of vectors.
/// 
/// The function cv::perspectiveTransform transforms every element of src by
/// treating it as a 2D or 3D vector, in the following way:
/// ![block formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%2C%20z%29%20%20%5Crightarrow%20%28x%27%2Fw%2C%20y%27%2Fw%2C%20z%27%2Fw%29)
/// where
/// ![block formula](https://latex.codecogs.com/png.latex?%28x%27%2C%20y%27%2C%20z%27%2C%20w%27%29%20%3D%20%20%5Ctexttt%7Bmat%7D%20%5Ccdot%20%5Cbegin%7Bbmatrix%7D%20x%20%26%20y%20%26%20z%20%26%201%20%20%5Cend%7Bbmatrix%7D)
/// and
/// ![block formula](https://latex.codecogs.com/png.latex?w%20%3D%20%20%5Cleft%5C%7B%20%5Cbegin%7Barray%7D%7Bl%20l%7D%20w%27%20%26%20%5Cmbox%7Bif%20%5C%28w%27%20%5Cne%200%5C%29%7D%5C%5C%20%5Cinfty%20%26%20%5Cmbox%7Botherwise%7D%5C%5C%20%5Cend%7Barray%7D%20%5Cright%2E)
/// 
/// Here a 3D vector transformation is shown. In case of a 2D vector
/// transformation, the z component is omitted.
/// 
/// 
/// Note: The function transforms a sparse set of 2D or 3D vectors. If you
/// want to transform an image using perspective transformation, use
/// warpPerspective . If you have an inverse problem, that is, you want to
/// compute the most probable perspective transformation out of several
/// pairs of corresponding points, you can use getPerspectiveTransform or
/// findHomography .
/// ## Parameters
/// * src: input two-channel or three-channel floating-point array; each
/// element is a 2D/3D vector to be transformed.
/// * dst: output array of the same size and type as src.
/// * m: 3x3 or 4x4 floating-point transformation matrix.
/// ## See also
/// transform, warpPerspective, getPerspectiveTransform, findHomography
pub fn perspective_transform(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, m: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(m);
	unsafe { sys::cv_perspectiveTransform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray()) }.into_result()
}

/// Calculates the rotation angle of 2D vectors.
/// 
/// The function cv::phase calculates the rotation angle of each 2D vector that
/// is formed from the corresponding elements of x and y :
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bangle%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Batan2%7D%20%28%20%5Ctexttt%7By%7D%20%28I%29%2C%20%5Ctexttt%7Bx%7D%20%28I%29%29)
/// 
/// The angle estimation accuracy is about 0.3 degrees. When x(I)=y(I)=0 ,
/// the corresponding angle(I) is set to 0.
/// ## Parameters
/// * x: input floating-point array of x-coordinates of 2D vectors.
/// * y: input array of y-coordinates of 2D vectors; it must have the
/// same size and the same type as x.
/// * angle: output array of vector angles; it has the same size and
/// same type as x .
/// * angleInDegrees: when true, the function calculates the angle in
/// degrees, otherwise, they are measured in radians.
/// 
/// ## C++ default parameters
/// * angle_in_degrees: false
pub fn phase(x: &dyn core::ToInputArray, y: &dyn core::ToInputArray, angle: &mut dyn core::ToOutputArray, angle_in_degrees: bool) -> Result<()> {
	input_array_arg!(x);
	input_array_arg!(y);
	output_array_arg!(angle);
	unsafe { sys::cv_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(x.as_raw__InputArray(), y.as_raw__InputArray(), angle.as_raw__OutputArray(), angle_in_degrees) }.into_result()
}

/// Calculates x and y coordinates of 2D vectors from their magnitude and angle.
/// 
/// The function cv::polarToCart calculates the Cartesian coordinates of each 2D
/// vector represented by the corresponding elements of magnitude and angle:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctexttt%7Bx%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bmagnitude%7D%20%28I%29%20%5Ccos%20%28%20%5Ctexttt%7Bangle%7D%20%28I%29%29%20%5C%5C%20%5Ctexttt%7By%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bmagnitude%7D%20%28I%29%20%5Csin%20%28%20%5Ctexttt%7Bangle%7D%20%28I%29%29%20%5C%5C%20%5Cend%7Barray%7D)
/// 
/// The relative accuracy of the estimated coordinates is about 1e-6.
/// ## Parameters
/// * magnitude: input floating-point array of magnitudes of 2D vectors;
/// it can be an empty matrix (=Mat()), in this case, the function assumes
/// that all the magnitudes are =1; if it is not empty, it must have the
/// same size and type as angle.
/// * angle: input floating-point array of angles of 2D vectors.
/// * x: output array of x-coordinates of 2D vectors; it has the same
/// size and type as angle.
/// * y: output array of y-coordinates of 2D vectors; it has the same
/// size and type as angle.
/// * angleInDegrees: when true, the input angles are measured in
/// degrees, otherwise, they are measured in radians.
/// ## See also
/// cartToPolar, magnitude, phase, exp, log, pow, sqrt
/// 
/// ## C++ default parameters
/// * angle_in_degrees: false
pub fn polar_to_cart(magnitude: &dyn core::ToInputArray, angle: &dyn core::ToInputArray, x: &mut dyn core::ToOutputArray, y: &mut dyn core::ToOutputArray, angle_in_degrees: bool) -> Result<()> {
	input_array_arg!(magnitude);
	input_array_arg!(angle);
	output_array_arg!(x);
	output_array_arg!(y);
	unsafe { sys::cv_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(magnitude.as_raw__InputArray(), angle.as_raw__InputArray(), x.as_raw__OutputArray(), y.as_raw__OutputArray(), angle_in_degrees) }.into_result()
}

/// Raises every array element to a power.
/// 
/// The function cv::pow raises every element of the input array to power :
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Cfork%7B%5Ctexttt%7Bsrc%7D%28I%29%5E%7Bpower%7D%7D%7Bif%20%5C%28%5Ctexttt%7Bpower%7D%5C%29%20is%20integer%7D%7B%7C%5Ctexttt%7Bsrc%7D%28I%29%7C%5E%7Bpower%7D%7D%7Botherwise%7D)
/// 
/// So, for a non-integer power exponent, the absolute values of input array
/// elements are used. However, it is possible to get true values for
/// negative values using some extra operations. In the example below,
/// computing the 5th root of array src shows:
/// ```ignore
///    Mat mask = src < 0;
///    pow(src, 1./5, dst);
///    subtract(Scalar::all(0), dst, dst, mask);
/// ```
/// 
/// For some values of power, such as integer values, 0.5 and -0.5,
/// specialized faster algorithms are used.
/// 
/// Special values (NaN, Inf) are not handled.
/// ## Parameters
/// * src: input array.
/// * power: exponent of power.
/// * dst: output array of the same size and type as src.
/// ## See also
/// sqrt, exp, log, cartToPolar, polarToCart
pub fn pow(src: &dyn core::ToInputArray, power: f64, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_pow_const__InputArrayR_double_const__OutputArrayR(src.as_raw__InputArray(), power, dst.as_raw__OutputArray()) }.into_result()
}

/// Shuffles the array elements randomly.
/// 
/// The function cv::randShuffle shuffles the specified 1D array by randomly choosing pairs of elements and
/// swapping them. The number of such swap operations will be dst.rows\*dst.cols\*iterFactor .
/// ## Parameters
/// * dst: input/output numerical 1D array.
/// * iterFactor: scale factor that determines the number of random swap operations (see the details
/// below).
/// * rng: optional random number generator used for shuffling; if it is zero, theRNG () is used
/// instead.
/// ## See also
/// RNG, sort
/// 
/// ## C++ default parameters
/// * iter_factor: 1.
/// * rng: 0
pub fn rand_shuffle(dst: &mut dyn core::ToInputOutputArray, iter_factor: f64, rng: &mut core::RNG) -> Result<()> {
	input_output_array_arg!(dst);
	unsafe { sys::cv_randShuffle_const__InputOutputArrayR_double_RNGX(dst.as_raw__InputOutputArray(), iter_factor, rng.as_raw_mut_RNG()) }.into_result()
}

/// Fills the array with normally distributed random numbers.
/// 
/// The function cv::randn fills the matrix dst with normally distributed random numbers with the specified
/// mean vector and the standard deviation matrix. The generated random numbers are clipped to fit the
/// value range of the output array data type.
/// ## Parameters
/// * dst: output array of random numbers; the array must be pre-allocated and have 1 to 4 channels.
/// * mean: mean value (expectation) of the generated random numbers.
/// * stddev: standard deviation of the generated random numbers; it can be either a vector (in
/// which case a diagonal standard deviation matrix is assumed) or a square matrix.
/// ## See also
/// RNG, randu
pub fn randn(dst: &mut dyn core::ToInputOutputArray, mean: &dyn core::ToInputArray, stddev: &dyn core::ToInputArray) -> Result<()> {
	input_output_array_arg!(dst);
	input_array_arg!(mean);
	input_array_arg!(stddev);
	unsafe { sys::cv_randn_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(dst.as_raw__InputOutputArray(), mean.as_raw__InputArray(), stddev.as_raw__InputArray()) }.into_result()
}

/// Generates a single uniformly-distributed random number or an array of random numbers.
/// 
/// Non-template variant of the function fills the matrix dst with uniformly-distributed
/// random numbers from the specified range:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Blow%7D%20%5Fc%20%20%5Cleq%20%5Ctexttt%7Bdst%7D%20%28I%29%5Fc%20%3C%20%20%5Ctexttt%7Bhigh%7D%20%5Fc)
/// ## Parameters
/// * dst: output array of random numbers; the array must be pre-allocated.
/// * low: inclusive lower boundary of the generated random numbers.
/// * high: exclusive upper boundary of the generated random numbers.
/// ## See also
/// RNG, randn, theRNG
pub fn randu(dst: &mut dyn core::ToInputOutputArray, low: &dyn core::ToInputArray, high: &dyn core::ToInputArray) -> Result<()> {
	input_output_array_arg!(dst);
	input_array_arg!(low);
	input_array_arg!(high);
	unsafe { sys::cv_randu_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(dst.as_raw__InputOutputArray(), low.as_raw__InputArray(), high.as_raw__InputArray()) }.into_result()
}

pub fn read_dmatch(node: &core::FileNode, value: &mut core::DMatch, default_value: core::DMatch) -> Result<()> {
	unsafe { sys::cv_read_const_FileNodeR_DMatchR_const_DMatchR(node.as_raw_FileNode(), value, &default_value) }.into_result()
}

pub fn read_keypoint(node: &core::FileNode, value: &mut core::KeyPoint, default_value: core::KeyPoint) -> Result<()> {
	unsafe { sys::cv_read_const_FileNodeR_KeyPointR_const_KeyPointR(node.as_raw_FileNode(), value, &default_value) }.into_result()
}

/// ## C++ default parameters
/// * default_mat: Mat()
pub fn read_mat(node: &core::FileNode, mat: &mut core::Mat, default_mat: &core::Mat) -> Result<()> {
	unsafe { sys::cv_read_const_FileNodeR_MatR_const_MatR(node.as_raw_FileNode(), mat.as_raw_mut_Mat(), default_mat.as_raw_Mat()) }.into_result()
}

/// ## C++ default parameters
/// * default_mat: SparseMat()
pub fn read_sparsemat(node: &core::FileNode, mat: &mut core::SparseMat, default_mat: &core::SparseMat) -> Result<()> {
	unsafe { sys::cv_read_const_FileNodeR_SparseMatR_const_SparseMatR(node.as_raw_FileNode(), mat.as_raw_mut_SparseMat(), default_mat.as_raw_SparseMat()) }.into_result()
}

pub fn read_f64(node: &core::FileNode, value: &mut f64, default_value: f64) -> Result<()> {
	unsafe { sys::cv_read_const_FileNodeR_doubleR_double(node.as_raw_FileNode(), value, default_value) }.into_result()
}

pub fn read_f32(node: &core::FileNode, value: &mut f32, default_value: f32) -> Result<()> {
	unsafe { sys::cv_read_const_FileNodeR_floatR_float(node.as_raw_FileNode(), value, default_value) }.into_result()
}

/// @relates cv::FileNode
pub fn read_i32(node: &core::FileNode, value: &mut i32, default_value: i32) -> Result<()> {
	unsafe { sys::cv_read_const_FileNodeR_intR_int(node.as_raw_FileNode(), value, default_value) }.into_result()
}

pub fn read_str(node: &core::FileNode, value: &mut String, default_value: &str) -> Result<()> {
	string_arg_output_send!(via value_via);
	extern_container_arg!(default_value);
	let out = unsafe { sys::cv_read_const_FileNodeR_stringR_const_stringR(node.as_raw_FileNode(), &mut value_via, default_value.opencv_to_extern()) }.into_result();
	string_arg_output_receive!(out, value_via => value);
	out
}

pub fn read_dmatch_vec_legacy(node: &core::FileNode, matches: &mut core::Vector::<core::DMatch>) -> Result<()> {
	unsafe { sys::cv_read_const_FileNodeR_vector_DMatch_R(node.as_raw_FileNode(), matches.as_raw_mut_VectorOfDMatch()) }.into_result()
}

pub fn read_keypoint_vec_legacy(node: &core::FileNode, keypoints: &mut core::Vector::<core::KeyPoint>) -> Result<()> {
	unsafe { sys::cv_read_const_FileNodeR_vector_KeyPoint_R(node.as_raw_FileNode(), keypoints.as_raw_mut_VectorOfKeyPoint()) }.into_result()
}

/// Reduces a matrix to a vector.
/// 
/// The function #reduce reduces the matrix to a vector by treating the matrix rows/columns as a set of
/// 1D vectors and performing the specified operation on the vectors until a single row/column is
/// obtained. For example, the function can be used to compute horizontal and vertical projections of a
/// raster image. In case of #REDUCE_MAX and #REDUCE_MIN , the output image should have the same type as the source one.
/// In case of #REDUCE_SUM and #REDUCE_AVG , the output may have a larger element bit-depth to preserve accuracy.
/// And multi-channel arrays are also supported in these two reduction modes.
/// 
/// The following code demonstrates its usage for a single channel matrix.
/// [example](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_reduce.cpp#L1)
/// 
/// And the following code demonstrates its usage for a two-channel matrix.
/// [example2](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_reduce.cpp#L1)
/// 
/// ## Parameters
/// * src: input 2D matrix.
/// * dst: output vector. Its size and type is defined by dim and dtype parameters.
/// * dim: dimension index along which the matrix is reduced. 0 means that the matrix is reduced to
/// a single row. 1 means that the matrix is reduced to a single column.
/// * rtype: reduction operation that could be one of #ReduceTypes
/// * dtype: when negative, the output vector will have the same type as the input matrix,
/// otherwise, its type will be CV_MAKE_TYPE(CV_MAT_DEPTH(dtype), src.channels()).
/// ## See also
/// repeat
/// 
/// ## C++ default parameters
/// * dtype: -1
pub fn reduce(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, dim: i32, rtype: i32, dtype: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_reduce_const__InputArrayR_const__OutputArrayR_int_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dim, rtype, dtype) }.into_result()
}

/// Fills the output array with repeated copies of the input array.
/// 
/// The function cv::repeat duplicates the input array one or more times along each of the two axes:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%5F%7Bij%7D%3D%20%5Ctexttt%7Bsrc%7D%20%5F%7Bi%5Cmod%20src%2Erows%2C%20%5C%3B%20j%5Cmod%20src%2Ecols%20%7D)
/// The second variant of the function is more convenient to use with @ref MatrixExpressions.
/// ## Parameters
/// * src: input array to replicate.
/// * ny: Flag to specify how many times the `src` is repeated along the
/// vertical axis.
/// * nx: Flag to specify how many times the `src` is repeated along the
/// horizontal axis.
/// * dst: output array of the same type as `src`.
/// ## See also
/// cv::reduce
/// 
/// ## Overloaded parameters
/// 
/// * src: input array to replicate.
/// * ny: Flag to specify how many times the `src` is repeated along the
/// vertical axis.
/// * nx: Flag to specify how many times the `src` is repeated along the
/// horizontal axis.
pub fn repeat(src: &core::Mat, ny: i32, nx: i32) -> Result<core::Mat> {
	unsafe { sys::cv_repeat_const_MatR_int_int(src.as_raw_Mat(), ny, nx) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

/// Fills the output array with repeated copies of the input array.
/// 
/// The function cv::repeat duplicates the input array one or more times along each of the two axes:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%5F%7Bij%7D%3D%20%5Ctexttt%7Bsrc%7D%20%5F%7Bi%5Cmod%20src%2Erows%2C%20%5C%3B%20j%5Cmod%20src%2Ecols%20%7D)
/// The second variant of the function is more convenient to use with @ref MatrixExpressions.
/// ## Parameters
/// * src: input array to replicate.
/// * ny: Flag to specify how many times the `src` is repeated along the
/// vertical axis.
/// * nx: Flag to specify how many times the `src` is repeated along the
/// horizontal axis.
/// * dst: output array of the same type as `src`.
/// ## See also
/// cv::reduce
pub fn repeat_to(src: &dyn core::ToInputArray, ny: i32, nx: i32, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_repeat_const__InputArrayR_int_int_const__OutputArrayR(src.as_raw__InputArray(), ny, nx, dst.as_raw__OutputArray()) }.into_result()
}

/// Rotates a 2D array in multiples of 90 degrees.
/// The function cv::rotate rotates the array in one of three different ways:
/// *   Rotate by 90 degrees clockwise (rotateCode = ROTATE_90_CLOCKWISE).
/// *   Rotate by 180 degrees clockwise (rotateCode = ROTATE_180).
/// *   Rotate by 270 degrees clockwise (rotateCode = ROTATE_90_COUNTERCLOCKWISE).
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same type as src.  The size is the same with ROTATE_180,
/// and the rows and cols are switched for ROTATE_90_CLOCKWISE and ROTATE_90_COUNTERCLOCKWISE.
/// * rotateCode: an enum to specify how to rotate the array; see the enum #RotateFlags
/// ## See also
/// transpose , repeat , completeSymm, flip, RotateFlags
pub fn rotate(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, rotate_code: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_rotate_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), rotate_code) }.into_result()
}

/// Override search data path by adding new search location
/// 
/// Use this only to override default behavior
/// Passed paths are used in LIFO order.
/// 
/// ## Parameters
/// * path: Path to used samples data
pub fn add_samples_data_search_path(path: &str) -> Result<()> {
	extern_container_arg!(path);
	unsafe { sys::cv_samples_addSamplesDataSearchPath_const_StringR(path.opencv_to_extern()) }.into_result()
}

/// Append samples search data sub directory
/// 
/// General usage is to add OpenCV modules name (`<opencv_contrib>/modules/<name>/samples/data` -> `<name>/samples/data` + `modules/<name>/samples/data`).
/// Passed subdirectories are used in LIFO order.
/// 
/// ## Parameters
/// * subdir: samples data sub directory
pub fn add_samples_data_search_sub_directory(subdir: &str) -> Result<()> {
	extern_container_arg!(subdir);
	unsafe { sys::cv_samples_addSamplesDataSearchSubDirectory_const_StringR(subdir.opencv_to_extern()) }.into_result()
}

/// ## C++ default parameters
/// * silent_mode: false
pub fn find_file_or_keep(relative_path: &str, silent_mode: bool) -> Result<String> {
	extern_container_arg!(relative_path);
	unsafe { sys::cv_samples_findFileOrKeep_const_StringR_bool(relative_path.opencv_to_extern(), silent_mode) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

/// Try to find requested data file
/// 
/// Search directories:
/// 
/// 1. Directories passed via `addSamplesDataSearchPath()`
/// 2. OPENCV_SAMPLES_DATA_PATH_HINT environment variable
/// 3. OPENCV_SAMPLES_DATA_PATH environment variable
///    If parameter value is not empty and nothing is found then stop searching.
/// 4. Detects build/install path based on:
///    a. current working directory (CWD)
///    b. and/or binary module location (opencv_core/opencv_world, doesn't work with static linkage)
/// 5. Scan `<source>/{,data,samples/data}` directories if build directory is detected or the current directory is in source tree.
/// 6. Scan `<install>/share/OpenCV` directory if install directory is detected.
/// ## See also
/// cv::utils::findDataFile
/// 
/// ## Parameters
/// * relative_path: Relative path to data file
/// * required: Specify "file not found" handling.
///        If true, function prints information message and raises cv::Exception.
///        If false, function returns empty result
/// * silentMode: Disables messages
/// ## Returns
/// Returns path (absolute or relative to the current directory) or empty string if file is not found
/// 
/// ## C++ default parameters
/// * required: true
/// * silent_mode: false
pub fn find_file(relative_path: &str, required: bool, silent_mode: bool) -> Result<String> {
	extern_container_arg!(relative_path);
	unsafe { sys::cv_samples_findFile_const_StringR_bool_bool(relative_path.opencv_to_extern(), required, silent_mode) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

/// Calculates the sum of a scaled array and another array.
/// 
/// The function scaleAdd is one of the classical primitive linear algebra operations, known as DAXPY
/// or SAXPY in [BLAS](http://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms). It calculates
/// the sum of a scaled array and another array:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bscale%7D%20%5Ccdot%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%2B%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
/// The function can also be emulated with a matrix expression, for example:
/// ```ignore
///    Mat A(3, 3, CV_64F);
///    ...
///    A.row(0) = A.row(1)*2 + A.row(2);
/// ```
/// 
/// ## Parameters
/// * src1: first input array.
/// * alpha: scale factor for the first array.
/// * src2: second input array of the same size and type as src1.
/// * dst: output array of the same size and type as src1.
/// ## See also
/// add, addWeighted, subtract, Mat::dot, Mat::convertTo
pub fn scale_add(src1: &dyn core::ToInputArray, alpha: f64, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_scaleAdd_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), alpha, src2.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Sets/resets the break-on-error mode.
/// 
/// When the break-on-error mode is set, the default error handler issues a hardware exception, which
/// can make debugging more convenient.
/// 
/// \return the previous state
pub fn set_break_on_error(flag: bool) -> Result<bool> {
	unsafe { sys::cv_setBreakOnError_bool(flag) }.into_result()
}

/// Initializes a scaled identity matrix.
/// 
/// The function cv::setIdentity initializes a scaled identity matrix:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bmtx%7D%20%28i%2Cj%29%3D%20%5Cfork%7B%5Ctexttt%7Bvalue%7D%7D%7B%20if%20%5C%28i%3Dj%5C%29%7D%7B0%7D%7Botherwise%7D)
/// 
/// The function can also be emulated using the matrix initializers and the
/// matrix expressions:
/// ```ignore
///    Mat A = Mat::eye(4, 3, CV_32F)*5;
///    // A will be set to [[5, 0, 0], [0, 5, 0], [0, 0, 5], [0, 0, 0]]
/// ```
/// 
/// ## Parameters
/// * mtx: matrix to initialize (not necessarily square).
/// * s: value to assign to diagonal elements.
/// ## See also
/// Mat::zeros, Mat::ones, Mat::setTo, Mat::operator=
/// 
/// ## C++ default parameters
/// * s: Scalar(1)
pub fn set_identity(mtx: &mut dyn core::ToInputOutputArray, s: core::Scalar) -> Result<()> {
	input_output_array_arg!(mtx);
	unsafe { sys::cv_setIdentity_const__InputOutputArrayR_const_ScalarR(mtx.as_raw__InputOutputArray(), &s) }.into_result()
}

/// OpenCV will try to set the number of threads for the next parallel region.
/// 
/// If threads == 0, OpenCV will disable threading optimizations and run all it's functions
/// sequentially. Passing threads \< 0 will reset threads number to system default. This function must
/// be called outside of parallel region.
/// 
/// OpenCV will try to run its functions with specified threads number, but some behaviour differs from
/// framework:
/// *   `TBB` - User-defined parallel constructions will run with the same threads number, if
///    another is not specified. If later on user creates his own scheduler, OpenCV will use it.
/// *   `OpenMP` - No special defined behaviour.
/// *   `Concurrency` - If threads == 1, OpenCV will disable threading optimizations and run its
///    functions sequentially.
/// *   `GCD` - Supports only values \<= 0.
/// *   `C=` - No special defined behaviour.
/// ## Parameters
/// * nthreads: Number of threads used by OpenCV.
/// ## See also
/// getNumThreads, getThreadNum
pub fn set_num_threads(nthreads: i32) -> Result<()> {
	unsafe { sys::cv_setNumThreads_int(nthreads) }.into_result()
}

/// Sets state of default random number generator.
/// 
/// The function cv::setRNGSeed sets state of default random number generator to custom value.
/// ## Parameters
/// * seed: new state for default random number generator
/// ## See also
/// RNG, randu, randn
pub fn set_rng_seed(seed: i32) -> Result<()> {
	unsafe { sys::cv_setRNGSeed_int(seed) }.into_result()
}

/// Enable/disable use of OpenVX
pub fn set_use_openvx(flag: bool) -> Result<()> {
	unsafe { sys::cv_setUseOpenVX_bool(flag) }.into_result()
}

/// Enables or disables the optimized code.
/// 
/// The function can be used to dynamically turn on and off optimized dispatched code (code that uses SSE4.2, AVX/AVX2,
/// and other instructions on the platforms that support it). It sets a global flag that is further
/// checked by OpenCV functions. Since the flag is not checked in the inner OpenCV loops, it is only
/// safe to call the function on the very top level in your application where you can be sure that no
/// other OpenCV function is currently executed.
/// 
/// By default, the optimized code is enabled unless you disable it in CMake. The current status can be
/// retrieved using useOptimized.
/// ## Parameters
/// * onoff: The boolean flag specifying whether the optimized code should be used (onoff=true)
/// or not (onoff=false).
pub fn set_use_optimized(onoff: bool) -> Result<()> {
	unsafe { sys::cv_setUseOptimized_bool(onoff) }.into_result()
}

/// Finds the real roots of a cubic equation.
/// 
/// The function solveCubic finds the real roots of a cubic equation:
/// *   if coeffs is a 4-element vector:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bcoeffs%7D%20%5B0%5D%20x%5E3%20%2B%20%20%5Ctexttt%7Bcoeffs%7D%20%5B1%5D%20x%5E2%20%2B%20%20%5Ctexttt%7Bcoeffs%7D%20%5B2%5D%20x%20%2B%20%20%5Ctexttt%7Bcoeffs%7D%20%5B3%5D%20%3D%200)
/// *   if coeffs is a 3-element vector:
/// ![block formula](https://latex.codecogs.com/png.latex?x%5E3%20%2B%20%20%5Ctexttt%7Bcoeffs%7D%20%5B0%5D%20x%5E2%20%2B%20%20%5Ctexttt%7Bcoeffs%7D%20%5B1%5D%20x%20%2B%20%20%5Ctexttt%7Bcoeffs%7D%20%5B2%5D%20%3D%200)
/// 
/// The roots are stored in the roots array.
/// ## Parameters
/// * coeffs: equation coefficients, an array of 3 or 4 elements.
/// * roots: output array of real roots that has 1 or 3 elements.
/// ## Returns
/// number of real roots. It can be 0, 1 or 2.
pub fn solve_cubic(coeffs: &dyn core::ToInputArray, roots: &mut dyn core::ToOutputArray) -> Result<i32> {
	input_array_arg!(coeffs);
	output_array_arg!(roots);
	unsafe { sys::cv_solveCubic_const__InputArrayR_const__OutputArrayR(coeffs.as_raw__InputArray(), roots.as_raw__OutputArray()) }.into_result()
}

/// Solve given (non-integer) linear programming problem using the Simplex Algorithm (Simplex Method).
/// 
/// What we mean here by "linear programming problem" (or LP problem, for short) can be formulated as:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cmbox%7BMaximize%20%7D%20c%5Ccdot%20x%5C%5C%0A%20%5Cmbox%7BSubject%20to%3A%7D%5C%5C%0A%20Ax%5Cleq%20b%5C%5C%0A%20x%5Cgeq%200)
/// 
/// Where ![inline formula](https://latex.codecogs.com/png.latex?c) is fixed `1`-by-`n` row-vector, ![inline formula](https://latex.codecogs.com/png.latex?A) is fixed `m`-by-`n` matrix, ![inline formula](https://latex.codecogs.com/png.latex?b) is fixed `m`-by-`1`
/// column vector and ![inline formula](https://latex.codecogs.com/png.latex?x) is an arbitrary `n`-by-`1` column vector, which satisfies the constraints.
/// 
/// Simplex algorithm is one of many algorithms that are designed to handle this sort of problems
/// efficiently. Although it is not optimal in theoretical sense (there exist algorithms that can solve
/// any problem written as above in polynomial time, while simplex method degenerates to exponential
/// time for some special cases), it is well-studied, easy to implement and is shown to work well for
/// real-life purposes.
/// 
/// The particular implementation is taken almost verbatim from **Introduction to Algorithms, third
/// edition** by T. H. Cormen, C. E. Leiserson, R. L. Rivest and Clifford Stein. In particular, the
/// Bland's rule <http://en.wikipedia.org/wiki/Bland%27s_rule> is used to prevent cycling.
/// 
/// ## Parameters
/// * Func: This row-vector corresponds to ![inline formula](https://latex.codecogs.com/png.latex?c) in the LP problem formulation (see above). It should
/// contain 32- or 64-bit floating point numbers. As a convenience, column-vector may be also submitted,
/// in the latter case it is understood to correspond to ![inline formula](https://latex.codecogs.com/png.latex?c%5ET).
/// * Constr: `m`-by-`n+1` matrix, whose rightmost column corresponds to ![inline formula](https://latex.codecogs.com/png.latex?b) in formulation above
/// and the remaining to ![inline formula](https://latex.codecogs.com/png.latex?A). It should contain 32- or 64-bit floating point numbers.
/// * z: The solution will be returned here as a column-vector - it corresponds to ![inline formula](https://latex.codecogs.com/png.latex?c) in the
/// formulation above. It will contain 64-bit floating point numbers.
/// ## Returns
/// One of cv::SolveLPResult
pub fn solve_lp(func: &dyn core::ToInputArray, constr: &dyn core::ToInputArray, z: &mut dyn core::ToOutputArray) -> Result<i32> {
	input_array_arg!(func);
	input_array_arg!(constr);
	output_array_arg!(z);
	unsafe { sys::cv_solveLP_const__InputArrayR_const__InputArrayR_const__OutputArrayR(func.as_raw__InputArray(), constr.as_raw__InputArray(), z.as_raw__OutputArray()) }.into_result()
}

/// Finds the real or complex roots of a polynomial equation.
/// 
/// The function cv::solvePoly finds real and complex roots of a polynomial equation:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bcoeffs%7D%20%5Bn%5D%20x%5E%7Bn%7D%20%2B%20%20%5Ctexttt%7Bcoeffs%7D%20%5Bn%2D1%5D%20x%5E%7Bn%2D1%7D%20%2B%20%2E%2E%2E%20%2B%20%20%5Ctexttt%7Bcoeffs%7D%20%5B1%5D%20x%20%2B%20%20%5Ctexttt%7Bcoeffs%7D%20%5B0%5D%20%3D%200)
/// ## Parameters
/// * coeffs: array of polynomial coefficients.
/// * roots: output (complex) array of roots.
/// * maxIters: maximum number of iterations the algorithm does.
/// 
/// ## C++ default parameters
/// * max_iters: 300
pub fn solve_poly(coeffs: &dyn core::ToInputArray, roots: &mut dyn core::ToOutputArray, max_iters: i32) -> Result<f64> {
	input_array_arg!(coeffs);
	output_array_arg!(roots);
	unsafe { sys::cv_solvePoly_const__InputArrayR_const__OutputArrayR_int(coeffs.as_raw__InputArray(), roots.as_raw__OutputArray(), max_iters) }.into_result()
}

/// Solves one or more linear systems or least-squares problems.
/// 
/// The function cv::solve solves a linear system or least-squares problem (the
/// latter is possible with SVD or QR methods, or by specifying the flag
/// #DECOMP_NORMAL ):
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Carg%20%5Cmin%20%5FX%20%5C%7C%20%5Ctexttt%7Bsrc1%7D%20%5Ccdot%20%5Ctexttt%7BX%7D%20%2D%20%20%5Ctexttt%7Bsrc2%7D%20%5C%7C)
/// 
/// If #DECOMP_LU or #DECOMP_CHOLESKY method is used, the function returns 1
/// if src1 (or ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsrc1%7D%5ET%5Ctexttt%7Bsrc1%7D) ) is non-singular. Otherwise,
/// it returns 0. In the latter case, dst is not valid. Other methods find a
/// pseudo-solution in case of a singular left-hand side part.
/// 
/// 
/// Note: If you want to find a unity-norm solution of an under-defined
/// singular system ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsrc1%7D%5Ccdot%5Ctexttt%7Bdst%7D%3D0) , the function solve
/// will not do the work. Use SVD::solveZ instead.
/// 
/// ## Parameters
/// * src1: input matrix on the left-hand side of the system.
/// * src2: input matrix on the right-hand side of the system.
/// * dst: output solution.
/// * flags: solution (matrix inversion) method (#DecompTypes)
/// ## See also
/// invert, SVD, eigen
/// 
/// ## C++ default parameters
/// * flags: DECOMP_LU
pub fn solve(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<bool> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_solve_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), flags) }.into_result()
}

/// Sorts each row or each column of a matrix.
/// 
/// The function cv::sortIdx sorts each matrix row or each matrix column in the
/// ascending or descending order. So you should pass two operation flags to
/// get desired behaviour. Instead of reordering the elements themselves, it
/// stores the indices of sorted elements in the output array. For example:
/// ```ignore
///    Mat A = Mat::eye(3,3,CV_32F), B;
///    sortIdx(A, B, SORT_EVERY_ROW + SORT_ASCENDING);
///    // B will probably contain
///    // (because of equal elements in A some permutations are possible):
///    // [[1, 2, 0], [0, 2, 1], [0, 1, 2]]
/// ```
/// 
/// ## Parameters
/// * src: input single-channel array.
/// * dst: output integer array of the same size as src.
/// * flags: operation flags that could be a combination of cv::SortFlags
/// ## See also
/// sort, randShuffle
pub fn sort_idx(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_sortIdx_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags) }.into_result()
}

/// Sorts each row or each column of a matrix.
/// 
/// The function cv::sort sorts each matrix row or each matrix column in
/// ascending or descending order. So you should pass two operation flags to
/// get desired behaviour. If you want to sort matrix rows or columns
/// lexicographically, you can use STL std::sort generic function with the
/// proper comparison predicate.
/// 
/// ## Parameters
/// * src: input single-channel array.
/// * dst: output array of the same size and type as src.
/// * flags: operation flags, a combination of #SortFlags
/// ## See also
/// sortIdx, randShuffle
pub fn sort(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_sort_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags) }.into_result()
}

/// Divides a multi-channel array into several single-channel arrays.
/// 
/// The function cv::split splits a multi-channel array into separate single-channel arrays:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bmv%7D%20%5Bc%5D%28I%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28I%29%5Fc)
/// If you need to extract a single channel or do some other sophisticated channel permutation, use
/// mixChannels .
/// 
/// The following example demonstrates how to split a 3-channel matrix into 3 single channel matrices.
/// [example](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_split.cpp#L1)
/// 
/// ## Parameters
/// * src: input multi-channel array.
/// * mvbegin: output array; the number of arrays must match src.channels(); the arrays themselves are
/// reallocated, if needed.
/// ## See also
/// merge, mixChannels, cvtColor
pub fn split_slice(src: &core::Mat, mvbegin: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_split_const_MatR_MatX(src.as_raw_Mat(), mvbegin.as_raw_mut_Mat()) }.into_result()
}

/// Divides a multi-channel array into several single-channel arrays.
/// 
/// The function cv::split splits a multi-channel array into separate single-channel arrays:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bmv%7D%20%5Bc%5D%28I%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28I%29%5Fc)
/// If you need to extract a single channel or do some other sophisticated channel permutation, use
/// mixChannels .
/// 
/// The following example demonstrates how to split a 3-channel matrix into 3 single channel matrices.
/// [example](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_split.cpp#L1)
/// 
/// ## Parameters
/// * src: input multi-channel array.
/// * mvbegin: output array; the number of arrays must match src.channels(); the arrays themselves are
/// reallocated, if needed.
/// ## See also
/// merge, mixChannels, cvtColor
/// 
/// ## Overloaded parameters
/// 
/// * m: input multi-channel array.
/// * mv: output vector of arrays; the arrays themselves are reallocated, if needed.
pub fn split(m: &dyn core::ToInputArray, mv: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(m);
	output_array_arg!(mv);
	unsafe { sys::cv_split_const__InputArrayR_const__OutputArrayR(m.as_raw__InputArray(), mv.as_raw__OutputArray()) }.into_result()
}

/// Calculates a square root of array elements.
/// 
/// The function cv::sqrt calculates a square root of each input array element.
/// In case of multi-channel arrays, each channel is processed
/// independently. The accuracy is approximately the same as of the built-in
/// std::sqrt .
/// ## Parameters
/// * src: input floating-point array.
/// * dst: output array of the same size and type as src.
pub fn sqrt(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_sqrt_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Calculates the per-element difference between two arrays or array and a scalar.
/// 
/// The function subtract calculates:
/// - Difference between two arrays, when both input arrays have the same size and the same number of
/// channels:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%20%5Ctexttt%7Bsrc2%7D%28I%29%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%28I%29%20%5Cne0)
/// - Difference between an array and a scalar, when src2 is constructed from Scalar or has the same
/// number of elements as `src1.channels()`:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%20%5Ctexttt%7Bsrc2%7D%20%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%28I%29%20%5Cne0)
/// - Difference between a scalar and an array, when src1 is constructed from Scalar or has the same
/// number of elements as `src2.channels()`:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%20%2D%20%20%5Ctexttt%7Bsrc2%7D%28I%29%20%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%28I%29%20%5Cne0)
/// - The reverse difference between a scalar and an array in the case of `SubRS`:
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc2%7D%20%2D%20%20%5Ctexttt%7Bsrc1%7D%28I%29%20%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%28I%29%20%5Cne0)
/// where I is a multi-dimensional index of array elements. In case of multi-channel arrays, each
/// channel is processed independently.
/// 
/// The first function in the list above can be replaced with matrix expressions:
/// ```ignore
///    dst = src1 - src2;
///    dst -= src1; // equivalent to subtract(dst, src1, dst);
/// ```
/// 
/// The input arrays and the output array can all have the same or different depths. For example, you
/// can subtract to 8-bit unsigned arrays and store the difference in a 16-bit signed array. Depth of
/// the output array is determined by dtype parameter. In the second and third cases above, as well as
/// in the first case, when src1.depth() == src2.depth(), dtype can be set to the default -1. In this
/// case the output array will have the same depth as the input array, be it src1, src2 or both.
/// 
/// Note: Saturation is not applied when the output array has the depth CV_32S. You may even get
/// result of an incorrect sign in the case of overflow.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array of the same size and the same number of channels as the input array.
/// * mask: optional operation mask; this is an 8-bit single channel array that specifies elements
/// of the output array to be changed.
/// * dtype: optional depth of the output array
/// ## See also
/// add, addWeighted, scaleAdd, Mat::convertTo
/// 
/// ## C++ default parameters
/// * mask: noArray()
/// * dtype: -1
pub fn subtract(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray, dtype: i32) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	input_array_arg!(mask);
	unsafe { sys::cv_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), dtype) }.into_result()
}

/// Calculates the sum of array elements.
/// 
/// The function cv::sum calculates and returns the sum of array elements,
/// independently for each channel.
/// ## Parameters
/// * src: input array that must have from 1 to 4 channels.
/// ## See also
/// countNonZero, mean, meanStdDev, norm, minMaxLoc, reduce
pub fn sum_elems(src: &dyn core::ToInputArray) -> Result<core::Scalar> {
	input_array_arg!(src);
	unsafe { sys::cv_sum_const__InputArrayR(src.as_raw__InputArray()) }.into_result()
}

/// Swaps two matrices
pub fn swap(a: &mut core::Mat, b: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_swap_MatR_MatR(a.as_raw_mut_Mat(), b.as_raw_mut_Mat()) }.into_result()
}

/// Swaps two matrices
/// 
/// ## Overloaded parameters
pub fn swap_umat(a: &mut core::UMat, b: &mut core::UMat) -> Result<()> {
	unsafe { sys::cv_swap_UMatR_UMatR(a.as_raw_mut_UMat(), b.as_raw_mut_UMat()) }.into_result()
}

/// ## C++ default parameters
/// * suffix: 0
pub fn tempfile(suffix: &str) -> Result<String> {
	extern_container_arg!(suffix);
	unsafe { sys::cv_tempfile_const_charX(suffix.opencv_to_extern()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

/// Returns the default random number generator.
/// 
/// The function cv::theRNG returns the default random number generator. For each thread, there is a
/// separate random number generator, so you can use the function safely in multi-thread environments.
/// If you just need to get a single random number using this generator or initialize an array, you can
/// use randu or randn instead. But if you are going to generate many random numbers inside a loop, it
/// is much faster to use this function to retrieve the generator and then use RNG::operator _Tp() .
/// ## See also
/// RNG, randu, randn
pub fn the_rng() -> Result<core::RNG> {
	unsafe { sys::cv_theRNG() }.into_result().map(|r| unsafe { core::RNG::opencv_from_extern(r) } )
}

/// Returns the trace of a matrix.
/// 
/// The function cv::trace returns the sum of the diagonal elements of the
/// matrix mtx .
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cmathrm%7Btr%7D%20%28%20%5Ctexttt%7Bmtx%7D%20%29%20%3D%20%20%5Csum%20%5Fi%20%20%5Ctexttt%7Bmtx%7D%20%28i%2Ci%29)
/// ## Parameters
/// * mtx: input matrix.
pub fn trace(mtx: &dyn core::ToInputArray) -> Result<core::Scalar> {
	input_array_arg!(mtx);
	unsafe { sys::cv_trace_const__InputArrayR(mtx.as_raw__InputArray()) }.into_result()
}

/// Performs the matrix transformation of every array element.
/// 
/// The function cv::transform performs the matrix transformation of every
/// element of the array src and stores the results in dst :
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bm%7D%20%5Ccdot%20%5Ctexttt%7Bsrc%7D%20%28I%29)
/// (when m.cols=src.channels() ), or
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bm%7D%20%5Ccdot%20%5B%20%5Ctexttt%7Bsrc%7D%20%28I%29%3B%201%5D)
/// (when m.cols=src.channels()+1 )
/// 
/// Every element of the N -channel array src is interpreted as N -element
/// vector that is transformed using the M x N or M x (N+1) matrix m to
/// M-element vector - the corresponding element of the output array dst .
/// 
/// The function may be used for geometrical transformation of
/// N -dimensional points, arbitrary linear color space transformation (such
/// as various kinds of RGB to YUV transforms), shuffling the image
/// channels, and so forth.
/// ## Parameters
/// * src: input array that must have as many channels (1 to 4) as
/// m.cols or m.cols-1.
/// * dst: output array of the same size and depth as src; it has as
/// many channels as m.rows.
/// * m: transformation 2x2 or 2x3 floating-point matrix.
/// ## See also
/// perspectiveTransform, getAffineTransform, estimateAffine2D, warpAffine, warpPerspective
pub fn transform(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, m: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(m);
	unsafe { sys::cv_transform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), m.as_raw__InputArray()) }.into_result()
}

/// Transposes a matrix.
/// 
/// The function cv::transpose transposes the matrix src :
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28i%2Cj%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28j%2Ci%29)
/// 
/// Note: No complex conjugation is done in case of a complex matrix. It
/// should be done separately if needed.
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same type as src.
pub fn transpose(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_transpose_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Returns string of cv::Mat depth value: CV_8UC3 -> "CV_8UC3" or "<invalid type>"
pub fn type_to_string(typ: i32) -> Result<String> {
	unsafe { sys::cv_typeToString_int(typ) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

/// Check if use of OpenVX is enabled
pub fn use_openvx() -> Result<bool> {
	unsafe { sys::cv_useOpenVX() }.into_result()
}

/// Returns the status of optimized code usage.
/// 
/// The function returns true if the optimized code is enabled. Otherwise, it returns false.
pub fn use_optimized() -> Result<bool> {
	unsafe { sys::cv_useOptimized() }.into_result()
}

pub fn dump_bool(argument: bool) -> Result<String> {
	unsafe { sys::cv_utils_dumpBool_bool(argument) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn dump_c_string(argument: &str) -> Result<String> {
	extern_container_arg!(argument);
	unsafe { sys::cv_utils_dumpCString_const_charX(argument.opencv_to_extern()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn dump_double(argument: f64) -> Result<String> {
	unsafe { sys::cv_utils_dumpDouble_double(argument) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn dump_float(argument: f32) -> Result<String> {
	unsafe { sys::cv_utils_dumpFloat_float(argument) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn dump_input_array_of_arrays(argument: &dyn core::ToInputArray) -> Result<String> {
	input_array_arg!(argument);
	unsafe { sys::cv_utils_dumpInputArrayOfArrays_const__InputArrayR(argument.as_raw__InputArray()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn dump_input_array(argument: &dyn core::ToInputArray) -> Result<String> {
	input_array_arg!(argument);
	unsafe { sys::cv_utils_dumpInputArray_const__InputArrayR(argument.as_raw__InputArray()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn dump_input_output_array_of_arrays(argument: &mut dyn core::ToInputOutputArray) -> Result<String> {
	input_output_array_arg!(argument);
	unsafe { sys::cv_utils_dumpInputOutputArrayOfArrays_const__InputOutputArrayR(argument.as_raw__InputOutputArray()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn dump_input_output_array(argument: &mut dyn core::ToInputOutputArray) -> Result<String> {
	input_output_array_arg!(argument);
	unsafe { sys::cv_utils_dumpInputOutputArray_const__InputOutputArrayR(argument.as_raw__InputOutputArray()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn dump_int(argument: i32) -> Result<String> {
	unsafe { sys::cv_utils_dumpInt_int(argument) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn dump_size_t(argument: size_t) -> Result<String> {
	unsafe { sys::cv_utils_dumpSizeT_size_t(argument) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

pub fn get_thread_id() -> Result<i32> {
	unsafe { sys::cv_utils_getThreadID() }.into_result()
}

/// Get global logging level
pub fn get_log_level() -> Result<core::LogLevel> {
	unsafe { sys::cv_utils_logging_getLogLevel() }.into_result()
}

pub fn get_log_tag_level(tag: &str) -> Result<core::LogLevel> {
	extern_container_arg!(tag);
	unsafe { sys::cv_utils_logging_getLogTagLevel_const_charX(tag.opencv_to_extern()) }.into_result()
}

/// Get global log tag
pub fn get_global_log_tag() -> Result<core::LogTag> {
	unsafe { sys::cv_utils_logging_internal_getGlobalLogTag() }.into_result().map(|r| unsafe { core::LogTag::opencv_from_extern(r) } )
}

/// Write log message
pub fn write_log_message_ex(log_level: core::LogLevel, tag: &str, file: &str, line: i32, func: &str, message: &str) -> Result<()> {
	extern_container_arg!(tag);
	extern_container_arg!(file);
	extern_container_arg!(func);
	extern_container_arg!(message);
	unsafe { sys::cv_utils_logging_internal_writeLogMessageEx_LogLevel_const_charX_const_charX_int_const_charX_const_charX(log_level, tag.opencv_to_extern(), file.opencv_to_extern(), line, func.opencv_to_extern(), message.opencv_to_extern()) }.into_result()
}

/// Write log message
pub fn write_log_message(log_level: core::LogLevel, message: &str) -> Result<()> {
	extern_container_arg!(message);
	unsafe { sys::cv_utils_logging_internal_writeLogMessage_LogLevel_const_charX(log_level, message.opencv_to_extern()) }.into_result()
}

pub fn register_log_tag(plogtag: &mut core::LogTag) -> Result<()> {
	unsafe { sys::cv_utils_logging_registerLogTag_LogTagX(plogtag.as_raw_mut_LogTag()) }.into_result()
}

/// Set global logging level
/// ## Returns
/// previous logging level
pub fn set_log_level(log_level: core::LogLevel) -> Result<core::LogLevel> {
	unsafe { sys::cv_utils_logging_setLogLevel_LogLevel(log_level) }.into_result()
}

pub fn set_log_tag_level(tag: &str, level: core::LogLevel) -> Result<()> {
	extern_container_arg!(tag);
	unsafe { sys::cv_utils_logging_setLogTagLevel_const_charX_LogLevel(tag.opencv_to_extern(), level) }.into_result()
}

pub fn test_async_array(argument: &dyn core::ToInputArray) -> Result<core::AsyncArray> {
	input_array_arg!(argument);
	unsafe { sys::cv_utils_testAsyncArray_const__InputArrayR(argument.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::AsyncArray::opencv_from_extern(r) } )
}

pub fn test_async_exception() -> Result<core::AsyncArray> {
	unsafe { sys::cv_utils_testAsyncException() }.into_result().map(|r| unsafe { core::AsyncArray::opencv_from_extern(r) } )
}

/// Converts VASurfaceID object to OutputArray.
/// ## Parameters
/// * display: - VADisplay object.
/// * surface: - source VASurfaceID object.
/// * size: - size of image represented by VASurfaceID object.
/// * dst: - destination OutputArray.
pub fn convert_from_va_surface(display: core::va_display, surface: core::va_surface_id, size: core::Size, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(dst);
	unsafe { sys::cv_va_intel_convertFromVASurface_VADisplay_VASurfaceID_Size_const__OutputArrayR(display, surface, size.opencv_to_extern(), dst.as_raw__OutputArray()) }.into_result()
}

/// Converts InputArray to VASurfaceID object.
/// ## Parameters
/// * display: - VADisplay object.
/// * src: - source InputArray.
/// * surface: - destination VASurfaceID object.
/// * size: - size of image represented by VASurfaceID object.
pub fn convert_to_va_surface(display: core::va_display, src: &dyn core::ToInputArray, surface: core::va_surface_id, size: core::Size) -> Result<()> {
	input_array_arg!(src);
	unsafe { sys::cv_va_intel_convertToVASurface_VADisplay_const__InputArrayR_VASurfaceID_Size(display, src.as_raw__InputArray(), surface, size.opencv_to_extern()) }.into_result()
}

/// Creates OpenCL context from VA.
/// ## Parameters
/// * display: - VADisplay for which CL interop should be established.
/// * tryInterop: - try to set up for interoperability, if true; set up for use slow copy if false.
/// ## Returns
/// Returns reference to OpenCL Context
/// 
/// ## C++ default parameters
/// * try_interop: true
pub fn initialize_context_from_va(display: core::va_display, try_interop: bool) -> Result<core::Context> {
	unsafe { sys::cv_va_intel_ocl_initializeContextFromVA_VADisplay_bool(display, try_interop) }.into_result().map(|r| unsafe { core::Context::opencv_from_extern(r) } )
}

/// Applies vertical concatenation to given matrices.
/// 
/// The function vertically concatenates two or more cv::Mat matrices (with the same number of cols).
/// ```ignore
///    cv::Mat matArray[] = { cv::Mat(1, 4, CV_8UC1, cv::Scalar(1)),
///                            cv::Mat(1, 4, CV_8UC1, cv::Scalar(2)),
///                            cv::Mat(1, 4, CV_8UC1, cv::Scalar(3)),};
/// 
///    cv::Mat out;
///    cv::vconcat( matArray, 3, out );
///    //out:
///    //[1,   1,   1,   1;
///    // 2,   2,   2,   2;
///    // 3,   3,   3,   3]
/// ```
/// 
/// ## Parameters
/// * src: input array or vector of matrices. all of the matrices must have the same number of cols and the same depth.
/// * nsrc: number of matrices in src.
/// * dst: output array. It has the same number of cols and depth as the src, and the sum of rows of the src.
/// ## See also
/// cv::hconcat(const Mat*, size_t, OutputArray), cv::hconcat(InputArrayOfArrays, OutputArray) and cv::hconcat(InputArray, InputArray, OutputArray)
/// 
/// ## Overloaded parameters
/// 
///  ```ignore
///    cv::Mat_<float> A = (cv::Mat_<float>(3, 2) << 1, 7,
///                                                   2, 8,
///                                                   3, 9);
///    cv::Mat_<float> B = (cv::Mat_<float>(3, 2) << 4, 10,
///                                                   5, 11,
///                                                   6, 12);
/// 
///    cv::Mat C;
///    cv::vconcat(A, B, C);
///    //C:
///    //[1, 7;
///    // 2, 8;
///    // 3, 9;
///    // 4, 10;
///    // 5, 11;
///    // 6, 12]
///  ```
/// 
/// * src1: first input array to be considered for vertical concatenation.
/// * src2: second input array to be considered for vertical concatenation.
/// * dst: output array. It has the same number of cols and depth as the src1 and src2, and the sum of rows of the src1 and src2.
pub fn vconcat2(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	unsafe { sys::cv_vconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Applies vertical concatenation to given matrices.
/// 
/// The function vertically concatenates two or more cv::Mat matrices (with the same number of cols).
/// ```ignore
///    cv::Mat matArray[] = { cv::Mat(1, 4, CV_8UC1, cv::Scalar(1)),
///                            cv::Mat(1, 4, CV_8UC1, cv::Scalar(2)),
///                            cv::Mat(1, 4, CV_8UC1, cv::Scalar(3)),};
/// 
///    cv::Mat out;
///    cv::vconcat( matArray, 3, out );
///    //out:
///    //[1,   1,   1,   1;
///    // 2,   2,   2,   2;
///    // 3,   3,   3,   3]
/// ```
/// 
/// ## Parameters
/// * src: input array or vector of matrices. all of the matrices must have the same number of cols and the same depth.
/// * nsrc: number of matrices in src.
/// * dst: output array. It has the same number of cols and depth as the src, and the sum of rows of the src.
/// ## See also
/// cv::hconcat(const Mat*, size_t, OutputArray), cv::hconcat(InputArrayOfArrays, OutputArray) and cv::hconcat(InputArray, InputArray, OutputArray)
/// 
/// ## Overloaded parameters
/// 
///  ```ignore
///    std::vector<cv::Mat> matrices = { cv::Mat(1, 4, CV_8UC1, cv::Scalar(1)),
///                                       cv::Mat(1, 4, CV_8UC1, cv::Scalar(2)),
///                                       cv::Mat(1, 4, CV_8UC1, cv::Scalar(3)),};
/// 
///    cv::Mat out;
///    cv::vconcat( matrices, out );
///    //out:
///    //[1,   1,   1,   1;
///    // 2,   2,   2,   2;
///    // 3,   3,   3,   3]
///  ```
/// 
/// * src: input array or vector of matrices. all of the matrices must have the same number of cols and the same depth
/// * dst: output array. It has the same number of cols and depth as the src, and the sum of rows of the src.
/// same depth.
pub fn vconcat(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	unsafe { sys::cv_vconcat_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

pub fn write_scalar_str(fs: &mut core::FileStorage, value: &str) -> Result<()> {
	extern_container_arg!(value);
	unsafe { sys::cv_writeScalar_FileStorageR_const_StringR(fs.as_raw_mut_FileStorage(), value.opencv_to_extern()) }.into_result()
}

pub fn write_scalar_f64(fs: &mut core::FileStorage, value: f64) -> Result<()> {
	unsafe { sys::cv_writeScalar_FileStorageR_double(fs.as_raw_mut_FileStorage(), value) }.into_result()
}

pub fn write_scalar_f32(fs: &mut core::FileStorage, value: f32) -> Result<()> {
	unsafe { sys::cv_writeScalar_FileStorageR_float(fs.as_raw_mut_FileStorage(), value) }.into_result()
}

pub fn write_scalar_i32(fs: &mut core::FileStorage, value: i32) -> Result<()> {
	unsafe { sys::cv_writeScalar_FileStorageR_int(fs.as_raw_mut_FileStorage(), value) }.into_result()
}

pub fn write_mat(fs: &mut core::FileStorage, name: &str, value: &core::Mat) -> Result<()> {
	extern_container_arg!(name);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_MatR(fs.as_raw_mut_FileStorage(), name.opencv_to_extern(), value.as_raw_Mat()) }.into_result()
}

pub fn write_sparsemat(fs: &mut core::FileStorage, name: &str, value: &core::SparseMat) -> Result<()> {
	extern_container_arg!(name);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_SparseMatR(fs.as_raw_mut_FileStorage(), name.opencv_to_extern(), value.as_raw_SparseMat()) }.into_result()
}

pub fn write_str(fs: &mut core::FileStorage, name: &str, value: &str) -> Result<()> {
	extern_container_arg!(name);
	extern_container_arg!(value);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_StringR(fs.as_raw_mut_FileStorage(), name.opencv_to_extern(), value.opencv_to_extern()) }.into_result()
}

pub fn write_dmatch_vec(fs: &mut core::FileStorage, name: &str, value: &core::Vector::<core::DMatch>) -> Result<()> {
	extern_container_arg!(name);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_vector_DMatch_R(fs.as_raw_mut_FileStorage(), name.opencv_to_extern(), value.as_raw_VectorOfDMatch()) }.into_result()
}

pub fn write_keypoint_vec(fs: &mut core::FileStorage, name: &str, value: &core::Vector::<core::KeyPoint>) -> Result<()> {
	extern_container_arg!(name);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_vector_KeyPoint_R(fs.as_raw_mut_FileStorage(), name.opencv_to_extern(), value.as_raw_VectorOfKeyPoint()) }.into_result()
}

pub fn write_f64(fs: &mut core::FileStorage, name: &str, value: f64) -> Result<()> {
	extern_container_arg!(name);
	unsafe { sys::cv_write_FileStorageR_const_StringR_double(fs.as_raw_mut_FileStorage(), name.opencv_to_extern(), value) }.into_result()
}

pub fn write_f32(fs: &mut core::FileStorage, name: &str, value: f32) -> Result<()> {
	extern_container_arg!(name);
	unsafe { sys::cv_write_FileStorageR_const_StringR_float(fs.as_raw_mut_FileStorage(), name.opencv_to_extern(), value) }.into_result()
}

/// @relates cv::FileStorage
pub fn write_i32(fs: &mut core::FileStorage, name: &str, value: i32) -> Result<()> {
	extern_container_arg!(name);
	unsafe { sys::cv_write_FileStorageR_const_StringR_int(fs.as_raw_mut_FileStorage(), name.opencv_to_extern(), value) }.into_result()
}

/// This is a base class for all more or less complex algorithms in OpenCV
/// 
/// especially for classes of algorithms, for which there can be multiple implementations. The examples
/// are stereo correspondence (for which there are algorithms like block matching, semi-global block
/// matching, graph-cut etc.), background subtraction (which can be done using mixture-of-gaussians
/// models, codebook-based algorithm etc.), optical flow (block matching, Lucas-Kanade, Horn-Schunck
/// etc.).
/// 
/// Here is example of SimpleBlobDetector use in your application via Algorithm interface:
/// [Algorithm](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_various.cpp#L1)
pub trait AlgorithmTrait {
	fn as_raw_Algorithm(&self) -> *const c_void;
	fn as_raw_mut_Algorithm(&mut self) -> *mut c_void;

	/// Clears the algorithm state
	fn clear(&mut self) -> Result<()> {
		unsafe { sys::cv_Algorithm_clear(self.as_raw_mut_Algorithm()) }.into_result()
	}
	
	/// Stores algorithm parameters in a file storage
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_Algorithm_write_const_FileStorageR(self.as_raw_Algorithm(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	/// simplified API for language bindings
	///  Stores algorithm parameters in a file storage
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * name: String()
	fn write_with_name(&self, fs: &core::Ptr::<core::FileStorage>, name: &str) -> Result<()> {
		extern_container_arg!(name);
		unsafe { sys::cv_Algorithm_write_const_const_Ptr_FileStorage_R_const_StringR(self.as_raw_Algorithm(), fs.as_raw_PtrOfFileStorage(), name.opencv_to_extern()) }.into_result()
	}
	
	/// Reads algorithm parameters from a file storage
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_Algorithm_read_const_FileNodeR(self.as_raw_mut_Algorithm(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	/// Returns true if the Algorithm is empty (e.g. in the very beginning or after unsuccessful read
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_Algorithm_empty_const(self.as_raw_Algorithm()) }.into_result()
	}
	
	/// Saves the algorithm to a file.
	/// In order to make this method work, the derived class must implement Algorithm::write(FileStorage& fs).
	fn save(&self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		unsafe { sys::cv_Algorithm_save_const_const_StringR(self.as_raw_Algorithm(), filename.opencv_to_extern()) }.into_result()
	}
	
	/// Returns the algorithm string identifier.
	/// This string is used as top level xml/yml node tag when the object is saved to a file or string.
	fn get_default_name(&self) -> Result<String> {
		unsafe { sys::cv_Algorithm_getDefaultName_const(self.as_raw_Algorithm()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

/// This is a base class for all more or less complex algorithms in OpenCV
/// 
/// especially for classes of algorithms, for which there can be multiple implementations. The examples
/// are stereo correspondence (for which there are algorithms like block matching, semi-global block
/// matching, graph-cut etc.), background subtraction (which can be done using mixture-of-gaussians
/// models, codebook-based algorithm etc.), optical flow (block matching, Lucas-Kanade, Horn-Schunck
/// etc.).
/// 
/// Here is example of SimpleBlobDetector use in your application via Algorithm interface:
/// [Algorithm](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_various.cpp#L1)
pub struct Algorithm {
	ptr: *mut c_void
}

opencv_type_boxed! { Algorithm }

impl Drop for Algorithm {
	fn drop(&mut self) {
		extern "C" { fn cv_Algorithm_delete(instance: *mut c_void); }
		unsafe { cv_Algorithm_delete(self.as_raw_mut_Algorithm()) };
	}
}

impl Algorithm {
	#[inline] pub fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Algorithm {}

impl core::AlgorithmTrait for Algorithm {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Algorithm {
	pub fn default() -> Result<core::Algorithm> {
		unsafe { sys::cv_Algorithm_Algorithm() }.into_result().map(|r| unsafe { core::Algorithm::opencv_from_extern(r) } )
	}
	
}

/// Returns result of asynchronous operations
/// 
/// Object has attached asynchronous state.
/// Assignment operator doesn't clone asynchronous state (it is shared between all instances).
/// 
/// Result can be fetched via get() method only once.
pub trait AsyncArrayTrait {
	fn as_raw_AsyncArray(&self) -> *const c_void;
	fn as_raw_mut_AsyncArray(&mut self) -> *mut c_void;

	fn release(&mut self) -> () {
		unsafe { sys::cv_AsyncArray_release(self.as_raw_mut_AsyncArray()) }.into_result().expect("Infallible function failed: release")
	}
	
	/// Fetch the result.
	/// ## Parameters
	/// * dst:[out] destination array
	/// 
	/// Waits for result until container has valid result.
	/// Throws exception if exception was stored as a result.
	/// 
	/// Throws exception on invalid container state.
	/// 
	/// 
	/// Note: Result or stored exception can be fetched only once.
	fn get(&self, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		unsafe { sys::cv_AsyncArray_get_const_const__OutputArrayR(self.as_raw_AsyncArray(), dst.as_raw__OutputArray()) }.into_result()
	}
	
	/// Retrieving the result with timeout
	/// ## Parameters
	/// * dst:[out] destination array
	/// * timeoutNs: timeout in nanoseconds, -1 for infinite wait
	/// 
	/// ## Returns
	/// true if result is ready, false if the timeout has expired
	/// 
	/// 
	/// Note: Result or stored exception can be fetched only once.
	fn get_with_timeout(&self, dst: &mut dyn core::ToOutputArray, timeout_ns: i64) -> Result<bool> {
		output_array_arg!(dst);
		unsafe { sys::cv_AsyncArray_get_const_const__OutputArrayR_int64_t(self.as_raw_AsyncArray(), dst.as_raw__OutputArray(), timeout_ns) }.into_result()
	}
	
	fn get_with_timeout_f64(&self, dst: &mut dyn core::ToOutputArray, timeout_ns: f64) -> Result<bool> {
		output_array_arg!(dst);
		unsafe { sys::cv_AsyncArray_get_const_const__OutputArrayR_double(self.as_raw_AsyncArray(), dst.as_raw__OutputArray(), timeout_ns) }.into_result()
	}
	
	fn wait_for(&self, timeout_ns: i64) -> Result<bool> {
		unsafe { sys::cv_AsyncArray_wait_for_const_int64_t(self.as_raw_AsyncArray(), timeout_ns) }.into_result()
	}
	
	fn wait_for_f64(&self, timeout_ns: f64) -> Result<bool> {
		unsafe { sys::cv_AsyncArray_wait_for_const_double(self.as_raw_AsyncArray(), timeout_ns) }.into_result()
	}
	
	fn valid(&self) -> bool {
		unsafe { sys::cv_AsyncArray_valid_const(self.as_raw_AsyncArray()) }.into_result().expect("Infallible function failed: valid")
	}
	
}

/// Returns result of asynchronous operations
/// 
/// Object has attached asynchronous state.
/// Assignment operator doesn't clone asynchronous state (it is shared between all instances).
/// 
/// Result can be fetched via get() method only once.
pub struct AsyncArray {
	ptr: *mut c_void
}

opencv_type_boxed! { AsyncArray }

impl Drop for AsyncArray {
	fn drop(&mut self) {
		extern "C" { fn cv_AsyncArray_delete(instance: *mut c_void); }
		unsafe { cv_AsyncArray_delete(self.as_raw_mut_AsyncArray()) };
	}
}

impl AsyncArray {
	#[inline] pub fn as_raw_AsyncArray(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_AsyncArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for AsyncArray {}

impl core::AsyncArrayTrait for AsyncArray {
	#[inline] fn as_raw_AsyncArray(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_AsyncArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AsyncArray {
	pub fn default() -> core::AsyncArray {
		unsafe { sys::cv_AsyncArray_AsyncArray() }.into_result().map(|r| unsafe { core::AsyncArray::opencv_from_extern(r) } ).expect("Infallible function failed: default")
	}
	
	pub fn copy(o: &core::AsyncArray) -> core::AsyncArray {
		unsafe { sys::cv_AsyncArray_AsyncArray_const_AsyncArrayR(o.as_raw_AsyncArray()) }.into_result().map(|r| unsafe { core::AsyncArray::opencv_from_extern(r) } ).expect("Infallible function failed: copy")
	}
	
	pub fn copy_mut(o: &mut core::AsyncArray) -> Result<core::AsyncArray> {
		unsafe { sys::cv_AsyncArray_AsyncArray_AsyncArrayR(o.as_raw_mut_AsyncArray()) }.into_result().map(|r| unsafe { core::AsyncArray::opencv_from_extern(r) } )
	}
	
}

/// Provides result of asynchronous operations
pub trait AsyncPromiseTrait {
	fn as_raw_AsyncPromise(&self) -> *const c_void;
	fn as_raw_mut_AsyncPromise(&mut self) -> *mut c_void;

	fn release(&mut self) -> () {
		unsafe { sys::cv_AsyncPromise_release(self.as_raw_mut_AsyncPromise()) }.into_result().expect("Infallible function failed: release")
	}
	
	/// Returns associated AsyncArray
	/// 
	/// Note: Can be called once
	fn get_array_result(&mut self) -> Result<core::AsyncArray> {
		unsafe { sys::cv_AsyncPromise_getArrayResult(self.as_raw_mut_AsyncPromise()) }.into_result().map(|r| unsafe { core::AsyncArray::opencv_from_extern(r) } )
	}
	
	/// Stores asynchronous result.
	/// ## Parameters
	/// * value: result
	fn set_value(&mut self, value: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(value);
		unsafe { sys::cv_AsyncPromise_setValue_const__InputArrayR(self.as_raw_mut_AsyncPromise(), value.as_raw__InputArray()) }.into_result()
	}
	
	/// Stores exception.
	/// ## Parameters
	/// * exception: exception to be raised in AsyncArray
	fn set_exception(&mut self, exception: &core::Exception) -> Result<()> {
		unsafe { sys::cv_AsyncPromise_setException_const_ExceptionR(self.as_raw_mut_AsyncPromise(), exception.as_raw_Exception()) }.into_result()
	}
	
	fn _get_impl(&self) -> *mut c_void {
		unsafe { sys::cv_AsyncPromise__getImpl_const(self.as_raw_AsyncPromise()) }.into_result().expect("Infallible function failed: _get_impl")
	}
	
}

/// Provides result of asynchronous operations
pub struct AsyncPromise {
	ptr: *mut c_void
}

opencv_type_boxed! { AsyncPromise }

impl Drop for AsyncPromise {
	fn drop(&mut self) {
		extern "C" { fn cv_AsyncPromise_delete(instance: *mut c_void); }
		unsafe { cv_AsyncPromise_delete(self.as_raw_mut_AsyncPromise()) };
	}
}

impl AsyncPromise {
	#[inline] pub fn as_raw_AsyncPromise(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_AsyncPromise(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for AsyncPromise {}

impl core::AsyncPromiseTrait for AsyncPromise {
	#[inline] fn as_raw_AsyncPromise(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_AsyncPromise(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AsyncPromise {
	pub fn default() -> core::AsyncPromise {
		unsafe { sys::cv_AsyncPromise_AsyncPromise() }.into_result().map(|r| unsafe { core::AsyncPromise::opencv_from_extern(r) } ).expect("Infallible function failed: default")
	}
	
	pub fn copy(o: &core::AsyncPromise) -> core::AsyncPromise {
		unsafe { sys::cv_AsyncPromise_AsyncPromise_const_AsyncPromiseR(o.as_raw_AsyncPromise()) }.into_result().map(|r| unsafe { core::AsyncPromise::opencv_from_extern(r) } ).expect("Infallible function failed: copy")
	}
	
	pub fn copy_mut(o: &mut core::AsyncPromise) -> Result<core::AsyncPromise> {
		unsafe { sys::cv_AsyncPromise_AsyncPromise_AsyncPromiseR(o.as_raw_mut_AsyncPromise()) }.into_result().map(|r| unsafe { core::AsyncPromise::opencv_from_extern(r) } )
	}
	
}

/// Designed for command line parsing
/// 
/// The sample below demonstrates how to use CommandLineParser:
/// ```ignore
///    CommandLineParser parser(argc, argv, keys);
///    parser.about("Application name v1.0.0");
/// 
///    if (parser.has("help"))
///    {
///        parser.printMessage();
///        return 0;
///    }
/// 
///    int N = parser.get<int>("N");
///    double fps = parser.get<double>("fps");
///    String path = parser.get<String>("path");
/// 
///    use_time_stamp = parser.has("timestamp");
/// 
///    String img1 = parser.get<String>(0);
///    String img2 = parser.get<String>(1);
/// 
///    int repeat = parser.get<int>(2);
/// 
///    if (!parser.check())
///    {
///        parser.printErrors();
///        return 0;
///    }
/// ```
/// 
/// 
/// ### Keys syntax
/// 
/// The keys parameter is a string containing several blocks, each one is enclosed in curly braces and
/// describes one argument. Each argument contains three parts separated by the `|` symbol:
/// 
/// -# argument names is a space-separated list of option synonyms (to mark argument as positional, prefix it with the `@` symbol)
/// -# default value will be used if the argument was not provided (can be empty)
/// -# help message (can be empty)
/// 
/// For example:
/// 
/// ```ignore
///    const String keys =
///        "{help h usage ? |      | print this message   }"
///        "{@image1        |      | image1 for compare   }"
///        "{@image2        |<none>| image2 for compare   }"
///        "{@repeat        |1     | number               }"
///        "{path           |.     | path to file         }"
///        "{fps            | -1.0 | fps for output video }"
///        "{N count        |100   | count of objects     }"
///        "{ts timestamp   |      | use time stamp       }"
///        ;
/// }
/// ```
/// 
/// 
/// Note that there are no default values for `help` and `timestamp` so we can check their presence using the `has()` method.
/// Arguments with default values are considered to be always present. Use the `get()` method in these cases to check their
/// actual value instead.
/// 
/// String keys like `get<String>("@image1")` return the empty string `""` by default - even with an empty default value.
/// Use the special `<none>` default value to enforce that the returned string must not be empty. (like in `get<String>("@image2")`)
/// 
/// ### Usage
/// 
/// For the described keys:
/// 
/// ```ignore
///    # Good call (3 positional parameters: image1, image2 and repeat; N is 200, ts is true)
///    $ ./app -N=200 1.png 2.jpg 19 -ts
/// 
///    # Bad call
///    $ ./app -fps=aaa
///    ERRORS:
///    Parameter "fps": can not convert: [aaa] to [double]
/// ```
/// 
pub trait CommandLineParserTrait {
	fn as_raw_CommandLineParser(&self) -> *const c_void;
	fn as_raw_mut_CommandLineParser(&mut self) -> *mut c_void;

	/// Returns application path
	/// 
	/// This method returns the path to the executable from the command line (`argv[0]`).
	/// 
	/// For example, if the application has been started with such a command:
	/// ```ignore
	/// $ ./bin/my-executable
	/// ```
	/// 
	/// this method will return `./bin`.
	fn get_path_to_application(&self) -> Result<String> {
		unsafe { sys::cv_CommandLineParser_getPathToApplication_const(self.as_raw_CommandLineParser()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// Check if field was provided in the command line
	/// 
	/// ## Parameters
	/// * name: argument name to check
	fn has(&self, name: &str) -> Result<bool> {
		extern_container_arg!(name);
		unsafe { sys::cv_CommandLineParser_has_const_const_StringR(self.as_raw_CommandLineParser(), name.opencv_to_extern()) }.into_result()
	}
	
	/// Check for parsing errors
	/// 
	/// Returns false if error occurred while accessing the parameters (bad conversion, missing arguments,
	/// etc.). Call @ref printErrors to print error messages list.
	fn check(&self) -> Result<bool> {
		unsafe { sys::cv_CommandLineParser_check_const(self.as_raw_CommandLineParser()) }.into_result()
	}
	
	/// Set the about message
	/// 
	/// The about message will be shown when @ref printMessage is called, right before arguments table.
	fn about(&mut self, message: &str) -> Result<()> {
		extern_container_arg!(message);
		unsafe { sys::cv_CommandLineParser_about_const_StringR(self.as_raw_mut_CommandLineParser(), message.opencv_to_extern()) }.into_result()
	}
	
	/// Print help message
	/// 
	/// This method will print standard help message containing the about message and arguments description.
	/// ## See also
	/// about
	fn print_message(&self) -> Result<()> {
		unsafe { sys::cv_CommandLineParser_printMessage_const(self.as_raw_CommandLineParser()) }.into_result()
	}
	
	/// Print list of errors occurred
	/// ## See also
	/// check
	fn print_errors(&self) -> Result<()> {
		unsafe { sys::cv_CommandLineParser_printErrors_const(self.as_raw_CommandLineParser()) }.into_result()
	}
	
}

/// Designed for command line parsing
/// 
/// The sample below demonstrates how to use CommandLineParser:
/// ```ignore
///    CommandLineParser parser(argc, argv, keys);
///    parser.about("Application name v1.0.0");
/// 
///    if (parser.has("help"))
///    {
///        parser.printMessage();
///        return 0;
///    }
/// 
///    int N = parser.get<int>("N");
///    double fps = parser.get<double>("fps");
///    String path = parser.get<String>("path");
/// 
///    use_time_stamp = parser.has("timestamp");
/// 
///    String img1 = parser.get<String>(0);
///    String img2 = parser.get<String>(1);
/// 
///    int repeat = parser.get<int>(2);
/// 
///    if (!parser.check())
///    {
///        parser.printErrors();
///        return 0;
///    }
/// ```
/// 
/// 
/// ### Keys syntax
/// 
/// The keys parameter is a string containing several blocks, each one is enclosed in curly braces and
/// describes one argument. Each argument contains three parts separated by the `|` symbol:
/// 
/// -# argument names is a space-separated list of option synonyms (to mark argument as positional, prefix it with the `@` symbol)
/// -# default value will be used if the argument was not provided (can be empty)
/// -# help message (can be empty)
/// 
/// For example:
/// 
/// ```ignore
///    const String keys =
///        "{help h usage ? |      | print this message   }"
///        "{@image1        |      | image1 for compare   }"
///        "{@image2        |<none>| image2 for compare   }"
///        "{@repeat        |1     | number               }"
///        "{path           |.     | path to file         }"
///        "{fps            | -1.0 | fps for output video }"
///        "{N count        |100   | count of objects     }"
///        "{ts timestamp   |      | use time stamp       }"
///        ;
/// }
/// ```
/// 
/// 
/// Note that there are no default values for `help` and `timestamp` so we can check their presence using the `has()` method.
/// Arguments with default values are considered to be always present. Use the `get()` method in these cases to check their
/// actual value instead.
/// 
/// String keys like `get<String>("@image1")` return the empty string `""` by default - even with an empty default value.
/// Use the special `<none>` default value to enforce that the returned string must not be empty. (like in `get<String>("@image2")`)
/// 
/// ### Usage
/// 
/// For the described keys:
/// 
/// ```ignore
///    # Good call (3 positional parameters: image1, image2 and repeat; N is 200, ts is true)
///    $ ./app -N=200 1.png 2.jpg 19 -ts
/// 
///    # Bad call
///    $ ./app -fps=aaa
///    ERRORS:
///    Parameter "fps": can not convert: [aaa] to [double]
/// ```
/// 
pub struct CommandLineParser {
	ptr: *mut c_void
}

opencv_type_boxed! { CommandLineParser }

impl Drop for CommandLineParser {
	fn drop(&mut self) {
		extern "C" { fn cv_CommandLineParser_delete(instance: *mut c_void); }
		unsafe { cv_CommandLineParser_delete(self.as_raw_mut_CommandLineParser()) };
	}
}

impl CommandLineParser {
	#[inline] pub fn as_raw_CommandLineParser(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_CommandLineParser(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CommandLineParser {}

impl core::CommandLineParserTrait for CommandLineParser {
	#[inline] fn as_raw_CommandLineParser(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_CommandLineParser(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CommandLineParser {
	/// Constructor
	/// 
	/// Initializes command line parser object
	/// 
	/// ## Parameters
	/// * argc: number of command line arguments (from main())
	/// * argv: array of command line arguments (from main())
	/// * keys: string describing acceptable command line parameters (see class description for syntax)
	pub fn new(argc: i32, argv: &[&str], keys: &str) -> Result<core::CommandLineParser> {
		string_array_arg!(argv);
		extern_container_arg!(keys);
		unsafe { sys::cv_CommandLineParser_CommandLineParser_int_const_charXX_const_StringR(argc, argv.as_ptr(), keys.opencv_to_extern()) }.into_result().map(|r| unsafe { core::CommandLineParser::opencv_from_extern(r) } )
	}
	
	/// Copy constructor
	pub fn copy(parser: &core::CommandLineParser) -> Result<core::CommandLineParser> {
		unsafe { sys::cv_CommandLineParser_CommandLineParser_const_CommandLineParserR(parser.as_raw_CommandLineParser()) }.into_result().map(|r| unsafe { core::CommandLineParser::opencv_from_extern(r) } )
	}
	
}

/// This class is used to perform the non-linear non-constrained minimization of a function
/// with known gradient,
/// 
/// defined on an *n*-dimensional Euclidean space, using the **Nonlinear Conjugate Gradient method**.
/// The implementation was done based on the beautifully clear explanatory article [An Introduction to
/// the Conjugate Gradient Method Without the Agonizing
/// Pain](http://www.cs.cmu.edu/~quake-papers/painless-conjugate-gradient.pdf) by Jonathan Richard
/// Shewchuk. The method can be seen as an adaptation of a standard Conjugate Gradient method (see, for
/// example <http://en.wikipedia.org/wiki/Conjugate_gradient_method>) for numerically solving the
/// systems of linear equations.
/// 
/// It should be noted, that this method, although deterministic, is rather a heuristic method and
/// therefore may converge to a local minima, not necessary a global one. What is even more disastrous,
/// most of its behaviour is ruled by gradient, therefore it essentially cannot distinguish between
/// local minima and maxima. Therefore, if it starts sufficiently near to the local maximum, it may
/// converge to it. Another obvious restriction is that it should be possible to compute the gradient of
/// a function at any point, thus it is preferable to have analytic expression for gradient and
/// computational burden should be born by the user.
/// 
/// The latter responsibility is accomplished via the getGradient method of a
/// MinProblemSolver::Function interface (which represents function being optimized). This method takes
/// point a point in *n*-dimensional space (first argument represents the array of coordinates of that
/// point) and compute its gradient (it should be stored in the second argument as an array).
/// 
/// 
/// Note: class ConjGradSolver thus does not add any new methods to the basic MinProblemSolver interface.
/// 
/// 
/// Note: term criteria should meet following condition:
/// ```ignore
///    termcrit.type == (TermCriteria::MAX_ITER + TermCriteria::EPS) && termcrit.epsilon > 0 && termcrit.maxCount > 0
///    // or
///    termcrit.type == TermCriteria::MAX_ITER) && termcrit.maxCount > 0
/// ```
/// 
pub trait ConjGradSolver: core::MinProblemSolver {
	fn as_raw_ConjGradSolver(&self) -> *const c_void;
	fn as_raw_mut_ConjGradSolver(&mut self) -> *mut c_void;

}

impl dyn ConjGradSolver + '_ {
	/// This function returns the reference to the ready-to-use ConjGradSolver object.
	/// 
	/// All the parameters are optional, so this procedure can be called even without parameters at
	/// all. In this case, the default values will be used. As default value for terminal criteria are
	/// the only sensible ones, MinProblemSolver::setFunction() should be called upon the obtained
	/// object, if the function was not given to create(). Otherwise, the two ways (submit it to
	/// create() or miss it out and call the MinProblemSolver::setFunction()) are absolutely equivalent
	/// (and will drop the same errors in the same way, should invalid input be detected).
	/// ## Parameters
	/// * f: Pointer to the function that will be minimized, similarly to the one you submit via
	/// MinProblemSolver::setFunction.
	/// * termcrit: Terminal criteria to the algorithm, similarly to the one you submit via
	/// MinProblemSolver::setTermCriteria.
	/// 
	/// ## C++ default parameters
	/// * f: Ptr<ConjGradSolver::Function>()
	/// * termcrit: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5000,0.000001)
	pub fn create(f: &core::Ptr::<dyn core::MinProblemSolver_Function>, termcrit: core::TermCriteria) -> Result<core::Ptr::<dyn core::ConjGradSolver>> {
		unsafe { sys::cv_ConjGradSolver_create_const_Ptr_Function_R_TermCriteria(f.as_raw_PtrOfMinProblemSolver_Function(), termcrit.opencv_to_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn core::ConjGradSolver>::opencv_from_extern(r) } )
	}
	
}
/// Class for matching keypoint descriptors
/// 
/// query descriptor index, train descriptor index, train image index, and distance between
/// descriptors.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DMatch {
	/// query descriptor index
	pub query_idx: i32,
	/// train descriptor index
	pub train_idx: i32,
	/// train image index
	pub img_idx: i32,
	pub distance: f32,
}

opencv_type_simple! { core::DMatch }

impl DMatch {
	/// ////////////////////////////// DMatch ////////////////////////////////
	pub fn default() -> Result<core::DMatch> {
		unsafe { sys::cv_DMatch_DMatch() }.into_result()
	}
	
	pub fn new(_query_idx: i32, _train_idx: i32, _distance: f32) -> Result<core::DMatch> {
		unsafe { sys::cv_DMatch_DMatch_int_int_float(_query_idx, _train_idx, _distance) }.into_result()
	}
	
	pub fn new_index(_query_idx: i32, _train_idx: i32, _img_idx: i32, _distance: f32) -> Result<core::DMatch> {
		unsafe { sys::cv_DMatch_DMatch_int_int_int_float(_query_idx, _train_idx, _img_idx, _distance) }.into_result()
	}
	
}

/// This class is used to perform the non-linear non-constrained minimization of a function,
/// 
/// defined on an `n`-dimensional Euclidean space, using the **Nelder-Mead method**, also known as
/// **downhill simplex method**. The basic idea about the method can be obtained from
/// <http://en.wikipedia.org/wiki/Nelder-Mead_method>.
/// 
/// It should be noted, that this method, although deterministic, is rather a heuristic and therefore
/// may converge to a local minima, not necessary a global one. It is iterative optimization technique,
/// which at each step uses an information about the values of a function evaluated only at `n+1`
/// points, arranged as a *simplex* in `n`-dimensional space (hence the second name of the method). At
/// each step new point is chosen to evaluate function at, obtained value is compared with previous
/// ones and based on this information simplex changes it's shape , slowly moving to the local minimum.
/// Thus this method is using *only* function values to make decision, on contrary to, say, Nonlinear
/// Conjugate Gradient method (which is also implemented in optim).
/// 
/// Algorithm stops when the number of function evaluations done exceeds termcrit.maxCount, when the
/// function values at the vertices of simplex are within termcrit.epsilon range or simplex becomes so
/// small that it can enclosed in a box with termcrit.epsilon sides, whatever comes first, for some
/// defined by user positive integer termcrit.maxCount and positive non-integer termcrit.epsilon.
/// 
/// 
/// Note: DownhillSolver is a derivative of the abstract interface
/// cv::MinProblemSolver, which in turn is derived from the Algorithm interface and is used to
/// encapsulate the functionality, common to all non-linear optimization algorithms in the optim
/// module.
/// 
/// 
/// Note: term criteria should meet following condition:
/// ```ignore
///    termcrit.type == (TermCriteria::MAX_ITER + TermCriteria::EPS) && termcrit.epsilon > 0 && termcrit.maxCount > 0
/// ```
/// 
pub trait DownhillSolver: core::MinProblemSolver {
	fn as_raw_DownhillSolver(&self) -> *const c_void;
	fn as_raw_mut_DownhillSolver(&mut self) -> *mut c_void;

	/// Returns the initial step that will be used in downhill simplex algorithm.
	/// 
	/// ## Parameters
	/// * step: Initial step that will be used in algorithm. Note, that although corresponding setter
	/// accepts column-vectors as well as row-vectors, this method will return a row-vector.
	/// ## See also
	/// DownhillSolver::setInitStep
	fn get_init_step(&self, step: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(step);
		unsafe { sys::cv_DownhillSolver_getInitStep_const_const__OutputArrayR(self.as_raw_DownhillSolver(), step.as_raw__OutputArray()) }.into_result()
	}
	
	/// Sets the initial step that will be used in downhill simplex algorithm.
	/// 
	/// Step, together with initial point (given in DownhillSolver::minimize) are two `n`-dimensional
	/// vectors that are used to determine the shape of initial simplex. Roughly said, initial point
	/// determines the position of a simplex (it will become simplex's centroid), while step determines the
	/// spread (size in each dimension) of a simplex. To be more precise, if ![inline formula](https://latex.codecogs.com/png.latex?s%2Cx%5F0%5Cin%5Cmathbb%7BR%7D%5En) are
	/// the initial step and initial point respectively, the vertices of a simplex will be:
	/// ![inline formula](https://latex.codecogs.com/png.latex?v%5F0%3A%3Dx%5F0%2D%5Cfrac%7B1%7D%7B2%7D%20s) and ![inline formula](https://latex.codecogs.com/png.latex?v%5Fi%3A%3Dx%5F0%2Bs%5Fi) for ![inline formula](https://latex.codecogs.com/png.latex?i%3D1%2C2%2C%5Cdots%2Cn) where ![inline formula](https://latex.codecogs.com/png.latex?s%5Fi) denotes
	/// projections of the initial step of *n*-th coordinate (the result of projection is treated to be
	/// vector given by ![inline formula](https://latex.codecogs.com/png.latex?s%5Fi%3A%3De%5Fi%5Ccdot%5Cleft%3Ce%5Fi%5Ccdot%20s%5Cright%3E), where ![inline formula](https://latex.codecogs.com/png.latex?e%5Fi) form canonical basis)
	/// 
	/// ## Parameters
	/// * step: Initial step that will be used in algorithm. Roughly said, it determines the spread
	/// (size in each dimension) of an initial simplex.
	fn set_init_step(&mut self, step: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(step);
		unsafe { sys::cv_DownhillSolver_setInitStep_const__InputArrayR(self.as_raw_mut_DownhillSolver(), step.as_raw__InputArray()) }.into_result()
	}
	
}

impl dyn DownhillSolver + '_ {
	/// This function returns the reference to the ready-to-use DownhillSolver object.
	/// 
	/// All the parameters are optional, so this procedure can be called even without parameters at
	/// all. In this case, the default values will be used. As default value for terminal criteria are
	/// the only sensible ones, MinProblemSolver::setFunction() and DownhillSolver::setInitStep()
	/// should be called upon the obtained object, if the respective parameters were not given to
	/// create(). Otherwise, the two ways (give parameters to createDownhillSolver() or miss them out
	/// and call the MinProblemSolver::setFunction() and DownhillSolver::setInitStep()) are absolutely
	/// equivalent (and will drop the same errors in the same way, should invalid input be detected).
	/// ## Parameters
	/// * f: Pointer to the function that will be minimized, similarly to the one you submit via
	/// MinProblemSolver::setFunction.
	/// * initStep: Initial step, that will be used to construct the initial simplex, similarly to the one
	/// you submit via MinProblemSolver::setInitStep.
	/// * termcrit: Terminal criteria to the algorithm, similarly to the one you submit via
	/// MinProblemSolver::setTermCriteria.
	/// 
	/// ## C++ default parameters
	/// * f: Ptr<MinProblemSolver::Function>()
	/// * init_step: Mat_<double>(1,1,0.0)
	/// * termcrit: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5000,0.000001)
	pub fn create(f: &core::Ptr::<dyn core::MinProblemSolver_Function>, init_step: &dyn core::ToInputArray, termcrit: core::TermCriteria) -> Result<core::Ptr::<dyn core::DownhillSolver>> {
		input_array_arg!(init_step);
		unsafe { sys::cv_DownhillSolver_create_const_Ptr_Function_R_const__InputArrayR_TermCriteria(f.as_raw_PtrOfMinProblemSolver_Function(), init_step.as_raw__InputArray(), termcrit.opencv_to_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn core::DownhillSolver>::opencv_from_extern(r) } )
	}
	
}
/// ! Class passed to an error.
/// 
/// This class encapsulates all or almost all necessary
/// information about the error happened in the program. The exception is
/// usually constructed and thrown implicitly via CV_Error and CV_Error_ macros.
/// ## See also
/// error
pub trait ExceptionTrait {
	fn as_raw_Exception(&self) -> *const c_void;
	fn as_raw_mut_Exception(&mut self) -> *mut c_void;

	/// the formatted error message
	fn msg(&self) -> String {
		unsafe { sys::cv_Exception_getPropMsg_const(self.as_raw_Exception()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: msg")
	}
	
	/// the formatted error message
	fn set_msg(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_Exception_setPropMsg_String(self.as_raw_mut_Exception(), val.opencv_to_extern_mut()) }.into_result().expect("Infallible function failed: set_msg")
	}
	
	/// error code see also: CVStatus
	fn code(&self) -> i32 {
		unsafe { sys::cv_Exception_getPropCode_const(self.as_raw_Exception()) }.into_result().expect("Infallible function failed: code")
	}
	
	/// error code see also: CVStatus
	fn set_code(&mut self, val: i32) -> () {
		unsafe { sys::cv_Exception_setPropCode_int(self.as_raw_mut_Exception(), val) }.into_result().expect("Infallible function failed: set_code")
	}
	
	/// error description
	fn err(&self) -> String {
		unsafe { sys::cv_Exception_getPropErr_const(self.as_raw_Exception()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: err")
	}
	
	/// error description
	fn set_err(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_Exception_setPropErr_String(self.as_raw_mut_Exception(), val.opencv_to_extern_mut()) }.into_result().expect("Infallible function failed: set_err")
	}
	
	/// function name. Available only when the compiler supports getting it
	fn func(&self) -> String {
		unsafe { sys::cv_Exception_getPropFunc_const(self.as_raw_Exception()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: func")
	}
	
	/// function name. Available only when the compiler supports getting it
	fn set_func(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_Exception_setPropFunc_String(self.as_raw_mut_Exception(), val.opencv_to_extern_mut()) }.into_result().expect("Infallible function failed: set_func")
	}
	
	/// source file name where the error has occurred
	fn file(&self) -> String {
		unsafe { sys::cv_Exception_getPropFile_const(self.as_raw_Exception()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: file")
	}
	
	/// source file name where the error has occurred
	fn set_file(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_Exception_setPropFile_String(self.as_raw_mut_Exception(), val.opencv_to_extern_mut()) }.into_result().expect("Infallible function failed: set_file")
	}
	
	/// line number in the source file where the error has occurred
	fn line(&self) -> i32 {
		unsafe { sys::cv_Exception_getPropLine_const(self.as_raw_Exception()) }.into_result().expect("Infallible function failed: line")
	}
	
	/// line number in the source file where the error has occurred
	fn set_line(&mut self, val: i32) -> () {
		unsafe { sys::cv_Exception_setPropLine_int(self.as_raw_mut_Exception(), val) }.into_result().expect("Infallible function failed: set_line")
	}
	
	/// !
	/// \return the error description and the context as a text string.
	fn what(&self) -> Result<String> {
		unsafe { sys::cv_Exception_what_const(self.as_raw_Exception()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn format_message(&mut self) -> Result<()> {
		unsafe { sys::cv_Exception_formatMessage(self.as_raw_mut_Exception()) }.into_result()
	}
	
}

/// ! Class passed to an error.
/// 
/// This class encapsulates all or almost all necessary
/// information about the error happened in the program. The exception is
/// usually constructed and thrown implicitly via CV_Error and CV_Error_ macros.
/// ## See also
/// error
pub struct Exception {
	ptr: *mut c_void
}

opencv_type_boxed! { Exception }

impl Drop for Exception {
	fn drop(&mut self) {
		extern "C" { fn cv_Exception_delete(instance: *mut c_void); }
		unsafe { cv_Exception_delete(self.as_raw_mut_Exception()) };
	}
}

impl Exception {
	#[inline] pub fn as_raw_Exception(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Exception(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Exception {}

impl core::ExceptionTrait for Exception {
	#[inline] fn as_raw_Exception(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Exception(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Exception {
	/// !
	/// Default constructor
	pub fn default() -> Result<core::Exception> {
		unsafe { sys::cv_Exception_Exception() }.into_result().map(|r| unsafe { core::Exception::opencv_from_extern(r) } )
	}
	
	/// !
	/// Full constructor. Normally the constructor is not called explicitly.
	/// Instead, the macros CV_Error(), CV_Error_() and CV_Assert() are used.
	pub fn new(_code: i32, _err: &str, _func: &str, _file: &str, _line: i32) -> Result<core::Exception> {
		extern_container_arg!(_err);
		extern_container_arg!(_func);
		extern_container_arg!(_file);
		unsafe { sys::cv_Exception_Exception_int_const_StringR_const_StringR_const_StringR_int(_code, _err.opencv_to_extern(), _func.opencv_to_extern(), _file.opencv_to_extern(), _line) }.into_result().map(|r| unsafe { core::Exception::opencv_from_extern(r) } )
	}
	
}

/// File Storage Node class.
/// 
/// The node is used to store each and every element of the file storage opened for reading. When
/// XML/YAML file is read, it is first parsed and stored in the memory as a hierarchical collection of
/// nodes. Each node can be a "leaf" that is contain a single number or a string, or be a collection of
/// other nodes. There can be named collections (mappings) where each element has a name and it is
/// accessed by a name, and ordered collections (sequences) where elements do not have names but rather
/// accessed by index. Type of the file node can be determined using FileNode::type method.
/// 
/// Note that file nodes are only used for navigating file storages opened for reading. When a file
/// storage is opened for writing, no data is stored in memory after it is written.
pub trait FileNodeTrait {
	fn as_raw_FileNode(&self) -> *const c_void;
	fn as_raw_mut_FileNode(&mut self) -> *mut c_void;

	fn fs(&self) -> core::FileStorage {
		unsafe { sys::cv_FileNode_getPropFs_const(self.as_raw_FileNode()) }.into_result().map(|r| unsafe { core::FileStorage::opencv_from_extern(r) } ).expect("Infallible function failed: fs")
	}
	
	fn block_idx(&self) -> size_t {
		unsafe { sys::cv_FileNode_getPropBlockIdx_const(self.as_raw_FileNode()) }.into_result().expect("Infallible function failed: block_idx")
	}
	
	fn set_block_idx(&mut self, val: size_t) -> () {
		unsafe { sys::cv_FileNode_setPropBlockIdx_size_t(self.as_raw_mut_FileNode(), val) }.into_result().expect("Infallible function failed: set_block_idx")
	}
	
	fn ofs(&self) -> size_t {
		unsafe { sys::cv_FileNode_getPropOfs_const(self.as_raw_FileNode()) }.into_result().expect("Infallible function failed: ofs")
	}
	
	fn set_ofs(&mut self, val: size_t) -> () {
		unsafe { sys::cv_FileNode_setPropOfs_size_t(self.as_raw_mut_FileNode(), val) }.into_result().expect("Infallible function failed: set_ofs")
	}
	
	/// Returns element of a mapping node or a sequence node.
	/// ## Parameters
	/// * nodename: Name of an element in the mapping node.
	/// ## Returns
	/// Returns the element with the given identifier.
	fn get(&self, nodename: &str) -> Result<core::FileNode> {
		extern_container_arg!(nodename);
		unsafe { sys::cv_FileNode_operator___const_const_StringR(self.as_raw_FileNode(), nodename.opencv_to_extern()) }.into_result().map(|r| unsafe { core::FileNode::opencv_from_extern(r) } )
	}
	
	/// Returns element of a mapping node or a sequence node.
	/// ## Parameters
	/// * nodename: Name of an element in the mapping node.
	/// ## Returns
	/// Returns the element with the given identifier.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * nodename: Name of an element in the mapping node.
	fn get_node(&self, nodename: &str) -> Result<core::FileNode> {
		extern_container_arg!(nodename);
		unsafe { sys::cv_FileNode_operator___const_const_charX(self.as_raw_FileNode(), nodename.opencv_to_extern()) }.into_result().map(|r| unsafe { core::FileNode::opencv_from_extern(r) } )
	}
	
	/// Returns element of a mapping node or a sequence node.
	/// ## Parameters
	/// * nodename: Name of an element in the mapping node.
	/// ## Returns
	/// Returns the element with the given identifier.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * i: Index of an element in the sequence node.
	fn at(&self, i: i32) -> Result<core::FileNode> {
		unsafe { sys::cv_FileNode_operator___const_int(self.as_raw_FileNode(), i) }.into_result().map(|r| unsafe { core::FileNode::opencv_from_extern(r) } )
	}
	
	/// Returns keys of a mapping node.
	/// ## Returns
	/// Keys of a mapping node.
	fn keys(&self) -> Result<core::Vector::<String>> {
		unsafe { sys::cv_FileNode_keys_const(self.as_raw_FileNode()) }.into_result().map(|r| unsafe { core::Vector::<String>::opencv_from_extern(r) } )
	}
	
	/// Returns type of the node.
	/// ## Returns
	/// Type of the node. See FileNode::Type
	fn typ(&self) -> Result<i32> {
		unsafe { sys::cv_FileNode_type_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns true if the node is empty
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_FileNode_empty_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns true if the node is a "none" object
	fn is_none(&self) -> Result<bool> {
		unsafe { sys::cv_FileNode_isNone_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns true if the node is a sequence
	fn is_seq(&self) -> Result<bool> {
		unsafe { sys::cv_FileNode_isSeq_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns true if the node is a mapping
	fn is_map(&self) -> Result<bool> {
		unsafe { sys::cv_FileNode_isMap_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns true if the node is an integer
	fn is_int(&self) -> Result<bool> {
		unsafe { sys::cv_FileNode_isInt_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns true if the node is a floating-point number
	fn is_real(&self) -> Result<bool> {
		unsafe { sys::cv_FileNode_isReal_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns true if the node is a text string
	fn is_string(&self) -> Result<bool> {
		unsafe { sys::cv_FileNode_isString_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns true if the node has a name
	fn is_named(&self) -> Result<bool> {
		unsafe { sys::cv_FileNode_isNamed_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns the node name or an empty string if the node is nameless
	fn name(&self) -> Result<String> {
		unsafe { sys::cv_FileNode_name_const(self.as_raw_FileNode()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// returns the number of elements in the node, if it is a sequence or mapping, or 1 otherwise.
	fn size(&self) -> Result<size_t> {
		unsafe { sys::cv_FileNode_size_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns raw size of the FileNode in bytes
	fn raw_size(&self) -> Result<size_t> {
		unsafe { sys::cv_FileNode_rawSize_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns the node content as an integer. If the node stores floating-point number, it is rounded.
	fn to_i32(&self) -> Result<i32> {
		unsafe { sys::cv_FileNode_operator_int_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns the node content as float
	fn to_f32(&self) -> Result<f32> {
		unsafe { sys::cv_FileNode_operator_float_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns the node content as double
	fn to_f64(&self) -> Result<f64> {
		unsafe { sys::cv_FileNode_operator_double_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// returns the node content as text string
	fn to_string(&self) -> Result<String> {
		unsafe { sys::cv_FileNode_operator_std_string_const(self.as_raw_FileNode()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn ptr(&mut self) -> Result<&mut u8> {
		unsafe { sys::cv_FileNode_ptr(self.as_raw_mut_FileNode()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	fn ptr_1(&self) -> Result<&u8> {
		unsafe { sys::cv_FileNode_ptr_const(self.as_raw_FileNode()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// returns iterator pointing to the first node element
	fn begin(&self) -> Result<core::FileNodeIterator> {
		unsafe { sys::cv_FileNode_begin_const(self.as_raw_FileNode()) }.into_result().map(|r| unsafe { core::FileNodeIterator::opencv_from_extern(r) } )
	}
	
	/// returns iterator pointing to the element following the last node element
	fn end(&self) -> Result<core::FileNodeIterator> {
		unsafe { sys::cv_FileNode_end_const(self.as_raw_FileNode()) }.into_result().map(|r| unsafe { core::FileNodeIterator::opencv_from_extern(r) } )
	}
	
	/// Reads node elements to the buffer with the specified format.
	/// 
	/// Usually it is more convenient to use operator `>>` instead of this method.
	/// ## Parameters
	/// * fmt: Specification of each array element. See @ref format_spec "format specification"
	/// * vec: Pointer to the destination array.
	/// * len: Number of bytes to read (buffer size limit). If it is greater than number of
	///            remaining elements then all of them will be read.
	fn read_raw(&self, fmt: &str, vec: *mut c_void, len: size_t) -> Result<()> {
		extern_container_arg!(fmt);
		unsafe { sys::cv_FileNode_readRaw_const_const_StringR_voidX_size_t(self.as_raw_FileNode(), fmt.opencv_to_extern(), vec, len) }.into_result()
	}
	
	/// Internal method used when reading FileStorage.
	/// Sets the type (int, real or string) and value of the previously created node.
	/// 
	/// ## C++ default parameters
	/// * len: -1
	fn set_value(&mut self, typ: i32, value: *const c_void, len: i32) -> Result<()> {
		unsafe { sys::cv_FileNode_setValue_int_const_voidX_int(self.as_raw_mut_FileNode(), typ, value, len) }.into_result()
	}
	
	/// Simplified reading API to use with bindings.
	fn real(&self) -> Result<f64> {
		unsafe { sys::cv_FileNode_real_const(self.as_raw_FileNode()) }.into_result()
	}
	
	/// Simplified reading API to use with bindings.
	fn string(&self) -> Result<String> {
		unsafe { sys::cv_FileNode_string_const(self.as_raw_FileNode()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// Simplified reading API to use with bindings.
	fn mat(&self) -> Result<core::Mat> {
		unsafe { sys::cv_FileNode_mat_const(self.as_raw_FileNode()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// File Storage Node class.
/// 
/// The node is used to store each and every element of the file storage opened for reading. When
/// XML/YAML file is read, it is first parsed and stored in the memory as a hierarchical collection of
/// nodes. Each node can be a "leaf" that is contain a single number or a string, or be a collection of
/// other nodes. There can be named collections (mappings) where each element has a name and it is
/// accessed by a name, and ordered collections (sequences) where elements do not have names but rather
/// accessed by index. Type of the file node can be determined using FileNode::type method.
/// 
/// Note that file nodes are only used for navigating file storages opened for reading. When a file
/// storage is opened for writing, no data is stored in memory after it is written.
pub struct FileNode {
	ptr: *mut c_void
}

opencv_type_boxed! { FileNode }

impl Drop for FileNode {
	fn drop(&mut self) {
		extern "C" { fn cv_FileNode_delete(instance: *mut c_void); }
		unsafe { cv_FileNode_delete(self.as_raw_mut_FileNode()) };
	}
}

impl FileNode {
	#[inline] pub fn as_raw_FileNode(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FileNode(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FileNode {}

impl core::FileNodeTrait for FileNode {
	#[inline] fn as_raw_FileNode(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FileNode(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FileNode {
	/// The constructors.
	/// 
	/// These constructors are used to create a default file node, construct it from obsolete structures or
	/// from the another file node.
	pub fn default() -> Result<core::FileNode> {
		unsafe { sys::cv_FileNode_FileNode() }.into_result().map(|r| unsafe { core::FileNode::opencv_from_extern(r) } )
	}
	
	/// The constructors.
	/// 
	/// These constructors are used to create a default file node, construct it from obsolete structures or
	/// from the another file node.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * fs: Pointer to the file storage structure.
	/// * blockIdx: Index of the memory block where the file node is stored
	/// * ofs: Offset in bytes from the beginning of the serialized storage
	pub fn new(fs: &core::FileStorage, block_idx: size_t, ofs: size_t) -> Result<core::FileNode> {
		unsafe { sys::cv_FileNode_FileNode_const_FileStorageX_size_t_size_t(fs.as_raw_FileStorage(), block_idx, ofs) }.into_result().map(|r| unsafe { core::FileNode::opencv_from_extern(r) } )
	}
	
	/// The constructors.
	/// 
	/// These constructors are used to create a default file node, construct it from obsolete structures or
	/// from the another file node.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * node: File node to be used as initialization for the created file node.
	pub fn copy(node: &core::FileNode) -> Result<core::FileNode> {
		unsafe { sys::cv_FileNode_FileNode_const_FileNodeR(node.as_raw_FileNode()) }.into_result().map(|r| unsafe { core::FileNode::opencv_from_extern(r) } )
	}
	
	pub fn is_map(flags: i32) -> Result<bool> {
		unsafe { sys::cv_FileNode_isMap_int(flags) }.into_result()
	}
	
	pub fn is_seq(flags: i32) -> Result<bool> {
		unsafe { sys::cv_FileNode_isSeq_int(flags) }.into_result()
	}
	
	pub fn is_collection(flags: i32) -> Result<bool> {
		unsafe { sys::cv_FileNode_isCollection_int(flags) }.into_result()
	}
	
	pub fn is_empty_collection(flags: i32) -> Result<bool> {
		unsafe { sys::cv_FileNode_isEmptyCollection_int(flags) }.into_result()
	}
	
	pub fn is_flow(flags: i32) -> Result<bool> {
		unsafe { sys::cv_FileNode_isFlow_int(flags) }.into_result()
	}
	
}

/// used to iterate through sequences and mappings.
/// 
/// A standard STL notation, with node.begin(), node.end() denoting the beginning and the end of a
/// sequence, stored in node. See the data reading sample in the beginning of the section.
pub trait FileNodeIteratorTrait {
	fn as_raw_FileNodeIterator(&self) -> *const c_void;
	fn as_raw_mut_FileNodeIterator(&mut self) -> *mut c_void;

	/// returns the currently observed element
	fn try_deref(&self) -> Result<core::FileNode> {
		unsafe { sys::cv_FileNodeIterator_operatorX_const(self.as_raw_FileNodeIterator()) }.into_result().map(|r| unsafe { core::FileNode::opencv_from_extern(r) } )
	}
	
	/// Reads node elements to the buffer with the specified format.
	/// 
	/// Usually it is more convenient to use operator `>>` instead of this method.
	/// ## Parameters
	/// * fmt: Specification of each array element. See @ref format_spec "format specification"
	/// * vec: Pointer to the destination array.
	/// * len: Number of bytes to read (buffer size limit). If it is greater than number of
	///            remaining elements then all of them will be read.
	/// 
	/// ## C++ default parameters
	/// * len: (size_t)INT_MAX
	fn read_raw(&mut self, fmt: &str, vec: *mut c_void, len: size_t) -> Result<core::FileNodeIterator> {
		extern_container_arg!(fmt);
		unsafe { sys::cv_FileNodeIterator_readRaw_const_StringR_voidX_size_t(self.as_raw_mut_FileNodeIterator(), fmt.opencv_to_extern(), vec, len) }.into_result().map(|r| unsafe { core::FileNodeIterator::opencv_from_extern(r) } )
	}
	
	/// returns the number of remaining (not read yet) elements
	fn remaining(&self) -> Result<size_t> {
		unsafe { sys::cv_FileNodeIterator_remaining_const(self.as_raw_FileNodeIterator()) }.into_result()
	}
	
	fn equal_to(&self, it: &core::FileNodeIterator) -> Result<bool> {
		unsafe { sys::cv_FileNodeIterator_equalTo_const_const_FileNodeIteratorR(self.as_raw_FileNodeIterator(), it.as_raw_FileNodeIterator()) }.into_result()
	}
	
}

/// used to iterate through sequences and mappings.
/// 
/// A standard STL notation, with node.begin(), node.end() denoting the beginning and the end of a
/// sequence, stored in node. See the data reading sample in the beginning of the section.
pub struct FileNodeIterator {
	ptr: *mut c_void
}

opencv_type_boxed! { FileNodeIterator }

impl Drop for FileNodeIterator {
	fn drop(&mut self) {
		extern "C" { fn cv_FileNodeIterator_delete(instance: *mut c_void); }
		unsafe { cv_FileNodeIterator_delete(self.as_raw_mut_FileNodeIterator()) };
	}
}

impl FileNodeIterator {
	#[inline] pub fn as_raw_FileNodeIterator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FileNodeIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FileNodeIterator {}

impl core::FileNodeIteratorTrait for FileNodeIterator {
	#[inline] fn as_raw_FileNodeIterator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FileNodeIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FileNodeIterator {
	/// The constructors.
	/// 
	/// These constructors are used to create a default iterator, set it to specific element in a file node
	/// or construct it from another iterator.
	pub fn default() -> Result<core::FileNodeIterator> {
		unsafe { sys::cv_FileNodeIterator_FileNodeIterator() }.into_result().map(|r| unsafe { core::FileNodeIterator::opencv_from_extern(r) } )
	}
	
	/// The constructors.
	/// 
	/// These constructors are used to create a default iterator, set it to specific element in a file node
	/// or construct it from another iterator.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * node: File node - the collection to iterate over;
	///        it can be a scalar (equivalent to 1-element collection) or "none" (equivalent to empty collection).
	/// * seekEnd: - true if iterator needs to be set after the last element of the node;
	///        that is:
	///            * node.begin() => FileNodeIterator(node, false)
	///            * node.end() => FileNodeIterator(node, true)
	pub fn new(node: &core::FileNode, seek_end: bool) -> Result<core::FileNodeIterator> {
		unsafe { sys::cv_FileNodeIterator_FileNodeIterator_const_FileNodeR_bool(node.as_raw_FileNode(), seek_end) }.into_result().map(|r| unsafe { core::FileNodeIterator::opencv_from_extern(r) } )
	}
	
	/// The constructors.
	/// 
	/// These constructors are used to create a default iterator, set it to specific element in a file node
	/// or construct it from another iterator.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * it: Iterator to be used as initialization for the created iterator.
	pub fn copy(it: &core::FileNodeIterator) -> Result<core::FileNodeIterator> {
		unsafe { sys::cv_FileNodeIterator_FileNodeIterator_const_FileNodeIteratorR(it.as_raw_FileNodeIterator()) }.into_result().map(|r| unsafe { core::FileNodeIterator::opencv_from_extern(r) } )
	}
	
}

/// XML/YAML/JSON file storage class that encapsulates all the information necessary for writing or
/// reading data to/from a file.
pub trait FileStorageTrait {
	fn as_raw_FileStorage(&self) -> *const c_void;
	fn as_raw_mut_FileStorage(&mut self) -> *mut c_void;

	fn state(&self) -> i32 {
		unsafe { sys::cv_FileStorage_getPropState_const(self.as_raw_FileStorage()) }.into_result().expect("Infallible function failed: state")
	}
	
	fn set_state(&mut self, val: i32) -> () {
		unsafe { sys::cv_FileStorage_setPropState_int(self.as_raw_mut_FileStorage(), val) }.into_result().expect("Infallible function failed: set_state")
	}
	
	fn elname(&self) -> String {
		unsafe { sys::cv_FileStorage_getPropElname_const(self.as_raw_FileStorage()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: elname")
	}
	
	fn set_elname(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_FileStorage_setPropElname_string(self.as_raw_mut_FileStorage(), val.opencv_to_extern_mut()) }.into_result().expect("Infallible function failed: set_elname")
	}
	
	/// Opens a file.
	/// 
	/// See description of parameters in FileStorage::FileStorage. The method calls FileStorage::release
	/// before opening the file.
	/// ## Parameters
	/// * filename: Name of the file to open or the text string to read the data from.
	/// Extension of the file (.xml, .yml/.yaml or .json) determines its format (XML, YAML or JSON
	/// respectively). Also you can append .gz to work with compressed files, for example myHugeMatrix.xml.gz. If both
	/// FileStorage::WRITE and FileStorage::MEMORY flags are specified, source is used just to specify
	/// the output file format (e.g. mydata.xml, .yml etc.). A file name can also contain parameters.
	/// You can use this format, "*?base64" (e.g. "file.json?base64" (case sensitive)), as an alternative to
	/// FileStorage::BASE64 flag.
	/// * flags: Mode of operation. One of FileStorage::Mode
	/// * encoding: Encoding of the file. Note that UTF-16 XML encoding is not supported currently and
	/// you should use 8-bit encoding instead of it.
	/// 
	/// ## C++ default parameters
	/// * encoding: String()
	fn open(&mut self, filename: &str, flags: i32, encoding: &str) -> Result<bool> {
		extern_container_arg!(filename);
		extern_container_arg!(encoding);
		unsafe { sys::cv_FileStorage_open_const_StringR_int_const_StringR(self.as_raw_mut_FileStorage(), filename.opencv_to_extern(), flags, encoding.opencv_to_extern()) }.into_result()
	}
	
	/// Checks whether the file is opened.
	/// 
	/// ## Returns
	/// true if the object is associated with the current file and false otherwise. It is a
	/// good practice to call this method after you tried to open a file.
	fn is_opened(&self) -> Result<bool> {
		unsafe { sys::cv_FileStorage_isOpened_const(self.as_raw_FileStorage()) }.into_result()
	}
	
	/// Closes the file and releases all the memory buffers.
	/// 
	/// Call this method after all I/O operations with the storage are finished.
	fn release(&mut self) -> Result<()> {
		unsafe { sys::cv_FileStorage_release(self.as_raw_mut_FileStorage()) }.into_result()
	}
	
	/// Closes the file and releases all the memory buffers.
	/// 
	/// Call this method after all I/O operations with the storage are finished. If the storage was
	/// opened for writing data and FileStorage::WRITE was specified
	fn release_and_get_string(&mut self) -> Result<String> {
		unsafe { sys::cv_FileStorage_releaseAndGetString(self.as_raw_mut_FileStorage()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// Returns the first element of the top-level mapping.
	/// ## Returns
	/// The first element of the top-level mapping.
	fn get_first_top_level_node(&self) -> Result<core::FileNode> {
		unsafe { sys::cv_FileStorage_getFirstTopLevelNode_const(self.as_raw_FileStorage()) }.into_result().map(|r| unsafe { core::FileNode::opencv_from_extern(r) } )
	}
	
	/// Returns the top-level mapping
	/// ## Parameters
	/// * streamidx: Zero-based index of the stream. In most cases there is only one stream in the file.
	/// However, YAML supports multiple streams and so there can be several.
	/// ## Returns
	/// The top-level mapping.
	/// 
	/// ## C++ default parameters
	/// * streamidx: 0
	fn root(&self, streamidx: i32) -> Result<core::FileNode> {
		unsafe { sys::cv_FileStorage_root_const_int(self.as_raw_FileStorage(), streamidx) }.into_result().map(|r| unsafe { core::FileNode::opencv_from_extern(r) } )
	}
	
	/// Returns the specified element of the top-level mapping.
	/// ## Parameters
	/// * nodename: Name of the file node.
	/// ## Returns
	/// Node with the given name.
	fn get(&self, nodename: &str) -> Result<core::FileNode> {
		extern_container_arg!(nodename);
		unsafe { sys::cv_FileStorage_operator___const_const_StringR(self.as_raw_FileStorage(), nodename.opencv_to_extern()) }.into_result().map(|r| unsafe { core::FileNode::opencv_from_extern(r) } )
	}
	
	/// Returns the specified element of the top-level mapping.
	/// ## Parameters
	/// * nodename: Name of the file node.
	/// ## Returns
	/// Node with the given name.
	/// 
	/// ## Overloaded parameters
	fn get_node(&self, nodename: &str) -> Result<core::FileNode> {
		extern_container_arg!(nodename);
		unsafe { sys::cv_FileStorage_operator___const_const_charX(self.as_raw_FileStorage(), nodename.opencv_to_extern()) }.into_result().map(|r| unsafe { core::FileNode::opencv_from_extern(r) } )
	}
	
	/// Simplified writing API to use with bindings.
	/// ## Parameters
	/// * name: Name of the written object
	/// * val: Value of the written object
	fn write_i32(&mut self, name: &str, val: i32) -> Result<()> {
		extern_container_arg!(name);
		unsafe { sys::cv_FileStorage_write_const_StringR_int(self.as_raw_mut_FileStorage(), name.opencv_to_extern(), val) }.into_result()
	}
	
	/// Simplified writing API to use with bindings.
	/// ## Parameters
	/// * name: Name of the written object
	/// * val: Value of the written object
	/// 
	/// ## Overloaded parameters
	fn write_f64(&mut self, name: &str, val: f64) -> Result<()> {
		extern_container_arg!(name);
		unsafe { sys::cv_FileStorage_write_const_StringR_double(self.as_raw_mut_FileStorage(), name.opencv_to_extern(), val) }.into_result()
	}
	
	/// Simplified writing API to use with bindings.
	/// ## Parameters
	/// * name: Name of the written object
	/// * val: Value of the written object
	/// 
	/// ## Overloaded parameters
	fn write_str(&mut self, name: &str, val: &str) -> Result<()> {
		extern_container_arg!(name);
		extern_container_arg!(val);
		unsafe { sys::cv_FileStorage_write_const_StringR_const_StringR(self.as_raw_mut_FileStorage(), name.opencv_to_extern(), val.opencv_to_extern()) }.into_result()
	}
	
	/// Simplified writing API to use with bindings.
	/// ## Parameters
	/// * name: Name of the written object
	/// * val: Value of the written object
	/// 
	/// ## Overloaded parameters
	fn write_mat(&mut self, name: &str, val: &core::Mat) -> Result<()> {
		extern_container_arg!(name);
		unsafe { sys::cv_FileStorage_write_const_StringR_const_MatR(self.as_raw_mut_FileStorage(), name.opencv_to_extern(), val.as_raw_Mat()) }.into_result()
	}
	
	/// Simplified writing API to use with bindings.
	/// ## Parameters
	/// * name: Name of the written object
	/// * val: Value of the written object
	/// 
	/// ## Overloaded parameters
	fn write_str_vec(&mut self, name: &str, val: &core::Vector::<String>) -> Result<()> {
		extern_container_arg!(name);
		unsafe { sys::cv_FileStorage_write_const_StringR_const_vector_String_R(self.as_raw_mut_FileStorage(), name.opencv_to_extern(), val.as_raw_VectorOfString()) }.into_result()
	}
	
	/// Writes multiple numbers.
	/// 
	/// Writes one or more numbers of the specified format to the currently written structure. Usually it is
	/// more convenient to use operator `<<` instead of this method.
	/// ## Parameters
	/// * fmt: Specification of each array element, see @ref format_spec "format specification"
	/// * vec: Pointer to the written array.
	/// * len: Number of the uchar elements to write.
	fn write_raw(&mut self, fmt: &str, vec: *const c_void, len: size_t) -> Result<()> {
		extern_container_arg!(fmt);
		unsafe { sys::cv_FileStorage_writeRaw_const_StringR_const_voidX_size_t(self.as_raw_mut_FileStorage(), fmt.opencv_to_extern(), vec, len) }.into_result()
	}
	
	/// Writes a comment.
	/// 
	/// The function writes a comment into file storage. The comments are skipped when the storage is read.
	/// ## Parameters
	/// * comment: The written comment, single-line or multi-line
	/// * append: If true, the function tries to put the comment at the end of current line.
	/// Else if the comment is multi-line, or if it does not fit at the end of the current
	/// line, the comment starts a new line.
	/// 
	/// ## C++ default parameters
	/// * append: false
	fn write_comment(&mut self, comment: &str, append: bool) -> Result<()> {
		extern_container_arg!(comment);
		unsafe { sys::cv_FileStorage_writeComment_const_StringR_bool(self.as_raw_mut_FileStorage(), comment.opencv_to_extern(), append) }.into_result()
	}
	
	fn start_write_struct(&mut self, name: &str, flags: i32, type_name: &str) -> Result<()> {
		extern_container_arg!(name);
		extern_container_arg!(type_name);
		unsafe { sys::cv_FileStorage_startWriteStruct_const_StringR_int_const_StringR(self.as_raw_mut_FileStorage(), name.opencv_to_extern(), flags, type_name.opencv_to_extern()) }.into_result()
	}
	
	fn end_write_struct(&mut self) -> Result<()> {
		unsafe { sys::cv_FileStorage_endWriteStruct(self.as_raw_mut_FileStorage()) }.into_result()
	}
	
	/// Returns the current format.
	/// ## Returns
	/// The current format, see FileStorage::Mode
	fn get_format(&self) -> Result<i32> {
		unsafe { sys::cv_FileStorage_getFormat_const(self.as_raw_FileStorage()) }.into_result()
	}
	
}

/// XML/YAML/JSON file storage class that encapsulates all the information necessary for writing or
/// reading data to/from a file.
pub struct FileStorage {
	ptr: *mut c_void
}

opencv_type_boxed! { FileStorage }

impl Drop for FileStorage {
	fn drop(&mut self) {
		extern "C" { fn cv_FileStorage_delete(instance: *mut c_void); }
		unsafe { cv_FileStorage_delete(self.as_raw_mut_FileStorage()) };
	}
}

impl FileStorage {
	#[inline] pub fn as_raw_FileStorage(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FileStorage(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FileStorage {}

impl core::FileStorageTrait for FileStorage {
	#[inline] fn as_raw_FileStorage(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FileStorage(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FileStorage {
	/// The constructors.
	/// 
	/// The full constructor opens the file. Alternatively you can use the default constructor and then
	/// call FileStorage::open.
	pub fn default() -> Result<core::FileStorage> {
		unsafe { sys::cv_FileStorage_FileStorage() }.into_result().map(|r| unsafe { core::FileStorage::opencv_from_extern(r) } )
	}
	
	/// The constructors.
	/// 
	/// The full constructor opens the file. Alternatively you can use the default constructor and then
	/// call FileStorage::open.
	/// 
	/// ## Overloaded parameters
	/// 
	///      @copydoc open()
	/// 
	/// ## C++ default parameters
	/// * encoding: String()
	pub fn new(filename: &str, flags: i32, encoding: &str) -> Result<core::FileStorage> {
		extern_container_arg!(filename);
		extern_container_arg!(encoding);
		unsafe { sys::cv_FileStorage_FileStorage_const_StringR_int_const_StringR(filename.opencv_to_extern(), flags, encoding.opencv_to_extern()) }.into_result().map(|r| unsafe { core::FileStorage::opencv_from_extern(r) } )
	}
	
	/// Returns the normalized object name for the specified name of a file.
	/// ## Parameters
	/// * filename: Name of a file
	/// ## Returns
	/// The normalized object name.
	pub fn get_default_object_name(filename: &str) -> Result<String> {
		extern_container_arg!(filename);
		unsafe { sys::cv_FileStorage_getDefaultObjectName_const_StringR(filename.opencv_to_extern()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

/// @todo document
pub trait Formatted {
	fn as_raw_Formatted(&self) -> *const c_void;
	fn as_raw_mut_Formatted(&mut self) -> *mut c_void;

	fn next(&mut self) -> Result<String> {
		unsafe { sys::cv_Formatted_next(self.as_raw_mut_Formatted()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn reset(&mut self) -> Result<()> {
		unsafe { sys::cv_Formatted_reset(self.as_raw_mut_Formatted()) }.into_result()
	}
	
}

/// @todo document
pub trait Formatter {
	fn as_raw_Formatter(&self) -> *const c_void;
	fn as_raw_mut_Formatter(&mut self) -> *mut c_void;

	fn format(&self, mtx: &core::Mat) -> Result<core::Ptr::<dyn core::Formatted>> {
		unsafe { sys::cv_Formatter_format_const_const_MatR(self.as_raw_Formatter(), mtx.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Ptr::<dyn core::Formatted>::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * p: 4
	fn set16f_precision(&mut self, p: i32) -> Result<()> {
		unsafe { sys::cv_Formatter_set16fPrecision_int(self.as_raw_mut_Formatter(), p) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * p: 8
	fn set32f_precision(&mut self, p: i32) -> Result<()> {
		unsafe { sys::cv_Formatter_set32fPrecision_int(self.as_raw_mut_Formatter(), p) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * p: 16
	fn set64f_precision(&mut self, p: i32) -> Result<()> {
		unsafe { sys::cv_Formatter_set64fPrecision_int(self.as_raw_mut_Formatter(), p) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * ml: true
	fn set_multiline(&mut self, ml: bool) -> Result<()> {
		unsafe { sys::cv_Formatter_setMultiline_bool(self.as_raw_mut_Formatter(), ml) }.into_result()
	}
	
}

impl dyn Formatter + '_ {
	/// ## C++ default parameters
	/// * fmt: FMT_DEFAULT
	pub fn get(fmt: core::Formatter_FormatType) -> Result<core::Ptr::<dyn core::Formatter>> {
		unsafe { sys::cv_Formatter_get_FormatType(fmt) }.into_result().map(|r| unsafe { core::Ptr::<dyn core::Formatter>::opencv_from_extern(r) } )
	}
	
}
pub trait HammingTrait {
	fn as_raw_Hamming(&self) -> *const c_void;
	fn as_raw_mut_Hamming(&mut self) -> *mut c_void;

}

pub struct Hamming {
	ptr: *mut c_void
}

opencv_type_boxed! { Hamming }

impl Drop for Hamming {
	fn drop(&mut self) {
		extern "C" { fn cv_Hamming_delete(instance: *mut c_void); }
		unsafe { cv_Hamming_delete(self.as_raw_mut_Hamming()) };
	}
}

impl Hamming {
	#[inline] pub fn as_raw_Hamming(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Hamming(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Hamming {}

impl core::HammingTrait for Hamming {
	#[inline] fn as_raw_Hamming(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Hamming(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Hamming {
	pub const normType: u32 = 6;
}

/// Data structure for salient point detectors.
/// 
/// The class instance stores a keypoint, i.e. a point feature found by one of many available keypoint
/// detectors, such as Harris corner detector, #FAST, %StarDetector, %SURF, %SIFT etc.
/// 
/// The keypoint is characterized by the 2D position, scale (proportional to the diameter of the
/// neighborhood that needs to be taken into account), orientation and some other parameters. The
/// keypoint neighborhood is then analyzed by another algorithm that builds a descriptor (usually
/// represented as a feature vector). The keypoints representing the same object in different images
/// can then be matched using %KDTree or another method.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyPoint {
	/// coordinates of the keypoints
	pub pt: core::Point2f,
	/// diameter of the meaningful keypoint neighborhood
	pub size: f32,
	/// computed orientation of the keypoint (-1 if not applicable);
	/// it's in [0,360) degrees and measured relative to
	/// image coordinate system, ie in clockwise.
	pub angle: f32,
	/// the response by which the most strong keypoints have been selected. Can be used for the further sorting or subsampling
	pub response: f32,
	/// octave (pyramid layer) from which the keypoint has been extracted
	pub octave: i32,
	/// object class (if the keypoints need to be clustered by an object they belong to)
	pub class_id: i32,
}

opencv_type_simple! { core::KeyPoint }

impl KeyPoint {
	/// the default constructor
	pub fn default() -> Result<core::KeyPoint> {
		unsafe { sys::cv_KeyPoint_KeyPoint() }.into_result()
	}
	
	/// ## Parameters
	/// * _pt: x & y coordinates of the keypoint
	/// * _size: keypoint diameter
	/// * _angle: keypoint orientation
	/// * _response: keypoint detector response on the keypoint (that is, strength of the keypoint)
	/// * _octave: pyramid octave in which the keypoint has been detected
	/// * _class_id: object id
	/// 
	/// ## C++ default parameters
	/// * _angle: -1
	/// * _response: 0
	/// * _octave: 0
	/// * _class_id: -1
	pub fn new_point(_pt: core::Point2f, _size: f32, _angle: f32, _response: f32, _octave: i32, _class_id: i32) -> Result<core::KeyPoint> {
		unsafe { sys::cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int(_pt.opencv_to_extern(), _size, _angle, _response, _octave, _class_id) }.into_result()
	}
	
	/// ## Parameters
	/// * x: x-coordinate of the keypoint
	/// * y: y-coordinate of the keypoint
	/// * _size: keypoint diameter
	/// * _angle: keypoint orientation
	/// * _response: keypoint detector response on the keypoint (that is, strength of the keypoint)
	/// * _octave: pyramid octave in which the keypoint has been detected
	/// * _class_id: object id
	/// 
	/// ## C++ default parameters
	/// * _angle: -1
	/// * _response: 0
	/// * _octave: 0
	/// * _class_id: -1
	pub fn new_coords(x: f32, y: f32, _size: f32, _angle: f32, _response: f32, _octave: i32, _class_id: i32) -> Result<core::KeyPoint> {
		unsafe { sys::cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int(x, y, _size, _angle, _response, _octave, _class_id) }.into_result()
	}
	
	pub fn hash(self) -> Result<size_t> {
		unsafe { sys::cv_KeyPoint_hash_const(self.opencv_to_extern()) }.into_result()
	}
	
	/// This method converts vector of keypoints to vector of points or the reverse, where each keypoint is
	/// assigned the same size and the same orientation.
	/// 
	/// ## Parameters
	/// * keypoints: Keypoints obtained from any feature detection algorithm like SIFT/SURF/ORB
	/// * points2f: Array of (x,y) coordinates of each keypoint
	/// * keypointIndexes: Array of indexes of keypoints to be converted to points. (Acts like a mask to
	/// convert only specified keypoints)
	/// 
	/// ## C++ default parameters
	/// * keypoint_indexes: std::vector<int>()
	pub fn convert(keypoints: &core::Vector::<core::KeyPoint>, points2f: &mut core::Vector::<core::Point2f>, keypoint_indexes: &core::Vector::<i32>) -> Result<()> {
		unsafe { sys::cv_KeyPoint_convert_const_vector_KeyPoint_R_vector_Point2f_R_const_vector_int_R(keypoints.as_raw_VectorOfKeyPoint(), points2f.as_raw_mut_VectorOfPoint2f(), keypoint_indexes.as_raw_VectorOfi32()) }.into_result()
	}
	
	/// This method converts vector of keypoints to vector of points or the reverse, where each keypoint is
	/// assigned the same size and the same orientation.
	/// 
	/// ## Parameters
	/// * keypoints: Keypoints obtained from any feature detection algorithm like SIFT/SURF/ORB
	/// * points2f: Array of (x,y) coordinates of each keypoint
	/// * keypointIndexes: Array of indexes of keypoints to be converted to points. (Acts like a mask to
	/// convert only specified keypoints)
	/// 
	/// ## Overloaded parameters
	/// 
	/// * points2f: Array of (x,y) coordinates of each keypoint
	/// * keypoints: Keypoints obtained from any feature detection algorithm like SIFT/SURF/ORB
	/// * size: keypoint diameter
	/// * response: keypoint detector response on the keypoint (that is, strength of the keypoint)
	/// * octave: pyramid octave in which the keypoint has been detected
	/// * class_id: object id
	/// 
	/// ## C++ default parameters
	/// * size: 1
	/// * response: 1
	/// * octave: 0
	/// * class_id: -1
	pub fn convert_to(points2f: &core::Vector::<core::Point2f>, keypoints: &mut core::Vector::<core::KeyPoint>, size: f32, response: f32, octave: i32, class_id: i32) -> Result<()> {
		unsafe { sys::cv_KeyPoint_convert_const_vector_Point2f_R_vector_KeyPoint_R_float_float_int_int(points2f.as_raw_VectorOfPoint2f(), keypoints.as_raw_mut_VectorOfKeyPoint(), size, response, octave, class_id) }.into_result()
	}
	
	/// This method computes overlap for pair of keypoints. Overlap is the ratio between area of keypoint
	/// regions' intersection and area of keypoint regions' union (considering keypoint region as circle).
	/// If they don't overlap, we get zero. If they coincide at same location with same size, we get 1.
	/// ## Parameters
	/// * kp1: First keypoint
	/// * kp2: Second keypoint
	pub fn overlap(kp1: core::KeyPoint, kp2: core::KeyPoint) -> Result<f32> {
		unsafe { sys::cv_KeyPoint_overlap_const_KeyPointR_const_KeyPointR(&kp1, &kp2) }.into_result()
	}
	
}

/// Linear Discriminant Analysis
/// @todo document this class
pub trait LDATrait {
	fn as_raw_LDA(&self) -> *const c_void;
	fn as_raw_mut_LDA(&mut self) -> *mut c_void;

	/// Serializes this object to a given filename.
	fn save(&self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		unsafe { sys::cv_LDA_save_const_const_StringR(self.as_raw_LDA(), filename.opencv_to_extern()) }.into_result()
	}
	
	/// Deserializes this object from a given filename.
	fn load(&mut self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		unsafe { sys::cv_LDA_load_const_StringR(self.as_raw_mut_LDA(), filename.opencv_to_extern()) }.into_result()
	}
	
	/// Serializes this object to a given cv::FileStorage.
	fn save_1(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_LDA_save_const_FileStorageR(self.as_raw_LDA(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	/// Deserializes this object from a given cv::FileStorage.
	fn load_1(&mut self, node: &core::FileStorage) -> Result<()> {
		unsafe { sys::cv_LDA_load_const_FileStorageR(self.as_raw_mut_LDA(), node.as_raw_FileStorage()) }.into_result()
	}
	
	/// Compute the discriminants for data in src (row aligned) and labels.
	fn compute(&mut self, src: &dyn core::ToInputArray, labels: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(labels);
		unsafe { sys::cv_LDA_compute_const__InputArrayR_const__InputArrayR(self.as_raw_mut_LDA(), src.as_raw__InputArray(), labels.as_raw__InputArray()) }.into_result()
	}
	
	/// Projects samples into the LDA subspace.
	/// src may be one or more row aligned samples.
	fn project(&mut self, src: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(src);
		unsafe { sys::cv_LDA_project_const__InputArrayR(self.as_raw_mut_LDA(), src.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Reconstructs projections from the LDA subspace.
	/// src may be one or more row aligned projections.
	fn reconstruct(&mut self, src: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(src);
		unsafe { sys::cv_LDA_reconstruct_const__InputArrayR(self.as_raw_mut_LDA(), src.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Returns the eigenvectors of this LDA.
	fn eigenvectors(&self) -> Result<core::Mat> {
		unsafe { sys::cv_LDA_eigenvectors_const(self.as_raw_LDA()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Returns the eigenvalues of this LDA.
	fn eigenvalues(&self) -> Result<core::Mat> {
		unsafe { sys::cv_LDA_eigenvalues_const(self.as_raw_LDA()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Linear Discriminant Analysis
/// @todo document this class
pub struct LDA {
	ptr: *mut c_void
}

opencv_type_boxed! { LDA }

impl Drop for LDA {
	fn drop(&mut self) {
		extern "C" { fn cv_LDA_delete(instance: *mut c_void); }
		unsafe { cv_LDA_delete(self.as_raw_mut_LDA()) };
	}
}

impl LDA {
	#[inline] pub fn as_raw_LDA(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_LDA(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for LDA {}

impl core::LDATrait for LDA {
	#[inline] fn as_raw_LDA(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_LDA(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LDA {
	/// constructor
	/// Initializes a LDA with num_components (default 0).
	/// 
	/// ## C++ default parameters
	/// * num_components: 0
	pub fn new(num_components: i32) -> Result<core::LDA> {
		unsafe { sys::cv_LDA_LDA_int(num_components) }.into_result().map(|r| unsafe { core::LDA::opencv_from_extern(r) } )
	}
	
	/// Initializes and performs a Discriminant Analysis with Fisher's
	/// Optimization Criterion on given data in src and corresponding labels
	/// in labels. If 0 (or less) number of components are given, they are
	/// automatically determined for given data in computation.
	/// 
	/// ## C++ default parameters
	/// * num_components: 0
	pub fn new_with_data(src: &dyn core::ToInputArray, labels: &dyn core::ToInputArray, num_components: i32) -> Result<core::LDA> {
		input_array_arg!(src);
		input_array_arg!(labels);
		unsafe { sys::cv_LDA_LDA_const__InputArrayR_const__InputArrayR_int(src.as_raw__InputArray(), labels.as_raw__InputArray(), num_components) }.into_result().map(|r| unsafe { core::LDA::opencv_from_extern(r) } )
	}
	
	pub fn subspace_project(w: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, src: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(w);
		input_array_arg!(mean);
		input_array_arg!(src);
		unsafe { sys::cv_LDA_subspaceProject_const__InputArrayR_const__InputArrayR_const__InputArrayR(w.as_raw__InputArray(), mean.as_raw__InputArray(), src.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	pub fn subspace_reconstruct(w: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, src: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(w);
		input_array_arg!(mean);
		input_array_arg!(src);
		unsafe { sys::cv_LDA_subspaceReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR(w.as_raw__InputArray(), mean.as_raw__InputArray(), src.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// n-dimensional dense array class \anchor CVMat_Details
/// 
/// The class Mat represents an n-dimensional dense numerical single-channel or multi-channel array. It
/// can be used to store real or complex-valued vectors and matrices, grayscale or color images, voxel
/// volumes, vector fields, point clouds, tensors, histograms (though, very high-dimensional histograms
/// may be better stored in a SparseMat ). The data layout of the array `M` is defined by the array
/// `M.step[]`, so that the address of element ![inline formula](https://latex.codecogs.com/png.latex?%28i%5F0%2C%2E%2E%2E%2Ci%5F%7BM%2Edims%2D1%7D%29), where ![inline formula](https://latex.codecogs.com/png.latex?0%5Cleq%20i%5Fk%3CM%2Esize%5Bk%5D), is
/// computed as:
/// ![block formula](https://latex.codecogs.com/png.latex?addr%28M%5F%7Bi%5F0%2C%2E%2E%2E%2Ci%5F%7BM%2Edims%2D1%7D%7D%29%20%3D%20M%2Edata%20%2B%20M%2Estep%5B0%5D%2Ai%5F0%20%2B%20M%2Estep%5B1%5D%2Ai%5F1%20%2B%20%2E%2E%2E%20%2B%20M%2Estep%5BM%2Edims%2D1%5D%2Ai%5F%7BM%2Edims%2D1%7D)
/// In case of a 2-dimensional array, the above formula is reduced to:
/// ![block formula](https://latex.codecogs.com/png.latex?addr%28M%5F%7Bi%2Cj%7D%29%20%3D%20M%2Edata%20%2B%20M%2Estep%5B0%5D%2Ai%20%2B%20M%2Estep%5B1%5D%2Aj)
/// Note that `M.step[i] >= M.step[i+1]` (in fact, `M.step[i] >= M.step[i+1]*M.size[i+1]` ). This means
/// that 2-dimensional matrices are stored row-by-row, 3-dimensional matrices are stored plane-by-plane,
/// and so on. M.step[M.dims-1] is minimal and always equal to the element size M.elemSize() .
/// 
/// So, the data layout in Mat is compatible with the majority of dense array types from the standard
/// toolkits and SDKs, such as Numpy (ndarray), Win32 (independent device bitmaps), and others,
/// that is, with any array that uses *steps* (or *strides*) to compute the position of a pixel.
/// Due to this compatibility, it is possible to make a Mat header for user-allocated data and process
/// it in-place using OpenCV functions.
/// 
/// There are many different ways to create a Mat object. The most popular options are listed below:
/// 
/// - Use the create(nrows, ncols, type) method or the similar Mat(nrows, ncols, type[, fillValue])
/// constructor. A new array of the specified size and type is allocated. type has the same meaning as
/// in the cvCreateMat method. For example, CV_8UC1 means a 8-bit single-channel array, CV_32FC2
/// means a 2-channel (complex) floating-point array, and so on.
/// ```ignore
///    // make a 7x7 complex matrix filled with 1+3j.
///    Mat M(7,7,CV_32FC2,Scalar(1,3));
///    // and now turn M to a 100x60 15-channel 8-bit matrix.
///    // The old content will be deallocated
///    M.create(100,60,CV_8UC(15));
/// ```
/// 
/// As noted in the introduction to this chapter, create() allocates only a new array when the shape
/// or type of the current array are different from the specified ones.
/// 
/// - Create a multi-dimensional array:
/// ```ignore
///    // create a 100x100x100 8-bit array
///    int sz[] = {100, 100, 100};
///    Mat bigCube(3, sz, CV_8U, Scalar::all(0));
/// ```
/// 
/// It passes the number of dimensions =1 to the Mat constructor but the created array will be
/// 2-dimensional with the number of columns set to 1. So, Mat::dims is always \>= 2 (can also be 0
/// when the array is empty).
/// 
/// - Use a copy constructor or assignment operator where there can be an array or expression on the
/// right side (see below). As noted in the introduction, the array assignment is an O(1) operation
/// because it only copies the header and increases the reference counter. The Mat::clone() method can
/// be used to get a full (deep) copy of the array when you need it.
/// 
/// - Construct a header for a part of another array. It can be a single row, single column, several
/// rows, several columns, rectangular region in the array (called a *minor* in algebra) or a
/// diagonal. Such operations are also O(1) because the new header references the same data. You can
/// actually modify a part of the array using this feature, for example:
/// ```ignore
///    // add the 5-th row, multiplied by 3 to the 3rd row
///    M.row(3) = M.row(3) + M.row(5)*3;
///    // now copy the 7-th column to the 1-st column
///    // M.col(1) = M.col(7); // this will not work
///    Mat M1 = M.col(1);
///    M.col(7).copyTo(M1);
///    // create a new 320x240 image
///    Mat img(Size(320,240),CV_8UC3);
///    // select a ROI
///    Mat roi(img, Rect(10,10,100,100));
///    // fill the ROI with (0,255,0) (which is green in RGB space);
///    // the original 320x240 image will be modified
///    roi = Scalar(0,255,0);
/// ```
/// 
/// Due to the additional datastart and dataend members, it is possible to compute a relative
/// sub-array position in the main *container* array using locateROI():
/// ```ignore
///    Mat A = Mat::eye(10, 10, CV_32S);
///    // extracts A columns, 1 (inclusive) to 3 (exclusive).
///    Mat B = A(Range::all(), Range(1, 3));
///    // extracts B rows, 5 (inclusive) to 9 (exclusive).
///    // that is, C \~ A(Range(5, 9), Range(1, 3))
///    Mat C = B(Range(5, 9), Range::all());
///    Size size; Point ofs;
///    C.locateROI(size, ofs);
///    // size will be (width=10,height=10) and the ofs will be (x=1, y=5)
/// ```
/// 
/// As in case of whole matrices, if you need a deep copy, use the `clone()` method of the extracted
/// sub-matrices.
/// 
/// - Make a header for user-allocated data. It can be useful to do the following:
///    -# Process "foreign" data using OpenCV (for example, when you implement a DirectShow\* filter or
///    a processing module for gstreamer, and so on). For example:
///    ```ignore
///        void process_video_frame(const unsigned char* pixels,
///                                  int width, int height, int step)
///        {
///            Mat img(height, width, CV_8UC3, pixels, step);
///            GaussianBlur(img, img, Size(7,7), 1.5, 1.5);
///        }
///    ```
/// 
///    -# Quickly initialize small matrices and/or get a super-fast element access.
///    ```ignore
///        double m[3][3] = {{a, b, c}, {d, e, f}, {g, h, i}};
///        Mat M = Mat(3, 3, CV_64F, m).inv();
///    ```
/// 
///    .
/// 
/// - Use MATLAB-style array initializers, zeros(), ones(), eye(), for example:
/// ```ignore
///    // create a double-precision identity matrix and add it to M.
///    M += Mat::eye(M.rows, M.cols, CV_64F);
/// ```
/// 
/// 
/// - Use a comma-separated initializer:
/// ```ignore
///    // create a 3x3 double-precision identity matrix
///    Mat M = (Mat_<double>(3,3) << 1, 0, 0, 0, 1, 0, 0, 0, 1);
/// ```
/// 
/// With this approach, you first call a constructor of the Mat class with the proper parameters, and
/// then you just put `<< operator` followed by comma-separated values that can be constants,
/// variables, expressions, and so on. Also, note the extra parentheses required to avoid compilation
/// errors.
/// 
/// Once the array is created, it is automatically managed via a reference-counting mechanism. If the
/// array header is built on top of user-allocated data, you should handle the data by yourself. The
/// array data is deallocated when no one points to it. If you want to release the data pointed by a
/// array header before the array destructor is called, use Mat::release().
/// 
/// The next important thing to learn about the array class is element access. This manual already
/// described how to compute an address of each array element. Normally, you are not required to use the
/// formula directly in the code. If you know the array element type (which can be retrieved using the
/// method Mat::type() ), you can access the element ![inline formula](https://latex.codecogs.com/png.latex?M%5F%7Bij%7D) of a 2-dimensional array as:
/// ```ignore
///    M.at<double>(i,j) += 1.f;
/// ```
/// 
/// assuming that `M` is a double-precision floating-point array. There are several variants of the method
/// at for a different number of dimensions.
/// 
/// If you need to process a whole row of a 2D array, the most efficient way is to get the pointer to
/// the row first, and then just use the plain C operator [] :
/// ```ignore
///    // compute sum of positive matrix elements
///    // (assuming that M is a double-precision matrix)
///    double sum=0;
///    for(int i = 0; i < M.rows; i++)
///    {
///        const double* Mi = M.ptr<double>(i);
///        for(int j = 0; j < M.cols; j++)
///            sum += std::max(Mi[j], 0.);
///    }
/// ```
/// 
/// Some operations, like the one above, do not actually depend on the array shape. They just process
/// elements of an array one by one (or elements from multiple arrays that have the same coordinates,
/// for example, array addition). Such operations are called *element-wise*. It makes sense to check
/// whether all the input/output arrays are continuous, namely, have no gaps at the end of each row. If
/// yes, process them as a long single row:
/// ```ignore
///    // compute the sum of positive matrix elements, optimized variant
///    double sum=0;
///    int cols = M.cols, rows = M.rows;
///    if(M.isContinuous())
///    {
///        cols *= rows;
///        rows = 1;
///    }
///    for(int i = 0; i < rows; i++)
///    {
///        const double* Mi = M.ptr<double>(i);
///        for(int j = 0; j < cols; j++)
///            sum += std::max(Mi[j], 0.);
///    }
/// ```
/// 
/// In case of the continuous matrix, the outer loop body is executed just once. So, the overhead is
/// smaller, which is especially noticeable in case of small matrices.
/// 
/// Finally, there are STL-style iterators that are smart enough to skip gaps between successive rows:
/// ```ignore
///    // compute sum of positive matrix elements, iterator-based variant
///    double sum=0;
///    MatConstIterator_<double> it = M.begin<double>(), it_end = M.end<double>();
///    for(; it != it_end; ++it)
///        sum += std::max(*it, 0.);
/// ```
/// 
/// The matrix iterators are random-access iterators, so they can be passed to any STL algorithm,
/// including std::sort().
/// 
/// 
/// Note: Matrix Expressions and arithmetic see MatExpr
pub trait MatTrait {
	fn as_raw_Mat(&self) -> *const c_void;
	fn as_raw_mut_Mat(&mut self) -> *mut c_void;

	/// ! includes several bit-fields:
	/// - the magic signature
	/// - continuity flag
	/// - depth
	/// - number of channels
	fn flags(&self) -> i32 {
		unsafe { sys::cv_Mat_getPropFlags_const(self.as_raw_Mat()) }.into_result().expect("Infallible function failed: flags")
	}
	
	/// ! includes several bit-fields:
	/// - the magic signature
	/// - continuity flag
	/// - depth
	/// - number of channels
	fn set_flags(&mut self, val: i32) -> () {
		unsafe { sys::cv_Mat_setPropFlags_int(self.as_raw_mut_Mat(), val) }.into_result().expect("Infallible function failed: set_flags")
	}
	
	/// the matrix dimensionality, >= 2
	fn dims(&self) -> i32 {
		unsafe { sys::cv_Mat_getPropDims_const(self.as_raw_Mat()) }.into_result().expect("Infallible function failed: dims")
	}
	
	/// the matrix dimensionality, >= 2
	fn set_dims(&mut self, val: i32) -> () {
		unsafe { sys::cv_Mat_setPropDims_int(self.as_raw_mut_Mat(), val) }.into_result().expect("Infallible function failed: set_dims")
	}
	
	/// the number of rows and columns or (-1, -1) when the matrix has more than 2 dimensions
	fn rows(&self) -> i32 {
		unsafe { sys::cv_Mat_getPropRows_const(self.as_raw_Mat()) }.into_result().expect("Infallible function failed: rows")
	}
	
	/// the number of rows and columns or (-1, -1) when the matrix has more than 2 dimensions
	fn set_rows(&mut self, val: i32) -> () {
		unsafe { sys::cv_Mat_setPropRows_int(self.as_raw_mut_Mat(), val) }.into_result().expect("Infallible function failed: set_rows")
	}
	
	/// the number of rows and columns or (-1, -1) when the matrix has more than 2 dimensions
	fn cols(&self) -> i32 {
		unsafe { sys::cv_Mat_getPropCols_const(self.as_raw_Mat()) }.into_result().expect("Infallible function failed: cols")
	}
	
	/// the number of rows and columns or (-1, -1) when the matrix has more than 2 dimensions
	fn set_cols(&mut self, val: i32) -> () {
		unsafe { sys::cv_Mat_setPropCols_int(self.as_raw_mut_Mat(), val) }.into_result().expect("Infallible function failed: set_cols")
	}
	
	/// pointer to the data
	fn data_mut(&mut self) -> &mut u8 {
		unsafe { sys::cv_Mat_getPropData(self.as_raw_mut_Mat()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: data_mut")
	}
	
	/// pointer to the data
	unsafe fn set_data(&mut self, val: &mut u8) -> () {
		{ sys::cv_Mat_setPropData_unsigned_charX(self.as_raw_mut_Mat(), val) }.into_result().expect("Infallible function failed: set_data")
	}
	
	/// helper fields used in locateROI and adjustROI
	fn datastart(&self) -> &u8 {
		unsafe { sys::cv_Mat_getPropDatastart_const(self.as_raw_Mat()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: datastart")
	}
	
	fn dataend(&self) -> &u8 {
		unsafe { sys::cv_Mat_getPropDataend_const(self.as_raw_Mat()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: dataend")
	}
	
	fn datalimit(&self) -> &u8 {
		unsafe { sys::cv_Mat_getPropDatalimit_const(self.as_raw_Mat()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: datalimit")
	}
	
	/// interaction with UMat
	fn u(&mut self) -> core::UMatData {
		unsafe { sys::cv_Mat_getPropU(self.as_raw_mut_Mat()) }.into_result().map(|r| unsafe { core::UMatData::opencv_from_extern(r) } ).expect("Infallible function failed: u")
	}
	
	/// interaction with UMat
	fn set_u(&mut self, val: &mut core::UMatData) -> () {
		unsafe { sys::cv_Mat_setPropU_UMatDataX(self.as_raw_mut_Mat(), val.as_raw_mut_UMatData()) }.into_result().expect("Infallible function failed: set_u")
	}
	
	fn mat_size(&self) -> core::MatSize {
		unsafe { sys::cv_Mat_getPropSize_const(self.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatSize::opencv_from_extern(r) } ).expect("Infallible function failed: mat_size")
	}
	
	fn mat_step(&self) -> core::MatStep {
		unsafe { sys::cv_Mat_getPropStep_const(self.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatStep::opencv_from_extern(r) } ).expect("Infallible function failed: mat_step")
	}
	
	/// retrieve UMat from Mat
	/// 
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	fn get_umat(&self, access_flags: core::AccessFlag, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		unsafe { sys::cv_Mat_getUMat_const_AccessFlag_UMatUsageFlags(self.as_raw_Mat(), access_flags, usage_flags) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// Creates a matrix header for the specified matrix row.
	/// 
	/// The method makes a new header for the specified matrix row and returns it. This is an O(1)
	/// operation, regardless of the matrix size. The underlying data of the new matrix is shared with the
	/// original matrix. Here is the example of one of the classical basic matrix processing operations,
	/// axpy, used by LU and many other algorithms:
	/// ```ignore
	///    inline void matrix_axpy(Mat& A, int i, int j, double alpha)
	///    {
	///        A.row(i) += A.row(j)*alpha;
	///    }
	/// ```
	/// 
	/// 
	/// Note: In the current implementation, the following code does not work as expected:
	/// ```ignore
	///    Mat A;
	///    ...
	///    A.row(i) = A.row(j); // will not work
	/// ```
	/// 
	/// This happens because A.row(i) forms a temporary header that is further assigned to another header.
	/// Remember that each of these operations is O(1), that is, no data is copied. Thus, the above
	/// assignment is not true if you may have expected the j-th row to be copied to the i-th row. To
	/// achieve that, you should either turn this simple assignment into an expression or use the
	/// Mat::copyTo method:
	/// ```ignore
	///    Mat A;
	///    ...
	///    // works, but looks a bit obscure.
	///    A.row(i) = A.row(j) + 0;
	///    // this is a bit longer, but the recommended method.
	///    A.row(j).copyTo(A.row(i));
	/// ```
	/// 
	/// ## Parameters
	/// * y: A 0-based row index.
	fn row(&self, y: i32) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_row_const_int(self.as_raw_Mat(), y) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Creates a matrix header for the specified matrix column.
	/// 
	/// The method makes a new header for the specified matrix column and returns it. This is an O(1)
	/// operation, regardless of the matrix size. The underlying data of the new matrix is shared with the
	/// original matrix. See also the Mat::row description.
	/// ## Parameters
	/// * x: A 0-based column index.
	fn col(&self, x: i32) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_col_const_int(self.as_raw_Mat(), x) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Creates a matrix header for the specified row span.
	/// 
	/// The method makes a new header for the specified row span of the matrix. Similarly to Mat::row and
	/// Mat::col , this is an O(1) operation.
	/// ## Parameters
	/// * startrow: An inclusive 0-based start index of the row span.
	/// * endrow: An exclusive 0-based ending index of the row span.
	fn row_bounds(&self, startrow: i32, endrow: i32) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_rowRange_const_int_int(self.as_raw_Mat(), startrow, endrow) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Creates a matrix header for the specified row span.
	/// 
	/// The method makes a new header for the specified row span of the matrix. Similarly to Mat::row and
	/// Mat::col , this is an O(1) operation.
	/// ## Parameters
	/// * startrow: An inclusive 0-based start index of the row span.
	/// * endrow: An exclusive 0-based ending index of the row span.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * r: Range structure containing both the start and the end indices.
	fn row_range(&self, r: &core::Range) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_rowRange_const_const_RangeR(self.as_raw_Mat(), r.as_raw_Range()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Creates a matrix header for the specified column span.
	/// 
	/// The method makes a new header for the specified column span of the matrix. Similarly to Mat::row and
	/// Mat::col , this is an O(1) operation.
	/// ## Parameters
	/// * startcol: An inclusive 0-based start index of the column span.
	/// * endcol: An exclusive 0-based ending index of the column span.
	fn col_bounds(&self, startcol: i32, endcol: i32) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_colRange_const_int_int(self.as_raw_Mat(), startcol, endcol) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Creates a matrix header for the specified column span.
	/// 
	/// The method makes a new header for the specified column span of the matrix. Similarly to Mat::row and
	/// Mat::col , this is an O(1) operation.
	/// ## Parameters
	/// * startcol: An inclusive 0-based start index of the column span.
	/// * endcol: An exclusive 0-based ending index of the column span.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * r: Range structure containing both the start and the end indices.
	fn col_range(&self, r: &core::Range) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_colRange_const_const_RangeR(self.as_raw_Mat(), r.as_raw_Range()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Extracts a diagonal from a matrix
	/// 
	/// The method makes a new header for the specified matrix diagonal. The new matrix is represented as a
	/// single-column matrix. Similarly to Mat::row and Mat::col, this is an O(1) operation.
	/// ## Parameters
	/// * d: index of the diagonal, with the following values:
	/// - `d=0` is the main diagonal.
	/// - `d<0` is a diagonal from the lower half. For example, d=-1 means the diagonal is set
	///   immediately below the main one.
	/// - `d>0` is a diagonal from the upper half. For example, d=1 means the diagonal is set
	///   immediately above the main one.
	/// For example:
	/// ```ignore
	///    Mat m = (Mat_<int>(3,3) <<
	///                1,2,3,
	///                4,5,6,
	///                7,8,9);
	///    Mat d0 = m.diag(0);
	///    Mat d1 = m.diag(1);
	///    Mat d_1 = m.diag(-1);
	/// ```
	/// 
	/// The resulting matrices are
	/// ```ignore
	///  d0 =
	///    [1;
	///    5;
	///    9]
	///  d1 =
	///    [2;
	///    6]
	///  d_1 =
	///    [4;
	///    8]
	/// ```
	/// 
	/// 
	/// ## C++ default parameters
	/// * d: 0
	fn diag(&self, d: i32) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_diag_const_int(self.as_raw_Mat(), d) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Creates a full copy of the array and the underlying data.
	/// 
	/// The method creates a full copy of the array. The original step[] is not taken into account. So, the
	/// array copy is a continuous array occupying total()*elemSize() bytes.
	fn clone(&self) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_clone_const(self.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Copies the matrix to another one.
	/// 
	/// The method copies the matrix data to another matrix. Before copying the data, the method invokes :
	/// ```ignore
	///    m.create(this->size(), this->type());
	/// ```
	/// 
	/// so that the destination matrix is reallocated if needed. While m.copyTo(m); works flawlessly, the
	/// function does not handle the case of a partial overlap between the source and the destination
	/// matrices.
	/// 
	/// When the operation mask is specified, if the Mat::create call shown above reallocates the matrix,
	/// the newly allocated matrix is initialized with all zeros before copying the data.
	/// ## Parameters
	/// * m: Destination matrix. If it does not have a proper size or type before the operation, it is
	/// reallocated.
	fn copy_to(&self, m: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(m);
		unsafe { sys::cv_Mat_copyTo_const_const__OutputArrayR(self.as_raw_Mat(), m.as_raw__OutputArray()) }.into_result()
	}
	
	/// Copies the matrix to another one.
	/// 
	/// The method copies the matrix data to another matrix. Before copying the data, the method invokes :
	/// ```ignore
	///    m.create(this->size(), this->type());
	/// ```
	/// 
	/// so that the destination matrix is reallocated if needed. While m.copyTo(m); works flawlessly, the
	/// function does not handle the case of a partial overlap between the source and the destination
	/// matrices.
	/// 
	/// When the operation mask is specified, if the Mat::create call shown above reallocates the matrix,
	/// the newly allocated matrix is initialized with all zeros before copying the data.
	/// ## Parameters
	/// * m: Destination matrix. If it does not have a proper size or type before the operation, it is
	/// reallocated.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * m: Destination matrix. If it does not have a proper size or type before the operation, it is
	///    reallocated.
	/// * mask: Operation mask of the same size as \*this. Its non-zero elements indicate which matrix
	///    elements need to be copied. The mask has to be of type CV_8U and can have 1 or multiple channels.
	fn copy_to_masked(&self, m: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		output_array_arg!(m);
		input_array_arg!(mask);
		unsafe { sys::cv_Mat_copyTo_const_const__OutputArrayR_const__InputArrayR(self.as_raw_Mat(), m.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
	}
	
	/// Converts an array to another data type with optional scaling.
	/// 
	/// The method converts source pixel values to the target data type. saturate_cast\<\> is applied at
	/// the end to avoid possible overflows:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?m%28x%2Cy%29%20%3D%20saturate%20%5C%5F%20cast%3CrType%3E%28%20%5Calpha%20%28%2Athis%29%28x%2Cy%29%20%2B%20%20%5Cbeta%20%29)
	/// ## Parameters
	/// * m: output matrix; if it does not have a proper size or type before the operation, it is
	/// reallocated.
	/// * rtype: desired output matrix type or, rather, the depth since the number of channels are the
	/// same as the input has; if rtype is negative, the output matrix will have the same type as the input.
	/// * alpha: optional scale factor.
	/// * beta: optional delta added to the scaled values.
	/// 
	/// ## C++ default parameters
	/// * alpha: 1
	/// * beta: 0
	fn convert_to(&self, m: &mut dyn core::ToOutputArray, rtype: i32, alpha: f64, beta: f64) -> Result<()> {
		output_array_arg!(m);
		unsafe { sys::cv_Mat_convertTo_const_const__OutputArrayR_int_double_double(self.as_raw_Mat(), m.as_raw__OutputArray(), rtype, alpha, beta) }.into_result()
	}
	
	/// Provides a functional form of convertTo.
	/// 
	/// This is an internally used method called by the @ref MatrixExpressions engine.
	/// ## Parameters
	/// * m: Destination array.
	/// * type: Desired destination array depth (or -1 if it should be the same as the source type).
	/// 
	/// ## C++ default parameters
	/// * typ: -1
	fn assign_to(&self, m: &mut core::Mat, typ: i32) -> Result<()> {
		unsafe { sys::cv_Mat_assignTo_const_MatR_int(self.as_raw_Mat(), m.as_raw_mut_Mat(), typ) }.into_result()
	}
	
	/// Sets all or some of the array elements to the specified value.
	/// 
	/// This is an advanced variant of the Mat::operator=(const Scalar& s) operator.
	/// ## Parameters
	/// * value: Assigned scalar converted to the actual array type.
	/// * mask: Operation mask of the same size as \*this. Its non-zero elements indicate which matrix
	/// elements need to be copied. The mask has to be of type CV_8U and can have 1 or multiple channels
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	fn set_to(&mut self, value: &dyn core::ToInputArray, mask: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(value);
		input_array_arg!(mask);
		unsafe { sys::cv_Mat_setTo_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Mat(), value.as_raw__InputArray(), mask.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Changes the shape and/or the number of channels of a 2D matrix without copying the data.
	/// 
	/// The method makes a new matrix header for \*this elements. The new matrix may have a different size
	/// and/or different number of channels. Any combination is possible if:
	/// *   No extra elements are included into the new matrix and no elements are excluded. Consequently,
	///    the product rows\*cols\*channels() must stay the same after the transformation.
	/// *   No data is copied. That is, this is an O(1) operation. Consequently, if you change the number of
	///    rows, or the operation changes the indices of elements row in some other way, the matrix must be
	///    continuous. See Mat::isContinuous .
	/// 
	/// For example, if there is a set of 3D points stored as an STL vector, and you want to represent the
	/// points as a 3xN matrix, do the following:
	/// ```ignore
	///    std::vector<Point3f> vec;
	///    ...
	///    Mat pointMat = Mat(vec). // convert vector to Mat, O(1) operation
	///                       reshape(1). // make Nx3 1-channel matrix out of Nx1 3-channel.
	///                                   // Also, an O(1) operation
	///                          t(); // finally, transpose the Nx3 matrix.
	///                               // This involves copying all the elements
	/// ```
	/// 
	/// ## Parameters
	/// * cn: New number of channels. If the parameter is 0, the number of channels remains the same.
	/// * rows: New number of rows. If the parameter is 0, the number of rows remains the same.
	/// 
	/// ## C++ default parameters
	/// * rows: 0
	fn reshape(&self, cn: i32, rows: i32) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_reshape_const_int_int(self.as_raw_Mat(), cn, rows) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Changes the shape and/or the number of channels of a 2D matrix without copying the data.
	/// 
	/// The method makes a new matrix header for \*this elements. The new matrix may have a different size
	/// and/or different number of channels. Any combination is possible if:
	/// *   No extra elements are included into the new matrix and no elements are excluded. Consequently,
	///    the product rows\*cols\*channels() must stay the same after the transformation.
	/// *   No data is copied. That is, this is an O(1) operation. Consequently, if you change the number of
	///    rows, or the operation changes the indices of elements row in some other way, the matrix must be
	///    continuous. See Mat::isContinuous .
	/// 
	/// For example, if there is a set of 3D points stored as an STL vector, and you want to represent the
	/// points as a 3xN matrix, do the following:
	/// ```ignore
	///    std::vector<Point3f> vec;
	///    ...
	///    Mat pointMat = Mat(vec). // convert vector to Mat, O(1) operation
	///                       reshape(1). // make Nx3 1-channel matrix out of Nx1 3-channel.
	///                                   // Also, an O(1) operation
	///                          t(); // finally, transpose the Nx3 matrix.
	///                               // This involves copying all the elements
	/// ```
	/// 
	/// ## Parameters
	/// * cn: New number of channels. If the parameter is 0, the number of channels remains the same.
	/// * rows: New number of rows. If the parameter is 0, the number of rows remains the same.
	/// 
	/// ## Overloaded parameters
	fn reshape_nd(&self, cn: i32, newsz: &[i32]) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_reshape_const_int_int_const_intX(self.as_raw_Mat(), cn, newsz.len() as _, newsz.as_ptr()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Changes the shape and/or the number of channels of a 2D matrix without copying the data.
	/// 
	/// The method makes a new matrix header for \*this elements. The new matrix may have a different size
	/// and/or different number of channels. Any combination is possible if:
	/// *   No extra elements are included into the new matrix and no elements are excluded. Consequently,
	///    the product rows\*cols\*channels() must stay the same after the transformation.
	/// *   No data is copied. That is, this is an O(1) operation. Consequently, if you change the number of
	///    rows, or the operation changes the indices of elements row in some other way, the matrix must be
	///    continuous. See Mat::isContinuous .
	/// 
	/// For example, if there is a set of 3D points stored as an STL vector, and you want to represent the
	/// points as a 3xN matrix, do the following:
	/// ```ignore
	///    std::vector<Point3f> vec;
	///    ...
	///    Mat pointMat = Mat(vec). // convert vector to Mat, O(1) operation
	///                       reshape(1). // make Nx3 1-channel matrix out of Nx1 3-channel.
	///                                   // Also, an O(1) operation
	///                          t(); // finally, transpose the Nx3 matrix.
	///                               // This involves copying all the elements
	/// ```
	/// 
	/// ## Parameters
	/// * cn: New number of channels. If the parameter is 0, the number of channels remains the same.
	/// * rows: New number of rows. If the parameter is 0, the number of rows remains the same.
	/// 
	/// ## Overloaded parameters
	fn reshape_nd_vec(&self, cn: i32, newshape: &core::Vector::<i32>) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_reshape_const_int_const_vector_int_R(self.as_raw_Mat(), cn, newshape.as_raw_VectorOfi32()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Transposes a matrix.
	/// 
	/// The method performs matrix transposition by means of matrix expressions. It does not perform the
	/// actual transposition but returns a temporary matrix transposition object that can be further used as
	/// a part of more complex matrix expressions or can be assigned to a matrix:
	/// ```ignore
	///    Mat A1 = A + Mat::eye(A.size(), A.type())*lambda;
	///    Mat C = A1.t()*A1; // compute (A + lambda*I)^t * (A + lamda*I)
	/// ```
	/// 
	fn t(&self) -> Result<core::MatExpr> {
		unsafe { sys::cv_Mat_t_const(self.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// Inverses a matrix.
	/// 
	/// The method performs a matrix inversion by means of matrix expressions. This means that a temporary
	/// matrix inversion object is returned by the method and can be used further as a part of more complex
	/// matrix expressions or can be assigned to a matrix.
	/// ## Parameters
	/// * method: Matrix inversion method. One of cv::DecompTypes
	/// 
	/// ## C++ default parameters
	/// * method: DECOMP_LU
	fn inv(&self, method: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_Mat_inv_const_int(self.as_raw_Mat(), method) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// Performs an element-wise multiplication or division of the two matrices.
	/// 
	/// The method returns a temporary object encoding per-element array multiplication, with optional
	/// scale. Note that this is not a matrix multiplication that corresponds to a simpler "\*" operator.
	/// 
	/// Example:
	/// ```ignore
	///    Mat C = A.mul(5/B); // equivalent to divide(A, B, C, 5)
	/// ```
	/// 
	/// ## Parameters
	/// * m: Another array of the same type and the same size as \*this, or a matrix expression.
	/// * scale: Optional scale factor.
	/// 
	/// ## C++ default parameters
	/// * scale: 1
	fn mul(&self, m: &dyn core::ToInputArray, scale: f64) -> Result<core::MatExpr> {
		input_array_arg!(m);
		unsafe { sys::cv_Mat_mul_const_const__InputArrayR_double(self.as_raw_Mat(), m.as_raw__InputArray(), scale) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// Computes a cross-product of two 3-element vectors.
	/// 
	/// The method computes a cross-product of two 3-element vectors. The vectors must be 3-element
	/// floating-point vectors of the same shape and size. The result is another 3-element vector of the
	/// same shape and type as operands.
	/// ## Parameters
	/// * m: Another cross-product operand.
	fn cross(&self, m: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(m);
		unsafe { sys::cv_Mat_cross_const_const__InputArrayR(self.as_raw_Mat(), m.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Computes a dot-product of two vectors.
	/// 
	/// The method computes a dot-product of two matrices. If the matrices are not single-column or
	/// single-row vectors, the top-to-bottom left-to-right scan ordering is used to treat them as 1D
	/// vectors. The vectors must have the same size and type. If the matrices have more than one channel,
	/// the dot products from all the channels are summed together.
	/// ## Parameters
	/// * m: another dot-product operand.
	fn dot(&self, m: &dyn core::ToInputArray) -> Result<f64> {
		input_array_arg!(m);
		unsafe { sys::cv_Mat_dot_const_const__InputArrayR(self.as_raw_Mat(), m.as_raw__InputArray()) }.into_result()
	}
	
	/// Allocates new array data if needed.
	/// 
	/// This is one of the key Mat methods. Most new-style OpenCV functions and methods that produce arrays
	/// call this method for each output array. The method uses the following algorithm:
	/// 
	/// -# If the current array shape and the type match the new ones, return immediately. Otherwise,
	///    de-reference the previous data by calling Mat::release.
	/// -# Initialize the new header.
	/// -# Allocate the new data of total()\*elemSize() bytes.
	/// -# Allocate the new, associated with the data, reference counter and set it to 1.
	/// 
	/// Such a scheme makes the memory management robust and efficient at the same time and helps avoid
	/// extra typing for you. This means that usually there is no need to explicitly allocate output arrays.
	/// That is, instead of writing:
	/// ```ignore
	///    Mat color;
	///    ...
	///    Mat gray(color.rows, color.cols, color.depth());
	///    cvtColor(color, gray, COLOR_BGR2GRAY);
	/// ```
	/// 
	/// you can simply write:
	/// ```ignore
	///    Mat color;
	///    ...
	///    Mat gray;
	///    cvtColor(color, gray, COLOR_BGR2GRAY);
	/// ```
	/// 
	/// because cvtColor, as well as the most of OpenCV functions, calls Mat::create() for the output array
	/// internally.
	/// ## Parameters
	/// * rows: New number of rows.
	/// * cols: New number of columns.
	/// * type: New matrix type.
	unsafe fn create_rows_cols(&mut self, rows: i32, cols: i32, typ: i32) -> Result<()> {
		{ sys::cv_Mat_create_int_int_int(self.as_raw_mut_Mat(), rows, cols, typ) }.into_result()
	}
	
	/// Allocates new array data if needed.
	/// 
	/// This is one of the key Mat methods. Most new-style OpenCV functions and methods that produce arrays
	/// call this method for each output array. The method uses the following algorithm:
	/// 
	/// -# If the current array shape and the type match the new ones, return immediately. Otherwise,
	///    de-reference the previous data by calling Mat::release.
	/// -# Initialize the new header.
	/// -# Allocate the new data of total()\*elemSize() bytes.
	/// -# Allocate the new, associated with the data, reference counter and set it to 1.
	/// 
	/// Such a scheme makes the memory management robust and efficient at the same time and helps avoid
	/// extra typing for you. This means that usually there is no need to explicitly allocate output arrays.
	/// That is, instead of writing:
	/// ```ignore
	///    Mat color;
	///    ...
	///    Mat gray(color.rows, color.cols, color.depth());
	///    cvtColor(color, gray, COLOR_BGR2GRAY);
	/// ```
	/// 
	/// you can simply write:
	/// ```ignore
	///    Mat color;
	///    ...
	///    Mat gray;
	///    cvtColor(color, gray, COLOR_BGR2GRAY);
	/// ```
	/// 
	/// because cvtColor, as well as the most of OpenCV functions, calls Mat::create() for the output array
	/// internally.
	/// ## Parameters
	/// * rows: New number of rows.
	/// * cols: New number of columns.
	/// * type: New matrix type.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * size: Alternative new matrix size specification: Size(cols, rows)
	/// * type: New matrix type.
	unsafe fn create_size(&mut self, size: core::Size, typ: i32) -> Result<()> {
		{ sys::cv_Mat_create_Size_int(self.as_raw_mut_Mat(), size.opencv_to_extern(), typ) }.into_result()
	}
	
	/// Allocates new array data if needed.
	/// 
	/// This is one of the key Mat methods. Most new-style OpenCV functions and methods that produce arrays
	/// call this method for each output array. The method uses the following algorithm:
	/// 
	/// -# If the current array shape and the type match the new ones, return immediately. Otherwise,
	///    de-reference the previous data by calling Mat::release.
	/// -# Initialize the new header.
	/// -# Allocate the new data of total()\*elemSize() bytes.
	/// -# Allocate the new, associated with the data, reference counter and set it to 1.
	/// 
	/// Such a scheme makes the memory management robust and efficient at the same time and helps avoid
	/// extra typing for you. This means that usually there is no need to explicitly allocate output arrays.
	/// That is, instead of writing:
	/// ```ignore
	///    Mat color;
	///    ...
	///    Mat gray(color.rows, color.cols, color.depth());
	///    cvtColor(color, gray, COLOR_BGR2GRAY);
	/// ```
	/// 
	/// you can simply write:
	/// ```ignore
	///    Mat color;
	///    ...
	///    Mat gray;
	///    cvtColor(color, gray, COLOR_BGR2GRAY);
	/// ```
	/// 
	/// because cvtColor, as well as the most of OpenCV functions, calls Mat::create() for the output array
	/// internally.
	/// ## Parameters
	/// * rows: New number of rows.
	/// * cols: New number of columns.
	/// * type: New matrix type.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * ndims: New array dimensionality.
	/// * sizes: Array of integers specifying a new array shape.
	/// * type: New matrix type.
	unsafe fn create_nd(&mut self, sizes: &[i32], typ: i32) -> Result<()> {
		{ sys::cv_Mat_create_int_const_intX_int(self.as_raw_mut_Mat(), sizes.len() as _, sizes.as_ptr(), typ) }.into_result()
	}
	
	/// Allocates new array data if needed.
	/// 
	/// This is one of the key Mat methods. Most new-style OpenCV functions and methods that produce arrays
	/// call this method for each output array. The method uses the following algorithm:
	/// 
	/// -# If the current array shape and the type match the new ones, return immediately. Otherwise,
	///    de-reference the previous data by calling Mat::release.
	/// -# Initialize the new header.
	/// -# Allocate the new data of total()\*elemSize() bytes.
	/// -# Allocate the new, associated with the data, reference counter and set it to 1.
	/// 
	/// Such a scheme makes the memory management robust and efficient at the same time and helps avoid
	/// extra typing for you. This means that usually there is no need to explicitly allocate output arrays.
	/// That is, instead of writing:
	/// ```ignore
	///    Mat color;
	///    ...
	///    Mat gray(color.rows, color.cols, color.depth());
	///    cvtColor(color, gray, COLOR_BGR2GRAY);
	/// ```
	/// 
	/// you can simply write:
	/// ```ignore
	///    Mat color;
	///    ...
	///    Mat gray;
	///    cvtColor(color, gray, COLOR_BGR2GRAY);
	/// ```
	/// 
	/// because cvtColor, as well as the most of OpenCV functions, calls Mat::create() for the output array
	/// internally.
	/// ## Parameters
	/// * rows: New number of rows.
	/// * cols: New number of columns.
	/// * type: New matrix type.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * sizes: Array of integers specifying a new array shape.
	/// * type: New matrix type.
	unsafe fn create_nd_vec(&mut self, sizes: &core::Vector::<i32>, typ: i32) -> Result<()> {
		{ sys::cv_Mat_create_const_vector_int_R_int(self.as_raw_mut_Mat(), sizes.as_raw_VectorOfi32(), typ) }.into_result()
	}
	
	/// Increments the reference counter.
	/// 
	/// The method increments the reference counter associated with the matrix data. If the matrix header
	/// points to an external data set (see Mat::Mat ), the reference counter is NULL, and the method has no
	/// effect in this case. Normally, to avoid memory leaks, the method should not be called explicitly. It
	/// is called implicitly by the matrix assignment operator. The reference counter increment is an atomic
	/// operation on the platforms that support it. Thus, it is safe to operate on the same matrices
	/// asynchronously in different threads.
	fn addref(&mut self) -> Result<()> {
		unsafe { sys::cv_Mat_addref(self.as_raw_mut_Mat()) }.into_result()
	}
	
	/// Decrements the reference counter and deallocates the matrix if needed.
	/// 
	/// The method decrements the reference counter associated with the matrix data. When the reference
	/// counter reaches 0, the matrix data is deallocated and the data and the reference counter pointers
	/// are set to NULL's. If the matrix header points to an external data set (see Mat::Mat ), the
	/// reference counter is NULL, and the method has no effect in this case.
	/// 
	/// This method can be called manually to force the matrix data deallocation. But since this method is
	/// automatically called in the destructor, or by any other method that changes the data pointer, it is
	/// usually not needed. The reference counter decrement and check for 0 is an atomic operation on the
	/// platforms that support it. Thus, it is safe to operate on the same matrices asynchronously in
	/// different threads.
	fn release(&mut self) -> Result<()> {
		unsafe { sys::cv_Mat_release(self.as_raw_mut_Mat()) }.into_result()
	}
	
	/// internal use function, consider to use 'release' method instead; deallocates the matrix data
	fn deallocate(&mut self) -> Result<()> {
		unsafe { sys::cv_Mat_deallocate(self.as_raw_mut_Mat()) }.into_result()
	}
	
	/// Reserves space for the certain number of rows.
	/// 
	/// The method reserves space for sz rows. If the matrix already has enough space to store sz rows,
	/// nothing happens. If the matrix is reallocated, the first Mat::rows rows are preserved. The method
	/// emulates the corresponding method of the STL vector class.
	/// ## Parameters
	/// * sz: Number of rows.
	fn reserve(&mut self, sz: size_t) -> Result<()> {
		unsafe { sys::cv_Mat_reserve_size_t(self.as_raw_mut_Mat(), sz) }.into_result()
	}
	
	/// Reserves space for the certain number of bytes.
	/// 
	/// The method reserves space for sz bytes. If the matrix already has enough space to store sz bytes,
	/// nothing happens. If matrix has to be reallocated its previous content could be lost.
	/// ## Parameters
	/// * sz: Number of bytes.
	fn reserve_buffer(&mut self, sz: size_t) -> Result<()> {
		unsafe { sys::cv_Mat_reserveBuffer_size_t(self.as_raw_mut_Mat(), sz) }.into_result()
	}
	
	/// Changes the number of matrix rows.
	/// 
	/// The methods change the number of matrix rows. If the matrix is reallocated, the first
	/// min(Mat::rows, sz) rows are preserved. The methods emulate the corresponding methods of the STL
	/// vector class.
	/// ## Parameters
	/// * sz: New number of rows.
	fn resize(&mut self, sz: size_t) -> Result<()> {
		unsafe { sys::cv_Mat_resize_size_t(self.as_raw_mut_Mat(), sz) }.into_result()
	}
	
	/// Changes the number of matrix rows.
	/// 
	/// The methods change the number of matrix rows. If the matrix is reallocated, the first
	/// min(Mat::rows, sz) rows are preserved. The methods emulate the corresponding methods of the STL
	/// vector class.
	/// ## Parameters
	/// * sz: New number of rows.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * sz: New number of rows.
	/// * s: Value assigned to the newly added elements.
	fn resize_with_default(&mut self, sz: size_t, s: core::Scalar) -> Result<()> {
		unsafe { sys::cv_Mat_resize_size_t_const_ScalarR(self.as_raw_mut_Mat(), sz, &s) }.into_result()
	}
	
	/// Adds elements to the bottom of the matrix.
	/// 
	/// The methods add one or more elements to the bottom of the matrix. They emulate the corresponding
	/// method of the STL vector class. When elem is Mat , its type and the number of columns must be the
	/// same as in the container matrix.
	/// ## Parameters
	/// * elem: Added element(s).
	/// 
	/// ## Overloaded parameters
	/// 
	/// * m: Added line(s).
	fn push_back(&mut self, m: &core::Mat) -> Result<()> {
		unsafe { sys::cv_Mat_push_back_const_MatR(self.as_raw_mut_Mat(), m.as_raw_Mat()) }.into_result()
	}
	
	/// Removes elements from the bottom of the matrix.
	/// 
	/// The method removes one or more rows from the bottom of the matrix.
	/// ## Parameters
	/// * nelems: Number of removed rows. If it is greater than the total number of rows, an exception
	/// is thrown.
	/// 
	/// ## C++ default parameters
	/// * nelems: 1
	fn pop_back(&mut self, nelems: size_t) -> Result<()> {
		unsafe { sys::cv_Mat_pop_back_size_t(self.as_raw_mut_Mat(), nelems) }.into_result()
	}
	
	/// Locates the matrix header within a parent matrix.
	/// 
	/// After you extracted a submatrix from a matrix using Mat::row, Mat::col, Mat::rowRange,
	/// Mat::colRange, and others, the resultant submatrix points just to the part of the original big
	/// matrix. However, each submatrix contains information (represented by datastart and dataend
	/// fields) that helps reconstruct the original matrix size and the position of the extracted
	/// submatrix within the original matrix. The method locateROI does exactly that.
	/// ## Parameters
	/// * wholeSize: Output parameter that contains the size of the whole matrix containing *this*
	/// as a part.
	/// * ofs: Output parameter that contains an offset of *this* inside the whole matrix.
	fn locate_roi(&self, whole_size: &mut core::Size, ofs: &mut core::Point) -> Result<()> {
		unsafe { sys::cv_Mat_locateROI_const_SizeR_PointR(self.as_raw_Mat(), whole_size, ofs) }.into_result()
	}
	
	/// Adjusts a submatrix size and position within the parent matrix.
	/// 
	/// The method is complimentary to Mat::locateROI . The typical use of these functions is to determine
	/// the submatrix position within the parent matrix and then shift the position somehow. Typically, it
	/// can be required for filtering operations when pixels outside of the ROI should be taken into
	/// account. When all the method parameters are positive, the ROI needs to grow in all directions by the
	/// specified amount, for example:
	/// ```ignore
	///    A.adjustROI(2, 2, 2, 2);
	/// ```
	/// 
	/// In this example, the matrix size is increased by 4 elements in each direction. The matrix is shifted
	/// by 2 elements to the left and 2 elements up, which brings in all the necessary pixels for the
	/// filtering with the 5x5 kernel.
	/// 
	/// adjustROI forces the adjusted ROI to be inside of the parent matrix that is boundaries of the
	/// adjusted ROI are constrained by boundaries of the parent matrix. For example, if the submatrix A is
	/// located in the first row of a parent matrix and you called A.adjustROI(2, 2, 2, 2) then A will not
	/// be increased in the upward direction.
	/// 
	/// The function is used internally by the OpenCV filtering functions, like filter2D , morphological
	/// operations, and so on.
	/// ## Parameters
	/// * dtop: Shift of the top submatrix boundary upwards.
	/// * dbottom: Shift of the bottom submatrix boundary downwards.
	/// * dleft: Shift of the left submatrix boundary to the left.
	/// * dright: Shift of the right submatrix boundary to the right.
	/// ## See also
	/// copyMakeBorder
	fn adjust_roi(&mut self, dtop: i32, dbottom: i32, dleft: i32, dright: i32) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_adjustROI_int_int_int_int(self.as_raw_mut_Mat(), dtop, dbottom, dleft, dright) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Reports whether the matrix is continuous or not.
	/// 
	/// The method returns true if the matrix elements are stored continuously without gaps at the end of
	/// each row. Otherwise, it returns false. Obviously, 1x1 or 1xN matrices are always continuous.
	/// Matrices created with Mat::create are always continuous. But if you extract a part of the matrix
	/// using Mat::col, Mat::diag, and so on, or constructed a matrix header for externally allocated data,
	/// such matrices may no longer have this property.
	/// 
	/// The continuity flag is stored as a bit in the Mat::flags field and is computed automatically when
	/// you construct a matrix header. Thus, the continuity check is a very fast operation, though
	/// theoretically it could be done as follows:
	/// ```ignore
	///    // alternative implementation of Mat::isContinuous()
	///    bool myCheckMatContinuity(const Mat& m)
	///    {
	///        //return (m.flags & Mat::CONTINUOUS_FLAG) != 0;
	///        return m.rows == 1 || m.step == m.cols*m.elemSize();
	///    }
	/// ```
	/// 
	/// The method is used in quite a few of OpenCV functions. The point is that element-wise operations
	/// (such as arithmetic and logical operations, math functions, alpha blending, color space
	/// transformations, and others) do not depend on the image geometry. Thus, if all the input and output
	/// arrays are continuous, the functions can process them as very long single-row vectors. The example
	/// below illustrates how an alpha-blending function can be implemented:
	/// ```ignore
	///    template<typename T>
	///    void alphaBlendRGBA(const Mat& src1, const Mat& src2, Mat& dst)
	///    {
	///        const float alpha_scale = (float)std::numeric_limits<T>::max(),
	///                    inv_scale = 1.f/alpha_scale;
	/// 
	///        CV_Assert( src1.type() == src2.type() &&
	///                    src1.type() == CV_MAKETYPE(traits::Depth<T>::value, 4) &&
	///                    src1.size() == src2.size());
	///        Size size = src1.size();
	///        dst.create(size, src1.type());
	/// 
	///        // here is the idiom: check the arrays for continuity and,
	///        // if this is the case,
	///        // treat the arrays as 1D vectors
	///        if( src1.isContinuous() && src2.isContinuous() && dst.isContinuous() )
	///        {
	///            size.width *= size.height;
	///            size.height = 1;
	///        }
	///        size.width *= 4;
	/// 
	///        for( int i = 0; i < size.height; i++ )
	///        {
	///            // when the arrays are continuous,
	///            // the outer loop is executed only once
	///            const T* ptr1 = src1.ptr<T>(i);
	///            const T* ptr2 = src2.ptr<T>(i);
	///            T* dptr = dst.ptr<T>(i);
	/// 
	///            for( int j = 0; j < size.width; j += 4 )
	///            {
	///                float alpha = ptr1[j+3]*inv_scale, beta = ptr2[j+3]*inv_scale;
	///                dptr[j] = saturate_cast<T>(ptr1[j]*alpha + ptr2[j]*beta);
	///                dptr[j+1] = saturate_cast<T>(ptr1[j+1]*alpha + ptr2[j+1]*beta);
	///                dptr[j+2] = saturate_cast<T>(ptr1[j+2]*alpha + ptr2[j+2]*beta);
	///                dptr[j+3] = saturate_cast<T>((1 - (1-alpha)*(1-beta))*alpha_scale);
	///            }
	///        }
	///    }
	/// ```
	/// 
	/// This approach, while being very simple, can boost the performance of a simple element-operation by
	/// 10-20 percents, especially if the image is rather small and the operation is quite simple.
	/// 
	/// Another OpenCV idiom in this function, a call of Mat::create for the destination array, that
	/// allocates the destination array unless it already has the proper size and type. And while the newly
	/// allocated arrays are always continuous, you still need to check the destination array because
	/// Mat::create does not always allocate a new matrix.
	fn is_continuous(&self) -> Result<bool> {
		unsafe { sys::cv_Mat_isContinuous_const(self.as_raw_Mat()) }.into_result()
	}
	
	/// returns true if the matrix is a submatrix of another matrix
	fn is_submatrix(&self) -> Result<bool> {
		unsafe { sys::cv_Mat_isSubmatrix_const(self.as_raw_Mat()) }.into_result()
	}
	
	/// Returns the matrix element size in bytes.
	/// 
	/// The method returns the matrix element size in bytes. For example, if the matrix type is CV_16SC3 ,
	/// the method returns 3\*sizeof(short) or 6.
	fn elem_size(&self) -> Result<size_t> {
		unsafe { sys::cv_Mat_elemSize_const(self.as_raw_Mat()) }.into_result()
	}
	
	/// Returns the size of each matrix element channel in bytes.
	/// 
	/// The method returns the matrix element channel size in bytes, that is, it ignores the number of
	/// channels. For example, if the matrix type is CV_16SC3 , the method returns sizeof(short) or 2.
	fn elem_size1(&self) -> Result<size_t> {
		unsafe { sys::cv_Mat_elemSize1_const(self.as_raw_Mat()) }.into_result()
	}
	
	/// Returns the type of a matrix element.
	/// 
	/// The method returns a matrix element type. This is an identifier compatible with the CvMat type
	/// system, like CV_16SC3 or 16-bit signed 3-channel array, and so on.
	fn typ(&self) -> Result<i32> {
		unsafe { sys::cv_Mat_type_const(self.as_raw_Mat()) }.into_result()
	}
	
	/// Returns the depth of a matrix element.
	/// 
	/// The method returns the identifier of the matrix element depth (the type of each individual channel).
	/// For example, for a 16-bit signed element array, the method returns CV_16S . A complete list of
	/// matrix types contains the following values:
	/// *   CV_8U - 8-bit unsigned integers ( 0..255 )
	/// *   CV_8S - 8-bit signed integers ( -128..127 )
	/// *   CV_16U - 16-bit unsigned integers ( 0..65535 )
	/// *   CV_16S - 16-bit signed integers ( -32768..32767 )
	/// *   CV_32S - 32-bit signed integers ( -2147483648..2147483647 )
	/// *   CV_32F - 32-bit floating-point numbers ( -FLT_MAX..FLT_MAX, INF, NAN )
	/// *   CV_64F - 64-bit floating-point numbers ( -DBL_MAX..DBL_MAX, INF, NAN )
	fn depth(&self) -> Result<i32> {
		unsafe { sys::cv_Mat_depth_const(self.as_raw_Mat()) }.into_result()
	}
	
	/// Returns the number of matrix channels.
	/// 
	/// The method returns the number of matrix channels.
	fn channels(&self) -> Result<i32> {
		unsafe { sys::cv_Mat_channels_const(self.as_raw_Mat()) }.into_result()
	}
	
	/// Returns a normalized step.
	/// 
	/// The method returns a matrix step divided by Mat::elemSize1() . It can be useful to quickly access an
	/// arbitrary matrix element.
	/// 
	/// ## C++ default parameters
	/// * i: 0
	fn step1(&self, i: i32) -> Result<size_t> {
		unsafe { sys::cv_Mat_step1_const_int(self.as_raw_Mat(), i) }.into_result()
	}
	
	/// Returns true if the array has no elements.
	/// 
	/// The method returns true if Mat::total() is 0 or if Mat::data is NULL. Because of pop_back() and
	/// resize() methods `M.total() == 0` does not imply that `M.data == NULL`.
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_Mat_empty_const(self.as_raw_Mat()) }.into_result()
	}
	
	/// Returns the total number of array elements.
	/// 
	/// The method returns the number of array elements (a number of pixels if the array represents an
	/// image).
	fn total(&self) -> Result<size_t> {
		unsafe { sys::cv_Mat_total_const(self.as_raw_Mat()) }.into_result()
	}
	
	/// Returns the total number of array elements.
	/// 
	/// The method returns the number of elements within a certain sub-array slice with startDim <= dim < endDim
	/// 
	/// ## C++ default parameters
	/// * end_dim: INT_MAX
	fn total_slice(&self, start_dim: i32, end_dim: i32) -> Result<size_t> {
		unsafe { sys::cv_Mat_total_const_int_int(self.as_raw_Mat(), start_dim, end_dim) }.into_result()
	}
	
	/// ## Parameters
	/// * elemChannels: Number of channels or number of columns the matrix should have.
	///                    For a 2-D matrix, when the matrix has only 1 column, then it should have
	///                    elemChannels channels; When the matrix has only 1 channel,
	///                    then it should have elemChannels columns.
	///                    For a 3-D matrix, it should have only one channel. Furthermore,
	///                    if the number of planes is not one, then the number of rows
	///                    within every plane has to be 1; if the number of rows within
	///                    every plane is not 1, then the number of planes has to be 1.
	/// * depth: The depth the matrix should have. Set it to -1 when any depth is fine.
	/// * requireContinuous: Set it to true to require the matrix to be continuous
	/// ## Returns
	/// -1 if the requirement is not satisfied.
	///        Otherwise, it returns the number of elements in the matrix. Note
	///        that an element may have multiple channels.
	/// 
	/// The following code demonstrates its usage for a 2-d matrix:
	/// [example-2d](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_mat_checkVector.cpp#L1)
	/// 
	/// The following code demonstrates its usage for a 3-d matrix:
	/// [example-3d](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_mat_checkVector.cpp#L1)
	/// 
	/// ## C++ default parameters
	/// * depth: -1
	/// * require_continuous: true
	fn check_vector(&self, elem_channels: i32, depth: i32, require_continuous: bool) -> Result<i32> {
		unsafe { sys::cv_Mat_checkVector_const_int_int_bool(self.as_raw_Mat(), elem_channels, depth, require_continuous) }.into_result()
	}
	
	/// Returns a pointer to the specified matrix row.
	/// 
	/// The methods return `uchar*` or typed pointer to the specified matrix row. See the sample in
	/// Mat::isContinuous to know how to use these methods.
	/// ## Parameters
	/// * i0: A 0-based row index.
	/// 
	/// ## C++ default parameters
	/// * i0: 0
	unsafe fn ptr_mut(&mut self, i0: i32) -> Result<&mut u8> {
		{ sys::cv_Mat_ptr_int(self.as_raw_mut_Mat(), i0) }.into_result().and_then(|x| { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Returns a pointer to the specified matrix row.
	/// 
	/// The methods return `uchar*` or typed pointer to the specified matrix row. See the sample in
	/// Mat::isContinuous to know how to use these methods.
	/// ## Parameters
	/// * i0: A 0-based row index.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * i0: 0
	unsafe fn ptr(&self, i0: i32) -> Result<&u8> {
		{ sys::cv_Mat_ptr_const_int(self.as_raw_Mat(), i0) }.into_result().and_then(|x| { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Returns a pointer to the specified matrix row.
	/// 
	/// The methods return `uchar*` or typed pointer to the specified matrix row. See the sample in
	/// Mat::isContinuous to know how to use these methods.
	/// ## Parameters
	/// * i0: A 0-based row index.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * row: Index along the dimension 0
	/// * col: Index along the dimension 1
	unsafe fn ptr_2d_mut(&mut self, row: i32, col: i32) -> Result<&mut u8> {
		{ sys::cv_Mat_ptr_int_int(self.as_raw_mut_Mat(), row, col) }.into_result().and_then(|x| { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Returns a pointer to the specified matrix row.
	/// 
	/// The methods return `uchar*` or typed pointer to the specified matrix row. See the sample in
	/// Mat::isContinuous to know how to use these methods.
	/// ## Parameters
	/// * i0: A 0-based row index.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * row: Index along the dimension 0
	/// * col: Index along the dimension 1
	unsafe fn ptr_2d(&self, row: i32, col: i32) -> Result<&u8> {
		{ sys::cv_Mat_ptr_const_int_int(self.as_raw_Mat(), row, col) }.into_result().and_then(|x| { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Returns a pointer to the specified matrix row.
	/// 
	/// The methods return `uchar*` or typed pointer to the specified matrix row. See the sample in
	/// Mat::isContinuous to know how to use these methods.
	/// ## Parameters
	/// * i0: A 0-based row index.
	/// 
	/// ## Overloaded parameters
	unsafe fn ptr_3d_mut(&mut self, i0: i32, i1: i32, i2: i32) -> Result<&mut u8> {
		{ sys::cv_Mat_ptr_int_int_int(self.as_raw_mut_Mat(), i0, i1, i2) }.into_result().and_then(|x| { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Returns a pointer to the specified matrix row.
	/// 
	/// The methods return `uchar*` or typed pointer to the specified matrix row. See the sample in
	/// Mat::isContinuous to know how to use these methods.
	/// ## Parameters
	/// * i0: A 0-based row index.
	/// 
	/// ## Overloaded parameters
	unsafe fn ptr_3d(&self, i0: i32, i1: i32, i2: i32) -> Result<&u8> {
		{ sys::cv_Mat_ptr_const_int_int_int(self.as_raw_Mat(), i0, i1, i2) }.into_result().and_then(|x| { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Returns a pointer to the specified matrix row.
	/// 
	/// The methods return `uchar*` or typed pointer to the specified matrix row. See the sample in
	/// Mat::isContinuous to know how to use these methods.
	/// ## Parameters
	/// * i0: A 0-based row index.
	/// 
	/// ## Overloaded parameters
	unsafe fn ptr_nd_mut(&mut self, idx: &[i32]) -> Result<&mut u8> {
		{ sys::cv_Mat_ptr_const_intX(self.as_raw_mut_Mat(), idx.as_ptr()) }.into_result().and_then(|x| { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Returns a pointer to the specified matrix row.
	/// 
	/// The methods return `uchar*` or typed pointer to the specified matrix row. See the sample in
	/// Mat::isContinuous to know how to use these methods.
	/// ## Parameters
	/// * i0: A 0-based row index.
	/// 
	/// ## Overloaded parameters
	unsafe fn ptr_nd(&self, idx: &[i32]) -> Result<&u8> {
		{ sys::cv_Mat_ptr_const_const_intX(self.as_raw_Mat(), idx.as_ptr()) }.into_result().and_then(|x| { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Returns a reference to the specified array element.
	/// 
	/// The template methods return a reference to the specified array element. For the sake of higher
	/// performance, the index range checks are only performed in the Debug configuration.
	/// 
	/// Note that the variants with a single index (i) can be used to access elements of single-row or
	/// single-column 2-dimensional arrays. That is, if, for example, A is a 1 x N floating-point matrix and
	/// B is an M x 1 integer matrix, you can simply write `A.at<float>(k+4)` and `B.at<int>(2*i+1)`
	/// instead of `A.at<float>(0,k+4)` and `B.at<int>(2*i+1,0)`, respectively.
	/// 
	/// The example below initializes a Hilbert matrix:
	/// ```ignore
	///    Mat H(100, 100, CV_64F);
	///    for(int i = 0; i < H.rows; i++)
	///        for(int j = 0; j < H.cols; j++)
	///            H.at<double>(i,j)=1./(i+j+1);
	/// ```
	/// 
	/// 
	/// Keep in mind that the size identifier used in the at operator cannot be chosen at random. It depends
	/// on the image from which you are trying to retrieve the data. The table below gives a better insight in this:
	///  - If matrix is of type `CV_8U` then use `Mat.at<uchar>(y,x)`.
	///  - If matrix is of type `CV_8S` then use `Mat.at<schar>(y,x)`.
	///  - If matrix is of type `CV_16U` then use `Mat.at<ushort>(y,x)`.
	///  - If matrix is of type `CV_16S` then use `Mat.at<short>(y,x)`.
	///  - If matrix is of type `CV_32S`  then use `Mat.at<int>(y,x)`.
	///  - If matrix is of type `CV_32F`  then use `Mat.at<float>(y,x)`.
	///  - If matrix is of type `CV_64F` then use `Mat.at<double>(y,x)`.
	/// 
	/// ## Parameters
	/// * i0: Index along the dimension 0
	/// 
	/// ## C++ default parameters
	/// * i0: 0
	fn at_mut<T: core::DataType>(&mut self, i0: i32) -> Result<&mut T> { core::mat_forward::at_mut(self, i0) }
	
	/// Returns a reference to the specified array element.
	/// 
	/// The template methods return a reference to the specified array element. For the sake of higher
	/// performance, the index range checks are only performed in the Debug configuration.
	/// 
	/// Note that the variants with a single index (i) can be used to access elements of single-row or
	/// single-column 2-dimensional arrays. That is, if, for example, A is a 1 x N floating-point matrix and
	/// B is an M x 1 integer matrix, you can simply write `A.at<float>(k+4)` and `B.at<int>(2*i+1)`
	/// instead of `A.at<float>(0,k+4)` and `B.at<int>(2*i+1,0)`, respectively.
	/// 
	/// The example below initializes a Hilbert matrix:
	/// ```ignore
	///    Mat H(100, 100, CV_64F);
	///    for(int i = 0; i < H.rows; i++)
	///        for(int j = 0; j < H.cols; j++)
	///            H.at<double>(i,j)=1./(i+j+1);
	/// ```
	/// 
	/// 
	/// Keep in mind that the size identifier used in the at operator cannot be chosen at random. It depends
	/// on the image from which you are trying to retrieve the data. The table below gives a better insight in this:
	///  - If matrix is of type `CV_8U` then use `Mat.at<uchar>(y,x)`.
	///  - If matrix is of type `CV_8S` then use `Mat.at<schar>(y,x)`.
	///  - If matrix is of type `CV_16U` then use `Mat.at<ushort>(y,x)`.
	///  - If matrix is of type `CV_16S` then use `Mat.at<short>(y,x)`.
	///  - If matrix is of type `CV_32S`  then use `Mat.at<int>(y,x)`.
	///  - If matrix is of type `CV_32F`  then use `Mat.at<float>(y,x)`.
	///  - If matrix is of type `CV_64F` then use `Mat.at<double>(y,x)`.
	/// 
	/// ## Parameters
	/// * i0: Index along the dimension 0
	/// 
	/// ## Overloaded parameters
	/// 
	/// * i0: Index along the dimension 0
	/// 
	/// ## C++ default parameters
	/// * i0: 0
	fn at<T: core::DataType>(&self, i0: i32) -> Result<&T> { core::mat_forward::at(self, i0) }
	
	/// Returns a reference to the specified array element.
	/// 
	/// The template methods return a reference to the specified array element. For the sake of higher
	/// performance, the index range checks are only performed in the Debug configuration.
	/// 
	/// Note that the variants with a single index (i) can be used to access elements of single-row or
	/// single-column 2-dimensional arrays. That is, if, for example, A is a 1 x N floating-point matrix and
	/// B is an M x 1 integer matrix, you can simply write `A.at<float>(k+4)` and `B.at<int>(2*i+1)`
	/// instead of `A.at<float>(0,k+4)` and `B.at<int>(2*i+1,0)`, respectively.
	/// 
	/// The example below initializes a Hilbert matrix:
	/// ```ignore
	///    Mat H(100, 100, CV_64F);
	///    for(int i = 0; i < H.rows; i++)
	///        for(int j = 0; j < H.cols; j++)
	///            H.at<double>(i,j)=1./(i+j+1);
	/// ```
	/// 
	/// 
	/// Keep in mind that the size identifier used in the at operator cannot be chosen at random. It depends
	/// on the image from which you are trying to retrieve the data. The table below gives a better insight in this:
	///  - If matrix is of type `CV_8U` then use `Mat.at<uchar>(y,x)`.
	///  - If matrix is of type `CV_8S` then use `Mat.at<schar>(y,x)`.
	///  - If matrix is of type `CV_16U` then use `Mat.at<ushort>(y,x)`.
	///  - If matrix is of type `CV_16S` then use `Mat.at<short>(y,x)`.
	///  - If matrix is of type `CV_32S`  then use `Mat.at<int>(y,x)`.
	///  - If matrix is of type `CV_32F`  then use `Mat.at<float>(y,x)`.
	///  - If matrix is of type `CV_64F` then use `Mat.at<double>(y,x)`.
	/// 
	/// ## Parameters
	/// * i0: Index along the dimension 0
	/// 
	/// ## Overloaded parameters
	/// 
	/// * row: Index along the dimension 0
	/// * col: Index along the dimension 1
	fn at_2d_mut<T: core::DataType>(&mut self, row: i32, col: i32) -> Result<&mut T> { core::mat_forward::at_2d_mut(self, row, col) }
	
	/// Returns a reference to the specified array element.
	/// 
	/// The template methods return a reference to the specified array element. For the sake of higher
	/// performance, the index range checks are only performed in the Debug configuration.
	/// 
	/// Note that the variants with a single index (i) can be used to access elements of single-row or
	/// single-column 2-dimensional arrays. That is, if, for example, A is a 1 x N floating-point matrix and
	/// B is an M x 1 integer matrix, you can simply write `A.at<float>(k+4)` and `B.at<int>(2*i+1)`
	/// instead of `A.at<float>(0,k+4)` and `B.at<int>(2*i+1,0)`, respectively.
	/// 
	/// The example below initializes a Hilbert matrix:
	/// ```ignore
	///    Mat H(100, 100, CV_64F);
	///    for(int i = 0; i < H.rows; i++)
	///        for(int j = 0; j < H.cols; j++)
	///            H.at<double>(i,j)=1./(i+j+1);
	/// ```
	/// 
	/// 
	/// Keep in mind that the size identifier used in the at operator cannot be chosen at random. It depends
	/// on the image from which you are trying to retrieve the data. The table below gives a better insight in this:
	///  - If matrix is of type `CV_8U` then use `Mat.at<uchar>(y,x)`.
	///  - If matrix is of type `CV_8S` then use `Mat.at<schar>(y,x)`.
	///  - If matrix is of type `CV_16U` then use `Mat.at<ushort>(y,x)`.
	///  - If matrix is of type `CV_16S` then use `Mat.at<short>(y,x)`.
	///  - If matrix is of type `CV_32S`  then use `Mat.at<int>(y,x)`.
	///  - If matrix is of type `CV_32F`  then use `Mat.at<float>(y,x)`.
	///  - If matrix is of type `CV_64F` then use `Mat.at<double>(y,x)`.
	/// 
	/// ## Parameters
	/// * i0: Index along the dimension 0
	/// 
	/// ## Overloaded parameters
	/// 
	/// * row: Index along the dimension 0
	/// * col: Index along the dimension 1
	fn at_2d<T: core::DataType>(&self, row: i32, col: i32) -> Result<&T> { core::mat_forward::at_2d(self, row, col) }
	
	/// Returns a reference to the specified array element.
	/// 
	/// The template methods return a reference to the specified array element. For the sake of higher
	/// performance, the index range checks are only performed in the Debug configuration.
	/// 
	/// Note that the variants with a single index (i) can be used to access elements of single-row or
	/// single-column 2-dimensional arrays. That is, if, for example, A is a 1 x N floating-point matrix and
	/// B is an M x 1 integer matrix, you can simply write `A.at<float>(k+4)` and `B.at<int>(2*i+1)`
	/// instead of `A.at<float>(0,k+4)` and `B.at<int>(2*i+1,0)`, respectively.
	/// 
	/// The example below initializes a Hilbert matrix:
	/// ```ignore
	///    Mat H(100, 100, CV_64F);
	///    for(int i = 0; i < H.rows; i++)
	///        for(int j = 0; j < H.cols; j++)
	///            H.at<double>(i,j)=1./(i+j+1);
	/// ```
	/// 
	/// 
	/// Keep in mind that the size identifier used in the at operator cannot be chosen at random. It depends
	/// on the image from which you are trying to retrieve the data. The table below gives a better insight in this:
	///  - If matrix is of type `CV_8U` then use `Mat.at<uchar>(y,x)`.
	///  - If matrix is of type `CV_8S` then use `Mat.at<schar>(y,x)`.
	///  - If matrix is of type `CV_16U` then use `Mat.at<ushort>(y,x)`.
	///  - If matrix is of type `CV_16S` then use `Mat.at<short>(y,x)`.
	///  - If matrix is of type `CV_32S`  then use `Mat.at<int>(y,x)`.
	///  - If matrix is of type `CV_32F`  then use `Mat.at<float>(y,x)`.
	///  - If matrix is of type `CV_64F` then use `Mat.at<double>(y,x)`.
	/// 
	/// ## Parameters
	/// * i0: Index along the dimension 0
	/// 
	/// ## Overloaded parameters
	/// 
	/// * i0: Index along the dimension 0
	/// * i1: Index along the dimension 1
	/// * i2: Index along the dimension 2
	fn at_3d_mut<T: core::DataType>(&mut self, i0: i32, i1: i32, i2: i32) -> Result<&mut T> { core::mat_forward::at_3d_mut(self, i0, i1, i2) }
	
	/// Returns a reference to the specified array element.
	/// 
	/// The template methods return a reference to the specified array element. For the sake of higher
	/// performance, the index range checks are only performed in the Debug configuration.
	/// 
	/// Note that the variants with a single index (i) can be used to access elements of single-row or
	/// single-column 2-dimensional arrays. That is, if, for example, A is a 1 x N floating-point matrix and
	/// B is an M x 1 integer matrix, you can simply write `A.at<float>(k+4)` and `B.at<int>(2*i+1)`
	/// instead of `A.at<float>(0,k+4)` and `B.at<int>(2*i+1,0)`, respectively.
	/// 
	/// The example below initializes a Hilbert matrix:
	/// ```ignore
	///    Mat H(100, 100, CV_64F);
	///    for(int i = 0; i < H.rows; i++)
	///        for(int j = 0; j < H.cols; j++)
	///            H.at<double>(i,j)=1./(i+j+1);
	/// ```
	/// 
	/// 
	/// Keep in mind that the size identifier used in the at operator cannot be chosen at random. It depends
	/// on the image from which you are trying to retrieve the data. The table below gives a better insight in this:
	///  - If matrix is of type `CV_8U` then use `Mat.at<uchar>(y,x)`.
	///  - If matrix is of type `CV_8S` then use `Mat.at<schar>(y,x)`.
	///  - If matrix is of type `CV_16U` then use `Mat.at<ushort>(y,x)`.
	///  - If matrix is of type `CV_16S` then use `Mat.at<short>(y,x)`.
	///  - If matrix is of type `CV_32S`  then use `Mat.at<int>(y,x)`.
	///  - If matrix is of type `CV_32F`  then use `Mat.at<float>(y,x)`.
	///  - If matrix is of type `CV_64F` then use `Mat.at<double>(y,x)`.
	/// 
	/// ## Parameters
	/// * i0: Index along the dimension 0
	/// 
	/// ## Overloaded parameters
	/// 
	/// * i0: Index along the dimension 0
	/// * i1: Index along the dimension 1
	/// * i2: Index along the dimension 2
	fn at_3d<T: core::DataType>(&self, i0: i32, i1: i32, i2: i32) -> Result<&T> { core::mat_forward::at_3d(self, i0, i1, i2) }
	
	/// Returns a reference to the specified array element.
	/// 
	/// The template methods return a reference to the specified array element. For the sake of higher
	/// performance, the index range checks are only performed in the Debug configuration.
	/// 
	/// Note that the variants with a single index (i) can be used to access elements of single-row or
	/// single-column 2-dimensional arrays. That is, if, for example, A is a 1 x N floating-point matrix and
	/// B is an M x 1 integer matrix, you can simply write `A.at<float>(k+4)` and `B.at<int>(2*i+1)`
	/// instead of `A.at<float>(0,k+4)` and `B.at<int>(2*i+1,0)`, respectively.
	/// 
	/// The example below initializes a Hilbert matrix:
	/// ```ignore
	///    Mat H(100, 100, CV_64F);
	///    for(int i = 0; i < H.rows; i++)
	///        for(int j = 0; j < H.cols; j++)
	///            H.at<double>(i,j)=1./(i+j+1);
	/// ```
	/// 
	/// 
	/// Keep in mind that the size identifier used in the at operator cannot be chosen at random. It depends
	/// on the image from which you are trying to retrieve the data. The table below gives a better insight in this:
	///  - If matrix is of type `CV_8U` then use `Mat.at<uchar>(y,x)`.
	///  - If matrix is of type `CV_8S` then use `Mat.at<schar>(y,x)`.
	///  - If matrix is of type `CV_16U` then use `Mat.at<ushort>(y,x)`.
	///  - If matrix is of type `CV_16S` then use `Mat.at<short>(y,x)`.
	///  - If matrix is of type `CV_32S`  then use `Mat.at<int>(y,x)`.
	///  - If matrix is of type `CV_32F`  then use `Mat.at<float>(y,x)`.
	///  - If matrix is of type `CV_64F` then use `Mat.at<double>(y,x)`.
	/// 
	/// ## Parameters
	/// * i0: Index along the dimension 0
	/// 
	/// ## Overloaded parameters
	/// 
	/// * idx: Array of Mat::dims indices.
	fn at_nd_mut<T: core::DataType>(&mut self, idx: &[i32]) -> Result<&mut T> { core::mat_forward::at_nd_mut(self, idx) }
	
	/// Returns a reference to the specified array element.
	/// 
	/// The template methods return a reference to the specified array element. For the sake of higher
	/// performance, the index range checks are only performed in the Debug configuration.
	/// 
	/// Note that the variants with a single index (i) can be used to access elements of single-row or
	/// single-column 2-dimensional arrays. That is, if, for example, A is a 1 x N floating-point matrix and
	/// B is an M x 1 integer matrix, you can simply write `A.at<float>(k+4)` and `B.at<int>(2*i+1)`
	/// instead of `A.at<float>(0,k+4)` and `B.at<int>(2*i+1,0)`, respectively.
	/// 
	/// The example below initializes a Hilbert matrix:
	/// ```ignore
	///    Mat H(100, 100, CV_64F);
	///    for(int i = 0; i < H.rows; i++)
	///        for(int j = 0; j < H.cols; j++)
	///            H.at<double>(i,j)=1./(i+j+1);
	/// ```
	/// 
	/// 
	/// Keep in mind that the size identifier used in the at operator cannot be chosen at random. It depends
	/// on the image from which you are trying to retrieve the data. The table below gives a better insight in this:
	///  - If matrix is of type `CV_8U` then use `Mat.at<uchar>(y,x)`.
	///  - If matrix is of type `CV_8S` then use `Mat.at<schar>(y,x)`.
	///  - If matrix is of type `CV_16U` then use `Mat.at<ushort>(y,x)`.
	///  - If matrix is of type `CV_16S` then use `Mat.at<short>(y,x)`.
	///  - If matrix is of type `CV_32S`  then use `Mat.at<int>(y,x)`.
	///  - If matrix is of type `CV_32F`  then use `Mat.at<float>(y,x)`.
	///  - If matrix is of type `CV_64F` then use `Mat.at<double>(y,x)`.
	/// 
	/// ## Parameters
	/// * i0: Index along the dimension 0
	/// 
	/// ## Overloaded parameters
	/// 
	/// * idx: Array of Mat::dims indices.
	fn at_nd<T: core::DataType>(&self, idx: &[i32]) -> Result<&T> { core::mat_forward::at_nd(self, idx) }
	
	/// Returns a reference to the specified array element.
	/// 
	/// The template methods return a reference to the specified array element. For the sake of higher
	/// performance, the index range checks are only performed in the Debug configuration.
	/// 
	/// Note that the variants with a single index (i) can be used to access elements of single-row or
	/// single-column 2-dimensional arrays. That is, if, for example, A is a 1 x N floating-point matrix and
	/// B is an M x 1 integer matrix, you can simply write `A.at<float>(k+4)` and `B.at<int>(2*i+1)`
	/// instead of `A.at<float>(0,k+4)` and `B.at<int>(2*i+1,0)`, respectively.
	/// 
	/// The example below initializes a Hilbert matrix:
	/// ```ignore
	///    Mat H(100, 100, CV_64F);
	///    for(int i = 0; i < H.rows; i++)
	///        for(int j = 0; j < H.cols; j++)
	///            H.at<double>(i,j)=1./(i+j+1);
	/// ```
	/// 
	/// 
	/// Keep in mind that the size identifier used in the at operator cannot be chosen at random. It depends
	/// on the image from which you are trying to retrieve the data. The table below gives a better insight in this:
	///  - If matrix is of type `CV_8U` then use `Mat.at<uchar>(y,x)`.
	///  - If matrix is of type `CV_8S` then use `Mat.at<schar>(y,x)`.
	///  - If matrix is of type `CV_16U` then use `Mat.at<ushort>(y,x)`.
	///  - If matrix is of type `CV_16S` then use `Mat.at<short>(y,x)`.
	///  - If matrix is of type `CV_32S`  then use `Mat.at<int>(y,x)`.
	///  - If matrix is of type `CV_32F`  then use `Mat.at<float>(y,x)`.
	///  - If matrix is of type `CV_64F` then use `Mat.at<double>(y,x)`.
	/// 
	/// ## Parameters
	/// * i0: Index along the dimension 0
	/// 
	/// ## Overloaded parameters
	/// 
	///    special versions for 2D arrays (especially convenient for referencing image pixels)
	/// * pt: Element position specified as Point(j,i) .
	fn at_pt_mut<T: core::DataType>(&mut self, pt: core::Point) -> Result<&mut T> { core::mat_forward::at_pt_mut(self, pt) }
	
	/// Returns a reference to the specified array element.
	/// 
	/// The template methods return a reference to the specified array element. For the sake of higher
	/// performance, the index range checks are only performed in the Debug configuration.
	/// 
	/// Note that the variants with a single index (i) can be used to access elements of single-row or
	/// single-column 2-dimensional arrays. That is, if, for example, A is a 1 x N floating-point matrix and
	/// B is an M x 1 integer matrix, you can simply write `A.at<float>(k+4)` and `B.at<int>(2*i+1)`
	/// instead of `A.at<float>(0,k+4)` and `B.at<int>(2*i+1,0)`, respectively.
	/// 
	/// The example below initializes a Hilbert matrix:
	/// ```ignore
	///    Mat H(100, 100, CV_64F);
	///    for(int i = 0; i < H.rows; i++)
	///        for(int j = 0; j < H.cols; j++)
	///            H.at<double>(i,j)=1./(i+j+1);
	/// ```
	/// 
	/// 
	/// Keep in mind that the size identifier used in the at operator cannot be chosen at random. It depends
	/// on the image from which you are trying to retrieve the data. The table below gives a better insight in this:
	///  - If matrix is of type `CV_8U` then use `Mat.at<uchar>(y,x)`.
	///  - If matrix is of type `CV_8S` then use `Mat.at<schar>(y,x)`.
	///  - If matrix is of type `CV_16U` then use `Mat.at<ushort>(y,x)`.
	///  - If matrix is of type `CV_16S` then use `Mat.at<short>(y,x)`.
	///  - If matrix is of type `CV_32S`  then use `Mat.at<int>(y,x)`.
	///  - If matrix is of type `CV_32F`  then use `Mat.at<float>(y,x)`.
	///  - If matrix is of type `CV_64F` then use `Mat.at<double>(y,x)`.
	/// 
	/// ## Parameters
	/// * i0: Index along the dimension 0
	/// 
	/// ## Overloaded parameters
	/// 
	///    special versions for 2D arrays (especially convenient for referencing image pixels)
	/// * pt: Element position specified as Point(j,i) .
	fn at_pt<T: core::DataType>(&self, pt: core::Point) -> Result<&T> { core::mat_forward::at_pt(self, pt) }
	
	/// internal use method: updates the continuity flag
	fn update_continuity_flag(&mut self) -> Result<()> {
		unsafe { sys::cv_Mat_updateContinuityFlag(self.as_raw_mut_Mat()) }.into_result()
	}
	
}

/// n-dimensional dense array class \anchor CVMat_Details
/// 
/// The class Mat represents an n-dimensional dense numerical single-channel or multi-channel array. It
/// can be used to store real or complex-valued vectors and matrices, grayscale or color images, voxel
/// volumes, vector fields, point clouds, tensors, histograms (though, very high-dimensional histograms
/// may be better stored in a SparseMat ). The data layout of the array `M` is defined by the array
/// `M.step[]`, so that the address of element ![inline formula](https://latex.codecogs.com/png.latex?%28i%5F0%2C%2E%2E%2E%2Ci%5F%7BM%2Edims%2D1%7D%29), where ![inline formula](https://latex.codecogs.com/png.latex?0%5Cleq%20i%5Fk%3CM%2Esize%5Bk%5D), is
/// computed as:
/// ![block formula](https://latex.codecogs.com/png.latex?addr%28M%5F%7Bi%5F0%2C%2E%2E%2E%2Ci%5F%7BM%2Edims%2D1%7D%7D%29%20%3D%20M%2Edata%20%2B%20M%2Estep%5B0%5D%2Ai%5F0%20%2B%20M%2Estep%5B1%5D%2Ai%5F1%20%2B%20%2E%2E%2E%20%2B%20M%2Estep%5BM%2Edims%2D1%5D%2Ai%5F%7BM%2Edims%2D1%7D)
/// In case of a 2-dimensional array, the above formula is reduced to:
/// ![block formula](https://latex.codecogs.com/png.latex?addr%28M%5F%7Bi%2Cj%7D%29%20%3D%20M%2Edata%20%2B%20M%2Estep%5B0%5D%2Ai%20%2B%20M%2Estep%5B1%5D%2Aj)
/// Note that `M.step[i] >= M.step[i+1]` (in fact, `M.step[i] >= M.step[i+1]*M.size[i+1]` ). This means
/// that 2-dimensional matrices are stored row-by-row, 3-dimensional matrices are stored plane-by-plane,
/// and so on. M.step[M.dims-1] is minimal and always equal to the element size M.elemSize() .
/// 
/// So, the data layout in Mat is compatible with the majority of dense array types from the standard
/// toolkits and SDKs, such as Numpy (ndarray), Win32 (independent device bitmaps), and others,
/// that is, with any array that uses *steps* (or *strides*) to compute the position of a pixel.
/// Due to this compatibility, it is possible to make a Mat header for user-allocated data and process
/// it in-place using OpenCV functions.
/// 
/// There are many different ways to create a Mat object. The most popular options are listed below:
/// 
/// - Use the create(nrows, ncols, type) method or the similar Mat(nrows, ncols, type[, fillValue])
/// constructor. A new array of the specified size and type is allocated. type has the same meaning as
/// in the cvCreateMat method. For example, CV_8UC1 means a 8-bit single-channel array, CV_32FC2
/// means a 2-channel (complex) floating-point array, and so on.
/// ```ignore
///    // make a 7x7 complex matrix filled with 1+3j.
///    Mat M(7,7,CV_32FC2,Scalar(1,3));
///    // and now turn M to a 100x60 15-channel 8-bit matrix.
///    // The old content will be deallocated
///    M.create(100,60,CV_8UC(15));
/// ```
/// 
/// As noted in the introduction to this chapter, create() allocates only a new array when the shape
/// or type of the current array are different from the specified ones.
/// 
/// - Create a multi-dimensional array:
/// ```ignore
///    // create a 100x100x100 8-bit array
///    int sz[] = {100, 100, 100};
///    Mat bigCube(3, sz, CV_8U, Scalar::all(0));
/// ```
/// 
/// It passes the number of dimensions =1 to the Mat constructor but the created array will be
/// 2-dimensional with the number of columns set to 1. So, Mat::dims is always \>= 2 (can also be 0
/// when the array is empty).
/// 
/// - Use a copy constructor or assignment operator where there can be an array or expression on the
/// right side (see below). As noted in the introduction, the array assignment is an O(1) operation
/// because it only copies the header and increases the reference counter. The Mat::clone() method can
/// be used to get a full (deep) copy of the array when you need it.
/// 
/// - Construct a header for a part of another array. It can be a single row, single column, several
/// rows, several columns, rectangular region in the array (called a *minor* in algebra) or a
/// diagonal. Such operations are also O(1) because the new header references the same data. You can
/// actually modify a part of the array using this feature, for example:
/// ```ignore
///    // add the 5-th row, multiplied by 3 to the 3rd row
///    M.row(3) = M.row(3) + M.row(5)*3;
///    // now copy the 7-th column to the 1-st column
///    // M.col(1) = M.col(7); // this will not work
///    Mat M1 = M.col(1);
///    M.col(7).copyTo(M1);
///    // create a new 320x240 image
///    Mat img(Size(320,240),CV_8UC3);
///    // select a ROI
///    Mat roi(img, Rect(10,10,100,100));
///    // fill the ROI with (0,255,0) (which is green in RGB space);
///    // the original 320x240 image will be modified
///    roi = Scalar(0,255,0);
/// ```
/// 
/// Due to the additional datastart and dataend members, it is possible to compute a relative
/// sub-array position in the main *container* array using locateROI():
/// ```ignore
///    Mat A = Mat::eye(10, 10, CV_32S);
///    // extracts A columns, 1 (inclusive) to 3 (exclusive).
///    Mat B = A(Range::all(), Range(1, 3));
///    // extracts B rows, 5 (inclusive) to 9 (exclusive).
///    // that is, C \~ A(Range(5, 9), Range(1, 3))
///    Mat C = B(Range(5, 9), Range::all());
///    Size size; Point ofs;
///    C.locateROI(size, ofs);
///    // size will be (width=10,height=10) and the ofs will be (x=1, y=5)
/// ```
/// 
/// As in case of whole matrices, if you need a deep copy, use the `clone()` method of the extracted
/// sub-matrices.
/// 
/// - Make a header for user-allocated data. It can be useful to do the following:
///    -# Process "foreign" data using OpenCV (for example, when you implement a DirectShow\* filter or
///    a processing module for gstreamer, and so on). For example:
///    ```ignore
///        void process_video_frame(const unsigned char* pixels,
///                                  int width, int height, int step)
///        {
///            Mat img(height, width, CV_8UC3, pixels, step);
///            GaussianBlur(img, img, Size(7,7), 1.5, 1.5);
///        }
///    ```
/// 
///    -# Quickly initialize small matrices and/or get a super-fast element access.
///    ```ignore
///        double m[3][3] = {{a, b, c}, {d, e, f}, {g, h, i}};
///        Mat M = Mat(3, 3, CV_64F, m).inv();
///    ```
/// 
///    .
/// 
/// - Use MATLAB-style array initializers, zeros(), ones(), eye(), for example:
/// ```ignore
///    // create a double-precision identity matrix and add it to M.
///    M += Mat::eye(M.rows, M.cols, CV_64F);
/// ```
/// 
/// 
/// - Use a comma-separated initializer:
/// ```ignore
///    // create a 3x3 double-precision identity matrix
///    Mat M = (Mat_<double>(3,3) << 1, 0, 0, 0, 1, 0, 0, 0, 1);
/// ```
/// 
/// With this approach, you first call a constructor of the Mat class with the proper parameters, and
/// then you just put `<< operator` followed by comma-separated values that can be constants,
/// variables, expressions, and so on. Also, note the extra parentheses required to avoid compilation
/// errors.
/// 
/// Once the array is created, it is automatically managed via a reference-counting mechanism. If the
/// array header is built on top of user-allocated data, you should handle the data by yourself. The
/// array data is deallocated when no one points to it. If you want to release the data pointed by a
/// array header before the array destructor is called, use Mat::release().
/// 
/// The next important thing to learn about the array class is element access. This manual already
/// described how to compute an address of each array element. Normally, you are not required to use the
/// formula directly in the code. If you know the array element type (which can be retrieved using the
/// method Mat::type() ), you can access the element ![inline formula](https://latex.codecogs.com/png.latex?M%5F%7Bij%7D) of a 2-dimensional array as:
/// ```ignore
///    M.at<double>(i,j) += 1.f;
/// ```
/// 
/// assuming that `M` is a double-precision floating-point array. There are several variants of the method
/// at for a different number of dimensions.
/// 
/// If you need to process a whole row of a 2D array, the most efficient way is to get the pointer to
/// the row first, and then just use the plain C operator [] :
/// ```ignore
///    // compute sum of positive matrix elements
///    // (assuming that M is a double-precision matrix)
///    double sum=0;
///    for(int i = 0; i < M.rows; i++)
///    {
///        const double* Mi = M.ptr<double>(i);
///        for(int j = 0; j < M.cols; j++)
///            sum += std::max(Mi[j], 0.);
///    }
/// ```
/// 
/// Some operations, like the one above, do not actually depend on the array shape. They just process
/// elements of an array one by one (or elements from multiple arrays that have the same coordinates,
/// for example, array addition). Such operations are called *element-wise*. It makes sense to check
/// whether all the input/output arrays are continuous, namely, have no gaps at the end of each row. If
/// yes, process them as a long single row:
/// ```ignore
///    // compute the sum of positive matrix elements, optimized variant
///    double sum=0;
///    int cols = M.cols, rows = M.rows;
///    if(M.isContinuous())
///    {
///        cols *= rows;
///        rows = 1;
///    }
///    for(int i = 0; i < rows; i++)
///    {
///        const double* Mi = M.ptr<double>(i);
///        for(int j = 0; j < cols; j++)
///            sum += std::max(Mi[j], 0.);
///    }
/// ```
/// 
/// In case of the continuous matrix, the outer loop body is executed just once. So, the overhead is
/// smaller, which is especially noticeable in case of small matrices.
/// 
/// Finally, there are STL-style iterators that are smart enough to skip gaps between successive rows:
/// ```ignore
///    // compute sum of positive matrix elements, iterator-based variant
///    double sum=0;
///    MatConstIterator_<double> it = M.begin<double>(), it_end = M.end<double>();
///    for(; it != it_end; ++it)
///        sum += std::max(*it, 0.);
/// ```
/// 
/// The matrix iterators are random-access iterators, so they can be passed to any STL algorithm,
/// including std::sort().
/// 
/// 
/// Note: Matrix Expressions and arithmetic see MatExpr
pub struct Mat {
	ptr: *mut c_void
}

opencv_type_boxed! { Mat }

impl Drop for Mat {
	fn drop(&mut self) {
		extern "C" { fn cv_Mat_delete(instance: *mut c_void); }
		unsafe { cv_Mat_delete(self.as_raw_mut_Mat()) };
	}
}

impl Mat {
	#[inline] pub fn as_raw_Mat(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Mat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Mat {}

impl core::MatTrait for Mat {
	#[inline] fn as_raw_Mat(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Mat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Mat {
	/// These are various constructors that form a matrix. As noted in the AutomaticAllocation, often
	/// the default constructor is enough, and the proper matrix will be allocated by an OpenCV function.
	/// The constructed matrix can further be assigned to another matrix or matrix expression or can be
	/// allocated with Mat::create . In the former case, the old content is de-referenced.
	pub fn default() -> Result<core::Mat> {
		unsafe { sys::cv_Mat_Mat() }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * rows: Number of rows in a 2D array.
	/// * cols: Number of columns in a 2D array.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	pub unsafe fn new_rows_cols(rows: i32, cols: i32, typ: i32) -> Result<core::Mat> {
		{ sys::cv_Mat_Mat_int_int_int(rows, cols, typ) }.into_result().map(|r| { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * size: 2D array size: Size(cols, rows) . In the Size() constructor, the number of rows and the
	///    number of columns go in the reverse order.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	pub unsafe fn new_size(size: core::Size, typ: i32) -> Result<core::Mat> {
		{ sys::cv_Mat_Mat_Size_int(size.opencv_to_extern(), typ) }.into_result().map(|r| { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * rows: Number of rows in a 2D array.
	/// * cols: Number of columns in a 2D array.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	/// * s: An optional value to initialize each matrix element with. To set all the matrix elements to
	///    the particular value after the construction, use the assignment operator
	///    Mat::operator=(const Scalar& value) .
	pub fn new_rows_cols_with_default(rows: i32, cols: i32, typ: i32, s: core::Scalar) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_Mat_int_int_int_const_ScalarR(rows, cols, typ, &s) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * size: 2D array size: Size(cols, rows) . In the Size() constructor, the number of rows and the
	///    number of columns go in the reverse order.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	/// * s: An optional value to initialize each matrix element with. To set all the matrix elements to
	///    the particular value after the construction, use the assignment operator
	///    Mat::operator=(const Scalar& value) .
	pub fn new_size_with_default(size: core::Size, typ: i32, s: core::Scalar) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_Mat_Size_int_const_ScalarR(size.opencv_to_extern(), typ, &s) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * ndims: Array dimensionality.
	/// * sizes: Array of integers specifying an n-dimensional array shape.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	pub unsafe fn new_nd(ndims: i32, sizes: &i32, typ: i32) -> Result<core::Mat> {
		{ sys::cv_Mat_Mat_int_const_intX_int(ndims, sizes, typ) }.into_result().map(|r| { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * sizes: Array of integers specifying an n-dimensional array shape.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	pub unsafe fn new_nd_vec(sizes: &core::Vector::<i32>, typ: i32) -> Result<core::Mat> {
		{ sys::cv_Mat_Mat_const_vector_int_R_int(sizes.as_raw_VectorOfi32(), typ) }.into_result().map(|r| { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * ndims: Array dimensionality.
	/// * sizes: Array of integers specifying an n-dimensional array shape.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	/// * s: An optional value to initialize each matrix element with. To set all the matrix elements to
	///    the particular value after the construction, use the assignment operator
	///    Mat::operator=(const Scalar& value) .
	pub fn new_nd_with_default(sizes: &[i32], typ: i32, s: core::Scalar) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_Mat_int_const_intX_int_const_ScalarR(sizes.len() as _, sizes.as_ptr(), typ, &s) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * sizes: Array of integers specifying an n-dimensional array shape.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	/// * s: An optional value to initialize each matrix element with. To set all the matrix elements to
	///    the particular value after the construction, use the assignment operator
	///    Mat::operator=(const Scalar& value) .
	pub fn new_nd_vec_with_default(sizes: &core::Vector::<i32>, typ: i32, s: core::Scalar) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_Mat_const_vector_int_R_int_const_ScalarR(sizes.as_raw_VectorOfi32(), typ, &s) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * m: Array that (as a whole or partly) is assigned to the constructed matrix. No data is copied
	///    by these constructors. Instead, the header pointing to m data or its sub-array is constructed and
	///    associated with it. The reference counter, if any, is incremented. So, when you modify the matrix
	///    formed using such a constructor, you also modify the corresponding elements of m . If you want to
	///    have an independent copy of the sub-array, use Mat::clone() .
	pub fn copy(m: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_Mat_const_MatR(m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * rows: Number of rows in a 2D array.
	/// * cols: Number of columns in a 2D array.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	/// * data: Pointer to the user data. Matrix constructors that take data and step parameters do not
	///    allocate matrix data. Instead, they just initialize the matrix header that points to the specified
	///    data, which means that no data is copied. This operation is very efficient and can be used to
	///    process external data using OpenCV functions. The external data is not automatically deallocated, so
	///    you should take care of it.
	/// * step: Number of bytes each matrix row occupies. The value should include the padding bytes at
	///    the end of each row, if any. If the parameter is missing (set to AUTO_STEP ), no padding is assumed
	///    and the actual step is calculated as cols*elemSize(). See Mat::elemSize.
	/// 
	/// ## C++ default parameters
	/// * step: AUTO_STEP
	pub unsafe fn new_rows_cols_with_data(rows: i32, cols: i32, typ: i32, data: *mut c_void, step: size_t) -> Result<core::Mat> {
		{ sys::cv_Mat_Mat_int_int_int_voidX_size_t(rows, cols, typ, data, step) }.into_result().map(|r| { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * size: 2D array size: Size(cols, rows) . In the Size() constructor, the number of rows and the
	///    number of columns go in the reverse order.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	/// * data: Pointer to the user data. Matrix constructors that take data and step parameters do not
	///    allocate matrix data. Instead, they just initialize the matrix header that points to the specified
	///    data, which means that no data is copied. This operation is very efficient and can be used to
	///    process external data using OpenCV functions. The external data is not automatically deallocated, so
	///    you should take care of it.
	/// * step: Number of bytes each matrix row occupies. The value should include the padding bytes at
	///    the end of each row, if any. If the parameter is missing (set to AUTO_STEP ), no padding is assumed
	///    and the actual step is calculated as cols*elemSize(). See Mat::elemSize.
	/// 
	/// ## C++ default parameters
	/// * step: AUTO_STEP
	pub unsafe fn new_size_with_data(size: core::Size, typ: i32, data: *mut c_void, step: size_t) -> Result<core::Mat> {
		{ sys::cv_Mat_Mat_Size_int_voidX_size_t(size.opencv_to_extern(), typ, data, step) }.into_result().map(|r| { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * ndims: Array dimensionality.
	/// * sizes: Array of integers specifying an n-dimensional array shape.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	/// * data: Pointer to the user data. Matrix constructors that take data and step parameters do not
	///    allocate matrix data. Instead, they just initialize the matrix header that points to the specified
	///    data, which means that no data is copied. This operation is very efficient and can be used to
	///    process external data using OpenCV functions. The external data is not automatically deallocated, so
	///    you should take care of it.
	/// * steps: Array of ndims-1 steps in case of a multi-dimensional array (the last step is always
	///    set to the element size). If not specified, the matrix is assumed to be continuous.
	/// 
	/// ## C++ default parameters
	/// * steps: 0
	pub unsafe fn new_nd_with_data(sizes: &[i32], typ: i32, data: *mut c_void, steps: &size_t) -> Result<core::Mat> {
		{ sys::cv_Mat_Mat_int_const_intX_int_voidX_const_size_tX(sizes.len() as _, sizes.as_ptr(), typ, data, steps) }.into_result().map(|r| { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * sizes: Array of integers specifying an n-dimensional array shape.
	/// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
	///    CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
	/// * data: Pointer to the user data. Matrix constructors that take data and step parameters do not
	///    allocate matrix data. Instead, they just initialize the matrix header that points to the specified
	///    data, which means that no data is copied. This operation is very efficient and can be used to
	///    process external data using OpenCV functions. The external data is not automatically deallocated, so
	///    you should take care of it.
	/// * steps: Array of ndims-1 steps in case of a multi-dimensional array (the last step is always
	///    set to the element size). If not specified, the matrix is assumed to be continuous.
	/// 
	/// ## C++ default parameters
	/// * steps: 0
	pub unsafe fn new_nd_vec_with_data(sizes: &core::Vector::<i32>, typ: i32, data: *mut c_void, steps: &size_t) -> Result<core::Mat> {
		{ sys::cv_Mat_Mat_const_vector_int_R_int_voidX_const_size_tX(sizes.as_raw_VectorOfi32(), typ, data, steps) }.into_result().map(|r| { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * m: Array that (as a whole or partly) is assigned to the constructed matrix. No data is copied
	///    by these constructors. Instead, the header pointing to m data or its sub-array is constructed and
	///    associated with it. The reference counter, if any, is incremented. So, when you modify the matrix
	///    formed using such a constructor, you also modify the corresponding elements of m . If you want to
	///    have an independent copy of the sub-array, use Mat::clone() .
	/// * rowRange: Range of the m rows to take. As usual, the range start is inclusive and the range
	///    end is exclusive. Use Range::all() to take all the rows.
	/// * colRange: Range of the m columns to take. Use Range::all() to take all the columns.
	/// 
	/// ## C++ default parameters
	/// * col_range: Range::all()
	pub fn rowscols(m: &core::Mat, row_range: &core::Range, col_range: &core::Range) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_Mat_const_MatR_const_RangeR_const_RangeR(m.as_raw_Mat(), row_range.as_raw_Range(), col_range.as_raw_Range()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * m: Array that (as a whole or partly) is assigned to the constructed matrix. No data is copied
	///    by these constructors. Instead, the header pointing to m data or its sub-array is constructed and
	///    associated with it. The reference counter, if any, is incremented. So, when you modify the matrix
	///    formed using such a constructor, you also modify the corresponding elements of m . If you want to
	///    have an independent copy of the sub-array, use Mat::clone() .
	/// * roi: Region of interest.
	pub fn roi(m: &core::Mat, roi: core::Rect) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_Mat_const_MatR_const_RectR(m.as_raw_Mat(), &roi) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * m: Array that (as a whole or partly) is assigned to the constructed matrix. No data is copied
	///    by these constructors. Instead, the header pointing to m data or its sub-array is constructed and
	///    associated with it. The reference counter, if any, is incremented. So, when you modify the matrix
	///    formed using such a constructor, you also modify the corresponding elements of m . If you want to
	///    have an independent copy of the sub-array, use Mat::clone() .
	/// * ranges: Array of selected ranges of m along each dimensionality.
	pub fn ranges(m: &core::Mat, ranges: &core::Vector::<core::Range>) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_Mat_const_MatR_const_vector_Range_R(m.as_raw_Mat(), ranges.as_raw_VectorOfRange()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// download data from GpuMat
	pub fn new(m: &core::GpuMat) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_Mat_const_GpuMatR(m.as_raw_GpuMat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// creates a diagonal matrix
	/// 
	/// The method creates a square diagonal matrix from specified main diagonal.
	/// ## Parameters
	/// * d: One-dimensional matrix that represents the main diagonal.
	pub fn diag_mat(d: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_diag_const_MatR(d.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Returns a zero array of the specified size and type.
	/// 
	/// The method returns a Matlab-style zero array initializer. It can be used to quickly form a constant
	/// array as a function parameter, part of a matrix expression, or as a matrix initializer:
	/// ```ignore
	///    Mat A;
	///    A = Mat::zeros(3, 3, CV_32F);
	/// ```
	/// 
	/// In the example above, a new matrix is allocated only if A is not a 3x3 floating-point matrix.
	/// Otherwise, the existing matrix A is filled with zeros.
	/// ## Parameters
	/// * rows: Number of rows.
	/// * cols: Number of columns.
	/// * type: Created matrix type.
	pub fn zeros(rows: i32, cols: i32, typ: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_Mat_zeros_int_int_int(rows, cols, typ) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// Returns a zero array of the specified size and type.
	/// 
	/// The method returns a Matlab-style zero array initializer. It can be used to quickly form a constant
	/// array as a function parameter, part of a matrix expression, or as a matrix initializer:
	/// ```ignore
	///    Mat A;
	///    A = Mat::zeros(3, 3, CV_32F);
	/// ```
	/// 
	/// In the example above, a new matrix is allocated only if A is not a 3x3 floating-point matrix.
	/// Otherwise, the existing matrix A is filled with zeros.
	/// ## Parameters
	/// * rows: Number of rows.
	/// * cols: Number of columns.
	/// * type: Created matrix type.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * size: Alternative to the matrix size specification Size(cols, rows) .
	/// * type: Created matrix type.
	pub fn zeros_size(size: core::Size, typ: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_Mat_zeros_Size_int(size.opencv_to_extern(), typ) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// Returns a zero array of the specified size and type.
	/// 
	/// The method returns a Matlab-style zero array initializer. It can be used to quickly form a constant
	/// array as a function parameter, part of a matrix expression, or as a matrix initializer:
	/// ```ignore
	///    Mat A;
	///    A = Mat::zeros(3, 3, CV_32F);
	/// ```
	/// 
	/// In the example above, a new matrix is allocated only if A is not a 3x3 floating-point matrix.
	/// Otherwise, the existing matrix A is filled with zeros.
	/// ## Parameters
	/// * rows: Number of rows.
	/// * cols: Number of columns.
	/// * type: Created matrix type.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * ndims: Array dimensionality.
	/// * sz: Array of integers specifying the array shape.
	/// * type: Created matrix type.
	pub fn zeros_nd(sz: &[i32], typ: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_Mat_zeros_int_const_intX_int(sz.len() as _, sz.as_ptr(), typ) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// Returns an array of all 1's of the specified size and type.
	/// 
	/// The method returns a Matlab-style 1's array initializer, similarly to Mat::zeros. Note that using
	/// this method you can initialize an array with an arbitrary value, using the following Matlab idiom:
	/// ```ignore
	///    Mat A = Mat::ones(100, 100, CV_8U)*3; // make 100x100 matrix filled with 3.
	/// ```
	/// 
	/// The above operation does not form a 100x100 matrix of 1's and then multiply it by 3. Instead, it
	/// just remembers the scale factor (3 in this case) and use it when actually invoking the matrix
	/// initializer.
	/// 
	/// Note: In case of multi-channels type, only the first channel will be initialized with 1's, the
	/// others will be set to 0's.
	/// ## Parameters
	/// * rows: Number of rows.
	/// * cols: Number of columns.
	/// * type: Created matrix type.
	pub fn ones(rows: i32, cols: i32, typ: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_Mat_ones_int_int_int(rows, cols, typ) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// Returns an array of all 1's of the specified size and type.
	/// 
	/// The method returns a Matlab-style 1's array initializer, similarly to Mat::zeros. Note that using
	/// this method you can initialize an array with an arbitrary value, using the following Matlab idiom:
	/// ```ignore
	///    Mat A = Mat::ones(100, 100, CV_8U)*3; // make 100x100 matrix filled with 3.
	/// ```
	/// 
	/// The above operation does not form a 100x100 matrix of 1's and then multiply it by 3. Instead, it
	/// just remembers the scale factor (3 in this case) and use it when actually invoking the matrix
	/// initializer.
	/// 
	/// Note: In case of multi-channels type, only the first channel will be initialized with 1's, the
	/// others will be set to 0's.
	/// ## Parameters
	/// * rows: Number of rows.
	/// * cols: Number of columns.
	/// * type: Created matrix type.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * size: Alternative to the matrix size specification Size(cols, rows) .
	/// * type: Created matrix type.
	pub fn ones_size(size: core::Size, typ: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_Mat_ones_Size_int(size.opencv_to_extern(), typ) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// Returns an array of all 1's of the specified size and type.
	/// 
	/// The method returns a Matlab-style 1's array initializer, similarly to Mat::zeros. Note that using
	/// this method you can initialize an array with an arbitrary value, using the following Matlab idiom:
	/// ```ignore
	///    Mat A = Mat::ones(100, 100, CV_8U)*3; // make 100x100 matrix filled with 3.
	/// ```
	/// 
	/// The above operation does not form a 100x100 matrix of 1's and then multiply it by 3. Instead, it
	/// just remembers the scale factor (3 in this case) and use it when actually invoking the matrix
	/// initializer.
	/// 
	/// Note: In case of multi-channels type, only the first channel will be initialized with 1's, the
	/// others will be set to 0's.
	/// ## Parameters
	/// * rows: Number of rows.
	/// * cols: Number of columns.
	/// * type: Created matrix type.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * ndims: Array dimensionality.
	/// * sz: Array of integers specifying the array shape.
	/// * type: Created matrix type.
	pub fn ones_nd(sz: &[i32], typ: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_Mat_ones_int_const_intX_int(sz.len() as _, sz.as_ptr(), typ) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// Returns an identity matrix of the specified size and type.
	/// 
	/// The method returns a Matlab-style identity matrix initializer, similarly to Mat::zeros. Similarly to
	/// Mat::ones, you can use a scale operation to create a scaled identity matrix efficiently:
	/// ```ignore
	///    // make a 4x4 diagonal matrix with 0.1's on the diagonal.
	///    Mat A = Mat::eye(4, 4, CV_32F)*0.1;
	/// ```
	/// 
	/// 
	/// Note: In case of multi-channels type, identity matrix will be initialized only for the first channel,
	/// the others will be set to 0's
	/// ## Parameters
	/// * rows: Number of rows.
	/// * cols: Number of columns.
	/// * type: Created matrix type.
	pub fn eye(rows: i32, cols: i32, typ: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_Mat_eye_int_int_int(rows, cols, typ) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// Returns an identity matrix of the specified size and type.
	/// 
	/// The method returns a Matlab-style identity matrix initializer, similarly to Mat::zeros. Similarly to
	/// Mat::ones, you can use a scale operation to create a scaled identity matrix efficiently:
	/// ```ignore
	///    // make a 4x4 diagonal matrix with 0.1's on the diagonal.
	///    Mat A = Mat::eye(4, 4, CV_32F)*0.1;
	/// ```
	/// 
	/// 
	/// Note: In case of multi-channels type, identity matrix will be initialized only for the first channel,
	/// the others will be set to 0's
	/// ## Parameters
	/// * rows: Number of rows.
	/// * cols: Number of columns.
	/// * type: Created matrix type.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * size: Alternative matrix size specification as Size(cols, rows) .
	/// * type: Created matrix type.
	pub fn eye_size(size: core::Size, typ: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_Mat_eye_Size_int(size.opencv_to_extern(), typ) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	pub fn copy_mut(m: &mut core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_Mat_Mat_MatR(m.as_raw_mut_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// /////////////////////////////// MatConstIterator //////////////////////////////////
pub trait MatConstIteratorTrait {
	fn as_raw_MatConstIterator(&self) -> *const c_void;
	fn as_raw_mut_MatConstIterator(&mut self) -> *mut c_void;

	fn m(&self) -> core::Mat {
		unsafe { sys::cv_MatConstIterator_getPropM_const(self.as_raw_MatConstIterator()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: m")
	}
	
	fn elem_size(&self) -> size_t {
		unsafe { sys::cv_MatConstIterator_getPropElemSize_const(self.as_raw_MatConstIterator()) }.into_result().expect("Infallible function failed: elem_size")
	}
	
	fn set_elem_size(&mut self, val: size_t) -> () {
		unsafe { sys::cv_MatConstIterator_setPropElemSize_size_t(self.as_raw_mut_MatConstIterator(), val) }.into_result().expect("Infallible function failed: set_elem_size")
	}
	
	fn ptr(&self) -> &u8 {
		unsafe { sys::cv_MatConstIterator_getPropPtr_const(self.as_raw_MatConstIterator()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: ptr")
	}
	
	fn slice_start(&self) -> &u8 {
		unsafe { sys::cv_MatConstIterator_getPropSliceStart_const(self.as_raw_MatConstIterator()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: slice_start")
	}
	
	fn slice_end(&self) -> &u8 {
		unsafe { sys::cv_MatConstIterator_getPropSliceEnd_const(self.as_raw_MatConstIterator()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: slice_end")
	}
	
	/// returns the current matrix element
	fn try_deref(&self) -> Result<&u8> {
		unsafe { sys::cv_MatConstIterator_operatorX_const(self.as_raw_MatConstIterator()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// returns the i-th matrix element, relative to the current
	fn get(&self, i: ptrdiff_t) -> Result<&u8> {
		unsafe { sys::cv_MatConstIterator_operator___const_ptrdiff_t(self.as_raw_MatConstIterator(), i) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// returns the current iterator position
	fn pos(&self) -> Result<core::Point> {
		unsafe { sys::cv_MatConstIterator_pos_const(self.as_raw_MatConstIterator()) }.into_result()
	}
	
	/// returns the current iterator position
	fn pos_to(&self, _idx: &mut i32) -> Result<()> {
		unsafe { sys::cv_MatConstIterator_pos_const_intX(self.as_raw_MatConstIterator(), _idx) }.into_result()
	}
	
	fn lpos(&self) -> Result<ptrdiff_t> {
		unsafe { sys::cv_MatConstIterator_lpos_const(self.as_raw_MatConstIterator()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * relative: false
	fn seek(&mut self, ofs: ptrdiff_t, relative: bool) -> Result<()> {
		unsafe { sys::cv_MatConstIterator_seek_ptrdiff_t_bool(self.as_raw_mut_MatConstIterator(), ofs, relative) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * relative: false
	fn seek_idx(&mut self, _idx: &i32, relative: bool) -> Result<()> {
		unsafe { sys::cv_MatConstIterator_seek_const_intX_bool(self.as_raw_mut_MatConstIterator(), _idx, relative) }.into_result()
	}
	
}

/// /////////////////////////////// MatConstIterator //////////////////////////////////
pub struct MatConstIterator {
	ptr: *mut c_void
}

opencv_type_boxed! { MatConstIterator }

impl Drop for MatConstIterator {
	fn drop(&mut self) {
		extern "C" { fn cv_MatConstIterator_delete(instance: *mut c_void); }
		unsafe { cv_MatConstIterator_delete(self.as_raw_mut_MatConstIterator()) };
	}
}

impl MatConstIterator {
	#[inline] pub fn as_raw_MatConstIterator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MatConstIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MatConstIterator {}

impl core::MatConstIteratorTrait for MatConstIterator {
	#[inline] fn as_raw_MatConstIterator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MatConstIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MatConstIterator {
	/// default constructor
	pub fn default() -> Result<core::MatConstIterator> {
		unsafe { sys::cv_MatConstIterator_MatConstIterator() }.into_result().map(|r| unsafe { core::MatConstIterator::opencv_from_extern(r) } )
	}
	
	/// constructor that sets the iterator to the beginning of the matrix
	pub fn over(_m: &core::Mat) -> Result<core::MatConstIterator> {
		unsafe { sys::cv_MatConstIterator_MatConstIterator_const_MatX(_m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatConstIterator::opencv_from_extern(r) } )
	}
	
	/// constructor that sets the iterator to the specified element of the matrix
	/// 
	/// ## C++ default parameters
	/// * _col: 0
	pub fn with_rows_cols(_m: &core::Mat, _row: i32, _col: i32) -> Result<core::MatConstIterator> {
		unsafe { sys::cv_MatConstIterator_MatConstIterator_const_MatX_int_int(_m.as_raw_Mat(), _row, _col) }.into_result().map(|r| unsafe { core::MatConstIterator::opencv_from_extern(r) } )
	}
	
	/// constructor that sets the iterator to the specified element of the matrix
	pub fn with_start(_m: &core::Mat, _pt: core::Point) -> Result<core::MatConstIterator> {
		unsafe { sys::cv_MatConstIterator_MatConstIterator_const_MatX_Point(_m.as_raw_Mat(), _pt.opencv_to_extern()) }.into_result().map(|r| unsafe { core::MatConstIterator::opencv_from_extern(r) } )
	}
	
	#[cfg(not(target_os = "windows"))]
	/// constructor that sets the iterator to the specified element of the matrix
	pub fn new_slice(_m: &core::Mat, _idx: &i32) -> Result<core::MatConstIterator> {
		unsafe { sys::cv_MatConstIterator_MatConstIterator_const_MatX_const_intX(_m.as_raw_Mat(), _idx) }.into_result().map(|r| unsafe { core::MatConstIterator::opencv_from_extern(r) } )
	}
	
	/// copy constructor
	pub fn copy(it: &core::MatConstIterator) -> Result<core::MatConstIterator> {
		unsafe { sys::cv_MatConstIterator_MatConstIterator_const_MatConstIteratorR(it.as_raw_MatConstIterator()) }.into_result().map(|r| unsafe { core::MatConstIterator::opencv_from_extern(r) } )
	}
	
}

/// Matrix expression representation
/// @anchor MatrixExpressions
/// This is a list of implemented matrix operations that can be combined in arbitrary complex
/// expressions (here A, B stand for matrices ( Mat ), s for a scalar ( Scalar ), alpha for a
/// real-valued scalar ( double )):
/// *   Addition, subtraction, negation: `A+B`, `A-B`, `A+s`, `A-s`, `s+A`, `s-A`, `-A`
/// *   Scaling: `A*alpha`
/// *   Per-element multiplication and division: `A.mul(B)`, `A/B`, `alpha/A`
/// *   Matrix multiplication: `A*B`
/// *   Transposition: `A.t()` (means A<sup>T</sup>)
/// *   Matrix inversion and pseudo-inversion, solving linear systems and least-squares problems:
///    `A.inv([method]) (~ A<sup>-1</sup>)`,   `A.inv([method])*B (~ X: AX=B)`
/// *   Comparison: `A cmpop B`, `A cmpop alpha`, `alpha cmpop A`, where *cmpop* is one of
///   `>`, `>=`, `==`, `!=`, `<=`, `<`. The result of comparison is an 8-bit single channel mask whose
///    elements are set to 255 (if the particular element or pair of elements satisfy the condition) or
///    0.
/// *   Bitwise logical operations: `A logicop B`, `A logicop s`, `s logicop A`, `~A`, where *logicop* is one of
///   `&`, `|`, `^`.
/// *   Element-wise minimum and maximum: `min(A, B)`, `min(A, alpha)`, `max(A, B)`, `max(A, alpha)`
/// *   Element-wise absolute value: `abs(A)`
/// *   Cross-product, dot-product: `A.cross(B)`, `A.dot(B)`
/// *   Any function of matrix or matrices and scalars that returns a matrix or a scalar, such as norm,
///    mean, sum, countNonZero, trace, determinant, repeat, and others.
/// *   Matrix initializers ( Mat::eye(), Mat::zeros(), Mat::ones() ), matrix comma-separated
///    initializers, matrix constructors and operators that extract sub-matrices (see Mat description).
/// *   Mat_<destination_type>() constructors to cast the result to the proper type.
/// 
/// Note: Comma-separated initializers and probably some other operations may require additional
/// explicit Mat() or Mat_<T>() constructor calls to resolve a possible ambiguity.
/// 
/// Here are examples of matrix expressions:
/// ```ignore
///    // compute pseudo-inverse of A, equivalent to A.inv(DECOMP_SVD)
///    SVD svd(A);
///    Mat pinvA = svd.vt.t()*Mat::diag(1./svd.w)*svd.u.t();
/// 
///    // compute the new vector of parameters in the Levenberg-Marquardt algorithm
///    x -= (A.t()*A + lambda*Mat::eye(A.cols,A.cols,A.type())).inv(DECOMP_CHOLESKY)*(A.t()*err);
/// 
///    // sharpen image using "unsharp mask" algorithm
///    Mat blurred; double sigma = 1, threshold = 5, amount = 1;
///    GaussianBlur(img, blurred, Size(), sigma, sigma);
///    Mat lowContrastMask = abs(img - blurred) < threshold;
///    Mat sharpened = img*(1+amount) + blurred*(-amount);
///    img.copyTo(sharpened, lowContrastMask);
/// ```
/// 
pub trait MatExprTrait {
	fn as_raw_MatExpr(&self) -> *const c_void;
	fn as_raw_mut_MatExpr(&mut self) -> *mut c_void;

	fn flags(&self) -> i32 {
		unsafe { sys::cv_MatExpr_getPropFlags_const(self.as_raw_MatExpr()) }.into_result().expect("Infallible function failed: flags")
	}
	
	fn set_flags(&mut self, val: i32) -> () {
		unsafe { sys::cv_MatExpr_setPropFlags_int(self.as_raw_mut_MatExpr(), val) }.into_result().expect("Infallible function failed: set_flags")
	}
	
	fn a(&mut self) -> core::Mat {
		unsafe { sys::cv_MatExpr_getPropA(self.as_raw_mut_MatExpr()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: a")
	}
	
	fn set_a(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_MatExpr_setPropA_Mat(self.as_raw_mut_MatExpr(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_a")
	}
	
	fn b(&mut self) -> core::Mat {
		unsafe { sys::cv_MatExpr_getPropB(self.as_raw_mut_MatExpr()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: b")
	}
	
	fn set_b(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_MatExpr_setPropB_Mat(self.as_raw_mut_MatExpr(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_b")
	}
	
	fn c(&mut self) -> core::Mat {
		unsafe { sys::cv_MatExpr_getPropC(self.as_raw_mut_MatExpr()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: c")
	}
	
	fn set_c(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_MatExpr_setPropC_Mat(self.as_raw_mut_MatExpr(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_c")
	}
	
	fn alpha(&self) -> f64 {
		unsafe { sys::cv_MatExpr_getPropAlpha_const(self.as_raw_MatExpr()) }.into_result().expect("Infallible function failed: alpha")
	}
	
	fn set_alpha(&mut self, val: f64) -> () {
		unsafe { sys::cv_MatExpr_setPropAlpha_double(self.as_raw_mut_MatExpr(), val) }.into_result().expect("Infallible function failed: set_alpha")
	}
	
	fn beta(&self) -> f64 {
		unsafe { sys::cv_MatExpr_getPropBeta_const(self.as_raw_MatExpr()) }.into_result().expect("Infallible function failed: beta")
	}
	
	fn set_beta(&mut self, val: f64) -> () {
		unsafe { sys::cv_MatExpr_setPropBeta_double(self.as_raw_mut_MatExpr(), val) }.into_result().expect("Infallible function failed: set_beta")
	}
	
	fn s(&self) -> core::Scalar {
		unsafe { sys::cv_MatExpr_getPropS_const(self.as_raw_MatExpr()) }.into_result().expect("Infallible function failed: s")
	}
	
	fn set_s(&mut self, val: core::Scalar) -> () {
		unsafe { sys::cv_MatExpr_setPropS_Scalar(self.as_raw_mut_MatExpr(), val.opencv_to_extern()) }.into_result().expect("Infallible function failed: set_s")
	}
	
	fn to_mat(&self) -> Result<core::Mat> {
		unsafe { sys::cv_MatExpr_operator_cv_Mat_const(self.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn size(&self) -> Result<core::Size> {
		unsafe { sys::cv_MatExpr_size_const(self.as_raw_MatExpr()) }.into_result()
	}
	
	fn typ(&self) -> Result<i32> {
		unsafe { sys::cv_MatExpr_type_const(self.as_raw_MatExpr()) }.into_result()
	}
	
	fn row(&self, y: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_MatExpr_row_const_int(self.as_raw_MatExpr(), y) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	fn col(&self, x: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_MatExpr_col_const_int(self.as_raw_MatExpr(), x) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * d: 0
	fn diag(&self, d: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_MatExpr_diag_const_int(self.as_raw_MatExpr(), d) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	fn t(&self) -> Result<core::MatExpr> {
		unsafe { sys::cv_MatExpr_t_const(self.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * method: DECOMP_LU
	fn inv(&self, method: i32) -> Result<core::MatExpr> {
		unsafe { sys::cv_MatExpr_inv_const_int(self.as_raw_MatExpr(), method) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * scale: 1
	fn mul_matexpr(&self, e: &core::MatExpr, scale: f64) -> Result<core::MatExpr> {
		unsafe { sys::cv_MatExpr_mul_const_const_MatExprR_double(self.as_raw_MatExpr(), e.as_raw_MatExpr(), scale) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * scale: 1
	fn mul(&self, m: &core::Mat, scale: f64) -> Result<core::MatExpr> {
		unsafe { sys::cv_MatExpr_mul_const_const_MatR_double(self.as_raw_MatExpr(), m.as_raw_Mat(), scale) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	fn cross(&self, m: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_MatExpr_cross_const_const_MatR(self.as_raw_MatExpr(), m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn dot(&self, m: &core::Mat) -> Result<f64> {
		unsafe { sys::cv_MatExpr_dot_const_const_MatR(self.as_raw_MatExpr(), m.as_raw_Mat()) }.into_result()
	}
	
}

/// Matrix expression representation
/// @anchor MatrixExpressions
/// This is a list of implemented matrix operations that can be combined in arbitrary complex
/// expressions (here A, B stand for matrices ( Mat ), s for a scalar ( Scalar ), alpha for a
/// real-valued scalar ( double )):
/// *   Addition, subtraction, negation: `A+B`, `A-B`, `A+s`, `A-s`, `s+A`, `s-A`, `-A`
/// *   Scaling: `A*alpha`
/// *   Per-element multiplication and division: `A.mul(B)`, `A/B`, `alpha/A`
/// *   Matrix multiplication: `A*B`
/// *   Transposition: `A.t()` (means A<sup>T</sup>)
/// *   Matrix inversion and pseudo-inversion, solving linear systems and least-squares problems:
///    `A.inv([method]) (~ A<sup>-1</sup>)`,   `A.inv([method])*B (~ X: AX=B)`
/// *   Comparison: `A cmpop B`, `A cmpop alpha`, `alpha cmpop A`, where *cmpop* is one of
///   `>`, `>=`, `==`, `!=`, `<=`, `<`. The result of comparison is an 8-bit single channel mask whose
///    elements are set to 255 (if the particular element or pair of elements satisfy the condition) or
///    0.
/// *   Bitwise logical operations: `A logicop B`, `A logicop s`, `s logicop A`, `~A`, where *logicop* is one of
///   `&`, `|`, `^`.
/// *   Element-wise minimum and maximum: `min(A, B)`, `min(A, alpha)`, `max(A, B)`, `max(A, alpha)`
/// *   Element-wise absolute value: `abs(A)`
/// *   Cross-product, dot-product: `A.cross(B)`, `A.dot(B)`
/// *   Any function of matrix or matrices and scalars that returns a matrix or a scalar, such as norm,
///    mean, sum, countNonZero, trace, determinant, repeat, and others.
/// *   Matrix initializers ( Mat::eye(), Mat::zeros(), Mat::ones() ), matrix comma-separated
///    initializers, matrix constructors and operators that extract sub-matrices (see Mat description).
/// *   Mat_<destination_type>() constructors to cast the result to the proper type.
/// 
/// Note: Comma-separated initializers and probably some other operations may require additional
/// explicit Mat() or Mat_<T>() constructor calls to resolve a possible ambiguity.
/// 
/// Here are examples of matrix expressions:
/// ```ignore
///    // compute pseudo-inverse of A, equivalent to A.inv(DECOMP_SVD)
///    SVD svd(A);
///    Mat pinvA = svd.vt.t()*Mat::diag(1./svd.w)*svd.u.t();
/// 
///    // compute the new vector of parameters in the Levenberg-Marquardt algorithm
///    x -= (A.t()*A + lambda*Mat::eye(A.cols,A.cols,A.type())).inv(DECOMP_CHOLESKY)*(A.t()*err);
/// 
///    // sharpen image using "unsharp mask" algorithm
///    Mat blurred; double sigma = 1, threshold = 5, amount = 1;
///    GaussianBlur(img, blurred, Size(), sigma, sigma);
///    Mat lowContrastMask = abs(img - blurred) < threshold;
///    Mat sharpened = img*(1+amount) + blurred*(-amount);
///    img.copyTo(sharpened, lowContrastMask);
/// ```
/// 
pub struct MatExpr {
	ptr: *mut c_void
}

opencv_type_boxed! { MatExpr }

impl Drop for MatExpr {
	fn drop(&mut self) {
		extern "C" { fn cv_MatExpr_delete(instance: *mut c_void); }
		unsafe { cv_MatExpr_delete(self.as_raw_mut_MatExpr()) };
	}
}

impl MatExpr {
	#[inline] pub fn as_raw_MatExpr(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MatExpr(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MatExpr {}

impl core::MatExprTrait for MatExpr {
	#[inline] fn as_raw_MatExpr(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MatExpr(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MatExpr {
	pub fn default() -> Result<core::MatExpr> {
		unsafe { sys::cv_MatExpr_MatExpr() }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	pub fn from_mat(m: &core::Mat) -> Result<core::MatExpr> {
		unsafe { sys::cv_MatExpr_MatExpr_const_MatR(m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * _a: Mat()
	/// * _b: Mat()
	/// * _c: Mat()
	/// * _alpha: 1
	/// * _beta: 1
	/// * _s: Scalar()
	pub fn new(_op: &dyn core::MatOp, _flags: i32, _a: &core::Mat, _b: &core::Mat, _c: &core::Mat, _alpha: f64, _beta: f64, _s: core::Scalar) -> Result<core::MatExpr> {
		unsafe { sys::cv_MatExpr_MatExpr_const_MatOpX_int_const_MatR_const_MatR_const_MatR_double_double_const_ScalarR(_op.as_raw_MatOp(), _flags, _a.as_raw_Mat(), _b.as_raw_Mat(), _c.as_raw_Mat(), _alpha, _beta, &_s) }.into_result().map(|r| unsafe { core::MatExpr::opencv_from_extern(r) } )
	}
	
}

/// ////////////////////////////// Matrix Expressions /////////////////////////////////
pub trait MatOp {
	fn as_raw_MatOp(&self) -> *const c_void;
	fn as_raw_mut_MatOp(&mut self) -> *mut c_void;

	fn element_wise(&self, expr: &core::MatExpr) -> Result<bool> {
		unsafe { sys::cv_MatOp_elementWise_const_const_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * typ: -1
	fn assign(&self, expr: &core::MatExpr, m: &mut core::Mat, typ: i32) -> Result<()> {
		unsafe { sys::cv_MatOp_assign_const_const_MatExprR_MatR_int(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat(), typ) }.into_result()
	}
	
	fn roi(&self, expr: &core::MatExpr, row_range: &core::Range, col_range: &core::Range, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_roi_const_const_MatExprR_const_RangeR_const_RangeR_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), row_range.as_raw_Range(), col_range.as_raw_Range(), res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	fn diag(&self, expr: &core::MatExpr, d: i32, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_diag_const_const_MatExprR_int_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), d, res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	fn aug_assign_add(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_MatOp_augAssignAdd_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat()) }.into_result()
	}
	
	fn aug_assign_subtract(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_MatOp_augAssignSubtract_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat()) }.into_result()
	}
	
	fn aug_assign_multiply(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_MatOp_augAssignMultiply_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat()) }.into_result()
	}
	
	fn aug_assign_divide(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_MatOp_augAssignDivide_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat()) }.into_result()
	}
	
	fn aug_assign_and(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_MatOp_augAssignAnd_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat()) }.into_result()
	}
	
	fn aug_assign_or(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_MatOp_augAssignOr_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat()) }.into_result()
	}
	
	fn aug_assign_xor(&self, expr: &core::MatExpr, m: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_MatOp_augAssignXor_const_const_MatExprR_MatR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), m.as_raw_mut_Mat()) }.into_result()
	}
	
	fn add(&self, expr1: &core::MatExpr, expr2: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_add_const_const_MatExprR_const_MatExprR_MatExprR(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), expr2.as_raw_MatExpr(), res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	fn add_scalar(&self, expr1: &core::MatExpr, s: core::Scalar, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_add_const_const_MatExprR_const_ScalarR_MatExprR(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), &s, res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	fn subtract(&self, expr1: &core::MatExpr, expr2: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_subtract_const_const_MatExprR_const_MatExprR_MatExprR(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), expr2.as_raw_MatExpr(), res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	fn subtract_scalar(&self, s: core::Scalar, expr: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_subtract_const_const_ScalarR_const_MatExprR_MatExprR(self.as_raw_MatOp(), &s, expr.as_raw_MatExpr(), res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * scale: 1
	fn multiply(&self, expr1: &core::MatExpr, expr2: &core::MatExpr, res: &mut core::MatExpr, scale: f64) -> Result<()> {
		unsafe { sys::cv_MatOp_multiply_const_const_MatExprR_const_MatExprR_MatExprR_double(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), expr2.as_raw_MatExpr(), res.as_raw_mut_MatExpr(), scale) }.into_result()
	}
	
	fn multiply_f64(&self, expr1: &core::MatExpr, s: f64, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_multiply_const_const_MatExprR_double_MatExprR(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), s, res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * scale: 1
	fn divide(&self, expr1: &core::MatExpr, expr2: &core::MatExpr, res: &mut core::MatExpr, scale: f64) -> Result<()> {
		unsafe { sys::cv_MatOp_divide_const_const_MatExprR_const_MatExprR_MatExprR_double(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), expr2.as_raw_MatExpr(), res.as_raw_mut_MatExpr(), scale) }.into_result()
	}
	
	fn divide_f64(&self, s: f64, expr: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_divide_const_double_const_MatExprR_MatExprR(self.as_raw_MatOp(), s, expr.as_raw_MatExpr(), res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	fn abs(&self, expr: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_abs_const_const_MatExprR_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	fn transpose(&self, expr: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_transpose_const_const_MatExprR_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	fn matmul(&self, expr1: &core::MatExpr, expr2: &core::MatExpr, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_matmul_const_const_MatExprR_const_MatExprR_MatExprR(self.as_raw_MatOp(), expr1.as_raw_MatExpr(), expr2.as_raw_MatExpr(), res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	fn invert(&self, expr: &core::MatExpr, method: i32, res: &mut core::MatExpr) -> Result<()> {
		unsafe { sys::cv_MatOp_invert_const_const_MatExprR_int_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr(), method, res.as_raw_mut_MatExpr()) }.into_result()
	}
	
	fn size(&self, expr: &core::MatExpr) -> Result<core::Size> {
		unsafe { sys::cv_MatOp_size_const_const_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr()) }.into_result()
	}
	
	fn typ(&self, expr: &core::MatExpr) -> Result<i32> {
		unsafe { sys::cv_MatOp_type_const_const_MatExprR(self.as_raw_MatOp(), expr.as_raw_MatExpr()) }.into_result()
	}
	
}

pub trait MatSizeTrait {
	fn as_raw_MatSize(&self) -> *const c_void;
	fn as_raw_mut_MatSize(&mut self) -> *mut c_void;

	fn p(&mut self) -> &mut i32 {
		unsafe { sys::cv_MatSize_getPropP(self.as_raw_mut_MatSize()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: p")
	}
	
	fn set_p(&mut self, val: &mut i32) -> () {
		unsafe { sys::cv_MatSize_setPropP_intX(self.as_raw_mut_MatSize(), val) }.into_result().expect("Infallible function failed: set_p")
	}
	
	fn dims(&self) -> Result<i32> {
		unsafe { sys::cv_MatSize_dims_const(self.as_raw_MatSize()) }.into_result()
	}
	
	fn get(&self, i: i32) -> Result<i32> {
		unsafe { sys::cv_MatSize_operator___const_int(self.as_raw_MatSize(), i) }.into_result()
	}
	
	fn get_mut(&mut self, i: i32) -> Result<i32> {
		unsafe { sys::cv_MatSize_operator___int(self.as_raw_mut_MatSize(), i) }.into_result()
	}
	
	fn to_ri32(&self) -> Result<&i32> {
		unsafe { sys::cv_MatSize_operator_const_intX_const(self.as_raw_MatSize()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
}

pub struct MatSize {
	ptr: *mut c_void
}

opencv_type_boxed! { MatSize }

impl Drop for MatSize {
	fn drop(&mut self) {
		extern "C" { fn cv_MatSize_delete(instance: *mut c_void); }
		unsafe { cv_MatSize_delete(self.as_raw_mut_MatSize()) };
	}
}

impl MatSize {
	#[inline] pub fn as_raw_MatSize(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MatSize(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MatSize {}

impl core::MatSizeTrait for MatSize {
	#[inline] fn as_raw_MatSize(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MatSize(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MatSize {
	/// ////////////////////////// MatSize ////////////////////////////
	pub fn new(_p: &mut i32) -> Result<core::MatSize> {
		unsafe { sys::cv_MatSize_MatSize_intX(_p) }.into_result().map(|r| unsafe { core::MatSize::opencv_from_extern(r) } )
	}
	
}

pub trait MatStepTrait {
	fn as_raw_MatStep(&self) -> *const c_void;
	fn as_raw_mut_MatStep(&mut self) -> *mut c_void;

	fn p(&mut self) -> &mut size_t {
		unsafe { sys::cv_MatStep_getPropP(self.as_raw_mut_MatStep()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: p")
	}
	
	fn set_p(&mut self, val: &mut size_t) -> () {
		unsafe { sys::cv_MatStep_setPropP_size_tX(self.as_raw_mut_MatStep(), val) }.into_result().expect("Infallible function failed: set_p")
	}
	
	fn buf(&mut self) -> &mut [size_t; 2] {
		unsafe { sys::cv_MatStep_getPropBuf(self.as_raw_mut_MatStep()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: buf")
	}
	
	fn get(&self, i: i32) -> Result<size_t> {
		unsafe { sys::cv_MatStep_operator___const_int(self.as_raw_MatStep(), i) }.into_result()
	}
	
	fn get_mut(&mut self, i: i32) -> Result<size_t> {
		unsafe { sys::cv_MatStep_operator___int(self.as_raw_mut_MatStep(), i) }.into_result()
	}
	
	fn to_size_t(&self) -> Result<size_t> {
		unsafe { sys::cv_MatStep_operator_size_t_const(self.as_raw_MatStep()) }.into_result()
	}
	
}

pub struct MatStep {
	ptr: *mut c_void
}

opencv_type_boxed! { MatStep }

impl Drop for MatStep {
	fn drop(&mut self) {
		extern "C" { fn cv_MatStep_delete(instance: *mut c_void); }
		unsafe { cv_MatStep_delete(self.as_raw_mut_MatStep()) };
	}
}

impl MatStep {
	#[inline] pub fn as_raw_MatStep(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MatStep(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MatStep {}

impl core::MatStepTrait for MatStep {
	#[inline] fn as_raw_MatStep(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MatStep(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MatStep {
	/// ////////////////////////// MatStep ////////////////////////////
	pub fn default() -> Result<core::MatStep> {
		unsafe { sys::cv_MatStep_MatStep() }.into_result().map(|r| unsafe { core::MatStep::opencv_from_extern(r) } )
	}
	
	pub fn new(s: size_t) -> Result<core::MatStep> {
		unsafe { sys::cv_MatStep_MatStep_size_t(s) }.into_result().map(|r| unsafe { core::MatStep::opencv_from_extern(r) } )
	}
	
}

/// @cond IGNORED
pub trait Matx_AddOpTrait {
	fn as_raw_Matx_AddOp(&self) -> *const c_void;
	fn as_raw_mut_Matx_AddOp(&mut self) -> *mut c_void;

}

/// @cond IGNORED
pub struct Matx_AddOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_AddOp }

impl Drop for Matx_AddOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_AddOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_AddOp_delete(self.as_raw_mut_Matx_AddOp()) };
	}
}

impl Matx_AddOp {
	#[inline] pub fn as_raw_Matx_AddOp(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Matx_AddOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Matx_AddOp {}

impl core::Matx_AddOpTrait for Matx_AddOp {
	#[inline] fn as_raw_Matx_AddOp(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Matx_AddOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_AddOp {
	pub fn default() -> Result<core::Matx_AddOp> {
		unsafe { sys::cv_Matx_AddOp_Matx_AddOp() }.into_result().map(|r| unsafe { core::Matx_AddOp::opencv_from_extern(r) } )
	}
	
	pub fn copy(unnamed: &core::Matx_AddOp) -> Result<core::Matx_AddOp> {
		unsafe { sys::cv_Matx_AddOp_Matx_AddOp_const_Matx_AddOpR(unnamed.as_raw_Matx_AddOp()) }.into_result().map(|r| unsafe { core::Matx_AddOp::opencv_from_extern(r) } )
	}
	
}

pub trait Matx_DivOpTrait {
	fn as_raw_Matx_DivOp(&self) -> *const c_void;
	fn as_raw_mut_Matx_DivOp(&mut self) -> *mut c_void;

}

pub struct Matx_DivOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_DivOp }

impl Drop for Matx_DivOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_DivOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_DivOp_delete(self.as_raw_mut_Matx_DivOp()) };
	}
}

impl Matx_DivOp {
	#[inline] pub fn as_raw_Matx_DivOp(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Matx_DivOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Matx_DivOp {}

impl core::Matx_DivOpTrait for Matx_DivOp {
	#[inline] fn as_raw_Matx_DivOp(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Matx_DivOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_DivOp {
	pub fn default() -> Result<core::Matx_DivOp> {
		unsafe { sys::cv_Matx_DivOp_Matx_DivOp() }.into_result().map(|r| unsafe { core::Matx_DivOp::opencv_from_extern(r) } )
	}
	
	pub fn copy(unnamed: &core::Matx_DivOp) -> Result<core::Matx_DivOp> {
		unsafe { sys::cv_Matx_DivOp_Matx_DivOp_const_Matx_DivOpR(unnamed.as_raw_Matx_DivOp()) }.into_result().map(|r| unsafe { core::Matx_DivOp::opencv_from_extern(r) } )
	}
	
}

pub trait Matx_MatMulOpTrait {
	fn as_raw_Matx_MatMulOp(&self) -> *const c_void;
	fn as_raw_mut_Matx_MatMulOp(&mut self) -> *mut c_void;

}

pub struct Matx_MatMulOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_MatMulOp }

impl Drop for Matx_MatMulOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_MatMulOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_MatMulOp_delete(self.as_raw_mut_Matx_MatMulOp()) };
	}
}

impl Matx_MatMulOp {
	#[inline] pub fn as_raw_Matx_MatMulOp(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Matx_MatMulOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Matx_MatMulOp {}

impl core::Matx_MatMulOpTrait for Matx_MatMulOp {
	#[inline] fn as_raw_Matx_MatMulOp(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Matx_MatMulOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_MatMulOp {
	pub fn default() -> Result<core::Matx_MatMulOp> {
		unsafe { sys::cv_Matx_MatMulOp_Matx_MatMulOp() }.into_result().map(|r| unsafe { core::Matx_MatMulOp::opencv_from_extern(r) } )
	}
	
	pub fn copy(unnamed: &core::Matx_MatMulOp) -> Result<core::Matx_MatMulOp> {
		unsafe { sys::cv_Matx_MatMulOp_Matx_MatMulOp_const_Matx_MatMulOpR(unnamed.as_raw_Matx_MatMulOp()) }.into_result().map(|r| unsafe { core::Matx_MatMulOp::opencv_from_extern(r) } )
	}
	
}

pub trait Matx_MulOpTrait {
	fn as_raw_Matx_MulOp(&self) -> *const c_void;
	fn as_raw_mut_Matx_MulOp(&mut self) -> *mut c_void;

}

pub struct Matx_MulOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_MulOp }

impl Drop for Matx_MulOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_MulOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_MulOp_delete(self.as_raw_mut_Matx_MulOp()) };
	}
}

impl Matx_MulOp {
	#[inline] pub fn as_raw_Matx_MulOp(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Matx_MulOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Matx_MulOp {}

impl core::Matx_MulOpTrait for Matx_MulOp {
	#[inline] fn as_raw_Matx_MulOp(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Matx_MulOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_MulOp {
	pub fn default() -> Result<core::Matx_MulOp> {
		unsafe { sys::cv_Matx_MulOp_Matx_MulOp() }.into_result().map(|r| unsafe { core::Matx_MulOp::opencv_from_extern(r) } )
	}
	
	pub fn copy(unnamed: &core::Matx_MulOp) -> Result<core::Matx_MulOp> {
		unsafe { sys::cv_Matx_MulOp_Matx_MulOp_const_Matx_MulOpR(unnamed.as_raw_Matx_MulOp()) }.into_result().map(|r| unsafe { core::Matx_MulOp::opencv_from_extern(r) } )
	}
	
}

pub trait Matx_ScaleOpTrait {
	fn as_raw_Matx_ScaleOp(&self) -> *const c_void;
	fn as_raw_mut_Matx_ScaleOp(&mut self) -> *mut c_void;

}

pub struct Matx_ScaleOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_ScaleOp }

impl Drop for Matx_ScaleOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_ScaleOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_ScaleOp_delete(self.as_raw_mut_Matx_ScaleOp()) };
	}
}

impl Matx_ScaleOp {
	#[inline] pub fn as_raw_Matx_ScaleOp(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Matx_ScaleOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Matx_ScaleOp {}

impl core::Matx_ScaleOpTrait for Matx_ScaleOp {
	#[inline] fn as_raw_Matx_ScaleOp(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Matx_ScaleOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_ScaleOp {
	pub fn default() -> Result<core::Matx_ScaleOp> {
		unsafe { sys::cv_Matx_ScaleOp_Matx_ScaleOp() }.into_result().map(|r| unsafe { core::Matx_ScaleOp::opencv_from_extern(r) } )
	}
	
	pub fn copy(unnamed: &core::Matx_ScaleOp) -> Result<core::Matx_ScaleOp> {
		unsafe { sys::cv_Matx_ScaleOp_Matx_ScaleOp_const_Matx_ScaleOpR(unnamed.as_raw_Matx_ScaleOp()) }.into_result().map(|r| unsafe { core::Matx_ScaleOp::opencv_from_extern(r) } )
	}
	
}

pub trait Matx_SubOpTrait {
	fn as_raw_Matx_SubOp(&self) -> *const c_void;
	fn as_raw_mut_Matx_SubOp(&mut self) -> *mut c_void;

}

pub struct Matx_SubOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_SubOp }

impl Drop for Matx_SubOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_SubOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_SubOp_delete(self.as_raw_mut_Matx_SubOp()) };
	}
}

impl Matx_SubOp {
	#[inline] pub fn as_raw_Matx_SubOp(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Matx_SubOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Matx_SubOp {}

impl core::Matx_SubOpTrait for Matx_SubOp {
	#[inline] fn as_raw_Matx_SubOp(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Matx_SubOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_SubOp {
	pub fn default() -> Result<core::Matx_SubOp> {
		unsafe { sys::cv_Matx_SubOp_Matx_SubOp() }.into_result().map(|r| unsafe { core::Matx_SubOp::opencv_from_extern(r) } )
	}
	
	pub fn copy(unnamed: &core::Matx_SubOp) -> Result<core::Matx_SubOp> {
		unsafe { sys::cv_Matx_SubOp_Matx_SubOp_const_Matx_SubOpR(unnamed.as_raw_Matx_SubOp()) }.into_result().map(|r| unsafe { core::Matx_SubOp::opencv_from_extern(r) } )
	}
	
}

pub trait Matx_TOpTrait {
	fn as_raw_Matx_TOp(&self) -> *const c_void;
	fn as_raw_mut_Matx_TOp(&mut self) -> *mut c_void;

}

pub struct Matx_TOp {
	ptr: *mut c_void
}

opencv_type_boxed! { Matx_TOp }

impl Drop for Matx_TOp {
	fn drop(&mut self) {
		extern "C" { fn cv_Matx_TOp_delete(instance: *mut c_void); }
		unsafe { cv_Matx_TOp_delete(self.as_raw_mut_Matx_TOp()) };
	}
}

impl Matx_TOp {
	#[inline] pub fn as_raw_Matx_TOp(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Matx_TOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Matx_TOp {}

impl core::Matx_TOpTrait for Matx_TOp {
	#[inline] fn as_raw_Matx_TOp(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Matx_TOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Matx_TOp {
	pub fn default() -> Result<core::Matx_TOp> {
		unsafe { sys::cv_Matx_TOp_Matx_TOp() }.into_result().map(|r| unsafe { core::Matx_TOp::opencv_from_extern(r) } )
	}
	
	pub fn copy(unnamed: &core::Matx_TOp) -> Result<core::Matx_TOp> {
		unsafe { sys::cv_Matx_TOp_Matx_TOp_const_Matx_TOpR(unnamed.as_raw_Matx_TOp()) }.into_result().map(|r| unsafe { core::Matx_TOp::opencv_from_extern(r) } )
	}
	
}

/// Basic interface for all solvers
pub trait MinProblemSolver: core::AlgorithmTrait {
	fn as_raw_MinProblemSolver(&self) -> *const c_void;
	fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void;

	/// Getter for the optimized function.
	/// 
	/// The optimized function is represented by Function interface, which requires derivatives to
	/// implement the calc(double*) and getDim() methods to evaluate the function.
	/// 
	/// ## Returns
	/// Smart-pointer to an object that implements Function interface - it represents the
	/// function that is being optimized. It can be empty, if no function was given so far.
	fn get_function(&self) -> Result<core::Ptr::<dyn core::MinProblemSolver_Function>> {
		unsafe { sys::cv_MinProblemSolver_getFunction_const(self.as_raw_MinProblemSolver()) }.into_result().map(|r| unsafe { core::Ptr::<dyn core::MinProblemSolver_Function>::opencv_from_extern(r) } )
	}
	
	/// Setter for the optimized function.
	/// 
	/// *It should be called at least once before the call to* minimize(), as default value is not usable.
	/// 
	/// ## Parameters
	/// * f: The new function to optimize.
	fn set_function(&mut self, f: &core::Ptr::<dyn core::MinProblemSolver_Function>) -> Result<()> {
		unsafe { sys::cv_MinProblemSolver_setFunction_const_Ptr_Function_R(self.as_raw_mut_MinProblemSolver(), f.as_raw_PtrOfMinProblemSolver_Function()) }.into_result()
	}
	
	/// Getter for the previously set terminal criteria for this algorithm.
	/// 
	/// ## Returns
	/// Deep copy of the terminal criteria used at the moment.
	fn get_term_criteria(&self) -> Result<core::TermCriteria> {
		unsafe { sys::cv_MinProblemSolver_getTermCriteria_const(self.as_raw_MinProblemSolver()) }.into_result()
	}
	
	/// Set terminal criteria for solver.
	/// 
	/// This method *is not necessary* to be called before the first call to minimize(), as the default
	/// value is sensible.
	/// 
	/// Algorithm stops when the number of function evaluations done exceeds termcrit.maxCount, when
	/// the function values at the vertices of simplex are within termcrit.epsilon range or simplex
	/// becomes so small that it can enclosed in a box with termcrit.epsilon sides, whatever comes
	/// first.
	/// ## Parameters
	/// * termcrit: Terminal criteria to be used, represented as cv::TermCriteria structure.
	fn set_term_criteria(&mut self, termcrit: core::TermCriteria) -> Result<()> {
		unsafe { sys::cv_MinProblemSolver_setTermCriteria_const_TermCriteriaR(self.as_raw_mut_MinProblemSolver(), &termcrit) }.into_result()
	}
	
	/// actually runs the algorithm and performs the minimization.
	/// 
	/// The sole input parameter determines the centroid of the starting simplex (roughly, it tells
	/// where to start), all the others (terminal criteria, initial step, function to be minimized) are
	/// supposed to be set via the setters before the call to this method or the default values (not
	/// always sensible) will be used.
	/// 
	/// ## Parameters
	/// * x: The initial point, that will become a centroid of an initial simplex. After the algorithm
	/// will terminate, it will be set to the point where the algorithm stops, the point of possible
	/// minimum.
	/// ## Returns
	/// The value of a function at the point found.
	fn minimize(&mut self, x: &mut dyn core::ToInputOutputArray) -> Result<f64> {
		input_output_array_arg!(x);
		unsafe { sys::cv_MinProblemSolver_minimize_const__InputOutputArrayR(self.as_raw_mut_MinProblemSolver(), x.as_raw__InputOutputArray()) }.into_result()
	}
	
}

/// Represents function being optimized
pub trait MinProblemSolver_Function {
	fn as_raw_MinProblemSolver_Function(&self) -> *const c_void;
	fn as_raw_mut_MinProblemSolver_Function(&mut self) -> *mut c_void;

	fn get_dims(&self) -> Result<i32> {
		unsafe { sys::cv_MinProblemSolver_Function_getDims_const(self.as_raw_MinProblemSolver_Function()) }.into_result()
	}
	
	fn get_gradient_eps(&self) -> Result<f64> {
		unsafe { sys::cv_MinProblemSolver_Function_getGradientEps_const(self.as_raw_MinProblemSolver_Function()) }.into_result()
	}
	
	fn calc(&self, x: &f64) -> Result<f64> {
		unsafe { sys::cv_MinProblemSolver_Function_calc_const_const_doubleX(self.as_raw_MinProblemSolver_Function(), x) }.into_result()
	}
	
	fn get_gradient(&mut self, x: &f64, grad: &mut f64) -> Result<()> {
		unsafe { sys::cv_MinProblemSolver_Function_getGradient_const_doubleX_doubleX(self.as_raw_mut_MinProblemSolver_Function(), x, grad) }.into_result()
	}
	
}

/// struct returned by cv::moments
/// 
/// The spatial moments ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BMoments%3A%3Am%7D%5F%7Bji%7D) are computed as:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bm%7D%20%5F%7Bji%7D%3D%20%5Csum%20%5F%7Bx%2Cy%7D%20%20%5Cleft%20%28%20%5Ctexttt%7Barray%7D%20%28x%2Cy%29%20%20%5Ccdot%20x%5Ej%20%20%5Ccdot%20y%5Ei%20%5Cright%20%29)
/// 
/// The central moments ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BMoments%3A%3Amu%7D%5F%7Bji%7D) are computed as:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bmu%7D%20%5F%7Bji%7D%3D%20%5Csum%20%5F%7Bx%2Cy%7D%20%20%5Cleft%20%28%20%5Ctexttt%7Barray%7D%20%28x%2Cy%29%20%20%5Ccdot%20%28x%20%2D%20%20%5Cbar%7Bx%7D%20%29%5Ej%20%20%5Ccdot%20%28y%20%2D%20%20%5Cbar%7By%7D%20%29%5Ei%20%5Cright%20%29)
/// 
/// where ![inline formula](https://latex.codecogs.com/png.latex?%28%5Cbar%7Bx%7D%2C%20%5Cbar%7By%7D%29) is the mass center:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbar%7Bx%7D%20%3D%20%5Cfrac%7B%5Ctexttt%7Bm%7D%5F%7B10%7D%7D%7B%5Ctexttt%7Bm%7D%5F%7B00%7D%7D%20%2C%20%5C%3B%20%5Cbar%7By%7D%20%3D%20%5Cfrac%7B%5Ctexttt%7Bm%7D%5F%7B01%7D%7D%7B%5Ctexttt%7Bm%7D%5F%7B00%7D%7D)
/// 
/// The normalized central moments ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BMoments%3A%3Anu%7D%5F%7Bij%7D) are computed as:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bnu%7D%20%5F%7Bji%7D%3D%20%5Cfrac%7B%5Ctexttt%7Bmu%7D%5F%7Bji%7D%7D%7B%5Ctexttt%7Bm%7D%5F%7B00%7D%5E%7B%28i%2Bj%29%2F2%2B1%7D%7D%20%2E)
/// 
/// 
/// Note:
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bmu%7D%5F%7B00%7D%3D%5Ctexttt%7Bm%7D%5F%7B00%7D), ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bnu%7D%5F%7B00%7D%3D1)
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bnu%7D%5F%7B10%7D%3D%5Ctexttt%7Bmu%7D%5F%7B10%7D%3D%5Ctexttt%7Bmu%7D%5F%7B01%7D%3D%5Ctexttt%7Bmu%7D%5F%7B10%7D%3D0) , hence the values are not
/// stored.
/// 
/// The moments of a contour are defined in the same way but computed using the Green's formula (see
/// <http://en.wikipedia.org/wiki/Green_theorem>). So, due to a limited raster resolution, the moments
/// computed for a contour are slightly different from the moments computed for the same rasterized
/// contour.
/// 
/// 
/// Note:
/// Since the contour moments are computed using Green formula, you may get seemingly odd results for
/// contours with self-intersections, e.g. a zero area (m00) for butterfly-shaped contours.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Moments {
	/// @name spatial moments
	pub m00: f64,
	/// @name spatial moments
	pub m10: f64,
	/// @name spatial moments
	pub m01: f64,
	/// @name spatial moments
	pub m20: f64,
	/// @name spatial moments
	pub m11: f64,
	/// @name spatial moments
	pub m02: f64,
	/// @name spatial moments
	pub m30: f64,
	/// @name spatial moments
	pub m21: f64,
	/// @name spatial moments
	pub m12: f64,
	/// @name spatial moments
	pub m03: f64,
	/// @name central moments
	pub mu20: f64,
	/// @name central moments
	pub mu11: f64,
	/// @name central moments
	pub mu02: f64,
	/// @name central moments
	pub mu30: f64,
	/// @name central moments
	pub mu21: f64,
	/// @name central moments
	pub mu12: f64,
	/// @name central moments
	pub mu03: f64,
	/// @name central normalized moments
	pub nu20: f64,
	/// @name central normalized moments
	pub nu11: f64,
	/// @name central normalized moments
	pub nu02: f64,
	/// @name central normalized moments
	pub nu30: f64,
	/// @name central normalized moments
	pub nu21: f64,
	/// @name central normalized moments
	pub nu12: f64,
	/// @name central normalized moments
	pub nu03: f64,
}

opencv_type_simple! { core::Moments }

impl Moments {
	/// the default constructor
	pub fn default() -> Result<core::Moments> {
		unsafe { sys::cv_Moments_Moments() }.into_result()
	}
	
	/// the full constructor
	pub fn new(m00: f64, m10: f64, m01: f64, m20: f64, m11: f64, m02: f64, m30: f64, m21: f64, m12: f64, m03: f64) -> Result<core::Moments> {
		unsafe { sys::cv_Moments_Moments_double_double_double_double_double_double_double_double_double_double(m00, m10, m01, m20, m11, m02, m30, m21, m12, m03) }.into_result()
	}
	
}

/// Principal Component Analysis
/// 
/// The class is used to calculate a special basis for a set of vectors. The
/// basis will consist of eigenvectors of the covariance matrix calculated
/// from the input set of vectors. The class %PCA can also transform
/// vectors to/from the new coordinate space defined by the basis. Usually,
/// in this new coordinate system, each vector from the original set (and
/// any linear combination of such vectors) can be quite accurately
/// approximated by taking its first few components, corresponding to the
/// eigenvectors of the largest eigenvalues of the covariance matrix.
/// Geometrically it means that you calculate a projection of the vector to
/// a subspace formed by a few eigenvectors corresponding to the dominant
/// eigenvalues of the covariance matrix. And usually such a projection is
/// very close to the original vector. So, you can represent the original
/// vector from a high-dimensional space with a much shorter vector
/// consisting of the projected vector's coordinates in the subspace. Such a
/// transformation is also known as Karhunen-Loeve Transform, or KLT.
/// See http://en.wikipedia.org/wiki/Principal_component_analysis
/// 
/// The sample below is the function that takes two matrices. The first
/// function stores a set of vectors (a row per vector) that is used to
/// calculate PCA. The second function stores another "test" set of vectors
/// (a row per vector). First, these vectors are compressed with PCA, then
/// reconstructed back, and then the reconstruction error norm is computed
/// and printed for each vector. :
/// 
/// ```ignore
/// using namespace cv;
/// 
/// PCA compressPCA(const Mat& pcaset, int maxComponents,
///                const Mat& testset, Mat& compressed)
/// {
///    PCA pca(pcaset, // pass the data
///            Mat(), // we do not have a pre-computed mean vector,
///                    // so let the PCA engine to compute it
///            PCA::DATA_AS_ROW, // indicate that the vectors
///                                 // are stored as matrix rows
///                                 // (use PCA::DATA_AS_COL if the vectors are
///                                 // the matrix columns)
///            maxComponents // specify, how many principal components to retain
///            );
///    // if there is no test data, just return the computed basis, ready-to-use
///    if( !testset.data )
///        return pca;
///    CV_Assert( testset.cols == pcaset.cols );
/// 
///    compressed.create(testset.rows, maxComponents, testset.type());
/// 
///    Mat reconstructed;
///    for( int i = 0; i < testset.rows; i++ )
///    {
///        Mat vec = testset.row(i), coeffs = compressed.row(i), reconstructed;
///        // compress the vector, the result will be stored
///        // in the i-th row of the output matrix
///        pca.project(vec, coeffs);
///        // and then reconstruct it
///        pca.backProject(coeffs, reconstructed);
///        // and measure the error
///        printf("%d. diff = %g\n", i, norm(vec, reconstructed, NORM_L2));
///    }
///    return pca;
/// }
/// ```
/// ## See also
/// calcCovarMatrix, mulTransposed, SVD, dft, dct
pub trait PCATrait {
	fn as_raw_PCA(&self) -> *const c_void;
	fn as_raw_mut_PCA(&mut self) -> *mut c_void;

	/// eigenvectors of the covariation matrix
	fn eigenvectors(&mut self) -> core::Mat {
		unsafe { sys::cv_PCA_getPropEigenvectors(self.as_raw_mut_PCA()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: eigenvectors")
	}
	
	/// eigenvectors of the covariation matrix
	fn set_eigenvectors(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_PCA_setPropEigenvectors_Mat(self.as_raw_mut_PCA(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_eigenvectors")
	}
	
	/// eigenvalues of the covariation matrix
	fn eigenvalues(&mut self) -> core::Mat {
		unsafe { sys::cv_PCA_getPropEigenvalues(self.as_raw_mut_PCA()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: eigenvalues")
	}
	
	/// eigenvalues of the covariation matrix
	fn set_eigenvalues(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_PCA_setPropEigenvalues_Mat(self.as_raw_mut_PCA(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_eigenvalues")
	}
	
	/// mean value subtracted before the projection and added after the back projection
	fn mean(&mut self) -> core::Mat {
		unsafe { sys::cv_PCA_getPropMean(self.as_raw_mut_PCA()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: mean")
	}
	
	/// mean value subtracted before the projection and added after the back projection
	fn set_mean(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_PCA_setPropMean_Mat(self.as_raw_mut_PCA(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_mean")
	}
	
	/// Projects vector(s) to the principal component subspace.
	/// 
	/// The methods project one or more vectors to the principal component
	/// subspace, where each vector projection is represented by coefficients in
	/// the principal component basis. The first form of the method returns the
	/// matrix that the second form writes to the result. So the first form can
	/// be used as a part of expression while the second form can be more
	/// efficient in a processing loop.
	/// ## Parameters
	/// * vec: input vector(s); must have the same dimensionality and the
	/// same layout as the input data used at %PCA phase, that is, if
	/// DATA_AS_ROW are specified, then `vec.cols==data.cols`
	/// (vector dimensionality) and `vec.rows` is the number of vectors to
	/// project, and the same is true for the PCA::DATA_AS_COL case.
	fn project(&self, vec: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(vec);
		unsafe { sys::cv_PCA_project_const_const__InputArrayR(self.as_raw_PCA(), vec.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Projects vector(s) to the principal component subspace.
	/// 
	/// The methods project one or more vectors to the principal component
	/// subspace, where each vector projection is represented by coefficients in
	/// the principal component basis. The first form of the method returns the
	/// matrix that the second form writes to the result. So the first form can
	/// be used as a part of expression while the second form can be more
	/// efficient in a processing loop.
	/// ## Parameters
	/// * vec: input vector(s); must have the same dimensionality and the
	/// same layout as the input data used at %PCA phase, that is, if
	/// DATA_AS_ROW are specified, then `vec.cols==data.cols`
	/// (vector dimensionality) and `vec.rows` is the number of vectors to
	/// project, and the same is true for the PCA::DATA_AS_COL case.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * vec: input vector(s); must have the same dimensionality and the
	///    same layout as the input data used at PCA phase, that is, if
	///    DATA_AS_ROW are specified, then `vec.cols==data.cols`
	///    (vector dimensionality) and `vec.rows` is the number of vectors to
	///    project, and the same is true for the PCA::DATA_AS_COL case.
	/// * result: output vectors; in case of PCA::DATA_AS_COL, the
	///    output matrix has as many columns as the number of input vectors, this
	///    means that `result.cols==vec.cols` and the number of rows match the
	///    number of principal components (for example, `maxComponents` parameter
	///    passed to the constructor).
	fn project_to(&self, vec: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(vec);
		output_array_arg!(result);
		unsafe { sys::cv_PCA_project_const_const__InputArrayR_const__OutputArrayR(self.as_raw_PCA(), vec.as_raw__InputArray(), result.as_raw__OutputArray()) }.into_result()
	}
	
	/// Reconstructs vectors from their PC projections.
	/// 
	/// The methods are inverse operations to PCA::project. They take PC
	/// coordinates of projected vectors and reconstruct the original vectors.
	/// Unless all the principal components have been retained, the
	/// reconstructed vectors are different from the originals. But typically,
	/// the difference is small if the number of components is large enough (but
	/// still much smaller than the original vector dimensionality). As a
	/// result, PCA is used.
	/// ## Parameters
	/// * vec: coordinates of the vectors in the principal component
	/// subspace, the layout and size are the same as of PCA::project output
	/// vectors.
	fn back_project(&self, vec: &dyn core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(vec);
		unsafe { sys::cv_PCA_backProject_const_const__InputArrayR(self.as_raw_PCA(), vec.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Reconstructs vectors from their PC projections.
	/// 
	/// The methods are inverse operations to PCA::project. They take PC
	/// coordinates of projected vectors and reconstruct the original vectors.
	/// Unless all the principal components have been retained, the
	/// reconstructed vectors are different from the originals. But typically,
	/// the difference is small if the number of components is large enough (but
	/// still much smaller than the original vector dimensionality). As a
	/// result, PCA is used.
	/// ## Parameters
	/// * vec: coordinates of the vectors in the principal component
	/// subspace, the layout and size are the same as of PCA::project output
	/// vectors.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * vec: coordinates of the vectors in the principal component
	///    subspace, the layout and size are the same as of PCA::project output
	///    vectors.
	/// * result: reconstructed vectors; the layout and size are the same as
	///    of PCA::project input vectors.
	fn back_project_to(&self, vec: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(vec);
		output_array_arg!(result);
		unsafe { sys::cv_PCA_backProject_const_const__InputArrayR_const__OutputArrayR(self.as_raw_PCA(), vec.as_raw__InputArray(), result.as_raw__OutputArray()) }.into_result()
	}
	
	/// write PCA objects
	/// 
	/// Writes @ref eigenvalues @ref eigenvectors and @ref mean to specified FileStorage
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_PCA_write_const_FileStorageR(self.as_raw_PCA(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	/// load PCA objects
	/// 
	/// Loads @ref eigenvalues @ref eigenvectors and @ref mean from specified FileNode
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_PCA_read_const_FileNodeR(self.as_raw_mut_PCA(), fn_.as_raw_FileNode()) }.into_result()
	}
	
}

/// Principal Component Analysis
/// 
/// The class is used to calculate a special basis for a set of vectors. The
/// basis will consist of eigenvectors of the covariance matrix calculated
/// from the input set of vectors. The class %PCA can also transform
/// vectors to/from the new coordinate space defined by the basis. Usually,
/// in this new coordinate system, each vector from the original set (and
/// any linear combination of such vectors) can be quite accurately
/// approximated by taking its first few components, corresponding to the
/// eigenvectors of the largest eigenvalues of the covariance matrix.
/// Geometrically it means that you calculate a projection of the vector to
/// a subspace formed by a few eigenvectors corresponding to the dominant
/// eigenvalues of the covariance matrix. And usually such a projection is
/// very close to the original vector. So, you can represent the original
/// vector from a high-dimensional space with a much shorter vector
/// consisting of the projected vector's coordinates in the subspace. Such a
/// transformation is also known as Karhunen-Loeve Transform, or KLT.
/// See http://en.wikipedia.org/wiki/Principal_component_analysis
/// 
/// The sample below is the function that takes two matrices. The first
/// function stores a set of vectors (a row per vector) that is used to
/// calculate PCA. The second function stores another "test" set of vectors
/// (a row per vector). First, these vectors are compressed with PCA, then
/// reconstructed back, and then the reconstruction error norm is computed
/// and printed for each vector. :
/// 
/// ```ignore
/// using namespace cv;
/// 
/// PCA compressPCA(const Mat& pcaset, int maxComponents,
///                const Mat& testset, Mat& compressed)
/// {
///    PCA pca(pcaset, // pass the data
///            Mat(), // we do not have a pre-computed mean vector,
///                    // so let the PCA engine to compute it
///            PCA::DATA_AS_ROW, // indicate that the vectors
///                                 // are stored as matrix rows
///                                 // (use PCA::DATA_AS_COL if the vectors are
///                                 // the matrix columns)
///            maxComponents // specify, how many principal components to retain
///            );
///    // if there is no test data, just return the computed basis, ready-to-use
///    if( !testset.data )
///        return pca;
///    CV_Assert( testset.cols == pcaset.cols );
/// 
///    compressed.create(testset.rows, maxComponents, testset.type());
/// 
///    Mat reconstructed;
///    for( int i = 0; i < testset.rows; i++ )
///    {
///        Mat vec = testset.row(i), coeffs = compressed.row(i), reconstructed;
///        // compress the vector, the result will be stored
///        // in the i-th row of the output matrix
///        pca.project(vec, coeffs);
///        // and then reconstruct it
///        pca.backProject(coeffs, reconstructed);
///        // and measure the error
///        printf("%d. diff = %g\n", i, norm(vec, reconstructed, NORM_L2));
///    }
///    return pca;
/// }
/// ```
/// ## See also
/// calcCovarMatrix, mulTransposed, SVD, dft, dct
pub struct PCA {
	ptr: *mut c_void
}

opencv_type_boxed! { PCA }

impl Drop for PCA {
	fn drop(&mut self) {
		extern "C" { fn cv_PCA_delete(instance: *mut c_void); }
		unsafe { cv_PCA_delete(self.as_raw_mut_PCA()) };
	}
}

impl PCA {
	#[inline] pub fn as_raw_PCA(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PCA(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for PCA {}

impl core::PCATrait for PCA {
	#[inline] fn as_raw_PCA(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_PCA(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PCA {
	/// default constructor
	/// 
	/// The default constructor initializes an empty %PCA structure. The other
	/// constructors initialize the structure and call PCA::operator()().
	pub fn default() -> Result<core::PCA> {
		unsafe { sys::cv_PCA_PCA() }.into_result().map(|r| unsafe { core::PCA::opencv_from_extern(r) } )
	}
	
	/// default constructor
	/// 
	/// The default constructor initializes an empty %PCA structure. The other
	/// constructors initialize the structure and call PCA::operator()().
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * data: input samples stored as matrix rows or matrix columns.
	/// * mean: optional mean value; if the matrix is empty (@c noArray()),
	///    the mean is computed from the data.
	/// * flags: operation flags; currently the parameter is only used to
	///    specify the data layout (PCA::Flags)
	/// * maxComponents: maximum number of components that %PCA should
	///    retain; by default, all the components are retained.
	/// 
	/// ## C++ default parameters
	/// * max_components: 0
	pub fn new(data: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, flags: i32, max_components: i32) -> Result<core::PCA> {
		input_array_arg!(data);
		input_array_arg!(mean);
		unsafe { sys::cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_int(data.as_raw__InputArray(), mean.as_raw__InputArray(), flags, max_components) }.into_result().map(|r| unsafe { core::PCA::opencv_from_extern(r) } )
	}
	
	/// default constructor
	/// 
	/// The default constructor initializes an empty %PCA structure. The other
	/// constructors initialize the structure and call PCA::operator()().
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * data: input samples stored as matrix rows or matrix columns.
	/// * mean: optional mean value; if the matrix is empty (noArray()),
	///    the mean is computed from the data.
	/// * flags: operation flags; currently the parameter is only used to
	///    specify the data layout (PCA::Flags)
	/// * retainedVariance: Percentage of variance that PCA should retain.
	///    Using this parameter will let the PCA decided how many components to
	///    retain but it will always keep at least 2.
	pub fn new_with_variance(data: &dyn core::ToInputArray, mean: &dyn core::ToInputArray, flags: i32, retained_variance: f64) -> Result<core::PCA> {
		input_array_arg!(data);
		input_array_arg!(mean);
		unsafe { sys::cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_double(data.as_raw__InputArray(), mean.as_raw__InputArray(), flags, retained_variance) }.into_result().map(|r| unsafe { core::PCA::opencv_from_extern(r) } )
	}
	
}

/// Base class for parallel data processors
pub trait ParallelLoopBody {
	fn as_raw_ParallelLoopBody(&self) -> *const c_void;
	fn as_raw_mut_ParallelLoopBody(&mut self) -> *mut c_void;

}

/// Random Number Generator
/// 
/// Random number generator. It encapsulates the state (currently, a 64-bit
/// integer) and has methods to return scalar random values and to fill
/// arrays with random values. Currently it supports uniform and Gaussian
/// (normal) distributions. The generator uses Multiply-With-Carry
/// algorithm, introduced by G. Marsaglia (
/// <http://en.wikipedia.org/wiki/Multiply-with-carry> ).
/// Gaussian-distribution random numbers are generated using the Ziggurat
/// algorithm ( <http://en.wikipedia.org/wiki/Ziggurat_algorithm> ),
/// introduced by G. Marsaglia and W. W. Tsang.
pub trait RNGTrait {
	fn as_raw_RNG(&self) -> *const c_void;
	fn as_raw_mut_RNG(&mut self) -> *mut c_void;

	fn state(&self) -> u64 {
		unsafe { sys::cv_RNG_getPropState_const(self.as_raw_RNG()) }.into_result().expect("Infallible function failed: state")
	}
	
	fn set_state(&mut self, val: u64) -> () {
		unsafe { sys::cv_RNG_setPropState_uint64_t(self.as_raw_mut_RNG(), val) }.into_result().expect("Infallible function failed: set_state")
	}
	
	/// The method updates the state using the MWC algorithm and returns the
	/// next 32-bit random number.
	fn next(&mut self) -> Result<u32> {
		unsafe { sys::cv_RNG_next(self.as_raw_mut_RNG()) }.into_result()
	}
	
	/// Each of the methods updates the state using the MWC algorithm and
	/// returns the next random number of the specified type. In case of integer
	/// types, the returned number is from the available value range for the
	/// specified type. In case of floating-point types, the returned value is
	/// from [0,1) range.
	fn to_u8(&mut self) -> Result<u8> {
		unsafe { sys::cv_RNG_operator_unsigned_char(self.as_raw_mut_RNG()) }.into_result()
	}
	
	fn to_i8(&mut self) -> Result<i8> {
		unsafe { sys::cv_RNG_operator_signed_char(self.as_raw_mut_RNG()) }.into_result()
	}
	
	fn to_u16(&mut self) -> Result<u16> {
		unsafe { sys::cv_RNG_operator_unsigned_short(self.as_raw_mut_RNG()) }.into_result()
	}
	
	fn to_i16(&mut self) -> Result<i16> {
		unsafe { sys::cv_RNG_operator_short(self.as_raw_mut_RNG()) }.into_result()
	}
	
	fn to_u32(&mut self) -> Result<u32> {
		unsafe { sys::cv_RNG_operator_unsigned_int(self.as_raw_mut_RNG()) }.into_result()
	}
	
	fn to_i32(&mut self) -> Result<i32> {
		unsafe { sys::cv_RNG_operator_int(self.as_raw_mut_RNG()) }.into_result()
	}
	
	fn to_f32(&mut self) -> Result<f32> {
		unsafe { sys::cv_RNG_operator_float(self.as_raw_mut_RNG()) }.into_result()
	}
	
	fn to_f64(&mut self) -> Result<f64> {
		unsafe { sys::cv_RNG_operator_double(self.as_raw_mut_RNG()) }.into_result()
	}
	
	/// returns uniformly distributed integer random number from [a,b) range
	/// 
	/// The methods transform the state using the MWC algorithm and return the
	/// next uniformly-distributed random number of the specified type, deduced
	/// from the input parameter type, from the range [a, b) . There is a nuance
	/// illustrated by the following sample:
	/// 
	/// ```ignore
	/// RNG rng;
	/// 
	/// // always produces 0
	/// double a = rng.uniform(0, 1);
	/// 
	/// // produces double from [0, 1)
	/// double a1 = rng.uniform((double)0, (double)1);
	/// 
	/// // produces float from [0, 1)
	/// float b = rng.uniform(0.f, 1.f);
	/// 
	/// // produces double from [0, 1)
	/// double c = rng.uniform(0., 1.);
	/// 
	/// // may cause compiler error because of ambiguity:
	/// //  RNG::uniform(0, (int)0.999999)? or RNG::uniform((double)0, 0.99999)?
	/// double d = rng.uniform(0, 0.999999);
	/// ```
	/// 
	/// 
	/// The compiler does not take into account the type of the variable to
	/// which you assign the result of RNG::uniform . The only thing that
	/// matters to the compiler is the type of a and b parameters. So, if you
	/// want a floating-point random number, but the range boundaries are
	/// integer numbers, either put dots in the end, if they are constants, or
	/// use explicit type cast operators, as in the a1 initialization above.
	/// ## Parameters
	/// * a: lower inclusive boundary of the returned random number.
	/// * b: upper non-inclusive boundary of the returned random number.
	fn uniform(&mut self, a: i32, b: i32) -> Result<i32> {
		unsafe { sys::cv_RNG_uniform_int_int(self.as_raw_mut_RNG(), a, b) }.into_result()
	}
	
	/// returns uniformly distributed integer random number from [a,b) range
	/// 
	/// The methods transform the state using the MWC algorithm and return the
	/// next uniformly-distributed random number of the specified type, deduced
	/// from the input parameter type, from the range [a, b) . There is a nuance
	/// illustrated by the following sample:
	/// 
	/// ```ignore
	/// RNG rng;
	/// 
	/// // always produces 0
	/// double a = rng.uniform(0, 1);
	/// 
	/// // produces double from [0, 1)
	/// double a1 = rng.uniform((double)0, (double)1);
	/// 
	/// // produces float from [0, 1)
	/// float b = rng.uniform(0.f, 1.f);
	/// 
	/// // produces double from [0, 1)
	/// double c = rng.uniform(0., 1.);
	/// 
	/// // may cause compiler error because of ambiguity:
	/// //  RNG::uniform(0, (int)0.999999)? or RNG::uniform((double)0, 0.99999)?
	/// double d = rng.uniform(0, 0.999999);
	/// ```
	/// 
	/// 
	/// The compiler does not take into account the type of the variable to
	/// which you assign the result of RNG::uniform . The only thing that
	/// matters to the compiler is the type of a and b parameters. So, if you
	/// want a floating-point random number, but the range boundaries are
	/// integer numbers, either put dots in the end, if they are constants, or
	/// use explicit type cast operators, as in the a1 initialization above.
	/// ## Parameters
	/// * a: lower inclusive boundary of the returned random number.
	/// * b: upper non-inclusive boundary of the returned random number.
	/// 
	/// ## Overloaded parameters
	fn uniform_1(&mut self, a: f32, b: f32) -> Result<f32> {
		unsafe { sys::cv_RNG_uniform_float_float(self.as_raw_mut_RNG(), a, b) }.into_result()
	}
	
	/// returns uniformly distributed integer random number from [a,b) range
	/// 
	/// The methods transform the state using the MWC algorithm and return the
	/// next uniformly-distributed random number of the specified type, deduced
	/// from the input parameter type, from the range [a, b) . There is a nuance
	/// illustrated by the following sample:
	/// 
	/// ```ignore
	/// RNG rng;
	/// 
	/// // always produces 0
	/// double a = rng.uniform(0, 1);
	/// 
	/// // produces double from [0, 1)
	/// double a1 = rng.uniform((double)0, (double)1);
	/// 
	/// // produces float from [0, 1)
	/// float b = rng.uniform(0.f, 1.f);
	/// 
	/// // produces double from [0, 1)
	/// double c = rng.uniform(0., 1.);
	/// 
	/// // may cause compiler error because of ambiguity:
	/// //  RNG::uniform(0, (int)0.999999)? or RNG::uniform((double)0, 0.99999)?
	/// double d = rng.uniform(0, 0.999999);
	/// ```
	/// 
	/// 
	/// The compiler does not take into account the type of the variable to
	/// which you assign the result of RNG::uniform . The only thing that
	/// matters to the compiler is the type of a and b parameters. So, if you
	/// want a floating-point random number, but the range boundaries are
	/// integer numbers, either put dots in the end, if they are constants, or
	/// use explicit type cast operators, as in the a1 initialization above.
	/// ## Parameters
	/// * a: lower inclusive boundary of the returned random number.
	/// * b: upper non-inclusive boundary of the returned random number.
	/// 
	/// ## Overloaded parameters
	fn uniform_2(&mut self, a: f64, b: f64) -> Result<f64> {
		unsafe { sys::cv_RNG_uniform_double_double(self.as_raw_mut_RNG(), a, b) }.into_result()
	}
	
	/// Fills arrays with random numbers.
	/// 
	/// ## Parameters
	/// * mat: 2D or N-dimensional matrix; currently matrices with more than
	/// 4 channels are not supported by the methods, use Mat::reshape as a
	/// possible workaround.
	/// * distType: distribution type, RNG::UNIFORM or RNG::NORMAL.
	/// * a: first distribution parameter; in case of the uniform
	/// distribution, this is an inclusive lower boundary, in case of the normal
	/// distribution, this is a mean value.
	/// * b: second distribution parameter; in case of the uniform
	/// distribution, this is a non-inclusive upper boundary, in case of the
	/// normal distribution, this is a standard deviation (diagonal of the
	/// standard deviation matrix or the full standard deviation matrix).
	/// * saturateRange: pre-saturation flag; for uniform distribution only;
	/// if true, the method will first convert a and b to the acceptable value
	/// range (according to the mat datatype) and then will generate uniformly
	/// distributed random numbers within the range [saturate(a), saturate(b)),
	/// if saturateRange=false, the method will generate uniformly distributed
	/// random numbers in the original range [a, b) and then will saturate them,
	/// it means, for example, that
	/// <tt>theRNG().fill(mat_8u, RNG::UNIFORM, -DBL_MAX, DBL_MAX)</tt> will likely
	/// produce array mostly filled with 0's and 255's, since the range (0, 255)
	/// is significantly smaller than [-DBL_MAX, DBL_MAX).
	/// 
	/// Each of the methods fills the matrix with the random values from the
	/// specified distribution. As the new numbers are generated, the RNG state
	/// is updated accordingly. In case of multiple-channel images, every
	/// channel is filled independently, which means that RNG cannot generate
	/// samples from the multi-dimensional Gaussian distribution with
	/// non-diagonal covariance matrix directly. To do that, the method
	/// generates samples from multi-dimensional standard Gaussian distribution
	/// with zero mean and identity covariation matrix, and then transforms them
	/// using transform to get samples from the specified Gaussian distribution.
	/// 
	/// ## C++ default parameters
	/// * saturate_range: false
	fn fill(&mut self, mat: &mut dyn core::ToInputOutputArray, dist_type: i32, a: &dyn core::ToInputArray, b: &dyn core::ToInputArray, saturate_range: bool) -> Result<()> {
		input_output_array_arg!(mat);
		input_array_arg!(a);
		input_array_arg!(b);
		unsafe { sys::cv_RNG_fill_const__InputOutputArrayR_int_const__InputArrayR_const__InputArrayR_bool(self.as_raw_mut_RNG(), mat.as_raw__InputOutputArray(), dist_type, a.as_raw__InputArray(), b.as_raw__InputArray(), saturate_range) }.into_result()
	}
	
	/// Returns the next random number sampled from the Gaussian distribution
	/// ## Parameters
	/// * sigma: standard deviation of the distribution.
	/// 
	/// The method transforms the state using the MWC algorithm and returns the
	/// next random number from the Gaussian distribution N(0,sigma) . That is,
	/// the mean value of the returned random numbers is zero and the standard
	/// deviation is the specified sigma .
	fn gaussian(&mut self, sigma: f64) -> Result<f64> {
		unsafe { sys::cv_RNG_gaussian_double(self.as_raw_mut_RNG(), sigma) }.into_result()
	}
	
}

/// Random Number Generator
/// 
/// Random number generator. It encapsulates the state (currently, a 64-bit
/// integer) and has methods to return scalar random values and to fill
/// arrays with random values. Currently it supports uniform and Gaussian
/// (normal) distributions. The generator uses Multiply-With-Carry
/// algorithm, introduced by G. Marsaglia (
/// <http://en.wikipedia.org/wiki/Multiply-with-carry> ).
/// Gaussian-distribution random numbers are generated using the Ziggurat
/// algorithm ( <http://en.wikipedia.org/wiki/Ziggurat_algorithm> ),
/// introduced by G. Marsaglia and W. W. Tsang.
pub struct RNG {
	ptr: *mut c_void
}

opencv_type_boxed! { RNG }

impl Drop for RNG {
	fn drop(&mut self) {
		extern "C" { fn cv_RNG_delete(instance: *mut c_void); }
		unsafe { cv_RNG_delete(self.as_raw_mut_RNG()) };
	}
}

impl RNG {
	#[inline] pub fn as_raw_RNG(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_RNG(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for RNG {}

impl core::RNGTrait for RNG {
	#[inline] fn as_raw_RNG(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_RNG(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RNG {
	/// constructor
	/// 
	/// These are the RNG constructors. The first form sets the state to some
	/// pre-defined value, equal to 2\*\*32-1 in the current implementation. The
	/// second form sets the state to the specified value. If you passed state=0
	/// , the constructor uses the above default value instead to avoid the
	/// singular random number sequence, consisting of all zeros.
	pub fn default() -> Result<core::RNG> {
		unsafe { sys::cv_RNG_RNG() }.into_result().map(|r| unsafe { core::RNG::opencv_from_extern(r) } )
	}
	
	/// constructor
	/// 
	/// These are the RNG constructors. The first form sets the state to some
	/// pre-defined value, equal to 2\*\*32-1 in the current implementation. The
	/// second form sets the state to the specified value. If you passed state=0
	/// , the constructor uses the above default value instead to avoid the
	/// singular random number sequence, consisting of all zeros.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * state: 64-bit value used to initialize the RNG.
	pub fn new(state: u64) -> Result<core::RNG> {
		unsafe { sys::cv_RNG_RNG_uint64_t(state) }.into_result().map(|r| unsafe { core::RNG::opencv_from_extern(r) } )
	}
	
}

/// Mersenne Twister random number generator
/// 
/// Inspired by http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/MT2002/CODES/mt19937ar.c
/// @todo document
pub trait RNG_MT19937Trait {
	fn as_raw_RNG_MT19937(&self) -> *const c_void;
	fn as_raw_mut_RNG_MT19937(&mut self) -> *mut c_void;

	fn seed(&mut self, s: u32) -> Result<()> {
		unsafe { sys::cv_RNG_MT19937_seed_unsigned_int(self.as_raw_mut_RNG_MT19937(), s) }.into_result()
	}
	
	fn next(&mut self) -> Result<u32> {
		unsafe { sys::cv_RNG_MT19937_next(self.as_raw_mut_RNG_MT19937()) }.into_result()
	}
	
	fn to_i32(&mut self) -> Result<i32> {
		unsafe { sys::cv_RNG_MT19937_operator_int(self.as_raw_mut_RNG_MT19937()) }.into_result()
	}
	
	fn to_u32(&mut self) -> Result<u32> {
		unsafe { sys::cv_RNG_MT19937_operator_unsigned_int(self.as_raw_mut_RNG_MT19937()) }.into_result()
	}
	
	fn to_f32(&mut self) -> Result<f32> {
		unsafe { sys::cv_RNG_MT19937_operator_float(self.as_raw_mut_RNG_MT19937()) }.into_result()
	}
	
	fn to_f64(&mut self) -> Result<f64> {
		unsafe { sys::cv_RNG_MT19937_operator_double(self.as_raw_mut_RNG_MT19937()) }.into_result()
	}
	
	/// returns uniformly distributed integer random number from [a,b) range
	fn uniform(&mut self, a: i32, b: i32) -> Result<i32> {
		unsafe { sys::cv_RNG_MT19937_uniform_int_int(self.as_raw_mut_RNG_MT19937(), a, b) }.into_result()
	}
	
	/// returns uniformly distributed floating-point random number from [a,b) range
	fn uniform_1(&mut self, a: f32, b: f32) -> Result<f32> {
		unsafe { sys::cv_RNG_MT19937_uniform_float_float(self.as_raw_mut_RNG_MT19937(), a, b) }.into_result()
	}
	
	/// returns uniformly distributed double-precision floating-point random number from [a,b) range
	fn uniform_2(&mut self, a: f64, b: f64) -> Result<f64> {
		unsafe { sys::cv_RNG_MT19937_uniform_double_double(self.as_raw_mut_RNG_MT19937(), a, b) }.into_result()
	}
	
}

/// Mersenne Twister random number generator
/// 
/// Inspired by http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/MT2002/CODES/mt19937ar.c
/// @todo document
pub struct RNG_MT19937 {
	ptr: *mut c_void
}

opencv_type_boxed! { RNG_MT19937 }

impl Drop for RNG_MT19937 {
	fn drop(&mut self) {
		extern "C" { fn cv_RNG_MT19937_delete(instance: *mut c_void); }
		unsafe { cv_RNG_MT19937_delete(self.as_raw_mut_RNG_MT19937()) };
	}
}

impl RNG_MT19937 {
	#[inline] pub fn as_raw_RNG_MT19937(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_RNG_MT19937(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for RNG_MT19937 {}

impl core::RNG_MT19937Trait for RNG_MT19937 {
	#[inline] fn as_raw_RNG_MT19937(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_RNG_MT19937(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RNG_MT19937 {
	pub fn default() -> Result<core::RNG_MT19937> {
		unsafe { sys::cv_RNG_MT19937_RNG_MT19937() }.into_result().map(|r| unsafe { core::RNG_MT19937::opencv_from_extern(r) } )
	}
	
	pub fn new(s: u32) -> Result<core::RNG_MT19937> {
		unsafe { sys::cv_RNG_MT19937_RNG_MT19937_unsigned_int(s) }.into_result().map(|r| unsafe { core::RNG_MT19937::opencv_from_extern(r) } )
	}
	
}

/// Template class specifying a continuous subsequence (slice) of a sequence.
/// 
/// The class is used to specify a row or a column span in a matrix ( Mat ) and for many other purposes.
/// Range(a,b) is basically the same as a:b in Matlab or a..b in Python. As in Python, start is an
/// inclusive left boundary of the range and end is an exclusive right boundary of the range. Such a
/// half-opened interval is usually denoted as ![inline formula](https://latex.codecogs.com/png.latex?%5Bstart%2Cend%29) .
/// 
/// The static method Range::all() returns a special variable that means "the whole sequence" or "the
/// whole range", just like " : " in Matlab or " ... " in Python. All the methods and functions in
/// OpenCV that take Range support this special Range::all() value. But, of course, in case of your own
/// custom processing, you will probably have to check and handle it explicitly:
/// ```ignore
///    void my_function(..., const Range& r, ....)
///    {
///        if(r == Range::all()) {
///            // process all the data
///        }
///        else {
///            // process [r.start, r.end)
///        }
///    }
/// ```
/// 
pub trait RangeTrait {
	fn as_raw_Range(&self) -> *const c_void;
	fn as_raw_mut_Range(&mut self) -> *mut c_void;

	fn start(&self) -> i32 {
		unsafe { sys::cv_Range_getPropStart_const(self.as_raw_Range()) }.into_result().expect("Infallible function failed: start")
	}
	
	fn set_start(&mut self, val: i32) -> () {
		unsafe { sys::cv_Range_setPropStart_int(self.as_raw_mut_Range(), val) }.into_result().expect("Infallible function failed: set_start")
	}
	
	fn end(&self) -> i32 {
		unsafe { sys::cv_Range_getPropEnd_const(self.as_raw_Range()) }.into_result().expect("Infallible function failed: end")
	}
	
	fn set_end(&mut self, val: i32) -> () {
		unsafe { sys::cv_Range_setPropEnd_int(self.as_raw_mut_Range(), val) }.into_result().expect("Infallible function failed: set_end")
	}
	
	fn size(&self) -> Result<i32> {
		unsafe { sys::cv_Range_size_const(self.as_raw_Range()) }.into_result()
	}
	
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_Range_empty_const(self.as_raw_Range()) }.into_result()
	}
	
}

/// Template class specifying a continuous subsequence (slice) of a sequence.
/// 
/// The class is used to specify a row or a column span in a matrix ( Mat ) and for many other purposes.
/// Range(a,b) is basically the same as a:b in Matlab or a..b in Python. As in Python, start is an
/// inclusive left boundary of the range and end is an exclusive right boundary of the range. Such a
/// half-opened interval is usually denoted as ![inline formula](https://latex.codecogs.com/png.latex?%5Bstart%2Cend%29) .
/// 
/// The static method Range::all() returns a special variable that means "the whole sequence" or "the
/// whole range", just like " : " in Matlab or " ... " in Python. All the methods and functions in
/// OpenCV that take Range support this special Range::all() value. But, of course, in case of your own
/// custom processing, you will probably have to check and handle it explicitly:
/// ```ignore
///    void my_function(..., const Range& r, ....)
///    {
///        if(r == Range::all()) {
///            // process all the data
///        }
///        else {
///            // process [r.start, r.end)
///        }
///    }
/// ```
/// 
pub struct Range {
	ptr: *mut c_void
}

opencv_type_boxed! { Range }

impl Drop for Range {
	fn drop(&mut self) {
		extern "C" { fn cv_Range_delete(instance: *mut c_void); }
		unsafe { cv_Range_delete(self.as_raw_mut_Range()) };
	}
}

impl Range {
	#[inline] pub fn as_raw_Range(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Range(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Range {}

impl core::RangeTrait for Range {
	#[inline] fn as_raw_Range(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Range(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Range {
	/// ////////////////////////////// Range /////////////////////////////////
	pub fn default() -> Result<core::Range> {
		unsafe { sys::cv_Range_Range() }.into_result().map(|r| unsafe { core::Range::opencv_from_extern(r) } )
	}
	
	pub fn new(_start: i32, _end: i32) -> Result<core::Range> {
		unsafe { sys::cv_Range_Range_int_int(_start, _end) }.into_result().map(|r| unsafe { core::Range::opencv_from_extern(r) } )
	}
	
	pub fn all() -> Result<core::Range> {
		unsafe { sys::cv_Range_all() }.into_result().map(|r| unsafe { core::Range::opencv_from_extern(r) } )
	}
	
}

/// The class represents rotated (i.e. not up-right) rectangles on a plane.
/// 
/// Each rectangle is specified by the center point (mass center), length of each side (represented by
/// #Size2f structure) and the rotation angle in degrees.
/// 
/// The sample below demonstrates how to use RotatedRect:
/// [RotatedRect_demo](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_various.cpp#L1)
/// ![image](https://docs.opencv.org/4.3.0/rotatedrect.png)
/// ## See also
/// CamShift, fitEllipse, minAreaRect, CvBox2D
pub trait RotatedRectTrait {
	fn as_raw_RotatedRect(&self) -> *const c_void;
	fn as_raw_mut_RotatedRect(&mut self) -> *mut c_void;

	/// returns the rectangle mass center
	fn center(&self) -> core::Point2f {
		unsafe { sys::cv_RotatedRect_getPropCenter_const(self.as_raw_RotatedRect()) }.into_result().expect("Infallible function failed: center")
	}
	
	/// returns the rectangle mass center
	fn set_center(&mut self, val: core::Point2f) -> () {
		unsafe { sys::cv_RotatedRect_setPropCenter_Point2f(self.as_raw_mut_RotatedRect(), val.opencv_to_extern()) }.into_result().expect("Infallible function failed: set_center")
	}
	
	/// returns width and height of the rectangle
	fn size(&self) -> core::Size2f {
		unsafe { sys::cv_RotatedRect_getPropSize_const(self.as_raw_RotatedRect()) }.into_result().expect("Infallible function failed: size")
	}
	
	/// returns width and height of the rectangle
	fn set_size(&mut self, val: core::Size2f) -> () {
		unsafe { sys::cv_RotatedRect_setPropSize_Size2f(self.as_raw_mut_RotatedRect(), val.opencv_to_extern()) }.into_result().expect("Infallible function failed: set_size")
	}
	
	/// returns the rotation angle. When the angle is 0, 90, 180, 270 etc., the rectangle becomes an up-right rectangle.
	fn angle(&self) -> f32 {
		unsafe { sys::cv_RotatedRect_getPropAngle_const(self.as_raw_RotatedRect()) }.into_result().expect("Infallible function failed: angle")
	}
	
	/// returns the rotation angle. When the angle is 0, 90, 180, 270 etc., the rectangle becomes an up-right rectangle.
	fn set_angle(&mut self, val: f32) -> () {
		unsafe { sys::cv_RotatedRect_setPropAngle_float(self.as_raw_mut_RotatedRect(), val) }.into_result().expect("Infallible function failed: set_angle")
	}
	
	/// returns 4 vertices of the rectangle
	/// ## Parameters
	/// * pts: The points array for storing rectangle vertices. The order is bottomLeft, topLeft, topRight, bottomRight.
	fn points(&self, pts: &mut [core::Point2f]) -> Result<()> {
		unsafe { sys::cv_RotatedRect_points_const_Point2fX(self.as_raw_RotatedRect(), pts.as_mut_ptr()) }.into_result()
	}
	
	/// returns the minimal up-right integer rectangle containing the rotated rectangle
	fn bounding_rect(&self) -> Result<core::Rect> {
		unsafe { sys::cv_RotatedRect_boundingRect_const(self.as_raw_RotatedRect()) }.into_result()
	}
	
	/// returns the minimal (exact) floating point rectangle containing the rotated rectangle, not intended for use with images
	fn bounding_rect2f(&self) -> Result<core::Rect_<f32>> {
		unsafe { sys::cv_RotatedRect_boundingRect2f_const(self.as_raw_RotatedRect()) }.into_result()
	}
	
}

/// The class represents rotated (i.e. not up-right) rectangles on a plane.
/// 
/// Each rectangle is specified by the center point (mass center), length of each side (represented by
/// #Size2f structure) and the rotation angle in degrees.
/// 
/// The sample below demonstrates how to use RotatedRect:
/// [RotatedRect_demo](https://github.com/opencv/opencv/blob/4.3.0/samples/cpp/tutorial_code/snippets/core_various.cpp#L1)
/// ![image](https://docs.opencv.org/4.3.0/rotatedrect.png)
/// ## See also
/// CamShift, fitEllipse, minAreaRect, CvBox2D
pub struct RotatedRect {
	ptr: *mut c_void
}

opencv_type_boxed! { RotatedRect }

impl Drop for RotatedRect {
	fn drop(&mut self) {
		extern "C" { fn cv_RotatedRect_delete(instance: *mut c_void); }
		unsafe { cv_RotatedRect_delete(self.as_raw_mut_RotatedRect()) };
	}
}

impl RotatedRect {
	#[inline] pub fn as_raw_RotatedRect(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_RotatedRect(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for RotatedRect {}

impl core::RotatedRectTrait for RotatedRect {
	#[inline] fn as_raw_RotatedRect(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_RotatedRect(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RotatedRect {
	/// default constructor
	pub fn default() -> Result<core::RotatedRect> {
		unsafe { sys::cv_RotatedRect_RotatedRect() }.into_result().map(|r| unsafe { core::RotatedRect::opencv_from_extern(r) } )
	}
	
	/// full constructor
	/// ## Parameters
	/// * center: The rectangle mass center.
	/// * size: Width and height of the rectangle.
	/// * angle: The rotation angle in a clockwise direction. When the angle is 0, 90, 180, 270 etc.,
	/// the rectangle becomes an up-right rectangle.
	pub fn new(center: core::Point2f, size: core::Size2f, angle: f32) -> Result<core::RotatedRect> {
		unsafe { sys::cv_RotatedRect_RotatedRect_const_Point2fR_const_Size2fR_float(&center, &size, angle) }.into_result().map(|r| unsafe { core::RotatedRect::opencv_from_extern(r) } )
	}
	
	/// Any 3 end points of the RotatedRect. They must be given in order (either clockwise or
	/// anticlockwise).
	pub fn for_points(point1: core::Point2f, point2: core::Point2f, point3: core::Point2f) -> Result<core::RotatedRect> {
		unsafe { sys::cv_RotatedRect_RotatedRect_const_Point2fR_const_Point2fR_const_Point2fR(&point1, &point2, &point3) }.into_result().map(|r| unsafe { core::RotatedRect::opencv_from_extern(r) } )
	}
	
}

/// Singular Value Decomposition
/// 
/// Class for computing Singular Value Decomposition of a floating-point
/// matrix. The Singular Value Decomposition is used to solve least-square
/// problems, under-determined linear systems, invert matrices, compute
/// condition numbers, and so on.
/// 
/// If you want to compute a condition number of a matrix or an absolute value of
/// its determinant, you do not need `u` and `vt`. You can pass
/// flags=SVD::NO_UV|... . Another flag SVD::FULL_UV indicates that full-size u
/// and vt must be computed, which is not necessary most of the time.
/// ## See also
/// invert, solve, eigen, determinant
pub trait SVDTrait {
	fn as_raw_SVD(&self) -> *const c_void;
	fn as_raw_mut_SVD(&mut self) -> *mut c_void;

	fn u(&mut self) -> core::Mat {
		unsafe { sys::cv_SVD_getPropU(self.as_raw_mut_SVD()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: u")
	}
	
	fn set_u(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_SVD_setPropU_Mat(self.as_raw_mut_SVD(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_u")
	}
	
	fn w(&mut self) -> core::Mat {
		unsafe { sys::cv_SVD_getPropW(self.as_raw_mut_SVD()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: w")
	}
	
	fn set_w(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_SVD_setPropW_Mat(self.as_raw_mut_SVD(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_w")
	}
	
	fn vt(&mut self) -> core::Mat {
		unsafe { sys::cv_SVD_getPropVt(self.as_raw_mut_SVD()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: vt")
	}
	
	fn set_vt(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_SVD_setPropVt_Mat(self.as_raw_mut_SVD(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_vt")
	}
	
	/// performs a singular value back substitution.
	/// 
	/// The method calculates a back substitution for the specified right-hand
	/// side:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bx%7D%20%3D%20%20%5Ctexttt%7Bvt%7D%20%5ET%20%20%5Ccdot%20diag%28%20%5Ctexttt%7Bw%7D%20%29%5E%7B%2D1%7D%20%20%5Ccdot%20%5Ctexttt%7Bu%7D%20%5ET%20%20%5Ccdot%20%5Ctexttt%7Brhs%7D%20%5Csim%20%5Ctexttt%7BA%7D%20%5E%7B%2D1%7D%20%20%5Ccdot%20%5Ctexttt%7Brhs%7D)
	/// 
	/// Using this technique you can either get a very accurate solution of the
	/// convenient linear system, or the best (in the least-squares terms)
	/// pseudo-solution of an overdetermined linear system.
	/// 
	/// ## Parameters
	/// * rhs: right-hand side of a linear system (u\*w\*v')\*dst = rhs to
	/// be solved, where A has been previously decomposed.
	/// 
	/// * dst: found solution of the system.
	/// 
	/// 
	/// Note: Explicit SVD with the further back substitution only makes sense
	/// if you need to solve many linear systems with the same left-hand side
	/// (for example, src ). If all you need is to solve a single system
	/// (possibly with multiple rhs immediately available), simply call solve
	/// add pass #DECOMP_SVD there. It does absolutely the same thing.
	fn back_subst(&self, rhs: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(rhs);
		output_array_arg!(dst);
		unsafe { sys::cv_SVD_backSubst_const_const__InputArrayR_const__OutputArrayR(self.as_raw_SVD(), rhs.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
	}
	
}

/// Singular Value Decomposition
/// 
/// Class for computing Singular Value Decomposition of a floating-point
/// matrix. The Singular Value Decomposition is used to solve least-square
/// problems, under-determined linear systems, invert matrices, compute
/// condition numbers, and so on.
/// 
/// If you want to compute a condition number of a matrix or an absolute value of
/// its determinant, you do not need `u` and `vt`. You can pass
/// flags=SVD::NO_UV|... . Another flag SVD::FULL_UV indicates that full-size u
/// and vt must be computed, which is not necessary most of the time.
/// ## See also
/// invert, solve, eigen, determinant
pub struct SVD {
	ptr: *mut c_void
}

opencv_type_boxed! { SVD }

impl Drop for SVD {
	fn drop(&mut self) {
		extern "C" { fn cv_SVD_delete(instance: *mut c_void); }
		unsafe { cv_SVD_delete(self.as_raw_mut_SVD()) };
	}
}

impl SVD {
	#[inline] pub fn as_raw_SVD(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SVD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SVD {}

impl core::SVDTrait for SVD {
	#[inline] fn as_raw_SVD(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SVD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SVD {
	/// the default constructor
	/// 
	/// initializes an empty SVD structure
	pub fn default() -> Result<core::SVD> {
		unsafe { sys::cv_SVD_SVD() }.into_result().map(|r| unsafe { core::SVD::opencv_from_extern(r) } )
	}
	
	/// the default constructor
	/// 
	/// initializes an empty SVD structure
	/// 
	/// ## Overloaded parameters
	/// 
	///    initializes an empty SVD structure and then calls SVD::operator()
	/// ## Parameters
	/// * src: decomposed matrix. The depth has to be CV_32F or CV_64F.
	/// * flags: operation flags (SVD::Flags)
	/// 
	/// ## C++ default parameters
	/// * flags: 0
	pub fn new(src: &dyn core::ToInputArray, flags: i32) -> Result<core::SVD> {
		input_array_arg!(src);
		unsafe { sys::cv_SVD_SVD_const__InputArrayR_int(src.as_raw__InputArray(), flags) }.into_result().map(|r| unsafe { core::SVD::opencv_from_extern(r) } )
	}
	
	/// decomposes matrix and stores the results to user-provided matrices
	/// 
	/// The methods/functions perform SVD of matrix. Unlike SVD::SVD constructor
	/// and SVD::operator(), they store the results to the user-provided
	/// matrices:
	/// 
	/// ```ignore
	/// Mat A, w, u, vt;
	/// SVD::compute(A, w, u, vt);
	/// ```
	/// 
	/// 
	/// ## Parameters
	/// * src: decomposed matrix. The depth has to be CV_32F or CV_64F.
	/// * w: calculated singular values
	/// * u: calculated left singular vectors
	/// * vt: transposed matrix of right singular vectors
	/// * flags: operation flags - see SVD::Flags.
	/// 
	/// ## C++ default parameters
	/// * flags: 0
	pub fn compute_ext(src: &dyn core::ToInputArray, w: &mut dyn core::ToOutputArray, u: &mut dyn core::ToOutputArray, vt: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(w);
		output_array_arg!(u);
		output_array_arg!(vt);
		unsafe { sys::cv_SVD_compute_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), w.as_raw__OutputArray(), u.as_raw__OutputArray(), vt.as_raw__OutputArray(), flags) }.into_result()
	}
	
	/// @todo document
	/// 
	/// ## Overloaded parameters
	/// 
	///    computes singular values of a matrix
	/// ## Parameters
	/// * src: decomposed matrix. The depth has to be CV_32F or CV_64F.
	/// * w: calculated singular values
	/// * flags: operation flags - see SVD::Flags.
	/// 
	/// ## C++ default parameters
	/// * flags: 0
	pub fn compute(src: &dyn core::ToInputArray, w: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(w);
		unsafe { sys::cv_SVD_compute_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), w.as_raw__OutputArray(), flags) }.into_result()
	}
	
	/// performs back substitution
	pub fn back_subst_multi(w: &dyn core::ToInputArray, u: &dyn core::ToInputArray, vt: &dyn core::ToInputArray, rhs: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(w);
		input_array_arg!(u);
		input_array_arg!(vt);
		input_array_arg!(rhs);
		output_array_arg!(dst);
		unsafe { sys::cv_SVD_backSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(w.as_raw__InputArray(), u.as_raw__InputArray(), vt.as_raw__InputArray(), rhs.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
	}
	
	/// solves an under-determined singular linear system
	/// 
	/// The method finds a unit-length solution x of a singular linear system
	/// A\*x = 0. Depending on the rank of A, there can be no solutions, a
	/// single solution or an infinite number of solutions. In general, the
	/// algorithm solves the following problem:
	/// ![block formula](https://latex.codecogs.com/png.latex?dst%20%3D%20%20%5Carg%20%5Cmin%20%5F%7Bx%3A%20%20%5C%7C%20x%20%5C%7C%20%3D1%7D%20%20%5C%7C%20src%20%20%5Ccdot%20x%20%20%5C%7C)
	/// ## Parameters
	/// * src: left-hand-side matrix.
	/// * dst: found solution.
	pub fn solve_z(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		unsafe { sys::cv_SVD_solveZ_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
	}
	
}

/// The class SparseMat represents multi-dimensional sparse numerical arrays.
/// 
/// Such a sparse array can store elements of any type that Mat can store. *Sparse* means that only
/// non-zero elements are stored (though, as a result of operations on a sparse matrix, some of its
/// stored elements can actually become 0. It is up to you to detect such elements and delete them
/// using SparseMat::erase ). The non-zero elements are stored in a hash table that grows when it is
/// filled so that the search time is O(1) in average (regardless of whether element is there or not).
/// Elements can be accessed using the following methods:
/// *   Query operations (SparseMat::ptr and the higher-level SparseMat::ref, SparseMat::value and
///    SparseMat::find), for example:
///    ```ignore
///        const int dims = 5;
///        int size[5] = {10, 10, 10, 10, 10};
///        SparseMat sparse_mat(dims, size, CV_32F);
///        for(int i = 0; i < 1000; i++)
///        {
///            int idx[dims];
///            for(int k = 0; k < dims; k++)
///                idx[k] = rand() % size[k];
///            sparse_mat.ref<float>(idx) += 1.f;
///        }
///        cout << "nnz = " << sparse_mat.nzcount() << endl;
///    ```
/// 
/// *   Sparse matrix iterators. They are similar to MatIterator but different from NAryMatIterator.
///    That is, the iteration loop is familiar to STL users:
///    ```ignore
///        // prints elements of a sparse floating-point matrix
///        // and the sum of elements.
///        SparseMatConstIterator_<float>
///            it = sparse_mat.begin<float>(),
///            it_end = sparse_mat.end<float>();
///        double s = 0;
///        int dims = sparse_mat.dims();
///        for(; it != it_end; ++it)
///        {
///            // print element indices and the element value
///            const SparseMat::Node* n = it.node();
///            printf("(");
///            for(int i = 0; i < dims; i++)
///                printf("%d%s", n->idx[i], i < dims-1 ? ", " : ")");
///            printf(": %g\n", it.value<float>());
///            s += *it;
///        }
///        printf("Element sum is %g\n", s);
///    ```
/// 
///    If you run this loop, you will notice that elements are not enumerated in a logical order
///    (lexicographical, and so on). They come in the same order as they are stored in the hash table
///    (semi-randomly). You may collect pointers to the nodes and sort them to get the proper ordering.
///    Note, however, that pointers to the nodes may become invalid when you add more elements to the
///    matrix. This may happen due to possible buffer reallocation.
/// *   Combination of the above 2 methods when you need to process 2 or more sparse matrices
///    simultaneously. For example, this is how you can compute unnormalized cross-correlation of the 2
///    floating-point sparse matrices:
///    ```ignore
///        double cross_corr(const SparseMat& a, const SparseMat& b)
///        {
///            const SparseMat *_a = &a, *_b = &b;
///            // if b contains less elements than a,
///            // it is faster to iterate through b
///            if(_a->nzcount() > _b->nzcount())
///                std::swap(_a, _b);
///            SparseMatConstIterator_<float> it = _a->begin<float>(),
///                                            it_end = _a->end<float>();
///            double ccorr = 0;
///            for(; it != it_end; ++it)
///            {
///                // take the next element from the first matrix
///                float avalue = *it;
///                const Node* anode = it.node();
///                // and try to find an element with the same index in the second matrix.
///                // since the hash value depends only on the element index,
///                // reuse the hash value stored in the node
///                float bvalue = _b->value<float>(anode->idx,&anode->hashval);
///                ccorr += avalue*bvalue;
///            }
///            return ccorr;
///        }
///    ```
/// 
pub trait SparseMatTrait {
	fn as_raw_SparseMat(&self) -> *const c_void;
	fn as_raw_mut_SparseMat(&mut self) -> *mut c_void;

	fn flags(&self) -> i32 {
		unsafe { sys::cv_SparseMat_getPropFlags_const(self.as_raw_SparseMat()) }.into_result().expect("Infallible function failed: flags")
	}
	
	fn set_flags(&mut self, val: i32) -> () {
		unsafe { sys::cv_SparseMat_setPropFlags_int(self.as_raw_mut_SparseMat(), val) }.into_result().expect("Infallible function failed: set_flags")
	}
	
	fn hdr(&mut self) -> core::SparseMat_Hdr {
		unsafe { sys::cv_SparseMat_getPropHdr(self.as_raw_mut_SparseMat()) }.into_result().map(|r| unsafe { core::SparseMat_Hdr::opencv_from_extern(r) } ).expect("Infallible function failed: hdr")
	}
	
	fn set_hdr(&mut self, val: &mut core::SparseMat_Hdr) -> () {
		unsafe { sys::cv_SparseMat_setPropHdr_HdrX(self.as_raw_mut_SparseMat(), val.as_raw_mut_SparseMat_Hdr()) }.into_result().expect("Infallible function failed: set_hdr")
	}
	
	/// creates full copy of the matrix
	fn clone(&self) -> Result<core::SparseMat> {
		unsafe { sys::cv_SparseMat_clone_const(self.as_raw_SparseMat()) }.into_result().map(|r| unsafe { core::SparseMat::opencv_from_extern(r) } )
	}
	
	/// copies all the data to the destination matrix. All the previous content of m is erased
	fn copy_to(&self, m: &mut core::SparseMat) -> Result<()> {
		unsafe { sys::cv_SparseMat_copyTo_const_SparseMatR(self.as_raw_SparseMat(), m.as_raw_mut_SparseMat()) }.into_result()
	}
	
	/// converts sparse matrix to dense matrix.
	fn copy_to_mat(&self, m: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_SparseMat_copyTo_const_MatR(self.as_raw_SparseMat(), m.as_raw_mut_Mat()) }.into_result()
	}
	
	/// multiplies all the matrix elements by the specified scale factor alpha and converts the results to the specified data type
	/// 
	/// ## C++ default parameters
	/// * alpha: 1
	fn convert_to(&self, m: &mut core::SparseMat, rtype: i32, alpha: f64) -> Result<()> {
		unsafe { sys::cv_SparseMat_convertTo_const_SparseMatR_int_double(self.as_raw_SparseMat(), m.as_raw_mut_SparseMat(), rtype, alpha) }.into_result()
	}
	
	/// converts sparse matrix to dense n-dim matrix with optional type conversion and scaling.
	/// 
	/// ## Parameters
	/// * m:[out] - output matrix; if it does not have a proper size or type before the operation,
	///        it is reallocated
	/// * rtype: - desired output matrix type or, rather, the depth since the number of channels
	///        are the same as the input has; if rtype is negative, the output matrix will have the
	///        same type as the input.
	/// * alpha: - optional scale factor
	/// * beta: - optional delta added to the scaled values
	/// 
	/// ## C++ default parameters
	/// * alpha: 1
	/// * beta: 0
	fn convert_to_1(&self, m: &mut core::Mat, rtype: i32, alpha: f64, beta: f64) -> Result<()> {
		unsafe { sys::cv_SparseMat_convertTo_const_MatR_int_double_double(self.as_raw_SparseMat(), m.as_raw_mut_Mat(), rtype, alpha, beta) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * typ: -1
	fn assign_to(&self, m: &mut core::SparseMat, typ: i32) -> Result<()> {
		unsafe { sys::cv_SparseMat_assignTo_const_SparseMatR_int(self.as_raw_SparseMat(), m.as_raw_mut_SparseMat(), typ) }.into_result()
	}
	
	/// reallocates sparse matrix.
	/// 
	///    If the matrix already had the proper size and type,
	///    it is simply cleared with clear(), otherwise,
	///    the old matrix is released (using release()) and the new one is allocated.
	fn create(&mut self, dims: i32, _sizes: &i32, _type: i32) -> Result<()> {
		unsafe { sys::cv_SparseMat_create_int_const_intX_int(self.as_raw_mut_SparseMat(), dims, _sizes, _type) }.into_result()
	}
	
	/// sets all the sparse matrix elements to 0, which means clearing the hash table.
	fn clear(&mut self) -> Result<()> {
		unsafe { sys::cv_SparseMat_clear(self.as_raw_mut_SparseMat()) }.into_result()
	}
	
	/// manually increments the reference counter to the header.
	fn addref(&mut self) -> Result<()> {
		unsafe { sys::cv_SparseMat_addref(self.as_raw_mut_SparseMat()) }.into_result()
	}
	
	fn release(&mut self) -> Result<()> {
		unsafe { sys::cv_SparseMat_release(self.as_raw_mut_SparseMat()) }.into_result()
	}
	
	/// returns the size of each element in bytes (not including the overhead - the space occupied by SparseMat::Node elements)
	fn elem_size(&self) -> Result<size_t> {
		unsafe { sys::cv_SparseMat_elemSize_const(self.as_raw_SparseMat()) }.into_result()
	}
	
	/// returns elemSize()/channels()
	fn elem_size1(&self) -> Result<size_t> {
		unsafe { sys::cv_SparseMat_elemSize1_const(self.as_raw_SparseMat()) }.into_result()
	}
	
	/// returns type of sparse matrix elements
	fn typ(&self) -> Result<i32> {
		unsafe { sys::cv_SparseMat_type_const(self.as_raw_SparseMat()) }.into_result()
	}
	
	/// returns the depth of sparse matrix elements
	fn depth(&self) -> Result<i32> {
		unsafe { sys::cv_SparseMat_depth_const(self.as_raw_SparseMat()) }.into_result()
	}
	
	/// returns the number of channels
	fn channels(&self) -> Result<i32> {
		unsafe { sys::cv_SparseMat_channels_const(self.as_raw_SparseMat()) }.into_result()
	}
	
	/// returns the array of sizes, or NULL if the matrix is not allocated
	fn size(&self) -> Result<&i32> {
		unsafe { sys::cv_SparseMat_size_const(self.as_raw_SparseMat()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// returns the size of i-th matrix dimension (or 0)
	fn size_1(&self, i: i32) -> Result<i32> {
		unsafe { sys::cv_SparseMat_size_const_int(self.as_raw_SparseMat(), i) }.into_result()
	}
	
	/// returns the matrix dimensionality
	fn dims(&self) -> Result<i32> {
		unsafe { sys::cv_SparseMat_dims_const(self.as_raw_SparseMat()) }.into_result()
	}
	
	/// returns the number of non-zero elements (=the number of hash table nodes)
	fn nzcount(&self) -> Result<size_t> {
		unsafe { sys::cv_SparseMat_nzcount_const(self.as_raw_SparseMat()) }.into_result()
	}
	
	/// computes the element hash value (1D case)
	fn hash(&self, i0: i32) -> Result<size_t> {
		unsafe { sys::cv_SparseMat_hash_const_int(self.as_raw_SparseMat(), i0) }.into_result()
	}
	
	/// computes the element hash value (2D case)
	fn hash_1(&self, i0: i32, i1: i32) -> Result<size_t> {
		unsafe { sys::cv_SparseMat_hash_const_int_int(self.as_raw_SparseMat(), i0, i1) }.into_result()
	}
	
	/// computes the element hash value (3D case)
	fn hash_2(&self, i0: i32, i1: i32, i2: i32) -> Result<size_t> {
		unsafe { sys::cv_SparseMat_hash_const_int_int_int(self.as_raw_SparseMat(), i0, i1, i2) }.into_result()
	}
	
	/// computes the element hash value (nD case)
	fn hash_3(&self, idx: &i32) -> Result<size_t> {
		unsafe { sys::cv_SparseMat_hash_const_const_intX(self.as_raw_SparseMat(), idx) }.into_result()
	}
	
	/// specialized variants for 1D, 2D, 3D cases and the generic_type one for n-D case.
	///    return pointer to the matrix element.
	///      - if the element is there (it's non-zero), the pointer to it is returned
	///      - if it's not there and createMissing=false, NULL pointer is returned
	///      - if it's not there and createMissing=true, then the new element
	///        is created and initialized with 0. Pointer to it is returned
	///      - if the optional hashval pointer is not NULL, the element hash value is
	///        not computed, but *hashval is taken instead.
	/// 
	/// returns pointer to the specified element (1D case)
	/// 
	/// ## C++ default parameters
	/// * hashval: 0
	fn ptr(&mut self, i0: i32, create_missing: bool, hashval: &mut size_t) -> Result<&mut u8> {
		unsafe { sys::cv_SparseMat_ptr_int_bool_size_tX(self.as_raw_mut_SparseMat(), i0, create_missing, hashval) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// returns pointer to the specified element (2D case)
	/// 
	/// ## C++ default parameters
	/// * hashval: 0
	fn ptr_1(&mut self, i0: i32, i1: i32, create_missing: bool, hashval: &mut size_t) -> Result<&mut u8> {
		unsafe { sys::cv_SparseMat_ptr_int_int_bool_size_tX(self.as_raw_mut_SparseMat(), i0, i1, create_missing, hashval) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// returns pointer to the specified element (3D case)
	/// 
	/// ## C++ default parameters
	/// * hashval: 0
	fn ptr_2(&mut self, i0: i32, i1: i32, i2: i32, create_missing: bool, hashval: &mut size_t) -> Result<&mut u8> {
		unsafe { sys::cv_SparseMat_ptr_int_int_int_bool_size_tX(self.as_raw_mut_SparseMat(), i0, i1, i2, create_missing, hashval) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// returns pointer to the specified element (nD case)
	/// 
	/// ## C++ default parameters
	/// * hashval: 0
	fn ptr_3(&mut self, idx: &i32, create_missing: bool, hashval: &mut size_t) -> Result<&mut u8> {
		unsafe { sys::cv_SparseMat_ptr_const_intX_bool_size_tX(self.as_raw_mut_SparseMat(), idx, create_missing, hashval) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// erases the specified element (2D case)
	/// 
	/// ## C++ default parameters
	/// * hashval: 0
	fn erase(&mut self, i0: i32, i1: i32, hashval: &mut size_t) -> Result<()> {
		unsafe { sys::cv_SparseMat_erase_int_int_size_tX(self.as_raw_mut_SparseMat(), i0, i1, hashval) }.into_result()
	}
	
	/// erases the specified element (3D case)
	/// 
	/// ## C++ default parameters
	/// * hashval: 0
	fn erase_1(&mut self, i0: i32, i1: i32, i2: i32, hashval: &mut size_t) -> Result<()> {
		unsafe { sys::cv_SparseMat_erase_int_int_int_size_tX(self.as_raw_mut_SparseMat(), i0, i1, i2, hashval) }.into_result()
	}
	
	/// erases the specified element (nD case)
	/// 
	/// ## C++ default parameters
	/// * hashval: 0
	fn erase_2(&mut self, idx: &i32, hashval: &mut size_t) -> Result<()> {
		unsafe { sys::cv_SparseMat_erase_const_intX_size_tX(self.as_raw_mut_SparseMat(), idx, hashval) }.into_result()
	}
	
	/// return the sparse matrix iterator pointing to the first sparse matrix element
	/// 
	/// returns the sparse matrix iterator at the matrix beginning
	fn begin_mut(&mut self) -> Result<core::SparseMatIterator> {
		unsafe { sys::cv_SparseMat_begin(self.as_raw_mut_SparseMat()) }.into_result().map(|r| unsafe { core::SparseMatIterator::opencv_from_extern(r) } )
	}
	
	/// returns the read-only sparse matrix iterator at the matrix beginning
	fn begin(&self) -> Result<core::SparseMatConstIterator> {
		unsafe { sys::cv_SparseMat_begin_const(self.as_raw_SparseMat()) }.into_result().map(|r| unsafe { core::SparseMatConstIterator::opencv_from_extern(r) } )
	}
	
	/// return the sparse matrix iterator pointing to the element following the last sparse matrix element
	/// 
	/// returns the sparse matrix iterator at the matrix end
	fn end_mut(&mut self) -> Result<core::SparseMatIterator> {
		unsafe { sys::cv_SparseMat_end(self.as_raw_mut_SparseMat()) }.into_result().map(|r| unsafe { core::SparseMatIterator::opencv_from_extern(r) } )
	}
	
	/// returns the read-only sparse matrix iterator at the matrix end
	fn end(&self) -> Result<core::SparseMatConstIterator> {
		unsafe { sys::cv_SparseMat_end_const(self.as_raw_SparseMat()) }.into_result().map(|r| unsafe { core::SparseMatConstIterator::opencv_from_extern(r) } )
	}
	
	/// /////////// some internal-use methods ///////////////
	fn node(&mut self, nidx: size_t) -> Result<core::SparseMat_Node> {
		unsafe { sys::cv_SparseMat_node_size_t(self.as_raw_mut_SparseMat(), nidx) }.into_result().map(|r| unsafe { core::SparseMat_Node::opencv_from_extern(r) } )
	}
	
	fn node_1(&self, nidx: size_t) -> Result<core::SparseMat_Node> {
		unsafe { sys::cv_SparseMat_node_const_size_t(self.as_raw_SparseMat(), nidx) }.into_result().map(|r| unsafe { core::SparseMat_Node::opencv_from_extern(r) } )
	}
	
	fn new_node(&mut self, idx: &i32, hashval: size_t) -> Result<&mut u8> {
		unsafe { sys::cv_SparseMat_newNode_const_intX_size_t(self.as_raw_mut_SparseMat(), idx, hashval) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	fn remove_node(&mut self, hidx: size_t, nidx: size_t, previdx: size_t) -> Result<()> {
		unsafe { sys::cv_SparseMat_removeNode_size_t_size_t_size_t(self.as_raw_mut_SparseMat(), hidx, nidx, previdx) }.into_result()
	}
	
	fn resize_hash_tab(&mut self, newsize: size_t) -> Result<()> {
		unsafe { sys::cv_SparseMat_resizeHashTab_size_t(self.as_raw_mut_SparseMat(), newsize) }.into_result()
	}
	
}

/// The class SparseMat represents multi-dimensional sparse numerical arrays.
/// 
/// Such a sparse array can store elements of any type that Mat can store. *Sparse* means that only
/// non-zero elements are stored (though, as a result of operations on a sparse matrix, some of its
/// stored elements can actually become 0. It is up to you to detect such elements and delete them
/// using SparseMat::erase ). The non-zero elements are stored in a hash table that grows when it is
/// filled so that the search time is O(1) in average (regardless of whether element is there or not).
/// Elements can be accessed using the following methods:
/// *   Query operations (SparseMat::ptr and the higher-level SparseMat::ref, SparseMat::value and
///    SparseMat::find), for example:
///    ```ignore
///        const int dims = 5;
///        int size[5] = {10, 10, 10, 10, 10};
///        SparseMat sparse_mat(dims, size, CV_32F);
///        for(int i = 0; i < 1000; i++)
///        {
///            int idx[dims];
///            for(int k = 0; k < dims; k++)
///                idx[k] = rand() % size[k];
///            sparse_mat.ref<float>(idx) += 1.f;
///        }
///        cout << "nnz = " << sparse_mat.nzcount() << endl;
///    ```
/// 
/// *   Sparse matrix iterators. They are similar to MatIterator but different from NAryMatIterator.
///    That is, the iteration loop is familiar to STL users:
///    ```ignore
///        // prints elements of a sparse floating-point matrix
///        // and the sum of elements.
///        SparseMatConstIterator_<float>
///            it = sparse_mat.begin<float>(),
///            it_end = sparse_mat.end<float>();
///        double s = 0;
///        int dims = sparse_mat.dims();
///        for(; it != it_end; ++it)
///        {
///            // print element indices and the element value
///            const SparseMat::Node* n = it.node();
///            printf("(");
///            for(int i = 0; i < dims; i++)
///                printf("%d%s", n->idx[i], i < dims-1 ? ", " : ")");
///            printf(": %g\n", it.value<float>());
///            s += *it;
///        }
///        printf("Element sum is %g\n", s);
///    ```
/// 
///    If you run this loop, you will notice that elements are not enumerated in a logical order
///    (lexicographical, and so on). They come in the same order as they are stored in the hash table
///    (semi-randomly). You may collect pointers to the nodes and sort them to get the proper ordering.
///    Note, however, that pointers to the nodes may become invalid when you add more elements to the
///    matrix. This may happen due to possible buffer reallocation.
/// *   Combination of the above 2 methods when you need to process 2 or more sparse matrices
///    simultaneously. For example, this is how you can compute unnormalized cross-correlation of the 2
///    floating-point sparse matrices:
///    ```ignore
///        double cross_corr(const SparseMat& a, const SparseMat& b)
///        {
///            const SparseMat *_a = &a, *_b = &b;
///            // if b contains less elements than a,
///            // it is faster to iterate through b
///            if(_a->nzcount() > _b->nzcount())
///                std::swap(_a, _b);
///            SparseMatConstIterator_<float> it = _a->begin<float>(),
///                                            it_end = _a->end<float>();
///            double ccorr = 0;
///            for(; it != it_end; ++it)
///            {
///                // take the next element from the first matrix
///                float avalue = *it;
///                const Node* anode = it.node();
///                // and try to find an element with the same index in the second matrix.
///                // since the hash value depends only on the element index,
///                // reuse the hash value stored in the node
///                float bvalue = _b->value<float>(anode->idx,&anode->hashval);
///                ccorr += avalue*bvalue;
///            }
///            return ccorr;
///        }
///    ```
/// 
pub struct SparseMat {
	ptr: *mut c_void
}

opencv_type_boxed! { SparseMat }

impl Drop for SparseMat {
	fn drop(&mut self) {
		extern "C" { fn cv_SparseMat_delete(instance: *mut c_void); }
		unsafe { cv_SparseMat_delete(self.as_raw_mut_SparseMat()) };
	}
}

impl SparseMat {
	#[inline] pub fn as_raw_SparseMat(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SparseMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SparseMat {}

impl core::SparseMatTrait for SparseMat {
	#[inline] fn as_raw_SparseMat(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SparseMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SparseMat {
	/// Various SparseMat constructors.
	pub fn default() -> Result<core::SparseMat> {
		unsafe { sys::cv_SparseMat_SparseMat() }.into_result().map(|r| unsafe { core::SparseMat::opencv_from_extern(r) } )
	}
	
	/// Various SparseMat constructors.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * dims: Array dimensionality.
	/// * _sizes: Sparce matrix size on all dementions.
	/// * _type: Sparse matrix data type.
	pub fn new(dims: i32, _sizes: &i32, _type: i32) -> Result<core::SparseMat> {
		unsafe { sys::cv_SparseMat_SparseMat_int_const_intX_int(dims, _sizes, _type) }.into_result().map(|r| unsafe { core::SparseMat::opencv_from_extern(r) } )
	}
	
	/// Various SparseMat constructors.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * m: Source matrix for copy constructor. If m is dense matrix (ocvMat) then it will be converted
	///    to sparse representation.
	pub fn copy(m: &core::SparseMat) -> Result<core::SparseMat> {
		unsafe { sys::cv_SparseMat_SparseMat_const_SparseMatR(m.as_raw_SparseMat()) }.into_result().map(|r| unsafe { core::SparseMat::opencv_from_extern(r) } )
	}
	
	/// Various SparseMat constructors.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * m: Source matrix for copy constructor. If m is dense matrix (ocvMat) then it will be converted
	///    to sparse representation.
	pub fn from_mat(m: &core::Mat) -> Result<core::SparseMat> {
		unsafe { sys::cv_SparseMat_SparseMat_const_MatR(m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::SparseMat::opencv_from_extern(r) } )
	}
	
}

/// the sparse matrix header
pub trait SparseMat_HdrTrait {
	fn as_raw_SparseMat_Hdr(&self) -> *const c_void;
	fn as_raw_mut_SparseMat_Hdr(&mut self) -> *mut c_void;

	fn refcount(&self) -> i32 {
		unsafe { sys::cv_SparseMat_Hdr_getPropRefcount_const(self.as_raw_SparseMat_Hdr()) }.into_result().expect("Infallible function failed: refcount")
	}
	
	fn set_refcount(&mut self, val: i32) -> () {
		unsafe { sys::cv_SparseMat_Hdr_setPropRefcount_int(self.as_raw_mut_SparseMat_Hdr(), val) }.into_result().expect("Infallible function failed: set_refcount")
	}
	
	fn dims(&self) -> i32 {
		unsafe { sys::cv_SparseMat_Hdr_getPropDims_const(self.as_raw_SparseMat_Hdr()) }.into_result().expect("Infallible function failed: dims")
	}
	
	fn set_dims(&mut self, val: i32) -> () {
		unsafe { sys::cv_SparseMat_Hdr_setPropDims_int(self.as_raw_mut_SparseMat_Hdr(), val) }.into_result().expect("Infallible function failed: set_dims")
	}
	
	fn value_offset(&self) -> i32 {
		unsafe { sys::cv_SparseMat_Hdr_getPropValueOffset_const(self.as_raw_SparseMat_Hdr()) }.into_result().expect("Infallible function failed: value_offset")
	}
	
	fn set_value_offset(&mut self, val: i32) -> () {
		unsafe { sys::cv_SparseMat_Hdr_setPropValueOffset_int(self.as_raw_mut_SparseMat_Hdr(), val) }.into_result().expect("Infallible function failed: set_value_offset")
	}
	
	fn node_size(&self) -> size_t {
		unsafe { sys::cv_SparseMat_Hdr_getPropNodeSize_const(self.as_raw_SparseMat_Hdr()) }.into_result().expect("Infallible function failed: node_size")
	}
	
	fn set_node_size(&mut self, val: size_t) -> () {
		unsafe { sys::cv_SparseMat_Hdr_setPropNodeSize_size_t(self.as_raw_mut_SparseMat_Hdr(), val) }.into_result().expect("Infallible function failed: set_node_size")
	}
	
	fn node_count(&self) -> size_t {
		unsafe { sys::cv_SparseMat_Hdr_getPropNodeCount_const(self.as_raw_SparseMat_Hdr()) }.into_result().expect("Infallible function failed: node_count")
	}
	
	fn set_node_count(&mut self, val: size_t) -> () {
		unsafe { sys::cv_SparseMat_Hdr_setPropNodeCount_size_t(self.as_raw_mut_SparseMat_Hdr(), val) }.into_result().expect("Infallible function failed: set_node_count")
	}
	
	fn free_list(&self) -> size_t {
		unsafe { sys::cv_SparseMat_Hdr_getPropFreeList_const(self.as_raw_SparseMat_Hdr()) }.into_result().expect("Infallible function failed: free_list")
	}
	
	fn set_free_list(&mut self, val: size_t) -> () {
		unsafe { sys::cv_SparseMat_Hdr_setPropFreeList_size_t(self.as_raw_mut_SparseMat_Hdr(), val) }.into_result().expect("Infallible function failed: set_free_list")
	}
	
	fn pool(&mut self) -> core::Vector::<u8> {
		unsafe { sys::cv_SparseMat_Hdr_getPropPool(self.as_raw_mut_SparseMat_Hdr()) }.into_result().map(|r| unsafe { core::Vector::<u8>::opencv_from_extern(r) } ).expect("Infallible function failed: pool")
	}
	
	fn set_pool(&mut self, mut val: core::Vector::<u8>) -> () {
		unsafe { sys::cv_SparseMat_Hdr_setPropPool_vector_unsigned_char_(self.as_raw_mut_SparseMat_Hdr(), val.as_raw_mut_VectorOfu8()) }.into_result().expect("Infallible function failed: set_pool")
	}
	
	fn hashtab(&mut self) -> core::Vector::<size_t> {
		unsafe { sys::cv_SparseMat_Hdr_getPropHashtab(self.as_raw_mut_SparseMat_Hdr()) }.into_result().map(|r| unsafe { core::Vector::<size_t>::opencv_from_extern(r) } ).expect("Infallible function failed: hashtab")
	}
	
	fn set_hashtab(&mut self, mut val: core::Vector::<size_t>) -> () {
		unsafe { sys::cv_SparseMat_Hdr_setPropHashtab_vector_size_t_(self.as_raw_mut_SparseMat_Hdr(), val.as_raw_mut_VectorOfsize_t()) }.into_result().expect("Infallible function failed: set_hashtab")
	}
	
	fn size(&mut self) -> &mut [i32; 32] {
		unsafe { sys::cv_SparseMat_Hdr_getPropSize(self.as_raw_mut_SparseMat_Hdr()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: size")
	}
	
	fn clear(&mut self) -> Result<()> {
		unsafe { sys::cv_SparseMat_Hdr_clear(self.as_raw_mut_SparseMat_Hdr()) }.into_result()
	}
	
}

/// the sparse matrix header
pub struct SparseMat_Hdr {
	ptr: *mut c_void
}

opencv_type_boxed! { SparseMat_Hdr }

impl Drop for SparseMat_Hdr {
	fn drop(&mut self) {
		extern "C" { fn cv_SparseMat_Hdr_delete(instance: *mut c_void); }
		unsafe { cv_SparseMat_Hdr_delete(self.as_raw_mut_SparseMat_Hdr()) };
	}
}

impl SparseMat_Hdr {
	#[inline] pub fn as_raw_SparseMat_Hdr(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SparseMat_Hdr(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SparseMat_Hdr {}

impl core::SparseMat_HdrTrait for SparseMat_Hdr {
	#[inline] fn as_raw_SparseMat_Hdr(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SparseMat_Hdr(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SparseMat_Hdr {
	pub fn new(_sizes: &[i32], _type: i32) -> Result<core::SparseMat_Hdr> {
		unsafe { sys::cv_SparseMat_Hdr_Hdr_int_const_intX_int(_sizes.len() as _, _sizes.as_ptr(), _type) }.into_result().map(|r| unsafe { core::SparseMat_Hdr::opencv_from_extern(r) } )
	}
	
}

/// sparse matrix node - element of a hash table
pub trait SparseMat_NodeTrait {
	fn as_raw_SparseMat_Node(&self) -> *const c_void;
	fn as_raw_mut_SparseMat_Node(&mut self) -> *mut c_void;

	/// hash value
	fn hashval(&self) -> size_t {
		unsafe { sys::cv_SparseMat_Node_getPropHashval_const(self.as_raw_SparseMat_Node()) }.into_result().expect("Infallible function failed: hashval")
	}
	
	/// hash value
	fn set_hashval(&mut self, val: size_t) -> () {
		unsafe { sys::cv_SparseMat_Node_setPropHashval_size_t(self.as_raw_mut_SparseMat_Node(), val) }.into_result().expect("Infallible function failed: set_hashval")
	}
	
	/// index of the next node in the same hash table entry
	fn next(&self) -> size_t {
		unsafe { sys::cv_SparseMat_Node_getPropNext_const(self.as_raw_SparseMat_Node()) }.into_result().expect("Infallible function failed: next")
	}
	
	/// index of the next node in the same hash table entry
	fn set_next(&mut self, val: size_t) -> () {
		unsafe { sys::cv_SparseMat_Node_setPropNext_size_t(self.as_raw_mut_SparseMat_Node(), val) }.into_result().expect("Infallible function failed: set_next")
	}
	
	/// index of the matrix element
	fn idx(&mut self) -> &mut [i32; 32] {
		unsafe { sys::cv_SparseMat_Node_getPropIdx(self.as_raw_mut_SparseMat_Node()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: idx")
	}
	
}

/// sparse matrix node - element of a hash table
pub struct SparseMat_Node {
	ptr: *mut c_void
}

opencv_type_boxed! { SparseMat_Node }

impl Drop for SparseMat_Node {
	fn drop(&mut self) {
		extern "C" { fn cv_SparseMat_Node_delete(instance: *mut c_void); }
		unsafe { cv_SparseMat_Node_delete(self.as_raw_mut_SparseMat_Node()) };
	}
}

impl SparseMat_Node {
	#[inline] pub fn as_raw_SparseMat_Node(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SparseMat_Node(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SparseMat_Node {}

impl core::SparseMat_NodeTrait for SparseMat_Node {
	#[inline] fn as_raw_SparseMat_Node(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SparseMat_Node(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SparseMat_Node {
}

/// Read-Only Sparse Matrix Iterator.
/// 
/// Here is how to use the iterator to compute the sum of floating-point sparse matrix elements:
/// 
/// \code
/// SparseMatConstIterator it = m.begin(), it_end = m.end();
/// double s = 0;
/// CV_Assert( m.type() == CV_32F );
/// for( ; it != it_end; ++it )
///    s += it.value<float>();
/// \endcode
pub trait SparseMatConstIteratorTrait {
	fn as_raw_SparseMatConstIterator(&self) -> *const c_void;
	fn as_raw_mut_SparseMatConstIterator(&mut self) -> *mut c_void;

	fn m(&self) -> core::SparseMat {
		unsafe { sys::cv_SparseMatConstIterator_getPropM_const(self.as_raw_SparseMatConstIterator()) }.into_result().map(|r| unsafe { core::SparseMat::opencv_from_extern(r) } ).expect("Infallible function failed: m")
	}
	
	fn hashidx(&self) -> size_t {
		unsafe { sys::cv_SparseMatConstIterator_getPropHashidx_const(self.as_raw_SparseMatConstIterator()) }.into_result().expect("Infallible function failed: hashidx")
	}
	
	fn set_hashidx(&mut self, val: size_t) -> () {
		unsafe { sys::cv_SparseMatConstIterator_setPropHashidx_size_t(self.as_raw_mut_SparseMatConstIterator(), val) }.into_result().expect("Infallible function failed: set_hashidx")
	}
	
	fn ptr(&mut self) -> &mut u8 {
		unsafe { sys::cv_SparseMatConstIterator_getPropPtr(self.as_raw_mut_SparseMatConstIterator()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: ptr")
	}
	
	fn set_ptr(&mut self, val: &mut u8) -> () {
		unsafe { sys::cv_SparseMatConstIterator_setPropPtr_unsigned_charX(self.as_raw_mut_SparseMatConstIterator(), val) }.into_result().expect("Infallible function failed: set_ptr")
	}
	
	/// returns the current node of the sparse matrix. it.node->idx is the current element index
	fn node(&self) -> Result<core::SparseMat_Node> {
		unsafe { sys::cv_SparseMatConstIterator_node_const(self.as_raw_SparseMatConstIterator()) }.into_result().map(|r| unsafe { core::SparseMat_Node::opencv_from_extern(r) } )
	}
	
	/// moves iterator to the element after the last element
	fn seek_end(&mut self) -> Result<()> {
		unsafe { sys::cv_SparseMatConstIterator_seekEnd(self.as_raw_mut_SparseMatConstIterator()) }.into_result()
	}
	
}

/// Read-Only Sparse Matrix Iterator.
/// 
/// Here is how to use the iterator to compute the sum of floating-point sparse matrix elements:
/// 
/// \code
/// SparseMatConstIterator it = m.begin(), it_end = m.end();
/// double s = 0;
/// CV_Assert( m.type() == CV_32F );
/// for( ; it != it_end; ++it )
///    s += it.value<float>();
/// \endcode
pub struct SparseMatConstIterator {
	ptr: *mut c_void
}

opencv_type_boxed! { SparseMatConstIterator }

impl Drop for SparseMatConstIterator {
	fn drop(&mut self) {
		extern "C" { fn cv_SparseMatConstIterator_delete(instance: *mut c_void); }
		unsafe { cv_SparseMatConstIterator_delete(self.as_raw_mut_SparseMatConstIterator()) };
	}
}

impl SparseMatConstIterator {
	#[inline] pub fn as_raw_SparseMatConstIterator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SparseMatConstIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SparseMatConstIterator {}

impl core::SparseMatConstIteratorTrait for SparseMatConstIterator {
	#[inline] fn as_raw_SparseMatConstIterator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SparseMatConstIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SparseMatConstIterator {
	/// the default constructor
	pub fn default() -> Result<core::SparseMatConstIterator> {
		unsafe { sys::cv_SparseMatConstIterator_SparseMatConstIterator() }.into_result().map(|r| unsafe { core::SparseMatConstIterator::opencv_from_extern(r) } )
	}
	
	/// the full constructor setting the iterator to the first sparse matrix element
	pub fn new(_m: &core::SparseMat) -> Result<core::SparseMatConstIterator> {
		unsafe { sys::cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatX(_m.as_raw_SparseMat()) }.into_result().map(|r| unsafe { core::SparseMatConstIterator::opencv_from_extern(r) } )
	}
	
	/// the copy constructor
	pub fn copy(it: &core::SparseMatConstIterator) -> Result<core::SparseMatConstIterator> {
		unsafe { sys::cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatConstIteratorR(it.as_raw_SparseMatConstIterator()) }.into_result().map(|r| unsafe { core::SparseMatConstIterator::opencv_from_extern(r) } )
	}
	
}

///  Read-write Sparse Matrix Iterator
/// 
/// The class is similar to cv::SparseMatConstIterator,
/// but can be used for in-place modification of the matrix elements.
pub trait SparseMatIteratorTrait: core::SparseMatConstIteratorTrait {
	fn as_raw_SparseMatIterator(&self) -> *const c_void;
	fn as_raw_mut_SparseMatIterator(&mut self) -> *mut c_void;

	/// returns pointer to the current sparse matrix node. it.node->idx is the index of the current element (do not modify it!)
	fn node(&self) -> Result<core::SparseMat_Node> {
		unsafe { sys::cv_SparseMatIterator_node_const(self.as_raw_SparseMatIterator()) }.into_result().map(|r| unsafe { core::SparseMat_Node::opencv_from_extern(r) } )
	}
	
}

///  Read-write Sparse Matrix Iterator
/// 
/// The class is similar to cv::SparseMatConstIterator,
/// but can be used for in-place modification of the matrix elements.
pub struct SparseMatIterator {
	ptr: *mut c_void
}

opencv_type_boxed! { SparseMatIterator }

impl Drop for SparseMatIterator {
	fn drop(&mut self) {
		extern "C" { fn cv_SparseMatIterator_delete(instance: *mut c_void); }
		unsafe { cv_SparseMatIterator_delete(self.as_raw_mut_SparseMatIterator()) };
	}
}

impl SparseMatIterator {
	#[inline] pub fn as_raw_SparseMatIterator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SparseMatIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SparseMatIterator {}

impl core::SparseMatConstIteratorTrait for SparseMatIterator {
	#[inline] fn as_raw_SparseMatConstIterator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SparseMatConstIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::SparseMatIteratorTrait for SparseMatIterator {
	#[inline] fn as_raw_SparseMatIterator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SparseMatIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SparseMatIterator {
	/// the default constructor
	pub fn default() -> Result<core::SparseMatIterator> {
		unsafe { sys::cv_SparseMatIterator_SparseMatIterator() }.into_result().map(|r| unsafe { core::SparseMatIterator::opencv_from_extern(r) } )
	}
	
	/// the full constructor setting the iterator to the first sparse matrix element
	pub fn new(_m: &mut core::SparseMat) -> Result<core::SparseMatIterator> {
		unsafe { sys::cv_SparseMatIterator_SparseMatIterator_SparseMatX(_m.as_raw_mut_SparseMat()) }.into_result().map(|r| unsafe { core::SparseMatIterator::opencv_from_extern(r) } )
	}
	
	#[cfg(not(target_os = "windows"))]
	/// the full constructor setting the iterator to the specified sparse matrix element
	pub fn new_1(_m: &mut core::SparseMat, idx: &i32) -> Result<core::SparseMatIterator> {
		unsafe { sys::cv_SparseMatIterator_SparseMatIterator_SparseMatX_const_intX(_m.as_raw_mut_SparseMat(), idx) }.into_result().map(|r| unsafe { core::SparseMatIterator::opencv_from_extern(r) } )
	}
	
	/// the copy constructor
	pub fn copy(it: &core::SparseMatIterator) -> Result<core::SparseMatIterator> {
		unsafe { sys::cv_SparseMatIterator_SparseMatIterator_const_SparseMatIteratorR(it.as_raw_SparseMatIterator()) }.into_result().map(|r| unsafe { core::SparseMatIterator::opencv_from_extern(r) } )
	}
	
}

/// TLS container base implementation
/// 
/// Don't use directly.
/// ## See also
/// TLSData, TLSDataAccumulator templates
pub trait TLSDataContainer {
	fn as_raw_TLSDataContainer(&self) -> *const c_void;
	fn as_raw_mut_TLSDataContainer(&mut self) -> *mut c_void;

	fn cleanup(&mut self) -> Result<()> {
		unsafe { sys::cv_TLSDataContainer_cleanup(self.as_raw_mut_TLSDataContainer()) }.into_result()
	}
	
}

/// The class defining termination criteria for iterative algorithms.
/// 
/// You can initialize it by default constructor and then override any parameters, or the structure may
/// be fully initialized using the advanced variant of the constructor.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TermCriteria {
	/// the type of termination criteria: COUNT, EPS or COUNT + EPS
	pub typ: i32,
	/// the maximum number of iterations/elements
	pub max_count: i32,
	/// the desired accuracy
	pub epsilon: f64,
}

opencv_type_simple! { core::TermCriteria }

impl TermCriteria {
	/// default constructor
	pub fn default() -> Result<core::TermCriteria> {
		unsafe { sys::cv_TermCriteria_TermCriteria() }.into_result()
	}
	
	/// ## Parameters
	/// * type: The type of termination criteria, one of TermCriteria::Type
	/// * maxCount: The maximum number of iterations or elements to compute.
	/// * epsilon: The desired accuracy or change in parameters at which the iterative algorithm stops.
	pub fn new(typ: i32, max_count: i32, epsilon: f64) -> Result<core::TermCriteria> {
		unsafe { sys::cv_TermCriteria_TermCriteria_int_int_double(typ, max_count, epsilon) }.into_result()
	}
	
	pub fn is_valid(self) -> Result<bool> {
		unsafe { sys::cv_TermCriteria_isValid_const(self.opencv_to_extern()) }.into_result()
	}
	
}

/// a Class to measure passing time.
/// 
/// The class computes passing time by counting the number of ticks per second. That is, the following code computes the
/// execution time in seconds:
/// ```ignore
/// TickMeter tm;
/// tm.start();
/// // do something ...
/// tm.stop();
/// std::cout << tm.getTimeSec();
/// ```
/// 
/// 
/// It is also possible to compute the average time over multiple runs:
/// ```ignore
/// TickMeter tm;
/// for (int i = 0; i < 100; i++)
/// {
///    tm.start();
///    // do something ...
///    tm.stop();
/// }
/// double average_time = tm.getTimeSec() / tm.getCounter();
/// std::cout << "Average time in second per iteration is: " << average_time << std::endl;
/// ```
/// ## See also
/// getTickCount, getTickFrequency
pub trait TickMeterTrait {
	fn as_raw_TickMeter(&self) -> *const c_void;
	fn as_raw_mut_TickMeter(&mut self) -> *mut c_void;

	/// starts counting ticks.
	fn start(&mut self) -> Result<()> {
		unsafe { sys::cv_TickMeter_start(self.as_raw_mut_TickMeter()) }.into_result()
	}
	
	/// stops counting ticks.
	fn stop(&mut self) -> Result<()> {
		unsafe { sys::cv_TickMeter_stop(self.as_raw_mut_TickMeter()) }.into_result()
	}
	
	/// returns counted ticks.
	fn get_time_ticks(&self) -> Result<i64> {
		unsafe { sys::cv_TickMeter_getTimeTicks_const(self.as_raw_TickMeter()) }.into_result()
	}
	
	/// returns passed time in microseconds.
	fn get_time_micro(&self) -> Result<f64> {
		unsafe { sys::cv_TickMeter_getTimeMicro_const(self.as_raw_TickMeter()) }.into_result()
	}
	
	/// returns passed time in milliseconds.
	fn get_time_milli(&self) -> Result<f64> {
		unsafe { sys::cv_TickMeter_getTimeMilli_const(self.as_raw_TickMeter()) }.into_result()
	}
	
	/// returns passed time in seconds.
	fn get_time_sec(&self) -> Result<f64> {
		unsafe { sys::cv_TickMeter_getTimeSec_const(self.as_raw_TickMeter()) }.into_result()
	}
	
	/// returns internal counter value.
	fn get_counter(&self) -> Result<i64> {
		unsafe { sys::cv_TickMeter_getCounter_const(self.as_raw_TickMeter()) }.into_result()
	}
	
	/// resets internal values.
	fn reset(&mut self) -> Result<()> {
		unsafe { sys::cv_TickMeter_reset(self.as_raw_mut_TickMeter()) }.into_result()
	}
	
}

/// a Class to measure passing time.
/// 
/// The class computes passing time by counting the number of ticks per second. That is, the following code computes the
/// execution time in seconds:
/// ```ignore
/// TickMeter tm;
/// tm.start();
/// // do something ...
/// tm.stop();
/// std::cout << tm.getTimeSec();
/// ```
/// 
/// 
/// It is also possible to compute the average time over multiple runs:
/// ```ignore
/// TickMeter tm;
/// for (int i = 0; i < 100; i++)
/// {
///    tm.start();
///    // do something ...
///    tm.stop();
/// }
/// double average_time = tm.getTimeSec() / tm.getCounter();
/// std::cout << "Average time in second per iteration is: " << average_time << std::endl;
/// ```
/// ## See also
/// getTickCount, getTickFrequency
pub struct TickMeter {
	ptr: *mut c_void
}

opencv_type_boxed! { TickMeter }

impl Drop for TickMeter {
	fn drop(&mut self) {
		extern "C" { fn cv_TickMeter_delete(instance: *mut c_void); }
		unsafe { cv_TickMeter_delete(self.as_raw_mut_TickMeter()) };
	}
}

impl TickMeter {
	#[inline] pub fn as_raw_TickMeter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_TickMeter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TickMeter {}

impl core::TickMeterTrait for TickMeter {
	#[inline] fn as_raw_TickMeter(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_TickMeter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TickMeter {
	/// the default constructor
	pub fn default() -> Result<core::TickMeter> {
		unsafe { sys::cv_TickMeter_TickMeter() }.into_result().map(|r| unsafe { core::TickMeter::opencv_from_extern(r) } )
	}
	
}

/// @todo document
pub trait UMatTrait {
	fn as_raw_UMat(&self) -> *const c_void;
	fn as_raw_mut_UMat(&mut self) -> *mut c_void;

	/// ! includes several bit-fields:
	/// - the magic signature
	/// - continuity flag
	/// - depth
	/// - number of channels
	fn flags(&self) -> i32 {
		unsafe { sys::cv_UMat_getPropFlags_const(self.as_raw_UMat()) }.into_result().expect("Infallible function failed: flags")
	}
	
	/// ! includes several bit-fields:
	/// - the magic signature
	/// - continuity flag
	/// - depth
	/// - number of channels
	fn set_flags(&mut self, val: i32) -> () {
		unsafe { sys::cv_UMat_setPropFlags_int(self.as_raw_mut_UMat(), val) }.into_result().expect("Infallible function failed: set_flags")
	}
	
	/// the matrix dimensionality, >= 2
	fn dims(&self) -> i32 {
		unsafe { sys::cv_UMat_getPropDims_const(self.as_raw_UMat()) }.into_result().expect("Infallible function failed: dims")
	}
	
	/// the matrix dimensionality, >= 2
	fn set_dims(&mut self, val: i32) -> () {
		unsafe { sys::cv_UMat_setPropDims_int(self.as_raw_mut_UMat(), val) }.into_result().expect("Infallible function failed: set_dims")
	}
	
	/// the number of rows and columns or (-1, -1) when the matrix has more than 2 dimensions
	fn rows(&self) -> i32 {
		unsafe { sys::cv_UMat_getPropRows_const(self.as_raw_UMat()) }.into_result().expect("Infallible function failed: rows")
	}
	
	/// the number of rows and columns or (-1, -1) when the matrix has more than 2 dimensions
	fn set_rows(&mut self, val: i32) -> () {
		unsafe { sys::cv_UMat_setPropRows_int(self.as_raw_mut_UMat(), val) }.into_result().expect("Infallible function failed: set_rows")
	}
	
	/// the number of rows and columns or (-1, -1) when the matrix has more than 2 dimensions
	fn cols(&self) -> i32 {
		unsafe { sys::cv_UMat_getPropCols_const(self.as_raw_UMat()) }.into_result().expect("Infallible function failed: cols")
	}
	
	/// the number of rows and columns or (-1, -1) when the matrix has more than 2 dimensions
	fn set_cols(&mut self, val: i32) -> () {
		unsafe { sys::cv_UMat_setPropCols_int(self.as_raw_mut_UMat(), val) }.into_result().expect("Infallible function failed: set_cols")
	}
	
	fn usage_flags(&self) -> core::UMatUsageFlags {
		unsafe { sys::cv_UMat_getPropUsageFlags_const(self.as_raw_UMat()) }.into_result().expect("Infallible function failed: usage_flags")
	}
	
	fn set_usage_flags(&mut self, val: core::UMatUsageFlags) -> () {
		unsafe { sys::cv_UMat_setPropUsageFlags_UMatUsageFlags(self.as_raw_mut_UMat(), val) }.into_result().expect("Infallible function failed: set_usage_flags")
	}
	
	fn u(&mut self) -> core::UMatData {
		unsafe { sys::cv_UMat_getPropU(self.as_raw_mut_UMat()) }.into_result().map(|r| unsafe { core::UMatData::opencv_from_extern(r) } ).expect("Infallible function failed: u")
	}
	
	fn set_u(&mut self, val: &mut core::UMatData) -> () {
		unsafe { sys::cv_UMat_setPropU_UMatDataX(self.as_raw_mut_UMat(), val.as_raw_mut_UMatData()) }.into_result().expect("Infallible function failed: set_u")
	}
	
	fn offset(&self) -> size_t {
		unsafe { sys::cv_UMat_getPropOffset_const(self.as_raw_UMat()) }.into_result().expect("Infallible function failed: offset")
	}
	
	fn set_offset(&mut self, val: size_t) -> () {
		unsafe { sys::cv_UMat_setPropOffset_size_t(self.as_raw_mut_UMat(), val) }.into_result().expect("Infallible function failed: set_offset")
	}
	
	fn mat_size(&self) -> core::MatSize {
		unsafe { sys::cv_UMat_getPropSize_const(self.as_raw_UMat()) }.into_result().map(|r| unsafe { core::MatSize::opencv_from_extern(r) } ).expect("Infallible function failed: mat_size")
	}
	
	fn mat_step(&self) -> core::MatStep {
		unsafe { sys::cv_UMat_getPropStep_const(self.as_raw_UMat()) }.into_result().map(|r| unsafe { core::MatStep::opencv_from_extern(r) } ).expect("Infallible function failed: mat_step")
	}
	
	fn get_mat(&self, flags: core::AccessFlag) -> Result<core::Mat> {
		unsafe { sys::cv_UMat_getMat_const_AccessFlag(self.as_raw_UMat(), flags) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// returns a new matrix header for the specified row
	fn row(&self, y: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_row_const_int(self.as_raw_UMat(), y) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// returns a new matrix header for the specified column
	fn col(&self, x: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_col_const_int(self.as_raw_UMat(), x) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// ... for the specified row span
	fn row_bounds(&self, startrow: i32, endrow: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_rowRange_const_int_int(self.as_raw_UMat(), startrow, endrow) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	fn row_range(&self, r: &core::Range) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_rowRange_const_const_RangeR(self.as_raw_UMat(), r.as_raw_Range()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// ... for the specified column span
	fn col_bounds(&self, startcol: i32, endcol: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_colRange_const_int_int(self.as_raw_UMat(), startcol, endcol) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	fn col_range(&self, r: &core::Range) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_colRange_const_const_RangeR(self.as_raw_UMat(), r.as_raw_Range()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// ... for the specified diagonal
	/// (d=0 - the main diagonal,
	///  >0 - a diagonal from the upper half,
	///  <0 - a diagonal from the lower half)
	/// 
	/// ## C++ default parameters
	/// * d: 0
	fn diag(&self, d: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_diag_const_int(self.as_raw_UMat(), d) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// returns deep copy of the matrix, i.e. the data is copied
	fn clone(&self) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_clone_const(self.as_raw_UMat()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// copies the matrix content to "m".
	fn copy_to(&self, m: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(m);
		unsafe { sys::cv_UMat_copyTo_const_const__OutputArrayR(self.as_raw_UMat(), m.as_raw__OutputArray()) }.into_result()
	}
	
	/// copies those matrix elements to "m" that are marked with non-zero mask elements.
	fn copy_to_masked(&self, m: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		output_array_arg!(m);
		input_array_arg!(mask);
		unsafe { sys::cv_UMat_copyTo_const_const__OutputArrayR_const__InputArrayR(self.as_raw_UMat(), m.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
	}
	
	/// converts matrix to another datatype with optional scaling. See cvConvertScale.
	/// 
	/// ## C++ default parameters
	/// * alpha: 1
	/// * beta: 0
	fn convert_to(&self, m: &mut dyn core::ToOutputArray, rtype: i32, alpha: f64, beta: f64) -> Result<()> {
		output_array_arg!(m);
		unsafe { sys::cv_UMat_convertTo_const_const__OutputArrayR_int_double_double(self.as_raw_UMat(), m.as_raw__OutputArray(), rtype, alpha, beta) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * typ: -1
	fn assign_to(&self, m: &mut core::UMat, typ: i32) -> Result<()> {
		unsafe { sys::cv_UMat_assignTo_const_UMatR_int(self.as_raw_UMat(), m.as_raw_mut_UMat(), typ) }.into_result()
	}
	
	/// sets some of the matrix elements to s, according to the mask
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	fn set_to(&mut self, value: &dyn core::ToInputArray, mask: &dyn core::ToInputArray) -> Result<core::UMat> {
		input_array_arg!(value);
		input_array_arg!(mask);
		unsafe { sys::cv_UMat_setTo_const__InputArrayR_const__InputArrayR(self.as_raw_mut_UMat(), value.as_raw__InputArray(), mask.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// creates alternative matrix header for the same data, with different
	/// 
	/// ## C++ default parameters
	/// * rows: 0
	fn reshape(&self, cn: i32, rows: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_reshape_const_int_int(self.as_raw_UMat(), cn, rows) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	fn reshape_1(&self, cn: i32, newndims: i32, newsz: &i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_reshape_const_int_int_const_intX(self.as_raw_UMat(), cn, newndims, newsz) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// matrix transposition by means of matrix expressions
	fn t(&self) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_t_const(self.as_raw_UMat()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// matrix inversion by means of matrix expressions
	/// 
	/// ## C++ default parameters
	/// * method: DECOMP_LU
	fn inv(&self, method: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_inv_const_int(self.as_raw_UMat(), method) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// per-element matrix multiplication by means of matrix expressions
	/// 
	/// ## C++ default parameters
	/// * scale: 1
	fn mul(&self, m: &dyn core::ToInputArray, scale: f64) -> Result<core::UMat> {
		input_array_arg!(m);
		unsafe { sys::cv_UMat_mul_const_const__InputArrayR_double(self.as_raw_UMat(), m.as_raw__InputArray(), scale) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// computes dot-product
	fn dot(&self, m: &dyn core::ToInputArray) -> Result<f64> {
		input_array_arg!(m);
		unsafe { sys::cv_UMat_dot_const_const__InputArrayR(self.as_raw_UMat(), m.as_raw__InputArray()) }.into_result()
	}
	
	/// allocates new matrix data unless the matrix already has specified size and type.
	/// 
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	unsafe fn create_rows_cols(&mut self, rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<()> {
		{ sys::cv_UMat_create_int_int_int_UMatUsageFlags(self.as_raw_mut_UMat(), rows, cols, typ, usage_flags) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	unsafe fn create_size(&mut self, size: core::Size, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<()> {
		{ sys::cv_UMat_create_Size_int_UMatUsageFlags(self.as_raw_mut_UMat(), size.opencv_to_extern(), typ, usage_flags) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	unsafe fn create_nd(&mut self, sizes: &[i32], typ: i32, usage_flags: core::UMatUsageFlags) -> Result<()> {
		{ sys::cv_UMat_create_int_const_intX_int_UMatUsageFlags(self.as_raw_mut_UMat(), sizes.len() as _, sizes.as_ptr(), typ, usage_flags) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	unsafe fn create_nd_vec(&mut self, sizes: &core::Vector::<i32>, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<()> {
		{ sys::cv_UMat_create_const_vector_int_R_int_UMatUsageFlags(self.as_raw_mut_UMat(), sizes.as_raw_VectorOfi32(), typ, usage_flags) }.into_result()
	}
	
	/// increases the reference counter; use with care to avoid memleaks
	fn addref(&mut self) -> Result<()> {
		unsafe { sys::cv_UMat_addref(self.as_raw_mut_UMat()) }.into_result()
	}
	
	/// decreases reference counter;
	fn release(&mut self) -> Result<()> {
		unsafe { sys::cv_UMat_release(self.as_raw_mut_UMat()) }.into_result()
	}
	
	/// deallocates the matrix data
	fn deallocate(&mut self) -> Result<()> {
		unsafe { sys::cv_UMat_deallocate(self.as_raw_mut_UMat()) }.into_result()
	}
	
	/// locates matrix header within a parent matrix. See below
	fn locate_roi(&self, whole_size: &mut core::Size, ofs: &mut core::Point) -> Result<()> {
		unsafe { sys::cv_UMat_locateROI_const_SizeR_PointR(self.as_raw_UMat(), whole_size, ofs) }.into_result()
	}
	
	/// moves/resizes the current matrix ROI inside the parent matrix.
	fn adjust_roi(&mut self, dtop: i32, dbottom: i32, dleft: i32, dright: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_adjustROI_int_int_int_int(self.as_raw_mut_UMat(), dtop, dbottom, dleft, dright) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// returns true iff the matrix data is continuous
	fn is_continuous(&self) -> Result<bool> {
		unsafe { sys::cv_UMat_isContinuous_const(self.as_raw_UMat()) }.into_result()
	}
	
	/// returns true if the matrix is a submatrix of another matrix
	fn is_submatrix(&self) -> Result<bool> {
		unsafe { sys::cv_UMat_isSubmatrix_const(self.as_raw_UMat()) }.into_result()
	}
	
	/// returns element size in bytes,
	fn elem_size(&self) -> Result<size_t> {
		unsafe { sys::cv_UMat_elemSize_const(self.as_raw_UMat()) }.into_result()
	}
	
	/// returns the size of element channel in bytes.
	fn elem_size1(&self) -> Result<size_t> {
		unsafe { sys::cv_UMat_elemSize1_const(self.as_raw_UMat()) }.into_result()
	}
	
	/// returns element type, similar to CV_MAT_TYPE(cvmat->type)
	fn typ(&self) -> Result<i32> {
		unsafe { sys::cv_UMat_type_const(self.as_raw_UMat()) }.into_result()
	}
	
	/// returns element type, similar to CV_MAT_DEPTH(cvmat->type)
	fn depth(&self) -> Result<i32> {
		unsafe { sys::cv_UMat_depth_const(self.as_raw_UMat()) }.into_result()
	}
	
	/// returns element type, similar to CV_MAT_CN(cvmat->type)
	fn channels(&self) -> Result<i32> {
		unsafe { sys::cv_UMat_channels_const(self.as_raw_UMat()) }.into_result()
	}
	
	/// returns step/elemSize1()
	/// 
	/// ## C++ default parameters
	/// * i: 0
	fn step1(&self, i: i32) -> Result<size_t> {
		unsafe { sys::cv_UMat_step1_const_int(self.as_raw_UMat(), i) }.into_result()
	}
	
	/// returns true if matrix data is NULL
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_UMat_empty_const(self.as_raw_UMat()) }.into_result()
	}
	
	/// returns the total number of matrix elements
	fn total(&self) -> Result<size_t> {
		unsafe { sys::cv_UMat_total_const(self.as_raw_UMat()) }.into_result()
	}
	
	/// returns N if the matrix is 1-channel (N x ptdim) or ptdim-channel (1 x N) or (N x 1); negative number otherwise
	/// 
	/// ## C++ default parameters
	/// * depth: -1
	/// * require_continuous: true
	fn check_vector(&self, elem_channels: i32, depth: i32, require_continuous: bool) -> Result<i32> {
		unsafe { sys::cv_UMat_checkVector_const_int_int_bool(self.as_raw_UMat(), elem_channels, depth, require_continuous) }.into_result()
	}
	
	/// ! Returns the OpenCL buffer handle on which UMat operates on.
	/// The UMat instance should be kept alive during the use of the handle to prevent the buffer to be
	/// returned to the OpenCV buffer pool.
	fn handle(&self, access_flags: core::AccessFlag) -> Result<*mut c_void> {
		unsafe { sys::cv_UMat_handle_const_AccessFlag(self.as_raw_UMat(), access_flags) }.into_result()
	}
	
	fn ndoffset(&self, ofs: &mut size_t) -> Result<()> {
		unsafe { sys::cv_UMat_ndoffset_const_size_tX(self.as_raw_UMat(), ofs) }.into_result()
	}
	
	/// internal use method: updates the continuity flag
	fn update_continuity_flag(&mut self) -> Result<()> {
		unsafe { sys::cv_UMat_updateContinuityFlag(self.as_raw_mut_UMat()) }.into_result()
	}
	
}

/// @todo document
pub struct UMat {
	ptr: *mut c_void
}

opencv_type_boxed! { UMat }

impl Drop for UMat {
	fn drop(&mut self) {
		extern "C" { fn cv_UMat_delete(instance: *mut c_void); }
		unsafe { cv_UMat_delete(self.as_raw_mut_UMat()) };
	}
}

impl UMat {
	#[inline] pub fn as_raw_UMat(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_UMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for UMat {}

impl core::UMatTrait for UMat {
	#[inline] fn as_raw_UMat(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_UMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl UMat {
	/// default constructor
	/// 
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	pub fn new(usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_UMat_UMatUsageFlags(usage_flags) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// constructs 2D matrix of the specified size and type
	/// 
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	pub unsafe fn new_rows_cols(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		{ sys::cv_UMat_UMat_int_int_int_UMatUsageFlags(rows, cols, typ, usage_flags) }.into_result().map(|r| { core::UMat::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	pub unsafe fn new_size(size: core::Size, typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		{ sys::cv_UMat_UMat_Size_int_UMatUsageFlags(size.opencv_to_extern(), typ, usage_flags) }.into_result().map(|r| { core::UMat::opencv_from_extern(r) } )
	}
	
	/// constructs 2D matrix and fills it with the specified value _s.
	/// 
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	pub fn new_rows_cols_with_default(rows: i32, cols: i32, typ: i32, s: core::Scalar, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_UMat_int_int_int_const_ScalarR_UMatUsageFlags(rows, cols, typ, &s, usage_flags) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	pub fn new_size_with_default(size: core::Size, typ: i32, s: core::Scalar, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_UMat_Size_int_const_ScalarR_UMatUsageFlags(size.opencv_to_extern(), typ, &s, usage_flags) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// constructs n-dimensional matrix
	/// 
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	pub unsafe fn new_nd(sizes: &[i32], typ: i32, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		{ sys::cv_UMat_UMat_int_const_intX_int_UMatUsageFlags(sizes.len() as _, sizes.as_ptr(), typ, usage_flags) }.into_result().map(|r| { core::UMat::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * usage_flags: USAGE_DEFAULT
	pub fn new_nd_with_default(sizes: &[i32], typ: i32, s: core::Scalar, usage_flags: core::UMatUsageFlags) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_UMat_int_const_intX_int_const_ScalarR_UMatUsageFlags(sizes.len() as _, sizes.as_ptr(), typ, &s, usage_flags) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// copy constructor
	pub fn copy(m: &core::UMat) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_UMat_const_UMatR(m.as_raw_UMat()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// creates a matrix header for a part of the bigger matrix
	/// 
	/// ## C++ default parameters
	/// * col_range: Range::all()
	pub fn rowscols(m: &core::UMat, row_range: &core::Range, col_range: &core::Range) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_UMat_const_UMatR_const_RangeR_const_RangeR(m.as_raw_UMat(), row_range.as_raw_Range(), col_range.as_raw_Range()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	pub fn roi(m: &core::UMat, roi: core::Rect) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_UMat_const_UMatR_const_RectR(m.as_raw_UMat(), &roi) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	pub fn ranges(m: &core::UMat, ranges: &core::Vector::<core::Range>) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_UMat_const_UMatR_const_vector_Range_R(m.as_raw_UMat(), ranges.as_raw_VectorOfRange()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// constructs a square diagonal matrix which main diagonal is vector "d"
	pub fn diag(d: &core::UMat) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_diag_const_UMatR(d.as_raw_UMat()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	/// Matlab-style matrix initialization
	pub fn zeros(rows: i32, cols: i32, typ: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_zeros_int_int_int(rows, cols, typ) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	pub fn zeros_1(size: core::Size, typ: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_zeros_Size_int(size.opencv_to_extern(), typ) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	pub fn zeros_2(ndims: i32, sz: &i32, typ: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_zeros_int_const_intX_int(ndims, sz, typ) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	pub fn ones(rows: i32, cols: i32, typ: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_ones_int_int_int(rows, cols, typ) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	pub fn ones_1(size: core::Size, typ: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_ones_Size_int(size.opencv_to_extern(), typ) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	pub fn ones_2(ndims: i32, sz: &i32, typ: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_ones_int_const_intX_int(ndims, sz, typ) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	pub fn eye(rows: i32, cols: i32, typ: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_eye_int_int_int(rows, cols, typ) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	pub fn eye_1(size: core::Size, typ: i32) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_eye_Size_int(size.opencv_to_extern(), typ) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	pub fn copy_mut(m: &mut core::UMat) -> Result<core::UMat> {
		unsafe { sys::cv_UMat_UMat_UMatR(m.as_raw_mut_UMat()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
}

pub trait UMatDataTrait {
	fn as_raw_UMatData(&self) -> *const c_void;
	fn as_raw_mut_UMatData(&mut self) -> *mut c_void;

	fn urefcount(&self) -> i32 {
		unsafe { sys::cv_UMatData_getPropUrefcount_const(self.as_raw_UMatData()) }.into_result().expect("Infallible function failed: urefcount")
	}
	
	fn set_urefcount(&mut self, val: i32) -> () {
		unsafe { sys::cv_UMatData_setPropUrefcount_int(self.as_raw_mut_UMatData(), val) }.into_result().expect("Infallible function failed: set_urefcount")
	}
	
	fn refcount(&self) -> i32 {
		unsafe { sys::cv_UMatData_getPropRefcount_const(self.as_raw_UMatData()) }.into_result().expect("Infallible function failed: refcount")
	}
	
	fn set_refcount(&mut self, val: i32) -> () {
		unsafe { sys::cv_UMatData_setPropRefcount_int(self.as_raw_mut_UMatData(), val) }.into_result().expect("Infallible function failed: set_refcount")
	}
	
	fn data(&mut self) -> &mut u8 {
		unsafe { sys::cv_UMatData_getPropData(self.as_raw_mut_UMatData()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: data")
	}
	
	unsafe fn set_data(&mut self, val: &mut u8) -> () {
		{ sys::cv_UMatData_setPropData_unsigned_charX(self.as_raw_mut_UMatData(), val) }.into_result().expect("Infallible function failed: set_data")
	}
	
	fn origdata(&mut self) -> &mut u8 {
		unsafe { sys::cv_UMatData_getPropOrigdata(self.as_raw_mut_UMatData()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: origdata")
	}
	
	fn set_origdata(&mut self, val: &mut u8) -> () {
		unsafe { sys::cv_UMatData_setPropOrigdata_unsigned_charX(self.as_raw_mut_UMatData(), val) }.into_result().expect("Infallible function failed: set_origdata")
	}
	
	fn size(&self) -> size_t {
		unsafe { sys::cv_UMatData_getPropSize_const(self.as_raw_UMatData()) }.into_result().expect("Infallible function failed: size")
	}
	
	fn set_size(&mut self, val: size_t) -> () {
		unsafe { sys::cv_UMatData_setPropSize_size_t(self.as_raw_mut_UMatData(), val) }.into_result().expect("Infallible function failed: set_size")
	}
	
	fn flags(&self) -> core::UMatData_MemoryFlag {
		unsafe { sys::cv_UMatData_getPropFlags_const(self.as_raw_UMatData()) }.into_result().expect("Infallible function failed: flags")
	}
	
	fn set_flags(&mut self, val: core::UMatData_MemoryFlag) -> () {
		unsafe { sys::cv_UMatData_setPropFlags_MemoryFlag(self.as_raw_mut_UMatData(), val) }.into_result().expect("Infallible function failed: set_flags")
	}
	
	fn handle(&mut self) -> *mut c_void {
		unsafe { sys::cv_UMatData_getPropHandle(self.as_raw_mut_UMatData()) }.into_result().expect("Infallible function failed: handle")
	}
	
	fn set_handle(&mut self, val: *mut c_void) -> () {
		unsafe { sys::cv_UMatData_setPropHandle_voidX(self.as_raw_mut_UMatData(), val) }.into_result().expect("Infallible function failed: set_handle")
	}
	
	fn userdata(&mut self) -> *mut c_void {
		unsafe { sys::cv_UMatData_getPropUserdata(self.as_raw_mut_UMatData()) }.into_result().expect("Infallible function failed: userdata")
	}
	
	fn set_userdata(&mut self, val: *mut c_void) -> () {
		unsafe { sys::cv_UMatData_setPropUserdata_voidX(self.as_raw_mut_UMatData(), val) }.into_result().expect("Infallible function failed: set_userdata")
	}
	
	fn allocator_flags_(&self) -> i32 {
		unsafe { sys::cv_UMatData_getPropAllocatorFlags__const(self.as_raw_UMatData()) }.into_result().expect("Infallible function failed: allocator_flags_")
	}
	
	fn set_allocator_flags_(&mut self, val: i32) -> () {
		unsafe { sys::cv_UMatData_setPropAllocatorFlags__int(self.as_raw_mut_UMatData(), val) }.into_result().expect("Infallible function failed: set_allocator_flags_")
	}
	
	fn mapcount(&self) -> i32 {
		unsafe { sys::cv_UMatData_getPropMapcount_const(self.as_raw_UMatData()) }.into_result().expect("Infallible function failed: mapcount")
	}
	
	fn set_mapcount(&mut self, val: i32) -> () {
		unsafe { sys::cv_UMatData_setPropMapcount_int(self.as_raw_mut_UMatData(), val) }.into_result().expect("Infallible function failed: set_mapcount")
	}
	
	fn original_umat_data(&mut self) -> core::UMatData {
		unsafe { sys::cv_UMatData_getPropOriginalUMatData(self.as_raw_mut_UMatData()) }.into_result().map(|r| unsafe { core::UMatData::opencv_from_extern(r) } ).expect("Infallible function failed: original_umat_data")
	}
	
	fn set_original_umat_data(&mut self, val: &mut core::UMatData) -> () {
		unsafe { sys::cv_UMatData_setPropOriginalUMatData_UMatDataX(self.as_raw_mut_UMatData(), val.as_raw_mut_UMatData()) }.into_result().expect("Infallible function failed: set_original_umat_data")
	}
	
	fn lock(&mut self) -> Result<()> {
		unsafe { sys::cv_UMatData_lock(self.as_raw_mut_UMatData()) }.into_result()
	}
	
	fn unlock(&mut self) -> Result<()> {
		unsafe { sys::cv_UMatData_unlock(self.as_raw_mut_UMatData()) }.into_result()
	}
	
	fn host_copy_obsolete(&self) -> Result<bool> {
		unsafe { sys::cv_UMatData_hostCopyObsolete_const(self.as_raw_UMatData()) }.into_result()
	}
	
	fn device_copy_obsolete(&self) -> Result<bool> {
		unsafe { sys::cv_UMatData_deviceCopyObsolete_const(self.as_raw_UMatData()) }.into_result()
	}
	
	fn device_mem_mapped(&self) -> Result<bool> {
		unsafe { sys::cv_UMatData_deviceMemMapped_const(self.as_raw_UMatData()) }.into_result()
	}
	
	fn copy_on_map(&self) -> Result<bool> {
		unsafe { sys::cv_UMatData_copyOnMap_const(self.as_raw_UMatData()) }.into_result()
	}
	
	fn temp_umat(&self) -> Result<bool> {
		unsafe { sys::cv_UMatData_tempUMat_const(self.as_raw_UMatData()) }.into_result()
	}
	
	fn temp_copied_umat(&self) -> Result<bool> {
		unsafe { sys::cv_UMatData_tempCopiedUMat_const(self.as_raw_UMatData()) }.into_result()
	}
	
	fn mark_host_copy_obsolete(&mut self, flag: bool) -> Result<()> {
		unsafe { sys::cv_UMatData_markHostCopyObsolete_bool(self.as_raw_mut_UMatData(), flag) }.into_result()
	}
	
	fn mark_device_copy_obsolete(&mut self, flag: bool) -> Result<()> {
		unsafe { sys::cv_UMatData_markDeviceCopyObsolete_bool(self.as_raw_mut_UMatData(), flag) }.into_result()
	}
	
	fn mark_device_mem_mapped(&mut self, flag: bool) -> Result<()> {
		unsafe { sys::cv_UMatData_markDeviceMemMapped_bool(self.as_raw_mut_UMatData(), flag) }.into_result()
	}
	
}

pub struct UMatData {
	ptr: *mut c_void
}

opencv_type_boxed! { UMatData }

impl Drop for UMatData {
	fn drop(&mut self) {
		extern "C" { fn cv_UMatData_delete(instance: *mut c_void); }
		unsafe { cv_UMatData_delete(self.as_raw_mut_UMatData()) };
	}
}

impl UMatData {
	#[inline] pub fn as_raw_UMatData(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_UMatData(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for UMatData {}

impl core::UMatDataTrait for UMatData {
	#[inline] fn as_raw_UMatData(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_UMatData(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl UMatData {
}

/// This is the proxy class for passing read-only input arrays into OpenCV functions.
/// 
/// It is defined as:
/// ```ignore
///    typedef const _InputArray& InputArray;
/// ```
/// 
/// where _InputArray is a class that can be constructed from `Mat`, `Mat_<T>`, `Matx<T, m, n>`,
/// `std::vector<T>`, `std::vector<std::vector<T> >`, `std::vector<Mat>`, `std::vector<Mat_<T> >`,
/// `UMat`, `std::vector<UMat>` or `double`. It can also be constructed from a matrix expression.
/// 
/// Since this is mostly implementation-level class, and its interface may change in future versions, we
/// do not describe it in details. There are a few key things, though, that should be kept in mind:
/// 
/// *   When you see in the reference manual or in OpenCV source code a function that takes
///    InputArray, it means that you can actually pass `Mat`, `Matx`, `vector<T>` etc. (see above the
///    complete list).
/// *   Optional input arguments: If some of the input arrays may be empty, pass cv::noArray() (or
///    simply cv::Mat() as you probably did before).
/// *   The class is designed solely for passing parameters. That is, normally you *should not*
///    declare class members, local and global variables of this type.
/// *   If you want to design your own function or a class method that can operate of arrays of
///    multiple types, you can use InputArray (or OutputArray) for the respective parameters. Inside
///    a function you should use _InputArray::getMat() method to construct a matrix header for the
///    array (without copying data). _InputArray::kind() can be used to distinguish Mat from
///    `vector<>` etc., but normally it is not needed.
/// 
/// Here is how you can use a function that takes InputArray :
/// ```ignore
///    std::vector<Point2f> vec;
///    // points or a circle
///    for( int i = 0; i < 30; i++ )
///        vec.push_back(Point2f((float)(100 + 30*cos(i*CV_PI*2/5)),
///                               (float)(100 - 30*sin(i*CV_PI*2/5))));
///    cv::transform(vec, vec, cv::Matx23f(0.707, -0.707, 10, 0.707, 0.707, 20));
/// ```
/// 
/// That is, we form an STL vector containing points, and apply in-place affine transformation to the
/// vector using the 2x3 matrix created inline as `Matx<float, 2, 3>` instance.
/// 
/// Here is how such a function can be implemented (for simplicity, we implement a very specific case of
/// it, according to the assertion statement inside) :
/// ```ignore
///    void myAffineTransform(InputArray _src, OutputArray _dst, InputArray _m)
///    {
///        // get Mat headers for input arrays. This is O(1) operation,
///        // unless _src and/or _m are matrix expressions.
///        Mat src = _src.getMat(), m = _m.getMat();
///        CV_Assert( src.type() == CV_32FC2 && m.type() == CV_32F && m.size() == Size(3, 2) );
/// 
///        // [re]create the output array so that it has the proper size and type.
///        // In case of Mat it calls Mat::create, in case of STL vector it calls vector::resize.
///        _dst.create(src.size(), src.type());
///        Mat dst = _dst.getMat();
/// 
///        for( int i = 0; i < src.rows; i++ )
///            for( int j = 0; j < src.cols; j++ )
///            {
///                Point2f pt = src.at<Point2f>(i, j);
///                dst.at<Point2f>(i, j) = Point2f(m.at<float>(0, 0)*pt.x +
///                                                 m.at<float>(0, 1)*pt.y +
///                                                 m.at<float>(0, 2),
///                                                 m.at<float>(1, 0)*pt.x +
///                                                 m.at<float>(1, 1)*pt.y +
///                                                 m.at<float>(1, 2));
///            }
///    }
/// ```
/// 
/// There is another related type, InputArrayOfArrays, which is currently defined as a synonym for
/// InputArray:
/// ```ignore
///    typedef InputArray InputArrayOfArrays;
/// ```
/// 
/// It denotes function arguments that are either vectors of vectors or vectors of matrices. A separate
/// synonym is needed to generate Python/Java etc. wrappers properly. At the function implementation
/// level their use is similar, but _InputArray::getMat(idx) should be used to get header for the
/// idx-th component of the outer vector and _InputArray::size().area() should be used to find the
/// number of components (vectors/matrices) of the outer vector.
/// 
/// In general, type support is limited to cv::Mat types. Other types are forbidden.
/// But in some cases we need to support passing of custom non-general Mat types, like arrays of cv::KeyPoint, cv::DMatch, etc.
/// This data is not intended to be interpreted as an image data, or processed somehow like regular cv::Mat.
/// To pass such custom type use rawIn() / rawOut() / rawInOut() wrappers.
/// Custom type is wrapped as Mat-compatible `CV_8UC<N>` values (N = sizeof(T), N <= CV_CN_MAX).
pub trait _InputArrayTrait {
	fn as_raw__InputArray(&self) -> *const c_void;
	fn as_raw_mut__InputArray(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * idx: -1
	fn get_mat(&self, idx: i32) -> Result<core::Mat> {
		unsafe { sys::cv__InputArray_getMat_const_int(self.as_raw__InputArray(), idx) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	fn get_mat_(&self, idx: i32) -> Result<core::Mat> {
		unsafe { sys::cv__InputArray_getMat__const_int(self.as_raw__InputArray(), idx) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	fn get_umat(&self, idx: i32) -> Result<core::UMat> {
		unsafe { sys::cv__InputArray_getUMat_const_int(self.as_raw__InputArray(), idx) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	fn get_mat_vector(&self, mv: &mut core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv__InputArray_getMatVector_const_vector_Mat_R(self.as_raw__InputArray(), mv.as_raw_mut_VectorOfMat()) }.into_result()
	}
	
	fn get_umat_vector(&self, umv: &mut core::Vector::<core::UMat>) -> Result<()> {
		unsafe { sys::cv__InputArray_getUMatVector_const_vector_UMat_R(self.as_raw__InputArray(), umv.as_raw_mut_VectorOfUMat()) }.into_result()
	}
	
	fn get_gpu_mat_vector(&self, gpumv: &mut core::Vector::<core::GpuMat>) -> Result<()> {
		unsafe { sys::cv__InputArray_getGpuMatVector_const_vector_GpuMat_R(self.as_raw__InputArray(), gpumv.as_raw_mut_VectorOfGpuMat()) }.into_result()
	}
	
	fn get_gpu_mat(&self) -> Result<core::GpuMat> {
		unsafe { sys::cv__InputArray_getGpuMat_const(self.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	fn get_flags(&self) -> Result<i32> {
		unsafe { sys::cv__InputArray_getFlags_const(self.as_raw__InputArray()) }.into_result()
	}
	
	fn get_obj(&self) -> Result<*mut c_void> {
		unsafe { sys::cv__InputArray_getObj_const(self.as_raw__InputArray()) }.into_result()
	}
	
	fn get_sz(&self) -> Result<core::Size> {
		unsafe { sys::cv__InputArray_getSz_const(self.as_raw__InputArray()) }.into_result()
	}
	
	fn kind(&self) -> Result<core::_InputArray_KindFlag> {
		unsafe { sys::cv__InputArray_kind_const(self.as_raw__InputArray()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn dims(&self, i: i32) -> Result<i32> {
		unsafe { sys::cv__InputArray_dims_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn cols(&self, i: i32) -> Result<i32> {
		unsafe { sys::cv__InputArray_cols_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn rows(&self, i: i32) -> Result<i32> {
		unsafe { sys::cv__InputArray_rows_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn size(&self, i: i32) -> Result<core::Size> {
		unsafe { sys::cv__InputArray_size_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn sizend(&self, sz: &mut i32, i: i32) -> Result<i32> {
		unsafe { sys::cv__InputArray_sizend_const_intX_int(self.as_raw__InputArray(), sz, i) }.into_result()
	}
	
	fn same_size(&self, arr: &dyn core::ToInputArray) -> Result<bool> {
		input_array_arg!(arr);
		unsafe { sys::cv__InputArray_sameSize_const_const__InputArrayR(self.as_raw__InputArray(), arr.as_raw__InputArray()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn total(&self, i: i32) -> Result<size_t> {
		unsafe { sys::cv__InputArray_total_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn typ(&self, i: i32) -> Result<i32> {
		unsafe { sys::cv__InputArray_type_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn depth(&self, i: i32) -> Result<i32> {
		unsafe { sys::cv__InputArray_depth_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn channels(&self, i: i32) -> Result<i32> {
		unsafe { sys::cv__InputArray_channels_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn is_continuous(&self, i: i32) -> Result<bool> {
		unsafe { sys::cv__InputArray_isContinuous_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn is_submatrix(&self, i: i32) -> Result<bool> {
		unsafe { sys::cv__InputArray_isSubmatrix_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv__InputArray_empty_const(self.as_raw__InputArray()) }.into_result()
	}
	
	fn copy_to(&self, arr: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(arr);
		unsafe { sys::cv__InputArray_copyTo_const_const__OutputArrayR(self.as_raw__InputArray(), arr.as_raw__OutputArray()) }.into_result()
	}
	
	fn copy_to_masked(&self, arr: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		output_array_arg!(arr);
		input_array_arg!(mask);
		unsafe { sys::cv__InputArray_copyTo_const_const__OutputArrayR_const__InputArrayR(self.as_raw__InputArray(), arr.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn offset(&self, i: i32) -> Result<size_t> {
		unsafe { sys::cv__InputArray_offset_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn step(&self, i: i32) -> Result<size_t> {
		unsafe { sys::cv__InputArray_step_const_int(self.as_raw__InputArray(), i) }.into_result()
	}
	
	fn is_mat(&self) -> Result<bool> {
		unsafe { sys::cv__InputArray_isMat_const(self.as_raw__InputArray()) }.into_result()
	}
	
	fn is_umat(&self) -> Result<bool> {
		unsafe { sys::cv__InputArray_isUMat_const(self.as_raw__InputArray()) }.into_result()
	}
	
	fn is_mat_vector(&self) -> Result<bool> {
		unsafe { sys::cv__InputArray_isMatVector_const(self.as_raw__InputArray()) }.into_result()
	}
	
	fn is_umat_vector(&self) -> Result<bool> {
		unsafe { sys::cv__InputArray_isUMatVector_const(self.as_raw__InputArray()) }.into_result()
	}
	
	fn is_matx(&self) -> Result<bool> {
		unsafe { sys::cv__InputArray_isMatx_const(self.as_raw__InputArray()) }.into_result()
	}
	
	fn is_vector(&self) -> Result<bool> {
		unsafe { sys::cv__InputArray_isVector_const(self.as_raw__InputArray()) }.into_result()
	}
	
	fn is_gpu_mat(&self) -> Result<bool> {
		unsafe { sys::cv__InputArray_isGpuMat_const(self.as_raw__InputArray()) }.into_result()
	}
	
	fn is_gpu_mat_vector(&self) -> Result<bool> {
		unsafe { sys::cv__InputArray_isGpuMatVector_const(self.as_raw__InputArray()) }.into_result()
	}
	
}

/// This is the proxy class for passing read-only input arrays into OpenCV functions.
/// 
/// It is defined as:
/// ```ignore
///    typedef const _InputArray& InputArray;
/// ```
/// 
/// where _InputArray is a class that can be constructed from `Mat`, `Mat_<T>`, `Matx<T, m, n>`,
/// `std::vector<T>`, `std::vector<std::vector<T> >`, `std::vector<Mat>`, `std::vector<Mat_<T> >`,
/// `UMat`, `std::vector<UMat>` or `double`. It can also be constructed from a matrix expression.
/// 
/// Since this is mostly implementation-level class, and its interface may change in future versions, we
/// do not describe it in details. There are a few key things, though, that should be kept in mind:
/// 
/// *   When you see in the reference manual or in OpenCV source code a function that takes
///    InputArray, it means that you can actually pass `Mat`, `Matx`, `vector<T>` etc. (see above the
///    complete list).
/// *   Optional input arguments: If some of the input arrays may be empty, pass cv::noArray() (or
///    simply cv::Mat() as you probably did before).
/// *   The class is designed solely for passing parameters. That is, normally you *should not*
///    declare class members, local and global variables of this type.
/// *   If you want to design your own function or a class method that can operate of arrays of
///    multiple types, you can use InputArray (or OutputArray) for the respective parameters. Inside
///    a function you should use _InputArray::getMat() method to construct a matrix header for the
///    array (without copying data). _InputArray::kind() can be used to distinguish Mat from
///    `vector<>` etc., but normally it is not needed.
/// 
/// Here is how you can use a function that takes InputArray :
/// ```ignore
///    std::vector<Point2f> vec;
///    // points or a circle
///    for( int i = 0; i < 30; i++ )
///        vec.push_back(Point2f((float)(100 + 30*cos(i*CV_PI*2/5)),
///                               (float)(100 - 30*sin(i*CV_PI*2/5))));
///    cv::transform(vec, vec, cv::Matx23f(0.707, -0.707, 10, 0.707, 0.707, 20));
/// ```
/// 
/// That is, we form an STL vector containing points, and apply in-place affine transformation to the
/// vector using the 2x3 matrix created inline as `Matx<float, 2, 3>` instance.
/// 
/// Here is how such a function can be implemented (for simplicity, we implement a very specific case of
/// it, according to the assertion statement inside) :
/// ```ignore
///    void myAffineTransform(InputArray _src, OutputArray _dst, InputArray _m)
///    {
///        // get Mat headers for input arrays. This is O(1) operation,
///        // unless _src and/or _m are matrix expressions.
///        Mat src = _src.getMat(), m = _m.getMat();
///        CV_Assert( src.type() == CV_32FC2 && m.type() == CV_32F && m.size() == Size(3, 2) );
/// 
///        // [re]create the output array so that it has the proper size and type.
///        // In case of Mat it calls Mat::create, in case of STL vector it calls vector::resize.
///        _dst.create(src.size(), src.type());
///        Mat dst = _dst.getMat();
/// 
///        for( int i = 0; i < src.rows; i++ )
///            for( int j = 0; j < src.cols; j++ )
///            {
///                Point2f pt = src.at<Point2f>(i, j);
///                dst.at<Point2f>(i, j) = Point2f(m.at<float>(0, 0)*pt.x +
///                                                 m.at<float>(0, 1)*pt.y +
///                                                 m.at<float>(0, 2),
///                                                 m.at<float>(1, 0)*pt.x +
///                                                 m.at<float>(1, 1)*pt.y +
///                                                 m.at<float>(1, 2));
///            }
///    }
/// ```
/// 
/// There is another related type, InputArrayOfArrays, which is currently defined as a synonym for
/// InputArray:
/// ```ignore
///    typedef InputArray InputArrayOfArrays;
/// ```
/// 
/// It denotes function arguments that are either vectors of vectors or vectors of matrices. A separate
/// synonym is needed to generate Python/Java etc. wrappers properly. At the function implementation
/// level their use is similar, but _InputArray::getMat(idx) should be used to get header for the
/// idx-th component of the outer vector and _InputArray::size().area() should be used to find the
/// number of components (vectors/matrices) of the outer vector.
/// 
/// In general, type support is limited to cv::Mat types. Other types are forbidden.
/// But in some cases we need to support passing of custom non-general Mat types, like arrays of cv::KeyPoint, cv::DMatch, etc.
/// This data is not intended to be interpreted as an image data, or processed somehow like regular cv::Mat.
/// To pass such custom type use rawIn() / rawOut() / rawInOut() wrappers.
/// Custom type is wrapped as Mat-compatible `CV_8UC<N>` values (N = sizeof(T), N <= CV_CN_MAX).
pub struct _InputArray {
	ptr: *mut c_void
}

opencv_type_boxed! { _InputArray }

impl Drop for _InputArray {
	fn drop(&mut self) {
		extern "C" { fn cv__InputArray_delete(instance: *mut c_void); }
		unsafe { cv__InputArray_delete(self.as_raw_mut__InputArray()) };
	}
}

impl _InputArray {
	#[inline] pub fn as_raw__InputArray(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut__InputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for _InputArray {}

impl core::_InputArrayTrait for _InputArray {
	#[inline] fn as_raw__InputArray(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut__InputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl _InputArray {
	pub fn default() -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray() }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
	pub fn new(_flags: i32, _obj: *mut c_void) -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray_int_voidX(_flags, _obj) }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_mat(m: &core::Mat) -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray_const_MatR(m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_matexpr(expr: &core::MatExpr) -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray_const_MatExprR(expr.as_raw_MatExpr()) }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_mat_vec(vec: &core::Vector::<core::Mat>) -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray_const_vector_Mat_R(vec.as_raw_VectorOfMat()) }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_bool_vec(vec: &core::Vector::<bool>) -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray_const_vector_bool_R(vec.as_raw_VectorOfbool()) }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_f64(val: &f64) -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray_const_doubleR(val) }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_gpumat(d_mat: &core::GpuMat) -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray_const_GpuMatR(d_mat.as_raw_GpuMat()) }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_gpumat_vec(d_mat_array: &core::Vector::<core::GpuMat>) -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray_const_vector_GpuMat_R(d_mat_array.as_raw_VectorOfGpuMat()) }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_hostmem(cuda_mem: &core::HostMem) -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray_const_HostMemR(cuda_mem.as_raw_HostMem()) }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_umat(um: &core::UMat) -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray_const_UMatR(um.as_raw_UMat()) }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_umat_vec(umv: &core::Vector::<core::UMat>) -> Result<core::_InputArray> {
		unsafe { sys::cv__InputArray__InputArray_const_vector_UMat_R(umv.as_raw_VectorOfUMat()) }.into_result().map(|r| unsafe { core::_InputArray::opencv_from_extern(r) } )
	}
	
}

pub trait _InputOutputArrayTrait: core::_OutputArrayTrait {
	fn as_raw__InputOutputArray(&self) -> *const c_void;
	fn as_raw_mut__InputOutputArray(&mut self) -> *mut c_void;

}

pub struct _InputOutputArray {
	ptr: *mut c_void
}

opencv_type_boxed! { _InputOutputArray }

impl Drop for _InputOutputArray {
	fn drop(&mut self) {
		extern "C" { fn cv__InputOutputArray_delete(instance: *mut c_void); }
		unsafe { cv__InputOutputArray_delete(self.as_raw_mut__InputOutputArray()) };
	}
}

impl _InputOutputArray {
	#[inline] pub fn as_raw__InputOutputArray(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut__InputOutputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for _InputOutputArray {}

impl core::_InputArrayTrait for _InputOutputArray {
	#[inline] fn as_raw__InputArray(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut__InputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::_InputOutputArrayTrait for _InputOutputArray {
	#[inline] fn as_raw__InputOutputArray(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut__InputOutputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::_OutputArrayTrait for _InputOutputArray {
	#[inline] fn as_raw__OutputArray(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut__OutputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl _InputOutputArray {
	/// ////////////////////////////////////////////////////////////////////////////////////////
	pub fn default() -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray() }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn new(_flags: i32, _obj: *mut c_void) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_int_voidX(_flags, _obj) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_mat_mut(m: &mut core::Mat) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_MatR(m.as_raw_mut_Mat()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_mat_vec_mut(vec: &mut core::Vector::<core::Mat>) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_vector_Mat_R(vec.as_raw_mut_VectorOfMat()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_gpumat_mut(d_mat: &mut core::GpuMat) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_GpuMatR(d_mat.as_raw_mut_GpuMat()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_hostmem_mut(cuda_mem: &mut core::HostMem) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_HostMemR(cuda_mem.as_raw_mut_HostMem()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_umat_mut(m: &mut core::UMat) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_UMatR(m.as_raw_mut_UMat()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_umat_vec_mut(vec: &mut core::Vector::<core::UMat>) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_vector_UMat_R(vec.as_raw_mut_VectorOfUMat()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_mat(m: &core::Mat) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_MatR(m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_mat_vec(vec: &core::Vector::<core::Mat>) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_vector_Mat_R(vec.as_raw_VectorOfMat()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_gpumat(d_mat: &core::GpuMat) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_GpuMatR(d_mat.as_raw_GpuMat()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_gpumat_vec(d_mat: &core::Vector::<core::GpuMat>) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_vector_GpuMat_R(d_mat.as_raw_VectorOfGpuMat()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_hostmem(cuda_mem: &core::HostMem) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_HostMemR(cuda_mem.as_raw_HostMem()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_umat(m: &core::UMat) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_UMatR(m.as_raw_UMat()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_umat_vec(vec: &core::Vector::<core::UMat>) -> Result<core::_InputOutputArray> {
		unsafe { sys::cv__InputOutputArray__InputOutputArray_const_vector_UMat_R(vec.as_raw_VectorOfUMat()) }.into_result().map(|r| unsafe { core::_InputOutputArray::opencv_from_extern(r) } )
	}
	
}

/// This type is very similar to InputArray except that it is used for input/output and output function
/// parameters.
/// 
/// Just like with InputArray, OpenCV users should not care about OutputArray, they just pass `Mat`,
/// `vector<T>` etc. to the functions. The same limitation as for `InputArray`: *Do not explicitly
/// create OutputArray instances* applies here too.
/// 
/// If you want to make your function polymorphic (i.e. accept different arrays as output parameters),
/// it is also not very difficult. Take the sample above as the reference. Note that
/// _OutputArray::create() needs to be called before _OutputArray::getMat(). This way you guarantee
/// that the output array is properly allocated.
/// 
/// Optional output parameters. If you do not need certain output array to be computed and returned to
/// you, pass cv::noArray(), just like you would in the case of optional input array. At the
/// implementation level, use _OutputArray::needed() to check if certain output array needs to be
/// computed or not.
/// 
/// There are several synonyms for OutputArray that are used to assist automatic Python/Java/... wrapper
/// generators:
/// ```ignore
///    typedef OutputArray OutputArrayOfArrays;
///    typedef OutputArray InputOutputArray;
///    typedef OutputArray InputOutputArrayOfArrays;
/// ```
/// 
pub trait _OutputArrayTrait: core::_InputArrayTrait {
	fn as_raw__OutputArray(&self) -> *const c_void;
	fn as_raw_mut__OutputArray(&mut self) -> *mut c_void;

	fn fixed_size(&self) -> Result<bool> {
		unsafe { sys::cv__OutputArray_fixedSize_const(self.as_raw__OutputArray()) }.into_result()
	}
	
	fn fixed_type(&self) -> Result<bool> {
		unsafe { sys::cv__OutputArray_fixedType_const(self.as_raw__OutputArray()) }.into_result()
	}
	
	fn needed(&self) -> Result<bool> {
		unsafe { sys::cv__OutputArray_needed_const(self.as_raw__OutputArray()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn get_mat_ref(&self, i: i32) -> Result<core::Mat> {
		unsafe { sys::cv__OutputArray_getMatRef_const_int(self.as_raw__OutputArray(), i) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * i: -1
	fn get_umat_ref(&self, i: i32) -> Result<core::UMat> {
		unsafe { sys::cv__OutputArray_getUMatRef_const_int(self.as_raw__OutputArray(), i) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	fn get_gpu_mat_ref(&self) -> Result<core::GpuMat> {
		unsafe { sys::cv__OutputArray_getGpuMatRef_const(self.as_raw__OutputArray()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	fn get_gpu_mat_vec_ref(&self) -> Result<core::Vector::<core::GpuMat>> {
		unsafe { sys::cv__OutputArray_getGpuMatVecRef_const(self.as_raw__OutputArray()) }.into_result().map(|r| unsafe { core::Vector::<core::GpuMat>::opencv_from_extern(r) } )
	}
	
	fn get_host_mem_ref(&self) -> Result<core::HostMem> {
		unsafe { sys::cv__OutputArray_getHostMemRef_const(self.as_raw__OutputArray()) }.into_result().map(|r| unsafe { core::HostMem::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * i: -1
	/// * allow_transposed: false
	/// * fixed_depth_mask: static_cast<_OutputArray::DepthMask>(0)
	fn create_size(&self, sz: core::Size, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask) -> Result<()> {
		unsafe { sys::cv__OutputArray_create_const_Size_int_int_bool_DepthMask(self.as_raw__OutputArray(), sz.opencv_to_extern(), typ, i, allow_transposed, fixed_depth_mask) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	/// * allow_transposed: false
	/// * fixed_depth_mask: static_cast<_OutputArray::DepthMask>(0)
	fn create(&self, rows: i32, cols: i32, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask) -> Result<()> {
		unsafe { sys::cv__OutputArray_create_const_int_int_int_int_bool_DepthMask(self.as_raw__OutputArray(), rows, cols, typ, i, allow_transposed, fixed_depth_mask) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * i: -1
	/// * allow_transposed: false
	/// * fixed_depth_mask: static_cast<_OutputArray::DepthMask>(0)
	fn create_nd(&self, size: &[i32], typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask) -> Result<()> {
		unsafe { sys::cv__OutputArray_create_const_int_const_intX_int_int_bool_DepthMask(self.as_raw__OutputArray(), size.len() as _, size.as_ptr(), typ, i, allow_transposed, fixed_depth_mask) }.into_result()
	}
	
	unsafe fn create_same_size(&self, arr: &dyn core::ToInputArray, mtype: i32) -> Result<()> {
		input_array_arg!(arr);
		{ sys::cv__OutputArray_createSameSize_const_const__InputArrayR_int(self.as_raw__OutputArray(), arr.as_raw__InputArray(), mtype) }.into_result()
	}
	
	fn release(&self) -> Result<()> {
		unsafe { sys::cv__OutputArray_release_const(self.as_raw__OutputArray()) }.into_result()
	}
	
	fn clear(&self) -> Result<()> {
		unsafe { sys::cv__OutputArray_clear_const(self.as_raw__OutputArray()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * mask: _InputArray()
	fn set_to(&self, value: &dyn core::ToInputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(value);
		input_array_arg!(mask);
		unsafe { sys::cv__OutputArray_setTo_const_const__InputArrayR_const__InputArrayR(self.as_raw__OutputArray(), value.as_raw__InputArray(), mask.as_raw__InputArray()) }.into_result()
	}
	
	fn assign(&self, u: &core::UMat) -> Result<()> {
		unsafe { sys::cv__OutputArray_assign_const_const_UMatR(self.as_raw__OutputArray(), u.as_raw_UMat()) }.into_result()
	}
	
	fn assign_1(&self, m: &core::Mat) -> Result<()> {
		unsafe { sys::cv__OutputArray_assign_const_const_MatR(self.as_raw__OutputArray(), m.as_raw_Mat()) }.into_result()
	}
	
	fn assign_2(&self, v: &core::Vector::<core::UMat>) -> Result<()> {
		unsafe { sys::cv__OutputArray_assign_const_const_vector_UMat_R(self.as_raw__OutputArray(), v.as_raw_VectorOfUMat()) }.into_result()
	}
	
	fn assign_3(&self, v: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv__OutputArray_assign_const_const_vector_Mat_R(self.as_raw__OutputArray(), v.as_raw_VectorOfMat()) }.into_result()
	}
	
	fn move_(&self, u: &mut core::UMat) -> Result<()> {
		unsafe { sys::cv__OutputArray_move_const_UMatR(self.as_raw__OutputArray(), u.as_raw_mut_UMat()) }.into_result()
	}
	
	fn move__1(&self, m: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv__OutputArray_move_const_MatR(self.as_raw__OutputArray(), m.as_raw_mut_Mat()) }.into_result()
	}
	
}

/// This type is very similar to InputArray except that it is used for input/output and output function
/// parameters.
/// 
/// Just like with InputArray, OpenCV users should not care about OutputArray, they just pass `Mat`,
/// `vector<T>` etc. to the functions. The same limitation as for `InputArray`: *Do not explicitly
/// create OutputArray instances* applies here too.
/// 
/// If you want to make your function polymorphic (i.e. accept different arrays as output parameters),
/// it is also not very difficult. Take the sample above as the reference. Note that
/// _OutputArray::create() needs to be called before _OutputArray::getMat(). This way you guarantee
/// that the output array is properly allocated.
/// 
/// Optional output parameters. If you do not need certain output array to be computed and returned to
/// you, pass cv::noArray(), just like you would in the case of optional input array. At the
/// implementation level, use _OutputArray::needed() to check if certain output array needs to be
/// computed or not.
/// 
/// There are several synonyms for OutputArray that are used to assist automatic Python/Java/... wrapper
/// generators:
/// ```ignore
///    typedef OutputArray OutputArrayOfArrays;
///    typedef OutputArray InputOutputArray;
///    typedef OutputArray InputOutputArrayOfArrays;
/// ```
/// 
pub struct _OutputArray {
	ptr: *mut c_void
}

opencv_type_boxed! { _OutputArray }

impl Drop for _OutputArray {
	fn drop(&mut self) {
		extern "C" { fn cv__OutputArray_delete(instance: *mut c_void); }
		unsafe { cv__OutputArray_delete(self.as_raw_mut__OutputArray()) };
	}
}

impl _OutputArray {
	#[inline] pub fn as_raw__OutputArray(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut__OutputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for _OutputArray {}

impl core::_InputArrayTrait for _OutputArray {
	#[inline] fn as_raw__InputArray(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut__InputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::_OutputArrayTrait for _OutputArray {
	#[inline] fn as_raw__OutputArray(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut__OutputArray(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl _OutputArray {
	/// /////////////////////////////////////////////////////////////////////////////////////
	pub fn default() -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray() }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn new(_flags: i32, _obj: *mut c_void) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_int_voidX(_flags, _obj) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_mat_mut(m: &mut core::Mat) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_MatR(m.as_raw_mut_Mat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_mat_vec_mut(vec: &mut core::Vector::<core::Mat>) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_vector_Mat_R(vec.as_raw_mut_VectorOfMat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_gpumat_mut(d_mat: &mut core::GpuMat) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_GpuMatR(d_mat.as_raw_mut_GpuMat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_gpumat_vec_mut(d_mat: &mut core::Vector::<core::GpuMat>) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_vector_GpuMat_R(d_mat.as_raw_mut_VectorOfGpuMat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_hostmem_mut(cuda_mem: &mut core::HostMem) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_HostMemR(cuda_mem.as_raw_mut_HostMem()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_umat_mut(m: &mut core::UMat) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_UMatR(m.as_raw_mut_UMat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_umat_vec_mut(vec: &mut core::Vector::<core::UMat>) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_vector_UMat_R(vec.as_raw_mut_VectorOfUMat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_mat(m: &core::Mat) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_const_MatR(m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_mat_vec(vec: &core::Vector::<core::Mat>) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_const_vector_Mat_R(vec.as_raw_VectorOfMat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_gpumat(d_mat: &core::GpuMat) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_const_GpuMatR(d_mat.as_raw_GpuMat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	#[cfg(not(target_os = "windows"))]
	pub fn from_gpumat_vec(d_mat: &core::Vector::<core::GpuMat>) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_const_vector_GpuMat_R(d_mat.as_raw_VectorOfGpuMat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_hostmem(cuda_mem: &core::HostMem) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_const_HostMemR(cuda_mem.as_raw_HostMem()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_umat(m: &core::UMat) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_const_UMatR(m.as_raw_UMat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
	pub fn from_umat_vec(vec: &core::Vector::<core::UMat>) -> Result<core::_OutputArray> {
		unsafe { sys::cv__OutputArray__OutputArray_const_vector_UMat_R(vec.as_raw_VectorOfUMat()) }.into_result().map(|r| unsafe { core::_OutputArray::opencv_from_extern(r) } )
	}
	
}

/// BufferPool for use with CUDA streams
/// 
/// BufferPool utilizes Stream's allocator to create new buffers for GpuMat's. It is
/// only useful when enabled with #setBufferPoolUsage.
/// 
/// ```ignore
///    setBufferPoolUsage(true);
/// ```
/// 
/// 
/// 
/// Note: #setBufferPoolUsage must be called \em before any Stream declaration.
/// 
/// Users may specify custom allocator for Stream and may implement their own stream based
/// functions utilizing the same underlying GPU memory management.
/// 
/// If custom allocator is not specified, BufferPool utilizes StackAllocator by
/// default. StackAllocator allocates a chunk of GPU device memory beforehand,
/// and when GpuMat is declared later on, it is given the pre-allocated memory.
/// This kind of strategy reduces the number of calls for memory allocating APIs
/// such as cudaMalloc or cudaMallocPitch.
/// 
/// Below is an example that utilizes BufferPool with StackAllocator:
/// 
/// ```ignore
///    #include <opencv2/opencv.hpp>
/// 
///    using namespace cv;
///    using namespace cv::cuda
/// 
///    int main()
///    {
///        setBufferPoolUsage(true);                               // Tell OpenCV that we are going to utilize BufferPool
///        setBufferPoolConfig(getDevice(), 1024 * 1024 * 64, 2);  // Allocate 64 MB, 2 stacks (default is 10 MB, 5 stacks)
/// 
///        Stream stream1, stream2;                                // Each stream uses 1 stack
///        BufferPool pool1(stream1), pool2(stream2);
/// 
///        GpuMat d_src1 = pool1.getBuffer(4096, 4096, CV_8UC1);   // 16MB
///        GpuMat d_dst1 = pool1.getBuffer(4096, 4096, CV_8UC3);   // 48MB, pool1 is now full
/// 
///        GpuMat d_src2 = pool2.getBuffer(1024, 1024, CV_8UC1);   // 1MB
///        GpuMat d_dst2 = pool2.getBuffer(1024, 1024, CV_8UC3);   // 3MB
/// 
///        cvtColor(d_src1, d_dst1, CV_GRAY2BGR, 0, stream1);
///        cvtColor(d_src2, d_dst2, CV_GRAY2BGR, 0, stream2);
///    }
/// ```
/// 
/// 
/// If we allocate another GpuMat on pool1 in the above example, it will be carried out by
/// the DefaultAllocator since the stack for pool1 is full.
/// 
/// ```ignore
///    GpuMat d_add1 = pool1.getBuffer(1024, 1024, CV_8UC1);   // Stack for pool1 is full, memory is allocated with DefaultAllocator
/// ```
/// 
/// 
/// If a third stream is declared in the above example, allocating with #getBuffer
/// within that stream will also be carried out by the DefaultAllocator because we've run out of
/// stacks.
/// 
/// ```ignore
///    Stream stream3;                                         // Only 2 stacks were allocated, we've run out of stacks
///    BufferPool pool3(stream3);
///    GpuMat d_src3 = pool3.getBuffer(1024, 1024, CV_8UC1);   // Memory is allocated with DefaultAllocator
/// ```
/// 
/// 
/// @warning When utilizing StackAllocator, deallocation order is important.
/// 
/// Just like a stack, deallocation must be done in LIFO order. Below is an example of
/// erroneous usage that violates LIFO rule. If OpenCV is compiled in Debug mode, this
/// sample code will emit CV_Assert error.
/// 
/// ```ignore
///    int main()
///    {
///        setBufferPoolUsage(true);                               // Tell OpenCV that we are going to utilize BufferPool
///        Stream stream;                                          // A default size (10 MB) stack is allocated to this stream
///        BufferPool pool(stream);
/// 
///        GpuMat mat1 = pool.getBuffer(1024, 1024, CV_8UC1);      // Allocate mat1 (1MB)
///        GpuMat mat2 = pool.getBuffer(1024, 1024, CV_8UC1);      // Allocate mat2 (1MB)
/// 
///        mat1.release();                                         // erroneous usage : mat2 must be deallocated before mat1
///    }
/// ```
/// 
/// 
/// Since C++ local variables are destroyed in the reverse order of construction,
/// the code sample below satisfies the LIFO rule. Local GpuMat's are deallocated
/// and the corresponding memory is automatically returned to the pool for later usage.
/// 
/// ```ignore
///    int main()
///    {
///        setBufferPoolUsage(true);                               // Tell OpenCV that we are going to utilize BufferPool
///        setBufferPoolConfig(getDevice(), 1024 * 1024 * 64, 2);  // Allocate 64 MB, 2 stacks (default is 10 MB, 5 stacks)
/// 
///        Stream stream1, stream2;                                // Each stream uses 1 stack
///        BufferPool pool1(stream1), pool2(stream2);
/// 
///        for (int i = 0; i < 10; i++)
///        {
///            GpuMat d_src1 = pool1.getBuffer(4096, 4096, CV_8UC1);   // 16MB
///            GpuMat d_dst1 = pool1.getBuffer(4096, 4096, CV_8UC3);   // 48MB, pool1 is now full
/// 
///            GpuMat d_src2 = pool2.getBuffer(1024, 1024, CV_8UC1);   // 1MB
///            GpuMat d_dst2 = pool2.getBuffer(1024, 1024, CV_8UC3);   // 3MB
/// 
///            d_src1.setTo(Scalar(i), stream1);
///            d_src2.setTo(Scalar(i), stream2);
/// 
///            cvtColor(d_src1, d_dst1, CV_GRAY2BGR, 0, stream1);
///            cvtColor(d_src2, d_dst2, CV_GRAY2BGR, 0, stream2);
///                                                                     // The order of destruction of the local variables is:
///                                                                     //   d_dst2 => d_src2 => d_dst1 => d_src1
///                                                                     // LIFO rule is satisfied, this code runs without error
///        }
///    }
/// ```
/// 
pub trait BufferPoolTrait {
	fn as_raw_BufferPool(&self) -> *const c_void;
	fn as_raw_mut_BufferPool(&mut self) -> *mut c_void;

	/// Allocates a new GpuMat of given size and type.
	fn get_buffer(&mut self, rows: i32, cols: i32, typ: i32) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_BufferPool_getBuffer_int_int_int(self.as_raw_mut_BufferPool(), rows, cols, typ) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// Allocates a new GpuMat of given size and type.
	fn get_buffer_1(&mut self, size: core::Size, typ: i32) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_BufferPool_getBuffer_Size_int(self.as_raw_mut_BufferPool(), size.opencv_to_extern(), typ) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// Returns the allocator associated with the stream.
	fn get_allocator(&self) -> Result<core::Ptr::<dyn core::GpuMat_Allocator>> {
		unsafe { sys::cv_cuda_BufferPool_getAllocator_const(self.as_raw_BufferPool()) }.into_result().map(|r| unsafe { core::Ptr::<dyn core::GpuMat_Allocator>::opencv_from_extern(r) } )
	}
	
}

/// BufferPool for use with CUDA streams
/// 
/// BufferPool utilizes Stream's allocator to create new buffers for GpuMat's. It is
/// only useful when enabled with #setBufferPoolUsage.
/// 
/// ```ignore
///    setBufferPoolUsage(true);
/// ```
/// 
/// 
/// 
/// Note: #setBufferPoolUsage must be called \em before any Stream declaration.
/// 
/// Users may specify custom allocator for Stream and may implement their own stream based
/// functions utilizing the same underlying GPU memory management.
/// 
/// If custom allocator is not specified, BufferPool utilizes StackAllocator by
/// default. StackAllocator allocates a chunk of GPU device memory beforehand,
/// and when GpuMat is declared later on, it is given the pre-allocated memory.
/// This kind of strategy reduces the number of calls for memory allocating APIs
/// such as cudaMalloc or cudaMallocPitch.
/// 
/// Below is an example that utilizes BufferPool with StackAllocator:
/// 
/// ```ignore
///    #include <opencv2/opencv.hpp>
/// 
///    using namespace cv;
///    using namespace cv::cuda
/// 
///    int main()
///    {
///        setBufferPoolUsage(true);                               // Tell OpenCV that we are going to utilize BufferPool
///        setBufferPoolConfig(getDevice(), 1024 * 1024 * 64, 2);  // Allocate 64 MB, 2 stacks (default is 10 MB, 5 stacks)
/// 
///        Stream stream1, stream2;                                // Each stream uses 1 stack
///        BufferPool pool1(stream1), pool2(stream2);
/// 
///        GpuMat d_src1 = pool1.getBuffer(4096, 4096, CV_8UC1);   // 16MB
///        GpuMat d_dst1 = pool1.getBuffer(4096, 4096, CV_8UC3);   // 48MB, pool1 is now full
/// 
///        GpuMat d_src2 = pool2.getBuffer(1024, 1024, CV_8UC1);   // 1MB
///        GpuMat d_dst2 = pool2.getBuffer(1024, 1024, CV_8UC3);   // 3MB
/// 
///        cvtColor(d_src1, d_dst1, CV_GRAY2BGR, 0, stream1);
///        cvtColor(d_src2, d_dst2, CV_GRAY2BGR, 0, stream2);
///    }
/// ```
/// 
/// 
/// If we allocate another GpuMat on pool1 in the above example, it will be carried out by
/// the DefaultAllocator since the stack for pool1 is full.
/// 
/// ```ignore
///    GpuMat d_add1 = pool1.getBuffer(1024, 1024, CV_8UC1);   // Stack for pool1 is full, memory is allocated with DefaultAllocator
/// ```
/// 
/// 
/// If a third stream is declared in the above example, allocating with #getBuffer
/// within that stream will also be carried out by the DefaultAllocator because we've run out of
/// stacks.
/// 
/// ```ignore
///    Stream stream3;                                         // Only 2 stacks were allocated, we've run out of stacks
///    BufferPool pool3(stream3);
///    GpuMat d_src3 = pool3.getBuffer(1024, 1024, CV_8UC1);   // Memory is allocated with DefaultAllocator
/// ```
/// 
/// 
/// @warning When utilizing StackAllocator, deallocation order is important.
/// 
/// Just like a stack, deallocation must be done in LIFO order. Below is an example of
/// erroneous usage that violates LIFO rule. If OpenCV is compiled in Debug mode, this
/// sample code will emit CV_Assert error.
/// 
/// ```ignore
///    int main()
///    {
///        setBufferPoolUsage(true);                               // Tell OpenCV that we are going to utilize BufferPool
///        Stream stream;                                          // A default size (10 MB) stack is allocated to this stream
///        BufferPool pool(stream);
/// 
///        GpuMat mat1 = pool.getBuffer(1024, 1024, CV_8UC1);      // Allocate mat1 (1MB)
///        GpuMat mat2 = pool.getBuffer(1024, 1024, CV_8UC1);      // Allocate mat2 (1MB)
/// 
///        mat1.release();                                         // erroneous usage : mat2 must be deallocated before mat1
///    }
/// ```
/// 
/// 
/// Since C++ local variables are destroyed in the reverse order of construction,
/// the code sample below satisfies the LIFO rule. Local GpuMat's are deallocated
/// and the corresponding memory is automatically returned to the pool for later usage.
/// 
/// ```ignore
///    int main()
///    {
///        setBufferPoolUsage(true);                               // Tell OpenCV that we are going to utilize BufferPool
///        setBufferPoolConfig(getDevice(), 1024 * 1024 * 64, 2);  // Allocate 64 MB, 2 stacks (default is 10 MB, 5 stacks)
/// 
///        Stream stream1, stream2;                                // Each stream uses 1 stack
///        BufferPool pool1(stream1), pool2(stream2);
/// 
///        for (int i = 0; i < 10; i++)
///        {
///            GpuMat d_src1 = pool1.getBuffer(4096, 4096, CV_8UC1);   // 16MB
///            GpuMat d_dst1 = pool1.getBuffer(4096, 4096, CV_8UC3);   // 48MB, pool1 is now full
/// 
///            GpuMat d_src2 = pool2.getBuffer(1024, 1024, CV_8UC1);   // 1MB
///            GpuMat d_dst2 = pool2.getBuffer(1024, 1024, CV_8UC3);   // 3MB
/// 
///            d_src1.setTo(Scalar(i), stream1);
///            d_src2.setTo(Scalar(i), stream2);
/// 
///            cvtColor(d_src1, d_dst1, CV_GRAY2BGR, 0, stream1);
///            cvtColor(d_src2, d_dst2, CV_GRAY2BGR, 0, stream2);
///                                                                     // The order of destruction of the local variables is:
///                                                                     //   d_dst2 => d_src2 => d_dst1 => d_src1
///                                                                     // LIFO rule is satisfied, this code runs without error
///        }
///    }
/// ```
/// 
pub struct BufferPool {
	ptr: *mut c_void
}

opencv_type_boxed! { BufferPool }

impl Drop for BufferPool {
	fn drop(&mut self) {
		extern "C" { fn cv_BufferPool_delete(instance: *mut c_void); }
		unsafe { cv_BufferPool_delete(self.as_raw_mut_BufferPool()) };
	}
}

impl BufferPool {
	#[inline] pub fn as_raw_BufferPool(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_BufferPool(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for BufferPool {}

impl core::BufferPoolTrait for BufferPool {
	#[inline] fn as_raw_BufferPool(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_BufferPool(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BufferPool {
	/// Gets the BufferPool for the given stream.
	pub fn new(stream: &mut core::Stream) -> Result<core::BufferPool> {
		unsafe { sys::cv_cuda_BufferPool_BufferPool_StreamR(stream.as_raw_mut_Stream()) }.into_result().map(|r| unsafe { core::BufferPool::opencv_from_extern(r) } )
	}
	
}

/// Class providing functionality for querying the specified GPU properties.
pub trait DeviceInfoTrait {
	fn as_raw_DeviceInfo(&self) -> *const c_void;
	fn as_raw_mut_DeviceInfo(&mut self) -> *mut c_void;

	/// Returns system index of the CUDA device starting with 0.
	fn device_id(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_deviceID_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// ASCII string identifying device
	fn name(&self) -> Result<String> {
		unsafe { sys::cv_cuda_DeviceInfo_name_const(self.as_raw_DeviceInfo()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// global memory available on device in bytes
	fn total_global_mem(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_DeviceInfo_totalGlobalMem_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// shared memory available per block in bytes
	fn shared_mem_per_block(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_DeviceInfo_sharedMemPerBlock_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// 32-bit registers available per block
	fn regs_per_block(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_regsPerBlock_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// warp size in threads
	fn warp_size(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_warpSize_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum pitch in bytes allowed by memory copies
	fn mem_pitch(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_DeviceInfo_memPitch_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum number of threads per block
	fn max_threads_per_block(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_maxThreadsPerBlock_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum size of each dimension of a block
	fn max_threads_dim(&self) -> Result<core::Vec3i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxThreadsDim_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum size of each dimension of a grid
	fn max_grid_size(&self) -> Result<core::Vec3i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxGridSize_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// clock frequency in kilohertz
	fn clock_rate(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_clockRate_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// constant memory available on device in bytes
	fn total_const_mem(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_DeviceInfo_totalConstMem_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// major compute capability
	fn major_version(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_majorVersion_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// minor compute capability
	fn minor_version(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_minorVersion_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// alignment requirement for textures
	fn texture_alignment(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_DeviceInfo_textureAlignment_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// pitch alignment requirement for texture references bound to pitched memory
	fn texture_pitch_alignment(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_DeviceInfo_texturePitchAlignment_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// number of multiprocessors on device
	fn multi_processor_count(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_multiProcessorCount_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// specified whether there is a run time limit on kernels
	fn kernel_exec_timeout_enabled(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_DeviceInfo_kernelExecTimeoutEnabled_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// device is integrated as opposed to discrete
	fn integrated(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_DeviceInfo_integrated_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// device can map host memory with cudaHostAlloc/cudaHostGetDevicePointer
	fn can_map_host_memory(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_DeviceInfo_canMapHostMemory_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// compute mode
	fn compute_mode(&self) -> Result<core::DeviceInfo_ComputeMode> {
		unsafe { sys::cv_cuda_DeviceInfo_computeMode_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 1D texture size
	fn max_texture1_d(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture1D_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 1D mipmapped texture size
	fn max_texture1_d_mipmap(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture1DMipmap_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum size for 1D textures bound to linear memory
	fn max_texture1_d_linear(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture1DLinear_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 2D texture dimensions
	fn max_texture_2d(&self) -> Result<core::Vec2i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture2D_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 2D mipmapped texture dimensions
	fn max_texture2_d_mipmap(&self) -> Result<core::Vec2i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture2DMipmap_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum dimensions (width, height, pitch) for 2D textures bound to pitched memory
	fn max_texture2_d_linear(&self) -> Result<core::Vec3i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture2DLinear_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 2D texture dimensions if texture gather operations have to be performed
	fn max_texture2_d_gather(&self) -> Result<core::Vec2i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture2DGather_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 3D texture dimensions
	fn max_texture_3d(&self) -> Result<core::Vec3i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture3D_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum Cubemap texture dimensions
	fn max_texture_cubemap(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTextureCubemap_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 1D layered texture dimensions
	fn max_texture1_d_layered(&self) -> Result<core::Vec2i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture1DLayered_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 2D layered texture dimensions
	fn max_texture2_d_layered(&self) -> Result<core::Vec3i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTexture2DLayered_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum Cubemap layered texture dimensions
	fn max_texture_cubemap_layered(&self) -> Result<core::Vec2i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxTextureCubemapLayered_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 1D surface size
	fn max_surface1_d(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_maxSurface1D_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 2D surface dimensions
	fn max_surface_2d(&self) -> Result<core::Vec2i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxSurface2D_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 3D surface dimensions
	fn max_surface_3d(&self) -> Result<core::Vec3i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxSurface3D_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 1D layered surface dimensions
	fn max_surface1_d_layered(&self) -> Result<core::Vec2i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxSurface1DLayered_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum 2D layered surface dimensions
	fn max_surface2_d_layered(&self) -> Result<core::Vec3i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxSurface2DLayered_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum Cubemap surface dimensions
	fn max_surface_cubemap(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_maxSurfaceCubemap_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum Cubemap layered surface dimensions
	fn max_surface_cubemap_layered(&self) -> Result<core::Vec2i> {
		unsafe { sys::cv_cuda_DeviceInfo_maxSurfaceCubemapLayered_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// alignment requirements for surfaces
	fn surface_alignment(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_DeviceInfo_surfaceAlignment_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// device can possibly execute multiple kernels concurrently
	fn concurrent_kernels(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_DeviceInfo_concurrentKernels_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// device has ECC support enabled
	fn ecc_enabled(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_DeviceInfo_ECCEnabled_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// PCI bus ID of the device
	fn pci_bus_id(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_pciBusID_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// PCI device ID of the device
	fn pci_device_id(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_pciDeviceID_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// PCI domain ID of the device
	fn pci_domain_id(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_pciDomainID_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// true if device is a Tesla device using TCC driver, false otherwise
	fn tcc_driver(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_DeviceInfo_tccDriver_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// number of asynchronous engines
	fn async_engine_count(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_asyncEngineCount_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// device shares a unified address space with the host
	fn unified_addressing(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_DeviceInfo_unifiedAddressing_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// peak memory clock frequency in kilohertz
	fn memory_clock_rate(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_memoryClockRate_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// global memory bus width in bits
	fn memory_bus_width(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_memoryBusWidth_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// size of L2 cache in bytes
	fn l2_cache_size(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_l2CacheSize_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// maximum resident threads per multiprocessor
	fn max_threads_per_multi_processor(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DeviceInfo_maxThreadsPerMultiProcessor_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// gets free and total device memory
	fn query_memory(&self, total_memory: &mut size_t, free_memory: &mut size_t) -> Result<()> {
		unsafe { sys::cv_cuda_DeviceInfo_queryMemory_const_size_tR_size_tR(self.as_raw_DeviceInfo(), total_memory, free_memory) }.into_result()
	}
	
	fn free_memory(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_DeviceInfo_freeMemory_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	fn total_memory(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_DeviceInfo_totalMemory_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
	/// Provides information on CUDA feature support.
	/// 
	/// ## Parameters
	/// * feature_set: Features to be checked. See cuda::FeatureSet.
	/// 
	/// This function returns true if the device has the specified CUDA feature. Otherwise, it returns false
	fn supports(&self, feature_set: core::FeatureSet) -> Result<bool> {
		unsafe { sys::cv_cuda_DeviceInfo_supports_const_FeatureSet(self.as_raw_DeviceInfo(), feature_set) }.into_result()
	}
	
	/// Checks the CUDA module and device compatibility.
	/// 
	/// This function returns true if the CUDA module can be run on the specified device. Otherwise, it
	/// returns false .
	fn is_compatible(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_DeviceInfo_isCompatible_const(self.as_raw_DeviceInfo()) }.into_result()
	}
	
}

/// Class providing functionality for querying the specified GPU properties.
pub struct DeviceInfo {
	ptr: *mut c_void
}

opencv_type_boxed! { DeviceInfo }

impl Drop for DeviceInfo {
	fn drop(&mut self) {
		extern "C" { fn cv_DeviceInfo_delete(instance: *mut c_void); }
		unsafe { cv_DeviceInfo_delete(self.as_raw_mut_DeviceInfo()) };
	}
}

impl DeviceInfo {
	#[inline] pub fn as_raw_DeviceInfo(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_DeviceInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for DeviceInfo {}

impl core::DeviceInfoTrait for DeviceInfo {
	#[inline] fn as_raw_DeviceInfo(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DeviceInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DeviceInfo {
	/// creates DeviceInfo object for the current GPU
	pub fn default() -> Result<core::DeviceInfo> {
		unsafe { sys::cv_cuda_DeviceInfo_DeviceInfo() }.into_result().map(|r| unsafe { core::DeviceInfo::opencv_from_extern(r) } )
	}
	
	/// The constructors.
	/// 
	/// ## Parameters
	/// * device_id: System index of the CUDA device starting with 0.
	/// 
	/// Constructs the DeviceInfo object for the specified device. If device_id parameter is missed, it
	/// constructs an object for the current device.
	pub fn new(device_id: i32) -> Result<core::DeviceInfo> {
		unsafe { sys::cv_cuda_DeviceInfo_DeviceInfo_int(device_id) }.into_result().map(|r| unsafe { core::DeviceInfo::opencv_from_extern(r) } )
	}
	
}

pub trait EventTrait {
	fn as_raw_Event(&self) -> *const c_void;
	fn as_raw_mut_Event(&mut self) -> *mut c_void;

	/// records an event
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn record(&mut self, stream: &mut core::Stream) -> Result<()> {
		unsafe { sys::cv_cuda_Event_record_StreamR(self.as_raw_mut_Event(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// queries an event's status
	fn query_if_complete(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_Event_queryIfComplete_const(self.as_raw_Event()) }.into_result()
	}
	
	/// waits for an event to complete
	fn wait_for_completion(&mut self) -> Result<()> {
		unsafe { sys::cv_cuda_Event_waitForCompletion(self.as_raw_mut_Event()) }.into_result()
	}
	
}

pub struct Event {
	ptr: *mut c_void
}

opencv_type_boxed! { Event }

impl Drop for Event {
	fn drop(&mut self) {
		extern "C" { fn cv_Event_delete(instance: *mut c_void); }
		unsafe { cv_Event_delete(self.as_raw_mut_Event()) };
	}
}

impl Event {
	#[inline] pub fn as_raw_Event(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Event(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Event {}

impl core::EventTrait for Event {
	#[inline] fn as_raw_Event(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Event(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Event {
	/// ## C++ default parameters
	/// * flags: Event::CreateFlags::DEFAULT
	pub fn new(flags: core::Event_CreateFlags) -> Result<core::Event> {
		unsafe { sys::cv_cuda_Event_Event_CreateFlags(flags) }.into_result().map(|r| unsafe { core::Event::opencv_from_extern(r) } )
	}
	
	/// computes the elapsed time between events
	pub fn elapsed_time(start: &core::Event, end: &core::Event) -> Result<f32> {
		unsafe { sys::cv_cuda_Event_elapsedTime_const_EventR_const_EventR(start.as_raw_Event(), end.as_raw_Event()) }.into_result()
	}
	
}

/// Base storage class for GPU memory with reference counting.
/// 
/// Its interface matches the Mat interface with the following limitations:
/// 
/// *   no arbitrary dimensions support (only 2D)
/// *   no functions that return references to their data (because references on GPU are not valid for
///    CPU)
/// *   no expression templates technique support
/// 
/// Beware that the latter limitation may lead to overloaded matrix operators that cause memory
/// allocations. The GpuMat class is convertible to cuda::PtrStepSz and cuda::PtrStep so it can be
/// passed directly to the kernel.
/// 
/// 
/// Note: In contrast with Mat, in most cases GpuMat::isContinuous() == false . This means that rows are
/// aligned to a size depending on the hardware. Single-row GpuMat is always a continuous matrix.
/// 
/// 
/// Note: You are not recommended to leave static or global GpuMat variables allocated, that is, to rely
/// on its destructor. The destruction order of such variables and CUDA context is undefined. GPU memory
/// release function returns error if the CUDA context has been destroyed before.
/// 
/// Some member functions are described as a "Blocking Call" while some are described as a
/// "Non-Blocking Call". Blocking functions are synchronous to host. It is guaranteed that the GPU
/// operation is finished when the function returns. However, non-blocking functions are asynchronous to
/// host. Those functions may return even if the GPU operation is not finished.
/// 
/// Compared to their blocking counterpart, non-blocking functions accept Stream as an additional
/// argument. If a non-default stream is passed, the GPU operation may overlap with operations in other
/// streams.
/// ## See also
/// Mat
pub trait GpuMatTrait {
	fn as_raw_GpuMat(&self) -> *const c_void;
	fn as_raw_mut_GpuMat(&mut self) -> *mut c_void;

	/// ! includes several bit-fields:
	/// - the magic signature
	/// - continuity flag
	/// - depth
	/// - number of channels
	fn flags(&self) -> i32 {
		unsafe { sys::cv_cuda_GpuMat_getPropFlags_const(self.as_raw_GpuMat()) }.into_result().expect("Infallible function failed: flags")
	}
	
	/// ! includes several bit-fields:
	/// - the magic signature
	/// - continuity flag
	/// - depth
	/// - number of channels
	fn set_flags(&mut self, val: i32) -> () {
		unsafe { sys::cv_cuda_GpuMat_setPropFlags_int(self.as_raw_mut_GpuMat(), val) }.into_result().expect("Infallible function failed: set_flags")
	}
	
	/// the number of rows and columns
	fn rows(&self) -> i32 {
		unsafe { sys::cv_cuda_GpuMat_getPropRows_const(self.as_raw_GpuMat()) }.into_result().expect("Infallible function failed: rows")
	}
	
	/// the number of rows and columns
	fn set_rows(&mut self, val: i32) -> () {
		unsafe { sys::cv_cuda_GpuMat_setPropRows_int(self.as_raw_mut_GpuMat(), val) }.into_result().expect("Infallible function failed: set_rows")
	}
	
	/// the number of rows and columns
	fn cols(&self) -> i32 {
		unsafe { sys::cv_cuda_GpuMat_getPropCols_const(self.as_raw_GpuMat()) }.into_result().expect("Infallible function failed: cols")
	}
	
	/// the number of rows and columns
	fn set_cols(&mut self, val: i32) -> () {
		unsafe { sys::cv_cuda_GpuMat_setPropCols_int(self.as_raw_mut_GpuMat(), val) }.into_result().expect("Infallible function failed: set_cols")
	}
	
	/// a distance between successive rows in bytes; includes the gap if any
	fn step(&self) -> size_t {
		unsafe { sys::cv_cuda_GpuMat_getPropStep_const(self.as_raw_GpuMat()) }.into_result().expect("Infallible function failed: step")
	}
	
	/// a distance between successive rows in bytes; includes the gap if any
	fn set_step(&mut self, val: size_t) -> () {
		unsafe { sys::cv_cuda_GpuMat_setPropStep_size_t(self.as_raw_mut_GpuMat(), val) }.into_result().expect("Infallible function failed: set_step")
	}
	
	/// pointer to the data
	fn data(&mut self) -> &mut u8 {
		unsafe { sys::cv_cuda_GpuMat_getPropData(self.as_raw_mut_GpuMat()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: data")
	}
	
	/// pointer to the data
	fn set_data(&mut self, val: &mut u8) -> () {
		unsafe { sys::cv_cuda_GpuMat_setPropData_unsigned_charX(self.as_raw_mut_GpuMat(), val) }.into_result().expect("Infallible function failed: set_data")
	}
	
	/// pointer to the reference counter;
	/// when GpuMat points to user-allocated data, the pointer is NULL
	fn refcount(&mut self) -> &mut i32 {
		unsafe { sys::cv_cuda_GpuMat_getPropRefcount(self.as_raw_mut_GpuMat()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: refcount")
	}
	
	/// pointer to the reference counter;
	/// when GpuMat points to user-allocated data, the pointer is NULL
	fn set_refcount(&mut self, val: &mut i32) -> () {
		unsafe { sys::cv_cuda_GpuMat_setPropRefcount_intX(self.as_raw_mut_GpuMat(), val) }.into_result().expect("Infallible function failed: set_refcount")
	}
	
	/// helper fields used in locateROI and adjustROI
	fn datastart(&mut self) -> &mut u8 {
		unsafe { sys::cv_cuda_GpuMat_getPropDatastart(self.as_raw_mut_GpuMat()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: datastart")
	}
	
	/// helper fields used in locateROI and adjustROI
	fn set_datastart(&mut self, val: &mut u8) -> () {
		unsafe { sys::cv_cuda_GpuMat_setPropDatastart_unsigned_charX(self.as_raw_mut_GpuMat(), val) }.into_result().expect("Infallible function failed: set_datastart")
	}
	
	fn dataend(&self) -> &u8 {
		unsafe { sys::cv_cuda_GpuMat_getPropDataend_const(self.as_raw_GpuMat()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: dataend")
	}
	
	/// allocator
	fn allocator(&mut self) -> types::AbstractRefMut<dyn core::GpuMat_Allocator> {
		unsafe { sys::cv_cuda_GpuMat_getPropAllocator(self.as_raw_mut_GpuMat()) }.into_result().map(|r| unsafe { types::AbstractRefMut::<dyn core::GpuMat_Allocator>::opencv_from_extern(r) } ).expect("Infallible function failed: allocator")
	}
	
	/// allocator
	unsafe fn set_allocator(&mut self, val: &mut dyn core::GpuMat_Allocator) -> () {
		{ sys::cv_cuda_GpuMat_setPropAllocator_AllocatorX(self.as_raw_mut_GpuMat(), val.as_raw_mut_GpuMat_Allocator()) }.into_result().expect("Infallible function failed: set_allocator")
	}
	
	/// allocates new GpuMat data unless the GpuMat already has specified size and type
	fn create(&mut self, rows: i32, cols: i32, typ: i32) -> Result<()> {
		unsafe { sys::cv_cuda_GpuMat_create_int_int_int(self.as_raw_mut_GpuMat(), rows, cols, typ) }.into_result()
	}
	
	fn create_1(&mut self, size: core::Size, typ: i32) -> Result<()> {
		unsafe { sys::cv_cuda_GpuMat_create_Size_int(self.as_raw_mut_GpuMat(), size.opencv_to_extern(), typ) }.into_result()
	}
	
	/// decreases reference counter, deallocate the data when reference counter reaches 0
	fn release(&mut self) -> Result<()> {
		unsafe { sys::cv_cuda_GpuMat_release(self.as_raw_mut_GpuMat()) }.into_result()
	}
	
	/// swaps with other smart pointer
	fn swap(&mut self, mat: &mut core::GpuMat) -> Result<()> {
		unsafe { sys::cv_cuda_GpuMat_swap_GpuMatR(self.as_raw_mut_GpuMat(), mat.as_raw_mut_GpuMat()) }.into_result()
	}
	
	/// Performs data upload to GpuMat (Blocking call)
	/// 
	/// This function copies data from host memory to device memory. As being a blocking call, it is
	/// guaranteed that the copy operation is finished when this function returns.
	fn upload(&mut self, arr: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(arr);
		unsafe { sys::cv_cuda_GpuMat_upload_const__InputArrayR(self.as_raw_mut_GpuMat(), arr.as_raw__InputArray()) }.into_result()
	}
	
	/// Performs data upload to GpuMat (Non-Blocking call)
	/// 
	/// This function copies data from host memory to device memory. As being a non-blocking call, this
	/// function may return even if the copy operation is not finished.
	/// 
	/// The copy operation may be overlapped with operations in other non-default streams if \p stream is
	/// not the default stream and \p dst is HostMem allocated with HostMem::PAGE_LOCKED option.
	fn upload_async(&mut self, arr: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(arr);
		unsafe { sys::cv_cuda_GpuMat_upload_const__InputArrayR_StreamR(self.as_raw_mut_GpuMat(), arr.as_raw__InputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Performs data download from GpuMat (Blocking call)
	/// 
	/// This function copies data from device memory to host memory. As being a blocking call, it is
	/// guaranteed that the copy operation is finished when this function returns.
	fn download(&self, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_GpuMat_download_const_const__OutputArrayR(self.as_raw_GpuMat(), dst.as_raw__OutputArray()) }.into_result()
	}
	
	/// Performs data download from GpuMat (Non-Blocking call)
	/// 
	/// This function copies data from device memory to host memory. As being a non-blocking call, this
	/// function may return even if the copy operation is not finished.
	/// 
	/// The copy operation may be overlapped with operations in other non-default streams if \p stream is
	/// not the default stream and \p dst is HostMem allocated with HostMem::PAGE_LOCKED option.
	fn download_async(&self, dst: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_GpuMat_download_const_const__OutputArrayR_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// returns deep copy of the GpuMat, i.e. the data is copied
	fn clone(&self) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_clone_const(self.as_raw_GpuMat()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// copies the GpuMat content to device memory (Blocking call)
	fn copy_to(&self, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_GpuMat_copyTo_const_const__OutputArrayR(self.as_raw_GpuMat(), dst.as_raw__OutputArray()) }.into_result()
	}
	
	/// copies the GpuMat content to device memory (Non-Blocking call)
	fn copy_to_1(&self, dst: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// copies those GpuMat elements to "m" that are marked with non-zero mask elements (Blocking call)
	fn copy_to_2(&self, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		output_array_arg!(dst);
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
	}
	
	/// copies those GpuMat elements to "m" that are marked with non-zero mask elements (Non-Blocking call)
	fn copy_to_3(&self, dst: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// sets some of the GpuMat elements to s (Blocking call)
	fn set_to(&mut self, s: core::Scalar) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_setTo_Scalar(self.as_raw_mut_GpuMat(), s.opencv_to_extern()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// sets some of the GpuMat elements to s (Non-Blocking call)
	fn set_to_1(&mut self, s: core::Scalar, stream: &mut core::Stream) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_setTo_Scalar_StreamR(self.as_raw_mut_GpuMat(), s.opencv_to_extern(), stream.as_raw_mut_Stream()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// sets some of the GpuMat elements to s, according to the mask (Blocking call)
	fn set_to_2(&mut self, s: core::Scalar, mask: &dyn core::ToInputArray) -> Result<core::GpuMat> {
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR(self.as_raw_mut_GpuMat(), s.opencv_to_extern(), mask.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// sets some of the GpuMat elements to s, according to the mask (Non-Blocking call)
	fn set_to_3(&mut self, s: core::Scalar, mask: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<core::GpuMat> {
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR_StreamR(self.as_raw_mut_GpuMat(), s.opencv_to_extern(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// converts GpuMat to another datatype (Blocking call)
	fn convert_to(&self, dst: &mut dyn core::ToOutputArray, rtype: i32) -> Result<()> {
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), rtype) }.into_result()
	}
	
	/// converts GpuMat to another datatype (Non-Blocking call)
	fn convert_to_1(&self, dst: &mut dyn core::ToOutputArray, rtype: i32, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), rtype, stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// converts GpuMat to another datatype with scaling (Blocking call)
	/// 
	/// ## C++ default parameters
	/// * beta: 0.0
	fn convert_to_2(&self, dst: &mut dyn core::ToOutputArray, rtype: i32, alpha: f64, beta: f64) -> Result<()> {
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), rtype, alpha, beta) }.into_result()
	}
	
	/// converts GpuMat to another datatype with scaling (Non-Blocking call)
	fn convert_to_3(&self, dst: &mut dyn core::ToOutputArray, rtype: i32, alpha: f64, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), rtype, alpha, stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// converts GpuMat to another datatype with scaling (Non-Blocking call)
	fn convert_to_4(&self, dst: &mut dyn core::ToOutputArray, rtype: i32, alpha: f64, beta: f64, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		unsafe { sys::cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double_StreamR(self.as_raw_GpuMat(), dst.as_raw__OutputArray(), rtype, alpha, beta, stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * typ: -1
	fn assign_to(&self, m: &mut core::GpuMat, typ: i32) -> Result<()> {
		unsafe { sys::cv_cuda_GpuMat_assignTo_const_GpuMatR_int(self.as_raw_GpuMat(), m.as_raw_mut_GpuMat(), typ) }.into_result()
	}
	
	/// returns pointer to y-th row
	/// 
	/// ## C++ default parameters
	/// * y: 0
	fn ptr(&mut self, y: i32) -> Result<&mut u8> {
		unsafe { sys::cv_cuda_GpuMat_ptr_int(self.as_raw_mut_GpuMat(), y) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// ## C++ default parameters
	/// * y: 0
	fn ptr_1(&self, y: i32) -> Result<&u8> {
		unsafe { sys::cv_cuda_GpuMat_ptr_const_int(self.as_raw_GpuMat(), y) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// returns a new GpuMat header for the specified row
	fn row(&self, y: i32) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_row_const_int(self.as_raw_GpuMat(), y) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// returns a new GpuMat header for the specified column
	fn col(&self, x: i32) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_col_const_int(self.as_raw_GpuMat(), x) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// ... for the specified row span
	fn row_range(&self, startrow: i32, endrow: i32) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_rowRange_const_int_int(self.as_raw_GpuMat(), startrow, endrow) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	fn row_range_1(&self, mut r: core::Range) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_rowRange_const_Range(self.as_raw_GpuMat(), r.as_raw_mut_Range()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// ... for the specified column span
	fn col_range(&self, startcol: i32, endcol: i32) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_colRange_const_int_int(self.as_raw_GpuMat(), startcol, endcol) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	fn col_range_1(&self, mut r: core::Range) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_colRange_const_Range(self.as_raw_GpuMat(), r.as_raw_mut_Range()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// creates alternative GpuMat header for the same data, with different
	/// number of channels and/or different number of rows
	/// 
	/// ## C++ default parameters
	/// * rows: 0
	fn reshape(&self, cn: i32, rows: i32) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_reshape_const_int_int(self.as_raw_GpuMat(), cn, rows) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// locates GpuMat header within a parent GpuMat
	fn locate_roi(&self, whole_size: &mut core::Size, ofs: &mut core::Point) -> Result<()> {
		unsafe { sys::cv_cuda_GpuMat_locateROI_const_SizeR_PointR(self.as_raw_GpuMat(), whole_size, ofs) }.into_result()
	}
	
	/// moves/resizes the current GpuMat ROI inside the parent GpuMat
	fn adjust_roi(&mut self, dtop: i32, dbottom: i32, dleft: i32, dright: i32) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_adjustROI_int_int_int_int(self.as_raw_mut_GpuMat(), dtop, dbottom, dleft, dright) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// returns true iff the GpuMat data is continuous
	/// (i.e. when there are no gaps between successive rows)
	fn is_continuous(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_GpuMat_isContinuous_const(self.as_raw_GpuMat()) }.into_result()
	}
	
	/// returns element size in bytes
	fn elem_size(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_GpuMat_elemSize_const(self.as_raw_GpuMat()) }.into_result()
	}
	
	/// returns the size of element channel in bytes
	fn elem_size1(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_GpuMat_elemSize1_const(self.as_raw_GpuMat()) }.into_result()
	}
	
	/// returns element type
	fn typ(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_GpuMat_type_const(self.as_raw_GpuMat()) }.into_result()
	}
	
	/// returns element type
	fn depth(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_GpuMat_depth_const(self.as_raw_GpuMat()) }.into_result()
	}
	
	/// returns number of channels
	fn channels(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_GpuMat_channels_const(self.as_raw_GpuMat()) }.into_result()
	}
	
	/// returns step/elemSize1()
	fn step1(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_GpuMat_step1_const(self.as_raw_GpuMat()) }.into_result()
	}
	
	/// returns GpuMat size : width == number of columns, height == number of rows
	fn size(&self) -> Result<core::Size> {
		unsafe { sys::cv_cuda_GpuMat_size_const(self.as_raw_GpuMat()) }.into_result()
	}
	
	/// returns true if GpuMat data is NULL
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_GpuMat_empty_const(self.as_raw_GpuMat()) }.into_result()
	}
	
	fn cuda_ptr(&self) -> Result<*mut c_void> {
		unsafe { sys::cv_cuda_GpuMat_cudaPtr_const(self.as_raw_GpuMat()) }.into_result()
	}
	
	/// internal use method: updates the continuity flag
	fn update_continuity_flag(&mut self) -> Result<()> {
		unsafe { sys::cv_cuda_GpuMat_updateContinuityFlag(self.as_raw_mut_GpuMat()) }.into_result()
	}
	
}

/// Base storage class for GPU memory with reference counting.
/// 
/// Its interface matches the Mat interface with the following limitations:
/// 
/// *   no arbitrary dimensions support (only 2D)
/// *   no functions that return references to their data (because references on GPU are not valid for
///    CPU)
/// *   no expression templates technique support
/// 
/// Beware that the latter limitation may lead to overloaded matrix operators that cause memory
/// allocations. The GpuMat class is convertible to cuda::PtrStepSz and cuda::PtrStep so it can be
/// passed directly to the kernel.
/// 
/// 
/// Note: In contrast with Mat, in most cases GpuMat::isContinuous() == false . This means that rows are
/// aligned to a size depending on the hardware. Single-row GpuMat is always a continuous matrix.
/// 
/// 
/// Note: You are not recommended to leave static or global GpuMat variables allocated, that is, to rely
/// on its destructor. The destruction order of such variables and CUDA context is undefined. GPU memory
/// release function returns error if the CUDA context has been destroyed before.
/// 
/// Some member functions are described as a "Blocking Call" while some are described as a
/// "Non-Blocking Call". Blocking functions are synchronous to host. It is guaranteed that the GPU
/// operation is finished when the function returns. However, non-blocking functions are asynchronous to
/// host. Those functions may return even if the GPU operation is not finished.
/// 
/// Compared to their blocking counterpart, non-blocking functions accept Stream as an additional
/// argument. If a non-default stream is passed, the GPU operation may overlap with operations in other
/// streams.
/// ## See also
/// Mat
pub struct GpuMat {
	ptr: *mut c_void
}

opencv_type_boxed! { GpuMat }

impl Drop for GpuMat {
	fn drop(&mut self) {
		extern "C" { fn cv_GpuMat_delete(instance: *mut c_void); }
		unsafe { cv_GpuMat_delete(self.as_raw_mut_GpuMat()) };
	}
}

impl GpuMat {
	#[inline] pub fn as_raw_GpuMat(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_GpuMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for GpuMat {}

impl core::GpuMatTrait for GpuMat {
	#[inline] fn as_raw_GpuMat(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_GpuMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GpuMat {
	/// default allocator
	pub fn default_allocator() -> Result<types::AbstractRefMut<'static, dyn core::GpuMat_Allocator>> {
		unsafe { sys::cv_cuda_GpuMat_defaultAllocator() }.into_result().map(|r| unsafe { types::AbstractRefMut::<'static, dyn core::GpuMat_Allocator>::opencv_from_extern(r) } )
	}
	
	pub unsafe fn set_default_allocator(allocator: &mut dyn core::GpuMat_Allocator) -> Result<()> {
		{ sys::cv_cuda_GpuMat_setDefaultAllocator_AllocatorX(allocator.as_raw_mut_GpuMat_Allocator()) }.into_result()
	}
	
	/// default constructor
	/// 
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	pub unsafe fn new(allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		{ sys::cv_cuda_GpuMat_GpuMat_AllocatorX(allocator.as_raw_mut_GpuMat_Allocator()) }.into_result().map(|r| { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// constructs GpuMat of the specified size and type
	/// 
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	pub unsafe fn new_rows_cols(rows: i32, cols: i32, typ: i32, allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		{ sys::cv_cuda_GpuMat_GpuMat_int_int_int_AllocatorX(rows, cols, typ, allocator.as_raw_mut_GpuMat_Allocator()) }.into_result().map(|r| { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	pub unsafe fn new_size(size: core::Size, typ: i32, allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		{ sys::cv_cuda_GpuMat_GpuMat_Size_int_AllocatorX(size.opencv_to_extern(), typ, allocator.as_raw_mut_GpuMat_Allocator()) }.into_result().map(|r| { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// constructs GpuMat and fills it with the specified value _s
	/// 
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	pub unsafe fn new_rows_cols_with_default(rows: i32, cols: i32, typ: i32, s: core::Scalar, allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		{ sys::cv_cuda_GpuMat_GpuMat_int_int_int_Scalar_AllocatorX(rows, cols, typ, s.opencv_to_extern(), allocator.as_raw_mut_GpuMat_Allocator()) }.into_result().map(|r| { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	pub unsafe fn new_size_with_default(size: core::Size, typ: i32, s: core::Scalar, allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		{ sys::cv_cuda_GpuMat_GpuMat_Size_int_Scalar_AllocatorX(size.opencv_to_extern(), typ, s.opencv_to_extern(), allocator.as_raw_mut_GpuMat_Allocator()) }.into_result().map(|r| { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// copy constructor
	pub fn copy(m: &core::GpuMat) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_GpuMat_const_GpuMatR(m.as_raw_GpuMat()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// constructor for GpuMat headers pointing to user-allocated data
	/// 
	/// ## C++ default parameters
	/// * step: Mat::AUTO_STEP
	pub unsafe fn new_rows_cols_with_data(rows: i32, cols: i32, typ: i32, data: *mut c_void, step: size_t) -> Result<core::GpuMat> {
		{ sys::cv_cuda_GpuMat_GpuMat_int_int_int_voidX_size_t(rows, cols, typ, data, step) }.into_result().map(|r| { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * step: Mat::AUTO_STEP
	pub unsafe fn new_size_with_data(size: core::Size, typ: i32, data: *mut c_void, step: size_t) -> Result<core::GpuMat> {
		{ sys::cv_cuda_GpuMat_GpuMat_Size_int_voidX_size_t(size.opencv_to_extern(), typ, data, step) }.into_result().map(|r| { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// creates a GpuMat header for a part of the bigger matrix
	pub fn rowscols(m: &core::GpuMat, mut row_range: core::Range, mut col_range: core::Range) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_GpuMat_const_GpuMatR_Range_Range(m.as_raw_GpuMat(), row_range.as_raw_mut_Range(), col_range.as_raw_mut_Range()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	pub fn roi(m: &core::GpuMat, roi: core::Rect) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_GpuMat_GpuMat_const_GpuMatR_Rect(m.as_raw_GpuMat(), roi.opencv_to_extern()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	/// builds GpuMat from host memory (Blocking call)
	/// 
	/// ## C++ default parameters
	/// * allocator: GpuMat::defaultAllocator()
	pub unsafe fn from_hostmem(arr: &dyn core::ToInputArray, allocator: &mut dyn core::GpuMat_Allocator) -> Result<core::GpuMat> {
		input_array_arg!(arr);
		{ sys::cv_cuda_GpuMat_GpuMat_const__InputArrayR_AllocatorX(arr.as_raw__InputArray(), allocator.as_raw_mut_GpuMat_Allocator()) }.into_result().map(|r| { core::GpuMat::opencv_from_extern(r) } )
	}
	
}

pub trait GpuMat_Allocator {
	fn as_raw_GpuMat_Allocator(&self) -> *const c_void;
	fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void;

	fn allocate(&mut self, mat: &mut core::GpuMat, rows: i32, cols: i32, elem_size: size_t) -> Result<bool> {
		unsafe { sys::cv_cuda_GpuMat_Allocator_allocate_GpuMatX_int_int_size_t(self.as_raw_mut_GpuMat_Allocator(), mat.as_raw_mut_GpuMat(), rows, cols, elem_size) }.into_result()
	}
	
	fn free(&mut self, mat: &mut core::GpuMat) -> Result<()> {
		unsafe { sys::cv_cuda_GpuMat_Allocator_free_GpuMatX(self.as_raw_mut_GpuMat_Allocator(), mat.as_raw_mut_GpuMat()) }.into_result()
	}
	
}

/// Class with reference counting wrapping special memory type allocation functions from CUDA.
/// 
/// Its interface is also Mat-like but with additional memory type parameters.
/// 
/// *   **PAGE_LOCKED** sets a page locked memory type used commonly for fast and asynchronous
///    uploading/downloading data from/to GPU.
/// *   **SHARED** specifies a zero copy memory allocation that enables mapping the host memory to GPU
///    address space, if supported.
/// *   **WRITE_COMBINED** sets the write combined buffer that is not cached by CPU. Such buffers are
///    used to supply GPU with data when GPU only reads it. The advantage is a better CPU cache
///    utilization.
/// 
/// 
/// Note: Allocation size of such memory types is usually limited. For more details, see *CUDA 2.2
/// Pinned Memory APIs* document or *CUDA C Programming Guide*.
pub trait HostMemTrait {
	fn as_raw_HostMem(&self) -> *const c_void;
	fn as_raw_mut_HostMem(&mut self) -> *mut c_void;

	fn flags(&self) -> i32 {
		unsafe { sys::cv_cuda_HostMem_getPropFlags_const(self.as_raw_HostMem()) }.into_result().expect("Infallible function failed: flags")
	}
	
	fn set_flags(&mut self, val: i32) -> () {
		unsafe { sys::cv_cuda_HostMem_setPropFlags_int(self.as_raw_mut_HostMem(), val) }.into_result().expect("Infallible function failed: set_flags")
	}
	
	fn rows(&self) -> i32 {
		unsafe { sys::cv_cuda_HostMem_getPropRows_const(self.as_raw_HostMem()) }.into_result().expect("Infallible function failed: rows")
	}
	
	fn set_rows(&mut self, val: i32) -> () {
		unsafe { sys::cv_cuda_HostMem_setPropRows_int(self.as_raw_mut_HostMem(), val) }.into_result().expect("Infallible function failed: set_rows")
	}
	
	fn cols(&self) -> i32 {
		unsafe { sys::cv_cuda_HostMem_getPropCols_const(self.as_raw_HostMem()) }.into_result().expect("Infallible function failed: cols")
	}
	
	fn set_cols(&mut self, val: i32) -> () {
		unsafe { sys::cv_cuda_HostMem_setPropCols_int(self.as_raw_mut_HostMem(), val) }.into_result().expect("Infallible function failed: set_cols")
	}
	
	fn step(&self) -> size_t {
		unsafe { sys::cv_cuda_HostMem_getPropStep_const(self.as_raw_HostMem()) }.into_result().expect("Infallible function failed: step")
	}
	
	fn set_step(&mut self, val: size_t) -> () {
		unsafe { sys::cv_cuda_HostMem_setPropStep_size_t(self.as_raw_mut_HostMem(), val) }.into_result().expect("Infallible function failed: set_step")
	}
	
	fn data(&mut self) -> &mut u8 {
		unsafe { sys::cv_cuda_HostMem_getPropData(self.as_raw_mut_HostMem()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: data")
	}
	
	fn set_data(&mut self, val: &mut u8) -> () {
		unsafe { sys::cv_cuda_HostMem_setPropData_unsigned_charX(self.as_raw_mut_HostMem(), val) }.into_result().expect("Infallible function failed: set_data")
	}
	
	fn refcount(&mut self) -> &mut i32 {
		unsafe { sys::cv_cuda_HostMem_getPropRefcount(self.as_raw_mut_HostMem()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: refcount")
	}
	
	fn set_refcount(&mut self, val: &mut i32) -> () {
		unsafe { sys::cv_cuda_HostMem_setPropRefcount_intX(self.as_raw_mut_HostMem(), val) }.into_result().expect("Infallible function failed: set_refcount")
	}
	
	fn datastart(&mut self) -> &mut u8 {
		unsafe { sys::cv_cuda_HostMem_getPropDatastart(self.as_raw_mut_HostMem()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: datastart")
	}
	
	fn set_datastart(&mut self, val: &mut u8) -> () {
		unsafe { sys::cv_cuda_HostMem_setPropDatastart_unsigned_charX(self.as_raw_mut_HostMem(), val) }.into_result().expect("Infallible function failed: set_datastart")
	}
	
	fn dataend(&self) -> &u8 {
		unsafe { sys::cv_cuda_HostMem_getPropDataend_const(self.as_raw_HostMem()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: dataend")
	}
	
	fn alloc_type(&self) -> core::HostMem_AllocType {
		unsafe { sys::cv_cuda_HostMem_getPropAlloc_type_const(self.as_raw_HostMem()) }.into_result().expect("Infallible function failed: alloc_type")
	}
	
	fn set_alloc_type(&mut self, val: core::HostMem_AllocType) -> () {
		unsafe { sys::cv_cuda_HostMem_setPropAlloc_type_AllocType(self.as_raw_mut_HostMem(), val) }.into_result().expect("Infallible function failed: set_alloc_type")
	}
	
	/// swaps with other smart pointer
	fn swap(&mut self, b: &mut core::HostMem) -> Result<()> {
		unsafe { sys::cv_cuda_HostMem_swap_HostMemR(self.as_raw_mut_HostMem(), b.as_raw_mut_HostMem()) }.into_result()
	}
	
	/// returns deep copy of the matrix, i.e. the data is copied
	fn clone(&self) -> Result<core::HostMem> {
		unsafe { sys::cv_cuda_HostMem_clone_const(self.as_raw_HostMem()) }.into_result().map(|r| unsafe { core::HostMem::opencv_from_extern(r) } )
	}
	
	/// allocates new matrix data unless the matrix already has specified size and type.
	fn create(&mut self, rows: i32, cols: i32, typ: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HostMem_create_int_int_int(self.as_raw_mut_HostMem(), rows, cols, typ) }.into_result()
	}
	
	fn create_1(&mut self, size: core::Size, typ: i32) -> Result<()> {
		unsafe { sys::cv_cuda_HostMem_create_Size_int(self.as_raw_mut_HostMem(), size.opencv_to_extern(), typ) }.into_result()
	}
	
	/// creates alternative HostMem header for the same data, with different
	/// number of channels and/or different number of rows
	/// 
	/// ## C++ default parameters
	/// * rows: 0
	fn reshape(&self, cn: i32, rows: i32) -> Result<core::HostMem> {
		unsafe { sys::cv_cuda_HostMem_reshape_const_int_int(self.as_raw_HostMem(), cn, rows) }.into_result().map(|r| unsafe { core::HostMem::opencv_from_extern(r) } )
	}
	
	/// decrements reference counter and released memory if needed.
	fn release(&mut self) -> Result<()> {
		unsafe { sys::cv_cuda_HostMem_release(self.as_raw_mut_HostMem()) }.into_result()
	}
	
	/// returns matrix header with disabled reference counting for HostMem data.
	fn create_mat_header(&self) -> Result<core::Mat> {
		unsafe { sys::cv_cuda_HostMem_createMatHeader_const(self.as_raw_HostMem()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Maps CPU memory to GPU address space and creates the cuda::GpuMat header without reference counting
	/// for it.
	/// 
	/// This can be done only if memory was allocated with the SHARED flag and if it is supported by the
	/// hardware. Laptops often share video and CPU memory, so address spaces can be mapped, which
	/// eliminates an extra copy.
	fn create_gpu_mat_header(&self) -> Result<core::GpuMat> {
		unsafe { sys::cv_cuda_HostMem_createGpuMatHeader_const(self.as_raw_HostMem()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } )
	}
	
	fn is_continuous(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_HostMem_isContinuous_const(self.as_raw_HostMem()) }.into_result()
	}
	
	fn elem_size(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_HostMem_elemSize_const(self.as_raw_HostMem()) }.into_result()
	}
	
	fn elem_size1(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_HostMem_elemSize1_const(self.as_raw_HostMem()) }.into_result()
	}
	
	fn typ(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HostMem_type_const(self.as_raw_HostMem()) }.into_result()
	}
	
	fn depth(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HostMem_depth_const(self.as_raw_HostMem()) }.into_result()
	}
	
	fn channels(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_HostMem_channels_const(self.as_raw_HostMem()) }.into_result()
	}
	
	fn step1(&self) -> Result<size_t> {
		unsafe { sys::cv_cuda_HostMem_step1_const(self.as_raw_HostMem()) }.into_result()
	}
	
	fn size(&self) -> Result<core::Size> {
		unsafe { sys::cv_cuda_HostMem_size_const(self.as_raw_HostMem()) }.into_result()
	}
	
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_HostMem_empty_const(self.as_raw_HostMem()) }.into_result()
	}
	
}

/// Class with reference counting wrapping special memory type allocation functions from CUDA.
/// 
/// Its interface is also Mat-like but with additional memory type parameters.
/// 
/// *   **PAGE_LOCKED** sets a page locked memory type used commonly for fast and asynchronous
///    uploading/downloading data from/to GPU.
/// *   **SHARED** specifies a zero copy memory allocation that enables mapping the host memory to GPU
///    address space, if supported.
/// *   **WRITE_COMBINED** sets the write combined buffer that is not cached by CPU. Such buffers are
///    used to supply GPU with data when GPU only reads it. The advantage is a better CPU cache
///    utilization.
/// 
/// 
/// Note: Allocation size of such memory types is usually limited. For more details, see *CUDA 2.2
/// Pinned Memory APIs* document or *CUDA C Programming Guide*.
pub struct HostMem {
	ptr: *mut c_void
}

opencv_type_boxed! { HostMem }

impl Drop for HostMem {
	fn drop(&mut self) {
		extern "C" { fn cv_HostMem_delete(instance: *mut c_void); }
		unsafe { cv_HostMem_delete(self.as_raw_mut_HostMem()) };
	}
}

impl HostMem {
	#[inline] pub fn as_raw_HostMem(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_HostMem(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for HostMem {}

impl core::HostMemTrait for HostMem {
	#[inline] fn as_raw_HostMem(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_HostMem(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl HostMem {
	/// ## C++ default parameters
	/// * alloc_type: HostMem::AllocType::PAGE_LOCKED
	pub fn new(alloc_type: core::HostMem_AllocType) -> Result<core::HostMem> {
		unsafe { sys::cv_cuda_HostMem_HostMem_AllocType(alloc_type) }.into_result().map(|r| unsafe { core::HostMem::opencv_from_extern(r) } )
	}
	
	pub fn copy(m: &core::HostMem) -> Result<core::HostMem> {
		unsafe { sys::cv_cuda_HostMem_HostMem_const_HostMemR(m.as_raw_HostMem()) }.into_result().map(|r| unsafe { core::HostMem::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * alloc_type: HostMem::AllocType::PAGE_LOCKED
	pub fn new_1(rows: i32, cols: i32, typ: i32, alloc_type: core::HostMem_AllocType) -> Result<core::HostMem> {
		unsafe { sys::cv_cuda_HostMem_HostMem_int_int_int_AllocType(rows, cols, typ, alloc_type) }.into_result().map(|r| unsafe { core::HostMem::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * alloc_type: HostMem::AllocType::PAGE_LOCKED
	pub fn new_2(size: core::Size, typ: i32, alloc_type: core::HostMem_AllocType) -> Result<core::HostMem> {
		unsafe { sys::cv_cuda_HostMem_HostMem_Size_int_AllocType(size.opencv_to_extern(), typ, alloc_type) }.into_result().map(|r| unsafe { core::HostMem::opencv_from_extern(r) } )
	}
	
	/// creates from host memory with coping data
	/// 
	/// ## C++ default parameters
	/// * alloc_type: HostMem::AllocType::PAGE_LOCKED
	pub fn new_3(arr: &dyn core::ToInputArray, alloc_type: core::HostMem_AllocType) -> Result<core::HostMem> {
		input_array_arg!(arr);
		unsafe { sys::cv_cuda_HostMem_HostMem_const__InputArrayR_AllocType(arr.as_raw__InputArray(), alloc_type) }.into_result().map(|r| unsafe { core::HostMem::opencv_from_extern(r) } )
	}
	
}

/// This class encapsulates a queue of asynchronous calls.
/// 
/// 
/// Note: Currently, you may face problems if an operation is enqueued twice with different data. Some
/// functions use the constant GPU memory, and next call may update the memory before the previous one
/// has been finished. But calling different operations asynchronously is safe because each operation
/// has its own constant buffer. Memory copy/upload/download/set operations to the buffers you hold are
/// also safe.
/// 
/// 
/// Note: The Stream class is not thread-safe. Please use different Stream objects for different CPU threads.
/// 
/// ```ignore
/// void thread1()
/// {
///    cv::cuda::Stream stream1;
///    cv::cuda::func1(..., stream1);
/// }
/// 
/// void thread2()
/// {
///    cv::cuda::Stream stream2;
///    cv::cuda::func2(..., stream2);
/// }
/// ```
/// 
/// 
/// 
/// Note: By default all CUDA routines are launched in Stream::Null() object, if the stream is not specified by user.
/// In multi-threading environment the stream objects must be passed explicitly (see previous note).
pub trait StreamTrait {
	fn as_raw_Stream(&self) -> *const c_void;
	fn as_raw_mut_Stream(&mut self) -> *mut c_void;

	/// Returns true if the current stream queue is finished. Otherwise, it returns false.
	fn query_if_complete(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_Stream_queryIfComplete_const(self.as_raw_Stream()) }.into_result()
	}
	
	/// Blocks the current CPU thread until all operations in the stream are complete.
	fn wait_for_completion(&mut self) -> Result<()> {
		unsafe { sys::cv_cuda_Stream_waitForCompletion(self.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Makes a compute stream wait on an event.
	fn wait_event(&mut self, event: &core::Event) -> Result<()> {
		unsafe { sys::cv_cuda_Stream_waitEvent_const_EventR(self.as_raw_mut_Stream(), event.as_raw_Event()) }.into_result()
	}
	
	/// Adds a callback to be called on the host after all currently enqueued items in the stream have
	/// completed.
	/// 
	/// 
	/// Note: Callbacks must not make any CUDA API calls. Callbacks must not perform any synchronization
	/// that may depend on outstanding device work or other callbacks that are not mandated to run earlier.
	/// Callbacks without a mandated order (in independent streams) execute in undefined order and may be
	/// serialized.
	fn enqueue_host_callback(&mut self, callback: core::Stream_StreamCallback) -> Result<()> {
		callback_arg!(callback_trampoline(status: i32, user_data: *mut c_void) -> () => user_data in callbacks => callback(status: i32) -> ());
		userdata_arg!(user_data in callbacks => callback);
		unsafe { sys::cv_cuda_Stream_enqueueHostCallback_StreamCallback_voidX(self.as_raw_mut_Stream(), callback_trampoline, user_data) }.into_result()
	}
	
	/// return Pointer to CUDA stream
	fn cuda_ptr(&self) -> Result<*mut c_void> {
		unsafe { sys::cv_cuda_Stream_cudaPtr_const(self.as_raw_Stream()) }.into_result()
	}
	
}

/// This class encapsulates a queue of asynchronous calls.
/// 
/// 
/// Note: Currently, you may face problems if an operation is enqueued twice with different data. Some
/// functions use the constant GPU memory, and next call may update the memory before the previous one
/// has been finished. But calling different operations asynchronously is safe because each operation
/// has its own constant buffer. Memory copy/upload/download/set operations to the buffers you hold are
/// also safe.
/// 
/// 
/// Note: The Stream class is not thread-safe. Please use different Stream objects for different CPU threads.
/// 
/// ```ignore
/// void thread1()
/// {
///    cv::cuda::Stream stream1;
///    cv::cuda::func1(..., stream1);
/// }
/// 
/// void thread2()
/// {
///    cv::cuda::Stream stream2;
///    cv::cuda::func2(..., stream2);
/// }
/// ```
/// 
/// 
/// 
/// Note: By default all CUDA routines are launched in Stream::Null() object, if the stream is not specified by user.
/// In multi-threading environment the stream objects must be passed explicitly (see previous note).
pub struct Stream {
	ptr: *mut c_void
}

opencv_type_boxed! { Stream }

impl Drop for Stream {
	fn drop(&mut self) {
		extern "C" { fn cv_Stream_delete(instance: *mut c_void); }
		unsafe { cv_Stream_delete(self.as_raw_mut_Stream()) };
	}
}

impl Stream {
	#[inline] pub fn as_raw_Stream(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Stream(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Stream {}

impl core::StreamTrait for Stream {
	#[inline] fn as_raw_Stream(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Stream(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Stream {
	/// creates a new asynchronous stream
	pub fn default() -> Result<core::Stream> {
		unsafe { sys::cv_cuda_Stream_Stream() }.into_result().map(|r| unsafe { core::Stream::opencv_from_extern(r) } )
	}
	
	/// creates a new asynchronous stream with custom allocator
	pub fn new(allocator: &core::Ptr::<dyn core::GpuMat_Allocator>) -> Result<core::Stream> {
		unsafe { sys::cv_cuda_Stream_Stream_const_Ptr_Allocator_R(allocator.as_raw_PtrOfGpuMat_Allocator()) }.into_result().map(|r| unsafe { core::Stream::opencv_from_extern(r) } )
	}
	
	/// return Stream object for default CUDA stream
	pub fn null() -> Result<core::Stream> {
		unsafe { sys::cv_cuda_Stream_Null() }.into_result().map(|r| unsafe { core::Stream::opencv_from_extern(r) } )
	}
	
}

/// Class providing a set of static methods to check what NVIDIA\* card architecture the CUDA module was
/// built for.
/// 
/// According to the CUDA C Programming Guide Version 3.2: "PTX code produced for some specific compute
/// capability can always be compiled to binary code of greater or equal compute capability".
pub trait TargetArchsTrait {
	fn as_raw_TargetArchs(&self) -> *const c_void;
	fn as_raw_mut_TargetArchs(&mut self) -> *mut c_void;

}

/// Class providing a set of static methods to check what NVIDIA\* card architecture the CUDA module was
/// built for.
/// 
/// According to the CUDA C Programming Guide Version 3.2: "PTX code produced for some specific compute
/// capability can always be compiled to binary code of greater or equal compute capability".
pub struct TargetArchs {
	ptr: *mut c_void
}

opencv_type_boxed! { TargetArchs }

impl Drop for TargetArchs {
	fn drop(&mut self) {
		extern "C" { fn cv_TargetArchs_delete(instance: *mut c_void); }
		unsafe { cv_TargetArchs_delete(self.as_raw_mut_TargetArchs()) };
	}
}

impl TargetArchs {
	#[inline] pub fn as_raw_TargetArchs(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_TargetArchs(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TargetArchs {}

impl core::TargetArchsTrait for TargetArchs {
	#[inline] fn as_raw_TargetArchs(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_TargetArchs(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TargetArchs {
	/// The following method checks whether the module was built with the support of the given feature:
	/// 
	/// ## Parameters
	/// * feature_set: Features to be checked. See :ocvcuda::FeatureSet.
	pub fn built_with(feature_set: core::FeatureSet) -> Result<bool> {
		unsafe { sys::cv_cuda_TargetArchs_builtWith_FeatureSet(feature_set) }.into_result()
	}
	
	/// There is a set of methods to check whether the module contains intermediate (PTX) or binary CUDA
	/// code for the given architecture(s):
	/// 
	/// ## Parameters
	/// * major: Major compute capability version.
	/// * minor: Minor compute capability version.
	pub fn has(major: i32, minor: i32) -> Result<bool> {
		unsafe { sys::cv_cuda_TargetArchs_has_int_int(major, minor) }.into_result()
	}
	
	pub fn has_ptx(major: i32, minor: i32) -> Result<bool> {
		unsafe { sys::cv_cuda_TargetArchs_hasPtx_int_int(major, minor) }.into_result()
	}
	
	pub fn has_bin(major: i32, minor: i32) -> Result<bool> {
		unsafe { sys::cv_cuda_TargetArchs_hasBin_int_int(major, minor) }.into_result()
	}
	
	pub fn has_equal_or_less_ptx(major: i32, minor: i32) -> Result<bool> {
		unsafe { sys::cv_cuda_TargetArchs_hasEqualOrLessPtx_int_int(major, minor) }.into_result()
	}
	
	pub fn has_equal_or_greater(major: i32, minor: i32) -> Result<bool> {
		unsafe { sys::cv_cuda_TargetArchs_hasEqualOrGreater_int_int(major, minor) }.into_result()
	}
	
	pub fn has_equal_or_greater_ptx(major: i32, minor: i32) -> Result<bool> {
		unsafe { sys::cv_cuda_TargetArchs_hasEqualOrGreaterPtx_int_int(major, minor) }.into_result()
	}
	
	pub fn has_equal_or_greater_bin(major: i32, minor: i32) -> Result<bool> {
		unsafe { sys::cv_cuda_TargetArchs_hasEqualOrGreaterBin_int_int(major, minor) }.into_result()
	}
	
}

pub trait Detail_CheckContextTrait {
	fn as_raw_Detail_CheckContext(&self) -> *const c_void;
	fn as_raw_mut_Detail_CheckContext(&mut self) -> *mut c_void;

	fn func(&self) -> String {
		unsafe { sys::cv_detail_CheckContext_getPropFunc_const(self.as_raw_Detail_CheckContext()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: func")
	}
	
	fn file(&self) -> String {
		unsafe { sys::cv_detail_CheckContext_getPropFile_const(self.as_raw_Detail_CheckContext()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: file")
	}
	
	fn line(&self) -> i32 {
		unsafe { sys::cv_detail_CheckContext_getPropLine_const(self.as_raw_Detail_CheckContext()) }.into_result().expect("Infallible function failed: line")
	}
	
	fn set_line(&mut self, val: i32) -> () {
		unsafe { sys::cv_detail_CheckContext_setPropLine_int(self.as_raw_mut_Detail_CheckContext(), val) }.into_result().expect("Infallible function failed: set_line")
	}
	
	fn test_op(&self) -> core::Detail_TestOp {
		unsafe { sys::cv_detail_CheckContext_getPropTestOp_const(self.as_raw_Detail_CheckContext()) }.into_result().expect("Infallible function failed: test_op")
	}
	
	fn set_test_op(&mut self, val: core::Detail_TestOp) -> () {
		unsafe { sys::cv_detail_CheckContext_setPropTestOp_TestOp(self.as_raw_mut_Detail_CheckContext(), val) }.into_result().expect("Infallible function failed: set_test_op")
	}
	
	fn message(&self) -> String {
		unsafe { sys::cv_detail_CheckContext_getPropMessage_const(self.as_raw_Detail_CheckContext()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: message")
	}
	
	fn p1_str(&self) -> String {
		unsafe { sys::cv_detail_CheckContext_getPropP1_str_const(self.as_raw_Detail_CheckContext()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: p1_str")
	}
	
	fn p2_str(&self) -> String {
		unsafe { sys::cv_detail_CheckContext_getPropP2_str_const(self.as_raw_Detail_CheckContext()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: p2_str")
	}
	
}

pub struct Detail_CheckContext {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_CheckContext }

impl Drop for Detail_CheckContext {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_CheckContext_delete(instance: *mut c_void); }
		unsafe { cv_Detail_CheckContext_delete(self.as_raw_mut_Detail_CheckContext()) };
	}
}

impl Detail_CheckContext {
	#[inline] pub fn as_raw_Detail_CheckContext(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_CheckContext(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_CheckContext {}

impl core::Detail_CheckContextTrait for Detail_CheckContext {
	#[inline] fn as_raw_Detail_CheckContext(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_CheckContext(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_CheckContext {
}

pub trait NodeDataTrait {
	fn as_raw_NodeData(&self) -> *const c_void;
	fn as_raw_mut_NodeData(&mut self) -> *mut c_void;

	fn m_fun_name(&self) -> String {
		unsafe { sys::cv_instr_NodeData_getPropM_funName_const(self.as_raw_NodeData()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: m_fun_name")
	}
	
	fn set_m_fun_name(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_instr_NodeData_setPropM_funName_String(self.as_raw_mut_NodeData(), val.opencv_to_extern_mut()) }.into_result().expect("Infallible function failed: set_m_fun_name")
	}
	
	fn m_instr_type(&self) -> core::TYPE {
		unsafe { sys::cv_instr_NodeData_getPropM_instrType_const(self.as_raw_NodeData()) }.into_result().expect("Infallible function failed: m_instr_type")
	}
	
	fn set_m_instr_type(&mut self, val: core::TYPE) -> () {
		unsafe { sys::cv_instr_NodeData_setPropM_instrType_TYPE(self.as_raw_mut_NodeData(), val) }.into_result().expect("Infallible function failed: set_m_instr_type")
	}
	
	fn m_impl_type(&self) -> core::IMPL {
		unsafe { sys::cv_instr_NodeData_getPropM_implType_const(self.as_raw_NodeData()) }.into_result().expect("Infallible function failed: m_impl_type")
	}
	
	fn set_m_impl_type(&mut self, val: core::IMPL) -> () {
		unsafe { sys::cv_instr_NodeData_setPropM_implType_IMPL(self.as_raw_mut_NodeData(), val) }.into_result().expect("Infallible function failed: set_m_impl_type")
	}
	
	fn m_file_name(&self) -> String {
		unsafe { sys::cv_instr_NodeData_getPropM_fileName_const(self.as_raw_NodeData()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: m_file_name")
	}
	
	fn m_line_num(&self) -> i32 {
		unsafe { sys::cv_instr_NodeData_getPropM_lineNum_const(self.as_raw_NodeData()) }.into_result().expect("Infallible function failed: m_line_num")
	}
	
	fn set_m_line_num(&mut self, val: i32) -> () {
		unsafe { sys::cv_instr_NodeData_setPropM_lineNum_int(self.as_raw_mut_NodeData(), val) }.into_result().expect("Infallible function failed: set_m_line_num")
	}
	
	fn m_ret_address(&mut self) -> *mut c_void {
		unsafe { sys::cv_instr_NodeData_getPropM_retAddress(self.as_raw_mut_NodeData()) }.into_result().expect("Infallible function failed: m_ret_address")
	}
	
	fn set_m_ret_address(&mut self, val: *mut c_void) -> () {
		unsafe { sys::cv_instr_NodeData_setPropM_retAddress_voidX(self.as_raw_mut_NodeData(), val) }.into_result().expect("Infallible function failed: set_m_ret_address")
	}
	
	fn m_always_expand(&self) -> bool {
		unsafe { sys::cv_instr_NodeData_getPropM_alwaysExpand_const(self.as_raw_NodeData()) }.into_result().expect("Infallible function failed: m_always_expand")
	}
	
	fn set_m_always_expand(&mut self, val: bool) -> () {
		unsafe { sys::cv_instr_NodeData_setPropM_alwaysExpand_bool(self.as_raw_mut_NodeData(), val) }.into_result().expect("Infallible function failed: set_m_always_expand")
	}
	
	fn m_fun_error(&self) -> bool {
		unsafe { sys::cv_instr_NodeData_getPropM_funError_const(self.as_raw_NodeData()) }.into_result().expect("Infallible function failed: m_fun_error")
	}
	
	fn set_m_fun_error(&mut self, val: bool) -> () {
		unsafe { sys::cv_instr_NodeData_setPropM_funError_bool(self.as_raw_mut_NodeData(), val) }.into_result().expect("Infallible function failed: set_m_fun_error")
	}
	
	fn m_counter(&self) -> i32 {
		unsafe { sys::cv_instr_NodeData_getPropM_counter_const(self.as_raw_NodeData()) }.into_result().expect("Infallible function failed: m_counter")
	}
	
	fn set_m_counter(&mut self, val: i32) -> () {
		unsafe { sys::cv_instr_NodeData_setPropM_counter_int(self.as_raw_mut_NodeData(), val) }.into_result().expect("Infallible function failed: set_m_counter")
	}
	
	fn m_ticks_total(&self) -> u64 {
		unsafe { sys::cv_instr_NodeData_getPropM_ticksTotal_const(self.as_raw_NodeData()) }.into_result().expect("Infallible function failed: m_ticks_total")
	}
	
	fn set_m_ticks_total(&mut self, val: u64) -> () {
		unsafe { sys::cv_instr_NodeData_setPropM_ticksTotal_uint64_t(self.as_raw_mut_NodeData(), val) }.into_result().expect("Infallible function failed: set_m_ticks_total")
	}
	
	fn m_threads(&self) -> i32 {
		unsafe { sys::cv_instr_NodeData_getPropM_threads_const(self.as_raw_NodeData()) }.into_result().expect("Infallible function failed: m_threads")
	}
	
	fn set_m_threads(&mut self, val: i32) -> () {
		unsafe { sys::cv_instr_NodeData_setPropM_threads_int(self.as_raw_mut_NodeData(), val) }.into_result().expect("Infallible function failed: set_m_threads")
	}
	
	fn get_total_ms(&self) -> Result<f64> {
		unsafe { sys::cv_instr_NodeData_getTotalMs_const(self.as_raw_NodeData()) }.into_result()
	}
	
	fn get_mean_ms(&self) -> Result<f64> {
		unsafe { sys::cv_instr_NodeData_getMeanMs_const(self.as_raw_NodeData()) }.into_result()
	}
	
}

pub struct NodeData {
	ptr: *mut c_void
}

opencv_type_boxed! { NodeData }

impl Drop for NodeData {
	fn drop(&mut self) {
		extern "C" { fn cv_NodeData_delete(instance: *mut c_void); }
		unsafe { cv_NodeData_delete(self.as_raw_mut_NodeData()) };
	}
}

impl NodeData {
	#[inline] pub fn as_raw_NodeData(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_NodeData(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for NodeData {}

impl core::NodeDataTrait for NodeData {
	#[inline] fn as_raw_NodeData(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_NodeData(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl NodeData {
	/// ## C++ default parameters
	/// * fun_name: 0
	/// * file_name: NULL
	/// * line_num: 0
	/// * ret_address: NULL
	/// * always_expand: false
	/// * instr_type: TYPE_GENERAL
	/// * impl_type: IMPL_PLAIN
	pub fn new(fun_name: &str, file_name: &str, line_num: i32, ret_address: *mut c_void, always_expand: bool, instr_type: core::TYPE, impl_type: core::IMPL) -> Result<core::NodeData> {
		extern_container_arg!(fun_name);
		extern_container_arg!(file_name);
		unsafe { sys::cv_instr_NodeData_NodeData_const_charX_const_charX_int_voidX_bool_TYPE_IMPL(fun_name.opencv_to_extern(), file_name.opencv_to_extern(), line_num, ret_address, always_expand, instr_type, impl_type) }.into_result().map(|r| unsafe { core::NodeData::opencv_from_extern(r) } )
	}
	
	pub fn copy_mut(ref_: &mut core::NodeData) -> Result<core::NodeData> {
		unsafe { sys::cv_instr_NodeData_NodeData_NodeDataR(ref_.as_raw_mut_NodeData()) }.into_result().map(|r| unsafe { core::NodeData::opencv_from_extern(r) } )
	}
	
}

pub trait WriteStructContextTrait {
	fn as_raw_WriteStructContext(&self) -> *const c_void;
	fn as_raw_mut_WriteStructContext(&mut self) -> *mut c_void;

}

pub struct WriteStructContext {
	ptr: *mut c_void
}

opencv_type_boxed! { WriteStructContext }

impl Drop for WriteStructContext {
	fn drop(&mut self) {
		extern "C" { fn cv_WriteStructContext_delete(instance: *mut c_void); }
		unsafe { cv_WriteStructContext_delete(self.as_raw_mut_WriteStructContext()) };
	}
}

impl WriteStructContext {
	#[inline] pub fn as_raw_WriteStructContext(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_WriteStructContext(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for WriteStructContext {}

impl core::WriteStructContextTrait for WriteStructContext {
	#[inline] fn as_raw_WriteStructContext(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WriteStructContext(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WriteStructContext {
	/// ## C++ default parameters
	/// * type_name: String()
	pub fn new(_fs: &mut core::FileStorage, name: &str, flags: i32, type_name: &str) -> Result<core::WriteStructContext> {
		extern_container_arg!(name);
		extern_container_arg!(type_name);
		unsafe { sys::cv_internal_WriteStructContext_WriteStructContext_FileStorageR_const_StringR_int_const_StringR(_fs.as_raw_mut_FileStorage(), name.opencv_to_extern(), flags, type_name.opencv_to_extern()) }.into_result().map(|r| unsafe { core::WriteStructContext::opencv_from_extern(r) } )
	}
	
}

pub trait ContextTrait {
	fn as_raw_Context(&self) -> *const c_void;
	fn as_raw_mut_Context(&mut self) -> *mut c_void;

	fn create(&mut self) -> Result<bool> {
		unsafe { sys::cv_ocl_Context_create(self.as_raw_mut_Context()) }.into_result()
	}
	
	fn create_with_type(&mut self, dtype: i32) -> Result<bool> {
		unsafe { sys::cv_ocl_Context_create_int(self.as_raw_mut_Context(), dtype) }.into_result()
	}
	
	fn ndevices(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Context_ndevices_const(self.as_raw_Context()) }.into_result()
	}
	
	fn device(&self, idx: size_t) -> Result<core::Device> {
		unsafe { sys::cv_ocl_Context_device_const_size_t(self.as_raw_Context(), idx) }.into_result().map(|r| unsafe { core::Device::opencv_from_extern(r) } )
	}
	
	fn get_prog(&mut self, prog: &core::ProgramSource, buildopt: &str, errmsg: &mut String) -> Result<core::Program> {
		extern_container_arg!(buildopt);
		string_arg_output_send!(via errmsg_via);
		let out = unsafe { sys::cv_ocl_Context_getProg_const_ProgramSourceR_const_StringR_StringR(self.as_raw_mut_Context(), prog.as_raw_ProgramSource(), buildopt.opencv_to_extern(), &mut errmsg_via) }.into_result().map(|r| unsafe { core::Program::opencv_from_extern(r) } );
		string_arg_output_receive!(out, errmsg_via => errmsg);
		out
	}
	
	fn unload_prog(&mut self, prog: &mut core::Program) -> Result<()> {
		unsafe { sys::cv_ocl_Context_unloadProg_ProgramR(self.as_raw_mut_Context(), prog.as_raw_mut_Program()) }.into_result()
	}
	
	fn ptr(&self) -> Result<*mut c_void> {
		unsafe { sys::cv_ocl_Context_ptr_const(self.as_raw_Context()) }.into_result()
	}
	
	fn use_svm(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Context_useSVM_const(self.as_raw_Context()) }.into_result()
	}
	
	fn set_use_svm(&mut self, enabled: bool) -> Result<()> {
		unsafe { sys::cv_ocl_Context_setUseSVM_bool(self.as_raw_mut_Context(), enabled) }.into_result()
	}
	
}

pub struct Context {
	ptr: *mut c_void
}

opencv_type_boxed! { Context }

impl Drop for Context {
	fn drop(&mut self) {
		extern "C" { fn cv_Context_delete(instance: *mut c_void); }
		unsafe { cv_Context_delete(self.as_raw_mut_Context()) };
	}
}

impl Context {
	#[inline] pub fn as_raw_Context(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Context(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Context {}

impl core::ContextTrait for Context {
	#[inline] fn as_raw_Context(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Context(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Context {
	pub fn default() -> Result<core::Context> {
		unsafe { sys::cv_ocl_Context_Context() }.into_result().map(|r| unsafe { core::Context::opencv_from_extern(r) } )
	}
	
	pub fn new_with_type(dtype: i32) -> Result<core::Context> {
		unsafe { sys::cv_ocl_Context_Context_int(dtype) }.into_result().map(|r| unsafe { core::Context::opencv_from_extern(r) } )
	}
	
	pub fn copy(c: &core::Context) -> Result<core::Context> {
		unsafe { sys::cv_ocl_Context_Context_const_ContextR(c.as_raw_Context()) }.into_result().map(|r| unsafe { core::Context::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * initialize: true
	pub fn get_default(initialize: bool) -> Result<core::Context> {
		unsafe { sys::cv_ocl_Context_getDefault_bool(initialize) }.into_result().map(|r| unsafe { core::Context::opencv_from_extern(r) } )
	}
	
}

pub trait DeviceTrait {
	fn as_raw_Device(&self) -> *const c_void;
	fn as_raw_mut_Device(&mut self) -> *mut c_void;

	fn set(&mut self, d: *mut c_void) -> Result<()> {
		unsafe { sys::cv_ocl_Device_set_voidX(self.as_raw_mut_Device(), d) }.into_result()
	}
	
	fn name(&self) -> Result<String> {
		unsafe { sys::cv_ocl_Device_name_const(self.as_raw_Device()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn extensions(&self) -> Result<String> {
		unsafe { sys::cv_ocl_Device_extensions_const(self.as_raw_Device()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn is_extension_supported(&self, extension_name: &str) -> Result<bool> {
		extern_container_arg!(extension_name);
		unsafe { sys::cv_ocl_Device_isExtensionSupported_const_const_StringR(self.as_raw_Device(), extension_name.opencv_to_extern()) }.into_result()
	}
	
	fn version(&self) -> Result<String> {
		unsafe { sys::cv_ocl_Device_version_const(self.as_raw_Device()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn vendor_name(&self) -> Result<String> {
		unsafe { sys::cv_ocl_Device_vendorName_const(self.as_raw_Device()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn opencl_c_version(&self) -> Result<String> {
		unsafe { sys::cv_ocl_Device_OpenCL_C_Version_const(self.as_raw_Device()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn opencl_version(&self) -> Result<String> {
		unsafe { sys::cv_ocl_Device_OpenCLVersion_const(self.as_raw_Device()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn device_version_major(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_deviceVersionMajor_const(self.as_raw_Device()) }.into_result()
	}
	
	fn device_version_minor(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_deviceVersionMinor_const(self.as_raw_Device()) }.into_result()
	}
	
	fn driver_version(&self) -> Result<String> {
		unsafe { sys::cv_ocl_Device_driverVersion_const(self.as_raw_Device()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn ptr(&self) -> Result<*mut c_void> {
		unsafe { sys::cv_ocl_Device_ptr_const(self.as_raw_Device()) }.into_result()
	}
	
	fn typ(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_type_const(self.as_raw_Device()) }.into_result()
	}
	
	fn address_bits(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_addressBits_const(self.as_raw_Device()) }.into_result()
	}
	
	fn available(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_available_const(self.as_raw_Device()) }.into_result()
	}
	
	fn compiler_available(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_compilerAvailable_const(self.as_raw_Device()) }.into_result()
	}
	
	fn linker_available(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_linkerAvailable_const(self.as_raw_Device()) }.into_result()
	}
	
	fn double_fp_config(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_doubleFPConfig_const(self.as_raw_Device()) }.into_result()
	}
	
	fn single_fp_config(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_singleFPConfig_const(self.as_raw_Device()) }.into_result()
	}
	
	fn half_fp_config(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_halfFPConfig_const(self.as_raw_Device()) }.into_result()
	}
	
	fn endian_little(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_endianLittle_const(self.as_raw_Device()) }.into_result()
	}
	
	fn error_correction_support(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_errorCorrectionSupport_const(self.as_raw_Device()) }.into_result()
	}
	
	fn execution_capabilities(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_executionCapabilities_const(self.as_raw_Device()) }.into_result()
	}
	
	fn global_mem_cache_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_globalMemCacheSize_const(self.as_raw_Device()) }.into_result()
	}
	
	fn global_mem_cache_type(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_globalMemCacheType_const(self.as_raw_Device()) }.into_result()
	}
	
	fn global_mem_cache_line_size(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_globalMemCacheLineSize_const(self.as_raw_Device()) }.into_result()
	}
	
	fn global_mem_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_globalMemSize_const(self.as_raw_Device()) }.into_result()
	}
	
	fn local_mem_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_localMemSize_const(self.as_raw_Device()) }.into_result()
	}
	
	fn local_mem_type(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_localMemType_const(self.as_raw_Device()) }.into_result()
	}
	
	fn host_unified_memory(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_hostUnifiedMemory_const(self.as_raw_Device()) }.into_result()
	}
	
	fn image_support(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_imageSupport_const(self.as_raw_Device()) }.into_result()
	}
	
	fn image_from_buffer_support(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_imageFromBufferSupport_const(self.as_raw_Device()) }.into_result()
	}
	
	fn image_pitch_alignment(&self) -> Result<u32> {
		unsafe { sys::cv_ocl_Device_imagePitchAlignment_const(self.as_raw_Device()) }.into_result()
	}
	
	fn image_base_address_alignment(&self) -> Result<u32> {
		unsafe { sys::cv_ocl_Device_imageBaseAddressAlignment_const(self.as_raw_Device()) }.into_result()
	}
	
	/// deprecated, use isExtensionSupported() method (probably with "cl_khr_subgroups" value)
	fn intel_subgroups_support(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_intelSubgroupsSupport_const(self.as_raw_Device()) }.into_result()
	}
	
	fn image2_d_max_width(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_image2DMaxWidth_const(self.as_raw_Device()) }.into_result()
	}
	
	fn image2_d_max_height(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_image2DMaxHeight_const(self.as_raw_Device()) }.into_result()
	}
	
	fn image3_d_max_width(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_image3DMaxWidth_const(self.as_raw_Device()) }.into_result()
	}
	
	fn image3_d_max_height(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_image3DMaxHeight_const(self.as_raw_Device()) }.into_result()
	}
	
	fn image3_d_max_depth(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_image3DMaxDepth_const(self.as_raw_Device()) }.into_result()
	}
	
	fn image_max_buffer_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_imageMaxBufferSize_const(self.as_raw_Device()) }.into_result()
	}
	
	fn image_max_array_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_imageMaxArraySize_const(self.as_raw_Device()) }.into_result()
	}
	
	fn vendor_id(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_vendorID_const(self.as_raw_Device()) }.into_result()
	}
	
	fn is_amd(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_isAMD_const(self.as_raw_Device()) }.into_result()
	}
	
	fn is_intel(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_isIntel_const(self.as_raw_Device()) }.into_result()
	}
	
	fn is_n_vidia(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Device_isNVidia_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_clock_frequency(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_maxClockFrequency_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_compute_units(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_maxComputeUnits_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_constant_args(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_maxConstantArgs_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_constant_buffer_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_maxConstantBufferSize_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_mem_alloc_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_maxMemAllocSize_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_parameter_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_maxParameterSize_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_read_image_args(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_maxReadImageArgs_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_write_image_args(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_maxWriteImageArgs_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_samplers(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_maxSamplers_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_work_group_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_maxWorkGroupSize_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_work_item_dims(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_maxWorkItemDims_const(self.as_raw_Device()) }.into_result()
	}
	
	fn max_work_item_sizes(&self, unnamed: &mut size_t) -> Result<()> {
		unsafe { sys::cv_ocl_Device_maxWorkItemSizes_const_size_tX(self.as_raw_Device(), unnamed) }.into_result()
	}
	
	fn mem_base_addr_align(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_memBaseAddrAlign_const(self.as_raw_Device()) }.into_result()
	}
	
	fn native_vector_width_char(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_nativeVectorWidthChar_const(self.as_raw_Device()) }.into_result()
	}
	
	fn native_vector_width_short(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_nativeVectorWidthShort_const(self.as_raw_Device()) }.into_result()
	}
	
	fn native_vector_width_int(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_nativeVectorWidthInt_const(self.as_raw_Device()) }.into_result()
	}
	
	fn native_vector_width_long(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_nativeVectorWidthLong_const(self.as_raw_Device()) }.into_result()
	}
	
	fn native_vector_width_float(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_nativeVectorWidthFloat_const(self.as_raw_Device()) }.into_result()
	}
	
	fn native_vector_width_double(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_nativeVectorWidthDouble_const(self.as_raw_Device()) }.into_result()
	}
	
	fn native_vector_width_half(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_nativeVectorWidthHalf_const(self.as_raw_Device()) }.into_result()
	}
	
	fn preferred_vector_width_char(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_preferredVectorWidthChar_const(self.as_raw_Device()) }.into_result()
	}
	
	fn preferred_vector_width_short(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_preferredVectorWidthShort_const(self.as_raw_Device()) }.into_result()
	}
	
	fn preferred_vector_width_int(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_preferredVectorWidthInt_const(self.as_raw_Device()) }.into_result()
	}
	
	fn preferred_vector_width_long(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_preferredVectorWidthLong_const(self.as_raw_Device()) }.into_result()
	}
	
	fn preferred_vector_width_float(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_preferredVectorWidthFloat_const(self.as_raw_Device()) }.into_result()
	}
	
	fn preferred_vector_width_double(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_preferredVectorWidthDouble_const(self.as_raw_Device()) }.into_result()
	}
	
	fn preferred_vector_width_half(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_Device_preferredVectorWidthHalf_const(self.as_raw_Device()) }.into_result()
	}
	
	fn printf_buffer_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_printfBufferSize_const(self.as_raw_Device()) }.into_result()
	}
	
	fn profiling_timer_resolution(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Device_profilingTimerResolution_const(self.as_raw_Device()) }.into_result()
	}
	
}

pub struct Device {
	ptr: *mut c_void
}

opencv_type_boxed! { Device }

impl Drop for Device {
	fn drop(&mut self) {
		extern "C" { fn cv_Device_delete(instance: *mut c_void); }
		unsafe { cv_Device_delete(self.as_raw_mut_Device()) };
	}
}

impl Device {
	#[inline] pub fn as_raw_Device(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Device(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Device {}

impl core::DeviceTrait for Device {
	#[inline] fn as_raw_Device(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Device(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Device {
	pub fn default() -> Result<core::Device> {
		unsafe { sys::cv_ocl_Device_Device() }.into_result().map(|r| unsafe { core::Device::opencv_from_extern(r) } )
	}
	
	pub fn new(d: *mut c_void) -> Result<core::Device> {
		unsafe { sys::cv_ocl_Device_Device_voidX(d) }.into_result().map(|r| unsafe { core::Device::opencv_from_extern(r) } )
	}
	
	pub fn copy(d: &core::Device) -> Result<core::Device> {
		unsafe { sys::cv_ocl_Device_Device_const_DeviceR(d.as_raw_Device()) }.into_result().map(|r| unsafe { core::Device::opencv_from_extern(r) } )
	}
	
	pub fn get_default() -> Result<core::Device> {
		unsafe { sys::cv_ocl_Device_getDefault() }.into_result().map(|r| unsafe { core::Device::opencv_from_extern(r) } )
	}
	
}

pub trait Image2DTrait {
	fn as_raw_Image2D(&self) -> *const c_void;
	fn as_raw_mut_Image2D(&mut self) -> *mut c_void;

	fn ptr(&self) -> Result<*mut c_void> {
		unsafe { sys::cv_ocl_Image2D_ptr_const(self.as_raw_Image2D()) }.into_result()
	}
	
}

pub struct Image2D {
	ptr: *mut c_void
}

opencv_type_boxed! { Image2D }

impl Drop for Image2D {
	fn drop(&mut self) {
		extern "C" { fn cv_Image2D_delete(instance: *mut c_void); }
		unsafe { cv_Image2D_delete(self.as_raw_mut_Image2D()) };
	}
}

impl Image2D {
	#[inline] pub fn as_raw_Image2D(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Image2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Image2D {}

impl core::Image2DTrait for Image2D {
	#[inline] fn as_raw_Image2D(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Image2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Image2D {
	pub fn default() -> Result<core::Image2D> {
		unsafe { sys::cv_ocl_Image2D_Image2D() }.into_result().map(|r| unsafe { core::Image2D::opencv_from_extern(r) } )
	}
	
	/// ## Parameters
	/// * src: UMat object from which to get image properties and data
	/// * norm: flag to enable the use of normalized channel data types
	/// * alias: flag indicating that the image should alias the src UMat. If true, changes to the
	///    image or src will be reflected in both objects.
	/// 
	/// ## C++ default parameters
	/// * norm: false
	/// * alias: false
	pub fn new(src: &core::UMat, norm: bool, alias: bool) -> Result<core::Image2D> {
		unsafe { sys::cv_ocl_Image2D_Image2D_const_UMatR_bool_bool(src.as_raw_UMat(), norm, alias) }.into_result().map(|r| unsafe { core::Image2D::opencv_from_extern(r) } )
	}
	
	pub fn copy(i: &core::Image2D) -> Result<core::Image2D> {
		unsafe { sys::cv_ocl_Image2D_Image2D_const_Image2DR(i.as_raw_Image2D()) }.into_result().map(|r| unsafe { core::Image2D::opencv_from_extern(r) } )
	}
	
	/// Indicates if creating an aliased image should succeed.
	/// Depends on the underlying platform and the dimensions of the UMat.
	pub fn can_create_alias(u: &core::UMat) -> Result<bool> {
		unsafe { sys::cv_ocl_Image2D_canCreateAlias_const_UMatR(u.as_raw_UMat()) }.into_result()
	}
	
	/// Indicates if the image format is supported.
	pub fn is_format_supported(depth: i32, cn: i32, norm: bool) -> Result<bool> {
		unsafe { sys::cv_ocl_Image2D_isFormatSupported_int_int_bool(depth, cn, norm) }.into_result()
	}
	
}

pub trait KernelTrait {
	fn as_raw_Kernel(&self) -> *const c_void;
	fn as_raw_mut_Kernel(&mut self) -> *mut c_void;

	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_ocl_Kernel_empty_const(self.as_raw_Kernel()) }.into_result()
	}
	
	fn create(&mut self, kname: &str, prog: &core::Program) -> Result<bool> {
		extern_container_arg!(kname);
		unsafe { sys::cv_ocl_Kernel_create_const_charX_const_ProgramR(self.as_raw_mut_Kernel(), kname.opencv_to_extern(), prog.as_raw_Program()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * errmsg: 0
	fn create_ext(&mut self, kname: &str, prog: &core::ProgramSource, buildopts: &str, errmsg: &mut String) -> Result<bool> {
		extern_container_arg!(kname);
		extern_container_arg!(buildopts);
		string_arg_output_send!(via errmsg_via);
		let out = unsafe { sys::cv_ocl_Kernel_create_const_charX_const_ProgramSourceR_const_StringR_StringX(self.as_raw_mut_Kernel(), kname.opencv_to_extern(), prog.as_raw_ProgramSource(), buildopts.opencv_to_extern(), &mut errmsg_via) }.into_result();
		string_arg_output_receive!(out, errmsg_via => errmsg);
		out
	}
	
	fn set(&mut self, i: i32, value: *const c_void, sz: size_t) -> Result<i32> {
		unsafe { sys::cv_ocl_Kernel_set_int_const_voidX_size_t(self.as_raw_mut_Kernel(), i, value, sz) }.into_result()
	}
	
	fn set_1(&mut self, i: i32, image_2d: &core::Image2D) -> Result<i32> {
		unsafe { sys::cv_ocl_Kernel_set_int_const_Image2DR(self.as_raw_mut_Kernel(), i, image_2d.as_raw_Image2D()) }.into_result()
	}
	
	fn set_umat(&mut self, i: i32, m: &core::UMat) -> Result<i32> {
		unsafe { sys::cv_ocl_Kernel_set_int_const_UMatR(self.as_raw_mut_Kernel(), i, m.as_raw_UMat()) }.into_result()
	}
	
	fn set_kernel_arg(&mut self, i: i32, arg: &core::KernelArg) -> Result<i32> {
		unsafe { sys::cv_ocl_Kernel_set_int_const_KernelArgR(self.as_raw_mut_Kernel(), i, arg.as_raw_KernelArg()) }.into_result()
	}
	
	/// Run the OpenCL kernel.
	/// ## Parameters
	/// * dims: the work problem dimensions. It is the length of globalsize and localsize. It can be either 1, 2 or 3.
	/// * globalsize: work items for each dimension. It is not the final globalsize passed to
	///   OpenCL. Each dimension will be adjusted to the nearest integer divisible by the corresponding
	///   value in localsize. If localsize is NULL, it will still be adjusted depending on dims. The
	///   adjusted values are greater than or equal to the original values.
	/// * localsize: work-group size for each dimension.
	/// * sync: specify whether to wait for OpenCL computation to finish before return.
	/// * q: command queue
	/// 
	/// ## C++ default parameters
	/// * q: Queue()
	fn run(&mut self, dims: i32, globalsize: &mut [size_t], localsize: &mut [size_t], sync: bool, q: &core::Queue) -> Result<bool> {
		unsafe { sys::cv_ocl_Kernel_run_int_size_tX_size_tX_bool_const_QueueR(self.as_raw_mut_Kernel(), dims, globalsize.as_mut_ptr(), localsize.as_mut_ptr(), sync, q.as_raw_Queue()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * q: Queue()
	fn run_task(&mut self, sync: bool, q: &core::Queue) -> Result<bool> {
		unsafe { sys::cv_ocl_Kernel_runTask_bool_const_QueueR(self.as_raw_mut_Kernel(), sync, q.as_raw_Queue()) }.into_result()
	}
	
	/// Similar to synchronized run() call with returning of kernel execution time
	/// Separate OpenCL command queue may be used (with CL_QUEUE_PROFILING_ENABLE)
	/// ## Returns
	/// Execution time in nanoseconds or negative number on error
	/// 
	/// ## C++ default parameters
	/// * q: Queue()
	fn run_profiling(&mut self, dims: i32, globalsize: &mut [size_t], localsize: &mut [size_t], q: &core::Queue) -> Result<i64> {
		unsafe { sys::cv_ocl_Kernel_runProfiling_int_size_tX_size_tX_const_QueueR(self.as_raw_mut_Kernel(), dims, globalsize.as_mut_ptr(), localsize.as_mut_ptr(), q.as_raw_Queue()) }.into_result()
	}
	
	fn work_group_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Kernel_workGroupSize_const(self.as_raw_Kernel()) }.into_result()
	}
	
	fn prefered_work_group_size_multiple(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Kernel_preferedWorkGroupSizeMultiple_const(self.as_raw_Kernel()) }.into_result()
	}
	
	fn compile_work_group_size(&self, wsz: &mut [size_t]) -> Result<bool> {
		unsafe { sys::cv_ocl_Kernel_compileWorkGroupSize_const_size_tX(self.as_raw_Kernel(), wsz.as_mut_ptr()) }.into_result()
	}
	
	fn local_mem_size(&self) -> Result<size_t> {
		unsafe { sys::cv_ocl_Kernel_localMemSize_const(self.as_raw_Kernel()) }.into_result()
	}
	
	fn ptr(&self) -> Result<*mut c_void> {
		unsafe { sys::cv_ocl_Kernel_ptr_const(self.as_raw_Kernel()) }.into_result()
	}
	
}

pub struct Kernel {
	ptr: *mut c_void
}

opencv_type_boxed! { Kernel }

impl Drop for Kernel {
	fn drop(&mut self) {
		extern "C" { fn cv_Kernel_delete(instance: *mut c_void); }
		unsafe { cv_Kernel_delete(self.as_raw_mut_Kernel()) };
	}
}

impl Kernel {
	#[inline] pub fn as_raw_Kernel(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Kernel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Kernel {}

impl core::KernelTrait for Kernel {
	#[inline] fn as_raw_Kernel(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Kernel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Kernel {
	pub fn default() -> Result<core::Kernel> {
		unsafe { sys::cv_ocl_Kernel_Kernel() }.into_result().map(|r| unsafe { core::Kernel::opencv_from_extern(r) } )
	}
	
	pub fn new(kname: &str, prog: &core::Program) -> Result<core::Kernel> {
		extern_container_arg!(kname);
		unsafe { sys::cv_ocl_Kernel_Kernel_const_charX_const_ProgramR(kname.opencv_to_extern(), prog.as_raw_Program()) }.into_result().map(|r| unsafe { core::Kernel::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * buildopts: String()
	/// * errmsg: 0
	pub fn new_1(kname: &str, prog: &core::ProgramSource, buildopts: &str, errmsg: &mut String) -> Result<core::Kernel> {
		extern_container_arg!(kname);
		extern_container_arg!(buildopts);
		string_arg_output_send!(via errmsg_via);
		let out = unsafe { sys::cv_ocl_Kernel_Kernel_const_charX_const_ProgramSourceR_const_StringR_StringX(kname.opencv_to_extern(), prog.as_raw_ProgramSource(), buildopts.opencv_to_extern(), &mut errmsg_via) }.into_result().map(|r| unsafe { core::Kernel::opencv_from_extern(r) } );
		string_arg_output_receive!(out, errmsg_via => errmsg);
		out
	}
	
	pub fn copy(k: &core::Kernel) -> Result<core::Kernel> {
		unsafe { sys::cv_ocl_Kernel_Kernel_const_KernelR(k.as_raw_Kernel()) }.into_result().map(|r| unsafe { core::Kernel::opencv_from_extern(r) } )
	}
	
}

pub trait KernelArgTrait {
	fn as_raw_KernelArg(&self) -> *const c_void;
	fn as_raw_mut_KernelArg(&mut self) -> *mut c_void;

	fn flags(&self) -> i32 {
		unsafe { sys::cv_ocl_KernelArg_getPropFlags_const(self.as_raw_KernelArg()) }.into_result().expect("Infallible function failed: flags")
	}
	
	fn set_flags(&mut self, val: i32) -> () {
		unsafe { sys::cv_ocl_KernelArg_setPropFlags_int(self.as_raw_mut_KernelArg(), val) }.into_result().expect("Infallible function failed: set_flags")
	}
	
	fn m(&mut self) -> core::UMat {
		unsafe { sys::cv_ocl_KernelArg_getPropM(self.as_raw_mut_KernelArg()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } ).expect("Infallible function failed: m")
	}
	
	fn set_m(&mut self, val: &mut core::UMat) -> () {
		unsafe { sys::cv_ocl_KernelArg_setPropM_UMatX(self.as_raw_mut_KernelArg(), val.as_raw_mut_UMat()) }.into_result().expect("Infallible function failed: set_m")
	}
	
	fn obj(&self) -> *const c_void {
		unsafe { sys::cv_ocl_KernelArg_getPropObj_const(self.as_raw_KernelArg()) }.into_result().expect("Infallible function failed: obj")
	}
	
	fn sz(&self) -> size_t {
		unsafe { sys::cv_ocl_KernelArg_getPropSz_const(self.as_raw_KernelArg()) }.into_result().expect("Infallible function failed: sz")
	}
	
	fn set_sz(&mut self, val: size_t) -> () {
		unsafe { sys::cv_ocl_KernelArg_setPropSz_size_t(self.as_raw_mut_KernelArg(), val) }.into_result().expect("Infallible function failed: set_sz")
	}
	
	fn wscale(&self) -> i32 {
		unsafe { sys::cv_ocl_KernelArg_getPropWscale_const(self.as_raw_KernelArg()) }.into_result().expect("Infallible function failed: wscale")
	}
	
	fn set_wscale(&mut self, val: i32) -> () {
		unsafe { sys::cv_ocl_KernelArg_setPropWscale_int(self.as_raw_mut_KernelArg(), val) }.into_result().expect("Infallible function failed: set_wscale")
	}
	
	fn iwscale(&self) -> i32 {
		unsafe { sys::cv_ocl_KernelArg_getPropIwscale_const(self.as_raw_KernelArg()) }.into_result().expect("Infallible function failed: iwscale")
	}
	
	fn set_iwscale(&mut self, val: i32) -> () {
		unsafe { sys::cv_ocl_KernelArg_setPropIwscale_int(self.as_raw_mut_KernelArg(), val) }.into_result().expect("Infallible function failed: set_iwscale")
	}
	
}

pub struct KernelArg {
	ptr: *mut c_void
}

opencv_type_boxed! { KernelArg }

impl Drop for KernelArg {
	fn drop(&mut self) {
		extern "C" { fn cv_KernelArg_delete(instance: *mut c_void); }
		unsafe { cv_KernelArg_delete(self.as_raw_mut_KernelArg()) };
	}
}

impl KernelArg {
	#[inline] pub fn as_raw_KernelArg(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_KernelArg(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for KernelArg {}

impl core::KernelArgTrait for KernelArg {
	#[inline] fn as_raw_KernelArg(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_KernelArg(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KernelArg {
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	/// * _obj: 0
	/// * _sz: 0
	pub fn new(_flags: i32, _m: &mut core::UMat, wscale: i32, iwscale: i32, _obj: *const c_void, _sz: size_t) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_KernelArg_int_UMatX_int_int_const_voidX_size_t(_flags, _m.as_raw_mut_UMat(), wscale, iwscale, _obj, _sz) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	pub fn default() -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_KernelArg() }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	pub fn local(local_mem_size: size_t) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_Local_size_t(local_mem_size) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	pub fn ptr_write_only(m: &core::UMat) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_PtrWriteOnly_const_UMatR(m.as_raw_UMat()) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	pub fn ptr_read_only(m: &core::UMat) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_PtrReadOnly_const_UMatR(m.as_raw_UMat()) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	pub fn ptr_read_write(m: &core::UMat) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_PtrReadWrite_const_UMatR(m.as_raw_UMat()) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	pub fn read_write(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_ReadWrite_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	pub fn read_write_no_size(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_ReadWriteNoSize_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	pub fn read_only(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_ReadOnly_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	pub fn write_only(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_WriteOnly_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	pub fn read_only_no_size(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_ReadOnlyNoSize_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * wscale: 1
	/// * iwscale: 1
	pub fn write_only_no_size(m: &core::UMat, wscale: i32, iwscale: i32) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_WriteOnlyNoSize_const_UMatR_int_int(m.as_raw_UMat(), wscale, iwscale) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
	pub fn constant(m: &core::Mat) -> Result<core::KernelArg> {
		unsafe { sys::cv_ocl_KernelArg_Constant_const_MatR(m.as_raw_Mat()) }.into_result().map(|r| unsafe { core::KernelArg::opencv_from_extern(r) } )
	}
	
}

pub trait PlatformTrait {
	fn as_raw_Platform(&self) -> *const c_void;
	fn as_raw_mut_Platform(&mut self) -> *mut c_void;

	fn ptr(&self) -> Result<*mut c_void> {
		unsafe { sys::cv_ocl_Platform_ptr_const(self.as_raw_Platform()) }.into_result()
	}
	
}

pub struct Platform {
	ptr: *mut c_void
}

opencv_type_boxed! { Platform }

impl Drop for Platform {
	fn drop(&mut self) {
		extern "C" { fn cv_Platform_delete(instance: *mut c_void); }
		unsafe { cv_Platform_delete(self.as_raw_mut_Platform()) };
	}
}

impl Platform {
	#[inline] pub fn as_raw_Platform(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Platform(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Platform {}

impl core::PlatformTrait for Platform {
	#[inline] fn as_raw_Platform(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Platform(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Platform {
	pub fn default() -> Result<core::Platform> {
		unsafe { sys::cv_ocl_Platform_Platform() }.into_result().map(|r| unsafe { core::Platform::opencv_from_extern(r) } )
	}
	
	pub fn copy(p: &core::Platform) -> Result<core::Platform> {
		unsafe { sys::cv_ocl_Platform_Platform_const_PlatformR(p.as_raw_Platform()) }.into_result().map(|r| unsafe { core::Platform::opencv_from_extern(r) } )
	}
	
	pub fn get_default() -> Result<core::Platform> {
		unsafe { sys::cv_ocl_Platform_getDefault() }.into_result().map(|r| unsafe { core::Platform::opencv_from_extern(r) } )
	}
	
}

pub trait PlatformInfoTrait {
	fn as_raw_PlatformInfo(&self) -> *const c_void;
	fn as_raw_mut_PlatformInfo(&mut self) -> *mut c_void;

	fn name(&self) -> Result<String> {
		unsafe { sys::cv_ocl_PlatformInfo_name_const(self.as_raw_PlatformInfo()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn vendor(&self) -> Result<String> {
		unsafe { sys::cv_ocl_PlatformInfo_vendor_const(self.as_raw_PlatformInfo()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn version(&self) -> Result<String> {
		unsafe { sys::cv_ocl_PlatformInfo_version_const(self.as_raw_PlatformInfo()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn device_number(&self) -> Result<i32> {
		unsafe { sys::cv_ocl_PlatformInfo_deviceNumber_const(self.as_raw_PlatformInfo()) }.into_result()
	}
	
	fn get_device(&self, device: &mut core::Device, d: i32) -> Result<()> {
		unsafe { sys::cv_ocl_PlatformInfo_getDevice_const_DeviceR_int(self.as_raw_PlatformInfo(), device.as_raw_mut_Device(), d) }.into_result()
	}
	
}

pub struct PlatformInfo {
	ptr: *mut c_void
}

opencv_type_boxed! { PlatformInfo }

impl Drop for PlatformInfo {
	fn drop(&mut self) {
		extern "C" { fn cv_PlatformInfo_delete(instance: *mut c_void); }
		unsafe { cv_PlatformInfo_delete(self.as_raw_mut_PlatformInfo()) };
	}
}

impl PlatformInfo {
	#[inline] pub fn as_raw_PlatformInfo(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PlatformInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for PlatformInfo {}

impl core::PlatformInfoTrait for PlatformInfo {
	#[inline] fn as_raw_PlatformInfo(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_PlatformInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PlatformInfo {
	pub fn default() -> Result<core::PlatformInfo> {
		unsafe { sys::cv_ocl_PlatformInfo_PlatformInfo() }.into_result().map(|r| unsafe { core::PlatformInfo::opencv_from_extern(r) } )
	}
	
	pub fn new(id: *mut c_void) -> Result<core::PlatformInfo> {
		unsafe { sys::cv_ocl_PlatformInfo_PlatformInfo_voidX(id) }.into_result().map(|r| unsafe { core::PlatformInfo::opencv_from_extern(r) } )
	}
	
	pub fn copy(i: &core::PlatformInfo) -> Result<core::PlatformInfo> {
		unsafe { sys::cv_ocl_PlatformInfo_PlatformInfo_const_PlatformInfoR(i.as_raw_PlatformInfo()) }.into_result().map(|r| unsafe { core::PlatformInfo::opencv_from_extern(r) } )
	}
	
}

pub trait ProgramTrait {
	fn as_raw_Program(&self) -> *const c_void;
	fn as_raw_mut_Program(&mut self) -> *mut c_void;

	fn create(&mut self, src: &core::ProgramSource, buildflags: &str, errmsg: &mut String) -> Result<bool> {
		extern_container_arg!(buildflags);
		string_arg_output_send!(via errmsg_via);
		let out = unsafe { sys::cv_ocl_Program_create_const_ProgramSourceR_const_StringR_StringR(self.as_raw_mut_Program(), src.as_raw_ProgramSource(), buildflags.opencv_to_extern(), &mut errmsg_via) }.into_result();
		string_arg_output_receive!(out, errmsg_via => errmsg);
		out
	}
	
	fn ptr(&self) -> Result<*mut c_void> {
		unsafe { sys::cv_ocl_Program_ptr_const(self.as_raw_Program()) }.into_result()
	}
	
	/// Query device-specific program binary.
	/// 
	/// Returns RAW OpenCL executable binary without additional attachments.
	/// ## See also
	/// ProgramSource::fromBinary
	/// 
	/// ## Parameters
	/// * binary:[out] output buffer
	fn get_binary(&self, binary: &mut core::Vector::<i8>) -> Result<()> {
		unsafe { sys::cv_ocl_Program_getBinary_const_vector_char_R(self.as_raw_Program(), binary.as_raw_mut_VectorOfi8()) }.into_result()
	}
	
	fn read(&mut self, buf: &str, buildflags: &str) -> Result<bool> {
		extern_container_arg!(buf);
		extern_container_arg!(buildflags);
		unsafe { sys::cv_ocl_Program_read_const_StringR_const_StringR(self.as_raw_mut_Program(), buf.opencv_to_extern(), buildflags.opencv_to_extern()) }.into_result()
	}
	
	fn write(&self, buf: &mut String) -> Result<bool> {
		string_arg_output_send!(via buf_via);
		let out = unsafe { sys::cv_ocl_Program_write_const_StringR(self.as_raw_Program(), &mut buf_via) }.into_result();
		string_arg_output_receive!(out, buf_via => buf);
		out
	}
	
	fn source(&self) -> Result<core::ProgramSource> {
		unsafe { sys::cv_ocl_Program_source_const(self.as_raw_Program()) }.into_result().map(|r| unsafe { core::ProgramSource::opencv_from_extern(r) } )
	}
	
	fn get_prefix(&self) -> Result<String> {
		unsafe { sys::cv_ocl_Program_getPrefix_const(self.as_raw_Program()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

pub struct Program {
	ptr: *mut c_void
}

opencv_type_boxed! { Program }

impl Drop for Program {
	fn drop(&mut self) {
		extern "C" { fn cv_Program_delete(instance: *mut c_void); }
		unsafe { cv_Program_delete(self.as_raw_mut_Program()) };
	}
}

impl Program {
	#[inline] pub fn as_raw_Program(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Program(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Program {}

impl core::ProgramTrait for Program {
	#[inline] fn as_raw_Program(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Program(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Program {
	pub fn default() -> Result<core::Program> {
		unsafe { sys::cv_ocl_Program_Program() }.into_result().map(|r| unsafe { core::Program::opencv_from_extern(r) } )
	}
	
	pub fn new(src: &core::ProgramSource, buildflags: &str, errmsg: &mut String) -> Result<core::Program> {
		extern_container_arg!(buildflags);
		string_arg_output_send!(via errmsg_via);
		let out = unsafe { sys::cv_ocl_Program_Program_const_ProgramSourceR_const_StringR_StringR(src.as_raw_ProgramSource(), buildflags.opencv_to_extern(), &mut errmsg_via) }.into_result().map(|r| unsafe { core::Program::opencv_from_extern(r) } );
		string_arg_output_receive!(out, errmsg_via => errmsg);
		out
	}
	
	pub fn copy(prog: &core::Program) -> Result<core::Program> {
		unsafe { sys::cv_ocl_Program_Program_const_ProgramR(prog.as_raw_Program()) }.into_result().map(|r| unsafe { core::Program::opencv_from_extern(r) } )
	}
	
	pub fn get_prefix_build_flags(buildflags: &str) -> Result<String> {
		extern_container_arg!(buildflags);
		unsafe { sys::cv_ocl_Program_getPrefix_const_StringR(buildflags.opencv_to_extern()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

pub trait ProgramSourceTrait {
	fn as_raw_ProgramSource(&self) -> *const c_void;
	fn as_raw_mut_ProgramSource(&mut self) -> *mut c_void;

	fn source(&self) -> Result<String> {
		unsafe { sys::cv_ocl_ProgramSource_source_const(self.as_raw_ProgramSource()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn hash(&self) -> Result<core::ProgramSource_hash_t> {
		unsafe { sys::cv_ocl_ProgramSource_hash_const(self.as_raw_ProgramSource()) }.into_result()
	}
	
}

pub struct ProgramSource {
	ptr: *mut c_void
}

opencv_type_boxed! { ProgramSource }

impl Drop for ProgramSource {
	fn drop(&mut self) {
		extern "C" { fn cv_ProgramSource_delete(instance: *mut c_void); }
		unsafe { cv_ProgramSource_delete(self.as_raw_mut_ProgramSource()) };
	}
}

impl ProgramSource {
	#[inline] pub fn as_raw_ProgramSource(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_ProgramSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for ProgramSource {}

impl core::ProgramSourceTrait for ProgramSource {
	#[inline] fn as_raw_ProgramSource(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ProgramSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ProgramSource {
	pub fn default() -> Result<core::ProgramSource> {
		unsafe { sys::cv_ocl_ProgramSource_ProgramSource() }.into_result().map(|r| unsafe { core::ProgramSource::opencv_from_extern(r) } )
	}
	
	pub fn new(module: &str, name: &str, code_str: &str, code_hash: &str) -> Result<core::ProgramSource> {
		extern_container_arg!(module);
		extern_container_arg!(name);
		extern_container_arg!(code_str);
		extern_container_arg!(code_hash);
		unsafe { sys::cv_ocl_ProgramSource_ProgramSource_const_StringR_const_StringR_const_StringR_const_StringR(module.opencv_to_extern(), name.opencv_to_extern(), code_str.opencv_to_extern(), code_hash.opencv_to_extern()) }.into_result().map(|r| unsafe { core::ProgramSource::opencv_from_extern(r) } )
	}
	
	pub fn from_str(prog: &str) -> Result<core::ProgramSource> {
		extern_container_arg!(prog);
		unsafe { sys::cv_ocl_ProgramSource_ProgramSource_const_StringR(prog.opencv_to_extern()) }.into_result().map(|r| unsafe { core::ProgramSource::opencv_from_extern(r) } )
	}
	
	pub fn copy(prog: &core::ProgramSource) -> Result<core::ProgramSource> {
		unsafe { sys::cv_ocl_ProgramSource_ProgramSource_const_ProgramSourceR(prog.as_raw_ProgramSource()) }.into_result().map(|r| unsafe { core::ProgramSource::opencv_from_extern(r) } )
	}
	
	/// Describe OpenCL program binary.
	/// Do not call clCreateProgramWithBinary() and/or clBuildProgram().
	/// 
	/// Caller should guarantee binary buffer lifetime greater than ProgramSource object (and any of its copies).
	/// 
	/// This kind of binary is not portable between platforms in general - it is specific to OpenCL vendor / device / driver version.
	/// 
	/// ## Parameters
	/// * module: name of program owner module
	/// * name: unique name of program (module+name is used as key for OpenCL program caching)
	/// * binary: buffer address. See buffer lifetime requirement in description.
	/// * size: buffer size
	/// * buildOptions: additional program-related build options passed to clBuildProgram()
	/// ## Returns
	/// created ProgramSource object
	/// 
	/// ## C++ default parameters
	/// * build_options: cv::String()
	pub fn from_binary(module: &str, name: &str, binary: &u8, size: size_t, build_options: &str) -> Result<core::ProgramSource> {
		extern_container_arg!(module);
		extern_container_arg!(name);
		extern_container_arg!(build_options);
		unsafe { sys::cv_ocl_ProgramSource_fromBinary_const_StringR_const_StringR_const_unsigned_charX_size_t_const_StringR(module.opencv_to_extern(), name.opencv_to_extern(), binary, size, build_options.opencv_to_extern()) }.into_result().map(|r| unsafe { core::ProgramSource::opencv_from_extern(r) } )
	}
	
	/// Describe OpenCL program in SPIR format.
	/// Do not call clCreateProgramWithBinary() and/or clBuildProgram().
	/// 
	/// Supports SPIR 1.2 by default (pass '-spir-std=X.Y' in buildOptions to override this behavior)
	/// 
	/// Caller should guarantee binary buffer lifetime greater than ProgramSource object (and any of its copies).
	/// 
	/// Programs in this format are portable between OpenCL implementations with 'khr_spir' extension:
	/// https://www.khronos.org/registry/OpenCL/sdk/2.0/docs/man/xhtml/cl_khr_spir.html
	/// (but they are not portable between different platforms: 32-bit / 64-bit)
	/// 
	/// Note: these programs can't support vendor specific extensions, like 'cl_intel_subgroups'.
	/// 
	/// ## Parameters
	/// * module: name of program owner module
	/// * name: unique name of program (module+name is used as key for OpenCL program caching)
	/// * binary: buffer address. See buffer lifetime requirement in description.
	/// * size: buffer size
	/// * buildOptions: additional program-related build options passed to clBuildProgram()
	///        (these options are added automatically: '-x spir' and '-spir-std=1.2')
	/// ## Returns
	/// created ProgramSource object.
	/// 
	/// ## C++ default parameters
	/// * build_options: cv::String()
	pub fn from_spir(module: &str, name: &str, binary: &u8, size: size_t, build_options: &str) -> Result<core::ProgramSource> {
		extern_container_arg!(module);
		extern_container_arg!(name);
		extern_container_arg!(build_options);
		unsafe { sys::cv_ocl_ProgramSource_fromSPIR_const_StringR_const_StringR_const_unsigned_charX_size_t_const_StringR(module.opencv_to_extern(), name.opencv_to_extern(), binary, size, build_options.opencv_to_extern()) }.into_result().map(|r| unsafe { core::ProgramSource::opencv_from_extern(r) } )
	}
	
}

pub trait QueueTrait {
	fn as_raw_Queue(&self) -> *const c_void;
	fn as_raw_mut_Queue(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * c: Context()
	/// * d: Device()
	fn create(&mut self, c: &core::Context, d: &core::Device) -> Result<bool> {
		unsafe { sys::cv_ocl_Queue_create_const_ContextR_const_DeviceR(self.as_raw_mut_Queue(), c.as_raw_Context(), d.as_raw_Device()) }.into_result()
	}
	
	fn finish(&mut self) -> Result<()> {
		unsafe { sys::cv_ocl_Queue_finish(self.as_raw_mut_Queue()) }.into_result()
	}
	
	fn ptr(&self) -> Result<*mut c_void> {
		unsafe { sys::cv_ocl_Queue_ptr_const(self.as_raw_Queue()) }.into_result()
	}
	
	/// Returns OpenCL command queue with enable profiling mode support
	fn get_profiling_queue(&self) -> Result<core::Queue> {
		unsafe { sys::cv_ocl_Queue_getProfilingQueue_const(self.as_raw_Queue()) }.into_result().map(|r| unsafe { core::Queue::opencv_from_extern(r) } )
	}
	
}

pub struct Queue {
	ptr: *mut c_void
}

opencv_type_boxed! { Queue }

impl Drop for Queue {
	fn drop(&mut self) {
		extern "C" { fn cv_Queue_delete(instance: *mut c_void); }
		unsafe { cv_Queue_delete(self.as_raw_mut_Queue()) };
	}
}

impl Queue {
	#[inline] pub fn as_raw_Queue(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Queue(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Queue {}

impl core::QueueTrait for Queue {
	#[inline] fn as_raw_Queue(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Queue(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Queue {
	pub fn default() -> Result<core::Queue> {
		unsafe { sys::cv_ocl_Queue_Queue() }.into_result().map(|r| unsafe { core::Queue::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * d: Device()
	pub fn new(c: &core::Context, d: &core::Device) -> Result<core::Queue> {
		unsafe { sys::cv_ocl_Queue_Queue_const_ContextR_const_DeviceR(c.as_raw_Context(), d.as_raw_Device()) }.into_result().map(|r| unsafe { core::Queue::opencv_from_extern(r) } )
	}
	
	pub fn copy(q: &core::Queue) -> Result<core::Queue> {
		unsafe { sys::cv_ocl_Queue_Queue_const_QueueR(q.as_raw_Queue()) }.into_result().map(|r| unsafe { core::Queue::opencv_from_extern(r) } )
	}
	
	pub fn get_default() -> Result<core::Queue> {
		unsafe { sys::cv_ocl_Queue_getDefault() }.into_result().map(|r| unsafe { core::Queue::opencv_from_extern(r) } )
	}
	
}

pub trait TimerTrait {
	fn as_raw_Timer(&self) -> *const c_void;
	fn as_raw_mut_Timer(&mut self) -> *mut c_void;

	fn start(&mut self) -> Result<()> {
		unsafe { sys::cv_ocl_Timer_start(self.as_raw_mut_Timer()) }.into_result()
	}
	
	fn stop(&mut self) -> Result<()> {
		unsafe { sys::cv_ocl_Timer_stop(self.as_raw_mut_Timer()) }.into_result()
	}
	
	fn duration_ns(&self) -> Result<u64> {
		unsafe { sys::cv_ocl_Timer_durationNS_const(self.as_raw_Timer()) }.into_result()
	}
	
}

pub struct Timer {
	ptr: *mut c_void
}

opencv_type_boxed! { Timer }

impl Drop for Timer {
	fn drop(&mut self) {
		extern "C" { fn cv_Timer_delete(instance: *mut c_void); }
		unsafe { cv_Timer_delete(self.as_raw_mut_Timer()) };
	}
}

impl Timer {
	#[inline] pub fn as_raw_Timer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Timer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Timer {}

impl core::TimerTrait for Timer {
	#[inline] fn as_raw_Timer(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Timer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Timer {
	pub fn new(q: &core::Queue) -> Result<core::Timer> {
		unsafe { sys::cv_ocl_Timer_Timer_const_QueueR(q.as_raw_Queue()) }.into_result().map(|r| unsafe { core::Timer::opencv_from_extern(r) } )
	}
	
}

pub trait LogTagTrait {
	fn as_raw_LogTag(&self) -> *const c_void;
	fn as_raw_mut_LogTag(&mut self) -> *mut c_void;

	fn name(&self) -> String {
		unsafe { sys::cv_utils_logging_LogTag_getPropName_const(self.as_raw_LogTag()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: name")
	}
	
	fn level(&self) -> core::LogLevel {
		unsafe { sys::cv_utils_logging_LogTag_getPropLevel_const(self.as_raw_LogTag()) }.into_result().expect("Infallible function failed: level")
	}
	
	fn set_level(&mut self, val: core::LogLevel) -> () {
		unsafe { sys::cv_utils_logging_LogTag_setPropLevel_LogLevel(self.as_raw_mut_LogTag(), val) }.into_result().expect("Infallible function failed: set_level")
	}
	
}

pub struct LogTag {
	ptr: *mut c_void
}

opencv_type_boxed! { LogTag }

impl Drop for LogTag {
	fn drop(&mut self) {
		extern "C" { fn cv_LogTag_delete(instance: *mut c_void); }
		unsafe { cv_LogTag_delete(self.as_raw_mut_LogTag()) };
	}
}

impl LogTag {
	#[inline] pub fn as_raw_LogTag(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_LogTag(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for LogTag {}

impl core::LogTagTrait for LogTag {
	#[inline] fn as_raw_LogTag(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_LogTag(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LogTag {
	pub fn new(_name: &str, _level: core::LogLevel) -> Result<core::LogTag> {
		extern_container_arg!(_name);
		unsafe { sys::cv_utils_logging_LogTag_LogTag_const_charX_LogLevel(_name.opencv_to_extern(), _level) }.into_result().map(|r| unsafe { core::LogTag::opencv_from_extern(r) } )
	}
	
}
pub use crate::manual::core::*;

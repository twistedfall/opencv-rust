//! # Clustering and Search in Multi-Dimensional Spaces
//!
//! This section documents OpenCV's interface to the FLANN library. FLANN (Fast Library for Approximate
//! Nearest Neighbors) is a library that contains a collection of algorithms optimized for fast nearest
//! neighbor search in large datasets and for high dimensional features. More information about FLANN
//! can be found in [Muja2009](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Muja2009) .
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{AutotunedIndexParamsTrait, AutotunedIndexParamsTraitConst, CompositeIndexParamsTrait, CompositeIndexParamsTraitConst, HierarchicalClusteringIndexParamsTrait, HierarchicalClusteringIndexParamsTraitConst, IndexParamsTrait, IndexParamsTraitConst, IndexTrait, IndexTraitConst, KDTreeIndexParamsTrait, KDTreeIndexParamsTraitConst, KMeansIndexParamsTrait, KMeansIndexParamsTraitConst, LinearIndexParamsTrait, LinearIndexParamsTraitConst, LshIndexParamsTrait, LshIndexParamsTraitConst, SavedIndexParamsTrait, SavedIndexParamsTraitConst, SearchParamsTrait, SearchParamsTraitConst};
}

// AUTOTUNED /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:89
pub const AUTOTUNED: i32 = 255;
// BITS_PER_BASE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/kmeans_index.h:53
pub const BITS_PER_BASE: i32 = 2;
// BITS_PER_CHAR /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/kmeans_index.h:52
pub const BITS_PER_CHAR: i32 = 8;
// BLOCKSIZE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/allocator.h:74
pub const BLOCKSIZE: u32 = 8192;
// CENTERS_GONZALES /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:103
pub const CENTERS_GONZALES: i32 = 1;
// CENTERS_KMEANSPP /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:104
pub const CENTERS_KMEANSPP: i32 = 2;
// CENTERS_RANDOM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:102
pub const CENTERS_RANDOM: i32 = 0;
// COMPOSITE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:86
pub const COMPOSITE: i32 = 3;
// CS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:140
pub const CS: i32 = 7;
// EUCLIDEAN /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:134
pub const EUCLIDEAN: i32 = 1;
// FLANN_CENTERS_GONZALES /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:97
pub const FLANN_CENTERS_GONZALES: i32 = 1;
// FLANN_CENTERS_GROUPWISE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:99
pub const FLANN_CENTERS_GROUPWISE: i32 = 3;
// FLANN_CENTERS_KMEANSPP /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:98
pub const FLANN_CENTERS_KMEANSPP: i32 = 2;
// FLANN_CENTERS_RANDOM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:96
pub const FLANN_CENTERS_RANDOM: i32 = 0;
// FLANN_CHECKS_AUTOTUNED /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:162
pub const FLANN_CHECKS_AUTOTUNED: i32 = -2;
// FLANN_CHECKS_UNLIMITED /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:161
pub const FLANN_CHECKS_UNLIMITED: i32 = -1;
// FLANN_DIST_CHI_SQUARE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:126
pub const FLANN_DIST_CHI_SQUARE: i32 = 7;
// FLANN_DIST_CS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:127
pub const FLANN_DIST_CS: i32 = 7;
// FLANN_DIST_DNAMMING /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:131
pub const FLANN_DIST_DNAMMING: i32 = 10;
// FLANN_DIST_EUCLIDEAN /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:118
pub const FLANN_DIST_EUCLIDEAN: i32 = 1;
// FLANN_DIST_HAMMING /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:130
pub const FLANN_DIST_HAMMING: i32 = 9;
// FLANN_DIST_HELLINGER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:125
pub const FLANN_DIST_HELLINGER: i32 = 6;
// FLANN_DIST_HIST_INTERSECT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:124
pub const FLANN_DIST_HIST_INTERSECT: i32 = 5;
// FLANN_DIST_KL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:129
pub const FLANN_DIST_KL: i32 = 8;
// FLANN_DIST_KULLBACK_LEIBLER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:128
pub const FLANN_DIST_KULLBACK_LEIBLER: i32 = 8;
// FLANN_DIST_L1 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:121
pub const FLANN_DIST_L1: i32 = 2;
// FLANN_DIST_L2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:119
pub const FLANN_DIST_L2: i32 = 1;
// FLANN_DIST_MANHATTAN /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:120
pub const FLANN_DIST_MANHATTAN: i32 = 2;
// FLANN_DIST_MAX /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:123
pub const FLANN_DIST_MAX: i32 = 4;
// FLANN_DIST_MINKOWSKI /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:122
pub const FLANN_DIST_MINKOWSKI: i32 = 3;
// FLANN_FLOAT32 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:155
pub const FLANN_FLOAT32: i32 = 8;
// FLANN_FLOAT64 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:156
pub const FLANN_FLOAT64: i32 = 9;
// FLANN_INDEX_AUTOTUNED /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:80
pub const FLANN_INDEX_AUTOTUNED: i32 = 255;
// FLANN_INDEX_COMPOSITE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:75
pub const FLANN_INDEX_COMPOSITE: i32 = 3;
// FLANN_INDEX_HIERARCHICAL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:77
pub const FLANN_INDEX_HIERARCHICAL: i32 = 5;
// FLANN_INDEX_KDTREE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:73
pub const FLANN_INDEX_KDTREE: i32 = 1;
// FLANN_INDEX_KDTREE_SINGLE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:76
pub const FLANN_INDEX_KDTREE_SINGLE: i32 = 4;
// FLANN_INDEX_KMEANS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:74
pub const FLANN_INDEX_KMEANS: i32 = 2;
// FLANN_INDEX_LINEAR /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:72
pub const FLANN_INDEX_LINEAR: i32 = 0;
// FLANN_INDEX_LSH /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:78
pub const FLANN_INDEX_LSH: i32 = 6;
// FLANN_INDEX_SAVED /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:79
pub const FLANN_INDEX_SAVED: i32 = 254;
// FLANN_INDEX_TYPE_16S /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:61
pub const FLANN_INDEX_TYPE_16S: i32 = 3;
// FLANN_INDEX_TYPE_16U /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:60
pub const FLANN_INDEX_TYPE_16U: i32 = 2;
// FLANN_INDEX_TYPE_32F /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:63
pub const FLANN_INDEX_TYPE_32F: i32 = 5;
// FLANN_INDEX_TYPE_32S /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:62
pub const FLANN_INDEX_TYPE_32S: i32 = 4;
// FLANN_INDEX_TYPE_64F /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:64
pub const FLANN_INDEX_TYPE_64F: i32 = 6;
// FLANN_INDEX_TYPE_8S /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:59
pub const FLANN_INDEX_TYPE_8S: i32 = 1;
// FLANN_INDEX_TYPE_8U /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:58
pub const FLANN_INDEX_TYPE_8U: i32 = 0;
// FLANN_INDEX_TYPE_ALGORITHM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:67
pub const FLANN_INDEX_TYPE_ALGORITHM: i32 = 9;
// FLANN_INDEX_TYPE_BOOL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:66
pub const FLANN_INDEX_TYPE_BOOL: i32 = 8;
// FLANN_INDEX_TYPE_STRING /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:65
pub const FLANN_INDEX_TYPE_STRING: i32 = 7;
// FLANN_INT16 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:148
pub const FLANN_INT16: i32 = 1;
// FLANN_INT32 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:149
pub const FLANN_INT32: i32 = 2;
// FLANN_INT64 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:150
pub const FLANN_INT64: i32 = 3;
// FLANN_INT8 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:147
pub const FLANN_INT8: i32 = 0;
// FLANN_LOG_ERROR /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:111
pub const FLANN_LOG_ERROR: i32 = 2;
// FLANN_LOG_FATAL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:110
pub const FLANN_LOG_FATAL: i32 = 1;
// FLANN_LOG_INFO /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:113
pub const FLANN_LOG_INFO: i32 = 4;
// FLANN_LOG_NONE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:109
pub const FLANN_LOG_NONE: i32 = 0;
// FLANN_LOG_WARN /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:112
pub const FLANN_LOG_WARN: i32 = 3;
// FLANN_SIGNATURE_ /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/saving.h:43
pub const FLANN_SIGNATURE_: &str = "FLANN_INDEX";
// FLANN_UINT16 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:152
pub const FLANN_UINT16: i32 = 5;
// FLANN_UINT32 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:153
pub const FLANN_UINT32: i32 = 6;
// FLANN_UINT64 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:154
pub const FLANN_UINT64: i32 = 7;
// FLANN_UINT8 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:151
pub const FLANN_UINT8: i32 = 4;
// FLANN_USE_BOOST /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/dynamic_bitset.h:41
pub const FLANN_USE_BOOST: i32 = 0;
// FLANN_VERSION_ /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/config.h:38
pub const FLANN_VERSION_: &str = "1.6.10";
// HELLINGER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:139
pub const HELLINGER: i32 = 6;
// HIST_INTERSECT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:138
pub const HIST_INTERSECT: i32 = 5;
// KDTREE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:84
pub const KDTREE: i32 = 1;
// KDTREE_SINGLE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:87
pub const KDTREE_SINGLE: i32 = 4;
// KL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:141
pub const KL: i32 = 8;
// KMEANS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:85
pub const KMEANS: i32 = 2;
// KULLBACK_LEIBLER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:142
pub const KULLBACK_LEIBLER: i32 = 8;
// LAST_VALUE_FLANN_INDEX_TYPE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:68
pub const LAST_VALUE_FLANN_INDEX_TYPE: i32 = 9;
// LINEAR /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:83
pub const LINEAR: i32 = 0;
// MANHATTAN /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:135
pub const MANHATTAN: i32 = 2;
// MAX_DIST /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:137
pub const MAX_DIST: i32 = 4;
// MINKOWSKI /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:136
pub const MINKOWSKI: i32 = 3;
// SAVED /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:88
pub const SAVED: i32 = 254;
// USE_UNORDERED_MAP /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/lsh_table.h:46
pub const USE_UNORDERED_MAP: i32 = 1;
/// Pooled storage allocator
///
/// The following routines allow for the efficient allocation of storage in
/// small chunks from a specified pool.  Rather than allowing each structure
/// to be freed individually, an entire pool of storage is freed at once.
/// This method has two advantages over just using malloc() and free().  First,
/// it is far more efficient for allocating small objects, as there is
/// no overhead for remembering all the information needed to free each
/// object or consolidating fragmented memory.  Second, the decision about
/// how long to keep an object is made at the time of allocation, and there
/// is no need to track down all the objects to free them.
// WORDSIZE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/allocator.h:73
pub const WORDSIZE: u32 = 16;
// FlannIndexType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:57
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FlannIndexType {
	FLANN_INDEX_TYPE_8U = 0,
	FLANN_INDEX_TYPE_8S = 1,
	FLANN_INDEX_TYPE_16U = 2,
	FLANN_INDEX_TYPE_16S = 3,
	FLANN_INDEX_TYPE_32S = 4,
	FLANN_INDEX_TYPE_32F = 5,
	FLANN_INDEX_TYPE_64F = 6,
	FLANN_INDEX_TYPE_STRING = 7,
	FLANN_INDEX_TYPE_BOOL = 8,
	FLANN_INDEX_TYPE_ALGORITHM = 9,
	// Duplicate, use FLANN_INDEX_TYPE_ALGORITHM instead
	// LAST_VALUE_FLANN_INDEX_TYPE = 9,
}

impl TryFrom<i32> for FlannIndexType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::FLANN_INDEX_TYPE_8U),
			1 => Ok(Self::FLANN_INDEX_TYPE_8S),
			2 => Ok(Self::FLANN_INDEX_TYPE_16U),
			3 => Ok(Self::FLANN_INDEX_TYPE_16S),
			4 => Ok(Self::FLANN_INDEX_TYPE_32S),
			5 => Ok(Self::FLANN_INDEX_TYPE_32F),
			6 => Ok(Self::FLANN_INDEX_TYPE_64F),
			7 => Ok(Self::FLANN_INDEX_TYPE_STRING),
			8 => Ok(Self::FLANN_INDEX_TYPE_BOOL),
			9 => Ok(Self::FLANN_INDEX_TYPE_ALGORITHM),
			// Duplicate of FLANN_INDEX_TYPE_ALGORITHM
			// 9 => Ok(Self::LAST_VALUE_FLANN_INDEX_TYPE),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::flann::FlannIndexType"))),
		}
	}
}

opencv_type_enum! { crate::flann::FlannIndexType }

// flann_algorithm_t /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:70
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum flann_algorithm_t {
	FLANN_INDEX_LINEAR = 0,
	FLANN_INDEX_KDTREE = 1,
	FLANN_INDEX_KMEANS = 2,
	FLANN_INDEX_COMPOSITE = 3,
	FLANN_INDEX_KDTREE_SINGLE = 4,
	FLANN_INDEX_HIERARCHICAL = 5,
	FLANN_INDEX_LSH = 6,
	FLANN_INDEX_SAVED = 254,
	FLANN_INDEX_AUTOTUNED = 255,
	// Duplicate, use FLANN_INDEX_LINEAR instead
	// LINEAR = 0,
	// Duplicate, use FLANN_INDEX_KDTREE instead
	// KDTREE = 1,
	// Duplicate, use FLANN_INDEX_KMEANS instead
	// KMEANS = 2,
	// Duplicate, use FLANN_INDEX_COMPOSITE instead
	// COMPOSITE = 3,
	// Duplicate, use FLANN_INDEX_KDTREE_SINGLE instead
	// KDTREE_SINGLE = 4,
	// Duplicate, use FLANN_INDEX_SAVED instead
	// SAVED = 254,
	// Duplicate, use FLANN_INDEX_AUTOTUNED instead
	// AUTOTUNED = 255,
}

impl TryFrom<i32> for flann_algorithm_t {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::FLANN_INDEX_LINEAR),
			1 => Ok(Self::FLANN_INDEX_KDTREE),
			2 => Ok(Self::FLANN_INDEX_KMEANS),
			3 => Ok(Self::FLANN_INDEX_COMPOSITE),
			4 => Ok(Self::FLANN_INDEX_KDTREE_SINGLE),
			5 => Ok(Self::FLANN_INDEX_HIERARCHICAL),
			6 => Ok(Self::FLANN_INDEX_LSH),
			254 => Ok(Self::FLANN_INDEX_SAVED),
			255 => Ok(Self::FLANN_INDEX_AUTOTUNED),
			// Duplicate of FLANN_INDEX_LINEAR
			// 0 => Ok(Self::LINEAR),
			// Duplicate of FLANN_INDEX_KDTREE
			// 1 => Ok(Self::KDTREE),
			// Duplicate of FLANN_INDEX_KMEANS
			// 2 => Ok(Self::KMEANS),
			// Duplicate of FLANN_INDEX_COMPOSITE
			// 3 => Ok(Self::COMPOSITE),
			// Duplicate of FLANN_INDEX_KDTREE_SINGLE
			// 4 => Ok(Self::KDTREE_SINGLE),
			// Duplicate of FLANN_INDEX_SAVED
			// 254 => Ok(Self::SAVED),
			// Duplicate of FLANN_INDEX_AUTOTUNED
			// 255 => Ok(Self::AUTOTUNED),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::flann::flann_algorithm_t"))),
		}
	}
}

opencv_type_enum! { crate::flann::flann_algorithm_t }

// flann_centers_init_t /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:94
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum flann_centers_init_t {
	FLANN_CENTERS_RANDOM = 0,
	FLANN_CENTERS_GONZALES = 1,
	FLANN_CENTERS_KMEANSPP = 2,
	FLANN_CENTERS_GROUPWISE = 3,
	// Duplicate, use FLANN_CENTERS_RANDOM instead
	// CENTERS_RANDOM = 0,
	// Duplicate, use FLANN_CENTERS_GONZALES instead
	// CENTERS_GONZALES = 1,
	// Duplicate, use FLANN_CENTERS_KMEANSPP instead
	// CENTERS_KMEANSPP = 2,
}

impl TryFrom<i32> for flann_centers_init_t {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::FLANN_CENTERS_RANDOM),
			1 => Ok(Self::FLANN_CENTERS_GONZALES),
			2 => Ok(Self::FLANN_CENTERS_KMEANSPP),
			3 => Ok(Self::FLANN_CENTERS_GROUPWISE),
			// Duplicate of FLANN_CENTERS_RANDOM
			// 0 => Ok(Self::CENTERS_RANDOM),
			// Duplicate of FLANN_CENTERS_GONZALES
			// 1 => Ok(Self::CENTERS_GONZALES),
			// Duplicate of FLANN_CENTERS_KMEANSPP
			// 2 => Ok(Self::CENTERS_KMEANSPP),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::flann::flann_centers_init_t"))),
		}
	}
}

opencv_type_enum! { crate::flann::flann_centers_init_t }

// flann_datatype_t /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:145
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum flann_datatype_t {
	FLANN_INT8 = 0,
	FLANN_INT16 = 1,
	FLANN_INT32 = 2,
	FLANN_INT64 = 3,
	FLANN_UINT8 = 4,
	FLANN_UINT16 = 5,
	FLANN_UINT32 = 6,
	FLANN_UINT64 = 7,
	FLANN_FLOAT32 = 8,
	FLANN_FLOAT64 = 9,
}

impl TryFrom<i32> for flann_datatype_t {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::FLANN_INT8),
			1 => Ok(Self::FLANN_INT16),
			2 => Ok(Self::FLANN_INT32),
			3 => Ok(Self::FLANN_INT64),
			4 => Ok(Self::FLANN_UINT8),
			5 => Ok(Self::FLANN_UINT16),
			6 => Ok(Self::FLANN_UINT32),
			7 => Ok(Self::FLANN_UINT64),
			8 => Ok(Self::FLANN_FLOAT32),
			9 => Ok(Self::FLANN_FLOAT64),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::flann::flann_datatype_t"))),
		}
	}
}

opencv_type_enum! { crate::flann::flann_datatype_t }

// flann_distance_t /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:116
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum flann_distance_t {
	FLANN_DIST_EUCLIDEAN = 1,
	// Duplicate, use FLANN_DIST_EUCLIDEAN instead
	// FLANN_DIST_L2 = 1,
	FLANN_DIST_MANHATTAN = 2,
	// Duplicate, use FLANN_DIST_MANHATTAN instead
	// FLANN_DIST_L1 = 2,
	FLANN_DIST_MINKOWSKI = 3,
	FLANN_DIST_MAX = 4,
	FLANN_DIST_HIST_INTERSECT = 5,
	FLANN_DIST_HELLINGER = 6,
	FLANN_DIST_CHI_SQUARE = 7,
	// Duplicate, use FLANN_DIST_CHI_SQUARE instead
	// FLANN_DIST_CS = 7,
	FLANN_DIST_KULLBACK_LEIBLER = 8,
	// Duplicate, use FLANN_DIST_KULLBACK_LEIBLER instead
	// FLANN_DIST_KL = 8,
	FLANN_DIST_HAMMING = 9,
	FLANN_DIST_DNAMMING = 10,
	// Duplicate, use FLANN_DIST_L2 instead
	// EUCLIDEAN = 1,
	// Duplicate, use FLANN_DIST_L1 instead
	// MANHATTAN = 2,
	// Duplicate, use FLANN_DIST_MINKOWSKI instead
	// MINKOWSKI = 3,
	// Duplicate, use FLANN_DIST_MAX instead
	// MAX_DIST = 4,
	// Duplicate, use FLANN_DIST_HIST_INTERSECT instead
	// HIST_INTERSECT = 5,
	// Duplicate, use FLANN_DIST_HELLINGER instead
	// HELLINGER = 6,
	// Duplicate, use FLANN_DIST_CS instead
	// CS = 7,
	// Duplicate, use FLANN_DIST_KL instead
	// KL = 8,
	// Duplicate, use KL instead
	// KULLBACK_LEIBLER = 8,
}

impl TryFrom<i32> for flann_distance_t {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::FLANN_DIST_EUCLIDEAN),
			// Duplicate of FLANN_DIST_EUCLIDEAN
			// 1 => Ok(Self::FLANN_DIST_L2),
			2 => Ok(Self::FLANN_DIST_MANHATTAN),
			// Duplicate of FLANN_DIST_MANHATTAN
			// 2 => Ok(Self::FLANN_DIST_L1),
			3 => Ok(Self::FLANN_DIST_MINKOWSKI),
			4 => Ok(Self::FLANN_DIST_MAX),
			5 => Ok(Self::FLANN_DIST_HIST_INTERSECT),
			6 => Ok(Self::FLANN_DIST_HELLINGER),
			7 => Ok(Self::FLANN_DIST_CHI_SQUARE),
			// Duplicate of FLANN_DIST_CHI_SQUARE
			// 7 => Ok(Self::FLANN_DIST_CS),
			8 => Ok(Self::FLANN_DIST_KULLBACK_LEIBLER),
			// Duplicate of FLANN_DIST_KULLBACK_LEIBLER
			// 8 => Ok(Self::FLANN_DIST_KL),
			9 => Ok(Self::FLANN_DIST_HAMMING),
			10 => Ok(Self::FLANN_DIST_DNAMMING),
			// Duplicate of FLANN_DIST_L2
			// 1 => Ok(Self::EUCLIDEAN),
			// Duplicate of FLANN_DIST_L1
			// 2 => Ok(Self::MANHATTAN),
			// Duplicate of FLANN_DIST_MINKOWSKI
			// 3 => Ok(Self::MINKOWSKI),
			// Duplicate of FLANN_DIST_MAX
			// 4 => Ok(Self::MAX_DIST),
			// Duplicate of FLANN_DIST_HIST_INTERSECT
			// 5 => Ok(Self::HIST_INTERSECT),
			// Duplicate of FLANN_DIST_HELLINGER
			// 6 => Ok(Self::HELLINGER),
			// Duplicate of FLANN_DIST_CS
			// 7 => Ok(Self::CS),
			// Duplicate of FLANN_DIST_KL
			// 8 => Ok(Self::KL),
			// Duplicate of KL
			// 8 => Ok(Self::KULLBACK_LEIBLER),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::flann::flann_distance_t"))),
		}
	}
}

opencv_type_enum! { crate::flann::flann_distance_t }

// flann_log_level_t /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/defines.h:107
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum flann_log_level_t {
	FLANN_LOG_NONE = 0,
	FLANN_LOG_FATAL = 1,
	FLANN_LOG_ERROR = 2,
	FLANN_LOG_WARN = 3,
	FLANN_LOG_INFO = 4,
}

impl TryFrom<i32> for flann_log_level_t {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::FLANN_LOG_NONE),
			1 => Ok(Self::FLANN_LOG_FATAL),
			2 => Ok(Self::FLANN_LOG_ERROR),
			3 => Ok(Self::FLANN_LOG_WARN),
			4 => Ok(Self::FLANN_LOG_INFO),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::flann::flann_log_level_t"))),
		}
	}
}

opencv_type_enum! { crate::flann::flann_log_level_t }

/// A bucket in an LSH table
// Bucket /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/lsh_table.h:84
pub type Bucket = core::Vector<crate::flann::FeatureIndex>;
/// The id from which we can get a bucket back in an LSH table
// BucketKey /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/lsh_table.h:80
pub type BucketKey = u32;
/// What is stored in an LSH bucket
// FeatureIndex /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/lsh_table.h:77
pub type FeatureIndex = u32;
// flann_distance_type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann.hpp:61
// ("cvflann::flann_distance_type", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn flann_distance_type() -> Result<crate::flann::flann_distance_t> {
	return_send!(via ocvrs_return);
	unsafe { sys::cvflann_flann_distance_type(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// set_distance_type(flann_distance_t, int)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann.hpp:62
// ("cvflann::set_distance_type", vec![(pred!(mut, ["distance_type", "order"], ["cvflann::flann_distance_t", "int"]), _)]),
#[inline]
pub fn set_distance_type(distance_type: crate::flann::flann_distance_t, order: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cvflann_set_distance_type_flann_distance_t_int(distance_type, order, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::flann::AutotunedIndexParams]
// AutotunedIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:116
pub trait AutotunedIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_AutotunedIndexParams(&self) -> *const c_void;

}

/// Mutable methods for [crate::flann::AutotunedIndexParams]
pub trait AutotunedIndexParamsTrait: crate::flann::AutotunedIndexParamsTraitConst + crate::flann::IndexParamsTrait {
	fn as_raw_mut_AutotunedIndexParams(&mut self) -> *mut c_void;

}

// AutotunedIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:116
pub struct AutotunedIndexParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { AutotunedIndexParams }

impl Drop for AutotunedIndexParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_flann_AutotunedIndexParams_delete(self.as_raw_mut_AutotunedIndexParams()) };
	}
}

unsafe impl Send for AutotunedIndexParams {}

impl crate::flann::IndexParamsTraitConst for AutotunedIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for AutotunedIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AutotunedIndexParams, crate::flann::IndexParamsTraitConst, as_raw_IndexParams, crate::flann::IndexParamsTrait, as_raw_mut_IndexParams }

impl crate::flann::AutotunedIndexParamsTraitConst for AutotunedIndexParams {
	#[inline] fn as_raw_AutotunedIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::AutotunedIndexParamsTrait for AutotunedIndexParams {
	#[inline] fn as_raw_mut_AutotunedIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AutotunedIndexParams, crate::flann::AutotunedIndexParamsTraitConst, as_raw_AutotunedIndexParams, crate::flann::AutotunedIndexParamsTrait, as_raw_mut_AutotunedIndexParams }

impl AutotunedIndexParams {
	/// ## C++ default parameters
	/// * target_precision: 0.8f
	/// * build_weight: 0.01f
	/// * memory_weight: 0
	/// * sample_fraction: 0.1f
	// AutotunedIndexParams(float, float, float, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:118
	// ("cv::flann::AutotunedIndexParams::AutotunedIndexParams", vec![(pred!(mut, ["target_precision", "build_weight", "memory_weight", "sample_fraction"], ["float", "float", "float", "float"]), _)]),
	#[inline]
	pub fn new(target_precision: f32, build_weight: f32, memory_weight: f32, sample_fraction: f32) -> Result<crate::flann::AutotunedIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_AutotunedIndexParams_AutotunedIndexParams_float_float_float_float(target_precision, build_weight, memory_weight, sample_fraction, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::AutotunedIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * target_precision: 0.8f
	/// * build_weight: 0.01f
	/// * memory_weight: 0
	/// * sample_fraction: 0.1f
	// cv::flann::AutotunedIndexParams::AutotunedIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:118
	// ("cv::flann::AutotunedIndexParams::AutotunedIndexParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::flann::AutotunedIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_AutotunedIndexParams_AutotunedIndexParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::AutotunedIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { AutotunedIndexParams, crate::flann::IndexParams, cv_flann_AutotunedIndexParams_to_IndexParams }

impl std::fmt::Debug for AutotunedIndexParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AutotunedIndexParams")
			.finish()
	}
}

/// Constant methods for [crate::flann::CompositeIndexParams]
// CompositeIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:110
pub trait CompositeIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_CompositeIndexParams(&self) -> *const c_void;

}

/// Mutable methods for [crate::flann::CompositeIndexParams]
pub trait CompositeIndexParamsTrait: crate::flann::CompositeIndexParamsTraitConst + crate::flann::IndexParamsTrait {
	fn as_raw_mut_CompositeIndexParams(&mut self) -> *mut c_void;

}

// CompositeIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:110
pub struct CompositeIndexParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { CompositeIndexParams }

impl Drop for CompositeIndexParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_flann_CompositeIndexParams_delete(self.as_raw_mut_CompositeIndexParams()) };
	}
}

unsafe impl Send for CompositeIndexParams {}

impl crate::flann::IndexParamsTraitConst for CompositeIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for CompositeIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CompositeIndexParams, crate::flann::IndexParamsTraitConst, as_raw_IndexParams, crate::flann::IndexParamsTrait, as_raw_mut_IndexParams }

impl crate::flann::CompositeIndexParamsTraitConst for CompositeIndexParams {
	#[inline] fn as_raw_CompositeIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::CompositeIndexParamsTrait for CompositeIndexParams {
	#[inline] fn as_raw_mut_CompositeIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CompositeIndexParams, crate::flann::CompositeIndexParamsTraitConst, as_raw_CompositeIndexParams, crate::flann::CompositeIndexParamsTrait, as_raw_mut_CompositeIndexParams }

impl CompositeIndexParams {
	/// ## C++ default parameters
	/// * trees: 4
	/// * branching: 32
	/// * iterations: 11
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * cb_index: 0.2f
	// CompositeIndexParams(int, int, int, cvflann::flann_centers_init_t, float)(Primitive, Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:112
	// ("cv::flann::CompositeIndexParams::CompositeIndexParams", vec![(pred!(mut, ["trees", "branching", "iterations", "centers_init", "cb_index"], ["int", "int", "int", "cvflann::flann_centers_init_t", "float"]), _)]),
	#[inline]
	pub fn new(trees: i32, branching: i32, iterations: i32, centers_init: crate::flann::flann_centers_init_t, cb_index: f32) -> Result<crate::flann::CompositeIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_CompositeIndexParams_CompositeIndexParams_int_int_int_flann_centers_init_t_float(trees, branching, iterations, centers_init, cb_index, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::CompositeIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * trees: 4
	/// * branching: 32
	/// * iterations: 11
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * cb_index: 0.2f
	// cv::flann::CompositeIndexParams::CompositeIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:112
	// ("cv::flann::CompositeIndexParams::CompositeIndexParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::flann::CompositeIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_CompositeIndexParams_CompositeIndexParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::CompositeIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { CompositeIndexParams, crate::flann::IndexParams, cv_flann_CompositeIndexParams_to_IndexParams }

impl std::fmt::Debug for CompositeIndexParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CompositeIndexParams")
			.finish()
	}
}

/// Constant methods for [crate::flann::HierarchicalClusteringIndexParams]
// HierarchicalClusteringIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:122
pub trait HierarchicalClusteringIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_HierarchicalClusteringIndexParams(&self) -> *const c_void;

}

/// Mutable methods for [crate::flann::HierarchicalClusteringIndexParams]
pub trait HierarchicalClusteringIndexParamsTrait: crate::flann::HierarchicalClusteringIndexParamsTraitConst + crate::flann::IndexParamsTrait {
	fn as_raw_mut_HierarchicalClusteringIndexParams(&mut self) -> *mut c_void;

}

// HierarchicalClusteringIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:122
pub struct HierarchicalClusteringIndexParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { HierarchicalClusteringIndexParams }

impl Drop for HierarchicalClusteringIndexParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_flann_HierarchicalClusteringIndexParams_delete(self.as_raw_mut_HierarchicalClusteringIndexParams()) };
	}
}

unsafe impl Send for HierarchicalClusteringIndexParams {}

impl crate::flann::IndexParamsTraitConst for HierarchicalClusteringIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for HierarchicalClusteringIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { HierarchicalClusteringIndexParams, crate::flann::IndexParamsTraitConst, as_raw_IndexParams, crate::flann::IndexParamsTrait, as_raw_mut_IndexParams }

impl crate::flann::HierarchicalClusteringIndexParamsTraitConst for HierarchicalClusteringIndexParams {
	#[inline] fn as_raw_HierarchicalClusteringIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::HierarchicalClusteringIndexParamsTrait for HierarchicalClusteringIndexParams {
	#[inline] fn as_raw_mut_HierarchicalClusteringIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { HierarchicalClusteringIndexParams, crate::flann::HierarchicalClusteringIndexParamsTraitConst, as_raw_HierarchicalClusteringIndexParams, crate::flann::HierarchicalClusteringIndexParamsTrait, as_raw_mut_HierarchicalClusteringIndexParams }

impl HierarchicalClusteringIndexParams {
	/// ## C++ default parameters
	/// * branching: 32
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * trees: 4
	/// * leaf_size: 100
	// HierarchicalClusteringIndexParams(int, cvflann::flann_centers_init_t, int, int)(Primitive, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:124
	// ("cv::flann::HierarchicalClusteringIndexParams::HierarchicalClusteringIndexParams", vec![(pred!(mut, ["branching", "centers_init", "trees", "leaf_size"], ["int", "cvflann::flann_centers_init_t", "int", "int"]), _)]),
	#[inline]
	pub fn new(branching: i32, centers_init: crate::flann::flann_centers_init_t, trees: i32, leaf_size: i32) -> Result<crate::flann::HierarchicalClusteringIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_HierarchicalClusteringIndexParams_HierarchicalClusteringIndexParams_int_flann_centers_init_t_int_int(branching, centers_init, trees, leaf_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::HierarchicalClusteringIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * branching: 32
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * trees: 4
	/// * leaf_size: 100
	// cv::flann::HierarchicalClusteringIndexParams::HierarchicalClusteringIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:124
	// ("cv::flann::HierarchicalClusteringIndexParams::HierarchicalClusteringIndexParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::flann::HierarchicalClusteringIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_HierarchicalClusteringIndexParams_HierarchicalClusteringIndexParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::HierarchicalClusteringIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { HierarchicalClusteringIndexParams, crate::flann::IndexParams, cv_flann_HierarchicalClusteringIndexParams_to_IndexParams }

impl std::fmt::Debug for HierarchicalClusteringIndexParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("HierarchicalClusteringIndexParams")
			.finish()
	}
}

/// Constant methods for [crate::flann::Index]
// Index /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:150
pub trait IndexTraitConst {
	fn as_raw_Index(&self) -> *const c_void;

	// save(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:165
	// ("cv::flann::Index::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
	#[inline]
	fn save(&self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_save_const_const_StringR(self.as_raw_Index(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDistance()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:168
	// ("cv::flann::Index::getDistance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_distance(&self) -> Result<crate::flann::flann_distance_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_getDistance_const(self.as_raw_Index(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getAlgorithm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:169
	// ("cv::flann::Index::getAlgorithm", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_algorithm(&self) -> Result<crate::flann::flann_algorithm_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_getAlgorithm_const(self.as_raw_Index(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::flann::Index]
pub trait IndexTrait: crate::flann::IndexTraitConst {
	fn as_raw_mut_Index(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * dist_type: cvflann::FLANN_DIST_L2
	// build(InputArray, const IndexParams &, cvflann::flann_distance_t)(InputArray, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:157
	// ("cv::flann::Index::build", vec![(pred!(mut, ["features", "params", "distType"], ["const cv::_InputArray*", "const cv::flann::IndexParams*", "cvflann::flann_distance_t"]), _)]),
	#[inline]
	fn build(&mut self, features: &impl ToInputArray, params: &impl crate::flann::IndexParamsTraitConst, dist_type: crate::flann::flann_distance_t) -> Result<()> {
		input_array_arg!(features);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_build_const__InputArrayR_const_IndexParamsR_flann_distance_t(self.as_raw_mut_Index(), features.as_raw__InputArray(), params.as_raw_IndexParams(), dist_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [IndexTrait::build] function uses the following default values for its arguments:
	/// * dist_type: cvflann::FLANN_DIST_L2
	// cv::flann::Index::build(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:157
	// ("cv::flann::Index::build", vec![(pred!(mut, ["features", "params"], ["const cv::_InputArray*", "const cv::flann::IndexParams*"]), _)]),
	#[inline]
	fn build_def(&mut self, features: &impl ToInputArray, params: &impl crate::flann::IndexParamsTraitConst) -> Result<()> {
		input_array_arg!(features);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_build_const__InputArrayR_const_IndexParamsR(self.as_raw_mut_Index(), features.as_raw__InputArray(), params.as_raw_IndexParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * params: SearchParams()
	// knnSearch(InputArray, OutputArray, OutputArray, int, const SearchParams &)(InputArray, OutputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:158
	// ("cv::flann::Index::knnSearch", vec![(pred!(mut, ["query", "indices", "dists", "knn", "params"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::flann::SearchParams*"]), _)]),
	#[inline]
	fn knn_search(&mut self, query: &impl ToInputArray, indices: &mut impl ToOutputArray, dists: &mut impl ToOutputArray, knn: i32, params: &impl crate::flann::SearchParamsTraitConst) -> Result<()> {
		input_array_arg!(query);
		output_array_arg!(indices);
		output_array_arg!(dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SearchParamsR(self.as_raw_mut_Index(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), knn, params.as_raw_SearchParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [IndexTrait::knn_search] function uses the following default values for its arguments:
	/// * params: SearchParams()
	// cv::flann::Index::knnSearch(InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:158
	// ("cv::flann::Index::knnSearch", vec![(pred!(mut, ["query", "indices", "dists", "knn"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	#[inline]
	fn knn_search_def(&mut self, query: &impl ToInputArray, indices: &mut impl ToOutputArray, dists: &mut impl ToOutputArray, knn: i32) -> Result<()> {
		input_array_arg!(query);
		output_array_arg!(indices);
		output_array_arg!(dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(self.as_raw_mut_Index(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), knn, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * params: SearchParams()
	// radiusSearch(InputArray, OutputArray, OutputArray, double, int, const SearchParams &)(InputArray, OutputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:161
	// ("cv::flann::Index::radiusSearch", vec![(pred!(mut, ["query", "indices", "dists", "radius", "maxResults", "params"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "int", "const cv::flann::SearchParams*"]), _)]),
	#[inline]
	fn radius_search(&mut self, query: &impl ToInputArray, indices: &mut impl ToOutputArray, dists: &mut impl ToOutputArray, radius: f64, max_results: i32, params: &impl crate::flann::SearchParamsTraitConst) -> Result<i32> {
		input_array_arg!(query);
		output_array_arg!(indices);
		output_array_arg!(dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_radiusSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_int_const_SearchParamsR(self.as_raw_mut_Index(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), radius, max_results, params.as_raw_SearchParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [IndexTrait::radius_search] function uses the following default values for its arguments:
	/// * params: SearchParams()
	// cv::flann::Index::radiusSearch(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:161
	// ("cv::flann::Index::radiusSearch", vec![(pred!(mut, ["query", "indices", "dists", "radius", "maxResults"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "int"]), _)]),
	#[inline]
	fn radius_search_def(&mut self, query: &impl ToInputArray, indices: &mut impl ToOutputArray, dists: &mut impl ToOutputArray, radius: f64, max_results: i32) -> Result<i32> {
		input_array_arg!(query);
		output_array_arg!(indices);
		output_array_arg!(dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_radiusSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_int(self.as_raw_mut_Index(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), radius, max_results, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// load(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:166
	// ("cv::flann::Index::load", vec![(pred!(mut, ["features", "filename"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
	#[inline]
	fn load(&mut self, features: &impl ToInputArray, filename: &str) -> Result<bool> {
		input_array_arg!(features);
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_load_const__InputArrayR_const_StringR(self.as_raw_mut_Index(), features.as_raw__InputArray(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:167
	// ("cv::flann::Index::release", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_release(self.as_raw_mut_Index(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Index /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:150
pub struct Index {
	ptr: *mut c_void,
}

opencv_type_boxed! { Index }

impl Drop for Index {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_flann_Index_delete(self.as_raw_mut_Index()) };
	}
}

unsafe impl Send for Index {}

impl crate::flann::IndexTraitConst for Index {
	#[inline] fn as_raw_Index(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexTrait for Index {
	#[inline] fn as_raw_mut_Index(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Index, crate::flann::IndexTraitConst, as_raw_Index, crate::flann::IndexTrait, as_raw_mut_Index }

impl Index {
	// Index()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:153
	// ("cv::flann::Index::Index", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::flann::Index> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_Index(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::Index::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * dist_type: cvflann::FLANN_DIST_L2
	// Index(InputArray, const IndexParams &, cvflann::flann_distance_t)(InputArray, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:154
	// ("cv::flann::Index::Index", vec![(pred!(mut, ["features", "params", "distType"], ["const cv::_InputArray*", "const cv::flann::IndexParams*", "cvflann::flann_distance_t"]), _)]),
	#[inline]
	pub fn new(features: &impl ToInputArray, params: &impl crate::flann::IndexParamsTraitConst, dist_type: crate::flann::flann_distance_t) -> Result<crate::flann::Index> {
		input_array_arg!(features);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_Index_const__InputArrayR_const_IndexParamsR_flann_distance_t(features.as_raw__InputArray(), params.as_raw_IndexParams(), dist_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::Index::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * dist_type: cvflann::FLANN_DIST_L2
	// cv::flann::Index::Index(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:154
	// ("cv::flann::Index::Index", vec![(pred!(mut, ["features", "params"], ["const cv::_InputArray*", "const cv::flann::IndexParams*"]), _)]),
	#[inline]
	pub fn new_def(features: &impl ToInputArray, params: &impl crate::flann::IndexParamsTraitConst) -> Result<crate::flann::Index> {
		input_array_arg!(features);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_Index_const__InputArrayR_const_IndexParamsR(features.as_raw__InputArray(), params.as_raw_IndexParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::Index::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Index {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Index")
			.finish()
	}
}

/// Constant methods for [crate::flann::IndexParams]
// IndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:71
pub trait IndexParamsTraitConst {
	fn as_raw_IndexParams(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * default_val: String()
	// getString(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:76
	// ("cv::flann::IndexParams::getString", vec![(pred!(const, ["key", "defaultVal"], ["const cv::String*", "const cv::String*"]), _)]),
	#[inline]
	fn get_string(&self, key: &str, default_val: &str) -> Result<String> {
		extern_container_arg!(key);
		extern_container_arg!(default_val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_getString_const_const_StringR_const_StringR(self.as_raw_IndexParams(), key.opencv_as_extern(), default_val.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [IndexParamsTraitConst::get_string] function uses the following default values for its arguments:
	/// * default_val: String()
	// cv::flann::IndexParams::getString(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:76
	// ("cv::flann::IndexParams::getString", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	#[inline]
	fn get_string_def(&self, key: &str) -> Result<String> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_getString_const_const_StringR(self.as_raw_IndexParams(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * default_val: -1
	// getInt(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:77
	// ("cv::flann::IndexParams::getInt", vec![(pred!(const, ["key", "defaultVal"], ["const cv::String*", "int"]), _)]),
	#[inline]
	fn get_int(&self, key: &str, default_val: i32) -> Result<i32> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_getInt_const_const_StringR_int(self.as_raw_IndexParams(), key.opencv_as_extern(), default_val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [IndexParamsTraitConst::get_int] function uses the following default values for its arguments:
	/// * default_val: -1
	// cv::flann::IndexParams::getInt(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:77
	// ("cv::flann::IndexParams::getInt", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	#[inline]
	fn get_int_def(&self, key: &str) -> Result<i32> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_getInt_const_const_StringR(self.as_raw_IndexParams(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * default_val: -1
	// getDouble(const String &, double)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:78
	// ("cv::flann::IndexParams::getDouble", vec![(pred!(const, ["key", "defaultVal"], ["const cv::String*", "double"]), _)]),
	#[inline]
	fn get_double(&self, key: &str, default_val: f64) -> Result<f64> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_getDouble_const_const_StringR_double(self.as_raw_IndexParams(), key.opencv_as_extern(), default_val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [IndexParamsTraitConst::get_double] function uses the following default values for its arguments:
	/// * default_val: -1
	// cv::flann::IndexParams::getDouble(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:78
	// ("cv::flann::IndexParams::getDouble", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	#[inline]
	fn get_double_def(&self, key: &str) -> Result<f64> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_getDouble_const_const_StringR(self.as_raw_IndexParams(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getAll(std::vector<String> &, std::vector<FlannIndexType> &, std::vector<String> &, std::vector<double> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:88
	// ("cv::flann::IndexParams::getAll", vec![(pred!(const, ["names", "types", "strValues", "numValues"], ["std::vector<cv::String>*", "std::vector<cv::flann::FlannIndexType>*", "std::vector<cv::String>*", "std::vector<double>*"]), _)]),
	#[inline]
	fn get_all(&self, names: &mut core::Vector<String>, types: &mut core::Vector<crate::flann::FlannIndexType>, str_values: &mut core::Vector<String>, num_values: &mut core::Vector<f64>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_getAll_const_vectorLStringGR_vectorLFlannIndexTypeGR_vectorLStringGR_vectorLdoubleGR(self.as_raw_IndexParams(), names.as_raw_mut_VectorOfString(), types.as_raw_mut_VectorOfFlannIndexType(), str_values.as_raw_mut_VectorOfString(), num_values.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::flann::IndexParams]
pub trait IndexParamsTrait: crate::flann::IndexParamsTraitConst {
	fn as_raw_mut_IndexParams(&mut self) -> *mut c_void;

	// cv::flann::IndexParams::params() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:93
	// ("cv::flann::IndexParams::params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn params(&mut self) -> *mut c_void {
		let ret = unsafe { sys::cv_flann_IndexParams_propParams(self.as_raw_mut_IndexParams()) };
		ret
	}

	// cv::flann::IndexParams::setParams(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:93
	// ("cv::flann::IndexParams::setParams", vec![(pred!(mut, ["val"], ["void*"]), _)]),
	#[inline]
	unsafe fn set_params(&mut self, val: *const c_void) {
		let ret = { sys::cv_flann_IndexParams_propParams_voidX(self.as_raw_mut_IndexParams(), val) };
		ret
	}

	// setString(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:80
	// ("cv::flann::IndexParams::setString", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const cv::String*"]), _)]),
	#[inline]
	fn set_string(&mut self, key: &str, value: &str) -> Result<()> {
		extern_container_arg!(key);
		extern_container_arg!(value);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setString_const_StringR_const_StringR(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setInt(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:81
	// ("cv::flann::IndexParams::setInt", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "int"]), _)]),
	#[inline]
	fn set_int(&mut self, key: &str, value: i32) -> Result<()> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setInt_const_StringR_int(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDouble(const String &, double)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:82
	// ("cv::flann::IndexParams::setDouble", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "double"]), _)]),
	#[inline]
	fn set_double(&mut self, key: &str, value: f64) -> Result<()> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setDouble_const_StringR_double(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFloat(const String &, float)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:83
	// ("cv::flann::IndexParams::setFloat", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "float"]), _)]),
	#[inline]
	fn set_float(&mut self, key: &str, value: f32) -> Result<()> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setFloat_const_StringR_float(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBool(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:84
	// ("cv::flann::IndexParams::setBool", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "bool"]), _)]),
	#[inline]
	fn set_bool(&mut self, key: &str, value: bool) -> Result<()> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setBool_const_StringR_bool(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setAlgorithm(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:85
	// ("cv::flann::IndexParams::setAlgorithm", vec![(pred!(mut, ["value"], ["int"]), _)]),
	#[inline]
	fn set_algorithm(&mut self, value: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setAlgorithm_int(self.as_raw_mut_IndexParams(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// IndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:71
pub struct IndexParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { IndexParams }

impl Drop for IndexParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_flann_IndexParams_delete(self.as_raw_mut_IndexParams()) };
	}
}

unsafe impl Send for IndexParams {}

impl crate::flann::IndexParamsTraitConst for IndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for IndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { IndexParams, crate::flann::IndexParamsTraitConst, as_raw_IndexParams, crate::flann::IndexParamsTrait, as_raw_mut_IndexParams }

impl IndexParams {
	// IndexParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:73
	// ("cv::flann::IndexParams::IndexParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::flann::IndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_IndexParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::IndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for IndexParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("IndexParams")
			.finish()
	}
}

/// Constant methods for [crate::flann::KDTreeIndexParams]
// KDTreeIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:100
pub trait KDTreeIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_KDTreeIndexParams(&self) -> *const c_void;

}

/// Mutable methods for [crate::flann::KDTreeIndexParams]
pub trait KDTreeIndexParamsTrait: crate::flann::IndexParamsTrait + crate::flann::KDTreeIndexParamsTraitConst {
	fn as_raw_mut_KDTreeIndexParams(&mut self) -> *mut c_void;

}

// KDTreeIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:100
pub struct KDTreeIndexParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { KDTreeIndexParams }

impl Drop for KDTreeIndexParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_flann_KDTreeIndexParams_delete(self.as_raw_mut_KDTreeIndexParams()) };
	}
}

unsafe impl Send for KDTreeIndexParams {}

impl crate::flann::IndexParamsTraitConst for KDTreeIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for KDTreeIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KDTreeIndexParams, crate::flann::IndexParamsTraitConst, as_raw_IndexParams, crate::flann::IndexParamsTrait, as_raw_mut_IndexParams }

impl crate::flann::KDTreeIndexParamsTraitConst for KDTreeIndexParams {
	#[inline] fn as_raw_KDTreeIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::KDTreeIndexParamsTrait for KDTreeIndexParams {
	#[inline] fn as_raw_mut_KDTreeIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KDTreeIndexParams, crate::flann::KDTreeIndexParamsTraitConst, as_raw_KDTreeIndexParams, crate::flann::KDTreeIndexParamsTrait, as_raw_mut_KDTreeIndexParams }

impl KDTreeIndexParams {
	/// ## C++ default parameters
	/// * trees: 4
	// KDTreeIndexParams(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:102
	// ("cv::flann::KDTreeIndexParams::KDTreeIndexParams", vec![(pred!(mut, ["trees"], ["int"]), _)]),
	#[inline]
	pub fn new(trees: i32) -> Result<crate::flann::KDTreeIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_KDTreeIndexParams_KDTreeIndexParams_int(trees, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::KDTreeIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * trees: 4
	// cv::flann::KDTreeIndexParams::KDTreeIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:102
	// ("cv::flann::KDTreeIndexParams::KDTreeIndexParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::flann::KDTreeIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_KDTreeIndexParams_KDTreeIndexParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::KDTreeIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { KDTreeIndexParams, crate::flann::IndexParams, cv_flann_KDTreeIndexParams_to_IndexParams }

impl std::fmt::Debug for KDTreeIndexParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("KDTreeIndexParams")
			.finish()
	}
}

/// Constant methods for [crate::flann::KMeansIndexParams]
// KMeansIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:128
pub trait KMeansIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_KMeansIndexParams(&self) -> *const c_void;

}

/// Mutable methods for [crate::flann::KMeansIndexParams]
pub trait KMeansIndexParamsTrait: crate::flann::IndexParamsTrait + crate::flann::KMeansIndexParamsTraitConst {
	fn as_raw_mut_KMeansIndexParams(&mut self) -> *mut c_void;

}

// KMeansIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:128
pub struct KMeansIndexParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { KMeansIndexParams }

impl Drop for KMeansIndexParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_flann_KMeansIndexParams_delete(self.as_raw_mut_KMeansIndexParams()) };
	}
}

unsafe impl Send for KMeansIndexParams {}

impl crate::flann::IndexParamsTraitConst for KMeansIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for KMeansIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KMeansIndexParams, crate::flann::IndexParamsTraitConst, as_raw_IndexParams, crate::flann::IndexParamsTrait, as_raw_mut_IndexParams }

impl crate::flann::KMeansIndexParamsTraitConst for KMeansIndexParams {
	#[inline] fn as_raw_KMeansIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::KMeansIndexParamsTrait for KMeansIndexParams {
	#[inline] fn as_raw_mut_KMeansIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KMeansIndexParams, crate::flann::KMeansIndexParamsTraitConst, as_raw_KMeansIndexParams, crate::flann::KMeansIndexParamsTrait, as_raw_mut_KMeansIndexParams }

impl KMeansIndexParams {
	/// ## C++ default parameters
	/// * branching: 32
	/// * iterations: 11
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * cb_index: 0.2f
	// KMeansIndexParams(int, int, cvflann::flann_centers_init_t, float)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:130
	// ("cv::flann::KMeansIndexParams::KMeansIndexParams", vec![(pred!(mut, ["branching", "iterations", "centers_init", "cb_index"], ["int", "int", "cvflann::flann_centers_init_t", "float"]), _)]),
	#[inline]
	pub fn new(branching: i32, iterations: i32, centers_init: crate::flann::flann_centers_init_t, cb_index: f32) -> Result<crate::flann::KMeansIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_KMeansIndexParams_KMeansIndexParams_int_int_flann_centers_init_t_float(branching, iterations, centers_init, cb_index, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::KMeansIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * branching: 32
	/// * iterations: 11
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * cb_index: 0.2f
	// cv::flann::KMeansIndexParams::KMeansIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:130
	// ("cv::flann::KMeansIndexParams::KMeansIndexParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::flann::KMeansIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_KMeansIndexParams_KMeansIndexParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::KMeansIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { KMeansIndexParams, crate::flann::IndexParams, cv_flann_KMeansIndexParams_to_IndexParams }

impl std::fmt::Debug for KMeansIndexParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("KMeansIndexParams")
			.finish()
	}
}

/// Constant methods for [crate::flann::LinearIndexParams]
// LinearIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:105
pub trait LinearIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_LinearIndexParams(&self) -> *const c_void;

}

/// Mutable methods for [crate::flann::LinearIndexParams]
pub trait LinearIndexParamsTrait: crate::flann::IndexParamsTrait + crate::flann::LinearIndexParamsTraitConst {
	fn as_raw_mut_LinearIndexParams(&mut self) -> *mut c_void;

}

// LinearIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:105
pub struct LinearIndexParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { LinearIndexParams }

impl Drop for LinearIndexParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_flann_LinearIndexParams_delete(self.as_raw_mut_LinearIndexParams()) };
	}
}

unsafe impl Send for LinearIndexParams {}

impl crate::flann::IndexParamsTraitConst for LinearIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for LinearIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LinearIndexParams, crate::flann::IndexParamsTraitConst, as_raw_IndexParams, crate::flann::IndexParamsTrait, as_raw_mut_IndexParams }

impl crate::flann::LinearIndexParamsTraitConst for LinearIndexParams {
	#[inline] fn as_raw_LinearIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::LinearIndexParamsTrait for LinearIndexParams {
	#[inline] fn as_raw_mut_LinearIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LinearIndexParams, crate::flann::LinearIndexParamsTraitConst, as_raw_LinearIndexParams, crate::flann::LinearIndexParamsTrait, as_raw_mut_LinearIndexParams }

impl LinearIndexParams {
	// LinearIndexParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:107
	// ("cv::flann::LinearIndexParams::LinearIndexParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::flann::LinearIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_LinearIndexParams_LinearIndexParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::LinearIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { LinearIndexParams, crate::flann::IndexParams, cv_flann_LinearIndexParams_to_IndexParams }

impl std::fmt::Debug for LinearIndexParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LinearIndexParams")
			.finish()
	}
}

/// Constant methods for [crate::flann::LshIndexParams]
// LshIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:134
pub trait LshIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_LshIndexParams(&self) -> *const c_void;

}

/// Mutable methods for [crate::flann::LshIndexParams]
pub trait LshIndexParamsTrait: crate::flann::IndexParamsTrait + crate::flann::LshIndexParamsTraitConst {
	fn as_raw_mut_LshIndexParams(&mut self) -> *mut c_void;

}

// LshIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:134
pub struct LshIndexParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { LshIndexParams }

impl Drop for LshIndexParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_flann_LshIndexParams_delete(self.as_raw_mut_LshIndexParams()) };
	}
}

unsafe impl Send for LshIndexParams {}

impl crate::flann::IndexParamsTraitConst for LshIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for LshIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LshIndexParams, crate::flann::IndexParamsTraitConst, as_raw_IndexParams, crate::flann::IndexParamsTrait, as_raw_mut_IndexParams }

impl crate::flann::LshIndexParamsTraitConst for LshIndexParams {
	#[inline] fn as_raw_LshIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::LshIndexParamsTrait for LshIndexParams {
	#[inline] fn as_raw_mut_LshIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LshIndexParams, crate::flann::LshIndexParamsTraitConst, as_raw_LshIndexParams, crate::flann::LshIndexParamsTrait, as_raw_mut_LshIndexParams }

impl LshIndexParams {
	// LshIndexParams(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:136
	// ("cv::flann::LshIndexParams::LshIndexParams", vec![(pred!(mut, ["table_number", "key_size", "multi_probe_level"], ["int", "int", "int"]), _)]),
	#[inline]
	pub fn new(table_number: i32, key_size: i32, multi_probe_level: i32) -> Result<crate::flann::LshIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_LshIndexParams_LshIndexParams_int_int_int(table_number, key_size, multi_probe_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::LshIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { LshIndexParams, crate::flann::IndexParams, cv_flann_LshIndexParams_to_IndexParams }

impl std::fmt::Debug for LshIndexParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LshIndexParams")
			.finish()
	}
}

/// Constant methods for [crate::flann::SavedIndexParams]
// SavedIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:139
pub trait SavedIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_SavedIndexParams(&self) -> *const c_void;

}

/// Mutable methods for [crate::flann::SavedIndexParams]
pub trait SavedIndexParamsTrait: crate::flann::IndexParamsTrait + crate::flann::SavedIndexParamsTraitConst {
	fn as_raw_mut_SavedIndexParams(&mut self) -> *mut c_void;

}

// SavedIndexParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:139
pub struct SavedIndexParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { SavedIndexParams }

impl Drop for SavedIndexParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_flann_SavedIndexParams_delete(self.as_raw_mut_SavedIndexParams()) };
	}
}

unsafe impl Send for SavedIndexParams {}

impl crate::flann::IndexParamsTraitConst for SavedIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for SavedIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SavedIndexParams, crate::flann::IndexParamsTraitConst, as_raw_IndexParams, crate::flann::IndexParamsTrait, as_raw_mut_IndexParams }

impl crate::flann::SavedIndexParamsTraitConst for SavedIndexParams {
	#[inline] fn as_raw_SavedIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::SavedIndexParamsTrait for SavedIndexParams {
	#[inline] fn as_raw_mut_SavedIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SavedIndexParams, crate::flann::SavedIndexParamsTraitConst, as_raw_SavedIndexParams, crate::flann::SavedIndexParamsTrait, as_raw_mut_SavedIndexParams }

impl SavedIndexParams {
	// SavedIndexParams(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:141
	// ("cv::flann::SavedIndexParams::SavedIndexParams", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn new(filename: &str) -> Result<crate::flann::SavedIndexParams> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_SavedIndexParams_SavedIndexParams_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::SavedIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SavedIndexParams, crate::flann::IndexParams, cv_flann_SavedIndexParams_to_IndexParams }

impl std::fmt::Debug for SavedIndexParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SavedIndexParams")
			.finish()
	}
}

/// Constant methods for [crate::flann::SearchParams]
// SearchParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:144
pub trait SearchParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_SearchParams(&self) -> *const c_void;

}

/// Mutable methods for [crate::flann::SearchParams]
pub trait SearchParamsTrait: crate::flann::IndexParamsTrait + crate::flann::SearchParamsTraitConst {
	fn as_raw_mut_SearchParams(&mut self) -> *mut c_void;

}

// SearchParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:144
pub struct SearchParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { SearchParams }

impl Drop for SearchParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_flann_SearchParams_delete(self.as_raw_mut_SearchParams()) };
	}
}

unsafe impl Send for SearchParams {}

impl crate::flann::IndexParamsTraitConst for SearchParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for SearchParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SearchParams, crate::flann::IndexParamsTraitConst, as_raw_IndexParams, crate::flann::IndexParamsTrait, as_raw_mut_IndexParams }

impl crate::flann::SearchParamsTraitConst for SearchParams {
	#[inline] fn as_raw_SearchParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::SearchParamsTrait for SearchParams {
	#[inline] fn as_raw_mut_SearchParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SearchParams, crate::flann::SearchParamsTraitConst, as_raw_SearchParams, crate::flann::SearchParamsTrait, as_raw_mut_SearchParams }

impl SearchParams {
	// SearchParams(int, float, bool, bool)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:146
	// ("cv::flann::SearchParams::SearchParams", vec![(pred!(mut, ["checks", "eps", "sorted", "explore_all_trees"], ["int", "float", "bool", "bool"]), _)]),
	#[inline]
	pub fn new(checks: i32, eps: f32, sorted: bool, explore_all_trees: bool) -> Result<crate::flann::SearchParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_SearchParams_SearchParams_int_float_bool_bool(checks, eps, sorted, explore_all_trees, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::SearchParams::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * checks: 32
	/// * eps: 0
	/// * sorted: true
	// SearchParams(int, float, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:147
	// ("cv::flann::SearchParams::SearchParams", vec![(pred!(mut, ["checks", "eps", "sorted"], ["int", "float", "bool"]), _)]),
	#[inline]
	pub fn new_1(checks: i32, eps: f32, sorted: bool) -> Result<crate::flann::SearchParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_SearchParams_SearchParams_int_float_bool(checks, eps, sorted, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::SearchParams::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * checks: 32
	/// * eps: 0
	/// * sorted: true
	// cv::flann::SearchParams::SearchParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:147
	// ("cv::flann::SearchParams::SearchParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::flann::SearchParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_SearchParams_SearchParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::SearchParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SearchParams, crate::flann::IndexParams, cv_flann_SearchParams_to_IndexParams }

impl std::fmt::Debug for SearchParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SearchParams")
			.finish()
	}
}

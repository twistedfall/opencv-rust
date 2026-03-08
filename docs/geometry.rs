pub mod geometry {
	//! # Computational geometry primitives module.
	use crate::mod_prelude::*;
	use crate::{core, sys, types};
	pub mod prelude {
		pub use super::{LevMarqTrait, LevMarqTraitConst, LevMarq_ReportTrait, LevMarq_ReportTraitConst, LevMarq_SettingsTrait, LevMarq_SettingsTraitConst, RegionGrowing3DTrait, RegionGrowing3DTraitConst, SACSegmentationTrait, SACSegmentationTraitConst, Subdiv2DTrait, Subdiv2DTraitConst};
	}

	pub const COV_POLISHER: i32 = 3;
	/// distance = max(|x1-x2|,|y1-y2|)
	pub const DIST_C: i32 = 3;
	/// distance = c^2(|x|/c-log(1+|x|/c)), c = 1.3998
	pub const DIST_FAIR: i32 = 5;
	/// distance = |x|<c ? x^2/2 : c(|x|-c/2), c=1.345
	pub const DIST_HUBER: i32 = 7;
	/// distance = |x1-x2| + |y1-y2|
	pub const DIST_L1: i32 = 1;
	/// L1-L2 metric: distance = 2(sqrt(1+x*x/2) - 1))
	pub const DIST_L12: i32 = 4;
	/// the simple euclidean distance
	pub const DIST_L2: i32 = 2;
	/// User defined distance
	pub const DIST_USER: i32 = -1;
	/// distance = c^2/2(1-exp(-(x/c)^2)), c = 2.9846
	pub const DIST_WELSCH: i32 = 6;
	/// 7-point algorithm
	pub const FM_7POINT: i32 = 1;
	/// 8-point algorithm
	pub const FM_8POINT: i32 = 2;
	/// least-median algorithm. 7-point algorithm is used.
	pub const FM_LMEDS: i32 = 4;
	/// RANSAC algorithm. It needs at least 15 points. 7-point algorithm is used.
	pub const FM_RANSAC: i32 = 8;
	/// One of the rectangle is fully enclosed in the other
	pub const INTERSECT_FULL: i32 = 2;
	/// No intersection
	pub const INTERSECT_NONE: i32 = 0;
	/// There is a partial intersection
	pub const INTERSECT_PARTIAL: i32 = 1;
	/// least-median of squares algorithm
	pub const LMEDS: i32 = 4;
	pub const LOCAL_OPTIM_GC: i32 = 3;
	pub const LOCAL_OPTIM_INNER_AND_ITER_LO: i32 = 2;
	pub const LOCAL_OPTIM_INNER_LO: i32 = 1;
	pub const LOCAL_OPTIM_NULL: i32 = 0;
	pub const LOCAL_OPTIM_SIGMA: i32 = 4;
	pub const LSQ_POLISHER: i32 = 1;
	pub const MAGSAC: i32 = 2;
	pub const MatrixType_AUTO: i32 = 0;
	pub const MatrixType_DENSE: i32 = 1;
	pub const MatrixType_SPARSE: i32 = 2;
	pub const NEIGH_FLANN_KNN: i32 = 0;
	pub const NEIGH_FLANN_RADIUS: i32 = 2;
	pub const NEIGH_GRID: i32 = 1;
	pub const NONE_POLISHER: i32 = 0;
	/// RANSAC algorithm
	pub const RANSAC: i32 = 8;
	/// RHO algorithm
	pub const RHO: i32 = 16;
	/// The RANSAC algorithm described in [fischler1981random](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_fischler1981random).
	pub const SAC_METHOD_RANSAC: i32 = 0;
	/// The 3D PLANE model coefficients in list **[a, b, c, d]**,
	/// corresponding to the coefficients of equation
	/// ![inline formula](https://latex.codecogs.com/png.latex?%20ax%20%2B%20by%20%2B%20cz%20%2B%20d%20%3D%200%20).
	pub const SAC_MODEL_PLANE: i32 = 0;
	/// The 3D SPHERE model coefficients in list **[center_x, center_y, center_z, radius]**,
	/// corresponding to the coefficients of equation
	/// ![inline formula](https://latex.codecogs.com/png.latex?%20%28x%20%2D%20center%5C%5Fx%29%5E2%20%2B%20%28y%20%2D%20center%5C%5Fy%29%5E2%20%2B%20%28z%20%2D%20center%5C%5Fz%29%5E2%20%3D%20radius%5E2%20).
	pub const SAC_MODEL_SPHERE: i32 = 1;
	pub const SAMPLING_NAPSAC: i32 = 2;
	pub const SAMPLING_PROGRESSIVE_NAPSAC: i32 = 1;
	pub const SAMPLING_PROSAC: i32 = 3;
	pub const SAMPLING_UNIFORM: i32 = 0;
	pub const SCORE_METHOD_LMEDS: i32 = 3;
	pub const SCORE_METHOD_MAGSAC: i32 = 2;
	pub const SCORE_METHOD_MSAC: i32 = 1;
	pub const SCORE_METHOD_RANSAC: i32 = 0;
	/// An Efficient Algebraic Solution to the Perspective-Three-Point Problem [Ke17](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Ke17)
	pub const SOLVEPNP_AP3P: i32 = 3;
	/// EPnP: Efficient Perspective-n-Point Camera Pose Estimation [lepetit2009epnp](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_lepetit2009epnp)
	pub const SOLVEPNP_EPNP: i32 = 1;
	/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Collins14) 
	///
	/// Object points must be coplanar.
	pub const SOLVEPNP_IPPE: i32 = 4;
	/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Collins14) 
	///
	/// This is a special case suitable for marker pose estimation.
	///
	/// 4 coplanar object points must be defined in the following order:
	///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
	pub const SOLVEPNP_IPPE_SQUARE: i32 = 5;
	/// Pose refinement using non-linear Levenberg-Marquardt minimization scheme [Madsen04](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade13) 
	///
	/// Initial solution for non-planar "objectPoints" needs at least 6 points and uses the DLT algorithm. 
	///
	/// Initial solution for planar "objectPoints" needs at least 4 points and uses pose from homography decomposition.
	pub const SOLVEPNP_ITERATIVE: i32 = 0;
	/// Used for count
	pub const SOLVEPNP_MAX_COUNT: i32 = 7;
	/// Revisiting the P3P Problem [ding2023revisiting](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_ding2023revisiting)
	pub const SOLVEPNP_P3P: i32 = 2;
	/// SQPnP: A Consistently Fast and Globally OptimalSolution to the Perspective-n-Point Problem [Terzakis2020SQPnP](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Terzakis2020SQPnP)
	pub const SOLVEPNP_SQPNP: i32 = 6;
	pub const Subdiv2D_NEXT_AROUND_DST: i32 = 34;
	pub const Subdiv2D_NEXT_AROUND_LEFT: i32 = 19;
	pub const Subdiv2D_NEXT_AROUND_ORG: i32 = 0;
	pub const Subdiv2D_NEXT_AROUND_RIGHT: i32 = 49;
	pub const Subdiv2D_PREV_AROUND_DST: i32 = 51;
	pub const Subdiv2D_PREV_AROUND_LEFT: i32 = 32;
	pub const Subdiv2D_PREV_AROUND_ORG: i32 = 17;
	pub const Subdiv2D_PREV_AROUND_RIGHT: i32 = 2;
	/// Point location error
	pub const Subdiv2D_PTLOC_ERROR: i32 = -2;
	/// Point inside some facet
	pub const Subdiv2D_PTLOC_INSIDE: i32 = 0;
	/// Point on some edge
	pub const Subdiv2D_PTLOC_ON_EDGE: i32 = 2;
	/// Point outside the subdivision bounding rect
	pub const Subdiv2D_PTLOC_OUTSIDE_RECT: i32 = -1;
	/// Point coincides with one of the subdivision vertices
	pub const Subdiv2D_PTLOC_VERTEX: i32 = 1;
	/// USAC, accurate settings
	pub const USAC_ACCURATE: i32 = 36;
	/// USAC algorithm, default settings
	pub const USAC_DEFAULT: i32 = 32;
	/// USAC, fast settings
	pub const USAC_FAST: i32 = 35;
	/// USAC, fundamental matrix 8 points
	pub const USAC_FM_8PTS: i32 = 34;
	/// USAC, runs MAGSAC++
	pub const USAC_MAGSAC: i32 = 38;
	/// USAC, parallel version
	pub const USAC_PARALLEL: i32 = 33;
	/// USAC, sorted points, runs PROSAC
	pub const USAC_PROSAC: i32 = 37;
	pub const VariableType_LINEAR: i32 = 0;
	pub const VariableType_SE3: i32 = 2;
	pub const VariableType_SO3: i32 = 1;
	/// Distance types for Distance Transform and M-estimators
	/// ## See also
	/// distanceTransform, fitLine
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum DistanceTypes {
		/// User defined distance
		DIST_USER = -1,
		/// distance = |x1-x2| + |y1-y2|
		DIST_L1 = 1,
		/// the simple euclidean distance
		DIST_L2 = 2,
		/// distance = max(|x1-x2|,|y1-y2|)
		DIST_C = 3,
		/// L1-L2 metric: distance = 2(sqrt(1+x*x/2) - 1))
		DIST_L12 = 4,
		/// distance = c^2(|x|/c-log(1+|x|/c)), c = 1.3998
		DIST_FAIR = 5,
		/// distance = c^2/2(1-exp(-(x/c)^2)), c = 2.9846
		DIST_WELSCH = 6,
		/// distance = |x|<c ? x^2/2 : c(|x|-c/2), c=1.345
		DIST_HUBER = 7,
	}

	opencv_type_enum! { crate::geometry::DistanceTypes { DIST_USER, DIST_L1, DIST_L2, DIST_C, DIST_L12, DIST_FAIR, DIST_WELSCH, DIST_HUBER } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum LocalOptimMethod {
		LOCAL_OPTIM_NULL = 0,
		LOCAL_OPTIM_INNER_LO = 1,
		LOCAL_OPTIM_INNER_AND_ITER_LO = 2,
		LOCAL_OPTIM_GC = 3,
		LOCAL_OPTIM_SIGMA = 4,
	}

	opencv_type_enum! { crate::geometry::LocalOptimMethod { LOCAL_OPTIM_NULL, LOCAL_OPTIM_INNER_LO, LOCAL_OPTIM_INNER_AND_ITER_LO, LOCAL_OPTIM_GC, LOCAL_OPTIM_SIGMA } }

	/// Type of matrix used in LevMarq solver
	///
	/// Matrix type can be dense, sparse or chosen automatically based on a matrix size, performance considerations or backend availability.
	///
	/// Note: only dense matrix is now supported
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum MatrixType {
		AUTO = 0,
		DENSE = 1,
		SPARSE = 2,
	}

	opencv_type_enum! { crate::geometry::MatrixType { AUTO, DENSE, SPARSE } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum NeighborSearchMethod {
		NEIGH_FLANN_KNN = 0,
		NEIGH_GRID = 1,
		NEIGH_FLANN_RADIUS = 2,
	}

	opencv_type_enum! { crate::geometry::NeighborSearchMethod { NEIGH_FLANN_KNN, NEIGH_GRID, NEIGH_FLANN_RADIUS } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum PolishingMethod {
		NONE_POLISHER = 0,
		LSQ_POLISHER = 1,
		MAGSAC = 2,
		COV_POLISHER = 3,
	}

	opencv_type_enum! { crate::geometry::PolishingMethod { NONE_POLISHER, LSQ_POLISHER, MAGSAC, COV_POLISHER } }

	/// types of intersection between rectangles
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum RectanglesIntersectTypes {
		/// No intersection
		INTERSECT_NONE = 0,
		/// There is a partial intersection
		INTERSECT_PARTIAL = 1,
		/// One of the rectangle is fully enclosed in the other
		INTERSECT_FULL = 2,
	}

	opencv_type_enum! { crate::geometry::RectanglesIntersectTypes { INTERSECT_NONE, INTERSECT_PARTIAL, INTERSECT_FULL } }

	/// type of the robust estimation algorithm
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum SacMethod {
		/// The RANSAC algorithm described in [fischler1981random](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_fischler1981random).
		SAC_METHOD_RANSAC = 0,
	}

	opencv_type_enum! { crate::geometry::SacMethod { SAC_METHOD_RANSAC } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum SacModelType {
		/// The 3D PLANE model coefficients in list **[a, b, c, d]**,
		/// corresponding to the coefficients of equation
		/// ![inline formula](https://latex.codecogs.com/png.latex?%20ax%20%2B%20by%20%2B%20cz%20%2B%20d%20%3D%200%20).
		SAC_MODEL_PLANE = 0,
		/// The 3D SPHERE model coefficients in list **[center_x, center_y, center_z, radius]**,
		/// corresponding to the coefficients of equation
		/// ![inline formula](https://latex.codecogs.com/png.latex?%20%28x%20%2D%20center%5C%5Fx%29%5E2%20%2B%20%28y%20%2D%20center%5C%5Fy%29%5E2%20%2B%20%28z%20%2D%20center%5C%5Fz%29%5E2%20%3D%20radius%5E2%20).
		SAC_MODEL_SPHERE = 1,
	}

	opencv_type_enum! { crate::geometry::SacModelType { SAC_MODEL_PLANE, SAC_MODEL_SPHERE } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum SamplingMethod {
		SAMPLING_UNIFORM = 0,
		SAMPLING_PROGRESSIVE_NAPSAC = 1,
		SAMPLING_NAPSAC = 2,
		SAMPLING_PROSAC = 3,
	}

	opencv_type_enum! { crate::geometry::SamplingMethod { SAMPLING_UNIFORM, SAMPLING_PROGRESSIVE_NAPSAC, SAMPLING_NAPSAC, SAMPLING_PROSAC } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ScoreMethod {
		SCORE_METHOD_RANSAC = 0,
		SCORE_METHOD_MSAC = 1,
		SCORE_METHOD_MAGSAC = 2,
		SCORE_METHOD_LMEDS = 3,
	}

	opencv_type_enum! { crate::geometry::ScoreMethod { SCORE_METHOD_RANSAC, SCORE_METHOD_MSAC, SCORE_METHOD_MAGSAC, SCORE_METHOD_LMEDS } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum SolvePnPMethod {
		/// Pose refinement using non-linear Levenberg-Marquardt minimization scheme [Madsen04](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade13) 
		///
		/// Initial solution for non-planar "objectPoints" needs at least 6 points and uses the DLT algorithm. 
		///
		/// Initial solution for planar "objectPoints" needs at least 4 points and uses pose from homography decomposition.
		SOLVEPNP_ITERATIVE = 0,
		/// EPnP: Efficient Perspective-n-Point Camera Pose Estimation [lepetit2009epnp](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_lepetit2009epnp)
		SOLVEPNP_EPNP = 1,
		/// Revisiting the P3P Problem [ding2023revisiting](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_ding2023revisiting)
		SOLVEPNP_P3P = 2,
		/// An Efficient Algebraic Solution to the Perspective-Three-Point Problem [Ke17](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Ke17)
		SOLVEPNP_AP3P = 3,
		/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Collins14) 
		///
		/// Object points must be coplanar.
		SOLVEPNP_IPPE = 4,
		/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Collins14) 
		///
		/// This is a special case suitable for marker pose estimation.
		///
		/// 4 coplanar object points must be defined in the following order:
		///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
		///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
		///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
		///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
		SOLVEPNP_IPPE_SQUARE = 5,
		/// SQPnP: A Consistently Fast and Globally OptimalSolution to the Perspective-n-Point Problem [Terzakis2020SQPnP](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Terzakis2020SQPnP)
		SOLVEPNP_SQPNP = 6,
		/// Used for count
		SOLVEPNP_MAX_COUNT = 7,
	}

	opencv_type_enum! { crate::geometry::SolvePnPMethod { SOLVEPNP_ITERATIVE, SOLVEPNP_EPNP, SOLVEPNP_P3P, SOLVEPNP_AP3P, SOLVEPNP_IPPE, SOLVEPNP_IPPE_SQUARE, SOLVEPNP_SQPNP, SOLVEPNP_MAX_COUNT } }

	/// Type of variables used in LevMarq solver
	///
	/// Variables can be linear, rotation (SO(3) group) or rigid transformation (SE(3) group) with corresponding jacobians and exponential updates.
	///
	/// Note: only linear variables are now supported
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum VariableType {
		LINEAR = 0,
		SO3 = 1,
		SE3 = 2,
	}

	opencv_type_enum! { crate::geometry::VariableType { LINEAR, SO3, SE3 } }

	/// Calculates seven Hu invariants.
	///
	/// The function calculates seven Hu invariants (introduced in [Hu62](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Hu62); see also
	/// <https://en.wikipedia.org/wiki/Image_moment>) defined as:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20hu%5B0%5D%3D%20%5Ceta%20%5F%7B20%7D%2B%20%5Ceta%20%5F%7B02%7D%20%5C%5C%20hu%5B1%5D%3D%28%20%5Ceta%20%5F%7B20%7D%2D%20%5Ceta%20%5F%7B02%7D%29%5E%7B2%7D%2B4%20%5Ceta%20%5F%7B11%7D%5E%7B2%7D%20%5C%5C%20hu%5B2%5D%3D%28%20%5Ceta%20%5F%7B30%7D%2D3%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2B%20%283%20%5Ceta%20%5F%7B21%7D%2D%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%20%5C%5C%20hu%5B3%5D%3D%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2B%20%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%20%5C%5C%20hu%5B4%5D%3D%28%20%5Ceta%20%5F%7B30%7D%2D3%20%5Ceta%20%5F%7B12%7D%29%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5B%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2D3%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%5D%2B%283%20%5Ceta%20%5F%7B21%7D%2D%20%5Ceta%20%5F%7B03%7D%29%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5B3%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2D%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%5D%20%5C%5C%20hu%5B5%5D%3D%28%20%5Ceta%20%5F%7B20%7D%2D%20%5Ceta%20%5F%7B02%7D%29%5B%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2D%20%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%5D%2B4%20%5Ceta%20%5F%7B11%7D%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%20%5C%5C%20hu%5B6%5D%3D%283%20%5Ceta%20%5F%7B21%7D%2D%20%5Ceta%20%5F%7B03%7D%29%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5B3%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2D%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%5D%2D%28%20%5Ceta%20%5F%7B30%7D%2D3%20%5Ceta%20%5F%7B12%7D%29%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5B3%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2D%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%5D%20%5C%5C%20%5Cend%7Barray%7D)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?%5Ceta%5F%7Bji%7D) stands for ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BMoments%3A%3Anu%7D%5F%7Bji%7D) .
	///
	/// These values are proved to be invariants to the image scale, rotation, and reflection except the
	/// seventh one, whose sign is changed by reflection. This invariance is proved with the assumption of
	/// infinite image resolution. In case of raster images, the computed Hu invariants for the original and
	/// transformed images are a bit different.
	///
	/// ## Parameters
	/// * moments: Input moments computed with moments .
	/// * hu: Output Hu invariants.
	/// ## See also
	/// matchShapes
	///
	/// ## Overloaded parameters
	#[inline]
	pub fn hu_moments_1(m: core::Moments, hu: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(hu);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HuMoments_const_MomentsR_const__OutputArrayR(&m, hu.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates seven Hu invariants.
	///
	/// The function calculates seven Hu invariants (introduced in [Hu62](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Hu62); see also
	/// <https://en.wikipedia.org/wiki/Image_moment>) defined as:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20hu%5B0%5D%3D%20%5Ceta%20%5F%7B20%7D%2B%20%5Ceta%20%5F%7B02%7D%20%5C%5C%20hu%5B1%5D%3D%28%20%5Ceta%20%5F%7B20%7D%2D%20%5Ceta%20%5F%7B02%7D%29%5E%7B2%7D%2B4%20%5Ceta%20%5F%7B11%7D%5E%7B2%7D%20%5C%5C%20hu%5B2%5D%3D%28%20%5Ceta%20%5F%7B30%7D%2D3%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2B%20%283%20%5Ceta%20%5F%7B21%7D%2D%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%20%5C%5C%20hu%5B3%5D%3D%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2B%20%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%20%5C%5C%20hu%5B4%5D%3D%28%20%5Ceta%20%5F%7B30%7D%2D3%20%5Ceta%20%5F%7B12%7D%29%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5B%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2D3%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%5D%2B%283%20%5Ceta%20%5F%7B21%7D%2D%20%5Ceta%20%5F%7B03%7D%29%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5B3%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2D%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%5D%20%5C%5C%20hu%5B5%5D%3D%28%20%5Ceta%20%5F%7B20%7D%2D%20%5Ceta%20%5F%7B02%7D%29%5B%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2D%20%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%5D%2B4%20%5Ceta%20%5F%7B11%7D%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%20%5C%5C%20hu%5B6%5D%3D%283%20%5Ceta%20%5F%7B21%7D%2D%20%5Ceta%20%5F%7B03%7D%29%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5B3%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2D%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%5D%2D%28%20%5Ceta%20%5F%7B30%7D%2D3%20%5Ceta%20%5F%7B12%7D%29%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5B3%28%20%5Ceta%20%5F%7B30%7D%2B%20%5Ceta%20%5F%7B12%7D%29%5E%7B2%7D%2D%28%20%5Ceta%20%5F%7B21%7D%2B%20%5Ceta%20%5F%7B03%7D%29%5E%7B2%7D%5D%20%5C%5C%20%5Cend%7Barray%7D)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?%5Ceta%5F%7Bji%7D) stands for ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BMoments%3A%3Anu%7D%5F%7Bji%7D) .
	///
	/// These values are proved to be invariants to the image scale, rotation, and reflection except the
	/// seventh one, whose sign is changed by reflection. This invariance is proved with the assumption of
	/// infinite image resolution. In case of raster images, the computed Hu invariants for the original and
	/// transformed images are a bit different.
	///
	/// ## Parameters
	/// * moments: Input moments computed with moments .
	/// * hu: Output Hu invariants.
	/// ## See also
	/// matchShapes
	#[inline]
	pub fn hu_moments(moments: core::Moments, hu: &mut [f64; 7]) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HuMoments_const_MomentsR_doubleXX(&moments, hu, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes an RQ decomposition of 3x3 matrices.
	///
	/// ## Parameters
	/// * src: 3x3 input matrix.
	/// * mtxR: Output 3x3 upper-triangular matrix.
	/// * mtxQ: Output 3x3 orthogonal matrix.
	/// * Qx: Optional output 3x3 rotation matrix around x-axis.
	/// * Qy: Optional output 3x3 rotation matrix around y-axis.
	/// * Qz: Optional output 3x3 rotation matrix around z-axis.
	///
	/// The function computes a RQ decomposition using the given rotations. This function is used in
	/// [decompose_projection_matrix] to decompose the left 3x3 submatrix of a projection matrix into a camera
	/// and a rotation matrix.
	///
	/// It optionally returns three rotation matrices, one for each axis, and the three Euler angles in
	/// degrees (as the return value) that could be used in OpenGL. Note, there is always more than one
	/// sequence of rotations about the three principal axes that results in the same orientation of an
	/// object, e.g. see [Slabaugh](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned three rotation matrices and corresponding three Euler angles
	/// are only one of the possible solutions.
	///
	/// ## Note
	/// This alternative version of [rq_decomp3x3] function uses the following default values for its arguments:
	/// * qx: noArray()
	/// * qy: noArray()
	/// * qz: noArray()
	#[inline]
	pub fn rq_decomp3x3_def(src: &impl ToInputArray, mtx_r: &mut impl ToOutputArray, mtx_q: &mut impl ToOutputArray) -> Result<core::Vec3d> {
		input_array_arg!(src);
		output_array_arg!(mtx_r);
		output_array_arg!(mtx_q);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RQDecomp3x3_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), mtx_r.as_raw__OutputArray(), mtx_q.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes an RQ decomposition of 3x3 matrices.
	///
	/// ## Parameters
	/// * src: 3x3 input matrix.
	/// * mtxR: Output 3x3 upper-triangular matrix.
	/// * mtxQ: Output 3x3 orthogonal matrix.
	/// * Qx: Optional output 3x3 rotation matrix around x-axis.
	/// * Qy: Optional output 3x3 rotation matrix around y-axis.
	/// * Qz: Optional output 3x3 rotation matrix around z-axis.
	///
	/// The function computes a RQ decomposition using the given rotations. This function is used in
	/// [decompose_projection_matrix] to decompose the left 3x3 submatrix of a projection matrix into a camera
	/// and a rotation matrix.
	///
	/// It optionally returns three rotation matrices, one for each axis, and the three Euler angles in
	/// degrees (as the return value) that could be used in OpenGL. Note, there is always more than one
	/// sequence of rotations about the three principal axes that results in the same orientation of an
	/// object, e.g. see [Slabaugh](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned three rotation matrices and corresponding three Euler angles
	/// are only one of the possible solutions.
	///
	/// ## C++ default parameters
	/// * qx: noArray()
	/// * qy: noArray()
	/// * qz: noArray()
	#[inline]
	pub fn rq_decomp3x3(src: &impl ToInputArray, mtx_r: &mut impl ToOutputArray, mtx_q: &mut impl ToOutputArray, qx: &mut impl ToOutputArray, qy: &mut impl ToOutputArray, qz: &mut impl ToOutputArray) -> Result<core::Vec3d> {
		input_array_arg!(src);
		output_array_arg!(mtx_r);
		output_array_arg!(mtx_q);
		output_array_arg!(qx);
		output_array_arg!(qy);
		output_array_arg!(qz);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RQDecomp3x3_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), mtx_r.as_raw__OutputArray(), mtx_q.as_raw__OutputArray(), qx.as_raw__OutputArray(), qy.as_raw__OutputArray(), qz.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Converts a rotation matrix to a rotation vector or vice versa.
	///
	/// ## Parameters
	/// * src: Input rotation vector (3x1 or 1x3) or rotation matrix (3x3).
	/// * dst: Output rotation matrix (3x3) or rotation vector (3x1 or 1x3), respectively.
	/// * jacobian: Optional output Jacobian matrix, 3x9 or 9x3, which is a matrix of partial
	/// derivatives of the output array components with respect to the input array components.
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctheta%20%5Cleftarrow%20norm%28r%29%20%5C%5C%20r%20%20%5Cleftarrow%20r%2F%20%5Ctheta%20%5C%5C%20R%20%3D%20%20%5Ccos%28%5Ctheta%29%20I%20%2B%20%281%2D%20%5Ccos%7B%5Ctheta%7D%20%29%20r%20r%5ET%20%2B%20%20%5Csin%28%5Ctheta%29%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2Dr%5Fz%20%26%20r%5Fy%5C%5C%20r%5Fz%20%26%200%20%26%20%2Dr%5Fx%5C%5C%20%2Dr%5Fy%20%26%20r%5Fx%20%26%200%20%5Cend%7Bbmatrix%7D%20%5Cend%7Barray%7D)
	///
	/// Inverse transformation can be also done easily, since
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Csin%20%28%20%5Ctheta%20%29%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2Dr%5Fz%20%26%20r%5Fy%5C%5C%20r%5Fz%20%26%200%20%26%20%2Dr%5Fx%5C%5C%20%2Dr%5Fy%20%26%20r%5Fx%20%26%200%20%5Cend%7Bbmatrix%7D%20%3D%20%5Cfrac%7BR%20%2D%20R%5ET%7D%7B2%7D)
	///
	/// A rotation vector is a convenient and most compact representation of a rotation matrix (since any
	/// rotation matrix has just 3 degrees of freedom). The representation is used in the global 3D geometry
	/// optimization procedures like [calibrateCamera], [stereoCalibrate], or [solvePnP] .
	///
	///
	/// Note: More information about the computation of the derivative of a 3D rotation matrix with respect to its exponential coordinate
	/// can be found in:
	///    - A Compact Formula for the Derivative of a 3-D Rotation in Exponential Coordinates, Guillermo Gallego, Anthony J. Yezzi [Gallego2014ACF](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Gallego2014ACF)
	///
	///
	/// Note: Useful information on SE(3) and Lie Groups can be found in:
	///    - A tutorial on SE(3) transformation parameterizations and on-manifold optimization, Jose-Luis Blanco [blanco2010tutorial](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_blanco2010tutorial)
	///    - Lie Groups for 2D and 3D Transformation, Ethan Eade [Eade17](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade17)
	///    - A micro Lie theory for state estimation in robotics, Joan Solà, Jérémie Deray, Dinesh Atchuthan [Sol2018AML](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Sol2018AML)
	///
	/// ## Note
	/// This alternative version of [rodrigues] function uses the following default values for its arguments:
	/// * jacobian: noArray()
	#[inline]
	pub fn rodrigues_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Rodrigues_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Converts a rotation matrix to a rotation vector or vice versa.
	///
	/// ## Parameters
	/// * src: Input rotation vector (3x1 or 1x3) or rotation matrix (3x3).
	/// * dst: Output rotation matrix (3x3) or rotation vector (3x1 or 1x3), respectively.
	/// * jacobian: Optional output Jacobian matrix, 3x9 or 9x3, which is a matrix of partial
	/// derivatives of the output array components with respect to the input array components.
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctheta%20%5Cleftarrow%20norm%28r%29%20%5C%5C%20r%20%20%5Cleftarrow%20r%2F%20%5Ctheta%20%5C%5C%20R%20%3D%20%20%5Ccos%28%5Ctheta%29%20I%20%2B%20%281%2D%20%5Ccos%7B%5Ctheta%7D%20%29%20r%20r%5ET%20%2B%20%20%5Csin%28%5Ctheta%29%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2Dr%5Fz%20%26%20r%5Fy%5C%5C%20r%5Fz%20%26%200%20%26%20%2Dr%5Fx%5C%5C%20%2Dr%5Fy%20%26%20r%5Fx%20%26%200%20%5Cend%7Bbmatrix%7D%20%5Cend%7Barray%7D)
	///
	/// Inverse transformation can be also done easily, since
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Csin%20%28%20%5Ctheta%20%29%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2Dr%5Fz%20%26%20r%5Fy%5C%5C%20r%5Fz%20%26%200%20%26%20%2Dr%5Fx%5C%5C%20%2Dr%5Fy%20%26%20r%5Fx%20%26%200%20%5Cend%7Bbmatrix%7D%20%3D%20%5Cfrac%7BR%20%2D%20R%5ET%7D%7B2%7D)
	///
	/// A rotation vector is a convenient and most compact representation of a rotation matrix (since any
	/// rotation matrix has just 3 degrees of freedom). The representation is used in the global 3D geometry
	/// optimization procedures like [calibrateCamera], [stereoCalibrate], or [solvePnP] .
	///
	///
	/// Note: More information about the computation of the derivative of a 3D rotation matrix with respect to its exponential coordinate
	/// can be found in:
	///    - A Compact Formula for the Derivative of a 3-D Rotation in Exponential Coordinates, Guillermo Gallego, Anthony J. Yezzi [Gallego2014ACF](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Gallego2014ACF)
	///
	///
	/// Note: Useful information on SE(3) and Lie Groups can be found in:
	///    - A tutorial on SE(3) transformation parameterizations and on-manifold optimization, Jose-Luis Blanco [blanco2010tutorial](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_blanco2010tutorial)
	///    - Lie Groups for 2D and 3D Transformation, Ethan Eade [Eade17](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade17)
	///    - A micro Lie theory for state estimation in robotics, Joan Solà, Jérémie Deray, Dinesh Atchuthan [Sol2018AML](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Sol2018AML)
	///
	/// ## C++ default parameters
	/// * jacobian: noArray()
	#[inline]
	pub fn rodrigues(src: &impl ToInputArray, dst: &mut impl ToOutputArray, jacobian: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		output_array_arg!(jacobian);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Rodrigues_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), jacobian.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Approximates a polygonal curve(s) with the specified precision.
	///
	/// T he function cv::approxPolyDP approximates a curve or a p*olygon with another curve/polygon with less
	/// vertices so that the distance between them is less or equal to the specified precision. It uses the
	/// Douglas-Peucker algorithm <https://en.wikipedia.org/wiki/Ramer-Douglas-Peucker_algorithm>
	///
	/// ## Parameters
	/// * curve: Input vector of a 2D point stored in std::vector or Mat
	/// * approxCurve: Result of the approximation. The type should match the type of the input curve.
	/// * epsilon: Parameter specifying the approximation accuracy. This is the maximum distance
	/// between the original curve and its approximation.
	/// * closed: If true, the approximated curve is closed (its first and last vertices are
	/// connected). Otherwise, it is not closed.
	#[inline]
	pub fn approx_poly_dp(curve: &impl ToInputArray, approx_curve: &mut impl ToOutputArray, epsilon: f64, closed: bool) -> Result<()> {
		input_array_arg!(curve);
		output_array_arg!(approx_curve);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_approxPolyDP_const__InputArrayR_const__OutputArrayR_double_bool(curve.as_raw__InputArray(), approx_curve.as_raw__OutputArray(), epsilon, closed, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Approximates a polygon with a convex hull with a specified accuracy and number of sides.
	///
	/// T he cv::approxPolyN function approximates a polygon with *a convex hull
	/// so that the difference between the contour area of the original contour and the new polygon is minimal.
	/// It uses a greedy algorithm for contracting two vertices into one in such a way that the additional area is minimal.
	/// Straight lines formed by each edge of the convex contour are drawn and the areas of the resulting triangles are considered.
	/// Each vertex will lie either on the original contour or outside it.
	///
	/// The algorithm based on the paper [LowIlie2003](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_LowIlie2003) .
	///
	/// ## Parameters
	/// * curve: Input vector of a 2D points stored in std::vector or Mat, points must be float or integer.
	/// * approxCurve: Result of the approximation. The type is vector of a 2D point (Point2f or Point) in std::vector or Mat.
	/// * nsides: The parameter defines the number of sides of the result polygon.
	/// * epsilon_percentage: defines the percentage of the maximum of additional area.
	/// If it equals -1, it is not used. Otherwise algorithm stops if additional area is greater than contourArea(_curve) * percentage.
	/// If additional area exceeds the limit, algorithm returns as many vertices as there were at the moment the limit was exceeded.
	/// * ensure_convex: If it is true, algorithm creates a convex hull of input contour. Otherwise input vector should be convex.
	///
	/// ## Note
	/// This alternative version of [approx_poly_n] function uses the following default values for its arguments:
	/// * epsilon_percentage: -1.0
	/// * ensure_convex: true
	#[inline]
	pub fn approx_poly_n_def(curve: &impl ToInputArray, approx_curve: &mut impl ToOutputArray, nsides: i32) -> Result<()> {
		input_array_arg!(curve);
		output_array_arg!(approx_curve);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_approxPolyN_const__InputArrayR_const__OutputArrayR_int(curve.as_raw__InputArray(), approx_curve.as_raw__OutputArray(), nsides, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Approximates a polygon with a convex hull with a specified accuracy and number of sides.
	///
	/// T he cv::approxPolyN function approximates a polygon with *a convex hull
	/// so that the difference between the contour area of the original contour and the new polygon is minimal.
	/// It uses a greedy algorithm for contracting two vertices into one in such a way that the additional area is minimal.
	/// Straight lines formed by each edge of the convex contour are drawn and the areas of the resulting triangles are considered.
	/// Each vertex will lie either on the original contour or outside it.
	///
	/// The algorithm based on the paper [LowIlie2003](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_LowIlie2003) .
	///
	/// ## Parameters
	/// * curve: Input vector of a 2D points stored in std::vector or Mat, points must be float or integer.
	/// * approxCurve: Result of the approximation. The type is vector of a 2D point (Point2f or Point) in std::vector or Mat.
	/// * nsides: The parameter defines the number of sides of the result polygon.
	/// * epsilon_percentage: defines the percentage of the maximum of additional area.
	/// If it equals -1, it is not used. Otherwise algorithm stops if additional area is greater than contourArea(_curve) * percentage.
	/// If additional area exceeds the limit, algorithm returns as many vertices as there were at the moment the limit was exceeded.
	/// * ensure_convex: If it is true, algorithm creates a convex hull of input contour. Otherwise input vector should be convex.
	///
	/// ## C++ default parameters
	/// * epsilon_percentage: -1.0
	/// * ensure_convex: true
	#[inline]
	pub fn approx_poly_n(curve: &impl ToInputArray, approx_curve: &mut impl ToOutputArray, nsides: i32, epsilon_percentage: f32, ensure_convex: bool) -> Result<()> {
		input_array_arg!(curve);
		output_array_arg!(approx_curve);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_approxPolyN_const__InputArrayR_const__OutputArrayR_int_float_bool(curve.as_raw__InputArray(), approx_curve.as_raw__OutputArray(), nsides, epsilon_percentage, ensure_convex, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates a contour perimeter or a curve length.
	///
	/// The function computes a curve length or a closed contour perimeter.
	///
	/// ## Parameters
	/// * curve: Input vector of 2D points, stored in std::vector or Mat.
	/// * closed: Flag indicating whether the curve is closed or not.
	#[inline]
	pub fn arc_length(curve: &impl ToInputArray, closed: bool) -> Result<f64> {
		input_array_arg!(curve);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_arcLength_const__InputArrayR_bool(curve.as_raw__InputArray(), closed, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates the up-right bounding rectangle of a point set or non-zero pixels of gray-scale image.
	///
	/// The function calculates and returns the minimal up-right bounding rectangle for the specified point set or
	/// non-zero pixels of gray-scale image.
	///
	/// ## Parameters
	/// * array: Input gray-scale image or 2D point set, stored in std::vector or Mat.
	#[inline]
	pub fn bounding_rect(array: &impl ToInputArray) -> Result<core::Rect> {
		input_array_arg!(array);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_boundingRect_const__InputArrayR(array.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the four vertices of a rotated rect. Useful to draw the rotated rectangle.
	///
	/// The function finds the four vertices of a rotated rectangle. The four vertices are returned
	/// in clockwise order starting from the point with greatest ![inline formula](https://latex.codecogs.com/png.latex?y). If two points have the
	/// same ![inline formula](https://latex.codecogs.com/png.latex?y) coordinate the rightmost is the starting point. This function is useful to draw the
	/// rectangle. In C++, instead of using this function, you can directly use RotatedRect::points method. Please
	/// visit the [tutorial_bounding_rotated_ellipses] "tutorial on Creating Bounding rotated boxes and ellipses
	/// for contours" for more information.
	///
	/// ## Parameters
	/// * box: The input rotated rectangle. It may be the output of [minAreaRect].
	/// * points: The output array of four vertices of rectangles.
	#[inline]
	pub fn box_points(box_: core::RotatedRect, points: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_boxPoints_RotatedRect_const__OutputArrayR(&box_, points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes useful camera characteristics from the camera intrinsic matrix.
	///
	/// ## Parameters
	/// * cameraMatrix: Input camera intrinsic matrix that can be estimated by [calibrate_camera] or
	/// [stereo_calibrate] .
	/// * imageSize: Input image size in pixels.
	/// * apertureWidth: Physical width in mm of the sensor.
	/// * apertureHeight: Physical height in mm of the sensor.
	/// * fovx: Output field of view in degrees along the horizontal sensor axis.
	/// * fovy: Output field of view in degrees along the vertical sensor axis.
	/// * focalLength: Focal length of the lens in mm.
	/// * principalPoint: Principal point in mm.
	/// * aspectRatio: ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy%2Ff%5Fx)
	///
	/// The function computes various useful camera characteristics from the previously estimated camera
	/// matrix.
	///
	///
	/// Note:
	///   Do keep in mind that the unity measure 'mm' stands for whatever unit of measure one chooses for
	///    the chessboard pitch (it can thus be any value).
	#[inline]
	pub fn calibration_matrix_values(camera_matrix: &impl ToInputArray, image_size: core::Size, aperture_width: f64, aperture_height: f64, fovx: &mut f64, fovy: &mut f64, focal_length: &mut f64, principal_point: &mut core::Point2d, aspect_ratio: &mut f64) -> Result<()> {
		input_array_arg!(camera_matrix);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrationMatrixValues_const__InputArrayR_Size_double_double_doubleR_doubleR_doubleR_Point2dR_doubleR(camera_matrix.as_raw__InputArray(), &image_size, aperture_width, aperture_height, fovx, fovy, focal_length, principal_point, aspect_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Combines two rotation-and-shift transformations.
	///
	/// ## Parameters
	/// * rvec1: First rotation vector.
	/// * tvec1: First translation vector.
	/// * rvec2: Second rotation vector.
	/// * tvec2: Second translation vector.
	/// * rvec3: Output rotation vector of the superposition.
	/// * tvec3: Output translation vector of the superposition.
	/// * dr3dr1: Optional output derivative of rvec3 with regard to rvec1
	/// * dr3dt1: Optional output derivative of rvec3 with regard to tvec1
	/// * dr3dr2: Optional output derivative of rvec3 with regard to rvec2
	/// * dr3dt2: Optional output derivative of rvec3 with regard to tvec2
	/// * dt3dr1: Optional output derivative of tvec3 with regard to rvec1
	/// * dt3dt1: Optional output derivative of tvec3 with regard to tvec1
	/// * dt3dr2: Optional output derivative of tvec3 with regard to rvec2
	/// * dt3dt2: Optional output derivative of tvec3 with regard to tvec2
	///
	/// The functions compute:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctexttt%7Brvec3%7D%20%3D%20%20%5Cmathrm%7Brodrigues%7D%20%5E%7B%2D1%7D%20%5Cleft%20%28%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec2%7D%20%29%20%20%5Ccdot%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec1%7D%20%29%20%5Cright%20%29%20%20%5C%5C%20%5Ctexttt%7Btvec3%7D%20%3D%20%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec2%7D%20%29%20%20%5Ccdot%20%5Ctexttt%7Btvec1%7D%20%2B%20%20%5Ctexttt%7Btvec2%7D%20%5Cend%7Barray%7D%20%2C)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathrm%7Brodrigues%7D) denotes a rotation vector to a rotation matrix transformation, and
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathrm%7Brodrigues%7D%5E%7B%2D1%7D) denotes the inverse transformation. See [rodrigues] for details.
	///
	/// Also, the functions can compute the derivatives of the output vectors with regards to the input
	/// vectors (see [mat_mul_deriv] ). The functions are used inside [stereo_calibrate] but can also be used in
	/// your own code where Levenberg-Marquardt or another gradient-based solver is used to optimize a
	/// function that contains a matrix multiplication.
	///
	/// ## Note
	/// This alternative version of [compose_rt] function uses the following default values for its arguments:
	/// * dr3dr1: noArray()
	/// * dr3dt1: noArray()
	/// * dr3dr2: noArray()
	/// * dr3dt2: noArray()
	/// * dt3dr1: noArray()
	/// * dt3dt1: noArray()
	/// * dt3dr2: noArray()
	/// * dt3dt2: noArray()
	#[inline]
	pub fn compose_rt_def(rvec1: &impl ToInputArray, tvec1: &impl ToInputArray, rvec2: &impl ToInputArray, tvec2: &impl ToInputArray, rvec3: &mut impl ToOutputArray, tvec3: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(rvec1);
		input_array_arg!(tvec1);
		input_array_arg!(rvec2);
		input_array_arg!(tvec2);
		output_array_arg!(rvec3);
		output_array_arg!(tvec3);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_composeRT_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(rvec1.as_raw__InputArray(), tvec1.as_raw__InputArray(), rvec2.as_raw__InputArray(), tvec2.as_raw__InputArray(), rvec3.as_raw__OutputArray(), tvec3.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Combines two rotation-and-shift transformations.
	///
	/// ## Parameters
	/// * rvec1: First rotation vector.
	/// * tvec1: First translation vector.
	/// * rvec2: Second rotation vector.
	/// * tvec2: Second translation vector.
	/// * rvec3: Output rotation vector of the superposition.
	/// * tvec3: Output translation vector of the superposition.
	/// * dr3dr1: Optional output derivative of rvec3 with regard to rvec1
	/// * dr3dt1: Optional output derivative of rvec3 with regard to tvec1
	/// * dr3dr2: Optional output derivative of rvec3 with regard to rvec2
	/// * dr3dt2: Optional output derivative of rvec3 with regard to tvec2
	/// * dt3dr1: Optional output derivative of tvec3 with regard to rvec1
	/// * dt3dt1: Optional output derivative of tvec3 with regard to tvec1
	/// * dt3dr2: Optional output derivative of tvec3 with regard to rvec2
	/// * dt3dt2: Optional output derivative of tvec3 with regard to tvec2
	///
	/// The functions compute:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctexttt%7Brvec3%7D%20%3D%20%20%5Cmathrm%7Brodrigues%7D%20%5E%7B%2D1%7D%20%5Cleft%20%28%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec2%7D%20%29%20%20%5Ccdot%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec1%7D%20%29%20%5Cright%20%29%20%20%5C%5C%20%5Ctexttt%7Btvec3%7D%20%3D%20%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec2%7D%20%29%20%20%5Ccdot%20%5Ctexttt%7Btvec1%7D%20%2B%20%20%5Ctexttt%7Btvec2%7D%20%5Cend%7Barray%7D%20%2C)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathrm%7Brodrigues%7D) denotes a rotation vector to a rotation matrix transformation, and
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathrm%7Brodrigues%7D%5E%7B%2D1%7D) denotes the inverse transformation. See [rodrigues] for details.
	///
	/// Also, the functions can compute the derivatives of the output vectors with regards to the input
	/// vectors (see [mat_mul_deriv] ). The functions are used inside [stereo_calibrate] but can also be used in
	/// your own code where Levenberg-Marquardt or another gradient-based solver is used to optimize a
	/// function that contains a matrix multiplication.
	///
	/// ## C++ default parameters
	/// * dr3dr1: noArray()
	/// * dr3dt1: noArray()
	/// * dr3dr2: noArray()
	/// * dr3dt2: noArray()
	/// * dt3dr1: noArray()
	/// * dt3dt1: noArray()
	/// * dt3dr2: noArray()
	/// * dt3dt2: noArray()
	#[inline]
	pub fn compose_rt(rvec1: &impl ToInputArray, tvec1: &impl ToInputArray, rvec2: &impl ToInputArray, tvec2: &impl ToInputArray, rvec3: &mut impl ToOutputArray, tvec3: &mut impl ToOutputArray, dr3dr1: &mut impl ToOutputArray, dr3dt1: &mut impl ToOutputArray, dr3dr2: &mut impl ToOutputArray, dr3dt2: &mut impl ToOutputArray, dt3dr1: &mut impl ToOutputArray, dt3dt1: &mut impl ToOutputArray, dt3dr2: &mut impl ToOutputArray, dt3dt2: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(rvec1);
		input_array_arg!(tvec1);
		input_array_arg!(rvec2);
		input_array_arg!(tvec2);
		output_array_arg!(rvec3);
		output_array_arg!(tvec3);
		output_array_arg!(dr3dr1);
		output_array_arg!(dr3dt1);
		output_array_arg!(dr3dr2);
		output_array_arg!(dr3dt2);
		output_array_arg!(dt3dr1);
		output_array_arg!(dt3dt1);
		output_array_arg!(dt3dr2);
		output_array_arg!(dt3dt2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_composeRT_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(rvec1.as_raw__InputArray(), tvec1.as_raw__InputArray(), rvec2.as_raw__InputArray(), tvec2.as_raw__InputArray(), rvec3.as_raw__OutputArray(), tvec3.as_raw__OutputArray(), dr3dr1.as_raw__OutputArray(), dr3dt1.as_raw__OutputArray(), dr3dr2.as_raw__OutputArray(), dr3dt2.as_raw__OutputArray(), dt3dr1.as_raw__OutputArray(), dt3dt1.as_raw__OutputArray(), dt3dr2.as_raw__OutputArray(), dt3dt2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// For points in an image of a stereo pair, computes the corresponding epilines in the other image.
	///
	/// ## Parameters
	/// * points: Input points. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Ctimes%201) or ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Ctimes%20N) matrix of type CV_32FC2 or
	/// vector\<Point2f\> .
	/// * whichImage: Index of the image (1 or 2) that contains the points .
	/// * F: Fundamental matrix that can be estimated using [find_fundamental_mat] or [stereo_rectify] .
	/// * lines: Output vector of the epipolar lines corresponding to the points in the other image.
	/// Each line ![inline formula](https://latex.codecogs.com/png.latex?ax%20%2B%20by%20%2B%20c%3D0) is encoded by 3 numbers ![inline formula](https://latex.codecogs.com/png.latex?%28a%2C%20b%2C%20c%29) .
	///
	/// For every point in one of the two images of a stereo pair, the function finds the equation of the
	/// corresponding epipolar line in the other image.
	///
	/// From the fundamental matrix definition (see [find_fundamental_mat] ), line ![inline formula](https://latex.codecogs.com/png.latex?l%5E%7B%282%29%7D%5Fi) in the second
	/// image for the point ![inline formula](https://latex.codecogs.com/png.latex?p%5E%7B%281%29%7D%5Fi) in the first image (when whichImage=1 ) is computed as:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?l%5E%7B%282%29%7D%5Fi%20%3D%20F%20p%5E%7B%281%29%7D%5Fi)
	///
	/// And vice versa, when whichImage=2, ![inline formula](https://latex.codecogs.com/png.latex?l%5E%7B%281%29%7D%5Fi) is computed from ![inline formula](https://latex.codecogs.com/png.latex?p%5E%7B%282%29%7D%5Fi) as:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?l%5E%7B%281%29%7D%5Fi%20%3D%20F%5ET%20p%5E%7B%282%29%7D%5Fi)
	///
	/// Line coefficients are defined up to a scale. They are normalized so that ![inline formula](https://latex.codecogs.com/png.latex?a%5Fi%5E2%2Bb%5Fi%5E2%3D1) .
	#[inline]
	pub fn compute_correspond_epilines(points: &impl ToInputArray, which_image: i32, f: &impl ToInputArray, lines: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		input_array_arg!(f);
		output_array_arg!(lines);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_computeCorrespondEpilines_const__InputArrayR_int_const__InputArrayR_const__OutputArrayR(points.as_raw__InputArray(), which_image, f.as_raw__InputArray(), lines.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates a contour area.
	///
	/// The function computes a contour area. Similarly to moments , the area is computed using the Green
	/// formula. Thus, the returned area and the number of non-zero pixels, if you draw the contour using
	/// [draw_contours] or [fill_poly] , can be different. Also, the function will most certainly give a wrong
	/// results for contours with self-intersections.
	///
	/// Example:
	/// ```C++
	///    vector<Point> contour;
	///    contour.push_back(Point2f(0, 0));
	///    contour.push_back(Point2f(10, 0));
	///    contour.push_back(Point2f(10, 10));
	///    contour.push_back(Point2f(5, 4));
	///
	///   double area0 = contourArea(contour);
	///    vector<Point> approx;
	///    approxPolyDP(contour, approx, 5, true);
	///    double area1 = contourArea(approx);
	///
	///   cout << "area0 =" << area0 << endl <<
	///            "area1 =" << area1 << endl <<
	///            "approx poly vertices" << approx.size() << endl;
	/// ```
	///
	/// ## Parameters
	/// * contour: Input vector of 2D points (contour vertices), stored in std::vector or Mat.
	/// * oriented: Oriented area flag. If it is true, the function returns a signed area value,
	/// depending on the contour orientation (clockwise or counter-clockwise). Using this feature you can
	/// determine orientation of a contour by taking the sign of an area. By default, the parameter is
	/// false, which means that the absolute value is returned.
	///
	/// ## Note
	/// This alternative version of [contour_area] function uses the following default values for its arguments:
	/// * oriented: false
	#[inline]
	pub fn contour_area_def(contour: &impl ToInputArray) -> Result<f64> {
		input_array_arg!(contour);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_contourArea_const__InputArrayR(contour.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates a contour area.
	///
	/// The function computes a contour area. Similarly to moments , the area is computed using the Green
	/// formula. Thus, the returned area and the number of non-zero pixels, if you draw the contour using
	/// [draw_contours] or [fill_poly] , can be different. Also, the function will most certainly give a wrong
	/// results for contours with self-intersections.
	///
	/// Example:
	/// ```C++
	///    vector<Point> contour;
	///    contour.push_back(Point2f(0, 0));
	///    contour.push_back(Point2f(10, 0));
	///    contour.push_back(Point2f(10, 10));
	///    contour.push_back(Point2f(5, 4));
	///
	///   double area0 = contourArea(contour);
	///    vector<Point> approx;
	///    approxPolyDP(contour, approx, 5, true);
	///    double area1 = contourArea(approx);
	///
	///   cout << "area0 =" << area0 << endl <<
	///            "area1 =" << area1 << endl <<
	///            "approx poly vertices" << approx.size() << endl;
	/// ```
	///
	/// ## Parameters
	/// * contour: Input vector of 2D points (contour vertices), stored in std::vector or Mat.
	/// * oriented: Oriented area flag. If it is true, the function returns a signed area value,
	/// depending on the contour orientation (clockwise or counter-clockwise). Using this feature you can
	/// determine orientation of a contour by taking the sign of an area. By default, the parameter is
	/// false, which means that the absolute value is returned.
	///
	/// ## C++ default parameters
	/// * oriented: false
	#[inline]
	pub fn contour_area(contour: &impl ToInputArray, oriented: bool) -> Result<f64> {
		input_array_arg!(contour);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_contourArea_const__InputArrayR_bool(contour.as_raw__InputArray(), oriented, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Converts points from homogeneous to Euclidean space.
	///
	/// ## Parameters
	/// * src: Input vector of N-dimensional points.
	/// * dst: Output vector of N-1-dimensional points.
	/// * dtype: The desired output array depth (either CV_32F or CV_64F are currently supported).
	///    If it's -1, then it's set automatically to CV_32F or CV_64F, depending on the input depth.
	///
	/// The function converts points homogeneous to Euclidean space using perspective projection. That is,
	/// each point (x1, x2, ... x(n-1), xn) is converted to (x1/xn, x2/xn, ..., x(n-1)/xn). When xn=0, the
	/// output point coordinates will be (0,0,0,...).
	///
	/// ## Note
	/// This alternative version of [convert_points_from_homogeneous] function uses the following default values for its arguments:
	/// * dtype: -1
	#[inline]
	pub fn convert_points_from_homogeneous_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_convertPointsFromHomogeneous_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Converts points from homogeneous to Euclidean space.
	///
	/// ## Parameters
	/// * src: Input vector of N-dimensional points.
	/// * dst: Output vector of N-1-dimensional points.
	/// * dtype: The desired output array depth (either CV_32F or CV_64F are currently supported).
	///    If it's -1, then it's set automatically to CV_32F or CV_64F, depending on the input depth.
	///
	/// The function converts points homogeneous to Euclidean space using perspective projection. That is,
	/// each point (x1, x2, ... x(n-1), xn) is converted to (x1/xn, x2/xn, ..., x(n-1)/xn). When xn=0, the
	/// output point coordinates will be (0,0,0,...).
	///
	/// ## C++ default parameters
	/// * dtype: -1
	#[inline]
	pub fn convert_points_from_homogeneous(src: &impl ToInputArray, dst: &mut impl ToOutputArray, dtype: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_convertPointsFromHomogeneous_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Converts points to/from homogeneous coordinates.
	///
	/// ## Parameters
	/// * src: Input array or vector of 2D, 3D, or 4D points.
	/// * dst: Output vector of 2D, 3D, or 4D points.
	///
	/// The function converts 2D or 3D points from/to homogeneous coordinates by calling either
	/// [convert_points_to_homogeneous] or #convertPointsFromHomogeneous.
	///
	///
	/// Note: The function is obsolete. Use one of the previous two functions instead.
	#[inline]
	pub fn convert_points_homogeneous(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_convertPointsHomogeneous_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Converts points from Euclidean to homogeneous space.
	///
	/// ## Parameters
	/// * src: Input vector of N-dimensional points.
	/// * dst: Output vector of N+1-dimensional points.
	/// * dtype: The desired output array depth (either CV_32F or CV_64F are currently supported).
	///    If it's -1, then it's set automatically to CV_32F or CV_64F, depending on the input depth.
	///
	/// The function converts points from Euclidean to homogeneous space by appending 1's to the tuple of
	/// point coordinates. That is, each point (x1, x2, ..., xn) is converted to (x1, x2, ..., xn, 1).
	///
	/// ## Note
	/// This alternative version of [convert_points_to_homogeneous] function uses the following default values for its arguments:
	/// * dtype: -1
	#[inline]
	pub fn convert_points_to_homogeneous_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_convertPointsToHomogeneous_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Converts points from Euclidean to homogeneous space.
	///
	/// ## Parameters
	/// * src: Input vector of N-dimensional points.
	/// * dst: Output vector of N+1-dimensional points.
	/// * dtype: The desired output array depth (either CV_32F or CV_64F are currently supported).
	///    If it's -1, then it's set automatically to CV_32F or CV_64F, depending on the input depth.
	///
	/// The function converts points from Euclidean to homogeneous space by appending 1's to the tuple of
	/// point coordinates. That is, each point (x1, x2, ..., xn) is converted to (x1, x2, ..., xn, 1).
	///
	/// ## C++ default parameters
	/// * dtype: -1
	#[inline]
	pub fn convert_points_to_homogeneous(src: &impl ToInputArray, dst: &mut impl ToOutputArray, dtype: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_convertPointsToHomogeneous_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the convex hull of a point set.
	///
	/// The function cv::convexHull finds the convex hull of a 2D point set using the Sklansky's algorithm [Sklansky82](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Sklansky82)
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
	/// Check [tutorial_hull] "the corresponding tutorial" for more details.
	///
	/// useful links:
	///
	/// <https://www.learnopencv.com/convex-hull-using-opencv-in-python-and-c/>
	///
	/// ## Note
	/// This alternative version of [convex_hull] function uses the following default values for its arguments:
	/// * clockwise: false
	/// * return_points: true
	#[inline]
	pub fn convex_hull_def(points: &impl ToInputArray, hull: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(hull);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_convexHull_const__InputArrayR_const__OutputArrayR(points.as_raw__InputArray(), hull.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the convex hull of a point set.
	///
	/// The function cv::convexHull finds the convex hull of a 2D point set using the Sklansky's algorithm [Sklansky82](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Sklansky82)
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
	/// Check [tutorial_hull] "the corresponding tutorial" for more details.
	///
	/// useful links:
	///
	/// <https://www.learnopencv.com/convex-hull-using-opencv-in-python-and-c/>
	///
	/// ## C++ default parameters
	/// * clockwise: false
	/// * return_points: true
	#[inline]
	pub fn convex_hull(points: &impl ToInputArray, hull: &mut impl ToOutputArray, clockwise: bool, return_points: bool) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(hull);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_convexHull_const__InputArrayR_const__OutputArrayR_bool_bool(points.as_raw__InputArray(), hull.as_raw__OutputArray(), clockwise, return_points, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the convexity defects of a contour.
	///
	/// The figure below displays convexity defects of a hand contour:
	///
	/// ![image](https://docs.opencv.org/5.0.0/defects.png)
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
	#[inline]
	pub fn convexity_defects(contour: &impl ToInputArray, convexhull: &impl ToInputArray, convexity_defects: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(contour);
		input_array_arg!(convexhull);
		output_array_arg!(convexity_defects);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_convexityDefects_const__InputArrayR_const__InputArrayR_const__OutputArrayR(contour.as_raw__InputArray(), convexhull.as_raw__InputArray(), convexity_defects.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Refines coordinates of corresponding points.
	///
	/// ## Parameters
	/// * F: 3x3 fundamental matrix.
	/// * points1: 1xN array containing the first set of points.
	/// * points2: 1xN array containing the second set of points.
	/// * newPoints1: The optimized points1.
	/// * newPoints2: The optimized points2.
	///
	/// The function implements the Optimal Triangulation Method (see Multiple View Geometry [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) for details).
	/// For each given point correspondence points1[i] \<-\> points2[i], and a fundamental matrix F, it
	/// computes the corrected correspondences newPoints1[i] \<-\> newPoints2[i] that minimize the geometric
	/// error ![inline formula](https://latex.codecogs.com/png.latex?d%28points1%5Bi%5D%2C%20newPoints1%5Bi%5D%29%5E2%20%2B%20d%28points2%5Bi%5D%2CnewPoints2%5Bi%5D%29%5E2) (where ![inline formula](https://latex.codecogs.com/png.latex?d%28a%2Cb%29) is the
	/// geometric distance between points ![inline formula](https://latex.codecogs.com/png.latex?a) and ![inline formula](https://latex.codecogs.com/png.latex?b) ) subject to the epipolar constraint
	/// ![inline formula](https://latex.codecogs.com/png.latex?newPoints2%5ET%20%5Ccdot%20F%20%5Ccdot%20newPoints1%20%3D%200) .
	#[inline]
	pub fn correct_matches(f: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, new_points1: &mut impl ToOutputArray, new_points2: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(f);
		input_array_arg!(points1);
		input_array_arg!(points2);
		output_array_arg!(new_points1);
		output_array_arg!(new_points2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_correctMatches_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(f.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), new_points1.as_raw__OutputArray(), new_points2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Decompose an essential matrix to possible rotations and translation.
	///
	/// ## Parameters
	/// * E: The input essential matrix.
	/// * R1: One possible rotation matrix.
	/// * R2: Another possible rotation matrix.
	/// * t: One possible translation.
	///
	/// This function decomposes the essential matrix E using svd decomposition [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00). In
	/// general, four possible poses exist for the decomposition of E. They are ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20t%5D),
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20%2Dt%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20t%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20%2Dt%5D).
	///
	/// If E gives the epipolar constraint ![inline formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20A%5E%7B%2DT%7D%20E%20A%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200) between the image
	/// points ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) in the first image and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) in second image, then any of the tuples
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20t%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20%2Dt%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20t%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20%2Dt%5D) is a change of basis from the first
	/// camera's coordinate system to the second camera's coordinate system. However, by decomposing E, one
	/// can only get the direction of the translation. For this reason, the translation t is returned with
	/// unit length.
	#[inline]
	pub fn decompose_essential_mat(e: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(e);
		output_array_arg!(r1);
		output_array_arg!(r2);
		output_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_decomposeEssentialMat_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(e.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Decompose a homography matrix to rotation(s), translation(s) and plane normal(s).
	///
	/// ## Parameters
	/// * H: The input homography matrix between two images.
	/// * K: The input camera intrinsic matrix.
	/// * rotations: Array of rotation matrices.
	/// * translations: Array of translation matrices.
	/// * normals: Array of plane normal matrices.
	///
	/// This function extracts relative camera motion between two views of a planar object and returns up to
	/// four mathematical solution tuples of rotation, translation, and plane normal. The decomposition of
	/// the homography matrix H is described in detail in [Malis2007](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Malis2007).
	///
	/// If the homography H, induced by the plane, gives the constraint
	/// ![block formula](https://latex.codecogs.com/png.latex?s%5Fi%20%5Cbegin%7Bbmatrix%7D%20x%27%5Fi%5C%5C%20y%27%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%5Csim%20H%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%5C%5C%20y%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D) on the source image points
	/// ![inline formula](https://latex.codecogs.com/png.latex?p%5Fi) and the destination image points ![inline formula](https://latex.codecogs.com/png.latex?p%27%5Fi), then the tuple of rotations[k] and
	/// translations[k] is a change of basis from the source camera's coordinate system to the destination
	/// camera's coordinate system. However, by decomposing H, one can only get the translation normalized
	/// by the (typically unknown) depth of the scene, i.e. its direction but with normalized length.
	///
	/// If point correspondences are available, at least two solutions may further be invalidated, by
	/// applying positive depth constraint, i.e. all points must be in front of the camera.
	#[inline]
	pub fn decompose_homography_mat(h: &impl ToInputArray, k: &impl ToInputArray, rotations: &mut impl ToOutputArray, translations: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<i32> {
		input_array_arg!(h);
		input_array_arg!(k);
		output_array_arg!(rotations);
		output_array_arg!(translations);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_decomposeHomographyMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(h.as_raw__InputArray(), k.as_raw__InputArray(), rotations.as_raw__OutputArray(), translations.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Decomposes a projection matrix into a rotation matrix and a camera intrinsic matrix.
	///
	/// ## Parameters
	/// * projMatrix: 3x4 input projection matrix P.
	/// * cameraMatrix: Output 3x3 camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D).
	/// * rotMatrix: Output 3x3 external rotation matrix R.
	/// * transVect: Output 4x1 translation vector T.
	/// * rotMatrixX: Optional 3x3 rotation matrix around x-axis.
	/// * rotMatrixY: Optional 3x3 rotation matrix around y-axis.
	/// * rotMatrixZ: Optional 3x3 rotation matrix around z-axis.
	/// * eulerAngles: Optional three-element vector containing three Euler angles of rotation in
	/// degrees.
	///
	/// The function computes a decomposition of a projection matrix into a calibration and a rotation
	/// matrix and the position of a camera.
	///
	/// It optionally returns three rotation matrices, one for each axis, and three Euler angles that could
	/// be used in OpenGL. Note, there is always more than one sequence of rotations about the three
	/// principal axes that results in the same orientation of an object, e.g. see [Slabaugh](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned
	/// three rotation matrices and corresponding three Euler angles are only one of the possible solutions.
	///
	/// The function is based on [rq_decomp3x3] .
	///
	/// ## Note
	/// This alternative version of [decompose_projection_matrix] function uses the following default values for its arguments:
	/// * rot_matrix_x: noArray()
	/// * rot_matrix_y: noArray()
	/// * rot_matrix_z: noArray()
	/// * euler_angles: noArray()
	#[inline]
	pub fn decompose_projection_matrix_def(proj_matrix: &impl ToInputArray, camera_matrix: &mut impl ToOutputArray, rot_matrix: &mut impl ToOutputArray, trans_vect: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(proj_matrix);
		output_array_arg!(camera_matrix);
		output_array_arg!(rot_matrix);
		output_array_arg!(trans_vect);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_decomposeProjectionMatrix_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(proj_matrix.as_raw__InputArray(), camera_matrix.as_raw__OutputArray(), rot_matrix.as_raw__OutputArray(), trans_vect.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Decomposes a projection matrix into a rotation matrix and a camera intrinsic matrix.
	///
	/// ## Parameters
	/// * projMatrix: 3x4 input projection matrix P.
	/// * cameraMatrix: Output 3x3 camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D).
	/// * rotMatrix: Output 3x3 external rotation matrix R.
	/// * transVect: Output 4x1 translation vector T.
	/// * rotMatrixX: Optional 3x3 rotation matrix around x-axis.
	/// * rotMatrixY: Optional 3x3 rotation matrix around y-axis.
	/// * rotMatrixZ: Optional 3x3 rotation matrix around z-axis.
	/// * eulerAngles: Optional three-element vector containing three Euler angles of rotation in
	/// degrees.
	///
	/// The function computes a decomposition of a projection matrix into a calibration and a rotation
	/// matrix and the position of a camera.
	///
	/// It optionally returns three rotation matrices, one for each axis, and three Euler angles that could
	/// be used in OpenGL. Note, there is always more than one sequence of rotations about the three
	/// principal axes that results in the same orientation of an object, e.g. see [Slabaugh](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned
	/// three rotation matrices and corresponding three Euler angles are only one of the possible solutions.
	///
	/// The function is based on [rq_decomp3x3] .
	///
	/// ## C++ default parameters
	/// * rot_matrix_x: noArray()
	/// * rot_matrix_y: noArray()
	/// * rot_matrix_z: noArray()
	/// * euler_angles: noArray()
	#[inline]
	pub fn decompose_projection_matrix(proj_matrix: &impl ToInputArray, camera_matrix: &mut impl ToOutputArray, rot_matrix: &mut impl ToOutputArray, trans_vect: &mut impl ToOutputArray, rot_matrix_x: &mut impl ToOutputArray, rot_matrix_y: &mut impl ToOutputArray, rot_matrix_z: &mut impl ToOutputArray, euler_angles: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(proj_matrix);
		output_array_arg!(camera_matrix);
		output_array_arg!(rot_matrix);
		output_array_arg!(trans_vect);
		output_array_arg!(rot_matrix_x);
		output_array_arg!(rot_matrix_y);
		output_array_arg!(rot_matrix_z);
		output_array_arg!(euler_angles);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_decomposeProjectionMatrix_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(proj_matrix.as_raw__InputArray(), camera_matrix.as_raw__OutputArray(), rot_matrix.as_raw__OutputArray(), trans_vect.as_raw__OutputArray(), rot_matrix_x.as_raw__OutputArray(), rot_matrix_y.as_raw__OutputArray(), rot_matrix_z.as_raw__OutputArray(), euler_angles.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes an optimal affine transformation between two 2D point sets.
	///
	/// It computes
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// ## Parameters
	/// * from: First input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%29).
	/// * to: Second input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29).
	/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
	/// * method: Robust method used to compute transformation. The following methods are possible:
	/// *   [RANSAC] - RANSAC-based robust method
	/// *   [LMEDS] - Least-Median robust method
	/// RANSAC is the default method.
	/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
	/// a point as an inlier. Applies only to RANSAC.
	/// * maxIters: The maximum number of robust method iterations.
	/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
	/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
	/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
	/// * refineIters: Maximum number of iterations of refining algorithm (Levenberg-Marquardt).
	/// Passing 0 will disable refining, so the output matrix will be output of robust method.
	///
	/// ## Returns
	/// Output 2D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?2%20%5Ctimes%203) or empty matrix if transformation
	/// could not be estimated. The returned matrix has the following form:
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20b%5F1%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20b%5F2%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// The function estimates an optimal 2D affine transformation between two 2D point sets using the
	/// selected robust algorithm.
	///
	/// The computed transformation is then refined further (using only inliers) with the
	/// Levenberg-Marquardt method to reduce the re-projection error even more.
	///
	///
	/// Note:
	/// The RANSAC method can handle practically any ratio of outliers but needs a threshold to
	/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
	/// correctly only when there are more than 50% of inliers.
	/// ## See also
	/// estimateAffinePartial2D, getAffineTransform
	///
	/// ## Note
	/// This alternative version of [estimate_affine_2d] function uses the following default values for its arguments:
	/// * inliers: noArray()
	/// * method: RANSAC
	/// * ransac_reproj_threshold: 3
	/// * max_iters: 2000
	/// * confidence: 0.99
	/// * refine_iters: 10
	#[inline]
	pub fn estimate_affine_2d_def(from: &impl ToInputArray, to: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(from);
		input_array_arg!(to);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateAffine2D_const__InputArrayR_const__InputArrayR(from.as_raw__InputArray(), to.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	#[inline]
	pub fn estimate_affine_2d_1(pts1: &impl ToInputArray, pts2: &impl ToInputArray, inliers: &mut impl ToOutputArray, params: crate::geometry::UsacParams) -> Result<core::Mat> {
		input_array_arg!(pts1);
		input_array_arg!(pts2);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateAffine2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(pts1.as_raw__InputArray(), pts2.as_raw__InputArray(), inliers.as_raw__OutputArray(), &params, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Computes an optimal affine transformation between two 2D point sets.
	///
	/// It computes
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// ## Parameters
	/// * from: First input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%29).
	/// * to: Second input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29).
	/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
	/// * method: Robust method used to compute transformation. The following methods are possible:
	/// *   [RANSAC] - RANSAC-based robust method
	/// *   [LMEDS] - Least-Median robust method
	/// RANSAC is the default method.
	/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
	/// a point as an inlier. Applies only to RANSAC.
	/// * maxIters: The maximum number of robust method iterations.
	/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
	/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
	/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
	/// * refineIters: Maximum number of iterations of refining algorithm (Levenberg-Marquardt).
	/// Passing 0 will disable refining, so the output matrix will be output of robust method.
	///
	/// ## Returns
	/// Output 2D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?2%20%5Ctimes%203) or empty matrix if transformation
	/// could not be estimated. The returned matrix has the following form:
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20b%5F1%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20b%5F2%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// The function estimates an optimal 2D affine transformation between two 2D point sets using the
	/// selected robust algorithm.
	///
	/// The computed transformation is then refined further (using only inliers) with the
	/// Levenberg-Marquardt method to reduce the re-projection error even more.
	///
	///
	/// Note:
	/// The RANSAC method can handle practically any ratio of outliers but needs a threshold to
	/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
	/// correctly only when there are more than 50% of inliers.
	/// ## See also
	/// estimateAffinePartial2D, getAffineTransform
	///
	/// ## C++ default parameters
	/// * inliers: noArray()
	/// * method: RANSAC
	/// * ransac_reproj_threshold: 3
	/// * max_iters: 2000
	/// * confidence: 0.99
	/// * refine_iters: 10
	#[inline]
	pub fn estimate_affine_2d(from: &impl ToInputArray, to: &impl ToInputArray, inliers: &mut impl ToOutputArray, method: i32, ransac_reproj_threshold: f64, max_iters: size_t, confidence: f64, refine_iters: size_t) -> Result<core::Mat> {
		input_array_arg!(from);
		input_array_arg!(to);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateAffine2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(from.as_raw__InputArray(), to.as_raw__InputArray(), inliers.as_raw__OutputArray(), method, ransac_reproj_threshold, max_iters, confidence, refine_iters, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Computes an optimal affine transformation between two 3D point sets.
	///
	/// It computes ![inline formula](https://latex.codecogs.com/png.latex?R%2Cs%2Ct) minimizing ![inline formula](https://latex.codecogs.com/png.latex?%5Csum%7Bi%7D%20dst%5Fi%20%2D%20c%20%5Ccdot%20R%20%5Ccdot%20src%5Fi%20)
	/// where ![inline formula](https://latex.codecogs.com/png.latex?R) is a 3x3 rotation matrix, ![inline formula](https://latex.codecogs.com/png.latex?t) is a 3x1 translation vector and ![inline formula](https://latex.codecogs.com/png.latex?s) is a
	/// scalar size value. This is an implementation of the algorithm by Umeyama \cite umeyama1991least .
	/// The estimated affine transform has a homogeneous scale which is a subclass of affine
	/// transformations with 7 degrees of freedom. The paired point sets need to comprise at least 3
	/// points each.
	///
	/// ## Parameters
	/// * src: First input 3D point set.
	/// * dst: Second input 3D point set.
	/// * scale: If null is passed, the scale parameter c will be assumed to be 1.0.
	/// Else the pointed-to variable will be set to the optimal scale.
	/// * force_rotation: If true, the returned rotation will never be a reflection.
	/// This might be unwanted, e.g. when optimizing a transform between a right- and a
	/// left-handed coordinate system.
	/// ## Returns
	/// 3D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%204) of the form
	/// ![block formula](https://latex.codecogs.com/png.latex?T%20%3D%0A%5Cbegin%7Bbmatrix%7D%0AR%20%26%20t%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// ## Note
	/// This alternative version of [estimate_affine_3d_1] function uses the following default values for its arguments:
	/// * scale: nullptr
	/// * force_rotation: true
	#[inline]
	pub fn estimate_affine_3d_1_def(src: &impl ToInputArray, dst: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(src);
		input_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateAffine3D_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Computes an optimal affine transformation between two 3D point sets.
	///
	/// It computes
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0Az%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20a%5F%7B13%7D%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20a%5F%7B23%7D%5C%5C%0Aa%5F%7B31%7D%20%26%20a%5F%7B32%7D%20%26%20a%5F%7B33%7D%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0AZ%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0Ab%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// ## Parameters
	/// * src: First input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%2CZ%29).
	/// * dst: Second input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%2Cz%29).
	/// * out: Output 3D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%204) of the form
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20a%5F%7B13%7D%20%26%20b%5F1%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20a%5F%7B23%7D%20%26%20b%5F2%5C%5C%0Aa%5F%7B31%7D%20%26%20a%5F%7B32%7D%20%26%20a%5F%7B33%7D%20%26%20b%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
	/// * ransacThreshold: Maximum reprojection error in the RANSAC algorithm to consider a point as
	/// an inlier.
	/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
	/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
	/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
	///
	/// ## Returns
	/// Whether a solution was found.
	///
	/// The function estimates an optimal 3D affine transformation between two 3D point sets using the
	/// RANSAC algorithm.
	///
	/// ## Note
	/// This alternative version of [estimate_affine_3d] function uses the following default values for its arguments:
	/// * ransac_threshold: 3
	/// * confidence: 0.99
	#[inline]
	pub fn estimate_affine_3d_def(src: &impl ToInputArray, dst: &impl ToInputArray, out: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(src);
		input_array_arg!(dst);
		output_array_arg!(out);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__InputArray(), out.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes an optimal affine transformation between two 3D point sets.
	///
	/// It computes
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0Az%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20a%5F%7B13%7D%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20a%5F%7B23%7D%5C%5C%0Aa%5F%7B31%7D%20%26%20a%5F%7B32%7D%20%26%20a%5F%7B33%7D%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0AZ%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0Ab%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// ## Parameters
	/// * src: First input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%2CZ%29).
	/// * dst: Second input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%2Cz%29).
	/// * out: Output 3D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%204) of the form
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20a%5F%7B13%7D%20%26%20b%5F1%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20a%5F%7B23%7D%20%26%20b%5F2%5C%5C%0Aa%5F%7B31%7D%20%26%20a%5F%7B32%7D%20%26%20a%5F%7B33%7D%20%26%20b%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
	/// * ransacThreshold: Maximum reprojection error in the RANSAC algorithm to consider a point as
	/// an inlier.
	/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
	/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
	/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
	///
	/// ## Returns
	/// Whether a solution was found.
	///
	/// The function estimates an optimal 3D affine transformation between two 3D point sets using the
	/// RANSAC algorithm.
	///
	/// ## C++ default parameters
	/// * ransac_threshold: 3
	/// * confidence: 0.99
	#[inline]
	pub fn estimate_affine_3d(src: &impl ToInputArray, dst: &impl ToInputArray, out: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray, ransac_threshold: f64, confidence: f64) -> Result<bool> {
		input_array_arg!(src);
		input_array_arg!(dst);
		output_array_arg!(out);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(src.as_raw__InputArray(), dst.as_raw__InputArray(), out.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ransac_threshold, confidence, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes an optimal affine transformation between two 3D point sets.
	///
	/// It computes ![inline formula](https://latex.codecogs.com/png.latex?R%2Cs%2Ct) minimizing ![inline formula](https://latex.codecogs.com/png.latex?%5Csum%7Bi%7D%20dst%5Fi%20%2D%20c%20%5Ccdot%20R%20%5Ccdot%20src%5Fi%20)
	/// where ![inline formula](https://latex.codecogs.com/png.latex?R) is a 3x3 rotation matrix, ![inline formula](https://latex.codecogs.com/png.latex?t) is a 3x1 translation vector and ![inline formula](https://latex.codecogs.com/png.latex?s) is a
	/// scalar size value. This is an implementation of the algorithm by Umeyama \cite umeyama1991least .
	/// The estimated affine transform has a homogeneous scale which is a subclass of affine
	/// transformations with 7 degrees of freedom. The paired point sets need to comprise at least 3
	/// points each.
	///
	/// ## Parameters
	/// * src: First input 3D point set.
	/// * dst: Second input 3D point set.
	/// * scale: If null is passed, the scale parameter c will be assumed to be 1.0.
	/// Else the pointed-to variable will be set to the optimal scale.
	/// * force_rotation: If true, the returned rotation will never be a reflection.
	/// This might be unwanted, e.g. when optimizing a transform between a right- and a
	/// left-handed coordinate system.
	/// ## Returns
	/// 3D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%204) of the form
	/// ![block formula](https://latex.codecogs.com/png.latex?T%20%3D%0A%5Cbegin%7Bbmatrix%7D%0AR%20%26%20t%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// ## C++ default parameters
	/// * scale: nullptr
	/// * force_rotation: true
	#[inline]
	pub fn estimate_affine_3d_1(src: &impl ToInputArray, dst: &impl ToInputArray, scale: &mut f64, force_rotation: bool) -> Result<core::Mat> {
		input_array_arg!(src);
		input_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_doubleX_bool(src.as_raw__InputArray(), dst.as_raw__InputArray(), scale, force_rotation, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Computes an optimal limited affine transformation with 4 degrees of freedom between
	/// two 2D point sets.
	///
	/// ## Parameters
	/// * from: First input 2D point set.
	/// * to: Second input 2D point set.
	/// * inliers: Output vector indicating which points are inliers.
	/// * method: Robust method used to compute transformation. The following methods are possible:
	/// *   [RANSAC] - RANSAC-based robust method
	/// *   [LMEDS] - Least-Median robust method
	/// RANSAC is the default method.
	/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
	/// a point as an inlier. Applies only to RANSAC.
	/// * maxIters: The maximum number of robust method iterations.
	/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
	/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
	/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
	/// * refineIters: Maximum number of iterations of refining algorithm (Levenberg-Marquardt).
	/// Passing 0 will disable refining, so the output matrix will be output of robust method.
	///
	/// ## Returns
	/// Output 2D affine transformation (4 degrees of freedom) matrix ![inline formula](https://latex.codecogs.com/png.latex?2%20%5Ctimes%203) or
	/// empty matrix if transformation could not be estimated.
	///
	/// The function estimates an optimal 2D affine transformation with 4 degrees of freedom limited to
	/// combinations of translation, rotation, and uniform scaling. Uses the selected algorithm for robust
	/// estimation.
	///
	/// The computed transformation is then refined further (using only inliers) with the
	/// Levenberg-Marquardt method to reduce the re-projection error even more.
	///
	/// Estimated transformation matrix is:
	/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Cbegin%7Bbmatrix%7D%20%5Ccos%28%5Ctheta%29%20%5Ccdot%20s%20%26%20%2D%5Csin%28%5Ctheta%29%20%5Ccdot%20s%20%26%20t%5Fx%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Csin%28%5Ctheta%29%20%5Ccdot%20s%20%26%20%5Ccos%28%5Ctheta%29%20%5Ccdot%20s%20%26%20t%5Fy%0A%5Cend%7Bbmatrix%7D%20)
	/// Where ![inline formula](https://latex.codecogs.com/png.latex?%20%5Ctheta%20) is the rotation angle, ![inline formula](https://latex.codecogs.com/png.latex?%20s%20) the scaling factor and ![inline formula](https://latex.codecogs.com/png.latex?%20t%5Fx%2C%20t%5Fy%20) are
	/// translations in ![inline formula](https://latex.codecogs.com/png.latex?%20x%2C%20y%20) axes respectively.
	///
	///
	/// Note:
	/// The RANSAC method can handle practically any ratio of outliers but need a threshold to
	/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
	/// correctly only when there are more than 50% of inliers.
	/// ## See also
	/// estimateAffine2D, getAffineTransform
	///
	/// ## Note
	/// This alternative version of [estimate_affine_partial_2d] function uses the following default values for its arguments:
	/// * inliers: noArray()
	/// * method: RANSAC
	/// * ransac_reproj_threshold: 3
	/// * max_iters: 2000
	/// * confidence: 0.99
	/// * refine_iters: 10
	#[inline]
	pub fn estimate_affine_partial_2d_def(from: &impl ToInputArray, to: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(from);
		input_array_arg!(to);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateAffinePartial2D_const__InputArrayR_const__InputArrayR(from.as_raw__InputArray(), to.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Computes an optimal limited affine transformation with 4 degrees of freedom between
	/// two 2D point sets.
	///
	/// ## Parameters
	/// * from: First input 2D point set.
	/// * to: Second input 2D point set.
	/// * inliers: Output vector indicating which points are inliers.
	/// * method: Robust method used to compute transformation. The following methods are possible:
	/// *   [RANSAC] - RANSAC-based robust method
	/// *   [LMEDS] - Least-Median robust method
	/// RANSAC is the default method.
	/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
	/// a point as an inlier. Applies only to RANSAC.
	/// * maxIters: The maximum number of robust method iterations.
	/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
	/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
	/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
	/// * refineIters: Maximum number of iterations of refining algorithm (Levenberg-Marquardt).
	/// Passing 0 will disable refining, so the output matrix will be output of robust method.
	///
	/// ## Returns
	/// Output 2D affine transformation (4 degrees of freedom) matrix ![inline formula](https://latex.codecogs.com/png.latex?2%20%5Ctimes%203) or
	/// empty matrix if transformation could not be estimated.
	///
	/// The function estimates an optimal 2D affine transformation with 4 degrees of freedom limited to
	/// combinations of translation, rotation, and uniform scaling. Uses the selected algorithm for robust
	/// estimation.
	///
	/// The computed transformation is then refined further (using only inliers) with the
	/// Levenberg-Marquardt method to reduce the re-projection error even more.
	///
	/// Estimated transformation matrix is:
	/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Cbegin%7Bbmatrix%7D%20%5Ccos%28%5Ctheta%29%20%5Ccdot%20s%20%26%20%2D%5Csin%28%5Ctheta%29%20%5Ccdot%20s%20%26%20t%5Fx%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Csin%28%5Ctheta%29%20%5Ccdot%20s%20%26%20%5Ccos%28%5Ctheta%29%20%5Ccdot%20s%20%26%20t%5Fy%0A%5Cend%7Bbmatrix%7D%20)
	/// Where ![inline formula](https://latex.codecogs.com/png.latex?%20%5Ctheta%20) is the rotation angle, ![inline formula](https://latex.codecogs.com/png.latex?%20s%20) the scaling factor and ![inline formula](https://latex.codecogs.com/png.latex?%20t%5Fx%2C%20t%5Fy%20) are
	/// translations in ![inline formula](https://latex.codecogs.com/png.latex?%20x%2C%20y%20) axes respectively.
	///
	///
	/// Note:
	/// The RANSAC method can handle practically any ratio of outliers but need a threshold to
	/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
	/// correctly only when there are more than 50% of inliers.
	/// ## See also
	/// estimateAffine2D, getAffineTransform
	///
	/// ## C++ default parameters
	/// * inliers: noArray()
	/// * method: RANSAC
	/// * ransac_reproj_threshold: 3
	/// * max_iters: 2000
	/// * confidence: 0.99
	/// * refine_iters: 10
	#[inline]
	pub fn estimate_affine_partial_2d(from: &impl ToInputArray, to: &impl ToInputArray, inliers: &mut impl ToOutputArray, method: i32, ransac_reproj_threshold: f64, max_iters: size_t, confidence: f64, refine_iters: size_t) -> Result<core::Mat> {
		input_array_arg!(from);
		input_array_arg!(to);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateAffinePartial2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(from.as_raw__InputArray(), to.as_raw__InputArray(), inliers.as_raw__OutputArray(), method, ransac_reproj_threshold, max_iters, confidence, refine_iters, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Computes a pure 2D translation between two 2D point sets.
	///
	/// It computes
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0A1%20%26%200%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0At%5Fx%5C%5C%0At%5Fy%0A%5Cend%7Bbmatrix%7D%2E%0A)
	///
	/// ## Parameters
	/// * from: First input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%29).
	/// * to: Second input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29).
	/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
	/// * method: Robust method used to compute the transformation. The following methods are possible:
	/// *   [RANSAC] - RANSAC-based robust method
	/// *   [LMEDS] - Least-Median robust method
	/// RANSAC is the default method.
	/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
	/// a point as an inlier. Applies only to RANSAC.
	/// * maxIters: The maximum number of robust method iterations.
	/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
	/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
	/// significantly. Values lower than 0.8–0.9 can result in an incorrectly estimated transformation.
	/// * refineIters: Maximum number of iterations of the refining algorithm. For pure translation
	/// the least-squares solution on inliers is closed-form, so passing 0 is recommended (no additional refine).
	///
	/// ## Returns
	/// A 2D translation vector ![inline formula](https://latex.codecogs.com/png.latex?%5Bt%5Fx%2C%20t%5Fy%5D%5ET) as `cv::Vec2d`. If the translation could not be
	/// estimated, both components are set to NaN and, if @p inliers is provided, the mask is filled with zeros.
	///
	/// \par Converting to a 2x3 transformation matrix:
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0A1%20%26%200%20%26%20t%5Fx%5C%5C%0A0%20%26%201%20%26%20t%5Fy%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// ```C++
	/// cv::Vec2d t = cv::estimateTranslation2D(from, to, inliers);
	/// cv::Mat T = (cv::Mat_<double>(2,3) << 1,0,t[0], 0,1,t[1]);
	/// ```
	///
	///
	/// The function estimates a pure 2D translation between two 2D point sets using the selected robust
	/// algorithm. Inliers are determined by the reprojection error threshold.
	///
	///
	/// Note:
	/// The RANSAC method can handle practically any ratio of outliers but needs a threshold to
	/// distinguish inliers from outliers. The method LMeDS does not need any threshold but works
	/// correctly only when there are more than 50% inliers.
	/// ## See also
	/// estimateAffine2D, estimateAffinePartial2D, getAffineTransform
	///
	/// ## Note
	/// This alternative version of [estimate_translation_2d] function uses the following default values for its arguments:
	/// * inliers: noArray()
	/// * method: RANSAC
	/// * ransac_reproj_threshold: 3
	/// * max_iters: 2000
	/// * confidence: 0.99
	/// * refine_iters: 0
	#[inline]
	pub fn estimate_translation_2d_def(from: &impl ToInputArray, to: &impl ToInputArray) -> Result<core::Vec2d> {
		input_array_arg!(from);
		input_array_arg!(to);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateTranslation2D_const__InputArrayR_const__InputArrayR(from.as_raw__InputArray(), to.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes a pure 2D translation between two 2D point sets.
	///
	/// It computes
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0A1%20%26%200%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0At%5Fx%5C%5C%0At%5Fy%0A%5Cend%7Bbmatrix%7D%2E%0A)
	///
	/// ## Parameters
	/// * from: First input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%29).
	/// * to: Second input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29).
	/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
	/// * method: Robust method used to compute the transformation. The following methods are possible:
	/// *   [RANSAC] - RANSAC-based robust method
	/// *   [LMEDS] - Least-Median robust method
	/// RANSAC is the default method.
	/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
	/// a point as an inlier. Applies only to RANSAC.
	/// * maxIters: The maximum number of robust method iterations.
	/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
	/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
	/// significantly. Values lower than 0.8–0.9 can result in an incorrectly estimated transformation.
	/// * refineIters: Maximum number of iterations of the refining algorithm. For pure translation
	/// the least-squares solution on inliers is closed-form, so passing 0 is recommended (no additional refine).
	///
	/// ## Returns
	/// A 2D translation vector ![inline formula](https://latex.codecogs.com/png.latex?%5Bt%5Fx%2C%20t%5Fy%5D%5ET) as `cv::Vec2d`. If the translation could not be
	/// estimated, both components are set to NaN and, if @p inliers is provided, the mask is filled with zeros.
	///
	/// \par Converting to a 2x3 transformation matrix:
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0A1%20%26%200%20%26%20t%5Fx%5C%5C%0A0%20%26%201%20%26%20t%5Fy%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// ```C++
	/// cv::Vec2d t = cv::estimateTranslation2D(from, to, inliers);
	/// cv::Mat T = (cv::Mat_<double>(2,3) << 1,0,t[0], 0,1,t[1]);
	/// ```
	///
	///
	/// The function estimates a pure 2D translation between two 2D point sets using the selected robust
	/// algorithm. Inliers are determined by the reprojection error threshold.
	///
	///
	/// Note:
	/// The RANSAC method can handle practically any ratio of outliers but needs a threshold to
	/// distinguish inliers from outliers. The method LMeDS does not need any threshold but works
	/// correctly only when there are more than 50% inliers.
	/// ## See also
	/// estimateAffine2D, estimateAffinePartial2D, getAffineTransform
	///
	/// ## C++ default parameters
	/// * inliers: noArray()
	/// * method: RANSAC
	/// * ransac_reproj_threshold: 3
	/// * max_iters: 2000
	/// * confidence: 0.99
	/// * refine_iters: 0
	#[inline]
	pub fn estimate_translation_2d(from: &impl ToInputArray, to: &impl ToInputArray, inliers: &mut impl ToOutputArray, method: i32, ransac_reproj_threshold: f64, max_iters: size_t, confidence: f64, refine_iters: size_t) -> Result<core::Vec2d> {
		input_array_arg!(from);
		input_array_arg!(to);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateTranslation2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(from.as_raw__InputArray(), to.as_raw__InputArray(), inliers.as_raw__OutputArray(), method, ransac_reproj_threshold, max_iters, confidence, refine_iters, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes an optimal translation between two 3D point sets.
	///
	/// It computes
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0Az%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0AZ%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0Ab%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// ## Parameters
	/// * src: First input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%2CZ%29).
	/// * dst: Second input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%2Cz%29).
	/// * out: Output 3D translation vector ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%201) of the form
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%20%5C%5C%0Ab%5F2%20%5C%5C%0Ab%5F3%20%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
	/// * ransacThreshold: Maximum reprojection error in the RANSAC algorithm to consider a point as
	/// an inlier.
	/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
	/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
	/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
	///
	/// ## Returns
	/// Whether a translation was found.
	///
	/// The function estimates an optimal 3D translation between two 3D point sets using the
	/// RANSAC algorithm.
	///
	/// ## Note
	/// This alternative version of [estimate_translation_3d] function uses the following default values for its arguments:
	/// * ransac_threshold: 3
	/// * confidence: 0.99
	#[inline]
	pub fn estimate_translation_3d_def(src: &impl ToInputArray, dst: &impl ToInputArray, out: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(src);
		input_array_arg!(dst);
		output_array_arg!(out);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateTranslation3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__InputArray(), out.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes an optimal translation between two 3D point sets.
	///
	/// It computes
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0Az%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0AZ%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0Ab%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	///
	/// ## Parameters
	/// * src: First input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%2CZ%29).
	/// * dst: Second input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%2Cz%29).
	/// * out: Output 3D translation vector ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%201) of the form
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%20%5C%5C%0Ab%5F2%20%5C%5C%0Ab%5F3%20%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
	/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
	/// * ransacThreshold: Maximum reprojection error in the RANSAC algorithm to consider a point as
	/// an inlier.
	/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
	/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
	/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
	///
	/// ## Returns
	/// Whether a translation was found.
	///
	/// The function estimates an optimal 3D translation between two 3D point sets using the
	/// RANSAC algorithm.
	///
	/// ## C++ default parameters
	/// * ransac_threshold: 3
	/// * confidence: 0.99
	#[inline]
	pub fn estimate_translation_3d(src: &impl ToInputArray, dst: &impl ToInputArray, out: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray, ransac_threshold: f64, confidence: f64) -> Result<bool> {
		input_array_arg!(src);
		input_array_arg!(dst);
		output_array_arg!(out);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateTranslation3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(src.as_raw__InputArray(), dst.as_raw__InputArray(), out.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ransac_threshold, confidence, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Point cloud sampling by Farthest Point Sampling(FPS).
	///
	/// FPS Algorithm:
	/// + Input: Point cloud *C*, *sampled_pts_size*, *dist_lower_limit*
	/// + Initialize: Set sampled point cloud S to the empty set
	/// + Step:
	///    1. Randomly take a seed point from C and take it from C to S;
	///    2. Find a point in C that is the farthest away from S and take it from C to S;
	///       (The distance from point to set S is the smallest distance from point to all points in S)
	///    3. Repeat *step 2* until the farthest distance of the point in C from S
	///       is less than *dist_lower_limit*, or the size of S is equal to *sampled_pts_size*.
	/// + Output: Sampled point cloud S
	///
	/// ## Parameters
	/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
	///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_pts_size: The desired point cloud size after sampling.
	/// * dist_lower_limit: Sampling is terminated early if the distance from
	///                  the farthest point to S is less than dist_lower_limit, default 0.
	/// * rng: Optional random number generator used for selecting seed point for FPS;
	///                  if it is nullptr, theRNG () is used instead.
	/// ## Returns
	/// The number of points actually sampled.
	///
	/// ## Overloaded parameters
	///
	///
	/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
	///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_scale: Range (0, 1), the percentage of the sampled point cloud to the original size,
	///                      that is, sampled size = original size * sampled_scale.
	/// * dist_lower_limit: Sampling is terminated early if the distance from
	///                  the farthest point to S is less than dist_lower_limit, default 0.
	/// * rng: Optional random number generator used for selecting seed point for FPS;
	///                  if it is nullptr, theRNG () is used instead.
	/// ## Returns
	/// The number of points actually sampled.
	///
	/// ## Note
	/// This alternative version of [farthest_point_sampling_1] function uses the following default values for its arguments:
	/// * dist_lower_limit: 0
	/// * rng: nullptr
	#[inline]
	pub fn farthest_point_sampling_1_def(sampled_point_flags: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_scale: f32) -> Result<i32> {
		output_array_arg!(sampled_point_flags);
		input_array_arg!(input_pts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_float(sampled_point_flags.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Point cloud sampling by Farthest Point Sampling(FPS).
	///
	/// FPS Algorithm:
	/// + Input: Point cloud *C*, *sampled_pts_size*, *dist_lower_limit*
	/// + Initialize: Set sampled point cloud S to the empty set
	/// + Step:
	///    1. Randomly take a seed point from C and take it from C to S;
	///    2. Find a point in C that is the farthest away from S and take it from C to S;
	///       (The distance from point to set S is the smallest distance from point to all points in S)
	///    3. Repeat *step 2* until the farthest distance of the point in C from S
	///       is less than *dist_lower_limit*, or the size of S is equal to *sampled_pts_size*.
	/// + Output: Sampled point cloud S
	///
	/// ## Parameters
	/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
	///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_pts_size: The desired point cloud size after sampling.
	/// * dist_lower_limit: Sampling is terminated early if the distance from
	///                  the farthest point to S is less than dist_lower_limit, default 0.
	/// * rng: Optional random number generator used for selecting seed point for FPS;
	///                  if it is nullptr, theRNG () is used instead.
	/// ## Returns
	/// The number of points actually sampled.
	///
	/// ## Overloaded parameters
	///
	///
	/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
	///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_scale: Range (0, 1), the percentage of the sampled point cloud to the original size,
	///                      that is, sampled size = original size * sampled_scale.
	/// * dist_lower_limit: Sampling is terminated early if the distance from
	///                  the farthest point to S is less than dist_lower_limit, default 0.
	/// * rng: Optional random number generator used for selecting seed point for FPS;
	///                  if it is nullptr, theRNG () is used instead.
	/// ## Returns
	/// The number of points actually sampled.
	///
	/// ## C++ default parameters
	/// * dist_lower_limit: 0
	/// * rng: nullptr
	#[inline]
	pub fn farthest_point_sampling_1(sampled_point_flags: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_scale: f32, dist_lower_limit: f32, rng: &mut impl core::RNGTrait) -> Result<i32> {
		output_array_arg!(sampled_point_flags);
		input_array_arg!(input_pts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_float_float_RNGX(sampled_point_flags.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_scale, dist_lower_limit, rng.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Point cloud sampling by Farthest Point Sampling(FPS).
	///
	/// FPS Algorithm:
	/// + Input: Point cloud *C*, *sampled_pts_size*, *dist_lower_limit*
	/// + Initialize: Set sampled point cloud S to the empty set
	/// + Step:
	///    1. Randomly take a seed point from C and take it from C to S;
	///    2. Find a point in C that is the farthest away from S and take it from C to S;
	///       (The distance from point to set S is the smallest distance from point to all points in S)
	///    3. Repeat *step 2* until the farthest distance of the point in C from S
	///       is less than *dist_lower_limit*, or the size of S is equal to *sampled_pts_size*.
	/// + Output: Sampled point cloud S
	///
	/// ## Parameters
	/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
	///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_pts_size: The desired point cloud size after sampling.
	/// * dist_lower_limit: Sampling is terminated early if the distance from
	///                  the farthest point to S is less than dist_lower_limit, default 0.
	/// * rng: Optional random number generator used for selecting seed point for FPS;
	///                  if it is nullptr, theRNG () is used instead.
	/// ## Returns
	/// The number of points actually sampled.
	///
	/// ## Note
	/// This alternative version of [farthest_point_sampling] function uses the following default values for its arguments:
	/// * dist_lower_limit: 0
	/// * rng: nullptr
	#[inline]
	pub fn farthest_point_sampling_def(sampled_point_flags: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_pts_size: i32) -> Result<i32> {
		output_array_arg!(sampled_point_flags);
		input_array_arg!(input_pts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_int(sampled_point_flags.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_pts_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Point cloud sampling by Farthest Point Sampling(FPS).
	///
	/// FPS Algorithm:
	/// + Input: Point cloud *C*, *sampled_pts_size*, *dist_lower_limit*
	/// + Initialize: Set sampled point cloud S to the empty set
	/// + Step:
	///    1. Randomly take a seed point from C and take it from C to S;
	///    2. Find a point in C that is the farthest away from S and take it from C to S;
	///       (The distance from point to set S is the smallest distance from point to all points in S)
	///    3. Repeat *step 2* until the farthest distance of the point in C from S
	///       is less than *dist_lower_limit*, or the size of S is equal to *sampled_pts_size*.
	/// + Output: Sampled point cloud S
	///
	/// ## Parameters
	/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
	///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_pts_size: The desired point cloud size after sampling.
	/// * dist_lower_limit: Sampling is terminated early if the distance from
	///                  the farthest point to S is less than dist_lower_limit, default 0.
	/// * rng: Optional random number generator used for selecting seed point for FPS;
	///                  if it is nullptr, theRNG () is used instead.
	/// ## Returns
	/// The number of points actually sampled.
	///
	/// ## C++ default parameters
	/// * dist_lower_limit: 0
	/// * rng: nullptr
	#[inline]
	pub fn farthest_point_sampling(sampled_point_flags: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_pts_size: i32, dist_lower_limit: f32, rng: &mut impl core::RNGTrait) -> Result<i32> {
		output_array_arg!(sampled_point_flags);
		input_array_arg!(input_pts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_int_float_RNGX(sampled_point_flags.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_pts_size, dist_lower_limit, rng.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Filters homography decompositions based on additional information.
	///
	/// ## Parameters
	/// * rotations: Vector of rotation matrices.
	/// * normals: Vector of plane normal matrices.
	/// * beforePoints: Vector of (rectified) visible reference points before the homography is applied
	/// * afterPoints: Vector of (rectified) visible reference points after the homography is applied
	/// * possibleSolutions: Vector of int indices representing the viable solution set after filtering
	/// * pointsMask: optional Mat/Vector of CV_8U, CV_8S or CV_Bool type representing the mask for the inliers
	/// as given by the [find_homography] function
	///
	/// This function is intended to filter the output of the [decompose_homography_mat] based on additional
	/// information as described in [Malis2007](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Malis2007) . The summary of the method: the [decompose_homography_mat] function
	/// returns 2 unique solutions and their "opposites" for a total of 4 solutions. If we have access to the
	/// sets of points visible in the camera frame before and after the homography transformation is applied,
	/// we can determine which are the true potential solutions and which are the opposites by verifying which
	/// homographies are consistent with all visible reference points being in front of the camera. The inputs
	/// are left unchanged; the filtered solution set is returned as indices into the existing one.
	///
	/// ## Note
	/// This alternative version of [filter_homography_decomp_by_visible_refpoints] function uses the following default values for its arguments:
	/// * points_mask: noArray()
	#[inline]
	pub fn filter_homography_decomp_by_visible_refpoints_def(rotations: &impl ToInputArray, normals: &impl ToInputArray, before_points: &impl ToInputArray, after_points: &impl ToInputArray, possible_solutions: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(rotations);
		input_array_arg!(normals);
		input_array_arg!(before_points);
		input_array_arg!(after_points);
		output_array_arg!(possible_solutions);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(rotations.as_raw__InputArray(), normals.as_raw__InputArray(), before_points.as_raw__InputArray(), after_points.as_raw__InputArray(), possible_solutions.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Filters homography decompositions based on additional information.
	///
	/// ## Parameters
	/// * rotations: Vector of rotation matrices.
	/// * normals: Vector of plane normal matrices.
	/// * beforePoints: Vector of (rectified) visible reference points before the homography is applied
	/// * afterPoints: Vector of (rectified) visible reference points after the homography is applied
	/// * possibleSolutions: Vector of int indices representing the viable solution set after filtering
	/// * pointsMask: optional Mat/Vector of CV_8U, CV_8S or CV_Bool type representing the mask for the inliers
	/// as given by the [find_homography] function
	///
	/// This function is intended to filter the output of the [decompose_homography_mat] based on additional
	/// information as described in [Malis2007](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Malis2007) . The summary of the method: the [decompose_homography_mat] function
	/// returns 2 unique solutions and their "opposites" for a total of 4 solutions. If we have access to the
	/// sets of points visible in the camera frame before and after the homography transformation is applied,
	/// we can determine which are the true potential solutions and which are the opposites by verifying which
	/// homographies are consistent with all visible reference points being in front of the camera. The inputs
	/// are left unchanged; the filtered solution set is returned as indices into the existing one.
	///
	/// ## C++ default parameters
	/// * points_mask: noArray()
	#[inline]
	pub fn filter_homography_decomp_by_visible_refpoints(rotations: &impl ToInputArray, normals: &impl ToInputArray, before_points: &impl ToInputArray, after_points: &impl ToInputArray, possible_solutions: &mut impl ToOutputArray, points_mask: &impl ToInputArray) -> Result<()> {
		input_array_arg!(rotations);
		input_array_arg!(normals);
		input_array_arg!(before_points);
		input_array_arg!(after_points);
		output_array_arg!(possible_solutions);
		input_array_arg!(points_mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(rotations.as_raw__InputArray(), normals.as_raw__InputArray(), before_points.as_raw__InputArray(), after_points.as_raw__InputArray(), possible_solutions.as_raw__OutputArray(), points_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates an essential matrix from the corresponding points in two images.
	///
	/// ## Parameters
	/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
	/// be floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1.
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix. If this assumption does not hold for your use case, use another
	/// function overload or [undistort_points] with `P = cv::NoArray()` for both cameras to transform image
	/// points to normalized image coordinates, which are valid for the identity camera intrinsic matrix.
	/// When passing these coordinates, pass the identity matrix for this parameter.
	/// * method: Method for computing an essential matrix.
	/// *   [RANSAC] for the RANSAC algorithm.
	/// *   [LMEDS] for the LMedS algorithm.
	/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
	/// confidence (probability) that the estimated matrix is correct.
	/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
	/// for the other points. The array is computed only in the RANSAC and LMedS methods.
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03) .
	/// [SteweniusCFS](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively. The result of this function may be passed further to
	/// [decompose_essential_mat] or [recover_pose] to recover the relative pose between cameras.
	///
	/// ## Overloaded parameters
	///
	/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
	/// be floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * focal: focal length of the camera. Note that this function assumes that points1 and points2
	/// are feature points from cameras with same focal length and principal point.
	/// * pp: principal point of the camera.
	/// * method: Method for computing a fundamental matrix.
	/// *   [RANSAC] for the RANSAC algorithm.
	/// *   [LMEDS] for the LMedS algorithm.
	/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
	/// confidence (probability) that the estimated matrix is correct.
	/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
	/// for the other points. The array is computed only in the RANSAC and LMedS methods.
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// This function differs from the one above that it computes camera intrinsic matrix from focal length and
	/// principal point:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?A%20%3D%0A%5Cbegin%7Bbmatrix%7D%0Af%20%26%200%20%26%20x%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%20f%20%26%20y%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D)
	///
	/// ## Note
	/// This alternative version of [find_essential_mat_1] function uses the following default values for its arguments:
	/// * focal: 1.0
	/// * pp: Point2d(0,0)
	/// * method: RANSAC
	/// * prob: 0.999
	/// * threshold: 1.0
	/// * max_iters: 1000
	/// * mask: noArray()
	#[inline]
	pub fn find_essential_mat_1_def(points1: &impl ToInputArray, points2: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Calculates an essential matrix from the corresponding points in two images.
	///
	/// ## Parameters
	/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
	/// be floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1.
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix. If this assumption does not hold for your use case, use another
	/// function overload or [undistort_points] with `P = cv::NoArray()` for both cameras to transform image
	/// points to normalized image coordinates, which are valid for the identity camera intrinsic matrix.
	/// When passing these coordinates, pass the identity matrix for this parameter.
	/// * method: Method for computing an essential matrix.
	/// *   [RANSAC] for the RANSAC algorithm.
	/// *   [LMEDS] for the LMedS algorithm.
	/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
	/// confidence (probability) that the estimated matrix is correct.
	/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
	/// for the other points. The array is computed only in the RANSAC and LMedS methods.
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03) .
	/// [SteweniusCFS](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively. The result of this function may be passed further to
	/// [decompose_essential_mat] or [recover_pose] to recover the relative pose between cameras.
	///
	/// ## Note
	/// This alternative version of [find_essential_mat] function uses the following default values for its arguments:
	/// * method: RANSAC
	/// * prob: 0.999
	/// * threshold: 1.0
	/// * max_iters: 1000
	/// * mask: noArray()
	#[inline]
	pub fn find_essential_mat_def(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		input_array_arg!(camera_matrix);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Calculates an essential matrix from the corresponding points in two images from potentially two different cameras.
	///
	/// ## Parameters
	/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
	/// be floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1.
	/// * cameraMatrix1: Camera matrix for the first camera ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
	/// * cameraMatrix2: Camera matrix for the second camera ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
	/// * distCoeffs1: Input vector of distortion coefficients for the first camera
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
	/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
	/// * distCoeffs2: Input vector of distortion coefficients for the second camera
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
	/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
	/// * method: Method for computing an essential matrix.
	/// *   [RANSAC] for the RANSAC algorithm.
	/// *   [LMEDS] for the LMedS algorithm.
	/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
	/// confidence (probability) that the estimated matrix is correct.
	/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
	/// for the other points. The array is computed only in the RANSAC and LMedS methods.
	///
	/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03) .
	/// [SteweniusCFS](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively. The result of this function may be passed further to
	/// [decompose_essential_mat] or  [recover_pose] to recover the relative pose between cameras.
	///
	/// ## Note
	/// This alternative version of [find_essential_mat_2] function uses the following default values for its arguments:
	/// * method: RANSAC
	/// * prob: 0.999
	/// * threshold: 1.0
	/// * mask: noArray()
	#[inline]
	pub fn find_essential_mat_2_def(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		input_array_arg!(camera_matrix1);
		input_array_arg!(dist_coeffs1);
		input_array_arg!(camera_matrix2);
		input_array_arg!(dist_coeffs2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	#[inline]
	pub fn find_essential_mat_3(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeff1: &impl ToInputArray, dist_coeff2: &impl ToInputArray, mask: &mut impl ToOutputArray, params: crate::geometry::UsacParams) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		input_array_arg!(camera_matrix1);
		input_array_arg!(camera_matrix2);
		input_array_arg!(dist_coeff1);
		input_array_arg!(dist_coeff2);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeff1.as_raw__InputArray(), dist_coeff2.as_raw__InputArray(), mask.as_raw__OutputArray(), &params, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Calculates an essential matrix from the corresponding points in two images from potentially two different cameras.
	///
	/// ## Parameters
	/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
	/// be floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1.
	/// * cameraMatrix1: Camera matrix for the first camera ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
	/// * cameraMatrix2: Camera matrix for the second camera ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
	/// * distCoeffs1: Input vector of distortion coefficients for the first camera
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
	/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
	/// * distCoeffs2: Input vector of distortion coefficients for the second camera
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
	/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
	/// * method: Method for computing an essential matrix.
	/// *   [RANSAC] for the RANSAC algorithm.
	/// *   [LMEDS] for the LMedS algorithm.
	/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
	/// confidence (probability) that the estimated matrix is correct.
	/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
	/// for the other points. The array is computed only in the RANSAC and LMedS methods.
	///
	/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03) .
	/// [SteweniusCFS](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively. The result of this function may be passed further to
	/// [decompose_essential_mat] or  [recover_pose] to recover the relative pose between cameras.
	///
	/// ## C++ default parameters
	/// * method: RANSAC
	/// * prob: 0.999
	/// * threshold: 1.0
	/// * mask: noArray()
	#[inline]
	pub fn find_essential_mat_2(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, method: i32, prob: f64, threshold: f64, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		input_array_arg!(camera_matrix1);
		input_array_arg!(dist_coeffs1);
		input_array_arg!(camera_matrix2);
		input_array_arg!(dist_coeffs2);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), method, prob, threshold, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Calculates an essential matrix from the corresponding points in two images.
	///
	/// ## Parameters
	/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
	/// be floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1.
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix. If this assumption does not hold for your use case, use another
	/// function overload or [undistort_points] with `P = cv::NoArray()` for both cameras to transform image
	/// points to normalized image coordinates, which are valid for the identity camera intrinsic matrix.
	/// When passing these coordinates, pass the identity matrix for this parameter.
	/// * method: Method for computing an essential matrix.
	/// *   [RANSAC] for the RANSAC algorithm.
	/// *   [LMEDS] for the LMedS algorithm.
	/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
	/// confidence (probability) that the estimated matrix is correct.
	/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
	/// for the other points. The array is computed only in the RANSAC and LMedS methods.
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03) .
	/// [SteweniusCFS](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively. The result of this function may be passed further to
	/// [decompose_essential_mat] or [recover_pose] to recover the relative pose between cameras.
	///
	/// ## C++ default parameters
	/// * method: RANSAC
	/// * prob: 0.999
	/// * threshold: 1.0
	/// * max_iters: 1000
	/// * mask: noArray()
	#[inline]
	pub fn find_essential_mat(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, method: i32, prob: f64, threshold: f64, max_iters: i32, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		input_array_arg!(camera_matrix);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_int_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), method, prob, threshold, max_iters, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Calculates an essential matrix from the corresponding points in two images.
	///
	/// ## Parameters
	/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
	/// be floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1.
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix. If this assumption does not hold for your use case, use another
	/// function overload or [undistort_points] with `P = cv::NoArray()` for both cameras to transform image
	/// points to normalized image coordinates, which are valid for the identity camera intrinsic matrix.
	/// When passing these coordinates, pass the identity matrix for this parameter.
	/// * method: Method for computing an essential matrix.
	/// *   [RANSAC] for the RANSAC algorithm.
	/// *   [LMEDS] for the LMedS algorithm.
	/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
	/// confidence (probability) that the estimated matrix is correct.
	/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
	/// for the other points. The array is computed only in the RANSAC and LMedS methods.
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03) .
	/// [SteweniusCFS](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively. The result of this function may be passed further to
	/// [decompose_essential_mat] or [recover_pose] to recover the relative pose between cameras.
	///
	/// ## Overloaded parameters
	///
	/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
	/// be floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * focal: focal length of the camera. Note that this function assumes that points1 and points2
	/// are feature points from cameras with same focal length and principal point.
	/// * pp: principal point of the camera.
	/// * method: Method for computing a fundamental matrix.
	/// *   [RANSAC] for the RANSAC algorithm.
	/// *   [LMEDS] for the LMedS algorithm.
	/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
	/// confidence (probability) that the estimated matrix is correct.
	/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
	/// for the other points. The array is computed only in the RANSAC and LMedS methods.
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// This function differs from the one above that it computes camera intrinsic matrix from focal length and
	/// principal point:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?A%20%3D%0A%5Cbegin%7Bbmatrix%7D%0Af%20%26%200%20%26%20x%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%20f%20%26%20y%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D)
	///
	/// ## C++ default parameters
	/// * focal: 1.0
	/// * pp: Point2d(0,0)
	/// * method: RANSAC
	/// * prob: 0.999
	/// * threshold: 1.0
	/// * max_iters: 1000
	/// * mask: noArray()
	#[inline]
	pub fn find_essential_mat_1(points1: &impl ToInputArray, points2: &impl ToInputArray, focal: f64, pp: core::Point2d, method: i32, prob: f64, threshold: f64, max_iters: i32, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_double_Point2d_int_double_double_int_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), focal, &pp, method, prob, threshold, max_iters, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @example samples/cpp/snippets/epipolar_lines.cpp
	/// An example using the findFundamentalMat function
	///
	/// Calculates a fundamental matrix from the corresponding points in two images.
	///
	/// ## Parameters
	/// * points1: Array of N points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * method: Method for computing a fundamental matrix.
	/// *   [FM_7POINT] for a 7-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%3D%207)
	/// *   [FM_8POINT] for an 8-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_RANSAC] for the RANSAC algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_LMEDS] for the LMedS algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// * ransacReprojThreshold: Parameter used only for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * confidence: Parameter used for the RANSAC and LMedS methods only. It specifies a desirable level
	/// of confidence (probability) that the estimated matrix is correct.
	/// * mask:[out] optional output mask
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20F%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?F) is a fundamental matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively.
	///
	/// The function calculates the fundamental matrix using one of four methods listed above and returns
	/// the found fundamental matrix. Normally just one matrix is found. But in case of the 7-point
	/// algorithm, the function may return up to 3 solutions ( ![inline formula](https://latex.codecogs.com/png.latex?9%20%5Ctimes%203) matrix that stores all 3
	/// matrices sequentially).
	///
	/// The calculated fundamental matrix may be passed further to [compute_correspond_epilines] that finds the
	/// epipolar lines corresponding to the specified points. It can also be passed to
	/// [stereo_rectify_uncalibrated] to compute the rectification transformation. :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    Mat fundamental_matrix =
	///      findFundamentalMat(points1, points2, FM_RANSAC, 3, 0.99);
	/// ```
	///
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [find_fundamental_mat_1] function uses the following default values for its arguments:
	/// * method: FM_RANSAC
	/// * ransac_reproj_threshold: 3.
	/// * confidence: 0.99
	/// * mask: noArray()
	#[inline]
	pub fn find_fundamental_mat_1_def(points1: &impl ToInputArray, points2: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @example samples/cpp/snippets/epipolar_lines.cpp
	/// An example using the findFundamentalMat function
	///
	/// Calculates a fundamental matrix from the corresponding points in two images.
	///
	/// ## Parameters
	/// * points1: Array of N points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * method: Method for computing a fundamental matrix.
	/// *   [FM_7POINT] for a 7-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%3D%207)
	/// *   [FM_8POINT] for an 8-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_RANSAC] for the RANSAC algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_LMEDS] for the LMedS algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// * ransacReprojThreshold: Parameter used only for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * confidence: Parameter used for the RANSAC and LMedS methods only. It specifies a desirable level
	/// of confidence (probability) that the estimated matrix is correct.
	/// * mask:[out] optional output mask
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20F%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?F) is a fundamental matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively.
	///
	/// The function calculates the fundamental matrix using one of four methods listed above and returns
	/// the found fundamental matrix. Normally just one matrix is found. But in case of the 7-point
	/// algorithm, the function may return up to 3 solutions ( ![inline formula](https://latex.codecogs.com/png.latex?9%20%5Ctimes%203) matrix that stores all 3
	/// matrices sequentially).
	///
	/// The calculated fundamental matrix may be passed further to [compute_correspond_epilines] that finds the
	/// epipolar lines corresponding to the specified points. It can also be passed to
	/// [stereo_rectify_uncalibrated] to compute the rectification transformation. :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    Mat fundamental_matrix =
	///      findFundamentalMat(points1, points2, FM_RANSAC, 3, 0.99);
	/// ```
	///
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [find_fundamental_mat_2] function uses the following default values for its arguments:
	/// * method: FM_RANSAC
	/// * ransac_reproj_threshold: 3.
	/// * confidence: 0.99
	#[inline]
	pub fn find_fundamental_mat_2_def(points1: &impl ToInputArray, points2: &impl ToInputArray, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	#[inline]
	pub fn find_fundamental_mat_3(points1: &impl ToInputArray, points2: &impl ToInputArray, mask: &mut impl ToOutputArray, params: crate::geometry::UsacParams) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), mask.as_raw__OutputArray(), &params, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @example samples/cpp/snippets/epipolar_lines.cpp
	/// An example using the findFundamentalMat function
	///
	/// Calculates a fundamental matrix from the corresponding points in two images.
	///
	/// ## Parameters
	/// * points1: Array of N points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * method: Method for computing a fundamental matrix.
	/// *   [FM_7POINT] for a 7-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%3D%207)
	/// *   [FM_8POINT] for an 8-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_RANSAC] for the RANSAC algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_LMEDS] for the LMedS algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// * ransacReprojThreshold: Parameter used only for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * confidence: Parameter used for the RANSAC and LMedS methods only. It specifies a desirable level
	/// of confidence (probability) that the estimated matrix is correct.
	/// * mask:[out] optional output mask
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20F%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?F) is a fundamental matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively.
	///
	/// The function calculates the fundamental matrix using one of four methods listed above and returns
	/// the found fundamental matrix. Normally just one matrix is found. But in case of the 7-point
	/// algorithm, the function may return up to 3 solutions ( ![inline formula](https://latex.codecogs.com/png.latex?9%20%5Ctimes%203) matrix that stores all 3
	/// matrices sequentially).
	///
	/// The calculated fundamental matrix may be passed further to [compute_correspond_epilines] that finds the
	/// epipolar lines corresponding to the specified points. It can also be passed to
	/// [stereo_rectify_uncalibrated] to compute the rectification transformation. :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    Mat fundamental_matrix =
	///      findFundamentalMat(points1, points2, FM_RANSAC, 3, 0.99);
	/// ```
	///
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * method: FM_RANSAC
	/// * ransac_reproj_threshold: 3.
	/// * confidence: 0.99
	#[inline]
	pub fn find_fundamental_mat_2(points1: &impl ToInputArray, points2: &impl ToInputArray, mask: &mut impl ToOutputArray, method: i32, ransac_reproj_threshold: f64, confidence: f64) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double(points1.as_raw__InputArray(), points2.as_raw__InputArray(), mask.as_raw__OutputArray(), method, ransac_reproj_threshold, confidence, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @example samples/cpp/snippets/epipolar_lines.cpp
	/// An example using the findFundamentalMat function
	///
	/// Calculates a fundamental matrix from the corresponding points in two images.
	///
	/// ## Parameters
	/// * points1: Array of N points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * method: Method for computing a fundamental matrix.
	/// *   [FM_7POINT] for a 7-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%3D%207)
	/// *   [FM_8POINT] for an 8-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_RANSAC] for the RANSAC algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_LMEDS] for the LMedS algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// * ransacReprojThreshold: Parameter used only for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * confidence: Parameter used for the RANSAC and LMedS methods only. It specifies a desirable level
	/// of confidence (probability) that the estimated matrix is correct.
	/// * mask:[out] optional output mask
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20F%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?F) is a fundamental matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively.
	///
	/// The function calculates the fundamental matrix using one of four methods listed above and returns
	/// the found fundamental matrix. Normally just one matrix is found. But in case of the 7-point
	/// algorithm, the function may return up to 3 solutions ( ![inline formula](https://latex.codecogs.com/png.latex?9%20%5Ctimes%203) matrix that stores all 3
	/// matrices sequentially).
	///
	/// The calculated fundamental matrix may be passed further to [compute_correspond_epilines] that finds the
	/// epipolar lines corresponding to the specified points. It can also be passed to
	/// [stereo_rectify_uncalibrated] to compute the rectification transformation. :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    Mat fundamental_matrix =
	///      findFundamentalMat(points1, points2, FM_RANSAC, 3, 0.99);
	/// ```
	///
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * method: FM_RANSAC
	/// * ransac_reproj_threshold: 3.
	/// * confidence: 0.99
	/// * mask: noArray()
	#[inline]
	pub fn find_fundamental_mat_1(points1: &impl ToInputArray, points2: &impl ToInputArray, method: i32, ransac_reproj_threshold: f64, confidence: f64, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), method, ransac_reproj_threshold, confidence, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @example samples/cpp/snippets/epipolar_lines.cpp
	/// An example using the findFundamentalMat function
	///
	/// Calculates a fundamental matrix from the corresponding points in two images.
	///
	/// ## Parameters
	/// * points1: Array of N points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * method: Method for computing a fundamental matrix.
	/// *   [FM_7POINT] for a 7-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%3D%207)
	/// *   [FM_8POINT] for an 8-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_RANSAC] for the RANSAC algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_LMEDS] for the LMedS algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// * ransacReprojThreshold: Parameter used only for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * confidence: Parameter used for the RANSAC and LMedS methods only. It specifies a desirable level
	/// of confidence (probability) that the estimated matrix is correct.
	/// * mask:[out] optional output mask
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20F%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?F) is a fundamental matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively.
	///
	/// The function calculates the fundamental matrix using one of four methods listed above and returns
	/// the found fundamental matrix. Normally just one matrix is found. But in case of the 7-point
	/// algorithm, the function may return up to 3 solutions ( ![inline formula](https://latex.codecogs.com/png.latex?9%20%5Ctimes%203) matrix that stores all 3
	/// matrices sequentially).
	///
	/// The calculated fundamental matrix may be passed further to [compute_correspond_epilines] that finds the
	/// epipolar lines corresponding to the specified points. It can also be passed to
	/// [stereo_rectify_uncalibrated] to compute the rectification transformation. :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    Mat fundamental_matrix =
	///      findFundamentalMat(points1, points2, FM_RANSAC, 3, 0.99);
	/// ```
	///
	///
	/// ## Note
	/// This alternative version of [find_fundamental_mat] function uses the following default values for its arguments:
	/// * mask: noArray()
	#[inline]
	pub fn find_fundamental_mat_def(points1: &impl ToInputArray, points2: &impl ToInputArray, method: i32, ransac_reproj_threshold: f64, confidence: f64, max_iters: i32) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_int(points1.as_raw__InputArray(), points2.as_raw__InputArray(), method, ransac_reproj_threshold, confidence, max_iters, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @example samples/cpp/snippets/epipolar_lines.cpp
	/// An example using the findFundamentalMat function
	///
	/// Calculates a fundamental matrix from the corresponding points in two images.
	///
	/// ## Parameters
	/// * points1: Array of N points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * method: Method for computing a fundamental matrix.
	/// *   [FM_7POINT] for a 7-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%3D%207)
	/// *   [FM_8POINT] for an 8-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_RANSAC] for the RANSAC algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// *   [FM_LMEDS] for the LMedS algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
	/// * ransacReprojThreshold: Parameter used only for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * confidence: Parameter used for the RANSAC and LMedS methods only. It specifies a desirable level
	/// of confidence (probability) that the estimated matrix is correct.
	/// * mask:[out] optional output mask
	/// * maxIters: The maximum number of robust method iterations.
	///
	/// The epipolar geometry is described by the following equation:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20F%20%5Bp%5F1%3B%201%5D%20%3D%200)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?F) is a fundamental matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
	/// second images, respectively.
	///
	/// The function calculates the fundamental matrix using one of four methods listed above and returns
	/// the found fundamental matrix. Normally just one matrix is found. But in case of the 7-point
	/// algorithm, the function may return up to 3 solutions ( ![inline formula](https://latex.codecogs.com/png.latex?9%20%5Ctimes%203) matrix that stores all 3
	/// matrices sequentially).
	///
	/// The calculated fundamental matrix may be passed further to [compute_correspond_epilines] that finds the
	/// epipolar lines corresponding to the specified points. It can also be passed to
	/// [stereo_rectify_uncalibrated] to compute the rectification transformation. :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    Mat fundamental_matrix =
	///      findFundamentalMat(points1, points2, FM_RANSAC, 3, 0.99);
	/// ```
	///
	///
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	pub fn find_fundamental_mat(points1: &impl ToInputArray, points2: &impl ToInputArray, method: i32, ransac_reproj_threshold: f64, confidence: f64, max_iters: i32, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_int_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), method, ransac_reproj_threshold, confidence, max_iters, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Finds a perspective transformation between two planes.
	///
	/// ## Parameters
	/// * srcPoints: Coordinates of the points in the original plane, a matrix of the type CV_32FC2
	/// or vector\<Point2f\> .
	/// * dstPoints: Coordinates of the points in the target plane, a matrix of the type CV_32FC2 or
	/// a vector\<Point2f\> .
	/// * method: Method used to compute a homography matrix. The following methods are possible:
	/// *   **0** - a regular method using all the points, i.e., the least squares method
	/// *   [RANSAC] - RANSAC-based robust method
	/// *   [LMEDS] - Least-Median robust method
	/// *   [RHO] - PROSAC-based robust method
	/// * ransacReprojThreshold: Maximum allowed reprojection error to treat a point pair as an inlier
	/// (used in the RANSAC and RHO methods only). That is, if
	/// ![block formula](https://latex.codecogs.com/png.latex?%5C%7C%20%5Ctexttt%7BdstPoints%7D%20%5Fi%20%2D%20%20%5Ctexttt%7BconvertPointsHomogeneous%7D%20%28%20%5Ctexttt%7BH%7D%20%5Ccdot%20%5Ctexttt%7BsrcPoints%7D%20%5Fi%29%20%5C%7C%5F2%20%20%3E%20%20%5Ctexttt%7BransacReprojThreshold%7D)
	/// then the point ![inline formula](https://latex.codecogs.com/png.latex?i) is considered as an outlier. If srcPoints and dstPoints are measured in pixels,
	/// it usually makes sense to set this parameter somewhere in the range of 1 to 10.
	/// * mask: Optional output mask set by a robust method ( RANSAC or LMeDS ). Note that the input
	/// mask values are ignored.
	/// * maxIters: The maximum number of RANSAC iterations.
	/// * confidence: Confidence level, between 0 and 1.
	///
	/// The function finds and returns the perspective transformation ![inline formula](https://latex.codecogs.com/png.latex?H) between the source and the
	/// destination planes:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?s%5Fi%20%20%5Cbegin%7Bbmatrix%7D%20x%27%5Fi%5C%5C%20y%27%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%5Csim%20H%20%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%5C%5C%20y%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D)
	///
	/// so that the back-projection error
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Csum%20%5Fi%20%5Cleft%20%28%20x%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B11%7D%20x%5Fi%20%2B%20h%5F%7B12%7D%20y%5Fi%20%2B%20h%5F%7B13%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2%2B%20%5Cleft%20%28%20y%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B21%7D%20x%5Fi%20%2B%20h%5F%7B22%7D%20y%5Fi%20%2B%20h%5F%7B23%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2)
	///
	/// is minimized. If the parameter method is set to the default value 0, the function uses all the point
	/// pairs to compute an initial homography estimate with a simple least-squares scheme.
	///
	/// However, if not all of the point pairs ( ![inline formula](https://latex.codecogs.com/png.latex?srcPoints%5Fi), ![inline formula](https://latex.codecogs.com/png.latex?dstPoints%5Fi) ) fit the rigid perspective
	/// transformation (that is, there are some outliers), this initial estimate will be poor. In this case,
	/// you can use one of the three robust methods. The methods RANSAC, LMeDS and RHO try many different
	/// random subsets of the corresponding point pairs (of four pairs each, collinear pairs are discarded), estimate the homography matrix
	/// using this subset and a simple least-squares algorithm, and then compute the quality/goodness of the
	/// computed homography (which is the number of inliers for RANSAC or the least median re-projection error for
	/// LMeDS). The best subset is then used to produce the initial estimate of the homography matrix and
	/// the mask of inliers/outliers.
	///
	/// Regardless of the method, robust or not, the computed homography matrix is refined further (using
	/// inliers only in case of a robust method) with the Levenberg-Marquardt method to reduce the
	/// re-projection error even more.
	///
	/// The methods RANSAC and RHO can handle practically any ratio of outliers but need a threshold to
	/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
	/// correctly only when there are more than 50% of inliers. Finally, if there are no outliers and the
	/// noise is rather small, use the default method (method=0).
	///
	/// The function is used to find initial intrinsic and extrinsic matrices. Homography matrix is
	/// determined up to a scale. If ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D) is non-zero, the matrix is normalized so that ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D%3D1).
	///
	/// Note: Whenever an ![inline formula](https://latex.codecogs.com/png.latex?H) matrix cannot be estimated, an empty one will be returned.
	/// ## See also
	/// getAffineTransform, estimateAffine2D, estimateAffinePartial2D, getPerspectiveTransform, warpPerspective,
	/// perspectiveTransform
	///
	/// ## Note
	/// This alternative version of [find_homography] function uses the following default values for its arguments:
	/// * method: 0
	/// * ransac_reproj_threshold: 3
	/// * mask: noArray()
	/// * max_iters: 2000
	/// * confidence: 0.995
	#[inline]
	pub fn find_homography_def(src_points: &impl ToInputArray, dst_points: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(src_points);
		input_array_arg!(dst_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findHomography_const__InputArrayR_const__InputArrayR(src_points.as_raw__InputArray(), dst_points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Finds a perspective transformation between two planes.
	///
	/// ## Parameters
	/// * srcPoints: Coordinates of the points in the original plane, a matrix of the type CV_32FC2
	/// or vector\<Point2f\> .
	/// * dstPoints: Coordinates of the points in the target plane, a matrix of the type CV_32FC2 or
	/// a vector\<Point2f\> .
	/// * method: Method used to compute a homography matrix. The following methods are possible:
	/// *   **0** - a regular method using all the points, i.e., the least squares method
	/// *   [RANSAC] - RANSAC-based robust method
	/// *   [LMEDS] - Least-Median robust method
	/// *   [RHO] - PROSAC-based robust method
	/// * ransacReprojThreshold: Maximum allowed reprojection error to treat a point pair as an inlier
	/// (used in the RANSAC and RHO methods only). That is, if
	/// ![block formula](https://latex.codecogs.com/png.latex?%5C%7C%20%5Ctexttt%7BdstPoints%7D%20%5Fi%20%2D%20%20%5Ctexttt%7BconvertPointsHomogeneous%7D%20%28%20%5Ctexttt%7BH%7D%20%5Ccdot%20%5Ctexttt%7BsrcPoints%7D%20%5Fi%29%20%5C%7C%5F2%20%20%3E%20%20%5Ctexttt%7BransacReprojThreshold%7D)
	/// then the point ![inline formula](https://latex.codecogs.com/png.latex?i) is considered as an outlier. If srcPoints and dstPoints are measured in pixels,
	/// it usually makes sense to set this parameter somewhere in the range of 1 to 10.
	/// * mask: Optional output mask set by a robust method ( RANSAC or LMeDS ). Note that the input
	/// mask values are ignored.
	/// * maxIters: The maximum number of RANSAC iterations.
	/// * confidence: Confidence level, between 0 and 1.
	///
	/// The function finds and returns the perspective transformation ![inline formula](https://latex.codecogs.com/png.latex?H) between the source and the
	/// destination planes:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?s%5Fi%20%20%5Cbegin%7Bbmatrix%7D%20x%27%5Fi%5C%5C%20y%27%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%5Csim%20H%20%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%5C%5C%20y%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D)
	///
	/// so that the back-projection error
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Csum%20%5Fi%20%5Cleft%20%28%20x%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B11%7D%20x%5Fi%20%2B%20h%5F%7B12%7D%20y%5Fi%20%2B%20h%5F%7B13%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2%2B%20%5Cleft%20%28%20y%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B21%7D%20x%5Fi%20%2B%20h%5F%7B22%7D%20y%5Fi%20%2B%20h%5F%7B23%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2)
	///
	/// is minimized. If the parameter method is set to the default value 0, the function uses all the point
	/// pairs to compute an initial homography estimate with a simple least-squares scheme.
	///
	/// However, if not all of the point pairs ( ![inline formula](https://latex.codecogs.com/png.latex?srcPoints%5Fi), ![inline formula](https://latex.codecogs.com/png.latex?dstPoints%5Fi) ) fit the rigid perspective
	/// transformation (that is, there are some outliers), this initial estimate will be poor. In this case,
	/// you can use one of the three robust methods. The methods RANSAC, LMeDS and RHO try many different
	/// random subsets of the corresponding point pairs (of four pairs each, collinear pairs are discarded), estimate the homography matrix
	/// using this subset and a simple least-squares algorithm, and then compute the quality/goodness of the
	/// computed homography (which is the number of inliers for RANSAC or the least median re-projection error for
	/// LMeDS). The best subset is then used to produce the initial estimate of the homography matrix and
	/// the mask of inliers/outliers.
	///
	/// Regardless of the method, robust or not, the computed homography matrix is refined further (using
	/// inliers only in case of a robust method) with the Levenberg-Marquardt method to reduce the
	/// re-projection error even more.
	///
	/// The methods RANSAC and RHO can handle practically any ratio of outliers but need a threshold to
	/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
	/// correctly only when there are more than 50% of inliers. Finally, if there are no outliers and the
	/// noise is rather small, use the default method (method=0).
	///
	/// The function is used to find initial intrinsic and extrinsic matrices. Homography matrix is
	/// determined up to a scale. If ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D) is non-zero, the matrix is normalized so that ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D%3D1).
	///
	/// Note: Whenever an ![inline formula](https://latex.codecogs.com/png.latex?H) matrix cannot be estimated, an empty one will be returned.
	/// ## See also
	/// getAffineTransform, estimateAffine2D, estimateAffinePartial2D, getPerspectiveTransform, warpPerspective,
	/// perspectiveTransform
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [find_homography_1] function uses the following default values for its arguments:
	/// * method: 0
	/// * ransac_reproj_threshold: 3
	#[inline]
	pub fn find_homography_1_def(src_points: &impl ToInputArray, dst_points: &impl ToInputArray, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
		input_array_arg!(src_points);
		input_array_arg!(dst_points);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src_points.as_raw__InputArray(), dst_points.as_raw__InputArray(), mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	#[inline]
	pub fn find_homography_2(src_points: &impl ToInputArray, dst_points: &impl ToInputArray, mask: &mut impl ToOutputArray, params: crate::geometry::UsacParams) -> Result<core::Mat> {
		input_array_arg!(src_points);
		input_array_arg!(dst_points);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(src_points.as_raw__InputArray(), dst_points.as_raw__InputArray(), mask.as_raw__OutputArray(), &params, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Finds a perspective transformation between two planes.
	///
	/// ## Parameters
	/// * srcPoints: Coordinates of the points in the original plane, a matrix of the type CV_32FC2
	/// or vector\<Point2f\> .
	/// * dstPoints: Coordinates of the points in the target plane, a matrix of the type CV_32FC2 or
	/// a vector\<Point2f\> .
	/// * method: Method used to compute a homography matrix. The following methods are possible:
	/// *   **0** - a regular method using all the points, i.e., the least squares method
	/// *   [RANSAC] - RANSAC-based robust method
	/// *   [LMEDS] - Least-Median robust method
	/// *   [RHO] - PROSAC-based robust method
	/// * ransacReprojThreshold: Maximum allowed reprojection error to treat a point pair as an inlier
	/// (used in the RANSAC and RHO methods only). That is, if
	/// ![block formula](https://latex.codecogs.com/png.latex?%5C%7C%20%5Ctexttt%7BdstPoints%7D%20%5Fi%20%2D%20%20%5Ctexttt%7BconvertPointsHomogeneous%7D%20%28%20%5Ctexttt%7BH%7D%20%5Ccdot%20%5Ctexttt%7BsrcPoints%7D%20%5Fi%29%20%5C%7C%5F2%20%20%3E%20%20%5Ctexttt%7BransacReprojThreshold%7D)
	/// then the point ![inline formula](https://latex.codecogs.com/png.latex?i) is considered as an outlier. If srcPoints and dstPoints are measured in pixels,
	/// it usually makes sense to set this parameter somewhere in the range of 1 to 10.
	/// * mask: Optional output mask set by a robust method ( RANSAC or LMeDS ). Note that the input
	/// mask values are ignored.
	/// * maxIters: The maximum number of RANSAC iterations.
	/// * confidence: Confidence level, between 0 and 1.
	///
	/// The function finds and returns the perspective transformation ![inline formula](https://latex.codecogs.com/png.latex?H) between the source and the
	/// destination planes:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?s%5Fi%20%20%5Cbegin%7Bbmatrix%7D%20x%27%5Fi%5C%5C%20y%27%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%5Csim%20H%20%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%5C%5C%20y%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D)
	///
	/// so that the back-projection error
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Csum%20%5Fi%20%5Cleft%20%28%20x%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B11%7D%20x%5Fi%20%2B%20h%5F%7B12%7D%20y%5Fi%20%2B%20h%5F%7B13%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2%2B%20%5Cleft%20%28%20y%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B21%7D%20x%5Fi%20%2B%20h%5F%7B22%7D%20y%5Fi%20%2B%20h%5F%7B23%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2)
	///
	/// is minimized. If the parameter method is set to the default value 0, the function uses all the point
	/// pairs to compute an initial homography estimate with a simple least-squares scheme.
	///
	/// However, if not all of the point pairs ( ![inline formula](https://latex.codecogs.com/png.latex?srcPoints%5Fi), ![inline formula](https://latex.codecogs.com/png.latex?dstPoints%5Fi) ) fit the rigid perspective
	/// transformation (that is, there are some outliers), this initial estimate will be poor. In this case,
	/// you can use one of the three robust methods. The methods RANSAC, LMeDS and RHO try many different
	/// random subsets of the corresponding point pairs (of four pairs each, collinear pairs are discarded), estimate the homography matrix
	/// using this subset and a simple least-squares algorithm, and then compute the quality/goodness of the
	/// computed homography (which is the number of inliers for RANSAC or the least median re-projection error for
	/// LMeDS). The best subset is then used to produce the initial estimate of the homography matrix and
	/// the mask of inliers/outliers.
	///
	/// Regardless of the method, robust or not, the computed homography matrix is refined further (using
	/// inliers only in case of a robust method) with the Levenberg-Marquardt method to reduce the
	/// re-projection error even more.
	///
	/// The methods RANSAC and RHO can handle practically any ratio of outliers but need a threshold to
	/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
	/// correctly only when there are more than 50% of inliers. Finally, if there are no outliers and the
	/// noise is rather small, use the default method (method=0).
	///
	/// The function is used to find initial intrinsic and extrinsic matrices. Homography matrix is
	/// determined up to a scale. If ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D) is non-zero, the matrix is normalized so that ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D%3D1).
	///
	/// Note: Whenever an ![inline formula](https://latex.codecogs.com/png.latex?H) matrix cannot be estimated, an empty one will be returned.
	/// ## See also
	/// getAffineTransform, estimateAffine2D, estimateAffinePartial2D, getPerspectiveTransform, warpPerspective,
	/// perspectiveTransform
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * method: 0
	/// * ransac_reproj_threshold: 3
	#[inline]
	pub fn find_homography_1(src_points: &impl ToInputArray, dst_points: &impl ToInputArray, mask: &mut impl ToOutputArray, method: i32, ransac_reproj_threshold: f64) -> Result<core::Mat> {
		input_array_arg!(src_points);
		input_array_arg!(dst_points);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double(src_points.as_raw__InputArray(), dst_points.as_raw__InputArray(), mask.as_raw__OutputArray(), method, ransac_reproj_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Finds a perspective transformation between two planes.
	///
	/// ## Parameters
	/// * srcPoints: Coordinates of the points in the original plane, a matrix of the type CV_32FC2
	/// or vector\<Point2f\> .
	/// * dstPoints: Coordinates of the points in the target plane, a matrix of the type CV_32FC2 or
	/// a vector\<Point2f\> .
	/// * method: Method used to compute a homography matrix. The following methods are possible:
	/// *   **0** - a regular method using all the points, i.e., the least squares method
	/// *   [RANSAC] - RANSAC-based robust method
	/// *   [LMEDS] - Least-Median robust method
	/// *   [RHO] - PROSAC-based robust method
	/// * ransacReprojThreshold: Maximum allowed reprojection error to treat a point pair as an inlier
	/// (used in the RANSAC and RHO methods only). That is, if
	/// ![block formula](https://latex.codecogs.com/png.latex?%5C%7C%20%5Ctexttt%7BdstPoints%7D%20%5Fi%20%2D%20%20%5Ctexttt%7BconvertPointsHomogeneous%7D%20%28%20%5Ctexttt%7BH%7D%20%5Ccdot%20%5Ctexttt%7BsrcPoints%7D%20%5Fi%29%20%5C%7C%5F2%20%20%3E%20%20%5Ctexttt%7BransacReprojThreshold%7D)
	/// then the point ![inline formula](https://latex.codecogs.com/png.latex?i) is considered as an outlier. If srcPoints and dstPoints are measured in pixels,
	/// it usually makes sense to set this parameter somewhere in the range of 1 to 10.
	/// * mask: Optional output mask set by a robust method ( RANSAC or LMeDS ). Note that the input
	/// mask values are ignored.
	/// * maxIters: The maximum number of RANSAC iterations.
	/// * confidence: Confidence level, between 0 and 1.
	///
	/// The function finds and returns the perspective transformation ![inline formula](https://latex.codecogs.com/png.latex?H) between the source and the
	/// destination planes:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?s%5Fi%20%20%5Cbegin%7Bbmatrix%7D%20x%27%5Fi%5C%5C%20y%27%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%5Csim%20H%20%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%5C%5C%20y%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D)
	///
	/// so that the back-projection error
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Csum%20%5Fi%20%5Cleft%20%28%20x%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B11%7D%20x%5Fi%20%2B%20h%5F%7B12%7D%20y%5Fi%20%2B%20h%5F%7B13%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2%2B%20%5Cleft%20%28%20y%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B21%7D%20x%5Fi%20%2B%20h%5F%7B22%7D%20y%5Fi%20%2B%20h%5F%7B23%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2)
	///
	/// is minimized. If the parameter method is set to the default value 0, the function uses all the point
	/// pairs to compute an initial homography estimate with a simple least-squares scheme.
	///
	/// However, if not all of the point pairs ( ![inline formula](https://latex.codecogs.com/png.latex?srcPoints%5Fi), ![inline formula](https://latex.codecogs.com/png.latex?dstPoints%5Fi) ) fit the rigid perspective
	/// transformation (that is, there are some outliers), this initial estimate will be poor. In this case,
	/// you can use one of the three robust methods. The methods RANSAC, LMeDS and RHO try many different
	/// random subsets of the corresponding point pairs (of four pairs each, collinear pairs are discarded), estimate the homography matrix
	/// using this subset and a simple least-squares algorithm, and then compute the quality/goodness of the
	/// computed homography (which is the number of inliers for RANSAC or the least median re-projection error for
	/// LMeDS). The best subset is then used to produce the initial estimate of the homography matrix and
	/// the mask of inliers/outliers.
	///
	/// Regardless of the method, robust or not, the computed homography matrix is refined further (using
	/// inliers only in case of a robust method) with the Levenberg-Marquardt method to reduce the
	/// re-projection error even more.
	///
	/// The methods RANSAC and RHO can handle practically any ratio of outliers but need a threshold to
	/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
	/// correctly only when there are more than 50% of inliers. Finally, if there are no outliers and the
	/// noise is rather small, use the default method (method=0).
	///
	/// The function is used to find initial intrinsic and extrinsic matrices. Homography matrix is
	/// determined up to a scale. If ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D) is non-zero, the matrix is normalized so that ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D%3D1).
	///
	/// Note: Whenever an ![inline formula](https://latex.codecogs.com/png.latex?H) matrix cannot be estimated, an empty one will be returned.
	/// ## See also
	/// getAffineTransform, estimateAffine2D, estimateAffinePartial2D, getPerspectiveTransform, warpPerspective,
	/// perspectiveTransform
	///
	/// ## C++ default parameters
	/// * method: 0
	/// * ransac_reproj_threshold: 3
	/// * mask: noArray()
	/// * max_iters: 2000
	/// * confidence: 0.995
	#[inline]
	pub fn find_homography(src_points: &impl ToInputArray, dst_points: &impl ToInputArray, method: i32, ransac_reproj_threshold: f64, mask: &mut impl ToOutputArray, max_iters: i32, confidence: f64) -> Result<core::Mat> {
		input_array_arg!(src_points);
		input_array_arg!(dst_points);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findHomography_const__InputArrayR_const__InputArrayR_int_double_const__OutputArrayR_const_int_const_double(src_points.as_raw__InputArray(), dst_points.as_raw__InputArray(), method, ransac_reproj_threshold, mask.as_raw__OutputArray(), max_iters, confidence, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Distorts 2D points using fisheye model.
	///
	/// ## Parameters
	/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
	/// the number of points in the view.
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * alpha: The skew coefficient.
	/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
	///
	/// Note that the function assumes the camera intrinsic matrix of the undistorted points to be identity.
	/// This means if you want to distort image points you have to multiply them with ![inline formula](https://latex.codecogs.com/png.latex?K%5E%7B%2D1%7D) or
	/// use another function overload.
	///
	/// ## Note
	/// This alternative version of [distort_points] function uses the following default values for its arguments:
	/// * alpha: 0
	#[inline]
	pub fn distort_points_def(undistorted: &impl ToInputArray, distorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray) -> Result<()> {
		input_array_arg!(undistorted);
		output_array_arg!(distorted);
		input_array_arg!(k);
		input_array_arg!(d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(undistorted.as_raw__InputArray(), distorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Distorts 2D points using fisheye model.
	///
	/// ## Parameters
	/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
	/// the number of points in the view.
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * alpha: The skew coefficient.
	/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
	///
	/// Note that the function assumes the camera intrinsic matrix of the undistorted points to be identity.
	/// This means if you want to distort image points you have to multiply them with ![inline formula](https://latex.codecogs.com/png.latex?K%5E%7B%2D1%7D) or
	/// use another function overload.
	///
	/// ## Overloaded parameters
	///
	/// Overload of distortPoints function to handle cases when undistorted points are got with non-identity
	/// camera matrix, e.g. output of #estimateNewCameraMatrixForUndistortRectify.
	/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
	/// the number of points in the view.
	/// * Kundistorted: Camera intrinsic matrix used as new camera matrix for undistortion.
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * alpha: The skew coefficient.
	/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
	/// ## See also
	/// estimateNewCameraMatrixForUndistortRectify
	///
	/// ## Note
	/// This alternative version of [distort_points_1] function uses the following default values for its arguments:
	/// * alpha: 0
	#[inline]
	pub fn distort_points_1_def(undistorted: &impl ToInputArray, distorted: &mut impl ToOutputArray, kundistorted: &impl ToInputArray, k: &impl ToInputArray, d: &impl ToInputArray) -> Result<()> {
		input_array_arg!(undistorted);
		output_array_arg!(distorted);
		input_array_arg!(kundistorted);
		input_array_arg!(k);
		input_array_arg!(d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(undistorted.as_raw__InputArray(), distorted.as_raw__OutputArray(), kundistorted.as_raw__InputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Distorts 2D points using fisheye model.
	///
	/// ## Parameters
	/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
	/// the number of points in the view.
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * alpha: The skew coefficient.
	/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
	///
	/// Note that the function assumes the camera intrinsic matrix of the undistorted points to be identity.
	/// This means if you want to distort image points you have to multiply them with ![inline formula](https://latex.codecogs.com/png.latex?K%5E%7B%2D1%7D) or
	/// use another function overload.
	///
	/// ## Overloaded parameters
	///
	/// Overload of distortPoints function to handle cases when undistorted points are got with non-identity
	/// camera matrix, e.g. output of #estimateNewCameraMatrixForUndistortRectify.
	/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
	/// the number of points in the view.
	/// * Kundistorted: Camera intrinsic matrix used as new camera matrix for undistortion.
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * alpha: The skew coefficient.
	/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
	/// ## See also
	/// estimateNewCameraMatrixForUndistortRectify
	///
	/// ## C++ default parameters
	/// * alpha: 0
	#[inline]
	pub fn distort_points_1(undistorted: &impl ToInputArray, distorted: &mut impl ToOutputArray, kundistorted: &impl ToInputArray, k: &impl ToInputArray, d: &impl ToInputArray, alpha: f64) -> Result<()> {
		input_array_arg!(undistorted);
		output_array_arg!(distorted);
		input_array_arg!(kundistorted);
		input_array_arg!(k);
		input_array_arg!(d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double(undistorted.as_raw__InputArray(), distorted.as_raw__OutputArray(), kundistorted.as_raw__InputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), alpha, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Distorts 2D points using fisheye model.
	///
	/// ## Parameters
	/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
	/// the number of points in the view.
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * alpha: The skew coefficient.
	/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
	///
	/// Note that the function assumes the camera intrinsic matrix of the undistorted points to be identity.
	/// This means if you want to distort image points you have to multiply them with ![inline formula](https://latex.codecogs.com/png.latex?K%5E%7B%2D1%7D) or
	/// use another function overload.
	///
	/// ## C++ default parameters
	/// * alpha: 0
	#[inline]
	pub fn distort_points(undistorted: &impl ToInputArray, distorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray, alpha: f64) -> Result<()> {
		input_array_arg!(undistorted);
		output_array_arg!(distorted);
		input_array_arg!(k);
		input_array_arg!(d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_double(undistorted.as_raw__InputArray(), distorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), alpha, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimates new camera intrinsic matrix for undistortion or rectification.
	///
	/// ## Parameters
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * image_size: Size of the image
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
	/// 1-channel or 1x1 3-channel
	/// * P: New camera intrinsic matrix (3x3) or new projection matrix (3x4)
	/// * balance: Sets the new focal length in range between the min focal length and the max focal
	/// length. Balance is in range of [0, 1].
	/// * new_size: the new size
	/// * fov_scale: Divisor for new focal length.
	///
	/// ## Note
	/// This alternative version of [estimate_new_camera_matrix_for_undistort_rectify] function uses the following default values for its arguments:
	/// * balance: 0.0
	/// * new_size: Size()
	/// * fov_scale: 1.0
	#[inline]
	pub fn estimate_new_camera_matrix_for_undistort_rectify_def(k: &impl ToInputArray, d: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, p: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(k);
		input_array_arg!(d);
		input_array_arg!(r);
		output_array_arg!(p);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR(k.as_raw__InputArray(), d.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), p.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimates new camera intrinsic matrix for undistortion or rectification.
	///
	/// ## Parameters
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * image_size: Size of the image
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
	/// 1-channel or 1x1 3-channel
	/// * P: New camera intrinsic matrix (3x3) or new projection matrix (3x4)
	/// * balance: Sets the new focal length in range between the min focal length and the max focal
	/// length. Balance is in range of [0, 1].
	/// * new_size: the new size
	/// * fov_scale: Divisor for new focal length.
	///
	/// ## C++ default parameters
	/// * balance: 0.0
	/// * new_size: Size()
	/// * fov_scale: 1.0
	#[inline]
	pub fn estimate_new_camera_matrix_for_undistort_rectify(k: &impl ToInputArray, d: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, p: &mut impl ToOutputArray, balance: f64, new_size: core::Size, fov_scale: f64) -> Result<()> {
		input_array_arg!(k);
		input_array_arg!(d);
		input_array_arg!(r);
		output_array_arg!(p);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_double_const_SizeR_double(k.as_raw__InputArray(), d.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), p.as_raw__OutputArray(), balance, &new_size, fov_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Projects points using fisheye model
	///
	/// ## Parameters
	/// * objectPoints: Array of object points, 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is
	/// the number of points in the view.
	/// * imagePoints: Output array of image points, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel, or
	/// vector\<Point2f\>.
	/// * affine: 
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * alpha: The skew coefficient.
	/// * jacobian: Optional output 2Nx15 jacobian matrix of derivatives of image points with respect
	/// to components of the focal lengths, coordinates of the principal point, distortion coefficients,
	/// rotation vector, translation vector, and the skew. In the old interface different components of
	/// the jacobian are returned via different output parameters.
	///
	/// The function computes projections of 3D points to the image plane given intrinsic and extrinsic
	/// camera parameters. Optionally, the function computes Jacobians - matrices of partial derivatives of
	/// image points coordinates (as functions of all the input parameters) with respect to the particular
	/// parameters, intrinsic and/or extrinsic.
	///
	/// ## Note
	/// This alternative version of [project_points_1] function uses the following default values for its arguments:
	/// * alpha: 0
	/// * jacobian: noArray()
	#[inline]
	pub fn project_points_1_def(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, affine: core::Affine3d, k: &impl ToInputArray, d: &impl ToInputArray) -> Result<()> {
		input_array_arg!(object_points);
		output_array_arg!(image_points);
		input_array_arg!(k);
		input_array_arg!(d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), &affine, k.as_raw__InputArray(), d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Projects points using fisheye model
	///
	/// ## Parameters
	/// * objectPoints: Array of object points, 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is
	/// the number of points in the view.
	/// * imagePoints: Output array of image points, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel, or
	/// vector\<Point2f\>.
	/// * affine: 
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * alpha: The skew coefficient.
	/// * jacobian: Optional output 2Nx15 jacobian matrix of derivatives of image points with respect
	/// to components of the focal lengths, coordinates of the principal point, distortion coefficients,
	/// rotation vector, translation vector, and the skew. In the old interface different components of
	/// the jacobian are returned via different output parameters.
	///
	/// The function computes projections of 3D points to the image plane given intrinsic and extrinsic
	/// camera parameters. Optionally, the function computes Jacobians - matrices of partial derivatives of
	/// image points coordinates (as functions of all the input parameters) with respect to the particular
	/// parameters, intrinsic and/or extrinsic.
	///
	/// ## C++ default parameters
	/// * alpha: 0
	/// * jacobian: noArray()
	#[inline]
	pub fn project_points_1(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, affine: core::Affine3d, k: &impl ToInputArray, d: &impl ToInputArray, alpha: f64, jacobian: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(object_points);
		output_array_arg!(image_points);
		input_array_arg!(k);
		input_array_arg!(d);
		output_array_arg!(jacobian);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), &affine, k.as_raw__InputArray(), d.as_raw__InputArray(), alpha, jacobian.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Projects points using fisheye model
	///
	/// ## Parameters
	/// * objectPoints: Array of object points, 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is
	/// the number of points in the view.
	/// * imagePoints: Output array of image points, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel, or
	/// vector\<Point2f\>.
	/// * affine: 
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * alpha: The skew coefficient.
	/// * jacobian: Optional output 2Nx15 jacobian matrix of derivatives of image points with respect
	/// to components of the focal lengths, coordinates of the principal point, distortion coefficients,
	/// rotation vector, translation vector, and the skew. In the old interface different components of
	/// the jacobian are returned via different output parameters.
	///
	/// The function computes projections of 3D points to the image plane given intrinsic and extrinsic
	/// camera parameters. Optionally, the function computes Jacobians - matrices of partial derivatives of
	/// image points coordinates (as functions of all the input parameters) with respect to the particular
	/// parameters, intrinsic and/or extrinsic.
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [project_points_2] function uses the following default values for its arguments:
	/// * alpha: 0
	/// * jacobian: noArray()
	#[inline]
	pub fn project_points_2_def(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, k: &impl ToInputArray, d: &impl ToInputArray) -> Result<()> {
		input_array_arg!(object_points);
		output_array_arg!(image_points);
		input_array_arg!(rvec);
		input_array_arg!(tvec);
		input_array_arg!(k);
		input_array_arg!(d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Projects points using fisheye model
	///
	/// ## Parameters
	/// * objectPoints: Array of object points, 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is
	/// the number of points in the view.
	/// * imagePoints: Output array of image points, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel, or
	/// vector\<Point2f\>.
	/// * affine: 
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
	/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * alpha: The skew coefficient.
	/// * jacobian: Optional output 2Nx15 jacobian matrix of derivatives of image points with respect
	/// to components of the focal lengths, coordinates of the principal point, distortion coefficients,
	/// rotation vector, translation vector, and the skew. In the old interface different components of
	/// the jacobian are returned via different output parameters.
	///
	/// The function computes projections of 3D points to the image plane given intrinsic and extrinsic
	/// camera parameters. Optionally, the function computes Jacobians - matrices of partial derivatives of
	/// image points coordinates (as functions of all the input parameters) with respect to the particular
	/// parameters, intrinsic and/or extrinsic.
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * alpha: 0
	/// * jacobian: noArray()
	#[inline]
	pub fn project_points_2(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, k: &impl ToInputArray, d: &impl ToInputArray, alpha: f64, jacobian: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(object_points);
		output_array_arg!(image_points);
		input_array_arg!(rvec);
		input_array_arg!(tvec);
		input_array_arg!(k);
		input_array_arg!(d);
		output_array_arg!(jacobian);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), alpha, jacobian.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an object pose from 3D-2D point correspondences using the RANSAC scheme for fisheye camera moodel.
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
	/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can be also passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients (4x1/1x4).
	/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system.
	/// * tvec: Output translation vector.
	/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
	/// the provided rvec and tvec values as initial approximations of the rotation and translation
	/// vectors, respectively, and further optimizes them.
	/// * iterationsCount: Number of iterations.
	/// * reprojectionError: Inlier threshold value used by the RANSAC procedure. The parameter value
	/// is the maximum allowed distance between the observed and computed point projections to consider it
	/// an inlier.
	/// * confidence: The probability that the algorithm produces a useful result.
	/// * inliers: Output vector that contains indices of inliers in objectPoints and imagePoints .
	/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
	/// * criteria: Termination criteria for internal undistortPoints call.
	/// The function interally undistorts points with [undistortPoints] and call [cv::solvePnP],
	/// thus the input are very similar. More information about Perspective-n-Points is described in [calib3d_solvePnP]
	/// for more information.
	///
	/// ## Note
	/// This alternative version of [solve_pnp_ransac_2] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	/// * iterations_count: 100
	/// * reprojection_error: 8.0
	/// * confidence: 0.99
	/// * inliers: noArray()
	/// * flags: SOLVEPNP_ITERATIVE
	/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,10,1e-8)
	#[inline]
	pub fn solve_pnp_ransac_2_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvec);
		output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an object pose from 3D-2D point correspondences using the RANSAC scheme for fisheye camera moodel.
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
	/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can be also passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients (4x1/1x4).
	/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system.
	/// * tvec: Output translation vector.
	/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
	/// the provided rvec and tvec values as initial approximations of the rotation and translation
	/// vectors, respectively, and further optimizes them.
	/// * iterationsCount: Number of iterations.
	/// * reprojectionError: Inlier threshold value used by the RANSAC procedure. The parameter value
	/// is the maximum allowed distance between the observed and computed point projections to consider it
	/// an inlier.
	/// * confidence: The probability that the algorithm produces a useful result.
	/// * inliers: Output vector that contains indices of inliers in objectPoints and imagePoints .
	/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
	/// * criteria: Termination criteria for internal undistortPoints call.
	/// The function interally undistorts points with [undistortPoints] and call [cv::solvePnP],
	/// thus the input are very similar. More information about Perspective-n-Points is described in [calib3d_solvePnP]
	/// for more information.
	///
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * iterations_count: 100
	/// * reprojection_error: 8.0
	/// * confidence: 0.99
	/// * inliers: noArray()
	/// * flags: SOLVEPNP_ITERATIVE
	/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,10,1e-8)
	#[inline]
	pub fn solve_pnp_ransac_2(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, confidence: f64, inliers: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvec);
		output_array_arg!(tvec);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_float_double_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, iterations_count, reprojection_error, confidence, inliers.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an object pose from 3D-2D point correspondences for fisheye camera model.
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
	/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can also be passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can also be passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients (4x1/1x4).
	/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system.
	/// * tvec: Output translation vector.
	/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
	/// the provided rvec and tvec values as initial approximations of the rotation and translation
	/// vectors, respectively, and further optimizes them.
	/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
	/// This function returns the rotation and the translation vectors that transform a 3D point expressed in the object
	/// coordinate frame to the camera coordinate frame, using different methods:
	/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): need 4 input points to return a unique solution.
	/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar.
	/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
	/// Number of input points must be 4. Object points must be defined in the following order:
	/// - point 0: [-squareLength / 2,  squareLength / 2, 0]
	/// - point 1: [ squareLength / 2,  squareLength / 2, 0]
	/// - point 2: [ squareLength / 2, -squareLength / 2, 0]
	/// - point 3: [-squareLength / 2, -squareLength / 2, 0]
	/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
	/// * criteria: Termination criteria for internal undistortPoints call.
	/// The function internally undistorts points with [undistortPoints] and call [cv::solvePnP],
	/// thus the input are very similar. Check there and Perspective-n-Points is described in [calib3d_solvePnP]
	/// for more information.
	///
	/// ## Note
	/// This alternative version of [solve_pnp_1] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,10,1e-8)
	#[inline]
	pub fn solve_pnp_1_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvec);
		output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an object pose from 3D-2D point correspondences for fisheye camera model.
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
	/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can also be passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can also be passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients (4x1/1x4).
	/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system.
	/// * tvec: Output translation vector.
	/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
	/// the provided rvec and tvec values as initial approximations of the rotation and translation
	/// vectors, respectively, and further optimizes them.
	/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
	/// This function returns the rotation and the translation vectors that transform a 3D point expressed in the object
	/// coordinate frame to the camera coordinate frame, using different methods:
	/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): need 4 input points to return a unique solution.
	/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar.
	/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
	/// Number of input points must be 4. Object points must be defined in the following order:
	/// - point 0: [-squareLength / 2,  squareLength / 2, 0]
	/// - point 1: [ squareLength / 2,  squareLength / 2, 0]
	/// - point 2: [ squareLength / 2, -squareLength / 2, 0]
	/// - point 3: [-squareLength / 2, -squareLength / 2, 0]
	/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
	/// * criteria: Termination criteria for internal undistortPoints call.
	/// The function internally undistorts points with [undistortPoints] and call [cv::solvePnP],
	/// thus the input are very similar. Check there and Perspective-n-Points is described in [calib3d_solvePnP]
	/// for more information.
	///
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,10,1e-8)
	#[inline]
	pub fn solve_pnp_1(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, use_extrinsic_guess: bool, flags: i32, criteria: core::TermCriteria) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvec);
		output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Undistorts 2D points using fisheye camera model
	///
	/// This function performs undistortion for fisheye camera models, which use a different distortion model
	/// compared to the standard pinhole camera model used by #undistortPoints. The fisheye model is suitable
	/// for wide-angle cameras.
	///
	/// The function transforms points from the distorted fisheye image to undistorted coordinates, optionally
	/// applying a rectification transformation (R) and projecting to a new image plane (P).
	///
	///
	/// Note: **Coordinate Systems:**
	/// - **Input (`distorted`)**: Points are expected in **pixel coordinates** of the distorted fisheye image,
	///    i.e., coordinates measured in pixels from the top-left corner of the image.
	/// - **Output (`undistorted`)**: The coordinate system depends on parameter `P`:
	///    - If `P` is provided (not empty): Output points are in **pixel coordinates** of the rectified/undistorted
	///    image plane, using the camera matrix `P`.
	///    - If `P` is empty or identity: Output points are in **normalized camera coordinates** (normalized image coordinates),
	///    which are dimensionless coordinates in the camera's focal plane, independent of intrinsic parameters.
	///
	///
	/// Note: **Fisheye vs. Standard Model:**
	/// Use this function (#cv::fisheye::undistortPoints) for fisheye cameras (wide-angle lenses).
	/// For standard pinhole cameras, use [undistort_points] instead. The fisheye model uses a different distortion
	/// parameterization (4 coefficients) compared to the standard model (4-14 coefficients).
	///
	/// ## Parameters
	/// * distorted: Array of distorted point coordinates in **pixel coordinates** of the fisheye image,
	/// 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is the number of points in the view.
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BK%7D) of the fisheye camera.
	/// * D: Input vector of fisheye distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye) (must contain exactly 4 coefficients).
	/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
	/// 1-channel or 1x1 3-channel. If empty, the identity transformation is used.
	/// * P: New camera intrinsic matrix (3x3) or new projection matrix (3x4). If empty or identity,
	/// output will be in normalized camera coordinates.
	/// * criteria: Termination criteria for the iterative undistortion algorithm.
	/// * undistorted: Output array of undistorted image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
	/// The coordinate system depends on parameter P (see above).
	///
	/// ## Note
	/// This alternative version of [undistort_points_1] function uses the following default values for its arguments:
	/// * r: noArray()
	/// * p: noArray()
	/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,10,1e-8)
	#[inline]
	pub fn undistort_points_1_def(distorted: &impl ToInputArray, undistorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray) -> Result<()> {
		input_array_arg!(distorted);
		output_array_arg!(undistorted);
		input_array_arg!(k);
		input_array_arg!(d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Undistorts 2D points using fisheye camera model
	///
	/// This function performs undistortion for fisheye camera models, which use a different distortion model
	/// compared to the standard pinhole camera model used by #undistortPoints. The fisheye model is suitable
	/// for wide-angle cameras.
	///
	/// The function transforms points from the distorted fisheye image to undistorted coordinates, optionally
	/// applying a rectification transformation (R) and projecting to a new image plane (P).
	///
	///
	/// Note: **Coordinate Systems:**
	/// - **Input (`distorted`)**: Points are expected in **pixel coordinates** of the distorted fisheye image,
	///    i.e., coordinates measured in pixels from the top-left corner of the image.
	/// - **Output (`undistorted`)**: The coordinate system depends on parameter `P`:
	///    - If `P` is provided (not empty): Output points are in **pixel coordinates** of the rectified/undistorted
	///    image plane, using the camera matrix `P`.
	///    - If `P` is empty or identity: Output points are in **normalized camera coordinates** (normalized image coordinates),
	///    which are dimensionless coordinates in the camera's focal plane, independent of intrinsic parameters.
	///
	///
	/// Note: **Fisheye vs. Standard Model:**
	/// Use this function (#cv::fisheye::undistortPoints) for fisheye cameras (wide-angle lenses).
	/// For standard pinhole cameras, use [undistort_points] instead. The fisheye model uses a different distortion
	/// parameterization (4 coefficients) compared to the standard model (4-14 coefficients).
	///
	/// ## Parameters
	/// * distorted: Array of distorted point coordinates in **pixel coordinates** of the fisheye image,
	/// 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is the number of points in the view.
	/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BK%7D) of the fisheye camera.
	/// * D: Input vector of fisheye distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye) (must contain exactly 4 coefficients).
	/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
	/// 1-channel or 1x1 3-channel. If empty, the identity transformation is used.
	/// * P: New camera intrinsic matrix (3x3) or new projection matrix (3x4). If empty or identity,
	/// output will be in normalized camera coordinates.
	/// * criteria: Termination criteria for the iterative undistortion algorithm.
	/// * undistorted: Output array of undistorted image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
	/// The coordinate system depends on parameter P (see above).
	///
	/// ## C++ default parameters
	/// * r: noArray()
	/// * p: noArray()
	/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,10,1e-8)
	#[inline]
	pub fn undistort_points_1(distorted: &impl ToInputArray, undistorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray, r: &impl ToInputArray, p: &impl ToInputArray, criteria: core::TermCriteria) -> Result<()> {
		input_array_arg!(distorted);
		output_array_arg!(undistorted);
		input_array_arg!(k);
		input_array_arg!(d);
		input_array_arg!(r);
		input_array_arg!(p);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), r.as_raw__InputArray(), p.as_raw__InputArray(), &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Fits an ellipse around a set of 2D points.
	///
	/// The function calculates the ellipse that fits a set of 2D points.
	/// It returns the rotated rectangle in which the ellipse is inscribed.
	/// The Approximate Mean Square (AMS) proposed by [Taubin1991](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Taubin1991) is used.
	///
	/// For an ellipse, this basis set is ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cchi%3D%20%5Cleft%28x%5E2%2C%20x%20y%2C%20y%5E2%2C%20x%2C%20y%2C%201%5Cright%29%20),
	/// which is a set of six free coefficients ![inline formula](https://latex.codecogs.com/png.latex?%20A%5ET%3D%5Cleft%5C%7BA%5F%7B%5Ctext%7Bxx%7D%7D%2CA%5F%7B%5Ctext%7Bxy%7D%7D%2CA%5F%7B%5Ctext%7Byy%7D%7D%2CA%5Fx%2CA%5Fy%2CA%5F0%5Cright%5C%7D%20).
	/// However, to specify an ellipse, all that is needed is five numbers; the major and minor axes lengths ![inline formula](https://latex.codecogs.com/png.latex?%20%28a%2Cb%29%20),
	/// the position ![inline formula](https://latex.codecogs.com/png.latex?%20%28x%5F0%2Cy%5F0%29%20), and the orientation ![inline formula](https://latex.codecogs.com/png.latex?%20%5Ctheta%20). This is because the basis set includes lines,
	/// quadratics, parabolic and hyperbolic functions as well as elliptical functions as possible fits.
	/// If the fit is found to be a parabolic or hyperbolic function then the standard [fit_ellipse] method is used.
	/// The AMS method restricts the fit to parabolic, hyperbolic and elliptical curves
	/// by imposing the condition that ![inline formula](https://latex.codecogs.com/png.latex?%20A%5ET%20%28%20D%5Fx%5ET%20D%5Fx%20%20%2B%20%20%20D%5Fy%5ET%20D%5Fy%29%20A%20%3D%201%20) where
	/// the matrices ![inline formula](https://latex.codecogs.com/png.latex?%20Dx%20) and ![inline formula](https://latex.codecogs.com/png.latex?%20Dy%20) are the partial derivatives of the design matrix ![inline formula](https://latex.codecogs.com/png.latex?%20D%20) with
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
	///
	///
	/// Note: Input point types are [Point2i] or [Point2f] and at least 5 points are required.
	///
	/// Note: [getClosestEllipsePoints] function can be used to compute the ellipse fitting error.
	#[inline]
	pub fn fit_ellipse_ams(points: &impl ToInputArray) -> Result<core::RotatedRect> {
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fitEllipseAMS_const__InputArrayR(points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Fits an ellipse around a set of 2D points.
	///
	/// The function calculates the ellipse that fits a set of 2D points.
	/// It returns the rotated rectangle in which the ellipse is inscribed.
	/// The Direct least square (Direct) method by [oy1998NumericallySD](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_oy1998NumericallySD) is used.
	///
	/// For an ellipse, this basis set is ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cchi%3D%20%5Cleft%28x%5E2%2C%20x%20y%2C%20y%5E2%2C%20x%2C%20y%2C%201%5Cright%29%20),
	/// which is a set of six free coefficients ![inline formula](https://latex.codecogs.com/png.latex?%20A%5ET%3D%5Cleft%5C%7BA%5F%7B%5Ctext%7Bxx%7D%7D%2CA%5F%7B%5Ctext%7Bxy%7D%7D%2CA%5F%7B%5Ctext%7Byy%7D%7D%2CA%5Fx%2CA%5Fy%2CA%5F0%5Cright%5C%7D%20).
	/// However, to specify an ellipse, all that is needed is five numbers; the major and minor axes lengths ![inline formula](https://latex.codecogs.com/png.latex?%20%28a%2Cb%29%20),
	/// the position ![inline formula](https://latex.codecogs.com/png.latex?%20%28x%5F0%2Cy%5F0%29%20), and the orientation ![inline formula](https://latex.codecogs.com/png.latex?%20%5Ctheta%20). This is because the basis set includes lines,
	/// quadratics, parabolic and hyperbolic functions as well as elliptical functions as possible fits.
	/// The Direct method confines the fit to ellipses by ensuring that ![inline formula](https://latex.codecogs.com/png.latex?%204%20A%5F%7Bxx%7D%20A%5F%7Byy%7D%2D%20A%5F%7Bxy%7D%5E2%20%3E%200%20).
	/// The condition imposed is that ![inline formula](https://latex.codecogs.com/png.latex?%204%20A%5F%7Bxx%7D%20A%5F%7Byy%7D%2D%20A%5F%7Bxy%7D%5E2%3D1%20) which satisfies the inequality
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
	/// The system produces only one positive eigenvalue ![inline formula](https://latex.codecogs.com/png.latex?%20%5Clambda) which is chosen as the solution
	/// with its eigenvector ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7Bu%7D). These are used to find the coefficients
	///
	/// \f{equation*}{
	/// A = \sqrt{\frac{1}{\mathbf{u}^T C \mathbf{u}}}  \mathbf{u}
	/// \f}
	/// The scaling factor guarantees that  ![inline formula](https://latex.codecogs.com/png.latex?A%5ET%20C%20A%20%3D1).
	///
	/// ## Parameters
	/// * points: Input 2D point set, stored in std::vector\<\> or Mat
	///
	///
	/// Note: Input point types are [Point2i] or [Point2f] and at least 5 points are required.
	///
	/// Note: [getClosestEllipsePoints] function can be used to compute the ellipse fitting error.
	#[inline]
	pub fn fit_ellipse_direct(points: &impl ToInputArray) -> Result<core::RotatedRect> {
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fitEllipseDirect_const__InputArrayR(points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Fits an ellipse around a set of 2D points.
	///
	/// The function calculates the ellipse that fits (in a least-squares sense) a set of 2D points best of
	/// all. It returns the rotated rectangle in which the ellipse is inscribed. The first algorithm described by [Fitzgibbon95](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Fitzgibbon95)
	/// is used. Developer should keep in mind that it is possible that the returned
	/// ellipse/rotatedRect data contains negative indices, due to the data points being close to the
	/// border of the containing Mat element.
	///
	/// ## Parameters
	/// * points: Input 2D point set, stored in std::vector\<\> or Mat
	///
	///
	/// Note: Input point types are [Point2i] or [Point2f] and at least 5 points are required.
	///
	/// Note: [getClosestEllipsePoints] function can be used to compute the ellipse fitting error.
	#[inline]
	pub fn fit_ellipse(points: &impl ToInputArray) -> Result<core::RotatedRect> {
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fitEllipse_const__InputArrayR(points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Fits a line to a 2D or 3D point set.
	///
	/// The function fitLine fits a line to a 2D or 3D point set by minimizing ![inline formula](https://latex.codecogs.com/png.latex?%5Csum%5Fi%20%5Crho%28r%5Fi%29) where
	/// ![inline formula](https://latex.codecogs.com/png.latex?r%5Fi) is a distance between the ![inline formula](https://latex.codecogs.com/png.latex?i%5E%7Bth%7D) point, the line and ![inline formula](https://latex.codecogs.com/png.latex?%5Crho%28r%29) is a distance function, one
	/// of the following:
	/// *  DIST_L2
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Crho%20%28r%29%20%3D%20r%5E2%2F2%20%20%5Cquad%20%5Ctext%7B%28the%20simplest%20and%20the%20fastest%20least%2Dsquares%20method%29%7D)
	/// - DIST_L1
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Crho%20%28r%29%20%3D%20r)
	/// - DIST_L12
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Crho%20%28r%29%20%3D%202%20%20%5Ccdot%20%28%20%5Csqrt%7B1%20%2B%20%5Cfrac%7Br%5E2%7D%7B2%7D%7D%20%2D%201%29)
	/// - DIST_FAIR
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Crho%20%5Cleft%20%28r%20%5Cright%20%29%20%3D%20C%5E2%20%20%5Ccdot%20%5Cleft%20%28%20%20%5Cfrac%7Br%7D%7BC%7D%20%2D%20%20%5Clog%7B%5Cleft%281%20%2B%20%5Cfrac%7Br%7D%7BC%7D%5Cright%29%7D%20%5Cright%20%29%20%20%5Cquad%20%5Ctext%7Bwhere%7D%20%5Cquad%20C%3D1%2E3998)
	/// - DIST_WELSCH
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Crho%20%5Cleft%20%28r%20%5Cright%20%29%20%3D%20%20%5Cfrac%7BC%5E2%7D%7B2%7D%20%5Ccdot%20%5Cleft%20%28%201%20%2D%20%20%5Cexp%7B%5Cleft%28%2D%5Cleft%28%5Cfrac%7Br%7D%7BC%7D%5Cright%29%5E2%5Cright%29%7D%20%5Cright%20%29%20%20%5Cquad%20%5Ctext%7Bwhere%7D%20%5Cquad%20C%3D2%2E9846)
	/// - DIST_HUBER
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Crho%20%28r%29%20%3D%20%20%5Cleft%5C%7B%20%5Cbegin%7Barray%7D%7Bl%20l%7D%20r%5E2%2F2%20%26%20%5Cmbox%7Bif%20%5C%28r%20%3C%20C%5C%29%7D%5C%5C%20C%20%5Ccdot%20%28r%2DC%2F2%29%20%26%20%5Cmbox%7Botherwise%7D%5C%5C%20%5Cend%7Barray%7D%20%5Cright%2E%20%5Cquad%20%5Ctext%7Bwhere%7D%20%5Cquad%20C%3D1%2E345)
	///
	/// The algorithm is based on the M-estimator ( <https://en.wikipedia.org/wiki/M-estimator> ) technique
	/// that iteratively fits the line using the weighted least-squares algorithm. After each iteration the
	/// weights ![inline formula](https://latex.codecogs.com/png.latex?w%5Fi) are adjusted to be inversely proportional to ![inline formula](https://latex.codecogs.com/png.latex?%5Crho%28r%5Fi%29) .
	///
	/// ## Parameters
	/// * points: Input vector of 2D or 3D points, stored in std::vector\<\> or Mat.
	/// * line: Output line parameters. In case of 2D fitting, it should be a vector of 4 elements
	/// (like Vec4f) - (vx, vy, x0, y0), where (vx, vy) is a normalized vector collinear to the line and
	/// (x0, y0) is a point on the line. In case of 3D fitting, it should be a vector of 6 elements (like
	/// Vec6f) - (vx, vy, vz, x0, y0, z0), where (vx, vy, vz) is a normalized vector collinear to the line
	/// and (x0, y0, z0) is a point on the line.
	/// * distType: Distance used by the M-estimator, see [distance_types]
	/// * param: Numerical parameter ( C ) for some types of distances. If it is 0, an optimal value
	/// is chosen.
	/// * reps: Sufficient accuracy for the radius (distance between the coordinate origin and the line).
	/// * aeps: Sufficient accuracy for the angle. 0.01 would be a good default value for reps and aeps.
	#[inline]
	pub fn fit_line(points: &impl ToInputArray, line: &mut impl ToOutputArray, dist_type: i32, param: f64, reps: f64, aeps: f64) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(line);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fitLine_const__InputArrayR_const__OutputArrayR_int_double_double_double(points.as_raw__InputArray(), line.as_raw__OutputArray(), dist_type, param, reps, aeps, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates an affine transform from three pairs of the corresponding points.
	///
	/// The function calculates the ![inline formula](https://latex.codecogs.com/png.latex?2%20%5Ctimes%203) matrix of an affine transform so that:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20x%27%5Fi%20%5C%5C%20y%27%5Fi%20%5Cend%7Bbmatrix%7D%20%3D%20%5Ctexttt%7Bmap%5Fmatrix%7D%20%5Ccdot%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%20%5C%5C%20y%5Fi%20%5C%5C%201%20%5Cend%7Bbmatrix%7D)
	///
	/// where
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?dst%28i%29%3D%28x%27%5Fi%2Cy%27%5Fi%29%2C%20src%28i%29%3D%28x%5Fi%2C%20y%5Fi%29%2C%20i%3D0%2C1%2C2)
	///
	/// ## Parameters
	/// * src: Coordinates of triangle vertices in the source image.
	/// * dst: Coordinates of the corresponding triangle vertices in the destination image.
	/// ## See also
	/// warpAffine, transform
	#[inline]
	pub fn get_affine_transform_slice(src: &[core::Point2f], dst: &[core::Point2f]) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getAffineTransform_const_Point2fX_const_Point2fX(src.as_ptr(), dst.as_ptr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	#[inline]
	pub fn get_affine_transform(src: &impl ToInputArray, dst: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(src);
		input_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getAffineTransform_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Compute for each 2d point the nearest 2d point located on a given ellipse.
	///
	/// The function computes the nearest 2d location on a given ellipse for a vector of 2d points and is based on [Chatfield2017](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Chatfield2017) code.
	/// This function can be used to compute for instance the ellipse fitting error.
	///
	/// ## Parameters
	/// * ellipse_params: Ellipse parameters
	/// * points: Input 2d points
	/// * closest_pts: For each 2d point, their corresponding closest 2d point located on a given ellipse
	///
	///
	/// Note: Input point types are [Point2i] or [Point2f]
	/// ## See also
	/// fitEllipse, fitEllipseAMS, fitEllipseDirect
	#[inline]
	pub fn get_closest_ellipse_points(ellipse_params: core::RotatedRect, points: &impl ToInputArray, closest_pts: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(closest_pts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getClosestEllipsePoints_const_RotatedRectR_const__InputArrayR_const__OutputArrayR(&ellipse_params, points.as_raw__InputArray(), closest_pts.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the default new camera matrix.
	///
	/// The function returns the camera matrix that is either an exact copy of the input cameraMatrix (when
	/// centerPrinicipalPoint=false ), or the modified one (when centerPrincipalPoint=true).
	///
	/// In the latter case, the new camera matrix will be:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%26%200%20%26%26%20%28%20%5Ctexttt%7BimgSize%2Ewidth%7D%20%2D1%29%2A0%2E5%20%20%5C%5C%200%20%26%26%20f%5Fy%20%26%26%20%28%20%5Ctexttt%7BimgSize%2Eheight%7D%20%2D1%29%2A0%2E5%20%20%5C%5C%200%20%26%26%200%20%26%26%201%20%5Cend%7Bbmatrix%7D%20%2C)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) are ![inline formula](https://latex.codecogs.com/png.latex?%280%2C0%29) and ![inline formula](https://latex.codecogs.com/png.latex?%281%2C1%29) elements of cameraMatrix, respectively.
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
	/// ## Note
	/// This alternative version of [get_default_new_camera_matrix] function uses the following default values for its arguments:
	/// * imgsize: Size()
	/// * center_principal_point: false
	#[inline]
	pub fn get_default_new_camera_matrix_def(camera_matrix: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(camera_matrix);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getDefaultNewCameraMatrix_const__InputArrayR(camera_matrix.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns the default new camera matrix.
	///
	/// The function returns the camera matrix that is either an exact copy of the input cameraMatrix (when
	/// centerPrinicipalPoint=false ), or the modified one (when centerPrincipalPoint=true).
	///
	/// In the latter case, the new camera matrix will be:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%26%200%20%26%26%20%28%20%5Ctexttt%7BimgSize%2Ewidth%7D%20%2D1%29%2A0%2E5%20%20%5C%5C%200%20%26%26%20f%5Fy%20%26%26%20%28%20%5Ctexttt%7BimgSize%2Eheight%7D%20%2D1%29%2A0%2E5%20%20%5C%5C%200%20%26%26%200%20%26%26%201%20%5Cend%7Bbmatrix%7D%20%2C)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) are ![inline formula](https://latex.codecogs.com/png.latex?%280%2C0%29) and ![inline formula](https://latex.codecogs.com/png.latex?%281%2C1%29) elements of cameraMatrix, respectively.
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
	/// ## C++ default parameters
	/// * imgsize: Size()
	/// * center_principal_point: false
	#[inline]
	pub fn get_default_new_camera_matrix(camera_matrix: &impl ToInputArray, imgsize: core::Size, center_principal_point: bool) -> Result<core::Mat> {
		input_array_arg!(camera_matrix);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getDefaultNewCameraMatrix_const__InputArrayR_Size_bool(camera_matrix.as_raw__InputArray(), &imgsize, center_principal_point, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns the new camera intrinsic matrix based on the free scaling parameter.
	///
	/// ## Parameters
	/// * cameraMatrix: Input camera intrinsic matrix.
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * imageSize: Original image size.
	/// * alpha: Free scaling parameter between 0 (when all the pixels in the undistorted image are
	/// valid) and 1 (when all the source image pixels are retained in the undistorted image). See
	/// [stereo_rectify] for details.
	/// * newImgSize: Image size after rectification. By default, it is set to imageSize .
	/// * validPixROI: Optional output rectangle that outlines all-good-pixels region in the
	/// undistorted image. See roi1, roi2 description in [stereo_rectify] .
	/// * centerPrincipalPoint: Optional flag that indicates whether in the new camera intrinsic matrix the
	/// principal point should be at the image center or not. By default, the principal point is chosen to
	/// best fit a subset of the source image (determined by alpha) to the corrected image.
	/// ## Returns
	/// new_camera_matrix Output new camera intrinsic matrix.
	///
	/// The function computes and returns the optimal new camera intrinsic matrix based on the free scaling parameter.
	/// By varying this parameter, you may retrieve only sensible pixels alpha=0 , keep all the original
	/// image pixels if there is valuable information in the corners alpha=1 , or get something in between.
	/// When alpha\>0 , the undistorted result is likely to have some black pixels corresponding to
	/// "virtual" pixels outside of the captured distorted image. The original camera intrinsic matrix, distortion
	/// coefficients, the computed new camera intrinsic matrix, and newImageSize should be passed to
	/// [init_undistort_rectify_map] to produce the maps for [remap] .
	///
	/// ## Note
	/// This alternative version of [get_optimal_new_camera_matrix] function uses the following default values for its arguments:
	/// * new_img_size: Size()
	/// * valid_pix_roi: 0
	/// * center_principal_point: false
	#[inline]
	pub fn get_optimal_new_camera_matrix_def(camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_size: core::Size, alpha: f64) -> Result<core::Mat> {
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getOptimalNewCameraMatrix_const__InputArrayR_const__InputArrayR_Size_double(camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), &image_size, alpha, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns the new camera intrinsic matrix based on the free scaling parameter.
	///
	/// ## Parameters
	/// * cameraMatrix: Input camera intrinsic matrix.
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * imageSize: Original image size.
	/// * alpha: Free scaling parameter between 0 (when all the pixels in the undistorted image are
	/// valid) and 1 (when all the source image pixels are retained in the undistorted image). See
	/// [stereo_rectify] for details.
	/// * newImgSize: Image size after rectification. By default, it is set to imageSize .
	/// * validPixROI: Optional output rectangle that outlines all-good-pixels region in the
	/// undistorted image. See roi1, roi2 description in [stereo_rectify] .
	/// * centerPrincipalPoint: Optional flag that indicates whether in the new camera intrinsic matrix the
	/// principal point should be at the image center or not. By default, the principal point is chosen to
	/// best fit a subset of the source image (determined by alpha) to the corrected image.
	/// ## Returns
	/// new_camera_matrix Output new camera intrinsic matrix.
	///
	/// The function computes and returns the optimal new camera intrinsic matrix based on the free scaling parameter.
	/// By varying this parameter, you may retrieve only sensible pixels alpha=0 , keep all the original
	/// image pixels if there is valuable information in the corners alpha=1 , or get something in between.
	/// When alpha\>0 , the undistorted result is likely to have some black pixels corresponding to
	/// "virtual" pixels outside of the captured distorted image. The original camera intrinsic matrix, distortion
	/// coefficients, the computed new camera intrinsic matrix, and newImageSize should be passed to
	/// [init_undistort_rectify_map] to produce the maps for [remap] .
	///
	/// ## C++ default parameters
	/// * new_img_size: Size()
	/// * valid_pix_roi: 0
	/// * center_principal_point: false
	#[inline]
	pub fn get_optimal_new_camera_matrix(camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_size: core::Size, alpha: f64, new_img_size: core::Size, valid_pix_roi: &mut core::Rect, center_principal_point: bool) -> Result<core::Mat> {
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getOptimalNewCameraMatrix_const__InputArrayR_const__InputArrayR_Size_double_Size_RectX_bool(camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), &image_size, alpha, &new_img_size, valid_pix_roi, center_principal_point, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Calculates a perspective transform from four pairs of the corresponding points.
	///
	/// The function calculates the ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%203) matrix of a perspective transform so that:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20t%5Fi%20x%27%5Fi%20%5C%5C%20t%5Fi%20y%27%5Fi%20%5C%5C%20t%5Fi%20%5Cend%7Bbmatrix%7D%20%3D%20%5Ctexttt%7Bmap%5Fmatrix%7D%20%5Ccdot%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%20%5C%5C%20y%5Fi%20%5C%5C%201%20%5Cend%7Bbmatrix%7D)
	///
	/// where
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?dst%28i%29%3D%28x%27%5Fi%2Cy%27%5Fi%29%2C%20src%28i%29%3D%28x%5Fi%2C%20y%5Fi%29%2C%20i%3D0%2C1%2C2%2C3)
	///
	/// ## Parameters
	/// * src: Coordinates of quadrangle vertices in the source image.
	/// * dst: Coordinates of the corresponding quadrangle vertices in the destination image.
	/// * solveMethod: method passed to cv::solve (#DecompTypes)
	/// ## See also
	/// findHomography, warpPerspective, perspectiveTransform
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [get_perspective_transform_slice] function uses the following default values for its arguments:
	/// * solve_method: DECOMP_LU
	#[inline]
	pub fn get_perspective_transform_slice_def(src: [core::Point2f; 4], dst: [core::Point2f; 4]) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getPerspectiveTransform_const_Point2fXX_const_Point2fXX(&src, &dst, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Calculates a perspective transform from four pairs of the corresponding points.
	///
	/// The function calculates the ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%203) matrix of a perspective transform so that:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20t%5Fi%20x%27%5Fi%20%5C%5C%20t%5Fi%20y%27%5Fi%20%5C%5C%20t%5Fi%20%5Cend%7Bbmatrix%7D%20%3D%20%5Ctexttt%7Bmap%5Fmatrix%7D%20%5Ccdot%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%20%5C%5C%20y%5Fi%20%5C%5C%201%20%5Cend%7Bbmatrix%7D)
	///
	/// where
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?dst%28i%29%3D%28x%27%5Fi%2Cy%27%5Fi%29%2C%20src%28i%29%3D%28x%5Fi%2C%20y%5Fi%29%2C%20i%3D0%2C1%2C2%2C3)
	///
	/// ## Parameters
	/// * src: Coordinates of quadrangle vertices in the source image.
	/// * dst: Coordinates of the corresponding quadrangle vertices in the destination image.
	/// * solveMethod: method passed to cv::solve (#DecompTypes)
	/// ## See also
	/// findHomography, warpPerspective, perspectiveTransform
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * solve_method: DECOMP_LU
	#[inline]
	pub fn get_perspective_transform_slice(src: [core::Point2f; 4], dst: [core::Point2f; 4], solve_method: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getPerspectiveTransform_const_Point2fXX_const_Point2fXX_int(&src, &dst, solve_method, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Calculates a perspective transform from four pairs of the corresponding points.
	///
	/// The function calculates the ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%203) matrix of a perspective transform so that:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20t%5Fi%20x%27%5Fi%20%5C%5C%20t%5Fi%20y%27%5Fi%20%5C%5C%20t%5Fi%20%5Cend%7Bbmatrix%7D%20%3D%20%5Ctexttt%7Bmap%5Fmatrix%7D%20%5Ccdot%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%20%5C%5C%20y%5Fi%20%5C%5C%201%20%5Cend%7Bbmatrix%7D)
	///
	/// where
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?dst%28i%29%3D%28x%27%5Fi%2Cy%27%5Fi%29%2C%20src%28i%29%3D%28x%5Fi%2C%20y%5Fi%29%2C%20i%3D0%2C1%2C2%2C3)
	///
	/// ## Parameters
	/// * src: Coordinates of quadrangle vertices in the source image.
	/// * dst: Coordinates of the corresponding quadrangle vertices in the destination image.
	/// * solveMethod: method passed to cv::solve (#DecompTypes)
	/// ## See also
	/// findHomography, warpPerspective, perspectiveTransform
	///
	/// ## Note
	/// This alternative version of [get_perspective_transform] function uses the following default values for its arguments:
	/// * solve_method: DECOMP_LU
	#[inline]
	pub fn get_perspective_transform_def(src: &impl ToInputArray, dst: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(src);
		input_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getPerspectiveTransform_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Calculates a perspective transform from four pairs of the corresponding points.
	///
	/// The function calculates the ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%203) matrix of a perspective transform so that:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20t%5Fi%20x%27%5Fi%20%5C%5C%20t%5Fi%20y%27%5Fi%20%5C%5C%20t%5Fi%20%5Cend%7Bbmatrix%7D%20%3D%20%5Ctexttt%7Bmap%5Fmatrix%7D%20%5Ccdot%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%20%5C%5C%20y%5Fi%20%5C%5C%201%20%5Cend%7Bbmatrix%7D)
	///
	/// where
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?dst%28i%29%3D%28x%27%5Fi%2Cy%27%5Fi%29%2C%20src%28i%29%3D%28x%5Fi%2C%20y%5Fi%29%2C%20i%3D0%2C1%2C2%2C3)
	///
	/// ## Parameters
	/// * src: Coordinates of quadrangle vertices in the source image.
	/// * dst: Coordinates of the corresponding quadrangle vertices in the destination image.
	/// * solveMethod: method passed to cv::solve (#DecompTypes)
	/// ## See also
	/// findHomography, warpPerspective, perspectiveTransform
	///
	/// ## C++ default parameters
	/// * solve_method: DECOMP_LU
	#[inline]
	pub fn get_perspective_transform(src: &impl ToInputArray, dst: &impl ToInputArray, solve_method: i32) -> Result<core::Mat> {
		input_array_arg!(src);
		input_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getPerspectiveTransform_const__InputArrayR_const__InputArrayR_int(src.as_raw__InputArray(), dst.as_raw__InputArray(), solve_method, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Calculates an affine matrix of 2D rotation.
	///
	/// The function calculates the following matrix:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%5Calpha%20%26%20%20%5Cbeta%20%26%20%281%2D%20%5Calpha%20%29%20%20%5Ccdot%20%5Ctexttt%7Bcenter%2Ex%7D%20%2D%20%20%5Cbeta%20%5Ccdot%20%5Ctexttt%7Bcenter%2Ey%7D%20%5C%5C%20%2D%20%5Cbeta%20%26%20%20%5Calpha%20%26%20%20%5Cbeta%20%5Ccdot%20%5Ctexttt%7Bcenter%2Ex%7D%20%2B%20%281%2D%20%5Calpha%20%29%20%20%5Ccdot%20%5Ctexttt%7Bcenter%2Ey%7D%20%5Cend%7Bbmatrix%7D)
	///
	/// where
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Calpha%20%3D%20%20%5Ctexttt%7Bscale%7D%20%5Ccdot%20%5Ccos%20%5Ctexttt%7Bangle%7D%20%2C%20%5C%5C%20%5Cbeta%20%3D%20%20%5Ctexttt%7Bscale%7D%20%5Ccdot%20%5Csin%20%5Ctexttt%7Bangle%7D%20%5Cend%7Barray%7D)
	///
	/// The transformation maps the rotation center to itself. If this is not the target, adjust the shift.
	///
	/// ## Parameters
	/// * center: Center of the rotation in the source image.
	/// * angle: Rotation angle in degrees. Positive values mean counter-clockwise rotation (the
	/// coordinate origin is assumed to be the top-left corner).
	/// * scale: Isotropic scale factor.
	/// ## See also
	/// getAffineTransform, warpAffine, transform
	#[inline]
	pub fn get_rotation_matrix_2d(center: core::Point2f, angle: f64, scale: f64) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getRotationMatrix2D_Point2f_double_double(&center, angle, scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## See also
	/// getRotationMatrix2D
	#[inline]
	pub fn get_rotation_matrix_2d_matx(center: core::Point2f, angle: f64, scale: f64) -> Result<core::Matx23d> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getRotationMatrix2D__Point2f_double_double(&center, angle, scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the inscribed and bounding rectangles for the "undisorted" image plane.
	///
	/// The functions emulates undistortion of the image plane using the specified camera matrix,
	/// distortion coefficients, the optional 3D rotation and the "new" camera matrix. In the case of
	/// noticeable radial (or maybe pinclusion) distortion the rectangular image plane is distorted and
	/// turns into some convex or concave shape. The function computes approximate inscribed (inner) and
	/// bounding (outer) rectangles after such undistortion. The rectangles can be used to adjust
	/// the newCameraMatrix so that the result image, for example, fits all the data from the original image
	/// (at the expense of possibly big "black" areas) or, for another example, gets rid of black areas at the expense
	/// some lost data near the original image edge. The function [get_optimal_new_camera_matrix] uses this function
	/// to compute the optimal new camera matrix.
	///
	/// ## Parameters
	/// * cameraMatrix: the original camera matrix.
	/// * distCoeffs: distortion coefficients.
	/// * R: the optional 3D rotation, applied before projection (see stereoRectify etc.)
	/// * newCameraMatrix: the new camera matrix after undistortion. Usually it matches the original cameraMatrix.
	/// * imgSize: the size of the image plane.
	/// * inner: the output maximal inscribed rectangle of the undistorted image plane.
	/// * outer: the output minimal bounding rectangle of the undistorted image plane.
	#[inline]
	pub fn get_undistort_rectangles(camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, r: &impl ToInputArray, new_camera_matrix: &impl ToInputArray, img_size: core::Size, inner: &mut core::Rect_<f64>, outer: &mut core::Rect_<f64>) -> Result<()> {
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_array_arg!(r);
		input_array_arg!(new_camera_matrix);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getUndistortRectangles_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_Rect_LdoubleGR_Rect_LdoubleGR(camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), r.as_raw__InputArray(), new_camera_matrix.as_raw__InputArray(), &img_size, inner, outer, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds intersection of two convex polygons
	///
	/// ## Parameters
	/// * p1: First polygon
	/// * p2: Second polygon
	/// * p12: Output polygon describing the intersecting area
	/// * handleNested: When true, an intersection is found if one of the polygons is fully enclosed in the other.
	/// When false, no intersection is found. If the polygons share a side or the vertex of one polygon lies on an edge
	/// of the other, they are not considered nested and an intersection will be found regardless of the value of handleNested.
	///
	/// ## Returns
	/// Area of intersecting polygon. May be negative, if algorithm has not converged, e.g. non-convex input.
	///
	///
	/// Note: intersectConvexConvex doesn't confirm that both polygons are convex and will return invalid results if they aren't.
	///
	/// ## Note
	/// This alternative version of [intersect_convex_convex] function uses the following default values for its arguments:
	/// * handle_nested: true
	#[inline]
	pub fn intersect_convex_convex_def(p1: &impl ToInputArray, p2: &impl ToInputArray, p12: &mut impl ToOutputArray) -> Result<f32> {
		input_array_arg!(p1);
		input_array_arg!(p2);
		output_array_arg!(p12);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_intersectConvexConvex_const__InputArrayR_const__InputArrayR_const__OutputArrayR(p1.as_raw__InputArray(), p2.as_raw__InputArray(), p12.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds intersection of two convex polygons
	///
	/// ## Parameters
	/// * p1: First polygon
	/// * p2: Second polygon
	/// * p12: Output polygon describing the intersecting area
	/// * handleNested: When true, an intersection is found if one of the polygons is fully enclosed in the other.
	/// When false, no intersection is found. If the polygons share a side or the vertex of one polygon lies on an edge
	/// of the other, they are not considered nested and an intersection will be found regardless of the value of handleNested.
	///
	/// ## Returns
	/// Area of intersecting polygon. May be negative, if algorithm has not converged, e.g. non-convex input.
	///
	///
	/// Note: intersectConvexConvex doesn't confirm that both polygons are convex and will return invalid results if they aren't.
	///
	/// ## C++ default parameters
	/// * handle_nested: true
	#[inline]
	pub fn intersect_convex_convex(p1: &impl ToInputArray, p2: &impl ToInputArray, p12: &mut impl ToOutputArray, handle_nested: bool) -> Result<f32> {
		input_array_arg!(p1);
		input_array_arg!(p2);
		output_array_arg!(p12);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_intersectConvexConvex_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(p1.as_raw__InputArray(), p2.as_raw__InputArray(), p12.as_raw__OutputArray(), handle_nested, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Inverts an affine transformation.
	///
	/// The function computes an inverse affine transformation represented by ![inline formula](https://latex.codecogs.com/png.latex?2%20%5Ctimes%203) matrix M:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20a%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20b%5F1%20%20%5C%5C%20a%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20b%5F2%20%5Cend%7Bbmatrix%7D)
	///
	/// The result is also a ![inline formula](https://latex.codecogs.com/png.latex?2%20%5Ctimes%203) matrix of the same type as M.
	///
	/// ## Parameters
	/// * M: Original affine transformation.
	/// * iM: Output reverse affine transformation.
	#[inline]
	pub fn invert_affine_transform(m: &impl ToInputArray, i_m: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(m);
		output_array_arg!(i_m);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_invertAffineTransform_const__InputArrayR_const__OutputArrayR(m.as_raw__InputArray(), i_m.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Tests a contour convexity.
	///
	/// The function tests whether the input contour is convex or not. The contour must be simple, that is,
	/// without self-intersections. Otherwise, the function output is undefined.
	///
	/// ## Parameters
	/// * contour: Input vector of 2D points, stored in std::vector\<\> or Mat
	#[inline]
	pub fn is_contour_convex(contour: &impl ToInputArray) -> Result<bool> {
		input_array_arg!(contour);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_isContourConvex_const__InputArrayR(contour.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes partial derivatives of the matrix product for each multiplied matrix.
	///
	/// ## Parameters
	/// * A: First multiplied matrix.
	/// * B: Second multiplied matrix.
	/// * dABdA: First output derivative matrix d(A\*B)/dA of size
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BA%2Erows%2AB%2Ecols%7D%20%5Ctimes%20%7BA%2Erows%2AA%2Ecols%7D) .
	/// * dABdB: Second output derivative matrix d(A\*B)/dB of size
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BA%2Erows%2AB%2Ecols%7D%20%5Ctimes%20%7BB%2Erows%2AB%2Ecols%7D) .
	///
	/// The function computes partial derivatives of the elements of the matrix product ![inline formula](https://latex.codecogs.com/png.latex?A%2AB) with regard to
	/// the elements of each of the two input matrices. The function is used to compute the Jacobian
	/// matrices in [stereo_calibrate] but can also be used in any other similar optimization function.
	#[inline]
	pub fn mat_mul_deriv(a: &impl ToInputArray, b: &impl ToInputArray, d_a_bd_a: &mut impl ToOutputArray, d_a_bd_b: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(a);
		input_array_arg!(b);
		output_array_arg!(d_a_bd_a);
		output_array_arg!(d_a_bd_b);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_matMulDeriv_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(a.as_raw__InputArray(), b.as_raw__InputArray(), d_a_bd_a.as_raw__OutputArray(), d_a_bd_b.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Compares two shapes.
	///
	/// The function compares two shapes. All three implemented methods use the Hu invariants (see #HuMoments)
	///
	/// ## Parameters
	/// * contour1: First contour or grayscale image.
	/// * contour2: Second contour or grayscale image.
	/// * method: Comparison method, see [shape_match_modes]
	/// * parameter: Method-specific parameter (not supported now).
	#[inline]
	pub fn match_shapes(contour1: &impl ToInputArray, contour2: &impl ToInputArray, method: i32, parameter: f64) -> Result<f64> {
		input_array_arg!(contour1);
		input_array_arg!(contour2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_matchShapes_const__InputArrayR_const__InputArrayR_int_double(contour1.as_raw__InputArray(), contour2.as_raw__InputArray(), method, parameter, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds a rotated rectangle of the minimum area enclosing the input 2D point set.
	///
	/// The function calculates and returns the minimum-area bounding rectangle (possibly rotated) for a
	/// specified point set. The angle of rotation represents the angle between the line connecting the starting
	/// and ending points (based on the clockwise order with greatest index for the corner with greatest ![inline formula](https://latex.codecogs.com/png.latex?y))
	/// and the horizontal axis. This angle always falls between ![inline formula](https://latex.codecogs.com/png.latex?%5B%2D90%2C%200%29) because, if the object
	/// rotates more than a rect angle, the next edge is used to measure the angle. The starting and ending points change
	/// as the object rotates.Developer should keep in mind that the returned RotatedRect can contain negative
	/// indices when data is close to the containing Mat element boundary.
	///
	/// ## Parameters
	/// * points: Input vector of 2D points, stored in std::vector\<\> or Mat
	#[inline]
	pub fn min_area_rect(points: &impl ToInputArray) -> Result<core::RotatedRect> {
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_minAreaRect_const__InputArrayR(points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds a circle of the minimum area enclosing a 2D point set.
	///
	/// The function finds the minimal enclosing circle of a 2D point set using an iterative algorithm.
	///
	/// ## Parameters
	/// * points: Input vector of 2D points, stored in std::vector\<\> or Mat
	/// * center: Output center of the circle.
	/// * radius: Output radius of the circle.
	#[inline]
	pub fn min_enclosing_circle(points: &impl ToInputArray, center: &mut core::Point2f, radius: &mut f32) -> Result<()> {
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_minEnclosingCircle_const__InputArrayR_Point2fR_floatR(points.as_raw__InputArray(), center, radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds a convex polygon of minimum area enclosing a 2D point set and returns its area.
	///
	/// This function takes a given set of 2D points and finds the enclosing polygon with k vertices and minimal
	/// area. It takes the set of points and the parameter k as input and returns the area of the minimal
	/// enclosing polygon.
	///
	/// The Implementation is based on a paper by Aggarwal, Chang and Yap [Aggarwal1985](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Aggarwal1985). They
	/// provide a ![inline formula](https://latex.codecogs.com/png.latex?%5Ctheta%28n%C2%B2log%28n%29log%28k%29%29) algorithm for finding the minimal convex polygon with k
	/// vertices enclosing a 2D convex polygon with n vertices (k < n). Since the [min_enclosing_convex_polygon]
	/// function takes a 2D point set as input, an additional preprocessing step of computing the convex hull
	/// of the 2D point set is required. The complexity of the [convex_hull] function is ![inline formula](https://latex.codecogs.com/png.latex?O%28n%20log%28n%29%29) which
	/// is lower than ![inline formula](https://latex.codecogs.com/png.latex?%5Ctheta%28n%C2%B2log%28n%29log%28k%29%29). Thus the overall complexity of the function is
	/// ![inline formula](https://latex.codecogs.com/png.latex?O%28n%C2%B2log%28n%29log%28k%29%29).
	///
	/// ## Parameters
	/// * points: Input vector of 2D points, stored in std::vector\<\> or Mat
	/// * polygon: Output vector of 2D points defining the vertices of the enclosing polygon
	/// * k: Number of vertices of the output polygon
	#[inline]
	pub fn min_enclosing_convex_polygon(points: &impl ToInputArray, polygon: &mut impl ToOutputArray, k: i32) -> Result<f64> {
		input_array_arg!(points);
		output_array_arg!(polygon);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_minEnclosingConvexPolygon_const__InputArrayR_const__OutputArrayR_int(points.as_raw__InputArray(), polygon.as_raw__OutputArray(), k, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds a triangle of minimum area enclosing a 2D point set and returns its area.
	///
	///  The function finds a triangle of minimum area enclosing the given set of 2D points and returns its
	///  area. The output for a given 2D point set is shown in the image below. 2D points are depicted in
	/// red* and the enclosing triangle in *yellow*.
	///
	///  ![Sample output of the minimum enclosing triangle function](https://docs.opencv.org/5.0.0/minenclosingtriangle.png)
	///
	///  The implementation of the algorithm is based on O'Rourke's [ORourke86](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_ORourke86) and Klee and Laskowski's
	///  [KleeLaskowski85](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_KleeLaskowski85) papers. O'Rourke provides a ![inline formula](https://latex.codecogs.com/png.latex?%5Ctheta%28n%29) algorithm for finding the minimal
	///  enclosing triangle of a 2D convex polygon with n vertices. Since the [min_enclosing_triangle] function
	///  takes a 2D point set as input an additional preprocessing step of computing the convex hull of the
	///  2D point set is required. The complexity of the [convex_hull] function is ![inline formula](https://latex.codecogs.com/png.latex?O%28n%20log%28n%29%29) which is higher
	///  than ![inline formula](https://latex.codecogs.com/png.latex?%5Ctheta%28n%29). Thus the overall complexity of the function is ![inline formula](https://latex.codecogs.com/png.latex?O%28n%20log%28n%29%29).
	///
	/// ## Parameters
	/// * points: Input vector of 2D points with depth CV_32S or CV_32F, stored in std::vector\<\> or Mat
	/// * triangle: Output vector of three 2D points defining the vertices of the triangle. The depth
	///  of the OutputArray must be CV_32F.
	#[inline]
	pub fn min_enclosing_triangle(points: &impl ToInputArray, triangle: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(points);
		output_array_arg!(triangle);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_minEnclosingTriangle_const__InputArrayR_const__OutputArrayR(points.as_raw__InputArray(), triangle.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates all of the moments up to the third order of a polygon or rasterized shape.
	///
	/// The function computes moments, up to the 3rd order, of a vector shape or a rasterized shape. The
	/// results are returned in the structure cv::Moments.
	///
	/// ## Parameters
	/// * array: Single channel raster image (CV_8U, CV_16U, CV_16S, CV_32F, CV_64F) or an array (
	/// ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Ctimes%20N) or ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Ctimes%201) ) of 2D points (Point or Point2f).
	/// * binaryImage: If it is true, all non-zero image pixels are treated as 1's. The parameter is
	/// used for images only.
	/// ## Returns
	/// moments.
	///
	///
	/// Note: Only applicable to contour moments calculations from Python bindings: Note that the numpy
	/// type for the input array should be either np.int32 or np.float32.
	///
	///
	/// Note: For contour-based moments, the zeroth-order moment \c m00 represents
	/// the contour area.
	///
	/// If the input contour is degenerate (for example, a single point or all points
	/// are collinear), the area is zero and therefore \c m00 == 0.
	///
	/// In this case, the centroid coordinates (\c m10/m00, \c m01/m00) are undefined
	/// and must be handled explicitly by the caller.
	///
	/// A common workaround is to compute the center using cv::boundingRect() or by
	/// averaging the input points.
	/// ## See also
	/// contourArea, arcLength
	///
	/// ## Note
	/// This alternative version of [moments] function uses the following default values for its arguments:
	/// * binary_image: false
	#[inline]
	pub fn moments_def(array: &impl ToInputArray) -> Result<core::Moments> {
		input_array_arg!(array);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_moments_const__InputArrayR(array.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates all of the moments up to the third order of a polygon or rasterized shape.
	///
	/// The function computes moments, up to the 3rd order, of a vector shape or a rasterized shape. The
	/// results are returned in the structure cv::Moments.
	///
	/// ## Parameters
	/// * array: Single channel raster image (CV_8U, CV_16U, CV_16S, CV_32F, CV_64F) or an array (
	/// ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Ctimes%20N) or ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Ctimes%201) ) of 2D points (Point or Point2f).
	/// * binaryImage: If it is true, all non-zero image pixels are treated as 1's. The parameter is
	/// used for images only.
	/// ## Returns
	/// moments.
	///
	///
	/// Note: Only applicable to contour moments calculations from Python bindings: Note that the numpy
	/// type for the input array should be either np.int32 or np.float32.
	///
	///
	/// Note: For contour-based moments, the zeroth-order moment \c m00 represents
	/// the contour area.
	///
	/// If the input contour is degenerate (for example, a single point or all points
	/// are collinear), the area is zero and therefore \c m00 == 0.
	///
	/// In this case, the centroid coordinates (\c m10/m00, \c m01/m00) are undefined
	/// and must be handled explicitly by the caller.
	///
	/// A common workaround is to compute the center using cv::boundingRect() or by
	/// averaging the input points.
	/// ## See also
	/// contourArea, arcLength
	///
	/// ## C++ default parameters
	/// * binary_image: false
	#[inline]
	pub fn moments(array: &impl ToInputArray, binary_image: bool) -> Result<core::Moments> {
		input_array_arg!(array);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_moments_const__InputArrayR_bool(array.as_raw__InputArray(), binary_image, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimate the normal and curvature of each point in point cloud from NN results.
	///
	/// Normal estimation by PCA:
	/// + Input: Nearest neighbor points of a specific point: ![inline formula](https://latex.codecogs.com/png.latex?%20pt%5C%5Fset%20)
	/// + Step:
	///    1. Calculate the ![inline formula](https://latex.codecogs.com/png.latex?%20mean%28%5Cbar%7Bx%7D%2C%5Cbar%7By%7D%2C%5Cbar%7Bz%7D%29%20) of ![inline formula](https://latex.codecogs.com/png.latex?%20pt%5C%5Fset%20);
	///    2. A 3x3 covariance matrix ![inline formula](https://latex.codecogs.com/png.latex?%20cov%20) is obtained by ![inline formula](https://latex.codecogs.com/png.latex?%20mean%5ET%20%5Ccdot%20mean%20);
	///    3. Calculate the eigenvalues(![inline formula](https://latex.codecogs.com/png.latex?%20%CE%BB%5F2%20%5Cge%20%CE%BB%5F1%20%5Cge%20%CE%BB%5F0%20)) and corresponding
	///        eigenvectors(![inline formula](https://latex.codecogs.com/png.latex?%20v%5F2%2C%20v%5F1%2C%20v%5F0%20)) of ![inline formula](https://latex.codecogs.com/png.latex?%20cov%20);
	///    4. ![inline formula](https://latex.codecogs.com/png.latex?%20v0%20) is the normal of the specific point,
	///        ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7B%CE%BB%5F0%7D%7B%CE%BB%5F0%20%2B%20%CE%BB%5F1%20%2B%20%CE%BB%5F2%7D%20) is the curvature of the specific point;
	/// + Output: Normal and curvature of the specific point.
	///
	/// ## Parameters
	/// * normals:[out] Normal of each point, support vector<Point3f> and Mat of size Nx3.
	/// * curvatures:[out] Curvature of each point, support vector<float> and Mat.
	/// * input_pts: Original point cloud, support vector<Point3f> and Mat of size Nx3/3xN.
	/// * nn_idx: Index information of nearest neighbors of all points. The first nearest neighbor of
	///               each point is itself. Support vector<vector<int>>, vector<Mat> and Mat of size NxK.
	///               If the information in a row is [0, 2, 1, -5, -1, 4, 7 ... negative number], it will
	///               use only non-negative indexes until it meets a negative number or bound of this row
	///               i.e. [0, 2, 1].
	/// * max_neighbor_num: The maximum number of neighbors want to use including itself. Setting to
	///               a non-positive number or default will use the information from nn_idx.
	///
	/// ## Note
	/// This alternative version of [normal_estimate] function uses the following default values for its arguments:
	/// * max_neighbor_num: 0
	#[inline]
	pub fn normal_estimate_def(normals: &mut impl ToOutputArray, curvatures: &mut impl ToOutputArray, input_pts: &impl ToInputArray, nn_idx: &impl ToInputArray) -> Result<()> {
		output_array_arg!(normals);
		output_array_arg!(curvatures);
		input_array_arg!(input_pts);
		input_array_arg!(nn_idx);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_normalEstimate_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(normals.as_raw__OutputArray(), curvatures.as_raw__OutputArray(), input_pts.as_raw__InputArray(), nn_idx.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimate the normal and curvature of each point in point cloud from NN results.
	///
	/// Normal estimation by PCA:
	/// + Input: Nearest neighbor points of a specific point: ![inline formula](https://latex.codecogs.com/png.latex?%20pt%5C%5Fset%20)
	/// + Step:
	///    1. Calculate the ![inline formula](https://latex.codecogs.com/png.latex?%20mean%28%5Cbar%7Bx%7D%2C%5Cbar%7By%7D%2C%5Cbar%7Bz%7D%29%20) of ![inline formula](https://latex.codecogs.com/png.latex?%20pt%5C%5Fset%20);
	///    2. A 3x3 covariance matrix ![inline formula](https://latex.codecogs.com/png.latex?%20cov%20) is obtained by ![inline formula](https://latex.codecogs.com/png.latex?%20mean%5ET%20%5Ccdot%20mean%20);
	///    3. Calculate the eigenvalues(![inline formula](https://latex.codecogs.com/png.latex?%20%CE%BB%5F2%20%5Cge%20%CE%BB%5F1%20%5Cge%20%CE%BB%5F0%20)) and corresponding
	///        eigenvectors(![inline formula](https://latex.codecogs.com/png.latex?%20v%5F2%2C%20v%5F1%2C%20v%5F0%20)) of ![inline formula](https://latex.codecogs.com/png.latex?%20cov%20);
	///    4. ![inline formula](https://latex.codecogs.com/png.latex?%20v0%20) is the normal of the specific point,
	///        ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7B%CE%BB%5F0%7D%7B%CE%BB%5F0%20%2B%20%CE%BB%5F1%20%2B%20%CE%BB%5F2%7D%20) is the curvature of the specific point;
	/// + Output: Normal and curvature of the specific point.
	///
	/// ## Parameters
	/// * normals:[out] Normal of each point, support vector<Point3f> and Mat of size Nx3.
	/// * curvatures:[out] Curvature of each point, support vector<float> and Mat.
	/// * input_pts: Original point cloud, support vector<Point3f> and Mat of size Nx3/3xN.
	/// * nn_idx: Index information of nearest neighbors of all points. The first nearest neighbor of
	///               each point is itself. Support vector<vector<int>>, vector<Mat> and Mat of size NxK.
	///               If the information in a row is [0, 2, 1, -5, -1, 4, 7 ... negative number], it will
	///               use only non-negative indexes until it meets a negative number or bound of this row
	///               i.e. [0, 2, 1].
	/// * max_neighbor_num: The maximum number of neighbors want to use including itself. Setting to
	///               a non-positive number or default will use the information from nn_idx.
	///
	/// ## C++ default parameters
	/// * max_neighbor_num: 0
	#[inline]
	pub fn normal_estimate(normals: &mut impl ToOutputArray, curvatures: &mut impl ToOutputArray, input_pts: &impl ToInputArray, nn_idx: &impl ToInputArray, max_neighbor_num: i32) -> Result<()> {
		output_array_arg!(normals);
		output_array_arg!(curvatures);
		input_array_arg!(input_pts);
		input_array_arg!(nn_idx);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_normalEstimate_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(normals.as_raw__OutputArray(), curvatures.as_raw__OutputArray(), input_pts.as_raw__InputArray(), nn_idx.as_raw__InputArray(), max_neighbor_num, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ![sample output](https://docs.opencv.org/5.0.0/pointpolygon.png)
	///
	/// ## Parameters
	/// * contour: Input contour.
	/// * pt: Point tested against the contour.
	/// * measureDist: If true, the function estimates the signed distance from the point to the
	/// nearest contour edge. Otherwise, the function only checks if the point is inside a contour or not.
	#[inline]
	pub fn point_polygon_test(contour: &impl ToInputArray, pt: core::Point2f, measure_dist: bool) -> Result<f64> {
		input_array_arg!(contour);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_pointPolygonTest_const__InputArrayR_Point2f_bool(contour.as_raw__InputArray(), &pt, measure_dist, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Projects 3D points to an image plane.
	///
	/// The function computes the 2D projections of 3D points to the image plane, given intrinsic and
	/// extrinsic camera parameters. Optionally, the function computes Jacobians -matrices of partial
	/// derivatives of image points coordinates (as functions of all the input parameters) with respect to
	/// the particular parameters, intrinsic and/or extrinsic. The Jacobians are used during the global
	/// optimization in [calibrateCamera], [solvePnP], and [stereoCalibrate]. The function itself
	/// can also be used to compute a re-projection error, given the current intrinsic and extrinsic
	/// parameters.
	///
	///
	/// Note: **Coordinate Systems:**
	/// - **Input (`objectPoints`)**: 3D points in the **world coordinate frame**.
	/// - **Output (`imagePoints`)**: 2D projections in **pixel coordinates** of the image plane, with distortion applied.
	///   The coordinates ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) are measured in pixels from the top-left corner of the image.
	///
	/// The transformation chain is: World coordinates → Camera coordinates (via rvec/tvec) → Normalized camera coordinates
	/// → Distortion applied → Pixel coordinates (via cameraMatrix).
	///
	/// ## Parameters
	/// * objectPoints: Array of object points expressed wrt. the world coordinate frame. A 3xN/Nx3
	/// 1-channel or 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is the number of points in the view.
	/// * rvec: The rotation vector ([Rodrigues]) that, together with tvec, performs a change of
	/// basis from world to camera coordinate system, see [calibrateCamera] for details.
	/// * tvec: The translation vector, see parameter description above.
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs) . If the vector is empty, the zero distortion coefficients are assumed.
	/// * imagePoints: Output array of image points in **pixel coordinates**, 1xN/Nx1 2-channel, or
	/// vector\<Point2f\> .
	/// * jacobian: Optional output 2Nx(10+\<numDistCoeffs\>) jacobian matrix of derivatives of image
	/// points with respect to components of the rotation vector, translation vector, focal lengths,
	/// coordinates of the principal point and the distortion coefficients. In the old interface different
	/// components of the jacobian are returned via different output parameters.
	/// * aspectRatio: Optional "fixed aspect ratio" parameter. If the parameter is not 0, the
	/// function assumes that the aspect ratio (![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%20%2F%20f%5Fy)) is fixed and correspondingly adjusts the
	/// jacobian matrix.
	///
	///
	/// Note: By setting rvec = tvec = ![inline formula](https://latex.codecogs.com/png.latex?%5B0%2C%200%2C%200%5D), or by setting cameraMatrix to a 3x3 identity matrix,
	/// or by passing zero distortion coefficients, one can get various useful partial cases of the
	/// function. This means, one can compute the distorted coordinates for a sparse set of points or apply
	/// a perspective transformation (and also compute the derivatives) in the ideal zero-distortion setup.
	///
	/// ## Note
	/// This alternative version of [project_points] function uses the following default values for its arguments:
	/// * jacobian: noArray()
	/// * aspect_ratio: 0
	#[inline]
	pub fn project_points_def(object_points: &impl ToInputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_points: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(object_points);
		input_array_arg!(rvec);
		input_array_arg!(tvec);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(image_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), image_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Projects 3D points to an image plane.
	///
	/// The function computes the 2D projections of 3D points to the image plane, given intrinsic and
	/// extrinsic camera parameters. Optionally, the function computes Jacobians -matrices of partial
	/// derivatives of image points coordinates (as functions of all the input parameters) with respect to
	/// the particular parameters, intrinsic and/or extrinsic. The Jacobians are used during the global
	/// optimization in [calibrateCamera], [solvePnP], and [stereoCalibrate]. The function itself
	/// can also be used to compute a re-projection error, given the current intrinsic and extrinsic
	/// parameters.
	///
	///
	/// Note: **Coordinate Systems:**
	/// - **Input (`objectPoints`)**: 3D points in the **world coordinate frame**.
	/// - **Output (`imagePoints`)**: 2D projections in **pixel coordinates** of the image plane, with distortion applied.
	///   The coordinates ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) are measured in pixels from the top-left corner of the image.
	///
	/// The transformation chain is: World coordinates → Camera coordinates (via rvec/tvec) → Normalized camera coordinates
	/// → Distortion applied → Pixel coordinates (via cameraMatrix).
	///
	/// ## Parameters
	/// * objectPoints: Array of object points expressed wrt. the world coordinate frame. A 3xN/Nx3
	/// 1-channel or 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is the number of points in the view.
	/// * rvec: The rotation vector ([Rodrigues]) that, together with tvec, performs a change of
	/// basis from world to camera coordinate system, see [calibrateCamera] for details.
	/// * tvec: The translation vector, see parameter description above.
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs) . If the vector is empty, the zero distortion coefficients are assumed.
	/// * imagePoints: Output array of image points in **pixel coordinates**, 1xN/Nx1 2-channel, or
	/// vector\<Point2f\> .
	/// * jacobian: Optional output 2Nx(10+\<numDistCoeffs\>) jacobian matrix of derivatives of image
	/// points with respect to components of the rotation vector, translation vector, focal lengths,
	/// coordinates of the principal point and the distortion coefficients. In the old interface different
	/// components of the jacobian are returned via different output parameters.
	/// * aspectRatio: Optional "fixed aspect ratio" parameter. If the parameter is not 0, the
	/// function assumes that the aspect ratio (![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%20%2F%20f%5Fy)) is fixed and correspondingly adjusts the
	/// jacobian matrix.
	///
	///
	/// Note: By setting rvec = tvec = ![inline formula](https://latex.codecogs.com/png.latex?%5B0%2C%200%2C%200%5D), or by setting cameraMatrix to a 3x3 identity matrix,
	/// or by passing zero distortion coefficients, one can get various useful partial cases of the
	/// function. This means, one can compute the distorted coordinates for a sparse set of points or apply
	/// a perspective transformation (and also compute the derivatives) in the ideal zero-distortion setup.
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [project_points_sep_j] function uses the following default values for its arguments:
	/// * dpdf: noArray()
	/// * dpdc: noArray()
	/// * dpdk: noArray()
	/// * dpdo: noArray()
	/// * aspect_ratio: 0.
	#[inline]
	pub fn project_points_sep_j_def(object_points: &impl ToInputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_points: &mut impl ToOutputArray, dpdr: &mut impl ToOutputArray, dpdt: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(object_points);
		input_array_arg!(rvec);
		input_array_arg!(tvec);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(image_points);
		output_array_arg!(dpdr);
		output_array_arg!(dpdt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), image_points.as_raw__OutputArray(), dpdr.as_raw__OutputArray(), dpdt.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Projects 3D points to an image plane.
	///
	/// The function computes the 2D projections of 3D points to the image plane, given intrinsic and
	/// extrinsic camera parameters. Optionally, the function computes Jacobians -matrices of partial
	/// derivatives of image points coordinates (as functions of all the input parameters) with respect to
	/// the particular parameters, intrinsic and/or extrinsic. The Jacobians are used during the global
	/// optimization in [calibrateCamera], [solvePnP], and [stereoCalibrate]. The function itself
	/// can also be used to compute a re-projection error, given the current intrinsic and extrinsic
	/// parameters.
	///
	///
	/// Note: **Coordinate Systems:**
	/// - **Input (`objectPoints`)**: 3D points in the **world coordinate frame**.
	/// - **Output (`imagePoints`)**: 2D projections in **pixel coordinates** of the image plane, with distortion applied.
	///   The coordinates ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) are measured in pixels from the top-left corner of the image.
	///
	/// The transformation chain is: World coordinates → Camera coordinates (via rvec/tvec) → Normalized camera coordinates
	/// → Distortion applied → Pixel coordinates (via cameraMatrix).
	///
	/// ## Parameters
	/// * objectPoints: Array of object points expressed wrt. the world coordinate frame. A 3xN/Nx3
	/// 1-channel or 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is the number of points in the view.
	/// * rvec: The rotation vector ([Rodrigues]) that, together with tvec, performs a change of
	/// basis from world to camera coordinate system, see [calibrateCamera] for details.
	/// * tvec: The translation vector, see parameter description above.
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs) . If the vector is empty, the zero distortion coefficients are assumed.
	/// * imagePoints: Output array of image points in **pixel coordinates**, 1xN/Nx1 2-channel, or
	/// vector\<Point2f\> .
	/// * jacobian: Optional output 2Nx(10+\<numDistCoeffs\>) jacobian matrix of derivatives of image
	/// points with respect to components of the rotation vector, translation vector, focal lengths,
	/// coordinates of the principal point and the distortion coefficients. In the old interface different
	/// components of the jacobian are returned via different output parameters.
	/// * aspectRatio: Optional "fixed aspect ratio" parameter. If the parameter is not 0, the
	/// function assumes that the aspect ratio (![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%20%2F%20f%5Fy)) is fixed and correspondingly adjusts the
	/// jacobian matrix.
	///
	///
	/// Note: By setting rvec = tvec = ![inline formula](https://latex.codecogs.com/png.latex?%5B0%2C%200%2C%200%5D), or by setting cameraMatrix to a 3x3 identity matrix,
	/// or by passing zero distortion coefficients, one can get various useful partial cases of the
	/// function. This means, one can compute the distorted coordinates for a sparse set of points or apply
	/// a perspective transformation (and also compute the derivatives) in the ideal zero-distortion setup.
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * dpdf: noArray()
	/// * dpdc: noArray()
	/// * dpdk: noArray()
	/// * dpdo: noArray()
	/// * aspect_ratio: 0.
	#[inline]
	pub fn project_points_sep_j(object_points: &impl ToInputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_points: &mut impl ToOutputArray, dpdr: &mut impl ToOutputArray, dpdt: &mut impl ToOutputArray, dpdf: &mut impl ToOutputArray, dpdc: &mut impl ToOutputArray, dpdk: &mut impl ToOutputArray, dpdo: &mut impl ToOutputArray, aspect_ratio: f64) -> Result<()> {
		input_array_arg!(object_points);
		input_array_arg!(rvec);
		input_array_arg!(tvec);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(image_points);
		output_array_arg!(dpdr);
		output_array_arg!(dpdt);
		output_array_arg!(dpdf);
		output_array_arg!(dpdc);
		output_array_arg!(dpdk);
		output_array_arg!(dpdo);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_double(object_points.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), image_points.as_raw__OutputArray(), dpdr.as_raw__OutputArray(), dpdt.as_raw__OutputArray(), dpdf.as_raw__OutputArray(), dpdc.as_raw__OutputArray(), dpdk.as_raw__OutputArray(), dpdo.as_raw__OutputArray(), aspect_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Projects 3D points to an image plane.
	///
	/// The function computes the 2D projections of 3D points to the image plane, given intrinsic and
	/// extrinsic camera parameters. Optionally, the function computes Jacobians -matrices of partial
	/// derivatives of image points coordinates (as functions of all the input parameters) with respect to
	/// the particular parameters, intrinsic and/or extrinsic. The Jacobians are used during the global
	/// optimization in [calibrateCamera], [solvePnP], and [stereoCalibrate]. The function itself
	/// can also be used to compute a re-projection error, given the current intrinsic and extrinsic
	/// parameters.
	///
	///
	/// Note: **Coordinate Systems:**
	/// - **Input (`objectPoints`)**: 3D points in the **world coordinate frame**.
	/// - **Output (`imagePoints`)**: 2D projections in **pixel coordinates** of the image plane, with distortion applied.
	///   The coordinates ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) are measured in pixels from the top-left corner of the image.
	///
	/// The transformation chain is: World coordinates → Camera coordinates (via rvec/tvec) → Normalized camera coordinates
	/// → Distortion applied → Pixel coordinates (via cameraMatrix).
	///
	/// ## Parameters
	/// * objectPoints: Array of object points expressed wrt. the world coordinate frame. A 3xN/Nx3
	/// 1-channel or 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is the number of points in the view.
	/// * rvec: The rotation vector ([Rodrigues]) that, together with tvec, performs a change of
	/// basis from world to camera coordinate system, see [calibrateCamera] for details.
	/// * tvec: The translation vector, see parameter description above.
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs) . If the vector is empty, the zero distortion coefficients are assumed.
	/// * imagePoints: Output array of image points in **pixel coordinates**, 1xN/Nx1 2-channel, or
	/// vector\<Point2f\> .
	/// * jacobian: Optional output 2Nx(10+\<numDistCoeffs\>) jacobian matrix of derivatives of image
	/// points with respect to components of the rotation vector, translation vector, focal lengths,
	/// coordinates of the principal point and the distortion coefficients. In the old interface different
	/// components of the jacobian are returned via different output parameters.
	/// * aspectRatio: Optional "fixed aspect ratio" parameter. If the parameter is not 0, the
	/// function assumes that the aspect ratio (![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%20%2F%20f%5Fy)) is fixed and correspondingly adjusts the
	/// jacobian matrix.
	///
	///
	/// Note: By setting rvec = tvec = ![inline formula](https://latex.codecogs.com/png.latex?%5B0%2C%200%2C%200%5D), or by setting cameraMatrix to a 3x3 identity matrix,
	/// or by passing zero distortion coefficients, one can get various useful partial cases of the
	/// function. This means, one can compute the distorted coordinates for a sparse set of points or apply
	/// a perspective transformation (and also compute the derivatives) in the ideal zero-distortion setup.
	///
	/// ## C++ default parameters
	/// * jacobian: noArray()
	/// * aspect_ratio: 0
	#[inline]
	pub fn project_points(object_points: &impl ToInputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_points: &mut impl ToOutputArray, jacobian: &mut impl ToOutputArray, aspect_ratio: f64) -> Result<()> {
		input_array_arg!(object_points);
		input_array_arg!(rvec);
		input_array_arg!(tvec);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(image_points);
		output_array_arg!(jacobian);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(object_points.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), image_points.as_raw__OutputArray(), jacobian.as_raw__OutputArray(), aspect_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Point cloud sampling by randomly select points.
	///
	/// Use cv::randShuffle to shuffle the point index list,
	/// then take the points corresponding to the front part of the list.
	///
	/// ## Parameters
	/// * sampled_pts: Point cloud after sampling.
	///                    Support cv::Mat(sampled_pts_size, 3, CV_32F), std::vector<cv::Point3f>.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_pts_size: The desired point cloud size after sampling.
	/// * rng: Optional random number generator used for cv::randShuffle;
	///                      if it is nullptr, theRNG () is used instead.
	///
	/// ## Overloaded parameters
	///
	///
	/// * sampled_pts: Point cloud after sampling.
	///                    Support cv::Mat(size * sampled_scale, 3, CV_32F), std::vector<cv::Point3f>.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_scale: Range (0, 1), the percentage of the sampled point cloud to the original size,
	///                      that is, sampled size = original size * sampled_scale.
	/// * rng: Optional random number generator used for cv::randShuffle;
	///                      if it is nullptr, theRNG () is used instead.
	///
	/// ## Note
	/// This alternative version of [random_sampling_1] function uses the following default values for its arguments:
	/// * rng: nullptr
	#[inline]
	pub fn random_sampling_1_def(sampled_pts: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_scale: f32) -> Result<()> {
		output_array_arg!(sampled_pts);
		input_array_arg!(input_pts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randomSampling_const__OutputArrayR_const__InputArrayR_float(sampled_pts.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Point cloud sampling by randomly select points.
	///
	/// Use cv::randShuffle to shuffle the point index list,
	/// then take the points corresponding to the front part of the list.
	///
	/// ## Parameters
	/// * sampled_pts: Point cloud after sampling.
	///                    Support cv::Mat(sampled_pts_size, 3, CV_32F), std::vector<cv::Point3f>.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_pts_size: The desired point cloud size after sampling.
	/// * rng: Optional random number generator used for cv::randShuffle;
	///                      if it is nullptr, theRNG () is used instead.
	///
	/// ## Overloaded parameters
	///
	///
	/// * sampled_pts: Point cloud after sampling.
	///                    Support cv::Mat(size * sampled_scale, 3, CV_32F), std::vector<cv::Point3f>.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_scale: Range (0, 1), the percentage of the sampled point cloud to the original size,
	///                      that is, sampled size = original size * sampled_scale.
	/// * rng: Optional random number generator used for cv::randShuffle;
	///                      if it is nullptr, theRNG () is used instead.
	///
	/// ## C++ default parameters
	/// * rng: nullptr
	#[inline]
	pub fn random_sampling_1(sampled_pts: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_scale: f32, rng: &mut impl core::RNGTrait) -> Result<()> {
		output_array_arg!(sampled_pts);
		input_array_arg!(input_pts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randomSampling_const__OutputArrayR_const__InputArrayR_float_RNGX(sampled_pts.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_scale, rng.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Point cloud sampling by randomly select points.
	///
	/// Use cv::randShuffle to shuffle the point index list,
	/// then take the points corresponding to the front part of the list.
	///
	/// ## Parameters
	/// * sampled_pts: Point cloud after sampling.
	///                    Support cv::Mat(sampled_pts_size, 3, CV_32F), std::vector<cv::Point3f>.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_pts_size: The desired point cloud size after sampling.
	/// * rng: Optional random number generator used for cv::randShuffle;
	///                      if it is nullptr, theRNG () is used instead.
	///
	/// ## Note
	/// This alternative version of [random_sampling] function uses the following default values for its arguments:
	/// * rng: nullptr
	#[inline]
	pub fn random_sampling_def(sampled_pts: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_pts_size: i32) -> Result<()> {
		output_array_arg!(sampled_pts);
		input_array_arg!(input_pts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randomSampling_const__OutputArrayR_const__InputArrayR_int(sampled_pts.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_pts_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Point cloud sampling by randomly select points.
	///
	/// Use cv::randShuffle to shuffle the point index list,
	/// then take the points corresponding to the front part of the list.
	///
	/// ## Parameters
	/// * sampled_pts: Point cloud after sampling.
	///                    Support cv::Mat(sampled_pts_size, 3, CV_32F), std::vector<cv::Point3f>.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * sampled_pts_size: The desired point cloud size after sampling.
	/// * rng: Optional random number generator used for cv::randShuffle;
	///                      if it is nullptr, theRNG () is used instead.
	///
	/// ## C++ default parameters
	/// * rng: nullptr
	#[inline]
	pub fn random_sampling(sampled_pts: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_pts_size: i32, rng: &mut impl core::RNGTrait) -> Result<()> {
		output_array_arg!(sampled_pts);
		input_array_arg!(input_pts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randomSampling_const__OutputArrayR_const__InputArrayR_int_RNGX(sampled_pts.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_pts_size, rng.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Recovers the relative camera rotation and the translation from corresponding points in two images from two different cameras, using chirality check. Returns the number of
	/// inliers that pass the check.
	///
	/// ## Parameters
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * cameraMatrix1: Input/output camera matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraMatrix2: Input/output camera matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs2: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * E: The output essential matrix.
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// described below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * method: Method for computing an essential matrix.
	/// *   [RANSAC] for the RANSAC algorithm.
	/// *   [LMEDS] for the LMedS algorithm.
	/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
	/// confidence (probability) that the estimated matrix is correct.
	/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	///
	/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
	/// possible pose hypotheses by doing chirality check. The chirality check means that the
	/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
	///
	/// This function can be used to process the output E and mask from [findEssentialMat]. In this
	/// scenario, points1 and points2 are the same input for findEssentialMat.:
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    // Input: camera calibration of both cameras, for example using intrinsic chessboard calibration.
	///    Mat cameraMatrix1, distCoeffs1, cameraMatrix2, distCoeffs2;
	///
	///    // Output: Essential matrix, relative rotation and relative translation.
	///    Mat E, R, t, mask;
	///
	///    recoverPose(points1, points2, cameraMatrix1, distCoeffs1, cameraMatrix2, distCoeffs2, E, R, t, mask);
	/// ```
	///
	///
	/// ## Note
	/// This alternative version of [recover_pose] function uses the following default values for its arguments:
	/// * method: cv::RANSAC
	/// * prob: 0.999
	/// * threshold: 1.0
	/// * mask: noArray()
	#[inline]
	pub fn recover_pose_def(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, e: &mut impl ToOutputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<i32> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		input_array_arg!(camera_matrix1);
		input_array_arg!(dist_coeffs1);
		input_array_arg!(camera_matrix2);
		input_array_arg!(dist_coeffs2);
		output_array_arg!(e);
		output_array_arg!(r);
		output_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), e.as_raw__OutputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Recovers the relative camera rotation and the translation from corresponding points in two images from two different cameras, using chirality check. Returns the number of
	/// inliers that pass the check.
	///
	/// ## Parameters
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * cameraMatrix1: Input/output camera matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraMatrix2: Input/output camera matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs2: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * E: The output essential matrix.
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// described below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * method: Method for computing an essential matrix.
	/// *   [RANSAC] for the RANSAC algorithm.
	/// *   [LMEDS] for the LMedS algorithm.
	/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
	/// confidence (probability) that the estimated matrix is correct.
	/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
	/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
	/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
	/// point localization, image resolution, and the image noise.
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	///
	/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
	/// possible pose hypotheses by doing chirality check. The chirality check means that the
	/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
	///
	/// This function can be used to process the output E and mask from [findEssentialMat]. In this
	/// scenario, points1 and points2 are the same input for findEssentialMat.:
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    // Input: camera calibration of both cameras, for example using intrinsic chessboard calibration.
	///    Mat cameraMatrix1, distCoeffs1, cameraMatrix2, distCoeffs2;
	///
	///    // Output: Essential matrix, relative rotation and relative translation.
	///    Mat E, R, t, mask;
	///
	///    recoverPose(points1, points2, cameraMatrix1, distCoeffs1, cameraMatrix2, distCoeffs2, E, R, t, mask);
	/// ```
	///
	///
	/// ## C++ default parameters
	/// * method: cv::RANSAC
	/// * prob: 0.999
	/// * threshold: 1.0
	/// * mask: noArray()
	#[inline]
	pub fn recover_pose(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, e: &mut impl ToOutputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, method: i32, prob: f64, threshold: f64, mask: &mut impl ToInputOutputArray) -> Result<i32> {
		input_array_arg!(points1);
		input_array_arg!(points2);
		input_array_arg!(camera_matrix1);
		input_array_arg!(dist_coeffs1);
		input_array_arg!(camera_matrix2);
		input_array_arg!(dist_coeffs2);
		output_array_arg!(e);
		output_array_arg!(r);
		output_array_arg!(t);
		input_output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_double_double_const__InputOutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), e.as_raw__OutputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), method, prob, threshold, mask.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Recovers the relative camera rotation and the translation from an estimated essential
	/// matrix and the corresponding points in two images, using chirality check. Returns the number of
	/// inliers that pass the check.
	///
	/// ## Parameters
	/// * E: The input essential matrix.
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix.
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// described below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	///
	/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
	/// possible pose hypotheses by doing chirality check. The chirality check means that the
	/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
	///
	/// This function can be used to process the output E and mask from [findEssentialMat]. In this
	/// scenario, points1 and points2 are the same input for [find_essential_mat] :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
	///    Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
	///
	///    Mat E, R, t, mask;
	///
	///    E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
	///    recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
	/// ```
	///
	///
	/// ## Note
	/// This alternative version of [recover_pose_1] function uses the following default values for its arguments:
	/// * mask: noArray()
	#[inline]
	pub fn recover_pose_1_def(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<i32> {
		input_array_arg!(e);
		input_array_arg!(points1);
		input_array_arg!(points2);
		input_array_arg!(camera_matrix);
		output_array_arg!(r);
		output_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Recovers the relative camera rotation and the translation from an estimated essential
	/// matrix and the corresponding points in two images, using chirality check. Returns the number of
	/// inliers that pass the check.
	///
	/// ## Parameters
	/// * E: The input essential matrix.
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix.
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// described below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	///
	/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
	/// possible pose hypotheses by doing chirality check. The chirality check means that the
	/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
	///
	/// This function can be used to process the output E and mask from [findEssentialMat]. In this
	/// scenario, points1 and points2 are the same input for [find_essential_mat] :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
	///    Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
	///
	///    Mat E, R, t, mask;
	///
	///    E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
	///    recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
	/// ```
	///
	///
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	pub fn recover_pose_1(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, mask: &mut impl ToInputOutputArray) -> Result<i32> {
		input_array_arg!(e);
		input_array_arg!(points1);
		input_array_arg!(points2);
		input_array_arg!(camera_matrix);
		output_array_arg!(r);
		output_array_arg!(t);
		input_output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), mask.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Recovers the relative camera rotation and the translation from an estimated essential
	/// matrix and the corresponding points in two images, using chirality check. Returns the number of
	/// inliers that pass the check.
	///
	/// ## Parameters
	/// * E: The input essential matrix.
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix.
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// described below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	///
	/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
	/// possible pose hypotheses by doing chirality check. The chirality check means that the
	/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
	///
	/// This function can be used to process the output E and mask from [findEssentialMat]. In this
	/// scenario, points1 and points2 are the same input for [find_essential_mat] :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
	///    Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
	///
	///    Mat E, R, t, mask;
	///
	///    E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
	///    recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
	/// ```
	///
	///
	/// ## Overloaded parameters
	///
	/// * E: The input essential matrix.
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1.
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix.
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// description below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * distanceThresh: threshold distance which is used to filter out far away points (i.e. infinite
	/// points).
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	/// * triangulatedPoints: 3D points which were reconstructed by triangulation.
	///
	/// This function differs from the one above that it outputs the triangulated 3D point that are used for
	/// the chirality check.
	///
	/// ## Note
	/// This alternative version of [recover_pose_3] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * triangulated_points: noArray()
	#[inline]
	pub fn recover_pose_3_def(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, distance_thresh: f64) -> Result<i32> {
		input_array_arg!(e);
		input_array_arg!(points1);
		input_array_arg!(points2);
		input_array_arg!(camera_matrix);
		output_array_arg!(r);
		output_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), distance_thresh, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Recovers the relative camera rotation and the translation from an estimated essential
	/// matrix and the corresponding points in two images, using chirality check. Returns the number of
	/// inliers that pass the check.
	///
	/// ## Parameters
	/// * E: The input essential matrix.
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix.
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// described below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	///
	/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
	/// possible pose hypotheses by doing chirality check. The chirality check means that the
	/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
	///
	/// This function can be used to process the output E and mask from [findEssentialMat]. In this
	/// scenario, points1 and points2 are the same input for [find_essential_mat] :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
	///    Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
	///
	///    Mat E, R, t, mask;
	///
	///    E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
	///    recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
	/// ```
	///
	///
	/// ## Overloaded parameters
	///
	/// * E: The input essential matrix.
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1.
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix.
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// description below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * distanceThresh: threshold distance which is used to filter out far away points (i.e. infinite
	/// points).
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	/// * triangulatedPoints: 3D points which were reconstructed by triangulation.
	///
	/// This function differs from the one above that it outputs the triangulated 3D point that are used for
	/// the chirality check.
	///
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * triangulated_points: noArray()
	#[inline]
	pub fn recover_pose_3(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, distance_thresh: f64, mask: &mut impl ToInputOutputArray, triangulated_points: &mut impl ToOutputArray) -> Result<i32> {
		input_array_arg!(e);
		input_array_arg!(points1);
		input_array_arg!(points2);
		input_array_arg!(camera_matrix);
		output_array_arg!(r);
		output_array_arg!(t);
		input_output_array_arg!(mask);
		output_array_arg!(triangulated_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_const__InputOutputArrayR_const__OutputArrayR(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), distance_thresh, mask.as_raw__InputOutputArray(), triangulated_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Recovers the relative camera rotation and the translation from an estimated essential
	/// matrix and the corresponding points in two images, using chirality check. Returns the number of
	/// inliers that pass the check.
	///
	/// ## Parameters
	/// * E: The input essential matrix.
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix.
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// described below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	///
	/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
	/// possible pose hypotheses by doing chirality check. The chirality check means that the
	/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
	///
	/// This function can be used to process the output E and mask from [findEssentialMat]. In this
	/// scenario, points1 and points2 are the same input for [find_essential_mat] :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
	///    Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
	///
	///    Mat E, R, t, mask;
	///
	///    E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
	///    recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
	/// ```
	///
	///
	/// ## Overloaded parameters
	///
	/// * E: The input essential matrix.
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// description below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * focal: Focal length of the camera. Note that this function assumes that points1 and points2
	/// are feature points from cameras with same focal length and principal point.
	/// * pp: principal point of the camera.
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	///
	/// This function differs from the one above that it computes camera intrinsic matrix from focal length and
	/// principal point:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?A%20%3D%0A%5Cbegin%7Bbmatrix%7D%0Af%20%26%200%20%26%20x%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%20f%20%26%20y%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D)
	///
	/// ## Note
	/// This alternative version of [recover_pose_2] function uses the following default values for its arguments:
	/// * focal: 1.0
	/// * pp: Point2d(0,0)
	/// * mask: noArray()
	#[inline]
	pub fn recover_pose_2_def(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<i32> {
		input_array_arg!(e);
		input_array_arg!(points1);
		input_array_arg!(points2);
		output_array_arg!(r);
		output_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Recovers the relative camera rotation and the translation from an estimated essential
	/// matrix and the corresponding points in two images, using chirality check. Returns the number of
	/// inliers that pass the check.
	///
	/// ## Parameters
	/// * E: The input essential matrix.
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// Note that this function assumes that points1 and points2 are feature points from cameras with the
	/// same camera intrinsic matrix.
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// described below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	///
	/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
	/// possible pose hypotheses by doing chirality check. The chirality check means that the
	/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
	///
	/// This function can be used to process the output E and mask from [findEssentialMat]. In this
	/// scenario, points1 and points2 are the same input for [find_essential_mat] :
	/// ```C++
	///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
	///    int point_count = 100;
	///    vector<Point2f> points1(point_count);
	///    vector<Point2f> points2(point_count);
	///
	///    // initialize the points here ...
	///    for( int i = 0; i < point_count; i++ )
	///    {
	///        points1[i] = ...;
	///        points2[i] = ...;
	///    }
	///
	///    // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
	///    Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
	///
	///    Mat E, R, t, mask;
	///
	///    E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
	///    recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
	/// ```
	///
	///
	/// ## Overloaded parameters
	///
	/// * E: The input essential matrix.
	/// * points1: Array of N 2D points from the first image. The point coordinates should be
	/// floating-point (single or double precision).
	/// * points2: Array of the second image points of the same size and format as points1 .
	/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
	/// that performs a change of basis from the first camera's coordinate system to the second camera's
	/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
	/// description below.
	/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
	/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
	/// length.
	/// * focal: Focal length of the camera. Note that this function assumes that points1 and points2
	/// are feature points from cameras with same focal length and principal point.
	/// * pp: principal point of the camera.
	/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
	/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
	/// recover pose. In the output mask only inliers which pass the chirality check.
	///
	/// This function differs from the one above that it computes camera intrinsic matrix from focal length and
	/// principal point:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?A%20%3D%0A%5Cbegin%7Bbmatrix%7D%0Af%20%26%200%20%26%20x%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%20f%20%26%20y%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D)
	///
	/// ## C++ default parameters
	/// * focal: 1.0
	/// * pp: Point2d(0,0)
	/// * mask: noArray()
	#[inline]
	pub fn recover_pose_2(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, focal: f64, pp: core::Point2d, mask: &mut impl ToInputOutputArray) -> Result<i32> {
		input_array_arg!(e);
		input_array_arg!(points1);
		input_array_arg!(points2);
		output_array_arg!(r);
		output_array_arg!(t);
		input_output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_Point2d_const__InputOutputArrayR(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), focal, &pp, mask.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds out if there is any intersection between two rotated rectangles.
	///
	/// If there is then the vertices of the intersecting region are returned as well.
	///
	/// Below are some examples of intersection configurations. The hatched pattern indicates the
	/// intersecting region and the red vertices are returned by the function.
	///
	/// ![intersection examples](https://docs.opencv.org/5.0.0/intersection.png)
	///
	/// ## Parameters
	/// * rect1: First rectangle
	/// * rect2: Second rectangle
	/// * intersectingRegion: The output array of the vertices of the intersecting region. It returns
	/// at most 8 vertices. Stored as std::vector\<cv::Point2f\> or cv::Mat as Mx1 of type CV_32FC2.
	/// ## Returns
	/// One of #RectanglesIntersectTypes
	#[inline]
	pub fn rotated_rectangle_intersection(rect1: core::RotatedRect, rect2: core::RotatedRect, intersecting_region: &mut impl ToOutputArray) -> Result<i32> {
		output_array_arg!(intersecting_region);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rotatedRectangleIntersection_const_RotatedRectR_const_RotatedRectR_const__OutputArrayR(&rect1, &rect2, intersecting_region.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates the Sampson Distance between two points.
	///
	/// The function cv::sampsonDistance calculates and returns the first order approximation of the geometric error as:
	/// ![block formula](https://latex.codecogs.com/png.latex?%0Asd%28%20%5Ctexttt%7Bpt1%7D%20%2C%20%5Ctexttt%7Bpt2%7D%20%29%3D%0A%5Cfrac%7B%28%5Ctexttt%7Bpt2%7D%5Et%20%5Ccdot%20%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpt1%7D%29%5E2%7D%0A%7B%28%28%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpt1%7D%29%280%29%29%5E2%20%2B%0A%28%28%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpt1%7D%29%281%29%29%5E2%20%2B%0A%28%28%5Ctexttt%7BF%7D%5Et%20%5Ccdot%20%5Ctexttt%7Bpt2%7D%29%280%29%29%5E2%20%2B%0A%28%28%5Ctexttt%7BF%7D%5Et%20%5Ccdot%20%5Ctexttt%7Bpt2%7D%29%281%29%29%5E2%7D%0A)
	/// The fundamental matrix may be calculated using the [find_fundamental_mat] function. See [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 11.4.3 for details.
	/// ## Parameters
	/// * pt1: first homogeneous 2d point
	/// * pt2: second homogeneous 2d point
	/// * F: fundamental matrix
	/// ## Returns
	/// The computed Sampson distance.
	#[inline]
	pub fn sampson_distance(pt1: &impl ToInputArray, pt2: &impl ToInputArray, f: &impl ToInputArray) -> Result<f64> {
		input_array_arg!(pt1);
		input_array_arg!(pt2);
		input_array_arg!(f);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sampsonDistance_const__InputArrayR_const__InputArrayR_const__InputArrayR(pt1.as_raw__InputArray(), pt2.as_raw__InputArray(), f.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an object pose ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20) from **3** 3D-2D point correspondences.
	///
	/// ![Perspective projection, from object to camera frame](https://docs.opencv.org/5.0.0/pinhole_homogeneous_transformation.jpg){ width=50% }
	/// ## See also
	/// [calib3d_solvePnP]
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, 3x3 1-channel or
	/// 1x3/3x1 3-channel. vector\<Point3f\> can be also passed here.
	/// * imagePoints: Array of corresponding image points, 3x2 1-channel or 1x3/3x1 2-channel.
	///  vector\<Point2f\> can be also passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * rvecs: Output rotation vectors (see [Rodrigues] ) that, together with tvecs, brings points from
	/// the model coordinate system to the camera coordinate system. A P3P problem has up to 4 solutions.
	/// * tvecs: Output translation vectors.
	/// * flags: Method for solving a P3P problem:
	/// *   [SOLVEPNP_P3P] Method is based on the paper of Ding, Y., Yang, J., Larsson, V., Olsson, C., & Åstrom, K.
	/// "Revisiting the P3P Problem" ([ding2023revisiting](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_ding2023revisiting)).
	/// *   [SOLVEPNP_AP3P] Method is based on the paper of T. Ke and S. Roumeliotis.
	/// "An Efficient Algebraic Solution to the Perspective-Three-Point Problem" ([Ke17](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Ke17)).
	///
	/// The function estimates the object pose given 3 object points, their corresponding image
	/// projections, as well as the camera intrinsic matrix and the distortion coefficients.
	///
	///
	/// Note:
	/// The solutions are sorted by reprojection errors (lowest to highest).
	#[inline]
	pub fn solve_p3p(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32) -> Result<i32> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solveP3P_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an object pose ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20) from 3D-2D point correspondences.
	///
	/// ![Perspective projection, from object to camera frame](https://docs.opencv.org/5.0.0/pinhole_homogeneous_transformation.jpg){ width=50% }
	/// ## See also
	/// [calib3d_solvePnP]
	///
	/// This function returns a list of all the possible solutions (a solution is a <rotation vector, translation vector>
	/// couple), depending on the number of input points and the chosen method:
	/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): 3 or 4 input points. Number of returned solutions can be between 0 and 4 with 3 input points.
	/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar. Returns 2 solutions.
	/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
	/// Number of input points must be 4 and 2 solutions are returned. Object points must be defined in the following order:
	///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
	/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
	/// Only 1 solution is returned.
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
	/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can be also passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * rvecs: Vector of output rotation vectors (see [Rodrigues] ) that, together with tvecs, brings points from
	/// the model coordinate system to the camera coordinate system.
	/// * tvecs: Vector of output translation vectors.
	/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
	/// the provided rvec and tvec values as initial approximations of the rotation and translation
	/// vectors, respectively, and further optimizes them.
	/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
	/// * rvec: Rotation vector used to initialize an iterative PnP refinement algorithm, when flag is [SOLVEPNP_ITERATIVE]
	/// and useExtrinsicGuess is set to true.
	/// * tvec: Translation vector used to initialize an iterative PnP refinement algorithm, when flag is [SOLVEPNP_ITERATIVE]
	/// and useExtrinsicGuess is set to true.
	/// * reprojectionError: Optional vector of reprojection error, that is the RMS error
	/// (![inline formula](https://latex.codecogs.com/png.latex?%20%5Ctext%7BRMSE%7D%20%3D%20%5Csqrt%7B%5Cfrac%7B%5Csum%5F%7Bi%7D%5E%7BN%7D%20%5Cleft%20%28%20%5Chat%7By%5Fi%7D%20%2D%20y%5Fi%20%5Cright%20%29%5E2%7D%7BN%7D%7D%20)) between the input image points
	/// and the 3D object points projected with the estimated pose.
	///
	/// More information is described in [calib3d_solvePnP]
	///
	///
	/// Note:
	///    *   An example of how to use solvePnP for planar augmented reality can be found at
	///        opencv_source_code/samples/python/plane_ar.py
	///    *   If you are using Python:
	///        - Numpy array slices won't work as input because solvePnP requires contiguous
	///        arrays (enforced by the assertion using cv::Mat::checkVector() around line 55 of
	///        modules/3d/src/solvepnp.cpp version 2.4.9)
	///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
	///        to its calling of [undistort_points] (around line 75 of modules/3d/src/solvepnp.cpp version 2.4.9)
	///        which requires 2-channel information.
	///        - Thus, given some data D = np.array(...) where D.shape = (N,M), in order to use a subset of
	///        it as, e.g., imagePoints, one must effectively copy it into a new array: imagePoints =
	///        np.ascontiguousarray(D[:,:2]).reshape((N,1,2))
	///    *   The minimum number of points is 4 in the general case. In the case of [SOLVEPNP_P3P] and [SOLVEPNP_AP3P]
	///        methods, it is required to use exactly 4 points (the first 3 points are used to estimate all the solutions
	///        of the P3P problem, the last one is used to retain the best solution that minimizes the reprojection error).
	///    *   With [SOLVEPNP_ITERATIVE] method and `useExtrinsicGuess=true`, the minimum number of points is 3 (3 points
	///        are sufficient to compute a pose but there are up to 4 solutions). The initial solution should be close to the
	///        global solution to converge.
	///    *   With [SOLVEPNP_IPPE] input points must be >= 4 and object points must be coplanar.
	///    *   With [SOLVEPNP_IPPE_SQUARE] this is a special case suitable for marker pose estimation.
	///        Number of input points must be 4. Object points must be defined in the following order:
	///          - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///          - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///          - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///          - point 3: [-squareLength / 2, -squareLength / 2, 0]
	///    *   With [SOLVEPNP_SQPNP] input points must be >= 3
	///
	/// ## Note
	/// This alternative version of [solve_pnp_generic] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	/// * rvec: noArray()
	/// * tvec: noArray()
	/// * reprojection_error: noArray()
	#[inline]
	pub fn solve_pnp_generic_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray) -> Result<i32> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an object pose ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20) from 3D-2D point correspondences.
	///
	/// ![Perspective projection, from object to camera frame](https://docs.opencv.org/5.0.0/pinhole_homogeneous_transformation.jpg){ width=50% }
	/// ## See also
	/// [calib3d_solvePnP]
	///
	/// This function returns a list of all the possible solutions (a solution is a <rotation vector, translation vector>
	/// couple), depending on the number of input points and the chosen method:
	/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): 3 or 4 input points. Number of returned solutions can be between 0 and 4 with 3 input points.
	/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar. Returns 2 solutions.
	/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
	/// Number of input points must be 4 and 2 solutions are returned. Object points must be defined in the following order:
	///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
	/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
	/// Only 1 solution is returned.
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
	/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can be also passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * rvecs: Vector of output rotation vectors (see [Rodrigues] ) that, together with tvecs, brings points from
	/// the model coordinate system to the camera coordinate system.
	/// * tvecs: Vector of output translation vectors.
	/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
	/// the provided rvec and tvec values as initial approximations of the rotation and translation
	/// vectors, respectively, and further optimizes them.
	/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
	/// * rvec: Rotation vector used to initialize an iterative PnP refinement algorithm, when flag is [SOLVEPNP_ITERATIVE]
	/// and useExtrinsicGuess is set to true.
	/// * tvec: Translation vector used to initialize an iterative PnP refinement algorithm, when flag is [SOLVEPNP_ITERATIVE]
	/// and useExtrinsicGuess is set to true.
	/// * reprojectionError: Optional vector of reprojection error, that is the RMS error
	/// (![inline formula](https://latex.codecogs.com/png.latex?%20%5Ctext%7BRMSE%7D%20%3D%20%5Csqrt%7B%5Cfrac%7B%5Csum%5F%7Bi%7D%5E%7BN%7D%20%5Cleft%20%28%20%5Chat%7By%5Fi%7D%20%2D%20y%5Fi%20%5Cright%20%29%5E2%7D%7BN%7D%7D%20)) between the input image points
	/// and the 3D object points projected with the estimated pose.
	///
	/// More information is described in [calib3d_solvePnP]
	///
	///
	/// Note:
	///    *   An example of how to use solvePnP for planar augmented reality can be found at
	///        opencv_source_code/samples/python/plane_ar.py
	///    *   If you are using Python:
	///        - Numpy array slices won't work as input because solvePnP requires contiguous
	///        arrays (enforced by the assertion using cv::Mat::checkVector() around line 55 of
	///        modules/3d/src/solvepnp.cpp version 2.4.9)
	///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
	///        to its calling of [undistort_points] (around line 75 of modules/3d/src/solvepnp.cpp version 2.4.9)
	///        which requires 2-channel information.
	///        - Thus, given some data D = np.array(...) where D.shape = (N,M), in order to use a subset of
	///        it as, e.g., imagePoints, one must effectively copy it into a new array: imagePoints =
	///        np.ascontiguousarray(D[:,:2]).reshape((N,1,2))
	///    *   The minimum number of points is 4 in the general case. In the case of [SOLVEPNP_P3P] and [SOLVEPNP_AP3P]
	///        methods, it is required to use exactly 4 points (the first 3 points are used to estimate all the solutions
	///        of the P3P problem, the last one is used to retain the best solution that minimizes the reprojection error).
	///    *   With [SOLVEPNP_ITERATIVE] method and `useExtrinsicGuess=true`, the minimum number of points is 3 (3 points
	///        are sufficient to compute a pose but there are up to 4 solutions). The initial solution should be close to the
	///        global solution to converge.
	///    *   With [SOLVEPNP_IPPE] input points must be >= 4 and object points must be coplanar.
	///    *   With [SOLVEPNP_IPPE_SQUARE] this is a special case suitable for marker pose estimation.
	///        Number of input points must be 4. Object points must be defined in the following order:
	///          - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///          - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///          - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///          - point 3: [-squareLength / 2, -squareLength / 2, 0]
	///    *   With [SOLVEPNP_SQPNP] input points must be >= 3
	///
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	/// * rvec: noArray()
	/// * tvec: noArray()
	/// * reprojection_error: noArray()
	#[inline]
	pub fn solve_pnp_generic(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, use_extrinsic_guess: bool, flags: i32, rvec: &impl ToInputArray, tvec: &impl ToInputArray, reprojection_error: &mut impl ToOutputArray) -> Result<i32> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		input_array_arg!(rvec);
		input_array_arg!(tvec);
		output_array_arg!(reprojection_error);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_const__InputArrayR_const__InputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), use_extrinsic_guess, flags, rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), reprojection_error.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an object pose ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20) from 3D-2D point correspondences using the RANSAC scheme to deal with bad matches.
	///
	/// ![Perspective projection, from object to camera frame](https://docs.opencv.org/5.0.0/pinhole_homogeneous_transformation.jpg){ width=50% }
	/// ## See also
	/// [calib3d_solvePnP]
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
	/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can be also passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system.
	/// * tvec: Output translation vector.
	/// * useExtrinsicGuess: Parameter used for [SOLVEPNP_ITERATIVE]. If true (1), the function uses
	/// the provided rvec and tvec values as initial approximations of the rotation and translation
	/// vectors, respectively, and further optimizes them.
	/// * iterationsCount: Number of iterations.
	/// * reprojectionError: Inlier threshold value used by the RANSAC procedure. The parameter value
	/// is the maximum allowed distance between the observed and computed point projections to consider it
	/// an inlier.
	/// * confidence: The probability that the algorithm produces a useful result.
	/// * inliers: Output vector that contains indices of inliers in objectPoints and imagePoints .
	/// * flags: Method for solving a PnP problem (see [solvePnP] ).
	///
	/// The function estimates an object pose given a set of object points, their corresponding image
	/// projections, as well as the camera intrinsic matrix and the distortion coefficients. This function finds such
	/// a pose that minimizes reprojection error, that is, the sum of squared distances between the observed
	/// projections imagePoints and the projected (using [projectPoints] ) objectPoints. The use of RANSAC
	/// makes the function resistant to outliers.
	///
	///
	/// Note:
	///    *   An example of how to use solvePnPRansac for object detection can be found at
	///        [tutorial_real_time_pose]
	///    *   The default method used to estimate the camera pose for the Minimal Sample Sets step
	///        is #SOLVEPNP_EPNP. Exceptions are:
	///          - if you choose [SOLVEPNP_P3P] or #SOLVEPNP_AP3P, these methods will be used.
	///          - if the number of input points is equal to 4, [SOLVEPNP_P3P] is used.
	///    *   The method used to estimate the camera pose using all the inliers is defined by the
	///        flags parameters unless it is equal to [SOLVEPNP_P3P] or #SOLVEPNP_AP3P. In this case,
	///        the method [SOLVEPNP_EPNP] will be used instead.
	///
	/// ## Note
	/// This alternative version of [solve_pnp_ransac] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	/// * iterations_count: 100
	/// * reprojection_error: 8.0
	/// * confidence: 0.99
	/// * inliers: noArray()
	/// * flags: SOLVEPNP_ITERATIVE
	#[inline]
	pub fn solve_pnp_ransac_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvec);
		output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an object pose ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20) from 3D-2D point correspondences using the RANSAC scheme to deal with bad matches.
	///
	/// ![Perspective projection, from object to camera frame](https://docs.opencv.org/5.0.0/pinhole_homogeneous_transformation.jpg){ width=50% }
	/// ## See also
	/// [calib3d_solvePnP]
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
	/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can be also passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system.
	/// * tvec: Output translation vector.
	/// * useExtrinsicGuess: Parameter used for [SOLVEPNP_ITERATIVE]. If true (1), the function uses
	/// the provided rvec and tvec values as initial approximations of the rotation and translation
	/// vectors, respectively, and further optimizes them.
	/// * iterationsCount: Number of iterations.
	/// * reprojectionError: Inlier threshold value used by the RANSAC procedure. The parameter value
	/// is the maximum allowed distance between the observed and computed point projections to consider it
	/// an inlier.
	/// * confidence: The probability that the algorithm produces a useful result.
	/// * inliers: Output vector that contains indices of inliers in objectPoints and imagePoints .
	/// * flags: Method for solving a PnP problem (see [solvePnP] ).
	///
	/// The function estimates an object pose given a set of object points, their corresponding image
	/// projections, as well as the camera intrinsic matrix and the distortion coefficients. This function finds such
	/// a pose that minimizes reprojection error, that is, the sum of squared distances between the observed
	/// projections imagePoints and the projected (using [projectPoints] ) objectPoints. The use of RANSAC
	/// makes the function resistant to outliers.
	///
	///
	/// Note:
	///    *   An example of how to use solvePnPRansac for object detection can be found at
	///        [tutorial_real_time_pose]
	///    *   The default method used to estimate the camera pose for the Minimal Sample Sets step
	///        is #SOLVEPNP_EPNP. Exceptions are:
	///          - if you choose [SOLVEPNP_P3P] or #SOLVEPNP_AP3P, these methods will be used.
	///          - if the number of input points is equal to 4, [SOLVEPNP_P3P] is used.
	///    *   The method used to estimate the camera pose using all the inliers is defined by the
	///        flags parameters unless it is equal to [SOLVEPNP_P3P] or #SOLVEPNP_AP3P. In this case,
	///        the method [SOLVEPNP_EPNP] will be used instead.
	///
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * iterations_count: 100
	/// * reprojection_error: 8.0
	/// * confidence: 0.99
	/// * inliers: noArray()
	/// * flags: SOLVEPNP_ITERATIVE
	#[inline]
	pub fn solve_pnp_ransac(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, confidence: f64, inliers: &mut impl ToOutputArray, flags: i32) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvec);
		output_array_arg!(tvec);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_float_double_const__OutputArrayR_int(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, iterations_count, reprojection_error, confidence, inliers.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [solve_pnp_ransac_1] function uses the following default values for its arguments:
	/// * params: UsacParams()
	#[inline]
	pub fn solve_pnp_ransac_1_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvec);
		output_array_arg!(tvec);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * params: UsacParams()
	#[inline]
	pub fn solve_pnp_ransac_1(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray, params: crate::geometry::UsacParams) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvec);
		output_array_arg!(tvec);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_UsacParamsR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), inliers.as_raw__OutputArray(), &params, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Refine a pose (the translation and the rotation that transform a 3D point expressed in the object coordinate frame
	/// to the camera coordinate frame) from a 3D-2D point correspondences and starting from an initial solution.
	/// ## See also
	/// [calib3d_solvePnP]
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or 1xN/Nx1 3-channel,
	/// where N is the number of points. vector\<Point3d\> can also be passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can also be passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * rvec: Input/Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system. Input values are used as an initial solution.
	/// * tvec: Input/Output translation vector. Input values are used as an initial solution.
	/// * criteria: Criteria when to stop the Levenberg-Marquard iterative algorithm.
	///
	/// The function refines the object pose given at least 3 object points, their corresponding image
	/// projections, an initial solution for the rotation and translation vector,
	/// as well as the camera intrinsic matrix and the distortion coefficients.
	/// The function minimizes the projection error with respect to the rotation and the translation vectors, according
	/// to a Levenberg-Marquardt iterative minimization [Madsen04](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade13) process.
	///
	/// ## Note
	/// This alternative version of [solve_pnp_refine_lm] function uses the following default values for its arguments:
	/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
	#[inline]
	pub fn solve_pnp_refine_lm_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray) -> Result<()> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnPRefineLM_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Refine a pose (the translation and the rotation that transform a 3D point expressed in the object coordinate frame
	/// to the camera coordinate frame) from a 3D-2D point correspondences and starting from an initial solution.
	/// ## See also
	/// [calib3d_solvePnP]
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or 1xN/Nx1 3-channel,
	/// where N is the number of points. vector\<Point3d\> can also be passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can also be passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * rvec: Input/Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system. Input values are used as an initial solution.
	/// * tvec: Input/Output translation vector. Input values are used as an initial solution.
	/// * criteria: Criteria when to stop the Levenberg-Marquard iterative algorithm.
	///
	/// The function refines the object pose given at least 3 object points, their corresponding image
	/// projections, an initial solution for the rotation and translation vector,
	/// as well as the camera intrinsic matrix and the distortion coefficients.
	/// The function minimizes the projection error with respect to the rotation and the translation vectors, according
	/// to a Levenberg-Marquardt iterative minimization [Madsen04](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade13) process.
	///
	/// ## C++ default parameters
	/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
	#[inline]
	pub fn solve_pnp_refine_lm(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray, criteria: core::TermCriteria) -> Result<()> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnPRefineLM_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Refine a pose (the translation and the rotation that transform a 3D point expressed in the object coordinate frame
	/// to the camera coordinate frame) from a 3D-2D point correspondences and starting from an initial solution.
	/// ## See also
	/// [calib3d_solvePnP]
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or 1xN/Nx1 3-channel,
	/// where N is the number of points. vector\<Point3d\> can also be passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can also be passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * rvec: Input/Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system. Input values are used as an initial solution.
	/// * tvec: Input/Output translation vector. Input values are used as an initial solution.
	/// * criteria: Criteria when to stop the Levenberg-Marquard iterative algorithm.
	/// * VVSlambda: Gain for the virtual visual servoing control law, equivalent to the ![inline formula](https://latex.codecogs.com/png.latex?%5Calpha)
	/// gain in the Damped Gauss-Newton formulation.
	///
	/// The function refines the object pose given at least 3 object points, their corresponding image
	/// projections, an initial solution for the rotation and translation vector,
	/// as well as the camera intrinsic matrix and the distortion coefficients.
	/// The function minimizes the projection error with respect to the rotation and the translation vectors, using a
	/// virtual visual servoing (VVS) [Chaumette06](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Chaumette06) [Marchand16](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Marchand16) scheme.
	///
	/// ## Note
	/// This alternative version of [solve_pnp_refine_vvs] function uses the following default values for its arguments:
	/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
	/// * vv_slambda: 1
	#[inline]
	pub fn solve_pnp_refine_vvs_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray) -> Result<()> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnPRefineVVS_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Refine a pose (the translation and the rotation that transform a 3D point expressed in the object coordinate frame
	/// to the camera coordinate frame) from a 3D-2D point correspondences and starting from an initial solution.
	/// ## See also
	/// [calib3d_solvePnP]
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or 1xN/Nx1 3-channel,
	/// where N is the number of points. vector\<Point3d\> can also be passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can also be passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * rvec: Input/Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system. Input values are used as an initial solution.
	/// * tvec: Input/Output translation vector. Input values are used as an initial solution.
	/// * criteria: Criteria when to stop the Levenberg-Marquard iterative algorithm.
	/// * VVSlambda: Gain for the virtual visual servoing control law, equivalent to the ![inline formula](https://latex.codecogs.com/png.latex?%5Calpha)
	/// gain in the Damped Gauss-Newton formulation.
	///
	/// The function refines the object pose given at least 3 object points, their corresponding image
	/// projections, an initial solution for the rotation and translation vector,
	/// as well as the camera intrinsic matrix and the distortion coefficients.
	/// The function minimizes the projection error with respect to the rotation and the translation vectors, using a
	/// virtual visual servoing (VVS) [Chaumette06](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Chaumette06) [Marchand16](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Marchand16) scheme.
	///
	/// ## C++ default parameters
	/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
	/// * vv_slambda: 1
	#[inline]
	pub fn solve_pnp_refine_vvs(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray, criteria: core::TermCriteria, vv_slambda: f64) -> Result<()> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnPRefineVVS_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria_double(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), &criteria, vv_slambda, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an object pose ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20) from 3D-2D point correspondences:
	///
	/// ![Perspective projection, from object to camera frame](https://docs.opencv.org/5.0.0/pinhole_homogeneous_transformation.jpg){ width=50% }
	/// ## See also
	/// [calib3d_solvePnP]
	///
	/// This function returns the rotation and the translation vectors that transform a 3D point expressed in the object
	/// coordinate frame to the camera coordinate frame, using different methods:
	/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): need 4 input points to return a unique solution.
	/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar.
	/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
	/// Number of input points must be 4. Object points must be defined in the following order:
	///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
	/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
	/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can be also passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system.
	/// * tvec: Output translation vector.
	/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
	/// the provided rvec and tvec values as initial approximations of the rotation and translation
	/// vectors, respectively, and further optimizes them.
	/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
	///
	/// More information about Perspective-n-Points is described in [calib3d_solvePnP]
	///
	///
	/// Note:
	///    *   An example of how to use solvePnP for planar augmented reality can be found at
	///        opencv_source_code/samples/python/plane_ar.py
	///    *   If you are using Python:
	///        - Numpy array slices won't work as input because solvePnP requires contiguous
	///        arrays (enforced by the assertion using cv::Mat::checkVector() around line 55 of
	///        modules/3d/src/solvepnp.cpp version 2.4.9)
	///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
	///        to its calling of [undistort_points] (around line 75 of modules/3d/src/solvepnp.cpp version 2.4.9)
	///        which requires 2-channel information.
	///        - Thus, given some data D = np.array(...) where D.shape = (N,M), in order to use a subset of
	///        it as, e.g., imagePoints, one must effectively copy it into a new array: imagePoints =
	///        np.ascontiguousarray(D[:,:2]).reshape((N,1,2))
	///    *   The minimum number of points is 4 in the general case. In the case of [SOLVEPNP_P3P] and [SOLVEPNP_AP3P]
	///        methods, it is required to use exactly 4 points (the first 3 points are used to estimate all the solutions
	///        of the P3P problem, the last one is used to retain the best solution that minimizes the reprojection error).
	///    *   With [SOLVEPNP_ITERATIVE] method and `useExtrinsicGuess=true`, the minimum number of points is 3 (3 points
	///        are sufficient to compute a pose but there are up to 4 solutions). The initial solution should be close to the
	///        global solution to converge. The function returns true if some solution is found. User code is responsible for
	///        solution quality assessment.
	///    *   With [SOLVEPNP_IPPE] input points must be >= 4 and object points must be coplanar.
	///    *   With [SOLVEPNP_IPPE_SQUARE] this is a special case suitable for marker pose estimation.
	///        Number of input points must be 4. Object points must be defined in the following order:
	///          - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///          - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///          - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///          - point 3: [-squareLength / 2, -squareLength / 2, 0]
	///    *  With [SOLVEPNP_SQPNP] input points must be >= 3
	///
	/// ## Note
	/// This alternative version of [solve_pnp] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	#[inline]
	pub fn solve_pnp_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvec);
		output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an object pose ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20) from 3D-2D point correspondences:
	///
	/// ![Perspective projection, from object to camera frame](https://docs.opencv.org/5.0.0/pinhole_homogeneous_transformation.jpg){ width=50% }
	/// ## See also
	/// [calib3d_solvePnP]
	///
	/// This function returns the rotation and the translation vectors that transform a 3D point expressed in the object
	/// coordinate frame to the camera coordinate frame, using different methods:
	/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): need 4 input points to return a unique solution.
	/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar.
	/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
	/// Number of input points must be 4. Object points must be defined in the following order:
	///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
	/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
	///
	/// ## Parameters
	/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
	/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
	/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
	/// where N is the number of points. vector\<Point2d\> can be also passed here.
	/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
	/// assumed.
	/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
	/// the model coordinate system to the camera coordinate system.
	/// * tvec: Output translation vector.
	/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
	/// the provided rvec and tvec values as initial approximations of the rotation and translation
	/// vectors, respectively, and further optimizes them.
	/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
	///
	/// More information about Perspective-n-Points is described in [calib3d_solvePnP]
	///
	///
	/// Note:
	///    *   An example of how to use solvePnP for planar augmented reality can be found at
	///        opencv_source_code/samples/python/plane_ar.py
	///    *   If you are using Python:
	///        - Numpy array slices won't work as input because solvePnP requires contiguous
	///        arrays (enforced by the assertion using cv::Mat::checkVector() around line 55 of
	///        modules/3d/src/solvepnp.cpp version 2.4.9)
	///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
	///        to its calling of [undistort_points] (around line 75 of modules/3d/src/solvepnp.cpp version 2.4.9)
	///        which requires 2-channel information.
	///        - Thus, given some data D = np.array(...) where D.shape = (N,M), in order to use a subset of
	///        it as, e.g., imagePoints, one must effectively copy it into a new array: imagePoints =
	///        np.ascontiguousarray(D[:,:2]).reshape((N,1,2))
	///    *   The minimum number of points is 4 in the general case. In the case of [SOLVEPNP_P3P] and [SOLVEPNP_AP3P]
	///        methods, it is required to use exactly 4 points (the first 3 points are used to estimate all the solutions
	///        of the P3P problem, the last one is used to retain the best solution that minimizes the reprojection error).
	///    *   With [SOLVEPNP_ITERATIVE] method and `useExtrinsicGuess=true`, the minimum number of points is 3 (3 points
	///        are sufficient to compute a pose but there are up to 4 solutions). The initial solution should be close to the
	///        global solution to converge. The function returns true if some solution is found. User code is responsible for
	///        solution quality assessment.
	///    *   With [SOLVEPNP_IPPE] input points must be >= 4 and object points must be coplanar.
	///    *   With [SOLVEPNP_IPPE_SQUARE] this is a special case suitable for marker pose estimation.
	///        Number of input points must be 4. Object points must be defined in the following order:
	///          - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///          - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///          - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///          - point 3: [-squareLength / 2, -squareLength / 2, 0]
	///    *  With [SOLVEPNP_SQPNP] input points must be >= 3
	///
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	#[inline]
	pub fn solve_pnp(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvec);
		output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// This function reconstructs 3-dimensional points (in homogeneous coordinates) by using
	/// their observations with a stereo camera.
	///
	/// ## Parameters
	/// * projMatr1: 3x4 projection matrix of the first camera, i.e. this matrix projects 3D points
	/// given in the world's coordinate system into the first image.
	/// * projMatr2: 3x4 projection matrix of the second camera, i.e. this matrix projects 3D points
	/// given in the world's coordinate system into the second image.
	/// * projPoints1: 2xN array of feature points in the first image. In the case of the c++ version,
	/// it can be also a vector of feature points or two-channel matrix of size 1xN or Nx1.
	/// * projPoints2: 2xN array of corresponding points in the second image. In the case of the c++
	/// version, it can be also a vector of feature points or two-channel matrix of size 1xN or Nx1.
	/// * points4D: 4xN array of reconstructed points in homogeneous coordinates. These points are
	/// returned in the world's coordinate system.
	///
	///
	/// Note:
	///    Keep in mind that all input data should be of float type in order for this function to work.
	///
	///
	/// Note:
	///    If the projection matrices from [stereoRectify] are used, then the returned points are
	///    represented in the first camera's rectified coordinate system.
	/// ## See also
	/// reprojectImageTo3D
	#[inline]
	pub fn triangulate_points(proj_matr1: &impl ToInputArray, proj_matr2: &impl ToInputArray, proj_points1: &impl ToInputArray, proj_points2: &impl ToInputArray, points4_d: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(proj_matr1);
		input_array_arg!(proj_matr2);
		input_array_arg!(proj_points1);
		input_array_arg!(proj_points2);
		output_array_arg!(points4_d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_triangulatePoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(proj_matr1.as_raw__InputArray(), proj_matr2.as_raw__InputArray(), proj_points1.as_raw__InputArray(), proj_points2.as_raw__InputArray(), points4_d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Compute undistorted image points position
	///
	/// ## Parameters
	/// * src: Observed points position, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel (CV_32FC2 or CV_64FC2) (or vector\<Point2f\> ).
	/// * dst: Output undistorted points position (1xN/Nx1 2-channel or vector\<Point2f\> ).
	/// * cameraMatrix: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
	/// * distCoeffs: Distortion coefficients
	///
	/// ## Note
	/// This alternative version of [undistort_image_points] function uses the following default values for its arguments:
	/// * unnamed: TermCriteria(TermCriteria::MAX_ITER,5,0.01)
	#[inline]
	pub fn undistort_image_points_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_undistortImagePoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Compute undistorted image points position
	///
	/// ## Parameters
	/// * src: Observed points position, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel (CV_32FC2 or CV_64FC2) (or vector\<Point2f\> ).
	/// * dst: Output undistorted points position (1xN/Nx1 2-channel or vector\<Point2f\> ).
	/// * cameraMatrix: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
	/// * distCoeffs: Distortion coefficients
	///
	/// ## C++ default parameters
	/// * unnamed: TermCriteria(TermCriteria::MAX_ITER,5,0.01)
	#[inline]
	pub fn undistort_image_points(src: &impl ToInputArray, dst: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, unnamed: core::TermCriteria) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_undistortImagePoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(src.as_raw__InputArray(), dst.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), &unnamed, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes the ideal point coordinates from the observed point coordinates.
	///
	/// The function is similar to [undistort] and [init_undistort_rectify_map] but it operates on a
	/// sparse set of points instead of a raster image. Also the function performs a reverse transformation
	/// to  #projectPoints. In case of a 3D object, it does not reconstruct its 3D coordinates, but for a
	/// planar object, it does, up to a translation vector, if the proper R is specified.
	///
	/// For each observed point coordinate ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) the function computes:
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Barray%7D%7Bl%7D%0Ax%5E%7B%22%7D%20%20%5Cleftarrow%20%28u%20%2D%20c%5Fx%29%2Ff%5Fx%20%20%5C%5C%0Ay%5E%7B%22%7D%20%20%5Cleftarrow%20%28v%20%2D%20c%5Fy%29%2Ff%5Fy%20%20%5C%5C%0A%28x%27%2Cy%27%29%20%3D%20undistort%28x%5E%7B%22%7D%2Cy%5E%7B%22%7D%2C%20%5Ctexttt%7BdistCoeffs%7D%29%20%5C%5C%0A%7B%5BX%5C%2CY%5C%2CW%5D%7D%20%5ET%20%20%5Cleftarrow%20R%2A%5Bx%27%20%5C%2C%20y%27%20%5C%2C%201%5D%5ET%20%20%5C%5C%0Ax%20%20%5Cleftarrow%20X%2FW%20%20%5C%5C%0Ay%20%20%5Cleftarrow%20Y%2FW%20%20%5C%5C%0A%5Ctext%7Bonly%20performed%20if%20P%20is%20specified%3A%7D%20%5C%5C%0Au%27%20%20%5Cleftarrow%20x%20%7Bf%27%7D%5Fx%20%2B%20%7Bc%27%7D%5Fx%20%20%5C%5C%0Av%27%20%20%5Cleftarrow%20y%20%7Bf%27%7D%5Fy%20%2B%20%7Bc%27%7D%5Fy%0A%5Cend%7Barray%7D%0A)
	///
	/// where *undistort* is an approximate iterative algorithm that estimates the normalized original
	/// point coordinates out of the normalized distorted point coordinates ("normalized" means that the
	/// coordinates do not depend on the camera matrix).
	///
	/// The function can be used for both a stereo camera head or a monocular camera (when R is empty).
	///
	///
	/// Note: **Coordinate Systems:**
	/// - **Input (`src`)**: Points are expected in **pixel coordinates** of the distorted image, i.e.,
	///   coordinates ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) measured in pixels from the top-left corner of the image.
	/// - **Output (`dst`)**: The coordinate system of output points depends on parameter `P`:
	///   - If `P` is provided (not empty): Output points are in **pixel coordinates** of the rectified/undistorted image plane, using the camera matrix `P`.
	///   - If `P` is empty or identity: Output points are in **normalized camera coordinates** (also called "normalized image coordinates"),
	///    which are dimensionless coordinates ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29) in the camera's focal plane, related to pixel coordinates by:
	///    ![inline formula](https://latex.codecogs.com/png.latex?x%20%3D%20%28u%20%2D%20c%5Fx%29%20%2F%20f%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?y%20%3D%20%28v%20%2D%20c%5Fy%29%20%2F%20f%5Fy). These normalized coordinates are independent of the camera's intrinsic parameters and are useful for 3D reconstruction or epipolar geometry.
	///
	/// ## Parameters
	/// * src: Observed point coordinates in **pixel coordinates** of the distorted image, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel (CV_32FC2 or CV_64FC2) (or
	/// vector\<Point2f\> ).
	/// * dst: Output ideal point coordinates (1xN/Nx1 2-channel or vector\<Point2f\> ) after undistortion and reverse perspective
	/// transformation. If matrix P is identity or omitted, dst will contain normalized point coordinates.
	/// * cameraMatrix: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
	/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
	/// * R: Rectification transformation in the object space (3x3 matrix). R1 or R2 computed by
	/// [stereo_rectify] can be passed here. If the matrix is empty, the identity transformation is used.
	/// * P: New camera matrix (3x3) or new projection matrix (3x4) ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%7Bf%27%7D%5Fx%20%26%200%20%26%20%7Bc%27%7D%5Fx%20%26%20t%5Fx%20%5C%5C%200%20%26%20%7Bf%27%7D%5Fy%20%26%20%7Bc%27%7D%5Fy%20%26%20t%5Fy%20%5C%5C%200%20%26%200%20%26%201%20%26%20t%5Fz%20%5Cend%7Bbmatrix%7D). P1 or P2 computed by
	/// [stereo_rectify] can be passed here. If the matrix is empty, the identity new camera matrix is used and output will be in normalized coordinates.
	/// * criteria: termination criteria for the iterative point undistortion algorithm
	///
	/// ## Note
	/// This alternative version of [undistort_points] function uses the following default values for its arguments:
	/// * r: noArray()
	/// * p: noArray()
	/// * criteria: TermCriteria(TermCriteria::MAX_ITER,5,0.01)
	#[inline]
	pub fn undistort_points_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes the ideal point coordinates from the observed point coordinates.
	///
	/// The function is similar to [undistort] and [init_undistort_rectify_map] but it operates on a
	/// sparse set of points instead of a raster image. Also the function performs a reverse transformation
	/// to  #projectPoints. In case of a 3D object, it does not reconstruct its 3D coordinates, but for a
	/// planar object, it does, up to a translation vector, if the proper R is specified.
	///
	/// For each observed point coordinate ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) the function computes:
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Barray%7D%7Bl%7D%0Ax%5E%7B%22%7D%20%20%5Cleftarrow%20%28u%20%2D%20c%5Fx%29%2Ff%5Fx%20%20%5C%5C%0Ay%5E%7B%22%7D%20%20%5Cleftarrow%20%28v%20%2D%20c%5Fy%29%2Ff%5Fy%20%20%5C%5C%0A%28x%27%2Cy%27%29%20%3D%20undistort%28x%5E%7B%22%7D%2Cy%5E%7B%22%7D%2C%20%5Ctexttt%7BdistCoeffs%7D%29%20%5C%5C%0A%7B%5BX%5C%2CY%5C%2CW%5D%7D%20%5ET%20%20%5Cleftarrow%20R%2A%5Bx%27%20%5C%2C%20y%27%20%5C%2C%201%5D%5ET%20%20%5C%5C%0Ax%20%20%5Cleftarrow%20X%2FW%20%20%5C%5C%0Ay%20%20%5Cleftarrow%20Y%2FW%20%20%5C%5C%0A%5Ctext%7Bonly%20performed%20if%20P%20is%20specified%3A%7D%20%5C%5C%0Au%27%20%20%5Cleftarrow%20x%20%7Bf%27%7D%5Fx%20%2B%20%7Bc%27%7D%5Fx%20%20%5C%5C%0Av%27%20%20%5Cleftarrow%20y%20%7Bf%27%7D%5Fy%20%2B%20%7Bc%27%7D%5Fy%0A%5Cend%7Barray%7D%0A)
	///
	/// where *undistort* is an approximate iterative algorithm that estimates the normalized original
	/// point coordinates out of the normalized distorted point coordinates ("normalized" means that the
	/// coordinates do not depend on the camera matrix).
	///
	/// The function can be used for both a stereo camera head or a monocular camera (when R is empty).
	///
	///
	/// Note: **Coordinate Systems:**
	/// - **Input (`src`)**: Points are expected in **pixel coordinates** of the distorted image, i.e.,
	///   coordinates ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) measured in pixels from the top-left corner of the image.
	/// - **Output (`dst`)**: The coordinate system of output points depends on parameter `P`:
	///   - If `P` is provided (not empty): Output points are in **pixel coordinates** of the rectified/undistorted image plane, using the camera matrix `P`.
	///   - If `P` is empty or identity: Output points are in **normalized camera coordinates** (also called "normalized image coordinates"),
	///    which are dimensionless coordinates ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29) in the camera's focal plane, related to pixel coordinates by:
	///    ![inline formula](https://latex.codecogs.com/png.latex?x%20%3D%20%28u%20%2D%20c%5Fx%29%20%2F%20f%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?y%20%3D%20%28v%20%2D%20c%5Fy%29%20%2F%20f%5Fy). These normalized coordinates are independent of the camera's intrinsic parameters and are useful for 3D reconstruction or epipolar geometry.
	///
	/// ## Parameters
	/// * src: Observed point coordinates in **pixel coordinates** of the distorted image, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel (CV_32FC2 or CV_64FC2) (or
	/// vector\<Point2f\> ).
	/// * dst: Output ideal point coordinates (1xN/Nx1 2-channel or vector\<Point2f\> ) after undistortion and reverse perspective
	/// transformation. If matrix P is identity or omitted, dst will contain normalized point coordinates.
	/// * cameraMatrix: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
	/// * distCoeffs: Input vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
	/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
	/// * R: Rectification transformation in the object space (3x3 matrix). R1 or R2 computed by
	/// [stereo_rectify] can be passed here. If the matrix is empty, the identity transformation is used.
	/// * P: New camera matrix (3x3) or new projection matrix (3x4) ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%7Bf%27%7D%5Fx%20%26%200%20%26%20%7Bc%27%7D%5Fx%20%26%20t%5Fx%20%5C%5C%200%20%26%20%7Bf%27%7D%5Fy%20%26%20%7Bc%27%7D%5Fy%20%26%20t%5Fy%20%5C%5C%200%20%26%200%20%26%201%20%26%20t%5Fz%20%5Cend%7Bbmatrix%7D). P1 or P2 computed by
	/// [stereo_rectify] can be passed here. If the matrix is empty, the identity new camera matrix is used and output will be in normalized coordinates.
	/// * criteria: termination criteria for the iterative point undistortion algorithm
	///
	/// ## C++ default parameters
	/// * r: noArray()
	/// * p: noArray()
	/// * criteria: TermCriteria(TermCriteria::MAX_ITER,5,0.01)
	#[inline]
	pub fn undistort_points(src: &impl ToInputArray, dst: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, r: &impl ToInputArray, p: &impl ToInputArray, criteria: core::TermCriteria) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_array_arg!(r);
		input_array_arg!(p);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(src.as_raw__InputArray(), dst.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), r.as_raw__InputArray(), p.as_raw__InputArray(), &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Point cloud sampling by Voxel Grid filter downsampling.
	///
	/// Creates a 3D voxel grid (a set of tiny 3D boxes in space) over the input
	/// point cloud data, in each voxel (i.e., 3D box), all the points present will be
	/// approximated (i.e., downsampled) with the point closest to their centroid.
	///
	/// ## Parameters
	/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
	///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * length: Grid length.
	/// * width: Grid width.
	/// * height: Grid height.
	/// ## Returns
	/// The number of points actually sampled.
	#[inline]
	pub fn voxel_grid_sampling(sampled_point_flags: &mut impl ToOutputArray, input_pts: &impl ToInputArray, length: f32, width: f32, height: f32) -> Result<i32> {
		output_array_arg!(sampled_point_flags);
		input_array_arg!(input_pts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_voxelGridSampling_const__OutputArrayR_const__InputArrayR_float_float_float(sampled_point_flags.as_raw__OutputArray(), input_pts.as_raw__InputArray(), length, width, height, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Levenberg-Marquadt solver
	///
	/// A Levenberg-Marquadt algorithm locally minimizes an objective function value (aka energy, cost or error) starting from
	/// current param vector.
	/// To do that, at each iteration it repeatedly calculates the energy at probe points until it's reduced.
	/// To calculate a probe point, a linear equation is solved: (J^T*J + lambda*D)*dx = -J^T*b where J is a function jacobian,
	/// b is a vector of residuals (aka errors or energy terms), D is a diagonal matrix generated from J^T*J diagonal
	/// and lambda changes for each probe point. Then the resulting dx is "added" to current variable and it forms
	/// a probe value. "Added" is quoted because in some groups (e.g. SO(3) group) such an increment can be a non-trivial operation.
	///
	/// For more details, please refer to Wikipedia page (<https://en.wikipedia.org/wiki/Levenberg%E2%80%93Marquardt_algorithm>).
	///
	/// This solver supports fixed variables and two forms of callback function:
	/// 1. Generating ordinary jacobian J and residual vector err ("long")
	/// 2. Generating normal equation matrix J^T*J and gradient vector J^T*err
	///
	/// Currently the solver supports dense jacobian matrix and linear parameter increment.
	pub struct LevMarq {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { LevMarq }

	impl Drop for LevMarq {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_LevMarq_delete(self.as_raw_mut_LevMarq()) };
		}
	}

	unsafe impl Send for LevMarq {}

	/// Constant methods for [crate::geometry::LevMarq]
	pub trait LevMarqTraitConst {
		fn as_raw_LevMarq(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::geometry::LevMarq]
	pub trait LevMarqTrait: crate::geometry::LevMarqTraitConst {
		fn as_raw_mut_LevMarq(&mut self) -> *mut c_void;

		/// Runs Levenberg-Marquadt algorithm using current settings and given parameters vector.
		/// The method returns the optimization report.
		#[inline]
		fn optimize(&mut self) -> Result<crate::geometry::LevMarq_Report> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_optimize(self.as_raw_mut_LevMarq(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Report::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Runs optimization using the passed vector of parameters as the start point.
		///
		/// The final vector of parameters (whether the algorithm converged or not) is stored at the same
		/// vector.
		/// This method can be used instead of the optimize() method if rerun with different start points is required.
		/// The method returns the optimization report.
		///
		/// ## Parameters
		/// * param: initial/final vector of parameters.
		///
		/// Note that the dimensionality of parameter space is defined by the size of param vector,
		/// and the dimensionality of optimized criteria is defined by the size of err vector
		/// computed by the callback.
		#[inline]
		fn run(&mut self, param: &mut impl ToInputOutputArray) -> Result<crate::geometry::LevMarq_Report> {
			input_output_array_arg!(param);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_run_const__InputOutputArrayR(self.as_raw_mut_LevMarq(), param.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Report::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl std::fmt::Debug for LevMarq {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("LevMarq")
				.finish()
		}
	}

	impl crate::geometry::LevMarqTraitConst for LevMarq {
		#[inline] fn as_raw_LevMarq(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::geometry::LevMarqTrait for LevMarq {
		#[inline] fn as_raw_mut_LevMarq(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LevMarq, crate::geometry::LevMarqTraitConst, as_raw_LevMarq, crate::geometry::LevMarqTrait, as_raw_mut_LevMarq }

	/// Optimization report
	///
	/// The structure is returned when optimization is over.
	pub struct LevMarq_Report {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { LevMarq_Report }

	impl Drop for LevMarq_Report {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_LevMarq_Report_delete(self.as_raw_mut_LevMarq_Report()) };
		}
	}

	unsafe impl Send for LevMarq_Report {}

	impl LevMarq_Report {
		#[inline]
		pub fn new(is_found: bool, n_iters: i32, final_energy: f64) -> Result<crate::geometry::LevMarq_Report> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Report_Report_bool_int_double(is_found, n_iters, final_energy, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Report::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::geometry::LevMarq_Report]
	pub trait LevMarq_ReportTraitConst {
		fn as_raw_LevMarq_Report(&self) -> *const c_void;

		#[inline]
		fn found(&self) -> bool {
			let ret = unsafe { sys::cv_LevMarq_Report_propFound_const(self.as_raw_LevMarq_Report()) };
			ret
		}

		#[inline]
		fn iters(&self) -> i32 {
			let ret = unsafe { sys::cv_LevMarq_Report_propIters_const(self.as_raw_LevMarq_Report()) };
			ret
		}

		#[inline]
		fn energy(&self) -> f64 {
			let ret = unsafe { sys::cv_LevMarq_Report_propEnergy_const(self.as_raw_LevMarq_Report()) };
			ret
		}

	}

	/// Mutable methods for [crate::geometry::LevMarq_Report]
	pub trait LevMarq_ReportTrait: crate::geometry::LevMarq_ReportTraitConst {
		fn as_raw_mut_LevMarq_Report(&mut self) -> *mut c_void;

		#[inline]
		fn set_found(&mut self, val: bool) {
			let ret = unsafe { sys::cv_LevMarq_Report_propFound_const_bool(self.as_raw_mut_LevMarq_Report(), val) };
			ret
		}

		#[inline]
		fn set_iters(&mut self, val: i32) {
			let ret = unsafe { sys::cv_LevMarq_Report_propIters_const_int(self.as_raw_mut_LevMarq_Report(), val) };
			ret
		}

		#[inline]
		fn set_energy(&mut self, val: f64) {
			let ret = unsafe { sys::cv_LevMarq_Report_propEnergy_const_double(self.as_raw_mut_LevMarq_Report(), val) };
			ret
		}

	}

	impl std::fmt::Debug for LevMarq_Report {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("LevMarq_Report")
				.field("found", &crate::geometry::LevMarq_ReportTraitConst::found(self))
				.field("iters", &crate::geometry::LevMarq_ReportTraitConst::iters(self))
				.field("energy", &crate::geometry::LevMarq_ReportTraitConst::energy(self))
				.finish()
		}
	}

	impl crate::geometry::LevMarq_ReportTraitConst for LevMarq_Report {
		#[inline] fn as_raw_LevMarq_Report(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::geometry::LevMarq_ReportTrait for LevMarq_Report {
		#[inline] fn as_raw_mut_LevMarq_Report(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LevMarq_Report, crate::geometry::LevMarq_ReportTraitConst, as_raw_LevMarq_Report, crate::geometry::LevMarq_ReportTrait, as_raw_mut_LevMarq_Report }

	/// Structure to keep LevMarq settings
	///
	/// The structure allows a user to pass algorithm parameters along with their names like this:
	/// ```C++
	/// MySolver solver(nVars, callback, MySolver::Settings().geodesicS(true).geoScale(1.0));
	/// ```
	///
	pub struct LevMarq_Settings {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { LevMarq_Settings }

	impl Drop for LevMarq_Settings {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_LevMarq_Settings_delete(self.as_raw_mut_LevMarq_Settings()) };
		}
	}

	unsafe impl Send for LevMarq_Settings {}

	impl LevMarq_Settings {
		#[inline]
		pub fn default() -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_Settings(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::geometry::LevMarq_Settings]
	pub trait LevMarq_SettingsTraitConst {
		fn as_raw_LevMarq_Settings(&self) -> *const c_void;

		#[inline]
		fn jacobi_scaling(&self) -> bool {
			let ret = unsafe { sys::cv_LevMarq_Settings_propJacobiScaling_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn up_double(&self) -> bool {
			let ret = unsafe { sys::cv_LevMarq_Settings_propUpDouble_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn use_step_quality(&self) -> bool {
			let ret = unsafe { sys::cv_LevMarq_Settings_propUseStepQuality_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn clamp_diagonal(&self) -> bool {
			let ret = unsafe { sys::cv_LevMarq_Settings_propClampDiagonal_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn step_norm_inf(&self) -> bool {
			let ret = unsafe { sys::cv_LevMarq_Settings_propStepNormInf_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn check_rel_energy_change(&self) -> bool {
			let ret = unsafe { sys::cv_LevMarq_Settings_propCheckRelEnergyChange_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn check_min_gradient(&self) -> bool {
			let ret = unsafe { sys::cv_LevMarq_Settings_propCheckMinGradient_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn check_step_norm(&self) -> bool {
			let ret = unsafe { sys::cv_LevMarq_Settings_propCheckStepNorm_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn geodesic(&self) -> bool {
			let ret = unsafe { sys::cv_LevMarq_Settings_propGeodesic_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn h_geo(&self) -> f64 {
			let ret = unsafe { sys::cv_LevMarq_Settings_propHGeo_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn geo_scale(&self) -> f64 {
			let ret = unsafe { sys::cv_LevMarq_Settings_propGeoScale_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn step_norm_tolerance(&self) -> f64 {
			let ret = unsafe { sys::cv_LevMarq_Settings_propStepNormTolerance_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn rel_energy_delta_tolerance(&self) -> f64 {
			let ret = unsafe { sys::cv_LevMarq_Settings_propRelEnergyDeltaTolerance_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn min_gradient_tolerance(&self) -> f64 {
			let ret = unsafe { sys::cv_LevMarq_Settings_propMinGradientTolerance_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn small_energy_tolerance(&self) -> f64 {
			let ret = unsafe { sys::cv_LevMarq_Settings_propSmallEnergyTolerance_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn max_iterations(&self) -> u32 {
			let ret = unsafe { sys::cv_LevMarq_Settings_propMaxIterations_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn initial_lambda(&self) -> f64 {
			let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLambda_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn initial_lm_up_factor(&self) -> f64 {
			let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLmUpFactor_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

		#[inline]
		fn initial_lm_down_factor(&self) -> f64 {
			let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLmDownFactor_const(self.as_raw_LevMarq_Settings()) };
			ret
		}

	}

	/// Mutable methods for [crate::geometry::LevMarq_Settings]
	pub trait LevMarq_SettingsTrait: crate::geometry::LevMarq_SettingsTraitConst {
		fn as_raw_mut_LevMarq_Settings(&mut self) -> *mut c_void;

		#[inline]
		fn set_jacobi_scaling(&mut self, val: bool) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propJacobiScaling_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_up_double(&mut self, val: bool) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propUpDouble_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_use_step_quality(&mut self, val: bool) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propUseStepQuality_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_clamp_diagonal(&mut self, val: bool) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propClampDiagonal_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_step_norm_inf(&mut self, val: bool) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propStepNormInf_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_check_rel_energy_change(&mut self, val: bool) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propCheckRelEnergyChange_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_check_min_gradient(&mut self, val: bool) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propCheckMinGradient_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_check_step_norm(&mut self, val: bool) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propCheckStepNorm_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_geodesic(&mut self, val: bool) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propGeodesic_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_h_geo(&mut self, val: f64) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propHGeo_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_geo_scale(&mut self, val: f64) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propGeoScale_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_step_norm_tolerance(&mut self, val: f64) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propStepNormTolerance_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_rel_energy_delta_tolerance(&mut self, val: f64) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propRelEnergyDeltaTolerance_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_min_gradient_tolerance(&mut self, val: f64) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propMinGradientTolerance_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_small_energy_tolerance(&mut self, val: f64) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propSmallEnergyTolerance_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_max_iterations(&mut self, val: u32) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propMaxIterations_const_unsigned_int(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_initial_lambda(&mut self, val: f64) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLambda_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_initial_lm_up_factor(&mut self, val: f64) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLmUpFactor_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_initial_lm_down_factor(&mut self, val: f64) {
			let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLmDownFactor_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
			ret
		}

		#[inline]
		fn set_jacobi_scaling_1(&mut self, v: bool) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setJacobiScaling_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_up_double_1(&mut self, v: bool) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setUpDouble_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_use_step_quality_1(&mut self, v: bool) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setUseStepQuality_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_clamp_diagonal_1(&mut self, v: bool) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setClampDiagonal_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_step_norm_inf_1(&mut self, v: bool) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setStepNormInf_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_check_rel_energy_change_1(&mut self, v: bool) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setCheckRelEnergyChange_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_check_min_gradient_1(&mut self, v: bool) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setCheckMinGradient_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_check_step_norm_1(&mut self, v: bool) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setCheckStepNorm_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_geodesic_1(&mut self, v: bool) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setGeodesic_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_h_geo_1(&mut self, v: f64) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setHGeo_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_geo_scale_1(&mut self, v: f64) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setGeoScale_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_step_norm_tolerance_1(&mut self, v: f64) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setStepNormTolerance_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_rel_energy_delta_tolerance_1(&mut self, v: f64) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setRelEnergyDeltaTolerance_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_min_gradient_tolerance_1(&mut self, v: f64) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setMinGradientTolerance_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_small_energy_tolerance_1(&mut self, v: f64) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setSmallEnergyTolerance_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_max_iterations_1(&mut self, v: i32) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setMaxIterations_int(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_initial_lambda_1(&mut self, v: f64) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setInitialLambda_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_initial_lm_up_factor_1(&mut self, v: f64) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setInitialLmUpFactor_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn set_initial_lm_down_factor_1(&mut self, v: f64) -> Result<crate::geometry::LevMarq_Settings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LevMarq_Settings_setInitialLmDownFactor_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::LevMarq_Settings::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl std::fmt::Debug for LevMarq_Settings {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("LevMarq_Settings")
				.field("jacobi_scaling", &crate::geometry::LevMarq_SettingsTraitConst::jacobi_scaling(self))
				.field("up_double", &crate::geometry::LevMarq_SettingsTraitConst::up_double(self))
				.field("use_step_quality", &crate::geometry::LevMarq_SettingsTraitConst::use_step_quality(self))
				.field("clamp_diagonal", &crate::geometry::LevMarq_SettingsTraitConst::clamp_diagonal(self))
				.field("step_norm_inf", &crate::geometry::LevMarq_SettingsTraitConst::step_norm_inf(self))
				.field("check_rel_energy_change", &crate::geometry::LevMarq_SettingsTraitConst::check_rel_energy_change(self))
				.field("check_min_gradient", &crate::geometry::LevMarq_SettingsTraitConst::check_min_gradient(self))
				.field("check_step_norm", &crate::geometry::LevMarq_SettingsTraitConst::check_step_norm(self))
				.field("geodesic", &crate::geometry::LevMarq_SettingsTraitConst::geodesic(self))
				.field("h_geo", &crate::geometry::LevMarq_SettingsTraitConst::h_geo(self))
				.field("geo_scale", &crate::geometry::LevMarq_SettingsTraitConst::geo_scale(self))
				.field("step_norm_tolerance", &crate::geometry::LevMarq_SettingsTraitConst::step_norm_tolerance(self))
				.field("rel_energy_delta_tolerance", &crate::geometry::LevMarq_SettingsTraitConst::rel_energy_delta_tolerance(self))
				.field("min_gradient_tolerance", &crate::geometry::LevMarq_SettingsTraitConst::min_gradient_tolerance(self))
				.field("small_energy_tolerance", &crate::geometry::LevMarq_SettingsTraitConst::small_energy_tolerance(self))
				.field("max_iterations", &crate::geometry::LevMarq_SettingsTraitConst::max_iterations(self))
				.field("initial_lambda", &crate::geometry::LevMarq_SettingsTraitConst::initial_lambda(self))
				.field("initial_lm_up_factor", &crate::geometry::LevMarq_SettingsTraitConst::initial_lm_up_factor(self))
				.field("initial_lm_down_factor", &crate::geometry::LevMarq_SettingsTraitConst::initial_lm_down_factor(self))
				.finish()
		}
	}

	impl crate::geometry::LevMarq_SettingsTraitConst for LevMarq_Settings {
		#[inline] fn as_raw_LevMarq_Settings(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::geometry::LevMarq_SettingsTrait for LevMarq_Settings {
		#[inline] fn as_raw_mut_LevMarq_Settings(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LevMarq_Settings, crate::geometry::LevMarq_SettingsTraitConst, as_raw_LevMarq_Settings, crate::geometry::LevMarq_SettingsTrait, as_raw_mut_LevMarq_Settings }

	/// Region Growing algorithm in 3D point cloud.
	///
	/// The key idea of region growing is to merge the nearest neighbor points that satisfy a certain
	/// angle threshold into the same region according to the normal between the two points, so as to
	/// achieve the purpose of segmentation. For more details, please refer to [Rabbani2006SegmentationOP](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Rabbani2006SegmentationOP).
	pub struct RegionGrowing3D {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { RegionGrowing3D }

	impl Drop for RegionGrowing3D {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_RegionGrowing3D_delete(self.as_raw_mut_RegionGrowing3D()) };
		}
	}

	unsafe impl Send for RegionGrowing3D {}

	impl RegionGrowing3D {
		#[inline]
		pub fn create() -> Result<core::Ptr<crate::geometry::RegionGrowing3D>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::geometry::RegionGrowing3D>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::geometry::RegionGrowing3D]
	pub trait RegionGrowing3DTraitConst {
		fn as_raw_RegionGrowing3D(&self) -> *const c_void;

		/// Get the minimum size of region.
		#[inline]
		fn get_min_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_getMinSize_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the maximum size of region.
		#[inline]
		fn get_max_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_getMaxSize_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get whether to use the smoothness mode.
		#[inline]
		fn get_smooth_mode_flag(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_getSmoothModeFlag_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get threshold value of the angle between normals.
		#[inline]
		fn get_smoothness_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_getSmoothnessThreshold_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get threshold value of curvature.
		#[inline]
		fn get_curvature_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_getCurvatureThreshold_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the maximum number of neighbors including itself.
		#[inline]
		fn get_max_number_of_neighbors(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_getMaxNumberOfNeighbors_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the maximum number of regions you want.
		#[inline]
		fn get_number_of_regions(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_getNumberOfRegions_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get whether the results need to be sorted you have set.
		#[inline]
		fn get_need_sort(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_getNeedSort_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the seed points.
		#[inline]
		fn get_seeds(&self, seeds: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(seeds);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_getSeeds_const_const__OutputArrayR(self.as_raw_RegionGrowing3D(), seeds.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the curvature of each point if you have set.
		#[inline]
		fn get_curvatures(&self, curvatures: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(curvatures);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_getCurvatures_const_const__OutputArrayR(self.as_raw_RegionGrowing3D(), curvatures.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::geometry::RegionGrowing3D]
	pub trait RegionGrowing3DTrait: crate::geometry::RegionGrowing3DTraitConst {
		fn as_raw_mut_RegionGrowing3D(&mut self) -> *mut c_void;

		/// Execute segmentation using the Region Growing algorithm.
		///
		/// ## Parameters
		/// * regions_idx:[out] Index information of all points in each region, support
		///               vector<vector<int>>, vector<Mat>.
		/// * labels:[out] The label corresponds to the model number, 0 means it does not belong to
		///               any model, range [0, Number of final resultant models obtained]. Support
		///               vector<int> and Mat.
		/// * input_pts: Original point cloud, support vector<Point3f> and Mat of size Nx3/3xN.
		/// * normals: Normal of each point, support vector<Point3f> and Mat of size Nx3.
		/// * nn_idx: Index information of nearest neighbors of all points. The first nearest
		///               neighbor of each point is itself. Support vector<vector<int>>, vector<Mat> and
		///               Mat of size NxK. If the information in a row is
		///               [0, 2, 1, -5, -1, 4, 7 ... negative number]
		///               it will use only non-negative indexes until it meets a negative number or bound
		///               of this row i.e. [0, 2, 1].
		/// ## Returns
		/// Number of final resultant regions obtained by segmentation.
		#[inline]
		fn segment(&mut self, regions_idx: &mut impl ToOutputArray, labels: &mut impl ToOutputArray, input_pts: &impl ToInputArray, normals: &impl ToInputArray, nn_idx: &impl ToInputArray) -> Result<i32> {
			output_array_arg!(regions_idx);
			output_array_arg!(labels);
			input_array_arg!(input_pts);
			input_array_arg!(normals);
			input_array_arg!(nn_idx);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_segment_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_RegionGrowing3D(), regions_idx.as_raw__OutputArray(), labels.as_raw__OutputArray(), input_pts.as_raw__InputArray(), normals.as_raw__InputArray(), nn_idx.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the minimum size of region.
		#[inline]
		fn set_min_size(&mut self, min_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_setMinSize_int(self.as_raw_mut_RegionGrowing3D(), min_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the maximum size of region.
		#[inline]
		fn set_max_size(&mut self, max_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_setMaxSize_int(self.as_raw_mut_RegionGrowing3D(), max_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set whether to use the smoothness mode. Default will be true.
		/// If true it will check the angle between the normal of the current point and the normal of its neighbor.
		/// Otherwise, it will check the angle between the normal of the seed point and the normal of current neighbor.
		#[inline]
		fn set_smooth_mode_flag(&mut self, smooth_mode: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_setSmoothModeFlag_bool(self.as_raw_mut_RegionGrowing3D(), smooth_mode, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set threshold value of the angle between normals, the input value is in radian.
		#[inline]
		fn set_smoothness_threshold(&mut self, smoothness_thr: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_setSmoothnessThreshold_double(self.as_raw_mut_RegionGrowing3D(), smoothness_thr, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set threshold value of curvature. Default will be 0.05.
		/// Only points with curvature less than the threshold will be considered to belong to the same region.
		/// If the curvature of each point is not set, this option will not work.
		#[inline]
		fn set_curvature_threshold(&mut self, curvature_thr: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_setCurvatureThreshold_double(self.as_raw_mut_RegionGrowing3D(), curvature_thr, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the maximum number of neighbors want to use including itself.
		/// Setting to a non-positive number or default will use the information from nn_idx.
		#[inline]
		fn set_max_number_of_neighbors(&mut self, max_neighbor_num: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_setMaxNumberOfNeighbors_int(self.as_raw_mut_RegionGrowing3D(), max_neighbor_num, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the maximum number of regions you want.
		#[inline]
		fn set_number_of_regions(&mut self, region_num: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_setNumberOfRegions_int(self.as_raw_mut_RegionGrowing3D(), region_num, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set whether the results need to be sorted in descending order by the number of points.
		#[inline]
		fn set_need_sort(&mut self, need_sort: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_setNeedSort_bool(self.as_raw_mut_RegionGrowing3D(), need_sort, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the seed points, it will grow according to the seeds.
		/// If noArray() is set, the default method will be used:
		/// 1. If the curvature of each point is set, the seeds will be sorted in ascending order of curvatures.
		/// 2. Otherwise, the natural order of the point cloud will be used.
		#[inline]
		fn set_seeds(&mut self, seeds: &impl ToInputArray) -> Result<()> {
			input_array_arg!(seeds);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_setSeeds_const__InputArrayR(self.as_raw_mut_RegionGrowing3D(), seeds.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the curvature of each point, support vector<float> and Mat. If not, you can set it to noArray().
		#[inline]
		fn set_curvatures(&mut self, curvatures: &impl ToInputArray) -> Result<()> {
			input_array_arg!(curvatures);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RegionGrowing3D_setCurvatures_const__InputArrayR(self.as_raw_mut_RegionGrowing3D(), curvatures.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for RegionGrowing3D {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("RegionGrowing3D")
				.finish()
		}
	}

	impl crate::geometry::RegionGrowing3DTraitConst for RegionGrowing3D {
		#[inline] fn as_raw_RegionGrowing3D(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::geometry::RegionGrowing3DTrait for RegionGrowing3D {
		#[inline] fn as_raw_mut_RegionGrowing3D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { RegionGrowing3D, crate::geometry::RegionGrowing3DTraitConst, as_raw_RegionGrowing3D, crate::geometry::RegionGrowing3DTrait, as_raw_mut_RegionGrowing3D }

	/// Sample Consensus algorithm segmentation of 3D point cloud model.
	///
	/// Example of segmenting plane from a 3D point cloud using the RANSAC algorithm:
	/// [planeSegmentationUsingRANSAC](https://github.com/opencv/opencv/blob/5.0.0/samples/cpp/tutorial_code/snippets/3d_sac_segmentation.cpp#L1)
	/// ## See also
	/// 1. Supported algorithms: enum SacMethod in ptcloud.hpp.
	/// 2. Supported models: enum SacModelType in ptcloud.hpp.
	pub struct SACSegmentation {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { SACSegmentation }

	impl Drop for SACSegmentation {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_SACSegmentation_delete(self.as_raw_mut_SACSegmentation()) };
		}
	}

	unsafe impl Send for SACSegmentation {}

	impl SACSegmentation {
		/// ## C++ default parameters
		/// * sac_model_type: SAC_MODEL_PLANE
		/// * sac_method: SAC_METHOD_RANSAC
		/// * threshold: 0.5
		/// * max_iterations: 1000
		#[inline]
		pub fn create(sac_model_type: crate::geometry::SacModelType, sac_method: crate::geometry::SacMethod, threshold: f64, max_iterations: i32) -> Result<core::Ptr<crate::geometry::SACSegmentation>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_create_SacModelType_SacMethod_double_int(sac_model_type, sac_method, threshold, max_iterations, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::geometry::SACSegmentation>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [SACSegmentation::create] function uses the following default values for its arguments:
		/// * sac_model_type: SAC_MODEL_PLANE
		/// * sac_method: SAC_METHOD_RANSAC
		/// * threshold: 0.5
		/// * max_iterations: 1000
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::geometry::SACSegmentation>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::geometry::SACSegmentation>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::geometry::SACSegmentation]
	pub trait SACSegmentationTraitConst {
		fn as_raw_SACSegmentation(&self) -> *const c_void;

		/// Get the type of sample consensus model used.
		#[inline]
		fn get_sac_model_type(&self) -> Result<crate::geometry::SacModelType> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_getSacModelType_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the type of sample consensus method used.
		#[inline]
		fn get_sac_method_type(&self) -> Result<crate::geometry::SacMethod> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_getSacMethodType_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the distance to the model threshold.
		#[inline]
		fn get_distance_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_getDistanceThreshold_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the minimum and maximum radius limits for the model.
		#[inline]
		fn get_radius_limits(&self, radius_min: &mut f64, radius_max: &mut f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_getRadiusLimits_const_doubleR_doubleR(self.as_raw_SACSegmentation(), radius_min, radius_max, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the maximum number of iterations to attempt.
		#[inline]
		fn get_max_iterations(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_getMaxIterations_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the confidence that ensure at least one of selections is an error-free set of data points.
		#[inline]
		fn get_confidence(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_getConfidence_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the expected number of models.
		#[inline]
		fn get_number_of_models_expected(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_getNumberOfModelsExpected_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get whether to use parallelism or not.
		#[inline]
		fn is_parallel(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_isParallel_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get state used to initialize the RNG(Random Number Generator).
		#[inline]
		fn get_random_generator_state(&self) -> Result<u64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_getRandomGeneratorState_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::geometry::SACSegmentation]
	pub trait SACSegmentationTrait: crate::geometry::SACSegmentationTraitConst {
		fn as_raw_mut_SACSegmentation(&mut self) -> *mut c_void;

		/// Execute segmentation using the sample consensus method.
		///
		/// ## Parameters
		/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
		/// * labels:[out] The label corresponds to the model number, 0 means it
		/// does not belong to any model, range [0, Number of final resultant models obtained].
		/// * models_coefficients:[out] The resultant models coefficients.
		/// Currently supports passing in cv::Mat. Models coefficients are placed in a matrix of NxK
		/// with depth CV_64F (will automatically adjust if the passing one does not look like this),
		/// where N is the number of models and K is the number of coefficients of one model.
		/// The coefficients for each model refer to the comments inside enumeration type SacModelType.
		/// ## Returns
		/// Number of final resultant models obtained by segmentation.
		#[inline]
		fn segment(&mut self, input_pts: &impl ToInputArray, labels: &mut impl ToOutputArray, models_coefficients: &mut impl ToOutputArray) -> Result<i32> {
			input_array_arg!(input_pts);
			output_array_arg!(labels);
			output_array_arg!(models_coefficients);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_segment_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SACSegmentation(), input_pts.as_raw__InputArray(), labels.as_raw__OutputArray(), models_coefficients.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the type of sample consensus model to use.
		#[inline]
		fn set_sac_model_type(&mut self, sac_model_type: crate::geometry::SacModelType) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_setSacModelType_SacModelType(self.as_raw_mut_SACSegmentation(), sac_model_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the type of sample consensus method to use.
		#[inline]
		fn set_sac_method_type(&mut self, sac_method: crate::geometry::SacMethod) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_setSacMethodType_SacMethod(self.as_raw_mut_SACSegmentation(), sac_method, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the distance to the model threshold.
		/// Considered as inlier point if distance to the model less than threshold.
		#[inline]
		fn set_distance_threshold(&mut self, threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_setDistanceThreshold_double(self.as_raw_mut_SACSegmentation(), threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the minimum and maximum radius limits for the model.
		/// Only used for models whose model parameters include a radius.
		#[inline]
		fn set_radius_limits(&mut self, radius_min: f64, radius_max: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_setRadiusLimits_double_double(self.as_raw_mut_SACSegmentation(), radius_min, radius_max, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the maximum number of iterations to attempt.
		#[inline]
		fn set_max_iterations(&mut self, max_iterations: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_setMaxIterations_int(self.as_raw_mut_SACSegmentation(), max_iterations, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the confidence that ensure at least one of selections is an error-free set of data points.
		#[inline]
		fn set_confidence(&mut self, confidence: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_setConfidence_double(self.as_raw_mut_SACSegmentation(), confidence, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the number of models expected.
		#[inline]
		fn set_number_of_models_expected(&mut self, number_of_models_expected: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_setNumberOfModelsExpected_int(self.as_raw_mut_SACSegmentation(), number_of_models_expected, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set whether to use parallelism or not.
		/// The number of threads is set by cv::setNumThreads(int nthreads).
		#[inline]
		fn set_parallel(&mut self, is_parallel: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_setParallel_bool(self.as_raw_mut_SACSegmentation(), is_parallel, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set state used to initialize the RNG(Random Number Generator).
		#[inline]
		fn set_random_generator_state(&mut self, rng_state: u64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SACSegmentation_setRandomGeneratorState_uint64_t(self.as_raw_mut_SACSegmentation(), rng_state, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for SACSegmentation {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SACSegmentation")
				.finish()
		}
	}

	impl crate::geometry::SACSegmentationTraitConst for SACSegmentation {
		#[inline] fn as_raw_SACSegmentation(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::geometry::SACSegmentationTrait for SACSegmentation {
		#[inline] fn as_raw_mut_SACSegmentation(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { SACSegmentation, crate::geometry::SACSegmentationTraitConst, as_raw_SACSegmentation, crate::geometry::SACSegmentationTrait, as_raw_mut_SACSegmentation }

	pub struct Subdiv2D {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Subdiv2D }

	impl Drop for Subdiv2D {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_Subdiv2D_delete(self.as_raw_mut_Subdiv2D()) };
		}
	}

	unsafe impl Send for Subdiv2D {}

	impl Subdiv2D {
		/// creates an empty Subdiv2D object.
		/// To create a new empty Delaunay subdivision you need to use the [init_delaunay] function.
		#[inline]
		pub fn default() -> Result<crate::geometry::Subdiv2D> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_Subdiv2D(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::Subdiv2D::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// creates an empty Subdiv2D object.
		/// To create a new empty Delaunay subdivision you need to use the [init_delaunay] function.
		///
		/// ## Overloaded parameters
		///
		///
		/// ## Parameters
		/// * rect: Rectangle that includes all of the 2D points that are to be added to the subdivision.
		///
		/// The function creates an empty Delaunay subdivision where 2D points can be added using the function
		/// insert() . All of the points to be added must be within the specified rectangle, otherwise a runtime
		/// error is raised.
		#[inline]
		pub fn new(rect: core::Rect) -> Result<crate::geometry::Subdiv2D> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_Subdiv2D_Rect(&rect, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::Subdiv2D::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// creates an empty Subdiv2D object.
		/// To create a new empty Delaunay subdivision you need to use the [init_delaunay] function.
		///
		/// ## Overloaded parameters
		#[inline]
		pub fn new_1(rect2f: core::Rect2f) -> Result<crate::geometry::Subdiv2D> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_Subdiv2D_Rect2f(&rect2f, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::geometry::Subdiv2D::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::geometry::Subdiv2D]
	pub trait Subdiv2DTraitConst {
		fn as_raw_Subdiv2D(&self) -> *const c_void;

		/// Returns a list of all edges.
		///
		/// ## Parameters
		/// * edgeList: Output vector.
		///
		/// The function gives each edge as a 4 numbers vector, where each two are one of the edge
		/// vertices. i.e. org_x = v[0], org_y = v[1], dst_x = v[2], dst_y = v[3].
		#[inline]
		fn get_edge_list(&self, edge_list: &mut core::Vector<core::Vec4f>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_getEdgeList_const_vectorLVec4fGR(self.as_raw_Subdiv2D(), edge_list.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns a list of the leading edge ID connected to each triangle.
		///
		/// ## Parameters
		/// * leadingEdgeList: Output vector.
		///
		/// The function gives one edge ID for each triangle.
		#[inline]
		fn get_leading_edge_list(&self, leading_edge_list: &mut core::Vector<i32>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_getLeadingEdgeList_const_vectorLintGR(self.as_raw_Subdiv2D(), leading_edge_list.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns a list of all triangles.
		///
		/// ## Parameters
		/// * triangleList: Output vector.
		///
		/// The function gives each triangle as a 6 numbers vector, where each two are one of the triangle
		/// vertices. i.e. p1_x = v[0], p1_y = v[1], p2_x = v[2], p2_y = v[3], p3_x = v[4], p3_y = v[5].
		#[inline]
		fn get_triangle_list(&self, triangle_list: &mut core::Vector<core::Vec6f>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_getTriangleList_const_vectorLVec6fGR(self.as_raw_Subdiv2D(), triangle_list.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns vertex location from vertex ID.
		///
		/// ## Parameters
		/// * vertex: vertex ID.
		/// * firstEdge: Optional. The first edge ID which is connected to the vertex.
		/// ## Returns
		/// vertex (x,y)
		///
		/// ## C++ default parameters
		/// * first_edge: 0
		#[inline]
		fn get_vertex(&self, vertex: i32, first_edge: &mut i32) -> Result<core::Point2f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_getVertex_const_int_intX(self.as_raw_Subdiv2D(), vertex, first_edge, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns vertex location from vertex ID.
		///
		/// ## Parameters
		/// * vertex: vertex ID.
		/// * firstEdge: Optional. The first edge ID which is connected to the vertex.
		/// ## Returns
		/// vertex (x,y)
		///
		/// ## Note
		/// This alternative version of [Subdiv2DTraitConst::get_vertex] function uses the following default values for its arguments:
		/// * first_edge: 0
		#[inline]
		fn get_vertex_def(&self, vertex: i32) -> Result<core::Point2f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_getVertex_const_int(self.as_raw_Subdiv2D(), vertex, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ![sample output](https://docs.opencv.org/5.0.0/quadedge.png)
		///
		/// ## Returns
		/// edge ID related to the input edge.
		#[inline]
		fn get_edge(&self, edge: i32, next_edge_type: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_getEdge_const_int_int(self.as_raw_Subdiv2D(), edge, next_edge_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns next edge around the edge origin.
		///
		/// ## Parameters
		/// * edge: Subdivision edge ID.
		///
		/// ## Returns
		/// an integer which is next edge ID around the edge origin: eOnext on the
		/// picture above if e is the input edge).
		#[inline]
		fn next_edge(&self, edge: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_nextEdge_const_int(self.as_raw_Subdiv2D(), edge, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Returns
		/// one of the edges ID of the same quad-edge as the input edge.
		#[inline]
		fn rotate_edge(&self, edge: i32, rotate: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_rotateEdge_const_int_int(self.as_raw_Subdiv2D(), edge, rotate, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn sym_edge(&self, edge: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_symEdge_const_int(self.as_raw_Subdiv2D(), edge, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns the edge origin.
		///
		/// ## Parameters
		/// * edge: Subdivision edge ID.
		/// * orgpt: Output vertex location.
		///
		/// ## Returns
		/// vertex ID.
		///
		/// ## C++ default parameters
		/// * orgpt: 0
		#[inline]
		fn edge_org(&self, edge: i32, orgpt: &mut core::Point2f) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_edgeOrg_const_int_Point2fX(self.as_raw_Subdiv2D(), edge, orgpt, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns the edge origin.
		///
		/// ## Parameters
		/// * edge: Subdivision edge ID.
		/// * orgpt: Output vertex location.
		///
		/// ## Returns
		/// vertex ID.
		///
		/// ## Note
		/// This alternative version of [Subdiv2DTraitConst::edge_org] function uses the following default values for its arguments:
		/// * orgpt: 0
		#[inline]
		fn edge_org_def(&self, edge: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_edgeOrg_const_int(self.as_raw_Subdiv2D(), edge, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns the edge destination.
		///
		/// ## Parameters
		/// * edge: Subdivision edge ID.
		/// * dstpt: Output vertex location.
		///
		/// ## Returns
		/// vertex ID.
		///
		/// ## C++ default parameters
		/// * dstpt: 0
		#[inline]
		fn edge_dst(&self, edge: i32, dstpt: &mut core::Point2f) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_edgeDst_const_int_Point2fX(self.as_raw_Subdiv2D(), edge, dstpt, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns the edge destination.
		///
		/// ## Parameters
		/// * edge: Subdivision edge ID.
		/// * dstpt: Output vertex location.
		///
		/// ## Returns
		/// vertex ID.
		///
		/// ## Note
		/// This alternative version of [Subdiv2DTraitConst::edge_dst] function uses the following default values for its arguments:
		/// * dstpt: 0
		#[inline]
		fn edge_dst_def(&self, edge: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_edgeDst_const_int(self.as_raw_Subdiv2D(), edge, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::geometry::Subdiv2D]
	pub trait Subdiv2DTrait: crate::geometry::Subdiv2DTraitConst {
		fn as_raw_mut_Subdiv2D(&mut self) -> *mut c_void;

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		///
		/// Creates a new empty Delaunay subdivision
		///
		/// ## Parameters
		/// * rect: Rectangle that includes all of the 2D points that are to be added to the subdivision.
		#[inline]
		fn init_delaunay(&mut self, rect: core::Rect) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_initDelaunay_Rect(self.as_raw_mut_Subdiv2D(), &rect, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		///
		/// Creates a new empty Delaunay subdivision
		///
		/// ## Parameters
		/// * rect: Rectangle that includes all of the 2d points that are to be added to the subdivision.
		#[inline]
		fn init_delaunay2f(&mut self, rect: core::Rect2f) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_initDelaunay_Rect2f(self.as_raw_mut_Subdiv2D(), &rect, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Insert a single point into a Delaunay triangulation.
		///
		/// ## Parameters
		/// * pt: Point to insert.
		///
		/// The function inserts a single point into a subdivision and modifies the subdivision topology
		/// appropriately. If a point with the same coordinates exists already, no new point is added.
		/// ## Returns
		/// the ID of the point.
		///
		///
		/// Note: If the point is outside of the triangulation specified rect a runtime error is raised.
		#[inline]
		fn insert(&mut self, pt: core::Point2f) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_insert_Point2f(self.as_raw_mut_Subdiv2D(), &pt, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Insert multiple points into a Delaunay triangulation.
		///
		/// ## Parameters
		/// * ptvec: Points to insert.
		///
		/// The function inserts a vector of points into a subdivision and modifies the subdivision topology
		/// appropriately.
		#[inline]
		fn insert_multiple(&mut self, ptvec: &core::Vector<core::Point2f>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_insert_const_vectorLPoint2fGR(self.as_raw_mut_Subdiv2D(), ptvec.as_raw_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Returns
		/// an integer which specify one of the following five cases for point location:
		/// *  The point falls into some facet. The function returns [PTLOC_INSIDE] and edge will contain one of
		///    edges of the facet.
		/// *  The point falls onto the edge. The function returns [PTLOC_ON_EDGE] and edge will contain this edge.
		/// *  The point coincides with one of the subdivision vertices. The function returns [PTLOC_VERTEX] and
		///    vertex will contain a pointer to the vertex.
		/// *  The point is outside the subdivision reference rectangle. The function returns [PTLOC_OUTSIDE_RECT]
		///    and no pointers are filled.
		/// *  One of input arguments is invalid. A runtime error is raised or, if silent or "parent" error
		///    processing mode is selected, [PTLOC_ERROR] is returned.
		#[inline]
		fn locate(&mut self, pt: core::Point2f, edge: &mut i32, vertex: &mut i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_locate_Point2f_intR_intR(self.as_raw_mut_Subdiv2D(), &pt, edge, vertex, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Finds the subdivision vertex closest to the given point.
		///
		/// ## Parameters
		/// * pt: Input point.
		/// * nearestPt: Output subdivision vertex point.
		///
		/// The function is another function that locates the input point within the subdivision. It finds the
		/// subdivision vertex that is the closest to the input point. It is not necessarily one of vertices
		/// of the facet containing the input point, though the facet (located using locate() ) is used as a
		/// starting point.
		///
		/// ## Returns
		/// vertex ID.
		///
		/// ## C++ default parameters
		/// * nearest_pt: 0
		#[inline]
		fn find_nearest(&mut self, pt: core::Point2f, nearest_pt: &mut core::Point2f) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_findNearest_Point2f_Point2fX(self.as_raw_mut_Subdiv2D(), &pt, nearest_pt, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Finds the subdivision vertex closest to the given point.
		///
		/// ## Parameters
		/// * pt: Input point.
		/// * nearestPt: Output subdivision vertex point.
		///
		/// The function is another function that locates the input point within the subdivision. It finds the
		/// subdivision vertex that is the closest to the input point. It is not necessarily one of vertices
		/// of the facet containing the input point, though the facet (located using locate() ) is used as a
		/// starting point.
		///
		/// ## Returns
		/// vertex ID.
		///
		/// ## Note
		/// This alternative version of [Subdiv2DTrait::find_nearest] function uses the following default values for its arguments:
		/// * nearest_pt: 0
		#[inline]
		fn find_nearest_def(&mut self, pt: core::Point2f) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_findNearest_Point2f(self.as_raw_mut_Subdiv2D(), &pt, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns a list of all Voronoi facets.
		///
		/// ## Parameters
		/// * idx: Vector of vertices IDs to consider. For all vertices you can pass empty vector.
		/// * facetList: Output vector of the Voronoi facets.
		/// * facetCenters: Output vector of the Voronoi facets center points.
		#[inline]
		fn get_voronoi_facet_list(&mut self, idx: &core::Vector<i32>, facet_list: &mut core::Vector<core::Vector<core::Point2f>>, facet_centers: &mut core::Vector<core::Point2f>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Subdiv2D_getVoronoiFacetList_const_vectorLintGR_vectorLvectorLPoint2fGGR_vectorLPoint2fGR(self.as_raw_mut_Subdiv2D(), idx.as_raw_VectorOfi32(), facet_list.as_raw_mut_VectorOfVectorOfPoint2f(), facet_centers.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Subdiv2D {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Subdiv2D")
				.finish()
		}
	}

	impl crate::geometry::Subdiv2DTraitConst for Subdiv2D {
		#[inline] fn as_raw_Subdiv2D(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::geometry::Subdiv2DTrait for Subdiv2D {
		#[inline] fn as_raw_mut_Subdiv2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Subdiv2D, crate::geometry::Subdiv2DTraitConst, as_raw_Subdiv2D, crate::geometry::Subdiv2DTrait, as_raw_mut_Subdiv2D }

	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct UsacParams {
		pub confidence: f64,
		pub is_parallel: bool,
		pub lo_iterations: i32,
		pub lo_method: crate::geometry::LocalOptimMethod,
		pub lo_sample_size: i32,
		pub max_iterations: i32,
		pub neighbors_search: crate::geometry::NeighborSearchMethod,
		pub random_generator_state: i32,
		pub sampler: crate::geometry::SamplingMethod,
		pub score: crate::geometry::ScoreMethod,
		pub threshold: f64,
		pub final_polisher: crate::geometry::PolishingMethod,
		pub final_polisher_iterations: i32,
	}

	opencv_type_simple! { crate::geometry::UsacParams }

	impl UsacParams {
		#[inline]
		pub fn default() -> Result<crate::geometry::UsacParams> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_UsacParams_UsacParams(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

}

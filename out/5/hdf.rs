//! # Hierarchical Data Format I/O routines
//!
//! This module provides storage routines for Hierarchical Data Format objects.
//!    # Hierarchical Data Format version 5
//!
//!    Hierarchical Data Format version 5
//!    --------------------------------------------------------
//!
//!    In order to use it, the hdf5 library has to be installed, which
//!    means cmake should find it using `find_package(HDF5)`.
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{HDF5Trait, HDF5TraitConst};
}

/// Get the chunk sizes of a dataset. see also: dsgetsize()
// H5_GETCHUNKDIMS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:66
pub const HDF5_H5_GETCHUNKDIMS: i32 = 102;
/// Get the dimension information of a dataset. see also: dsgetsize()
// H5_GETDIMS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:64
pub const HDF5_H5_GETDIMS: i32 = 100;
/// Get the maximum dimension information of a dataset. see also: dsgetsize()
// H5_GETMAXDIMS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:65
pub const HDF5_H5_GETMAXDIMS: i32 = 101;
/// No compression, see also: dscreate()
// H5_NONE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:63
pub const HDF5_H5_NONE: i32 = -1;
/// The dimension size is unlimited, see also: dscreate()
// H5_UNLIMITED /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:62
pub const HDF5_H5_UNLIMITED: i32 = -1;
/// Open or create hdf5 file
/// ## Parameters
/// * HDF5Filename: specify the HDF5 filename.
///
/// Returns a pointer to the hdf5 object class
///
///
/// Note: If the specified file does not exist, it will be created using default properties.
/// Otherwise, it is opened in read and write mode with default access properties.
/// Any operations except dscreate() functions on object
/// will be thread safe. Multiple datasets can be created inside a single hdf5 file, and can be accessed
/// from the same hdf5 object from multiple instances as long read or write operations are done over
/// non-overlapping regions of dataset. Single hdf5 file also can be opened by multiple instances,
/// reads and writes can be instantiated at the same time as long as non-overlapping regions are involved. Object
/// is released using close().
///
/// - Example below opens and then releases the file.
/// ```C++
///   // open / auto create hdf5 file
///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
///   // ...
///   // release
///   h5io->close();
/// ```
///
///
/// ![Visualization of 10x10 CV_64FC2 (Hilbert matrix) using HDFView tool](https://docs.opencv.org/5.0.0/hdfview_demo.gif)
///
/// - Text dump (3x3 Hilbert matrix) of hdf5 dataset using **h5dump** tool:
/// ```C++
/// $ h5dump test.h5
/// HDF5 "test.h5" {
/// GROUP "/" {
///    DATASET "hilbert" {
///       DATATYPE  H5T_ARRAY { [2] H5T_IEEE_F64LE }
///       DATASPACE  SIMPLE { ( 3, 3 ) / ( 3, 3 ) }
///       DATA {
///       (0,0): [ 1, -1 ], [ 0.5, -0.5 ], [ 0.333333, -0.333333 ],
///       (1,0): [ 0.5, -0.5 ], [ 0.333333, -0.333333 ], [ 0.25, -0.25 ],
///       (2,0): [ 0.333333, -0.333333 ], [ 0.25, -0.25 ], [ 0.2, -0.2 ]
///       }
///    }
/// }
/// }
/// ```
///
// open(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:802
// ("cv::hdf::open", vec![(pred!(mut, ["HDF5Filename"], ["const cv::String*"]), _)]),
#[inline]
pub fn open(hdf5_filename: &str) -> Result<core::Ptr<crate::hdf::HDF5>> {
	extern_container_arg!(hdf5_filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_hdf_open_const_StringR(hdf5_filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::hdf::HDF5>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Constant methods for [crate::hdf::HDF5]
// HDF5 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:56
pub trait HDF5TraitConst {
	fn as_raw_HDF5(&self) -> *const c_void;

	/// Check if label exists or not.
	/// ## Parameters
	/// * label: specify the hdf5 dataset label.
	///
	/// Returns **true** if dataset exists, and **false** otherwise.
	///
	///
	/// Note: Checks if dataset, group or other object type (hdf5 link) exists under the label name. It is thread safe.
	// hlexists(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:104
	// ("cv::hdf::HDF5::hlexists", vec![(pred!(const, ["label"], ["const cv::String*"]), _)]),
	#[inline]
	fn hlexists(&self, label: &str) -> Result<bool> {
		extern_container_arg!(label);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_hlexists_const_const_StringR(self.as_raw_HDF5(), label.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Check whether a given attribute exits or not in the root group.
	///
	/// ## Parameters
	/// * atlabel: the attribute name to be checked.
	/// ## Returns
	/// true if the attribute exists, false otherwise.
	/// ## See also
	/// atdelete, atwrite, atread
	// atexists(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:114
	// ("cv::hdf::HDF5::atexists", vec![(pred!(const, ["atlabel"], ["const cv::String*"]), _)]),
	#[inline]
	fn atexists(&self, atlabel: &str) -> Result<bool> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atexists_const_const_StringR(self.as_raw_HDF5(), atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Create and allocate storage for two dimensional single or multi channel dataset.
	/// ## Parameters
	/// * rows: declare amount of rows
	/// * cols: declare amount of columns
	/// * type: type to be used, e.g, CV_8UC3, CV_32FC1 and etc.
	/// * dslabel: specify the hdf5 dataset label. Existing dataset label will cause an error.
	/// * compresslevel: specify the compression level 0-9 to be used, H5_NONE is the default value and means no compression.
	///                      The value 0 also means no compression.
	///                      A value 9 indicating the best compression ration. Note
	///                      that a higher compression level indicates a higher computational cost. It relies
	///                      on GNU gzip for compression.
	/// * dims_chunks: each array member specifies the chunking size to be used for block I/O,
	///        by default NULL means none at all.
	///
	///
	/// Note: If the dataset already exists, an exception will be thrown (CV_Error() is called).
	///
	/// - Existence of the dataset can be checked using hlexists(), see in this example:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create space for 100x50 CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "hilbert" ) )
	///    h5io->dscreate( 100, 50, CV_64FC2, "hilbert" );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	///
	/// Note: Activating compression requires internal chunking. Chunking can significantly improve access
	/// speed both at read and write time, especially for windowed access logic that shifts offset inside dataset.
	/// If no custom chunking is specified, the default one will be invoked by the size of the **whole** dataset
	/// as a single big chunk of data.
	///
	/// - See example of level 9 compression using internal default chunking:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create level 9 compressed space for CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "hilbert", 9 ) )
	///    h5io->dscreate( 100, 50, CV_64FC2, "hilbert", 9 );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	///
	/// Note: A value of H5_UNLIMITED for **rows** or **cols** or both means **unlimited** data on the specified dimension,
	/// thus, it is possible to expand anytime such a dataset on row, col or on both directions. Presence of H5_UNLIMITED on any
	/// dimension **requires** to define custom chunking. No default chunking will be defined in the unlimited scenario since
	/// default size on that dimension will be zero, and will grow once dataset is written. Writing into a dataset that has
	/// H5_UNLIMITED on some of its dimensions requires dsinsert() that allows growth on unlimited dimensions, instead of dswrite()
	/// that allows to write only in predefined data space.
	///
	/// - Example below shows no compression but unlimited dimension on cols using 100x100 internal chunking:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create level 9 compressed space for CV_64FC2 matrix
	///   int chunks[2] = { 100, 100 };
	///   h5io->dscreate( 100, cv::hdf::HDF5::H5_UNLIMITED, CV_64FC2, "hilbert", cv::hdf::HDF5::H5_NONE, chunks );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	///
	/// Note: It is **not** thread safe, it must be called only once at dataset creation, otherwise an exception will occur.
	/// Multiple datasets inside a single hdf5 file are allowed.
	///
	/// ## Overloaded parameters
	// dscreate(const int, const int, const int, const String &)(Primitive, Primitive, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:203
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["rows", "cols", "type", "dslabel"], ["const int", "const int", "const int", "const cv::String*"]), _)]),
	#[inline]
	fn dscreate(&self, rows: i32, cols: i32, typ: i32, dslabel: &str) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR(self.as_raw_HDF5(), rows, cols, typ, dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Create and allocate storage for two dimensional single or multi channel dataset.
	/// ## Parameters
	/// * rows: declare amount of rows
	/// * cols: declare amount of columns
	/// * type: type to be used, e.g, CV_8UC3, CV_32FC1 and etc.
	/// * dslabel: specify the hdf5 dataset label. Existing dataset label will cause an error.
	/// * compresslevel: specify the compression level 0-9 to be used, H5_NONE is the default value and means no compression.
	///                      The value 0 also means no compression.
	///                      A value 9 indicating the best compression ration. Note
	///                      that a higher compression level indicates a higher computational cost. It relies
	///                      on GNU gzip for compression.
	/// * dims_chunks: each array member specifies the chunking size to be used for block I/O,
	///        by default NULL means none at all.
	///
	///
	/// Note: If the dataset already exists, an exception will be thrown (CV_Error() is called).
	///
	/// - Existence of the dataset can be checked using hlexists(), see in this example:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create space for 100x50 CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "hilbert" ) )
	///    h5io->dscreate( 100, 50, CV_64FC2, "hilbert" );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	///
	/// Note: Activating compression requires internal chunking. Chunking can significantly improve access
	/// speed both at read and write time, especially for windowed access logic that shifts offset inside dataset.
	/// If no custom chunking is specified, the default one will be invoked by the size of the **whole** dataset
	/// as a single big chunk of data.
	///
	/// - See example of level 9 compression using internal default chunking:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create level 9 compressed space for CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "hilbert", 9 ) )
	///    h5io->dscreate( 100, 50, CV_64FC2, "hilbert", 9 );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	///
	/// Note: A value of H5_UNLIMITED for **rows** or **cols** or both means **unlimited** data on the specified dimension,
	/// thus, it is possible to expand anytime such a dataset on row, col or on both directions. Presence of H5_UNLIMITED on any
	/// dimension **requires** to define custom chunking. No default chunking will be defined in the unlimited scenario since
	/// default size on that dimension will be zero, and will grow once dataset is written. Writing into a dataset that has
	/// H5_UNLIMITED on some of its dimensions requires dsinsert() that allows growth on unlimited dimensions, instead of dswrite()
	/// that allows to write only in predefined data space.
	///
	/// - Example below shows no compression but unlimited dimension on cols using 100x100 internal chunking:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create level 9 compressed space for CV_64FC2 matrix
	///   int chunks[2] = { 100, 100 };
	///   h5io->dscreate( 100, cv::hdf::HDF5::H5_UNLIMITED, CV_64FC2, "hilbert", cv::hdf::HDF5::H5_NONE, chunks );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	///
	/// Note: It is **not** thread safe, it must be called only once at dataset creation, otherwise an exception will occur.
	/// Multiple datasets inside a single hdf5 file are allowed.
	///
	/// ## Overloaded parameters
	// dscreate(const int, const int, const int, const String &, const int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:206
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["rows", "cols", "type", "dslabel", "compresslevel"], ["const int", "const int", "const int", "const cv::String*", "const int"]), _)]),
	#[inline]
	fn dscreate_compress(&self, rows: i32, cols: i32, typ: i32, dslabel: &str, compresslevel: i32) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int(self.as_raw_HDF5(), rows, cols, typ, dslabel.opencv_as_extern(), compresslevel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Create and allocate storage for two dimensional single or multi channel dataset.
	/// ## Parameters
	/// * rows: declare amount of rows
	/// * cols: declare amount of columns
	/// * type: type to be used, e.g, CV_8UC3, CV_32FC1 and etc.
	/// * dslabel: specify the hdf5 dataset label. Existing dataset label will cause an error.
	/// * compresslevel: specify the compression level 0-9 to be used, H5_NONE is the default value and means no compression.
	///                      The value 0 also means no compression.
	///                      A value 9 indicating the best compression ration. Note
	///                      that a higher compression level indicates a higher computational cost. It relies
	///                      on GNU gzip for compression.
	/// * dims_chunks: each array member specifies the chunking size to be used for block I/O,
	///        by default NULL means none at all.
	///
	///
	/// Note: If the dataset already exists, an exception will be thrown (CV_Error() is called).
	///
	/// - Existence of the dataset can be checked using hlexists(), see in this example:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create space for 100x50 CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "hilbert" ) )
	///    h5io->dscreate( 100, 50, CV_64FC2, "hilbert" );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	///
	/// Note: Activating compression requires internal chunking. Chunking can significantly improve access
	/// speed both at read and write time, especially for windowed access logic that shifts offset inside dataset.
	/// If no custom chunking is specified, the default one will be invoked by the size of the **whole** dataset
	/// as a single big chunk of data.
	///
	/// - See example of level 9 compression using internal default chunking:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create level 9 compressed space for CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "hilbert", 9 ) )
	///    h5io->dscreate( 100, 50, CV_64FC2, "hilbert", 9 );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	///
	/// Note: A value of H5_UNLIMITED for **rows** or **cols** or both means **unlimited** data on the specified dimension,
	/// thus, it is possible to expand anytime such a dataset on row, col or on both directions. Presence of H5_UNLIMITED on any
	/// dimension **requires** to define custom chunking. No default chunking will be defined in the unlimited scenario since
	/// default size on that dimension will be zero, and will grow once dataset is written. Writing into a dataset that has
	/// H5_UNLIMITED on some of its dimensions requires dsinsert() that allows growth on unlimited dimensions, instead of dswrite()
	/// that allows to write only in predefined data space.
	///
	/// - Example below shows no compression but unlimited dimension on cols using 100x100 internal chunking:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create level 9 compressed space for CV_64FC2 matrix
	///   int chunks[2] = { 100, 100 };
	///   h5io->dscreate( 100, cv::hdf::HDF5::H5_UNLIMITED, CV_64FC2, "hilbert", cv::hdf::HDF5::H5_NONE, chunks );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	///
	/// Note: It is **not** thread safe, it must be called only once at dataset creation, otherwise an exception will occur.
	/// Multiple datasets inside a single hdf5 file are allowed.
	///
	/// ## Overloaded parameters
	// dscreate(const int, const int, const int, const String &, const int, const vector<int> &)(Primitive, Primitive, Primitive, InString, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:209
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["rows", "cols", "type", "dslabel", "compresslevel", "dims_chunks"], ["const int", "const int", "const int", "const cv::String*", "const int", "const std::vector<int>*"]), _)]),
	#[inline]
	fn dscreate_compress_dims(&self, rows: i32, cols: i32, typ: i32, dslabel: &str, compresslevel: i32, dims_chunks: &core::Vector<i32>) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int_const_vectorLintGR(self.as_raw_HDF5(), rows, cols, typ, dslabel.opencv_as_extern(), compresslevel, dims_chunks.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// dscreate(const int, const int *, const int, const String &)(Primitive, VariableArray, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:282
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["n_dims", "sizes", "type", "dslabel"], ["const int", "const int*", "const int", "const cv::String*"]), _)]),
	#[inline]
	fn dscreate_nd(&self, sizes: &[i32], typ: i32, dslabel: &str) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR(self.as_raw_HDF5(), sizes.len().try_into()?, sizes.as_ptr(), typ, dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// dscreate(const int, const int *, const int, const String &, const int)(Primitive, VariableArray, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:285
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["n_dims", "sizes", "type", "dslabel", "compresslevel"], ["const int", "const int*", "const int", "const cv::String*", "const int"]), _)]),
	#[inline]
	fn dscreate_nd_compress(&self, sizes: &[i32], typ: i32, dslabel: &str, compresslevel: i32) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int(self.as_raw_HDF5(), sizes.len().try_into()?, sizes.as_ptr(), typ, dslabel.opencv_as_extern(), compresslevel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * compresslevel: HDF5::H5_NONE
	/// * dims_chunks: vector<int>()
	// dscreate(const vector<int> &, const int, const String &, const int, const vector<int> &)(CppPassByVoidPtr, Primitive, InString, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:288
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["sizes", "type", "dslabel", "compresslevel", "dims_chunks"], ["const std::vector<int>*", "const int", "const cv::String*", "const int", "const std::vector<int>*"]), _)]),
	#[inline]
	fn dscreate_nd_vec_compress_dims(&self, sizes: &core::Vector<i32>, typ: i32, dslabel: &str, compresslevel: i32, dims_chunks: &core::Vector<i32>) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_vectorLintGR_const_int_const_StringR_const_int_const_vectorLintGR(self.as_raw_HDF5(), sizes.as_raw_VectorOfi32(), typ, dslabel.opencv_as_extern(), compresslevel, dims_chunks.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [HDF5TraitConst::dscreate_nd_vec_compress_dims] function uses the following default values for its arguments:
	/// * compresslevel: HDF5::H5_NONE
	/// * dims_chunks: vector<int>()
	// cv::hdf::HDF5::dscreate(CppPassByVoidPtr, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:288
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["sizes", "type", "dslabel"], ["const std::vector<int>*", "const int", "const cv::String*"]), _)]),
	#[inline]
	fn dscreate_nd_vec_compress_dims_def(&self, sizes: &core::Vector<i32>, typ: i32, dslabel: &str) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_vectorLintGR_const_int_const_StringR(self.as_raw_HDF5(), sizes.as_raw_VectorOfi32(), typ, dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Create and allocate storage for n-dimensional dataset, single or multichannel type.
	/// ## Parameters
	/// * n_dims: declare number of dimensions
	/// * sizes: array containing sizes for each dimensions
	/// * type: type to be used, e.g., CV_8UC3, CV_32FC1, etc.
	/// * dslabel: specify the hdf5 dataset label. Existing dataset label will cause an error.
	/// * compresslevel: specify the compression level 0-9 to be used, H5_NONE is the default value and means no compression.
	///                      The value 0 also means no compression.
	///                      A value 9 indicating the best compression ration. Note
	///                      that a higher compression level indicates a higher computational cost. It relies
	///                      on GNU gzip for compression.
	/// * dims_chunks: each array member specifies chunking sizes to be used for block I/O,
	///        by default NULL means none at all.
	///
	/// Note: If the dataset already exists, an exception will be thrown. Existence of the dataset can be checked
	/// using hlexists().
	///
	/// - See example below that creates a 6 dimensional storage space:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create space for 6 dimensional CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "nddata" ) )
	///    int n_dims = 5;
	///    int dsdims[n_dims] = { 100, 100, 20, 10, 5, 5 };
	///    h5io->dscreate( n_dims, sizes, CV_64FC2, "nddata" );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	///
	/// Note: Activating compression requires internal chunking. Chunking can significantly improve access
	/// speed both at read and write time, especially for windowed access logic that shifts offset inside dataset.
	/// If no custom chunking is specified, the default one will be invoked by the size of **whole** dataset
	/// as single big chunk of data.
	///
	/// - See example of level 0 compression (shallow) using chunking against the first
	/// dimension, thus storage will consists of 100 chunks of data:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create space for 6 dimensional CV_64FC2 matrix
	///   if ( ! h5io->hlexists( "nddata" ) )
	///    int n_dims = 5;
	///    int dsdims[n_dims] = { 100, 100, 20, 10, 5, 5 };
	///    int chunks[n_dims] = {   1, 100, 20, 10, 5, 5 };
	///    h5io->dscreate( n_dims, dsdims, CV_64FC2, "nddata", 0, chunks );
	///   else
	///    printf("DS already created, skipping\n" );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	///
	/// Note: A value of H5_UNLIMITED inside the **sizes** array means **unlimited** data on that dimension, thus it is
	/// possible to expand anytime such dataset on those unlimited directions. Presence of H5_UNLIMITED on any dimension
	/// **requires** to define custom chunking. No default chunking will be defined in unlimited scenario since the default size
	/// on that dimension will be zero, and will grow once dataset is written. Writing into dataset that has H5_UNLIMITED on
	/// some of its dimension requires dsinsert() instead of dswrite() that allows growth on unlimited dimension instead of
	/// dswrite() that allows to write only in predefined data space.
	///
	/// - Example below shows a 3 dimensional dataset using no compression with all unlimited sizes and one unit chunking:
	/// ```C++
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   int n_dims = 3;
	///   int chunks[n_dims] = { 1, 1, 1 };
	///   int dsdims[n_dims] = { cv::hdf::HDF5::H5_UNLIMITED, cv::hdf::HDF5::H5_UNLIMITED, cv::hdf::HDF5::H5_UNLIMITED };
	///   h5io->dscreate( n_dims, dsdims, CV_64FC2, "nddata", cv::hdf::HDF5::H5_NONE, chunks );
	///   // release
	///   h5io->close();
	/// ```
	///
	// dscreate(const int, const int *, const int, const String &, const int, const int *)(Primitive, VariableArray, Primitive, InString, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:362
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["n_dims", "sizes", "type", "dslabel", "compresslevel", "dims_chunks"], ["const int", "const int*", "const int", "const cv::String*", "const int", "const int*"]), _)]),
	#[inline]
	fn dscreate_nd_compress_dims(&self, sizes: &[i32], typ: i32, dslabel: &str, compresslevel: i32, dims_chunks: &i32) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int_const_intX(self.as_raw_HDF5(), sizes.len().try_into()?, sizes.as_ptr(), typ, dslabel.opencv_as_extern(), compresslevel, dims_chunks, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Fetch dataset sizes
	/// ## Parameters
	/// * dslabel: specify the hdf5 dataset label to be measured.
	/// * dims_flag: will fetch dataset dimensions on H5_GETDIMS, dataset maximum dimensions on H5_GETMAXDIMS,
	///                  and chunk sizes on H5_GETCHUNKDIMS.
	///
	/// Returns vector object containing sizes of dataset on each dimensions.
	///
	///
	/// Note: Resulting vector size will match the amount of dataset dimensions. By default H5_GETDIMS will return
	/// actual dataset dimensions. Using H5_GETMAXDIM flag will get maximum allowed dimension which normally match
	/// actual dataset dimension but can hold H5_UNLIMITED value if dataset was prepared in **unlimited** mode on
	/// some of its dimension. It can be useful to check existing dataset dimensions before overwrite it as whole or subset.
	/// Trying to write with oversized source data into dataset target will thrown exception. The H5_GETCHUNKDIMS will
	/// return the dimension of chunk if dataset was created with chunking options otherwise returned vector size
	/// will be zero.
	///
	/// ## C++ default parameters
	/// * dims_flag: HDF5::H5_GETDIMS
	// dsgetsize(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:380
	// ("cv::hdf::HDF5::dsgetsize", vec![(pred!(const, ["dslabel", "dims_flag"], ["const cv::String*", "int"]), _)]),
	#[inline]
	fn dsgetsize(&self, dslabel: &str, dims_flag: i32) -> Result<core::Vector<i32>> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsgetsize_const_const_StringR_int(self.as_raw_HDF5(), dslabel.opencv_as_extern(), dims_flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Fetch dataset sizes
	/// ## Parameters
	/// * dslabel: specify the hdf5 dataset label to be measured.
	/// * dims_flag: will fetch dataset dimensions on H5_GETDIMS, dataset maximum dimensions on H5_GETMAXDIMS,
	///                  and chunk sizes on H5_GETCHUNKDIMS.
	///
	/// Returns vector object containing sizes of dataset on each dimensions.
	///
	///
	/// Note: Resulting vector size will match the amount of dataset dimensions. By default H5_GETDIMS will return
	/// actual dataset dimensions. Using H5_GETMAXDIM flag will get maximum allowed dimension which normally match
	/// actual dataset dimension but can hold H5_UNLIMITED value if dataset was prepared in **unlimited** mode on
	/// some of its dimension. It can be useful to check existing dataset dimensions before overwrite it as whole or subset.
	/// Trying to write with oversized source data into dataset target will thrown exception. The H5_GETCHUNKDIMS will
	/// return the dimension of chunk if dataset was created with chunking options otherwise returned vector size
	/// will be zero.
	///
	/// ## Note
	/// This alternative version of [HDF5TraitConst::dsgetsize] function uses the following default values for its arguments:
	/// * dims_flag: HDF5::H5_GETDIMS
	// cv::hdf::HDF5::dsgetsize(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:380
	// ("cv::hdf::HDF5::dsgetsize", vec![(pred!(const, ["dslabel"], ["const cv::String*"]), _)]),
	#[inline]
	fn dsgetsize_def(&self, dslabel: &str) -> Result<core::Vector<i32>> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsgetsize_const_const_StringR(self.as_raw_HDF5(), dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Fetch dataset type
	/// ## Parameters
	/// * dslabel: specify the hdf5 dataset label to be checked.
	///
	/// Returns the stored matrix type. This is an identifier compatible with the CvMat type system,
	/// like e.g. CV_16SC5 (16-bit signed 5-channel array), and so on.
	///
	///
	/// Note: Result can be parsed with CV_MAT_CN() to obtain amount of channels and CV_MAT_DEPTH() to obtain native cvdata type.
	/// It is thread safe.
	// dsgettype(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:391
	// ("cv::hdf::HDF5::dsgettype", vec![(pred!(const, ["dslabel"], ["const cv::String*"]), _)]),
	#[inline]
	fn dsgettype(&self, dslabel: &str) -> Result<i32> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsgettype_const_const_StringR(self.as_raw_HDF5(), dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// dswrite(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:394
	// ("cv::hdf::HDF5::dswrite", vec![(pred!(const, ["Array", "dslabel"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
	#[inline]
	fn dswrite(&self, array: &impl ToInputArray, dslabel: &str) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * dims_counts: vector<int>()
	// dswrite(InputArray, const String &, const vector<int> &, const vector<int> &)(InputArray, InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:399
	// ("cv::hdf::HDF5::dswrite", vec![(pred!(const, ["Array", "dslabel", "dims_offset", "dims_counts"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*", "const std::vector<int>*"]), _)]),
	#[inline]
	fn dswrite_offset(&self, array: &impl ToInputArray, dslabel: &str, dims_offset: &core::Vector<i32>, dims_counts: &core::Vector<i32>) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_vectorLintGR_const_vectorLintGR(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), dims_offset.as_raw_VectorOfi32(), dims_counts.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [HDF5TraitConst::dswrite_offset] function uses the following default values for its arguments:
	/// * dims_counts: vector<int>()
	// cv::hdf::HDF5::dswrite(InputArray, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:399
	// ("cv::hdf::HDF5::dswrite", vec![(pred!(const, ["Array", "dslabel", "dims_offset"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*"]), _)]),
	#[inline]
	fn dswrite_offset_def(&self, array: &impl ToInputArray, dslabel: &str, dims_offset: &core::Vector<i32>) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_vectorLintGR(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), dims_offset.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// dsinsert(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:468
	// ("cv::hdf::HDF5::dsinsert", vec![(pred!(const, ["Array", "dslabel"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
	#[inline]
	fn dsinsert(&self, array: &impl ToInputArray, dslabel: &str) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * dims_counts: vector<int>()
	// dsinsert(InputArray, const String &, const vector<int> &, const vector<int> &)(InputArray, InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:473
	// ("cv::hdf::HDF5::dsinsert", vec![(pred!(const, ["Array", "dslabel", "dims_offset", "dims_counts"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*", "const std::vector<int>*"]), _)]),
	#[inline]
	fn dsinsert_offset(&self, array: &impl ToInputArray, dslabel: &str, dims_offset: &core::Vector<i32>, dims_counts: &core::Vector<i32>) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_vectorLintGR_const_vectorLintGR(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), dims_offset.as_raw_VectorOfi32(), dims_counts.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [HDF5TraitConst::dsinsert_offset] function uses the following default values for its arguments:
	/// * dims_counts: vector<int>()
	// cv::hdf::HDF5::dsinsert(InputArray, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:473
	// ("cv::hdf::HDF5::dsinsert", vec![(pred!(const, ["Array", "dslabel", "dims_offset"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*"]), _)]),
	#[inline]
	fn dsinsert_offset_def(&self, array: &impl ToInputArray, dslabel: &str, dims_offset: &core::Vector<i32>) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_vectorLintGR(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), dims_offset.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// dsread(OutputArray, const String &)(OutputArray, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:528
	// ("cv::hdf::HDF5::dsread", vec![(pred!(const, ["Array", "dslabel"], ["const cv::_OutputArray*", "const cv::String*"]), _)]),
	#[inline]
	fn dsread(&self, array: &mut impl ToOutputArray, dslabel: &str) -> Result<()> {
		output_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR(self.as_raw_HDF5(), array.as_raw__OutputArray(), dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * dims_counts: vector<int>()
	// dsread(OutputArray, const String &, const vector<int> &, const vector<int> &)(OutputArray, InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:533
	// ("cv::hdf::HDF5::dsread", vec![(pred!(const, ["Array", "dslabel", "dims_offset", "dims_counts"], ["const cv::_OutputArray*", "const cv::String*", "const std::vector<int>*", "const std::vector<int>*"]), _)]),
	#[inline]
	fn dsread_offset(&self, array: &mut impl ToOutputArray, dslabel: &str, dims_offset: &core::Vector<i32>, dims_counts: &core::Vector<i32>) -> Result<()> {
		output_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_vectorLintGR_const_vectorLintGR(self.as_raw_HDF5(), array.as_raw__OutputArray(), dslabel.opencv_as_extern(), dims_offset.as_raw_VectorOfi32(), dims_counts.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [HDF5TraitConst::dsread_offset] function uses the following default values for its arguments:
	/// * dims_counts: vector<int>()
	// cv::hdf::HDF5::dsread(OutputArray, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:533
	// ("cv::hdf::HDF5::dsread", vec![(pred!(const, ["Array", "dslabel", "dims_offset"], ["const cv::_OutputArray*", "const cv::String*", "const std::vector<int>*"]), _)]),
	#[inline]
	fn dsread_offset_def(&self, array: &mut impl ToOutputArray, dslabel: &str, dims_offset: &core::Vector<i32>) -> Result<()> {
		output_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_vectorLintGR(self.as_raw_HDF5(), array.as_raw__OutputArray(), dslabel.opencv_as_extern(), dims_offset.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Fetch keypoint dataset size
	/// ## Parameters
	/// * kplabel: specify the hdf5 dataset label to be measured.
	/// * dims_flag: will fetch dataset dimensions on H5_GETDIMS, and dataset maximum dimensions on H5_GETMAXDIMS.
	///
	/// Returns size of keypoints dataset.
	///
	///
	/// Note: Resulting size will match the amount of keypoints. By default H5_GETDIMS will return actual dataset dimension.
	/// Using H5_GETMAXDIM flag will get maximum allowed dimension which normally match actual dataset dimension but can hold
	/// H5_UNLIMITED value if dataset was prepared in **unlimited** mode. It can be useful to check existing dataset dimension
	/// before overwrite it as whole or subset. Trying to write with oversized source data into dataset target will thrown
	/// exception. The H5_GETCHUNKDIMS will return the dimension of chunk if dataset was created with chunking options otherwise
	/// returned vector size will be zero.
	///
	/// ## C++ default parameters
	/// * dims_flag: HDF5::H5_GETDIMS
	// kpgetsize(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:591
	// ("cv::hdf::HDF5::kpgetsize", vec![(pred!(const, ["kplabel", "dims_flag"], ["const cv::String*", "int"]), _)]),
	#[inline]
	fn kpgetsize(&self, kplabel: &str, dims_flag: i32) -> Result<i32> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpgetsize_const_const_StringR_int(self.as_raw_HDF5(), kplabel.opencv_as_extern(), dims_flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Fetch keypoint dataset size
	/// ## Parameters
	/// * kplabel: specify the hdf5 dataset label to be measured.
	/// * dims_flag: will fetch dataset dimensions on H5_GETDIMS, and dataset maximum dimensions on H5_GETMAXDIMS.
	///
	/// Returns size of keypoints dataset.
	///
	///
	/// Note: Resulting size will match the amount of keypoints. By default H5_GETDIMS will return actual dataset dimension.
	/// Using H5_GETMAXDIM flag will get maximum allowed dimension which normally match actual dataset dimension but can hold
	/// H5_UNLIMITED value if dataset was prepared in **unlimited** mode. It can be useful to check existing dataset dimension
	/// before overwrite it as whole or subset. Trying to write with oversized source data into dataset target will thrown
	/// exception. The H5_GETCHUNKDIMS will return the dimension of chunk if dataset was created with chunking options otherwise
	/// returned vector size will be zero.
	///
	/// ## Note
	/// This alternative version of [HDF5TraitConst::kpgetsize] function uses the following default values for its arguments:
	/// * dims_flag: HDF5::H5_GETDIMS
	// cv::hdf::HDF5::kpgetsize(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:591
	// ("cv::hdf::HDF5::kpgetsize", vec![(pred!(const, ["kplabel"], ["const cv::String*"]), _)]),
	#[inline]
	fn kpgetsize_def(&self, kplabel: &str) -> Result<i32> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpgetsize_const_const_StringR(self.as_raw_HDF5(), kplabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Create and allocate special storage for cv::KeyPoint dataset.
	/// ## Parameters
	/// * size: declare fixed number of KeyPoints
	/// * kplabel: specify the hdf5 dataset label, any existing dataset with the same label will be overwritten.
	/// * compresslevel: specify the compression level 0-9 to be used, H5_NONE is default and means no compression.
	/// * chunks: each array member specifies chunking sizes to be used for block I/O,
	///        H5_NONE is default and means no compression.
	///
	/// Note: If the dataset already exists an exception will be thrown. Existence of the dataset can be checked
	/// using hlexists().
	///
	/// - See example below that creates space for 100 keypoints in the dataset:
	/// ```C++
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   if ( ! h5io->hlexists( "keypoints" ) )
	///    h5io->kpcreate( 100, "keypoints" );
	///   else
	///    printf("DS already created, skipping\n" );
	/// ```
	///
	///
	///
	/// Note: A value of H5_UNLIMITED for **size** means **unlimited** keypoints, thus is possible to expand anytime such
	/// dataset by adding or inserting. Presence of H5_UNLIMITED **require** to define custom chunking. No default chunking
	/// will be defined in unlimited scenario since default size on that dimension will be zero, and will grow once dataset
	/// is written. Writing into dataset that have H5_UNLIMITED on some of its dimension requires kpinsert() that allow
	/// growth on unlimited dimension instead of kpwrite() that allows to write only in predefined data space.
	///
	/// - See example below that creates unlimited space for keypoints chunking size of 100 but no compression:
	/// ```C++
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   if ( ! h5io->hlexists( "keypoints" ) )
	///    h5io->kpcreate( cv::hdf::HDF5::H5_UNLIMITED, "keypoints", cv::hdf::HDF5::H5_NONE, 100 );
	///   else
	///    printf("DS already created, skipping\n" );
	/// ```
	///
	///
	/// ## C++ default parameters
	/// * compresslevel: H5_NONE
	/// * chunks: H5_NONE
	// kpcreate(const int, const String &, const int, const int)(Primitive, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:628
	// ("cv::hdf::HDF5::kpcreate", vec![(pred!(const, ["size", "kplabel", "compresslevel", "chunks"], ["const int", "const cv::String*", "const int", "const int"]), _)]),
	#[inline]
	fn kpcreate(&self, size: i32, kplabel: &str, compresslevel: i32, chunks: i32) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpcreate_const_const_int_const_StringR_const_int_const_int(self.as_raw_HDF5(), size, kplabel.opencv_as_extern(), compresslevel, chunks, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Create and allocate special storage for cv::KeyPoint dataset.
	/// ## Parameters
	/// * size: declare fixed number of KeyPoints
	/// * kplabel: specify the hdf5 dataset label, any existing dataset with the same label will be overwritten.
	/// * compresslevel: specify the compression level 0-9 to be used, H5_NONE is default and means no compression.
	/// * chunks: each array member specifies chunking sizes to be used for block I/O,
	///        H5_NONE is default and means no compression.
	///
	/// Note: If the dataset already exists an exception will be thrown. Existence of the dataset can be checked
	/// using hlexists().
	///
	/// - See example below that creates space for 100 keypoints in the dataset:
	/// ```C++
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   if ( ! h5io->hlexists( "keypoints" ) )
	///    h5io->kpcreate( 100, "keypoints" );
	///   else
	///    printf("DS already created, skipping\n" );
	/// ```
	///
	///
	///
	/// Note: A value of H5_UNLIMITED for **size** means **unlimited** keypoints, thus is possible to expand anytime such
	/// dataset by adding or inserting. Presence of H5_UNLIMITED **require** to define custom chunking. No default chunking
	/// will be defined in unlimited scenario since default size on that dimension will be zero, and will grow once dataset
	/// is written. Writing into dataset that have H5_UNLIMITED on some of its dimension requires kpinsert() that allow
	/// growth on unlimited dimension instead of kpwrite() that allows to write only in predefined data space.
	///
	/// - See example below that creates unlimited space for keypoints chunking size of 100 but no compression:
	/// ```C++
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   if ( ! h5io->hlexists( "keypoints" ) )
	///    h5io->kpcreate( cv::hdf::HDF5::H5_UNLIMITED, "keypoints", cv::hdf::HDF5::H5_NONE, 100 );
	///   else
	///    printf("DS already created, skipping\n" );
	/// ```
	///
	///
	/// ## Note
	/// This alternative version of [HDF5TraitConst::kpcreate] function uses the following default values for its arguments:
	/// * compresslevel: H5_NONE
	/// * chunks: H5_NONE
	// cv::hdf::HDF5::kpcreate(Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:628
	// ("cv::hdf::HDF5::kpcreate", vec![(pred!(const, ["size", "kplabel"], ["const int", "const cv::String*"]), _)]),
	#[inline]
	fn kpcreate_def(&self, size: i32, kplabel: &str) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpcreate_const_const_int_const_StringR(self.as_raw_HDF5(), size, kplabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Write or overwrite list of KeyPoint into specified dataset of hdf5 file.
	/// ## Parameters
	/// * keypoints: specify keypoints data list to be written.
	/// * kplabel: specify the target hdf5 dataset label.
	/// * offset: specify the offset location on dataset from where keypoints will be (over)written into dataset.
	/// * counts: specify the amount of keypoints that will be written into dataset.
	///
	/// Writes vector<KeyPoint> object into targeted dataset.
	///
	///
	/// Note: If dataset is not created and does not exist it will be created **automatically**. It is thread safe but
	/// it is recommended that writes to happen over separate non overlapping regions. Multiple datasets can be written
	/// inside single hdf5 file.
	///
	/// - Example below writes a 100 keypoints into a dataset. No dataset precreation required. If routine is called multiple
	/// times dataset will be just overwritten:
	/// ```C++
	///   // generate 100 dummy keypoints
	///   std::vector<cv::KeyPoint> keypoints;
	///   for(int i = 0; i < 100; i++)
	///    keypoints.push_back( cv::KeyPoint(i, -i, 1, -1, 0, 0, -1) );
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // write / overwrite dataset
	///   h5io->kpwrite( keypoints, "keypoints" );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	/// - Example below uses smaller set of 50 keypoints and writes into compressed space of 100 keypoints optimised by 10 chunks.
	/// Same keypoint set is written three times, first into first half (0->50) and at second half (50->75) then into remaining slots
	/// (75->99) of data space using offset and count parameters to settle the window for write access.If routine is called multiple times
	/// dataset will be just overwritten:
	/// ```C++
	///   // generate 50 dummy keypoints
	///   std::vector<cv::KeyPoint> keypoints;
	///   for(int i = 0; i < 50; i++)
	///    keypoints.push_back( cv::KeyPoint(i, -i, 1, -1, 0, 0, -1) );
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create maximum compressed space of size 100 with chunk size 10
	///   h5io->kpcreate( 100, "keypoints", 9, 10 );
	///   // write into first half
	///   h5io->kpwrite( keypoints, "keypoints", 0 );
	///   // write first 25 keypoints into second half
	///   h5io->kpwrite( keypoints, "keypoints", 50, 25 );
	///   // write first 25 keypoints into remained space of second half
	///   h5io->kpwrite( keypoints, "keypoints", 75, 25 );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	/// ## C++ default parameters
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	// kpwrite(const vector<KeyPoint>, const String &, const int, const int)(CppPassByVoidPtr, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:681
	// ("cv::hdf::HDF5::kpwrite", vec![(pred!(const, ["keypoints", "kplabel", "offset", "counts"], ["const std::vector<cv::KeyPoint>", "const cv::String*", "const int", "const int"]), _)]),
	#[inline]
	fn kpwrite(&self, keypoints: core::Vector<core::KeyPoint>, kplabel: &str, offset: i32, counts: i32) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpwrite_const_const_vectorLKeyPointG_const_StringR_const_int_const_int(self.as_raw_HDF5(), keypoints.as_raw_VectorOfKeyPoint(), kplabel.opencv_as_extern(), offset, counts, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Write or overwrite list of KeyPoint into specified dataset of hdf5 file.
	/// ## Parameters
	/// * keypoints: specify keypoints data list to be written.
	/// * kplabel: specify the target hdf5 dataset label.
	/// * offset: specify the offset location on dataset from where keypoints will be (over)written into dataset.
	/// * counts: specify the amount of keypoints that will be written into dataset.
	///
	/// Writes vector<KeyPoint> object into targeted dataset.
	///
	///
	/// Note: If dataset is not created and does not exist it will be created **automatically**. It is thread safe but
	/// it is recommended that writes to happen over separate non overlapping regions. Multiple datasets can be written
	/// inside single hdf5 file.
	///
	/// - Example below writes a 100 keypoints into a dataset. No dataset precreation required. If routine is called multiple
	/// times dataset will be just overwritten:
	/// ```C++
	///   // generate 100 dummy keypoints
	///   std::vector<cv::KeyPoint> keypoints;
	///   for(int i = 0; i < 100; i++)
	///    keypoints.push_back( cv::KeyPoint(i, -i, 1, -1, 0, 0, -1) );
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // write / overwrite dataset
	///   h5io->kpwrite( keypoints, "keypoints" );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	/// - Example below uses smaller set of 50 keypoints and writes into compressed space of 100 keypoints optimised by 10 chunks.
	/// Same keypoint set is written three times, first into first half (0->50) and at second half (50->75) then into remaining slots
	/// (75->99) of data space using offset and count parameters to settle the window for write access.If routine is called multiple times
	/// dataset will be just overwritten:
	/// ```C++
	///   // generate 50 dummy keypoints
	///   std::vector<cv::KeyPoint> keypoints;
	///   for(int i = 0; i < 50; i++)
	///    keypoints.push_back( cv::KeyPoint(i, -i, 1, -1, 0, 0, -1) );
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create maximum compressed space of size 100 with chunk size 10
	///   h5io->kpcreate( 100, "keypoints", 9, 10 );
	///   // write into first half
	///   h5io->kpwrite( keypoints, "keypoints", 0 );
	///   // write first 25 keypoints into second half
	///   h5io->kpwrite( keypoints, "keypoints", 50, 25 );
	///   // write first 25 keypoints into remained space of second half
	///   h5io->kpwrite( keypoints, "keypoints", 75, 25 );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	/// ## Note
	/// This alternative version of [HDF5TraitConst::kpwrite] function uses the following default values for its arguments:
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	// cv::hdf::HDF5::kpwrite(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:681
	// ("cv::hdf::HDF5::kpwrite", vec![(pred!(const, ["keypoints", "kplabel"], ["const std::vector<cv::KeyPoint>", "const cv::String*"]), _)]),
	#[inline]
	fn kpwrite_def(&self, keypoints: core::Vector<core::KeyPoint>, kplabel: &str) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpwrite_const_const_vectorLKeyPointG_const_StringR(self.as_raw_HDF5(), keypoints.as_raw_VectorOfKeyPoint(), kplabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Insert or overwrite list of KeyPoint into specified dataset and autoexpand dataset size if **unlimited** property allows.
	/// ## Parameters
	/// * keypoints: specify keypoints data list to be written.
	/// * kplabel: specify the target hdf5 dataset label.
	/// * offset: specify the offset location on dataset from where keypoints will be (over)written into dataset.
	/// * counts: specify the amount of keypoints that will be written into dataset.
	///
	/// Writes vector<KeyPoint> object into targeted dataset and **autoexpand** dataset dimension if allowed.
	///
	///
	/// Note: Unlike kpwrite(), datasets are **not** created **automatically**. If dsinsert() happen over outer region of dataset
	/// and dataset has been created in **unlimited** mode then dataset is expanded, otherwise exception is thrown. To create datasets
	/// with **unlimited** property see kpcreate() and the optional H5_UNLIMITED flag at creation time. It is not thread safe over same
	/// dataset but multiple datasets can be merged inside single hdf5 file.
	///
	/// - Example below creates **unlimited** space for keypoints storage, and inserts a list of 10 keypoints ten times into that space.
	/// Final dataset will have 100 keypoints. Chunks size is 10 just optimized against list of keypoints. If routine is called multiple
	/// times dataset will be just overwritten:
	/// ```C++
	///   // generate 10 dummy keypoints
	///   std::vector<cv::KeyPoint> keypoints;
	///   for(int i = 0; i < 10; i++)
	///    keypoints.push_back( cv::KeyPoint(i, -i, 1, -1, 0, 0, -1) );
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create unlimited size space with chunk size of 10
	///   h5io->kpcreate( cv::hdf::HDF5::H5_UNLIMITED, "keypoints", -1, 10 );
	///   // insert 10 times same 10 keypoints
	///   for(int i = 0; i < 10; i++)
	///    h5io->kpinsert( keypoints, "keypoints", i * 10 );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	/// ## C++ default parameters
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	// kpinsert(const vector<KeyPoint>, const String &, const int, const int)(CppPassByVoidPtr, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:716
	// ("cv::hdf::HDF5::kpinsert", vec![(pred!(const, ["keypoints", "kplabel", "offset", "counts"], ["const std::vector<cv::KeyPoint>", "const cv::String*", "const int", "const int"]), _)]),
	#[inline]
	fn kpinsert(&self, keypoints: core::Vector<core::KeyPoint>, kplabel: &str, offset: i32, counts: i32) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpinsert_const_const_vectorLKeyPointG_const_StringR_const_int_const_int(self.as_raw_HDF5(), keypoints.as_raw_VectorOfKeyPoint(), kplabel.opencv_as_extern(), offset, counts, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Insert or overwrite list of KeyPoint into specified dataset and autoexpand dataset size if **unlimited** property allows.
	/// ## Parameters
	/// * keypoints: specify keypoints data list to be written.
	/// * kplabel: specify the target hdf5 dataset label.
	/// * offset: specify the offset location on dataset from where keypoints will be (over)written into dataset.
	/// * counts: specify the amount of keypoints that will be written into dataset.
	///
	/// Writes vector<KeyPoint> object into targeted dataset and **autoexpand** dataset dimension if allowed.
	///
	///
	/// Note: Unlike kpwrite(), datasets are **not** created **automatically**. If dsinsert() happen over outer region of dataset
	/// and dataset has been created in **unlimited** mode then dataset is expanded, otherwise exception is thrown. To create datasets
	/// with **unlimited** property see kpcreate() and the optional H5_UNLIMITED flag at creation time. It is not thread safe over same
	/// dataset but multiple datasets can be merged inside single hdf5 file.
	///
	/// - Example below creates **unlimited** space for keypoints storage, and inserts a list of 10 keypoints ten times into that space.
	/// Final dataset will have 100 keypoints. Chunks size is 10 just optimized against list of keypoints. If routine is called multiple
	/// times dataset will be just overwritten:
	/// ```C++
	///   // generate 10 dummy keypoints
	///   std::vector<cv::KeyPoint> keypoints;
	///   for(int i = 0; i < 10; i++)
	///    keypoints.push_back( cv::KeyPoint(i, -i, 1, -1, 0, 0, -1) );
	///   // open / autocreate hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // create unlimited size space with chunk size of 10
	///   h5io->kpcreate( cv::hdf::HDF5::H5_UNLIMITED, "keypoints", -1, 10 );
	///   // insert 10 times same 10 keypoints
	///   for(int i = 0; i < 10; i++)
	///    h5io->kpinsert( keypoints, "keypoints", i * 10 );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	/// ## Note
	/// This alternative version of [HDF5TraitConst::kpinsert] function uses the following default values for its arguments:
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	// cv::hdf::HDF5::kpinsert(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:716
	// ("cv::hdf::HDF5::kpinsert", vec![(pred!(const, ["keypoints", "kplabel"], ["const std::vector<cv::KeyPoint>", "const cv::String*"]), _)]),
	#[inline]
	fn kpinsert_def(&self, keypoints: core::Vector<core::KeyPoint>, kplabel: &str) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpinsert_const_const_vectorLKeyPointG_const_StringR(self.as_raw_HDF5(), keypoints.as_raw_VectorOfKeyPoint(), kplabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Read specific keypoint dataset from hdf5 file into vector<KeyPoint> object.
	/// ## Parameters
	/// * keypoints: vector<KeyPoint> container where data reads will be returned.
	/// * kplabel: specify the source hdf5 dataset label.
	/// * offset: specify the offset location over dataset from where read starts.
	/// * counts: specify the amount of keypoints from dataset to read.
	///
	/// Reads out vector<KeyPoint> object reflecting the stored dataset.
	///
	///
	/// Note: If hdf5 file does not exist an exception will be thrown. Use hlexists() to check dataset presence.
	/// It is thread safe.
	///
	/// - Example below reads a dataset containing keypoints starting with second entry:
	/// ```C++
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // blank KeyPoint container
	///   std::vector<cv::KeyPoint> keypoints;
	///   // read keypoints starting second one
	///   h5io->kpread( keypoints, "keypoints", 1 );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	/// - Example below perform read of 3 keypoints from second entry.
	/// ```C++
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // blank KeyPoint container
	///   std::vector<cv::KeyPoint> keypoints;
	///   // read three keypoints starting second one
	///   h5io->kpread( keypoints, "keypoints", 1, 3 );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	/// ## C++ default parameters
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	// kpread(vector<KeyPoint> &, const String &, const int, const int)(CppPassByVoidPtr, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:754
	// ("cv::hdf::HDF5::kpread", vec![(pred!(const, ["keypoints", "kplabel", "offset", "counts"], ["std::vector<cv::KeyPoint>*", "const cv::String*", "const int", "const int"]), _)]),
	#[inline]
	fn kpread(&self, keypoints: &mut core::Vector<core::KeyPoint>, kplabel: &str, offset: i32, counts: i32) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpread_const_vectorLKeyPointGR_const_StringR_const_int_const_int(self.as_raw_HDF5(), keypoints.as_raw_mut_VectorOfKeyPoint(), kplabel.opencv_as_extern(), offset, counts, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Read specific keypoint dataset from hdf5 file into vector<KeyPoint> object.
	/// ## Parameters
	/// * keypoints: vector<KeyPoint> container where data reads will be returned.
	/// * kplabel: specify the source hdf5 dataset label.
	/// * offset: specify the offset location over dataset from where read starts.
	/// * counts: specify the amount of keypoints from dataset to read.
	///
	/// Reads out vector<KeyPoint> object reflecting the stored dataset.
	///
	///
	/// Note: If hdf5 file does not exist an exception will be thrown. Use hlexists() to check dataset presence.
	/// It is thread safe.
	///
	/// - Example below reads a dataset containing keypoints starting with second entry:
	/// ```C++
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // blank KeyPoint container
	///   std::vector<cv::KeyPoint> keypoints;
	///   // read keypoints starting second one
	///   h5io->kpread( keypoints, "keypoints", 1 );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	/// - Example below perform read of 3 keypoints from second entry.
	/// ```C++
	///   // open hdf5 file
	///   cv::Ptr<cv::hdf::HDF5> h5io = cv::hdf::open( "mytest.h5" );
	///   // blank KeyPoint container
	///   std::vector<cv::KeyPoint> keypoints;
	///   // read three keypoints starting second one
	///   h5io->kpread( keypoints, "keypoints", 1, 3 );
	///   // release
	///   h5io->close();
	/// ```
	///
	///
	/// ## Note
	/// This alternative version of [HDF5TraitConst::kpread] function uses the following default values for its arguments:
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	// cv::hdf::HDF5::kpread(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:754
	// ("cv::hdf::HDF5::kpread", vec![(pred!(const, ["keypoints", "kplabel"], ["std::vector<cv::KeyPoint>*", "const cv::String*"]), _)]),
	#[inline]
	fn kpread_def(&self, keypoints: &mut core::Vector<core::KeyPoint>, kplabel: &str) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpread_const_vectorLKeyPointGR_const_StringR(self.as_raw_HDF5(), keypoints.as_raw_mut_VectorOfKeyPoint(), kplabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::hdf::HDF5]
pub trait HDF5Trait: crate::hdf::HDF5TraitConst {
	fn as_raw_mut_HDF5(&mut self) -> *mut c_void;

	/// Close and release hdf5 object.
	// close()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:73
	// ("cv::hdf::HDF5::close", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn close(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_close(self.as_raw_mut_HDF5(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Create a group.
	/// ## Parameters
	/// * grlabel: specify the hdf5 group label.
	///
	/// Create a hdf5 group with default properties. The group is closed automatically after creation.
	///
	///
	/// Note: Groups are useful for better organising multiple datasets. It is possible to create subgroups within any group.
	/// Existence of a particular group can be checked using hlexists(). In case of subgroups, a label would be e.g: 'Group1/SubGroup1'
	/// where SubGroup1 is within the root group Group1. Before creating a subgroup, its parent group MUST be created.
	///
	/// - In this example, Group1 will have one subgroup called SubGroup1:
	///
	///  [create_group](https://github.com/opencv/opencv_contrib/blob/5.0.0/modules/hdf/samples/create_groups.cpp#L1)
	///
	///  The corresponding result visualized using the HDFView tool is
	///
	///  ![Visualization of groups using the HDFView tool](https://docs.opencv.org/5.0.0/create_groups.png)
	///
	///
	/// Note: When a dataset is created with dscreate() or kpcreate(), it can be created within a group by specifying the
	/// full path within the label. In our example, it would be: 'Group1/SubGroup1/MyDataSet'. It is not thread safe.
	// grcreate(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:95
	// ("cv::hdf::HDF5::grcreate", vec![(pred!(mut, ["grlabel"], ["const cv::String*"]), _)]),
	#[inline]
	fn grcreate(&mut self, grlabel: &str) -> Result<()> {
		extern_container_arg!(grlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_grcreate_const_StringR(self.as_raw_mut_HDF5(), grlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Delete an attribute from the root group.
	///
	/// ## Parameters
	/// * atlabel: the attribute to be deleted.
	///
	///
	/// Note: CV_Error() is called if the given attribute does not exist. Use atexists()
	/// to check whether it exists or not beforehand.
	/// ## See also
	/// atexists, atwrite, atread
	// atdelete(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:126
	// ("cv::hdf::HDF5::atdelete", vec![(pred!(mut, ["atlabel"], ["const cv::String*"]), _)]),
	#[inline]
	fn atdelete(&mut self, atlabel: &str) -> Result<()> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atdelete_const_StringR(self.as_raw_mut_HDF5(), atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Write an attribute inside the root group.
	///
	/// ## Parameters
	/// * value: attribute value.
	/// * atlabel: attribute name.
	///
	/// The following example demonstrates how to write an attribute of type cv::String:
	///
	///  [snippets_write_str](https://github.com/opencv/opencv_contrib/blob/5.0.0/modules/hdf/samples/read_write_attributes.cpp#L1)
	///
	///
	/// Note: CV_Error() is called if the given attribute already exists. Use atexists()
	/// to check whether it exists or not beforehand. And use atdelete() to delete
	/// it if it already exists.
	/// ## See also
	/// atexists, atdelete, atread
	// atwrite(const int, const String &)(Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:144
	// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const int", "const cv::String*"]), _)]),
	#[inline]
	fn atwrite_i32(&mut self, value: i32, atlabel: &str) -> Result<()> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atwrite_const_int_const_StringR(self.as_raw_mut_HDF5(), value, atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Read an attribute from the root group.
	///
	/// ## Parameters
	/// * value: address where the attribute is read into
	/// * atlabel: attribute name
	///
	/// The following example demonstrates how to read an attribute of type cv::String:
	///
	///  [snippets_read_str](https://github.com/opencv/opencv_contrib/blob/5.0.0/modules/hdf/samples/read_write_attributes.cpp#L1)
	///
	///
	/// Note: The attribute MUST exist, otherwise CV_Error() is called. Use atexists()
	/// to check if it exists beforehand.
	/// ## See also
	/// atexists, atdelete, atwrite
	// atread(int *, const String &)(Indirect, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:161
	// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["int*", "const cv::String*"]), _)]),
	#[inline]
	fn atread_i32(&mut self, value: &mut i32, atlabel: &str) -> Result<()> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atread_intX_const_StringR(self.as_raw_mut_HDF5(), value, atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Write an attribute inside the root group.
	///
	/// ## Parameters
	/// * value: attribute value.
	/// * atlabel: attribute name.
	///
	/// The following example demonstrates how to write an attribute of type cv::String:
	///
	///  [snippets_write_str](https://github.com/opencv/opencv_contrib/blob/5.0.0/modules/hdf/samples/read_write_attributes.cpp#L1)
	///
	///
	/// Note: CV_Error() is called if the given attribute already exists. Use atexists()
	/// to check whether it exists or not beforehand. And use atdelete() to delete
	/// it if it already exists.
	/// ## See also
	/// atexists, atdelete, atread
	///
	/// ## Overloaded parameters
	// atwrite(const double, const String &)(Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:164
	// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const double", "const cv::String*"]), _)]),
	#[inline]
	fn atwrite_f64(&mut self, value: f64, atlabel: &str) -> Result<()> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atwrite_const_double_const_StringR(self.as_raw_mut_HDF5(), value, atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Read an attribute from the root group.
	///
	/// ## Parameters
	/// * value: address where the attribute is read into
	/// * atlabel: attribute name
	///
	/// The following example demonstrates how to read an attribute of type cv::String:
	///
	///  [snippets_read_str](https://github.com/opencv/opencv_contrib/blob/5.0.0/modules/hdf/samples/read_write_attributes.cpp#L1)
	///
	///
	/// Note: The attribute MUST exist, otherwise CV_Error() is called. Use atexists()
	/// to check if it exists beforehand.
	/// ## See also
	/// atexists, atdelete, atwrite
	///
	/// ## Overloaded parameters
	// atread(double *, const String &)(Indirect, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:167
	// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["double*", "const cv::String*"]), _)]),
	#[inline]
	fn atread_f64(&mut self, value: &mut f64, atlabel: &str) -> Result<()> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atread_doubleX_const_StringR(self.as_raw_mut_HDF5(), value, atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Write an attribute inside the root group.
	///
	/// ## Parameters
	/// * value: attribute value.
	/// * atlabel: attribute name.
	///
	/// The following example demonstrates how to write an attribute of type cv::String:
	///
	///  [snippets_write_str](https://github.com/opencv/opencv_contrib/blob/5.0.0/modules/hdf/samples/read_write_attributes.cpp#L1)
	///
	///
	/// Note: CV_Error() is called if the given attribute already exists. Use atexists()
	/// to check whether it exists or not beforehand. And use atdelete() to delete
	/// it if it already exists.
	/// ## See also
	/// atexists, atdelete, atread
	///
	/// ## Overloaded parameters
	// atwrite(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:170
	// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const cv::String*", "const cv::String*"]), _)]),
	#[inline]
	fn atwrite_str(&mut self, value: &str, atlabel: &str) -> Result<()> {
		extern_container_arg!(value);
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atwrite_const_StringR_const_StringR(self.as_raw_mut_HDF5(), value.opencv_as_extern(), atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Read an attribute from the root group.
	///
	/// ## Parameters
	/// * value: address where the attribute is read into
	/// * atlabel: attribute name
	///
	/// The following example demonstrates how to read an attribute of type cv::String:
	///
	///  [snippets_read_str](https://github.com/opencv/opencv_contrib/blob/5.0.0/modules/hdf/samples/read_write_attributes.cpp#L1)
	///
	///
	/// Note: The attribute MUST exist, otherwise CV_Error() is called. Use atexists()
	/// to check if it exists beforehand.
	/// ## See also
	/// atexists, atdelete, atwrite
	///
	/// ## Overloaded parameters
	// atread(String *, const String &)(OutString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:173
	// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["cv::String*", "const cv::String*"]), _)]),
	#[inline]
	fn atread_str(&mut self, value: &mut String, atlabel: &str) -> Result<()> {
		string_arg_output_send!(via value_via);
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atread_StringX_const_StringR(self.as_raw_mut_HDF5(), &mut value_via, atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(value_via => value);
		Ok(ret)
	}

	/// Write an attribute into the root group.
	///
	/// ## Parameters
	/// * value: attribute value. Currently, only n-d continuous multi-channel arrays are supported.
	/// * atlabel: attribute name.
	///
	///
	/// Note: CV_Error() is called if the given attribute already exists. Use atexists()
	/// to check whether it exists or not beforehand. And use atdelete() to delete
	/// it if it already exists.
	/// ## See also
	/// atexists, atdelete, atread.
	// atwrite(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:187
	// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
	#[inline]
	fn atwrite(&mut self, value: &impl ToInputArray, atlabel: &str) -> Result<()> {
		input_array_arg!(value);
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atwrite_const__InputArrayR_const_StringR(self.as_raw_mut_HDF5(), value.as_raw__InputArray(), atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Read an attribute from the root group.
	///
	/// ## Parameters
	/// * value: attribute value. Currently, only n-d continuous multi-channel arrays are supported.
	/// * atlabel: attribute name.
	///
	///
	/// Note: The attribute MUST exist, otherwise CV_Error() is called. Use atexists()
	/// to check if it exists beforehand.
	/// ## See also
	/// atexists, atdelete, atwrite
	// atread(OutputArray, const String &)(OutputArray, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:200
	// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["const cv::_OutputArray*", "const cv::String*"]), _)]),
	#[inline]
	fn atread(&mut self, value: &mut impl ToOutputArray, atlabel: &str) -> Result<()> {
		output_array_arg!(value);
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atread_const__OutputArrayR_const_StringR(self.as_raw_mut_HDF5(), value.as_raw__OutputArray(), atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Hierarchical Data Format version 5 interface.
///
/// Notice that this module is compiled only when hdf5 is correctly installed.
// HDF5 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/hdf/hdf5.hpp:56
pub struct HDF5 {
	ptr: *mut c_void,
}

opencv_type_boxed! { HDF5 }

impl Drop for HDF5 {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_hdf_HDF5_delete(self.as_raw_mut_HDF5()) };
	}
}

unsafe impl Send for HDF5 {}

impl crate::hdf::HDF5TraitConst for HDF5 {
	#[inline] fn as_raw_HDF5(&self) -> *const c_void { self.as_raw() }
}

impl crate::hdf::HDF5Trait for HDF5 {
	#[inline] fn as_raw_mut_HDF5(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { HDF5, crate::hdf::HDF5TraitConst, as_raw_HDF5, crate::hdf::HDF5Trait, as_raw_mut_HDF5 }

impl HDF5 {
}

impl std::fmt::Debug for HDF5 {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("HDF5")
			.finish()
	}
}

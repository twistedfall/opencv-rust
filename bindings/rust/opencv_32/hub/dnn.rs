//! # Deep Neural Network module
//!   This module contains:
//!       - API for new layers creation, layers are building bricks of neural networks;
//!       - set of built-in most-useful Layers;
//!       - API to constuct and modify comprehensive neural networks from layers;
//!       - functionality for loading serialized networks models from differnet frameworks.
//! 
//!   Functionality of this module is designed only for forward pass computations (i. e. network testing).
//!   A network training is in principle not supported.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::DictValueTrait, super::DictTrait, super::BlobShapeTrait, super::BlobTrait, super::LayerParamsTrait, super::Layer, super::NetTrait, super::Importer, super::LayerFactoryTrait, super::LSTMLayer, super::RNNLayer, super::BaseConvolutionLayer, super::ConvolutionLayer, super::DeconvolutionLayer, super::LRNLayer, super::PoolingLayer, super::SoftmaxLayer, super::InnerProductLayer, super::MVNLayer, super::ReshapeLayer, super::ConcatLayer, super::SplitLayer, super::SliceLayer, super::ReLULayer, super::TanHLayer, super::SigmoidLayer, super::BNLLLayer, super::AbsLayer, super::PowerLayer, super::CropLayer, super::EltwiseLayer, super::_RangeTrait };
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Blob_AllocFlag {
	ALLOC_MAT = 1 as isize,
	ALLOC_UMAT = 2 as isize,
	ALLOC_BOTH = 3 as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Blob_DataState {
	UNINITIALIZED = 0 as isize,
	HEAD_AT_MAT = 1 as isize,
	HEAD_AT_UMAT = 2 as isize,
	SYNCED = 3 as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EltwiseLayer_EltwiseOp {
	PROD = 0 as isize,
	SUM = 1 as isize,
	MAX = 2 as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LRNLayer_Type {
	CHANNEL_NRM = 0 as isize,
	SPATIAL_NRM = 1 as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PoolingLayer_Type {
	MAX = 0 as isize,
	AVE = 1 as isize,
	STOCHASTIC = 2 as isize,
}

/// Each Layer class must provide this function to the factory
pub type LayerFactory_Constuctor = Option<extern "C" fn(*mut c_void) -> *mut c_void>;
/// Container for strings and integers.
pub type Net_LayerId = crate::dnn::DictValue;
/// Creates the importer of <a href="http://caffe.berkeleyvision.org">Caffe</a> framework network.
/// ## Parameters
/// * prototxt: path to the .prototxt file with text description of the network architecture.
/// * caffeModel: path to the .caffemodel file with learned network.
/// ## Returns
/// Pointer to the created importer, NULL in failure cases.
/// 
/// ## C++ default parameters
/// * caffe_model: String()
pub fn create_caffe_importer(prototxt: &str, caffe_model: &str) -> Result<types::PtrOfImporter> {
	string_arg!(prototxt);
	string_arg!(caffe_model);
	unsafe { sys::cv_dnn_createCaffeImporter_const_StringX_const_StringX(prototxt.as_ptr(), caffe_model.as_ptr()) }.into_result().map(|ptr| types::PtrOfImporter { ptr })
}

/// Creates the importer of <a href="http://www.tensorflow.org">TensorFlow</a> framework network.
/// ## Parameters
/// * model: path to the .pb file with binary protobuf description of the network architecture.
/// ## Returns
/// Pointer to the created importer, NULL in failure cases.
pub fn create_tensorflow_importer(model: &str) -> Result<types::PtrOfImporter> {
	string_arg!(model);
	unsafe { sys::cv_dnn_createTensorflowImporter_const_StringX(model.as_ptr()) }.into_result().map(|ptr| types::PtrOfImporter { ptr })
}

/// Creates the importer of <a href="http://torch.ch">Torch7</a> framework network.
/// ## Parameters
/// * filename: path to the file, dumped from Torch by using torch.save() function.
/// * isBinary: specifies whether the network was serialized in ascii mode or binary.
/// ## Returns
/// Pointer to the created importer, NULL in failure cases.
/// 
///  @warning Torch7 importer is experimental now, you need explicitly set CMake `opencv_dnn_BUILD_TORCH_IMPORTER` flag to compile its.
/// 
///  
/// Note: Ascii mode of Torch serializer is more preferable, because binary mode extensively use `long` type of C language,
///  which has various bit-length on different systems.
/// 
/// The loading file must contain serialized <a href="https://github.com/torch/nn/blob/master/doc/module.md">nn.Module</a> object
/// with importing network. Try to eliminate a custom objects from serialazing data to avoid importing errors.
/// 
/// List of supported layers (i.e. object instances derived from Torch nn.Module class):
/// - nn.Sequential
/// - nn.Parallel
/// - nn.Concat
/// - nn.Linear
/// - nn.SpatialConvolution
/// - nn.SpatialMaxPooling, nn.SpatialAveragePooling
/// - nn.ReLU, nn.TanH, nn.Sigmoid
/// - nn.Reshape
/// 
/// Also some equivalents of these classes from cunn, cudnn, and fbcunn may be successfully imported.
/// 
/// ## C++ default parameters
/// * is_binary: true
pub fn create_torch_importer(filename: &str, is_binary: bool) -> Result<types::PtrOfImporter> {
	string_arg!(filename);
	unsafe { sys::cv_dnn_createTorchImporter_const_StringX_bool(filename.as_ptr(), is_binary) }.into_result().map(|ptr| types::PtrOfImporter { ptr })
}

/// Initialize dnn module and built-in layers.
/// 
/// This function automatically called on most of OpenCV builds,
/// but you need to call it manually on some specific configurations (iOS for example).
pub fn init_module() -> Result<()> {
	unsafe { sys::cv_dnn_initModule() }.into_result()
}

/// Reads a network model stored in Caffe model files.
/// @details This is shortcut consisting from createCaffeImporter and Net::populateNet calls.
/// 
/// ## C++ default parameters
/// * caffe_model: String()
pub fn read_net_from_caffe(prototxt: &str, caffe_model: &str) -> Result<crate::dnn::Net> {
	string_arg!(prototxt);
	string_arg!(caffe_model);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_StringX_const_StringX(prototxt.as_ptr(), caffe_model.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Loads blob which was serialized as torch.Tensor object of Torch7 framework.
/// @warning This function has the same limitations as createTorchImporter().
/// 
/// ## C++ default parameters
/// * is_binary: true
pub fn read_torch_blob(filename: &str, is_binary: bool) -> Result<crate::dnn::Blob> {
	string_arg!(filename);
	unsafe { sys::cv_dnn_readTorchBlob_const_StringX_bool(filename.as_ptr(), is_binary) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
}

pub trait AbsLayer: crate::dnn::Layer {
	fn as_raw_AbsLayer(&self) -> *mut c_void;
}

impl dyn AbsLayer + '_ {
	pub fn create() -> Result<types::PtrOfAbsLayer> {
		unsafe { sys::cv_dnn_AbsLayer_create() }.into_result().map(|ptr| types::PtrOfAbsLayer { ptr })
	}
	
}
pub trait BNLLLayer: crate::dnn::Layer {
	fn as_raw_BNLLLayer(&self) -> *mut c_void;
}

impl dyn BNLLLayer + '_ {
	pub fn create() -> Result<types::PtrOfBNLLLayer> {
		unsafe { sys::cv_dnn_BNLLLayer_create() }.into_result().map(|ptr| types::PtrOfBNLLLayer { ptr })
	}
	
}
pub trait BaseConvolutionLayer: crate::dnn::Layer {
	fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void;
	fn kernel(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_kernel_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: kernel")
	}
	
	fn set_kernel(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setKernel_Size(self.as_raw_BaseConvolutionLayer(), &val) }.into_result().expect("Infallible function failed: set_kernel")
	}
	
	fn stride(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_stride_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: stride")
	}
	
	fn set_stride(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setStride_Size(self.as_raw_BaseConvolutionLayer(), &val) }.into_result().expect("Infallible function failed: set_stride")
	}
	
	fn pad(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_pad_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: pad")
	}
	
	fn set_pad(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setPad_Size(self.as_raw_BaseConvolutionLayer(), &val) }.into_result().expect("Infallible function failed: set_pad")
	}
	
	fn dilation(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_dilation_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: dilation")
	}
	
	fn set_dilation(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setDilation_Size(self.as_raw_BaseConvolutionLayer(), &val) }.into_result().expect("Infallible function failed: set_dilation")
	}
	
	fn pad_mode(&self) -> String {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_padMode_const(self.as_raw_BaseConvolutionLayer()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: pad_mode")
	}
	
	fn set_pad_mode(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setPadMode_String(self.as_raw_BaseConvolutionLayer(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_pad_mode")
	}
	
}

/// This class provides methods for continuous n-dimensional CPU and GPU array processing.
/// 
/// The class is realized as a wrapper over @ref cv::Mat and @ref cv::UMat.
/// It will support methods for switching and logical synchronization between CPU and GPU.
pub trait BlobTrait {
	fn as_raw_Blob(&self) -> *mut c_void;
	/// Works like Blob::fromImages() but in-place.
	/// 
	/// ## C++ default parameters
	/// * dst_cn: -1
	fn batch_from_images(&mut self, image: &dyn core::ToInputArray, dst_cn: i32) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_dnn_Blob_batchFromImages_const__InputArrayX_int(self.as_raw_Blob(), image.as_raw__InputArray(), dst_cn) }.into_result()
	}
	
	/// Creates blob with specified @p shape and @p type.
	/// 
	/// ## C++ default parameters
	/// * typ: CV_32F
	/// * alloc_flags: ALLOC_MAT
	fn create(&mut self, shape: &crate::dnn::BlobShape, typ: i32, alloc_flags: i32) -> Result<()> {
		unsafe { sys::cv_dnn_Blob_create_const_BlobShapeX_int_int(self.as_raw_Blob(), shape.as_raw_BlobShape(), typ, alloc_flags) }.into_result()
	}
	
	/// Creates blob from Mat or UMat without copying the data.
	/// @details If in is Mat then Mat data is populated, otherwise - UMat.
	fn fill(&mut self, in_: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(in_);
		unsafe { sys::cv_dnn_Blob_fill_const__InputArrayX(self.as_raw_Blob(), in_.as_raw__InputArray()) }.into_result()
	}
	
	/// Creates blob from user data.
	/// @details If @p deepCopy is false then CPU data will not be allocated.
	/// 
	/// ## C++ default parameters
	/// * deep_copy: true
	fn fill_1(&mut self, shape: &crate::dnn::BlobShape, typ: i32, data: *mut c_void, deep_copy: bool) -> Result<()> {
		unsafe { sys::cv_dnn_Blob_fill_const_BlobShapeX_int_voidX_bool(self.as_raw_Blob(), shape.as_raw_BlobShape(), typ, data, deep_copy) }.into_result()
	}
	
	/// Sets @p value to the last used data (if @p allocFlags = -1).
	/// @details If @p allocFlags != -1 then destination data (Mat or UMat) is determined by flags from AllocFlag enum like in create().
	/// 
	/// ## C++ default parameters
	/// * alloc_flags: -1
	fn set_to(&mut self, value: &dyn core::ToInputArray, alloc_flags: i32) -> Result<()> {
		input_array_arg!(value);
		unsafe { sys::cv_dnn_Blob_setTo_const__InputArrayX_int(self.as_raw_Blob(), value.as_raw__InputArray(), alloc_flags) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * write_only: true
	fn mat_ref(&mut self, write_only: bool) -> Result<core::Mat> {
		unsafe { sys::cv_dnn_Blob_matRef_bool(self.as_raw_Blob(), write_only) }.into_result().map(|ptr| core::Mat { ptr })
	}
	
	fn mat_ref_const(&self) -> Result<core::Mat> {
		unsafe { sys::cv_dnn_Blob_matRefConst_const(self.as_raw_Blob()) }.into_result().map(|ptr| core::Mat { ptr })
	}
	
	/// ## C++ default parameters
	/// * write_only: true
	fn umat_ref(&mut self, write_only: bool) -> Result<core::UMat> {
		unsafe { sys::cv_dnn_Blob_umatRef_bool(self.as_raw_Blob(), write_only) }.into_result().map(|ptr| core::UMat { ptr })
	}
	
	fn umat_ref_const(&self) -> Result<core::UMat> {
		unsafe { sys::cv_dnn_Blob_umatRefConst_const(self.as_raw_Blob()) }.into_result().map(|ptr| core::UMat { ptr })
	}
	
	/// ## C++ default parameters
	/// * sync_data: true
	fn update_mat(&self, sync_data: bool) -> Result<()> {
		unsafe { sys::cv_dnn_Blob_updateMat_const_bool(self.as_raw_Blob(), sync_data) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * sync_data: true
	fn update_umat(&self, sync_data: bool) -> Result<()> {
		unsafe { sys::cv_dnn_Blob_updateUMat_const_bool(self.as_raw_Blob(), sync_data) }.into_result()
	}
	
	fn sync(&self) -> Result<()> {
		unsafe { sys::cv_dnn_Blob_sync_const(self.as_raw_Blob()) }.into_result()
	}
	
	/// Returns number of blob dimensions.
	fn dims(&self) -> Result<i32> {
		unsafe { sys::cv_dnn_Blob_dims_const(self.as_raw_Blob()) }.into_result()
	}
	
	/// Returns the size of the specified @p axis.
	/// 
	/// Negative @p axis is supported, in this case a counting starts from the last axis,
	/// i. e. -1 corresponds to last axis.
	/// If non-existing axis was passed then an error will be generated.
	fn size(&self, axis: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_Blob_size_const_int(self.as_raw_Blob(), axis) }.into_result()
	}
	
	/// Returns the size of the specified @p axis.
	/// 
	/// Does the same thing as size(int) const, but if non-existing axis will be passed then 1 will be returned,
	/// therefore this function always finishes successfully.
	fn xsize(&self, axis: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_Blob_xsize_const_int(self.as_raw_Blob(), axis) }.into_result()
	}
	
	/// Computes the product of sizes of axes among the specified axes range [@p startAxis; @p endAxis).
	/// ## Parameters
	/// * startAxis: the first axis to include in the range.
	/// * endAxis: the first axis to exclude from the range.
	/// @details Negative axis indexing can be used.
	/// 
	/// ## C++ default parameters
	/// * start_axis: 0
	/// * end_axis: INT_MAX
	fn total(&self, start_axis: i32, end_axis: i32) -> Result<size_t> {
		unsafe { sys::cv_dnn_Blob_total_const_int_int(self.as_raw_Blob(), start_axis, end_axis) }.into_result()
	}
	
	/// Converts @p axis index to canonical format (where 0 <= @p axis < dims()).
	fn canonical_axis(&self, axis: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_Blob_canonicalAxis_const_int(self.as_raw_Blob(), axis) }.into_result()
	}
	
	/// Returns shape of the blob.
	fn shape(&self) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_Blob_shape_const(self.as_raw_Blob()) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	/// Checks equality of two blobs shapes.
	fn equal_shape(&self, other: &crate::dnn::Blob) -> Result<bool> {
		unsafe { sys::cv_dnn_Blob_equalShape_const_const_BlobX(self.as_raw_Blob(), other.as_raw_Blob()) }.into_result()
	}
	
	/// Returns slice of first two dimensions.
	/// @details The behaviour is similar to the following numpy code: blob[n, cn, ...]
	fn get_plane(&mut self, n: i32, cn: i32) -> Result<core::Mat> {
		unsafe { sys::cv_dnn_Blob_getPlane_int_int(self.as_raw_Blob(), n, cn) }.into_result().map(|ptr| core::Mat { ptr })
	}
	
	/// Returns slice of first dimension.
	///  @details The behaviour is similar to getPlane(), but returns all
	/// channels * rows * cols values, corresponding to the n-th value
	/// of the first dimension.
	fn get_planes(&mut self, n: i32) -> Result<core::Mat> {
		unsafe { sys::cv_dnn_Blob_getPlanes_int(self.as_raw_Blob(), n) }.into_result().map(|ptr| core::Mat { ptr })
	}
	
	fn cols(&self) -> Result<i32> {
		unsafe { sys::cv_dnn_Blob_cols_const(self.as_raw_Blob()) }.into_result()
	}
	
	fn rows(&self) -> Result<i32> {
		unsafe { sys::cv_dnn_Blob_rows_const(self.as_raw_Blob()) }.into_result()
	}
	
	fn channels(&self) -> Result<i32> {
		unsafe { sys::cv_dnn_Blob_channels_const(self.as_raw_Blob()) }.into_result()
	}
	
	fn num(&self) -> Result<i32> {
		unsafe { sys::cv_dnn_Blob_num_const(self.as_raw_Blob()) }.into_result()
	}
	
	fn size2(&self) -> Result<core::Size> {
		unsafe { sys::cv_dnn_Blob_size2_const(self.as_raw_Blob()) }.into_result()
	}
	
	fn shape4(&self) -> Result<core::Vec4i> {
		unsafe { sys::cv_dnn_Blob_shape4_const(self.as_raw_Blob()) }.into_result()
	}
	
	/// Returns linear index of the element with specified coordinates in the blob.
	/// 
	/// If @p n < dims() then unspecified coordinates will be filled by zeros.
	/// If @p n > dims() then extra coordinates will be ignored.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * n: 0
	/// * cn: 0
	/// * row: 0
	/// * col: 0
	fn offset(&self, n: i32, cn: i32, row: i32, col: i32) -> Result<size_t> {
		unsafe { sys::cv_dnn_Blob_offset_const_int_int_int_int(self.as_raw_Blob(), n, cn, row, col) }.into_result()
	}
	
	/// Returns pointer to the blob element with the specified position, stored in CPU memory.
	/// 
	/// @p n correspond to the first axis, @p cn - to the second, etc.
	/// If dims() > 4 then unspecified coordinates will be filled by zeros.
	/// If dims() < 4 then extra coordinates will be ignored.
	/// 
	/// ## C++ default parameters
	/// * n: 0
	/// * cn: 0
	/// * row: 0
	/// * col: 0
	fn ptr(&mut self, n: i32, cn: i32, row: i32, col: i32) -> Result<&mut u8> {
		unsafe { sys::cv_dnn_Blob_ptr_int_int_int_int(self.as_raw_Blob(), n, cn, row, col) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// ptr<float>()
	/// 
	/// ## C++ default parameters
	/// * n: 0
	/// * cn: 0
	/// * row: 0
	/// * col: 0
	fn ptrf(&mut self, n: i32, cn: i32, row: i32, col: i32) -> Result<&mut f32> {
		unsafe { sys::cv_dnn_Blob_ptrf_int_int_int_int(self.as_raw_Blob(), n, cn, row, col) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Shares data from other @p blob.
	/// ## Returns
	/// *this
	fn share_from(&mut self, blob: &crate::dnn::Blob) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Blob_shareFrom_const_BlobX(self.as_raw_Blob(), blob.as_raw_Blob()) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
	}
	
	/// Changes shape of the blob without copying the data.
	/// ## Returns
	/// *this
	fn reshape(&mut self, shape: &crate::dnn::BlobShape) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Blob_reshape_const_BlobShapeX(self.as_raw_Blob(), shape.as_raw_BlobShape()) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
	}
	
	/// Changes shape of the blob without copying the data.
	/// ## Returns
	/// shallow copy of original blob with new shape.
	fn reshaped(&self, new_shape: &crate::dnn::BlobShape) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Blob_reshaped_const_const_BlobShapeX(self.as_raw_Blob(), new_shape.as_raw_BlobShape()) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
	}
	
	fn typ(&self) -> Result<i32> {
		unsafe { sys::cv_dnn_Blob_type_const(self.as_raw_Blob()) }.into_result()
	}
	
	fn elem_size(&self) -> Result<i32> {
		unsafe { sys::cv_dnn_Blob_elemSize_const(self.as_raw_Blob()) }.into_result()
	}
	
	fn get_state(&self) -> Result<i32> {
		unsafe { sys::cv_dnn_Blob_getState_const(self.as_raw_Blob()) }.into_result()
	}
	
}

/// This class provides methods for continuous n-dimensional CPU and GPU array processing.
/// 
/// The class is realized as a wrapper over @ref cv::Mat and @ref cv::UMat.
/// It will support methods for switching and logical synchronization between CPU and GPU.
pub struct Blob {
	pub(crate) ptr: *mut c_void
}

impl Drop for Blob {
	fn drop(&mut self) {
		extern "C" { fn cv_Blob_delete(instance: *mut c_void); }
		unsafe { cv_Blob_delete(self.as_raw_Blob()) };
	}
}

impl Blob {
	pub fn as_raw_Blob(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Blob {}

impl crate::dnn::BlobTrait for Blob {
	fn as_raw_Blob(&self) -> *mut c_void { self.ptr }
}

impl Blob {
	pub fn default() -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Blob_Blob() }.into_result().map(|ptr| crate::dnn::Blob { ptr })
	}
	
	/// Constructs blob with specified @p shape and @p type.
	/// 
	/// ## C++ default parameters
	/// * typ: CV_32F
	/// * alloc_flags: ALLOC_MAT
	pub fn new(shape: &crate::dnn::BlobShape, typ: i32, alloc_flags: i32) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Blob_Blob_const_BlobShapeX_int_int(shape.as_raw_BlobShape(), typ, alloc_flags) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
	}
	
	/// Constructs Blob from existing Mat or UMat.
	pub fn new_1(data: &dyn core::ToInputArray) -> Result<crate::dnn::Blob> {
		input_array_arg!(data);
		unsafe { sys::cv_dnn_Blob_Blob_const__InputArrayX(data.as_raw__InputArray()) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
	}
	
	/// Constructs 4-dimensional blob (so-called batch) from image or array of images.
	/// ## Parameters
	/// * image: 2-dimensional multi-channel or 3-dimensional single-channel image (or array of such images)
	/// * dstCn: specifies size of second axis of ouptut blob
	/// 
	/// ## C++ default parameters
	/// * dst_cn: -1
	pub fn from_images(image: &dyn core::ToInputArray, dst_cn: i32) -> Result<crate::dnn::Blob> {
		input_array_arg!(image);
		unsafe { sys::cv_dnn_Blob_fromImages_const__InputArrayX_int(image.as_raw__InputArray(), dst_cn) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
	}
	
}

/// Lightweight class for storing and processing a shape of blob (or anything else).
pub trait BlobShapeTrait {
	fn as_raw_BlobShape(&self) -> *mut c_void;
	/// Returns number of dimensions.
	fn dims(&self) -> Result<i32> {
		unsafe { sys::cv_dnn_BlobShape_dims_const(self.as_raw_BlobShape()) }.into_result()
	}
	
	/// Returns reference to the size of the specified @p axis.
	/// 
	/// Negative @p axis is supported, in this case a counting starts from the last axis,
	/// i. e. -1 corresponds to last axis.
	/// If non-existing axis was passed then an error will be generated.
	fn size(&mut self, axis: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_BlobShape_size_int(self.as_raw_BlobShape(), axis) }.into_result()
	}
	
	/// Returns the size of the specified @p axis.
	/// ## See also
	/// size()
	fn size_1(&self, axis: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_BlobShape_size_const_int(self.as_raw_BlobShape(), axis) }.into_result()
	}
	
	fn get(&self, axis: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_BlobShape_operator___const_int(self.as_raw_BlobShape(), axis) }.into_result()
	}
	
	fn get_mut(&mut self, axis: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_BlobShape_operator___int(self.as_raw_BlobShape(), axis) }.into_result()
	}
	
	/// Returns the size of the specified @p axis.
	/// 
	/// Does the same thing as size(int) const, but if non-existing axis will be passed then 1 will be returned,
	/// therefore this function always finishes successfully.
	fn xsize(&self, axis: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_BlobShape_xsize_const_int(self.as_raw_BlobShape(), axis) }.into_result()
	}
	
	/// Converts @p axis index to canonical format (where 0 <= @p axis < dims()).
	fn canonical_axis(&self, axis: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_BlobShape_canonicalAxis_const_int(self.as_raw_BlobShape(), axis) }.into_result()
	}
	
	/// Returns the product of all sizes of axes.
	fn total(&self) -> Result<ptrdiff_t> {
		unsafe { sys::cv_dnn_BlobShape_total_const(self.as_raw_BlobShape()) }.into_result()
	}
	
	/// Computes the product of sizes of axes among the specified axes range [@p startAxis; @p endAxis).
	/// @details Negative axis indexing can be used. see also: Blob::total(int,int)
	/// 
	/// ## C++ default parameters
	/// * end_axis: INT_MAX
	fn total_1(&self, start_axis: i32, end_axis: i32) -> Result<ptrdiff_t> {
		unsafe { sys::cv_dnn_BlobShape_total_const_int_int(self.as_raw_BlobShape(), start_axis, end_axis) }.into_result()
	}
	
	/// Constructs new shape from axes in range [@p startAxis; @p endAxis).
	/// @details Negative axis indexing can be used. see also: Blob::total(int,int)
	/// 
	/// ## C++ default parameters
	/// * end_axis: INT_MAX
	fn slice(&self, start_axis: i32, end_axis: i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_slice_const_int_int(self.as_raw_BlobShape(), start_axis, end_axis) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	/// Returns pointer to the first element of continuous size array.
	fn ptr(&self) -> Result<&i32> {
		unsafe { sys::cv_dnn_BlobShape_ptr_const(self.as_raw_BlobShape()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Returns pointer to the first element of continuous size array.
	/// 
	/// ## Overloaded parameters
	fn ptr_1(&mut self) -> Result<&mut i32> {
		unsafe { sys::cv_dnn_BlobShape_ptr(self.as_raw_BlobShape()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	fn equal(&self, other: &crate::dnn::BlobShape) -> Result<bool> {
		unsafe { sys::cv_dnn_BlobShape_equal_const_const_BlobShapeX(self.as_raw_BlobShape(), other.as_raw_BlobShape()) }.into_result()
	}
	
	fn is_empty(&self) -> Result<bool> {
		unsafe { sys::cv_dnn_BlobShape_isEmpty_const(self.as_raw_BlobShape()) }.into_result()
	}
	
}

/// Lightweight class for storing and processing a shape of blob (or anything else).
pub struct BlobShape {
	pub(crate) ptr: *mut c_void
}

impl Drop for BlobShape {
	fn drop(&mut self) {
		extern "C" { fn cv_BlobShape_delete(instance: *mut c_void); }
		unsafe { cv_BlobShape_delete(self.as_raw_BlobShape()) };
	}
}

impl BlobShape {
	pub fn as_raw_BlobShape(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for BlobShape {}

impl crate::dnn::BlobShapeTrait for BlobShape {
	fn as_raw_BlobShape(&self) -> *mut c_void { self.ptr }
}

impl BlobShape {
	pub fn default() -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape() }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	pub fn new(s0: i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_int(s0) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	pub fn new_1(s0: i32, s1: i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_int_int(s0, s1) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	pub fn new_2(s0: i32, s1: i32, s2: i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_int_int_int(s0, s1, s2) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	pub fn new_3(num: i32, cn: i32, rows: i32, cols: i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_int_int_int_int(num, cn, rows, cols) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	/// Creates n-dim shape from the @p sizes array; if @p sizes is NULL then shape will contain unspecified data
	pub fn new_4(ndims: i32, sizes: &i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_int_const_intX(ndims, sizes) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	pub fn new_5(sizes: &types::VectorOfi32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_const_vector_int_X(sizes.as_raw_VectorOfi32()) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	/// Creates n-dim shape and fill its by @p fill
	/// 
	/// ## C++ default parameters
	/// * fill: 1
	pub fn all(ndims: i32, fill: i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_all_int_int(ndims, fill) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	pub fn like(m: &core::Mat) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_like_const_MatX(m.as_raw_Mat()) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	pub fn like_1(m: &core::UMat) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_like_const_UMatX(m.as_raw_UMat()) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
	pub fn empty() -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_empty() }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
	}
	
}

pub trait ConcatLayer: crate::dnn::Layer {
	fn as_raw_ConcatLayer(&self) -> *mut c_void;
	fn axis(&self) -> i32 {
		unsafe { sys::cv_dnn_ConcatLayer_axis_const(self.as_raw_ConcatLayer()) }.into_result().expect("Infallible function failed: axis")
	}
	
	fn set_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_ConcatLayer_setAxis_int(self.as_raw_ConcatLayer(), val) }.into_result().expect("Infallible function failed: set_axis")
	}
	
}

impl dyn ConcatLayer + '_ {
	/// ## C++ default parameters
	/// * axis: 1
	pub fn create(axis: i32) -> Result<types::PtrOfConcatLayer> {
		unsafe { sys::cv_dnn_ConcatLayer_create_int(axis) }.into_result().map(|ptr| types::PtrOfConcatLayer { ptr })
	}
	
}
pub trait ConvolutionLayer: crate::dnn::BaseConvolutionLayer {
	fn as_raw_ConvolutionLayer(&self) -> *mut c_void;
}

impl dyn ConvolutionLayer + '_ {
	/// ## C++ default parameters
	/// * kernel: Size(3,3)
	/// * stride: Size(1,1)
	/// * pad: Size(0,0)
	/// * dilation: Size(1,1)
	pub fn create(kernel: core::Size, stride: core::Size, pad: core::Size, dilation: core::Size) -> Result<types::PtrOfBaseConvolutionLayer> {
		unsafe { sys::cv_dnn_ConvolutionLayer_create_Size_Size_Size_Size(&kernel, &stride, &pad, &dilation) }.into_result().map(|ptr| types::PtrOfBaseConvolutionLayer { ptr })
	}
	
}
pub trait CropLayer: crate::dnn::Layer {
	fn as_raw_CropLayer(&self) -> *mut c_void;
	fn start_axis(&self) -> i32 {
		unsafe { sys::cv_dnn_CropLayer_startAxis_const(self.as_raw_CropLayer()) }.into_result().expect("Infallible function failed: start_axis")
	}
	
	fn set_start_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_CropLayer_setStartAxis_int(self.as_raw_CropLayer(), val) }.into_result().expect("Infallible function failed: set_start_axis")
	}
	
	fn offset(&mut self) -> types::VectorOfi32 {
		unsafe { sys::cv_dnn_CropLayer_offset(self.as_raw_CropLayer()) }.into_result().map(|ptr| types::VectorOfi32 { ptr }).expect("Infallible function failed: offset")
	}
	
	fn set_offset(&mut self, val: types::VectorOfi32) -> () {
		unsafe { sys::cv_dnn_CropLayer_setOffset_vector_int_(self.as_raw_CropLayer(), val.as_raw_VectorOfi32()) }.into_result().expect("Infallible function failed: set_offset")
	}
	
}

impl dyn CropLayer + '_ {
	pub fn create(start_axis: i32, offset: &types::VectorOfi32) -> Result<types::PtrOfCropLayer> {
		unsafe { sys::cv_dnn_CropLayer_create_int_const_vector_int_X(start_axis, offset.as_raw_VectorOfi32()) }.into_result().map(|ptr| types::PtrOfCropLayer { ptr })
	}
	
}
pub trait DeconvolutionLayer: crate::dnn::BaseConvolutionLayer {
	fn as_raw_DeconvolutionLayer(&self) -> *mut c_void;
}

impl dyn DeconvolutionLayer + '_ {
	/// ## C++ default parameters
	/// * kernel: Size(3,3)
	/// * stride: Size(1,1)
	/// * pad: Size(0,0)
	/// * dilation: Size(1,1)
	pub fn create(kernel: core::Size, stride: core::Size, pad: core::Size, dilation: core::Size) -> Result<types::PtrOfBaseConvolutionLayer> {
		unsafe { sys::cv_dnn_DeconvolutionLayer_create_Size_Size_Size_Size(&kernel, &stride, &pad, &dilation) }.into_result().map(|ptr| types::PtrOfBaseConvolutionLayer { ptr })
	}
	
}
/// This class implements name-value dictionary, values are instances of DictValue.
pub trait DictTrait {
	fn as_raw_Dict(&self) -> *mut c_void;
	/// Checks a presence of the @p key in the dictionary.
	fn has(&self, key: &str) -> Result<bool> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_has_const_const_StringX(self.as_raw_Dict(), key.as_ptr()) }.into_result()
	}
	
	/// If the @p key in the dictionary then returns pointer to its value, else returns NULL.
	fn ptr(&mut self, key: &str) -> Result<crate::dnn::DictValue> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_ptr_const_StringX(self.as_raw_Dict(), key.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	/// If the @p key in the dictionary then returns its value, else an error will be generated.
	fn get(&self, key: &str) -> Result<crate::dnn::DictValue> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_get_const_const_StringX(self.as_raw_Dict(), key.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set_str(&mut self, key: &str, value: &str) -> Result<String> {
		string_arg!(key);
		string_arg!(value);
		unsafe { sys::cv_dnn_Dict_set_cv_String_const_StringX_const_StringX(self.as_raw_Dict(), key.as_ptr(), value.as_ptr()) }.into_result().map(crate::templ::receive_string)
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set(&mut self, key: &str, value: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_set_cv_dnn_DictValue_const_StringX_const_DictValueX(self.as_raw_Dict(), key.as_ptr(), value.as_raw_DictValue()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set_f64(&mut self, key: &str, value: &f64) -> Result<f64> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_set_double_const_StringX_const_doubleX(self.as_raw_Dict(), key.as_ptr(), value) }.into_result()
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set_i64(&mut self, key: &str, value: &i64) -> Result<i64> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_set_int64_t_const_StringX_const_int64_tX(self.as_raw_Dict(), key.as_ptr(), value) }.into_result()
	}
	
}

/// This class implements name-value dictionary, values are instances of DictValue.
pub struct Dict {
	pub(crate) ptr: *mut c_void
}

impl Drop for Dict {
	fn drop(&mut self) {
		extern "C" { fn cv_Dict_delete(instance: *mut c_void); }
		unsafe { cv_Dict_delete(self.as_raw_Dict()) };
	}
}

impl Dict {
	pub fn as_raw_Dict(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Dict {}

impl crate::dnn::DictTrait for Dict {
	fn as_raw_Dict(&self) -> *mut c_void { self.ptr }
}

impl Dict {
}

/// This struct stores the scalar value (or array) of one of the following type: double, cv::String or int64.
/// @todo Maybe int64 is useless because double type exactly stores at least 2^52 integers.
pub trait DictValueTrait {
	fn as_raw_DictValue(&self) -> *mut c_void;
	/// ## C++ default parameters
	/// * idx: -1
	fn get_str(&self, idx: i32) -> Result<String> {
		unsafe { sys::cv_dnn_DictValue_get_cv_String_const_int(self.as_raw_DictValue(), idx) }.into_result().map(crate::templ::receive_string)
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	fn get_f64(&self, idx: i32) -> Result<f64> {
		unsafe { sys::cv_dnn_DictValue_get_double_const_int(self.as_raw_DictValue(), idx) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	fn get_i32(&self, idx: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_DictValue_get_int_const_int(self.as_raw_DictValue(), idx) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	fn get_i64(&self, idx: i32) -> Result<i64> {
		unsafe { sys::cv_dnn_DictValue_get_int64_t_const_int(self.as_raw_DictValue(), idx) }.into_result()
	}
	
	fn size(&self) -> Result<i32> {
		unsafe { sys::cv_dnn_DictValue_size_const(self.as_raw_DictValue()) }.into_result()
	}
	
	fn is_int(&self) -> Result<bool> {
		unsafe { sys::cv_dnn_DictValue_isInt_const(self.as_raw_DictValue()) }.into_result()
	}
	
	fn is_string(&self) -> Result<bool> {
		unsafe { sys::cv_dnn_DictValue_isString_const(self.as_raw_DictValue()) }.into_result()
	}
	
	fn is_real(&self) -> Result<bool> {
		unsafe { sys::cv_dnn_DictValue_isReal_const(self.as_raw_DictValue()) }.into_result()
	}
	
}

/// This struct stores the scalar value (or array) of one of the following type: double, cv::String or int64.
/// @todo Maybe int64 is useless because double type exactly stores at least 2^52 integers.
pub struct DictValue {
	pub(crate) ptr: *mut c_void
}

impl Drop for DictValue {
	fn drop(&mut self) {
		extern "C" { fn cv_DictValue_delete(instance: *mut c_void); }
		unsafe { cv_DictValue_delete(self.as_raw_DictValue()) };
	}
}

impl DictValue {
	pub fn as_raw_DictValue(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for DictValue {}

impl crate::dnn::DictValueTrait for DictValue {
	fn as_raw_DictValue(&self) -> *mut c_void { self.ptr }
}

impl DictValue {
	pub fn copy(r: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_const_DictValueX(r.as_raw_DictValue()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	/// ## C++ default parameters
	/// * i: 0
	pub fn from_i64(i: i64) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_int64_t(i) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	pub fn from_i32(i: i32) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_int(i) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	pub fn from_u32(p: u32) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_unsigned_int(p) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	pub fn from_f64(p: f64) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_double(p) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	pub fn from_str(s: &str) -> Result<crate::dnn::DictValue> {
		string_arg!(s);
		unsafe { sys::cv_dnn_DictValue_DictValue_const_charX(s.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
}

pub trait EltwiseLayer: crate::dnn::Layer {
	fn as_raw_EltwiseLayer(&self) -> *mut c_void;
}

impl dyn EltwiseLayer + '_ {
	pub fn create(op: crate::dnn::EltwiseLayer_EltwiseOp, coeffs: &types::VectorOfi32) -> Result<types::PtrOfEltwiseLayer> {
		unsafe { sys::cv_dnn_EltwiseLayer_create_EltwiseOp_const_vector_int_X(op, coeffs.as_raw_VectorOfi32()) }.into_result().map(|ptr| types::PtrOfEltwiseLayer { ptr })
	}
	
}
/// Small interface class for loading trained serialized models of different dnn-frameworks.
pub trait Importer {
	fn as_raw_Importer(&self) -> *mut c_void;
	/// Adds loaded layers into the @p net and sets connections between them.
	fn populate_net(&mut self, net: crate::dnn::Net) -> Result<()> {
		unsafe { sys::cv_dnn_Importer_populateNet_Net(self.as_raw_Importer(), net.as_raw_Net()) }.into_result()
	}
	
}

pub trait InnerProductLayer: crate::dnn::Layer {
	fn as_raw_InnerProductLayer(&self) -> *mut c_void;
	fn axis(&self) -> i32 {
		unsafe { sys::cv_dnn_InnerProductLayer_axis_const(self.as_raw_InnerProductLayer()) }.into_result().expect("Infallible function failed: axis")
	}
	
	fn set_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_InnerProductLayer_setAxis_int(self.as_raw_InnerProductLayer(), val) }.into_result().expect("Infallible function failed: set_axis")
	}
	
}

impl dyn InnerProductLayer + '_ {
	/// ## C++ default parameters
	/// * axis: 1
	pub fn create(axis: i32) -> Result<types::PtrOfInnerProductLayer> {
		unsafe { sys::cv_dnn_InnerProductLayer_create_int(axis) }.into_result().map(|ptr| types::PtrOfInnerProductLayer { ptr })
	}
	
}
pub trait LRNLayer: crate::dnn::Layer {
	fn as_raw_LRNLayer(&self) -> *mut c_void;
	fn typ(&self) -> i32 {
		unsafe { sys::cv_dnn_LRNLayer_type_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: typ")
	}
	
	fn set_type(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setType_int(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_type")
	}
	
	fn size(&self) -> i32 {
		unsafe { sys::cv_dnn_LRNLayer_size_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: size")
	}
	
	fn set_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setSize_int(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_size")
	}
	
	fn alpha(&self) -> f64 {
		unsafe { sys::cv_dnn_LRNLayer_alpha_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: alpha")
	}
	
	fn set_alpha(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setAlpha_double(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_alpha")
	}
	
	fn beta(&self) -> f64 {
		unsafe { sys::cv_dnn_LRNLayer_beta_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: beta")
	}
	
	fn set_beta(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setBeta_double(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_beta")
	}
	
	fn bias(&self) -> f64 {
		unsafe { sys::cv_dnn_LRNLayer_bias_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: bias")
	}
	
	fn set_bias(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setBias_double(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_bias")
	}
	
	fn norm_by_size(&self) -> bool {
		unsafe { sys::cv_dnn_LRNLayer_normBySize_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: norm_by_size")
	}
	
	fn set_norm_by_size(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setNormBySize_bool(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_norm_by_size")
	}
	
}

impl dyn LRNLayer + '_ {
	/// ## C++ default parameters
	/// * typ: LRNLayer::CHANNEL_NRM
	/// * size: 5
	/// * alpha: 1
	/// * beta: 0.75
	/// * bias: 1
	/// * norm_by_size: true
	pub fn create(typ: i32, size: i32, alpha: f64, beta: f64, bias: f64, norm_by_size: bool) -> Result<types::PtrOfLRNLayer> {
		unsafe { sys::cv_dnn_LRNLayer_create_int_int_double_double_double_bool(typ, size, alpha, beta, bias, norm_by_size) }.into_result().map(|ptr| types::PtrOfLRNLayer { ptr })
	}
	
}
/// LSTM recurrent layer
pub trait LSTMLayer: crate::dnn::Layer {
	fn as_raw_LSTMLayer(&self) -> *mut c_void;
	/// Set trained weights for LSTM layer.
	/// LSTM behavior on each step is defined by current input, previous output, previous cell state and learned weights.
	/// 
	/// Let @f$x_t@f$ be current input, @f$h_t@f$ be current output, @f$c_t@f$ be current state.
	/// Than current output and current cell state is computed as follows:
	/// @f{eqnarray*}{
	/// h_t &= o_t \odot tanh(c_t),               \\
	/// c_t &= f_t \odot c_{t-1} + i_t \odot g_t, \\
	/// @f}
	/// where @f$\odot@f$ is per-element multiply operation and @f$i_t, f_t, o_t, g_t@f$ is internal gates that are computed using learned wights.
	/// 
	/// Gates are computed as follows:
	/// @f{eqnarray*}{
	/// i_t &= sigmoid&(W_{xi} x_t + W_{hi} h_{t-1} + b_i), \\
	/// f_t &= sigmoid&(W_{xf} x_t + W_{hf} h_{t-1} + b_f), \\
	/// o_t &= sigmoid&(W_{xo} x_t + W_{ho} h_{t-1} + b_o), \\
	/// g_t &= tanh   &(W_{xg} x_t + W_{hg} h_{t-1} + b_g), \\
	/// @f}
	/// where @f$W_{x?}@f$, @f$W_{h?}@f$ and @f$b_{?}@f$ are learned weights represented as matrices:
	/// @f$W_{x?} \in R^{N_h \times N_x}@f$, @f$W_{h?} \in R^{N_h \times N_h}@f$, @f$b_? \in R^{N_h}@f$.
	/// 
	/// For simplicity and performance purposes we use @f$ W_x = [W_{xi}; W_{xf}; W_{xo}, W_{xg}] @f$
	/// (i.e. @f$W_x@f$ is vertical contacentaion of @f$ W_{x?} @f$), @f$ W_x \in R^{4N_h \times N_x} @f$.
	/// The same for @f$ W_h = [W_{hi}; W_{hf}; W_{ho}, W_{hg}], W_h \in R^{4N_h \times N_h} @f$
	/// and for @f$ b = [b_i; b_f, b_o, b_g]@f$, @f$b \in R^{4N_h} @f$.
	/// 
	/// ## Parameters
	/// * Wh: is matrix defining how previous output is transformed to internal gates (i.e. according to abovemtioned notation is @f$ W_h @f$)
	/// * Wx: is matrix defining how current input is transformed to internal gates (i.e. according to abovemtioned notation is @f$ W_x @f$)
	/// * b: is bias vector (i.e. according to abovemtioned notation is @f$ b @f$)
	fn set_weights(&mut self, wh: &crate::dnn::Blob, wx: &crate::dnn::Blob, b: &crate::dnn::Blob) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setWeights_const_BlobX_const_BlobX_const_BlobX(self.as_raw_LSTMLayer(), wh.as_raw_Blob(), wx.as_raw_Blob(), b.as_raw_Blob()) }.into_result()
	}
	
	/// Specifies shape of output blob which will be [[`T`], `N`] + @p outTailShape.
	/// @details If this parameter is empty or unset then @p outTailShape = [`Wh`.size(0)] will be used,
	/// where `Wh` is parameter from setWeights().
	/// 
	/// ## C++ default parameters
	/// * out_tail_shape: BlobShape::empty()
	fn set_out_shape(&mut self, out_tail_shape: &crate::dnn::BlobShape) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setOutShape_const_BlobShapeX(self.as_raw_LSTMLayer(), out_tail_shape.as_raw_BlobShape()) }.into_result()
	}
	
	/// Set @f$ h_{t-1} @f$ value that will be used in next forward() calls.
	/// @details By-default @f$ h_{t-1} @f$ is inited by zeros and updated after each forward() call.
	fn set_h(&mut self, h: &crate::dnn::Blob) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setH_const_BlobX(self.as_raw_LSTMLayer(), h.as_raw_Blob()) }.into_result()
	}
	
	/// Returns current @f$ h_{t-1} @f$ value (deep copy).
	fn get_h(&self) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_LSTMLayer_getH_const(self.as_raw_LSTMLayer()) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
	}
	
	/// Set @f$ c_{t-1} @f$ value that will be used in next forward() calls.
	/// @details By-default @f$ c_{t-1} @f$ is inited by zeros and updated after each forward() call.
	fn set_c(&mut self, c: &crate::dnn::Blob) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setC_const_BlobX(self.as_raw_LSTMLayer(), c.as_raw_Blob()) }.into_result()
	}
	
	/// Returns current @f$ c_{t-1} @f$ value (deep copy).
	fn get_c(&self) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_LSTMLayer_getC_const(self.as_raw_LSTMLayer()) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
	}
	
	/// Specifies either interpet first dimension of input blob as timestamp dimenion either as sample.
	/// 
	/// If flag is set to true then shape of input blob will be interpeted as [`T`, `N`, `[data dims]`] where `T` specifies number of timpestamps, `N` is number of independent streams.
	/// In this case each forward() call will iterate through `T` timestamps and update layer's state `T` times.
	/// 
	/// If flag is set to false then shape of input blob will be interpeted as [`N`, `[data dims]`].
	/// In this case each forward() call will make one iteration and produce one timestamp with shape [`N`, `[out dims]`].
	/// 
	/// ## C++ default parameters
	/// * use_: true
	fn set_use_timstamps_dim(&mut self, use_: bool) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setUseTimstampsDim_bool(self.as_raw_LSTMLayer(), use_) }.into_result()
	}
	
	/// If this flag is set to true then layer will produce @f$ c_t @f$ as second output.
	/// @details Shape of the second output is the same as first output.
	/// 
	/// ## C++ default parameters
	/// * produce: false
	fn set_produce_cell_output(&mut self, produce: bool) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setProduceCellOutput_bool(self.as_raw_LSTMLayer(), produce) }.into_result()
	}
	
	/// In common case it use single input with @f$x_t@f$ values to compute output(s) @f$h_t@f$ (and @f$c_t@f$).
	/// ## Parameters
	/// * input: should contain packed values @f$x_t@f$
	/// * output: contains computed outputs: @f$h_t@f$ (and @f$c_t@f$ if setProduceCellOutput() flag was set to true).
	/// 
	/// If setUseTimstampsDim() is set to true then @p input[0] should has at least two dimensions with the following shape: [`T`, `N`, `[data dims]`],
	/// where `T` specifies number of timpestamps, `N` is number of independent streams (i.e. @f$ x_{t_0 + t}^{stream} @f$ is stored inside @p input[0][t, stream, ...]).
	/// 
	/// If setUseTimstampsDim() is set to fase then @p input[0] should contain single timestamp, its shape should has form [`N`, `[data dims]`] with at least one dimension.
	/// (i.e. @f$ x_{t}^{stream} @f$ is stored inside @p input[0][stream, ...]).
	fn forward(&mut self, input: &mut types::VectorOfBlob, output: &mut types::VectorOfBlob) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_forward_vector_BlobX_X_vector_Blob_X(self.as_raw_LSTMLayer(), input.as_raw_VectorOfBlob(), output.as_raw_VectorOfBlob()) }.into_result()
	}
	
	fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
		string_arg!(input_name);
		unsafe { sys::cv_dnn_LSTMLayer_inputNameToIndex_String(self.as_raw_LSTMLayer(), input_name.as_ptr() as _) }.into_result()
	}
	
	fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
		string_arg!(output_name);
		unsafe { sys::cv_dnn_LSTMLayer_outputNameToIndex_String(self.as_raw_LSTMLayer(), output_name.as_ptr() as _) }.into_result()
	}
	
}

impl dyn LSTMLayer + '_ {
	/// Creates instance of LSTM layer
	pub fn create() -> Result<types::PtrOfLSTMLayer> {
		unsafe { sys::cv_dnn_LSTMLayer_create() }.into_result().map(|ptr| types::PtrOfLSTMLayer { ptr })
	}
	
}
/// This interface class allows to build new Layers - are building blocks of networks.
/// 
/// Each class, derived from Layer, must implement allocate() methods to declare own outputs and forward() to compute outputs.
/// Also before using the new layer into networks you must register your layer by using one of @ref dnnLayerFactory "LayerFactory" macros.
pub trait Layer {
	fn as_raw_Layer(&self) -> *mut c_void;
	/// List of learned parameters must be stored here to allow read them by using Net::getParam().
	fn blobs(&mut self) -> types::VectorOfBlob {
		unsafe { sys::cv_dnn_Layer_blobs(self.as_raw_Layer()) }.into_result().map(|ptr| types::VectorOfBlob { ptr }).expect("Infallible function failed: blobs")
	}
	
	/// List of learned parameters must be stored here to allow read them by using Net::getParam().
	fn set_blobs(&mut self, val: types::VectorOfBlob) -> () {
		unsafe { sys::cv_dnn_Layer_setBlobs_vector_Blob_(self.as_raw_Layer(), val.as_raw_VectorOfBlob()) }.into_result().expect("Infallible function failed: set_blobs")
	}
	
	/// Name of the layer instance, can be used for logging or other internal purposes.
	fn name(&self) -> String {
		unsafe { sys::cv_dnn_Layer_name_const(self.as_raw_Layer()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: name")
	}
	
	/// Name of the layer instance, can be used for logging or other internal purposes.
	fn set_name(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_Layer_setName_String(self.as_raw_Layer(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_name")
	}
	
	/// Type name which was used for creating layer by layer factory.
	fn typ(&self) -> String {
		unsafe { sys::cv_dnn_Layer_type_const(self.as_raw_Layer()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: typ")
	}
	
	/// Type name which was used for creating layer by layer factory.
	fn set_type(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_Layer_setType_String(self.as_raw_Layer(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_type")
	}
	
	/// Allocates internal buffers and output blobs with respect to the shape of inputs.
	/// ## Parameters
	/// * input: vector of already allocated input blobs
	/// * output:[out] vector of output blobs, which must be allocated
	/// 
	/// This method must create each produced blob according to shape of @p input blobs and internal layer params.
	/// If this method is called first time then @p output vector consists from empty blobs and its size determined by number of output connections.
	/// This method can be called multiple times if size of any @p input blob was changed.
	fn allocate(&mut self, input: &types::VectorOfBlob, output: &mut types::VectorOfBlob) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_allocate_const_vector_BlobX_X_vector_Blob_X(self.as_raw_Layer(), input.as_raw_VectorOfBlob(), output.as_raw_VectorOfBlob()) }.into_result()
	}
	
	/// Given the @p input blobs, computes the output @p blobs.
	/// ## Parameters
	/// * input: the input blobs.
	/// * output:[out] allocated output blobs, which will store results of the computation.
	fn forward(&mut self, input: &mut types::VectorOfBlob, output: &mut types::VectorOfBlob) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_forward_vector_BlobX_X_vector_Blob_X(self.as_raw_Layer(), input.as_raw_VectorOfBlob(), output.as_raw_VectorOfBlob()) }.into_result()
	}
	
	/// Allocates internal buffers and output blobs with respect to the shape of inputs.
	/// ## Parameters
	/// * input: vector of already allocated input blobs
	/// * output:[out] vector of output blobs, which must be allocated
	/// 
	/// This method must create each produced blob according to shape of @p input blobs and internal layer params.
	/// If this method is called first time then @p output vector consists from empty blobs and its size determined by number of output connections.
	/// This method can be called multiple times if size of any @p input blob was changed.
	/// 
	/// ## Overloaded parameters
	fn allocate_1(&mut self, inputs: &types::VectorOfBlob, outputs: &mut types::VectorOfBlob) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_allocate_const_vector_Blob_X_vector_Blob_X(self.as_raw_Layer(), inputs.as_raw_VectorOfBlob(), outputs.as_raw_VectorOfBlob()) }.into_result()
	}
	
	/// Allocates internal buffers and output blobs with respect to the shape of inputs.
	/// ## Parameters
	/// * input: vector of already allocated input blobs
	/// * output:[out] vector of output blobs, which must be allocated
	/// 
	/// This method must create each produced blob according to shape of @p input blobs and internal layer params.
	/// If this method is called first time then @p output vector consists from empty blobs and its size determined by number of output connections.
	/// This method can be called multiple times if size of any @p input blob was changed.
	/// 
	/// ## Overloaded parameters
	fn allocate_2(&mut self, inputs: &types::VectorOfBlob) -> Result<types::VectorOfBlob> {
		unsafe { sys::cv_dnn_Layer_allocate_const_vector_Blob_X(self.as_raw_Layer(), inputs.as_raw_VectorOfBlob()) }.into_result().map(|ptr| types::VectorOfBlob { ptr })
	}
	
	/// Given the @p input blobs, computes the output @p blobs.
	/// ## Parameters
	/// * input: the input blobs.
	/// * output:[out] allocated output blobs, which will store results of the computation.
	/// 
	/// ## Overloaded parameters
	fn forward_1(&mut self, inputs: &types::VectorOfBlob, outputs: &mut types::VectorOfBlob) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_forward_const_vector_Blob_X_vector_Blob_X(self.as_raw_Layer(), inputs.as_raw_VectorOfBlob(), outputs.as_raw_VectorOfBlob()) }.into_result()
	}
	
	/// Allocates layer and computes output.
	fn run(&mut self, inputs: &types::VectorOfBlob, outputs: &mut types::VectorOfBlob) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_run_const_vector_Blob_X_vector_Blob_X(self.as_raw_Layer(), inputs.as_raw_VectorOfBlob(), outputs.as_raw_VectorOfBlob()) }.into_result()
	}
	
	/// Returns index of input blob into the input array.
	/// ## Parameters
	/// * inputName: label of input blob
	/// 
	/// Each layer input and output can be labeled to easily identify them using "%<layer_name%>[.output_name]" notation.
	/// This method maps label of input blob to its index into input vector.
	fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
		string_arg!(input_name);
		unsafe { sys::cv_dnn_Layer_inputNameToIndex_String(self.as_raw_Layer(), input_name.as_ptr() as _) }.into_result()
	}
	
	/// Returns index of output blob in output array.
	/// ## See also
	/// inputNameToIndex()
	fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
		string_arg!(output_name);
		unsafe { sys::cv_dnn_Layer_outputNameToIndex_String(self.as_raw_Layer(), output_name.as_ptr() as _) }.into_result()
	}
	
	fn set_params_from(&mut self, params: &crate::dnn::LayerParams) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_setParamsFrom_const_LayerParamsX(self.as_raw_Layer(), params.as_raw_LayerParams()) }.into_result()
	}
	
}

/// %Layer factory allows to create instances of registered layers.
pub trait LayerFactoryTrait {
	fn as_raw_LayerFactory(&self) -> *mut c_void;
}

/// %Layer factory allows to create instances of registered layers.
pub struct LayerFactory {
	pub(crate) ptr: *mut c_void
}

impl Drop for LayerFactory {
	fn drop(&mut self) {
		extern "C" { fn cv_LayerFactory_delete(instance: *mut c_void); }
		unsafe { cv_LayerFactory_delete(self.as_raw_LayerFactory()) };
	}
}

impl LayerFactory {
	pub fn as_raw_LayerFactory(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for LayerFactory {}

impl crate::dnn::LayerFactoryTrait for LayerFactory {
	fn as_raw_LayerFactory(&self) -> *mut c_void { self.ptr }
}

impl LayerFactory {
	/// Registers the layer class with typename @p type and specified @p constructor.
	pub fn register_layer(typ: &str, constructor: crate::dnn::LayerFactory_Constuctor) -> Result<()> {
		string_arg!(typ);
		unsafe { sys::cv_dnn_LayerFactory_registerLayer_const_StringX_Constuctor(typ.as_ptr(), constructor) }.into_result()
	}
	
	/// Unregisters registered layer with specified type name.
	pub fn unregister_layer(typ: &str) -> Result<()> {
		string_arg!(typ);
		unsafe { sys::cv_dnn_LayerFactory_unregisterLayer_const_StringX(typ.as_ptr()) }.into_result()
	}
	
	/// Creates instance of registered layer.
	/// ## Parameters
	/// * type: type name of creating layer.
	/// * params: parameters which will be used for layer initialization.
	pub fn create_layer_instance(typ: &str, params: &mut crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
		string_arg!(typ);
		unsafe { sys::cv_dnn_LayerFactory_createLayerInstance_const_StringX_LayerParamsX(typ.as_ptr(), params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
}

/// This class provides all data needed to initialize layer.
/// 
/// It includes dictionary with scalar params (which can be readed by using Dict interface),
/// blob params #blobs and optional meta information: #name and #type of layer instance.
pub trait LayerParamsTrait: crate::dnn::DictTrait {
	fn as_raw_LayerParams(&self) -> *mut c_void;
	/// List of learned parameters stored as blobs.
	fn blobs(&mut self) -> types::VectorOfBlob {
		unsafe { sys::cv_dnn_LayerParams_blobs(self.as_raw_LayerParams()) }.into_result().map(|ptr| types::VectorOfBlob { ptr }).expect("Infallible function failed: blobs")
	}
	
	/// List of learned parameters stored as blobs.
	fn set_blobs(&mut self, val: types::VectorOfBlob) -> () {
		unsafe { sys::cv_dnn_LayerParams_setBlobs_vector_Blob_(self.as_raw_LayerParams(), val.as_raw_VectorOfBlob()) }.into_result().expect("Infallible function failed: set_blobs")
	}
	
	/// Name of the layer instance (optional, can be used internal purposes).
	fn name(&self) -> String {
		unsafe { sys::cv_dnn_LayerParams_name_const(self.as_raw_LayerParams()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: name")
	}
	
	/// Name of the layer instance (optional, can be used internal purposes).
	fn set_name(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_LayerParams_setName_String(self.as_raw_LayerParams(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_name")
	}
	
	/// Type name which was used for creating layer by layer factory (optional).
	fn typ(&self) -> String {
		unsafe { sys::cv_dnn_LayerParams_type_const(self.as_raw_LayerParams()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: typ")
	}
	
	/// Type name which was used for creating layer by layer factory (optional).
	fn set_type(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_LayerParams_setType_String(self.as_raw_LayerParams(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_type")
	}
	
}

/// This class provides all data needed to initialize layer.
/// 
/// It includes dictionary with scalar params (which can be readed by using Dict interface),
/// blob params #blobs and optional meta information: #name and #type of layer instance.
pub struct LayerParams {
	pub(crate) ptr: *mut c_void
}

impl Drop for LayerParams {
	fn drop(&mut self) {
		extern "C" { fn cv_LayerParams_delete(instance: *mut c_void); }
		unsafe { cv_LayerParams_delete(self.as_raw_LayerParams()) };
	}
}

impl LayerParams {
	pub fn as_raw_LayerParams(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for LayerParams {}

impl crate::dnn::DictTrait for LayerParams {
	fn as_raw_Dict(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerParamsTrait for LayerParams {
	fn as_raw_LayerParams(&self) -> *mut c_void { self.ptr }
}

impl LayerParams {
}

pub trait MVNLayer: crate::dnn::Layer {
	fn as_raw_MVNLayer(&self) -> *mut c_void;
	fn eps(&self) -> f64 {
		unsafe { sys::cv_dnn_MVNLayer_eps_const(self.as_raw_MVNLayer()) }.into_result().expect("Infallible function failed: eps")
	}
	
	fn set_eps(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_MVNLayer_setEps_double(self.as_raw_MVNLayer(), val) }.into_result().expect("Infallible function failed: set_eps")
	}
	
	fn norm_variance(&self) -> bool {
		unsafe { sys::cv_dnn_MVNLayer_normVariance_const(self.as_raw_MVNLayer()) }.into_result().expect("Infallible function failed: norm_variance")
	}
	
	fn set_norm_variance(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_MVNLayer_setNormVariance_bool(self.as_raw_MVNLayer(), val) }.into_result().expect("Infallible function failed: set_norm_variance")
	}
	
	fn across_channels(&self) -> bool {
		unsafe { sys::cv_dnn_MVNLayer_acrossChannels_const(self.as_raw_MVNLayer()) }.into_result().expect("Infallible function failed: across_channels")
	}
	
	fn set_across_channels(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_MVNLayer_setAcrossChannels_bool(self.as_raw_MVNLayer(), val) }.into_result().expect("Infallible function failed: set_across_channels")
	}
	
}

impl dyn MVNLayer + '_ {
	/// ## C++ default parameters
	/// * norm_variance: true
	/// * across_channels: false
	/// * eps: 1e-9
	pub fn create(norm_variance: bool, across_channels: bool, eps: f64) -> Result<types::PtrOfMVNLayer> {
		unsafe { sys::cv_dnn_MVNLayer_create_bool_bool_double(norm_variance, across_channels, eps) }.into_result().map(|ptr| types::PtrOfMVNLayer { ptr })
	}
	
}
/// This class allows to create and manipulate comprehensive artificial neural networks.
/// 
/// Neural network is presented as directed acyclic graph (DAG), where vertices are Layer instances,
/// and edges specify relationships between layers inputs and outputs.
/// 
/// Each network layer has unique integer id and unique string name inside its network.
/// LayerId can store either layer name or layer id.
/// 
/// This class supports reference counting of its instances, i. e. copies point to the same instance.
pub trait NetTrait {
	fn as_raw_Net(&self) -> *mut c_void;
	/// Returns true if there are no layers in the network.
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_dnn_Net_empty_const(self.as_raw_Net()) }.into_result()
	}
	
	/// Adds new layer to the net.
	/// ## Parameters
	/// * name: unique name of the adding layer.
	/// * type: typename of the adding layer (type must be registered in LayerRegister).
	/// * params: parameters which will be used to initialize the creating layer.
	/// ## Returns
	/// unique identifier of created layer, or -1 if a failure will happen.
	fn add_layer(&mut self, name: &str, typ: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
		string_arg!(name);
		string_arg!(typ);
		unsafe { sys::cv_dnn_Net_addLayer_const_StringX_const_StringX_LayerParamsX(self.as_raw_Net(), name.as_ptr(), typ.as_ptr(), params.as_raw_LayerParams()) }.into_result()
	}
	
	/// Adds new layer and connects its first input to the first output of previously added layer.
	/// ## See also
	/// addLayer()
	fn add_layer_to_prev(&mut self, name: &str, typ: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
		string_arg!(name);
		string_arg!(typ);
		unsafe { sys::cv_dnn_Net_addLayerToPrev_const_StringX_const_StringX_LayerParamsX(self.as_raw_Net(), name.as_ptr(), typ.as_ptr(), params.as_raw_LayerParams()) }.into_result()
	}
	
	/// Converts string name of the layer to the integer identifier.
	/// ## Returns
	/// id of the layer, or -1 if the layer wasn't found.
	fn get_layer_id(&mut self, layer: &str) -> Result<i32> {
		string_arg!(layer);
		unsafe { sys::cv_dnn_Net_getLayerId_const_StringX(self.as_raw_Net(), layer.as_ptr()) }.into_result()
	}
	
	fn get_layer_names(&self) -> Result<types::VectorOfString> {
		unsafe { sys::cv_dnn_Net_getLayerNames_const(self.as_raw_Net()) }.into_result().map(|ptr| types::VectorOfString { ptr })
	}
	
	/// Returns pointer to layer with specified name which the network use.
	fn get_layer(&mut self, layer_id: crate::dnn::Net_LayerId) -> Result<types::PtrOfLayer> {
		unsafe { sys::cv_dnn_Net_getLayer_LayerId(self.as_raw_Net(), layer_id.as_raw_DictValue()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
	/// Delete layer for the network (not implemented yet)
	fn delete_layer(&mut self, layer: crate::dnn::Net_LayerId) -> Result<()> {
		unsafe { sys::cv_dnn_Net_deleteLayer_LayerId(self.as_raw_Net(), layer.as_raw_DictValue()) }.into_result()
	}
	
	/// Connects output of the first layer to input of the second layer.
	/// ## Parameters
	/// * outPin: descriptor of the first layer output.
	/// * inpPin: descriptor of the second layer input.
	/// 
	/// Descriptors have the following template <DFN>&lt;layer_name&gt;[.input_number]</DFN>:
	/// - the first part of the template <DFN>layer_name</DFN> is sting name of the added layer.
	///   If this part is empty then the network input pseudo layer will be used;
	/// - the second optional part of the template <DFN>input_number</DFN>
	///   is either number of the layer input, either label one.
	///   If this part is omitted then the first layer input will be used.
	/// ## See also
	/// setNetInputs(), Layer::inputNameToIndex(), Layer::outputNameToIndex()
	fn connect_first_second(&mut self, out_pin: &str, inp_pin: &str) -> Result<()> {
		string_arg!(out_pin);
		string_arg!(inp_pin);
		unsafe { sys::cv_dnn_Net_connect_String_String(self.as_raw_Net(), out_pin.as_ptr() as _, inp_pin.as_ptr() as _) }.into_result()
	}
	
	/// Connects #@p outNum output of the first layer to #@p inNum input of the second layer.
	/// ## Parameters
	/// * outLayerId: identifier of the first layer
	/// * inpLayerId: identifier of the second layer
	/// * outNum: number of the first layer output
	/// * inpNum: number of the second layer input
	fn connect(&mut self, out_layer_id: i32, out_num: i32, inp_layer_id: i32, inp_num: i32) -> Result<()> {
		unsafe { sys::cv_dnn_Net_connect_int_int_int_int(self.as_raw_Net(), out_layer_id, out_num, inp_layer_id, inp_num) }.into_result()
	}
	
	/// Sets outputs names of the network input pseudo layer.
	/// 
	/// Each net always has special own the network input pseudo layer with id=0.
	/// This layer stores the user blobs only and don't make any computations.
	/// In fact, this layer provides the only way to pass user data into the network.
	/// As any other layer, this layer can label its outputs and this function provides an easy way to do this.
	fn set_net_inputs(&mut self, input_blob_names: &types::VectorOfString) -> Result<()> {
		unsafe { sys::cv_dnn_Net_setNetInputs_const_vector_String_X(self.as_raw_Net(), input_blob_names.as_raw_VectorOfString()) }.into_result()
	}
	
	/// Initializes and allocates all layers.
	fn allocate(&mut self) -> Result<()> {
		unsafe { sys::cv_dnn_Net_allocate(self.as_raw_Net()) }.into_result()
	}
	
	/// Runs forward pass to compute output of layer @p toLayer.
	/// @details By default runs forward pass for the whole network.
	/// 
	/// ## C++ default parameters
	/// * to_layer: String()
	fn forward(&mut self, to_layer: crate::dnn::Net_LayerId) -> Result<()> {
		unsafe { sys::cv_dnn_Net_forward_LayerId(self.as_raw_Net(), to_layer.as_raw_DictValue()) }.into_result()
	}
	
	/// Runs forward pass to compute output of layer @p toLayer, but computations start from @p startLayer
	fn forward_1(&mut self, start_layer: crate::dnn::Net_LayerId, to_layer: crate::dnn::Net_LayerId) -> Result<()> {
		unsafe { sys::cv_dnn_Net_forward_LayerId_LayerId(self.as_raw_Net(), start_layer.as_raw_DictValue(), to_layer.as_raw_DictValue()) }.into_result()
	}
	
	/// Runs forward pass to compute output of layer @p toLayer, but computations start from @p startLayer
	/// 
	/// ## Overloaded parameters
	fn forward_2(&mut self, start_layers: &types::VectorOfNet_LayerId, to_layers: &types::VectorOfNet_LayerId) -> Result<()> {
		unsafe { sys::cv_dnn_Net_forward_const_vector_LayerId_X_const_vector_LayerId_X(self.as_raw_Net(), start_layers.as_raw_VectorOfNet_LayerId(), to_layers.as_raw_VectorOfNet_LayerId()) }.into_result()
	}
	
	/// Optimized forward.
	/// @warning Not implemented yet.
	/// @details Makes forward only those layers which weren't changed after previous forward().
	fn forward_opt(&mut self, to_layer: crate::dnn::Net_LayerId) -> Result<()> {
		unsafe { sys::cv_dnn_Net_forwardOpt_LayerId(self.as_raw_Net(), to_layer.as_raw_DictValue()) }.into_result()
	}
	
	/// Optimized forward.
	/// @warning Not implemented yet.
	/// @details Makes forward only those layers which weren't changed after previous forward().
	/// 
	/// ## Overloaded parameters
	fn forward_opt_1(&mut self, to_layers: &types::VectorOfNet_LayerId) -> Result<()> {
		unsafe { sys::cv_dnn_Net_forwardOpt_const_vector_LayerId_X(self.as_raw_Net(), to_layers.as_raw_VectorOfNet_LayerId()) }.into_result()
	}
	
	/// Sets the new value for the layer output blob
	/// ## Parameters
	/// * outputName: descriptor of the updating layer output blob.
	/// * blob: new blob.
	/// ## See also
	/// connect(String, String) to know format of the descriptor.
	/// 
	/// Note: If updating blob is not empty then @p blob must have the same shape,
	/// because network reshaping is not implemented yet.
	fn set_blob(&mut self, output_name: &str, blob: &crate::dnn::Blob) -> Result<()> {
		string_arg!(output_name);
		unsafe { sys::cv_dnn_Net_setBlob_String_const_BlobX(self.as_raw_Net(), output_name.as_ptr() as _, blob.as_raw_Blob()) }.into_result()
	}
	
	/// Returns the layer output blob.
	/// ## Parameters
	/// * outputName: the descriptor of the returning layer output blob.
	/// ## See also
	/// connect(String, String)
	fn get_blob(&mut self, output_name: &str) -> Result<crate::dnn::Blob> {
		string_arg!(output_name);
		unsafe { sys::cv_dnn_Net_getBlob_String(self.as_raw_Net(), output_name.as_ptr() as _) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
	}
	
	/// Sets the new value for the learned param of the layer.
	/// ## Parameters
	/// * layer: name or id of the layer.
	/// * numParam: index of the layer parameter in the Layer::blobs array.
	/// * blob: the new value.
	/// ## See also
	/// Layer::blobs
	/// 
	/// Note: If shape of the new blob differs from the previous shape,
	/// then the following forward pass may fail.
	fn set_param(&mut self, layer: crate::dnn::Net_LayerId, num_param: i32, blob: &crate::dnn::Blob) -> Result<()> {
		unsafe { sys::cv_dnn_Net_setParam_LayerId_int_const_BlobX(self.as_raw_Net(), layer.as_raw_DictValue(), num_param, blob.as_raw_Blob()) }.into_result()
	}
	
	/// Returns parameter blob of the layer.
	/// ## Parameters
	/// * layer: name or id of the layer.
	/// * numParam: index of the layer parameter in the Layer::blobs array.
	/// ## See also
	/// Layer::blobs
	/// 
	/// ## C++ default parameters
	/// * num_param: 0
	fn get_param(&mut self, layer: crate::dnn::Net_LayerId, num_param: i32) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Net_getParam_LayerId_int(self.as_raw_Net(), layer.as_raw_DictValue(), num_param) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
	}
	
}

/// This class allows to create and manipulate comprehensive artificial neural networks.
/// 
/// Neural network is presented as directed acyclic graph (DAG), where vertices are Layer instances,
/// and edges specify relationships between layers inputs and outputs.
/// 
/// Each network layer has unique integer id and unique string name inside its network.
/// LayerId can store either layer name or layer id.
/// 
/// This class supports reference counting of its instances, i. e. copies point to the same instance.
pub struct Net {
	pub(crate) ptr: *mut c_void
}

impl Drop for Net {
	fn drop(&mut self) {
		extern "C" { fn cv_Net_delete(instance: *mut c_void); }
		unsafe { cv_Net_delete(self.as_raw_Net()) };
	}
}

impl Net {
	pub fn as_raw_Net(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Net {}

impl crate::dnn::NetTrait for Net {
	fn as_raw_Net(&self) -> *mut c_void { self.ptr }
}

impl Net {
	pub fn default() -> Result<crate::dnn::Net> {
		unsafe { sys::cv_dnn_Net_Net() }.into_result().map(|ptr| crate::dnn::Net { ptr })
	}
	
}

pub trait PoolingLayer: crate::dnn::Layer {
	fn as_raw_PoolingLayer(&self) -> *mut c_void;
	fn typ(&self) -> i32 {
		unsafe { sys::cv_dnn_PoolingLayer_type_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: typ")
	}
	
	fn set_type(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setType_int(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_type")
	}
	
	fn kernel(&self) -> core::Size {
		unsafe { sys::cv_dnn_PoolingLayer_kernel_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: kernel")
	}
	
	fn set_kernel(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setKernel_Size(self.as_raw_PoolingLayer(), &val) }.into_result().expect("Infallible function failed: set_kernel")
	}
	
	fn stride(&self) -> core::Size {
		unsafe { sys::cv_dnn_PoolingLayer_stride_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: stride")
	}
	
	fn set_stride(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setStride_Size(self.as_raw_PoolingLayer(), &val) }.into_result().expect("Infallible function failed: set_stride")
	}
	
	fn pad(&self) -> core::Size {
		unsafe { sys::cv_dnn_PoolingLayer_pad_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: pad")
	}
	
	fn set_pad(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPad_Size(self.as_raw_PoolingLayer(), &val) }.into_result().expect("Infallible function failed: set_pad")
	}
	
	fn global_pooling(&self) -> bool {
		unsafe { sys::cv_dnn_PoolingLayer_globalPooling_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: global_pooling")
	}
	
	fn set_global_pooling(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setGlobalPooling_bool(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_global_pooling")
	}
	
	fn pad_mode(&self) -> String {
		unsafe { sys::cv_dnn_PoolingLayer_padMode_const(self.as_raw_PoolingLayer()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: pad_mode")
	}
	
	fn set_pad_mode(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_PoolingLayer_setPadMode_String(self.as_raw_PoolingLayer(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_pad_mode")
	}
	
}

impl dyn PoolingLayer + '_ {
	/// ## C++ default parameters
	/// * typ: PoolingLayer::MAX
	/// * kernel: Size(2,2)
	/// * stride: Size(1,1)
	/// * pad: Size(0,0)
	/// * pad_mode: ""
	pub fn create(typ: i32, kernel: core::Size, stride: core::Size, pad: core::Size, pad_mode: &str) -> Result<types::PtrOfPoolingLayer> {
		string_arg!(pad_mode);
		unsafe { sys::cv_dnn_PoolingLayer_create_int_Size_Size_Size_const_StringX(typ, &kernel, &stride, &pad, pad_mode.as_ptr()) }.into_result().map(|ptr| types::PtrOfPoolingLayer { ptr })
	}
	
	/// ## C++ default parameters
	/// * typ: PoolingLayer::MAX
	pub fn create_global(typ: i32) -> Result<types::PtrOfPoolingLayer> {
		unsafe { sys::cv_dnn_PoolingLayer_createGlobal_int(typ) }.into_result().map(|ptr| types::PtrOfPoolingLayer { ptr })
	}
	
}
pub trait PowerLayer: crate::dnn::Layer {
	fn as_raw_PowerLayer(&self) -> *mut c_void;
	fn power(&self) -> f64 {
		unsafe { sys::cv_dnn_PowerLayer_power_const(self.as_raw_PowerLayer()) }.into_result().expect("Infallible function failed: power")
	}
	
	fn set_power(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_PowerLayer_setPower_double(self.as_raw_PowerLayer(), val) }.into_result().expect("Infallible function failed: set_power")
	}
	
	fn scale(&self) -> f64 {
		unsafe { sys::cv_dnn_PowerLayer_scale_const(self.as_raw_PowerLayer()) }.into_result().expect("Infallible function failed: scale")
	}
	
	fn set_scale(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_PowerLayer_setScale_double(self.as_raw_PowerLayer(), val) }.into_result().expect("Infallible function failed: set_scale")
	}
	
	fn shift(&self) -> f64 {
		unsafe { sys::cv_dnn_PowerLayer_shift_const(self.as_raw_PowerLayer()) }.into_result().expect("Infallible function failed: shift")
	}
	
	fn set_shift(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_PowerLayer_setShift_double(self.as_raw_PowerLayer(), val) }.into_result().expect("Infallible function failed: set_shift")
	}
	
}

impl dyn PowerLayer + '_ {
	/// ## C++ default parameters
	/// * power: 1
	/// * scale: 1
	/// * shift: 0
	pub fn create(power: f64, scale: f64, shift: f64) -> Result<types::PtrOfPowerLayer> {
		unsafe { sys::cv_dnn_PowerLayer_create_double_double_double(power, scale, shift) }.into_result().map(|ptr| types::PtrOfPowerLayer { ptr })
	}
	
}
/// Classical recurrent layer
pub trait RNNLayer: crate::dnn::Layer {
	fn as_raw_RNNLayer(&self) -> *mut c_void;
	/// Setups learned weights.
	/// 
	/// Recurrent-layer behavior on each step is defined by current input @f$ x_t @f$, previous state @f$ h_t @f$ and learned weights as follows:
	/// @f{eqnarray*}{
	/// h_t &= tanh&(W_{hh} h_{t-1} + W_{xh} x_t + b_h),  \\
	/// o_t &= tanh&(W_{ho} h_t + b_o),
	/// @f}
	/// 
	/// ## Parameters
	/// * Wxh: is @f$ W_{xh} @f$ matrix
	/// * bh: is @f$ b_{h}  @f$ vector
	/// * Whh: is @f$ W_{hh} @f$ matrix
	/// * Who: is @f$ W_{xo} @f$ matrix
	/// * bo: is @f$ b_{o}  @f$ vector
	fn set_weights(&mut self, wxh: &crate::dnn::Blob, bh: &crate::dnn::Blob, whh: &crate::dnn::Blob, who: &crate::dnn::Blob, bo: &crate::dnn::Blob) -> Result<()> {
		unsafe { sys::cv_dnn_RNNLayer_setWeights_const_BlobX_const_BlobX_const_BlobX_const_BlobX_const_BlobX(self.as_raw_RNNLayer(), wxh.as_raw_Blob(), bh.as_raw_Blob(), whh.as_raw_Blob(), who.as_raw_Blob(), bo.as_raw_Blob()) }.into_result()
	}
	
	/// If this flag is set to true then layer will produce @f$ h_t @f$ as second output.
	/// @details Shape of the second output is the same as first output.
	/// 
	/// ## C++ default parameters
	/// * produce: false
	fn set_produce_hidden_output(&mut self, produce: bool) -> Result<()> {
		unsafe { sys::cv_dnn_RNNLayer_setProduceHiddenOutput_bool(self.as_raw_RNNLayer(), produce) }.into_result()
	}
	
	/// Accepts two inputs @f$x_t@f$ and @f$h_{t-1}@f$ and compute two outputs @f$o_t@f$ and @f$h_t@f$.
	/// 
	/// ## Parameters
	/// * input: should contain packed input @f$x_t@f$.
	/// * output: should contain output @f$o_t@f$ (and @f$h_t@f$ if setProduceHiddenOutput() is set to true).
	/// 
	/// @p input[0] should have shape [`T`, `N`, `data_dims`] where `T` and `N` is number of timestamps and number of independent samples of @f$x_t@f$ respectively.
	/// 
	/// @p output[0] will have shape [`T`, `N`, @f$N_o@f$], where @f$N_o@f$ is number of rows in @f$ W_{xo} @f$ matrix.
	/// 
	/// If setProduceHiddenOutput() is set to true then @p output[1] will contain a Blob with shape [`T`, `N`, @f$N_h@f$], where @f$N_h@f$ is number of rows in @f$ W_{hh} @f$ matrix.
	fn forward(&mut self, input: &mut types::VectorOfBlob, output: &mut types::VectorOfBlob) -> Result<()> {
		unsafe { sys::cv_dnn_RNNLayer_forward_vector_BlobX_X_vector_Blob_X(self.as_raw_RNNLayer(), input.as_raw_VectorOfBlob(), output.as_raw_VectorOfBlob()) }.into_result()
	}
	
}

impl dyn RNNLayer + '_ {
	/// Creates instance of RNNLayer
	pub fn create() -> Result<types::PtrOfRNNLayer> {
		unsafe { sys::cv_dnn_RNNLayer_create() }.into_result().map(|ptr| types::PtrOfRNNLayer { ptr })
	}
	
}
pub trait ReLULayer: crate::dnn::Layer {
	fn as_raw_ReLULayer(&self) -> *mut c_void;
	fn negative_slope(&self) -> f64 {
		unsafe { sys::cv_dnn_ReLULayer_negativeSlope_const(self.as_raw_ReLULayer()) }.into_result().expect("Infallible function failed: negative_slope")
	}
	
	fn set_negative_slope(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_ReLULayer_setNegativeSlope_double(self.as_raw_ReLULayer(), val) }.into_result().expect("Infallible function failed: set_negative_slope")
	}
	
}

impl dyn ReLULayer + '_ {
	/// ## C++ default parameters
	/// * negative_slope: 0
	pub fn create(negative_slope: f64) -> Result<types::PtrOfReLULayer> {
		unsafe { sys::cv_dnn_ReLULayer_create_double(negative_slope) }.into_result().map(|ptr| types::PtrOfReLULayer { ptr })
	}
	
}
pub trait ReshapeLayer: crate::dnn::Layer {
	fn as_raw_ReshapeLayer(&self) -> *mut c_void;
	fn new_shape_desc(&mut self) -> crate::dnn::BlobShape {
		unsafe { sys::cv_dnn_ReshapeLayer_newShapeDesc(self.as_raw_ReshapeLayer()) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr }).expect("Infallible function failed: new_shape_desc")
	}
	
	fn set_new_shape_desc(&mut self, val: crate::dnn::BlobShape) -> () {
		unsafe { sys::cv_dnn_ReshapeLayer_setNewShapeDesc_BlobShape(self.as_raw_ReshapeLayer(), val.as_raw_BlobShape()) }.into_result().expect("Infallible function failed: set_new_shape_desc")
	}
	
	fn new_shape_range(&mut self) -> core::Range {
		unsafe { sys::cv_dnn_ReshapeLayer_newShapeRange(self.as_raw_ReshapeLayer()) }.into_result().map(|ptr| core::Range { ptr }).expect("Infallible function failed: new_shape_range")
	}
	
	fn set_new_shape_range(&mut self, val: core::Range) -> () {
		unsafe { sys::cv_dnn_ReshapeLayer_setNewShapeRange_Range(self.as_raw_ReshapeLayer(), val.as_raw_Range()) }.into_result().expect("Infallible function failed: set_new_shape_range")
	}
	
}

impl dyn ReshapeLayer + '_ {
	/// ## C++ default parameters
	/// * applying_range: Range::all()
	/// * enable_reordering: false
	pub fn create(new_shape: &crate::dnn::BlobShape, applying_range: core::Range, enable_reordering: bool) -> Result<types::PtrOfReshapeLayer> {
		unsafe { sys::cv_dnn_ReshapeLayer_create_const_BlobShapeX_Range_bool(new_shape.as_raw_BlobShape(), applying_range.as_raw_Range(), enable_reordering) }.into_result().map(|ptr| types::PtrOfReshapeLayer { ptr })
	}
	
}
pub trait SigmoidLayer: crate::dnn::Layer {
	fn as_raw_SigmoidLayer(&self) -> *mut c_void;
}

impl dyn SigmoidLayer + '_ {
	pub fn create() -> Result<types::PtrOfSigmoidLayer> {
		unsafe { sys::cv_dnn_SigmoidLayer_create() }.into_result().map(|ptr| types::PtrOfSigmoidLayer { ptr })
	}
	
}
pub trait SliceLayer: crate::dnn::Layer {
	fn as_raw_SliceLayer(&self) -> *mut c_void;
	fn axis(&self) -> i32 {
		unsafe { sys::cv_dnn_SliceLayer_axis_const(self.as_raw_SliceLayer()) }.into_result().expect("Infallible function failed: axis")
	}
	
	fn set_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_SliceLayer_setAxis_int(self.as_raw_SliceLayer(), val) }.into_result().expect("Infallible function failed: set_axis")
	}
	
	fn slice_indices(&mut self) -> types::VectorOfi32 {
		unsafe { sys::cv_dnn_SliceLayer_sliceIndices(self.as_raw_SliceLayer()) }.into_result().map(|ptr| types::VectorOfi32 { ptr }).expect("Infallible function failed: slice_indices")
	}
	
	fn set_slice_indices(&mut self, val: types::VectorOfi32) -> () {
		unsafe { sys::cv_dnn_SliceLayer_setSliceIndices_vector_int_(self.as_raw_SliceLayer(), val.as_raw_VectorOfi32()) }.into_result().expect("Infallible function failed: set_slice_indices")
	}
	
}

impl dyn SliceLayer + '_ {
	pub fn create(axis: i32) -> Result<types::PtrOfSliceLayer> {
		unsafe { sys::cv_dnn_SliceLayer_create_int(axis) }.into_result().map(|ptr| types::PtrOfSliceLayer { ptr })
	}
	
	pub fn create_1(axis: i32, slice_indices: &types::VectorOfi32) -> Result<types::PtrOfSliceLayer> {
		unsafe { sys::cv_dnn_SliceLayer_create_int_const_vector_int_X(axis, slice_indices.as_raw_VectorOfi32()) }.into_result().map(|ptr| types::PtrOfSliceLayer { ptr })
	}
	
}
pub trait SoftmaxLayer: crate::dnn::Layer {
	fn as_raw_SoftmaxLayer(&self) -> *mut c_void;
}

impl dyn SoftmaxLayer + '_ {
	/// ## C++ default parameters
	/// * axis: 1
	pub fn create(axis: i32) -> Result<types::PtrOfSoftmaxLayer> {
		unsafe { sys::cv_dnn_SoftmaxLayer_create_int(axis) }.into_result().map(|ptr| types::PtrOfSoftmaxLayer { ptr })
	}
	
}
pub trait SplitLayer: crate::dnn::Layer {
	fn as_raw_SplitLayer(&self) -> *mut c_void;
	/// Number of copies that will be produced (is ignored when negative).
	fn outputs_count(&self) -> i32 {
		unsafe { sys::cv_dnn_SplitLayer_outputsCount_const(self.as_raw_SplitLayer()) }.into_result().expect("Infallible function failed: outputs_count")
	}
	
	/// Number of copies that will be produced (is ignored when negative).
	fn set_outputs_count(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_SplitLayer_setOutputsCount_int(self.as_raw_SplitLayer(), val) }.into_result().expect("Infallible function failed: set_outputs_count")
	}
	
}

impl dyn SplitLayer + '_ {
	/// ## C++ default parameters
	/// * outputs_count: -1
	pub fn create(outputs_count: i32) -> Result<types::PtrOfSplitLayer> {
		unsafe { sys::cv_dnn_SplitLayer_create_int(outputs_count) }.into_result().map(|ptr| types::PtrOfSplitLayer { ptr })
	}
	
}
pub trait TanHLayer: crate::dnn::Layer {
	fn as_raw_TanHLayer(&self) -> *mut c_void;
}

impl dyn TanHLayer + '_ {
	pub fn create() -> Result<types::PtrOfTanHLayer> {
		unsafe { sys::cv_dnn_TanHLayer_create() }.into_result().map(|ptr| types::PtrOfTanHLayer { ptr })
	}
	
}
pub trait _RangeTrait: core::RangeTrait {
	fn as_raw__Range(&self) -> *mut c_void;
}

pub struct _Range {
	pub(crate) ptr: *mut c_void
}

impl Drop for _Range {
	fn drop(&mut self) {
		extern "C" { fn cv__Range_delete(instance: *mut c_void); }
		unsafe { cv__Range_delete(self.as_raw__Range()) };
	}
}

impl _Range {
	pub fn as_raw__Range(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for _Range {}

impl core::RangeTrait for _Range {
	fn as_raw_Range(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::_RangeTrait for _Range {
	fn as_raw__Range(&self) -> *mut c_void { self.ptr }
}

impl _Range {
	pub fn new(r: &core::Range) -> Result<crate::dnn::_Range> {
		unsafe { sys::cv_dnn__Range__Range_const_RangeX(r.as_raw_Range()) }.into_result().map(|ptr| crate::dnn::_Range { ptr })
	}
	
	/// ## C++ default parameters
	/// * size: 1
	pub fn new_1(start: i32, size: i32) -> Result<crate::dnn::_Range> {
		unsafe { sys::cv_dnn__Range__Range_int_int(start, size) }.into_result().map(|ptr| crate::dnn::_Range { ptr })
	}
	
}
pub use crate::manual::dnn::*;

#![allow(unused_parens)]
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
	ALLOC_MAT = 1,
	ALLOC_UMAT = 2,
	ALLOC_BOTH = 3,
}

opencv_type_enum! { crate::dnn::Blob_AllocFlag }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Blob_DataState {
	UNINITIALIZED = 0,
	HEAD_AT_MAT = 1,
	HEAD_AT_UMAT = 2,
	SYNCED = 3,
}

opencv_type_enum! { crate::dnn::Blob_DataState }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EltwiseLayer_EltwiseOp {
	PROD = 0,
	SUM = 1,
	MAX = 2,
}

opencv_type_enum! { crate::dnn::EltwiseLayer_EltwiseOp }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LRNLayer_Type {
	CHANNEL_NRM = 0,
	SPATIAL_NRM = 1,
}

opencv_type_enum! { crate::dnn::LRNLayer_Type }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PoolingLayer_Type {
	MAX = 0,
	AVE = 1,
	STOCHASTIC = 2,
}

opencv_type_enum! { crate::dnn::PoolingLayer_Type }

/// Each Layer class must provide this function to the factory
pub type LayerFactory_Constuctor = Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>;
/// Container for strings and integers.
pub type Net_LayerId = crate::dnn::DictValue;
pub type Shape = crate::dnn::BlobShape;
/// Creates the importer of <a href="http://caffe.berkeleyvision.org">Caffe</a> framework network.
/// ## Parameters
/// * prototxt: path to the .prototxt file with text description of the network architecture.
/// * caffeModel: path to the .caffemodel file with learned network.
/// ## Returns
/// Pointer to the created importer, NULL in failure cases.
/// 
/// ## C++ default parameters
/// * caffe_model: String()
pub fn create_caffe_importer(prototxt: &str, caffe_model: &str) -> Result<core::Ptr::<dyn crate::dnn::Importer>> {
	extern_container_arg!(prototxt);
	extern_container_arg!(caffe_model);
	unsafe { sys::cv_dnn_createCaffeImporter_const_StringR_const_StringR(prototxt.opencv_as_extern(), caffe_model.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::Importer>::opencv_from_extern(r) } )
}

/// Creates the importer of <a href="http://www.tensorflow.org">TensorFlow</a> framework network.
/// ## Parameters
/// * model: path to the .pb file with binary protobuf description of the network architecture.
/// ## Returns
/// Pointer to the created importer, NULL in failure cases.
pub fn create_tensorflow_importer(model: &str) -> Result<core::Ptr::<dyn crate::dnn::Importer>> {
	extern_container_arg!(model);
	unsafe { sys::cv_dnn_createTensorflowImporter_const_StringR(model.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::Importer>::opencv_from_extern(r) } )
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
pub fn create_torch_importer(filename: &str, is_binary: bool) -> Result<core::Ptr::<dyn crate::dnn::Importer>> {
	extern_container_arg!(filename);
	unsafe { sys::cv_dnn_createTorchImporter_const_StringR_bool(filename.opencv_as_extern(), is_binary) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::Importer>::opencv_from_extern(r) } )
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
	extern_container_arg!(prototxt);
	extern_container_arg!(caffe_model);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_StringR_const_StringR(prototxt.opencv_as_extern(), caffe_model.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::dnn::Net::opencv_from_extern(r) } )
}

/// Loads blob which was serialized as torch.Tensor object of Torch7 framework.
/// @warning This function has the same limitations as createTorchImporter().
/// 
/// ## C++ default parameters
/// * is_binary: true
pub fn read_torch_blob(filename: &str, is_binary: bool) -> Result<crate::dnn::Blob> {
	extern_container_arg!(filename);
	unsafe { sys::cv_dnn_readTorchBlob_const_StringR_bool(filename.opencv_as_extern(), is_binary) }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
}

pub trait AbsLayer: crate::dnn::Layer {
	fn as_raw_AbsLayer(&self) -> *const c_void;
	fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void;

}

impl dyn AbsLayer + '_ {
	pub fn create() -> Result<core::Ptr::<dyn crate::dnn::AbsLayer>> {
		unsafe { sys::cv_dnn_AbsLayer_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::AbsLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait BNLLLayer: crate::dnn::Layer {
	fn as_raw_BNLLLayer(&self) -> *const c_void;
	fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void;

}

impl dyn BNLLLayer + '_ {
	pub fn create() -> Result<core::Ptr::<dyn crate::dnn::BNLLLayer>> {
		unsafe { sys::cv_dnn_BNLLLayer_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::BNLLLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait BaseConvolutionLayer: crate::dnn::Layer {
	fn as_raw_BaseConvolutionLayer(&self) -> *const c_void;
	fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void;

	fn kernel(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropKernel_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: kernel")
	}
	
	fn set_kernel(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropKernel_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_kernel")
	}
	
	fn stride(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropStride_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: stride")
	}
	
	fn set_stride(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropStride_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_stride")
	}
	
	fn pad(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropPad_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: pad")
	}
	
	fn set_pad(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropPad_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_pad")
	}
	
	fn dilation(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropDilation_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: dilation")
	}
	
	fn set_dilation(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropDilation_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_dilation")
	}
	
	fn pad_mode(&self) -> String {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropPadMode_const(self.as_raw_BaseConvolutionLayer()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: pad_mode")
	}
	
	fn set_pad_mode(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropPadMode_String(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_pad_mode")
	}
	
}

/// This class provides methods for continuous n-dimensional CPU and GPU array processing.
/// 
/// The class is realized as a wrapper over @ref cv::Mat and @ref cv::UMat.
/// It will support methods for switching and logical synchronization between CPU and GPU.
pub trait BlobTrait {
	fn as_raw_Blob(&self) -> *const c_void;
	fn as_raw_mut_Blob(&mut self) -> *mut c_void;

	/// Works like Blob::fromImages() but in-place.
	/// 
	/// ## C++ default parameters
	/// * dst_cn: -1
	fn batch_from_images(&mut self, image: &dyn core::ToInputArray, dst_cn: i32) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_dnn_Blob_batchFromImages_const__InputArrayR_int(self.as_raw_mut_Blob(), image.as_raw__InputArray(), dst_cn) }.into_result()
	}
	
	/// Creates blob with specified @p shape and @p type.
	/// 
	/// ## C++ default parameters
	/// * typ: CV_32F
	/// * alloc_flags: ALLOC_MAT
	fn create(&mut self, shape: &crate::dnn::BlobShape, typ: i32, alloc_flags: i32) -> Result<()> {
		unsafe { sys::cv_dnn_Blob_create_const_BlobShapeR_int_int(self.as_raw_mut_Blob(), shape.as_raw_BlobShape(), typ, alloc_flags) }.into_result()
	}
	
	/// Creates blob from Mat or UMat without copying the data.
	/// @details If in is Mat then Mat data is populated, otherwise - UMat.
	fn fill(&mut self, in_: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(in_);
		unsafe { sys::cv_dnn_Blob_fill_const__InputArrayR(self.as_raw_mut_Blob(), in_.as_raw__InputArray()) }.into_result()
	}
	
	/// Creates blob from user data.
	/// @details If @p deepCopy is false then CPU data will not be allocated.
	/// 
	/// ## C++ default parameters
	/// * deep_copy: true
	fn fill_1(&mut self, shape: &crate::dnn::BlobShape, typ: i32, data: *mut c_void, deep_copy: bool) -> Result<()> {
		unsafe { sys::cv_dnn_Blob_fill_const_BlobShapeR_int_voidX_bool(self.as_raw_mut_Blob(), shape.as_raw_BlobShape(), typ, data, deep_copy) }.into_result()
	}
	
	/// Sets @p value to the last used data (if @p allocFlags = -1).
	/// @details If @p allocFlags != -1 then destination data (Mat or UMat) is determined by flags from AllocFlag enum like in create().
	/// 
	/// ## C++ default parameters
	/// * alloc_flags: -1
	fn set_to(&mut self, value: &dyn core::ToInputArray, alloc_flags: i32) -> Result<()> {
		input_array_arg!(value);
		unsafe { sys::cv_dnn_Blob_setTo_const__InputArrayR_int(self.as_raw_mut_Blob(), value.as_raw__InputArray(), alloc_flags) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * write_only: true
	fn mat_ref(&mut self, write_only: bool) -> Result<core::Mat> {
		unsafe { sys::cv_dnn_Blob_matRef_bool(self.as_raw_mut_Blob(), write_only) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn mat_ref_const(&self) -> Result<core::Mat> {
		unsafe { sys::cv_dnn_Blob_matRefConst_const(self.as_raw_Blob()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * write_only: true
	fn umat_ref(&mut self, write_only: bool) -> Result<core::UMat> {
		unsafe { sys::cv_dnn_Blob_umatRef_bool(self.as_raw_mut_Blob(), write_only) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	fn umat_ref_const(&self) -> Result<core::UMat> {
		unsafe { sys::cv_dnn_Blob_umatRefConst_const(self.as_raw_Blob()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
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
		unsafe { sys::cv_dnn_Blob_shape_const(self.as_raw_Blob()) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	/// Checks equality of two blobs shapes.
	fn equal_shape(&self, other: &crate::dnn::Blob) -> Result<bool> {
		unsafe { sys::cv_dnn_Blob_equalShape_const_const_BlobR(self.as_raw_Blob(), other.as_raw_Blob()) }.into_result()
	}
	
	/// Returns slice of first two dimensions.
	/// @details The behaviour is similar to the following numpy code: blob[n, cn, ...]
	fn get_plane(&mut self, n: i32, cn: i32) -> Result<core::Mat> {
		unsafe { sys::cv_dnn_Blob_getPlane_int_int(self.as_raw_mut_Blob(), n, cn) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Returns slice of first dimension.
	///  @details The behaviour is similar to getPlane(), but returns all
	/// channels * rows * cols values, corresponding to the n-th value
	/// of the first dimension.
	fn get_planes(&mut self, n: i32) -> Result<core::Mat> {
		unsafe { sys::cv_dnn_Blob_getPlanes_int(self.as_raw_mut_Blob(), n) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
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
		unsafe { sys::cv_dnn_Blob_ptr_int_int_int_int(self.as_raw_mut_Blob(), n, cn, row, col) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// ptr<float>()
	/// 
	/// ## C++ default parameters
	/// * n: 0
	/// * cn: 0
	/// * row: 0
	/// * col: 0
	fn ptrf(&mut self, n: i32, cn: i32, row: i32, col: i32) -> Result<&mut f32> {
		unsafe { sys::cv_dnn_Blob_ptrf_int_int_int_int(self.as_raw_mut_Blob(), n, cn, row, col) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Shares data from other @p blob.
	/// ## Returns
	/// *this
	fn share_from(&mut self, blob: &crate::dnn::Blob) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Blob_shareFrom_const_BlobR(self.as_raw_mut_Blob(), blob.as_raw_Blob()) }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
	}
	
	/// Changes shape of the blob without copying the data.
	/// ## Returns
	/// *this
	fn reshape(&mut self, shape: &crate::dnn::BlobShape) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Blob_reshape_const_BlobShapeR(self.as_raw_mut_Blob(), shape.as_raw_BlobShape()) }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
	}
	
	/// Changes shape of the blob without copying the data.
	/// ## Returns
	/// shallow copy of original blob with new shape.
	fn reshaped(&self, new_shape: &crate::dnn::BlobShape) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Blob_reshaped_const_const_BlobShapeR(self.as_raw_Blob(), new_shape.as_raw_BlobShape()) }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
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
	ptr: *mut c_void
}

opencv_type_boxed! { Blob }

impl Drop for Blob {
	fn drop(&mut self) {
		extern "C" { fn cv_Blob_delete(instance: *mut c_void); }
		unsafe { cv_Blob_delete(self.as_raw_mut_Blob()) };
	}
}

impl Blob {
	#[inline] pub fn as_raw_Blob(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Blob(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Blob {}

impl crate::dnn::BlobTrait for Blob {
	#[inline] fn as_raw_Blob(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Blob(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Blob {
	pub fn default() -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Blob_Blob() }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
	}
	
	/// Constructs blob with specified @p shape and @p type.
	/// 
	/// ## C++ default parameters
	/// * typ: CV_32F
	/// * alloc_flags: ALLOC_MAT
	pub fn new(shape: &crate::dnn::BlobShape, typ: i32, alloc_flags: i32) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Blob_Blob_const_BlobShapeR_int_int(shape.as_raw_BlobShape(), typ, alloc_flags) }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
	}
	
	/// Constructs Blob from existing Mat or UMat.
	pub fn new_1(data: &dyn core::ToInputArray) -> Result<crate::dnn::Blob> {
		input_array_arg!(data);
		unsafe { sys::cv_dnn_Blob_Blob_const__InputArrayR(data.as_raw__InputArray()) }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
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
		unsafe { sys::cv_dnn_Blob_fromImages_const__InputArrayR_int(image.as_raw__InputArray(), dst_cn) }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
	}
	
}

/// Lightweight class for storing and processing a shape of blob (or anything else).
pub trait BlobShapeTrait {
	fn as_raw_BlobShape(&self) -> *const c_void;
	fn as_raw_mut_BlobShape(&mut self) -> *mut c_void;

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
		unsafe { sys::cv_dnn_BlobShape_size_int(self.as_raw_mut_BlobShape(), axis) }.into_result()
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
		unsafe { sys::cv_dnn_BlobShape_operator___int(self.as_raw_mut_BlobShape(), axis) }.into_result()
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
		unsafe { sys::cv_dnn_BlobShape_slice_const_int_int(self.as_raw_BlobShape(), start_axis, end_axis) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	/// Returns pointer to the first element of continuous size array.
	fn ptr(&self) -> Result<&i32> {
		unsafe { sys::cv_dnn_BlobShape_ptr_const(self.as_raw_BlobShape()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Returns pointer to the first element of continuous size array.
	/// 
	/// ## Overloaded parameters
	fn ptr_1(&mut self) -> Result<&mut i32> {
		unsafe { sys::cv_dnn_BlobShape_ptr(self.as_raw_mut_BlobShape()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	fn equal(&self, other: &crate::dnn::BlobShape) -> Result<bool> {
		unsafe { sys::cv_dnn_BlobShape_equal_const_const_BlobShapeR(self.as_raw_BlobShape(), other.as_raw_BlobShape()) }.into_result()
	}
	
	fn add(&self, r: &crate::dnn::BlobShape) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_operatorA_const_const_BlobShapeR(self.as_raw_BlobShape(), r.as_raw_BlobShape()) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	fn is_empty(&self) -> Result<bool> {
		unsafe { sys::cv_dnn_BlobShape_isEmpty_const(self.as_raw_BlobShape()) }.into_result()
	}
	
}

/// Lightweight class for storing and processing a shape of blob (or anything else).
pub struct BlobShape {
	ptr: *mut c_void
}

opencv_type_boxed! { BlobShape }

impl Drop for BlobShape {
	fn drop(&mut self) {
		extern "C" { fn cv_BlobShape_delete(instance: *mut c_void); }
		unsafe { cv_BlobShape_delete(self.as_raw_mut_BlobShape()) };
	}
}

impl BlobShape {
	#[inline] pub fn as_raw_BlobShape(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_BlobShape(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for BlobShape {}

impl crate::dnn::BlobShapeTrait for BlobShape {
	#[inline] fn as_raw_BlobShape(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_BlobShape(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BlobShape {
	pub fn default() -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape() }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	pub fn new(s0: i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_int(s0) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	pub fn new_1(s0: i32, s1: i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_int_int(s0, s1) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	pub fn new_2(s0: i32, s1: i32, s2: i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_int_int_int(s0, s1, s2) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	pub fn new_3(num: i32, cn: i32, rows: i32, cols: i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_int_int_int_int(num, cn, rows, cols) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	/// Creates n-dim shape from the @p sizes array; if @p sizes is NULL then shape will contain unspecified data
	pub fn new_4(ndims: i32, sizes: &i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_int_const_intX(ndims, sizes) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	pub fn new_5(sizes: &core::Vector::<i32>) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_BlobShape_const_vector_int_R(sizes.as_raw_VectorOfi32()) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	/// Creates n-dim shape and fill its by @p fill
	/// 
	/// ## C++ default parameters
	/// * fill: 1
	pub fn all(ndims: i32, fill: i32) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_all_int_int(ndims, fill) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	pub fn like(m: &core::Mat) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_like_const_MatR(m.as_raw_Mat()) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	pub fn like_1(m: &core::UMat) -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_like_const_UMatR(m.as_raw_UMat()) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
	pub fn empty() -> Result<crate::dnn::BlobShape> {
		unsafe { sys::cv_dnn_BlobShape_empty() }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } )
	}
	
}

pub trait ConcatLayer: crate::dnn::Layer {
	fn as_raw_ConcatLayer(&self) -> *const c_void;
	fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void;

	fn axis(&self) -> i32 {
		unsafe { sys::cv_dnn_ConcatLayer_getPropAxis_const(self.as_raw_ConcatLayer()) }.into_result().expect("Infallible function failed: axis")
	}
	
	fn set_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_ConcatLayer_setPropAxis_int(self.as_raw_mut_ConcatLayer(), val) }.into_result().expect("Infallible function failed: set_axis")
	}
	
}

impl dyn ConcatLayer + '_ {
	/// ## C++ default parameters
	/// * axis: 1
	pub fn create(axis: i32) -> Result<core::Ptr::<dyn crate::dnn::ConcatLayer>> {
		unsafe { sys::cv_dnn_ConcatLayer_create_int(axis) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::ConcatLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait ConvolutionLayer: crate::dnn::BaseConvolutionLayer {
	fn as_raw_ConvolutionLayer(&self) -> *const c_void;
	fn as_raw_mut_ConvolutionLayer(&mut self) -> *mut c_void;

}

impl dyn ConvolutionLayer + '_ {
	/// ## C++ default parameters
	/// * kernel: Size(3,3)
	/// * stride: Size(1,1)
	/// * pad: Size(0,0)
	/// * dilation: Size(1,1)
	pub fn create(kernel: core::Size, stride: core::Size, pad: core::Size, dilation: core::Size) -> Result<core::Ptr::<dyn crate::dnn::BaseConvolutionLayer>> {
		unsafe { sys::cv_dnn_ConvolutionLayer_create_Size_Size_Size_Size(kernel.opencv_as_extern(), stride.opencv_as_extern(), pad.opencv_as_extern(), dilation.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::BaseConvolutionLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait CropLayer: crate::dnn::Layer {
	fn as_raw_CropLayer(&self) -> *const c_void;
	fn as_raw_mut_CropLayer(&mut self) -> *mut c_void;

	fn start_axis(&self) -> i32 {
		unsafe { sys::cv_dnn_CropLayer_getPropStartAxis_const(self.as_raw_CropLayer()) }.into_result().expect("Infallible function failed: start_axis")
	}
	
	fn set_start_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_CropLayer_setPropStartAxis_int(self.as_raw_mut_CropLayer(), val) }.into_result().expect("Infallible function failed: set_start_axis")
	}
	
	fn offset(&mut self) -> core::Vector::<i32> {
		unsafe { sys::cv_dnn_CropLayer_getPropOffset(self.as_raw_mut_CropLayer()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } ).expect("Infallible function failed: offset")
	}
	
	fn set_offset(&mut self, mut val: core::Vector::<i32>) -> () {
		unsafe { sys::cv_dnn_CropLayer_setPropOffset_vector_int_(self.as_raw_mut_CropLayer(), val.as_raw_mut_VectorOfi32()) }.into_result().expect("Infallible function failed: set_offset")
	}
	
}

impl dyn CropLayer + '_ {
	pub fn create(start_axis: i32, offset: &core::Vector::<i32>) -> Result<core::Ptr::<dyn crate::dnn::CropLayer>> {
		unsafe { sys::cv_dnn_CropLayer_create_int_const_vector_int_R(start_axis, offset.as_raw_VectorOfi32()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::CropLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait DeconvolutionLayer: crate::dnn::BaseConvolutionLayer {
	fn as_raw_DeconvolutionLayer(&self) -> *const c_void;
	fn as_raw_mut_DeconvolutionLayer(&mut self) -> *mut c_void;

}

impl dyn DeconvolutionLayer + '_ {
	/// ## C++ default parameters
	/// * kernel: Size(3,3)
	/// * stride: Size(1,1)
	/// * pad: Size(0,0)
	/// * dilation: Size(1,1)
	pub fn create(kernel: core::Size, stride: core::Size, pad: core::Size, dilation: core::Size) -> Result<core::Ptr::<dyn crate::dnn::BaseConvolutionLayer>> {
		unsafe { sys::cv_dnn_DeconvolutionLayer_create_Size_Size_Size_Size(kernel.opencv_as_extern(), stride.opencv_as_extern(), pad.opencv_as_extern(), dilation.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::BaseConvolutionLayer>::opencv_from_extern(r) } )
	}
	
}
/// This class implements name-value dictionary, values are instances of DictValue.
pub trait DictTrait {
	fn as_raw_Dict(&self) -> *const c_void;
	fn as_raw_mut_Dict(&mut self) -> *mut c_void;

	/// Checks a presence of the @p key in the dictionary.
	fn has(&self, key: &str) -> Result<bool> {
		extern_container_arg!(key);
		unsafe { sys::cv_dnn_Dict_has_const_const_StringR(self.as_raw_Dict(), key.opencv_as_extern()) }.into_result()
	}
	
	/// If the @p key in the dictionary then returns pointer to its value, else returns NULL.
	unsafe fn ptr_mut(&mut self, key: &str) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(key);
		{ sys::cv_dnn_Dict_ptr_const_StringR(self.as_raw_mut_Dict(), key.opencv_as_extern()) }.into_result().map(|r| { crate::dnn::DictValue::opencv_from_extern(r) } )
	}
	
	/// If the @p key in the dictionary then returns its value, else an error will be generated.
	fn get(&self, key: &str) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(key);
		unsafe { sys::cv_dnn_Dict_get_const_const_StringR(self.as_raw_Dict(), key.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::dnn::DictValue::opencv_from_extern(r) } )
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set_str(&mut self, key: &str, value: &str) -> Result<String> {
		extern_container_arg!(key);
		extern_container_arg!(value);
		unsafe { sys::cv_dnn_Dict_set_cv_String_const_StringR_const_StringR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value.opencv_as_extern()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set(&mut self, key: &str, value: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(key);
		unsafe { sys::cv_dnn_Dict_set_cv_dnn_DictValue_const_StringR_const_DictValueR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value.as_raw_DictValue()) }.into_result().map(|r| unsafe { crate::dnn::DictValue::opencv_from_extern(r) } )
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set_f64(&mut self, key: &str, value: &f64) -> Result<f64> {
		extern_container_arg!(key);
		unsafe { sys::cv_dnn_Dict_set_double_const_StringR_const_doubleR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value) }.into_result()
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set_i64(&mut self, key: &str, value: &i64) -> Result<i64> {
		extern_container_arg!(key);
		unsafe { sys::cv_dnn_Dict_set_int64_t_const_StringR_const_int64_tR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value) }.into_result()
	}
	
}

/// This class implements name-value dictionary, values are instances of DictValue.
pub struct Dict {
	ptr: *mut c_void
}

opencv_type_boxed! { Dict }

impl Drop for Dict {
	fn drop(&mut self) {
		extern "C" { fn cv_Dict_delete(instance: *mut c_void); }
		unsafe { cv_Dict_delete(self.as_raw_mut_Dict()) };
	}
}

impl Dict {
	#[inline] pub fn as_raw_Dict(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Dict(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Dict {}

impl crate::dnn::DictTrait for Dict {
	#[inline] fn as_raw_Dict(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Dict(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Dict {
}

/// This struct stores the scalar value (or array) of one of the following type: double, cv::String or int64.
/// @todo Maybe int64 is useless because double type exactly stores at least 2^52 integers.
pub trait DictValueTrait {
	fn as_raw_DictValue(&self) -> *const c_void;
	fn as_raw_mut_DictValue(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * idx: -1
	fn get_str(&self, idx: i32) -> Result<String> {
		unsafe { sys::cv_dnn_DictValue_get_cv_String_const_int(self.as_raw_DictValue(), idx) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
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
	ptr: *mut c_void
}

opencv_type_boxed! { DictValue }

impl Drop for DictValue {
	fn drop(&mut self) {
		extern "C" { fn cv_DictValue_delete(instance: *mut c_void); }
		unsafe { cv_DictValue_delete(self.as_raw_mut_DictValue()) };
	}
}

impl DictValue {
	#[inline] pub fn as_raw_DictValue(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_DictValue(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for DictValue {}

impl crate::dnn::DictValueTrait for DictValue {
	#[inline] fn as_raw_DictValue(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DictValue(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DictValue {
	pub fn copy(r: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_const_DictValueR(r.as_raw_DictValue()) }.into_result().map(|r| unsafe { crate::dnn::DictValue::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * i: 0
	pub fn from_i64(i: i64) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_int64_t(i) }.into_result().map(|r| unsafe { crate::dnn::DictValue::opencv_from_extern(r) } )
	}
	
	pub fn from_i32(i: i32) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_int(i) }.into_result().map(|r| unsafe { crate::dnn::DictValue::opencv_from_extern(r) } )
	}
	
	pub fn from_u32(p: u32) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_unsigned_int(p) }.into_result().map(|r| unsafe { crate::dnn::DictValue::opencv_from_extern(r) } )
	}
	
	pub fn from_f64(p: f64) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_double(p) }.into_result().map(|r| unsafe { crate::dnn::DictValue::opencv_from_extern(r) } )
	}
	
	pub fn from_str(s: &str) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(s);
		unsafe { sys::cv_dnn_DictValue_DictValue_const_charX(s.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::dnn::DictValue::opencv_from_extern(r) } )
	}
	
}

pub trait EltwiseLayer: crate::dnn::Layer {
	fn as_raw_EltwiseLayer(&self) -> *const c_void;
	fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void;

}

impl dyn EltwiseLayer + '_ {
	pub fn create(op: crate::dnn::EltwiseLayer_EltwiseOp, coeffs: &core::Vector::<i32>) -> Result<core::Ptr::<dyn crate::dnn::EltwiseLayer>> {
		unsafe { sys::cv_dnn_EltwiseLayer_create_EltwiseOp_const_vector_int_R(op, coeffs.as_raw_VectorOfi32()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::EltwiseLayer>::opencv_from_extern(r) } )
	}
	
}
/// Small interface class for loading trained serialized models of different dnn-frameworks.
pub trait Importer {
	fn as_raw_Importer(&self) -> *const c_void;
	fn as_raw_mut_Importer(&mut self) -> *mut c_void;

	/// Adds loaded layers into the @p net and sets connections between them.
	fn populate_net(&mut self, mut net: crate::dnn::Net) -> Result<()> {
		unsafe { sys::cv_dnn_Importer_populateNet_Net(self.as_raw_mut_Importer(), net.as_raw_mut_Net()) }.into_result()
	}
	
}

pub trait InnerProductLayer: crate::dnn::Layer {
	fn as_raw_InnerProductLayer(&self) -> *const c_void;
	fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void;

	fn axis(&self) -> i32 {
		unsafe { sys::cv_dnn_InnerProductLayer_getPropAxis_const(self.as_raw_InnerProductLayer()) }.into_result().expect("Infallible function failed: axis")
	}
	
	fn set_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_InnerProductLayer_setPropAxis_int(self.as_raw_mut_InnerProductLayer(), val) }.into_result().expect("Infallible function failed: set_axis")
	}
	
}

impl dyn InnerProductLayer + '_ {
	/// ## C++ default parameters
	/// * axis: 1
	pub fn create(axis: i32) -> Result<core::Ptr::<dyn crate::dnn::InnerProductLayer>> {
		unsafe { sys::cv_dnn_InnerProductLayer_create_int(axis) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::InnerProductLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait LRNLayer: crate::dnn::Layer {
	fn as_raw_LRNLayer(&self) -> *const c_void;
	fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void;

	fn typ(&self) -> i32 {
		unsafe { sys::cv_dnn_LRNLayer_getPropType_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: typ")
	}
	
	fn set_type(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setPropType_int(self.as_raw_mut_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_type")
	}
	
	fn size(&self) -> i32 {
		unsafe { sys::cv_dnn_LRNLayer_getPropSize_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: size")
	}
	
	fn set_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setPropSize_int(self.as_raw_mut_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_size")
	}
	
	fn alpha(&self) -> f64 {
		unsafe { sys::cv_dnn_LRNLayer_getPropAlpha_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: alpha")
	}
	
	fn set_alpha(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setPropAlpha_double(self.as_raw_mut_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_alpha")
	}
	
	fn beta(&self) -> f64 {
		unsafe { sys::cv_dnn_LRNLayer_getPropBeta_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: beta")
	}
	
	fn set_beta(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setPropBeta_double(self.as_raw_mut_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_beta")
	}
	
	fn bias(&self) -> f64 {
		unsafe { sys::cv_dnn_LRNLayer_getPropBias_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: bias")
	}
	
	fn set_bias(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setPropBias_double(self.as_raw_mut_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_bias")
	}
	
	fn norm_by_size(&self) -> bool {
		unsafe { sys::cv_dnn_LRNLayer_getPropNormBySize_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: norm_by_size")
	}
	
	fn set_norm_by_size(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setPropNormBySize_bool(self.as_raw_mut_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_norm_by_size")
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
	pub fn create(typ: i32, size: i32, alpha: f64, beta: f64, bias: f64, norm_by_size: bool) -> Result<core::Ptr::<dyn crate::dnn::LRNLayer>> {
		unsafe { sys::cv_dnn_LRNLayer_create_int_int_double_double_double_bool(typ, size, alpha, beta, bias, norm_by_size) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::LRNLayer>::opencv_from_extern(r) } )
	}
	
}
/// LSTM recurrent layer
pub trait LSTMLayer: crate::dnn::Layer {
	fn as_raw_LSTMLayer(&self) -> *const c_void;
	fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void;

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
		unsafe { sys::cv_dnn_LSTMLayer_setWeights_const_BlobR_const_BlobR_const_BlobR(self.as_raw_mut_LSTMLayer(), wh.as_raw_Blob(), wx.as_raw_Blob(), b.as_raw_Blob()) }.into_result()
	}
	
	/// Specifies shape of output blob which will be [[`T`], `N`] + @p outTailShape.
	/// @details If this parameter is empty or unset then @p outTailShape = [`Wh`.size(0)] will be used,
	/// where `Wh` is parameter from setWeights().
	/// 
	/// ## C++ default parameters
	/// * out_tail_shape: BlobShape::empty()
	fn set_out_shape(&mut self, out_tail_shape: &crate::dnn::BlobShape) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setOutShape_const_BlobShapeR(self.as_raw_mut_LSTMLayer(), out_tail_shape.as_raw_BlobShape()) }.into_result()
	}
	
	/// Set @f$ h_{t-1} @f$ value that will be used in next forward() calls.
	/// @details By-default @f$ h_{t-1} @f$ is inited by zeros and updated after each forward() call.
	fn set_h(&mut self, h: &crate::dnn::Blob) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setH_const_BlobR(self.as_raw_mut_LSTMLayer(), h.as_raw_Blob()) }.into_result()
	}
	
	/// Returns current @f$ h_{t-1} @f$ value (deep copy).
	fn get_h(&self) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_LSTMLayer_getH_const(self.as_raw_LSTMLayer()) }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
	}
	
	/// Set @f$ c_{t-1} @f$ value that will be used in next forward() calls.
	/// @details By-default @f$ c_{t-1} @f$ is inited by zeros and updated after each forward() call.
	fn set_c(&mut self, c: &crate::dnn::Blob) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setC_const_BlobR(self.as_raw_mut_LSTMLayer(), c.as_raw_Blob()) }.into_result()
	}
	
	/// Returns current @f$ c_{t-1} @f$ value (deep copy).
	fn get_c(&self) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_LSTMLayer_getC_const(self.as_raw_LSTMLayer()) }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
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
		unsafe { sys::cv_dnn_LSTMLayer_setUseTimstampsDim_bool(self.as_raw_mut_LSTMLayer(), use_) }.into_result()
	}
	
	/// If this flag is set to true then layer will produce @f$ c_t @f$ as second output.
	/// @details Shape of the second output is the same as first output.
	/// 
	/// ## C++ default parameters
	/// * produce: false
	fn set_produce_cell_output(&mut self, produce: bool) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setProduceCellOutput_bool(self.as_raw_mut_LSTMLayer(), produce) }.into_result()
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
	fn forward(&mut self, input: &mut core::Vector::<crate::dnn::Blob>, output: &mut core::Vector::<crate::dnn::Blob>) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_forward_vector_BlobX_R_vector_Blob_R(self.as_raw_mut_LSTMLayer(), input.as_raw_mut_VectorOfBlob(), output.as_raw_mut_VectorOfBlob()) }.into_result()
	}
	
	fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
		extern_container_arg!(mut input_name);
		unsafe { sys::cv_dnn_LSTMLayer_inputNameToIndex_String(self.as_raw_mut_LSTMLayer(), input_name.opencv_as_extern_mut()) }.into_result()
	}
	
	fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
		extern_container_arg!(mut output_name);
		unsafe { sys::cv_dnn_LSTMLayer_outputNameToIndex_String(self.as_raw_mut_LSTMLayer(), output_name.opencv_as_extern_mut()) }.into_result()
	}
	
}

impl dyn LSTMLayer + '_ {
	/// Creates instance of LSTM layer
	pub fn create() -> Result<core::Ptr::<dyn crate::dnn::LSTMLayer>> {
		unsafe { sys::cv_dnn_LSTMLayer_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::LSTMLayer>::opencv_from_extern(r) } )
	}
	
}
/// This interface class allows to build new Layers - are building blocks of networks.
/// 
/// Each class, derived from Layer, must implement allocate() methods to declare own outputs and forward() to compute outputs.
/// Also before using the new layer into networks you must register your layer by using one of @ref dnnLayerFactory "LayerFactory" macros.
pub trait Layer {
	fn as_raw_Layer(&self) -> *const c_void;
	fn as_raw_mut_Layer(&mut self) -> *mut c_void;

	/// List of learned parameters must be stored here to allow read them by using Net::getParam().
	fn blobs(&mut self) -> core::Vector::<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Layer_getPropBlobs(self.as_raw_mut_Layer()) }.into_result().map(|r| unsafe { core::Vector::<crate::dnn::Blob>::opencv_from_extern(r) } ).expect("Infallible function failed: blobs")
	}
	
	/// List of learned parameters must be stored here to allow read them by using Net::getParam().
	fn set_blobs(&mut self, mut val: core::Vector::<crate::dnn::Blob>) -> () {
		unsafe { sys::cv_dnn_Layer_setPropBlobs_vector_Blob_(self.as_raw_mut_Layer(), val.as_raw_mut_VectorOfBlob()) }.into_result().expect("Infallible function failed: set_blobs")
	}
	
	/// Name of the layer instance, can be used for logging or other internal purposes.
	fn name(&self) -> String {
		unsafe { sys::cv_dnn_Layer_getPropName_const(self.as_raw_Layer()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: name")
	}
	
	/// Name of the layer instance, can be used for logging or other internal purposes.
	fn set_name(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_dnn_Layer_setPropName_String(self.as_raw_mut_Layer(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_name")
	}
	
	/// Type name which was used for creating layer by layer factory.
	fn typ(&self) -> String {
		unsafe { sys::cv_dnn_Layer_getPropType_const(self.as_raw_Layer()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: typ")
	}
	
	/// Type name which was used for creating layer by layer factory.
	fn set_type(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_dnn_Layer_setPropType_String(self.as_raw_mut_Layer(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_type")
	}
	
	/// Allocates internal buffers and output blobs with respect to the shape of inputs.
	/// ## Parameters
	/// * input: vector of already allocated input blobs
	/// * output:[out] vector of output blobs, which must be allocated
	/// 
	/// This method must create each produced blob according to shape of @p input blobs and internal layer params.
	/// If this method is called first time then @p output vector consists from empty blobs and its size determined by number of output connections.
	/// This method can be called multiple times if size of any @p input blob was changed.
	fn allocate(&mut self, input: &core::Vector::<crate::dnn::Blob>, output: &mut core::Vector::<crate::dnn::Blob>) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_allocate_const_vector_BlobX_R_vector_Blob_R(self.as_raw_mut_Layer(), input.as_raw_VectorOfBlob(), output.as_raw_mut_VectorOfBlob()) }.into_result()
	}
	
	/// Given the @p input blobs, computes the output @p blobs.
	/// ## Parameters
	/// * input: the input blobs.
	/// * output:[out] allocated output blobs, which will store results of the computation.
	fn forward(&mut self, input: &mut core::Vector::<crate::dnn::Blob>, output: &mut core::Vector::<crate::dnn::Blob>) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_forward_vector_BlobX_R_vector_Blob_R(self.as_raw_mut_Layer(), input.as_raw_mut_VectorOfBlob(), output.as_raw_mut_VectorOfBlob()) }.into_result()
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
	fn allocate_1(&mut self, inputs: &core::Vector::<crate::dnn::Blob>, outputs: &mut core::Vector::<crate::dnn::Blob>) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_allocate_const_vector_Blob_R_vector_Blob_R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfBlob(), outputs.as_raw_mut_VectorOfBlob()) }.into_result()
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
	fn allocate_2(&mut self, inputs: &core::Vector::<crate::dnn::Blob>) -> Result<core::Vector::<crate::dnn::Blob>> {
		unsafe { sys::cv_dnn_Layer_allocate_const_vector_Blob_R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfBlob()) }.into_result().map(|r| unsafe { core::Vector::<crate::dnn::Blob>::opencv_from_extern(r) } )
	}
	
	/// Given the @p input blobs, computes the output @p blobs.
	/// ## Parameters
	/// * input: the input blobs.
	/// * output:[out] allocated output blobs, which will store results of the computation.
	/// 
	/// ## Overloaded parameters
	fn forward_1(&mut self, inputs: &core::Vector::<crate::dnn::Blob>, outputs: &mut core::Vector::<crate::dnn::Blob>) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_forward_const_vector_Blob_R_vector_Blob_R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfBlob(), outputs.as_raw_mut_VectorOfBlob()) }.into_result()
	}
	
	/// Allocates layer and computes output.
	fn run(&mut self, inputs: &core::Vector::<crate::dnn::Blob>, outputs: &mut core::Vector::<crate::dnn::Blob>) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_run_const_vector_Blob_R_vector_Blob_R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfBlob(), outputs.as_raw_mut_VectorOfBlob()) }.into_result()
	}
	
	/// Returns index of input blob into the input array.
	/// ## Parameters
	/// * inputName: label of input blob
	/// 
	/// Each layer input and output can be labeled to easily identify them using "%<layer_name%>[.output_name]" notation.
	/// This method maps label of input blob to its index into input vector.
	fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
		extern_container_arg!(mut input_name);
		unsafe { sys::cv_dnn_Layer_inputNameToIndex_String(self.as_raw_mut_Layer(), input_name.opencv_as_extern_mut()) }.into_result()
	}
	
	/// Returns index of output blob in output array.
	/// ## See also
	/// inputNameToIndex()
	fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
		extern_container_arg!(mut output_name);
		unsafe { sys::cv_dnn_Layer_outputNameToIndex_String(self.as_raw_mut_Layer(), output_name.opencv_as_extern_mut()) }.into_result()
	}
	
	fn set_params_from(&mut self, params: &crate::dnn::LayerParams) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_setParamsFrom_const_LayerParamsR(self.as_raw_mut_Layer(), params.as_raw_LayerParams()) }.into_result()
	}
	
}

/// %Layer factory allows to create instances of registered layers.
pub trait LayerFactoryTrait {
	fn as_raw_LayerFactory(&self) -> *const c_void;
	fn as_raw_mut_LayerFactory(&mut self) -> *mut c_void;

}

/// %Layer factory allows to create instances of registered layers.
pub struct LayerFactory {
	ptr: *mut c_void
}

opencv_type_boxed! { LayerFactory }

impl Drop for LayerFactory {
	fn drop(&mut self) {
		extern "C" { fn cv_LayerFactory_delete(instance: *mut c_void); }
		unsafe { cv_LayerFactory_delete(self.as_raw_mut_LayerFactory()) };
	}
}

impl LayerFactory {
	#[inline] pub fn as_raw_LayerFactory(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_LayerFactory(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for LayerFactory {}

impl crate::dnn::LayerFactoryTrait for LayerFactory {
	#[inline] fn as_raw_LayerFactory(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_LayerFactory(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LayerFactory {
	/// Registers the layer class with typename @p type and specified @p constructor.
	pub fn register_layer(typ: &str, constructor: crate::dnn::LayerFactory_Constuctor) -> Result<()> {
		extern_container_arg!(typ);
		unsafe { sys::cv_dnn_LayerFactory_registerLayer_const_StringR_Constuctor(typ.opencv_as_extern(), constructor) }.into_result()
	}
	
	/// Unregisters registered layer with specified type name.
	pub fn unregister_layer(typ: &str) -> Result<()> {
		extern_container_arg!(typ);
		unsafe { sys::cv_dnn_LayerFactory_unregisterLayer_const_StringR(typ.opencv_as_extern()) }.into_result()
	}
	
	/// Creates instance of registered layer.
	/// ## Parameters
	/// * type: type name of creating layer.
	/// * params: parameters which will be used for layer initialization.
	pub fn create_layer_instance(typ: &str, params: &mut crate::dnn::LayerParams) -> Result<core::Ptr::<dyn crate::dnn::Layer>> {
		extern_container_arg!(typ);
		unsafe { sys::cv_dnn_LayerFactory_createLayerInstance_const_StringR_LayerParamsR(typ.opencv_as_extern(), params.as_raw_mut_LayerParams()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::Layer>::opencv_from_extern(r) } )
	}
	
}

/// This class provides all data needed to initialize layer.
/// 
/// It includes dictionary with scalar params (which can be readed by using Dict interface),
/// blob params #blobs and optional meta information: #name and #type of layer instance.
pub trait LayerParamsTrait: crate::dnn::DictTrait {
	fn as_raw_LayerParams(&self) -> *const c_void;
	fn as_raw_mut_LayerParams(&mut self) -> *mut c_void;

	/// List of learned parameters stored as blobs.
	fn blobs(&mut self) -> core::Vector::<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_LayerParams_getPropBlobs(self.as_raw_mut_LayerParams()) }.into_result().map(|r| unsafe { core::Vector::<crate::dnn::Blob>::opencv_from_extern(r) } ).expect("Infallible function failed: blobs")
	}
	
	/// List of learned parameters stored as blobs.
	fn set_blobs(&mut self, mut val: core::Vector::<crate::dnn::Blob>) -> () {
		unsafe { sys::cv_dnn_LayerParams_setPropBlobs_vector_Blob_(self.as_raw_mut_LayerParams(), val.as_raw_mut_VectorOfBlob()) }.into_result().expect("Infallible function failed: set_blobs")
	}
	
	/// Name of the layer instance (optional, can be used internal purposes).
	fn name(&self) -> String {
		unsafe { sys::cv_dnn_LayerParams_getPropName_const(self.as_raw_LayerParams()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: name")
	}
	
	/// Name of the layer instance (optional, can be used internal purposes).
	fn set_name(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_dnn_LayerParams_setPropName_String(self.as_raw_mut_LayerParams(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_name")
	}
	
	/// Type name which was used for creating layer by layer factory (optional).
	fn typ(&self) -> String {
		unsafe { sys::cv_dnn_LayerParams_getPropType_const(self.as_raw_LayerParams()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: typ")
	}
	
	/// Type name which was used for creating layer by layer factory (optional).
	fn set_type(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_dnn_LayerParams_setPropType_String(self.as_raw_mut_LayerParams(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_type")
	}
	
}

/// This class provides all data needed to initialize layer.
/// 
/// It includes dictionary with scalar params (which can be readed by using Dict interface),
/// blob params #blobs and optional meta information: #name and #type of layer instance.
pub struct LayerParams {
	ptr: *mut c_void
}

opencv_type_boxed! { LayerParams }

impl Drop for LayerParams {
	fn drop(&mut self) {
		extern "C" { fn cv_LayerParams_delete(instance: *mut c_void); }
		unsafe { cv_LayerParams_delete(self.as_raw_mut_LayerParams()) };
	}
}

impl LayerParams {
	#[inline] pub fn as_raw_LayerParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_LayerParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for LayerParams {}

impl crate::dnn::DictTrait for LayerParams {
	#[inline] fn as_raw_Dict(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Dict(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerParamsTrait for LayerParams {
	#[inline] fn as_raw_LayerParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_LayerParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LayerParams {
}

pub trait MVNLayer: crate::dnn::Layer {
	fn as_raw_MVNLayer(&self) -> *const c_void;
	fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void;

	fn eps(&self) -> f64 {
		unsafe { sys::cv_dnn_MVNLayer_getPropEps_const(self.as_raw_MVNLayer()) }.into_result().expect("Infallible function failed: eps")
	}
	
	fn set_eps(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_MVNLayer_setPropEps_double(self.as_raw_mut_MVNLayer(), val) }.into_result().expect("Infallible function failed: set_eps")
	}
	
	fn norm_variance(&self) -> bool {
		unsafe { sys::cv_dnn_MVNLayer_getPropNormVariance_const(self.as_raw_MVNLayer()) }.into_result().expect("Infallible function failed: norm_variance")
	}
	
	fn set_norm_variance(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_MVNLayer_setPropNormVariance_bool(self.as_raw_mut_MVNLayer(), val) }.into_result().expect("Infallible function failed: set_norm_variance")
	}
	
	fn across_channels(&self) -> bool {
		unsafe { sys::cv_dnn_MVNLayer_getPropAcrossChannels_const(self.as_raw_MVNLayer()) }.into_result().expect("Infallible function failed: across_channels")
	}
	
	fn set_across_channels(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_MVNLayer_setPropAcrossChannels_bool(self.as_raw_mut_MVNLayer(), val) }.into_result().expect("Infallible function failed: set_across_channels")
	}
	
}

impl dyn MVNLayer + '_ {
	/// ## C++ default parameters
	/// * norm_variance: true
	/// * across_channels: false
	/// * eps: 1e-9
	pub fn create(norm_variance: bool, across_channels: bool, eps: f64) -> Result<core::Ptr::<dyn crate::dnn::MVNLayer>> {
		unsafe { sys::cv_dnn_MVNLayer_create_bool_bool_double(norm_variance, across_channels, eps) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::MVNLayer>::opencv_from_extern(r) } )
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
	fn as_raw_Net(&self) -> *const c_void;
	fn as_raw_mut_Net(&mut self) -> *mut c_void;

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
		extern_container_arg!(name);
		extern_container_arg!(typ);
		unsafe { sys::cv_dnn_Net_addLayer_const_StringR_const_StringR_LayerParamsR(self.as_raw_mut_Net(), name.opencv_as_extern(), typ.opencv_as_extern(), params.as_raw_mut_LayerParams()) }.into_result()
	}
	
	/// Adds new layer and connects its first input to the first output of previously added layer.
	/// ## See also
	/// addLayer()
	fn add_layer_to_prev(&mut self, name: &str, typ: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
		extern_container_arg!(name);
		extern_container_arg!(typ);
		unsafe { sys::cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_LayerParamsR(self.as_raw_mut_Net(), name.opencv_as_extern(), typ.opencv_as_extern(), params.as_raw_mut_LayerParams()) }.into_result()
	}
	
	/// Converts string name of the layer to the integer identifier.
	/// ## Returns
	/// id of the layer, or -1 if the layer wasn't found.
	fn get_layer_id(&mut self, layer: &str) -> Result<i32> {
		extern_container_arg!(layer);
		unsafe { sys::cv_dnn_Net_getLayerId_const_StringR(self.as_raw_mut_Net(), layer.opencv_as_extern()) }.into_result()
	}
	
	fn get_layer_names(&self) -> Result<core::Vector::<String>> {
		unsafe { sys::cv_dnn_Net_getLayerNames_const(self.as_raw_Net()) }.into_result().map(|r| unsafe { core::Vector::<String>::opencv_from_extern(r) } )
	}
	
	/// Returns pointer to layer with specified name which the network use.
	fn get_layer(&mut self, mut layer_id: crate::dnn::Net_LayerId) -> Result<core::Ptr::<dyn crate::dnn::Layer>> {
		unsafe { sys::cv_dnn_Net_getLayer_LayerId(self.as_raw_mut_Net(), layer_id.as_raw_mut_DictValue()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::Layer>::opencv_from_extern(r) } )
	}
	
	/// Delete layer for the network (not implemented yet)
	fn delete_layer(&mut self, mut layer: crate::dnn::Net_LayerId) -> Result<()> {
		unsafe { sys::cv_dnn_Net_deleteLayer_LayerId(self.as_raw_mut_Net(), layer.as_raw_mut_DictValue()) }.into_result()
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
		extern_container_arg!(mut out_pin);
		extern_container_arg!(mut inp_pin);
		unsafe { sys::cv_dnn_Net_connect_String_String(self.as_raw_mut_Net(), out_pin.opencv_as_extern_mut(), inp_pin.opencv_as_extern_mut()) }.into_result()
	}
	
	/// Connects #@p outNum output of the first layer to #@p inNum input of the second layer.
	/// ## Parameters
	/// * outLayerId: identifier of the first layer
	/// * inpLayerId: identifier of the second layer
	/// * outNum: number of the first layer output
	/// * inpNum: number of the second layer input
	fn connect(&mut self, out_layer_id: i32, out_num: i32, inp_layer_id: i32, inp_num: i32) -> Result<()> {
		unsafe { sys::cv_dnn_Net_connect_int_int_int_int(self.as_raw_mut_Net(), out_layer_id, out_num, inp_layer_id, inp_num) }.into_result()
	}
	
	/// Sets outputs names of the network input pseudo layer.
	/// 
	/// Each net always has special own the network input pseudo layer with id=0.
	/// This layer stores the user blobs only and don't make any computations.
	/// In fact, this layer provides the only way to pass user data into the network.
	/// As any other layer, this layer can label its outputs and this function provides an easy way to do this.
	fn set_net_inputs(&mut self, input_blob_names: &core::Vector::<String>) -> Result<()> {
		unsafe { sys::cv_dnn_Net_setNetInputs_const_vector_String_R(self.as_raw_mut_Net(), input_blob_names.as_raw_VectorOfString()) }.into_result()
	}
	
	/// Initializes and allocates all layers.
	fn allocate(&mut self) -> Result<()> {
		unsafe { sys::cv_dnn_Net_allocate(self.as_raw_mut_Net()) }.into_result()
	}
	
	/// Runs forward pass to compute output of layer @p toLayer.
	/// @details By default runs forward pass for the whole network.
	/// 
	/// ## C++ default parameters
	/// * to_layer: String()
	fn forward(&mut self, mut to_layer: crate::dnn::Net_LayerId) -> Result<()> {
		unsafe { sys::cv_dnn_Net_forward_LayerId(self.as_raw_mut_Net(), to_layer.as_raw_mut_DictValue()) }.into_result()
	}
	
	/// Runs forward pass to compute output of layer @p toLayer, but computations start from @p startLayer
	fn forward_1(&mut self, mut start_layer: crate::dnn::Net_LayerId, mut to_layer: crate::dnn::Net_LayerId) -> Result<()> {
		unsafe { sys::cv_dnn_Net_forward_LayerId_LayerId(self.as_raw_mut_Net(), start_layer.as_raw_mut_DictValue(), to_layer.as_raw_mut_DictValue()) }.into_result()
	}
	
	/// Runs forward pass to compute output of layer @p toLayer, but computations start from @p startLayer
	/// 
	/// ## Overloaded parameters
	fn forward_2(&mut self, start_layers: &core::Vector::<crate::dnn::Net_LayerId>, to_layers: &core::Vector::<crate::dnn::Net_LayerId>) -> Result<()> {
		unsafe { sys::cv_dnn_Net_forward_const_vector_LayerId_R_const_vector_LayerId_R(self.as_raw_mut_Net(), start_layers.as_raw_VectorOfNet_LayerId(), to_layers.as_raw_VectorOfNet_LayerId()) }.into_result()
	}
	
	/// Optimized forward.
	/// @warning Not implemented yet.
	/// @details Makes forward only those layers which weren't changed after previous forward().
	fn forward_opt(&mut self, mut to_layer: crate::dnn::Net_LayerId) -> Result<()> {
		unsafe { sys::cv_dnn_Net_forwardOpt_LayerId(self.as_raw_mut_Net(), to_layer.as_raw_mut_DictValue()) }.into_result()
	}
	
	/// Optimized forward.
	/// @warning Not implemented yet.
	/// @details Makes forward only those layers which weren't changed after previous forward().
	/// 
	/// ## Overloaded parameters
	fn forward_opt_1(&mut self, to_layers: &core::Vector::<crate::dnn::Net_LayerId>) -> Result<()> {
		unsafe { sys::cv_dnn_Net_forwardOpt_const_vector_LayerId_R(self.as_raw_mut_Net(), to_layers.as_raw_VectorOfNet_LayerId()) }.into_result()
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
		extern_container_arg!(mut output_name);
		unsafe { sys::cv_dnn_Net_setBlob_String_const_BlobR(self.as_raw_mut_Net(), output_name.opencv_as_extern_mut(), blob.as_raw_Blob()) }.into_result()
	}
	
	/// Returns the layer output blob.
	/// ## Parameters
	/// * outputName: the descriptor of the returning layer output blob.
	/// ## See also
	/// connect(String, String)
	fn get_blob(&mut self, output_name: &str) -> Result<crate::dnn::Blob> {
		extern_container_arg!(mut output_name);
		unsafe { sys::cv_dnn_Net_getBlob_String(self.as_raw_mut_Net(), output_name.opencv_as_extern_mut()) }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
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
	fn set_param(&mut self, mut layer: crate::dnn::Net_LayerId, num_param: i32, blob: &crate::dnn::Blob) -> Result<()> {
		unsafe { sys::cv_dnn_Net_setParam_LayerId_int_const_BlobR(self.as_raw_mut_Net(), layer.as_raw_mut_DictValue(), num_param, blob.as_raw_Blob()) }.into_result()
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
	fn get_param(&mut self, mut layer: crate::dnn::Net_LayerId, num_param: i32) -> Result<crate::dnn::Blob> {
		unsafe { sys::cv_dnn_Net_getParam_LayerId_int(self.as_raw_mut_Net(), layer.as_raw_mut_DictValue(), num_param) }.into_result().map(|r| unsafe { crate::dnn::Blob::opencv_from_extern(r) } )
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
	ptr: *mut c_void
}

opencv_type_boxed! { Net }

impl Drop for Net {
	fn drop(&mut self) {
		extern "C" { fn cv_Net_delete(instance: *mut c_void); }
		unsafe { cv_Net_delete(self.as_raw_mut_Net()) };
	}
}

impl Net {
	#[inline] pub fn as_raw_Net(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Net(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Net {}

impl crate::dnn::NetTrait for Net {
	#[inline] fn as_raw_Net(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Net(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Net {
	pub fn default() -> Result<crate::dnn::Net> {
		unsafe { sys::cv_dnn_Net_Net() }.into_result().map(|r| unsafe { crate::dnn::Net::opencv_from_extern(r) } )
	}
	
}

pub trait PoolingLayer: crate::dnn::Layer {
	fn as_raw_PoolingLayer(&self) -> *const c_void;
	fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void;

	fn typ(&self) -> i32 {
		unsafe { sys::cv_dnn_PoolingLayer_getPropType_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: typ")
	}
	
	fn set_type(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPropType_int(self.as_raw_mut_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_type")
	}
	
	fn kernel(&self) -> core::Size {
		unsafe { sys::cv_dnn_PoolingLayer_getPropKernel_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: kernel")
	}
	
	fn set_kernel(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPropKernel_Size(self.as_raw_mut_PoolingLayer(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_kernel")
	}
	
	fn stride(&self) -> core::Size {
		unsafe { sys::cv_dnn_PoolingLayer_getPropStride_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: stride")
	}
	
	fn set_stride(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPropStride_Size(self.as_raw_mut_PoolingLayer(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_stride")
	}
	
	fn pad(&self) -> core::Size {
		unsafe { sys::cv_dnn_PoolingLayer_getPropPad_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: pad")
	}
	
	fn set_pad(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPropPad_Size(self.as_raw_mut_PoolingLayer(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_pad")
	}
	
	fn global_pooling(&self) -> bool {
		unsafe { sys::cv_dnn_PoolingLayer_getPropGlobalPooling_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: global_pooling")
	}
	
	fn set_global_pooling(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPropGlobalPooling_bool(self.as_raw_mut_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_global_pooling")
	}
	
	fn pad_mode(&self) -> String {
		unsafe { sys::cv_dnn_PoolingLayer_getPropPadMode_const(self.as_raw_PoolingLayer()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: pad_mode")
	}
	
	fn set_pad_mode(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_dnn_PoolingLayer_setPropPadMode_String(self.as_raw_mut_PoolingLayer(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_pad_mode")
	}
	
}

impl dyn PoolingLayer + '_ {
	/// ## C++ default parameters
	/// * typ: PoolingLayer::MAX
	/// * kernel: Size(2,2)
	/// * stride: Size(1,1)
	/// * pad: Size(0,0)
	/// * pad_mode: ""
	pub fn create(typ: i32, kernel: core::Size, stride: core::Size, pad: core::Size, pad_mode: &str) -> Result<core::Ptr::<dyn crate::dnn::PoolingLayer>> {
		extern_container_arg!(pad_mode);
		unsafe { sys::cv_dnn_PoolingLayer_create_int_Size_Size_Size_const_StringR(typ, kernel.opencv_as_extern(), stride.opencv_as_extern(), pad.opencv_as_extern(), pad_mode.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::PoolingLayer>::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * typ: PoolingLayer::MAX
	pub fn create_global(typ: i32) -> Result<core::Ptr::<dyn crate::dnn::PoolingLayer>> {
		unsafe { sys::cv_dnn_PoolingLayer_createGlobal_int(typ) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::PoolingLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait PowerLayer: crate::dnn::Layer {
	fn as_raw_PowerLayer(&self) -> *const c_void;
	fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void;

	fn power(&self) -> f64 {
		unsafe { sys::cv_dnn_PowerLayer_getPropPower_const(self.as_raw_PowerLayer()) }.into_result().expect("Infallible function failed: power")
	}
	
	fn set_power(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_PowerLayer_setPropPower_double(self.as_raw_mut_PowerLayer(), val) }.into_result().expect("Infallible function failed: set_power")
	}
	
	fn scale(&self) -> f64 {
		unsafe { sys::cv_dnn_PowerLayer_getPropScale_const(self.as_raw_PowerLayer()) }.into_result().expect("Infallible function failed: scale")
	}
	
	fn set_scale(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_PowerLayer_setPropScale_double(self.as_raw_mut_PowerLayer(), val) }.into_result().expect("Infallible function failed: set_scale")
	}
	
	fn shift(&self) -> f64 {
		unsafe { sys::cv_dnn_PowerLayer_getPropShift_const(self.as_raw_PowerLayer()) }.into_result().expect("Infallible function failed: shift")
	}
	
	fn set_shift(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_PowerLayer_setPropShift_double(self.as_raw_mut_PowerLayer(), val) }.into_result().expect("Infallible function failed: set_shift")
	}
	
}

impl dyn PowerLayer + '_ {
	/// ## C++ default parameters
	/// * power: 1
	/// * scale: 1
	/// * shift: 0
	pub fn create(power: f64, scale: f64, shift: f64) -> Result<core::Ptr::<dyn crate::dnn::PowerLayer>> {
		unsafe { sys::cv_dnn_PowerLayer_create_double_double_double(power, scale, shift) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::PowerLayer>::opencv_from_extern(r) } )
	}
	
}
/// Classical recurrent layer
pub trait RNNLayer: crate::dnn::Layer {
	fn as_raw_RNNLayer(&self) -> *const c_void;
	fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void;

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
		unsafe { sys::cv_dnn_RNNLayer_setWeights_const_BlobR_const_BlobR_const_BlobR_const_BlobR_const_BlobR(self.as_raw_mut_RNNLayer(), wxh.as_raw_Blob(), bh.as_raw_Blob(), whh.as_raw_Blob(), who.as_raw_Blob(), bo.as_raw_Blob()) }.into_result()
	}
	
	/// If this flag is set to true then layer will produce @f$ h_t @f$ as second output.
	/// @details Shape of the second output is the same as first output.
	/// 
	/// ## C++ default parameters
	/// * produce: false
	fn set_produce_hidden_output(&mut self, produce: bool) -> Result<()> {
		unsafe { sys::cv_dnn_RNNLayer_setProduceHiddenOutput_bool(self.as_raw_mut_RNNLayer(), produce) }.into_result()
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
	fn forward(&mut self, input: &mut core::Vector::<crate::dnn::Blob>, output: &mut core::Vector::<crate::dnn::Blob>) -> Result<()> {
		unsafe { sys::cv_dnn_RNNLayer_forward_vector_BlobX_R_vector_Blob_R(self.as_raw_mut_RNNLayer(), input.as_raw_mut_VectorOfBlob(), output.as_raw_mut_VectorOfBlob()) }.into_result()
	}
	
}

impl dyn RNNLayer + '_ {
	/// Creates instance of RNNLayer
	pub fn create() -> Result<core::Ptr::<dyn crate::dnn::RNNLayer>> {
		unsafe { sys::cv_dnn_RNNLayer_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::RNNLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait ReLULayer: crate::dnn::Layer {
	fn as_raw_ReLULayer(&self) -> *const c_void;
	fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void;

	fn negative_slope(&self) -> f64 {
		unsafe { sys::cv_dnn_ReLULayer_getPropNegativeSlope_const(self.as_raw_ReLULayer()) }.into_result().expect("Infallible function failed: negative_slope")
	}
	
	fn set_negative_slope(&mut self, val: f64) -> () {
		unsafe { sys::cv_dnn_ReLULayer_setPropNegativeSlope_double(self.as_raw_mut_ReLULayer(), val) }.into_result().expect("Infallible function failed: set_negative_slope")
	}
	
}

impl dyn ReLULayer + '_ {
	/// ## C++ default parameters
	/// * negative_slope: 0
	pub fn create(negative_slope: f64) -> Result<core::Ptr::<dyn crate::dnn::ReLULayer>> {
		unsafe { sys::cv_dnn_ReLULayer_create_double(negative_slope) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::ReLULayer>::opencv_from_extern(r) } )
	}
	
}
pub trait ReshapeLayer: crate::dnn::Layer {
	fn as_raw_ReshapeLayer(&self) -> *const c_void;
	fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void;

	fn new_shape_desc(&mut self) -> crate::dnn::BlobShape {
		unsafe { sys::cv_dnn_ReshapeLayer_getPropNewShapeDesc(self.as_raw_mut_ReshapeLayer()) }.into_result().map(|r| unsafe { crate::dnn::BlobShape::opencv_from_extern(r) } ).expect("Infallible function failed: new_shape_desc")
	}
	
	fn set_new_shape_desc(&mut self, mut val: crate::dnn::BlobShape) -> () {
		unsafe { sys::cv_dnn_ReshapeLayer_setPropNewShapeDesc_BlobShape(self.as_raw_mut_ReshapeLayer(), val.as_raw_mut_BlobShape()) }.into_result().expect("Infallible function failed: set_new_shape_desc")
	}
	
	fn new_shape_range(&mut self) -> core::Range {
		unsafe { sys::cv_dnn_ReshapeLayer_getPropNewShapeRange(self.as_raw_mut_ReshapeLayer()) }.into_result().map(|r| unsafe { core::Range::opencv_from_extern(r) } ).expect("Infallible function failed: new_shape_range")
	}
	
	fn set_new_shape_range(&mut self, mut val: core::Range) -> () {
		unsafe { sys::cv_dnn_ReshapeLayer_setPropNewShapeRange_Range(self.as_raw_mut_ReshapeLayer(), val.as_raw_mut_Range()) }.into_result().expect("Infallible function failed: set_new_shape_range")
	}
	
}

impl dyn ReshapeLayer + '_ {
	/// ## C++ default parameters
	/// * applying_range: Range::all()
	/// * enable_reordering: false
	pub fn create(new_shape: &crate::dnn::BlobShape, mut applying_range: core::Range, enable_reordering: bool) -> Result<core::Ptr::<dyn crate::dnn::ReshapeLayer>> {
		unsafe { sys::cv_dnn_ReshapeLayer_create_const_BlobShapeR_Range_bool(new_shape.as_raw_BlobShape(), applying_range.as_raw_mut_Range(), enable_reordering) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::ReshapeLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait SigmoidLayer: crate::dnn::Layer {
	fn as_raw_SigmoidLayer(&self) -> *const c_void;
	fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void;

}

impl dyn SigmoidLayer + '_ {
	pub fn create() -> Result<core::Ptr::<dyn crate::dnn::SigmoidLayer>> {
		unsafe { sys::cv_dnn_SigmoidLayer_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::SigmoidLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait SliceLayer: crate::dnn::Layer {
	fn as_raw_SliceLayer(&self) -> *const c_void;
	fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void;

	fn axis(&self) -> i32 {
		unsafe { sys::cv_dnn_SliceLayer_getPropAxis_const(self.as_raw_SliceLayer()) }.into_result().expect("Infallible function failed: axis")
	}
	
	fn set_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_SliceLayer_setPropAxis_int(self.as_raw_mut_SliceLayer(), val) }.into_result().expect("Infallible function failed: set_axis")
	}
	
	fn slice_indices(&mut self) -> core::Vector::<i32> {
		unsafe { sys::cv_dnn_SliceLayer_getPropSliceIndices(self.as_raw_mut_SliceLayer()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } ).expect("Infallible function failed: slice_indices")
	}
	
	fn set_slice_indices(&mut self, mut val: core::Vector::<i32>) -> () {
		unsafe { sys::cv_dnn_SliceLayer_setPropSliceIndices_vector_int_(self.as_raw_mut_SliceLayer(), val.as_raw_mut_VectorOfi32()) }.into_result().expect("Infallible function failed: set_slice_indices")
	}
	
}

impl dyn SliceLayer + '_ {
	pub fn create(axis: i32) -> Result<core::Ptr::<dyn crate::dnn::SliceLayer>> {
		unsafe { sys::cv_dnn_SliceLayer_create_int(axis) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::SliceLayer>::opencv_from_extern(r) } )
	}
	
	pub fn create_1(axis: i32, slice_indices: &core::Vector::<i32>) -> Result<core::Ptr::<dyn crate::dnn::SliceLayer>> {
		unsafe { sys::cv_dnn_SliceLayer_create_int_const_vector_int_R(axis, slice_indices.as_raw_VectorOfi32()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::SliceLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait SoftmaxLayer: crate::dnn::Layer {
	fn as_raw_SoftmaxLayer(&self) -> *const c_void;
	fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void;

}

impl dyn SoftmaxLayer + '_ {
	/// ## C++ default parameters
	/// * axis: 1
	pub fn create(axis: i32) -> Result<core::Ptr::<dyn crate::dnn::SoftmaxLayer>> {
		unsafe { sys::cv_dnn_SoftmaxLayer_create_int(axis) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::SoftmaxLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait SplitLayer: crate::dnn::Layer {
	fn as_raw_SplitLayer(&self) -> *const c_void;
	fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void;

	/// Number of copies that will be produced (is ignored when negative).
	fn outputs_count(&self) -> i32 {
		unsafe { sys::cv_dnn_SplitLayer_getPropOutputsCount_const(self.as_raw_SplitLayer()) }.into_result().expect("Infallible function failed: outputs_count")
	}
	
	/// Number of copies that will be produced (is ignored when negative).
	fn set_outputs_count(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_SplitLayer_setPropOutputsCount_int(self.as_raw_mut_SplitLayer(), val) }.into_result().expect("Infallible function failed: set_outputs_count")
	}
	
}

impl dyn SplitLayer + '_ {
	/// ## C++ default parameters
	/// * outputs_count: -1
	pub fn create(outputs_count: i32) -> Result<core::Ptr::<dyn crate::dnn::SplitLayer>> {
		unsafe { sys::cv_dnn_SplitLayer_create_int(outputs_count) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::SplitLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait TanHLayer: crate::dnn::Layer {
	fn as_raw_TanHLayer(&self) -> *const c_void;
	fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void;

}

impl dyn TanHLayer + '_ {
	pub fn create() -> Result<core::Ptr::<dyn crate::dnn::TanHLayer>> {
		unsafe { sys::cv_dnn_TanHLayer_create() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::dnn::TanHLayer>::opencv_from_extern(r) } )
	}
	
}
pub trait _RangeTrait: core::RangeTrait {
	fn as_raw__Range(&self) -> *const c_void;
	fn as_raw_mut__Range(&mut self) -> *mut c_void;

}

pub struct _Range {
	ptr: *mut c_void
}

opencv_type_boxed! { _Range }

impl Drop for _Range {
	fn drop(&mut self) {
		extern "C" { fn cv__Range_delete(instance: *mut c_void); }
		unsafe { cv__Range_delete(self.as_raw_mut__Range()) };
	}
}

impl _Range {
	#[inline] pub fn as_raw__Range(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut__Range(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for _Range {}

impl core::RangeTrait for _Range {
	#[inline] fn as_raw_Range(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Range(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::_RangeTrait for _Range {
	#[inline] fn as_raw__Range(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut__Range(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl _Range {
	pub fn new(r: &core::Range) -> Result<crate::dnn::_Range> {
		unsafe { sys::cv_dnn__Range__Range_const_RangeR(r.as_raw_Range()) }.into_result().map(|r| unsafe { crate::dnn::_Range::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * size: 1
	pub fn new_1(start: i32, size: i32) -> Result<crate::dnn::_Range> {
		unsafe { sys::cv_dnn__Range__Range_int_int(start, size) }.into_result().map(|r| unsafe { crate::dnn::_Range::opencv_from_extern(r) } )
	}
	
}
pub use crate::manual::dnn::*;

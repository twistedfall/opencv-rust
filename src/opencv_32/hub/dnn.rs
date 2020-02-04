//! # Deep Neural Network module
//! This module contains:
//! - API for new layers creation, layers are building bricks of neural networks;
//! - set of built-in most-useful Layers;
//! - API to constuct and modify comprehensive neural networks from layers;
//! - functionality for loading serialized networks models from differnet frameworks.
//!
//! Functionality of this module is designed only for forward pass computations (i. e. network testing).
//! A network training is in principle not supported.
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

pub const Blob_ALLOC_MAT: i32 = 1 << 0;
pub const Blob_ALLOC_UMAT: i32 = 1 << 1;
pub const Blob_HEAD_AT_MAT: i32 = 1 << 0;
pub const Blob_HEAD_AT_UMAT: i32 = 1 << 1;
pub const Blob_UNINITIALIZED: i32 = 0;
pub const EltwiseLayer_PROD: i32 = 0;
pub const EltwiseLayer_SUM: i32 = 1;
pub const LRNLayer_CHANNEL_NRM: i32 = 0;
pub const LRNLayer_SPATIAL_NRM: i32 = 1;
pub const PoolingLayer_AVE: i32 = 1;
pub const PoolingLayer_MAX: i32 = 0;
pub const PoolingLayer_STOCHASTIC: i32 = 2;

///
/// ## C++ default parameters
/// * src_range: Range::all()
pub fn compute_shape_by_reshape_mask(src_shape: &crate::dnn::BlobShape, mask_shape: &crate::dnn::BlobShape, src_range: &core::Range) -> Result<crate::dnn::BlobShape> {
    unsafe { sys::cv_dnn_computeShapeByReshapeMask_BlobShape_BlobShape_Range(src_shape.as_raw_BlobShape(), mask_shape.as_raw_BlobShape(), src_range.as_raw_Range()) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
}

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
    unsafe { sys::cv_dnn_createCaffeImporter_String_String(prototxt.as_ptr(), caffe_model.as_ptr()) }.into_result().map(|ptr| types::PtrOfImporter { ptr })
}

/// Creates the importer of <a href="http://www.tensorflow.org">TensorFlow</a> framework network.
/// ## Parameters
/// * model: path to the .pb file with binary protobuf description of the network architecture.
/// ## Returns
/// Pointer to the created importer, NULL in failure cases.
pub fn create_tensorflow_importer(model: &str) -> Result<types::PtrOfImporter> {
    string_arg!(model);
    unsafe { sys::cv_dnn_createTensorflowImporter_String(model.as_ptr()) }.into_result().map(|ptr| types::PtrOfImporter { ptr })
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
    unsafe { sys::cv_dnn_createTorchImporter_String_bool(filename.as_ptr(), is_binary) }.into_result().map(|ptr| types::PtrOfImporter { ptr })
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
    unsafe { sys::cv_dnn_readNetFromCaffe_String_String(prototxt.as_ptr(), caffe_model.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Loads blob which was serialized as torch.Tensor object of Torch7 framework.
///  @warning This function has the same limitations as createTorchImporter().
///
/// ## C++ default parameters
/// * is_binary: true
pub fn read_torch_blob(filename: &str, is_binary: bool) -> Result<crate::dnn::Blob> {
    string_arg!(filename);
    unsafe { sys::cv_dnn_readTorchBlob_String_bool(filename.as_ptr(), is_binary) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
}

// boxed class cv::dnn::AbsLayer
pub struct AbsLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for AbsLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_AbsLayer_delete(self.ptr) };
    }
}

impl AbsLayer {
    #[inline(always)] pub fn as_raw_AbsLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for AbsLayer {}

impl crate::dnn::Layer for AbsLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl AbsLayer {
    pub fn create() -> Result<types::PtrOfAbsLayer> {
        unsafe { sys::cv_dnn_AbsLayer_create() }.into_result().map(|ptr| types::PtrOfAbsLayer { ptr })
    }
    
}

// boxed class cv::dnn::BNLLLayer
pub struct BNLLLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BNLLLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_BNLLLayer_delete(self.ptr) };
    }
}

impl BNLLLayer {
    #[inline(always)] pub fn as_raw_BNLLLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BNLLLayer {}

impl crate::dnn::Layer for BNLLLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl BNLLLayer {
    pub fn create() -> Result<types::PtrOfBNLLLayer> {
        unsafe { sys::cv_dnn_BNLLLayer_create() }.into_result().map(|ptr| types::PtrOfBNLLLayer { ptr })
    }
    
}

// Generating impl for trait crate::dnn::BaseConvolutionLayer
pub trait BaseConvolutionLayerTrait: crate::dnn::Layer {
    fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void;
    fn kernel(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_kernel_const(self.as_raw_BaseConvolutionLayer()) }.into_result()
    }
    
    fn set_kernel(&mut self, val: core::Size) -> Result<()> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_set_kernel_Size(self.as_raw_BaseConvolutionLayer(), val) }.into_result()
    }
    
    fn stride(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_stride_const(self.as_raw_BaseConvolutionLayer()) }.into_result()
    }
    
    fn set_stride(&mut self, val: core::Size) -> Result<()> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_set_stride_Size(self.as_raw_BaseConvolutionLayer(), val) }.into_result()
    }
    
    fn pad(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_pad_const(self.as_raw_BaseConvolutionLayer()) }.into_result()
    }
    
    fn set_pad(&mut self, val: core::Size) -> Result<()> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_set_pad_Size(self.as_raw_BaseConvolutionLayer(), val) }.into_result()
    }
    
    fn dilation(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_dilation_const(self.as_raw_BaseConvolutionLayer()) }.into_result()
    }
    
    fn set_dilation(&mut self, val: core::Size) -> Result<()> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_set_dilation_Size(self.as_raw_BaseConvolutionLayer(), val) }.into_result()
    }
    
    fn pad_mode(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_padMode(self.as_raw_BaseConvolutionLayer()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    fn set_pad_mode(&mut self, val: &str) -> Result<()> {
        string_arg!(mut val);
        unsafe { sys::cv_dnn_BaseConvolutionLayer_set_padMode_String(self.as_raw_BaseConvolutionLayer(), val.as_ptr() as _) }.into_result()
    }
    
}

// boxed class cv::dnn::BaseConvolutionLayer
pub struct BaseConvolutionLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BaseConvolutionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_BaseConvolutionLayer_delete(self.ptr) };
    }
}

impl BaseConvolutionLayer {
    #[inline(always)] pub fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BaseConvolutionLayer {}

impl crate::dnn::BaseConvolutionLayerTrait for BaseConvolutionLayer {
    #[inline(always)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::Layer for BaseConvolutionLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::dnn::Blob
/// This class provides methods for continuous n-dimensional CPU and GPU array processing.
///
/// The class is realized as a wrapper over @ref cv::Mat and @ref cv::UMat.
/// It will support methods for switching and logical synchronization between CPU and GPU.
pub struct Blob {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for Blob {
    fn drop(&mut self) {
        unsafe { sys::cv_Blob_delete(self.ptr) };
    }
}

impl Blob {
    #[inline(always)] pub fn as_raw_Blob(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Blob {}

impl Blob {
    pub fn default() -> Result<crate::dnn::Blob> {
        unsafe { sys::cv_dnn_Blob_Blob() }.into_result().map(|ptr| crate::dnn::Blob { ptr })
    }
    
    /// Constructs blob with specified @p shape and @p type.
    ///
    /// ## C++ default parameters
    /// * _type: CV_32F
    /// * alloc_flags: ALLOC_MAT
    pub fn new(shape: &crate::dnn::BlobShape, _type: i32, alloc_flags: i32) -> Result<crate::dnn::Blob> {
        unsafe { sys::cv_dnn_Blob_Blob_BlobShape_int_int(shape.as_raw_BlobShape(), _type, alloc_flags) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
    }
    
    /// Constructs Blob from existing Mat or UMat.
    pub fn new_1(data: &dyn core::ToInputArray) -> Result<crate::dnn::Blob> {
        input_array_arg!(data);
        unsafe { sys::cv_dnn_Blob_Blob__InputArray(data.as_raw__InputArray()) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
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
        unsafe { sys::cv_dnn_Blob_fromImages__InputArray_int(image.as_raw__InputArray(), dst_cn) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
    }
    
    /// Works like Blob::fromImages() but in-place.
    ///
    /// ## C++ default parameters
    /// * dst_cn: -1
    pub fn batch_from_images(&mut self, image: &dyn core::ToInputArray, dst_cn: i32) -> Result<()> {
        input_array_arg!(image);
        unsafe { sys::cv_dnn_Blob_batchFromImages__InputArray_int(self.as_raw_Blob(), image.as_raw__InputArray(), dst_cn) }.into_result()
    }
    
    /// Creates blob with specified @p shape and @p type.
    ///
    /// ## C++ default parameters
    /// * _type: CV_32F
    /// * alloc_flags: ALLOC_MAT
    pub fn create(&mut self, shape: &crate::dnn::BlobShape, _type: i32, alloc_flags: i32) -> Result<()> {
        unsafe { sys::cv_dnn_Blob_create_BlobShape_int_int(self.as_raw_Blob(), shape.as_raw_BlobShape(), _type, alloc_flags) }.into_result()
    }
    
    /// Creates blob from Mat or UMat without copying the data.
    /// @details If in is Mat then Mat data is populated, otherwise - UMat.
    pub fn fill(&mut self, _in: &dyn core::ToInputArray) -> Result<()> {
        input_array_arg!(_in);
        unsafe { sys::cv_dnn_Blob_fill__InputArray(self.as_raw_Blob(), _in.as_raw__InputArray()) }.into_result()
    }
    
    /// Creates blob from user data.
    ///  @details If @p deepCopy is false then CPU data will not be allocated.
    ///
    /// ## C++ default parameters
    /// * deep_copy: true
    pub fn fill_1(&mut self, shape: &crate::dnn::BlobShape, _type: i32, data: &mut c_void, deep_copy: bool) -> Result<()> {
        unsafe { sys::cv_dnn_Blob_fill_BlobShape_int_void_X_bool(self.as_raw_Blob(), shape.as_raw_BlobShape(), _type, data, deep_copy) }.into_result()
    }
    
    /// Sets @p value to the last used data (if @p allocFlags = -1).
    /// @details If @p allocFlags != -1 then destination data (Mat or UMat) is determined by flags from AllocFlag enum like in create().
    ///
    /// ## C++ default parameters
    /// * alloc_flags: -1
    pub fn set_to(&mut self, value: &dyn core::ToInputArray, alloc_flags: i32) -> Result<()> {
        input_array_arg!(value);
        unsafe { sys::cv_dnn_Blob_setTo__InputArray_int(self.as_raw_Blob(), value.as_raw__InputArray(), alloc_flags) }.into_result()
    }
    
    /// Returns reference to cv::Mat, containing blob data.
    ///
    /// ## C++ default parameters
    /// * write_only: true
    pub fn mat_ref(&mut self, write_only: bool) -> Result<core::Mat> {
        unsafe { sys::cv_dnn_Blob_matRef_bool(self.as_raw_Blob(), write_only) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns reference to cv::Mat, containing blob data, for read-only purposes.
    pub fn mat_ref_const(&self) -> Result<core::Mat> {
        unsafe { sys::cv_dnn_Blob_matRefConst_const(self.as_raw_Blob()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns reference to cv::UMat, containing blob data.
    ///
    /// ## C++ default parameters
    /// * write_only: true
    pub fn umat_ref(&mut self, write_only: bool) -> Result<core::UMat> {
        unsafe { sys::cv_dnn_Blob_umatRef_bool(self.as_raw_Blob(), write_only) }.into_result().map(|ptr| core::UMat { ptr })
    }
    
    /// Returns reference to cv::UMat, containing blob data, for read-only purposes.
    pub fn umat_ref_const(&self) -> Result<core::UMat> {
        unsafe { sys::cv_dnn_Blob_umatRefConst_const(self.as_raw_Blob()) }.into_result().map(|ptr| core::UMat { ptr })
    }
    
    /// Actualizes data stored inside Mat of Blob; if @p syncData is false then only shape will be actualized.
    ///
    /// ## C++ default parameters
    /// * sync_data: true
    pub fn update_mat(&self, sync_data: bool) -> Result<()> {
        unsafe { sys::cv_dnn_Blob_updateMat_const_bool(self.as_raw_Blob(), sync_data) }.into_result()
    }
    
    /// Actualizes data stored inside Mat of Blob; if @p syncData is false then only shape will be actualized.
    ///
    /// ## C++ default parameters
    /// * sync_data: true
    pub fn update_u_mat(&self, sync_data: bool) -> Result<()> {
        unsafe { sys::cv_dnn_Blob_updateUMat_const_bool(self.as_raw_Blob(), sync_data) }.into_result()
    }
    
    /// Updates Mat and UMat of Blob.
    pub fn sync(&self) -> Result<()> {
        unsafe { sys::cv_dnn_Blob_sync_const(self.as_raw_Blob()) }.into_result()
    }
    
    /// Returns number of blob dimensions.
    pub fn dims(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_Blob_dims_const(self.as_raw_Blob()) }.into_result()
    }
    
    /// Returns the size of the specified @p axis.
    ///
    /// Negative @p axis is supported, in this case a counting starts from the last axis,
    /// i. e. -1 corresponds to last axis.
    /// If non-existing axis was passed then an error will be generated.
    pub fn size(&self, axis: i32) -> Result<i32> {
        unsafe { sys::cv_dnn_Blob_size_const_int(self.as_raw_Blob(), axis) }.into_result()
    }
    
    /// Returns the size of the specified @p axis.
    ///
    /// Does the same thing as size(int) const, but if non-existing axis will be passed then 1 will be returned,
    /// therefore this function always finishes successfully.
    pub fn xsize(&self, axis: i32) -> Result<i32> {
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
    pub fn total(&self, start_axis: i32, end_axis: i32) -> Result<size_t> {
        unsafe { sys::cv_dnn_Blob_total_const_int_int(self.as_raw_Blob(), start_axis, end_axis) }.into_result()
    }
    
    /// Converts @p axis index to canonical format (where 0 <= @p axis < dims()).
    pub fn canonical_axis(&self, axis: i32) -> Result<i32> {
        unsafe { sys::cv_dnn_Blob_canonicalAxis_const_int(self.as_raw_Blob(), axis) }.into_result()
    }
    
    /// Returns shape of the blob.
    pub fn shape(&self) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_Blob_shape_const(self.as_raw_Blob()) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    /// Checks equality of two blobs shapes.
    pub fn equal_shape(&self, other: &crate::dnn::Blob) -> Result<bool> {
        unsafe { sys::cv_dnn_Blob_equalShape_const_Blob(self.as_raw_Blob(), other.as_raw_Blob()) }.into_result()
    }
    
    /// Returns slice of first two dimensions.
    ///  @details The behaviour is similar to the following numpy code: blob[n, cn, ...]
    pub fn get_plane(&mut self, n: i32, cn: i32) -> Result<core::Mat> {
        unsafe { sys::cv_dnn_Blob_getPlane_int_int(self.as_raw_Blob(), n, cn) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns slice of first dimension.
    ///  @details The behaviour is similar to getPlane(), but returns all
    /// channels * rows * cols values, corresponding to the n-th value
    /// of the first dimension.
    pub fn get_planes(&mut self, n: i32) -> Result<core::Mat> {
        unsafe { sys::cv_dnn_Blob_getPlanes_int(self.as_raw_Blob(), n) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns size of the fourth axis blob.
    pub fn cols(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_Blob_cols_const(self.as_raw_Blob()) }.into_result()
    }
    
    /// Returns size of the thrid  axis blob.
    pub fn rows(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_Blob_rows_const(self.as_raw_Blob()) }.into_result()
    }
    
    /// Returns size of the second axis blob.
    pub fn channels(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_Blob_channels_const(self.as_raw_Blob()) }.into_result()
    }
    
    /// Returns size of the first  axis blob.
    pub fn num(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_Blob_num_const(self.as_raw_Blob()) }.into_result()
    }
    
    /// Returns cv::Size(cols(), rows())
    pub fn size2(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_Blob_size2_const(self.as_raw_Blob()) }.into_result()
    }
    
    /// Returns shape of first four blob axes.
    pub fn shape4(&self) -> Result<core::Vec4i> {
        unsafe { sys::cv_dnn_Blob_shape4_const(self.as_raw_Blob()) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * n: 0
    /// * cn: 0
    /// * row: 0
    /// * col: 0
    pub fn offset(&self, n: i32, cn: i32, row: i32, col: i32) -> Result<size_t> {
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
    pub fn ptr(&mut self, n: i32, cn: i32, row: i32, col: i32) -> Result<&mut u8> {
        unsafe { sys::cv_dnn_Blob_ptr_int_int_int_int(self.as_raw_Blob(), n, cn, row, col) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
    }
    
    /// ptr<float>()
    ///
    /// ## C++ default parameters
    /// * n: 0
    /// * cn: 0
    /// * row: 0
    /// * col: 0
    pub fn ptrf(&mut self, n: i32, cn: i32, row: i32, col: i32) -> Result<&mut f32> {
        unsafe { sys::cv_dnn_Blob_ptrf_int_int_int_int(self.as_raw_Blob(), n, cn, row, col) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
    }
    
    /// Shares data from other @p blob.
    /// ## Returns
    /// *this
    pub fn share_from(&mut self, blob: &crate::dnn::Blob) -> Result<crate::dnn::Blob> {
        unsafe { sys::cv_dnn_Blob_shareFrom_Blob(self.as_raw_Blob(), blob.as_raw_Blob()) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
    }
    
    /// Changes shape of the blob without copying the data.
    /// ## Returns
    /// *this
    pub fn reshape(&mut self, shape: &crate::dnn::BlobShape) -> Result<crate::dnn::Blob> {
        unsafe { sys::cv_dnn_Blob_reshape_BlobShape(self.as_raw_Blob(), shape.as_raw_BlobShape()) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
    }
    
    /// Changes shape of the blob without copying the data.
    /// ## Returns
    /// shallow copy of original blob with new shape.
    pub fn reshaped(&self, new_shape: &crate::dnn::BlobShape) -> Result<crate::dnn::Blob> {
        unsafe { sys::cv_dnn_Blob_reshaped_const_BlobShape(self.as_raw_Blob(), new_shape.as_raw_BlobShape()) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
    }
    
    /// Returns type of the blob.
    pub fn _type(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_Blob_type_const(self.as_raw_Blob()) }.into_result()
    }
    
    /// Returns size of single element in bytes.
    pub fn elem_size(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_Blob_elemSize_const(self.as_raw_Blob()) }.into_result()
    }
    
    /// Returns current state of the blob, @see DataState.
    pub fn get_state(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_Blob_getState_const(self.as_raw_Blob()) }.into_result()
    }
    
}

// boxed class cv::dnn::BlobShape
/// Lightweight class for storing and processing a shape of blob (or anything else).
pub struct BlobShape {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BlobShape {
    fn drop(&mut self) {
        unsafe { sys::cv_BlobShape_delete(self.ptr) };
    }
}

impl BlobShape {
    #[inline(always)] pub fn as_raw_BlobShape(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BlobShape {}

impl BlobShape {
    /// Creates [1, 1, 1, 1] shape @todo Make more clearer behavior.
    pub fn default() -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_BlobShape() }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    /// Creates 1-dim shape [@p s0]
    pub fn new(s0: i32) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_BlobShape_int(s0) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    pub fn new_1(s0: i32, s1: i32) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_BlobShape_int_int(s0, s1) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    pub fn new_2(s0: i32, s1: i32, s2: i32) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_BlobShape_int_int_int(s0, s1, s2) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    /// Creates 4-dim shape [@p num, @p cn, @p rows, @p cols]
    pub fn new_3(num: i32, cn: i32, rows: i32, cols: i32) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_BlobShape_int_int_int_int(num, cn, rows, cols) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    /// Creates n-dim shape from the @p sizes array; if @p sizes is NULL then shape will contain unspecified data
    pub fn new_4(ndims: i32, sizes: &i32) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_BlobShape_int_const_int_X(ndims, sizes) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    /// Creates n-dim shape from the @p sizes vector
    pub fn new_5(sizes: &types::VectorOfint) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_BlobShape_VectorOfint(sizes.as_raw_VectorOfint()) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    /// Creates n-dim shape and fill its by @p fill
    ///
    /// ## C++ default parameters
    /// * fill: 1
    pub fn all(ndims: i32, fill: i32) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_all_int_int(ndims, fill) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    /// Returns number of dimensions.
    pub fn dims(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_BlobShape_dims_const(self.as_raw_BlobShape()) }.into_result()
    }
    
    /// Returns reference to the size of the specified @p axis.
    ///
    /// Negative @p axis is supported, in this case a counting starts from the last axis,
    /// i. e. -1 corresponds to last axis.
    /// If non-existing axis was passed then an error will be generated.
    pub fn size(&mut self, axis: i32) -> Result<i32> {
        unsafe { sys::cv_dnn_BlobShape_size_int(self.as_raw_BlobShape(), axis) }.into_result()
    }
    
    /// Returns the size of the specified @p axis.
    ///  @see size()
    pub fn size_1(&self, axis: i32) -> Result<i32> {
        unsafe { sys::cv_dnn_BlobShape_size_const_int(self.as_raw_BlobShape(), axis) }.into_result()
    }
    
    /// Returns the size of the specified @p axis.
    ///
    /// Does the same thing as size(int) const, but if non-existing axis will be passed then 1 will be returned,
    /// therefore this function always finishes successfully.
    pub fn xsize(&self, axis: i32) -> Result<i32> {
        unsafe { sys::cv_dnn_BlobShape_xsize_const_int(self.as_raw_BlobShape(), axis) }.into_result()
    }
    
    /// Converts @p axis index to canonical format (where 0 <= @p axis < dims()).
    pub fn canonical_axis(&self, axis: i32) -> Result<i32> {
        unsafe { sys::cv_dnn_BlobShape_canonicalAxis_const_int(self.as_raw_BlobShape(), axis) }.into_result()
    }
    
    /// Returns the product of all sizes of axes.
    pub fn total(&self) -> Result<ptrdiff_t> {
        unsafe { sys::cv_dnn_BlobShape_total_const(self.as_raw_BlobShape()) }.into_result()
    }
    
    /// Computes the product of sizes of axes among the specified axes range [@p startAxis; @p endAxis).
    /// @details Negative axis indexing can be used. ## See also
    /// Blob::total(int,int)
    ///
    /// ## C++ default parameters
    /// * end_axis: INT_MAX
    pub fn total_1(&self, start_axis: i32, end_axis: i32) -> Result<ptrdiff_t> {
        unsafe { sys::cv_dnn_BlobShape_total_const_int_int(self.as_raw_BlobShape(), start_axis, end_axis) }.into_result()
    }
    
    /// Constructs new shape from axes in range [@p startAxis; @p endAxis).
    /// @details Negative axis indexing can be used. ## See also
    /// Blob::total(int,int)
    ///
    /// ## C++ default parameters
    /// * end_axis: INT_MAX
    pub fn slice(&self, start_axis: i32, end_axis: i32) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_slice_const_int_int(self.as_raw_BlobShape(), start_axis, end_axis) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    /// Returns pointer to the first element of continuous size array.
    pub fn ptr(&self) -> Result<&i32> {
        unsafe { sys::cv_dnn_BlobShape_ptr_const(self.as_raw_BlobShape()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
    }
    
    pub fn ptr_1(&mut self) -> Result<&mut i32> {
        unsafe { sys::cv_dnn_BlobShape_ptr(self.as_raw_BlobShape()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
    }
    
    /// Checks equality of two shapes.
    pub fn equal(&self, other: &crate::dnn::BlobShape) -> Result<bool> {
        unsafe { sys::cv_dnn_BlobShape_equal_const_BlobShape(self.as_raw_BlobShape(), other.as_raw_BlobShape()) }.into_result()
    }
    
    /// Returns shape of passed Mat.
    pub fn like(m: &core::Mat) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_like_Mat(m.as_raw_Mat()) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    /// Returns shape of passed UMat.
    pub fn like_1(m: &core::UMat) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_like_UMat(m.as_raw_UMat()) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    /// Returns empty shape [].
    pub fn empty() -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_BlobShape_empty() }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    /// Returns true if shape is empty (i.e []).
    pub fn is_empty(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_BlobShape_isEmpty_const(self.as_raw_BlobShape()) }.into_result()
    }
    
}

// boxed class cv::dnn::ConcatLayer
pub struct ConcatLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ConcatLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ConcatLayer_delete(self.ptr) };
    }
}

impl ConcatLayer {
    #[inline(always)] pub fn as_raw_ConcatLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ConcatLayer {}

impl crate::dnn::Layer for ConcatLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ConcatLayer {
    ///
    /// ## C++ default parameters
    /// * axis: 1
    pub fn create(axis: i32) -> Result<types::PtrOfConcatLayer> {
        unsafe { sys::cv_dnn_ConcatLayer_create_int(axis) }.into_result().map(|ptr| types::PtrOfConcatLayer { ptr })
    }
    
}

// boxed class cv::dnn::ConvolutionLayer
pub struct ConvolutionLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ConvolutionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ConvolutionLayer_delete(self.ptr) };
    }
}

impl ConvolutionLayer {
    #[inline(always)] pub fn as_raw_ConvolutionLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ConvolutionLayer {}

impl crate::dnn::BaseConvolutionLayerTrait for ConvolutionLayer {
    #[inline(always)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::Layer for ConvolutionLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ConvolutionLayer {
    ///
    /// ## C++ default parameters
    /// * kernel: Size(3, 3)
    /// * stride: Size(1, 1)
    /// * pad: Size(0, 0)
    /// * dilation: Size(1, 1)
    pub fn create(kernel: core::Size, stride: core::Size, pad: core::Size, dilation: core::Size) -> Result<types::PtrOfBaseConvolutionLayer> {
        unsafe { sys::cv_dnn_ConvolutionLayer_create_Size_Size_Size_Size(kernel, stride, pad, dilation) }.into_result().map(|ptr| types::PtrOfBaseConvolutionLayer { ptr })
    }
    
}

// boxed class cv::dnn::CropLayer
pub struct CropLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for CropLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_CropLayer_delete(self.ptr) };
    }
}

impl CropLayer {
    #[inline(always)] pub fn as_raw_CropLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for CropLayer {}

impl crate::dnn::Layer for CropLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl CropLayer {
    pub fn start_axis(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_CropLayer_startAxis_const(self.as_raw_CropLayer()) }.into_result()
    }
    
    pub fn set_start_axis(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_dnn_CropLayer_set_startAxis_int(self.as_raw_CropLayer(), val) }.into_result()
    }
    
    pub fn create(start_axis: i32, offset: &types::VectorOfint) -> Result<types::PtrOfCropLayer> {
        unsafe { sys::cv_dnn_CropLayer_create_int_VectorOfint(start_axis, offset.as_raw_VectorOfint()) }.into_result().map(|ptr| types::PtrOfCropLayer { ptr })
    }
    
}

// boxed class cv::dnn::DeconvolutionLayer
pub struct DeconvolutionLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for DeconvolutionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_DeconvolutionLayer_delete(self.ptr) };
    }
}

impl DeconvolutionLayer {
    #[inline(always)] pub fn as_raw_DeconvolutionLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for DeconvolutionLayer {}

impl crate::dnn::BaseConvolutionLayerTrait for DeconvolutionLayer {
    #[inline(always)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::Layer for DeconvolutionLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl DeconvolutionLayer {
    ///
    /// ## C++ default parameters
    /// * kernel: Size(3, 3)
    /// * stride: Size(1, 1)
    /// * pad: Size(0, 0)
    /// * dilation: Size(1, 1)
    pub fn create(kernel: core::Size, stride: core::Size, pad: core::Size, dilation: core::Size) -> Result<types::PtrOfBaseConvolutionLayer> {
        unsafe { sys::cv_dnn_DeconvolutionLayer_create_Size_Size_Size_Size(kernel, stride, pad, dilation) }.into_result().map(|ptr| types::PtrOfBaseConvolutionLayer { ptr })
    }
    
}

// Generating impl for trait crate::dnn::Dict
/// This class implements name-value dictionary, values are instances of DictValue.
pub trait DictTrait {
    fn as_raw_Dict(&self) -> *mut c_void;
    /// Checks a presence of the @p key in the dictionary.
    fn has(&self, key: &str) -> Result<bool> {
        string_arg!(key);
        unsafe { sys::cv_dnn_Dict_has_const_String(self.as_raw_Dict(), key.as_ptr()) }.into_result()
    }
    
    /// If the @p key in the dictionary then returns pointer to its value, else returns NULL.
    unsafe fn ptr_mut(&mut self, key: &str) -> Result<crate::dnn::DictValue> {
        string_arg!(key);
        { sys::cv_dnn_Dict_ptr_String(self.as_raw_Dict(), key.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// If the @p key in the dictionary then returns its value, else an error will be generated.
    fn get(&self, key: &str) -> Result<crate::dnn::DictValue> {
        string_arg!(key);
        unsafe { sys::cv_dnn_Dict_get_const_String(self.as_raw_Dict(), key.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
    fn set(&mut self, key: &str, value: &mut crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
        string_arg!(key);
        unsafe { sys::cv_dnn_Dict_set_String_DictValue(self.as_raw_Dict(), key.as_ptr(), value.as_raw_DictValue()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
}

// boxed class cv::dnn::Dict
/// This class implements name-value dictionary, values are instances of DictValue.
pub struct Dict {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for Dict {
    fn drop(&mut self) {
        unsafe { sys::cv_Dict_delete(self.ptr) };
    }
}

impl Dict {
    #[inline(always)] pub fn as_raw_Dict(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Dict {}

impl crate::dnn::DictTrait for Dict {
    #[inline(always)] fn as_raw_Dict(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::dnn::DictValue
/// This struct stores the scalar value (or array) of one of the following type: double, cv::String or int64.
///  @todo Maybe int64 is useless because double type exactly stores at least 2^52 integers.
pub struct DictValue {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for DictValue {
    fn drop(&mut self) {
        unsafe { sys::cv_DictValue_delete(self.ptr) };
    }
}

impl DictValue {
    #[inline(always)] pub fn as_raw_DictValue(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for DictValue {}

impl DictValue {
    pub fn copy(r: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
        unsafe { sys::cv_dnn_DictValue_DictValue_DictValue(r.as_raw_DictValue()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Constructs integer scalar
    ///
    /// ## C++ default parameters
    /// * i: 0
    pub fn from_i64(i: i64) -> Result<crate::dnn::DictValue> {
        unsafe { sys::cv_dnn_DictValue_DictValue_int64(i) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Constructs integer scalar
    pub fn from_i32(i: i32) -> Result<crate::dnn::DictValue> {
        unsafe { sys::cv_dnn_DictValue_DictValue_int(i) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Constructs integer scalar
    pub fn from_u32(p: u32) -> Result<crate::dnn::DictValue> {
        unsafe { sys::cv_dnn_DictValue_DictValue_unsigned(p) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Constructs floating point scalar
    pub fn from_f64(p: f64) -> Result<crate::dnn::DictValue> {
        unsafe { sys::cv_dnn_DictValue_DictValue_double(p) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    pub fn from_str(s: &str) -> Result<crate::dnn::DictValue> {
        string_arg!(s);
        unsafe { sys::cv_dnn_DictValue_DictValue_const_char_X(s.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    pub fn size(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_DictValue_size_const(self.as_raw_DictValue()) }.into_result()
    }
    
    pub fn is_int(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_DictValue_isInt_const(self.as_raw_DictValue()) }.into_result()
    }
    
    pub fn is_string(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_DictValue_isString_const(self.as_raw_DictValue()) }.into_result()
    }
    
    pub fn is_real(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_DictValue_isReal_const(self.as_raw_DictValue()) }.into_result()
    }
    
}

// boxed class cv::dnn::EltwiseLayer
pub struct EltwiseLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for EltwiseLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_EltwiseLayer_delete(self.ptr) };
    }
}

impl EltwiseLayer {
    #[inline(always)] pub fn as_raw_EltwiseLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for EltwiseLayer {}

impl crate::dnn::Layer for EltwiseLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

// Generating impl for trait crate::dnn::Importer
/// Small interface class for loading trained serialized models of different dnn-frameworks.
pub trait Importer {
    fn as_raw_Importer(&self) -> *mut c_void;
    /// Adds loaded layers into the @p net and sets connections between them.
    fn populate_net(&mut self, net: &crate::dnn::Net) -> Result<()> {
        unsafe { sys::cv_dnn_Importer_populateNet_Net(self.as_raw_Importer(), net.as_raw_Net()) }.into_result()
    }
    
}

// boxed class cv::dnn::InnerProductLayer
pub struct InnerProductLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for InnerProductLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_InnerProductLayer_delete(self.ptr) };
    }
}

impl InnerProductLayer {
    #[inline(always)] pub fn as_raw_InnerProductLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for InnerProductLayer {}

impl crate::dnn::Layer for InnerProductLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl InnerProductLayer {
    pub fn axis(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_InnerProductLayer_axis_const(self.as_raw_InnerProductLayer()) }.into_result()
    }
    
    pub fn set_axis(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_dnn_InnerProductLayer_set_axis_int(self.as_raw_InnerProductLayer(), val) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * axis: 1
    pub fn create(axis: i32) -> Result<types::PtrOfInnerProductLayer> {
        unsafe { sys::cv_dnn_InnerProductLayer_create_int(axis) }.into_result().map(|ptr| types::PtrOfInnerProductLayer { ptr })
    }
    
}

// boxed class cv::dnn::LRNLayer
pub struct LRNLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for LRNLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_LRNLayer_delete(self.ptr) };
    }
}

impl LRNLayer {
    #[inline(always)] pub fn as_raw_LRNLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LRNLayer {}

impl crate::dnn::Layer for LRNLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl LRNLayer {
    pub fn _type(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_LRNLayer_type_const(self.as_raw_LRNLayer()) }.into_result()
    }
    
    pub fn set_type(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_dnn_LRNLayer_set_type_int(self.as_raw_LRNLayer(), val) }.into_result()
    }
    
    pub fn size(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_LRNLayer_size_const(self.as_raw_LRNLayer()) }.into_result()
    }
    
    pub fn set_size(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_dnn_LRNLayer_set_size_int(self.as_raw_LRNLayer(), val) }.into_result()
    }
    
    pub fn alpha(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_LRNLayer_alpha_const(self.as_raw_LRNLayer()) }.into_result()
    }
    
    pub fn set_alpha(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_dnn_LRNLayer_set_alpha_double(self.as_raw_LRNLayer(), val) }.into_result()
    }
    
    pub fn beta(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_LRNLayer_beta_const(self.as_raw_LRNLayer()) }.into_result()
    }
    
    pub fn set_beta(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_dnn_LRNLayer_set_beta_double(self.as_raw_LRNLayer(), val) }.into_result()
    }
    
    pub fn bias(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_LRNLayer_bias_const(self.as_raw_LRNLayer()) }.into_result()
    }
    
    pub fn set_bias(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_dnn_LRNLayer_set_bias_double(self.as_raw_LRNLayer(), val) }.into_result()
    }
    
    pub fn norm_by_size(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_LRNLayer_normBySize_const(self.as_raw_LRNLayer()) }.into_result()
    }
    
    pub fn set_norm_by_size(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_dnn_LRNLayer_set_normBySize_bool(self.as_raw_LRNLayer(), val) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * _type: LRNLayer::CHANNEL_NRM
    /// * size: 5
    /// * alpha: 1
    /// * beta: 0.75
    /// * bias: 1
    /// * norm_by_size: true
    pub fn create(_type: i32, size: i32, alpha: f64, beta: f64, bias: f64, norm_by_size: bool) -> Result<types::PtrOfLRNLayer> {
        unsafe { sys::cv_dnn_LRNLayer_create_int_int_double_double_double_bool(_type, size, alpha, beta, bias, norm_by_size) }.into_result().map(|ptr| types::PtrOfLRNLayer { ptr })
    }
    
}

// Generating impl for trait crate::dnn::LSTMLayer
/// # Partial List of Implemented Layers
/// This subsection of dnn module contains information about bult-in layers and their descriptions.
///
/// Classes listed here, in fact, provides C++ API for creating intances of bult-in layers.
/// In addition to this way of layers instantiation, there is a more common factory API (see @ref dnnLayerFactory), it allows to create layers dynamically (by name) and register new ones.
/// You can use both API, but factory API is less convinient for native C++ programming and basically designed for use inside importers (see @ref Importer, @ref createCaffeImporter(), @ref createTorchImporter()).
///
/// Bult-in layers partially reproduce functionality of corresponding Caffe and Torch7 layers.
/// In partuclar, the following layers and Caffe @ref Importer were tested to reproduce <a href="http://caffe.berkeleyvision.org/tutorial/layers.html">Caffe</a> functionality:
/// - Convolution
/// - Deconvolution
/// - Pooling
/// - InnerProduct
/// - TanH, ReLU, Sigmoid, BNLL, Power, AbsVal
/// - Softmax
/// - Reshape, Flatten, Slice, Split
/// - LRN
/// - MVN
/// - Dropout (since it does nothing on forward pass -))
///
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
        unsafe { sys::cv_dnn_LSTMLayer_setWeights_Blob_Blob_Blob(self.as_raw_LSTMLayer(), wh.as_raw_Blob(), wx.as_raw_Blob(), b.as_raw_Blob()) }.into_result()
    }
    
    /// Specifies shape of output blob which will be [[`T`], `N`] + @p outTailShape.
    /// @details If this parameter is empty or unset then @p outTailShape = [`Wh`.size(0)] will be used,
    /// where `Wh` is parameter from setWeights().
    ///
    /// ## C++ default parameters
    /// * out_tail_shape: BlobShape::empty()
    fn set_out_shape(&mut self, out_tail_shape: &crate::dnn::BlobShape) -> Result<()> {
        unsafe { sys::cv_dnn_LSTMLayer_setOutShape_BlobShape(self.as_raw_LSTMLayer(), out_tail_shape.as_raw_BlobShape()) }.into_result()
    }
    
    /// Set @f$ h_{t-1} @f$ value that will be used in next forward() calls.
    /// @details By-default @f$ h_{t-1} @f$ is inited by zeros and updated after each forward() call.
    fn set_h(&mut self, h: &crate::dnn::Blob) -> Result<()> {
        unsafe { sys::cv_dnn_LSTMLayer_setH_Blob(self.as_raw_LSTMLayer(), h.as_raw_Blob()) }.into_result()
    }
    
    /// Returns current @f$ h_{t-1} @f$ value (deep copy).
    fn get_h(&self) -> Result<crate::dnn::Blob> {
        unsafe { sys::cv_dnn_LSTMLayer_getH_const(self.as_raw_LSTMLayer()) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
    }
    
    /// Set @f$ c_{t-1} @f$ value that will be used in next forward() calls.
    /// @details By-default @f$ c_{t-1} @f$ is inited by zeros and updated after each forward() call.
    fn set_c(&mut self, c: &crate::dnn::Blob) -> Result<()> {
        unsafe { sys::cv_dnn_LSTMLayer_setC_Blob(self.as_raw_LSTMLayer(), c.as_raw_Blob()) }.into_result()
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
    /// * _use: true
    fn set_use_timstamps_dim(&mut self, _use: bool) -> Result<()> {
        unsafe { sys::cv_dnn_LSTMLayer_setUseTimstampsDim_bool(self.as_raw_LSTMLayer(), _use) }.into_result()
    }
    
    /// If this flag is set to true then layer will produce @f$ c_t @f$ as second output.
    /// @details Shape of the second output is the same as first output.
    ///
    /// ## C++ default parameters
    /// * produce: false
    fn set_produce_cell_output(&mut self, produce: bool) -> Result<()> {
        unsafe { sys::cv_dnn_LSTMLayer_setProduceCellOutput_bool(self.as_raw_LSTMLayer(), produce) }.into_result()
    }
    
    fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
        string_arg!(mut input_name);
        unsafe { sys::cv_dnn_LSTMLayer_inputNameToIndex_String(self.as_raw_LSTMLayer(), input_name.as_ptr() as _) }.into_result()
    }
    
    fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
        string_arg!(mut output_name);
        unsafe { sys::cv_dnn_LSTMLayer_outputNameToIndex_String(self.as_raw_LSTMLayer(), output_name.as_ptr() as _) }.into_result()
    }
    
}

impl dyn LSTMLayer + '_ {
    /// Creates instance of LSTM layer
    pub fn create() -> Result<types::PtrOfLSTMLayer> {
        unsafe { sys::cv_dnn_LSTMLayer_create() }.into_result().map(|ptr| types::PtrOfLSTMLayer { ptr })
    }
    
}

// Generating impl for trait crate::dnn::Layer
/// This interface class allows to build new Layers - are building blocks of networks.
///
/// Each class, derived from Layer, must implement allocate() methods to declare own outputs and forward() to compute outputs.
/// Also before using the new layer into networks you must register your layer by using one of @ref dnnLayerFactory "LayerFactory" macros.
pub trait Layer {
    fn as_raw_Layer(&self) -> *mut c_void;
    /// Name of the layer instance, can be used for logging or other internal purposes.
    fn name(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_Layer_name(self.as_raw_Layer()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Name of the layer instance, can be used for logging or other internal purposes.
    fn set_name(&mut self, val: &str) -> Result<()> {
        string_arg!(mut val);
        unsafe { sys::cv_dnn_Layer_set_name_String(self.as_raw_Layer(), val.as_ptr() as _) }.into_result()
    }
    
    /// Type name which was used for creating layer by layer factory.
    fn _type(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_Layer_type(self.as_raw_Layer()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Type name which was used for creating layer by layer factory.
    fn set_type(&mut self, val: &str) -> Result<()> {
        string_arg!(mut val);
        unsafe { sys::cv_dnn_Layer_set_type_String(self.as_raw_Layer(), val.as_ptr() as _) }.into_result()
    }
    
    
    fn allocate(&mut self, inputs: &types::VectorOfBlob, outputs: &mut types::VectorOfBlob) -> Result<()> {
        unsafe { sys::cv_dnn_Layer_allocate_VectorOfBlob_VectorOfBlob(self.as_raw_Layer(), inputs.as_raw_VectorOfBlob(), outputs.as_raw_VectorOfBlob()) }.into_result()
    }
    
    
    fn allocate_1(&mut self, inputs: &types::VectorOfBlob) -> Result<types::VectorOfBlob> {
        unsafe { sys::cv_dnn_Layer_allocate_VectorOfBlob(self.as_raw_Layer(), inputs.as_raw_VectorOfBlob()) }.into_result().map(|ptr| types::VectorOfBlob { ptr })
    }
    
    
    fn forward(&mut self, inputs: &types::VectorOfBlob, outputs: &mut types::VectorOfBlob) -> Result<()> {
        unsafe { sys::cv_dnn_Layer_forward_VectorOfBlob_VectorOfBlob(self.as_raw_Layer(), inputs.as_raw_VectorOfBlob(), outputs.as_raw_VectorOfBlob()) }.into_result()
    }
    
    /// Allocates layer and computes output.
    fn run(&mut self, inputs: &types::VectorOfBlob, outputs: &mut types::VectorOfBlob) -> Result<()> {
        unsafe { sys::cv_dnn_Layer_run_VectorOfBlob_VectorOfBlob(self.as_raw_Layer(), inputs.as_raw_VectorOfBlob(), outputs.as_raw_VectorOfBlob()) }.into_result()
    }
    
    /// Returns index of input blob into the input array.
    /// ## Parameters
    /// * inputName: label of input blob
    ///
    /// Each layer input and output can be labeled to easily identify them using "%<layer_name%>[.output_name]" notation.
    /// This method maps label of input blob to its index into input vector.
    fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
        string_arg!(mut input_name);
        unsafe { sys::cv_dnn_Layer_inputNameToIndex_String(self.as_raw_Layer(), input_name.as_ptr() as _) }.into_result()
    }
    
    /// Returns index of output blob in output array.
    ///  @see inputNameToIndex()
    fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
        string_arg!(mut output_name);
        unsafe { sys::cv_dnn_Layer_outputNameToIndex_String(self.as_raw_Layer(), output_name.as_ptr() as _) }.into_result()
    }
    
    /// Initializes only #name, #type and #blobs fields.
    fn set_params_from(&mut self, params: &crate::dnn::LayerParams) -> Result<()> {
        unsafe { sys::cv_dnn_Layer_setParamsFrom_LayerParams(self.as_raw_Layer(), params.as_raw_LayerParams()) }.into_result()
    }
    
}

// boxed class cv::dnn::LayerFactory
/// %Layer factory allows to create instances of registered layers.
pub struct LayerFactory {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for LayerFactory {
    fn drop(&mut self) {
        unsafe { sys::cv_LayerFactory_delete(self.ptr) };
    }
}

impl LayerFactory {
    #[inline(always)] pub fn as_raw_LayerFactory(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LayerFactory {}

impl LayerFactory {
    /// Unregisters registered layer with specified type name.
    pub fn unregister_layer(_type: &str) -> Result<()> {
        string_arg!(_type);
        unsafe { sys::cv_dnn_LayerFactory_unregisterLayer_String(_type.as_ptr()) }.into_result()
    }
    
    /// Creates instance of registered layer.
    /// ## Parameters
    /// * type: type name of creating layer.
    /// * params: parameters which will be used for layer initialization.
    pub fn create_layer_instance(_type: &str, params: &mut crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
        string_arg!(_type);
        unsafe { sys::cv_dnn_LayerFactory_createLayerInstance_String_LayerParams(_type.as_ptr(), params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
}

// boxed class cv::dnn::LayerParams
/// This class provides all data needed to initialize layer.
///
/// It includes dictionary with scalar params (which can be readed by using Dict interface),
/// blob params #blobs and optional meta information: #name and #type of layer instance.
pub struct LayerParams {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for LayerParams {
    fn drop(&mut self) {
        unsafe { sys::cv_LayerParams_delete(self.ptr) };
    }
}

impl LayerParams {
    #[inline(always)] pub fn as_raw_LayerParams(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LayerParams {}

impl crate::dnn::DictTrait for LayerParams {
    #[inline(always)] fn as_raw_Dict(&self) -> *mut c_void { self.ptr }
}

impl LayerParams {
    /// Name of the layer instance (optional, can be used internal purposes).
    pub fn name(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_LayerParams_name(self.as_raw_LayerParams()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Name of the layer instance (optional, can be used internal purposes).
    pub fn set_name(&mut self, val: &str) -> Result<()> {
        string_arg!(mut val);
        unsafe { sys::cv_dnn_LayerParams_set_name_String(self.as_raw_LayerParams(), val.as_ptr() as _) }.into_result()
    }
    
    /// Type name which was used for creating layer by layer factory (optional).
    pub fn _type(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_LayerParams_type(self.as_raw_LayerParams()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Type name which was used for creating layer by layer factory (optional).
    pub fn set_type(&mut self, val: &str) -> Result<()> {
        string_arg!(mut val);
        unsafe { sys::cv_dnn_LayerParams_set_type_String(self.as_raw_LayerParams(), val.as_ptr() as _) }.into_result()
    }
    
    pub fn default() -> Result<crate::dnn::LayerParams> {
        unsafe { sys::cv_dnn_LayerParams_LayerParams() }.into_result().map(|ptr| crate::dnn::LayerParams { ptr })
    }
    
}

// boxed class cv::dnn::MVNLayer
pub struct MVNLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for MVNLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_MVNLayer_delete(self.ptr) };
    }
}

impl MVNLayer {
    #[inline(always)] pub fn as_raw_MVNLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MVNLayer {}

impl crate::dnn::Layer for MVNLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl MVNLayer {
    pub fn eps(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_MVNLayer_eps_const(self.as_raw_MVNLayer()) }.into_result()
    }
    
    pub fn set_eps(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_dnn_MVNLayer_set_eps_double(self.as_raw_MVNLayer(), val) }.into_result()
    }
    
    pub fn norm_variance(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_MVNLayer_normVariance_const(self.as_raw_MVNLayer()) }.into_result()
    }
    
    pub fn set_norm_variance(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_dnn_MVNLayer_set_normVariance_bool(self.as_raw_MVNLayer(), val) }.into_result()
    }
    
    pub fn across_channels(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_MVNLayer_acrossChannels_const(self.as_raw_MVNLayer()) }.into_result()
    }
    
    pub fn set_across_channels(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_dnn_MVNLayer_set_acrossChannels_bool(self.as_raw_MVNLayer(), val) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * norm_variance: true
    /// * across_channels: false
    /// * eps: 1e-9
    pub fn create(norm_variance: bool, across_channels: bool, eps: f64) -> Result<types::PtrOfMVNLayer> {
        unsafe { sys::cv_dnn_MVNLayer_create_bool_bool_double(norm_variance, across_channels, eps) }.into_result().map(|ptr| types::PtrOfMVNLayer { ptr })
    }
    
}

// boxed class cv::dnn::Net
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
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for Net {
    fn drop(&mut self) {
        unsafe { sys::cv_Net_delete(self.ptr) };
    }
}

impl Net {
    #[inline(always)] pub fn as_raw_Net(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Net {}

impl Net {
    /// Default constructor.
    pub fn default() -> Result<crate::dnn::Net> {
        unsafe { sys::cv_dnn_Net_Net() }.into_result().map(|ptr| crate::dnn::Net { ptr })
    }
    
    /// Returns true if there are no layers in the network.
    pub fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_Net_empty_const(self.as_raw_Net()) }.into_result()
    }
    
    /// Adds new layer to the net.
    /// ## Parameters
    /// * name: unique name of the adding layer.
    /// * type: typename of the adding layer (type must be registered in LayerRegister).
    /// * params: parameters which will be used to initialize the creating layer.
    /// ## Returns
    /// unique identifier of created layer, or -1 if a failure will happen.
    pub fn add_layer(&mut self, name: &str, _type: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
        string_arg!(name);
        string_arg!(_type);
        unsafe { sys::cv_dnn_Net_addLayer_String_String_LayerParams(self.as_raw_Net(), name.as_ptr(), _type.as_ptr(), params.as_raw_LayerParams()) }.into_result()
    }
    
    /// Adds new layer and connects its first input to the first output of previously added layer.
    ///  @see addLayer()
    pub fn add_layer_to_prev(&mut self, name: &str, _type: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
        string_arg!(name);
        string_arg!(_type);
        unsafe { sys::cv_dnn_Net_addLayerToPrev_String_String_LayerParams(self.as_raw_Net(), name.as_ptr(), _type.as_ptr(), params.as_raw_LayerParams()) }.into_result()
    }
    
    /// Converts string name of the layer to the integer identifier.
    /// ## Returns
    /// id of the layer, or -1 if the layer wasn't found.
    pub fn get_layer_id(&mut self, layer: &str) -> Result<i32> {
        string_arg!(layer);
        unsafe { sys::cv_dnn_Net_getLayerId_String(self.as_raw_Net(), layer.as_ptr()) }.into_result()
    }
    
    pub fn get_layer_names(&self) -> Result<types::VectorOfString> {
        unsafe { sys::cv_dnn_Net_getLayerNames_const(self.as_raw_Net()) }.into_result().map(|ptr| types::VectorOfString { ptr })
    }
    
    /// Returns pointer to layer with specified name which the network use.
    pub fn get_layer(&mut self, layer_id: &crate::dnn::DictValue) -> Result<types::PtrOfLayer> {
        unsafe { sys::cv_dnn_Net_getLayer_DictValue(self.as_raw_Net(), layer_id.as_raw_DictValue()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
    /// Delete layer for the network (not implemented yet)
    pub fn delete_layer(&mut self, layer: &crate::dnn::DictValue) -> Result<()> {
        unsafe { sys::cv_dnn_Net_deleteLayer_DictValue(self.as_raw_Net(), layer.as_raw_DictValue()) }.into_result()
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
    ///
    ///  @see setNetInputs(), Layer::inputNameToIndex(), Layer::outputNameToIndex()
    pub fn connect_first_second(&mut self, out_pin: &str, inp_pin: &str) -> Result<()> {
        string_arg!(mut out_pin);
        string_arg!(mut inp_pin);
        unsafe { sys::cv_dnn_Net_connect_String_String(self.as_raw_Net(), out_pin.as_ptr() as _, inp_pin.as_ptr() as _) }.into_result()
    }
    
    /// Connects #@p outNum output of the first layer to #@p inNum input of the second layer.
    /// ## Parameters
    /// * outLayerId: identifier of the first layer
    /// * inpLayerId: identifier of the second layer
    /// * outNum: number of the first layer output
    /// * inpNum: number of the second layer input
    pub fn connect(&mut self, out_layer_id: i32, out_num: i32, inp_layer_id: i32, inp_num: i32) -> Result<()> {
        unsafe { sys::cv_dnn_Net_connect_int_int_int_int(self.as_raw_Net(), out_layer_id, out_num, inp_layer_id, inp_num) }.into_result()
    }
    
    /// Sets outputs names of the network input pseudo layer.
    ///
    /// Each net always has special own the network input pseudo layer with id=0.
    /// This layer stores the user blobs only and don't make any computations.
    /// In fact, this layer provides the only way to pass user data into the network.
    /// As any other layer, this layer can label its outputs and this function provides an easy way to do this.
    pub fn set_net_inputs(&mut self, input_blob_names: &types::VectorOfString) -> Result<()> {
        unsafe { sys::cv_dnn_Net_setNetInputs_VectorOfString(self.as_raw_Net(), input_blob_names.as_raw_VectorOfString()) }.into_result()
    }
    
    /// Initializes and allocates all layers.
    pub fn allocate(&mut self) -> Result<()> {
        unsafe { sys::cv_dnn_Net_allocate(self.as_raw_Net()) }.into_result()
    }
    
    /// Runs forward pass to compute output of layer @p toLayer.
    /// @details By default runs forward pass for the whole network.
    ///
    /// ## C++ default parameters
    /// * to_layer: String()
    pub fn forward(&mut self, to_layer: &crate::dnn::DictValue) -> Result<()> {
        unsafe { sys::cv_dnn_Net_forward_DictValue(self.as_raw_Net(), to_layer.as_raw_DictValue()) }.into_result()
    }
    
    /// Runs forward pass to compute output of layer @p toLayer, but computations start from @p startLayer
    pub fn forward_1(&mut self, start_layer: &crate::dnn::DictValue, to_layer: &crate::dnn::DictValue) -> Result<()> {
        unsafe { sys::cv_dnn_Net_forward_DictValue_DictValue(self.as_raw_Net(), start_layer.as_raw_DictValue(), to_layer.as_raw_DictValue()) }.into_result()
    }
    
    pub fn forward_2(&mut self, start_layers: &types::VectorOfDictValue, to_layers: &types::VectorOfDictValue) -> Result<()> {
        unsafe { sys::cv_dnn_Net_forward_VectorOfDictValue_VectorOfDictValue(self.as_raw_Net(), start_layers.as_raw_VectorOfDictValue(), to_layers.as_raw_VectorOfDictValue()) }.into_result()
    }
    
    /// Optimized forward.
    ///  @warning Not implemented yet.
    ///  @details Makes forward only those layers which weren't changed after previous forward().
    pub fn forward_opt(&mut self, to_layer: &crate::dnn::DictValue) -> Result<()> {
        unsafe { sys::cv_dnn_Net_forwardOpt_DictValue(self.as_raw_Net(), to_layer.as_raw_DictValue()) }.into_result()
    }
    
    pub fn forward_opt_1(&mut self, to_layers: &types::VectorOfDictValue) -> Result<()> {
        unsafe { sys::cv_dnn_Net_forwardOpt_VectorOfDictValue(self.as_raw_Net(), to_layers.as_raw_VectorOfDictValue()) }.into_result()
    }
    
    /// Sets the new value for the layer output blob
    /// ## Parameters
    /// * outputName: descriptor of the updating layer output blob.
    /// * blob: new blob.
    ///  @see connect(String, String) to know format of the descriptor.
    ///
    /// Note: If updating blob is not empty then @p blob must have the same shape,
    ///  because network reshaping is not implemented yet.
    pub fn set_blob(&mut self, output_name: &str, blob: &crate::dnn::Blob) -> Result<()> {
        string_arg!(mut output_name);
        unsafe { sys::cv_dnn_Net_setBlob_String_Blob(self.as_raw_Net(), output_name.as_ptr() as _, blob.as_raw_Blob()) }.into_result()
    }
    
    /// Returns the layer output blob.
    /// ## Parameters
    /// * outputName: the descriptor of the returning layer output blob.
    ///  @see connect(String, String)
    pub fn get_blob(&mut self, output_name: &str) -> Result<crate::dnn::Blob> {
        string_arg!(mut output_name);
        unsafe { sys::cv_dnn_Net_getBlob_String(self.as_raw_Net(), output_name.as_ptr() as _) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
    }
    
    /// Sets the new value for the learned param of the layer.
    /// ## Parameters
    /// * layer: name or id of the layer.
    /// * numParam: index of the layer parameter in the Layer::blobs array.
    /// * blob: the new value.
    ///  @see Layer::blobs
    ///
    /// Note: If shape of the new blob differs from the previous shape,
    ///  then the following forward pass may fail.
    pub fn set_param(&mut self, layer: &crate::dnn::DictValue, num_param: i32, blob: &crate::dnn::Blob) -> Result<()> {
        unsafe { sys::cv_dnn_Net_setParam_DictValue_int_Blob(self.as_raw_Net(), layer.as_raw_DictValue(), num_param, blob.as_raw_Blob()) }.into_result()
    }
    
    /// Returns parameter blob of the layer.
    /// ## Parameters
    /// * layer: name or id of the layer.
    /// * numParam: index of the layer parameter in the Layer::blobs array.
    ///  @see Layer::blobs
    ///
    /// ## C++ default parameters
    /// * num_param: 0
    pub fn get_param(&mut self, layer: &crate::dnn::DictValue, num_param: i32) -> Result<crate::dnn::Blob> {
        unsafe { sys::cv_dnn_Net_getParam_DictValue_int(self.as_raw_Net(), layer.as_raw_DictValue(), num_param) }.into_result().map(|ptr| crate::dnn::Blob { ptr })
    }
    
}

// boxed class cv::dnn::PoolingLayer
pub struct PoolingLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for PoolingLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_PoolingLayer_delete(self.ptr) };
    }
}

impl PoolingLayer {
    #[inline(always)] pub fn as_raw_PoolingLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PoolingLayer {}

impl crate::dnn::Layer for PoolingLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl PoolingLayer {
    pub fn _type(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_PoolingLayer_type_const(self.as_raw_PoolingLayer()) }.into_result()
    }
    
    pub fn set_type(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_dnn_PoolingLayer_set_type_int(self.as_raw_PoolingLayer(), val) }.into_result()
    }
    
    pub fn kernel(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_PoolingLayer_kernel_const(self.as_raw_PoolingLayer()) }.into_result()
    }
    
    pub fn set_kernel(&mut self, val: core::Size) -> Result<()> {
        unsafe { sys::cv_dnn_PoolingLayer_set_kernel_Size(self.as_raw_PoolingLayer(), val) }.into_result()
    }
    
    pub fn stride(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_PoolingLayer_stride_const(self.as_raw_PoolingLayer()) }.into_result()
    }
    
    pub fn set_stride(&mut self, val: core::Size) -> Result<()> {
        unsafe { sys::cv_dnn_PoolingLayer_set_stride_Size(self.as_raw_PoolingLayer(), val) }.into_result()
    }
    
    pub fn pad(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_PoolingLayer_pad_const(self.as_raw_PoolingLayer()) }.into_result()
    }
    
    pub fn set_pad(&mut self, val: core::Size) -> Result<()> {
        unsafe { sys::cv_dnn_PoolingLayer_set_pad_Size(self.as_raw_PoolingLayer(), val) }.into_result()
    }
    
    pub fn global_pooling(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_PoolingLayer_globalPooling_const(self.as_raw_PoolingLayer()) }.into_result()
    }
    
    pub fn set_global_pooling(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_dnn_PoolingLayer_set_globalPooling_bool(self.as_raw_PoolingLayer(), val) }.into_result()
    }
    
    pub fn pad_mode(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_PoolingLayer_padMode(self.as_raw_PoolingLayer()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    pub fn set_pad_mode(&mut self, val: &str) -> Result<()> {
        string_arg!(mut val);
        unsafe { sys::cv_dnn_PoolingLayer_set_padMode_String(self.as_raw_PoolingLayer(), val.as_ptr() as _) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * _type: PoolingLayer::MAX
    /// * kernel: Size(2, 2)
    /// * stride: Size(1, 1)
    /// * pad: Size(0, 0)
    /// * pad_mode: ""
    pub fn create(_type: i32, kernel: core::Size, stride: core::Size, pad: core::Size, pad_mode: &str) -> Result<types::PtrOfPoolingLayer> {
        string_arg!(pad_mode);
        unsafe { sys::cv_dnn_PoolingLayer_create_int_Size_Size_Size_String(_type, kernel, stride, pad, pad_mode.as_ptr()) }.into_result().map(|ptr| types::PtrOfPoolingLayer { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * _type: PoolingLayer::MAX
    pub fn create_global(_type: i32) -> Result<types::PtrOfPoolingLayer> {
        unsafe { sys::cv_dnn_PoolingLayer_createGlobal_int(_type) }.into_result().map(|ptr| types::PtrOfPoolingLayer { ptr })
    }
    
}

// boxed class cv::dnn::PowerLayer
pub struct PowerLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for PowerLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_PowerLayer_delete(self.ptr) };
    }
}

impl PowerLayer {
    #[inline(always)] pub fn as_raw_PowerLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PowerLayer {}

impl crate::dnn::Layer for PowerLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl PowerLayer {
    pub fn power(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_PowerLayer_power_const(self.as_raw_PowerLayer()) }.into_result()
    }
    
    pub fn set_power(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_dnn_PowerLayer_set_power_double(self.as_raw_PowerLayer(), val) }.into_result()
    }
    
    pub fn scale(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_PowerLayer_scale_const(self.as_raw_PowerLayer()) }.into_result()
    }
    
    pub fn set_scale(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_dnn_PowerLayer_set_scale_double(self.as_raw_PowerLayer(), val) }.into_result()
    }
    
    pub fn shift(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_PowerLayer_shift_const(self.as_raw_PowerLayer()) }.into_result()
    }
    
    pub fn set_shift(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_dnn_PowerLayer_set_shift_double(self.as_raw_PowerLayer(), val) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * power: 1
    /// * scale: 1
    /// * shift: 0
    pub fn create(power: f64, scale: f64, shift: f64) -> Result<types::PtrOfPowerLayer> {
        unsafe { sys::cv_dnn_PowerLayer_create_double_double_double(power, scale, shift) }.into_result().map(|ptr| types::PtrOfPowerLayer { ptr })
    }
    
}

// Generating impl for trait crate::dnn::RNNLayer
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
        unsafe { sys::cv_dnn_RNNLayer_setWeights_Blob_Blob_Blob_Blob_Blob(self.as_raw_RNNLayer(), wxh.as_raw_Blob(), bh.as_raw_Blob(), whh.as_raw_Blob(), who.as_raw_Blob(), bo.as_raw_Blob()) }.into_result()
    }
    
    /// If this flag is set to true then layer will produce @f$ h_t @f$ as second output.
    /// @details Shape of the second output is the same as first output.
    ///
    /// ## C++ default parameters
    /// * produce: false
    fn set_produce_hidden_output(&mut self, produce: bool) -> Result<()> {
        unsafe { sys::cv_dnn_RNNLayer_setProduceHiddenOutput_bool(self.as_raw_RNNLayer(), produce) }.into_result()
    }
    
}

impl dyn RNNLayer + '_ {
    /// Creates instance of RNNLayer
    pub fn create() -> Result<types::PtrOfRNNLayer> {
        unsafe { sys::cv_dnn_RNNLayer_create() }.into_result().map(|ptr| types::PtrOfRNNLayer { ptr })
    }
    
}

// boxed class cv::dnn::ReLULayer
pub struct ReLULayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ReLULayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ReLULayer_delete(self.ptr) };
    }
}

impl ReLULayer {
    #[inline(always)] pub fn as_raw_ReLULayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ReLULayer {}

impl crate::dnn::Layer for ReLULayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ReLULayer {
    pub fn negative_slope(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_ReLULayer_negativeSlope_const(self.as_raw_ReLULayer()) }.into_result()
    }
    
    pub fn set_negative_slope(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_dnn_ReLULayer_set_negativeSlope_double(self.as_raw_ReLULayer(), val) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * negative_slope: 0
    pub fn create(negative_slope: f64) -> Result<types::PtrOfReLULayer> {
        unsafe { sys::cv_dnn_ReLULayer_create_double(negative_slope) }.into_result().map(|ptr| types::PtrOfReLULayer { ptr })
    }
    
}

// boxed class cv::dnn::ReshapeLayer
pub struct ReshapeLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ReshapeLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ReshapeLayer_delete(self.ptr) };
    }
}

impl ReshapeLayer {
    #[inline(always)] pub fn as_raw_ReshapeLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ReshapeLayer {}

impl crate::dnn::Layer for ReshapeLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ReshapeLayer {
    pub fn new_shape_desc(&mut self) -> Result<crate::dnn::BlobShape> {
        unsafe { sys::cv_dnn_ReshapeLayer_newShapeDesc(self.as_raw_ReshapeLayer()) }.into_result().map(|ptr| crate::dnn::BlobShape { ptr })
    }
    
    pub fn set_new_shape_desc(&mut self, val: crate::dnn::BlobShape) -> Result<()> {
        unsafe { sys::cv_dnn_ReshapeLayer_set_newShapeDesc_BlobShape(self.as_raw_ReshapeLayer(), val.as_raw_BlobShape()) }.into_result()
    }
    
    pub fn new_shape_range(&mut self) -> Result<core::Range> {
        unsafe { sys::cv_dnn_ReshapeLayer_newShapeRange(self.as_raw_ReshapeLayer()) }.into_result().map(|ptr| core::Range { ptr })
    }
    
    pub fn set_new_shape_range(&mut self, val: core::Range) -> Result<()> {
        unsafe { sys::cv_dnn_ReshapeLayer_set_newShapeRange_Range(self.as_raw_ReshapeLayer(), val.as_raw_Range()) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * applying_range: Range::all()
    /// * enable_reordering: false
    pub fn create(new_shape: &crate::dnn::BlobShape, applying_range: &core::Range, enable_reordering: bool) -> Result<types::PtrOfReshapeLayer> {
        unsafe { sys::cv_dnn_ReshapeLayer_create_BlobShape_Range_bool(new_shape.as_raw_BlobShape(), applying_range.as_raw_Range(), enable_reordering) }.into_result().map(|ptr| types::PtrOfReshapeLayer { ptr })
    }
    
}

// boxed class cv::dnn::SigmoidLayer
pub struct SigmoidLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SigmoidLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SigmoidLayer_delete(self.ptr) };
    }
}

impl SigmoidLayer {
    #[inline(always)] pub fn as_raw_SigmoidLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SigmoidLayer {}

impl crate::dnn::Layer for SigmoidLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl SigmoidLayer {
    pub fn create() -> Result<types::PtrOfSigmoidLayer> {
        unsafe { sys::cv_dnn_SigmoidLayer_create() }.into_result().map(|ptr| types::PtrOfSigmoidLayer { ptr })
    }
    
}

// boxed class cv::dnn::SliceLayer
pub struct SliceLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SliceLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SliceLayer_delete(self.ptr) };
    }
}

impl SliceLayer {
    #[inline(always)] pub fn as_raw_SliceLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SliceLayer {}

impl crate::dnn::Layer for SliceLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl SliceLayer {
    pub fn axis(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_SliceLayer_axis_const(self.as_raw_SliceLayer()) }.into_result()
    }
    
    pub fn set_axis(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_dnn_SliceLayer_set_axis_int(self.as_raw_SliceLayer(), val) }.into_result()
    }
    
    pub fn create(axis: i32) -> Result<types::PtrOfSliceLayer> {
        unsafe { sys::cv_dnn_SliceLayer_create_int(axis) }.into_result().map(|ptr| types::PtrOfSliceLayer { ptr })
    }
    
    pub fn create_1(axis: i32, slice_indices: &types::VectorOfint) -> Result<types::PtrOfSliceLayer> {
        unsafe { sys::cv_dnn_SliceLayer_create_int_VectorOfint(axis, slice_indices.as_raw_VectorOfint()) }.into_result().map(|ptr| types::PtrOfSliceLayer { ptr })
    }
    
}

// boxed class cv::dnn::SoftmaxLayer
pub struct SoftmaxLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SoftmaxLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SoftmaxLayer_delete(self.ptr) };
    }
}

impl SoftmaxLayer {
    #[inline(always)] pub fn as_raw_SoftmaxLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SoftmaxLayer {}

impl crate::dnn::Layer for SoftmaxLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl SoftmaxLayer {
    ///
    /// ## C++ default parameters
    /// * axis: 1
    pub fn create(axis: i32) -> Result<types::PtrOfSoftmaxLayer> {
        unsafe { sys::cv_dnn_SoftmaxLayer_create_int(axis) }.into_result().map(|ptr| types::PtrOfSoftmaxLayer { ptr })
    }
    
}

// boxed class cv::dnn::SplitLayer
pub struct SplitLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SplitLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SplitLayer_delete(self.ptr) };
    }
}

impl SplitLayer {
    #[inline(always)] pub fn as_raw_SplitLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SplitLayer {}

impl crate::dnn::Layer for SplitLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl SplitLayer {
    ///
    /// ## C++ default parameters
    /// * outputs_count: -1
    pub fn create(outputs_count: i32) -> Result<types::PtrOfSplitLayer> {
        unsafe { sys::cv_dnn_SplitLayer_create_int(outputs_count) }.into_result().map(|ptr| types::PtrOfSplitLayer { ptr })
    }
    
}

// boxed class cv::dnn::TanHLayer
pub struct TanHLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for TanHLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_TanHLayer_delete(self.ptr) };
    }
}

impl TanHLayer {
    #[inline(always)] pub fn as_raw_TanHLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for TanHLayer {}

impl crate::dnn::Layer for TanHLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl TanHLayer {
    pub fn create() -> Result<types::PtrOfTanHLayer> {
        unsafe { sys::cv_dnn_TanHLayer_create() }.into_result().map(|ptr| types::PtrOfTanHLayer { ptr })
    }
    
}

// boxed class cv::dnn::_LayerStaticRegisterer
pub struct _LayerStaticRegisterer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for _LayerStaticRegisterer {
    fn drop(&mut self) {
        unsafe { sys::cv__LayerStaticRegisterer_delete(self.ptr) };
    }
}

impl _LayerStaticRegisterer {
    #[inline(always)] pub fn as_raw__LayerStaticRegisterer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for _LayerStaticRegisterer {}

pub const Blob_ALLOC_BOTH: i32 = 0x3; // 3
pub const Blob_SYNCED: i32 = 0x3; // 3
pub use crate::manual::dnn::*;

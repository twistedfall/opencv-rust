//! # Deep Neural Network module
//! This module contains:
//! - API for new layers creation, layers are building bricks of neural networks;
//! - set of built-in most-useful Layers;
//! - API to constuct and modify comprehensive neural networks from layers;
//! - functionality for loading serialized networks models from differnet frameworks.
//!
//! Functionality of this module is designed only for forward pass computations (i. e. network testing).
//! A network training is in principle not supported.
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};

pub const EltwiseLayer_PROD: i32 = 0;
pub const EltwiseLayer_SUM: i32 = 1;
pub const LRNLayer_CHANNEL_NRM: i32 = 0;
pub const LRNLayer_SPATIAL_NRM: i32 = 1;
pub const PoolingLayer_AVE: i32 = 1;
pub const PoolingLayer_MAX: i32 = 0;
pub const PoolingLayer_STOCHASTIC: i32 = 2;

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

// boxed class cv::dnn::AbsLayer
pub struct AbsLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::dnn::AbsLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_AbsLayer_delete(self.ptr) };
    }
}
impl crate::dnn::AbsLayer {
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

impl Drop for crate::dnn::BNLLLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_BNLLLayer_delete(self.ptr) };
    }
}
impl crate::dnn::BNLLLayer {
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

// Generating impl for trait cv::dnn::BaseConvolutionLayer (trait)
pub trait BaseConvolutionLayer: crate::dnn::Layer {
    #[inline(always)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void;
    fn kernel(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_kernel_const(self.as_raw_BaseConvolutionLayer()) }.into_result()
    }
    
    fn stride(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_stride_const(self.as_raw_BaseConvolutionLayer()) }.into_result()
    }
    
    fn pad(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_pad_const(self.as_raw_BaseConvolutionLayer()) }.into_result()
    }
    
    fn dilation(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_dilation_const(self.as_raw_BaseConvolutionLayer()) }.into_result()
    }
    
    fn pad_mode(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_BaseConvolutionLayer_padMode(self.as_raw_BaseConvolutionLayer()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    fn set_pad_mode(&mut self, val: &str) -> Result<()> {
        string_arg!(mut val);
        unsafe { sys::cv_dnn_BaseConvolutionLayer_set_padMode_String(self.as_raw_BaseConvolutionLayer(), val.as_ptr() as _) }.into_result()
    }
    
}

// boxed class cv::dnn::ConcatLayer
pub struct ConcatLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::dnn::ConcatLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ConcatLayer_delete(self.ptr) };
    }
}
impl crate::dnn::ConcatLayer {
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

impl Drop for crate::dnn::ConvolutionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ConvolutionLayer_delete(self.ptr) };
    }
}
impl crate::dnn::ConvolutionLayer {
    #[inline(always)] pub fn as_raw_ConvolutionLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ConvolutionLayer {}

impl crate::dnn::BaseConvolutionLayer for ConvolutionLayer {
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

impl Drop for crate::dnn::CropLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_CropLayer_delete(self.ptr) };
    }
}
impl crate::dnn::CropLayer {
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
    
    pub fn create(start_axis: i32, offset: &types::VectorOfint) -> Result<types::PtrOfCropLayer> {
        unsafe { sys::cv_dnn_CropLayer_create_int_VectorOfint(start_axis, offset.as_raw_VectorOfint()) }.into_result().map(|ptr| types::PtrOfCropLayer { ptr })
    }
    
}

// boxed class cv::dnn::DeconvolutionLayer
pub struct DeconvolutionLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::dnn::DeconvolutionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_DeconvolutionLayer_delete(self.ptr) };
    }
}
impl crate::dnn::DeconvolutionLayer {
    #[inline(always)] pub fn as_raw_DeconvolutionLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for DeconvolutionLayer {}

impl crate::dnn::BaseConvolutionLayer for DeconvolutionLayer {
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

// Generating impl for trait cv::dnn::Dict (trait)
/// This class implements name-value dictionary, values are instances of DictValue.
pub trait Dict {
    #[inline(always)] fn as_raw_Dict(&self) -> *mut c_void;
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

// boxed class cv::dnn::DictValue
/// This struct stores the scalar value (or array) of one of the following type: double, cv::String or int64.
///  @todo Maybe int64 is useless because double type exactly stores at least 2^52 integers.
pub struct DictValue {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::dnn::DictValue {
    fn drop(&mut self) {
        unsafe { sys::cv_DictValue_delete(self.ptr) };
    }
}
impl crate::dnn::DictValue {
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

impl Drop for crate::dnn::EltwiseLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_EltwiseLayer_delete(self.ptr) };
    }
}
impl crate::dnn::EltwiseLayer {
    #[inline(always)] pub fn as_raw_EltwiseLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for EltwiseLayer {}

impl crate::dnn::Layer for EltwiseLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl EltwiseLayer {

}

// Generating impl for trait cv::dnn::Importer (trait)
/// Small interface class for loading trained serialized models of different dnn-frameworks.
pub trait Importer {
    #[inline(always)] fn as_raw_Importer(&self) -> *mut c_void;
    /// Adds loaded layers into the @p net and sets connections between them.
    fn populate_net(&mut self, net: &crate::dnn::Net) -> Result<()> {
        unsafe { sys::cv_dnn_Importer_populateNet_Net(self.as_raw_Importer(), net.as_raw_Net()) }.into_result()
    }
    
}

// boxed class cv::dnn::InnerProductLayer
pub struct InnerProductLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::dnn::InnerProductLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_InnerProductLayer_delete(self.ptr) };
    }
}
impl crate::dnn::InnerProductLayer {
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

impl Drop for crate::dnn::LRNLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_LRNLayer_delete(self.ptr) };
    }
}
impl crate::dnn::LRNLayer {
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
    
    pub fn size(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_LRNLayer_size_const(self.as_raw_LRNLayer()) }.into_result()
    }
    
    pub fn alpha(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_LRNLayer_alpha_const(self.as_raw_LRNLayer()) }.into_result()
    }
    
    pub fn beta(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_LRNLayer_beta_const(self.as_raw_LRNLayer()) }.into_result()
    }
    
    pub fn bias(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_LRNLayer_bias_const(self.as_raw_LRNLayer()) }.into_result()
    }
    
    pub fn norm_by_size(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_LRNLayer_normBySize_const(self.as_raw_LRNLayer()) }.into_result()
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

// Generating impl for trait cv::dnn::LSTMLayer (trait)
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
    #[inline(always)] fn as_raw_LSTMLayer(&self) -> *mut c_void;
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

// Generating impl for trait cv::dnn::Layer (trait)
/// This interface class allows to build new Layers - are building blocks of networks.
///
/// Each class, derived from Layer, must implement allocate() methods to declare own outputs and forward() to compute outputs.
/// Also before using the new layer into networks you must register your layer by using one of @ref dnnLayerFactory "LayerFactory" macros.
pub trait Layer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void;
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

impl Drop for crate::dnn::LayerFactory {
    fn drop(&mut self) {
        unsafe { sys::cv_LayerFactory_delete(self.ptr) };
    }
}
impl crate::dnn::LayerFactory {
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

impl Drop for crate::dnn::LayerParams {
    fn drop(&mut self) {
        unsafe { sys::cv_LayerParams_delete(self.ptr) };
    }
}
impl crate::dnn::LayerParams {
    #[inline(always)] pub fn as_raw_LayerParams(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LayerParams {}

impl crate::dnn::Dict for LayerParams {
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

impl Drop for crate::dnn::MVNLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_MVNLayer_delete(self.ptr) };
    }
}
impl crate::dnn::MVNLayer {
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
    
    pub fn norm_variance(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_MVNLayer_normVariance_const(self.as_raw_MVNLayer()) }.into_result()
    }
    
    pub fn across_channels(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_MVNLayer_acrossChannels_const(self.as_raw_MVNLayer()) }.into_result()
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

impl Drop for crate::dnn::Net {
    fn drop(&mut self) {
        unsafe { sys::cv_Net_delete(self.ptr) };
    }
}
impl crate::dnn::Net {
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
    
}

// boxed class cv::dnn::PoolingLayer
pub struct PoolingLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::dnn::PoolingLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_PoolingLayer_delete(self.ptr) };
    }
}
impl crate::dnn::PoolingLayer {
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
    
    pub fn kernel(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_PoolingLayer_kernel_const(self.as_raw_PoolingLayer()) }.into_result()
    }
    
    pub fn stride(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_PoolingLayer_stride_const(self.as_raw_PoolingLayer()) }.into_result()
    }
    
    pub fn pad(&self) -> Result<core::Size> {
        unsafe { sys::cv_dnn_PoolingLayer_pad_const(self.as_raw_PoolingLayer()) }.into_result()
    }
    
    pub fn global_pooling(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_PoolingLayer_globalPooling_const(self.as_raw_PoolingLayer()) }.into_result()
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

impl Drop for crate::dnn::PowerLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_PowerLayer_delete(self.ptr) };
    }
}
impl crate::dnn::PowerLayer {
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
    
    pub fn scale(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_PowerLayer_scale_const(self.as_raw_PowerLayer()) }.into_result()
    }
    
    pub fn shift(&self) -> Result<f64> {
        unsafe { sys::cv_dnn_PowerLayer_shift_const(self.as_raw_PowerLayer()) }.into_result()
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

// Generating impl for trait cv::dnn::RNNLayer (trait)
/// Classical recurrent layer
pub trait RNNLayer: crate::dnn::Layer {
    #[inline(always)] fn as_raw_RNNLayer(&self) -> *mut c_void;
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

impl Drop for crate::dnn::ReLULayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ReLULayer_delete(self.ptr) };
    }
}
impl crate::dnn::ReLULayer {
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

impl Drop for crate::dnn::ReshapeLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ReshapeLayer_delete(self.ptr) };
    }
}
impl crate::dnn::ReshapeLayer {
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

    pub fn new_shape_range(&mut self) -> Result<core::Range> {
        unsafe { sys::cv_dnn_ReshapeLayer_newShapeRange(self.as_raw_ReshapeLayer()) }.into_result().map(|ptr| core::Range { ptr })
    }
    
    pub fn set_new_shape_range(&mut self, val: core::Range) -> Result<()> {
        unsafe { sys::cv_dnn_ReshapeLayer_set_newShapeRange_Range(self.as_raw_ReshapeLayer(), val.as_raw_Range()) }.into_result()
    }
    
}

// boxed class cv::dnn::SigmoidLayer
pub struct SigmoidLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::dnn::SigmoidLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SigmoidLayer_delete(self.ptr) };
    }
}
impl crate::dnn::SigmoidLayer {
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

impl Drop for crate::dnn::SliceLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SliceLayer_delete(self.ptr) };
    }
}
impl crate::dnn::SliceLayer {
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

impl Drop for crate::dnn::SoftmaxLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SoftmaxLayer_delete(self.ptr) };
    }
}
impl crate::dnn::SoftmaxLayer {
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

impl Drop for crate::dnn::SplitLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SplitLayer_delete(self.ptr) };
    }
}
impl crate::dnn::SplitLayer {
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

impl Drop for crate::dnn::TanHLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_TanHLayer_delete(self.ptr) };
    }
}
impl crate::dnn::TanHLayer {
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

impl Drop for crate::dnn::_LayerStaticRegisterer {
    fn drop(&mut self) {
        unsafe { sys::cv__LayerStaticRegisterer_delete(self.ptr) };
    }
}
impl crate::dnn::_LayerStaticRegisterer {
    #[inline(always)] pub fn as_raw__LayerStaticRegisterer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for _LayerStaticRegisterer {}

impl _LayerStaticRegisterer {

}

pub use crate::manual::dnn::*;

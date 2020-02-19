#include "dnn.hpp"
#include "dnn_types.hpp"

extern "C" {
	Result<void*> cv_dnn_createCaffeImporter_const_StringX_const_StringX(const char* prototxt, const char* caffeModel) {
		try {
			cv::Ptr<cv::dnn::Importer> ret = cv::dnn::createCaffeImporter(cv::String(prototxt), cv::String(caffeModel));
			return Ok<void*>(new cv::Ptr<cv::dnn::Importer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_createTensorflowImporter_const_StringX(const char* model) {
		try {
			cv::Ptr<cv::dnn::Importer> ret = cv::dnn::createTensorflowImporter(cv::String(model));
			return Ok<void*>(new cv::Ptr<cv::dnn::Importer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_createTorchImporter_const_StringX_bool(const char* filename, bool isBinary) {
		try {
			cv::Ptr<cv::dnn::Importer> ret = cv::dnn::createTorchImporter(cv::String(filename), isBinary);
			return Ok<void*>(new cv::Ptr<cv::dnn::Importer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_initModule() {
		try {
			cv::dnn::initModule();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_readNetFromCaffe_const_StringX_const_StringX(const char* prototxt, const char* caffeModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(cv::String(prototxt), cv::String(caffeModel));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readTorchBlob_const_StringX_bool(const char* filename, bool isBinary) {
		try {
			cv::dnn::Blob ret = cv::dnn::readTorchBlob(cv::String(filename), isBinary);
			return Ok<void*>(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_AbsLayer_create() {
		try {
			cv::Ptr<cv::dnn::AbsLayer> ret = cv::dnn::AbsLayer::create();
			return Ok<void*>(new cv::Ptr<cv::dnn::AbsLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_BNLLLayer_create() {
		try {
			cv::Ptr<cv::dnn::BNLLLayer> ret = cv::dnn::BNLLLayer::create();
			return Ok<void*>(new cv::Ptr<cv::dnn::BNLLLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_kernel_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->kernel;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setKernel_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->kernel = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_stride_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->stride;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setStride_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->stride = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_pad_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->pad;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPad_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->pad = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_dilation_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->dilation;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setDilation_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->dilation = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_BaseConvolutionLayer_padMode_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->padMode;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPadMode_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->padMode = cv::String(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Blob_delete(cv::dnn::Blob* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_Blob_Blob() {
		try {
			cv::dnn::Blob* ret = new cv::dnn::Blob();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Blob_Blob_const_BlobShapeX_int_int(void* shape, int type, int allocFlags) {
		try {
			cv::dnn::Blob* ret = new cv::dnn::Blob(*reinterpret_cast<const cv::dnn::BlobShape*>(shape), type, allocFlags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Blob_Blob_const__InputArrayX(void* data) {
		try {
			cv::dnn::Blob* ret = new cv::dnn::Blob(*reinterpret_cast<const cv::_InputArray*>(data));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Blob_fromImages_const__InputArrayX_int(void* image, int dstCn) {
		try {
			cv::dnn::Blob ret = cv::dnn::Blob::fromImages(*reinterpret_cast<const cv::_InputArray*>(image), dstCn);
			return Ok<void*>(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Blob_batchFromImages_const__InputArrayX_int(void* instance, void* image, int dstCn) {
		try {
			reinterpret_cast<cv::dnn::Blob*>(instance)->batchFromImages(*reinterpret_cast<const cv::_InputArray*>(image), dstCn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Blob_create_const_BlobShapeX_int_int(void* instance, void* shape, int type, int allocFlags) {
		try {
			reinterpret_cast<cv::dnn::Blob*>(instance)->create(*reinterpret_cast<const cv::dnn::BlobShape*>(shape), type, allocFlags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Blob_fill_const__InputArrayX(void* instance, void* in) {
		try {
			reinterpret_cast<cv::dnn::Blob*>(instance)->fill(*reinterpret_cast<const cv::_InputArray*>(in));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Blob_fill_const_BlobShapeX_int_voidX_bool(void* instance, void* shape, int type, void* data, bool deepCopy) {
		try {
			reinterpret_cast<cv::dnn::Blob*>(instance)->fill(*reinterpret_cast<const cv::dnn::BlobShape*>(shape), type, data, deepCopy);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Blob_setTo_const__InputArrayX_int(void* instance, void* value, int allocFlags) {
		try {
			reinterpret_cast<cv::dnn::Blob*>(instance)->setTo(*reinterpret_cast<const cv::_InputArray*>(value), allocFlags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Blob_matRef_bool(void* instance, bool writeOnly) {
		try {
			cv::Mat ret = reinterpret_cast<cv::dnn::Blob*>(instance)->matRef(writeOnly);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Blob_matRefConst_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::dnn::Blob*>(instance)->matRefConst();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Blob_umatRef_bool(void* instance, bool writeOnly) {
		try {
			cv::UMat ret = reinterpret_cast<cv::dnn::Blob*>(instance)->umatRef(writeOnly);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Blob_umatRefConst_const(void* instance) {
		try {
			cv::UMat ret = reinterpret_cast<cv::dnn::Blob*>(instance)->umatRefConst();
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Blob_updateMat_const_bool(void* instance, bool syncData) {
		try {
			reinterpret_cast<cv::dnn::Blob*>(instance)->updateMat(syncData);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Blob_updateUMat_const_bool(void* instance, bool syncData) {
		try {
			reinterpret_cast<cv::dnn::Blob*>(instance)->updateUMat(syncData);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Blob_sync_const(void* instance) {
		try {
			reinterpret_cast<cv::dnn::Blob*>(instance)->sync();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_Blob_dims_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::Blob*>(instance)->dims();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Blob_size_const_int(void* instance, int axis) {
		try {
			int ret = reinterpret_cast<cv::dnn::Blob*>(instance)->size(axis);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Blob_xsize_const_int(void* instance, int axis) {
		try {
			int ret = reinterpret_cast<cv::dnn::Blob*>(instance)->xsize(axis);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<size_t> cv_dnn_Blob_total_const_int_int(void* instance, int startAxis, int endAxis) {
		try {
			size_t ret = reinterpret_cast<cv::dnn::Blob*>(instance)->total(startAxis, endAxis);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_dnn_Blob_canonicalAxis_const_int(void* instance, int axis) {
		try {
			int ret = reinterpret_cast<cv::dnn::Blob*>(instance)->canonicalAxis(axis);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_dnn_Blob_shape_const(void* instance) {
		try {
			cv::dnn::BlobShape ret = reinterpret_cast<cv::dnn::Blob*>(instance)->shape();
			return Ok<void*>(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_dnn_Blob_equalShape_const_const_BlobX(void* instance, void* other) {
		try {
			bool ret = reinterpret_cast<cv::dnn::Blob*>(instance)->equalShape(*reinterpret_cast<const cv::dnn::Blob*>(other));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_dnn_Blob_getPlane_int_int(void* instance, int n, int cn) {
		try {
			cv::Mat ret = reinterpret_cast<cv::dnn::Blob*>(instance)->getPlane(n, cn);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Blob_getPlanes_int(void* instance, int n) {
		try {
			cv::Mat ret = reinterpret_cast<cv::dnn::Blob*>(instance)->getPlanes(n);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_Blob_cols_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::Blob*>(instance)->cols();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Blob_rows_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::Blob*>(instance)->rows();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Blob_channels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::Blob*>(instance)->channels();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Blob_num_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::Blob*>(instance)->num();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<cv::Size> cv_dnn_Blob_size2_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::Blob*>(instance)->size2();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<cv::Vec4i> cv_dnn_Blob_shape4_const(void* instance) {
		try {
			cv::Vec4i ret = reinterpret_cast<cv::dnn::Blob*>(instance)->shape4();
			return Ok<cv::Vec4i>(ret);
		} OCVRS_CATCH(Result<cv::Vec4i>)
	}
	
	Result<size_t> cv_dnn_Blob_offset_const_int_int_int_int(void* instance, int n, int cn, int row, int col) {
		try {
			size_t ret = reinterpret_cast<cv::dnn::Blob*>(instance)->offset(n, cn, row, col);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<unsigned char*> cv_dnn_Blob_ptr_int_int_int_int(void* instance, int n, int cn, int row, int col) {
		try {
			unsigned char* ret = reinterpret_cast<cv::dnn::Blob*>(instance)->ptr(n, cn, row, col);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result<float*> cv_dnn_Blob_ptrf_int_int_int_int(void* instance, int n, int cn, int row, int col) {
		try {
			float* ret = reinterpret_cast<cv::dnn::Blob*>(instance)->ptrf(n, cn, row, col);
			return Ok<float*>(ret);
		} OCVRS_CATCH(Result<float*>)
	}
	
	Result<void*> cv_dnn_Blob_shareFrom_const_BlobX(void* instance, void* blob) {
		try {
			cv::dnn::Blob ret = reinterpret_cast<cv::dnn::Blob*>(instance)->shareFrom(*reinterpret_cast<const cv::dnn::Blob*>(blob));
			return Ok<void*>(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Blob_reshape_const_BlobShapeX(void* instance, void* shape) {
		try {
			cv::dnn::Blob ret = reinterpret_cast<cv::dnn::Blob*>(instance)->reshape(*reinterpret_cast<const cv::dnn::BlobShape*>(shape));
			return Ok<void*>(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Blob_reshaped_const_const_BlobShapeX(void* instance, void* newShape) {
		try {
			cv::dnn::Blob ret = reinterpret_cast<cv::dnn::Blob*>(instance)->reshaped(*reinterpret_cast<const cv::dnn::BlobShape*>(newShape));
			return Ok<void*>(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_Blob_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::Blob*>(instance)->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Blob_elemSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::Blob*>(instance)->elemSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Blob_getState_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::Blob*>(instance)->getState();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_BlobShape_delete(cv::dnn::BlobShape* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_BlobShape_BlobShape() {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_BlobShape_BlobShape_int(int s0) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(s0);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_BlobShape_BlobShape_int_int(int s0, int s1) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(s0, s1);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_BlobShape_BlobShape_int_int_int(int s0, int s1, int s2) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(s0, s1, s2);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_BlobShape_BlobShape_int_int_int_int(int num, int cn, int rows, int cols) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(num, cn, rows, cols);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_BlobShape_BlobShape_int_const_intX(int ndims, const int* sizes) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(ndims, sizes);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_BlobShape_BlobShape_const_vector_int_X(void* sizes) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(*reinterpret_cast<const std::vector<int>*>(sizes));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_BlobShape_all_int_int(int ndims, int fill) {
		try {
			cv::dnn::BlobShape ret = cv::dnn::BlobShape::all(ndims, fill);
			return Ok<void*>(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_BlobShape_dims_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->dims();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_BlobShape_size_int(void* instance, int axis) {
		try {
			int ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->size(axis);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_BlobShape_size_const_int(void* instance, int axis) {
		try {
			int ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->size(axis);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_BlobShape_operator___const_int(void* instance, int axis) {
		try {
			int ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->operator[](axis);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_BlobShape_operator___int(void* instance, int axis) {
		try {
			int ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->operator[](axis);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_BlobShape_xsize_const_int(void* instance, int axis) {
		try {
			int ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->xsize(axis);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_BlobShape_canonicalAxis_const_int(void* instance, int axis) {
		try {
			int ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->canonicalAxis(axis);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<ptrdiff_t> cv_dnn_BlobShape_total_const(void* instance) {
		try {
			ptrdiff_t ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->total();
			return Ok<ptrdiff_t>(ret);
		} OCVRS_CATCH(Result<ptrdiff_t>)
	}
	
	Result<ptrdiff_t> cv_dnn_BlobShape_total_const_int_int(void* instance, int startAxis, int endAxis) {
		try {
			ptrdiff_t ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->total(startAxis, endAxis);
			return Ok<ptrdiff_t>(ret);
		} OCVRS_CATCH(Result<ptrdiff_t>)
	}
	
	Result<void*> cv_dnn_BlobShape_slice_const_int_int(void* instance, int startAxis, int endAxis) {
		try {
			cv::dnn::BlobShape ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->slice(startAxis, endAxis);
			return Ok<void*>(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<const int*> cv_dnn_BlobShape_ptr_const(void* instance) {
		try {
			const int* ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->ptr();
			return Ok<const int*>(ret);
		} OCVRS_CATCH(Result<const int*>)
	}
	
	Result<int*> cv_dnn_BlobShape_ptr(void* instance) {
		try {
			int* ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->ptr();
			return Ok<int*>(ret);
		} OCVRS_CATCH(Result<int*>)
	}
	
	Result<bool> cv_dnn_BlobShape_equal_const_const_BlobShapeX(void* instance, void* other) {
		try {
			bool ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->equal(*reinterpret_cast<const cv::dnn::BlobShape*>(other));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_dnn_BlobShape_like_const_MatX(void* m) {
		try {
			cv::dnn::BlobShape ret = cv::dnn::BlobShape::like(*reinterpret_cast<const cv::Mat*>(m));
			return Ok<void*>(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_BlobShape_like_const_UMatX(void* m) {
		try {
			cv::dnn::BlobShape ret = cv::dnn::BlobShape::like(*reinterpret_cast<const cv::UMat*>(m));
			return Ok<void*>(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_BlobShape_empty() {
		try {
			cv::dnn::BlobShape ret = cv::dnn::BlobShape::empty();
			return Ok<void*>(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_dnn_BlobShape_isEmpty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::BlobShape*>(instance)->isEmpty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_dnn_ConcatLayer_axis_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::ConcatLayer*>(instance)->axis;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_ConcatLayer_setAxis_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::ConcatLayer*>(instance)->axis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_ConcatLayer_create_int(int axis) {
		try {
			cv::Ptr<cv::dnn::ConcatLayer> ret = cv::dnn::ConcatLayer::create(axis);
			return Ok<void*>(new cv::Ptr<cv::dnn::ConcatLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_ConvolutionLayer_create_Size_Size_Size_Size(cv::Size kernel, cv::Size stride, cv::Size pad, cv::Size dilation) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::ConvolutionLayer::create(kernel, stride, pad, dilation);
			return Ok<void*>(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_CropLayer_startAxis_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::CropLayer*>(instance)->startAxis;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_CropLayer_setStartAxis_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::CropLayer*>(instance)->startAxis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_CropLayer_offset(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::dnn::CropLayer*>(instance)->offset;
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_CropLayer_setOffset_vector_int_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::CropLayer*>(instance)->offset = *reinterpret_cast<std::vector<int>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_CropLayer_create_int_const_vector_int_X(int start_axis, void* offset) {
		try {
			cv::Ptr<cv::dnn::CropLayer> ret = cv::dnn::CropLayer::create(start_axis, *reinterpret_cast<const std::vector<int>*>(offset));
			return Ok<void*>(new cv::Ptr<cv::dnn::CropLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DeconvolutionLayer_create_Size_Size_Size_Size(cv::Size kernel, cv::Size stride, cv::Size pad, cv::Size dilation) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::DeconvolutionLayer::create(kernel, stride, pad, dilation);
			return Ok<void*>(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Dict_delete(cv::dnn::Dict* instance) {
		delete instance;
	}
	Result<bool> cv_dnn_Dict_has_const_const_StringX(void* instance, const char* key) {
		try {
			bool ret = reinterpret_cast<cv::dnn::Dict*>(instance)->has(cv::String(key));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_dnn_Dict_ptr_const_StringX(void* instance, const char* key) {
		try {
			cv::dnn::DictValue* ret = reinterpret_cast<cv::dnn::Dict*>(instance)->ptr(cv::String(key));
			return Ok<void*>(new cv::dnn::DictValue*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Dict_get_const_const_StringX(void* instance, const char* key) {
		try {
			cv::dnn::DictValue ret = reinterpret_cast<cv::dnn::Dict*>(instance)->get(cv::String(key));
			return Ok<void*>(new cv::dnn::DictValue(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Dict_set_cv_String_const_StringX_const_StringX(void* instance, const char* key, const char* value) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::Dict*>(instance)->set<cv::String>(cv::String(key), cv::String(value));
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Dict_set_cv_dnn_DictValue_const_StringX_const_DictValueX(void* instance, const char* key, void* value) {
		try {
			cv::dnn::DictValue ret = reinterpret_cast<cv::dnn::Dict*>(instance)->set<cv::dnn::DictValue>(cv::String(key), *reinterpret_cast<const cv::dnn::DictValue*>(value));
			return Ok<void*>(new cv::dnn::DictValue(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_dnn_Dict_set_double_const_StringX_const_doubleX(void* instance, const char* key, const double* value) {
		try {
			double ret = reinterpret_cast<cv::dnn::Dict*>(instance)->set<double>(cv::String(key), *value);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int64_t> cv_dnn_Dict_set_int64_t_const_StringX_const_int64_tX(void* instance, const char* key, const int64_t* value) {
		try {
			int64_t ret = reinterpret_cast<cv::dnn::Dict*>(instance)->set<int64_t>(cv::String(key), *value);
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	void cv_DictValue_delete(cv::dnn::DictValue* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_DictValue_DictValue_const_DictValueX(void* r) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(*reinterpret_cast<const cv::dnn::DictValue*>(r));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_DictValue_int64_t(int64_t i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_DictValue_int(int i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_DictValue_unsigned_int(unsigned int p) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_DictValue_double(double p) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_DictValue_const_charX(const char* s) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(s);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_get_cv_String_const_int(void* instance, int idx) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->get<cv::String>(idx);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_dnn_DictValue_get_double_const_int(void* instance, int idx) {
		try {
			double ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->get<double>(idx);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_dnn_DictValue_get_int_const_int(void* instance, int idx) {
		try {
			int ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->get<int>(idx);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int64_t> cv_dnn_DictValue_get_int64_t_const_int(void* instance, int idx) {
		try {
			int64_t ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->get<int64_t>(idx);
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<int> cv_dnn_DictValue_size_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->size();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_dnn_DictValue_isInt_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->isInt();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_dnn_DictValue_isString_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->isString();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_dnn_DictValue_isReal_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->isReal();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_dnn_EltwiseLayer_create_EltwiseOp_const_vector_int_X(cv::dnn::EltwiseLayer::EltwiseOp op, void* coeffs) {
		try {
			cv::Ptr<cv::dnn::EltwiseLayer> ret = cv::dnn::EltwiseLayer::create(op, *reinterpret_cast<const std::vector<int>*>(coeffs));
			return Ok<void*>(new cv::Ptr<cv::dnn::EltwiseLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Importer_populateNet_Net(void* instance, void* net) {
		try {
			reinterpret_cast<cv::dnn::Importer*>(instance)->populateNet(*reinterpret_cast<cv::dnn::Net*>(net));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_InnerProductLayer_axis_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::InnerProductLayer*>(instance)->axis;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_InnerProductLayer_setAxis_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::InnerProductLayer*>(instance)->axis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_InnerProductLayer_create_int(int axis) {
		try {
			cv::Ptr<cv::dnn::InnerProductLayer> ret = cv::dnn::InnerProductLayer::create(axis);
			return Ok<void*>(new cv::Ptr<cv::dnn::InnerProductLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_LRNLayer_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->type;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_LRNLayer_setType_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->type = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_LRNLayer_size_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->size;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_LRNLayer_setSize_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->size = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_dnn_LRNLayer_alpha_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->alpha;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_dnn_LRNLayer_setAlpha_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->alpha = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_dnn_LRNLayer_beta_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->beta;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_dnn_LRNLayer_setBeta_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->beta = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_dnn_LRNLayer_bias_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->bias;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_dnn_LRNLayer_setBias_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->bias = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_LRNLayer_normBySize_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->normBySize;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_LRNLayer_setNormBySize_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->normBySize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_LRNLayer_create_int_int_double_double_double_bool(int type, int size, double alpha, double beta, double bias, bool normBySize) {
		try {
			cv::Ptr<cv::dnn::LRNLayer> ret = cv::dnn::LRNLayer::create(type, size, alpha, beta, bias, normBySize);
			return Ok<void*>(new cv::Ptr<cv::dnn::LRNLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_LSTMLayer_create() {
		try {
			cv::Ptr<cv::dnn::LSTMLayer> ret = cv::dnn::LSTMLayer::create();
			return Ok<void*>(new cv::Ptr<cv::dnn::LSTMLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LSTMLayer_setWeights_const_BlobX_const_BlobX_const_BlobX(void* instance, void* Wh, void* Wx, void* b) {
		try {
			reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->setWeights(*reinterpret_cast<const cv::dnn::Blob*>(Wh), *reinterpret_cast<const cv::dnn::Blob*>(Wx), *reinterpret_cast<const cv::dnn::Blob*>(b));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LSTMLayer_setOutShape_const_BlobShapeX(void* instance, void* outTailShape) {
		try {
			reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->setOutShape(*reinterpret_cast<const cv::dnn::BlobShape*>(outTailShape));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LSTMLayer_setH_const_BlobX(void* instance, void* H) {
		try {
			reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->setH(*reinterpret_cast<const cv::dnn::Blob*>(H));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_LSTMLayer_getH_const(void* instance) {
		try {
			cv::dnn::Blob ret = reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->getH();
			return Ok<void*>(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LSTMLayer_setC_const_BlobX(void* instance, void* C) {
		try {
			reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->setC(*reinterpret_cast<const cv::dnn::Blob*>(C));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_LSTMLayer_getC_const(void* instance) {
		try {
			cv::dnn::Blob ret = reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->getC();
			return Ok<void*>(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LSTMLayer_setUseTimstampsDim_bool(void* instance, bool use) {
		try {
			reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->setUseTimstampsDim(use);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LSTMLayer_setProduceCellOutput_bool(void* instance, bool produce) {
		try {
			reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->setProduceCellOutput(produce);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LSTMLayer_forward_vector_BlobX_X_vector_Blob_X(void* instance, void* input, void* output) {
		try {
			reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->forward(*reinterpret_cast<std::vector<cv::dnn::Blob*>*>(input), *reinterpret_cast<std::vector<cv::dnn::Blob>*>(output));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_LSTMLayer_inputNameToIndex_String(void* instance, char* inputName) {
		try {
			int ret = reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->inputNameToIndex(cv::String(inputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_LSTMLayer_outputNameToIndex_String(void* instance, char* outputName) {
		try {
			int ret = reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->outputNameToIndex(cv::String(outputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_dnn_Layer_blobs(void* instance) {
		try {
			std::vector<cv::dnn::Blob> ret = reinterpret_cast<cv::dnn::Layer*>(instance)->blobs;
			return Ok<void*>(new std::vector<cv::dnn::Blob>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_setBlobs_vector_Blob_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->blobs = *reinterpret_cast<std::vector<cv::dnn::Blob>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Layer_name_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::Layer*>(instance)->name;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_setName_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->name = cv::String(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Layer_type_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::Layer*>(instance)->type;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_setType_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->type = cv::String(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_allocate_const_vector_BlobX_X_vector_Blob_X(void* instance, void* input, void* output) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->allocate(*reinterpret_cast<const std::vector<cv::dnn::Blob*>*>(input), *reinterpret_cast<std::vector<cv::dnn::Blob>*>(output));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_forward_vector_BlobX_X_vector_Blob_X(void* instance, void* input, void* output) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->forward(*reinterpret_cast<std::vector<cv::dnn::Blob*>*>(input), *reinterpret_cast<std::vector<cv::dnn::Blob>*>(output));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_allocate_const_vector_Blob_X_vector_Blob_X(void* instance, void* inputs, void* outputs) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->allocate(*reinterpret_cast<const std::vector<cv::dnn::Blob>*>(inputs), *reinterpret_cast<std::vector<cv::dnn::Blob>*>(outputs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Layer_allocate_const_vector_Blob_X(void* instance, void* inputs) {
		try {
			std::vector<cv::dnn::Blob> ret = reinterpret_cast<cv::dnn::Layer*>(instance)->allocate(*reinterpret_cast<const std::vector<cv::dnn::Blob>*>(inputs));
			return Ok<void*>(new std::vector<cv::dnn::Blob>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_forward_const_vector_Blob_X_vector_Blob_X(void* instance, void* inputs, void* outputs) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->forward(*reinterpret_cast<const std::vector<cv::dnn::Blob>*>(inputs), *reinterpret_cast<std::vector<cv::dnn::Blob>*>(outputs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_run_const_vector_Blob_X_vector_Blob_X(void* instance, void* inputs, void* outputs) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->run(*reinterpret_cast<const std::vector<cv::dnn::Blob>*>(inputs), *reinterpret_cast<std::vector<cv::dnn::Blob>*>(outputs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_Layer_inputNameToIndex_String(void* instance, char* inputName) {
		try {
			int ret = reinterpret_cast<cv::dnn::Layer*>(instance)->inputNameToIndex(cv::String(inputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Layer_outputNameToIndex_String(void* instance, char* outputName) {
		try {
			int ret = reinterpret_cast<cv::dnn::Layer*>(instance)->outputNameToIndex(cv::String(outputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_Layer_setParamsFrom_const_LayerParamsX(void* instance, void* params) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->setParamsFrom(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LayerFactory_delete(cv::dnn::LayerFactory* instance) {
		delete instance;
	}
	Result_void cv_dnn_LayerFactory_registerLayer_const_StringX_Constuctor(const char* type, cv::dnn::LayerFactory::Constuctor constructor) {
		try {
			cv::dnn::LayerFactory::registerLayer(cv::String(type), constructor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LayerFactory_unregisterLayer_const_StringX(const char* type) {
		try {
			cv::dnn::LayerFactory::unregisterLayer(cv::String(type));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_LayerFactory_createLayerInstance_const_StringX_LayerParamsX(const char* type, void* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::LayerFactory::createLayerInstance(cv::String(type), *reinterpret_cast<cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_LayerParams_blobs(void* instance) {
		try {
			std::vector<cv::dnn::Blob> ret = reinterpret_cast<cv::dnn::LayerParams*>(instance)->blobs;
			return Ok<void*>(new std::vector<cv::dnn::Blob>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LayerParams_setBlobs_vector_Blob_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::LayerParams*>(instance)->blobs = *reinterpret_cast<std::vector<cv::dnn::Blob>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_LayerParams_name_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::LayerParams*>(instance)->name;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LayerParams_setName_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::LayerParams*>(instance)->name = cv::String(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_LayerParams_type_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::LayerParams*>(instance)->type;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LayerParams_setType_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::LayerParams*>(instance)->type = cv::String(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LayerParams_delete(cv::dnn::LayerParams* instance) {
		delete instance;
	}
	Result<double> cv_dnn_MVNLayer_eps_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::dnn::MVNLayer*>(instance)->eps;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_dnn_MVNLayer_setEps_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::dnn::MVNLayer*>(instance)->eps = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_MVNLayer_normVariance_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::MVNLayer*>(instance)->normVariance;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_MVNLayer_setNormVariance_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::MVNLayer*>(instance)->normVariance = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_MVNLayer_acrossChannels_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::MVNLayer*>(instance)->acrossChannels;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_MVNLayer_setAcrossChannels_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::MVNLayer*>(instance)->acrossChannels = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_MVNLayer_create_bool_bool_double(bool normVariance, bool acrossChannels, double eps) {
		try {
			cv::Ptr<cv::dnn::MVNLayer> ret = cv::dnn::MVNLayer::create(normVariance, acrossChannels, eps);
			return Ok<void*>(new cv::Ptr<cv::dnn::MVNLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Net_delete(cv::dnn::Net* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_Net_Net() {
		try {
			cv::dnn::Net* ret = new cv::dnn::Net();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_dnn_Net_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::Net*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_dnn_Net_addLayer_const_StringX_const_StringX_LayerParamsX(void* instance, const char* name, const char* type, void* params) {
		try {
			int ret = reinterpret_cast<cv::dnn::Net*>(instance)->addLayer(cv::String(name), cv::String(type), *reinterpret_cast<cv::dnn::LayerParams*>(params));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Net_addLayerToPrev_const_StringX_const_StringX_LayerParamsX(void* instance, const char* name, const char* type, void* params) {
		try {
			int ret = reinterpret_cast<cv::dnn::Net*>(instance)->addLayerToPrev(cv::String(name), cv::String(type), *reinterpret_cast<cv::dnn::LayerParams*>(params));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Net_getLayerId_const_StringX(void* instance, const char* layer) {
		try {
			int ret = reinterpret_cast<cv::dnn::Net*>(instance)->getLayerId(cv::String(layer));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_dnn_Net_getLayerNames_const(void* instance) {
		try {
			std::vector<cv::String> ret = reinterpret_cast<cv::dnn::Net*>(instance)->getLayerNames();
			return Ok<void*>(new std::vector<cv::String>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Net_getLayer_LayerId(void* instance, void* layerId) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = reinterpret_cast<cv::dnn::Net*>(instance)->getLayer(*reinterpret_cast<cv::dnn::Net::LayerId*>(layerId));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Net_deleteLayer_LayerId(void* instance, void* layer) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->deleteLayer(*reinterpret_cast<cv::dnn::Net::LayerId*>(layer));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_connect_String_String(void* instance, char* outPin, char* inpPin) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->connect(cv::String(outPin), cv::String(inpPin));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_connect_int_int_int_int(void* instance, int outLayerId, int outNum, int inpLayerId, int inpNum) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->connect(outLayerId, outNum, inpLayerId, inpNum);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setNetInputs_const_vector_String_X(void* instance, void* inputBlobNames) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->setNetInputs(*reinterpret_cast<const std::vector<cv::String>*>(inputBlobNames));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_allocate(void* instance) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->allocate();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_forward_LayerId(void* instance, void* toLayer) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->forward(*reinterpret_cast<cv::dnn::Net::LayerId*>(toLayer));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_forward_LayerId_LayerId(void* instance, void* startLayer, void* toLayer) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->forward(*reinterpret_cast<cv::dnn::Net::LayerId*>(startLayer), *reinterpret_cast<cv::dnn::Net::LayerId*>(toLayer));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_forward_const_vector_LayerId_X_const_vector_LayerId_X(void* instance, void* startLayers, void* toLayers) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->forward(*reinterpret_cast<const std::vector<cv::dnn::Net::LayerId>*>(startLayers), *reinterpret_cast<const std::vector<cv::dnn::Net::LayerId>*>(toLayers));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_forwardOpt_LayerId(void* instance, void* toLayer) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->forwardOpt(*reinterpret_cast<cv::dnn::Net::LayerId*>(toLayer));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_forwardOpt_const_vector_LayerId_X(void* instance, void* toLayers) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->forwardOpt(*reinterpret_cast<const std::vector<cv::dnn::Net::LayerId>*>(toLayers));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setBlob_String_const_BlobX(void* instance, char* outputName, void* blob) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->setBlob(cv::String(outputName), *reinterpret_cast<const cv::dnn::Blob*>(blob));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Net_getBlob_String(void* instance, char* outputName) {
		try {
			cv::dnn::Blob ret = reinterpret_cast<cv::dnn::Net*>(instance)->getBlob(cv::String(outputName));
			return Ok<void*>(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Net_setParam_LayerId_int_const_BlobX(void* instance, void* layer, int numParam, void* blob) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->setParam(*reinterpret_cast<cv::dnn::Net::LayerId*>(layer), numParam, *reinterpret_cast<const cv::dnn::Blob*>(blob));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Net_getParam_LayerId_int(void* instance, void* layer, int numParam) {
		try {
			cv::dnn::Blob ret = reinterpret_cast<cv::dnn::Net*>(instance)->getParam(*reinterpret_cast<cv::dnn::Net::LayerId*>(layer), numParam);
			return Ok<void*>(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_PoolingLayer_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->type;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setType_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->type = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_kernel_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->kernel;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_PoolingLayer_setKernel_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->kernel = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_stride_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->stride;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_PoolingLayer_setStride_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->stride = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_pad_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPad_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_PoolingLayer_globalPooling_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->globalPooling;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_PoolingLayer_setGlobalPooling_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->globalPooling = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_PoolingLayer_padMode_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->padMode;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPadMode_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->padMode = cv::String(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_PoolingLayer_create_int_Size_Size_Size_const_StringX(int type, cv::Size kernel, cv::Size stride, cv::Size pad, const char* padMode) {
		try {
			cv::Ptr<cv::dnn::PoolingLayer> ret = cv::dnn::PoolingLayer::create(type, kernel, stride, pad, cv::String(padMode));
			return Ok<void*>(new cv::Ptr<cv::dnn::PoolingLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_PoolingLayer_createGlobal_int(int type) {
		try {
			cv::Ptr<cv::dnn::PoolingLayer> ret = cv::dnn::PoolingLayer::createGlobal(type);
			return Ok<void*>(new cv::Ptr<cv::dnn::PoolingLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_dnn_PowerLayer_power_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::dnn::PowerLayer*>(instance)->power;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_dnn_PowerLayer_setPower_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::dnn::PowerLayer*>(instance)->power = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_dnn_PowerLayer_scale_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::dnn::PowerLayer*>(instance)->scale;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_dnn_PowerLayer_setScale_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::dnn::PowerLayer*>(instance)->scale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_dnn_PowerLayer_shift_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::dnn::PowerLayer*>(instance)->shift;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_dnn_PowerLayer_setShift_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::dnn::PowerLayer*>(instance)->shift = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_PowerLayer_create_double_double_double(double power, double scale, double shift) {
		try {
			cv::Ptr<cv::dnn::PowerLayer> ret = cv::dnn::PowerLayer::create(power, scale, shift);
			return Ok<void*>(new cv::Ptr<cv::dnn::PowerLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_RNNLayer_create() {
		try {
			cv::Ptr<cv::dnn::RNNLayer> ret = cv::dnn::RNNLayer::create();
			return Ok<void*>(new cv::Ptr<cv::dnn::RNNLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_RNNLayer_setWeights_const_BlobX_const_BlobX_const_BlobX_const_BlobX_const_BlobX(void* instance, void* Wxh, void* bh, void* Whh, void* Who, void* bo) {
		try {
			reinterpret_cast<cv::dnn::RNNLayer*>(instance)->setWeights(*reinterpret_cast<const cv::dnn::Blob*>(Wxh), *reinterpret_cast<const cv::dnn::Blob*>(bh), *reinterpret_cast<const cv::dnn::Blob*>(Whh), *reinterpret_cast<const cv::dnn::Blob*>(Who), *reinterpret_cast<const cv::dnn::Blob*>(bo));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_RNNLayer_setProduceHiddenOutput_bool(void* instance, bool produce) {
		try {
			reinterpret_cast<cv::dnn::RNNLayer*>(instance)->setProduceHiddenOutput(produce);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_RNNLayer_forward_vector_BlobX_X_vector_Blob_X(void* instance, void* input, void* output) {
		try {
			reinterpret_cast<cv::dnn::RNNLayer*>(instance)->forward(*reinterpret_cast<std::vector<cv::dnn::Blob*>*>(input), *reinterpret_cast<std::vector<cv::dnn::Blob>*>(output));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_dnn_ReLULayer_negativeSlope_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::dnn::ReLULayer*>(instance)->negativeSlope;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_dnn_ReLULayer_setNegativeSlope_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::dnn::ReLULayer*>(instance)->negativeSlope = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_ReLULayer_create_double(double negativeSlope) {
		try {
			cv::Ptr<cv::dnn::ReLULayer> ret = cv::dnn::ReLULayer::create(negativeSlope);
			return Ok<void*>(new cv::Ptr<cv::dnn::ReLULayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_ReshapeLayer_newShapeDesc(void* instance) {
		try {
			cv::dnn::BlobShape ret = reinterpret_cast<cv::dnn::ReshapeLayer*>(instance)->newShapeDesc;
			return Ok<void*>(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_ReshapeLayer_setNewShapeDesc_BlobShape(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::ReshapeLayer*>(instance)->newShapeDesc = *reinterpret_cast<cv::dnn::BlobShape*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_ReshapeLayer_newShapeRange(void* instance) {
		try {
			cv::Range ret = reinterpret_cast<cv::dnn::ReshapeLayer*>(instance)->newShapeRange;
			return Ok<void*>(new cv::Range(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_ReshapeLayer_setNewShapeRange_Range(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::ReshapeLayer*>(instance)->newShapeRange = *reinterpret_cast<cv::Range*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_ReshapeLayer_create_const_BlobShapeX_Range_bool(void* newShape, void* applyingRange, bool enableReordering) {
		try {
			cv::Ptr<cv::dnn::ReshapeLayer> ret = cv::dnn::ReshapeLayer::create(*reinterpret_cast<const cv::dnn::BlobShape*>(newShape), *reinterpret_cast<cv::Range*>(applyingRange), enableReordering);
			return Ok<void*>(new cv::Ptr<cv::dnn::ReshapeLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_SigmoidLayer_create() {
		try {
			cv::Ptr<cv::dnn::SigmoidLayer> ret = cv::dnn::SigmoidLayer::create();
			return Ok<void*>(new cv::Ptr<cv::dnn::SigmoidLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_SliceLayer_axis_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::SliceLayer*>(instance)->axis;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_SliceLayer_setAxis_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::SliceLayer*>(instance)->axis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_SliceLayer_sliceIndices(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::dnn::SliceLayer*>(instance)->sliceIndices;
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_SliceLayer_setSliceIndices_vector_int_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::SliceLayer*>(instance)->sliceIndices = *reinterpret_cast<std::vector<int>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_SliceLayer_create_int(int axis) {
		try {
			cv::Ptr<cv::dnn::SliceLayer> ret = cv::dnn::SliceLayer::create(axis);
			return Ok<void*>(new cv::Ptr<cv::dnn::SliceLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_SliceLayer_create_int_const_vector_int_X(int axis, void* sliceIndices) {
		try {
			cv::Ptr<cv::dnn::SliceLayer> ret = cv::dnn::SliceLayer::create(axis, *reinterpret_cast<const std::vector<int>*>(sliceIndices));
			return Ok<void*>(new cv::Ptr<cv::dnn::SliceLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_SoftmaxLayer_create_int(int axis) {
		try {
			cv::Ptr<cv::dnn::SoftmaxLayer> ret = cv::dnn::SoftmaxLayer::create(axis);
			return Ok<void*>(new cv::Ptr<cv::dnn::SoftmaxLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_SplitLayer_outputsCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::SplitLayer*>(instance)->outputsCount;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_SplitLayer_setOutputsCount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::SplitLayer*>(instance)->outputsCount = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_SplitLayer_create_int(int outputsCount) {
		try {
			cv::Ptr<cv::dnn::SplitLayer> ret = cv::dnn::SplitLayer::create(outputsCount);
			return Ok<void*>(new cv::Ptr<cv::dnn::SplitLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_TanHLayer_create() {
		try {
			cv::Ptr<cv::dnn::TanHLayer> ret = cv::dnn::TanHLayer::create();
			return Ok<void*>(new cv::Ptr<cv::dnn::TanHLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv__Range_delete(cv::dnn::_Range* instance) {
		delete instance;
	}
	Result<void*> cv_dnn__Range__Range_const_RangeX(void* r) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(*reinterpret_cast<const cv::Range*>(r));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn__Range__Range_int_int(int start, int size) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(start, size);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
}

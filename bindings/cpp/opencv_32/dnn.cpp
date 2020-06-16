#include "dnn.hpp"
#include "dnn_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::dnn::Importer>*> cv_dnn_createCaffeImporter_const_StringR_const_StringR(const char* prototxt, const char* caffeModel) {
		try {
			cv::Ptr<cv::dnn::Importer> ret = cv::dnn::createCaffeImporter(cv::String(prototxt), cv::String(caffeModel));
			return Ok(new cv::Ptr<cv::dnn::Importer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Importer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::Importer>*> cv_dnn_createTensorflowImporter_const_StringR(const char* model) {
		try {
			cv::Ptr<cv::dnn::Importer> ret = cv::dnn::createTensorflowImporter(cv::String(model));
			return Ok(new cv::Ptr<cv::dnn::Importer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Importer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::Importer>*> cv_dnn_createTorchImporter_const_StringR_bool(const char* filename, bool isBinary) {
		try {
			cv::Ptr<cv::dnn::Importer> ret = cv::dnn::createTorchImporter(cv::String(filename), isBinary);
			return Ok(new cv::Ptr<cv::dnn::Importer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Importer>*>))
	}
	
	Result_void cv_dnn_initModule() {
		try {
			cv::dnn::initModule();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromCaffe_const_StringR_const_StringR(const char* prototxt, const char* caffeModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(cv::String(prototxt), cv::String(caffeModel));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Blob*> cv_dnn_readTorchBlob_const_StringR_bool(const char* filename, bool isBinary) {
		try {
			cv::dnn::Blob ret = cv::dnn::readTorchBlob(cv::String(filename), isBinary);
			return Ok(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result<cv::Ptr<cv::dnn::AbsLayer>*> cv_dnn_AbsLayer_create() {
		try {
			cv::Ptr<cv::dnn::AbsLayer> ret = cv::dnn::AbsLayer::create();
			return Ok(new cv::Ptr<cv::dnn::AbsLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::AbsLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::BNLLLayer>*> cv_dnn_BNLLLayer_create() {
		try {
			cv::Ptr<cv::dnn::BNLLLayer> ret = cv::dnn::BNLLLayer::create();
			return Ok(new cv::Ptr<cv::dnn::BNLLLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::BNLLLayer>*>))
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_getPropKernel_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->kernel;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropKernel_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
		try {
			instance->kernel = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_getPropStride_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->stride;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropStride_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
		try {
			instance->stride = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_getPropPad_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->pad;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropPad_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
		try {
			instance->pad = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_getPropDilation_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->dilation;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropDilation_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
		try {
			instance->dilation = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_BaseConvolutionLayer_getPropPadMode_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::String ret = instance->padMode;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropPadMode_String(cv::dnn::BaseConvolutionLayer* instance, char* val) {
		try {
			instance->padMode = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Blob_delete(cv::dnn::Blob* instance) {
		delete instance;
	}
	Result<cv::dnn::Blob*> cv_dnn_Blob_Blob() {
		try {
			cv::dnn::Blob* ret = new cv::dnn::Blob();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result<cv::dnn::Blob*> cv_dnn_Blob_Blob_const_BlobShapeR_int_int(const cv::dnn::BlobShape* shape, int type, int allocFlags) {
		try {
			cv::dnn::Blob* ret = new cv::dnn::Blob(*shape, type, allocFlags);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result<cv::dnn::Blob*> cv_dnn_Blob_Blob_const__InputArrayR(const cv::_InputArray* data) {
		try {
			cv::dnn::Blob* ret = new cv::dnn::Blob(*data);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result<cv::dnn::Blob*> cv_dnn_Blob_fromImages_const__InputArrayR_int(const cv::_InputArray* image, int dstCn) {
		try {
			cv::dnn::Blob ret = cv::dnn::Blob::fromImages(*image, dstCn);
			return Ok(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result_void cv_dnn_Blob_batchFromImages_const__InputArrayR_int(cv::dnn::Blob* instance, const cv::_InputArray* image, int dstCn) {
		try {
			instance->batchFromImages(*image, dstCn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Blob_create_const_BlobShapeR_int_int(cv::dnn::Blob* instance, const cv::dnn::BlobShape* shape, int type, int allocFlags) {
		try {
			instance->create(*shape, type, allocFlags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Blob_fill_const__InputArrayR(cv::dnn::Blob* instance, const cv::_InputArray* in) {
		try {
			instance->fill(*in);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Blob_fill_const_BlobShapeR_int_voidX_bool(cv::dnn::Blob* instance, const cv::dnn::BlobShape* shape, int type, void* data, bool deepCopy) {
		try {
			instance->fill(*shape, type, data, deepCopy);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Blob_setTo_const__InputArrayR_int(cv::dnn::Blob* instance, const cv::_InputArray* value, int allocFlags) {
		try {
			instance->setTo(*value, allocFlags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_dnn_Blob_matRef_bool(cv::dnn::Blob* instance, bool writeOnly) {
		try {
			cv::Mat ret = instance->matRef(writeOnly);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_dnn_Blob_matRefConst_const(const cv::dnn::Blob* instance) {
		try {
			cv::Mat ret = instance->matRefConst();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::UMat*> cv_dnn_Blob_umatRef_bool(cv::dnn::Blob* instance, bool writeOnly) {
		try {
			cv::UMat ret = instance->umatRef(writeOnly);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_dnn_Blob_umatRefConst_const(const cv::dnn::Blob* instance) {
		try {
			cv::UMat ret = instance->umatRefConst();
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result_void cv_dnn_Blob_updateMat_const_bool(const cv::dnn::Blob* instance, bool syncData) {
		try {
			instance->updateMat(syncData);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Blob_updateUMat_const_bool(const cv::dnn::Blob* instance, bool syncData) {
		try {
			instance->updateUMat(syncData);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Blob_sync_const(const cv::dnn::Blob* instance) {
		try {
			instance->sync();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_Blob_dims_const(const cv::dnn::Blob* instance) {
		try {
			int ret = instance->dims();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Blob_size_const_int(const cv::dnn::Blob* instance, int axis) {
		try {
			int ret = instance->size(axis);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Blob_xsize_const_int(const cv::dnn::Blob* instance, int axis) {
		try {
			int ret = instance->xsize(axis);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_dnn_Blob_total_const_int_int(const cv::dnn::Blob* instance, int startAxis, int endAxis) {
		try {
			size_t ret = instance->total(startAxis, endAxis);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_dnn_Blob_canonicalAxis_const_int(const cv::dnn::Blob* instance, int axis) {
		try {
			int ret = instance->canonicalAxis(axis);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_Blob_shape_const(const cv::dnn::Blob* instance) {
		try {
			cv::dnn::BlobShape ret = instance->shape();
			return Ok(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<bool> cv_dnn_Blob_equalShape_const_const_BlobR(const cv::dnn::Blob* instance, const cv::dnn::Blob* other) {
		try {
			bool ret = instance->equalShape(*other);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Mat*> cv_dnn_Blob_getPlane_int_int(cv::dnn::Blob* instance, int n, int cn) {
		try {
			cv::Mat ret = instance->getPlane(n, cn);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_dnn_Blob_getPlanes_int(cv::dnn::Blob* instance, int n) {
		try {
			cv::Mat ret = instance->getPlanes(n);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<int> cv_dnn_Blob_cols_const(const cv::dnn::Blob* instance) {
		try {
			int ret = instance->cols();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Blob_rows_const(const cv::dnn::Blob* instance) {
		try {
			int ret = instance->rows();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Blob_channels_const(const cv::dnn::Blob* instance) {
		try {
			int ret = instance->channels();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Blob_num_const(const cv::dnn::Blob* instance) {
		try {
			int ret = instance->num();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Size> cv_dnn_Blob_size2_const(const cv::dnn::Blob* instance) {
		try {
			cv::Size ret = instance->size2();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<cv::Vec4i> cv_dnn_Blob_shape4_const(const cv::dnn::Blob* instance) {
		try {
			cv::Vec4i ret = instance->shape4();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec4i>))
	}
	
	Result<size_t> cv_dnn_Blob_offset_const_int_int_int_int(const cv::dnn::Blob* instance, int n, int cn, int row, int col) {
		try {
			size_t ret = instance->offset(n, cn, row, col);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<unsigned char*> cv_dnn_Blob_ptr_int_int_int_int(cv::dnn::Blob* instance, int n, int cn, int row, int col) {
		try {
			unsigned char* ret = instance->ptr(n, cn, row, col);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result<float*> cv_dnn_Blob_ptrf_int_int_int_int(cv::dnn::Blob* instance, int n, int cn, int row, int col) {
		try {
			float* ret = instance->ptrf(n, cn, row, col);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float*>))
	}
	
	Result<cv::dnn::Blob*> cv_dnn_Blob_shareFrom_const_BlobR(cv::dnn::Blob* instance, const cv::dnn::Blob* blob) {
		try {
			cv::dnn::Blob ret = instance->shareFrom(*blob);
			return Ok(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result<cv::dnn::Blob*> cv_dnn_Blob_reshape_const_BlobShapeR(cv::dnn::Blob* instance, const cv::dnn::BlobShape* shape) {
		try {
			cv::dnn::Blob ret = instance->reshape(*shape);
			return Ok(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result<cv::dnn::Blob*> cv_dnn_Blob_reshaped_const_const_BlobShapeR(const cv::dnn::Blob* instance, const cv::dnn::BlobShape* newShape) {
		try {
			cv::dnn::Blob ret = instance->reshaped(*newShape);
			return Ok(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result<int> cv_dnn_Blob_type_const(const cv::dnn::Blob* instance) {
		try {
			int ret = instance->type();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Blob_elemSize_const(const cv::dnn::Blob* instance) {
		try {
			int ret = instance->elemSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Blob_getState_const(const cv::dnn::Blob* instance) {
		try {
			int ret = instance->getState();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_BlobShape_delete(cv::dnn::BlobShape* instance) {
		delete instance;
	}
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_BlobShape() {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_BlobShape_int(int s0) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(s0);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_BlobShape_int_int(int s0, int s1) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(s0, s1);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_BlobShape_int_int_int(int s0, int s1, int s2) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(s0, s1, s2);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_BlobShape_int_int_int_int(int num, int cn, int rows, int cols) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(num, cn, rows, cols);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_BlobShape_int_const_intX(int ndims, const int* sizes) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(ndims, sizes);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_BlobShape_const_vector_int_R(const std::vector<int>* sizes) {
		try {
			cv::dnn::BlobShape* ret = new cv::dnn::BlobShape(*sizes);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_all_int_int(int ndims, int fill) {
		try {
			cv::dnn::BlobShape ret = cv::dnn::BlobShape::all(ndims, fill);
			return Ok(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<int> cv_dnn_BlobShape_dims_const(const cv::dnn::BlobShape* instance) {
		try {
			int ret = instance->dims();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_BlobShape_size_int(cv::dnn::BlobShape* instance, int axis) {
		try {
			int ret = instance->size(axis);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_BlobShape_size_const_int(const cv::dnn::BlobShape* instance, int axis) {
		try {
			int ret = instance->size(axis);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_BlobShape_operator___const_int(const cv::dnn::BlobShape* instance, int axis) {
		try {
			int ret = instance->operator[](axis);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_BlobShape_operator___int(cv::dnn::BlobShape* instance, int axis) {
		try {
			int ret = instance->operator[](axis);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_BlobShape_xsize_const_int(const cv::dnn::BlobShape* instance, int axis) {
		try {
			int ret = instance->xsize(axis);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_BlobShape_canonicalAxis_const_int(const cv::dnn::BlobShape* instance, int axis) {
		try {
			int ret = instance->canonicalAxis(axis);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<ptrdiff_t> cv_dnn_BlobShape_total_const(const cv::dnn::BlobShape* instance) {
		try {
			ptrdiff_t ret = instance->total();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<ptrdiff_t>))
	}
	
	Result<ptrdiff_t> cv_dnn_BlobShape_total_const_int_int(const cv::dnn::BlobShape* instance, int startAxis, int endAxis) {
		try {
			ptrdiff_t ret = instance->total(startAxis, endAxis);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<ptrdiff_t>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_slice_const_int_int(const cv::dnn::BlobShape* instance, int startAxis, int endAxis) {
		try {
			cv::dnn::BlobShape ret = instance->slice(startAxis, endAxis);
			return Ok(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<const int*> cv_dnn_BlobShape_ptr_const(const cv::dnn::BlobShape* instance) {
		try {
			const int* ret = instance->ptr();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const int*>))
	}
	
	Result<int*> cv_dnn_BlobShape_ptr(cv::dnn::BlobShape* instance) {
		try {
			int* ret = instance->ptr();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int*>))
	}
	
	Result<bool> cv_dnn_BlobShape_equal_const_const_BlobShapeR(const cv::dnn::BlobShape* instance, const cv::dnn::BlobShape* other) {
		try {
			bool ret = instance->equal(*other);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_operatorA_const_const_BlobShapeR(const cv::dnn::BlobShape* instance, const cv::dnn::BlobShape* r) {
		try {
			cv::dnn::BlobShape ret = instance->operator+(*r);
			return Ok(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_like_const_MatR(const cv::Mat* m) {
		try {
			cv::dnn::BlobShape ret = cv::dnn::BlobShape::like(*m);
			return Ok(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_like_const_UMatR(const cv::UMat* m) {
		try {
			cv::dnn::BlobShape ret = cv::dnn::BlobShape::like(*m);
			return Ok(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_BlobShape_empty() {
		try {
			cv::dnn::BlobShape ret = cv::dnn::BlobShape::empty();
			return Ok(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result<bool> cv_dnn_BlobShape_isEmpty_const(const cv::dnn::BlobShape* instance) {
		try {
			bool ret = instance->isEmpty();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_dnn_ConcatLayer_getPropAxis_const(const cv::dnn::ConcatLayer* instance) {
		try {
			int ret = instance->axis;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_ConcatLayer_setPropAxis_int(cv::dnn::ConcatLayer* instance, int val) {
		try {
			instance->axis = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::ConcatLayer>*> cv_dnn_ConcatLayer_create_int(int axis) {
		try {
			cv::Ptr<cv::dnn::ConcatLayer> ret = cv::dnn::ConcatLayer::create(axis);
			return Ok(new cv::Ptr<cv::dnn::ConcatLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ConcatLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*> cv_dnn_ConvolutionLayer_create_Size_Size_Size_Size(const cv::Size* kernel, const cv::Size* stride, const cv::Size* pad, const cv::Size* dilation) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::ConvolutionLayer::create(*kernel, *stride, *pad, *dilation);
			return Ok(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*>))
	}
	
	Result<int> cv_dnn_CropLayer_getPropStartAxis_const(const cv::dnn::CropLayer* instance) {
		try {
			int ret = instance->startAxis;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_CropLayer_setPropStartAxis_int(cv::dnn::CropLayer* instance, int val) {
		try {
			instance->startAxis = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<int>*> cv_dnn_CropLayer_getPropOffset(cv::dnn::CropLayer* instance) {
		try {
			std::vector<int> ret = instance->offset;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result_void cv_dnn_CropLayer_setPropOffset_vector_int_(cv::dnn::CropLayer* instance, std::vector<int>* val) {
		try {
			instance->offset = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::CropLayer>*> cv_dnn_CropLayer_create_int_const_vector_int_R(int start_axis, const std::vector<int>* offset) {
		try {
			cv::Ptr<cv::dnn::CropLayer> ret = cv::dnn::CropLayer::create(start_axis, *offset);
			return Ok(new cv::Ptr<cv::dnn::CropLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::CropLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*> cv_dnn_DeconvolutionLayer_create_Size_Size_Size_Size(const cv::Size* kernel, const cv::Size* stride, const cv::Size* pad, const cv::Size* dilation) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::DeconvolutionLayer::create(*kernel, *stride, *pad, *dilation);
			return Ok(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*>))
	}
	
	void cv_Dict_delete(cv::dnn::Dict* instance) {
		delete instance;
	}
	Result<bool> cv_dnn_Dict_has_const_const_StringR(const cv::dnn::Dict* instance, const char* key) {
		try {
			bool ret = instance->has(cv::String(key));
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::dnn::DictValue**> cv_dnn_Dict_ptr_const_StringR(cv::dnn::Dict* instance, const char* key) {
		try {
			cv::dnn::DictValue* ret = instance->ptr(cv::String(key));
			return Ok(new cv::dnn::DictValue*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue**>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_Dict_get_const_const_StringR(const cv::dnn::Dict* instance, const char* key) {
		try {
			cv::dnn::DictValue ret = instance->get(cv::String(key));
			return Ok(new cv::dnn::DictValue(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<void*> cv_dnn_Dict_set_cv_String_const_StringR_const_StringR(cv::dnn::Dict* instance, const char* key, const char* value) {
		try {
			cv::String ret = instance->set<cv::String>(cv::String(key), cv::String(value));
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_Dict_set_cv_dnn_DictValue_const_StringR_const_DictValueR(cv::dnn::Dict* instance, const char* key, const cv::dnn::DictValue* value) {
		try {
			cv::dnn::DictValue ret = instance->set<cv::dnn::DictValue>(cv::String(key), *value);
			return Ok(new cv::dnn::DictValue(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<double> cv_dnn_Dict_set_double_const_StringR_const_doubleR(cv::dnn::Dict* instance, const char* key, const double* value) {
		try {
			double ret = instance->set<double>(cv::String(key), *value);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int64_t> cv_dnn_Dict_set_int64_t_const_StringR_const_int64_tR(cv::dnn::Dict* instance, const char* key, const int64_t* value) {
		try {
			int64_t ret = instance->set<int64_t>(cv::String(key), *value);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	void cv_DictValue_delete(cv::dnn::DictValue* instance) {
		delete instance;
	}
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_const_DictValueR(const cv::dnn::DictValue* r) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(*r);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_int64_t(int64_t i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_int(int i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_unsigned_int(unsigned int p) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_double(double p) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_const_charX(const char* s) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(s);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<void*> cv_dnn_DictValue_get_cv_String_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			cv::String ret = instance->get<cv::String>(idx);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<double> cv_dnn_DictValue_get_double_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			double ret = instance->get<double>(idx);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int> cv_dnn_DictValue_get_int_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			int ret = instance->get<int>(idx);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int64_t> cv_dnn_DictValue_get_int64_t_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			int64_t ret = instance->get<int64_t>(idx);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result<int> cv_dnn_DictValue_size_const(const cv::dnn::DictValue* instance) {
		try {
			int ret = instance->size();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_dnn_DictValue_isInt_const(const cv::dnn::DictValue* instance) {
		try {
			bool ret = instance->isInt();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_dnn_DictValue_isString_const(const cv::dnn::DictValue* instance) {
		try {
			bool ret = instance->isString();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_dnn_DictValue_isReal_const(const cv::dnn::DictValue* instance) {
		try {
			bool ret = instance->isReal();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Ptr<cv::dnn::EltwiseLayer>*> cv_dnn_EltwiseLayer_create_EltwiseOp_const_vector_int_R(cv::dnn::EltwiseLayer::EltwiseOp op, const std::vector<int>* coeffs) {
		try {
			cv::Ptr<cv::dnn::EltwiseLayer> ret = cv::dnn::EltwiseLayer::create(op, *coeffs);
			return Ok(new cv::Ptr<cv::dnn::EltwiseLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::EltwiseLayer>*>))
	}
	
	Result_void cv_dnn_Importer_populateNet_Net(cv::dnn::Importer* instance, cv::dnn::Net* net) {
		try {
			instance->populateNet(*net);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_InnerProductLayer_getPropAxis_const(const cv::dnn::InnerProductLayer* instance) {
		try {
			int ret = instance->axis;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_InnerProductLayer_setPropAxis_int(cv::dnn::InnerProductLayer* instance, int val) {
		try {
			instance->axis = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::InnerProductLayer>*> cv_dnn_InnerProductLayer_create_int(int axis) {
		try {
			cv::Ptr<cv::dnn::InnerProductLayer> ret = cv::dnn::InnerProductLayer::create(axis);
			return Ok(new cv::Ptr<cv::dnn::InnerProductLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::InnerProductLayer>*>))
	}
	
	Result<int> cv_dnn_LRNLayer_getPropType_const(const cv::dnn::LRNLayer* instance) {
		try {
			int ret = instance->type;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropType_int(cv::dnn::LRNLayer* instance, int val) {
		try {
			instance->type = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_LRNLayer_getPropSize_const(const cv::dnn::LRNLayer* instance) {
		try {
			int ret = instance->size;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropSize_int(cv::dnn::LRNLayer* instance, int val) {
		try {
			instance->size = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_dnn_LRNLayer_getPropAlpha_const(const cv::dnn::LRNLayer* instance) {
		try {
			double ret = instance->alpha;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropAlpha_double(cv::dnn::LRNLayer* instance, double val) {
		try {
			instance->alpha = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_dnn_LRNLayer_getPropBeta_const(const cv::dnn::LRNLayer* instance) {
		try {
			double ret = instance->beta;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropBeta_double(cv::dnn::LRNLayer* instance, double val) {
		try {
			instance->beta = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_dnn_LRNLayer_getPropBias_const(const cv::dnn::LRNLayer* instance) {
		try {
			double ret = instance->bias;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropBias_double(cv::dnn::LRNLayer* instance, double val) {
		try {
			instance->bias = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_LRNLayer_getPropNormBySize_const(const cv::dnn::LRNLayer* instance) {
		try {
			bool ret = instance->normBySize;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropNormBySize_bool(cv::dnn::LRNLayer* instance, bool val) {
		try {
			instance->normBySize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::LRNLayer>*> cv_dnn_LRNLayer_create_int_int_double_double_double_bool(int type, int size, double alpha, double beta, double bias, bool normBySize) {
		try {
			cv::Ptr<cv::dnn::LRNLayer> ret = cv::dnn::LRNLayer::create(type, size, alpha, beta, bias, normBySize);
			return Ok(new cv::Ptr<cv::dnn::LRNLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::LRNLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::LSTMLayer>*> cv_dnn_LSTMLayer_create() {
		try {
			cv::Ptr<cv::dnn::LSTMLayer> ret = cv::dnn::LSTMLayer::create();
			return Ok(new cv::Ptr<cv::dnn::LSTMLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::LSTMLayer>*>))
	}
	
	Result_void cv_dnn_LSTMLayer_setWeights_const_BlobR_const_BlobR_const_BlobR(cv::dnn::LSTMLayer* instance, const cv::dnn::Blob* Wh, const cv::dnn::Blob* Wx, const cv::dnn::Blob* b) {
		try {
			instance->setWeights(*Wh, *Wx, *b);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_LSTMLayer_setOutShape_const_BlobShapeR(cv::dnn::LSTMLayer* instance, const cv::dnn::BlobShape* outTailShape) {
		try {
			instance->setOutShape(*outTailShape);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_LSTMLayer_setH_const_BlobR(cv::dnn::LSTMLayer* instance, const cv::dnn::Blob* H) {
		try {
			instance->setH(*H);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::dnn::Blob*> cv_dnn_LSTMLayer_getH_const(const cv::dnn::LSTMLayer* instance) {
		try {
			cv::dnn::Blob ret = instance->getH();
			return Ok(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result_void cv_dnn_LSTMLayer_setC_const_BlobR(cv::dnn::LSTMLayer* instance, const cv::dnn::Blob* C) {
		try {
			instance->setC(*C);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::dnn::Blob*> cv_dnn_LSTMLayer_getC_const(const cv::dnn::LSTMLayer* instance) {
		try {
			cv::dnn::Blob ret = instance->getC();
			return Ok(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result_void cv_dnn_LSTMLayer_setUseTimstampsDim_bool(cv::dnn::LSTMLayer* instance, bool use) {
		try {
			instance->setUseTimstampsDim(use);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_LSTMLayer_setProduceCellOutput_bool(cv::dnn::LSTMLayer* instance, bool produce) {
		try {
			instance->setProduceCellOutput(produce);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_LSTMLayer_forward_vector_BlobX_R_vector_Blob_R(cv::dnn::LSTMLayer* instance, std::vector<cv::dnn::Blob*>* input, std::vector<cv::dnn::Blob>* output) {
		try {
			instance->forward(*input, *output);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_LSTMLayer_inputNameToIndex_String(cv::dnn::LSTMLayer* instance, char* inputName) {
		try {
			int ret = instance->inputNameToIndex(cv::String(inputName));
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_LSTMLayer_outputNameToIndex_String(cv::dnn::LSTMLayer* instance, char* outputName) {
		try {
			int ret = instance->outputNameToIndex(cv::String(outputName));
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<std::vector<cv::dnn::Blob>*> cv_dnn_Layer_getPropBlobs(cv::dnn::Layer* instance) {
		try {
			std::vector<cv::dnn::Blob> ret = instance->blobs;
			return Ok(new std::vector<cv::dnn::Blob>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::dnn::Blob>*>))
	}
	
	Result_void cv_dnn_Layer_setPropBlobs_vector_Blob_(cv::dnn::Layer* instance, std::vector<cv::dnn::Blob>* val) {
		try {
			instance->blobs = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_Layer_getPropName_const(const cv::dnn::Layer* instance) {
		try {
			cv::String ret = instance->name;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_Layer_setPropName_String(cv::dnn::Layer* instance, char* val) {
		try {
			instance->name = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_Layer_getPropType_const(const cv::dnn::Layer* instance) {
		try {
			cv::String ret = instance->type;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_Layer_setPropType_String(cv::dnn::Layer* instance, char* val) {
		try {
			instance->type = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Layer_allocate_const_vector_BlobX_R_vector_Blob_R(cv::dnn::Layer* instance, const std::vector<cv::dnn::Blob*>* input, std::vector<cv::dnn::Blob>* output) {
		try {
			instance->allocate(*input, *output);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Layer_forward_vector_BlobX_R_vector_Blob_R(cv::dnn::Layer* instance, std::vector<cv::dnn::Blob*>* input, std::vector<cv::dnn::Blob>* output) {
		try {
			instance->forward(*input, *output);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Layer_allocate_const_vector_Blob_R_vector_Blob_R(cv::dnn::Layer* instance, const std::vector<cv::dnn::Blob>* inputs, std::vector<cv::dnn::Blob>* outputs) {
		try {
			instance->allocate(*inputs, *outputs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::dnn::Blob>*> cv_dnn_Layer_allocate_const_vector_Blob_R(cv::dnn::Layer* instance, const std::vector<cv::dnn::Blob>* inputs) {
		try {
			std::vector<cv::dnn::Blob> ret = instance->allocate(*inputs);
			return Ok(new std::vector<cv::dnn::Blob>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::dnn::Blob>*>))
	}
	
	Result_void cv_dnn_Layer_forward_const_vector_Blob_R_vector_Blob_R(cv::dnn::Layer* instance, const std::vector<cv::dnn::Blob>* inputs, std::vector<cv::dnn::Blob>* outputs) {
		try {
			instance->forward(*inputs, *outputs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Layer_run_const_vector_Blob_R_vector_Blob_R(cv::dnn::Layer* instance, const std::vector<cv::dnn::Blob>* inputs, std::vector<cv::dnn::Blob>* outputs) {
		try {
			instance->run(*inputs, *outputs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_Layer_inputNameToIndex_String(cv::dnn::Layer* instance, char* inputName) {
		try {
			int ret = instance->inputNameToIndex(cv::String(inputName));
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Layer_outputNameToIndex_String(cv::dnn::Layer* instance, char* outputName) {
		try {
			int ret = instance->outputNameToIndex(cv::String(outputName));
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_Layer_setParamsFrom_const_LayerParamsR(cv::dnn::Layer* instance, const cv::dnn::LayerParams* params) {
		try {
			instance->setParamsFrom(*params);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_LayerFactory_delete(cv::dnn::LayerFactory* instance) {
		delete instance;
	}
	Result_void cv_dnn_LayerFactory_registerLayer_const_StringR_Constuctor(const char* type, cv::dnn::LayerFactory::Constuctor constructor) {
		try {
			cv::dnn::LayerFactory::registerLayer(cv::String(type), constructor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_LayerFactory_unregisterLayer_const_StringR(const char* type) {
		try {
			cv::dnn::LayerFactory::unregisterLayer(cv::String(type));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_LayerFactory_createLayerInstance_const_StringR_LayerParamsR(const char* type, cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::LayerFactory::createLayerInstance(cv::String(type), *params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	Result<std::vector<cv::dnn::Blob>*> cv_dnn_LayerParams_getPropBlobs(cv::dnn::LayerParams* instance) {
		try {
			std::vector<cv::dnn::Blob> ret = instance->blobs;
			return Ok(new std::vector<cv::dnn::Blob>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::dnn::Blob>*>))
	}
	
	Result_void cv_dnn_LayerParams_setPropBlobs_vector_Blob_(cv::dnn::LayerParams* instance, std::vector<cv::dnn::Blob>* val) {
		try {
			instance->blobs = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_LayerParams_getPropName_const(const cv::dnn::LayerParams* instance) {
		try {
			cv::String ret = instance->name;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_LayerParams_setPropName_String(cv::dnn::LayerParams* instance, char* val) {
		try {
			instance->name = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_LayerParams_getPropType_const(const cv::dnn::LayerParams* instance) {
		try {
			cv::String ret = instance->type;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_LayerParams_setPropType_String(cv::dnn::LayerParams* instance, char* val) {
		try {
			instance->type = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_LayerParams_delete(cv::dnn::LayerParams* instance) {
		delete instance;
	}
	Result<double> cv_dnn_MVNLayer_getPropEps_const(const cv::dnn::MVNLayer* instance) {
		try {
			double ret = instance->eps;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_dnn_MVNLayer_setPropEps_double(cv::dnn::MVNLayer* instance, double val) {
		try {
			instance->eps = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_MVNLayer_getPropNormVariance_const(const cv::dnn::MVNLayer* instance) {
		try {
			bool ret = instance->normVariance;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_MVNLayer_setPropNormVariance_bool(cv::dnn::MVNLayer* instance, bool val) {
		try {
			instance->normVariance = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_MVNLayer_getPropAcrossChannels_const(const cv::dnn::MVNLayer* instance) {
		try {
			bool ret = instance->acrossChannels;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_MVNLayer_setPropAcrossChannels_bool(cv::dnn::MVNLayer* instance, bool val) {
		try {
			instance->acrossChannels = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::MVNLayer>*> cv_dnn_MVNLayer_create_bool_bool_double(bool normVariance, bool acrossChannels, double eps) {
		try {
			cv::Ptr<cv::dnn::MVNLayer> ret = cv::dnn::MVNLayer::create(normVariance, acrossChannels, eps);
			return Ok(new cv::Ptr<cv::dnn::MVNLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::MVNLayer>*>))
	}
	
	void cv_Net_delete(cv::dnn::Net* instance) {
		delete instance;
	}
	Result<cv::dnn::Net*> cv_dnn_Net_Net() {
		try {
			cv::dnn::Net* ret = new cv::dnn::Net();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<bool> cv_dnn_Net_empty_const(const cv::dnn::Net* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_dnn_Net_addLayer_const_StringR_const_StringR_LayerParamsR(cv::dnn::Net* instance, const char* name, const char* type, cv::dnn::LayerParams* params) {
		try {
			int ret = instance->addLayer(cv::String(name), cv::String(type), *params);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_LayerParamsR(cv::dnn::Net* instance, const char* name, const char* type, cv::dnn::LayerParams* params) {
		try {
			int ret = instance->addLayerToPrev(cv::String(name), cv::String(type), *params);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Net_getLayerId_const_StringR(cv::dnn::Net* instance, const char* layer) {
		try {
			int ret = instance->getLayerId(cv::String(layer));
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<std::vector<cv::String>*> cv_dnn_Net_getLayerNames_const(const cv::dnn::Net* instance) {
		try {
			std::vector<cv::String> ret = instance->getLayerNames();
			return Ok(new std::vector<cv::String>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::String>*>))
	}
	
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_Net_getLayer_LayerId(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layerId) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = instance->getLayer(*layerId);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	Result_void cv_dnn_Net_deleteLayer_LayerId(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layer) {
		try {
			instance->deleteLayer(*layer);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_connect_String_String(cv::dnn::Net* instance, char* outPin, char* inpPin) {
		try {
			instance->connect(cv::String(outPin), cv::String(inpPin));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_connect_int_int_int_int(cv::dnn::Net* instance, int outLayerId, int outNum, int inpLayerId, int inpNum) {
		try {
			instance->connect(outLayerId, outNum, inpLayerId, inpNum);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_setNetInputs_const_vector_String_R(cv::dnn::Net* instance, const std::vector<cv::String>* inputBlobNames) {
		try {
			instance->setNetInputs(*inputBlobNames);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_allocate(cv::dnn::Net* instance) {
		try {
			instance->allocate();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_forward_LayerId(cv::dnn::Net* instance, cv::dnn::Net::LayerId* toLayer) {
		try {
			instance->forward(*toLayer);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_forward_LayerId_LayerId(cv::dnn::Net* instance, cv::dnn::Net::LayerId* startLayer, cv::dnn::Net::LayerId* toLayer) {
		try {
			instance->forward(*startLayer, *toLayer);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_forward_const_vector_LayerId_R_const_vector_LayerId_R(cv::dnn::Net* instance, const std::vector<cv::dnn::Net::LayerId>* startLayers, const std::vector<cv::dnn::Net::LayerId>* toLayers) {
		try {
			instance->forward(*startLayers, *toLayers);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_forwardOpt_LayerId(cv::dnn::Net* instance, cv::dnn::Net::LayerId* toLayer) {
		try {
			instance->forwardOpt(*toLayer);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_forwardOpt_const_vector_LayerId_R(cv::dnn::Net* instance, const std::vector<cv::dnn::Net::LayerId>* toLayers) {
		try {
			instance->forwardOpt(*toLayers);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_setBlob_String_const_BlobR(cv::dnn::Net* instance, char* outputName, const cv::dnn::Blob* blob) {
		try {
			instance->setBlob(cv::String(outputName), *blob);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::dnn::Blob*> cv_dnn_Net_getBlob_String(cv::dnn::Net* instance, char* outputName) {
		try {
			cv::dnn::Blob ret = instance->getBlob(cv::String(outputName));
			return Ok(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result_void cv_dnn_Net_setParam_LayerId_int_const_BlobR(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layer, int numParam, const cv::dnn::Blob* blob) {
		try {
			instance->setParam(*layer, numParam, *blob);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::dnn::Blob*> cv_dnn_Net_getParam_LayerId_int(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layer, int numParam) {
		try {
			cv::dnn::Blob ret = instance->getParam(*layer, numParam);
			return Ok(new cv::dnn::Blob(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Blob*>))
	}
	
	Result<int> cv_dnn_PoolingLayer_getPropType_const(const cv::dnn::PoolingLayer* instance) {
		try {
			int ret = instance->type;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropType_int(cv::dnn::PoolingLayer* instance, int val) {
		try {
			instance->type = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_getPropKernel_const(const cv::dnn::PoolingLayer* instance) {
		try {
			cv::Size ret = instance->kernel;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropKernel_Size(cv::dnn::PoolingLayer* instance, const cv::Size* val) {
		try {
			instance->kernel = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_getPropStride_const(const cv::dnn::PoolingLayer* instance) {
		try {
			cv::Size ret = instance->stride;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropStride_Size(cv::dnn::PoolingLayer* instance, const cv::Size* val) {
		try {
			instance->stride = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_getPropPad_const(const cv::dnn::PoolingLayer* instance) {
		try {
			cv::Size ret = instance->pad;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropPad_Size(cv::dnn::PoolingLayer* instance, const cv::Size* val) {
		try {
			instance->pad = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_PoolingLayer_getPropGlobalPooling_const(const cv::dnn::PoolingLayer* instance) {
		try {
			bool ret = instance->globalPooling;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropGlobalPooling_bool(cv::dnn::PoolingLayer* instance, bool val) {
		try {
			instance->globalPooling = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_PoolingLayer_getPropPadMode_const(const cv::dnn::PoolingLayer* instance) {
		try {
			cv::String ret = instance->padMode;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropPadMode_String(cv::dnn::PoolingLayer* instance, char* val) {
		try {
			instance->padMode = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::PoolingLayer>*> cv_dnn_PoolingLayer_create_int_Size_Size_Size_const_StringR(int type, const cv::Size* kernel, const cv::Size* stride, const cv::Size* pad, const char* padMode) {
		try {
			cv::Ptr<cv::dnn::PoolingLayer> ret = cv::dnn::PoolingLayer::create(type, *kernel, *stride, *pad, cv::String(padMode));
			return Ok(new cv::Ptr<cv::dnn::PoolingLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::PoolingLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::PoolingLayer>*> cv_dnn_PoolingLayer_createGlobal_int(int type) {
		try {
			cv::Ptr<cv::dnn::PoolingLayer> ret = cv::dnn::PoolingLayer::createGlobal(type);
			return Ok(new cv::Ptr<cv::dnn::PoolingLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::PoolingLayer>*>))
	}
	
	Result<double> cv_dnn_PowerLayer_getPropPower_const(const cv::dnn::PowerLayer* instance) {
		try {
			double ret = instance->power;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_dnn_PowerLayer_setPropPower_double(cv::dnn::PowerLayer* instance, double val) {
		try {
			instance->power = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_dnn_PowerLayer_getPropScale_const(const cv::dnn::PowerLayer* instance) {
		try {
			double ret = instance->scale;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_dnn_PowerLayer_setPropScale_double(cv::dnn::PowerLayer* instance, double val) {
		try {
			instance->scale = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_dnn_PowerLayer_getPropShift_const(const cv::dnn::PowerLayer* instance) {
		try {
			double ret = instance->shift;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_dnn_PowerLayer_setPropShift_double(cv::dnn::PowerLayer* instance, double val) {
		try {
			instance->shift = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::PowerLayer>*> cv_dnn_PowerLayer_create_double_double_double(double power, double scale, double shift) {
		try {
			cv::Ptr<cv::dnn::PowerLayer> ret = cv::dnn::PowerLayer::create(power, scale, shift);
			return Ok(new cv::Ptr<cv::dnn::PowerLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::PowerLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::RNNLayer>*> cv_dnn_RNNLayer_create() {
		try {
			cv::Ptr<cv::dnn::RNNLayer> ret = cv::dnn::RNNLayer::create();
			return Ok(new cv::Ptr<cv::dnn::RNNLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::RNNLayer>*>))
	}
	
	Result_void cv_dnn_RNNLayer_setWeights_const_BlobR_const_BlobR_const_BlobR_const_BlobR_const_BlobR(cv::dnn::RNNLayer* instance, const cv::dnn::Blob* Wxh, const cv::dnn::Blob* bh, const cv::dnn::Blob* Whh, const cv::dnn::Blob* Who, const cv::dnn::Blob* bo) {
		try {
			instance->setWeights(*Wxh, *bh, *Whh, *Who, *bo);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_RNNLayer_setProduceHiddenOutput_bool(cv::dnn::RNNLayer* instance, bool produce) {
		try {
			instance->setProduceHiddenOutput(produce);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_RNNLayer_forward_vector_BlobX_R_vector_Blob_R(cv::dnn::RNNLayer* instance, std::vector<cv::dnn::Blob*>* input, std::vector<cv::dnn::Blob>* output) {
		try {
			instance->forward(*input, *output);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_dnn_ReLULayer_getPropNegativeSlope_const(const cv::dnn::ReLULayer* instance) {
		try {
			double ret = instance->negativeSlope;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_dnn_ReLULayer_setPropNegativeSlope_double(cv::dnn::ReLULayer* instance, double val) {
		try {
			instance->negativeSlope = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::ReLULayer>*> cv_dnn_ReLULayer_create_double(double negativeSlope) {
		try {
			cv::Ptr<cv::dnn::ReLULayer> ret = cv::dnn::ReLULayer::create(negativeSlope);
			return Ok(new cv::Ptr<cv::dnn::ReLULayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ReLULayer>*>))
	}
	
	Result<cv::dnn::BlobShape*> cv_dnn_ReshapeLayer_getPropNewShapeDesc(cv::dnn::ReshapeLayer* instance) {
		try {
			cv::dnn::BlobShape ret = instance->newShapeDesc;
			return Ok(new cv::dnn::BlobShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::BlobShape*>))
	}
	
	Result_void cv_dnn_ReshapeLayer_setPropNewShapeDesc_BlobShape(cv::dnn::ReshapeLayer* instance, cv::dnn::BlobShape* val) {
		try {
			instance->newShapeDesc = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Range*> cv_dnn_ReshapeLayer_getPropNewShapeRange(cv::dnn::ReshapeLayer* instance) {
		try {
			cv::Range ret = instance->newShapeRange;
			return Ok(new cv::Range(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Range*>))
	}
	
	Result_void cv_dnn_ReshapeLayer_setPropNewShapeRange_Range(cv::dnn::ReshapeLayer* instance, cv::Range* val) {
		try {
			instance->newShapeRange = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::ReshapeLayer>*> cv_dnn_ReshapeLayer_create_const_BlobShapeR_Range_bool(const cv::dnn::BlobShape* newShape, cv::Range* applyingRange, bool enableReordering) {
		try {
			cv::Ptr<cv::dnn::ReshapeLayer> ret = cv::dnn::ReshapeLayer::create(*newShape, *applyingRange, enableReordering);
			return Ok(new cv::Ptr<cv::dnn::ReshapeLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ReshapeLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::SigmoidLayer>*> cv_dnn_SigmoidLayer_create() {
		try {
			cv::Ptr<cv::dnn::SigmoidLayer> ret = cv::dnn::SigmoidLayer::create();
			return Ok(new cv::Ptr<cv::dnn::SigmoidLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::SigmoidLayer>*>))
	}
	
	Result<int> cv_dnn_SliceLayer_getPropAxis_const(const cv::dnn::SliceLayer* instance) {
		try {
			int ret = instance->axis;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_SliceLayer_setPropAxis_int(cv::dnn::SliceLayer* instance, int val) {
		try {
			instance->axis = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<int>*> cv_dnn_SliceLayer_getPropSliceIndices(cv::dnn::SliceLayer* instance) {
		try {
			std::vector<int> ret = instance->sliceIndices;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result_void cv_dnn_SliceLayer_setPropSliceIndices_vector_int_(cv::dnn::SliceLayer* instance, std::vector<int>* val) {
		try {
			instance->sliceIndices = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::SliceLayer>*> cv_dnn_SliceLayer_create_int(int axis) {
		try {
			cv::Ptr<cv::dnn::SliceLayer> ret = cv::dnn::SliceLayer::create(axis);
			return Ok(new cv::Ptr<cv::dnn::SliceLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::SliceLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::SliceLayer>*> cv_dnn_SliceLayer_create_int_const_vector_int_R(int axis, const std::vector<int>* sliceIndices) {
		try {
			cv::Ptr<cv::dnn::SliceLayer> ret = cv::dnn::SliceLayer::create(axis, *sliceIndices);
			return Ok(new cv::Ptr<cv::dnn::SliceLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::SliceLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::SoftmaxLayer>*> cv_dnn_SoftmaxLayer_create_int(int axis) {
		try {
			cv::Ptr<cv::dnn::SoftmaxLayer> ret = cv::dnn::SoftmaxLayer::create(axis);
			return Ok(new cv::Ptr<cv::dnn::SoftmaxLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::SoftmaxLayer>*>))
	}
	
	Result<int> cv_dnn_SplitLayer_getPropOutputsCount_const(const cv::dnn::SplitLayer* instance) {
		try {
			int ret = instance->outputsCount;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_SplitLayer_setPropOutputsCount_int(cv::dnn::SplitLayer* instance, int val) {
		try {
			instance->outputsCount = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::SplitLayer>*> cv_dnn_SplitLayer_create_int(int outputsCount) {
		try {
			cv::Ptr<cv::dnn::SplitLayer> ret = cv::dnn::SplitLayer::create(outputsCount);
			return Ok(new cv::Ptr<cv::dnn::SplitLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::SplitLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::TanHLayer>*> cv_dnn_TanHLayer_create() {
		try {
			cv::Ptr<cv::dnn::TanHLayer> ret = cv::dnn::TanHLayer::create();
			return Ok(new cv::Ptr<cv::dnn::TanHLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::TanHLayer>*>))
	}
	
	void cv__Range_delete(cv::dnn::_Range* instance) {
		delete instance;
	}
	Result<cv::dnn::_Range*> cv_dnn__Range__Range_const_RangeR(const cv::Range* r) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(*r);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::_Range*>))
	}
	
	Result<cv::dnn::_Range*> cv_dnn__Range__Range_int_int(int start, int size) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(start, size);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::_Range*>))
	}
	
}

#include "hdf.hpp"
#include "hdf_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::hdf::HDF5>*> cv_hdf_open_String(char* HDF5Filename) {
		try {
			cv::Ptr<cv::hdf::HDF5> ret = cv::hdf::open(cv::String(HDF5Filename));
			return Ok(new cv::Ptr<cv::hdf::HDF5>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::hdf::HDF5>*>)
	}
	
	Result_void cv_hdf_HDF5_close(cv::hdf::HDF5* instance) {
		try {
			instance->close();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_grcreate_String(cv::hdf::HDF5* instance, char* grlabel) {
		try {
			instance->grcreate(cv::String(grlabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_hdf_HDF5_hlexists_const_String(const cv::hdf::HDF5* instance, char* label) {
		try {
			bool ret = instance->hlexists(cv::String(label));
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_int_int_String(const cv::hdf::HDF5* instance, int rows, int cols, int type, char* dslabel) {
		try {
			instance->dscreate(rows, cols, type, cv::String(dslabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_int_int_String_int(const cv::hdf::HDF5* instance, int rows, int cols, int type, char* dslabel, int compresslevel) {
		try {
			instance->dscreate(rows, cols, type, cv::String(dslabel), compresslevel);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_int_int_String_int_const_vector_int_X(const cv::hdf::HDF5* instance, int rows, int cols, int type, char* dslabel, int compresslevel, const std::vector<int>* dims_chunks) {
		try {
			instance->dscreate(rows, cols, type, cv::String(dslabel), compresslevel, *dims_chunks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_int_int_String_int_const_intX(const cv::hdf::HDF5* instance, int rows, int cols, int type, char* dslabel, int compresslevel, const int* dims_chunks) {
		try {
			instance->dscreate(rows, cols, type, cv::String(dslabel), compresslevel, dims_chunks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_const_intX_int_String(const cv::hdf::HDF5* instance, int n_dims, const int* sizes, int type, char* dslabel) {
		try {
			instance->dscreate(n_dims, sizes, type, cv::String(dslabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_const_intX_int_String_int(const cv::hdf::HDF5* instance, int n_dims, const int* sizes, int type, char* dslabel, int compresslevel) {
		try {
			instance->dscreate(n_dims, sizes, type, cv::String(dslabel), compresslevel);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_const_vector_int_X_int_String_int_const_vector_int_X(const cv::hdf::HDF5* instance, const std::vector<int>* sizes, int type, char* dslabel, int compresslevel, const std::vector<int>* dims_chunks) {
		try {
			instance->dscreate(*sizes, type, cv::String(dslabel), compresslevel, *dims_chunks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_const_intX_int_String_int_const_intX(const cv::hdf::HDF5* instance, int n_dims, const int* sizes, int type, char* dslabel, int compresslevel, const int* dims_chunks) {
		try {
			instance->dscreate(n_dims, sizes, type, cv::String(dslabel), compresslevel, dims_chunks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<int>*> cv_hdf_HDF5_dsgetsize_const_String_int(const cv::hdf::HDF5* instance, char* dslabel, int dims_flag) {
		try {
			std::vector<int> ret = instance->dsgetsize(cv::String(dslabel), dims_flag);
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result<int> cv_hdf_HDF5_dsgettype_const_String(const cv::hdf::HDF5* instance, char* dslabel) {
		try {
			int ret = instance->dsgettype(cv::String(dslabel));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_hdf_HDF5_dswrite_const_const__InputArrayX_String(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, char* dslabel) {
		try {
			instance->dswrite(*Array, cv::String(dslabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dswrite_const_const__InputArrayX_String_const_intX(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, char* dslabel, const int* dims_offset) {
		try {
			instance->dswrite(*Array, cv::String(dslabel), dims_offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dswrite_const_const__InputArrayX_String_const_vector_int_X_const_vector_int_X(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, char* dslabel, const std::vector<int>* dims_offset, const std::vector<int>* dims_counts) {
		try {
			instance->dswrite(*Array, cv::String(dslabel), *dims_offset, *dims_counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dswrite_const_const__InputArrayX_String_const_intX_const_intX(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, char* dslabel, const int* dims_offset, const int* dims_counts) {
		try {
			instance->dswrite(*Array, cv::String(dslabel), dims_offset, dims_counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsinsert_const_const__InputArrayX_String(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, char* dslabel) {
		try {
			instance->dsinsert(*Array, cv::String(dslabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsinsert_const_const__InputArrayX_String_const_intX(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, char* dslabel, const int* dims_offset) {
		try {
			instance->dsinsert(*Array, cv::String(dslabel), dims_offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsinsert_const_const__InputArrayX_String_const_vector_int_X_const_vector_int_X(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, char* dslabel, const std::vector<int>* dims_offset, const std::vector<int>* dims_counts) {
		try {
			instance->dsinsert(*Array, cv::String(dslabel), *dims_offset, *dims_counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsinsert_const_const__InputArrayX_String_const_intX_const_intX(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, char* dslabel, const int* dims_offset, const int* dims_counts) {
		try {
			instance->dsinsert(*Array, cv::String(dslabel), dims_offset, dims_counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsread_const_const__OutputArrayX_String(const cv::hdf::HDF5* instance, const cv::_OutputArray* Array, char* dslabel) {
		try {
			instance->dsread(*Array, cv::String(dslabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsread_const_const__OutputArrayX_String_const_intX(const cv::hdf::HDF5* instance, const cv::_OutputArray* Array, char* dslabel, const int* dims_offset) {
		try {
			instance->dsread(*Array, cv::String(dslabel), dims_offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsread_const_const__OutputArrayX_String_const_vector_int_X_const_vector_int_X(const cv::hdf::HDF5* instance, const cv::_OutputArray* Array, char* dslabel, const std::vector<int>* dims_offset, const std::vector<int>* dims_counts) {
		try {
			instance->dsread(*Array, cv::String(dslabel), *dims_offset, *dims_counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsread_const_const__OutputArrayX_String_const_intX_const_intX(const cv::hdf::HDF5* instance, const cv::_OutputArray* Array, char* dslabel, const int* dims_offset, const int* dims_counts) {
		try {
			instance->dsread(*Array, cv::String(dslabel), dims_offset, dims_counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_hdf_HDF5_kpgetsize_const_String_int(const cv::hdf::HDF5* instance, char* kplabel, int dims_flag) {
		try {
			int ret = instance->kpgetsize(cv::String(kplabel), dims_flag);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_hdf_HDF5_kpcreate_const_int_String_int_int(const cv::hdf::HDF5* instance, int size, char* kplabel, int compresslevel, int chunks) {
		try {
			instance->kpcreate(size, cv::String(kplabel), compresslevel, chunks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_kpwrite_const_vector_KeyPoint__String_int_int(const cv::hdf::HDF5* instance, const std::vector<cv::KeyPoint>* keypoints, char* kplabel, int offset, int counts) {
		try {
			instance->kpwrite(*keypoints, cv::String(kplabel), offset, counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_kpinsert_const_vector_KeyPoint__String_int_int(const cv::hdf::HDF5* instance, const std::vector<cv::KeyPoint>* keypoints, char* kplabel, int offset, int counts) {
		try {
			instance->kpinsert(*keypoints, cv::String(kplabel), offset, counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_kpread_const_vector_KeyPoint_X_String_int_int(const cv::hdf::HDF5* instance, std::vector<cv::KeyPoint>* keypoints, char* kplabel, int offset, int counts) {
		try {
			instance->kpread(*keypoints, cv::String(kplabel), offset, counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}

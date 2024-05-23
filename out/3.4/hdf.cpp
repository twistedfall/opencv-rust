#include "hdf.hpp"
#include "hdf_types.hpp"

extern "C" {
	// open(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:802
	// ("cv::hdf::open", vec![(pred!(mut, ["HDF5Filename"], ["const cv::String*"]), _)]),
	void cv_hdf_open_const_StringR(const char* HDF5Filename, Result<cv::Ptr<cv::hdf::HDF5>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::hdf::HDF5> ret = cv::hdf::open(cv::String(HDF5Filename));
			Ok(new cv::Ptr<cv::hdf::HDF5>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// close()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:73
	// ("cv::hdf::HDF5::close", vec![(pred!(mut, [], []), _)]),
	void cv_hdf_HDF5_close(cv::hdf::HDF5* instance, ResultVoid* ocvrs_return) {
		try {
			instance->close();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// grcreate(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:95
	// ("cv::hdf::HDF5::grcreate", vec![(pred!(mut, ["grlabel"], ["const cv::String*"]), _)]),
	void cv_hdf_HDF5_grcreate_const_StringR(cv::hdf::HDF5* instance, const char* grlabel, ResultVoid* ocvrs_return) {
		try {
			instance->grcreate(cv::String(grlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hlexists(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:104
	// ("cv::hdf::HDF5::hlexists", vec![(pred!(const, ["label"], ["const cv::String*"]), _)]),
	void cv_hdf_HDF5_hlexists_const_const_StringR(const cv::hdf::HDF5* instance, const char* label, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->hlexists(cv::String(label));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// atexists(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:114
	// ("cv::hdf::HDF5::atexists", vec![(pred!(const, ["atlabel"], ["const cv::String*"]), _)]),
	void cv_hdf_HDF5_atexists_const_const_StringR(const cv::hdf::HDF5* instance, const char* atlabel, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->atexists(cv::String(atlabel));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// atdelete(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:126
	// ("cv::hdf::HDF5::atdelete", vec![(pred!(mut, ["atlabel"], ["const cv::String*"]), _)]),
	void cv_hdf_HDF5_atdelete_const_StringR(cv::hdf::HDF5* instance, const char* atlabel, ResultVoid* ocvrs_return) {
		try {
			instance->atdelete(cv::String(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// atwrite(const int, const String &)(Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:144
	// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const int", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_atwrite_const_int_const_StringR(cv::hdf::HDF5* instance, const int value, const char* atlabel, ResultVoid* ocvrs_return) {
		try {
			instance->atwrite(value, cv::String(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// atread(int *, const String &)(Indirect, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:161
	// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["int*", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_atread_intX_const_StringR(cv::hdf::HDF5* instance, int* value, const char* atlabel, ResultVoid* ocvrs_return) {
		try {
			instance->atread(value, cv::String(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// atwrite(const double, const String &)(Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:164
	// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const double", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_atwrite_const_double_const_StringR(cv::hdf::HDF5* instance, const double value, const char* atlabel, ResultVoid* ocvrs_return) {
		try {
			instance->atwrite(value, cv::String(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// atread(double *, const String &)(Indirect, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:167
	// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["double*", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_atread_doubleX_const_StringR(cv::hdf::HDF5* instance, double* value, const char* atlabel, ResultVoid* ocvrs_return) {
		try {
			instance->atread(value, cv::String(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// atwrite(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:170
	// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_atwrite_const_StringR_const_StringR(cv::hdf::HDF5* instance, const char* value, const char* atlabel, ResultVoid* ocvrs_return) {
		try {
			instance->atwrite(cv::String(value), cv::String(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// atread(String *, const String &)(OutString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:173
	// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["cv::String*", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_atread_StringX_const_StringR(cv::hdf::HDF5* instance, void** value, const char* atlabel, ResultVoid* ocvrs_return) {
		try {
			cv::String value_out;
			instance->atread(&value_out, cv::String(atlabel));
			*value = ocvrs_create_string(value_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// atwrite(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:187
	// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_atwrite_const__InputArrayR_const_StringR(cv::hdf::HDF5* instance, const cv::_InputArray* value, const char* atlabel, ResultVoid* ocvrs_return) {
		try {
			instance->atwrite(*value, cv::String(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// atread(OutputArray, const String &)(OutputArray, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:200
	// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["const cv::_OutputArray*", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_atread_const__OutputArrayR_const_StringR(cv::hdf::HDF5* instance, const cv::_OutputArray* value, const char* atlabel, ResultVoid* ocvrs_return) {
		try {
			instance->atread(*value, cv::String(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dscreate(const int, const int, const int, const String &)(Primitive, Primitive, Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:203
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["rows", "cols", "type", "dslabel"], ["const int", "const int", "const int", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR(const cv::hdf::HDF5* instance, const int rows, const int cols, const int type, const char* dslabel, ResultVoid* ocvrs_return) {
		try {
			instance->dscreate(rows, cols, type, cv::String(dslabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dscreate(const int, const int, const int, const String &, const int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:206
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["rows", "cols", "type", "dslabel", "compresslevel"], ["const int", "const int", "const int", "const cv::String*", "const int"]), _)]),
	void cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int(const cv::hdf::HDF5* instance, const int rows, const int cols, const int type, const char* dslabel, const int compresslevel, ResultVoid* ocvrs_return) {
		try {
			instance->dscreate(rows, cols, type, cv::String(dslabel), compresslevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dscreate(const int, const int, const int, const String &, const int, const vector<int> &)(Primitive, Primitive, Primitive, InString, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:209
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["rows", "cols", "type", "dslabel", "compresslevel", "dims_chunks"], ["const int", "const int", "const int", "const cv::String*", "const int", "const std::vector<int>*"]), _)]),
	void cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int_const_vectorLintGR(const cv::hdf::HDF5* instance, const int rows, const int cols, const int type, const char* dslabel, const int compresslevel, const std::vector<int>* dims_chunks, ResultVoid* ocvrs_return) {
		try {
			instance->dscreate(rows, cols, type, cv::String(dslabel), compresslevel, *dims_chunks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dscreate(const int, const int *, const int, const String &)(Primitive, VariableArray, Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:282
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["n_dims", "sizes", "type", "dslabel"], ["const int", "const int*", "const int", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR(const cv::hdf::HDF5* instance, const int n_dims, const int* sizes, const int type, const char* dslabel, ResultVoid* ocvrs_return) {
		try {
			instance->dscreate(n_dims, sizes, type, cv::String(dslabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dscreate(const int, const int *, const int, const String &, const int)(Primitive, VariableArray, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:285
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["n_dims", "sizes", "type", "dslabel", "compresslevel"], ["const int", "const int*", "const int", "const cv::String*", "const int"]), _)]),
	void cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int(const cv::hdf::HDF5* instance, const int n_dims, const int* sizes, const int type, const char* dslabel, const int compresslevel, ResultVoid* ocvrs_return) {
		try {
			instance->dscreate(n_dims, sizes, type, cv::String(dslabel), compresslevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dscreate(const vector<int> &, const int, const String &, const int, const vector<int> &)(CppPassByVoidPtr, Primitive, InString, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:288
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["sizes", "type", "dslabel", "compresslevel", "dims_chunks"], ["const std::vector<int>*", "const int", "const cv::String*", "const int", "const std::vector<int>*"]), _)]),
	void cv_hdf_HDF5_dscreate_const_const_vectorLintGR_const_int_const_StringR_const_int_const_vectorLintGR(const cv::hdf::HDF5* instance, const std::vector<int>* sizes, const int type, const char* dslabel, const int compresslevel, const std::vector<int>* dims_chunks, ResultVoid* ocvrs_return) {
		try {
			instance->dscreate(*sizes, type, cv::String(dslabel), compresslevel, *dims_chunks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hdf::HDF5::dscreate(CppPassByVoidPtr, Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:288
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["sizes", "type", "dslabel"], ["const std::vector<int>*", "const int", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_dscreate_const_const_vectorLintGR_const_int_const_StringR(const cv::hdf::HDF5* instance, const std::vector<int>* sizes, const int type, const char* dslabel, ResultVoid* ocvrs_return) {
		try {
			instance->dscreate(*sizes, type, cv::String(dslabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dscreate(const int, const int *, const int, const String &, const int, const int *)(Primitive, VariableArray, Primitive, InString, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:362
	// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["n_dims", "sizes", "type", "dslabel", "compresslevel", "dims_chunks"], ["const int", "const int*", "const int", "const cv::String*", "const int", "const int*"]), _)]),
	void cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int_const_intX(const cv::hdf::HDF5* instance, const int n_dims, const int* sizes, const int type, const char* dslabel, const int compresslevel, const int* dims_chunks, ResultVoid* ocvrs_return) {
		try {
			instance->dscreate(n_dims, sizes, type, cv::String(dslabel), compresslevel, dims_chunks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dsgetsize(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:380
	// ("cv::hdf::HDF5::dsgetsize", vec![(pred!(const, ["dslabel", "dims_flag"], ["const cv::String*", "int"]), _)]),
	void cv_hdf_HDF5_dsgetsize_const_const_StringR_int(const cv::hdf::HDF5* instance, const char* dslabel, int dims_flag, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->dsgetsize(cv::String(dslabel), dims_flag);
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hdf::HDF5::dsgetsize(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:380
	// ("cv::hdf::HDF5::dsgetsize", vec![(pred!(const, ["dslabel"], ["const cv::String*"]), _)]),
	void cv_hdf_HDF5_dsgetsize_const_const_StringR(const cv::hdf::HDF5* instance, const char* dslabel, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->dsgetsize(cv::String(dslabel));
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dsgettype(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:391
	// ("cv::hdf::HDF5::dsgettype", vec![(pred!(const, ["dslabel"], ["const cv::String*"]), _)]),
	void cv_hdf_HDF5_dsgettype_const_const_StringR(const cv::hdf::HDF5* instance, const char* dslabel, Result<int>* ocvrs_return) {
		try {
			int ret = instance->dsgettype(cv::String(dslabel));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dswrite(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:394
	// ("cv::hdf::HDF5::dswrite", vec![(pred!(const, ["Array", "dslabel"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, ResultVoid* ocvrs_return) {
		try {
			instance->dswrite(*Array, cv::String(dslabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dswrite(InputArray, const String &, const vector<int> &, const vector<int> &)(InputArray, InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:399
	// ("cv::hdf::HDF5::dswrite", vec![(pred!(const, ["Array", "dslabel", "dims_offset", "dims_counts"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*", "const std::vector<int>*"]), _)]),
	void cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_vectorLintGR_const_vectorLintGR(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, const std::vector<int>* dims_offset, const std::vector<int>* dims_counts, ResultVoid* ocvrs_return) {
		try {
			instance->dswrite(*Array, cv::String(dslabel), *dims_offset, *dims_counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hdf::HDF5::dswrite(InputArray, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:399
	// ("cv::hdf::HDF5::dswrite", vec![(pred!(const, ["Array", "dslabel", "dims_offset"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*"]), _)]),
	void cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_vectorLintGR(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, const std::vector<int>* dims_offset, ResultVoid* ocvrs_return) {
		try {
			instance->dswrite(*Array, cv::String(dslabel), *dims_offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dsinsert(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:468
	// ("cv::hdf::HDF5::dsinsert", vec![(pred!(const, ["Array", "dslabel"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, ResultVoid* ocvrs_return) {
		try {
			instance->dsinsert(*Array, cv::String(dslabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dsinsert(InputArray, const String &, const vector<int> &, const vector<int> &)(InputArray, InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:473
	// ("cv::hdf::HDF5::dsinsert", vec![(pred!(const, ["Array", "dslabel", "dims_offset", "dims_counts"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*", "const std::vector<int>*"]), _)]),
	void cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_vectorLintGR_const_vectorLintGR(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, const std::vector<int>* dims_offset, const std::vector<int>* dims_counts, ResultVoid* ocvrs_return) {
		try {
			instance->dsinsert(*Array, cv::String(dslabel), *dims_offset, *dims_counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hdf::HDF5::dsinsert(InputArray, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:473
	// ("cv::hdf::HDF5::dsinsert", vec![(pred!(const, ["Array", "dslabel", "dims_offset"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*"]), _)]),
	void cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_vectorLintGR(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, const std::vector<int>* dims_offset, ResultVoid* ocvrs_return) {
		try {
			instance->dsinsert(*Array, cv::String(dslabel), *dims_offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dsread(OutputArray, const String &)(OutputArray, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:528
	// ("cv::hdf::HDF5::dsread", vec![(pred!(const, ["Array", "dslabel"], ["const cv::_OutputArray*", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR(const cv::hdf::HDF5* instance, const cv::_OutputArray* Array, const char* dslabel, ResultVoid* ocvrs_return) {
		try {
			instance->dsread(*Array, cv::String(dslabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dsread(OutputArray, const String &, const vector<int> &, const vector<int> &)(OutputArray, InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:533
	// ("cv::hdf::HDF5::dsread", vec![(pred!(const, ["Array", "dslabel", "dims_offset", "dims_counts"], ["const cv::_OutputArray*", "const cv::String*", "const std::vector<int>*", "const std::vector<int>*"]), _)]),
	void cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_vectorLintGR_const_vectorLintGR(const cv::hdf::HDF5* instance, const cv::_OutputArray* Array, const char* dslabel, const std::vector<int>* dims_offset, const std::vector<int>* dims_counts, ResultVoid* ocvrs_return) {
		try {
			instance->dsread(*Array, cv::String(dslabel), *dims_offset, *dims_counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hdf::HDF5::dsread(OutputArray, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:533
	// ("cv::hdf::HDF5::dsread", vec![(pred!(const, ["Array", "dslabel", "dims_offset"], ["const cv::_OutputArray*", "const cv::String*", "const std::vector<int>*"]), _)]),
	void cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_vectorLintGR(const cv::hdf::HDF5* instance, const cv::_OutputArray* Array, const char* dslabel, const std::vector<int>* dims_offset, ResultVoid* ocvrs_return) {
		try {
			instance->dsread(*Array, cv::String(dslabel), *dims_offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// kpgetsize(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:591
	// ("cv::hdf::HDF5::kpgetsize", vec![(pred!(const, ["kplabel", "dims_flag"], ["const cv::String*", "int"]), _)]),
	void cv_hdf_HDF5_kpgetsize_const_const_StringR_int(const cv::hdf::HDF5* instance, const char* kplabel, int dims_flag, Result<int>* ocvrs_return) {
		try {
			int ret = instance->kpgetsize(cv::String(kplabel), dims_flag);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hdf::HDF5::kpgetsize(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:591
	// ("cv::hdf::HDF5::kpgetsize", vec![(pred!(const, ["kplabel"], ["const cv::String*"]), _)]),
	void cv_hdf_HDF5_kpgetsize_const_const_StringR(const cv::hdf::HDF5* instance, const char* kplabel, Result<int>* ocvrs_return) {
		try {
			int ret = instance->kpgetsize(cv::String(kplabel));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// kpcreate(const int, const String &, const int, const int)(Primitive, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:628
	// ("cv::hdf::HDF5::kpcreate", vec![(pred!(const, ["size", "kplabel", "compresslevel", "chunks"], ["const int", "const cv::String*", "const int", "const int"]), _)]),
	void cv_hdf_HDF5_kpcreate_const_const_int_const_StringR_const_int_const_int(const cv::hdf::HDF5* instance, const int size, const char* kplabel, const int compresslevel, const int chunks, ResultVoid* ocvrs_return) {
		try {
			instance->kpcreate(size, cv::String(kplabel), compresslevel, chunks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hdf::HDF5::kpcreate(Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:628
	// ("cv::hdf::HDF5::kpcreate", vec![(pred!(const, ["size", "kplabel"], ["const int", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_kpcreate_const_const_int_const_StringR(const cv::hdf::HDF5* instance, const int size, const char* kplabel, ResultVoid* ocvrs_return) {
		try {
			instance->kpcreate(size, cv::String(kplabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// kpwrite(const vector<KeyPoint>, const String &, const int, const int)(CppPassByVoidPtr, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:681
	// ("cv::hdf::HDF5::kpwrite", vec![(pred!(const, ["keypoints", "kplabel", "offset", "counts"], ["const std::vector<cv::KeyPoint>", "const cv::String*", "const int", "const int"]), _)]),
	void cv_hdf_HDF5_kpwrite_const_const_vectorLKeyPointG_const_StringR_const_int_const_int(const cv::hdf::HDF5* instance, const std::vector<cv::KeyPoint>* keypoints, const char* kplabel, const int offset, const int counts, ResultVoid* ocvrs_return) {
		try {
			instance->kpwrite(*keypoints, cv::String(kplabel), offset, counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hdf::HDF5::kpwrite(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:681
	// ("cv::hdf::HDF5::kpwrite", vec![(pred!(const, ["keypoints", "kplabel"], ["const std::vector<cv::KeyPoint>", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_kpwrite_const_const_vectorLKeyPointG_const_StringR(const cv::hdf::HDF5* instance, const std::vector<cv::KeyPoint>* keypoints, const char* kplabel, ResultVoid* ocvrs_return) {
		try {
			instance->kpwrite(*keypoints, cv::String(kplabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// kpinsert(const vector<KeyPoint>, const String &, const int, const int)(CppPassByVoidPtr, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:716
	// ("cv::hdf::HDF5::kpinsert", vec![(pred!(const, ["keypoints", "kplabel", "offset", "counts"], ["const std::vector<cv::KeyPoint>", "const cv::String*", "const int", "const int"]), _)]),
	void cv_hdf_HDF5_kpinsert_const_const_vectorLKeyPointG_const_StringR_const_int_const_int(const cv::hdf::HDF5* instance, const std::vector<cv::KeyPoint>* keypoints, const char* kplabel, const int offset, const int counts, ResultVoid* ocvrs_return) {
		try {
			instance->kpinsert(*keypoints, cv::String(kplabel), offset, counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hdf::HDF5::kpinsert(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:716
	// ("cv::hdf::HDF5::kpinsert", vec![(pred!(const, ["keypoints", "kplabel"], ["const std::vector<cv::KeyPoint>", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_kpinsert_const_const_vectorLKeyPointG_const_StringR(const cv::hdf::HDF5* instance, const std::vector<cv::KeyPoint>* keypoints, const char* kplabel, ResultVoid* ocvrs_return) {
		try {
			instance->kpinsert(*keypoints, cv::String(kplabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// kpread(vector<KeyPoint> &, const String &, const int, const int)(CppPassByVoidPtr, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:754
	// ("cv::hdf::HDF5::kpread", vec![(pred!(const, ["keypoints", "kplabel", "offset", "counts"], ["std::vector<cv::KeyPoint>*", "const cv::String*", "const int", "const int"]), _)]),
	void cv_hdf_HDF5_kpread_const_vectorLKeyPointGR_const_StringR_const_int_const_int(const cv::hdf::HDF5* instance, std::vector<cv::KeyPoint>* keypoints, const char* kplabel, const int offset, const int counts, ResultVoid* ocvrs_return) {
		try {
			instance->kpread(*keypoints, cv::String(kplabel), offset, counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hdf::HDF5::kpread(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/hdf/hdf5.hpp:754
	// ("cv::hdf::HDF5::kpread", vec![(pred!(const, ["keypoints", "kplabel"], ["std::vector<cv::KeyPoint>*", "const cv::String*"]), _)]),
	void cv_hdf_HDF5_kpread_const_vectorLKeyPointGR_const_StringR(const cv::hdf::HDF5* instance, std::vector<cv::KeyPoint>* keypoints, const char* kplabel, ResultVoid* ocvrs_return) {
		try {
			instance->kpread(*keypoints, cv::String(kplabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hdf::HDF5::delete() generated
	// ("cv::hdf::HDF5::delete", vec![(pred!(mut, [], []), _)]),
	void cv_hdf_HDF5_delete(cv::hdf::HDF5* instance) {
			delete instance;
	}

}

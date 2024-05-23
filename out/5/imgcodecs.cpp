#include "ocvrs_common.hpp"
#include <opencv2/imgcodecs.hpp>
#include "imgcodecs_types.hpp"

extern "C" {
	// haveImageReader(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:447
	// ("cv::haveImageReader", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_haveImageReader_const_StringR(const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::haveImageReader(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// haveImageWriter(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:463
	// ("cv::haveImageWriter", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_haveImageWriter_const_StringR(const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::haveImageWriter(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imcount(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:315
	// ("cv::imcount", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_imcount_const_StringR(const char* filename, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = cv::imcount(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imcount(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:315
	// ("cv::imcount", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_imcount_const_StringR_int(const char* filename, int flags, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = cv::imcount(std::string(filename), flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imdecode(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:379
	// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_imdecode_const__InputArrayR_int(const cv::_InputArray* buf, int flags, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imdecode(*buf, flags);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imdecode(InputArray, int, Mat *)(InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:388
	// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags", "dst"], ["const cv::_InputArray*", "int", "cv::Mat*"]), _)]),
	void cv_imdecode_const__InputArrayR_int_MatX(const cv::_InputArray* buf, int flags, cv::Mat* dst, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imdecode(*buf, flags, dst);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imdecodemulti(InputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:403
	// ("cv::imdecodemulti", vec![(pred!(mut, ["buf", "flags", "mats"], ["const cv::_InputArray*", "int", "std::vector<cv::Mat>*"]), _)]),
	void cv_imdecodemulti_const__InputArrayR_int_vectorLMatGR(const cv::_InputArray* buf, int flags, std::vector<cv::Mat>* mats, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imdecodemulti(*buf, flags, *mats);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imdecodemulti(InputArray, int, std::vector<Mat> &, const cv::Range &)(InputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:403
	// ("cv::imdecodemulti", vec![(pred!(mut, ["buf", "flags", "mats", "range"], ["const cv::_InputArray*", "int", "std::vector<cv::Mat>*", "const cv::Range*"]), _)]),
	void cv_imdecodemulti_const__InputArrayR_int_vectorLMatGR_const_RangeR(const cv::_InputArray* buf, int flags, std::vector<cv::Mat>* mats, const cv::Range* range, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imdecodemulti(*buf, flags, *mats, *range);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imencode(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:415
	// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*"]), _)]),
	void cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR(const char* ext, const cv::_InputArray* img, std::vector<unsigned char>* buf, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imencode(std::string(ext), *img, *buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imencode(const String &, InputArray, std::vector<uchar> &, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:415
	// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf", "params"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*", "const std::vector<int>*"]), _)]),
	void cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(const char* ext, const cv::_InputArray* img, std::vector<unsigned char>* buf, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imencode(std::string(ext), *img, *buf, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imencodemulti(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:429
	// ("cv::imencodemulti", vec![(pred!(mut, ["ext", "imgs", "buf"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*"]), _)]),
	void cv_imencodemulti_const_StringR_const__InputArrayR_vectorLunsigned_charGR(const char* ext, const cv::_InputArray* imgs, std::vector<unsigned char>* buf, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imencodemulti(std::string(ext), *imgs, *buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imencodemulti(const String &, InputArrayOfArrays, std::vector<uchar> &, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:429
	// ("cv::imencodemulti", vec![(pred!(mut, ["ext", "imgs", "buf", "params"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*", "const std::vector<int>*"]), _)]),
	void cv_imencodemulti_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(const char* ext, const cv::_InputArray* imgs, std::vector<unsigned char>* buf, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imencodemulti(std::string(ext), *imgs, *buf, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imread(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:272
	// ("cv::imread", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_imread_const_StringR(const char* filename, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imread(std::string(filename));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imread(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:283
	// ("cv::imread", vec![(pred!(mut, ["filename", "dst"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
	void cv_imread_const_StringR_const__OutputArrayR(const char* filename, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::imread(std::string(filename), *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imread(const String &, OutputArray, int)(InString, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:283
	// ("cv::imread", vec![(pred!(mut, ["filename", "dst", "flags"], ["const cv::String*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_imread_const_StringR_const__OutputArrayR_int(const char* filename, const cv::_OutputArray* dst, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::imread(std::string(filename), *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imread(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:272
	// ("cv::imread", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_imread_const_StringR_int(const char* filename, int flags, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imread(std::string(filename), flags);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imreadmulti(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:293
	// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats"], ["const cv::String*", "std::vector<cv::Mat>*"]), _)]),
	void cv_imreadmulti_const_StringR_vectorLMatGR(const char* filename, std::vector<cv::Mat>* mats, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *mats);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imreadmulti(const String &, std::vector<Mat> &, int)(InString, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:293
	// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "flags"], ["const cv::String*", "std::vector<cv::Mat>*", "int"]), _)]),
	void cv_imreadmulti_const_StringR_vectorLMatGR_int(const char* filename, std::vector<cv::Mat>* mats, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *mats, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imreadmulti(InString, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:305
	// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "start", "count"], ["const cv::String*", "std::vector<cv::Mat>*", "int", "int"]), _)]),
	void cv_imreadmulti_const_StringR_vectorLMatGR_int_int(const char* filename, std::vector<cv::Mat>* mats, int start, int count, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *mats, start, count);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imreadmulti(const String &, std::vector<Mat> &, int, int, int)(InString, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:305
	// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "start", "count", "flags"], ["const cv::String*", "std::vector<cv::Mat>*", "int", "int", "int"]), _)]),
	void cv_imreadmulti_const_StringR_vectorLMatGR_int_int_int(const char* filename, std::vector<cv::Mat>* mats, int start, int count, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *mats, start, count, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imwrite(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:357
	// ("cv::imwrite", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_imwrite_const_StringR_const__InputArrayR(const char* filename, const cv::_InputArray* img, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwrite(std::string(filename), *img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imwrite(const String &, InputArray, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:357
	// ("cv::imwrite", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
	void cv_imwrite_const_StringR_const__InputArrayR_const_vectorLintGR(const char* filename, const cv::_InputArray* img, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwrite(std::string(filename), *img, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imwritemulti(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:362
	// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_imwritemulti_const_StringR_const__InputArrayR(const char* filename, const cv::_InputArray* img, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwritemulti(std::string(filename), *img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imwritemulti(const String &, InputArrayOfArrays, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:362
	// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
	void cv_imwritemulti_const_StringR_const__InputArrayR_const_vectorLintGR(const char* filename, const cv::_InputArray* img, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwritemulti(std::string(filename), *img, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ImageCollection()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:495
	// ("cv::ImageCollection::ImageCollection", vec![(pred!(mut, [], []), _)]),
	void cv_ImageCollection_ImageCollection(Result<cv::ImageCollection*>* ocvrs_return) {
		try {
			cv::ImageCollection* ret = new cv::ImageCollection();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ImageCollection(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:496
	// ("cv::ImageCollection::ImageCollection", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_ImageCollection_ImageCollection_const_StringR_int(const char* filename, int flags, Result<cv::ImageCollection*>* ocvrs_return) {
		try {
			cv::ImageCollection* ret = new cv::ImageCollection(std::string(filename), flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// init(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:497
	// ("cv::ImageCollection::init", vec![(pred!(mut, ["img", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_ImageCollection_init_const_StringR_int(cv::ImageCollection* instance, const char* img, int flags, ResultVoid* ocvrs_return) {
		try {
			instance->init(std::string(img), flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:498
	// ("cv::ImageCollection::size", vec![(pred!(const, [], []), _)]),
	void cv_ImageCollection_size_const(const cv::ImageCollection* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// at(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:499
	// ("cv::ImageCollection::at", vec![(pred!(mut, ["index"], ["int"]), _)]),
	void cv_ImageCollection_at_int(cv::ImageCollection* instance, int index, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->at(index);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:500
	// ("cv::ImageCollection::operator[]", vec![(pred!(mut, ["index"], ["int"]), _)]),
	void cv_ImageCollection_operator___int(cv::ImageCollection* instance, int index, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->operator[](index);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// releaseCache(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:501
	// ("cv::ImageCollection::releaseCache", vec![(pred!(mut, ["index"], ["int"]), _)]),
	void cv_ImageCollection_releaseCache_int(cv::ImageCollection* instance, int index, ResultVoid* ocvrs_return) {
		try {
			instance->releaseCache(index);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// begin()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:502
	// ("cv::ImageCollection::begin", vec![(pred!(mut, [], []), _)]),
	void cv_ImageCollection_begin(cv::ImageCollection* instance, Result<cv::ImageCollection::iterator*>* ocvrs_return) {
		try {
			cv::ImageCollection::iterator ret = instance->begin();
			Ok(new cv::ImageCollection::iterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// end()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:503
	// ("cv::ImageCollection::end", vec![(pred!(mut, [], []), _)]),
	void cv_ImageCollection_end(cv::ImageCollection* instance, Result<cv::ImageCollection::iterator*>* ocvrs_return) {
		try {
			cv::ImageCollection::iterator ret = instance->end();
			Ok(new cv::ImageCollection::iterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ImageCollection::delete() generated
	// ("cv::ImageCollection::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ImageCollection_delete(cv::ImageCollection* instance) {
			delete instance;
	}

	// iterator(ImageCollection *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:481
	// ("cv::ImageCollection::iterator::iterator", vec![(pred!(mut, ["col"], ["cv::ImageCollection*"]), _)]),
	void cv_ImageCollection_iterator_iterator_ImageCollectionX(cv::ImageCollection* col, Result<cv::ImageCollection::iterator*>* ocvrs_return) {
		try {
			cv::ImageCollection::iterator* ret = new cv::ImageCollection::iterator(col);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// iterator(ImageCollection *, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:482
	// ("cv::ImageCollection::iterator::iterator", vec![(pred!(mut, ["col", "end"], ["cv::ImageCollection*", "int"]), _)]),
	void cv_ImageCollection_iterator_iterator_ImageCollectionX_int(cv::ImageCollection* col, int end, Result<cv::ImageCollection::iterator*>* ocvrs_return) {
		try {
			cv::ImageCollection::iterator* ret = new cv::ImageCollection::iterator(col, end);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:483
	// ("cv::ImageCollection::iterator::operator*", vec![(pred!(mut, [], []), _)]),
	void cv_ImageCollection_iterator_operatorX(cv::ImageCollection::iterator* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->operator*();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator++()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:485
	// ("cv::ImageCollection::iterator::operator++", vec![(pred!(mut, [], []), _)]),
	void cv_ImageCollection_iterator_operatorAA(cv::ImageCollection::iterator* instance, Result<cv::ImageCollection::iterator*>* ocvrs_return) {
		try {
			cv::ImageCollection::iterator ret = instance->operator++();
			Ok(new cv::ImageCollection::iterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ImageCollection::iterator::delete() generated
	// ("cv::ImageCollection::iterator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ImageCollection_iterator_delete(cv::ImageCollection::iterator* instance) {
			delete instance;
	}

}

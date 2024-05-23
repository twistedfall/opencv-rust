#include "ocvrs_common.hpp"
#include <opencv2/imgcodecs.hpp>
#include "imgcodecs_types.hpp"

extern "C" {
	// haveImageReader(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:535
	// ("cv::haveImageReader", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_haveImageReader_const_StringR(const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::haveImageReader(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// haveImageWriter(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:551
	// ("cv::haveImageWriter", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_haveImageWriter_const_StringR(const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::haveImageWriter(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imcount(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:400
	// ("cv::imcount", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_imcount_const_StringR(const char* filename, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = cv::imcount(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imcount(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:400
	// ("cv::imcount", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_imcount_const_StringR_int(const char* filename, int flags, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = cv::imcount(std::string(filename), flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imdecode(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:467
	// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_imdecode_const__InputArrayR_int(const cv::_InputArray* buf, int flags, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imdecode(*buf, flags);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imdecode(InputArray, int, Mat *)(InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:476
	// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags", "dst"], ["const cv::_InputArray*", "int", "cv::Mat*"]), _)]),
	void cv_imdecode_const__InputArrayR_int_MatX(const cv::_InputArray* buf, int flags, cv::Mat* dst, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imdecode(*buf, flags, dst);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imdecodemulti(InputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:491
	// ("cv::imdecodemulti", vec![(pred!(mut, ["buf", "flags", "mats"], ["const cv::_InputArray*", "int", "std::vector<cv::Mat>*"]), _)]),
	void cv_imdecodemulti_const__InputArrayR_int_vectorLMatGR(const cv::_InputArray* buf, int flags, std::vector<cv::Mat>* mats, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imdecodemulti(*buf, flags, *mats);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imdecodemulti(InputArray, int, std::vector<Mat> &, const cv::Range &)(InputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:491
	// ("cv::imdecodemulti", vec![(pred!(mut, ["buf", "flags", "mats", "range"], ["const cv::_InputArray*", "int", "std::vector<cv::Mat>*", "const cv::Range*"]), _)]),
	void cv_imdecodemulti_const__InputArrayR_int_vectorLMatGR_const_RangeR(const cv::_InputArray* buf, int flags, std::vector<cv::Mat>* mats, const cv::Range* range, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imdecodemulti(*buf, flags, *mats, *range);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imencode(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:503
	// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*"]), _)]),
	void cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR(const char* ext, const cv::_InputArray* img, std::vector<unsigned char>* buf, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imencode(std::string(ext), *img, *buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imencode(const String &, InputArray, std::vector<uchar> &, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:503
	// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf", "params"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*", "const std::vector<int>*"]), _)]),
	void cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(const char* ext, const cv::_InputArray* img, std::vector<unsigned char>* buf, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imencode(std::string(ext), *img, *buf, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imencodemulti(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:517
	// ("cv::imencodemulti", vec![(pred!(mut, ["ext", "imgs", "buf"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*"]), _)]),
	void cv_imencodemulti_const_StringR_const__InputArrayR_vectorLunsigned_charGR(const char* ext, const cv::_InputArray* imgs, std::vector<unsigned char>* buf, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imencodemulti(std::string(ext), *imgs, *buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imencodemulti(const String &, InputArrayOfArrays, std::vector<uchar> &, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:517
	// ("cv::imencodemulti", vec![(pred!(mut, ["ext", "imgs", "buf", "params"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*", "const std::vector<int>*"]), _)]),
	void cv_imencodemulti_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(const char* ext, const cv::_InputArray* imgs, std::vector<unsigned char>* buf, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imencodemulti(std::string(ext), *imgs, *buf, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imread(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:325
	// ("cv::imread", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_imread_const_StringR(const char* filename, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imread(std::string(filename));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imread(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:336
	// ("cv::imread", vec![(pred!(mut, ["filename", "dst"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
	void cv_imread_const_StringR_const__OutputArrayR(const char* filename, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::imread(std::string(filename), *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imread(const String &, OutputArray, int)(InString, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:336
	// ("cv::imread", vec![(pred!(mut, ["filename", "dst", "flags"], ["const cv::String*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_imread_const_StringR_const__OutputArrayR_int(const char* filename, const cv::_OutputArray* dst, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::imread(std::string(filename), *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imread(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:325
	// ("cv::imread", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_imread_const_StringR_int(const char* filename, int flags, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imread(std::string(filename), flags);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imreadanimation(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:376
	// ("cv::imreadanimation", vec![(pred!(mut, ["filename", "animation"], ["const cv::String*", "cv::Animation*"]), _)]),
	void cv_imreadanimation_const_StringR_AnimationR(const char* filename, cv::Animation* animation, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadanimation(std::string(filename), *animation);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imreadanimation(const String &, Animation &, int, int)(InString, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:376
	// ("cv::imreadanimation", vec![(pred!(mut, ["filename", "animation", "start", "count"], ["const cv::String*", "cv::Animation*", "int", "int"]), _)]),
	void cv_imreadanimation_const_StringR_AnimationR_int_int(const char* filename, cv::Animation* animation, int start, int count, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadanimation(std::string(filename), *animation, start, count);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imreadmulti(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:346
	// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats"], ["const cv::String*", "std::vector<cv::Mat>*"]), _)]),
	void cv_imreadmulti_const_StringR_vectorLMatGR(const char* filename, std::vector<cv::Mat>* mats, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *mats);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imreadmulti(const String &, std::vector<Mat> &, int)(InString, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:346
	// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "flags"], ["const cv::String*", "std::vector<cv::Mat>*", "int"]), _)]),
	void cv_imreadmulti_const_StringR_vectorLMatGR_int(const char* filename, std::vector<cv::Mat>* mats, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *mats, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imreadmulti(InString, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:358
	// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "start", "count"], ["const cv::String*", "std::vector<cv::Mat>*", "int", "int"]), _)]),
	void cv_imreadmulti_const_StringR_vectorLMatGR_int_int(const char* filename, std::vector<cv::Mat>* mats, int start, int count, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *mats, start, count);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imreadmulti(const String &, std::vector<Mat> &, int, int, int)(InString, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:358
	// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "start", "count", "flags"], ["const cv::String*", "std::vector<cv::Mat>*", "int", "int", "int"]), _)]),
	void cv_imreadmulti_const_StringR_vectorLMatGR_int_int_int(const char* filename, std::vector<cv::Mat>* mats, int start, int count, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *mats, start, count, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imwrite(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:445
	// ("cv::imwrite", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_imwrite_const_StringR_const__InputArrayR(const char* filename, const cv::_InputArray* img, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwrite(std::string(filename), *img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imwrite(const String &, InputArray, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:445
	// ("cv::imwrite", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
	void cv_imwrite_const_StringR_const__InputArrayR_const_vectorLintGR(const char* filename, const cv::_InputArray* img, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwrite(std::string(filename), *img, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imwriteanimation(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:390
	// ("cv::imwriteanimation", vec![(pred!(mut, ["filename", "animation"], ["const cv::String*", "const cv::Animation*"]), _)]),
	void cv_imwriteanimation_const_StringR_const_AnimationR(const char* filename, const cv::Animation* animation, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwriteanimation(std::string(filename), *animation);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imwriteanimation(const String &, const Animation &, const std::vector<int> &)(InString, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:390
	// ("cv::imwriteanimation", vec![(pred!(mut, ["filename", "animation", "params"], ["const cv::String*", "const cv::Animation*", "const std::vector<int>*"]), _)]),
	void cv_imwriteanimation_const_StringR_const_AnimationR_const_vectorLintGR(const char* filename, const cv::Animation* animation, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwriteanimation(std::string(filename), *animation, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imwritemulti(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:450
	// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_imwritemulti_const_StringR_const__InputArrayR(const char* filename, const cv::_InputArray* img, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwritemulti(std::string(filename), *img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imwritemulti(const String &, InputArrayOfArrays, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:450
	// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
	void cv_imwritemulti_const_StringR_const__InputArrayR_const_vectorLintGR(const char* filename, const cv::_InputArray* img, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwritemulti(std::string(filename), *img, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Animation(int, Scalar)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:270
	// ("cv::Animation::Animation", vec![(pred!(mut, ["loopCount", "bgColor"], ["int", "cv::Scalar"]), _)]),
	void cv_Animation_Animation_int_Scalar(int loopCount, cv::Scalar* bgColor, Result<cv::Animation*>* ocvrs_return) {
		try {
			cv::Animation* ret = new cv::Animation(loopCount, *bgColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Animation::Animation() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:270
	// ("cv::Animation::Animation", vec![(pred!(mut, [], []), _)]),
	void cv_Animation_Animation(Result<cv::Animation*>* ocvrs_return) {
		try {
			cv::Animation* ret = new cv::Animation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Animation::implicitClone() generated
	// ("cv::Animation::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::Animation* cv_Animation_implicitClone_const(const cv::Animation* instance) {
			return new cv::Animation(*instance);
	}

	// cv::Animation::loop_count() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:250
	// ("cv::Animation::loop_count", vec![(pred!(const, [], []), _)]),
	int cv_Animation_propLoop_count_const(const cv::Animation* instance) {
			int ret = instance->loop_count;
			return ret;
	}

	// cv::Animation::setLoop_count(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:250
	// ("cv::Animation::setLoop_count", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_Animation_propLoop_count_const_int(cv::Animation* instance, const int val) {
			instance->loop_count = val;
	}

	// cv::Animation::bgcolor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:252
	// ("cv::Animation::bgcolor", vec![(pred!(const, [], []), _)]),
	void cv_Animation_propBgcolor_const(const cv::Animation* instance, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = instance->bgcolor;
			*ocvrs_return = ret;
	}

	// cv::Animation::setBgcolor(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:252
	// ("cv::Animation::setBgcolor", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
	void cv_Animation_propBgcolor_const_Scalar(cv::Animation* instance, const cv::Scalar* val) {
			instance->bgcolor = *val;
	}

	// cv::Animation::durations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:254
	// ("cv::Animation::durations", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_Animation_propDurations_const(const cv::Animation* instance) {
			std::vector<int> ret = instance->durations;
			return new std::vector<int>(ret);
	}

	// cv::Animation::setDurations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:254
	// ("cv::Animation::setDurations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_Animation_propDurations_const_vectorLintG(cv::Animation* instance, const std::vector<int>* val) {
			instance->durations = *val;
	}

	// cv::Animation::frames() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:256
	// ("cv::Animation::frames", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_Animation_propFrames_const(const cv::Animation* instance) {
			std::vector<cv::Mat> ret = instance->frames;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::Animation::setFrames(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:256
	// ("cv::Animation::setFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_Animation_propFrames_const_vectorLMatG(cv::Animation* instance, const std::vector<cv::Mat>* val) {
			instance->frames = *val;
	}

	// cv::Animation::delete() generated
	// ("cv::Animation::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Animation_delete(cv::Animation* instance) {
			delete instance;
	}

	// ImageCollection()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:583
	// ("cv::ImageCollection::ImageCollection", vec![(pred!(mut, [], []), _)]),
	void cv_ImageCollection_ImageCollection(Result<cv::ImageCollection*>* ocvrs_return) {
		try {
			cv::ImageCollection* ret = new cv::ImageCollection();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ImageCollection(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:584
	// ("cv::ImageCollection::ImageCollection", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_ImageCollection_ImageCollection_const_StringR_int(const char* filename, int flags, Result<cv::ImageCollection*>* ocvrs_return) {
		try {
			cv::ImageCollection* ret = new cv::ImageCollection(std::string(filename), flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// init(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:585
	// ("cv::ImageCollection::init", vec![(pred!(mut, ["img", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_ImageCollection_init_const_StringR_int(cv::ImageCollection* instance, const char* img, int flags, ResultVoid* ocvrs_return) {
		try {
			instance->init(std::string(img), flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:586
	// ("cv::ImageCollection::size", vec![(pred!(const, [], []), _)]),
	void cv_ImageCollection_size_const(const cv::ImageCollection* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// at(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:587
	// ("cv::ImageCollection::at", vec![(pred!(mut, ["index"], ["int"]), _)]),
	void cv_ImageCollection_at_int(cv::ImageCollection* instance, int index, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->at(index);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:588
	// ("cv::ImageCollection::operator[]", vec![(pred!(mut, ["index"], ["int"]), _)]),
	void cv_ImageCollection_operator___int(cv::ImageCollection* instance, int index, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->operator[](index);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// releaseCache(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:589
	// ("cv::ImageCollection::releaseCache", vec![(pred!(mut, ["index"], ["int"]), _)]),
	void cv_ImageCollection_releaseCache_int(cv::ImageCollection* instance, int index, ResultVoid* ocvrs_return) {
		try {
			instance->releaseCache(index);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// begin()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:590
	// ("cv::ImageCollection::begin", vec![(pred!(mut, [], []), _)]),
	void cv_ImageCollection_begin(cv::ImageCollection* instance, Result<cv::ImageCollection::iterator*>* ocvrs_return) {
		try {
			cv::ImageCollection::iterator ret = instance->begin();
			Ok(new cv::ImageCollection::iterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// end()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:591
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

	// iterator(ImageCollection *)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:569
	// ("cv::ImageCollection::iterator::iterator", vec![(pred!(mut, ["col"], ["cv::ImageCollection*"]), _)]),
	void cv_ImageCollection_iterator_iterator_ImageCollectionX(cv::ImageCollection* col, Result<cv::ImageCollection::iterator*>* ocvrs_return) {
		try {
			cv::ImageCollection::iterator* ret = new cv::ImageCollection::iterator(col);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// iterator(ImageCollection *, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:570
	// ("cv::ImageCollection::iterator::iterator", vec![(pred!(mut, ["col", "end"], ["cv::ImageCollection*", "int"]), _)]),
	void cv_ImageCollection_iterator_iterator_ImageCollectionX_int(cv::ImageCollection* col, int end, Result<cv::ImageCollection::iterator*>* ocvrs_return) {
		try {
			cv::ImageCollection::iterator* ret = new cv::ImageCollection::iterator(col, end);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:571
	// ("cv::ImageCollection::iterator::operator*", vec![(pred!(mut, [], []), _)]),
	void cv_ImageCollection_iterator_operatorX(cv::ImageCollection::iterator* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->operator*();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator++()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:573
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

#include "ocvrs_common.hpp"
#include <opencv2/line_descriptor.hpp>
#include "line_descriptor_types.hpp"

extern "C" {
	// cv::line_descriptor::drawKeylines(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1401
	// ("cv::line_descriptor::drawKeylines", vec![(pred!(mut, ["image", "keylines", "outImage"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*"]), _)]),
	void cv_line_descriptor_drawKeylines_const_MatR_const_vectorLKeyLineGR_MatR(const cv::Mat* image, const std::vector<cv::line_descriptor::KeyLine>* keylines, cv::Mat* outImage, ResultVoid* ocvrs_return) {
		try {
			cv::line_descriptor::drawKeylines(*image, *keylines, *outImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawKeylines(const Mat &, const std::vector<KeyLine> &, Mat &, const Scalar &, int)(TraitClass, CppPassByVoidPtr, TraitClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1401
	// ("cv::line_descriptor::drawKeylines", vec![(pred!(mut, ["image", "keylines", "outImage", "color", "flags"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*", "const cv::Scalar*", "int"]), _)]),
	void cv_line_descriptor_drawKeylines_const_MatR_const_vectorLKeyLineGR_MatR_const_ScalarR_int(const cv::Mat* image, const std::vector<cv::line_descriptor::KeyLine>* keylines, cv::Mat* outImage, const cv::Scalar* color, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::line_descriptor::drawKeylines(*image, *keylines, *outImage, *color, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::drawLineMatches(TraitClass, CppPassByVoidPtr, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1388
	// ("cv::line_descriptor::drawLineMatches", vec![(pred!(mut, ["img1", "keylines1", "img2", "keylines2", "matches1to2", "outImg"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const std::vector<cv::DMatch>*", "cv::Mat*"]), _)]),
	void cv_line_descriptor_drawLineMatches_const_MatR_const_vectorLKeyLineGR_const_MatR_const_vectorLKeyLineGR_const_vectorLDMatchGR_MatR(const cv::Mat* img1, const std::vector<cv::line_descriptor::KeyLine>* keylines1, const cv::Mat* img2, const std::vector<cv::line_descriptor::KeyLine>* keylines2, const std::vector<cv::DMatch>* matches1to2, cv::Mat* outImg, ResultVoid* ocvrs_return) {
		try {
			cv::line_descriptor::drawLineMatches(*img1, *keylines1, *img2, *keylines2, *matches1to2, *outImg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawLineMatches(const Mat &, const std::vector<KeyLine> &, const Mat &, const std::vector<KeyLine> &, const std::vector<DMatch> &, Mat &, const Scalar &, const Scalar &, const std::vector<char> &, int)(TraitClass, CppPassByVoidPtr, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass, SimpleClass, SimpleClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1388
	// ("cv::line_descriptor::drawLineMatches", vec![(pred!(mut, ["img1", "keylines1", "img2", "keylines2", "matches1to2", "outImg", "matchColor", "singleLineColor", "matchesMask", "flags"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const std::vector<cv::DMatch>*", "cv::Mat*", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<char>*", "int"]), _)]),
	void cv_line_descriptor_drawLineMatches_const_MatR_const_vectorLKeyLineGR_const_MatR_const_vectorLKeyLineGR_const_vectorLDMatchGR_MatR_const_ScalarR_const_ScalarR_const_vectorLcharGR_int(const cv::Mat* img1, const std::vector<cv::line_descriptor::KeyLine>* keylines1, const cv::Mat* img2, const std::vector<cv::line_descriptor::KeyLine>* keylines2, const std::vector<cv::DMatch>* matches1to2, cv::Mat* outImg, const cv::Scalar* matchColor, const cv::Scalar* singleLineColor, const std::vector<char>* matchesMask, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::line_descriptor::drawLineMatches(*img1, *keylines1, *img2, *keylines2, *matches1to2, *outImg, *matchColor, *singleLineColor, *matchesMask, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BinaryDescriptor(const BinaryDescriptor::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:221
	// ("cv::line_descriptor::BinaryDescriptor::BinaryDescriptor", vec![(pred!(mut, ["parameters"], ["const cv::line_descriptor::BinaryDescriptor::Params*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_BinaryDescriptor_const_ParamsR(const cv::line_descriptor::BinaryDescriptor::Params* parameters, Result<cv::line_descriptor::BinaryDescriptor*>* ocvrs_return) {
		try {
			cv::line_descriptor::BinaryDescriptor* ret = new cv::line_descriptor::BinaryDescriptor(*parameters);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptor::BinaryDescriptor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:221
	// ("cv::line_descriptor::BinaryDescriptor::BinaryDescriptor", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptor_BinaryDescriptor(Result<cv::line_descriptor::BinaryDescriptor*>* ocvrs_return) {
		try {
			cv::line_descriptor::BinaryDescriptor* ret = new cv::line_descriptor::BinaryDescriptor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBinaryDescriptor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:226
	// ("cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor(Result<cv::Ptr<cv::line_descriptor::BinaryDescriptor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::line_descriptor::BinaryDescriptor> ret = cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor();
			Ok(new cv::Ptr<cv::line_descriptor::BinaryDescriptor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBinaryDescriptor(Params)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:227
	// ("cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor", vec![(pred!(mut, ["parameters"], ["cv::line_descriptor::BinaryDescriptor::Params"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor_Params(cv::line_descriptor::BinaryDescriptor::Params* parameters, Result<cv::Ptr<cv::line_descriptor::BinaryDescriptor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::line_descriptor::BinaryDescriptor> ret = cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor(*parameters);
			Ok(new cv::Ptr<cv::line_descriptor::BinaryDescriptor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumOfOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:234
	// ("cv::line_descriptor::BinaryDescriptor::getNumOfOctaves", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptor_getNumOfOctaves(cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumOfOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumOfOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:238
	// ("cv::line_descriptor::BinaryDescriptor::setNumOfOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_setNumOfOctaves_int(cv::line_descriptor::BinaryDescriptor* instance, int octaves, ResultVoid* ocvrs_return) {
		try {
			instance->setNumOfOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWidthOfBand()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:241
	// ("cv::line_descriptor::BinaryDescriptor::getWidthOfBand", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptor_getWidthOfBand(cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWidthOfBand();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWidthOfBand(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:245
	// ("cv::line_descriptor::BinaryDescriptor::setWidthOfBand", vec![(pred!(mut, ["width"], ["int"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_setWidthOfBand_int(cv::line_descriptor::BinaryDescriptor* instance, int width, ResultVoid* ocvrs_return) {
		try {
			instance->setWidthOfBand(width);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getReductionRatio()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:248
	// ("cv::line_descriptor::BinaryDescriptor::getReductionRatio", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptor_getReductionRatio(cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getReductionRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setReductionRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:252
	// ("cv::line_descriptor::BinaryDescriptor::setReductionRatio", vec![(pred!(mut, ["rRatio"], ["int"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_setReductionRatio_int(cv::line_descriptor::BinaryDescriptor* instance, int rRatio, ResultVoid* ocvrs_return) {
		try {
			instance->setReductionRatio(rRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const cv::FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:258
	// ("cv::line_descriptor::BinaryDescriptor::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_read_const_FileNodeR(cv::line_descriptor::BinaryDescriptor* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(cv::FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:264
	// ("cv::line_descriptor::BinaryDescriptor::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_write_const_FileStorageR(const cv::line_descriptor::BinaryDescriptor* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(const Mat &, std::vector<KeyLine> &, const Mat &)(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:272
	// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(mut, ["image", "keypoints", "mask"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "const cv::Mat*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_detect_const_MatR_vectorLKeyLineGR_const_MatR(cv::line_descriptor::BinaryDescriptor* instance, const cv::Mat* image, std::vector<cv::line_descriptor::KeyLine>* keypoints, const cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptor::detect(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:272
	// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(mut, ["image", "keypoints"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_detect_const_MatR_vectorLKeyLineGR(cv::line_descriptor::BinaryDescriptor* instance, const cv::Mat* image, std::vector<cv::line_descriptor::KeyLine>* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, const std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:280
	// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(const, ["images", "keylines", "masks"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "const std::vector<cv::Mat>*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_const_vectorLMatGR(const cv::line_descriptor::BinaryDescriptor* instance, const std::vector<cv::Mat>* images, std::vector<std::vector<cv::line_descriptor::KeyLine>>* keylines, const std::vector<cv::Mat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*images, *keylines, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptor::detect(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:280
	// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(const, ["images", "keylines"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR(const cv::line_descriptor::BinaryDescriptor* instance, const std::vector<cv::Mat>* images, std::vector<std::vector<cv::line_descriptor::KeyLine>>* keylines, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*images, *keylines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(const Mat &, std::vector<KeyLine> &, Mat &, bool)(TraitClass, CppPassByVoidPtr, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:290
	// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["image", "keylines", "descriptors", "returnFloatDescr"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*", "bool"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_compute_const_const_MatR_vectorLKeyLineGR_MatR_bool(const cv::line_descriptor::BinaryDescriptor* instance, const cv::Mat* image, std::vector<cv::line_descriptor::KeyLine>* keylines, cv::Mat* descriptors, bool returnFloatDescr, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *keylines, *descriptors, returnFloatDescr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptor::compute(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:290
	// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["image", "keylines", "descriptors"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_compute_const_const_MatR_vectorLKeyLineGR_MatR(const cv::line_descriptor::BinaryDescriptor* instance, const cv::Mat* image, std::vector<cv::line_descriptor::KeyLine>* keylines, cv::Mat* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *keylines, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, std::vector<Mat> &, bool)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:299
	// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["images", "keylines", "descriptors", "returnFloatDescr"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "std::vector<cv::Mat>*", "bool"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_compute_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_vectorLMatGR_bool(const cv::line_descriptor::BinaryDescriptor* instance, const std::vector<cv::Mat>* images, std::vector<std::vector<cv::line_descriptor::KeyLine>>* keylines, std::vector<cv::Mat>* descriptors, bool returnFloatDescr, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*images, *keylines, *descriptors, returnFloatDescr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptor::compute(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:299
	// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["images", "keylines", "descriptors"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "std::vector<cv::Mat>*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_compute_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_vectorLMatGR(const cv::line_descriptor::BinaryDescriptor* instance, const std::vector<cv::Mat>* images, std::vector<std::vector<cv::line_descriptor::KeyLine>>* keylines, std::vector<cv::Mat>* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*images, *keylines, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:304
	// ("cv::line_descriptor::BinaryDescriptor::descriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptor_descriptorSize_const(const cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:308
	// ("cv::line_descriptor::BinaryDescriptor::descriptorType", vec![(pred!(const, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptor_descriptorType_const(const cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// defaultNorm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:312
	// ("cv::line_descriptor::BinaryDescriptor::defaultNorm", vec![(pred!(const, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptor_defaultNorm_const(const cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->defaultNorm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, InputArray, std::vector<KeyLine> &, OutputArray, bool, bool)(InputArray, InputArray, CppPassByVoidPtr, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:326
	// ("cv::line_descriptor::BinaryDescriptor::operator()", vec![(pred!(const, ["image", "mask", "keylines", "descriptors", "useProvidedKeyLines", "returnFloatDescr"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::line_descriptor::KeyLine>*", "const cv::_OutputArray*", "bool", "bool"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_operator___const_const__InputArrayR_const__InputArrayR_vectorLKeyLineGR_const__OutputArrayR_bool_bool(const cv::line_descriptor::BinaryDescriptor* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::line_descriptor::KeyLine>* keylines, const cv::_OutputArray* descriptors, bool useProvidedKeyLines, bool returnFloatDescr, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*image, *mask, *keylines, *descriptors, useProvidedKeyLines, returnFloatDescr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptor::operator()(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:326
	// ("cv::line_descriptor::BinaryDescriptor::operator()", vec![(pred!(const, ["image", "mask", "keylines", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::line_descriptor::KeyLine>*", "const cv::_OutputArray*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_operator___const_const__InputArrayR_const__InputArrayR_vectorLKeyLineGR_const__OutputArrayR(const cv::line_descriptor::BinaryDescriptor* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::line_descriptor::KeyLine>* keylines, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*image, *mask, *keylines, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptor::to_Algorithm() generated
	// ("cv::line_descriptor::BinaryDescriptor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_line_descriptor_BinaryDescriptor_to_Algorithm(cv::line_descriptor::BinaryDescriptor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::line_descriptor::BinaryDescriptor::delete() generated
	// ("cv::line_descriptor::BinaryDescriptor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptor_delete(cv::line_descriptor::BinaryDescriptor* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:191
	// ("cv::line_descriptor::BinaryDescriptor::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptor_Params_Params(Result<cv::line_descriptor::BinaryDescriptor::Params*>* ocvrs_return) {
		try {
			cv::line_descriptor::BinaryDescriptor::Params* ret = new cv::line_descriptor::BinaryDescriptor::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:207
	// ("cv::line_descriptor::BinaryDescriptor::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_Params_read_const_FileNodeR(cv::line_descriptor::BinaryDescriptor::Params* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:210
	// ("cv::line_descriptor::BinaryDescriptor::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_Params_write_const_FileStorageR(const cv::line_descriptor::BinaryDescriptor::Params* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptor::Params::numOfOctave_() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:195
	// ("cv::line_descriptor::BinaryDescriptor::Params::numOfOctave_", vec![(pred!(const, [], []), _)]),
	int cv_line_descriptor_BinaryDescriptor_Params_propNumOfOctave__const(const cv::line_descriptor::BinaryDescriptor::Params* instance) {
			int ret = instance->numOfOctave_;
			return ret;
	}

	// cv::line_descriptor::BinaryDescriptor::Params::setNumOfOctave_(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:195
	// ("cv::line_descriptor::BinaryDescriptor::Params::setNumOfOctave_", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_Params_propNumOfOctave__const_int(cv::line_descriptor::BinaryDescriptor::Params* instance, const int val) {
			instance->numOfOctave_ = val;
	}

	// cv::line_descriptor::BinaryDescriptor::Params::widthOfBand_() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:199
	// ("cv::line_descriptor::BinaryDescriptor::Params::widthOfBand_", vec![(pred!(const, [], []), _)]),
	int cv_line_descriptor_BinaryDescriptor_Params_propWidthOfBand__const(const cv::line_descriptor::BinaryDescriptor::Params* instance) {
			int ret = instance->widthOfBand_;
			return ret;
	}

	// cv::line_descriptor::BinaryDescriptor::Params::setWidthOfBand_(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:199
	// ("cv::line_descriptor::BinaryDescriptor::Params::setWidthOfBand_", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_Params_propWidthOfBand__const_int(cv::line_descriptor::BinaryDescriptor::Params* instance, const int val) {
			instance->widthOfBand_ = val;
	}

	// cv::line_descriptor::BinaryDescriptor::Params::reductionRatio() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:202
	// ("cv::line_descriptor::BinaryDescriptor::Params::reductionRatio", vec![(pred!(const, [], []), _)]),
	int cv_line_descriptor_BinaryDescriptor_Params_propReductionRatio_const(const cv::line_descriptor::BinaryDescriptor::Params* instance) {
			int ret = instance->reductionRatio;
			return ret;
	}

	// cv::line_descriptor::BinaryDescriptor::Params::setReductionRatio(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:202
	// ("cv::line_descriptor::BinaryDescriptor::Params::setReductionRatio", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_Params_propReductionRatio_const_int(cv::line_descriptor::BinaryDescriptor::Params* instance, const int val) {
			instance->reductionRatio = val;
	}

	// cv::line_descriptor::BinaryDescriptor::Params::ksize_() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:204
	// ("cv::line_descriptor::BinaryDescriptor::Params::ksize_", vec![(pred!(const, [], []), _)]),
	int cv_line_descriptor_BinaryDescriptor_Params_propKsize__const(const cv::line_descriptor::BinaryDescriptor::Params* instance) {
			int ret = instance->ksize_;
			return ret;
	}

	// cv::line_descriptor::BinaryDescriptor::Params::setKsize_(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:204
	// ("cv::line_descriptor::BinaryDescriptor::Params::setKsize_", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_line_descriptor_BinaryDescriptor_Params_propKsize__const_int(cv::line_descriptor::BinaryDescriptor::Params* instance, const int val) {
			instance->ksize_ = val;
	}

	// cv::line_descriptor::BinaryDescriptor::Params::delete() generated
	// ("cv::line_descriptor::BinaryDescriptor::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptor_Params_delete(cv::line_descriptor::BinaryDescriptor::Params* instance) {
			delete instance;
	}

	// match(const Mat &, const Mat &, std::vector<DMatch> &, const Mat &)(TraitClass, TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1031
	// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "mask"], ["const cv::Mat*", "const cv::Mat*", "std::vector<cv::DMatch>*", "const cv::Mat*"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_match_const_const_MatR_const_MatR_vectorLDMatchGR_const_MatR(const cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, const cv::Mat* trainDescriptors, std::vector<cv::DMatch>* matches, const cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *trainDescriptors, *matches, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptorMatcher::match(TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1031
	// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches"], ["const cv::Mat*", "const cv::Mat*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_match_const_const_MatR_const_MatR_vectorLDMatchGR(const cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, const cv::Mat* trainDescriptors, std::vector<cv::DMatch>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *trainDescriptors, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// match(const Mat &, std::vector<DMatch> &, const std::vector<Mat> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1040
	// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches", "masks"], ["const cv::Mat*", "std::vector<cv::DMatch>*", "const std::vector<cv::Mat>*"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_match_const_MatR_vectorLDMatchGR_const_vectorLMatGR(cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, std::vector<cv::DMatch>* matches, const std::vector<cv::Mat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *matches, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptorMatcher::match(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1040
	// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches"], ["const cv::Mat*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_match_const_MatR_vectorLDMatchGR(cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, std::vector<cv::DMatch>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// knnMatch(const Mat &, const Mat &, std::vector<std::vector<DMatch>> &, int, const Mat &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1053
	// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k", "mask", "compactResult"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int", "const cv::Mat*", "bool"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_int_const_MatR_bool(const cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, const cv::Mat* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::Mat* mask, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *trainDescriptors, *matches, k, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptorMatcher::knnMatch(TraitClass, TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1053
	// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_int(const cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, const cv::Mat* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *trainDescriptors, *matches, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// knnMatch(const Mat &, std::vector<std::vector<DMatch>> &, int, const std::vector<Mat> &, bool)(TraitClass, CppPassByVoidPtr, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1066
	// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k", "masks", "compactResult"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int", "const std::vector<cv::Mat>*", "bool"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_MatR_vectorLvectorLDMatchGGR_int_const_vectorLMatGR_bool(cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const std::vector<cv::Mat>* masks, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *matches, k, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptorMatcher::knnMatch(TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1066
	// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_MatR_vectorLvectorLDMatchGGR_int(cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *matches, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusMatch(const Mat &, const Mat &, std::vector<std::vector<DMatch>> &, float, const Mat &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1080
	// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance", "mask", "compactResult"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float", "const cv::Mat*", "bool"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_float_const_MatR_bool(const cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, const cv::Mat* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::Mat* mask, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *trainDescriptors, *matches, maxDistance, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch(TraitClass, TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1080
	// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_float(const cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, const cv::Mat* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *trainDescriptors, *matches, maxDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusMatch(const Mat &, std::vector<std::vector<DMatch>> &, float, const std::vector<Mat> &, bool)(TraitClass, CppPassByVoidPtr, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1093
	// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance", "masks", "compactResult"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float", "const std::vector<cv::Mat>*", "bool"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_MatR_vectorLvectorLDMatchGGR_float_const_vectorLMatGR_bool(cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const std::vector<cv::Mat>* masks, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *matches, maxDistance, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch(TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1093
	// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_MatR_vectorLvectorLDMatchGGR_float(cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *matches, maxDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// add(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1104
	// ("cv::line_descriptor::BinaryDescriptorMatcher::add", vec![(pred!(mut, ["descriptors"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_add_const_vectorLMatGR(cv::line_descriptor::BinaryDescriptorMatcher* instance, const std::vector<cv::Mat>* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// train()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1111
	// ("cv::line_descriptor::BinaryDescriptorMatcher::train", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_train(cv::line_descriptor::BinaryDescriptorMatcher* instance, ResultVoid* ocvrs_return) {
		try {
			instance->train();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBinaryDescriptorMatcher()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1115
	// ("cv::line_descriptor::BinaryDescriptorMatcher::createBinaryDescriptorMatcher", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_createBinaryDescriptorMatcher(Result<cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher> ret = cv::line_descriptor::BinaryDescriptorMatcher::createBinaryDescriptorMatcher();
			Ok(new cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1119
	// ("cv::line_descriptor::BinaryDescriptorMatcher::clear", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_clear(cv::line_descriptor::BinaryDescriptorMatcher* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BinaryDescriptorMatcher()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:1125
	// ("cv::line_descriptor::BinaryDescriptorMatcher::BinaryDescriptorMatcher", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_BinaryDescriptorMatcher(Result<cv::line_descriptor::BinaryDescriptorMatcher*>* ocvrs_return) {
		try {
			cv::line_descriptor::BinaryDescriptorMatcher* ret = new cv::line_descriptor::BinaryDescriptorMatcher();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::BinaryDescriptorMatcher::to_Algorithm() generated
	// ("cv::line_descriptor::BinaryDescriptorMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_line_descriptor_BinaryDescriptorMatcher_to_Algorithm(cv::line_descriptor::BinaryDescriptorMatcher* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::line_descriptor::BinaryDescriptorMatcher::delete() generated
	// ("cv::line_descriptor::BinaryDescriptorMatcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_BinaryDescriptorMatcher_delete(cv::line_descriptor::BinaryDescriptorMatcher* instance) {
			delete instance;
	}

	// getStartPoint()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:146
	// ("cv::line_descriptor::KeyLine::getStartPoint", vec![(pred!(const, [], []), _)]),
	void cv_line_descriptor_KeyLine_getStartPoint_const(const cv::line_descriptor::KeyLine* instance, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getStartPoint();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEndPoint()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:152
	// ("cv::line_descriptor::KeyLine::getEndPoint", vec![(pred!(const, [], []), _)]),
	void cv_line_descriptor_KeyLine_getEndPoint_const(const cv::line_descriptor::KeyLine* instance, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getEndPoint();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStartPointInOctave()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:158
	// ("cv::line_descriptor::KeyLine::getStartPointInOctave", vec![(pred!(const, [], []), _)]),
	void cv_line_descriptor_KeyLine_getStartPointInOctave_const(const cv::line_descriptor::KeyLine* instance, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getStartPointInOctave();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEndPointInOctave()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:164
	// ("cv::line_descriptor::KeyLine::getEndPointInOctave", vec![(pred!(const, [], []), _)]),
	void cv_line_descriptor_KeyLine_getEndPointInOctave_const(const cv::line_descriptor::KeyLine* instance, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getEndPointInOctave();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// KeyLine()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:170
	// ("cv::line_descriptor::KeyLine::KeyLine", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_KeyLine_KeyLine(Result<cv::line_descriptor::KeyLine>* ocvrs_return) {
		try {
			cv::line_descriptor::KeyLine ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// LSDDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:929
	// ("cv::line_descriptor::LSDDetector::LSDDetector", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_LSDDetector_LSDDetector(Result<cv::line_descriptor::LSDDetector*>* ocvrs_return) {
		try {
			cv::line_descriptor::LSDDetector* ret = new cv::line_descriptor::LSDDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// LSDDetector(LSDParam)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:934
	// ("cv::line_descriptor::LSDDetector::LSDDetector", vec![(pred!(mut, ["_params"], ["cv::line_descriptor::LSDParam"]), _)]),
	void cv_line_descriptor_LSDDetector_LSDDetector_LSDParam(cv::line_descriptor::LSDParam* _params, Result<cv::line_descriptor::LSDDetector*>* ocvrs_return) {
		try {
			cv::line_descriptor::LSDDetector* ret = new cv::line_descriptor::LSDDetector(*_params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLSDDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:941
	// ("cv::line_descriptor::LSDDetector::createLSDDetector", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_LSDDetector_createLSDDetector(Result<cv::Ptr<cv::line_descriptor::LSDDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::line_descriptor::LSDDetector> ret = cv::line_descriptor::LSDDetector::createLSDDetector();
			Ok(new cv::Ptr<cv::line_descriptor::LSDDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLSDDetector(LSDParam)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:944
	// ("cv::line_descriptor::LSDDetector::createLSDDetector", vec![(pred!(mut, ["params"], ["cv::line_descriptor::LSDParam"]), _)]),
	void cv_line_descriptor_LSDDetector_createLSDDetector_LSDParam(cv::line_descriptor::LSDParam* params, Result<cv::Ptr<cv::line_descriptor::LSDDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::line_descriptor::LSDDetector> ret = cv::line_descriptor::LSDDetector::createLSDDetector(*params);
			Ok(new cv::Ptr<cv::line_descriptor::LSDDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(const Mat &, std::vector<KeyLine> &, int, int, const Mat &)(TraitClass, CppPassByVoidPtr, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:955
	// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(mut, ["image", "keypoints", "scale", "numOctaves", "mask"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "int", "int", "const cv::Mat*"]), _)]),
	void cv_line_descriptor_LSDDetector_detect_const_MatR_vectorLKeyLineGR_int_int_const_MatR(cv::line_descriptor::LSDDetector* instance, const cv::Mat* image, std::vector<cv::line_descriptor::KeyLine>* keypoints, int scale, int numOctaves, const cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints, scale, numOctaves, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::LSDDetector::detect(TraitClass, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:955
	// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(mut, ["image", "keypoints", "scale", "numOctaves"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "int", "int"]), _)]),
	void cv_line_descriptor_LSDDetector_detect_const_MatR_vectorLKeyLineGR_int_int(cv::line_descriptor::LSDDetector* instance, const cv::Mat* image, std::vector<cv::line_descriptor::KeyLine>* keypoints, int scale, int numOctaves, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints, scale, numOctaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, int, int, const std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:964
	// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(const, ["images", "keylines", "scale", "numOctaves", "masks"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "int", "int", "const std::vector<cv::Mat>*"]), _)]),
	void cv_line_descriptor_LSDDetector_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_int_int_const_vectorLMatGR(const cv::line_descriptor::LSDDetector* instance, const std::vector<cv::Mat>* images, std::vector<std::vector<cv::line_descriptor::KeyLine>>* keylines, int scale, int numOctaves, const std::vector<cv::Mat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*images, *keylines, scale, numOctaves, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::LSDDetector::detect(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:964
	// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(const, ["images", "keylines", "scale", "numOctaves"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "int", "int"]), _)]),
	void cv_line_descriptor_LSDDetector_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_int_int(const cv::line_descriptor::LSDDetector* instance, const std::vector<cv::Mat>* images, std::vector<std::vector<cv::line_descriptor::KeyLine>>* keylines, int scale, int numOctaves, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*images, *keylines, scale, numOctaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line_descriptor::LSDDetector::to_Algorithm() generated
	// ("cv::line_descriptor::LSDDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_line_descriptor_LSDDetector_to_Algorithm(cv::line_descriptor::LSDDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::line_descriptor::LSDDetector::delete() generated
	// ("cv::line_descriptor::LSDDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_LSDDetector_delete(cv::line_descriptor::LSDDetector* instance) {
			delete instance;
	}

	// LSDParam()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/line_descriptor/descriptor.hpp:914
	// ("cv::line_descriptor::LSDParam::LSDParam", vec![(pred!(mut, [], []), _)]),
	void cv_line_descriptor_LSDParam_LSDParam(Result<cv::line_descriptor::LSDParam>* ocvrs_return) {
		try {
			cv::line_descriptor::LSDParam ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}

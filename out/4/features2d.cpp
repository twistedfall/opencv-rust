#include "ocvrs_common.hpp"
#include <opencv2/features2d.hpp>
#include "features2d_types.hpp"

extern "C" {
	// cv::AGAST(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:658
	// ("cv::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
	void cv_AGAST_const__InputArrayR_vectorLKeyPointGR_int(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, ResultVoid* ocvrs_return) {
		try {
			cv::AGAST(*image, *keypoints, threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// AGAST(InputArray, std::vector<KeyPoint> &, int, bool)(InputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:658
	// ("cv::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool"]), _)]),
	void cv_AGAST_const__InputArrayR_vectorLKeyPointGR_int_bool(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, ResultVoid* ocvrs_return) {
		try {
			cv::AGAST(*image, *keypoints, threshold, nonmaxSuppression);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// AGAST(InputArray, std::vector<KeyPoint> &, int, bool, AgastFeatureDetector::DetectorType)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:679
	// ("cv::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "cv::AgastFeatureDetector::DetectorType"]), _)]),
	void cv_AGAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, cv::AgastFeatureDetector::DetectorType type, ResultVoid* ocvrs_return) {
		try {
			cv::AGAST(*image, *keypoints, threshold, nonmaxSuppression, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FAST(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:602
	// ("cv::FAST", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
	void cv_FAST_const__InputArrayR_vectorLKeyPointGR_int(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, ResultVoid* ocvrs_return) {
		try {
			cv::FAST(*image, *keypoints, threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FAST(InputArray, std::vector<KeyPoint> &, int, bool)(InputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:602
	// ("cv::FAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool"]), _)]),
	void cv_FAST_const__InputArrayR_vectorLKeyPointGR_int_bool(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, ResultVoid* ocvrs_return) {
		try {
			cv::FAST(*image, *keypoints, threshold, nonmaxSuppression);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FAST(InputArray, std::vector<KeyPoint> &, int, bool, FastFeatureDetector::DetectorType)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:623
	// ("cv::FAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "cv::FastFeatureDetector::DetectorType"]), _)]),
	void cv_FAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, cv::FastFeatureDetector::DetectorType type, ResultVoid* ocvrs_return) {
		try {
			cv::FAST(*image, *keypoints, threshold, nonmaxSuppression, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeRecallPrecisionCurve(const std::vector<std::vector<DMatch>> &, const std::vector<std::vector<uchar>> &, std::vector<Point2f> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1431
	// ("cv::computeRecallPrecisionCurve", vec![(pred!(mut, ["matches1to2", "correctMatches1to2Mask", "recallPrecisionCurve"], ["const std::vector<std::vector<cv::DMatch>>*", "const std::vector<std::vector<unsigned char>>*", "std::vector<cv::Point2f>*"]), _)]),
	void cv_computeRecallPrecisionCurve_const_vectorLvectorLDMatchGGR_const_vectorLvectorLunsigned_charGGR_vectorLPoint2fGR(const std::vector<std::vector<cv::DMatch>>* matches1to2, const std::vector<std::vector<unsigned char>>* correctMatches1to2Mask, std::vector<cv::Point2f>* recallPrecisionCurve, ResultVoid* ocvrs_return) {
		try {
			cv::computeRecallPrecisionCurve(*matches1to2, *correctMatches1to2Mask, *recallPrecisionCurve);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::drawKeypoints(InputArray, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1372
	// ("cv::drawKeypoints", vec![(pred!(mut, ["image", "keypoints", "outImage"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputOutputArray*"]), _)]),
	void cv_drawKeypoints_const__InputArrayR_const_vectorLKeyPointGR_const__InputOutputArrayR(const cv::_InputArray* image, const std::vector<cv::KeyPoint>* keypoints, const cv::_InputOutputArray* outImage, ResultVoid* ocvrs_return) {
		try {
			cv::drawKeypoints(*image, *keypoints, *outImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawKeypoints(InputArray, const std::vector<KeyPoint> &, InputOutputArray, const Scalar &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputOutputArray, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1372
	// ("cv::drawKeypoints", vec![(pred!(mut, ["image", "keypoints", "outImage", "color", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "cv::DrawMatchesFlags"]), _)]),
	void cv_drawKeypoints_const__InputArrayR_const_vectorLKeyPointGR_const__InputOutputArrayR_const_ScalarR_DrawMatchesFlags(const cv::_InputArray* image, const std::vector<cv::KeyPoint>* keypoints, const cv::_InputOutputArray* outImage, const cv::Scalar* color, cv::DrawMatchesFlags flags, ResultVoid* ocvrs_return) {
		try {
			cv::drawKeypoints(*image, *keypoints, *outImage, *color, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1397
	// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*"]), _)]),
	void cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, const cv::_InputOutputArray* outImg, ResultVoid* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, InputOutputArray, const Scalar &, const Scalar &, const std::vector<char> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1397
	// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<char>*", "cv::DrawMatchesFlags"]), _)]),
	void cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vectorLcharGR_DrawMatchesFlags(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, const cv::_InputOutputArray* outImg, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, const std::vector<char>* matchesMask, cv::DrawMatchesFlags flags, ResultVoid* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, *matchColor, *singlePointColor, *matchesMask, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1404
	// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchesThickness"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*", "const int"]), _)]),
	void cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_int(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, const cv::_InputOutputArray* outImg, const int matchesThickness, ResultVoid* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, matchesThickness);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, InputOutputArray, const int, const Scalar &, const Scalar &, const std::vector<char> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1404
	// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchesThickness", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*", "const int", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<char>*", "cv::DrawMatchesFlags"]), _)]),
	void cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_int_const_ScalarR_const_ScalarR_const_vectorLcharGR_DrawMatchesFlags(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, const cv::_InputOutputArray* outImg, const int matchesThickness, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, const std::vector<char>* matchesMask, cv::DrawMatchesFlags flags, ResultVoid* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, matchesThickness, *matchColor, *singlePointColor, *matchesMask, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1411
	// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<std::vector<cv::DMatch>>*", "const cv::_InputOutputArray*"]), _)]),
	void cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLvectorLDMatchGGR_const__InputOutputArrayR(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<std::vector<cv::DMatch>>* matches1to2, const cv::_InputOutputArray* outImg, ResultVoid* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<std::vector<DMatch>> &, InputOutputArray, const Scalar &, const Scalar &, const std::vector<std::vector<char>> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1411
	// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<std::vector<cv::DMatch>>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<std::vector<char>>*", "cv::DrawMatchesFlags"]), _)]),
	void cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLvectorLDMatchGGR_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vectorLvectorLcharGGR_DrawMatchesFlags(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<std::vector<cv::DMatch>>* matches1to2, const cv::_InputOutputArray* outImg, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, const std::vector<std::vector<char>>* matchesMask, cv::DrawMatchesFlags flags, ResultVoid* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, *matchColor, *singlePointColor, *matchesMask, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::evaluateFeatureDetector(TraitClass, TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1426
	// ("cv::evaluateFeatureDetector", vec![(pred!(mut, ["img1", "img2", "H1to2", "keypoints1", "keypoints2", "repeatability", "correspCount"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "std::vector<cv::KeyPoint>*", "std::vector<cv::KeyPoint>*", "float*", "int*"]), _)]),
	void cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR(const cv::Mat* img1, const cv::Mat* img2, const cv::Mat* H1to2, std::vector<cv::KeyPoint>* keypoints1, std::vector<cv::KeyPoint>* keypoints2, float* repeatability, int* correspCount, ResultVoid* ocvrs_return) {
		try {
			cv::evaluateFeatureDetector(*img1, *img2, *H1to2, keypoints1, keypoints2, *repeatability, *correspCount);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// evaluateFeatureDetector(const Mat &, const Mat &, const Mat &, std::vector<KeyPoint> *, std::vector<KeyPoint> *, float &, int &, const Ptr<FeatureDetector> &)(TraitClass, TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Indirect, Indirect, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1426
	// ("cv::evaluateFeatureDetector", vec![(pred!(mut, ["img1", "img2", "H1to2", "keypoints1", "keypoints2", "repeatability", "correspCount", "fdetector"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "std::vector<cv::KeyPoint>*", "std::vector<cv::KeyPoint>*", "float*", "int*", "const cv::Ptr<cv::Feature2D>*"]), _)]),
	void cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR_const_PtrLFeature2DGR(const cv::Mat* img1, const cv::Mat* img2, const cv::Mat* H1to2, std::vector<cv::KeyPoint>* keypoints1, std::vector<cv::KeyPoint>* keypoints2, float* repeatability, int* correspCount, const cv::Ptr<cv::Feature2D>* fdetector, ResultVoid* ocvrs_return) {
		try {
			cv::evaluateFeatureDetector(*img1, *img2, *H1to2, keypoints1, keypoints2, *repeatability, *correspCount, *fdetector);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNearestPoint(const std::vector<Point2f> &, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1436
	// ("cv::getNearestPoint", vec![(pred!(mut, ["recallPrecisionCurve", "l_precision"], ["const std::vector<cv::Point2f>*", "float"]), _)]),
	void cv_getNearestPoint_const_vectorLPoint2fGR_float(const std::vector<cv::Point2f>* recallPrecisionCurve, float l_precision, Result<int>* ocvrs_return) {
		try {
			int ret = cv::getNearestPoint(*recallPrecisionCurve, l_precision);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRecall(const std::vector<Point2f> &, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1435
	// ("cv::getRecall", vec![(pred!(mut, ["recallPrecisionCurve", "l_precision"], ["const std::vector<cv::Point2f>*", "float"]), _)]),
	void cv_getRecall_const_vectorLPoint2fGR_float(const std::vector<cv::Point2f>* recallPrecisionCurve, float l_precision, Result<float>* ocvrs_return) {
		try {
			float ret = cv::getRecall(*recallPrecisionCurve, l_precision);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(AKAZE::DescriptorType, int, int, float, int, int, KAZE::DiffusivityType, int)(Enum, Primitive, Primitive, Primitive, Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:884
	// ("cv::AKAZE::create", vec![(pred!(mut, ["descriptor_type", "descriptor_size", "descriptor_channels", "threshold", "nOctaves", "nOctaveLayers", "diffusivity", "max_points"], ["cv::AKAZE::DescriptorType", "int", "int", "float", "int", "int", "cv::KAZE::DiffusivityType", "int"]), _)]),
	void cv_AKAZE_create_DescriptorType_int_int_float_int_int_DiffusivityType_int(cv::AKAZE::DescriptorType descriptor_type, int descriptor_size, int descriptor_channels, float threshold, int nOctaves, int nOctaveLayers, cv::KAZE::DiffusivityType diffusivity, int max_points, Result<cv::Ptr<cv::AKAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AKAZE> ret = cv::AKAZE::create(descriptor_type, descriptor_size, descriptor_channels, threshold, nOctaves, nOctaveLayers, diffusivity, max_points);
			Ok(new cv::Ptr<cv::AKAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::AKAZE::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:884
	// ("cv::AKAZE::create", vec![(pred!(mut, [], []), _)]),
	void cv_AKAZE_create(Result<cv::Ptr<cv::AKAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AKAZE> ret = cv::AKAZE::create();
			Ok(new cv::Ptr<cv::AKAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDescriptorType(AKAZE::DescriptorType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:890
	// ("cv::AKAZE::setDescriptorType", vec![(pred!(mut, ["dtype"], ["cv::AKAZE::DescriptorType"]), _)]),
	void cv_AKAZE_setDescriptorType_DescriptorType(cv::AKAZE* instance, cv::AKAZE::DescriptorType dtype, ResultVoid* ocvrs_return) {
		try {
			instance->setDescriptorType(dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:891
	// ("cv::AKAZE::getDescriptorType", vec![(pred!(const, [], []), _)]),
	void cv_AKAZE_getDescriptorType_const(const cv::AKAZE* instance, Result<cv::AKAZE::DescriptorType>* ocvrs_return) {
		try {
			cv::AKAZE::DescriptorType ret = instance->getDescriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDescriptorSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:893
	// ("cv::AKAZE::setDescriptorSize", vec![(pred!(mut, ["dsize"], ["int"]), _)]),
	void cv_AKAZE_setDescriptorSize_int(cv::AKAZE* instance, int dsize, ResultVoid* ocvrs_return) {
		try {
			instance->setDescriptorSize(dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:894
	// ("cv::AKAZE::getDescriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_AKAZE_getDescriptorSize_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDescriptorChannels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:896
	// ("cv::AKAZE::setDescriptorChannels", vec![(pred!(mut, ["dch"], ["int"]), _)]),
	void cv_AKAZE_setDescriptorChannels_int(cv::AKAZE* instance, int dch, ResultVoid* ocvrs_return) {
		try {
			instance->setDescriptorChannels(dch);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorChannels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:897
	// ("cv::AKAZE::getDescriptorChannels", vec![(pred!(const, [], []), _)]),
	void cv_AKAZE_getDescriptorChannels_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorChannels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:899
	// ("cv::AKAZE::setThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	void cv_AKAZE_setThreshold_double(cv::AKAZE* instance, double threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:900
	// ("cv::AKAZE::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_AKAZE_getThreshold_const(const cv::AKAZE* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:902
	// ("cv::AKAZE::setNOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	void cv_AKAZE_setNOctaves_int(cv::AKAZE* instance, int octaves, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:903
	// ("cv::AKAZE::getNOctaves", vec![(pred!(const, [], []), _)]),
	void cv_AKAZE_getNOctaves_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:905
	// ("cv::AKAZE::setNOctaveLayers", vec![(pred!(mut, ["octaveLayers"], ["int"]), _)]),
	void cv_AKAZE_setNOctaveLayers_int(cv::AKAZE* instance, int octaveLayers, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaveLayers(octaveLayers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:906
	// ("cv::AKAZE::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	void cv_AKAZE_getNOctaveLayers_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaveLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDiffusivity(KAZE::DiffusivityType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:908
	// ("cv::AKAZE::setDiffusivity", vec![(pred!(mut, ["diff"], ["cv::KAZE::DiffusivityType"]), _)]),
	void cv_AKAZE_setDiffusivity_DiffusivityType(cv::AKAZE* instance, cv::KAZE::DiffusivityType diff, ResultVoid* ocvrs_return) {
		try {
			instance->setDiffusivity(diff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDiffusivity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:909
	// ("cv::AKAZE::getDiffusivity", vec![(pred!(const, [], []), _)]),
	void cv_AKAZE_getDiffusivity_const(const cv::AKAZE* instance, Result<cv::KAZE::DiffusivityType>* ocvrs_return) {
		try {
			cv::KAZE::DiffusivityType ret = instance->getDiffusivity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:910
	// ("cv::AKAZE::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_AKAZE_getDefaultName_const(const cv::AKAZE* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxPoints(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:912
	// ("cv::AKAZE::setMaxPoints", vec![(pred!(mut, ["max_points"], ["int"]), _)]),
	void cv_AKAZE_setMaxPoints_int(cv::AKAZE* instance, int max_points, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxPoints(max_points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxPoints()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:913
	// ("cv::AKAZE::getMaxPoints", vec![(pred!(const, [], []), _)]),
	void cv_AKAZE_getMaxPoints_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxPoints();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::AKAZE::to_Algorithm() generated
	// ("cv::AKAZE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_AKAZE_to_Algorithm(cv::AKAZE* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::AKAZE::to_Feature2D() generated
	// ("cv::AKAZE::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_AKAZE_to_Feature2D(cv::AKAZE* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::AKAZE::delete() generated
	// ("cv::AKAZE::delete", vec![(pred!(mut, [], []), _)]),
	void cv_AKAZE_delete(cv::AKAZE* instance) {
			delete instance;
	}

	// create(const Ptr<Feature2D> &, int, int, float, float)(CppPassByVoidPtr, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:251
	// ("cv::AffineFeature::create", vec![(pred!(mut, ["backend", "maxTilt", "minTilt", "tiltStep", "rotateStepBase"], ["const cv::Ptr<cv::Feature2D>*", "int", "int", "float", "float"]), _)]),
	void cv_AffineFeature_create_const_PtrLFeature2DGR_int_int_float_float(const cv::Ptr<cv::Feature2D>* backend, int maxTilt, int minTilt, float tiltStep, float rotateStepBase, Result<cv::Ptr<cv::AffineFeature>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AffineFeature> ret = cv::AffineFeature::create(*backend, maxTilt, minTilt, tiltStep, rotateStepBase);
			Ok(new cv::Ptr<cv::AffineFeature>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::AffineFeature::create(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:251
	// ("cv::AffineFeature::create", vec![(pred!(mut, ["backend"], ["const cv::Ptr<cv::Feature2D>*"]), _)]),
	void cv_AffineFeature_create_const_PtrLFeature2DGR(const cv::Ptr<cv::Feature2D>* backend, Result<cv::Ptr<cv::AffineFeature>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AffineFeature> ret = cv::AffineFeature::create(*backend);
			Ok(new cv::Ptr<cv::AffineFeature>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setViewParams(const std::vector<float> &, const std::vector<float> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:254
	// ("cv::AffineFeature::setViewParams", vec![(pred!(mut, ["tilts", "rolls"], ["const std::vector<float>*", "const std::vector<float>*"]), _)]),
	void cv_AffineFeature_setViewParams_const_vectorLfloatGR_const_vectorLfloatGR(cv::AffineFeature* instance, const std::vector<float>* tilts, const std::vector<float>* rolls, ResultVoid* ocvrs_return) {
		try {
			instance->setViewParams(*tilts, *rolls);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getViewParams(std::vector<float> &, std::vector<float> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:255
	// ("cv::AffineFeature::getViewParams", vec![(pred!(const, ["tilts", "rolls"], ["std::vector<float>*", "std::vector<float>*"]), _)]),
	void cv_AffineFeature_getViewParams_const_vectorLfloatGR_vectorLfloatGR(const cv::AffineFeature* instance, std::vector<float>* tilts, std::vector<float>* rolls, ResultVoid* ocvrs_return) {
		try {
			instance->getViewParams(*tilts, *rolls);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:256
	// ("cv::AffineFeature::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_AffineFeature_getDefaultName_const(const cv::AffineFeature* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::AffineFeature::to_Algorithm() generated
	// ("cv::AffineFeature::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_AffineFeature_to_Algorithm(cv::AffineFeature* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::AffineFeature::to_Feature2D() generated
	// ("cv::AffineFeature::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_AffineFeature_to_Feature2D(cv::AffineFeature* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::AffineFeature::delete() generated
	// ("cv::AffineFeature::delete", vec![(pred!(mut, [], []), _)]),
	void cv_AffineFeature_delete(cv::AffineFeature* instance) {
			delete instance;
	}

	// create(int, bool, AgastFeatureDetector::DetectorType)(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:642
	// ("cv::AgastFeatureDetector::create", vec![(pred!(mut, ["threshold", "nonmaxSuppression", "type"], ["int", "bool", "cv::AgastFeatureDetector::DetectorType"]), _)]),
	void cv_AgastFeatureDetector_create_int_bool_DetectorType(int threshold, bool nonmaxSuppression, cv::AgastFeatureDetector::DetectorType type, Result<cv::Ptr<cv::AgastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AgastFeatureDetector> ret = cv::AgastFeatureDetector::create(threshold, nonmaxSuppression, type);
			Ok(new cv::Ptr<cv::AgastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::AgastFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:642
	// ("cv::AgastFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_AgastFeatureDetector_create(Result<cv::Ptr<cv::AgastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AgastFeatureDetector> ret = cv::AgastFeatureDetector::create();
			Ok(new cv::Ptr<cv::AgastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:646
	// ("cv::AgastFeatureDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	void cv_AgastFeatureDetector_setThreshold_int(cv::AgastFeatureDetector* instance, int threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:647
	// ("cv::AgastFeatureDetector::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_AgastFeatureDetector_getThreshold_const(const cv::AgastFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNonmaxSuppression(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:649
	// ("cv::AgastFeatureDetector::setNonmaxSuppression", vec![(pred!(mut, ["f"], ["bool"]), _)]),
	void cv_AgastFeatureDetector_setNonmaxSuppression_bool(cv::AgastFeatureDetector* instance, bool f, ResultVoid* ocvrs_return) {
		try {
			instance->setNonmaxSuppression(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNonmaxSuppression()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:650
	// ("cv::AgastFeatureDetector::getNonmaxSuppression", vec![(pred!(const, [], []), _)]),
	void cv_AgastFeatureDetector_getNonmaxSuppression_const(const cv::AgastFeatureDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getNonmaxSuppression();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setType(AgastFeatureDetector::DetectorType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:652
	// ("cv::AgastFeatureDetector::setType", vec![(pred!(mut, ["type"], ["cv::AgastFeatureDetector::DetectorType"]), _)]),
	void cv_AgastFeatureDetector_setType_DetectorType(cv::AgastFeatureDetector* instance, cv::AgastFeatureDetector::DetectorType type, ResultVoid* ocvrs_return) {
		try {
			instance->setType(type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:653
	// ("cv::AgastFeatureDetector::getType", vec![(pred!(const, [], []), _)]),
	void cv_AgastFeatureDetector_getType_const(const cv::AgastFeatureDetector* instance, Result<cv::AgastFeatureDetector::DetectorType>* ocvrs_return) {
		try {
			cv::AgastFeatureDetector::DetectorType ret = instance->getType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:654
	// ("cv::AgastFeatureDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_AgastFeatureDetector_getDefaultName_const(const cv::AgastFeatureDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::AgastFeatureDetector::to_Algorithm() generated
	// ("cv::AgastFeatureDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_AgastFeatureDetector_to_Algorithm(cv::AgastFeatureDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::AgastFeatureDetector::to_Feature2D() generated
	// ("cv::AgastFeatureDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_AgastFeatureDetector_to_Feature2D(cv::AgastFeatureDetector* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::AgastFeatureDetector::delete() generated
	// ("cv::AgastFeatureDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_AgastFeatureDetector_delete(cv::AgastFeatureDetector* instance) {
			delete instance;
	}

	// BFMatcher(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1253
	// ("cv::BFMatcher::BFMatcher", vec![(pred!(mut, ["normType", "crossCheck"], ["int", "bool"]), _)]),
	void cv_BFMatcher_BFMatcher_int_bool(int normType, bool crossCheck, Result<cv::BFMatcher*>* ocvrs_return) {
		try {
			cv::BFMatcher* ret = new cv::BFMatcher(normType, crossCheck);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BFMatcher::BFMatcher() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1253
	// ("cv::BFMatcher::BFMatcher", vec![(pred!(mut, [], []), _)]),
	void cv_BFMatcher_BFMatcher(Result<cv::BFMatcher*>* ocvrs_return) {
		try {
			cv::BFMatcher* ret = new cv::BFMatcher();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1257
	// ("cv::BFMatcher::isMaskSupported", vec![(pred!(const, [], []), _)]),
	void cv_BFMatcher_isMaskSupported_const(const cv::BFMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMaskSupported();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1271
	// ("cv::BFMatcher::create", vec![(pred!(mut, ["normType", "crossCheck"], ["int", "bool"]), _)]),
	void cv_BFMatcher_create_int_bool(int normType, bool crossCheck, Result<cv::Ptr<cv::BFMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BFMatcher> ret = cv::BFMatcher::create(normType, crossCheck);
			Ok(new cv::Ptr<cv::BFMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BFMatcher::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1271
	// ("cv::BFMatcher::create", vec![(pred!(mut, [], []), _)]),
	void cv_BFMatcher_create(Result<cv::Ptr<cv::BFMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BFMatcher> ret = cv::BFMatcher::create();
			Ok(new cv::Ptr<cv::BFMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1273
	// ("cv::BFMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
	void cv_BFMatcher_clone_const_bool(const cv::BFMatcher* instance, bool emptyTrainData, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BFMatcher::clone() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1273
	// ("cv::BFMatcher::clone", vec![(pred!(const, [], []), _)]),
	void cv_BFMatcher_clone_const(const cv::BFMatcher* instance, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone();
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BFMatcher::to_Algorithm() generated
	// ("cv::BFMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_BFMatcher_to_Algorithm(cv::BFMatcher* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::BFMatcher::to_DescriptorMatcher() generated
	// ("cv::BFMatcher::to_DescriptorMatcher", vec![(pred!(mut, [], []), _)]),
	cv::DescriptorMatcher* cv_BFMatcher_to_DescriptorMatcher(cv::BFMatcher* instance) {
			return dynamic_cast<cv::DescriptorMatcher*>(instance);
	}

	// cv::BFMatcher::delete() generated
	// ("cv::BFMatcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_BFMatcher_delete(cv::BFMatcher* instance) {
			delete instance;
	}

	// BOWImgDescriptorExtractor(const Ptr<Feature2D> &, const Ptr<DescriptorMatcher> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1541
	// ("cv::BOWImgDescriptorExtractor::BOWImgDescriptorExtractor", vec![(pred!(mut, ["dextractor", "dmatcher"], ["const cv::Ptr<cv::Feature2D>*", "const cv::Ptr<cv::DescriptorMatcher>*"]), _)]),
	void cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLFeature2DGR_const_PtrLDescriptorMatcherGR(const cv::Ptr<cv::Feature2D>* dextractor, const cv::Ptr<cv::DescriptorMatcher>* dmatcher, Result<cv::BOWImgDescriptorExtractor*>* ocvrs_return) {
		try {
			cv::BOWImgDescriptorExtractor* ret = new cv::BOWImgDescriptorExtractor(*dextractor, *dmatcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BOWImgDescriptorExtractor(const Ptr<DescriptorMatcher> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1544
	// ("cv::BOWImgDescriptorExtractor::BOWImgDescriptorExtractor", vec![(pred!(mut, ["dmatcher"], ["const cv::Ptr<cv::DescriptorMatcher>*"]), _)]),
	void cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLDescriptorMatcherGR(const cv::Ptr<cv::DescriptorMatcher>* dmatcher, Result<cv::BOWImgDescriptorExtractor*>* ocvrs_return) {
		try {
			cv::BOWImgDescriptorExtractor* ret = new cv::BOWImgDescriptorExtractor(*dmatcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVocabulary(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1552
	// ("cv::BOWImgDescriptorExtractor::setVocabulary", vec![(pred!(mut, ["vocabulary"], ["const cv::Mat*"]), _)]),
	void cv_BOWImgDescriptorExtractor_setVocabulary_const_MatR(cv::BOWImgDescriptorExtractor* instance, const cv::Mat* vocabulary, ResultVoid* ocvrs_return) {
		try {
			instance->setVocabulary(*vocabulary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVocabulary()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1556
	// ("cv::BOWImgDescriptorExtractor::getVocabulary", vec![(pred!(const, [], []), _)]),
	void cv_BOWImgDescriptorExtractor_getVocabulary_const(const cv::BOWImgDescriptorExtractor* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->getVocabulary();
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, std::vector<KeyPoint> &, OutputArray, std::vector<std::vector<int>> *, Mat *)(InputArray, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1568
	// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor", "pointIdxsOfClusters", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*", "std::vector<std::vector<int>>*", "cv::Mat*"]), _)]),
	void cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR_vectorLvectorLintGGX_MatX(cv::BOWImgDescriptorExtractor* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* imgDescriptor, std::vector<std::vector<int>>* pointIdxsOfClusters, cv::Mat* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *keypoints, *imgDescriptor, pointIdxsOfClusters, descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BOWImgDescriptorExtractor::compute(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1568
	// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	void cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(cv::BOWImgDescriptorExtractor* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* imgDescriptor, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *keypoints, *imgDescriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, OutputArray, std::vector<std::vector<int>> *)(InputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1577
	// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["keypointDescriptors", "imgDescriptor", "pointIdxsOfClusters"], ["const cv::_InputArray*", "const cv::_OutputArray*", "std::vector<std::vector<int>>*"]), _)]),
	void cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR_vectorLvectorLintGGX(cv::BOWImgDescriptorExtractor* instance, const cv::_InputArray* keypointDescriptors, const cv::_OutputArray* imgDescriptor, std::vector<std::vector<int>>* pointIdxsOfClusters, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*keypointDescriptors, *imgDescriptor, pointIdxsOfClusters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BOWImgDescriptorExtractor::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1577
	// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["keypointDescriptors", "imgDescriptor"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR(cv::BOWImgDescriptorExtractor* instance, const cv::_InputArray* keypointDescriptors, const cv::_OutputArray* imgDescriptor, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*keypointDescriptors, *imgDescriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute2(const Mat &, std::vector<KeyPoint> &, Mat &)(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1581
	// ("cv::BOWImgDescriptorExtractor::compute2", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor"], ["const cv::Mat*", "std::vector<cv::KeyPoint>*", "cv::Mat*"]), _)]),
	void cv_BOWImgDescriptorExtractor_compute2_const_MatR_vectorLKeyPointGR_MatR(cv::BOWImgDescriptorExtractor* instance, const cv::Mat* image, std::vector<cv::KeyPoint>* keypoints, cv::Mat* imgDescriptor, ResultVoid* ocvrs_return) {
		try {
			instance->compute2(*image, *keypoints, *imgDescriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1586
	// ("cv::BOWImgDescriptorExtractor::descriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_BOWImgDescriptorExtractor_descriptorSize_const(const cv::BOWImgDescriptorExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1590
	// ("cv::BOWImgDescriptorExtractor::descriptorType", vec![(pred!(const, [], []), _)]),
	void cv_BOWImgDescriptorExtractor_descriptorType_const(const cv::BOWImgDescriptorExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BOWImgDescriptorExtractor::delete() generated
	// ("cv::BOWImgDescriptorExtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_BOWImgDescriptorExtractor_delete(cv::BOWImgDescriptorExtractor* instance) {
			delete instance;
	}

	// BOWKMeansTrainer(int, const TermCriteria &, int, int)(Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1505
	// ("cv::BOWKMeansTrainer::BOWKMeansTrainer", vec![(pred!(mut, ["clusterCount", "termcrit", "attempts", "flags"], ["int", "const cv::TermCriteria*", "int", "int"]), _)]),
	void cv_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaR_int_int(int clusterCount, const cv::TermCriteria* termcrit, int attempts, int flags, Result<cv::BOWKMeansTrainer*>* ocvrs_return) {
		try {
			cv::BOWKMeansTrainer* ret = new cv::BOWKMeansTrainer(clusterCount, *termcrit, attempts, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BOWKMeansTrainer::BOWKMeansTrainer(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1505
	// ("cv::BOWKMeansTrainer::BOWKMeansTrainer", vec![(pred!(mut, ["clusterCount"], ["int"]), _)]),
	void cv_BOWKMeansTrainer_BOWKMeansTrainer_int(int clusterCount, Result<cv::BOWKMeansTrainer*>* ocvrs_return) {
		try {
			cv::BOWKMeansTrainer* ret = new cv::BOWKMeansTrainer(clusterCount);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cluster()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1510
	// ("cv::BOWKMeansTrainer::cluster", vec![(pred!(const, [], []), _)]),
	void cv_BOWKMeansTrainer_cluster_const(const cv::BOWKMeansTrainer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cluster(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1511
	// ("cv::BOWKMeansTrainer::cluster", vec![(pred!(const, ["descriptors"], ["const cv::Mat*"]), _)]),
	void cv_BOWKMeansTrainer_cluster_const_const_MatR(const cv::BOWKMeansTrainer* instance, const cv::Mat* descriptors, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster(*descriptors);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BOWKMeansTrainer::to_BOWTrainer() generated
	// ("cv::BOWKMeansTrainer::to_BOWTrainer", vec![(pred!(mut, [], []), _)]),
	cv::BOWTrainer* cv_BOWKMeansTrainer_to_BOWTrainer(cv::BOWKMeansTrainer* instance) {
			return dynamic_cast<cv::BOWTrainer*>(instance);
	}

	// cv::BOWKMeansTrainer::delete() generated
	// ("cv::BOWKMeansTrainer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_BOWKMeansTrainer_delete(cv::BOWKMeansTrainer* instance) {
			delete instance;
	}

	// add(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1465
	// ("cv::BOWTrainer::add", vec![(pred!(mut, ["descriptors"], ["const cv::Mat*"]), _)]),
	void cv_BOWTrainer_add_const_MatR(cv::BOWTrainer* instance, const cv::Mat* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptors()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1469
	// ("cv::BOWTrainer::getDescriptors", vec![(pred!(const, [], []), _)]),
	void cv_BOWTrainer_getDescriptors_const(const cv::BOWTrainer* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getDescriptors();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorsCount()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1473
	// ("cv::BOWTrainer::descriptorsCount", vec![(pred!(const, [], []), _)]),
	void cv_BOWTrainer_descriptorsCount_const(const cv::BOWTrainer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorsCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1475
	// ("cv::BOWTrainer::clear", vec![(pred!(mut, [], []), _)]),
	void cv_BOWTrainer_clear(cv::BOWTrainer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cluster()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1478
	// ("cv::BOWTrainer::cluster", vec![(pred!(const, [], []), _)]),
	void cv_BOWTrainer_cluster_const(const cv::BOWTrainer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cluster(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1489
	// ("cv::BOWTrainer::cluster", vec![(pred!(const, ["descriptors"], ["const cv::Mat*"]), _)]),
	void cv_BOWTrainer_cluster_const_const_MatR(const cv::BOWTrainer* instance, const cv::Mat* descriptors, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster(*descriptors);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BOWTrainer::to_BOWKMeansTrainer() generated
	// ("cv::BOWTrainer::to_BOWKMeansTrainer", vec![(pred!(mut, [], []), _)]),
	cv::BOWKMeansTrainer* cv_BOWTrainer_to_BOWKMeansTrainer(cv::BOWTrainer* instance) {
			return dynamic_cast<cv::BOWKMeansTrainer*>(instance);
	}

	// cv::BOWTrainer::delete() generated
	// ("cv::BOWTrainer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_BOWTrainer_delete(cv::BOWTrainer* instance) {
			delete instance;
	}

	// create(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:363
	// ("cv::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "patternScale"], ["int", "int", "float"]), _)]),
	void cv_BRISK_create_int_int_float(int thresh, int octaves, float patternScale, Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(thresh, octaves, patternScale);
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BRISK::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:363
	// ("cv::BRISK::create", vec![(pred!(mut, [], []), _)]),
	void cv_BRISK_create(Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create();
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:376
	// ("cv::BRISK::create", vec![(pred!(mut, ["radiusList", "numberList", "dMax", "dMin", "indexChange"], ["const std::vector<float>*", "const std::vector<int>*", "float", "float", "const std::vector<int>*"]), _)]),
	void cv_BRISK_create_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(const std::vector<float>* radiusList, const std::vector<int>* numberList, float dMax, float dMin, const std::vector<int>* indexChange, Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(*radiusList, *numberList, dMax, dMin, *indexChange);
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BRISK::create(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:376
	// ("cv::BRISK::create", vec![(pred!(mut, ["radiusList", "numberList"], ["const std::vector<float>*", "const std::vector<int>*"]), _)]),
	void cv_BRISK_create_const_vectorLfloatGR_const_vectorLintGR(const std::vector<float>* radiusList, const std::vector<int>* numberList, Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(*radiusList, *numberList);
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:392
	// ("cv::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "radiusList", "numberList", "dMax", "dMin", "indexChange"], ["int", "int", "const std::vector<float>*", "const std::vector<int>*", "float", "float", "const std::vector<int>*"]), _)]),
	void cv_BRISK_create_int_int_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(int thresh, int octaves, const std::vector<float>* radiusList, const std::vector<int>* numberList, float dMax, float dMin, const std::vector<int>* indexChange, Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(thresh, octaves, *radiusList, *numberList, dMax, dMin, *indexChange);
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BRISK::create(Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:392
	// ("cv::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "radiusList", "numberList"], ["int", "int", "const std::vector<float>*", "const std::vector<int>*"]), _)]),
	void cv_BRISK_create_int_int_const_vectorLfloatGR_const_vectorLintGR(int thresh, int octaves, const std::vector<float>* radiusList, const std::vector<int>* numberList, Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(thresh, octaves, *radiusList, *numberList);
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:395
	// ("cv::BRISK::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_BRISK_getDefaultName_const(const cv::BRISK* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:400
	// ("cv::BRISK::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	void cv_BRISK_setThreshold_int(cv::BRISK* instance, int threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:401
	// ("cv::BRISK::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_BRISK_getThreshold_const(const cv::BRISK* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:406
	// ("cv::BRISK::setOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	void cv_BRISK_setOctaves_int(cv::BRISK* instance, int octaves, ResultVoid* ocvrs_return) {
		try {
			instance->setOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOctaves()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:407
	// ("cv::BRISK::getOctaves", vec![(pred!(const, [], []), _)]),
	void cv_BRISK_getOctaves_const(const cv::BRISK* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPatternScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:412
	// ("cv::BRISK::setPatternScale", vec![(pred!(mut, ["patternScale"], ["float"]), _)]),
	void cv_BRISK_setPatternScale_float(cv::BRISK* instance, float patternScale, ResultVoid* ocvrs_return) {
		try {
			instance->setPatternScale(patternScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPatternScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:413
	// ("cv::BRISK::getPatternScale", vec![(pred!(const, [], []), _)]),
	void cv_BRISK_getPatternScale_const(const cv::BRISK* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getPatternScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BRISK::to_Algorithm() generated
	// ("cv::BRISK::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_BRISK_to_Algorithm(cv::BRISK* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::BRISK::to_Feature2D() generated
	// ("cv::BRISK::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_BRISK_to_Feature2D(cv::BRISK* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::BRISK::delete() generated
	// ("cv::BRISK::delete", vec![(pred!(mut, [], []), _)]),
	void cv_BRISK_delete(cv::BRISK* instance) {
			delete instance;
	}

	// add(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1017
	// ("cv::DescriptorMatcher::add", vec![(pred!(mut, ["descriptors"], ["const cv::_InputArray*"]), _)]),
	void cv_DescriptorMatcher_add_const__InputArrayR(cv::DescriptorMatcher* instance, const cv::_InputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrainDescriptors()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1021
	// ("cv::DescriptorMatcher::getTrainDescriptors", vec![(pred!(const, [], []), _)]),
	void cv_DescriptorMatcher_getTrainDescriptors_const(const cv::DescriptorMatcher* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getTrainDescriptors();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1025
	// ("cv::DescriptorMatcher::clear", vec![(pred!(mut, [], []), _)]),
	void cv_DescriptorMatcher_clear(cv::DescriptorMatcher* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1029
	// ("cv::DescriptorMatcher::empty", vec![(pred!(const, [], []), _)]),
	void cv_DescriptorMatcher_empty_const(const cv::DescriptorMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1033
	// ("cv::DescriptorMatcher::isMaskSupported", vec![(pred!(const, [], []), _)]),
	void cv_DescriptorMatcher_isMaskSupported_const(const cv::DescriptorMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMaskSupported();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// train()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1042
	// ("cv::DescriptorMatcher::train", vec![(pred!(mut, [], []), _)]),
	void cv_DescriptorMatcher_train(cv::DescriptorMatcher* instance, ResultVoid* ocvrs_return) {
		try {
			instance->train();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// match(InputArray, InputArray, std::vector<DMatch> &, InputArray)(InputArray, InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1060
	// ("cv::DescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*", "const cv::_InputArray*"]), _)]),
	void cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<cv::DMatch>* matches, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *trainDescriptors, *matches, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DescriptorMatcher::match(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1060
	// ("cv::DescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vectorLDMatchGR(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<cv::DMatch>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *trainDescriptors, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// knnMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, int, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1081
	// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k", "mask", "compactResult"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int", "const cv::_InputArray*", "bool"]), _)]),
	void cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::_InputArray* mask, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *trainDescriptors, *matches, k, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DescriptorMatcher::knnMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1081
	// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
	void cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *trainDescriptors, *matches, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, float, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1104
	// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance", "mask", "compactResult"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float", "const cv::_InputArray*", "bool"]), _)]),
	void cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::_InputArray* mask, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *trainDescriptors, *matches, maxDistance, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DescriptorMatcher::radiusMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1104
	// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
	void cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *trainDescriptors, *matches, maxDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// match(InputArray, std::vector<DMatch> &, InputArrayOfArrays)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1115
	// ("cv::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches", "masks"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*", "const cv::_InputArray*"]), _)]),
	void cv_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<cv::DMatch>* matches, const cv::_InputArray* masks, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *matches, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DescriptorMatcher::match(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1115
	// ("cv::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<cv::DMatch>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// knnMatch(InputArray, std::vector<std::vector<DMatch>> &, int, InputArrayOfArrays, bool)(InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1128
	// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k", "masks", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int", "const cv::_InputArray*", "bool"]), _)]),
	void cv_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::_InputArray* masks, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *matches, k, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DescriptorMatcher::knnMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1128
	// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
	void cv_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *matches, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusMatch(InputArray, std::vector<std::vector<DMatch>> &, float, InputArrayOfArrays, bool)(InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1142
	// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance", "masks", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float", "const cv::_InputArray*", "bool"]), _)]),
	void cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::_InputArray* masks, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *matches, maxDistance, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DescriptorMatcher::radiusMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1142
	// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
	void cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *matches, maxDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1146
	// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["fileName"], ["const cv::String*"]), _)]),
	void cv_DescriptorMatcher_write_const_const_StringR(const cv::DescriptorMatcher* instance, const char* fileName, ResultVoid* ocvrs_return) {
		try {
			instance->write(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1152
	// ("cv::DescriptorMatcher::read", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
	void cv_DescriptorMatcher_read_const_StringR(cv::DescriptorMatcher* instance, const char* fileName, ResultVoid* ocvrs_return) {
		try {
			instance->read(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1159
	// ("cv::DescriptorMatcher::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	void cv_DescriptorMatcher_read_const_FileNodeR(cv::DescriptorMatcher* instance, const cv::FileNode* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1161
	// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	void cv_DescriptorMatcher_write_const_FileStorageR(const cv::DescriptorMatcher* instance, cv::FileStorage* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1169
	// ("cv::DescriptorMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
	void cv_DescriptorMatcher_clone_const_bool(const cv::DescriptorMatcher* instance, bool emptyTrainData, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DescriptorMatcher::clone() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1169
	// ("cv::DescriptorMatcher::clone", vec![(pred!(const, [], []), _)]),
	void cv_DescriptorMatcher_clone_const(const cv::DescriptorMatcher* instance, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone();
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1182
	// ("cv::DescriptorMatcher::create", vec![(pred!(mut, ["descriptorMatcherType"], ["const cv::String*"]), _)]),
	void cv_DescriptorMatcher_create_const_StringR(const char* descriptorMatcherType, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = cv::DescriptorMatcher::create(std::string(descriptorMatcherType));
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const DescriptorMatcher::MatcherType &)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1184
	// ("cv::DescriptorMatcher::create", vec![(pred!(mut, ["matcherType"], ["const cv::DescriptorMatcher::MatcherType*"]), _)]),
	void cv_DescriptorMatcher_create_const_MatcherTypeR(const cv::DescriptorMatcher::MatcherType matcherType, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = cv::DescriptorMatcher::create(matcherType);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1188
	// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_DescriptorMatcher_write_const_FileStorageR_const_StringR(const cv::DescriptorMatcher* instance, cv::FileStorage* fs, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(const Ptr<FileStorage> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1190
	// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["fs", "name"], ["const cv::Ptr<cv::FileStorage>*", "const cv::String*"]), _)]),
	void cv_DescriptorMatcher_write_const_const_PtrLFileStorageGR_const_StringR(const cv::DescriptorMatcher* instance, const cv::Ptr<cv::FileStorage>* fs, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DescriptorMatcher::to_BFMatcher() generated
	// ("cv::DescriptorMatcher::to_BFMatcher", vec![(pred!(mut, [], []), _)]),
	cv::BFMatcher* cv_DescriptorMatcher_to_BFMatcher(cv::DescriptorMatcher* instance) {
			return dynamic_cast<cv::BFMatcher*>(instance);
	}

	// cv::DescriptorMatcher::to_FlannBasedMatcher() generated
	// ("cv::DescriptorMatcher::to_FlannBasedMatcher", vec![(pred!(mut, [], []), _)]),
	cv::FlannBasedMatcher* cv_DescriptorMatcher_to_FlannBasedMatcher(cv::DescriptorMatcher* instance) {
			return dynamic_cast<cv::FlannBasedMatcher*>(instance);
	}

	// cv::DescriptorMatcher::to_Algorithm() generated
	// ("cv::DescriptorMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_DescriptorMatcher_to_Algorithm(cv::DescriptorMatcher* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::DescriptorMatcher::delete() generated
	// ("cv::DescriptorMatcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DescriptorMatcher_delete(cv::DescriptorMatcher* instance) {
			delete instance;
	}

	// create(int, bool, FastFeatureDetector::DetectorType)(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:586
	// ("cv::FastFeatureDetector::create", vec![(pred!(mut, ["threshold", "nonmaxSuppression", "type"], ["int", "bool", "cv::FastFeatureDetector::DetectorType"]), _)]),
	void cv_FastFeatureDetector_create_int_bool_DetectorType(int threshold, bool nonmaxSuppression, cv::FastFeatureDetector::DetectorType type, Result<cv::Ptr<cv::FastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FastFeatureDetector> ret = cv::FastFeatureDetector::create(threshold, nonmaxSuppression, type);
			Ok(new cv::Ptr<cv::FastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FastFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:586
	// ("cv::FastFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_FastFeatureDetector_create(Result<cv::Ptr<cv::FastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FastFeatureDetector> ret = cv::FastFeatureDetector::create();
			Ok(new cv::Ptr<cv::FastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:590
	// ("cv::FastFeatureDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	void cv_FastFeatureDetector_setThreshold_int(cv::FastFeatureDetector* instance, int threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:591
	// ("cv::FastFeatureDetector::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_FastFeatureDetector_getThreshold_const(const cv::FastFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNonmaxSuppression(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:593
	// ("cv::FastFeatureDetector::setNonmaxSuppression", vec![(pred!(mut, ["f"], ["bool"]), _)]),
	void cv_FastFeatureDetector_setNonmaxSuppression_bool(cv::FastFeatureDetector* instance, bool f, ResultVoid* ocvrs_return) {
		try {
			instance->setNonmaxSuppression(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNonmaxSuppression()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:594
	// ("cv::FastFeatureDetector::getNonmaxSuppression", vec![(pred!(const, [], []), _)]),
	void cv_FastFeatureDetector_getNonmaxSuppression_const(const cv::FastFeatureDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getNonmaxSuppression();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setType(FastFeatureDetector::DetectorType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:596
	// ("cv::FastFeatureDetector::setType", vec![(pred!(mut, ["type"], ["cv::FastFeatureDetector::DetectorType"]), _)]),
	void cv_FastFeatureDetector_setType_DetectorType(cv::FastFeatureDetector* instance, cv::FastFeatureDetector::DetectorType type, ResultVoid* ocvrs_return) {
		try {
			instance->setType(type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:597
	// ("cv::FastFeatureDetector::getType", vec![(pred!(const, [], []), _)]),
	void cv_FastFeatureDetector_getType_const(const cv::FastFeatureDetector* instance, Result<cv::FastFeatureDetector::DetectorType>* ocvrs_return) {
		try {
			cv::FastFeatureDetector::DetectorType ret = instance->getType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:598
	// ("cv::FastFeatureDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_FastFeatureDetector_getDefaultName_const(const cv::FastFeatureDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FastFeatureDetector::to_Algorithm() generated
	// ("cv::FastFeatureDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_FastFeatureDetector_to_Algorithm(cv::FastFeatureDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::FastFeatureDetector::to_Feature2D() generated
	// ("cv::FastFeatureDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_FastFeatureDetector_to_Feature2D(cv::FastFeatureDetector* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::FastFeatureDetector::delete() generated
	// ("cv::FastFeatureDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_FastFeatureDetector_delete(cv::FastFeatureDetector* instance) {
			delete instance;
	}

	// detect(InputArray, std::vector<KeyPoint> &, InputArray)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:151
	// ("cv::Feature2D::detect", vec![(pred!(mut, ["image", "keypoints", "mask"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_InputArray*"]), _)]),
	void cv_Feature2D_detect_const__InputArrayR_vectorLKeyPointGR_const__InputArrayR(cv::Feature2D* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Feature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:151
	// ("cv::Feature2D::detect", vec![(pred!(mut, ["image", "keypoints"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*"]), _)]),
	void cv_Feature2D_detect_const__InputArrayR_vectorLKeyPointGR(cv::Feature2D* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, InputArrayOfArrays)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:162
	// ("cv::Feature2D::detect", vec![(pred!(mut, ["images", "keypoints", "masks"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*", "const cv::_InputArray*"]), _)]),
	void cv_Feature2D_detect_const__InputArrayR_vectorLvectorLKeyPointGGR_const__InputArrayR(cv::Feature2D* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, const cv::_InputArray* masks, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*images, *keypoints, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Feature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:162
	// ("cv::Feature2D::detect", vec![(pred!(mut, ["images", "keypoints"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*"]), _)]),
	void cv_Feature2D_detect_const__InputArrayR_vectorLvectorLKeyPointGGR(cv::Feature2D* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*images, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, std::vector<KeyPoint> &, OutputArray)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:177
	// ("cv::Feature2D::compute", vec![(pred!(mut, ["image", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	void cv_Feature2D_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(cv::Feature2D* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:191
	// ("cv::Feature2D::compute", vec![(pred!(mut, ["images", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*", "const cv::_OutputArray*"]), _)]),
	void cv_Feature2D_compute_const__InputArrayR_vectorLvectorLKeyPointGGR_const__OutputArrayR(cv::Feature2D* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*images, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndCompute(InputArray, InputArray, std::vector<KeyPoint> &, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:196
	// ("cv::Feature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR_bool(cv::Feature2D* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors, bool useProvidedKeypoints, ResultVoid* ocvrs_return) {
		try {
			instance->detectAndCompute(*image, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Feature2D::detectAndCompute(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:196
	// ("cv::Feature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	void cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(cv::Feature2D* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->detectAndCompute(*image, *mask, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:201
	// ("cv::Feature2D::descriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_Feature2D_descriptorSize_const(const cv::Feature2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:202
	// ("cv::Feature2D::descriptorType", vec![(pred!(const, [], []), _)]),
	void cv_Feature2D_descriptorType_const(const cv::Feature2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// defaultNorm()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:203
	// ("cv::Feature2D::defaultNorm", vec![(pred!(const, [], []), _)]),
	void cv_Feature2D_defaultNorm_const(const cv::Feature2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->defaultNorm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:205
	// ("cv::Feature2D::write", vec![(pred!(const, ["fileName"], ["const cv::String*"]), _)]),
	void cv_Feature2D_write_const_const_StringR(const cv::Feature2D* instance, const char* fileName, ResultVoid* ocvrs_return) {
		try {
			instance->write(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:207
	// ("cv::Feature2D::read", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
	void cv_Feature2D_read_const_StringR(cv::Feature2D* instance, const char* fileName, ResultVoid* ocvrs_return) {
		try {
			instance->read(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:209
	// ("cv::Feature2D::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	void cv_Feature2D_write_const_FileStorageR(const cv::Feature2D* instance, cv::FileStorage* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:212
	// ("cv::Feature2D::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	void cv_Feature2D_read_const_FileNodeR(cv::Feature2D* instance, const cv::FileNode* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:215
	// ("cv::Feature2D::empty", vec![(pred!(const, [], []), _)]),
	void cv_Feature2D_empty_const(const cv::Feature2D* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:216
	// ("cv::Feature2D::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_Feature2D_getDefaultName_const(const cv::Feature2D* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:219
	// ("cv::Feature2D::write", vec![(pred!(const, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_Feature2D_write_const_FileStorageR_const_StringR(const cv::Feature2D* instance, cv::FileStorage* fs, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(const Ptr<FileStorage> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:221
	// ("cv::Feature2D::write", vec![(pred!(const, ["fs", "name"], ["const cv::Ptr<cv::FileStorage>*", "const cv::String*"]), _)]),
	void cv_Feature2D_write_const_const_PtrLFileStorageGR_const_StringR(const cv::Feature2D* instance, const cv::Ptr<cv::FileStorage>* fs, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Feature2D::defaultNew() generated
	// ("cv::Feature2D::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::Feature2D* cv_Feature2D_defaultNew_const() {
			cv::Feature2D* ret = new cv::Feature2D();
			return ret;
	}

	// cv::Feature2D::to_AKAZE() generated
	// ("cv::Feature2D::to_AKAZE", vec![(pred!(mut, [], []), _)]),
	cv::AKAZE* cv_Feature2D_to_AKAZE(cv::Feature2D* instance) {
			return dynamic_cast<cv::AKAZE*>(instance);
	}

	// cv::Feature2D::to_AffineFeature() generated
	// ("cv::Feature2D::to_AffineFeature", vec![(pred!(mut, [], []), _)]),
	cv::AffineFeature* cv_Feature2D_to_AffineFeature(cv::Feature2D* instance) {
			return dynamic_cast<cv::AffineFeature*>(instance);
	}

	// cv::Feature2D::to_AgastFeatureDetector() generated
	// ("cv::Feature2D::to_AgastFeatureDetector", vec![(pred!(mut, [], []), _)]),
	cv::AgastFeatureDetector* cv_Feature2D_to_AgastFeatureDetector(cv::Feature2D* instance) {
			return dynamic_cast<cv::AgastFeatureDetector*>(instance);
	}

	// cv::Feature2D::to_BRISK() generated
	// ("cv::Feature2D::to_BRISK", vec![(pred!(mut, [], []), _)]),
	cv::BRISK* cv_Feature2D_to_BRISK(cv::Feature2D* instance) {
			return dynamic_cast<cv::BRISK*>(instance);
	}

	// cv::Feature2D::to_FastFeatureDetector() generated
	// ("cv::Feature2D::to_FastFeatureDetector", vec![(pred!(mut, [], []), _)]),
	cv::FastFeatureDetector* cv_Feature2D_to_FastFeatureDetector(cv::Feature2D* instance) {
			return dynamic_cast<cv::FastFeatureDetector*>(instance);
	}

	// cv::Feature2D::to_GFTTDetector() generated
	// ("cv::Feature2D::to_GFTTDetector", vec![(pred!(mut, [], []), _)]),
	cv::GFTTDetector* cv_Feature2D_to_GFTTDetector(cv::Feature2D* instance) {
			return dynamic_cast<cv::GFTTDetector*>(instance);
	}

	// cv::Feature2D::to_KAZE() generated
	// ("cv::Feature2D::to_KAZE", vec![(pred!(mut, [], []), _)]),
	cv::KAZE* cv_Feature2D_to_KAZE(cv::Feature2D* instance) {
			return dynamic_cast<cv::KAZE*>(instance);
	}

	// cv::Feature2D::to_MSER() generated
	// ("cv::Feature2D::to_MSER", vec![(pred!(mut, [], []), _)]),
	cv::MSER* cv_Feature2D_to_MSER(cv::Feature2D* instance) {
			return dynamic_cast<cv::MSER*>(instance);
	}

	// cv::Feature2D::to_ORB() generated
	// ("cv::Feature2D::to_ORB", vec![(pred!(mut, [], []), _)]),
	cv::ORB* cv_Feature2D_to_ORB(cv::Feature2D* instance) {
			return dynamic_cast<cv::ORB*>(instance);
	}

	// cv::Feature2D::to_SIFT() generated
	// ("cv::Feature2D::to_SIFT", vec![(pred!(mut, [], []), _)]),
	cv::SIFT* cv_Feature2D_to_SIFT(cv::Feature2D* instance) {
			return dynamic_cast<cv::SIFT*>(instance);
	}

	// cv::Feature2D::to_SimpleBlobDetector() generated
	// ("cv::Feature2D::to_SimpleBlobDetector", vec![(pred!(mut, [], []), _)]),
	cv::SimpleBlobDetector* cv_Feature2D_to_SimpleBlobDetector(cv::Feature2D* instance) {
			return dynamic_cast<cv::SimpleBlobDetector*>(instance);
	}

	// cv::Feature2D::to_Algorithm() generated
	// ("cv::Feature2D::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_Feature2D_to_Algorithm(cv::Feature2D* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::Feature2D::delete() generated
	// ("cv::Feature2D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Feature2D_delete(cv::Feature2D* instance) {
			delete instance;
	}

	// FlannBasedMatcher(const Ptr<flann::IndexParams> &, const Ptr<flann::SearchParams> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1296
	// ("cv::FlannBasedMatcher::FlannBasedMatcher", vec![(pred!(mut, ["indexParams", "searchParams"], ["const cv::Ptr<cv::flann::IndexParams>*", "const cv::Ptr<cv::flann::SearchParams>*"]), _)]),
	void cv_FlannBasedMatcher_FlannBasedMatcher_const_PtrLIndexParamsGR_const_PtrLSearchParamsGR(const cv::Ptr<cv::flann::IndexParams>* indexParams, const cv::Ptr<cv::flann::SearchParams>* searchParams, Result<cv::FlannBasedMatcher*>* ocvrs_return) {
		try {
			cv::FlannBasedMatcher* ret = new cv::FlannBasedMatcher(*indexParams, *searchParams);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FlannBasedMatcher::FlannBasedMatcher() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1296
	// ("cv::FlannBasedMatcher::FlannBasedMatcher", vec![(pred!(mut, [], []), _)]),
	void cv_FlannBasedMatcher_FlannBasedMatcher(Result<cv::FlannBasedMatcher*>* ocvrs_return) {
		try {
			cv::FlannBasedMatcher* ret = new cv::FlannBasedMatcher();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// add(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1299
	// ("cv::FlannBasedMatcher::add", vec![(pred!(mut, ["descriptors"], ["const cv::_InputArray*"]), _)]),
	void cv_FlannBasedMatcher_add_const__InputArrayR(cv::FlannBasedMatcher* instance, const cv::_InputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1300
	// ("cv::FlannBasedMatcher::clear", vec![(pred!(mut, [], []), _)]),
	void cv_FlannBasedMatcher_clear(cv::FlannBasedMatcher* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1303
	// ("cv::FlannBasedMatcher::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	void cv_FlannBasedMatcher_read_const_FileNodeR(cv::FlannBasedMatcher* instance, const cv::FileNode* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1305
	// ("cv::FlannBasedMatcher::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	void cv_FlannBasedMatcher_write_const_FileStorageR(const cv::FlannBasedMatcher* instance, cv::FileStorage* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// train()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1307
	// ("cv::FlannBasedMatcher::train", vec![(pred!(mut, [], []), _)]),
	void cv_FlannBasedMatcher_train(cv::FlannBasedMatcher* instance, ResultVoid* ocvrs_return) {
		try {
			instance->train();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1308
	// ("cv::FlannBasedMatcher::isMaskSupported", vec![(pred!(const, [], []), _)]),
	void cv_FlannBasedMatcher_isMaskSupported_const(const cv::FlannBasedMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMaskSupported();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1310
	// ("cv::FlannBasedMatcher::create", vec![(pred!(mut, [], []), _)]),
	void cv_FlannBasedMatcher_create(Result<cv::Ptr<cv::FlannBasedMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FlannBasedMatcher> ret = cv::FlannBasedMatcher::create();
			Ok(new cv::Ptr<cv::FlannBasedMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1312
	// ("cv::FlannBasedMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
	void cv_FlannBasedMatcher_clone_const_bool(const cv::FlannBasedMatcher* instance, bool emptyTrainData, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FlannBasedMatcher::clone() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1312
	// ("cv::FlannBasedMatcher::clone", vec![(pred!(const, [], []), _)]),
	void cv_FlannBasedMatcher_clone_const(const cv::FlannBasedMatcher* instance, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone();
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FlannBasedMatcher::to_Algorithm() generated
	// ("cv::FlannBasedMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_FlannBasedMatcher_to_Algorithm(cv::FlannBasedMatcher* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::FlannBasedMatcher::to_DescriptorMatcher() generated
	// ("cv::FlannBasedMatcher::to_DescriptorMatcher", vec![(pred!(mut, [], []), _)]),
	cv::DescriptorMatcher* cv_FlannBasedMatcher_to_DescriptorMatcher(cv::FlannBasedMatcher* instance) {
			return dynamic_cast<cv::DescriptorMatcher*>(instance);
	}

	// cv::FlannBasedMatcher::delete() generated
	// ("cv::FlannBasedMatcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_FlannBasedMatcher_delete(cv::FlannBasedMatcher* instance) {
			delete instance;
	}

	// create(int, double, double, int, bool, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:687
	// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "useHarrisDetector", "k"], ["int", "double", "double", "int", "bool", "double"]), _)]),
	void cv_GFTTDetector_create_int_double_double_int_bool_double(int maxCorners, double qualityLevel, double minDistance, int blockSize, bool useHarrisDetector, double k, Result<cv::Ptr<cv::GFTTDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create(maxCorners, qualityLevel, minDistance, blockSize, useHarrisDetector, k);
			Ok(new cv::Ptr<cv::GFTTDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GFTTDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:687
	// ("cv::GFTTDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_GFTTDetector_create(Result<cv::Ptr<cv::GFTTDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create();
			Ok(new cv::Ptr<cv::GFTTDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, double, double, int, int, bool, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:689
	// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "gradiantSize", "useHarrisDetector", "k"], ["int", "double", "double", "int", "int", "bool", "double"]), _)]),
	void cv_GFTTDetector_create_int_double_double_int_int_bool_double(int maxCorners, double qualityLevel, double minDistance, int blockSize, int gradiantSize, bool useHarrisDetector, double k, Result<cv::Ptr<cv::GFTTDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create(maxCorners, qualityLevel, minDistance, blockSize, gradiantSize, useHarrisDetector, k);
			Ok(new cv::Ptr<cv::GFTTDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GFTTDetector::create(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:689
	// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "gradiantSize"], ["int", "double", "double", "int", "int"]), _)]),
	void cv_GFTTDetector_create_int_double_double_int_int(int maxCorners, double qualityLevel, double minDistance, int blockSize, int gradiantSize, Result<cv::Ptr<cv::GFTTDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create(maxCorners, qualityLevel, minDistance, blockSize, gradiantSize);
			Ok(new cv::Ptr<cv::GFTTDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:691
	// ("cv::GFTTDetector::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	void cv_GFTTDetector_setMaxFeatures_int(cv::GFTTDetector* instance, int maxFeatures, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxFeatures(maxFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:692
	// ("cv::GFTTDetector::getMaxFeatures", vec![(pred!(const, [], []), _)]),
	void cv_GFTTDetector_getMaxFeatures_const(const cv::GFTTDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setQualityLevel(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:694
	// ("cv::GFTTDetector::setQualityLevel", vec![(pred!(mut, ["qlevel"], ["double"]), _)]),
	void cv_GFTTDetector_setQualityLevel_double(cv::GFTTDetector* instance, double qlevel, ResultVoid* ocvrs_return) {
		try {
			instance->setQualityLevel(qlevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getQualityLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:695
	// ("cv::GFTTDetector::getQualityLevel", vec![(pred!(const, [], []), _)]),
	void cv_GFTTDetector_getQualityLevel_const(const cv::GFTTDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getQualityLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDistance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:697
	// ("cv::GFTTDetector::setMinDistance", vec![(pred!(mut, ["minDistance"], ["double"]), _)]),
	void cv_GFTTDetector_setMinDistance_double(cv::GFTTDetector* instance, double minDistance, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDistance(minDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDistance()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:698
	// ("cv::GFTTDetector::getMinDistance", vec![(pred!(const, [], []), _)]),
	void cv_GFTTDetector_getMinDistance_const(const cv::GFTTDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDistance();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:700
	// ("cv::GFTTDetector::setBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
	void cv_GFTTDetector_setBlockSize_int(cv::GFTTDetector* instance, int blockSize, ResultVoid* ocvrs_return) {
		try {
			instance->setBlockSize(blockSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:701
	// ("cv::GFTTDetector::getBlockSize", vec![(pred!(const, [], []), _)]),
	void cv_GFTTDetector_getBlockSize_const(const cv::GFTTDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGradientSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:703
	// ("cv::GFTTDetector::setGradientSize", vec![(pred!(mut, ["gradientSize_"], ["int"]), _)]),
	void cv_GFTTDetector_setGradientSize_int(cv::GFTTDetector* instance, int gradientSize_, ResultVoid* ocvrs_return) {
		try {
			instance->setGradientSize(gradientSize_);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGradientSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:704
	// ("cv::GFTTDetector::getGradientSize", vec![(pred!(mut, [], []), _)]),
	void cv_GFTTDetector_getGradientSize(cv::GFTTDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getGradientSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHarrisDetector(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:706
	// ("cv::GFTTDetector::setHarrisDetector", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_GFTTDetector_setHarrisDetector_bool(cv::GFTTDetector* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setHarrisDetector(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHarrisDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:707
	// ("cv::GFTTDetector::getHarrisDetector", vec![(pred!(const, [], []), _)]),
	void cv_GFTTDetector_getHarrisDetector_const(const cv::GFTTDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getHarrisDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setK(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:709
	// ("cv::GFTTDetector::setK", vec![(pred!(mut, ["k"], ["double"]), _)]),
	void cv_GFTTDetector_setK_double(cv::GFTTDetector* instance, double k, ResultVoid* ocvrs_return) {
		try {
			instance->setK(k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:710
	// ("cv::GFTTDetector::getK", vec![(pred!(const, [], []), _)]),
	void cv_GFTTDetector_getK_const(const cv::GFTTDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:711
	// ("cv::GFTTDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_GFTTDetector_getDefaultName_const(const cv::GFTTDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GFTTDetector::to_Algorithm() generated
	// ("cv::GFTTDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_GFTTDetector_to_Algorithm(cv::GFTTDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::GFTTDetector::to_Feature2D() generated
	// ("cv::GFTTDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_GFTTDetector_to_Feature2D(cv::GFTTDetector* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::GFTTDetector::delete() generated
	// ("cv::GFTTDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GFTTDetector_delete(cv::GFTTDetector* instance) {
			delete instance;
	}

	// create(bool, bool, float, int, int, KAZE::DiffusivityType)(Primitive, Primitive, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:816
	// ("cv::KAZE::create", vec![(pred!(mut, ["extended", "upright", "threshold", "nOctaves", "nOctaveLayers", "diffusivity"], ["bool", "bool", "float", "int", "int", "cv::KAZE::DiffusivityType"]), _)]),
	void cv_KAZE_create_bool_bool_float_int_int_DiffusivityType(bool extended, bool upright, float threshold, int nOctaves, int nOctaveLayers, cv::KAZE::DiffusivityType diffusivity, Result<cv::Ptr<cv::KAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::KAZE> ret = cv::KAZE::create(extended, upright, threshold, nOctaves, nOctaveLayers, diffusivity);
			Ok(new cv::Ptr<cv::KAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KAZE::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:816
	// ("cv::KAZE::create", vec![(pred!(mut, [], []), _)]),
	void cv_KAZE_create(Result<cv::Ptr<cv::KAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::KAZE> ret = cv::KAZE::create();
			Ok(new cv::Ptr<cv::KAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setExtended(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:821
	// ("cv::KAZE::setExtended", vec![(pred!(mut, ["extended"], ["bool"]), _)]),
	void cv_KAZE_setExtended_bool(cv::KAZE* instance, bool extended, ResultVoid* ocvrs_return) {
		try {
			instance->setExtended(extended);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getExtended()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:822
	// ("cv::KAZE::getExtended", vec![(pred!(const, [], []), _)]),
	void cv_KAZE_getExtended_const(const cv::KAZE* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getExtended();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUpright(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:824
	// ("cv::KAZE::setUpright", vec![(pred!(mut, ["upright"], ["bool"]), _)]),
	void cv_KAZE_setUpright_bool(cv::KAZE* instance, bool upright, ResultVoid* ocvrs_return) {
		try {
			instance->setUpright(upright);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUpright()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:825
	// ("cv::KAZE::getUpright", vec![(pred!(const, [], []), _)]),
	void cv_KAZE_getUpright_const(const cv::KAZE* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUpright();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:827
	// ("cv::KAZE::setThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	void cv_KAZE_setThreshold_double(cv::KAZE* instance, double threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:828
	// ("cv::KAZE::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_KAZE_getThreshold_const(const cv::KAZE* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:830
	// ("cv::KAZE::setNOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	void cv_KAZE_setNOctaves_int(cv::KAZE* instance, int octaves, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:831
	// ("cv::KAZE::getNOctaves", vec![(pred!(const, [], []), _)]),
	void cv_KAZE_getNOctaves_const(const cv::KAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:833
	// ("cv::KAZE::setNOctaveLayers", vec![(pred!(mut, ["octaveLayers"], ["int"]), _)]),
	void cv_KAZE_setNOctaveLayers_int(cv::KAZE* instance, int octaveLayers, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaveLayers(octaveLayers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:834
	// ("cv::KAZE::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	void cv_KAZE_getNOctaveLayers_const(const cv::KAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaveLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDiffusivity(KAZE::DiffusivityType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:836
	// ("cv::KAZE::setDiffusivity", vec![(pred!(mut, ["diff"], ["cv::KAZE::DiffusivityType"]), _)]),
	void cv_KAZE_setDiffusivity_DiffusivityType(cv::KAZE* instance, cv::KAZE::DiffusivityType diff, ResultVoid* ocvrs_return) {
		try {
			instance->setDiffusivity(diff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDiffusivity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:837
	// ("cv::KAZE::getDiffusivity", vec![(pred!(const, [], []), _)]),
	void cv_KAZE_getDiffusivity_const(const cv::KAZE* instance, Result<cv::KAZE::DiffusivityType>* ocvrs_return) {
		try {
			cv::KAZE::DiffusivityType ret = instance->getDiffusivity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:838
	// ("cv::KAZE::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_KAZE_getDefaultName_const(const cv::KAZE* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KAZE::to_Algorithm() generated
	// ("cv::KAZE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_KAZE_to_Algorithm(cv::KAZE* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::KAZE::to_Feature2D() generated
	// ("cv::KAZE::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_KAZE_to_Feature2D(cv::KAZE* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::KAZE::delete() generated
	// ("cv::KAZE::delete", vec![(pred!(mut, [], []), _)]),
	void cv_KAZE_delete(cv::KAZE* instance) {
			delete instance;
	}

	// KeyPointsFilter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:95
	// ("cv::KeyPointsFilter::KeyPointsFilter", vec![(pred!(mut, [], []), _)]),
	void cv_KeyPointsFilter_KeyPointsFilter(Result<cv::KeyPointsFilter*>* ocvrs_return) {
		try {
			cv::KeyPointsFilter* ret = new cv::KeyPointsFilter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// runByImageBorder(std::vector<KeyPoint> &, Size, int)(CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:100
	// ("cv::KeyPointsFilter::runByImageBorder", vec![(pred!(mut, ["keypoints", "imageSize", "borderSize"], ["std::vector<cv::KeyPoint>*", "cv::Size", "int"]), _)]),
	void cv_KeyPointsFilter_runByImageBorder_vectorLKeyPointGR_Size_int(std::vector<cv::KeyPoint>* keypoints, cv::Size* imageSize, int borderSize, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPointsFilter::runByImageBorder(*keypoints, *imageSize, borderSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// runByKeypointSize(std::vector<KeyPoint> &, float, float)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:104
	// ("cv::KeyPointsFilter::runByKeypointSize", vec![(pred!(mut, ["keypoints", "minSize", "maxSize"], ["std::vector<cv::KeyPoint>*", "float", "float"]), _)]),
	void cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float_float(std::vector<cv::KeyPoint>* keypoints, float minSize, float maxSize, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPointsFilter::runByKeypointSize(*keypoints, minSize, maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KeyPointsFilter::runByKeypointSize(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:104
	// ("cv::KeyPointsFilter::runByKeypointSize", vec![(pred!(mut, ["keypoints", "minSize"], ["std::vector<cv::KeyPoint>*", "float"]), _)]),
	void cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float(std::vector<cv::KeyPoint>* keypoints, float minSize, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPointsFilter::runByKeypointSize(*keypoints, minSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// runByPixelsMask(std::vector<KeyPoint> &, const Mat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:109
	// ("cv::KeyPointsFilter::runByPixelsMask", vec![(pred!(mut, ["keypoints", "mask"], ["std::vector<cv::KeyPoint>*", "const cv::Mat*"]), _)]),
	void cv_KeyPointsFilter_runByPixelsMask_vectorLKeyPointGR_const_MatR(std::vector<cv::KeyPoint>* keypoints, const cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPointsFilter::runByPixelsMask(*keypoints, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// runByPixelsMask2VectorPoint(std::vector<KeyPoint> &, std::vector<std::vector<Point>> &, const Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:113
	// ("cv::KeyPointsFilter::runByPixelsMask2VectorPoint", vec![(pred!(mut, ["keypoints", "removeFrom", "mask"], ["std::vector<cv::KeyPoint>*", "std::vector<std::vector<cv::Point>>*", "const cv::Mat*"]), _)]),
	void cv_KeyPointsFilter_runByPixelsMask2VectorPoint_vectorLKeyPointGR_vectorLvectorLPointGGR_const_MatR(std::vector<cv::KeyPoint>* keypoints, std::vector<std::vector<cv::Point>>* removeFrom, const cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPointsFilter::runByPixelsMask2VectorPoint(*keypoints, *removeFrom, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// removeDuplicated(std::vector<KeyPoint> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:117
	// ("cv::KeyPointsFilter::removeDuplicated", vec![(pred!(mut, ["keypoints"], ["std::vector<cv::KeyPoint>*"]), _)]),
	void cv_KeyPointsFilter_removeDuplicated_vectorLKeyPointGR(std::vector<cv::KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPointsFilter::removeDuplicated(*keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// removeDuplicatedSorted(std::vector<KeyPoint> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:121
	// ("cv::KeyPointsFilter::removeDuplicatedSorted", vec![(pred!(mut, ["keypoints"], ["std::vector<cv::KeyPoint>*"]), _)]),
	void cv_KeyPointsFilter_removeDuplicatedSorted_vectorLKeyPointGR(std::vector<cv::KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPointsFilter::removeDuplicatedSorted(*keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// retainBest(std::vector<KeyPoint> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:126
	// ("cv::KeyPointsFilter::retainBest", vec![(pred!(mut, ["keypoints", "npoints"], ["std::vector<cv::KeyPoint>*", "int"]), _)]),
	void cv_KeyPointsFilter_retainBest_vectorLKeyPointGR_int(std::vector<cv::KeyPoint>* keypoints, int npoints, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPointsFilter::retainBest(*keypoints, npoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KeyPointsFilter::delete() generated
	// ("cv::KeyPointsFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_KeyPointsFilter_delete(cv::KeyPointsFilter* instance) {
			delete instance;
	}

	// create(int, int, int, double, double, int, double, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:522
	// ("cv::MSER::create", vec![(pred!(mut, ["delta", "min_area", "max_area", "max_variation", "min_diversity", "max_evolution", "area_threshold", "min_margin", "edge_blur_size"], ["int", "int", "int", "double", "double", "int", "double", "double", "int"]), _)]),
	void cv_MSER_create_int_int_int_double_double_int_double_double_int(int delta, int min_area, int max_area, double max_variation, double min_diversity, int max_evolution, double area_threshold, double min_margin, int edge_blur_size, Result<cv::Ptr<cv::MSER>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::MSER> ret = cv::MSER::create(delta, min_area, max_area, max_variation, min_diversity, max_evolution, area_threshold, min_margin, edge_blur_size);
			Ok(new cv::Ptr<cv::MSER>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MSER::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:522
	// ("cv::MSER::create", vec![(pred!(mut, [], []), _)]),
	void cv_MSER_create(Result<cv::Ptr<cv::MSER>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::MSER> ret = cv::MSER::create();
			Ok(new cv::Ptr<cv::MSER>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectRegions(InputArray, std::vector<std::vector<Point>> &, std::vector<Rect> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:533
	// ("cv::MSER::detectRegions", vec![(pred!(mut, ["image", "msers", "bboxes"], ["const cv::_InputArray*", "std::vector<std::vector<cv::Point>>*", "std::vector<cv::Rect>*"]), _)]),
	void cv_MSER_detectRegions_const__InputArrayR_vectorLvectorLPointGGR_vectorLRectGR(cv::MSER* instance, const cv::_InputArray* image, std::vector<std::vector<cv::Point>>* msers, std::vector<cv::Rect>* bboxes, ResultVoid* ocvrs_return) {
		try {
			instance->detectRegions(*image, *msers, *bboxes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDelta(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:537
	// ("cv::MSER::setDelta", vec![(pred!(mut, ["delta"], ["int"]), _)]),
	void cv_MSER_setDelta_int(cv::MSER* instance, int delta, ResultVoid* ocvrs_return) {
		try {
			instance->setDelta(delta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDelta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:538
	// ("cv::MSER::getDelta", vec![(pred!(const, [], []), _)]),
	void cv_MSER_getDelta_const(const cv::MSER* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDelta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinArea(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:540
	// ("cv::MSER::setMinArea", vec![(pred!(mut, ["minArea"], ["int"]), _)]),
	void cv_MSER_setMinArea_int(cv::MSER* instance, int minArea, ResultVoid* ocvrs_return) {
		try {
			instance->setMinArea(minArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinArea()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:541
	// ("cv::MSER::getMinArea", vec![(pred!(const, [], []), _)]),
	void cv_MSER_getMinArea_const(const cv::MSER* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinArea();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxArea(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:543
	// ("cv::MSER::setMaxArea", vec![(pred!(mut, ["maxArea"], ["int"]), _)]),
	void cv_MSER_setMaxArea_int(cv::MSER* instance, int maxArea, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxArea(maxArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxArea()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:544
	// ("cv::MSER::getMaxArea", vec![(pred!(const, [], []), _)]),
	void cv_MSER_getMaxArea_const(const cv::MSER* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxArea();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxVariation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:546
	// ("cv::MSER::setMaxVariation", vec![(pred!(mut, ["maxVariation"], ["double"]), _)]),
	void cv_MSER_setMaxVariation_double(cv::MSER* instance, double maxVariation, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxVariation(maxVariation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxVariation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:547
	// ("cv::MSER::getMaxVariation", vec![(pred!(const, [], []), _)]),
	void cv_MSER_getMaxVariation_const(const cv::MSER* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxVariation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDiversity(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:549
	// ("cv::MSER::setMinDiversity", vec![(pred!(mut, ["minDiversity"], ["double"]), _)]),
	void cv_MSER_setMinDiversity_double(cv::MSER* instance, double minDiversity, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDiversity(minDiversity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDiversity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:550
	// ("cv::MSER::getMinDiversity", vec![(pred!(const, [], []), _)]),
	void cv_MSER_getMinDiversity_const(const cv::MSER* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDiversity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxEvolution(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:552
	// ("cv::MSER::setMaxEvolution", vec![(pred!(mut, ["maxEvolution"], ["int"]), _)]),
	void cv_MSER_setMaxEvolution_int(cv::MSER* instance, int maxEvolution, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxEvolution(maxEvolution);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxEvolution()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:553
	// ("cv::MSER::getMaxEvolution", vec![(pred!(const, [], []), _)]),
	void cv_MSER_getMaxEvolution_const(const cv::MSER* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxEvolution();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAreaThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:555
	// ("cv::MSER::setAreaThreshold", vec![(pred!(mut, ["areaThreshold"], ["double"]), _)]),
	void cv_MSER_setAreaThreshold_double(cv::MSER* instance, double areaThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setAreaThreshold(areaThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAreaThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:556
	// ("cv::MSER::getAreaThreshold", vec![(pred!(const, [], []), _)]),
	void cv_MSER_getAreaThreshold_const(const cv::MSER* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAreaThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinMargin(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:558
	// ("cv::MSER::setMinMargin", vec![(pred!(mut, ["min_margin"], ["double"]), _)]),
	void cv_MSER_setMinMargin_double(cv::MSER* instance, double min_margin, ResultVoid* ocvrs_return) {
		try {
			instance->setMinMargin(min_margin);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinMargin()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:559
	// ("cv::MSER::getMinMargin", vec![(pred!(const, [], []), _)]),
	void cv_MSER_getMinMargin_const(const cv::MSER* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinMargin();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEdgeBlurSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:561
	// ("cv::MSER::setEdgeBlurSize", vec![(pred!(mut, ["edge_blur_size"], ["int"]), _)]),
	void cv_MSER_setEdgeBlurSize_int(cv::MSER* instance, int edge_blur_size, ResultVoid* ocvrs_return) {
		try {
			instance->setEdgeBlurSize(edge_blur_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdgeBlurSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:562
	// ("cv::MSER::getEdgeBlurSize", vec![(pred!(const, [], []), _)]),
	void cv_MSER_getEdgeBlurSize_const(const cv::MSER* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEdgeBlurSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPass2Only(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:564
	// ("cv::MSER::setPass2Only", vec![(pred!(mut, ["f"], ["bool"]), _)]),
	void cv_MSER_setPass2Only_bool(cv::MSER* instance, bool f, ResultVoid* ocvrs_return) {
		try {
			instance->setPass2Only(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPass2Only()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:565
	// ("cv::MSER::getPass2Only", vec![(pred!(const, [], []), _)]),
	void cv_MSER_getPass2Only_const(const cv::MSER* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getPass2Only();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:567
	// ("cv::MSER::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_MSER_getDefaultName_const(const cv::MSER* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MSER::to_Algorithm() generated
	// ("cv::MSER::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_MSER_to_Algorithm(cv::MSER* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::MSER::to_Feature2D() generated
	// ("cv::MSER::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_MSER_to_Feature2D(cv::MSER* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::MSER::delete() generated
	// ("cv::MSER::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MSER_delete(cv::MSER* instance) {
			delete instance;
	}

	// create(int, float, int, int, int, int, ORB::ScoreType, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:460
	// ("cv::ORB::create", vec![(pred!(mut, ["nfeatures", "scaleFactor", "nlevels", "edgeThreshold", "firstLevel", "WTA_K", "scoreType", "patchSize", "fastThreshold"], ["int", "float", "int", "int", "int", "int", "cv::ORB::ScoreType", "int", "int"]), _)]),
	void cv_ORB_create_int_float_int_int_int_int_ScoreType_int_int(int nfeatures, float scaleFactor, int nlevels, int edgeThreshold, int firstLevel, int WTA_K, cv::ORB::ScoreType scoreType, int patchSize, int fastThreshold, Result<cv::Ptr<cv::ORB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ORB> ret = cv::ORB::create(nfeatures, scaleFactor, nlevels, edgeThreshold, firstLevel, WTA_K, scoreType, patchSize, fastThreshold);
			Ok(new cv::Ptr<cv::ORB>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ORB::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:460
	// ("cv::ORB::create", vec![(pred!(mut, [], []), _)]),
	void cv_ORB_create(Result<cv::Ptr<cv::ORB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ORB> ret = cv::ORB::create();
			Ok(new cv::Ptr<cv::ORB>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:463
	// ("cv::ORB::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	void cv_ORB_setMaxFeatures_int(cv::ORB* instance, int maxFeatures, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxFeatures(maxFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:464
	// ("cv::ORB::getMaxFeatures", vec![(pred!(const, [], []), _)]),
	void cv_ORB_getMaxFeatures_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:466
	// ("cv::ORB::setScaleFactor", vec![(pred!(mut, ["scaleFactor"], ["double"]), _)]),
	void cv_ORB_setScaleFactor_double(cv::ORB* instance, double scaleFactor, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(scaleFactor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:467
	// ("cv::ORB::getScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_ORB_getScaleFactor_const(const cv::ORB* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:469
	// ("cv::ORB::setNLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
	void cv_ORB_setNLevels_int(cv::ORB* instance, int nlevels, ResultVoid* ocvrs_return) {
		try {
			instance->setNLevels(nlevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:470
	// ("cv::ORB::getNLevels", vec![(pred!(const, [], []), _)]),
	void cv_ORB_getNLevels_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEdgeThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:472
	// ("cv::ORB::setEdgeThreshold", vec![(pred!(mut, ["edgeThreshold"], ["int"]), _)]),
	void cv_ORB_setEdgeThreshold_int(cv::ORB* instance, int edgeThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setEdgeThreshold(edgeThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:473
	// ("cv::ORB::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
	void cv_ORB_getEdgeThreshold_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEdgeThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFirstLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:475
	// ("cv::ORB::setFirstLevel", vec![(pred!(mut, ["firstLevel"], ["int"]), _)]),
	void cv_ORB_setFirstLevel_int(cv::ORB* instance, int firstLevel, ResultVoid* ocvrs_return) {
		try {
			instance->setFirstLevel(firstLevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFirstLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:476
	// ("cv::ORB::getFirstLevel", vec![(pred!(const, [], []), _)]),
	void cv_ORB_getFirstLevel_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFirstLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWTA_K(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:478
	// ("cv::ORB::setWTA_K", vec![(pred!(mut, ["wta_k"], ["int"]), _)]),
	void cv_ORB_setWTA_K_int(cv::ORB* instance, int wta_k, ResultVoid* ocvrs_return) {
		try {
			instance->setWTA_K(wta_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWTA_K()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:479
	// ("cv::ORB::getWTA_K", vec![(pred!(const, [], []), _)]),
	void cv_ORB_getWTA_K_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWTA_K();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScoreType(ORB::ScoreType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:481
	// ("cv::ORB::setScoreType", vec![(pred!(mut, ["scoreType"], ["cv::ORB::ScoreType"]), _)]),
	void cv_ORB_setScoreType_ScoreType(cv::ORB* instance, cv::ORB::ScoreType scoreType, ResultVoid* ocvrs_return) {
		try {
			instance->setScoreType(scoreType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScoreType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:482
	// ("cv::ORB::getScoreType", vec![(pred!(const, [], []), _)]),
	void cv_ORB_getScoreType_const(const cv::ORB* instance, Result<cv::ORB::ScoreType>* ocvrs_return) {
		try {
			cv::ORB::ScoreType ret = instance->getScoreType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:484
	// ("cv::ORB::setPatchSize", vec![(pred!(mut, ["patchSize"], ["int"]), _)]),
	void cv_ORB_setPatchSize_int(cv::ORB* instance, int patchSize, ResultVoid* ocvrs_return) {
		try {
			instance->setPatchSize(patchSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPatchSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:485
	// ("cv::ORB::getPatchSize", vec![(pred!(const, [], []), _)]),
	void cv_ORB_getPatchSize_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPatchSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFastThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:487
	// ("cv::ORB::setFastThreshold", vec![(pred!(mut, ["fastThreshold"], ["int"]), _)]),
	void cv_ORB_setFastThreshold_int(cv::ORB* instance, int fastThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setFastThreshold(fastThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFastThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:488
	// ("cv::ORB::getFastThreshold", vec![(pred!(const, [], []), _)]),
	void cv_ORB_getFastThreshold_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFastThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:489
	// ("cv::ORB::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_ORB_getDefaultName_const(const cv::ORB* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ORB::to_Algorithm() generated
	// ("cv::ORB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ORB_to_Algorithm(cv::ORB* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ORB::to_Feature2D() generated
	// ("cv::ORB::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_ORB_to_Feature2D(cv::ORB* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::ORB::delete() generated
	// ("cv::ORB::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ORB_delete(cv::ORB* instance) {
			delete instance;
	}

	// create(int, int, double, double, double, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:294
	// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "enable_precise_upscale"], ["int", "int", "double", "double", "double", "bool"]), _)]),
	void cv_SIFT_create_int_int_double_double_double_bool(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma, bool enable_precise_upscale, Result<cv::Ptr<cv::SIFT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SIFT> ret = cv::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma, enable_precise_upscale);
			Ok(new cv::Ptr<cv::SIFT>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SIFT::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:294
	// ("cv::SIFT::create", vec![(pred!(mut, [], []), _)]),
	void cv_SIFT_create(Result<cv::Ptr<cv::SIFT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SIFT> ret = cv::SIFT::create();
			Ok(new cv::Ptr<cv::SIFT>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, double, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:325
	// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "descriptorType", "enable_precise_upscale"], ["int", "int", "double", "double", "double", "int", "bool"]), _)]),
	void cv_SIFT_create_int_int_double_double_double_int_bool(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma, int descriptorType, bool enable_precise_upscale, Result<cv::Ptr<cv::SIFT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SIFT> ret = cv::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma, descriptorType, enable_precise_upscale);
			Ok(new cv::Ptr<cv::SIFT>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SIFT::create(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:325
	// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "descriptorType"], ["int", "int", "double", "double", "double", "int"]), _)]),
	void cv_SIFT_create_int_int_double_double_double_int(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma, int descriptorType, Result<cv::Ptr<cv::SIFT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SIFT> ret = cv::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma, descriptorType);
			Ok(new cv::Ptr<cv::SIFT>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:329
	// ("cv::SIFT::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_SIFT_getDefaultName_const(const cv::SIFT* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:331
	// ("cv::SIFT::setNFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	void cv_SIFT_setNFeatures_int(cv::SIFT* instance, int maxFeatures, ResultVoid* ocvrs_return) {
		try {
			instance->setNFeatures(maxFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNFeatures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:332
	// ("cv::SIFT::getNFeatures", vec![(pred!(const, [], []), _)]),
	void cv_SIFT_getNFeatures_const(const cv::SIFT* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:334
	// ("cv::SIFT::setNOctaveLayers", vec![(pred!(mut, ["nOctaveLayers"], ["int"]), _)]),
	void cv_SIFT_setNOctaveLayers_int(cv::SIFT* instance, int nOctaveLayers, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaveLayers(nOctaveLayers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:335
	// ("cv::SIFT::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	void cv_SIFT_getNOctaveLayers_const(const cv::SIFT* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaveLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setContrastThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:337
	// ("cv::SIFT::setContrastThreshold", vec![(pred!(mut, ["contrastThreshold"], ["double"]), _)]),
	void cv_SIFT_setContrastThreshold_double(cv::SIFT* instance, double contrastThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setContrastThreshold(contrastThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getContrastThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:338
	// ("cv::SIFT::getContrastThreshold", vec![(pred!(const, [], []), _)]),
	void cv_SIFT_getContrastThreshold_const(const cv::SIFT* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getContrastThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEdgeThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:340
	// ("cv::SIFT::setEdgeThreshold", vec![(pred!(mut, ["edgeThreshold"], ["double"]), _)]),
	void cv_SIFT_setEdgeThreshold_double(cv::SIFT* instance, double edgeThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setEdgeThreshold(edgeThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:341
	// ("cv::SIFT::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
	void cv_SIFT_getEdgeThreshold_const(const cv::SIFT* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getEdgeThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:343
	// ("cv::SIFT::setSigma", vec![(pred!(mut, ["sigma"], ["double"]), _)]),
	void cv_SIFT_setSigma_double(cv::SIFT* instance, double sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setSigma(sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:344
	// ("cv::SIFT::getSigma", vec![(pred!(const, [], []), _)]),
	void cv_SIFT_getSigma_const(const cv::SIFT* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SIFT::to_Algorithm() generated
	// ("cv::SIFT::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_SIFT_to_Algorithm(cv::SIFT* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::SIFT::to_Feature2D() generated
	// ("cv::SIFT::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_SIFT_to_Feature2D(cv::SIFT* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::SIFT::delete() generated
	// ("cv::SIFT::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SIFT_delete(cv::SIFT* instance) {
			delete instance;
	}

	// create(const SimpleBlobDetector::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:779
	// ("cv::SimpleBlobDetector::create", vec![(pred!(mut, ["parameters"], ["const cv::SimpleBlobDetector::Params*"]), _)]),
	void cv_SimpleBlobDetector_create_const_ParamsR(const cv::SimpleBlobDetector::Params* parameters, Result<cv::Ptr<cv::SimpleBlobDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SimpleBlobDetector> ret = cv::SimpleBlobDetector::create(*parameters);
			Ok(new cv::Ptr<cv::SimpleBlobDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SimpleBlobDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:779
	// ("cv::SimpleBlobDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_SimpleBlobDetector_create(Result<cv::Ptr<cv::SimpleBlobDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SimpleBlobDetector> ret = cv::SimpleBlobDetector::create();
			Ok(new cv::Ptr<cv::SimpleBlobDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParams(const SimpleBlobDetector::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:781
	// ("cv::SimpleBlobDetector::setParams", vec![(pred!(mut, ["params"], ["const cv::SimpleBlobDetector::Params*"]), _)]),
	void cv_SimpleBlobDetector_setParams_const_ParamsR(cv::SimpleBlobDetector* instance, const cv::SimpleBlobDetector::Params* params, ResultVoid* ocvrs_return) {
		try {
			instance->setParams(*params);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:782
	// ("cv::SimpleBlobDetector::getParams", vec![(pred!(const, [], []), _)]),
	void cv_SimpleBlobDetector_getParams_const(const cv::SimpleBlobDetector* instance, Result<cv::SimpleBlobDetector::Params>* ocvrs_return) {
		try {
			cv::SimpleBlobDetector::Params ret = instance->getParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:784
	// ("cv::SimpleBlobDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_SimpleBlobDetector_getDefaultName_const(const cv::SimpleBlobDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlobContours()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:785
	// ("cv::SimpleBlobDetector::getBlobContours", vec![(pred!(const, [], []), _)]),
	void cv_SimpleBlobDetector_getBlobContours_const(const cv::SimpleBlobDetector* instance, Result<std::vector<std::vector<cv::Point>>*>* ocvrs_return) {
		try {
			const std::vector<std::vector<cv::Point>> ret = instance->getBlobContours();
			Ok(new const std::vector<std::vector<cv::Point>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SimpleBlobDetector::to_Algorithm() generated
	// ("cv::SimpleBlobDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_SimpleBlobDetector_to_Algorithm(cv::SimpleBlobDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::SimpleBlobDetector::to_Feature2D() generated
	// ("cv::SimpleBlobDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_SimpleBlobDetector_to_Feature2D(cv::SimpleBlobDetector* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::SimpleBlobDetector::delete() generated
	// ("cv::SimpleBlobDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SimpleBlobDetector_delete(cv::SimpleBlobDetector* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:750
	// ("cv::SimpleBlobDetector::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_SimpleBlobDetector_Params_Params(Result<cv::SimpleBlobDetector::Params>* ocvrs_return) {
		try {
			cv::SimpleBlobDetector::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:774
	// ("cv::SimpleBlobDetector::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_SimpleBlobDetector_Params_read_const_FileNodeR(cv::SimpleBlobDetector::Params* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:775
	// ("cv::SimpleBlobDetector::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_SimpleBlobDetector_Params_write_const_FileStorageR(const cv::SimpleBlobDetector::Params* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}

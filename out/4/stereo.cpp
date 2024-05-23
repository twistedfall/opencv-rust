#include "ocvrs_common.hpp"
#include <opencv2/stereo.hpp>
#include "stereo_types.hpp"

extern "C" {
	// censusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:22
	// ("cv::stereo::censusTransform", vec![(pred!(mut, ["image1", "image2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_censusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(const cv::Mat* image1, const cv::Mat* image2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::censusTransform(*image1, *image2, kernelSize, *dist1, *dist2, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// censusTransform(const Mat &, int, Mat &, const int)(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:24
	// ("cv::stereo::censusTransform", vec![(pred!(mut, ["image1", "kernelSize", "dist1", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_censusTransform_const_MatR_int_MatR_const_int(const cv::Mat* image1, int kernelSize, cv::Mat* dist1, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::censusTransform(*image1, kernelSize, *dist1, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereo::modifiedCensusTransform(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:29
	// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// modifiedCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int, int, const Mat &, const Mat &)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:29
	// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type", "t", "integralImage1", "integralImage2"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int", "int", "const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int_int_const_MatR_const_MatR(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, int t, const cv::Mat* integralImage1, const cv::Mat* integralImage2, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type, t, *integralImage1, *integralImage2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereo::modifiedCensusTransform(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:31
	// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int(const cv::Mat* img1, int kernelSize, cv::Mat* dist, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, kernelSize, *dist, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// modifiedCensusTransform(const Mat &, int, Mat &, const int, int, const Mat &)(TraitClass, Primitive, TraitClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:31
	// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist", "type", "t", "integralImage"], ["const cv::Mat*", "int", "cv::Mat*", "const int", "int", "const cv::Mat*"]), _)]),
	void cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int_int_const_MatR(const cv::Mat* img1, int kernelSize, cv::Mat* dist, const int type, int t, const cv::Mat* integralImage, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, kernelSize, *dist, type, t, *integralImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// starCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:39
	// ("cv::stereo::starCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_stereo_starCensusTransform_const_MatR_const_MatR_int_MatR_MatR(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::starCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// starCensusTransform(const Mat &, int, Mat &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:41
	// ("cv::stereo::starCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist"], ["const cv::Mat*", "int", "cv::Mat*"]), _)]),
	void cv_stereo_starCensusTransform_const_MatR_int_MatR(const cv::Mat* img1, int kernelSize, cv::Mat* dist, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::starCensusTransform(*img1, kernelSize, *dist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// symetricCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:35
	// ("cv::stereo::symetricCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_symetricCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::symetricCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// symetricCensusTransform(const Mat &, int, Mat &, const int)(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:37
	// ("cv::stereo::symetricCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist1", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_symetricCensusTransform_const_MatR_int_MatR_const_int(const cv::Mat* img1, int kernelSize, cv::Mat* dist1, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::symetricCensusTransform(*img1, kernelSize, *dist1, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// MatchQuasiDense()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:34
	// ("cv::stereo::MatchQuasiDense::MatchQuasiDense", vec![(pred!(mut, [], []), _)]),
	void cv_stereo_MatchQuasiDense_MatchQuasiDense(Result<cv::stereo::MatchQuasiDense>* ocvrs_return) {
		try {
			cv::stereo::MatchQuasiDense ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(const MatchQuasiDense &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:36
	// ("cv::stereo::MatchQuasiDense::operator<", vec![(pred!(const, ["rhs"], ["const cv::stereo::MatchQuasiDense*"]), _)]),
	void cv_stereo_MatchQuasiDense_operatorL_const_const_MatchQuasiDenseR(const cv::stereo::MatchQuasiDense* instance, const cv::stereo::MatchQuasiDense* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator<(*rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadParameters(cv::String)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:119
	// ("cv::stereo::QuasiDenseStereo::loadParameters", vec![(pred!(mut, ["filepath"], ["cv::String"]), _)]),
	void cv_stereo_QuasiDenseStereo_loadParameters_String(cv::stereo::QuasiDenseStereo* instance, const char* filepath, Result<int>* ocvrs_return) {
		try {
			int ret = instance->loadParameters(std::string(filepath));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// saveParameters(cv::String)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:130
	// ("cv::stereo::QuasiDenseStereo::saveParameters", vec![(pred!(mut, ["filepath"], ["cv::String"]), _)]),
	void cv_stereo_QuasiDenseStereo_saveParameters_String(cv::stereo::QuasiDenseStereo* instance, const char* filepath, Result<int>* ocvrs_return) {
		try {
			int ret = instance->saveParameters(std::string(filepath));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSparseMatches(std::vector<MatchQuasiDense> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:139
	// ("cv::stereo::QuasiDenseStereo::getSparseMatches", vec![(pred!(mut, ["sMatches"], ["std::vector<cv::stereo::MatchQuasiDense>*"]), _)]),
	void cv_stereo_QuasiDenseStereo_getSparseMatches_vectorLMatchQuasiDenseGR(cv::stereo::QuasiDenseStereo* instance, std::vector<cv::stereo::MatchQuasiDense>* sMatches, ResultVoid* ocvrs_return) {
		try {
			instance->getSparseMatches(*sMatches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDenseMatches(std::vector<MatchQuasiDense> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:148
	// ("cv::stereo::QuasiDenseStereo::getDenseMatches", vec![(pred!(mut, ["denseMatches"], ["std::vector<cv::stereo::MatchQuasiDense>*"]), _)]),
	void cv_stereo_QuasiDenseStereo_getDenseMatches_vectorLMatchQuasiDenseGR(cv::stereo::QuasiDenseStereo* instance, std::vector<cv::stereo::MatchQuasiDense>* denseMatches, ResultVoid* ocvrs_return) {
		try {
			instance->getDenseMatches(*denseMatches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(const cv::Mat &, const cv::Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:164
	// ("cv::stereo::QuasiDenseStereo::process", vec![(pred!(mut, ["imgLeft", "imgRight"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_stereo_QuasiDenseStereo_process_const_MatR_const_MatR(cv::stereo::QuasiDenseStereo* instance, const cv::Mat* imgLeft, const cv::Mat* imgRight, ResultVoid* ocvrs_return) {
		try {
			instance->process(*imgLeft, *imgRight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMatch(const int, const int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:175
	// ("cv::stereo::QuasiDenseStereo::getMatch", vec![(pred!(mut, ["x", "y"], ["const int", "const int"]), _)]),
	void cv_stereo_QuasiDenseStereo_getMatch_const_int_const_int(cv::stereo::QuasiDenseStereo* instance, const int x, const int y, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getMatch(x, y);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDisparity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:185
	// ("cv::stereo::QuasiDenseStereo::getDisparity", vec![(pred!(mut, [], []), _)]),
	void cv_stereo_QuasiDenseStereo_getDisparity(cv::stereo::QuasiDenseStereo* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getDisparity();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(cv::Size, cv::String)(SimpleClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:188
	// ("cv::stereo::QuasiDenseStereo::create", vec![(pred!(mut, ["monoImgSize", "paramFilepath"], ["cv::Size", "cv::String"]), _)]),
	void cv_stereo_QuasiDenseStereo_create_Size_String(cv::Size* monoImgSize, const char* paramFilepath, Result<cv::Ptr<cv::stereo::QuasiDenseStereo>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::stereo::QuasiDenseStereo> ret = cv::stereo::QuasiDenseStereo::create(*monoImgSize, std::string(paramFilepath));
			Ok(new cv::Ptr<cv::stereo::QuasiDenseStereo>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereo::QuasiDenseStereo::create(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:188
	// ("cv::stereo::QuasiDenseStereo::create", vec![(pred!(mut, ["monoImgSize"], ["cv::Size"]), _)]),
	void cv_stereo_QuasiDenseStereo_create_Size(cv::Size* monoImgSize, Result<cv::Ptr<cv::stereo::QuasiDenseStereo>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::stereo::QuasiDenseStereo> ret = cv::stereo::QuasiDenseStereo::create(*monoImgSize);
			Ok(new cv::Ptr<cv::stereo::QuasiDenseStereo>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereo::QuasiDenseStereo::Param() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:191
	// ("cv::stereo::QuasiDenseStereo::Param", vec![(pred!(const, [], []), _)]),
	void cv_stereo_QuasiDenseStereo_propParam_const(const cv::stereo::QuasiDenseStereo* instance, cv::stereo::PropagationParameters* ocvrs_return) {
			cv::stereo::PropagationParameters ret = instance->Param;
			*ocvrs_return = ret;
	}

	// cv::stereo::QuasiDenseStereo::setParam(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:191
	// ("cv::stereo::QuasiDenseStereo::setParam", vec![(pred!(mut, ["val"], ["const cv::stereo::PropagationParameters"]), _)]),
	void cv_stereo_QuasiDenseStereo_propParam_const_PropagationParameters(cv::stereo::QuasiDenseStereo* instance, const cv::stereo::PropagationParameters* val) {
			instance->Param = *val;
	}

	// cv::stereo::QuasiDenseStereo::delete() generated
	// ("cv::stereo::QuasiDenseStereo::delete", vec![(pred!(mut, [], []), _)]),
	void cv_stereo_QuasiDenseStereo_delete(cv::stereo::QuasiDenseStereo* instance) {
			delete instance;
	}

}

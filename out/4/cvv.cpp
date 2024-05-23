#include "ocvrs_common.hpp"
#include <opencv2/cvv.hpp>
#include "cvv_types.hpp"

extern "C" {
	// debugDMatch(cv::InputArray, std::vector<cv::KeyPoint>, cv::InputArray, std::vector<cv::KeyPoint>, std::vector<cv::DMatch>, const CallMetaData &, const char *, const char *, bool)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass, InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/dmatch.hpp:24
	// ("cvv::impl::debugDMatch", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches", "data", "description", "view", "useTrainDescriptor"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>", "const cv::_InputArray*", "std::vector<cv::KeyPoint>", "std::vector<cv::DMatch>", "const cvv::impl::CallMetaData*", "const char*", "const char*", "bool"]), _)]),
	void cvv_impl_debugDMatch_const__InputArrayR_vectorLKeyPointG_const__InputArrayR_vectorLKeyPointG_vectorLDMatchG_const_CallMetaDataR_const_charX_const_charX_bool(const cv::_InputArray* img1, std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, std::vector<cv::KeyPoint>* keypoints2, std::vector<cv::DMatch>* matches, const cvv::impl::CallMetaData* data, const char* description, const char* view, bool useTrainDescriptor, ResultVoid* ocvrs_return) {
		try {
			cvv::impl::debugDMatch(*img1, *keypoints1, *img2, *keypoints2, *matches, *data, description, view, useTrainDescriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// debugFilter(cv::InputArray, cv::InputArray, const CallMetaData &, const char *, const char *)(InputArray, InputArray, TraitClass, InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/filter.hpp:24
	// ("cvv::impl::debugFilter", vec![(pred!(mut, ["original", "result", "data", "description", "view"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cvv::impl::CallMetaData*", "const char*", "const char*"]), _)]),
	void cvv_impl_debugFilter_const__InputArrayR_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(const cv::_InputArray* original, const cv::_InputArray* result, const cvv::impl::CallMetaData* data, const char* description, const char* view, ResultVoid* ocvrs_return) {
		try {
			cvv::impl::debugFilter(*original, *result, *data, description, view);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// finalShow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/final_show.hpp:15
	// ("cvv::impl::finalShow", vec![(pred!(mut, [], []), _)]),
	void cvv_impl_finalShow(ResultVoid* ocvrs_return) {
		try {
			cvv::impl::finalShow();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// showImage(cv::InputArray, const CallMetaData &, const char *, const char *)(InputArray, TraitClass, InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/show_image.hpp:24
	// ("cvv::impl::showImage", vec![(pred!(mut, ["img", "data", "description", "view"], ["const cv::_InputArray*", "const cvv::impl::CallMetaData*", "const char*", "const char*"]), _)]),
	void cvv_impl_showImage_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(const cv::_InputArray* img, const cvv::impl::CallMetaData* data, const char* description, const char* view, ResultVoid* ocvrs_return) {
		try {
			cvv::impl::showImage(*img, *data, description, view);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CallMetaData()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:26
	// ("cvv::impl::CallMetaData::CallMetaData", vec![(pred!(mut, [], []), _)]),
	void cvv_impl_CallMetaData_CallMetaData(Result<cvv::impl::CallMetaData*>* ocvrs_return) {
		try {
			cvv::impl::CallMetaData* ret = new cvv::impl::CallMetaData();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CallMetaData(const char *, size_t, const char *)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:36
	// ("cvv::impl::CallMetaData::CallMetaData", vec![(pred!(mut, ["file", "line", "function"], ["const char*", "size_t", "const char*"]), _)]),
	void cvv_impl_CallMetaData_CallMetaData_const_charX_size_t_const_charX(const char* file, size_t line, const char* function, Result<cvv::impl::CallMetaData*>* ocvrs_return) {
		try {
			cvv::impl::CallMetaData* ret = new cvv::impl::CallMetaData(file, line, function);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator bool()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:40
	// ("cvv::impl::CallMetaData::operator bool", vec![(pred!(mut, [], []), _)]),
	void cvv_impl_CallMetaData_operator_bool(cvv::impl::CallMetaData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator bool();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cvv::impl::CallMetaData::implicitClone() generated
	// ("cvv::impl::CallMetaData::implicitClone", vec![(pred!(const, [], []), _)]),
	cvv::impl::CallMetaData* cvv_impl_CallMetaData_implicitClone_const(const cvv::impl::CallMetaData* instance) {
			return new cvv::impl::CallMetaData(*instance);
	}

	// cvv::impl::CallMetaData::file() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:46
	// ("cvv::impl::CallMetaData::file", vec![(pred!(const, [], []), _)]),
	void* cvv_impl_CallMetaData_propFile_const(const cvv::impl::CallMetaData* instance) {
			const char* ret = instance->file;
			return ocvrs_create_string(ret);
	}

	// cvv::impl::CallMetaData::line() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:47
	// ("cvv::impl::CallMetaData::line", vec![(pred!(const, [], []), _)]),
	const size_t cvv_impl_CallMetaData_propLine_const(const cvv::impl::CallMetaData* instance) {
			const size_t ret = instance->line;
			return ret;
	}

	// cvv::impl::CallMetaData::function() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:48
	// ("cvv::impl::CallMetaData::function", vec![(pred!(const, [], []), _)]),
	void* cvv_impl_CallMetaData_propFunction_const(const cvv::impl::CallMetaData* instance) {
			const char* ret = instance->function;
			return ocvrs_create_string(ret);
	}

	// cvv::impl::CallMetaData::isKnown() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:53
	// ("cvv::impl::CallMetaData::isKnown", vec![(pred!(const, [], []), _)]),
	const bool cvv_impl_CallMetaData_propIsKnown_const(const cvv::impl::CallMetaData* instance) {
			const bool ret = instance->isKnown;
			return ret;
	}

	// cvv::impl::CallMetaData::delete() generated
	// ("cvv::impl::CallMetaData::delete", vec![(pred!(mut, [], []), _)]),
	void cvv_impl_CallMetaData_delete(cvv::impl::CallMetaData* instance) {
			delete instance;
	}

}

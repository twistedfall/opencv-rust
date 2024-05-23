#include "ocvrs_common.hpp"
#include <opencv2/hfs.hpp>
#include "hfs_types.hpp"

extern "C" {
	// setSegEgbThresholdI(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:54
	// ("cv::hfs::HfsSegment::setSegEgbThresholdI", vec![(pred!(mut, ["c"], ["float"]), _)]),
	void cv_hfs_HfsSegment_setSegEgbThresholdI_float(cv::hfs::HfsSegment* instance, float c, ResultVoid* ocvrs_return) {
		try {
			instance->setSegEgbThresholdI(c);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSegEgbThresholdI()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:55
	// ("cv::hfs::HfsSegment::getSegEgbThresholdI", vec![(pred!(mut, [], []), _)]),
	void cv_hfs_HfsSegment_getSegEgbThresholdI(cv::hfs::HfsSegment* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSegEgbThresholdI();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinRegionSizeI(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:63
	// ("cv::hfs::HfsSegment::setMinRegionSizeI", vec![(pred!(mut, ["n"], ["int"]), _)]),
	void cv_hfs_HfsSegment_setMinRegionSizeI_int(cv::hfs::HfsSegment* instance, int n, ResultVoid* ocvrs_return) {
		try {
			instance->setMinRegionSizeI(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinRegionSizeI()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:64
	// ("cv::hfs::HfsSegment::getMinRegionSizeI", vec![(pred!(mut, [], []), _)]),
	void cv_hfs_HfsSegment_getMinRegionSizeI(cv::hfs::HfsSegment* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinRegionSizeI();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSegEgbThresholdII(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:73
	// ("cv::hfs::HfsSegment::setSegEgbThresholdII", vec![(pred!(mut, ["c"], ["float"]), _)]),
	void cv_hfs_HfsSegment_setSegEgbThresholdII_float(cv::hfs::HfsSegment* instance, float c, ResultVoid* ocvrs_return) {
		try {
			instance->setSegEgbThresholdII(c);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSegEgbThresholdII()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:74
	// ("cv::hfs::HfsSegment::getSegEgbThresholdII", vec![(pred!(mut, [], []), _)]),
	void cv_hfs_HfsSegment_getSegEgbThresholdII(cv::hfs::HfsSegment* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSegEgbThresholdII();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinRegionSizeII(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:81
	// ("cv::hfs::HfsSegment::setMinRegionSizeII", vec![(pred!(mut, ["n"], ["int"]), _)]),
	void cv_hfs_HfsSegment_setMinRegionSizeII_int(cv::hfs::HfsSegment* instance, int n, ResultVoid* ocvrs_return) {
		try {
			instance->setMinRegionSizeII(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinRegionSizeII()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:82
	// ("cv::hfs::HfsSegment::getMinRegionSizeII", vec![(pred!(mut, [], []), _)]),
	void cv_hfs_HfsSegment_getMinRegionSizeII(cv::hfs::HfsSegment* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinRegionSizeII();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSpatialWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:94
	// ("cv::hfs::HfsSegment::setSpatialWeight", vec![(pred!(mut, ["w"], ["float"]), _)]),
	void cv_hfs_HfsSegment_setSpatialWeight_float(cv::hfs::HfsSegment* instance, float w, ResultVoid* ocvrs_return) {
		try {
			instance->setSpatialWeight(w);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSpatialWeight()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:95
	// ("cv::hfs::HfsSegment::getSpatialWeight", vec![(pred!(mut, [], []), _)]),
	void cv_hfs_HfsSegment_getSpatialWeight(cv::hfs::HfsSegment* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSpatialWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSlicSpixelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:105
	// ("cv::hfs::HfsSegment::setSlicSpixelSize", vec![(pred!(mut, ["n"], ["int"]), _)]),
	void cv_hfs_HfsSegment_setSlicSpixelSize_int(cv::hfs::HfsSegment* instance, int n, ResultVoid* ocvrs_return) {
		try {
			instance->setSlicSpixelSize(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSlicSpixelSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:106
	// ("cv::hfs::HfsSegment::getSlicSpixelSize", vec![(pred!(mut, [], []), _)]),
	void cv_hfs_HfsSegment_getSlicSpixelSize(cv::hfs::HfsSegment* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSlicSpixelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumSlicIter(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:113
	// ("cv::hfs::HfsSegment::setNumSlicIter", vec![(pred!(mut, ["n"], ["int"]), _)]),
	void cv_hfs_HfsSegment_setNumSlicIter_int(cv::hfs::HfsSegment* instance, int n, ResultVoid* ocvrs_return) {
		try {
			instance->setNumSlicIter(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumSlicIter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:114
	// ("cv::hfs::HfsSegment::getNumSlicIter", vec![(pred!(mut, [], []), _)]),
	void cv_hfs_HfsSegment_getNumSlicIter(cv::hfs::HfsSegment* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumSlicIter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// performSegmentGpu(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:125
	// ("cv::hfs::HfsSegment::performSegmentGpu", vec![(pred!(mut, ["src", "ifDraw"], ["const cv::_InputArray*", "bool"]), _)]),
	void cv_hfs_HfsSegment_performSegmentGpu_const__InputArrayR_bool(cv::hfs::HfsSegment* instance, const cv::_InputArray* src, bool ifDraw, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->performSegmentGpu(*src, ifDraw);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hfs::HfsSegment::performSegmentGpu(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:125
	// ("cv::hfs::HfsSegment::performSegmentGpu", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_hfs_HfsSegment_performSegmentGpu_const__InputArrayR(cv::hfs::HfsSegment* instance, const cv::_InputArray* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->performSegmentGpu(*src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// performSegmentCpu(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:131
	// ("cv::hfs::HfsSegment::performSegmentCpu", vec![(pred!(mut, ["src", "ifDraw"], ["const cv::_InputArray*", "bool"]), _)]),
	void cv_hfs_HfsSegment_performSegmentCpu_const__InputArrayR_bool(cv::hfs::HfsSegment* instance, const cv::_InputArray* src, bool ifDraw, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->performSegmentCpu(*src, ifDraw);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hfs::HfsSegment::performSegmentCpu(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:131
	// ("cv::hfs::HfsSegment::performSegmentCpu", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_hfs_HfsSegment_performSegmentCpu_const__InputArrayR(cv::hfs::HfsSegment* instance, const cv::_InputArray* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->performSegmentCpu(*src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, float, int, float, int, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:144
	// ("cv::hfs::HfsSegment::create", vec![(pred!(mut, ["height", "width", "segEgbThresholdI", "minRegionSizeI", "segEgbThresholdII", "minRegionSizeII", "spatialWeight", "slicSpixelSize", "numSlicIter"], ["int", "int", "float", "int", "float", "int", "float", "int", "int"]), _)]),
	void cv_hfs_HfsSegment_create_int_int_float_int_float_int_float_int_int(int height, int width, float segEgbThresholdI, int minRegionSizeI, float segEgbThresholdII, int minRegionSizeII, float spatialWeight, int slicSpixelSize, int numSlicIter, Result<cv::Ptr<cv::hfs::HfsSegment>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::hfs::HfsSegment> ret = cv::hfs::HfsSegment::create(height, width, segEgbThresholdI, minRegionSizeI, segEgbThresholdII, minRegionSizeII, spatialWeight, slicSpixelSize, numSlicIter);
			Ok(new cv::Ptr<cv::hfs::HfsSegment>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hfs::HfsSegment::create(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hfs.hpp:144
	// ("cv::hfs::HfsSegment::create", vec![(pred!(mut, ["height", "width"], ["int", "int"]), _)]),
	void cv_hfs_HfsSegment_create_int_int(int height, int width, Result<cv::Ptr<cv::hfs::HfsSegment>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::hfs::HfsSegment> ret = cv::hfs::HfsSegment::create(height, width);
			Ok(new cv::Ptr<cv::hfs::HfsSegment>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::hfs::HfsSegment::to_Algorithm() generated
	// ("cv::hfs::HfsSegment::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_hfs_HfsSegment_to_Algorithm(cv::hfs::HfsSegment* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::hfs::HfsSegment::delete() generated
	// ("cv::hfs::HfsSegment::delete", vec![(pred!(mut, [], []), _)]),
	void cv_hfs_HfsSegment_delete(cv::hfs::HfsSegment* instance) {
			delete instance;
	}

}

#include "ocvrs_common.hpp"
#include <opencv2/hfs.hpp>
#include "hfs_types.hpp"

extern "C" {
	Result_void cv_hfs_HfsSegment_setSegEgbThresholdI_float(cv::hfs::HfsSegment* instance, float c) {
		try {
			instance->setSegEgbThresholdI(c);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_hfs_HfsSegment_getSegEgbThresholdI(cv::hfs::HfsSegment* instance) {
		try {
			float ret = instance->getSegEgbThresholdI();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_hfs_HfsSegment_setMinRegionSizeI_int(cv::hfs::HfsSegment* instance, int n) {
		try {
			instance->setMinRegionSizeI(n);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_hfs_HfsSegment_getMinRegionSizeI(cv::hfs::HfsSegment* instance) {
		try {
			int ret = instance->getMinRegionSizeI();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_hfs_HfsSegment_setSegEgbThresholdII_float(cv::hfs::HfsSegment* instance, float c) {
		try {
			instance->setSegEgbThresholdII(c);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_hfs_HfsSegment_getSegEgbThresholdII(cv::hfs::HfsSegment* instance) {
		try {
			float ret = instance->getSegEgbThresholdII();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_hfs_HfsSegment_setMinRegionSizeII_int(cv::hfs::HfsSegment* instance, int n) {
		try {
			instance->setMinRegionSizeII(n);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_hfs_HfsSegment_getMinRegionSizeII(cv::hfs::HfsSegment* instance) {
		try {
			int ret = instance->getMinRegionSizeII();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_hfs_HfsSegment_setSpatialWeight_float(cv::hfs::HfsSegment* instance, float w) {
		try {
			instance->setSpatialWeight(w);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_hfs_HfsSegment_getSpatialWeight(cv::hfs::HfsSegment* instance) {
		try {
			float ret = instance->getSpatialWeight();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_hfs_HfsSegment_setSlicSpixelSize_int(cv::hfs::HfsSegment* instance, int n) {
		try {
			instance->setSlicSpixelSize(n);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_hfs_HfsSegment_getSlicSpixelSize(cv::hfs::HfsSegment* instance) {
		try {
			int ret = instance->getSlicSpixelSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_hfs_HfsSegment_setNumSlicIter_int(cv::hfs::HfsSegment* instance, int n) {
		try {
			instance->setNumSlicIter(n);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_hfs_HfsSegment_getNumSlicIter(cv::hfs::HfsSegment* instance) {
		try {
			int ret = instance->getNumSlicIter();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Mat*> cv_hfs_HfsSegment_performSegmentGpu_const__InputArrayR_bool(cv::hfs::HfsSegment* instance, const cv::_InputArray* src, bool ifDraw) {
		try {
			cv::Mat ret = instance->performSegmentGpu(*src, ifDraw);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_hfs_HfsSegment_performSegmentCpu_const__InputArrayR_bool(cv::hfs::HfsSegment* instance, const cv::_InputArray* src, bool ifDraw) {
		try {
			cv::Mat ret = instance->performSegmentCpu(*src, ifDraw);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Ptr<cv::hfs::HfsSegment>*> cv_hfs_HfsSegment_create_int_int_float_int_float_int_float_int_int(int height, int width, float segEgbThresholdI, int minRegionSizeI, float segEgbThresholdII, int minRegionSizeII, float spatialWeight, int slicSpixelSize, int numSlicIter) {
		try {
			cv::Ptr<cv::hfs::HfsSegment> ret = cv::hfs::HfsSegment::create(height, width, segEgbThresholdI, minRegionSizeI, segEgbThresholdII, minRegionSizeII, spatialWeight, slicSpixelSize, numSlicIter);
			return Ok(new cv::Ptr<cv::hfs::HfsSegment>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::hfs::HfsSegment>*>))
	}
	
}

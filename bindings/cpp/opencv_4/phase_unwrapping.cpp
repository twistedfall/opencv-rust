#include "common.hpp"
#include <opencv2/phase_unwrapping.hpp>
#include "phase_unwrapping_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>*> cv_phase_unwrapping_HistogramPhaseUnwrapping_create_const_ParamsX(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* parameters) {
		try {
			cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping> ret = cv::phase_unwrapping::HistogramPhaseUnwrapping::create(*parameters);
			return Ok(new cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>*>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_getInverseReliabilityMap_const__OutputArrayX(cv::phase_unwrapping::HistogramPhaseUnwrapping* instance, const cv::_OutputArray* reliabilityMap) {
		try {
			instance->getInverseReliabilityMap(*reliabilityMap);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_width_const(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		try {
			int ret = instance->width;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setWidth_int(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance, int val) {
		try {
			instance->width = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_height_const(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		try {
			int ret = instance->height;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setHeight_int(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance, int val) {
		try {
			instance->height = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_histThresh_const(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		try {
			float ret = instance->histThresh;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setHistThresh_float(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance, float val) {
		try {
			instance->histThresh = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_nbrOfSmallBins_const(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		try {
			int ret = instance->nbrOfSmallBins;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setNbrOfSmallBins_int(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance, int val) {
		try {
			instance->nbrOfSmallBins = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_nbrOfLargeBins_const(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		try {
			int ret = instance->nbrOfLargeBins;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setNbrOfLargeBins_int(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance, int val) {
		try {
			instance->nbrOfLargeBins = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_HistogramPhaseUnwrapping_Params_delete(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		delete instance;
	}
	Result<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_Params() {
		try {
			cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* ret = new cv::phase_unwrapping::HistogramPhaseUnwrapping::Params();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>)
	}
	
	Result_void cv_phase_unwrapping_PhaseUnwrapping_unwrapPhaseMap_const__InputArrayX_const__OutputArrayX_const__InputArrayX(cv::phase_unwrapping::PhaseUnwrapping* instance, const cv::_InputArray* wrappedPhaseMap, const cv::_OutputArray* unwrappedPhaseMap, const cv::_InputArray* shadowMask) {
		try {
			instance->unwrapPhaseMap(*wrappedPhaseMap, *unwrappedPhaseMap, *shadowMask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}

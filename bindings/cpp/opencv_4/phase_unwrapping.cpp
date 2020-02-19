#include "common.hpp"
#include <opencv2/phase_unwrapping.hpp>
#include "phase_unwrapping_types.hpp"

extern "C" {
	Result<void*> cv_phase_unwrapping_HistogramPhaseUnwrapping_create_const_ParamsX(void* parameters) {
		try {
			cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping> ret = cv::phase_unwrapping::HistogramPhaseUnwrapping::create(*reinterpret_cast<const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(parameters));
			return Ok<void*>(new cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_getInverseReliabilityMap_const__OutputArrayX(void* instance, void* reliabilityMap) {
		try {
			reinterpret_cast<cv::phase_unwrapping::HistogramPhaseUnwrapping*>(instance)->getInverseReliabilityMap(*reinterpret_cast<const cv::_OutputArray*>(reliabilityMap));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_width_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(instance)->width;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setWidth_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(instance)->width = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_height_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(instance)->height;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setHeight_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(instance)->height = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_histThresh_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(instance)->histThresh;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setHistThresh_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(instance)->histThresh = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_nbrOfSmallBins_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(instance)->nbrOfSmallBins;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setNbrOfSmallBins_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(instance)->nbrOfSmallBins = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_nbrOfLargeBins_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(instance)->nbrOfLargeBins;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_setNbrOfLargeBins_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params*>(instance)->nbrOfLargeBins = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_HistogramPhaseUnwrapping_Params_delete(cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* instance) {
		delete instance;
	}
	Result<void*> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_Params() {
		try {
			cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* ret = new cv::phase_unwrapping::HistogramPhaseUnwrapping::Params();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_phase_unwrapping_PhaseUnwrapping_unwrapPhaseMap_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* instance, void* wrappedPhaseMap, void* unwrappedPhaseMap, void* shadowMask) {
		try {
			reinterpret_cast<cv::phase_unwrapping::PhaseUnwrapping*>(instance)->unwrapPhaseMap(*reinterpret_cast<const cv::_InputArray*>(wrappedPhaseMap), *reinterpret_cast<const cv::_OutputArray*>(unwrappedPhaseMap), *reinterpret_cast<const cv::_InputArray*>(shadowMask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}

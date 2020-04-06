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
	
	Result<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params> cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_Params() {
		try {
			cv::phase_unwrapping::HistogramPhaseUnwrapping::Params ret;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params>)
	}
	
	Result_void cv_phase_unwrapping_PhaseUnwrapping_unwrapPhaseMap_const__InputArrayX_const__OutputArrayX_const__InputArrayX(cv::phase_unwrapping::PhaseUnwrapping* instance, const cv::_InputArray* wrappedPhaseMap, const cv::_OutputArray* unwrappedPhaseMap, const cv::_InputArray* shadowMask) {
		try {
			instance->unwrapPhaseMap(*wrappedPhaseMap, *unwrappedPhaseMap, *shadowMask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}

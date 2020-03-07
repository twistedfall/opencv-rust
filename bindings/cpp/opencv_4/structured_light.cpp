#include "common.hpp"
#include <opencv2/structured_light.hpp>
#include "structured_light_types.hpp"

extern "C" {
	Result<void*> cv_structured_light_GrayCodePattern_create_const_ParamsX(void* parameters) {
		try {
			cv::Ptr<cv::structured_light::GrayCodePattern> ret = cv::structured_light::GrayCodePattern::create(*reinterpret_cast<const cv::structured_light::GrayCodePattern::Params*>(parameters));
			return Ok<void*>(new cv::Ptr<cv::structured_light::GrayCodePattern>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_structured_light_GrayCodePattern_create_int_int(int width, int height) {
		try {
			cv::Ptr<cv::structured_light::GrayCodePattern> ret = cv::structured_light::GrayCodePattern::create(width, height);
			return Ok<void*>(new cv::Ptr<cv::structured_light::GrayCodePattern>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<size_t> cv_structured_light_GrayCodePattern_getNumberOfPatternImages_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::structured_light::GrayCodePattern*>(instance)->getNumberOfPatternImages();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_structured_light_GrayCodePattern_setWhiteThreshold_size_t(void* instance, size_t value) {
		try {
			reinterpret_cast<cv::structured_light::GrayCodePattern*>(instance)->setWhiteThreshold(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_structured_light_GrayCodePattern_setBlackThreshold_size_t(void* instance, size_t value) {
		try {
			reinterpret_cast<cv::structured_light::GrayCodePattern*>(instance)->setBlackThreshold(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_structured_light_GrayCodePattern_getImagesForShadowMasks_const_const__InputOutputArrayX_const__InputOutputArrayX(void* instance, void* blackImage, void* whiteImage) {
		try {
			reinterpret_cast<cv::structured_light::GrayCodePattern*>(instance)->getImagesForShadowMasks(*reinterpret_cast<const cv::_InputOutputArray*>(blackImage), *reinterpret_cast<const cv::_InputOutputArray*>(whiteImage));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_structured_light_GrayCodePattern_getProjPixel_const_const__InputArrayX_int_int_PointX(void* instance, void* patternImages, int x, int y, cv::Point* projPix) {
		try {
			bool ret = reinterpret_cast<cv::structured_light::GrayCodePattern*>(instance)->getProjPixel(*reinterpret_cast<const cv::_InputArray*>(patternImages), x, y, *projPix);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_structured_light_GrayCodePattern_Params_width_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::structured_light::GrayCodePattern::Params*>(instance)->width;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_GrayCodePattern_Params_setWidth_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::structured_light::GrayCodePattern::Params*>(instance)->width = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_GrayCodePattern_Params_height_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::structured_light::GrayCodePattern::Params*>(instance)->height;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_GrayCodePattern_Params_setHeight_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::structured_light::GrayCodePattern::Params*>(instance)->height = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_GrayCodePattern_Params_delete(cv::structured_light::GrayCodePattern::Params* instance) {
		delete instance;
	}
	Result<void*> cv_structured_light_GrayCodePattern_Params_Params() {
		try {
			cv::structured_light::GrayCodePattern::Params* ret = new cv::structured_light::GrayCodePattern::Params();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_structured_light_SinusoidalPattern_create_Ptr_Params_(void* parameters) {
		try {
			cv::Ptr<cv::structured_light::SinusoidalPattern> ret = cv::structured_light::SinusoidalPattern::create(*reinterpret_cast<cv::Ptr<cv::structured_light::SinusoidalPattern::Params>*>(parameters));
			return Ok<void*>(new cv::Ptr<cv::structured_light::SinusoidalPattern>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_computePhaseMap_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__InputArrayX(void* instance, void* patternImages, void* wrappedPhaseMap, void* shadowMask, void* fundamental) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern*>(instance)->computePhaseMap(*reinterpret_cast<const cv::_InputArray*>(patternImages), *reinterpret_cast<const cv::_OutputArray*>(wrappedPhaseMap), *reinterpret_cast<const cv::_OutputArray*>(shadowMask), *reinterpret_cast<const cv::_InputArray*>(fundamental));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_unwrapPhaseMap_const__InputArrayX_const__OutputArrayX_Size_const__InputArrayX(void* instance, void* wrappedPhaseMap, void* unwrappedPhaseMap, const cv::Size* camSize, void* shadowMask) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern*>(instance)->unwrapPhaseMap(*reinterpret_cast<const cv::_InputArray*>(wrappedPhaseMap), *reinterpret_cast<const cv::_OutputArray*>(unwrappedPhaseMap), *camSize, *reinterpret_cast<const cv::_InputArray*>(shadowMask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_findProCamMatches_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* instance, void* projUnwrappedPhaseMap, void* camUnwrappedPhaseMap, void* matches) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern*>(instance)->findProCamMatches(*reinterpret_cast<const cv::_InputArray*>(projUnwrappedPhaseMap), *reinterpret_cast<const cv::_InputArray*>(camUnwrappedPhaseMap), *reinterpret_cast<const cv::_OutputArray*>(matches));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_computeDataModulationTerm_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* instance, void* patternImages, void* dataModulationTerm, void* shadowMask) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern*>(instance)->computeDataModulationTerm(*reinterpret_cast<const cv::_InputArray*>(patternImages), *reinterpret_cast<const cv::_OutputArray*>(dataModulationTerm), *reinterpret_cast<const cv::_InputArray*>(shadowMask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_SinusoidalPattern_Params_width_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->width;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setWidth_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->width = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_SinusoidalPattern_Params_height_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->height;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setHeight_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->height = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_SinusoidalPattern_Params_nbrOfPeriods_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->nbrOfPeriods;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setNbrOfPeriods_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->nbrOfPeriods = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_structured_light_SinusoidalPattern_Params_shiftValue_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->shiftValue;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setShiftValue_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->shiftValue = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_SinusoidalPattern_Params_methodId_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->methodId;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setMethodId_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->methodId = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_SinusoidalPattern_Params_nbrOfPixelsBetweenMarkers_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->nbrOfPixelsBetweenMarkers;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setNbrOfPixelsBetweenMarkers_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->nbrOfPixelsBetweenMarkers = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_structured_light_SinusoidalPattern_Params_horizontal_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->horizontal;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setHorizontal_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->horizontal = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_structured_light_SinusoidalPattern_Params_setMarkers_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->setMarkers;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setSetMarkers_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->setMarkers = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_structured_light_SinusoidalPattern_Params_markersLocation(void* instance) {
		try {
			std::vector<cv::Point2f> ret = reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->markersLocation;
			return Ok<void*>(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setMarkersLocation_vector_Point2f_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::structured_light::SinusoidalPattern::Params*>(instance)->markersLocation = *reinterpret_cast<std::vector<cv::Point2f>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SinusoidalPattern_Params_delete(cv::structured_light::SinusoidalPattern::Params* instance) {
		delete instance;
	}
	Result<void*> cv_structured_light_SinusoidalPattern_Params_Params() {
		try {
			cv::structured_light::SinusoidalPattern::Params* ret = new cv::structured_light::SinusoidalPattern::Params();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_structured_light_StructuredLightPattern_generate_const__OutputArrayX(void* instance, void* patternImages) {
		try {
			bool ret = reinterpret_cast<cv::structured_light::StructuredLightPattern*>(instance)->generate(*reinterpret_cast<const cv::_OutputArray*>(patternImages));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_structured_light_StructuredLightPattern_decode_const_const_vector_vector_Mat__X_const__OutputArrayX_const__InputArrayX_const__InputArrayX_int(void* instance, void* patternImages, void* disparityMap, void* blackImages, void* whiteImages, int flags) {
		try {
			bool ret = reinterpret_cast<cv::structured_light::StructuredLightPattern*>(instance)->decode(*reinterpret_cast<const std::vector<std::vector<cv::Mat>>*>(patternImages), *reinterpret_cast<const cv::_OutputArray*>(disparityMap), *reinterpret_cast<const cv::_InputArray*>(blackImages), *reinterpret_cast<const cv::_InputArray*>(whiteImages), flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
}

#include "common.hpp"
#include <opencv2/structured_light.hpp>
#include "structured_light_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::structured_light::GrayCodePattern>*> cv_structured_light_GrayCodePattern_create_const_ParamsX(const cv::structured_light::GrayCodePattern::Params* parameters) {
		try {
			cv::Ptr<cv::structured_light::GrayCodePattern> ret = cv::structured_light::GrayCodePattern::create(*parameters);
			return Ok(new cv::Ptr<cv::structured_light::GrayCodePattern>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::structured_light::GrayCodePattern>*>)
	}
	
	Result<cv::Ptr<cv::structured_light::GrayCodePattern>*> cv_structured_light_GrayCodePattern_create_int_int(int width, int height) {
		try {
			cv::Ptr<cv::structured_light::GrayCodePattern> ret = cv::structured_light::GrayCodePattern::create(width, height);
			return Ok(new cv::Ptr<cv::structured_light::GrayCodePattern>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::structured_light::GrayCodePattern>*>)
	}
	
	Result<size_t> cv_structured_light_GrayCodePattern_getNumberOfPatternImages_const(const cv::structured_light::GrayCodePattern* instance) {
		try {
			size_t ret = instance->getNumberOfPatternImages();
			return Ok(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_structured_light_GrayCodePattern_setWhiteThreshold_size_t(cv::structured_light::GrayCodePattern* instance, size_t value) {
		try {
			instance->setWhiteThreshold(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_structured_light_GrayCodePattern_setBlackThreshold_size_t(cv::structured_light::GrayCodePattern* instance, size_t value) {
		try {
			instance->setBlackThreshold(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_structured_light_GrayCodePattern_getImagesForShadowMasks_const_const__InputOutputArrayX_const__InputOutputArrayX(const cv::structured_light::GrayCodePattern* instance, const cv::_InputOutputArray* blackImage, const cv::_InputOutputArray* whiteImage) {
		try {
			instance->getImagesForShadowMasks(*blackImage, *whiteImage);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_structured_light_GrayCodePattern_getProjPixel_const_const__InputArrayX_int_int_PointX(const cv::structured_light::GrayCodePattern* instance, const cv::_InputArray* patternImages, int x, int y, cv::Point* projPix) {
		try {
			bool ret = instance->getProjPixel(*patternImages, x, y, *projPix);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_structured_light_GrayCodePattern_Params_width_const(const cv::structured_light::GrayCodePattern::Params* instance) {
		try {
			int ret = instance->width;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_GrayCodePattern_Params_setWidth_int(cv::structured_light::GrayCodePattern::Params* instance, int val) {
		try {
			instance->width = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_GrayCodePattern_Params_height_const(const cv::structured_light::GrayCodePattern::Params* instance) {
		try {
			int ret = instance->height;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_GrayCodePattern_Params_setHeight_int(cv::structured_light::GrayCodePattern::Params* instance, int val) {
		try {
			instance->height = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_GrayCodePattern_Params_delete(cv::structured_light::GrayCodePattern::Params* instance) {
		delete instance;
	}
	Result<cv::structured_light::GrayCodePattern::Params*> cv_structured_light_GrayCodePattern_Params_Params() {
		try {
			cv::structured_light::GrayCodePattern::Params* ret = new cv::structured_light::GrayCodePattern::Params();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::structured_light::GrayCodePattern::Params*>)
	}
	
	Result<cv::Ptr<cv::structured_light::SinusoidalPattern>*> cv_structured_light_SinusoidalPattern_create_Ptr_Params_(cv::Ptr<cv::structured_light::SinusoidalPattern::Params>* parameters) {
		try {
			cv::Ptr<cv::structured_light::SinusoidalPattern> ret = cv::structured_light::SinusoidalPattern::create(*parameters);
			return Ok(new cv::Ptr<cv::structured_light::SinusoidalPattern>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::structured_light::SinusoidalPattern>*>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_computePhaseMap_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__InputArrayX(cv::structured_light::SinusoidalPattern* instance, const cv::_InputArray* patternImages, const cv::_OutputArray* wrappedPhaseMap, const cv::_OutputArray* shadowMask, const cv::_InputArray* fundamental) {
		try {
			instance->computePhaseMap(*patternImages, *wrappedPhaseMap, *shadowMask, *fundamental);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_unwrapPhaseMap_const__InputArrayX_const__OutputArrayX_Size_const__InputArrayX(cv::structured_light::SinusoidalPattern* instance, const cv::_InputArray* wrappedPhaseMap, const cv::_OutputArray* unwrappedPhaseMap, const cv::Size* camSize, const cv::_InputArray* shadowMask) {
		try {
			instance->unwrapPhaseMap(*wrappedPhaseMap, *unwrappedPhaseMap, *camSize, *shadowMask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_findProCamMatches_const__InputArrayX_const__InputArrayX_const__OutputArrayX(cv::structured_light::SinusoidalPattern* instance, const cv::_InputArray* projUnwrappedPhaseMap, const cv::_InputArray* camUnwrappedPhaseMap, const cv::_OutputArray* matches) {
		try {
			instance->findProCamMatches(*projUnwrappedPhaseMap, *camUnwrappedPhaseMap, *matches);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_computeDataModulationTerm_const__InputArrayX_const__OutputArrayX_const__InputArrayX(cv::structured_light::SinusoidalPattern* instance, const cv::_InputArray* patternImages, const cv::_OutputArray* dataModulationTerm, const cv::_InputArray* shadowMask) {
		try {
			instance->computeDataModulationTerm(*patternImages, *dataModulationTerm, *shadowMask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_SinusoidalPattern_Params_width_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
		try {
			int ret = instance->width;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setWidth_int(cv::structured_light::SinusoidalPattern::Params* instance, int val) {
		try {
			instance->width = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_SinusoidalPattern_Params_height_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
		try {
			int ret = instance->height;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setHeight_int(cv::structured_light::SinusoidalPattern::Params* instance, int val) {
		try {
			instance->height = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_SinusoidalPattern_Params_nbrOfPeriods_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
		try {
			int ret = instance->nbrOfPeriods;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setNbrOfPeriods_int(cv::structured_light::SinusoidalPattern::Params* instance, int val) {
		try {
			instance->nbrOfPeriods = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_structured_light_SinusoidalPattern_Params_shiftValue_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
		try {
			float ret = instance->shiftValue;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setShiftValue_float(cv::structured_light::SinusoidalPattern::Params* instance, float val) {
		try {
			instance->shiftValue = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_SinusoidalPattern_Params_methodId_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
		try {
			int ret = instance->methodId;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setMethodId_int(cv::structured_light::SinusoidalPattern::Params* instance, int val) {
		try {
			instance->methodId = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_structured_light_SinusoidalPattern_Params_nbrOfPixelsBetweenMarkers_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
		try {
			int ret = instance->nbrOfPixelsBetweenMarkers;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setNbrOfPixelsBetweenMarkers_int(cv::structured_light::SinusoidalPattern::Params* instance, int val) {
		try {
			instance->nbrOfPixelsBetweenMarkers = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_structured_light_SinusoidalPattern_Params_horizontal_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
		try {
			bool ret = instance->horizontal;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setHorizontal_bool(cv::structured_light::SinusoidalPattern::Params* instance, bool val) {
		try {
			instance->horizontal = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_structured_light_SinusoidalPattern_Params_setMarkers_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
		try {
			bool ret = instance->setMarkers;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setSetMarkers_bool(cv::structured_light::SinusoidalPattern::Params* instance, bool val) {
		try {
			instance->setMarkers = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Point2f>*> cv_structured_light_SinusoidalPattern_Params_markersLocation(cv::structured_light::SinusoidalPattern::Params* instance) {
		try {
			std::vector<cv::Point2f> ret = instance->markersLocation;
			return Ok(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Point2f>*>)
	}
	
	Result_void cv_structured_light_SinusoidalPattern_Params_setMarkersLocation_vector_Point2f_(cv::structured_light::SinusoidalPattern::Params* instance, std::vector<cv::Point2f>* val) {
		try {
			instance->markersLocation = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SinusoidalPattern_Params_delete(cv::structured_light::SinusoidalPattern::Params* instance) {
		delete instance;
	}
	Result<cv::structured_light::SinusoidalPattern::Params*> cv_structured_light_SinusoidalPattern_Params_Params() {
		try {
			cv::structured_light::SinusoidalPattern::Params* ret = new cv::structured_light::SinusoidalPattern::Params();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::structured_light::SinusoidalPattern::Params*>)
	}
	
	Result<bool> cv_structured_light_StructuredLightPattern_generate_const__OutputArrayX(cv::structured_light::StructuredLightPattern* instance, const cv::_OutputArray* patternImages) {
		try {
			bool ret = instance->generate(*patternImages);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_structured_light_StructuredLightPattern_decode_const_const_vector_vector_Mat__X_const__OutputArrayX_const__InputArrayX_const__InputArrayX_int(const cv::structured_light::StructuredLightPattern* instance, const std::vector<std::vector<cv::Mat>>* patternImages, const cv::_OutputArray* disparityMap, const cv::_InputArray* blackImages, const cv::_InputArray* whiteImages, int flags) {
		try {
			bool ret = instance->decode(*patternImages, *disparityMap, *blackImages, *whiteImages, flags);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
}

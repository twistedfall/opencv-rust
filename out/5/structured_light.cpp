#include "ocvrs_common.hpp"
#include <opencv2/structured_light.hpp>
#include "structured_light_types.hpp"

extern "C" {
	// create(const GrayCodePattern::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:86
	// ("cv::structured_light::GrayCodePattern::create", vec![(pred!(mut, ["parameters"], ["const cv::structured_light::GrayCodePattern::Params*"]), _)]),
	void cv_structured_light_GrayCodePattern_create_const_ParamsR(const cv::structured_light::GrayCodePattern::Params* parameters, Result<cv::Ptr<cv::structured_light::GrayCodePattern>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::structured_light::GrayCodePattern> ret = cv::structured_light::GrayCodePattern::create(*parameters);
			Ok(new cv::Ptr<cv::structured_light::GrayCodePattern>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::structured_light::GrayCodePattern::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:86
	// ("cv::structured_light::GrayCodePattern::create", vec![(pred!(mut, [], []), _)]),
	void cv_structured_light_GrayCodePattern_create(Result<cv::Ptr<cv::structured_light::GrayCodePattern>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::structured_light::GrayCodePattern> ret = cv::structured_light::GrayCodePattern::create();
			Ok(new cv::Ptr<cv::structured_light::GrayCodePattern>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:90
	// ("cv::structured_light::GrayCodePattern::create", vec![(pred!(mut, ["width", "height"], ["int", "int"]), _)]),
	void cv_structured_light_GrayCodePattern_create_int_int(int width, int height, Result<cv::Ptr<cv::structured_light::GrayCodePattern>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::structured_light::GrayCodePattern> ret = cv::structured_light::GrayCodePattern::create(width, height);
			Ok(new cv::Ptr<cv::structured_light::GrayCodePattern>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumberOfPatternImages()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:98
	// ("cv::structured_light::GrayCodePattern::getNumberOfPatternImages", vec![(pred!(const, [], []), _)]),
	void cv_structured_light_GrayCodePattern_getNumberOfPatternImages_const(const cv::structured_light::GrayCodePattern* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getNumberOfPatternImages();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWhiteThreshold(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:108
	// ("cv::structured_light::GrayCodePattern::setWhiteThreshold", vec![(pred!(mut, ["value"], ["size_t"]), _)]),
	void cv_structured_light_GrayCodePattern_setWhiteThreshold_size_t(cv::structured_light::GrayCodePattern* instance, size_t value, ResultVoid* ocvrs_return) {
		try {
			instance->setWhiteThreshold(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlackThreshold(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:118
	// ("cv::structured_light::GrayCodePattern::setBlackThreshold", vec![(pred!(mut, ["value"], ["size_t"]), _)]),
	void cv_structured_light_GrayCodePattern_setBlackThreshold_size_t(cv::structured_light::GrayCodePattern* instance, size_t value, ResultVoid* ocvrs_return) {
		try {
			instance->setBlackThreshold(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getImagesForShadowMasks(InputOutputArray, InputOutputArray)(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:130
	// ("cv::structured_light::GrayCodePattern::getImagesForShadowMasks", vec![(pred!(const, ["blackImage", "whiteImage"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_structured_light_GrayCodePattern_getImagesForShadowMasks_const_const__InputOutputArrayR_const__InputOutputArrayR(const cv::structured_light::GrayCodePattern* instance, const cv::_InputOutputArray* blackImage, const cv::_InputOutputArray* whiteImage, ResultVoid* ocvrs_return) {
		try {
			instance->getImagesForShadowMasks(*blackImage, *whiteImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getProjPixel(InputArrayOfArrays, int, int, Point &)(InputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:143
	// ("cv::structured_light::GrayCodePattern::getProjPixel", vec![(pred!(const, ["patternImages", "x", "y", "projPix"], ["const cv::_InputArray*", "int", "int", "cv::Point*"]), _)]),
	void cv_structured_light_GrayCodePattern_getProjPixel_const_const__InputArrayR_int_int_PointR(const cv::structured_light::GrayCodePattern* instance, const cv::_InputArray* patternImages, int x, int y, cv::Point* projPix, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getProjPixel(*patternImages, x, y, *projPix);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::structured_light::GrayCodePattern::to_Algorithm() generated
	// ("cv::structured_light::GrayCodePattern::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_structured_light_GrayCodePattern_to_Algorithm(cv::structured_light::GrayCodePattern* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::structured_light::GrayCodePattern::to_StructuredLightPattern() generated
	// ("cv::structured_light::GrayCodePattern::to_StructuredLightPattern", vec![(pred!(mut, [], []), _)]),
	cv::structured_light::StructuredLightPattern* cv_structured_light_GrayCodePattern_to_StructuredLightPattern(cv::structured_light::GrayCodePattern* instance) {
			return dynamic_cast<cv::structured_light::StructuredLightPattern*>(instance);
	}

	// cv::structured_light::GrayCodePattern::delete() generated
	// ("cv::structured_light::GrayCodePattern::delete", vec![(pred!(mut, [], []), _)]),
	void cv_structured_light_GrayCodePattern_delete(cv::structured_light::GrayCodePattern* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:78
	// ("cv::structured_light::GrayCodePattern::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_structured_light_GrayCodePattern_Params_Params(Result<cv::structured_light::GrayCodePattern::Params*>* ocvrs_return) {
		try {
			cv::structured_light::GrayCodePattern::Params* ret = new cv::structured_light::GrayCodePattern::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::structured_light::GrayCodePattern::Params::width() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:79
	// ("cv::structured_light::GrayCodePattern::Params::width", vec![(pred!(const, [], []), _)]),
	int cv_structured_light_GrayCodePattern_Params_propWidth_const(const cv::structured_light::GrayCodePattern::Params* instance) {
			int ret = instance->width;
			return ret;
	}

	// cv::structured_light::GrayCodePattern::Params::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:79
	// ("cv::structured_light::GrayCodePattern::Params::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_structured_light_GrayCodePattern_Params_propWidth_const_int(cv::structured_light::GrayCodePattern::Params* instance, const int val) {
			instance->width = val;
	}

	// cv::structured_light::GrayCodePattern::Params::height() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:80
	// ("cv::structured_light::GrayCodePattern::Params::height", vec![(pred!(const, [], []), _)]),
	int cv_structured_light_GrayCodePattern_Params_propHeight_const(const cv::structured_light::GrayCodePattern::Params* instance) {
			int ret = instance->height;
			return ret;
	}

	// cv::structured_light::GrayCodePattern::Params::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:80
	// ("cv::structured_light::GrayCodePattern::Params::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_structured_light_GrayCodePattern_Params_propHeight_const_int(cv::structured_light::GrayCodePattern::Params* instance, const int val) {
			instance->height = val;
	}

	// cv::structured_light::GrayCodePattern::Params::delete() generated
	// ("cv::structured_light::GrayCodePattern::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_structured_light_GrayCodePattern_Params_delete(cv::structured_light::GrayCodePattern::Params* instance) {
			delete instance;
	}

	// create(Ptr<SinusoidalPattern::Params>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:100
	// ("cv::structured_light::SinusoidalPattern::create", vec![(pred!(mut, ["parameters"], ["cv::Ptr<cv::structured_light::SinusoidalPattern::Params>"]), _)]),
	void cv_structured_light_SinusoidalPattern_create_PtrLParamsG(cv::Ptr<cv::structured_light::SinusoidalPattern::Params>* parameters, Result<cv::Ptr<cv::structured_light::SinusoidalPattern>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::structured_light::SinusoidalPattern> ret = cv::structured_light::SinusoidalPattern::create(*parameters);
			Ok(new cv::Ptr<cv::structured_light::SinusoidalPattern>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::structured_light::SinusoidalPattern::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:100
	// ("cv::structured_light::SinusoidalPattern::create", vec![(pred!(mut, [], []), _)]),
	void cv_structured_light_SinusoidalPattern_create(Result<cv::Ptr<cv::structured_light::SinusoidalPattern>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::structured_light::SinusoidalPattern> ret = cv::structured_light::SinusoidalPattern::create();
			Ok(new cv::Ptr<cv::structured_light::SinusoidalPattern>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computePhaseMap(InputArrayOfArrays, OutputArray, OutputArray, InputArray)(InputArray, OutputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:110
	// ("cv::structured_light::SinusoidalPattern::computePhaseMap", vec![(pred!(mut, ["patternImages", "wrappedPhaseMap", "shadowMask", "fundamental"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_structured_light_SinusoidalPattern_computePhaseMap_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(cv::structured_light::SinusoidalPattern* instance, const cv::_InputArray* patternImages, const cv::_OutputArray* wrappedPhaseMap, const cv::_OutputArray* shadowMask, const cv::_InputArray* fundamental, ResultVoid* ocvrs_return) {
		try {
			instance->computePhaseMap(*patternImages, *wrappedPhaseMap, *shadowMask, *fundamental);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::structured_light::SinusoidalPattern::computePhaseMap(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:110
	// ("cv::structured_light::SinusoidalPattern::computePhaseMap", vec![(pred!(mut, ["patternImages", "wrappedPhaseMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_structured_light_SinusoidalPattern_computePhaseMap_const__InputArrayR_const__OutputArrayR(cv::structured_light::SinusoidalPattern* instance, const cv::_InputArray* patternImages, const cv::_OutputArray* wrappedPhaseMap, ResultVoid* ocvrs_return) {
		try {
			instance->computePhaseMap(*patternImages, *wrappedPhaseMap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unwrapPhaseMap(InputArray, OutputArray, cv::Size, InputArray)(InputArray, OutputArray, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:122
	// ("cv::structured_light::SinusoidalPattern::unwrapPhaseMap", vec![(pred!(mut, ["wrappedPhaseMap", "unwrappedPhaseMap", "camSize", "shadowMask"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "const cv::_InputArray*"]), _)]),
	void cv_structured_light_SinusoidalPattern_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_Size_const__InputArrayR(cv::structured_light::SinusoidalPattern* instance, const cv::_InputArray* wrappedPhaseMap, const cv::_OutputArray* unwrappedPhaseMap, cv::Size* camSize, const cv::_InputArray* shadowMask, ResultVoid* ocvrs_return) {
		try {
			instance->unwrapPhaseMap(*wrappedPhaseMap, *unwrappedPhaseMap, *camSize, *shadowMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::structured_light::SinusoidalPattern::unwrapPhaseMap(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:122
	// ("cv::structured_light::SinusoidalPattern::unwrapPhaseMap", vec![(pred!(mut, ["wrappedPhaseMap", "unwrappedPhaseMap", "camSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
	void cv_structured_light_SinusoidalPattern_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_Size(cv::structured_light::SinusoidalPattern* instance, const cv::_InputArray* wrappedPhaseMap, const cv::_OutputArray* unwrappedPhaseMap, cv::Size* camSize, ResultVoid* ocvrs_return) {
		try {
			instance->unwrapPhaseMap(*wrappedPhaseMap, *unwrappedPhaseMap, *camSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findProCamMatches(InputArray, InputArray, OutputArrayOfArrays)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:133
	// ("cv::structured_light::SinusoidalPattern::findProCamMatches", vec![(pred!(mut, ["projUnwrappedPhaseMap", "camUnwrappedPhaseMap", "matches"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_structured_light_SinusoidalPattern_findProCamMatches_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::structured_light::SinusoidalPattern* instance, const cv::_InputArray* projUnwrappedPhaseMap, const cv::_InputArray* camUnwrappedPhaseMap, const cv::_OutputArray* matches, ResultVoid* ocvrs_return) {
		try {
			instance->findProCamMatches(*projUnwrappedPhaseMap, *camUnwrappedPhaseMap, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeDataModulationTerm(InputArrayOfArrays, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:143
	// ("cv::structured_light::SinusoidalPattern::computeDataModulationTerm", vec![(pred!(mut, ["patternImages", "dataModulationTerm", "shadowMask"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_structured_light_SinusoidalPattern_computeDataModulationTerm_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::structured_light::SinusoidalPattern* instance, const cv::_InputArray* patternImages, const cv::_OutputArray* dataModulationTerm, const cv::_InputArray* shadowMask, ResultVoid* ocvrs_return) {
		try {
			instance->computeDataModulationTerm(*patternImages, *dataModulationTerm, *shadowMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::structured_light::SinusoidalPattern::to_Algorithm() generated
	// ("cv::structured_light::SinusoidalPattern::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_structured_light_SinusoidalPattern_to_Algorithm(cv::structured_light::SinusoidalPattern* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::structured_light::SinusoidalPattern::to_StructuredLightPattern() generated
	// ("cv::structured_light::SinusoidalPattern::to_StructuredLightPattern", vec![(pred!(mut, [], []), _)]),
	cv::structured_light::StructuredLightPattern* cv_structured_light_SinusoidalPattern_to_StructuredLightPattern(cv::structured_light::SinusoidalPattern* instance) {
			return dynamic_cast<cv::structured_light::StructuredLightPattern*>(instance);
	}

	// cv::structured_light::SinusoidalPattern::delete() generated
	// ("cv::structured_light::SinusoidalPattern::delete", vec![(pred!(mut, [], []), _)]),
	void cv_structured_light_SinusoidalPattern_delete(cv::structured_light::SinusoidalPattern* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:84
	// ("cv::structured_light::SinusoidalPattern::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_structured_light_SinusoidalPattern_Params_Params(Result<cv::structured_light::SinusoidalPattern::Params*>* ocvrs_return) {
		try {
			cv::structured_light::SinusoidalPattern::Params* ret = new cv::structured_light::SinusoidalPattern::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::structured_light::SinusoidalPattern::Params::width() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:85
	// ("cv::structured_light::SinusoidalPattern::Params::width", vec![(pred!(const, [], []), _)]),
	int cv_structured_light_SinusoidalPattern_Params_propWidth_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
			int ret = instance->width;
			return ret;
	}

	// cv::structured_light::SinusoidalPattern::Params::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:85
	// ("cv::structured_light::SinusoidalPattern::Params::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_structured_light_SinusoidalPattern_Params_propWidth_const_int(cv::structured_light::SinusoidalPattern::Params* instance, const int val) {
			instance->width = val;
	}

	// cv::structured_light::SinusoidalPattern::Params::height() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:86
	// ("cv::structured_light::SinusoidalPattern::Params::height", vec![(pred!(const, [], []), _)]),
	int cv_structured_light_SinusoidalPattern_Params_propHeight_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
			int ret = instance->height;
			return ret;
	}

	// cv::structured_light::SinusoidalPattern::Params::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:86
	// ("cv::structured_light::SinusoidalPattern::Params::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_structured_light_SinusoidalPattern_Params_propHeight_const_int(cv::structured_light::SinusoidalPattern::Params* instance, const int val) {
			instance->height = val;
	}

	// cv::structured_light::SinusoidalPattern::Params::nbrOfPeriods() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:87
	// ("cv::structured_light::SinusoidalPattern::Params::nbrOfPeriods", vec![(pred!(const, [], []), _)]),
	int cv_structured_light_SinusoidalPattern_Params_propNbrOfPeriods_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
			int ret = instance->nbrOfPeriods;
			return ret;
	}

	// cv::structured_light::SinusoidalPattern::Params::setNbrOfPeriods(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:87
	// ("cv::structured_light::SinusoidalPattern::Params::setNbrOfPeriods", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_structured_light_SinusoidalPattern_Params_propNbrOfPeriods_const_int(cv::structured_light::SinusoidalPattern::Params* instance, const int val) {
			instance->nbrOfPeriods = val;
	}

	// cv::structured_light::SinusoidalPattern::Params::shiftValue() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:88
	// ("cv::structured_light::SinusoidalPattern::Params::shiftValue", vec![(pred!(const, [], []), _)]),
	float cv_structured_light_SinusoidalPattern_Params_propShiftValue_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
			float ret = instance->shiftValue;
			return ret;
	}

	// cv::structured_light::SinusoidalPattern::Params::setShiftValue(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:88
	// ("cv::structured_light::SinusoidalPattern::Params::setShiftValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_structured_light_SinusoidalPattern_Params_propShiftValue_const_float(cv::structured_light::SinusoidalPattern::Params* instance, const float val) {
			instance->shiftValue = val;
	}

	// cv::structured_light::SinusoidalPattern::Params::methodId() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:89
	// ("cv::structured_light::SinusoidalPattern::Params::methodId", vec![(pred!(const, [], []), _)]),
	int cv_structured_light_SinusoidalPattern_Params_propMethodId_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
			int ret = instance->methodId;
			return ret;
	}

	// cv::structured_light::SinusoidalPattern::Params::setMethodId(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:89
	// ("cv::structured_light::SinusoidalPattern::Params::setMethodId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_structured_light_SinusoidalPattern_Params_propMethodId_const_int(cv::structured_light::SinusoidalPattern::Params* instance, const int val) {
			instance->methodId = val;
	}

	// cv::structured_light::SinusoidalPattern::Params::nbrOfPixelsBetweenMarkers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:90
	// ("cv::structured_light::SinusoidalPattern::Params::nbrOfPixelsBetweenMarkers", vec![(pred!(const, [], []), _)]),
	int cv_structured_light_SinusoidalPattern_Params_propNbrOfPixelsBetweenMarkers_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
			int ret = instance->nbrOfPixelsBetweenMarkers;
			return ret;
	}

	// cv::structured_light::SinusoidalPattern::Params::setNbrOfPixelsBetweenMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:90
	// ("cv::structured_light::SinusoidalPattern::Params::setNbrOfPixelsBetweenMarkers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_structured_light_SinusoidalPattern_Params_propNbrOfPixelsBetweenMarkers_const_int(cv::structured_light::SinusoidalPattern::Params* instance, const int val) {
			instance->nbrOfPixelsBetweenMarkers = val;
	}

	// cv::structured_light::SinusoidalPattern::Params::horizontal() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:91
	// ("cv::structured_light::SinusoidalPattern::Params::horizontal", vec![(pred!(const, [], []), _)]),
	bool cv_structured_light_SinusoidalPattern_Params_propHorizontal_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
			bool ret = instance->horizontal;
			return ret;
	}

	// cv::structured_light::SinusoidalPattern::Params::setHorizontal(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:91
	// ("cv::structured_light::SinusoidalPattern::Params::setHorizontal", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_structured_light_SinusoidalPattern_Params_propHorizontal_const_bool(cv::structured_light::SinusoidalPattern::Params* instance, const bool val) {
			instance->horizontal = val;
	}

	// cv::structured_light::SinusoidalPattern::Params::setMarkers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:92
	// ("cv::structured_light::SinusoidalPattern::Params::setMarkers", vec![(pred!(const, [], []), _)]),
	bool cv_structured_light_SinusoidalPattern_Params_propSetMarkers_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
			bool ret = instance->setMarkers;
			return ret;
	}

	// cv::structured_light::SinusoidalPattern::Params::setSetMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:92
	// ("cv::structured_light::SinusoidalPattern::Params::setSetMarkers", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_structured_light_SinusoidalPattern_Params_propSetMarkers_const_bool(cv::structured_light::SinusoidalPattern::Params* instance, const bool val) {
			instance->setMarkers = val;
	}

	// cv::structured_light::SinusoidalPattern::Params::markersLocation() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:93
	// ("cv::structured_light::SinusoidalPattern::Params::markersLocation", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point2f>* cv_structured_light_SinusoidalPattern_Params_propMarkersLocation_const(const cv::structured_light::SinusoidalPattern::Params* instance) {
			std::vector<cv::Point2f> ret = instance->markersLocation;
			return new std::vector<cv::Point2f>(ret);
	}

	// cv::structured_light::SinusoidalPattern::Params::setMarkersLocation(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:93
	// ("cv::structured_light::SinusoidalPattern::Params::setMarkersLocation", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point2f>"]), _)]),
	void cv_structured_light_SinusoidalPattern_Params_propMarkersLocation_const_vectorLPoint2fG(cv::structured_light::SinusoidalPattern::Params* instance, const std::vector<cv::Point2f>* val) {
			instance->markersLocation = *val;
	}

	// cv::structured_light::SinusoidalPattern::Params::delete() generated
	// ("cv::structured_light::SinusoidalPattern::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_structured_light_SinusoidalPattern_Params_delete(cv::structured_light::SinusoidalPattern::Params* instance) {
			delete instance;
	}

	// generate(OutputArrayOfArrays)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/structured_light.hpp:69
	// ("cv::structured_light::StructuredLightPattern::generate", vec![(pred!(mut, ["patternImages"], ["const cv::_OutputArray*"]), _)]),
	void cv_structured_light_StructuredLightPattern_generate_const__OutputArrayR(cv::structured_light::StructuredLightPattern* instance, const cv::_OutputArray* patternImages, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->generate(*patternImages);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decode(const std::vector<std::vector<Mat>> &, OutputArray, InputArrayOfArrays, InputArrayOfArrays, int)(CppPassByVoidPtr, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/structured_light.hpp:81
	// ("cv::structured_light::StructuredLightPattern::decode", vec![(pred!(const, ["patternImages", "disparityMap", "blackImages", "whiteImages", "flags"], ["const std::vector<std::vector<cv::Mat>>*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_structured_light_StructuredLightPattern_decode_const_const_vectorLvectorLMatGGR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::structured_light::StructuredLightPattern* instance, const std::vector<std::vector<cv::Mat>>* patternImages, const cv::_OutputArray* disparityMap, const cv::_InputArray* blackImages, const cv::_InputArray* whiteImages, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decode(*patternImages, *disparityMap, *blackImages, *whiteImages, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::structured_light::StructuredLightPattern::decode(CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/structured_light.hpp:81
	// ("cv::structured_light::StructuredLightPattern::decode", vec![(pred!(const, ["patternImages", "disparityMap"], ["const std::vector<std::vector<cv::Mat>>*", "const cv::_OutputArray*"]), _)]),
	void cv_structured_light_StructuredLightPattern_decode_const_const_vectorLvectorLMatGGR_const__OutputArrayR(const cv::structured_light::StructuredLightPattern* instance, const std::vector<std::vector<cv::Mat>>* patternImages, const cv::_OutputArray* disparityMap, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decode(*patternImages, *disparityMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::structured_light::StructuredLightPattern::to_GrayCodePattern() generated
	// ("cv::structured_light::StructuredLightPattern::to_GrayCodePattern", vec![(pred!(mut, [], []), _)]),
	cv::structured_light::GrayCodePattern* cv_structured_light_StructuredLightPattern_to_GrayCodePattern(cv::structured_light::StructuredLightPattern* instance) {
			return dynamic_cast<cv::structured_light::GrayCodePattern*>(instance);
	}

	// cv::structured_light::StructuredLightPattern::to_SinusoidalPattern() generated
	// ("cv::structured_light::StructuredLightPattern::to_SinusoidalPattern", vec![(pred!(mut, [], []), _)]),
	cv::structured_light::SinusoidalPattern* cv_structured_light_StructuredLightPattern_to_SinusoidalPattern(cv::structured_light::StructuredLightPattern* instance) {
			return dynamic_cast<cv::structured_light::SinusoidalPattern*>(instance);
	}

	// cv::structured_light::StructuredLightPattern::to_Algorithm() generated
	// ("cv::structured_light::StructuredLightPattern::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_structured_light_StructuredLightPattern_to_Algorithm(cv::structured_light::StructuredLightPattern* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::structured_light::StructuredLightPattern::delete() generated
	// ("cv::structured_light::StructuredLightPattern::delete", vec![(pred!(mut, [], []), _)]),
	void cv_structured_light_StructuredLightPattern_delete(cv::structured_light::StructuredLightPattern* instance) {
			delete instance;
	}

}

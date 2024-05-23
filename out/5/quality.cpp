#include "ocvrs_common.hpp"
#include <opencv2/quality.hpp>
#include "quality_types.hpp"

extern "C" {
	// compute(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybrisque.hpp:33
	// ("cv::quality::QualityBRISQUE::compute", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_quality_QualityBRISQUE_compute_const__InputArrayR(cv::quality::QualityBRISQUE* instance, const cv::_InputArray* img, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const cv::String &, const cv::String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybrisque.hpp:40
	// ("cv::quality::QualityBRISQUE::create", vec![(pred!(mut, ["model_file_path", "range_file_path"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_quality_QualityBRISQUE_create_const_StringR_const_StringR(const char* model_file_path, const char* range_file_path, Result<cv::Ptr<cv::quality::QualityBRISQUE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualityBRISQUE> ret = cv::quality::QualityBRISQUE::create(std::string(model_file_path), std::string(range_file_path));
			Ok(new cv::Ptr<cv::quality::QualityBRISQUE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const cv::Ptr<cv::ml::SVM> &, const cv::Mat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybrisque.hpp:47
	// ("cv::quality::QualityBRISQUE::create", vec![(pred!(mut, ["model", "range"], ["const cv::Ptr<cv::ml::SVM>*", "const cv::Mat*"]), _)]),
	void cv_quality_QualityBRISQUE_create_const_PtrLSVMGR_const_MatR(const cv::Ptr<cv::ml::SVM>* model, const cv::Mat* range, Result<cv::Ptr<cv::quality::QualityBRISQUE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualityBRISQUE> ret = cv::quality::QualityBRISQUE::create(*model, *range);
			Ok(new cv::Ptr<cv::quality::QualityBRISQUE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, const cv::String &, const cv::String &)(InputArray, InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybrisque.hpp:56
	// ("cv::quality::QualityBRISQUE::compute", vec![(pred!(mut, ["img", "model_file_path", "range_file_path"], ["const cv::_InputArray*", "const cv::String*", "const cv::String*"]), _)]),
	void cv_quality_QualityBRISQUE_compute_const__InputArrayR_const_StringR_const_StringR(const cv::_InputArray* img, const char* model_file_path, const char* range_file_path, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::quality::QualityBRISQUE::compute(*img, std::string(model_file_path), std::string(range_file_path));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeFeatures(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybrisque.hpp:63
	// ("cv::quality::QualityBRISQUE::computeFeatures", vec![(pred!(mut, ["img", "features"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_quality_QualityBRISQUE_computeFeatures_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* img, const cv::_OutputArray* features, ResultVoid* ocvrs_return) {
		try {
			cv::quality::QualityBRISQUE::computeFeatures(*img, *features);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::quality::QualityBRISQUE::to_Algorithm() generated
	// ("cv::quality::QualityBRISQUE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_quality_QualityBRISQUE_to_Algorithm(cv::quality::QualityBRISQUE* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::quality::QualityBRISQUE::to_QualityBase() generated
	// ("cv::quality::QualityBRISQUE::to_QualityBase", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityBase* cv_quality_QualityBRISQUE_to_QualityBase(cv::quality::QualityBRISQUE* instance) {
			return dynamic_cast<cv::quality::QualityBase*>(instance);
	}

	// cv::quality::QualityBRISQUE::delete() generated
	// ("cv::quality::QualityBRISQUE::delete", vec![(pred!(mut, [], []), _)]),
	void cv_quality_QualityBRISQUE_delete(cv::quality::QualityBRISQUE* instance) {
			delete instance;
	}

	// compute(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybase.hpp:35
	// ("cv::quality::QualityBase::compute", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_quality_QualityBase_compute_const__InputArrayR(cv::quality::QualityBase* instance, const cv::_InputArray* img, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getQualityMap(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybase.hpp:38
	// ("cv::quality::QualityBase::getQualityMap", vec![(pred!(const, ["dst"], ["const cv::_OutputArray*"]), _)]),
	void cv_quality_QualityBase_getQualityMap_const_const__OutputArrayR(const cv::quality::QualityBase* instance, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->getQualityMap(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybase.hpp:46
	// ("cv::quality::QualityBase::clear", vec![(pred!(mut, [], []), _)]),
	void cv_quality_QualityBase_clear(cv::quality::QualityBase* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybase.hpp:49
	// ("cv::quality::QualityBase::empty", vec![(pred!(const, [], []), _)]),
	void cv_quality_QualityBase_empty_const(const cv::quality::QualityBase* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::quality::QualityBase::to_QualityBRISQUE() generated
	// ("cv::quality::QualityBase::to_QualityBRISQUE", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityBRISQUE* cv_quality_QualityBase_to_QualityBRISQUE(cv::quality::QualityBase* instance) {
			return dynamic_cast<cv::quality::QualityBRISQUE*>(instance);
	}

	// cv::quality::QualityBase::to_QualityGMSD() generated
	// ("cv::quality::QualityBase::to_QualityGMSD", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityGMSD* cv_quality_QualityBase_to_QualityGMSD(cv::quality::QualityBase* instance) {
			return dynamic_cast<cv::quality::QualityGMSD*>(instance);
	}

	// cv::quality::QualityBase::to_QualityMSE() generated
	// ("cv::quality::QualityBase::to_QualityMSE", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityMSE* cv_quality_QualityBase_to_QualityMSE(cv::quality::QualityBase* instance) {
			return dynamic_cast<cv::quality::QualityMSE*>(instance);
	}

	// cv::quality::QualityBase::to_QualityPSNR() generated
	// ("cv::quality::QualityBase::to_QualityPSNR", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityPSNR* cv_quality_QualityBase_to_QualityPSNR(cv::quality::QualityBase* instance) {
			return dynamic_cast<cv::quality::QualityPSNR*>(instance);
	}

	// cv::quality::QualityBase::to_QualitySSIM() generated
	// ("cv::quality::QualityBase::to_QualitySSIM", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualitySSIM* cv_quality_QualityBase_to_QualitySSIM(cv::quality::QualityBase* instance) {
			return dynamic_cast<cv::quality::QualitySSIM*>(instance);
	}

	// cv::quality::QualityBase::to_Algorithm() generated
	// ("cv::quality::QualityBase::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_quality_QualityBase_to_Algorithm(cv::quality::QualityBase* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::quality::QualityBase::delete() generated
	// ("cv::quality::QualityBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_quality_QualityBase_delete(cv::quality::QualityBase* instance) {
			delete instance;
	}

	// compute(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitygmsd.hpp:28
	// ("cv::quality::QualityGMSD::compute", vec![(pred!(mut, ["cmp"], ["const cv::_InputArray*"]), _)]),
	void cv_quality_QualityGMSD_compute_const__InputArrayR(cv::quality::QualityGMSD* instance, const cv::_InputArray* cmp, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*cmp);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitygmsd.hpp:31
	// ("cv::quality::QualityGMSD::empty", vec![(pred!(const, [], []), _)]),
	void cv_quality_QualityGMSD_empty_const(const cv::quality::QualityGMSD* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitygmsd.hpp:34
	// ("cv::quality::QualityGMSD::clear", vec![(pred!(mut, [], []), _)]),
	void cv_quality_QualityGMSD_clear(cv::quality::QualityGMSD* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitygmsd.hpp:40
	// ("cv::quality::QualityGMSD::create", vec![(pred!(mut, ["ref"], ["const cv::_InputArray*"]), _)]),
	void cv_quality_QualityGMSD_create_const__InputArrayR(const cv::_InputArray* ref, Result<cv::Ptr<cv::quality::QualityGMSD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualityGMSD> ret = cv::quality::QualityGMSD::create(*ref);
			Ok(new cv::Ptr<cv::quality::QualityGMSD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitygmsd.hpp:49
	// ("cv::quality::QualityGMSD::compute", vec![(pred!(mut, ["ref", "cmp", "qualityMap"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_quality_QualityGMSD_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::quality::QualityGMSD::compute(*ref, *cmp, *qualityMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::quality::QualityGMSD::to_Algorithm() generated
	// ("cv::quality::QualityGMSD::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_quality_QualityGMSD_to_Algorithm(cv::quality::QualityGMSD* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::quality::QualityGMSD::to_QualityBase() generated
	// ("cv::quality::QualityGMSD::to_QualityBase", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityBase* cv_quality_QualityGMSD_to_QualityBase(cv::quality::QualityGMSD* instance) {
			return dynamic_cast<cv::quality::QualityBase*>(instance);
	}

	// cv::quality::QualityGMSD::delete() generated
	// ("cv::quality::QualityGMSD::delete", vec![(pred!(mut, [], []), _)]),
	void cv_quality_QualityGMSD_delete(cv::quality::QualityGMSD* instance) {
			delete instance;
	}

	// compute(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitymse.hpp:25
	// ("cv::quality::QualityMSE::compute", vec![(pred!(mut, ["cmpImgs"], ["const cv::_InputArray*"]), _)]),
	void cv_quality_QualityMSE_compute_const__InputArrayR(cv::quality::QualityMSE* instance, const cv::_InputArray* cmpImgs, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*cmpImgs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitymse.hpp:28
	// ("cv::quality::QualityMSE::empty", vec![(pred!(const, [], []), _)]),
	void cv_quality_QualityMSE_empty_const(const cv::quality::QualityMSE* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitymse.hpp:31
	// ("cv::quality::QualityMSE::clear", vec![(pred!(mut, [], []), _)]),
	void cv_quality_QualityMSE_clear(cv::quality::QualityMSE* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitymse.hpp:37
	// ("cv::quality::QualityMSE::create", vec![(pred!(mut, ["ref"], ["const cv::_InputArray*"]), _)]),
	void cv_quality_QualityMSE_create_const__InputArrayR(const cv::_InputArray* ref, Result<cv::Ptr<cv::quality::QualityMSE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualityMSE> ret = cv::quality::QualityMSE::create(*ref);
			Ok(new cv::Ptr<cv::quality::QualityMSE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitymse.hpp:46
	// ("cv::quality::QualityMSE::compute", vec![(pred!(mut, ["ref", "cmp", "qualityMap"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_quality_QualityMSE_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::quality::QualityMSE::compute(*ref, *cmp, *qualityMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::quality::QualityMSE::to_Algorithm() generated
	// ("cv::quality::QualityMSE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_quality_QualityMSE_to_Algorithm(cv::quality::QualityMSE* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::quality::QualityMSE::to_QualityBase() generated
	// ("cv::quality::QualityMSE::to_QualityBase", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityBase* cv_quality_QualityMSE_to_QualityBase(cv::quality::QualityMSE* instance) {
			return dynamic_cast<cv::quality::QualityBase*>(instance);
	}

	// cv::quality::QualityMSE::delete() generated
	// ("cv::quality::QualityMSE::delete", vec![(pred!(mut, [], []), _)]),
	void cv_quality_QualityMSE_delete(cv::quality::QualityMSE* instance) {
			delete instance;
	}

	// create(InputArray, double)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:38
	// ("cv::quality::QualityPSNR::create", vec![(pred!(mut, ["ref", "maxPixelValue"], ["const cv::_InputArray*", "double"]), _)]),
	void cv_quality_QualityPSNR_create_const__InputArrayR_double(const cv::_InputArray* ref, double maxPixelValue, Result<cv::Ptr<cv::quality::QualityPSNR>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualityPSNR> ret = cv::quality::QualityPSNR::create(*ref, maxPixelValue);
			Ok(new cv::Ptr<cv::quality::QualityPSNR>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::quality::QualityPSNR::create(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:38
	// ("cv::quality::QualityPSNR::create", vec![(pred!(mut, ["ref"], ["const cv::_InputArray*"]), _)]),
	void cv_quality_QualityPSNR_create_const__InputArrayR(const cv::_InputArray* ref, Result<cv::Ptr<cv::quality::QualityPSNR>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualityPSNR> ret = cv::quality::QualityPSNR::create(*ref);
			Ok(new cv::Ptr<cv::quality::QualityPSNR>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:48
	// ("cv::quality::QualityPSNR::compute", vec![(pred!(mut, ["cmp"], ["const cv::_InputArray*"]), _)]),
	void cv_quality_QualityPSNR_compute_const__InputArrayR(cv::quality::QualityPSNR* instance, const cv::_InputArray* cmp, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*cmp);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:59
	// ("cv::quality::QualityPSNR::empty", vec![(pred!(const, [], []), _)]),
	void cv_quality_QualityPSNR_empty_const(const cv::quality::QualityPSNR* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:62
	// ("cv::quality::QualityPSNR::clear", vec![(pred!(mut, [], []), _)]),
	void cv_quality_QualityPSNR_clear(cv::quality::QualityPSNR* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, InputArray, OutputArray, double)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:72
	// ("cv::quality::QualityPSNR::compute", vec![(pred!(mut, ["ref", "cmp", "qualityMap", "maxPixelValue"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_quality_QualityPSNR_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap, double maxPixelValue, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::quality::QualityPSNR::compute(*ref, *cmp, *qualityMap, maxPixelValue);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::quality::QualityPSNR::compute(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:72
	// ("cv::quality::QualityPSNR::compute", vec![(pred!(mut, ["ref", "cmp", "qualityMap"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_quality_QualityPSNR_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::quality::QualityPSNR::compute(*ref, *cmp, *qualityMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxPixelValue()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:81
	// ("cv::quality::QualityPSNR::getMaxPixelValue", vec![(pred!(const, [], []), _)]),
	void cv_quality_QualityPSNR_getMaxPixelValue_const(const cv::quality::QualityPSNR* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxPixelValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxPixelValue(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:87
	// ("cv::quality::QualityPSNR::setMaxPixelValue", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_quality_QualityPSNR_setMaxPixelValue_double(cv::quality::QualityPSNR* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxPixelValue(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::quality::QualityPSNR::to_Algorithm() generated
	// ("cv::quality::QualityPSNR::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_quality_QualityPSNR_to_Algorithm(cv::quality::QualityPSNR* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::quality::QualityPSNR::to_QualityBase() generated
	// ("cv::quality::QualityPSNR::to_QualityBase", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityBase* cv_quality_QualityPSNR_to_QualityBase(cv::quality::QualityPSNR* instance) {
			return dynamic_cast<cv::quality::QualityBase*>(instance);
	}

	// cv::quality::QualityPSNR::delete() generated
	// ("cv::quality::QualityPSNR::delete", vec![(pred!(mut, [], []), _)]),
	void cv_quality_QualityPSNR_delete(cv::quality::QualityPSNR* instance) {
			delete instance;
	}

	// compute(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualityssim.hpp:27
	// ("cv::quality::QualitySSIM::compute", vec![(pred!(mut, ["cmp"], ["const cv::_InputArray*"]), _)]),
	void cv_quality_QualitySSIM_compute_const__InputArrayR(cv::quality::QualitySSIM* instance, const cv::_InputArray* cmp, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*cmp);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualityssim.hpp:30
	// ("cv::quality::QualitySSIM::empty", vec![(pred!(const, [], []), _)]),
	void cv_quality_QualitySSIM_empty_const(const cv::quality::QualitySSIM* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualityssim.hpp:33
	// ("cv::quality::QualitySSIM::clear", vec![(pred!(mut, [], []), _)]),
	void cv_quality_QualitySSIM_clear(cv::quality::QualitySSIM* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualityssim.hpp:39
	// ("cv::quality::QualitySSIM::create", vec![(pred!(mut, ["ref"], ["const cv::_InputArray*"]), _)]),
	void cv_quality_QualitySSIM_create_const__InputArrayR(const cv::_InputArray* ref, Result<cv::Ptr<cv::quality::QualitySSIM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualitySSIM> ret = cv::quality::QualitySSIM::create(*ref);
			Ok(new cv::Ptr<cv::quality::QualitySSIM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualityssim.hpp:48
	// ("cv::quality::QualitySSIM::compute", vec![(pred!(mut, ["ref", "cmp", "qualityMap"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_quality_QualitySSIM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::quality::QualitySSIM::compute(*ref, *cmp, *qualityMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::quality::QualitySSIM::to_Algorithm() generated
	// ("cv::quality::QualitySSIM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_quality_QualitySSIM_to_Algorithm(cv::quality::QualitySSIM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::quality::QualitySSIM::to_QualityBase() generated
	// ("cv::quality::QualitySSIM::to_QualityBase", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityBase* cv_quality_QualitySSIM_to_QualityBase(cv::quality::QualitySSIM* instance) {
			return dynamic_cast<cv::quality::QualityBase*>(instance);
	}

	// cv::quality::QualitySSIM::delete() generated
	// ("cv::quality::QualitySSIM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_quality_QualitySSIM_delete(cv::quality::QualitySSIM* instance) {
			delete instance;
	}

}

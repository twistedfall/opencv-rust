#include "ocvrs_common.hpp"
#include <opencv2/img_hash.hpp>
#include "img_hash_types.hpp"

extern "C" {
	// averageHash(cv::InputArray, cv::OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/average_hash.hpp:33
	// ("cv::img_hash::averageHash", vec![(pred!(mut, ["inputArr", "outputArr"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_img_hash_averageHash_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, ResultVoid* ocvrs_return) {
		try {
			cv::img_hash::averageHash(*inputArr, *outputArr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::blockMeanHash(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/block_mean_hash.hpp:44
	// ("cv::img_hash::blockMeanHash", vec![(pred!(mut, ["inputArr", "outputArr"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_img_hash_blockMeanHash_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, ResultVoid* ocvrs_return) {
		try {
			cv::img_hash::blockMeanHash(*inputArr, *outputArr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blockMeanHash(cv::InputArray, cv::OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/block_mean_hash.hpp:44
	// ("cv::img_hash::blockMeanHash", vec![(pred!(mut, ["inputArr", "outputArr", "mode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_img_hash_blockMeanHash_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, int mode, ResultVoid* ocvrs_return) {
		try {
			cv::img_hash::blockMeanHash(*inputArr, *outputArr, mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// colorMomentHash(cv::InputArray, cv::OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/color_moment_hash.hpp:35
	// ("cv::img_hash::colorMomentHash", vec![(pred!(mut, ["inputArr", "outputArr"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_img_hash_colorMomentHash_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, ResultVoid* ocvrs_return) {
		try {
			cv::img_hash::colorMomentHash(*inputArr, *outputArr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::marrHildrethHash(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/marr_hildreth_hash.hpp:56
	// ("cv::img_hash::marrHildrethHash", vec![(pred!(mut, ["inputArr", "outputArr"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_img_hash_marrHildrethHash_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, ResultVoid* ocvrs_return) {
		try {
			cv::img_hash::marrHildrethHash(*inputArr, *outputArr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// marrHildrethHash(cv::InputArray, cv::OutputArray, float, float)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/marr_hildreth_hash.hpp:56
	// ("cv::img_hash::marrHildrethHash", vec![(pred!(mut, ["inputArr", "outputArr", "alpha", "scale"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float"]), _)]),
	void cv_img_hash_marrHildrethHash_const__InputArrayR_const__OutputArrayR_float_float(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, float alpha, float scale, ResultVoid* ocvrs_return) {
		try {
			cv::img_hash::marrHildrethHash(*inputArr, *outputArr, alpha, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pHash(cv::InputArray, cv::OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/phash.hpp:35
	// ("cv::img_hash::pHash", vec![(pred!(mut, ["inputArr", "outputArr"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_img_hash_pHash_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, ResultVoid* ocvrs_return) {
		try {
			cv::img_hash::pHash(*inputArr, *outputArr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::radialVarianceHash(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:48
	// ("cv::img_hash::radialVarianceHash", vec![(pred!(mut, ["inputArr", "outputArr"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_img_hash_radialVarianceHash_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, ResultVoid* ocvrs_return) {
		try {
			cv::img_hash::radialVarianceHash(*inputArr, *outputArr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radialVarianceHash(cv::InputArray, cv::OutputArray, double, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:48
	// ("cv::img_hash::radialVarianceHash", vec![(pred!(mut, ["inputArr", "outputArr", "sigma", "numOfAngleLine"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "int"]), _)]),
	void cv_img_hash_radialVarianceHash_const__InputArrayR_const__OutputArrayR_double_int(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, double sigma, int numOfAngleLine, ResultVoid* ocvrs_return) {
		try {
			cv::img_hash::radialVarianceHash(*inputArr, *outputArr, sigma, numOfAngleLine);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/average_hash.hpp:24
	// ("cv::img_hash::AverageHash::create", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_AverageHash_create(Result<cv::Ptr<cv::img_hash::AverageHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::AverageHash> ret = cv::img_hash::AverageHash::create();
			Ok(new cv::Ptr<cv::img_hash::AverageHash>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::AverageHash::to_Algorithm() generated
	// ("cv::img_hash::AverageHash::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_img_hash_AverageHash_to_Algorithm(cv::img_hash::AverageHash* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::img_hash::AverageHash::to_ImgHashBase() generated
	// ("cv::img_hash::AverageHash::to_ImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::ImgHashBase* cv_img_hash_AverageHash_to_ImgHashBase(cv::img_hash::AverageHash* instance) {
			return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}

	// cv::img_hash::AverageHash::delete() generated
	// ("cv::img_hash::AverageHash::delete", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_AverageHash_delete(cv::img_hash::AverageHash* instance) {
			delete instance;
	}

	// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/block_mean_hash.hpp:32
	// ("cv::img_hash::BlockMeanHash::setMode", vec![(pred!(mut, ["mode"], ["int"]), _)]),
	void cv_img_hash_BlockMeanHash_setMode_int(cv::img_hash::BlockMeanHash* instance, int mode, ResultVoid* ocvrs_return) {
		try {
			instance->setMode(mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMean()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/block_mean_hash.hpp:33
	// ("cv::img_hash::BlockMeanHash::getMean", vec![(pred!(const, [], []), _)]),
	void cv_img_hash_BlockMeanHash_getMean_const(const cv::img_hash::BlockMeanHash* instance, Result<std::vector<double>*>* ocvrs_return) {
		try {
			std::vector<double> ret = instance->getMean();
			Ok(new std::vector<double>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/block_mean_hash.hpp:34
	// ("cv::img_hash::BlockMeanHash::create", vec![(pred!(mut, ["mode"], ["int"]), _)]),
	void cv_img_hash_BlockMeanHash_create_int(int mode, Result<cv::Ptr<cv::img_hash::BlockMeanHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::BlockMeanHash> ret = cv::img_hash::BlockMeanHash::create(mode);
			Ok(new cv::Ptr<cv::img_hash::BlockMeanHash>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::BlockMeanHash::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/block_mean_hash.hpp:34
	// ("cv::img_hash::BlockMeanHash::create", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_BlockMeanHash_create(Result<cv::Ptr<cv::img_hash::BlockMeanHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::BlockMeanHash> ret = cv::img_hash::BlockMeanHash::create();
			Ok(new cv::Ptr<cv::img_hash::BlockMeanHash>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::BlockMeanHash::to_Algorithm() generated
	// ("cv::img_hash::BlockMeanHash::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_img_hash_BlockMeanHash_to_Algorithm(cv::img_hash::BlockMeanHash* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::img_hash::BlockMeanHash::to_ImgHashBase() generated
	// ("cv::img_hash::BlockMeanHash::to_ImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::ImgHashBase* cv_img_hash_BlockMeanHash_to_ImgHashBase(cv::img_hash::BlockMeanHash* instance) {
			return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}

	// cv::img_hash::BlockMeanHash::delete() generated
	// ("cv::img_hash::BlockMeanHash::delete", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_BlockMeanHash_delete(cv::img_hash::BlockMeanHash* instance) {
			delete instance;
	}

	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/color_moment_hash.hpp:23
	// ("cv::img_hash::ColorMomentHash::create", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_ColorMomentHash_create(Result<cv::Ptr<cv::img_hash::ColorMomentHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::ColorMomentHash> ret = cv::img_hash::ColorMomentHash::create();
			Ok(new cv::Ptr<cv::img_hash::ColorMomentHash>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::ColorMomentHash::to_Algorithm() generated
	// ("cv::img_hash::ColorMomentHash::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_img_hash_ColorMomentHash_to_Algorithm(cv::img_hash::ColorMomentHash* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::img_hash::ColorMomentHash::to_ImgHashBase() generated
	// ("cv::img_hash::ColorMomentHash::to_ImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::ImgHashBase* cv_img_hash_ColorMomentHash_to_ImgHashBase(cv::img_hash::ColorMomentHash* instance) {
			return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}

	// cv::img_hash::ColorMomentHash::delete() generated
	// ("cv::img_hash::ColorMomentHash::delete", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_ColorMomentHash_delete(cv::img_hash::ColorMomentHash* instance) {
			delete instance;
	}

	// compute(cv::InputArray, cv::OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/img_hash_base.hpp:28
	// ("cv::img_hash::ImgHashBase::compute", vec![(pred!(mut, ["inputArr", "outputArr"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_img_hash_ImgHashBase_compute_const__InputArrayR_const__OutputArrayR(cv::img_hash::ImgHashBase* instance, const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*inputArr, *outputArr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compare(cv::InputArray, cv::InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/img_hash_base.hpp:35
	// ("cv::img_hash::ImgHashBase::compare", vec![(pred!(const, ["hashOne", "hashTwo"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_img_hash_ImgHashBase_compare_const_const__InputArrayR_const__InputArrayR(const cv::img_hash::ImgHashBase* instance, const cv::_InputArray* hashOne, const cv::_InputArray* hashTwo, Result<double>* ocvrs_return) {
		try {
			double ret = instance->compare(*hashOne, *hashTwo);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::ImgHashBase::to_Algorithm() generated
	// ("cv::img_hash::ImgHashBase::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_img_hash_ImgHashBase_to_Algorithm(cv::img_hash::ImgHashBase* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::img_hash::ImgHashBase::delete() generated
	// ("cv::img_hash::ImgHashBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_ImgHashBase_delete(cv::img_hash::ImgHashBase* instance) {
			delete instance;
	}

	// getAlpha()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/marr_hildreth_hash.hpp:26
	// ("cv::img_hash::MarrHildrethHash::getAlpha", vec![(pred!(const, [], []), _)]),
	void cv_img_hash_MarrHildrethHash_getAlpha_const(const cv::img_hash::MarrHildrethHash* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/marr_hildreth_hash.hpp:31
	// ("cv::img_hash::MarrHildrethHash::getScale", vec![(pred!(const, [], []), _)]),
	void cv_img_hash_MarrHildrethHash_getScale_const(const cv::img_hash::MarrHildrethHash* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setKernelParam(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/marr_hildreth_hash.hpp:37
	// ("cv::img_hash::MarrHildrethHash::setKernelParam", vec![(pred!(mut, ["alpha", "scale"], ["float", "float"]), _)]),
	void cv_img_hash_MarrHildrethHash_setKernelParam_float_float(cv::img_hash::MarrHildrethHash* instance, float alpha, float scale, ResultVoid* ocvrs_return) {
		try {
			instance->setKernelParam(alpha, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/marr_hildreth_hash.hpp:43
	// ("cv::img_hash::MarrHildrethHash::create", vec![(pred!(mut, ["alpha", "scale"], ["float", "float"]), _)]),
	void cv_img_hash_MarrHildrethHash_create_float_float(float alpha, float scale, Result<cv::Ptr<cv::img_hash::MarrHildrethHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::MarrHildrethHash> ret = cv::img_hash::MarrHildrethHash::create(alpha, scale);
			Ok(new cv::Ptr<cv::img_hash::MarrHildrethHash>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::MarrHildrethHash::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/marr_hildreth_hash.hpp:43
	// ("cv::img_hash::MarrHildrethHash::create", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_MarrHildrethHash_create(Result<cv::Ptr<cv::img_hash::MarrHildrethHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::MarrHildrethHash> ret = cv::img_hash::MarrHildrethHash::create();
			Ok(new cv::Ptr<cv::img_hash::MarrHildrethHash>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::MarrHildrethHash::to_Algorithm() generated
	// ("cv::img_hash::MarrHildrethHash::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_img_hash_MarrHildrethHash_to_Algorithm(cv::img_hash::MarrHildrethHash* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::img_hash::MarrHildrethHash::to_ImgHashBase() generated
	// ("cv::img_hash::MarrHildrethHash::to_ImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::ImgHashBase* cv_img_hash_MarrHildrethHash_to_ImgHashBase(cv::img_hash::MarrHildrethHash* instance) {
			return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}

	// cv::img_hash::MarrHildrethHash::delete() generated
	// ("cv::img_hash::MarrHildrethHash::delete", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_MarrHildrethHash_delete(cv::img_hash::MarrHildrethHash* instance) {
			delete instance;
	}

	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/phash.hpp:25
	// ("cv::img_hash::PHash::create", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_PHash_create(Result<cv::Ptr<cv::img_hash::PHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::PHash> ret = cv::img_hash::PHash::create();
			Ok(new cv::Ptr<cv::img_hash::PHash>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::PHash::to_Algorithm() generated
	// ("cv::img_hash::PHash::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_img_hash_PHash_to_Algorithm(cv::img_hash::PHash* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::img_hash::PHash::to_ImgHashBase() generated
	// ("cv::img_hash::PHash::to_ImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::ImgHashBase* cv_img_hash_PHash_to_ImgHashBase(cv::img_hash::PHash* instance) {
			return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}

	// cv::img_hash::PHash::delete() generated
	// ("cv::img_hash::PHash::delete", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_PHash_delete(cv::img_hash::PHash* instance) {
			delete instance;
	}

	// create(double, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:24
	// ("cv::img_hash::RadialVarianceHash::create", vec![(pred!(mut, ["sigma", "numOfAngleLine"], ["double", "int"]), _)]),
	void cv_img_hash_RadialVarianceHash_create_double_int(double sigma, int numOfAngleLine, Result<cv::Ptr<cv::img_hash::RadialVarianceHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::RadialVarianceHash> ret = cv::img_hash::RadialVarianceHash::create(sigma, numOfAngleLine);
			Ok(new cv::Ptr<cv::img_hash::RadialVarianceHash>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::RadialVarianceHash::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:24
	// ("cv::img_hash::RadialVarianceHash::create", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_RadialVarianceHash_create(Result<cv::Ptr<cv::img_hash::RadialVarianceHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::RadialVarianceHash> ret = cv::img_hash::RadialVarianceHash::create();
			Ok(new cv::Ptr<cv::img_hash::RadialVarianceHash>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumOfAngleLine()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:26
	// ("cv::img_hash::RadialVarianceHash::getNumOfAngleLine", vec![(pred!(const, [], []), _)]),
	void cv_img_hash_RadialVarianceHash_getNumOfAngleLine_const(const cv::img_hash::RadialVarianceHash* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumOfAngleLine();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:27
	// ("cv::img_hash::RadialVarianceHash::getSigma", vec![(pred!(const, [], []), _)]),
	void cv_img_hash_RadialVarianceHash_getSigma_const(const cv::img_hash::RadialVarianceHash* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumOfAngleLine(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:29
	// ("cv::img_hash::RadialVarianceHash::setNumOfAngleLine", vec![(pred!(mut, ["value"], ["int"]), _)]),
	void cv_img_hash_RadialVarianceHash_setNumOfAngleLine_int(cv::img_hash::RadialVarianceHash* instance, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setNumOfAngleLine(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:30
	// ("cv::img_hash::RadialVarianceHash::setSigma", vec![(pred!(mut, ["value"], ["double"]), _)]),
	void cv_img_hash_RadialVarianceHash_setSigma_double(cv::img_hash::RadialVarianceHash* instance, double value, ResultVoid* ocvrs_return) {
		try {
			instance->setSigma(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFeatures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:33
	// ("cv::img_hash::RadialVarianceHash::getFeatures", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_RadialVarianceHash_getFeatures(cv::img_hash::RadialVarianceHash* instance, Result<std::vector<double>*>* ocvrs_return) {
		try {
			std::vector<double> ret = instance->getFeatures();
			Ok(new std::vector<double>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHash()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:34
	// ("cv::img_hash::RadialVarianceHash::getHash", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_RadialVarianceHash_getHash(cv::img_hash::RadialVarianceHash* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getHash();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPixPerLine(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:35
	// ("cv::img_hash::RadialVarianceHash::getPixPerLine", vec![(pred!(mut, ["input"], ["const cv::Mat*"]), _)]),
	void cv_img_hash_RadialVarianceHash_getPixPerLine_const_MatR(cv::img_hash::RadialVarianceHash* instance, const cv::Mat* input, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getPixPerLine(*input);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getProjection()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/img_hash/radial_variance_hash.hpp:36
	// ("cv::img_hash::RadialVarianceHash::getProjection", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_RadialVarianceHash_getProjection(cv::img_hash::RadialVarianceHash* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getProjection();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::img_hash::RadialVarianceHash::to_Algorithm() generated
	// ("cv::img_hash::RadialVarianceHash::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_img_hash_RadialVarianceHash_to_Algorithm(cv::img_hash::RadialVarianceHash* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::img_hash::RadialVarianceHash::to_ImgHashBase() generated
	// ("cv::img_hash::RadialVarianceHash::to_ImgHashBase", vec![(pred!(mut, [], []), _)]),
	cv::img_hash::ImgHashBase* cv_img_hash_RadialVarianceHash_to_ImgHashBase(cv::img_hash::RadialVarianceHash* instance) {
			return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}

	// cv::img_hash::RadialVarianceHash::delete() generated
	// ("cv::img_hash::RadialVarianceHash::delete", vec![(pred!(mut, [], []), _)]),
	void cv_img_hash_RadialVarianceHash_delete(cv::img_hash::RadialVarianceHash* instance) {
			delete instance;
	}

}

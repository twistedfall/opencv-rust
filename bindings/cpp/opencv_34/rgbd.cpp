#include "common.hpp"
#include <opencv2/rgbd.hpp>
#include "rgbd_types.hpp"

extern "C" {
	Result_void cv_linemod_colormap_const_MatR_MatR(const cv::Mat* quantized, cv::Mat* dst) {
		try {
			cv::linemod::colormap(*quantized, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::linemod::Detector>*> cv_linemod_getDefaultLINE() {
		try {
			cv::Ptr<cv::linemod::Detector> ret = cv::linemod::getDefaultLINE();
			return Ok(new cv::Ptr<cv::linemod::Detector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::Detector>*>))
	}
	
	Result<cv::Ptr<cv::linemod::Detector>*> cv_linemod_getDefaultLINEMOD() {
		try {
			cv::Ptr<cv::linemod::Detector> ret = cv::linemod::getDefaultLINEMOD();
			return Ok(new cv::Ptr<cv::linemod::Detector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::Detector>*>))
	}
	
	Result_void cv_rgbd_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* depth, const cv::_InputArray* in_K, const cv::_InputArray* in_points, const cv::_OutputArray* points3d) {
		try {
			cv::rgbd::depthTo3dSparse(*depth, *in_K, *in_points, *points3d);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* depth, const cv::_InputArray* K, const cv::_OutputArray* points3d, const cv::_InputArray* mask) {
		try {
			cv::rgbd::depthTo3d(*depth, *K, *points3d, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_rgbd_isValidDepth_const_doubleR(const double* depth) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_rgbd_isValidDepth_const_floatR(const float* depth) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_rgbd_isValidDepth_const_intR(const int* depth) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_rgbd_isValidDepth_const_shortR(const short* depth) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_rgbd_isValidDepth_const_unsigned_intR(const unsigned int* depth) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_rgbd_isValidDepth_const_unsigned_shortR(const unsigned short* depth) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(const cv::_InputArray* unregisteredCameraMatrix, const cv::_InputArray* registeredCameraMatrix, const cv::_InputArray* registeredDistCoeffs, const cv::_InputArray* Rt, const cv::_InputArray* unregisteredDepth, const cv::Size* outputImagePlaneSize, const cv::_OutputArray* registeredDepth, bool depthDilation) {
		try {
			cv::rgbd::registerDepth(*unregisteredCameraMatrix, *registeredCameraMatrix, *registeredDistCoeffs, *Rt, *unregisteredDepth, *outputImagePlaneSize, *registeredDepth, depthDilation);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR(const cv::_InputArray* in, int depth, const cv::_OutputArray* out) {
		try {
			cv::rgbd::rescaleDepth(*in, depth, *out);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* Rt, const cv::Mat* cameraMatrix, const cv::Mat* distCoeff, const cv::_OutputArray* warpedImage, const cv::_OutputArray* warpedDepth, const cv::_OutputArray* warpedMask) {
		try {
			cv::rgbd::warpFrame(*image, *depth, *mask, *Rt, *cameraMatrix, *distCoeff, *warpedImage, *warpedDepth, *warpedMask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_linemod_ColorGradient_getPropWeak_threshold_const(const cv::linemod::ColorGradient* instance) {
		try {
			float ret = instance->weak_threshold;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_linemod_ColorGradient_setPropWeak_threshold_float(cv::linemod::ColorGradient* instance, float val) {
		try {
			instance->weak_threshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_linemod_ColorGradient_getPropNum_features_const(const cv::linemod::ColorGradient* instance) {
		try {
			size_t ret = instance->num_features;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_linemod_ColorGradient_setPropNum_features_size_t(cv::linemod::ColorGradient* instance, size_t val) {
		try {
			instance->num_features = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_linemod_ColorGradient_getPropStrong_threshold_const(const cv::linemod::ColorGradient* instance) {
		try {
			float ret = instance->strong_threshold;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_linemod_ColorGradient_setPropStrong_threshold_float(cv::linemod::ColorGradient* instance, float val) {
		try {
			instance->strong_threshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Linemod_ColorGradient_delete(cv::linemod::ColorGradient* instance) {
		delete instance;
	}
	Result<cv::linemod::ColorGradient*> cv_linemod_ColorGradient_ColorGradient() {
		try {
			cv::linemod::ColorGradient* ret = new cv::linemod::ColorGradient();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::ColorGradient*>))
	}
	
	Result<cv::linemod::ColorGradient*> cv_linemod_ColorGradient_ColorGradient_float_size_t_float(float weak_threshold, size_t num_features, float strong_threshold) {
		try {
			cv::linemod::ColorGradient* ret = new cv::linemod::ColorGradient(weak_threshold, num_features, strong_threshold);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::ColorGradient*>))
	}
	
	Result<cv::Ptr<cv::linemod::ColorGradient>*> cv_linemod_ColorGradient_create_float_size_t_float(float weak_threshold, size_t num_features, float strong_threshold) {
		try {
			cv::Ptr<cv::linemod::ColorGradient> ret = cv::linemod::ColorGradient::create(weak_threshold, num_features, strong_threshold);
			return Ok(new cv::Ptr<cv::linemod::ColorGradient>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::ColorGradient>*>))
	}
	
	Result<void*> cv_linemod_ColorGradient_name_const(const cv::linemod::ColorGradient* instance) {
		try {
			cv::String ret = instance->name();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_linemod_ColorGradient_read_const_FileNodeR(cv::linemod::ColorGradient* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_linemod_ColorGradient_write_const_FileStorageR(const cv::linemod::ColorGradient* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_linemod_DepthNormal_getPropDistance_threshold_const(const cv::linemod::DepthNormal* instance) {
		try {
			int ret = instance->distance_threshold;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_linemod_DepthNormal_setPropDistance_threshold_int(cv::linemod::DepthNormal* instance, int val) {
		try {
			instance->distance_threshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_linemod_DepthNormal_getPropDifference_threshold_const(const cv::linemod::DepthNormal* instance) {
		try {
			int ret = instance->difference_threshold;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_linemod_DepthNormal_setPropDifference_threshold_int(cv::linemod::DepthNormal* instance, int val) {
		try {
			instance->difference_threshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_linemod_DepthNormal_getPropNum_features_const(const cv::linemod::DepthNormal* instance) {
		try {
			size_t ret = instance->num_features;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_linemod_DepthNormal_setPropNum_features_size_t(cv::linemod::DepthNormal* instance, size_t val) {
		try {
			instance->num_features = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_linemod_DepthNormal_getPropExtract_threshold_const(const cv::linemod::DepthNormal* instance) {
		try {
			int ret = instance->extract_threshold;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_linemod_DepthNormal_setPropExtract_threshold_int(cv::linemod::DepthNormal* instance, int val) {
		try {
			instance->extract_threshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Linemod_DepthNormal_delete(cv::linemod::DepthNormal* instance) {
		delete instance;
	}
	Result<cv::linemod::DepthNormal*> cv_linemod_DepthNormal_DepthNormal() {
		try {
			cv::linemod::DepthNormal* ret = new cv::linemod::DepthNormal();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::DepthNormal*>))
	}
	
	Result<cv::linemod::DepthNormal*> cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(int distance_threshold, int difference_threshold, size_t num_features, int extract_threshold) {
		try {
			cv::linemod::DepthNormal* ret = new cv::linemod::DepthNormal(distance_threshold, difference_threshold, num_features, extract_threshold);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::DepthNormal*>))
	}
	
	Result<cv::Ptr<cv::linemod::DepthNormal>*> cv_linemod_DepthNormal_create_int_int_size_t_int(int distance_threshold, int difference_threshold, size_t num_features, int extract_threshold) {
		try {
			cv::Ptr<cv::linemod::DepthNormal> ret = cv::linemod::DepthNormal::create(distance_threshold, difference_threshold, num_features, extract_threshold);
			return Ok(new cv::Ptr<cv::linemod::DepthNormal>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::DepthNormal>*>))
	}
	
	Result<void*> cv_linemod_DepthNormal_name_const(const cv::linemod::DepthNormal* instance) {
		try {
			cv::String ret = instance->name();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_linemod_DepthNormal_read_const_FileNodeR(cv::linemod::DepthNormal* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_linemod_DepthNormal_write_const_FileStorageR(const cv::linemod::DepthNormal* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Linemod_Detector_delete(cv::linemod::Detector* instance) {
		delete instance;
	}
	Result<cv::linemod::Detector*> cv_linemod_Detector_Detector() {
		try {
			cv::linemod::Detector* ret = new cv::linemod::Detector();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Detector*>))
	}
	
	Result<cv::linemod::Detector*> cv_linemod_Detector_Detector_const_vector_Ptr_Modality__R_const_vector_int_R(const std::vector<cv::Ptr<cv::linemod::Modality>>* modalities, const std::vector<int>* T_pyramid) {
		try {
			cv::linemod::Detector* ret = new cv::linemod::Detector(*modalities, *T_pyramid);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Detector*>))
	}
	
	Result_void cv_linemod_Detector_match_const_const_vector_Mat_R_float_vector_Match_R_const_vector_String_R_const__OutputArrayR_const_vector_Mat_R(const cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, float threshold, std::vector<cv::linemod::Match>* matches, const std::vector<cv::String>* class_ids, const cv::_OutputArray* quantized_images, const std::vector<cv::Mat>* masks) {
		try {
			instance->match(*sources, threshold, *matches, *class_ids, *quantized_images, *masks);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_linemod_Detector_addTemplate_const_vector_Mat_R_const_StringR_const_MatR_RectX(cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, const char* class_id, const cv::Mat* object_mask, cv::Rect* bounding_box) {
		try {
			int ret = instance->addTemplate(*sources, cv::String(class_id), *object_mask, bounding_box);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_linemod_Detector_addSyntheticTemplate_const_vector_Template_R_const_StringR(cv::linemod::Detector* instance, const std::vector<cv::linemod::Template>* templates, const char* class_id) {
		try {
			int ret = instance->addSyntheticTemplate(*templates, cv::String(class_id));
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<std::vector<cv::Ptr<cv::linemod::Modality>>*> cv_linemod_Detector_getModalities_const(const cv::linemod::Detector* instance) {
		try {
			std::vector<cv::Ptr<cv::linemod::Modality>> ret = instance->getModalities();
			return Ok(new std::vector<cv::Ptr<cv::linemod::Modality>>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Ptr<cv::linemod::Modality>>*>))
	}
	
	Result<int> cv_linemod_Detector_getT_const_int(const cv::linemod::Detector* instance, int pyramid_level) {
		try {
			int ret = instance->getT(pyramid_level);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_linemod_Detector_pyramidLevels_const(const cv::linemod::Detector* instance) {
		try {
			int ret = instance->pyramidLevels();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<std::vector<cv::linemod::Template>*> cv_linemod_Detector_getTemplates_const_const_StringR_int(const cv::linemod::Detector* instance, const char* class_id, int template_id) {
		try {
			std::vector<cv::linemod::Template> ret = instance->getTemplates(cv::String(class_id), template_id);
			return Ok(new std::vector<cv::linemod::Template>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::linemod::Template>*>))
	}
	
	Result<int> cv_linemod_Detector_numTemplates_const(const cv::linemod::Detector* instance) {
		try {
			int ret = instance->numTemplates();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_linemod_Detector_numTemplates_const_const_StringR(const cv::linemod::Detector* instance, const char* class_id) {
		try {
			int ret = instance->numTemplates(cv::String(class_id));
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_linemod_Detector_numClasses_const(const cv::linemod::Detector* instance) {
		try {
			int ret = instance->numClasses();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<std::vector<cv::String>*> cv_linemod_Detector_classIds_const(const cv::linemod::Detector* instance) {
		try {
			std::vector<cv::String> ret = instance->classIds();
			return Ok(new std::vector<cv::String>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::String>*>))
	}
	
	Result_void cv_linemod_Detector_read_const_FileNodeR(cv::linemod::Detector* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_linemod_Detector_write_const_FileStorageR(const cv::linemod::Detector* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(cv::linemod::Detector* instance, const cv::FileNode* fn, const char* class_id_override) {
		try {
			cv::String ret = instance->readClass(*fn, cv::String(class_id_override));
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(const cv::linemod::Detector* instance, const char* class_id, cv::FileStorage* fs) {
		try {
			instance->writeClass(cv::String(class_id), *fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_linemod_Detector_readClasses_const_vector_String_R_const_StringR(cv::linemod::Detector* instance, const std::vector<cv::String>* class_ids, const char* format) {
		try {
			instance->readClasses(*class_ids, cv::String(format));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_linemod_Detector_writeClasses_const_const_StringR(const cv::linemod::Detector* instance, const char* format) {
		try {
			instance->writeClasses(cv::String(format));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::linemod::Feature> cv_linemod_Feature_Feature() {
		try {
			cv::linemod::Feature ret;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Feature>))
	}
	
	Result<cv::linemod::Feature> cv_linemod_Feature_Feature_int_int_int(int x, int y, int label) {
		try {
			cv::linemod::Feature ret(x, y, label);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Feature>))
	}
	
	Result_void cv_linemod_Feature_read_const_FileNodeR(cv::linemod::Feature instance, const cv::FileNode* fn) {
		try {
			instance.read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_linemod_Feature_write_const_FileStorageR(const cv::linemod::Feature instance, cv::FileStorage* fs) {
		try {
			instance.write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_linemod_Match_getPropX_const(const cv::linemod::Match* instance) {
		try {
			int ret = instance->x;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_linemod_Match_setPropX_int(cv::linemod::Match* instance, int val) {
		try {
			instance->x = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_linemod_Match_getPropY_const(const cv::linemod::Match* instance) {
		try {
			int ret = instance->y;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_linemod_Match_setPropY_int(cv::linemod::Match* instance, int val) {
		try {
			instance->y = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_linemod_Match_getPropSimilarity_const(const cv::linemod::Match* instance) {
		try {
			float ret = instance->similarity;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_linemod_Match_setPropSimilarity_float(cv::linemod::Match* instance, float val) {
		try {
			instance->similarity = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_linemod_Match_getPropClass_id_const(const cv::linemod::Match* instance) {
		try {
			cv::String ret = instance->class_id;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_linemod_Match_setPropClass_id_String(cv::linemod::Match* instance, char* val) {
		try {
			instance->class_id = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_linemod_Match_getPropTemplate_id_const(const cv::linemod::Match* instance) {
		try {
			int ret = instance->template_id;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_linemod_Match_setPropTemplate_id_int(cv::linemod::Match* instance, int val) {
		try {
			instance->template_id = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Linemod_Match_delete(cv::linemod::Match* instance) {
		delete instance;
	}
	Result<cv::linemod::Match*> cv_linemod_Match_Match() {
		try {
			cv::linemod::Match* ret = new cv::linemod::Match();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Match*>))
	}
	
	Result<cv::linemod::Match*> cv_linemod_Match_Match_int_int_float_const_StringR_int(int x, int y, float similarity, const char* class_id, int template_id) {
		try {
			cv::linemod::Match* ret = new cv::linemod::Match(x, y, similarity, cv::String(class_id), template_id);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Match*>))
	}
	
	Result<cv::Ptr<cv::linemod::QuantizedPyramid>*> cv_linemod_Modality_process_const_const_MatR_const_MatR(const cv::linemod::Modality* instance, const cv::Mat* src, const cv::Mat* mask) {
		try {
			cv::Ptr<cv::linemod::QuantizedPyramid> ret = instance->process(*src, *mask);
			return Ok(new cv::Ptr<cv::linemod::QuantizedPyramid>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::QuantizedPyramid>*>))
	}
	
	Result<void*> cv_linemod_Modality_name_const(const cv::linemod::Modality* instance) {
		try {
			cv::String ret = instance->name();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_linemod_Modality_read_const_FileNodeR(cv::linemod::Modality* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_linemod_Modality_write_const_FileStorageR(const cv::linemod::Modality* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::linemod::Modality>*> cv_linemod_Modality_create_const_StringR(const char* modality_type) {
		try {
			cv::Ptr<cv::linemod::Modality> ret = cv::linemod::Modality::create(cv::String(modality_type));
			return Ok(new cv::Ptr<cv::linemod::Modality>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::Modality>*>))
	}
	
	Result<cv::Ptr<cv::linemod::Modality>*> cv_linemod_Modality_create_const_FileNodeR(const cv::FileNode* fn) {
		try {
			cv::Ptr<cv::linemod::Modality> ret = cv::linemod::Modality::create(*fn);
			return Ok(new cv::Ptr<cv::linemod::Modality>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::Modality>*>))
	}
	
	Result_void cv_linemod_QuantizedPyramid_quantize_const_MatR(const cv::linemod::QuantizedPyramid* instance, cv::Mat* dst) {
		try {
			instance->quantize(*dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(const cv::linemod::QuantizedPyramid* instance, cv::linemod::Template* templ) {
		try {
			bool ret = instance->extractTemplate(*templ);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_linemod_QuantizedPyramid_pyrDown(cv::linemod::QuantizedPyramid* instance) {
		try {
			instance->pyrDown();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_linemod_Template_getPropWidth_const(const cv::linemod::Template* instance) {
		try {
			int ret = instance->width;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_linemod_Template_setPropWidth_int(cv::linemod::Template* instance, int val) {
		try {
			instance->width = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_linemod_Template_getPropHeight_const(const cv::linemod::Template* instance) {
		try {
			int ret = instance->height;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_linemod_Template_setPropHeight_int(cv::linemod::Template* instance, int val) {
		try {
			instance->height = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_linemod_Template_getPropPyramid_level_const(const cv::linemod::Template* instance) {
		try {
			int ret = instance->pyramid_level;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_linemod_Template_setPropPyramid_level_int(cv::linemod::Template* instance, int val) {
		try {
			instance->pyramid_level = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::linemod::Feature>*> cv_linemod_Template_getPropFeatures(cv::linemod::Template* instance) {
		try {
			std::vector<cv::linemod::Feature> ret = instance->features;
			return Ok(new std::vector<cv::linemod::Feature>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::linemod::Feature>*>))
	}
	
	Result_void cv_linemod_Template_setPropFeatures_vector_Feature_(cv::linemod::Template* instance, std::vector<cv::linemod::Feature>* val) {
		try {
			instance->features = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Linemod_Template_delete(cv::linemod::Template* instance) {
		delete instance;
	}
	Result_void cv_linemod_Template_read_const_FileNodeR(cv::linemod::Template* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_linemod_Template_write_const_FileStorageR(const cv::linemod::Template* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_DepthCleaner_delete(cv::rgbd::DepthCleaner* instance) {
		delete instance;
	}
	Result<cv::rgbd::DepthCleaner*> cv_rgbd_DepthCleaner_DepthCleaner() {
		try {
			cv::rgbd::DepthCleaner* ret = new cv::rgbd::DepthCleaner();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::DepthCleaner*>))
	}
	
	Result<cv::rgbd::DepthCleaner*> cv_rgbd_DepthCleaner_DepthCleaner_int_int_int(int depth, int window_size, int method) {
		try {
			cv::rgbd::DepthCleaner* ret = new cv::rgbd::DepthCleaner(depth, window_size, method);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::DepthCleaner*>))
	}
	
	Result<cv::Ptr<cv::rgbd::DepthCleaner>*> cv_rgbd_DepthCleaner_create_int_int_int(int depth, int window_size, int method) {
		try {
			cv::Ptr<cv::rgbd::DepthCleaner> ret = cv::rgbd::DepthCleaner::create(depth, window_size, method);
			return Ok(new cv::Ptr<cv::rgbd::DepthCleaner>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::DepthCleaner>*>))
	}
	
	Result_void cv_rgbd_DepthCleaner_initialize_const(const cv::rgbd::DepthCleaner* instance) {
		try {
			instance->initialize();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_DepthCleaner_getWindowSize_const(const cv::rgbd::DepthCleaner* instance) {
		try {
			int ret = instance->getWindowSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_DepthCleaner_setWindowSize_int(cv::rgbd::DepthCleaner* instance, int val) {
		try {
			instance->setWindowSize(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_DepthCleaner_getDepth_const(const cv::rgbd::DepthCleaner* instance) {
		try {
			int ret = instance->getDepth();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_DepthCleaner_setDepth_int(cv::rgbd::DepthCleaner* instance, int val) {
		try {
			instance->setDepth(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_DepthCleaner_getMethod_const(const cv::rgbd::DepthCleaner* instance) {
		try {
			int ret = instance->getMethod();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_DepthCleaner_setMethod_int(cv::rgbd::DepthCleaner* instance, int val) {
		try {
			instance->setMethod(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_ICPOdometry_delete(cv::rgbd::ICPOdometry* instance) {
		delete instance;
	}
	Result<cv::rgbd::ICPOdometry*> cv_rgbd_ICPOdometry_ICPOdometry() {
		try {
			cv::rgbd::ICPOdometry* ret = new cv::rgbd::ICPOdometry();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::ICPOdometry*>))
	}
	
	Result<cv::rgbd::ICPOdometry*> cv_rgbd_ICPOdometry_ICPOdometry_const_MatR_float_float_float_float_const_vector_int_R_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, int transformType) {
		try {
			cv::rgbd::ICPOdometry* ret = new cv::rgbd::ICPOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, transformType);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::ICPOdometry*>))
	}
	
	Result<cv::Ptr<cv::rgbd::ICPOdometry>*> cv_rgbd_ICPOdometry_create_const_MatR_float_float_float_float_const_vector_int_R_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, int transformType) {
		try {
			cv::Ptr<cv::rgbd::ICPOdometry> ret = cv::rgbd::ICPOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, transformType);
			return Ok(new cv::Ptr<cv::rgbd::ICPOdometry>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::ICPOdometry>*>))
	}
	
	Result<cv::Size> cv_rgbd_ICPOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(const cv::rgbd::ICPOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<cv::Mat*> cv_rgbd_ICPOdometry_getCameraMatrix_const(const cv::rgbd::ICPOdometry* instance) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_ICPOdometry_setCameraMatrix_const_MatR(cv::rgbd::ICPOdometry* instance, const cv::Mat* val) {
		try {
			instance->setCameraMatrix(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_ICPOdometry_getMinDepth_const(const cv::rgbd::ICPOdometry* instance) {
		try {
			double ret = instance->getMinDepth();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_ICPOdometry_setMinDepth_double(cv::rgbd::ICPOdometry* instance, double val) {
		try {
			instance->setMinDepth(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_ICPOdometry_getMaxDepth_const(const cv::rgbd::ICPOdometry* instance) {
		try {
			double ret = instance->getMaxDepth();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_ICPOdometry_setMaxDepth_double(cv::rgbd::ICPOdometry* instance, double val) {
		try {
			instance->setMaxDepth(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_ICPOdometry_getMaxDepthDiff_const(const cv::rgbd::ICPOdometry* instance) {
		try {
			double ret = instance->getMaxDepthDiff();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_ICPOdometry_setMaxDepthDiff_double(cv::rgbd::ICPOdometry* instance, double val) {
		try {
			instance->setMaxDepthDiff(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_rgbd_ICPOdometry_getIterationCounts_const(const cv::rgbd::ICPOdometry* instance) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_ICPOdometry_setIterationCounts_const_MatR(cv::rgbd::ICPOdometry* instance, const cv::Mat* val) {
		try {
			instance->setIterationCounts(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_ICPOdometry_getMaxPointsPart_const(const cv::rgbd::ICPOdometry* instance) {
		try {
			double ret = instance->getMaxPointsPart();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_ICPOdometry_setMaxPointsPart_double(cv::rgbd::ICPOdometry* instance, double val) {
		try {
			instance->setMaxPointsPart(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_ICPOdometry_getTransformType_const(const cv::rgbd::ICPOdometry* instance) {
		try {
			int ret = instance->getTransformType();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_ICPOdometry_setTransformType_int(cv::rgbd::ICPOdometry* instance, int val) {
		try {
			instance->setTransformType(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_ICPOdometry_getMaxTranslation_const(const cv::rgbd::ICPOdometry* instance) {
		try {
			double ret = instance->getMaxTranslation();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_ICPOdometry_setMaxTranslation_double(cv::rgbd::ICPOdometry* instance, double val) {
		try {
			instance->setMaxTranslation(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_ICPOdometry_getMaxRotation_const(const cv::rgbd::ICPOdometry* instance) {
		try {
			double ret = instance->getMaxRotation();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_ICPOdometry_setMaxRotation_double(cv::rgbd::ICPOdometry* instance, double val) {
		try {
			instance->setMaxRotation(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::rgbd::RgbdNormals>*> cv_rgbd_ICPOdometry_getNormalsComputer_const(const cv::rgbd::ICPOdometry* instance) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = instance->getNormalsComputer();
			return Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdNormals>*>))
	}
	
	Result<float> cv_rgbd_Odometry_DEFAULT_MIN_DEPTH() {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MIN_DEPTH();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<float> cv_rgbd_Odometry_DEFAULT_MAX_DEPTH() {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_DEPTH();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<float> cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF() {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_DEPTH_DIFF();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<float> cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART() {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_POINTS_PART();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<float> cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION() {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_TRANSLATION();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<float> cv_rgbd_Odometry_DEFAULT_MAX_ROTATION() {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_ROTATION();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<bool> cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const_MatR(const cv::rgbd::Odometry* instance, const cv::Mat* srcImage, const cv::Mat* srcDepth, const cv::Mat* srcMask, const cv::Mat* dstImage, const cv::Mat* dstDepth, const cv::Mat* dstMask, const cv::_OutputArray* Rt, const cv::Mat* initRt) {
		try {
			bool ret = instance->compute(*srcImage, *srcDepth, *srcMask, *dstImage, *dstDepth, *dstMask, *Rt, *initRt);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_rgbd_Odometry_compute_const_Ptr_OdometryFrame_R_Ptr_OdometryFrame_R_const__OutputArrayR_const_MatR(const cv::rgbd::Odometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* srcFrame, cv::Ptr<cv::rgbd::OdometryFrame>* dstFrame, const cv::_OutputArray* Rt, const cv::Mat* initRt) {
		try {
			bool ret = instance->compute(*srcFrame, *dstFrame, *Rt, *initRt);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Size> cv_rgbd_Odometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(const cv::rgbd::Odometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<cv::Ptr<cv::rgbd::Odometry>*> cv_rgbd_Odometry_create_const_StringR(const char* odometryType) {
		try {
			cv::Ptr<cv::rgbd::Odometry> ret = cv::rgbd::Odometry::create(cv::String(odometryType));
			return Ok(new cv::Ptr<cv::rgbd::Odometry>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::Odometry>*>))
	}
	
	Result<cv::Mat*> cv_rgbd_Odometry_getCameraMatrix_const(const cv::rgbd::Odometry* instance) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_Odometry_setCameraMatrix_const_MatR(cv::rgbd::Odometry* instance, const cv::Mat* val) {
		try {
			instance->setCameraMatrix(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_Odometry_getTransformType_const(const cv::rgbd::Odometry* instance) {
		try {
			int ret = instance->getTransformType();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_Odometry_setTransformType_int(cv::rgbd::Odometry* instance, int val) {
		try {
			instance->setTransformType(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_rgbd_OdometryFrame_getPropPyramidImage(cv::rgbd::OdometryFrame* instance) {
		try {
			std::vector<cv::Mat> ret = instance->pyramidImage;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_rgbd_OdometryFrame_setPropPyramidImage_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
		try {
			instance->pyramidImage = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_rgbd_OdometryFrame_getPropPyramidDepth(cv::rgbd::OdometryFrame* instance) {
		try {
			std::vector<cv::Mat> ret = instance->pyramidDepth;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_rgbd_OdometryFrame_setPropPyramidDepth_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
		try {
			instance->pyramidDepth = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_rgbd_OdometryFrame_getPropPyramidMask(cv::rgbd::OdometryFrame* instance) {
		try {
			std::vector<cv::Mat> ret = instance->pyramidMask;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_rgbd_OdometryFrame_setPropPyramidMask_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
		try {
			instance->pyramidMask = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_rgbd_OdometryFrame_getPropPyramidCloud(cv::rgbd::OdometryFrame* instance) {
		try {
			std::vector<cv::Mat> ret = instance->pyramidCloud;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_rgbd_OdometryFrame_setPropPyramidCloud_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
		try {
			instance->pyramidCloud = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_rgbd_OdometryFrame_getPropPyramid_dI_dx(cv::rgbd::OdometryFrame* instance) {
		try {
			std::vector<cv::Mat> ret = instance->pyramid_dI_dx;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_rgbd_OdometryFrame_setPropPyramid_dI_dx_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
		try {
			instance->pyramid_dI_dx = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_rgbd_OdometryFrame_getPropPyramid_dI_dy(cv::rgbd::OdometryFrame* instance) {
		try {
			std::vector<cv::Mat> ret = instance->pyramid_dI_dy;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_rgbd_OdometryFrame_setPropPyramid_dI_dy_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
		try {
			instance->pyramid_dI_dy = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_rgbd_OdometryFrame_getPropPyramidTexturedMask(cv::rgbd::OdometryFrame* instance) {
		try {
			std::vector<cv::Mat> ret = instance->pyramidTexturedMask;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_rgbd_OdometryFrame_setPropPyramidTexturedMask_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
		try {
			instance->pyramidTexturedMask = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_rgbd_OdometryFrame_getPropPyramidNormals(cv::rgbd::OdometryFrame* instance) {
		try {
			std::vector<cv::Mat> ret = instance->pyramidNormals;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_rgbd_OdometryFrame_setPropPyramidNormals_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
		try {
			instance->pyramidNormals = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_rgbd_OdometryFrame_getPropPyramidNormalsMask(cv::rgbd::OdometryFrame* instance) {
		try {
			std::vector<cv::Mat> ret = instance->pyramidNormalsMask;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_rgbd_OdometryFrame_setPropPyramidNormalsMask_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
		try {
			instance->pyramidNormalsMask = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_OdometryFrame_delete(cv::rgbd::OdometryFrame* instance) {
		delete instance;
	}
	Result<cv::rgbd::OdometryFrame*> cv_rgbd_OdometryFrame_OdometryFrame() {
		try {
			cv::rgbd::OdometryFrame* ret = new cv::rgbd::OdometryFrame();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::OdometryFrame*>))
	}
	
	Result<cv::rgbd::OdometryFrame*> cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID) {
		try {
			cv::rgbd::OdometryFrame* ret = new cv::rgbd::OdometryFrame(*image, *depth, *mask, *normals, ID);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::OdometryFrame*>))
	}
	
	Result<cv::Ptr<cv::rgbd::OdometryFrame>*> cv_rgbd_OdometryFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID) {
		try {
			cv::Ptr<cv::rgbd::OdometryFrame> ret = cv::rgbd::OdometryFrame::create(*image, *depth, *mask, *normals, ID);
			return Ok(new cv::Ptr<cv::rgbd::OdometryFrame>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::OdometryFrame>*>))
	}
	
	Result_void cv_rgbd_OdometryFrame_release(cv::rgbd::OdometryFrame* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rgbd_OdometryFrame_releasePyramids(cv::rgbd::OdometryFrame* instance) {
		try {
			instance->releasePyramids();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_RgbdFrame_getPropID_const(const cv::rgbd::RgbdFrame* instance) {
		try {
			int ret = instance->ID;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_RgbdFrame_setPropID_int(cv::rgbd::RgbdFrame* instance, int val) {
		try {
			instance->ID = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_rgbd_RgbdFrame_getPropImage(cv::rgbd::RgbdFrame* instance) {
		try {
			cv::Mat ret = instance->image;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_RgbdFrame_setPropImage_Mat(cv::rgbd::RgbdFrame* instance, cv::Mat* val) {
		try {
			instance->image = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_rgbd_RgbdFrame_getPropDepth(cv::rgbd::RgbdFrame* instance) {
		try {
			cv::Mat ret = instance->depth;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_RgbdFrame_setPropDepth_Mat(cv::rgbd::RgbdFrame* instance, cv::Mat* val) {
		try {
			instance->depth = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_rgbd_RgbdFrame_getPropMask(cv::rgbd::RgbdFrame* instance) {
		try {
			cv::Mat ret = instance->mask;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_RgbdFrame_setPropMask_Mat(cv::rgbd::RgbdFrame* instance, cv::Mat* val) {
		try {
			instance->mask = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_rgbd_RgbdFrame_getPropNormals(cv::rgbd::RgbdFrame* instance) {
		try {
			cv::Mat ret = instance->normals;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_RgbdFrame_setPropNormals_Mat(cv::rgbd::RgbdFrame* instance, cv::Mat* val) {
		try {
			instance->normals = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_RgbdFrame_delete(cv::rgbd::RgbdFrame* instance) {
		delete instance;
	}
	Result<cv::rgbd::RgbdFrame*> cv_rgbd_RgbdFrame_RgbdFrame() {
		try {
			cv::rgbd::RgbdFrame* ret = new cv::rgbd::RgbdFrame();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdFrame*>))
	}
	
	Result<cv::rgbd::RgbdFrame*> cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID) {
		try {
			cv::rgbd::RgbdFrame* ret = new cv::rgbd::RgbdFrame(*image, *depth, *mask, *normals, ID);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdFrame*>))
	}
	
	Result<cv::Ptr<cv::rgbd::RgbdFrame>*> cv_rgbd_RgbdFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID) {
		try {
			cv::Ptr<cv::rgbd::RgbdFrame> ret = cv::rgbd::RgbdFrame::create(*image, *depth, *mask, *normals, ID);
			return Ok(new cv::Ptr<cv::rgbd::RgbdFrame>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdFrame>*>))
	}
	
	Result_void cv_rgbd_RgbdFrame_release(cv::rgbd::RgbdFrame* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_RgbdICPOdometry_delete(cv::rgbd::RgbdICPOdometry* instance) {
		delete instance;
	}
	Result<cv::rgbd::RgbdICPOdometry*> cv_rgbd_RgbdICPOdometry_RgbdICPOdometry() {
		try {
			cv::rgbd::RgbdICPOdometry* ret = new cv::rgbd::RgbdICPOdometry();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdICPOdometry*>))
	}
	
	Result<cv::rgbd::RgbdICPOdometry*> cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR_float_float_float_float_const_vector_int_R_const_vector_float_R_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, int transformType) {
		try {
			cv::rgbd::RgbdICPOdometry* ret = new cv::rgbd::RgbdICPOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, *minGradientMagnitudes, transformType);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdICPOdometry*>))
	}
	
	Result<cv::Ptr<cv::rgbd::RgbdICPOdometry>*> cv_rgbd_RgbdICPOdometry_create_const_MatR_float_float_float_float_const_vector_int_R_const_vector_float_R_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, int transformType) {
		try {
			cv::Ptr<cv::rgbd::RgbdICPOdometry> ret = cv::rgbd::RgbdICPOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, *minGradientMagnitudes, transformType);
			return Ok(new cv::Ptr<cv::rgbd::RgbdICPOdometry>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdICPOdometry>*>))
	}
	
	Result<cv::Size> cv_rgbd_RgbdICPOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(const cv::rgbd::RgbdICPOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<cv::Mat*> cv_rgbd_RgbdICPOdometry_getCameraMatrix_const(const cv::rgbd::RgbdICPOdometry* instance) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_RgbdICPOdometry_setCameraMatrix_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val) {
		try {
			instance->setCameraMatrix(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdICPOdometry_getMinDepth_const(const cv::rgbd::RgbdICPOdometry* instance) {
		try {
			double ret = instance->getMinDepth();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdICPOdometry_setMinDepth_double(cv::rgbd::RgbdICPOdometry* instance, double val) {
		try {
			instance->setMinDepth(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdICPOdometry_getMaxDepth_const(const cv::rgbd::RgbdICPOdometry* instance) {
		try {
			double ret = instance->getMaxDepth();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdICPOdometry_setMaxDepth_double(cv::rgbd::RgbdICPOdometry* instance, double val) {
		try {
			instance->setMaxDepth(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdICPOdometry_getMaxDepthDiff_const(const cv::rgbd::RgbdICPOdometry* instance) {
		try {
			double ret = instance->getMaxDepthDiff();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdICPOdometry_setMaxDepthDiff_double(cv::rgbd::RgbdICPOdometry* instance, double val) {
		try {
			instance->setMaxDepthDiff(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdICPOdometry_getMaxPointsPart_const(const cv::rgbd::RgbdICPOdometry* instance) {
		try {
			double ret = instance->getMaxPointsPart();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdICPOdometry_setMaxPointsPart_double(cv::rgbd::RgbdICPOdometry* instance, double val) {
		try {
			instance->setMaxPointsPart(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_rgbd_RgbdICPOdometry_getIterationCounts_const(const cv::rgbd::RgbdICPOdometry* instance) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_RgbdICPOdometry_setIterationCounts_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val) {
		try {
			instance->setIterationCounts(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_rgbd_RgbdICPOdometry_getMinGradientMagnitudes_const(const cv::rgbd::RgbdICPOdometry* instance) {
		try {
			cv::Mat ret = instance->getMinGradientMagnitudes();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_RgbdICPOdometry_setMinGradientMagnitudes_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val) {
		try {
			instance->setMinGradientMagnitudes(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_RgbdICPOdometry_getTransformType_const(const cv::rgbd::RgbdICPOdometry* instance) {
		try {
			int ret = instance->getTransformType();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_RgbdICPOdometry_setTransformType_int(cv::rgbd::RgbdICPOdometry* instance, int val) {
		try {
			instance->setTransformType(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdICPOdometry_getMaxTranslation_const(const cv::rgbd::RgbdICPOdometry* instance) {
		try {
			double ret = instance->getMaxTranslation();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdICPOdometry_setMaxTranslation_double(cv::rgbd::RgbdICPOdometry* instance, double val) {
		try {
			instance->setMaxTranslation(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdICPOdometry_getMaxRotation_const(const cv::rgbd::RgbdICPOdometry* instance) {
		try {
			double ret = instance->getMaxRotation();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdICPOdometry_setMaxRotation_double(cv::rgbd::RgbdICPOdometry* instance, double val) {
		try {
			instance->setMaxRotation(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::rgbd::RgbdNormals>*> cv_rgbd_RgbdICPOdometry_getNormalsComputer_const(const cv::rgbd::RgbdICPOdometry* instance) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = instance->getNormalsComputer();
			return Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdNormals>*>))
	}
	
	void cv_RgbdNormals_delete(cv::rgbd::RgbdNormals* instance) {
		delete instance;
	}
	Result<cv::rgbd::RgbdNormals*> cv_rgbd_RgbdNormals_RgbdNormals() {
		try {
			cv::rgbd::RgbdNormals* ret = new cv::rgbd::RgbdNormals();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdNormals*>))
	}
	
	Result<cv::rgbd::RgbdNormals*> cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR_int_int(int rows, int cols, int depth, const cv::_InputArray* K, int window_size, int method) {
		try {
			cv::rgbd::RgbdNormals* ret = new cv::rgbd::RgbdNormals(rows, cols, depth, *K, window_size, method);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdNormals*>))
	}
	
	Result<cv::Ptr<cv::rgbd::RgbdNormals>*> cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR_int_int(int rows, int cols, int depth, const cv::_InputArray* K, int window_size, int method) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = cv::rgbd::RgbdNormals::create(rows, cols, depth, *K, window_size, method);
			return Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdNormals>*>))
	}
	
	Result_void cv_rgbd_RgbdNormals_initialize_const(const cv::rgbd::RgbdNormals* instance) {
		try {
			instance->initialize();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_RgbdNormals_getRows_const(const cv::rgbd::RgbdNormals* instance) {
		try {
			int ret = instance->getRows();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_RgbdNormals_setRows_int(cv::rgbd::RgbdNormals* instance, int val) {
		try {
			instance->setRows(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_RgbdNormals_getCols_const(const cv::rgbd::RgbdNormals* instance) {
		try {
			int ret = instance->getCols();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_RgbdNormals_setCols_int(cv::rgbd::RgbdNormals* instance, int val) {
		try {
			instance->setCols(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_RgbdNormals_getWindowSize_const(const cv::rgbd::RgbdNormals* instance) {
		try {
			int ret = instance->getWindowSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_RgbdNormals_setWindowSize_int(cv::rgbd::RgbdNormals* instance, int val) {
		try {
			instance->setWindowSize(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_RgbdNormals_getDepth_const(const cv::rgbd::RgbdNormals* instance) {
		try {
			int ret = instance->getDepth();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_RgbdNormals_setDepth_int(cv::rgbd::RgbdNormals* instance, int val) {
		try {
			instance->setDepth(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_rgbd_RgbdNormals_getK_const(const cv::rgbd::RgbdNormals* instance) {
		try {
			cv::Mat ret = instance->getK();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_RgbdNormals_setK_const_MatR(cv::rgbd::RgbdNormals* instance, const cv::Mat* val) {
		try {
			instance->setK(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_RgbdNormals_getMethod_const(const cv::rgbd::RgbdNormals* instance) {
		try {
			int ret = instance->getMethod();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_RgbdNormals_setMethod_int(cv::rgbd::RgbdNormals* instance, int val) {
		try {
			instance->setMethod(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_RgbdOdometry_delete(cv::rgbd::RgbdOdometry* instance) {
		delete instance;
	}
	Result<cv::rgbd::RgbdOdometry*> cv_rgbd_RgbdOdometry_RgbdOdometry() {
		try {
			cv::rgbd::RgbdOdometry* ret = new cv::rgbd::RgbdOdometry();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdOdometry*>))
	}
	
	Result<cv::rgbd::RgbdOdometry*> cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR_float_float_float_const_vector_int_R_const_vector_float_R_float_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, float maxPointsPart, int transformType) {
		try {
			cv::rgbd::RgbdOdometry* ret = new cv::rgbd::RgbdOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, *iterCounts, *minGradientMagnitudes, maxPointsPart, transformType);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdOdometry*>))
	}
	
	Result<cv::Ptr<cv::rgbd::RgbdOdometry>*> cv_rgbd_RgbdOdometry_create_const_MatR_float_float_float_const_vector_int_R_const_vector_float_R_float_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, float maxPointsPart, int transformType) {
		try {
			cv::Ptr<cv::rgbd::RgbdOdometry> ret = cv::rgbd::RgbdOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, *iterCounts, *minGradientMagnitudes, maxPointsPart, transformType);
			return Ok(new cv::Ptr<cv::rgbd::RgbdOdometry>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdOdometry>*>))
	}
	
	Result<cv::Size> cv_rgbd_RgbdOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(const cv::rgbd::RgbdOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<cv::Mat*> cv_rgbd_RgbdOdometry_getCameraMatrix_const(const cv::rgbd::RgbdOdometry* instance) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val) {
		try {
			instance->setCameraMatrix(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdOdometry_getMinDepth_const(const cv::rgbd::RgbdOdometry* instance) {
		try {
			double ret = instance->getMinDepth();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdOdometry_setMinDepth_double(cv::rgbd::RgbdOdometry* instance, double val) {
		try {
			instance->setMinDepth(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdOdometry_getMaxDepth_const(const cv::rgbd::RgbdOdometry* instance) {
		try {
			double ret = instance->getMaxDepth();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdOdometry_setMaxDepth_double(cv::rgbd::RgbdOdometry* instance, double val) {
		try {
			instance->setMaxDepth(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(const cv::rgbd::RgbdOdometry* instance) {
		try {
			double ret = instance->getMaxDepthDiff();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(cv::rgbd::RgbdOdometry* instance, double val) {
		try {
			instance->setMaxDepthDiff(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_rgbd_RgbdOdometry_getIterationCounts_const(const cv::rgbd::RgbdOdometry* instance) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_RgbdOdometry_setIterationCounts_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val) {
		try {
			instance->setIterationCounts(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_rgbd_RgbdOdometry_getMinGradientMagnitudes_const(const cv::rgbd::RgbdOdometry* instance) {
		try {
			cv::Mat ret = instance->getMinGradientMagnitudes();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val) {
		try {
			instance->setMinGradientMagnitudes(*val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdOdometry_getMaxPointsPart_const(const cv::rgbd::RgbdOdometry* instance) {
		try {
			double ret = instance->getMaxPointsPart();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdOdometry_setMaxPointsPart_double(cv::rgbd::RgbdOdometry* instance, double val) {
		try {
			instance->setMaxPointsPart(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_RgbdOdometry_getTransformType_const(const cv::rgbd::RgbdOdometry* instance) {
		try {
			int ret = instance->getTransformType();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_RgbdOdometry_setTransformType_int(cv::rgbd::RgbdOdometry* instance, int val) {
		try {
			instance->setTransformType(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdOdometry_getMaxTranslation_const(const cv::rgbd::RgbdOdometry* instance) {
		try {
			double ret = instance->getMaxTranslation();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdOdometry_setMaxTranslation_double(cv::rgbd::RgbdOdometry* instance, double val) {
		try {
			instance->setMaxTranslation(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdOdometry_getMaxRotation_const(const cv::rgbd::RgbdOdometry* instance) {
		try {
			double ret = instance->getMaxRotation();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdOdometry_setMaxRotation_double(cv::rgbd::RgbdOdometry* instance, double val) {
		try {
			instance->setMaxRotation(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_RgbdPlane_delete(cv::rgbd::RgbdPlane* instance) {
		delete instance;
	}
	Result<cv::rgbd::RgbdPlane*> cv_rgbd_RgbdPlane_RgbdPlane_int(int method) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane(method);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdPlane*>))
	}
	
	Result<cv::rgbd::RgbdPlane*> cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double_double_double_double(int method, int block_size, int min_size, double threshold, double sensor_error_a, double sensor_error_b, double sensor_error_c) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdPlane*>))
	}
	
	Result<cv::Ptr<cv::rgbd::RgbdPlane>*> cv_rgbd_RgbdPlane_create_int_int_int_double_double_double_double(int method, int block_size, int min_size, double threshold, double sensor_error_a, double sensor_error_b, double sensor_error_c) {
		try {
			cv::Ptr<cv::rgbd::RgbdPlane> ret = cv::rgbd::RgbdPlane::create(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c);
			return Ok(new cv::Ptr<cv::rgbd::RgbdPlane>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdPlane>*>))
	}
	
	Result<int> cv_rgbd_RgbdPlane_getBlockSize_const(const cv::rgbd::RgbdPlane* instance) {
		try {
			int ret = instance->getBlockSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_RgbdPlane_setBlockSize_int(cv::rgbd::RgbdPlane* instance, int val) {
		try {
			instance->setBlockSize(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_RgbdPlane_getMinSize_const(const cv::rgbd::RgbdPlane* instance) {
		try {
			int ret = instance->getMinSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_RgbdPlane_setMinSize_int(cv::rgbd::RgbdPlane* instance, int val) {
		try {
			instance->setMinSize(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rgbd_RgbdPlane_getMethod_const(const cv::rgbd::RgbdPlane* instance) {
		try {
			int ret = instance->getMethod();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_rgbd_RgbdPlane_setMethod_int(cv::rgbd::RgbdPlane* instance, int val) {
		try {
			instance->setMethod(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdPlane_getThreshold_const(const cv::rgbd::RgbdPlane* instance) {
		try {
			double ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdPlane_setThreshold_double(cv::rgbd::RgbdPlane* instance, double val) {
		try {
			instance->setThreshold(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdPlane_getSensorErrorA_const(const cv::rgbd::RgbdPlane* instance) {
		try {
			double ret = instance->getSensorErrorA();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdPlane_setSensorErrorA_double(cv::rgbd::RgbdPlane* instance, double val) {
		try {
			instance->setSensorErrorA(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdPlane_getSensorErrorB_const(const cv::rgbd::RgbdPlane* instance) {
		try {
			double ret = instance->getSensorErrorB();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdPlane_setSensorErrorB_double(cv::rgbd::RgbdPlane* instance, double val) {
		try {
			instance->setSensorErrorB(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_rgbd_RgbdPlane_getSensorErrorC_const(const cv::rgbd::RgbdPlane* instance) {
		try {
			double ret = instance->getSensorErrorC();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_rgbd_RgbdPlane_setSensorErrorC_double(cv::rgbd::RgbdPlane* instance, double val) {
		try {
			instance->setSensorErrorC(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}

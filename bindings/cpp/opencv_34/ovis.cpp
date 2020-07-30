#include "common.hpp"
#include <opencv2/ovis.hpp>
#include "ovis_types.hpp"

extern "C" {
	Result_void cv_ovis_addResourceLocation_const_StringR(const char* path) {
		try {
			cv::ovis::addResourceLocation(cv::String(path));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_createGridMesh_const_StringR_const_Size2fR_const_SizeR(const char* name, const cv::Size2f* size, const cv::Size* segments) {
		try {
			cv::ovis::createGridMesh(cv::String(name), *size, *segments);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_createPlaneMesh_const_StringR_const_Size2fR_const__InputArrayR(const char* name, const cv::Size2f* size, const cv::_InputArray* image) {
		try {
			cv::ovis::createPlaneMesh(cv::String(name), *size, *image);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_createPointCloudMesh_const_StringR_const__InputArrayR_const__InputArrayR(const char* name, const cv::_InputArray* vertices, const cv::_InputArray* colors) {
		try {
			cv::ovis::createPointCloudMesh(cv::String(name), *vertices, *colors);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::ovis::WindowScene>*> cv_ovis_createWindow_const_StringR_const_SizeR_int(const char* title, const cv::Size* size, int flags) {
		try {
			cv::Ptr<cv::ovis::WindowScene> ret = cv::ovis::createWindow(cv::String(title), *size, flags);
			return Ok(new cv::Ptr<cv::ovis::WindowScene>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ovis::WindowScene>*>))
	}
	
	Result_void cv_ovis_setMaterialProperty_const_StringR_const_StringR_const_ScalarR(const char* name, const char* prop, const cv::Scalar* value) {
		try {
			cv::ovis::setMaterialProperty(cv::String(name), cv::String(prop), *value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_setMaterialProperty_const_StringR_int_const_ScalarR(const char* name, int prop, const cv::Scalar* value) {
		try {
			cv::ovis::setMaterialProperty(cv::String(name), prop, *value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_setMaterialProperty_const_StringR_int_const_StringR(const char* name, int prop, const char* value) {
		try {
			cv::ovis::setMaterialProperty(cv::String(name), prop, cv::String(value));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_updateTexture_const_StringR_const__InputArrayR(const char* name, const cv::_InputArray* image) {
		try {
			cv::ovis::updateTexture(cv::String(name), *image);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_ovis_waitKey_int(int delay) {
		try {
			int ret = cv::ovis::waitKey(delay);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ovis_WindowScene_setBackground_const__InputArrayR(cv::ovis::WindowScene* instance, const cv::_InputArray* image) {
		try {
			instance->setBackground(*image);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_setBackground_const_ScalarR(cv::ovis::WindowScene* instance, const cv::Scalar* color) {
		try {
			instance->setBackground(*color);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_setCompositors_const_vector_String_R(cv::ovis::WindowScene* instance, const std::vector<cv::String>* names) {
		try {
			instance->setCompositors(*names);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_createEntity_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(cv::ovis::WindowScene* instance, const char* name, const char* meshname, const cv::_InputArray* tvec, const cv::_InputArray* rot) {
		try {
			instance->createEntity(cv::String(name), cv::String(meshname), *tvec, *rot);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_removeEntity_const_StringR(cv::ovis::WindowScene* instance, const char* name) {
		try {
			instance->removeEntity(cv::String(name));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_setEntityProperty_const_StringR_int_const_ScalarR(cv::ovis::WindowScene* instance, const char* name, int prop, const cv::Scalar* value) {
		try {
			instance->setEntityProperty(cv::String(name), prop, *value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_setEntityProperty_const_StringR_int_const_StringR(cv::ovis::WindowScene* instance, const char* name, int prop, const char* value) {
		try {
			instance->setEntityProperty(cv::String(name), prop, cv::String(value));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_getEntityProperty_const_StringR_int_const__OutputArrayR(cv::ovis::WindowScene* instance, const char* name, int prop, const cv::_OutputArray* value) {
		try {
			instance->getEntityProperty(cv::String(name), prop, *value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect2d> cv_ovis_WindowScene_createCameraEntity_const_StringR_const__InputArrayR_const_SizeR_float_const__InputArrayR_const__InputArrayR(cv::ovis::WindowScene* instance, const char* name, const cv::_InputArray* K, const cv::Size* imsize, float zFar, const cv::_InputArray* tvec, const cv::_InputArray* rot) {
		try {
			cv::Rect2d ret = instance->createCameraEntity(cv::String(name), *K, *imsize, zFar, *tvec, *rot);
			return Ok<cv::Rect2d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect2d>))
	}
	
	Result_void cv_ovis_WindowScene_createLightEntity_const_StringR_const__InputArrayR_const__InputArrayR_const_ScalarR_const_ScalarR(cv::ovis::WindowScene* instance, const char* name, const cv::_InputArray* tvec, const cv::_InputArray* rot, const cv::Scalar* diffuseColor, const cv::Scalar* specularColor) {
		try {
			instance->createLightEntity(cv::String(name), *tvec, *rot, *diffuseColor, *specularColor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_updateEntityPose_const_StringR_const__InputArrayR_const__InputArrayR(cv::ovis::WindowScene* instance, const char* name, const cv::_InputArray* tvec, const cv::_InputArray* rot) {
		try {
			instance->updateEntityPose(cv::String(name), *tvec, *rot);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_setEntityPose_const_StringR_const__InputArrayR_const__InputArrayR_bool(cv::ovis::WindowScene* instance, const char* name, const cv::_InputArray* tvec, const cv::_InputArray* rot, bool invert) {
		try {
			instance->setEntityPose(cv::String(name), *tvec, *rot, invert);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_getScreenshot_const__OutputArrayR(cv::ovis::WindowScene* instance, const cv::_OutputArray* frame) {
		try {
			instance->getScreenshot(*frame);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_getCompositorTexture_const_StringR_const_StringR_const__OutputArrayR_int(cv::ovis::WindowScene* instance, const char* compname, const char* texname, const cv::_OutputArray* out, int mrtIndex) {
		try {
			instance->getCompositorTexture(cv::String(compname), cv::String(texname), *out, mrtIndex);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_getDepth_const__OutputArrayR(cv::ovis::WindowScene* instance, const cv::_OutputArray* depth) {
		try {
			instance->getDepth(*depth);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_fixCameraYawAxis_bool_const__InputArrayR(cv::ovis::WindowScene* instance, bool useFixed, const cv::_InputArray* up) {
		try {
			instance->fixCameraYawAxis(useFixed, *up);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_setCameraPose_const__InputArrayR_const__InputArrayR_bool(cv::ovis::WindowScene* instance, const cv::_InputArray* tvec, const cv::_InputArray* rot, bool invert) {
		try {
			instance->setCameraPose(*tvec, *rot, invert);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_setCameraLookAt_const_StringR_const__InputArrayR(cv::ovis::WindowScene* instance, const char* target, const cv::_InputArray* offset) {
		try {
			instance->setCameraLookAt(cv::String(target), *offset);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_getCameraPose_const__OutputArrayR_const__OutputArrayR_bool(cv::ovis::WindowScene* instance, const cv::_OutputArray* R, const cv::_OutputArray* tvec, bool invert) {
		try {
			instance->getCameraPose(*R, *tvec, invert);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ovis_WindowScene_setCameraIntrinsics_const__InputArrayR_const_SizeR_float_float(cv::ovis::WindowScene* instance, const cv::_InputArray* K, const cv::Size* imsize, float zNear, float zFar) {
		try {
			instance->setCameraIntrinsics(*K, *imsize, zNear, zFar);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}

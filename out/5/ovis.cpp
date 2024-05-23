#include "ocvrs_common.hpp"
#include <opencv2/ovis.hpp>
#include "ovis_types.hpp"

extern "C" {
	// addResourceLocation(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:324
	// ("cv::ovis::addResourceLocation", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	void cv_ovis_addResourceLocation_const_StringR(const char* path, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::addResourceLocation(std::string(path));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::createGridMesh(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:404
	// ("cv::ovis::createGridMesh", vec![(pred!(mut, ["name", "size"], ["const cv::String*", "const cv::Size2f*"]), _)]),
	void cv_ovis_createGridMesh_const_StringR_const_Size2fR(const char* name, const cv::Size2f* size, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::createGridMesh(std::string(name), *size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createGridMesh(const String &, const Size2f &, const Size &)(InString, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:404
	// ("cv::ovis::createGridMesh", vec![(pred!(mut, ["name", "size", "segments"], ["const cv::String*", "const cv::Size2f*", "const cv::Size*"]), _)]),
	void cv_ovis_createGridMesh_const_StringR_const_Size2fR_const_SizeR(const char* name, const cv::Size2f* size, const cv::Size* segments, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::createGridMesh(std::string(name), *size, *segments);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::createPlaneMesh(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:384
	// ("cv::ovis::createPlaneMesh", vec![(pred!(mut, ["name", "size"], ["const cv::String*", "const cv::Size2f*"]), _)]),
	void cv_ovis_createPlaneMesh_const_StringR_const_Size2fR(const char* name, const cv::Size2f* size, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::createPlaneMesh(std::string(name), *size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createPlaneMesh(const String &, const Size2f &, InputArray)(InString, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:384
	// ("cv::ovis::createPlaneMesh", vec![(pred!(mut, ["name", "size", "image"], ["const cv::String*", "const cv::Size2f*", "const cv::_InputArray*"]), _)]),
	void cv_ovis_createPlaneMesh_const_StringR_const_Size2fR_const__InputArrayR(const char* name, const cv::Size2f* size, const cv::_InputArray* image, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::createPlaneMesh(std::string(name), *size, *image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::createPointCloudMesh(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:394
	// ("cv::ovis::createPointCloudMesh", vec![(pred!(mut, ["name", "vertices"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_ovis_createPointCloudMesh_const_StringR_const__InputArrayR(const char* name, const cv::_InputArray* vertices, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::createPointCloudMesh(std::string(name), *vertices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createPointCloudMesh(const String &, InputArray, InputArray)(InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:394
	// ("cv::ovis::createPointCloudMesh", vec![(pred!(mut, ["name", "vertices", "colors"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ovis_createPointCloudMesh_const_StringR_const__InputArrayR_const__InputArrayR(const char* name, const cv::_InputArray* vertices, const cv::_InputArray* colors, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::createPointCloudMesh(std::string(name), *vertices, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::createTriangleMesh(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:415
	// ("cv::ovis::createTriangleMesh", vec![(pred!(mut, ["name", "vertices"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_ovis_createTriangleMesh_const_StringR_const__InputArrayR(const char* name, const cv::_InputArray* vertices, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::createTriangleMesh(std::string(name), *vertices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createTriangleMesh(const String &, InputArray, InputArray, InputArray)(InString, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:415
	// ("cv::ovis::createTriangleMesh", vec![(pred!(mut, ["name", "vertices", "normals", "indices"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ovis_createTriangleMesh_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const char* name, const cv::_InputArray* vertices, const cv::_InputArray* normals, const cv::_InputArray* indices, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::createTriangleMesh(std::string(name), *vertices, *normals, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::createWindow(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:337
	// ("cv::ovis::createWindow", vec![(pred!(mut, ["title", "size"], ["const cv::String*", "const cv::Size*"]), _)]),
	void cv_ovis_createWindow_const_StringR_const_SizeR(const char* title, const cv::Size* size, Result<cv::Ptr<cv::ovis::WindowScene>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ovis::WindowScene> ret = cv::ovis::createWindow(std::string(title), *size);
			Ok(new cv::Ptr<cv::ovis::WindowScene>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createWindow(const String &, const Size &, int)(InString, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:337
	// ("cv::ovis::createWindow", vec![(pred!(mut, ["title", "size", "flags"], ["const cv::String*", "const cv::Size*", "int"]), _)]),
	void cv_ovis_createWindow_const_StringR_const_SizeR_int(const char* title, const cv::Size* size, int flags, Result<cv::Ptr<cv::ovis::WindowScene>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ovis::WindowScene> ret = cv::ovis::createWindow(std::string(title), *size, flags);
			Ok(new cv::Ptr<cv::ovis::WindowScene>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaterialProperty(const String &, const String &, const Scalar &)(InString, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:374
	// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "const cv::String*", "const cv::Scalar*"]), _)]),
	void cv_ovis_setMaterialProperty_const_StringR_const_StringR_const_ScalarR(const char* name, const char* prop, const cv::Scalar* value, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::setMaterialProperty(std::string(name), std::string(prop), *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaterialProperty(const String &, int, const Scalar &)(InString, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:355
	// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::Scalar*"]), _)]),
	void cv_ovis_setMaterialProperty_const_StringR_int_const_ScalarR(const char* name, int prop, const cv::Scalar* value, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::setMaterialProperty(std::string(name), prop, *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaterialProperty(const String &, int, const String &)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:358
	// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::String*"]), _)]),
	void cv_ovis_setMaterialProperty_const_StringR_int_const_StringR(const char* name, int prop, const char* value, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::setMaterialProperty(std::string(name), prop, std::string(value));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaterialProperty(const String &, int, InputArray)(InString, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:366
	// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::_InputArray*"]), _)]),
	void cv_ovis_setMaterialProperty_const_StringR_int_const__InputArrayR(const char* name, int prop, const cv::_InputArray* value, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::setMaterialProperty(std::string(name), prop, *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateTexture(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:418
	// ("cv::ovis::updateTexture", vec![(pred!(mut, ["name", "image"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_ovis_updateTexture_const_StringR_const__InputArrayR(const char* name, const cv::_InputArray* image, ResultVoid* ocvrs_return) {
		try {
			cv::ovis::updateTexture(std::string(name), *image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::waitKey() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:347
	// ("cv::ovis::waitKey", vec![(pred!(mut, [], []), _)]),
	void cv_ovis_waitKey(Result<int>* ocvrs_return) {
		try {
			int ret = cv::ovis::waitKey();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// waitKey(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:347
	// ("cv::ovis::waitKey", vec![(pred!(mut, ["delay"], ["int"]), _)]),
	void cv_ovis_waitKey_int(int delay, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ovis::waitKey(delay);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackground(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:94
	// ("cv::ovis::WindowScene::setBackground", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_ovis_WindowScene_setBackground_const__InputArrayR(cv::ovis::WindowScene* instance, const cv::_InputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->setBackground(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackground(const Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:97
	// ("cv::ovis::WindowScene::setBackground", vec![(pred!(mut, ["color"], ["const cv::Scalar*"]), _)]),
	void cv_ovis_WindowScene_setBackground_const_ScalarR(cv::ovis::WindowScene* instance, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			instance->setBackground(*color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCompositors(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:107
	// ("cv::ovis::WindowScene::setCompositors", vec![(pred!(mut, ["names"], ["const std::vector<cv::String>*"]), _)]),
	void cv_ovis_WindowScene_setCompositors_const_vectorLStringGR(cv::ovis::WindowScene* instance, const std::vector<cv::String>* names, ResultVoid* ocvrs_return) {
		try {
			instance->setCompositors(*names);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createEntity(const String &, const String &, InputArray, InputArray)(InString, InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:120
	// ("cv::ovis::WindowScene::createEntity", vec![(pred!(mut, ["name", "meshname", "tvec", "rot"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ovis_WindowScene_createEntity_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(cv::ovis::WindowScene* instance, const char* name, const char* meshname, const cv::_InputArray* tvec, const cv::_InputArray* rot, ResultVoid* ocvrs_return) {
		try {
			instance->createEntity(std::string(name), std::string(meshname), *tvec, *rot);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::createEntity(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:120
	// ("cv::ovis::WindowScene::createEntity", vec![(pred!(mut, ["name", "meshname"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ovis_WindowScene_createEntity_const_StringR_const_StringR(cv::ovis::WindowScene* instance, const char* name, const char* meshname, ResultVoid* ocvrs_return) {
		try {
			instance->createEntity(std::string(name), std::string(meshname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// removeEntity(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:127
	// ("cv::ovis::WindowScene::removeEntity", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
	void cv_ovis_WindowScene_removeEntity_const_StringR(cv::ovis::WindowScene* instance, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->removeEntity(std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEntityProperty(const String &, int, const String &, int)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:136
	// ("cv::ovis::WindowScene::setEntityProperty", vec![(pred!(mut, ["name", "prop", "value", "subEntityIdx"], ["const cv::String*", "int", "const cv::String*", "int"]), _)]),
	void cv_ovis_WindowScene_setEntityProperty_const_StringR_int_const_StringR_int(cv::ovis::WindowScene* instance, const char* name, int prop, const char* value, int subEntityIdx, ResultVoid* ocvrs_return) {
		try {
			instance->setEntityProperty(std::string(name), prop, std::string(value), subEntityIdx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::setEntityProperty(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:136
	// ("cv::ovis::WindowScene::setEntityProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::String*"]), _)]),
	void cv_ovis_WindowScene_setEntityProperty_const_StringR_int_const_StringR(cv::ovis::WindowScene* instance, const char* name, int prop, const char* value, ResultVoid* ocvrs_return) {
		try {
			instance->setEntityProperty(std::string(name), prop, std::string(value));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEntityProperty(const String &, int, const Scalar &)(InString, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:140
	// ("cv::ovis::WindowScene::setEntityProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::Scalar*"]), _)]),
	void cv_ovis_WindowScene_setEntityProperty_const_StringR_int_const_ScalarR(cv::ovis::WindowScene* instance, const char* name, int prop, const cv::Scalar* value, ResultVoid* ocvrs_return) {
		try {
			instance->setEntityProperty(std::string(name), prop, *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEntityProperty(const String &, int, OutputArray)(InString, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:148
	// ("cv::ovis::WindowScene::getEntityProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::_OutputArray*"]), _)]),
	void cv_ovis_WindowScene_getEntityProperty_const_StringR_int_const__OutputArrayR(cv::ovis::WindowScene* instance, const char* name, int prop, const cv::_OutputArray* value, ResultVoid* ocvrs_return) {
		try {
			instance->getEntityProperty(std::string(name), prop, *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createCameraEntity(const String &, InputArray, const Size &, float, InputArray, InputArray, const Scalar &)(InString, InputArray, SimpleClass, Primitive, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:163
	// ("cv::ovis::WindowScene::createCameraEntity", vec![(pred!(mut, ["name", "K", "imsize", "zFar", "tvec", "rot", "color"], ["const cv::String*", "const cv::_InputArray*", "const cv::Size*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
	void cv_ovis_WindowScene_createCameraEntity_const_StringR_const__InputArrayR_const_SizeR_float_const__InputArrayR_const__InputArrayR_const_ScalarR(cv::ovis::WindowScene* instance, const char* name, const cv::_InputArray* K, const cv::Size* imsize, float zFar, const cv::_InputArray* tvec, const cv::_InputArray* rot, const cv::Scalar* color, Result<cv::Rect2d>* ocvrs_return) {
		try {
			cv::Rect2d ret = instance->createCameraEntity(std::string(name), *K, *imsize, zFar, *tvec, *rot, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::createCameraEntity(InString, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:163
	// ("cv::ovis::WindowScene::createCameraEntity", vec![(pred!(mut, ["name", "K", "imsize", "zFar"], ["const cv::String*", "const cv::_InputArray*", "const cv::Size*", "float"]), _)]),
	void cv_ovis_WindowScene_createCameraEntity_const_StringR_const__InputArrayR_const_SizeR_float(cv::ovis::WindowScene* instance, const char* name, const cv::_InputArray* K, const cv::Size* imsize, float zFar, Result<cv::Rect2d>* ocvrs_return) {
		try {
			cv::Rect2d ret = instance->createCameraEntity(std::string(name), *K, *imsize, zFar);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLightEntity(const String &, InputArray, InputArray, const Scalar &, const Scalar &)(InString, InputArray, InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:176
	// ("cv::ovis::WindowScene::createLightEntity", vec![(pred!(mut, ["name", "tvec", "rot", "diffuseColor", "specularColor"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Scalar*", "const cv::Scalar*"]), _)]),
	void cv_ovis_WindowScene_createLightEntity_const_StringR_const__InputArrayR_const__InputArrayR_const_ScalarR_const_ScalarR(cv::ovis::WindowScene* instance, const char* name, const cv::_InputArray* tvec, const cv::_InputArray* rot, const cv::Scalar* diffuseColor, const cv::Scalar* specularColor, ResultVoid* ocvrs_return) {
		try {
			instance->createLightEntity(std::string(name), *tvec, *rot, *diffuseColor, *specularColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::createLightEntity(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:176
	// ("cv::ovis::WindowScene::createLightEntity", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
	void cv_ovis_WindowScene_createLightEntity_const_StringR(cv::ovis::WindowScene* instance, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->createLightEntity(std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateEntityPose(const String &, InputArray, InputArray)(InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:187
	// ("cv::ovis::WindowScene::updateEntityPose", vec![(pred!(mut, ["name", "tvec", "rot"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ovis_WindowScene_updateEntityPose_const_StringR_const__InputArrayR_const__InputArrayR(cv::ovis::WindowScene* instance, const char* name, const cv::_InputArray* tvec, const cv::_InputArray* rot, ResultVoid* ocvrs_return) {
		try {
			instance->updateEntityPose(std::string(name), *tvec, *rot);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::updateEntityPose(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:187
	// ("cv::ovis::WindowScene::updateEntityPose", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
	void cv_ovis_WindowScene_updateEntityPose_const_StringR(cv::ovis::WindowScene* instance, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->updateEntityPose(std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEntityPose(const String &, InputArray, InputArray, bool)(InString, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:197
	// ("cv::ovis::WindowScene::setEntityPose", vec![(pred!(mut, ["name", "tvec", "rot", "invert"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
	void cv_ovis_WindowScene_setEntityPose_const_StringR_const__InputArrayR_const__InputArrayR_bool(cv::ovis::WindowScene* instance, const char* name, const cv::_InputArray* tvec, const cv::_InputArray* rot, bool invert, ResultVoid* ocvrs_return) {
		try {
			instance->setEntityPose(std::string(name), *tvec, *rot, invert);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::setEntityPose(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:197
	// ("cv::ovis::WindowScene::setEntityPose", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
	void cv_ovis_WindowScene_setEntityPose_const_StringR(cv::ovis::WindowScene* instance, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->setEntityPose(std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEntityPose(const String &, OutputArray, OutputArray, bool)(InString, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:207
	// ("cv::ovis::WindowScene::getEntityPose", vec![(pred!(mut, ["name", "R", "tvec", "invert"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_ovis_WindowScene_getEntityPose_const_StringR_const__OutputArrayR_const__OutputArrayR_bool(cv::ovis::WindowScene* instance, const char* name, const cv::_OutputArray* R, const cv::_OutputArray* tvec, bool invert, ResultVoid* ocvrs_return) {
		try {
			instance->getEntityPose(std::string(name), *R, *tvec, invert);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::getEntityPose(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:207
	// ("cv::ovis::WindowScene::getEntityPose", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
	void cv_ovis_WindowScene_getEntityPose_const_StringR(cv::ovis::WindowScene* instance, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->getEntityPose(std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEntityAnimations(const String &, std::vector<String> &)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:215
	// ("cv::ovis::WindowScene::getEntityAnimations", vec![(pred!(mut, ["name", "out"], ["const cv::String*", "std::vector<cv::String>*"]), _)]),
	void cv_ovis_WindowScene_getEntityAnimations_const_StringR_vectorLStringGR(cv::ovis::WindowScene* instance, const char* name, std::vector<cv::String>* out, ResultVoid* ocvrs_return) {
		try {
			instance->getEntityAnimations(std::string(name), *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// playEntityAnimation(const String &, const String &, bool)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:224
	// ("cv::ovis::WindowScene::playEntityAnimation", vec![(pred!(mut, ["name", "animname", "loop"], ["const cv::String*", "const cv::String*", "bool"]), _)]),
	void cv_ovis_WindowScene_playEntityAnimation_const_StringR_const_StringR_bool(cv::ovis::WindowScene* instance, const char* name, const char* animname, bool loop, ResultVoid* ocvrs_return) {
		try {
			instance->playEntityAnimation(std::string(name), std::string(animname), loop);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::playEntityAnimation(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:224
	// ("cv::ovis::WindowScene::playEntityAnimation", vec![(pred!(mut, ["name", "animname"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ovis_WindowScene_playEntityAnimation_const_StringR_const_StringR(cv::ovis::WindowScene* instance, const char* name, const char* animname, ResultVoid* ocvrs_return) {
		try {
			instance->playEntityAnimation(std::string(name), std::string(animname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stopEntityAnimation(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:232
	// ("cv::ovis::WindowScene::stopEntityAnimation", vec![(pred!(mut, ["name", "animname"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ovis_WindowScene_stopEntityAnimation_const_StringR_const_StringR(cv::ovis::WindowScene* instance, const char* name, const char* animname, ResultVoid* ocvrs_return) {
		try {
			instance->stopEntityAnimation(std::string(name), std::string(animname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScreenshot(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:237
	// ("cv::ovis::WindowScene::getScreenshot", vec![(pred!(mut, ["frame"], ["const cv::_OutputArray*"]), _)]),
	void cv_ovis_WindowScene_getScreenshot_const__OutputArrayR(cv::ovis::WindowScene* instance, const cv::_OutputArray* frame, ResultVoid* ocvrs_return) {
		try {
			instance->getScreenshot(*frame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCompositorTexture(const String &, const String &, OutputArray, int)(InString, InString, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:246
	// ("cv::ovis::WindowScene::getCompositorTexture", vec![(pred!(mut, ["compname", "texname", "out", "mrtIndex"], ["const cv::String*", "const cv::String*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ovis_WindowScene_getCompositorTexture_const_StringR_const_StringR_const__OutputArrayR_int(cv::ovis::WindowScene* instance, const char* compname, const char* texname, const cv::_OutputArray* out, int mrtIndex, ResultVoid* ocvrs_return) {
		try {
			instance->getCompositorTexture(std::string(compname), std::string(texname), *out, mrtIndex);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::getCompositorTexture(InString, InString, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:246
	// ("cv::ovis::WindowScene::getCompositorTexture", vec![(pred!(mut, ["compname", "texname", "out"], ["const cv::String*", "const cv::String*", "const cv::_OutputArray*"]), _)]),
	void cv_ovis_WindowScene_getCompositorTexture_const_StringR_const_StringR_const__OutputArrayR(cv::ovis::WindowScene* instance, const char* compname, const char* texname, const cv::_OutputArray* out, ResultVoid* ocvrs_return) {
		try {
			instance->getCompositorTexture(std::string(compname), std::string(texname), *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDepth(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:254
	// ("cv::ovis::WindowScene::getDepth", vec![(pred!(mut, ["depth"], ["const cv::_OutputArray*"]), _)]),
	void cv_ovis_WindowScene_getDepth_const__OutputArrayR(cv::ovis::WindowScene* instance, const cv::_OutputArray* depth, ResultVoid* ocvrs_return) {
		try {
			instance->getDepth(*depth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fixCameraYawAxis(bool, InputArray)(Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:263
	// ("cv::ovis::WindowScene::fixCameraYawAxis", vec![(pred!(mut, ["useFixed", "up"], ["bool", "const cv::_InputArray*"]), _)]),
	void cv_ovis_WindowScene_fixCameraYawAxis_bool_const__InputArrayR(cv::ovis::WindowScene* instance, bool useFixed, const cv::_InputArray* up, ResultVoid* ocvrs_return) {
		try {
			instance->fixCameraYawAxis(useFixed, *up);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::fixCameraYawAxis(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:263
	// ("cv::ovis::WindowScene::fixCameraYawAxis", vec![(pred!(mut, ["useFixed"], ["bool"]), _)]),
	void cv_ovis_WindowScene_fixCameraYawAxis_bool(cv::ovis::WindowScene* instance, bool useFixed, ResultVoid* ocvrs_return) {
		try {
			instance->fixCameraYawAxis(useFixed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraPose(InputArray, InputArray, bool)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:271
	// ("cv::ovis::WindowScene::setCameraPose", vec![(pred!(mut, ["tvec", "rot", "invert"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
	void cv_ovis_WindowScene_setCameraPose_const__InputArrayR_const__InputArrayR_bool(cv::ovis::WindowScene* instance, const cv::_InputArray* tvec, const cv::_InputArray* rot, bool invert, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraPose(*tvec, *rot, invert);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::setCameraPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:271
	// ("cv::ovis::WindowScene::setCameraPose", vec![(pred!(mut, [], []), _)]),
	void cv_ovis_WindowScene_setCameraPose(cv::ovis::WindowScene* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraPose();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraLookAt(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:279
	// ("cv::ovis::WindowScene::setCameraLookAt", vec![(pred!(mut, ["target", "offset"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_ovis_WindowScene_setCameraLookAt_const_StringR_const__InputArrayR(cv::ovis::WindowScene* instance, const char* target, const cv::_InputArray* offset, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraLookAt(std::string(target), *offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::setCameraLookAt(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:279
	// ("cv::ovis::WindowScene::setCameraLookAt", vec![(pred!(mut, ["target"], ["const cv::String*"]), _)]),
	void cv_ovis_WindowScene_setCameraLookAt_const_StringR(cv::ovis::WindowScene* instance, const char* target, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraLookAt(std::string(target));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEntityLookAt(const String &, const String &, InputArray)(InString, InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:288
	// ("cv::ovis::WindowScene::setEntityLookAt", vec![(pred!(mut, ["origin", "target", "offset"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_ovis_WindowScene_setEntityLookAt_const_StringR_const_StringR_const__InputArrayR(cv::ovis::WindowScene* instance, const char* origin, const char* target, const cv::_InputArray* offset, ResultVoid* ocvrs_return) {
		try {
			instance->setEntityLookAt(std::string(origin), std::string(target), *offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::setEntityLookAt(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:288
	// ("cv::ovis::WindowScene::setEntityLookAt", vec![(pred!(mut, ["origin", "target"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ovis_WindowScene_setEntityLookAt_const_StringR_const_StringR(cv::ovis::WindowScene* instance, const char* origin, const char* target, ResultVoid* ocvrs_return) {
		try {
			instance->setEntityLookAt(std::string(origin), std::string(target));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraPose(OutputArray, OutputArray, bool)(OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:296
	// ("cv::ovis::WindowScene::getCameraPose", vec![(pred!(mut, ["R", "tvec", "invert"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_ovis_WindowScene_getCameraPose_const__OutputArrayR_const__OutputArrayR_bool(cv::ovis::WindowScene* instance, const cv::_OutputArray* R, const cv::_OutputArray* tvec, bool invert, ResultVoid* ocvrs_return) {
		try {
			instance->getCameraPose(*R, *tvec, invert);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::getCameraPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:296
	// ("cv::ovis::WindowScene::getCameraPose", vec![(pred!(mut, [], []), _)]),
	void cv_ovis_WindowScene_getCameraPose(cv::ovis::WindowScene* instance, ResultVoid* ocvrs_return) {
		try {
			instance->getCameraPose();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraIntrinsics(InputArray, const Size &, float, float)(InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:308
	// ("cv::ovis::WindowScene::setCameraIntrinsics", vec![(pred!(mut, ["K", "imsize", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::Size*", "float", "float"]), _)]),
	void cv_ovis_WindowScene_setCameraIntrinsics_const__InputArrayR_const_SizeR_float_float(cv::ovis::WindowScene* instance, const cv::_InputArray* K, const cv::Size* imsize, float zNear, float zFar, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraIntrinsics(*K, *imsize, zNear, zFar);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::setCameraIntrinsics(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:308
	// ("cv::ovis::WindowScene::setCameraIntrinsics", vec![(pred!(mut, ["K", "imsize"], ["const cv::_InputArray*", "const cv::Size*"]), _)]),
	void cv_ovis_WindowScene_setCameraIntrinsics_const__InputArrayR_const_SizeR(cv::ovis::WindowScene* instance, const cv::_InputArray* K, const cv::Size* imsize, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraIntrinsics(*K, *imsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:314
	// ("cv::ovis::WindowScene::update", vec![(pred!(mut, [], []), _)]),
	void cv_ovis_WindowScene_update(cv::ovis::WindowScene* instance, ResultVoid* ocvrs_return) {
		try {
			instance->update();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ovis::WindowScene::delete() generated
	// ("cv::ovis::WindowScene::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ovis_WindowScene_delete(cv::ovis::WindowScene* instance) {
			delete instance;
	}

}

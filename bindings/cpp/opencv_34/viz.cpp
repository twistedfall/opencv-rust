#include "ocvrs_common.hpp"
#include <opencv2/viz.hpp>
#include "viz_types.hpp"

extern "C" {
	Result_void cv_viz_computeNormals_const_MeshR_const__OutputArrayR(const cv::viz::Mesh* mesh, const cv::_OutputArray* normals) {
		try {
			cv::viz::computeNormals(*mesh, *normals);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::viz::Viz3d*> cv_viz_getWindowByName_const_StringR(const char* window_name) {
		try {
			cv::viz::Viz3d ret = cv::viz::getWindowByName(cv::String(window_name));
			return Ok(new cv::viz::Viz3d(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Viz3d*>))
	}
	
	Result<cv::viz::Viz3d*> cv_viz_imshow_const_StringR_const__InputArrayR_const_SizeR(const char* window_name, const cv::_InputArray* image, const cv::Size* window_size) {
		try {
			cv::viz::Viz3d ret = cv::viz::imshow(cv::String(window_name), *image, *window_size);
			return Ok(new cv::viz::Viz3d(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Viz3d*>))
	}
	
	Result<cv::Affine3d> cv_viz_makeCameraPose_const_Vec3dR_const_Vec3dR_const_Vec3dR(const cv::Vec3d* position, const cv::Vec3d* focal_point, const cv::Vec3d* y_dir) {
		try {
			cv::Affine3d ret = cv::viz::makeCameraPose(*position, *focal_point, *y_dir);
			return Ok<cv::Affine3d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3d>))
	}
	
	Result<cv::Affine3d> cv_viz_makeTransformToGlobal_const_Vec3dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(const cv::Vec3d* axis_x, const cv::Vec3d* axis_y, const cv::Vec3d* axis_z, const cv::Vec3d* origin) {
		try {
			cv::Affine3d ret = cv::viz::makeTransformToGlobal(*axis_x, *axis_y, *axis_z, *origin);
			return Ok<cv::Affine3d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3d>))
	}
	
	Result<cv::Mat*> cv_viz_readCloud_const_StringR_const__OutputArrayR_const__OutputArrayR(const char* file, const cv::_OutputArray* colors, const cv::_OutputArray* normals) {
		try {
			cv::Mat ret = cv::viz::readCloud(cv::String(file), *colors, *normals);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::viz::Mesh*> cv_viz_readMesh_const_StringR(const char* file) {
		try {
			cv::viz::Mesh ret = cv::viz::readMesh(cv::String(file));
			return Ok(new cv::viz::Mesh(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Mesh*>))
	}
	
	Result<bool> cv_viz_readPose_const_StringR_Affine3dR_const_StringR(const char* file, cv::Affine3d* pose, const char* tag) {
		try {
			bool ret = cv::viz::readPose(cv::String(file), *pose, cv::String(tag));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_viz_readTrajectory_const__OutputArrayR_const_StringR_int_int_const_StringR(const cv::_OutputArray* traj, const char* files_format, int start, int end, const char* tag) {
		try {
			cv::viz::readTrajectory(*traj, cv::String(files_format), start, end, cv::String(tag));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_unregisterAllWindows() {
		try {
			cv::viz::unregisterAllWindows();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_writeCloud_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR_bool(const char* file, const cv::_InputArray* cloud, const cv::_InputArray* colors, const cv::_InputArray* normals, bool binary) {
		try {
			cv::viz::writeCloud(cv::String(file), *cloud, *colors, *normals, binary);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_writePose_const_StringR_const_Affine3dR_const_StringR(const char* file, const cv::Affine3d* pose, const char* tag) {
		try {
			cv::viz::writePose(cv::String(file), *pose, cv::String(tag));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_writeTrajectory_const__InputArrayR_const_StringR_int_const_StringR(const cv::_InputArray* traj, const char* files_format, int start, const char* tag) {
		try {
			cv::viz::writeTrajectory(*traj, cv::String(files_format), start, cv::String(tag));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Camera_delete(cv::viz::Camera* instance) {
		delete instance;
	}
	Result<cv::viz::Camera*> cv_viz_Camera_Camera_double_double_double_double_const_SizeR(double fx, double fy, double cx, double cy, const cv::Size* window_size) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(fx, fy, cx, cy, *window_size);
			return Ok<cv::viz::Camera*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	Result<cv::viz::Camera*> cv_viz_Camera_Camera_const_Vec2dR_const_SizeR(const cv::Vec2d* fov, const cv::Size* window_size) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(*fov, *window_size);
			return Ok<cv::viz::Camera*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	Result<cv::viz::Camera*> cv_viz_Camera_Camera_const_Matx33dR_const_SizeR(const cv::Matx33d* K, const cv::Size* window_size) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(*K, *window_size);
			return Ok<cv::viz::Camera*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	Result<cv::viz::Camera*> cv_viz_Camera_Camera_const_Matx44dR_const_SizeR(const cv::Matx44d* proj, const cv::Size* window_size) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(*proj, *window_size);
			return Ok<cv::viz::Camera*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	Result<const cv::Vec2d> cv_viz_Camera_getClip_const(const cv::viz::Camera* instance) {
		try {
			const cv::Vec2d ret = instance->getClip();
			return Ok<const cv::Vec2d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Vec2d>))
	}
	
	Result_void cv_viz_Camera_setClip_const_Vec2dR(cv::viz::Camera* instance, const cv::Vec2d* clip) {
		try {
			instance->setClip(*clip);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const cv::Size> cv_viz_Camera_getWindowSize_const(const cv::viz::Camera* instance) {
		try {
			const cv::Size ret = instance->getWindowSize();
			return Ok<const cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Size>))
	}
	
	Result_void cv_viz_Camera_setWindowSize_const_SizeR(cv::viz::Camera* instance, const cv::Size* window_size) {
		try {
			instance->setWindowSize(*window_size);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const cv::Vec2d> cv_viz_Camera_getFov_const(const cv::viz::Camera* instance) {
		try {
			const cv::Vec2d ret = instance->getFov();
			return Ok<const cv::Vec2d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Vec2d>))
	}
	
	Result_void cv_viz_Camera_setFov_const_Vec2dR(cv::viz::Camera* instance, const cv::Vec2d* fov) {
		try {
			instance->setFov(*fov);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const cv::Vec2d> cv_viz_Camera_getPrincipalPoint_const(const cv::viz::Camera* instance) {
		try {
			const cv::Vec2d ret = instance->getPrincipalPoint();
			return Ok<const cv::Vec2d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Vec2d>))
	}
	
	Result<const cv::Vec2d> cv_viz_Camera_getFocalLength_const(const cv::viz::Camera* instance) {
		try {
			const cv::Vec2d ret = instance->getFocalLength();
			return Ok<const cv::Vec2d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Vec2d>))
	}
	
	Result_void cv_viz_Camera_computeProjectionMatrix_const_Matx44dR(const cv::viz::Camera* instance, cv::Matx44d* proj) {
		try {
			instance->computeProjectionMatrix(*proj);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::viz::Camera*> cv_viz_Camera_KinectCamera_const_SizeR(const cv::Size* window_size) {
		try {
			cv::viz::Camera ret = cv::viz::Camera::KinectCamera(*window_size);
			return Ok(new cv::viz::Camera(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	void cv_Color_delete(cv::viz::Color* instance) {
		delete instance;
	}
	Result<cv::viz::Color*> cv_viz_Color_Color() {
		try {
			cv::viz::Color* ret = new cv::viz::Color();
			return Ok<cv::viz::Color*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_Color_double(double gray) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(gray);
			return Ok<cv::viz::Color*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_Color_double_double_double(double blue, double green, double red) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(blue, green, red);
			return Ok<cv::viz::Color*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_Color_const_ScalarR(const cv::Scalar* color) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(*color);
			return Ok<cv::viz::Color*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::Vec3b> cv_viz_Color_operator_cv_Vec3b_const(const cv::viz::Color* instance) {
		try {
			cv::Vec3b ret = instance->operator cv::Vec3b();
			return Ok<cv::Vec3b>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3b>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_black() {
		try {
			cv::viz::Color ret = cv::viz::Color::black();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_blue() {
		try {
			cv::viz::Color ret = cv::viz::Color::blue();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_green() {
		try {
			cv::viz::Color ret = cv::viz::Color::green();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_red() {
		try {
			cv::viz::Color ret = cv::viz::Color::red();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_cyan() {
		try {
			cv::viz::Color ret = cv::viz::Color::cyan();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_yellow() {
		try {
			cv::viz::Color ret = cv::viz::Color::yellow();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_magenta() {
		try {
			cv::viz::Color ret = cv::viz::Color::magenta();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_white() {
		try {
			cv::viz::Color ret = cv::viz::Color::white();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_gray() {
		try {
			cv::viz::Color ret = cv::viz::Color::gray();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_silver() {
		try {
			cv::viz::Color ret = cv::viz::Color::silver();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_mlab() {
		try {
			cv::viz::Color ret = cv::viz::Color::mlab();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_navy() {
		try {
			cv::viz::Color ret = cv::viz::Color::navy();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_maroon() {
		try {
			cv::viz::Color ret = cv::viz::Color::maroon();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_teal() {
		try {
			cv::viz::Color ret = cv::viz::Color::teal();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_olive() {
		try {
			cv::viz::Color ret = cv::viz::Color::olive();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_purple() {
		try {
			cv::viz::Color ret = cv::viz::Color::purple();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_azure() {
		try {
			cv::viz::Color ret = cv::viz::Color::azure();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_chartreuse() {
		try {
			cv::viz::Color ret = cv::viz::Color::chartreuse();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_rose() {
		try {
			cv::viz::Color ret = cv::viz::Color::rose();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_lime() {
		try {
			cv::viz::Color ret = cv::viz::Color::lime();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_gold() {
		try {
			cv::viz::Color ret = cv::viz::Color::gold();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_orange() {
		try {
			cv::viz::Color ret = cv::viz::Color::orange();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_orange_red() {
		try {
			cv::viz::Color ret = cv::viz::Color::orange_red();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_indigo() {
		try {
			cv::viz::Color ret = cv::viz::Color::indigo();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_brown() {
		try {
			cv::viz::Color ret = cv::viz::Color::brown();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_apricot() {
		try {
			cv::viz::Color ret = cv::viz::Color::apricot();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_pink() {
		try {
			cv::viz::Color ret = cv::viz::Color::pink();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_raspberry() {
		try {
			cv::viz::Color ret = cv::viz::Color::raspberry();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_cherry() {
		try {
			cv::viz::Color ret = cv::viz::Color::cherry();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_violet() {
		try {
			cv::viz::Color ret = cv::viz::Color::violet();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_amethyst() {
		try {
			cv::viz::Color ret = cv::viz::Color::amethyst();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_bluberry() {
		try {
			cv::viz::Color ret = cv::viz::Color::bluberry();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_celestial_blue() {
		try {
			cv::viz::Color ret = cv::viz::Color::celestial_blue();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_turquoise() {
		try {
			cv::viz::Color ret = cv::viz::Color::turquoise();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::Color*> cv_viz_Color_not_set() {
		try {
			cv::viz::Color ret = cv::viz::Color::not_set();
			return Ok(new cv::viz::Color(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	Result<cv::viz::KeyboardEvent::Action> cv_viz_KeyboardEvent_getPropAction_const(const cv::viz::KeyboardEvent* instance) {
		try {
			cv::viz::KeyboardEvent::Action ret = instance->action;
			return Ok<cv::viz::KeyboardEvent::Action>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::KeyboardEvent::Action>))
	}
	
	Result_void cv_viz_KeyboardEvent_setPropAction_Action(cv::viz::KeyboardEvent* instance, cv::viz::KeyboardEvent::Action val) {
		try {
			instance->action = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_viz_KeyboardEvent_getPropSymbol_const(const cv::viz::KeyboardEvent* instance) {
		try {
			cv::String ret = instance->symbol;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_viz_KeyboardEvent_setPropSymbol_String(cv::viz::KeyboardEvent* instance, char* val) {
		try {
			instance->symbol = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned char> cv_viz_KeyboardEvent_getPropCode_const(const cv::viz::KeyboardEvent* instance) {
		try {
			unsigned char ret = instance->code;
			return Ok<unsigned char>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char>))
	}
	
	Result_void cv_viz_KeyboardEvent_setPropCode_unsigned_char(cv::viz::KeyboardEvent* instance, unsigned char val) {
		try {
			instance->code = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_viz_KeyboardEvent_getPropModifiers_const(const cv::viz::KeyboardEvent* instance) {
		try {
			int ret = instance->modifiers;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_viz_KeyboardEvent_setPropModifiers_int(cv::viz::KeyboardEvent* instance, int val) {
		try {
			instance->modifiers = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_KeyboardEvent_delete(cv::viz::KeyboardEvent* instance) {
		delete instance;
	}
	Result<cv::viz::KeyboardEvent*> cv_viz_KeyboardEvent_KeyboardEvent_Action_const_StringR_unsigned_char_int(cv::viz::KeyboardEvent::Action action, const char* symbol, unsigned char code, int modifiers) {
		try {
			cv::viz::KeyboardEvent* ret = new cv::viz::KeyboardEvent(action, cv::String(symbol), code, modifiers);
			return Ok<cv::viz::KeyboardEvent*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::KeyboardEvent*>))
	}
	
	Result<cv::Mat*> cv_viz_Mesh_getPropCloud(cv::viz::Mesh* instance) {
		try {
			cv::Mat ret = instance->cloud;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_viz_Mesh_setPropCloud_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
		try {
			instance->cloud = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_viz_Mesh_getPropColors(cv::viz::Mesh* instance) {
		try {
			cv::Mat ret = instance->colors;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_viz_Mesh_setPropColors_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
		try {
			instance->colors = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_viz_Mesh_getPropNormals(cv::viz::Mesh* instance) {
		try {
			cv::Mat ret = instance->normals;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_viz_Mesh_setPropNormals_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
		try {
			instance->normals = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_viz_Mesh_getPropPolygons(cv::viz::Mesh* instance) {
		try {
			cv::Mat ret = instance->polygons;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_viz_Mesh_setPropPolygons_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
		try {
			instance->polygons = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_viz_Mesh_getPropTexture(cv::viz::Mesh* instance) {
		try {
			cv::Mat ret = instance->texture;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_viz_Mesh_setPropTexture_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
		try {
			instance->texture = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_viz_Mesh_getPropTcoords(cv::viz::Mesh* instance) {
		try {
			cv::Mat ret = instance->tcoords;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_viz_Mesh_setPropTcoords_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
		try {
			instance->tcoords = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Mesh_delete(cv::viz::Mesh* instance) {
		delete instance;
	}
	Result<cv::viz::Mesh*> cv_viz_Mesh_load_const_StringR_int(const char* file, int type) {
		try {
			cv::viz::Mesh ret = cv::viz::Mesh::load(cv::String(file), type);
			return Ok(new cv::viz::Mesh(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Mesh*>))
	}
	
	Result<cv::viz::MouseEvent::Type> cv_viz_MouseEvent_getPropType_const(const cv::viz::MouseEvent* instance) {
		try {
			cv::viz::MouseEvent::Type ret = instance->type;
			return Ok<cv::viz::MouseEvent::Type>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::MouseEvent::Type>))
	}
	
	Result_void cv_viz_MouseEvent_setPropType_Type(cv::viz::MouseEvent* instance, cv::viz::MouseEvent::Type val) {
		try {
			instance->type = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::viz::MouseEvent::MouseButton> cv_viz_MouseEvent_getPropButton_const(const cv::viz::MouseEvent* instance) {
		try {
			cv::viz::MouseEvent::MouseButton ret = instance->button;
			return Ok<cv::viz::MouseEvent::MouseButton>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::MouseEvent::MouseButton>))
	}
	
	Result_void cv_viz_MouseEvent_setPropButton_MouseButton(cv::viz::MouseEvent* instance, cv::viz::MouseEvent::MouseButton val) {
		try {
			instance->button = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Point> cv_viz_MouseEvent_getPropPointer_const(const cv::viz::MouseEvent* instance) {
		try {
			cv::Point ret = instance->pointer;
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result_void cv_viz_MouseEvent_setPropPointer_Point(cv::viz::MouseEvent* instance, cv::Point* val) {
		try {
			instance->pointer = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_viz_MouseEvent_getPropModifiers_const(const cv::viz::MouseEvent* instance) {
		try {
			int ret = instance->modifiers;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_viz_MouseEvent_setPropModifiers_int(cv::viz::MouseEvent* instance, int val) {
		try {
			instance->modifiers = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MouseEvent_delete(cv::viz::MouseEvent* instance) {
		delete instance;
	}
	Result<cv::viz::MouseEvent*> cv_viz_MouseEvent_MouseEvent_const_TypeR_const_MouseButtonR_const_PointR_int(const cv::viz::MouseEvent::Type* type, const cv::viz::MouseEvent::MouseButton* button, const cv::Point* pointer, int modifiers) {
		try {
			cv::viz::MouseEvent* ret = new cv::viz::MouseEvent(*type, *button, *pointer, modifiers);
			return Ok<cv::viz::MouseEvent*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::MouseEvent*>))
	}
	
	void cv_Viz3d_delete(cv::viz::Viz3d* instance) {
		delete instance;
	}
	Result<cv::viz::Viz3d*> cv_viz_Viz3d_Viz3d_const_StringR(const char* window_name) {
		try {
			cv::viz::Viz3d* ret = new cv::viz::Viz3d(cv::String(window_name));
			return Ok<cv::viz::Viz3d*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Viz3d*>))
	}
	
	Result<cv::viz::Viz3d*> cv_viz_Viz3d_Viz3d_const_Viz3dR(const cv::viz::Viz3d* unnamed) {
		try {
			cv::viz::Viz3d* ret = new cv::viz::Viz3d(*unnamed);
			return Ok<cv::viz::Viz3d*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Viz3d*>))
	}
	
	Result_void cv_viz_Viz3d_showWidget_const_StringR_const_WidgetR_const_Affine3dR(cv::viz::Viz3d* instance, const char* id, const cv::viz::Widget* widget, const cv::Affine3d* pose) {
		try {
			instance->showWidget(cv::String(id), *widget, *pose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_removeWidget_const_StringR(cv::viz::Viz3d* instance, const char* id) {
		try {
			instance->removeWidget(cv::String(id));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::viz::Widget*> cv_viz_Viz3d_getWidget_const_const_StringR(const cv::viz::Viz3d* instance, const char* id) {
		try {
			cv::viz::Widget ret = instance->getWidget(cv::String(id));
			return Ok(new cv::viz::Widget(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget*>))
	}
	
	Result_void cv_viz_Viz3d_removeAllWidgets(cv::viz::Viz3d* instance) {
		try {
			instance->removeAllWidgets();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_showImage_const__InputArrayR_const_SizeR(cv::viz::Viz3d* instance, const cv::_InputArray* image, const cv::Size* window_size) {
		try {
			instance->showImage(*image, *window_size);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_setWidgetPose_const_StringR_const_Affine3dR(cv::viz::Viz3d* instance, const char* id, const cv::Affine3d* pose) {
		try {
			instance->setWidgetPose(cv::String(id), *pose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_updateWidgetPose_const_StringR_const_Affine3dR(cv::viz::Viz3d* instance, const char* id, const cv::Affine3d* pose) {
		try {
			instance->updateWidgetPose(cv::String(id), *pose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Affine3d> cv_viz_Viz3d_getWidgetPose_const_const_StringR(const cv::viz::Viz3d* instance, const char* id) {
		try {
			cv::Affine3d ret = instance->getWidgetPose(cv::String(id));
			return Ok<cv::Affine3d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3d>))
	}
	
	Result_void cv_viz_Viz3d_setCamera_const_CameraR(cv::viz::Viz3d* instance, const cv::viz::Camera* camera) {
		try {
			instance->setCamera(*camera);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::viz::Camera*> cv_viz_Viz3d_getCamera_const(const cv::viz::Viz3d* instance) {
		try {
			cv::viz::Camera ret = instance->getCamera();
			return Ok(new cv::viz::Camera(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	Result<cv::Affine3d> cv_viz_Viz3d_getViewerPose(cv::viz::Viz3d* instance) {
		try {
			cv::Affine3d ret = instance->getViewerPose();
			return Ok<cv::Affine3d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3d>))
	}
	
	Result_void cv_viz_Viz3d_setViewerPose_const_Affine3dR(cv::viz::Viz3d* instance, const cv::Affine3d* pose) {
		try {
			instance->setViewerPose(*pose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_resetCameraViewpoint_const_StringR(cv::viz::Viz3d* instance, const char* id) {
		try {
			instance->resetCameraViewpoint(cv::String(id));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_resetCamera(cv::viz::Viz3d* instance) {
		try {
			instance->resetCamera();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_convertToWindowCoordinates_const_Point3dR_Point3dR(cv::viz::Viz3d* instance, const cv::Point3d* pt, cv::Point3d* window_coord) {
		try {
			instance->convertToWindowCoordinates(*pt, *window_coord);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_converTo3DRay_const_Point3dR_Point3dR_Vec3dR(cv::viz::Viz3d* instance, const cv::Point3d* window_coord, cv::Point3d* origin, cv::Vec3d* direction) {
		try {
			instance->converTo3DRay(*window_coord, *origin, *direction);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_viz_Viz3d_getWindowSize_const(const cv::viz::Viz3d* instance) {
		try {
			cv::Size ret = instance->getWindowSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_viz_Viz3d_setWindowSize_const_SizeR(cv::viz::Viz3d* instance, const cv::Size* window_size) {
		try {
			instance->setWindowSize(*window_size);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_viz_Viz3d_getWindowName_const(const cv::viz::Viz3d* instance) {
		try {
			cv::String ret = instance->getWindowName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::Mat*> cv_viz_Viz3d_getScreenshot_const(const cv::viz::Viz3d* instance) {
		try {
			cv::Mat ret = instance->getScreenshot();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_viz_Viz3d_saveScreenshot_const_StringR(cv::viz::Viz3d* instance, const char* file) {
		try {
			instance->saveScreenshot(cv::String(file));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_setWindowPosition_const_PointR(cv::viz::Viz3d* instance, const cv::Point* window_position) {
		try {
			instance->setWindowPosition(*window_position);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_setFullScreen_bool(cv::viz::Viz3d* instance, bool mode) {
		try {
			instance->setFullScreen(mode);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_setBackgroundColor_const_ColorR_const_ColorR(cv::viz::Viz3d* instance, const cv::viz::Viz3d::Color* color, const cv::viz::Viz3d::Color* color2) {
		try {
			instance->setBackgroundColor(*color, *color2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_setBackgroundTexture_const__InputArrayR(cv::viz::Viz3d* instance, const cv::_InputArray* image) {
		try {
			instance->setBackgroundTexture(*image);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_setBackgroundMeshLab(cv::viz::Viz3d* instance) {
		try {
			instance->setBackgroundMeshLab();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_spin(cv::viz::Viz3d* instance) {
		try {
			instance->spin();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_spinOnce_int_bool(cv::viz::Viz3d* instance, int time, bool force_redraw) {
		try {
			instance->spinOnce(time, force_redraw);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_setOffScreenRendering(cv::viz::Viz3d* instance) {
		try {
			instance->setOffScreenRendering();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_removeAllLights(cv::viz::Viz3d* instance) {
		try {
			instance->removeAllLights();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_addLight_Vec3d_Vec3d_Color_Color_Color_Color(cv::viz::Viz3d* instance, cv::Vec3d* position, cv::Vec3d* focalPoint, cv::viz::Viz3d::Color* color, cv::viz::Viz3d::Color* diffuseColor, cv::viz::Viz3d::Color* ambientColor, cv::viz::Viz3d::Color* specularColor) {
		try {
			instance->addLight(*position, *focalPoint, *color, *diffuseColor, *ambientColor, *specularColor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_viz_Viz3d_wasStopped_const(const cv::viz::Viz3d* instance) {
		try {
			bool ret = instance->wasStopped();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_viz_Viz3d_close(cv::viz::Viz3d* instance) {
		try {
			instance->close();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_registerKeyboardCallback_KeyboardCallback_voidX(cv::viz::Viz3d* instance, cv::viz::Viz3d::KeyboardCallback callback, void* cookie) {
		try {
			instance->registerKeyboardCallback(callback, cookie);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_registerMouseCallback_MouseCallback_voidX(cv::viz::Viz3d* instance, cv::viz::Viz3d::MouseCallback callback, void* cookie) {
		try {
			instance->registerMouseCallback(callback, cookie);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_setRenderingProperty_const_StringR_int_double(cv::viz::Viz3d* instance, const char* id, int property, double value) {
		try {
			instance->setRenderingProperty(cv::String(id), property, value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_viz_Viz3d_getRenderingProperty_const_StringR_int(cv::viz::Viz3d* instance, const char* id, int property) {
		try {
			double ret = instance->getRenderingProperty(cv::String(id), property);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_viz_Viz3d_setRepresentation_int(cv::viz::Viz3d* instance, int representation) {
		try {
			instance->setRepresentation(representation);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Viz3d_setGlobalWarnings_bool(cv::viz::Viz3d* instance, bool enabled) {
		try {
			instance->setGlobalWarnings(enabled);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_WArrow_delete(cv::viz::WArrow* instance) {
		delete instance;
	}
	Result<cv::viz::WArrow*> cv_viz_WArrow_WArrow_const_Point3dR_const_Point3dR_double_const_ColorR(const cv::Point3d* pt1, const cv::Point3d* pt2, double thickness, const cv::viz::Color* color) {
		try {
			cv::viz::WArrow* ret = new cv::viz::WArrow(*pt1, *pt2, thickness, *color);
			return Ok<cv::viz::WArrow*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WArrow*>))
	}
	
	void cv_WCameraPosition_delete(cv::viz::WCameraPosition* instance) {
		delete instance;
	}
	Result<cv::viz::WCameraPosition*> cv_viz_WCameraPosition_WCameraPosition_double(double scale) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(scale);
			return Ok<cv::viz::WCameraPosition*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCameraPosition*>))
	}
	
	Result<cv::viz::WCameraPosition*> cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_double_const_ColorR(const cv::Matx33d* K, double scale, const cv::viz::Color* color) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*K, scale, *color);
			return Ok<cv::viz::WCameraPosition*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCameraPosition*>))
	}
	
	Result<cv::viz::WCameraPosition*> cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_double_const_ColorR(const cv::Vec2d* fov, double scale, const cv::viz::Color* color) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*fov, scale, *color);
			return Ok<cv::viz::WCameraPosition*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCameraPosition*>))
	}
	
	Result<cv::viz::WCameraPosition*> cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_const__InputArrayR_double_const_ColorR(const cv::Matx33d* K, const cv::_InputArray* image, double scale, const cv::viz::Color* color) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*K, *image, scale, *color);
			return Ok<cv::viz::WCameraPosition*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCameraPosition*>))
	}
	
	Result<cv::viz::WCameraPosition*> cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_const__InputArrayR_double_const_ColorR(const cv::Vec2d* fov, const cv::_InputArray* image, double scale, const cv::viz::Color* color) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*fov, *image, scale, *color);
			return Ok<cv::viz::WCameraPosition*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCameraPosition*>))
	}
	
	void cv_WCircle_delete(cv::viz::WCircle* instance) {
		delete instance;
	}
	Result<cv::viz::WCircle*> cv_viz_WCircle_WCircle_double_double_const_ColorR(double radius, double thickness, const cv::viz::Color* color) {
		try {
			cv::viz::WCircle* ret = new cv::viz::WCircle(radius, thickness, *color);
			return Ok<cv::viz::WCircle*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCircle*>))
	}
	
	Result<cv::viz::WCircle*> cv_viz_WCircle_WCircle_double_const_Point3dR_const_Vec3dR_double_const_ColorR(double radius, const cv::Point3d* center, const cv::Vec3d* normal, double thickness, const cv::viz::Color* color) {
		try {
			cv::viz::WCircle* ret = new cv::viz::WCircle(radius, *center, *normal, thickness, *color);
			return Ok<cv::viz::WCircle*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCircle*>))
	}
	
	void cv_WCloud_delete(cv::viz::WCloud* instance) {
		delete instance;
	}
	Result<cv::viz::WCloud*> cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR(const cv::_InputArray* cloud, const cv::_InputArray* colors) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *colors);
			return Ok<cv::viz::WCloud*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloud*>))
	}
	
	Result<cv::viz::WCloud*> cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR(const cv::_InputArray* cloud, const cv::viz::Color* color) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *color);
			return Ok<cv::viz::WCloud*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloud*>))
	}
	
	Result<cv::viz::WCloud*> cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* cloud, const cv::_InputArray* colors, const cv::_InputArray* normals) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *colors, *normals);
			return Ok<cv::viz::WCloud*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloud*>))
	}
	
	Result<cv::viz::WCloud*> cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR_const__InputArrayR(const cv::_InputArray* cloud, const cv::viz::Color* color, const cv::_InputArray* normals) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *color, *normals);
			return Ok<cv::viz::WCloud*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloud*>))
	}
	
	void cv_WCloudCollection_delete(cv::viz::WCloudCollection* instance) {
		delete instance;
	}
	Result<cv::viz::WCloudCollection*> cv_viz_WCloudCollection_WCloudCollection() {
		try {
			cv::viz::WCloudCollection* ret = new cv::viz::WCloudCollection();
			return Ok<cv::viz::WCloudCollection*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloudCollection*>))
	}
	
	Result_void cv_viz_WCloudCollection_addCloud_const__InputArrayR_const__InputArrayR_const_Affine3dR(cv::viz::WCloudCollection* instance, const cv::_InputArray* cloud, const cv::_InputArray* colors, const cv::Affine3d* pose) {
		try {
			instance->addCloud(*cloud, *colors, *pose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_WCloudCollection_addCloud_const__InputArrayR_const_ColorR_const_Affine3dR(cv::viz::WCloudCollection* instance, const cv::_InputArray* cloud, const cv::viz::Color* color, const cv::Affine3d* pose) {
		try {
			instance->addCloud(*cloud, *color, *pose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_WCloudCollection_finalize(cv::viz::WCloudCollection* instance) {
		try {
			instance->finalize();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_WCloudNormals_delete(cv::viz::WCloudNormals* instance) {
		delete instance;
	}
	Result<cv::viz::WCloudNormals*> cv_viz_WCloudNormals_WCloudNormals_const__InputArrayR_const__InputArrayR_int_double_const_ColorR(const cv::_InputArray* cloud, const cv::_InputArray* normals, int level, double scale, const cv::viz::Color* color) {
		try {
			cv::viz::WCloudNormals* ret = new cv::viz::WCloudNormals(*cloud, *normals, level, scale, *color);
			return Ok<cv::viz::WCloudNormals*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloudNormals*>))
	}
	
	void cv_WCone_delete(cv::viz::WCone* instance) {
		delete instance;
	}
	Result<cv::viz::WCone*> cv_viz_WCone_WCone_double_double_int_const_ColorR(double length, double radius, int resolution, const cv::viz::Color* color) {
		try {
			cv::viz::WCone* ret = new cv::viz::WCone(length, radius, resolution, *color);
			return Ok<cv::viz::WCone*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCone*>))
	}
	
	Result<cv::viz::WCone*> cv_viz_WCone_WCone_double_const_Point3dR_const_Point3dR_int_const_ColorR(double radius, const cv::Point3d* center, const cv::Point3d* tip, int resolution, const cv::viz::Color* color) {
		try {
			cv::viz::WCone* ret = new cv::viz::WCone(radius, *center, *tip, resolution, *color);
			return Ok<cv::viz::WCone*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCone*>))
	}
	
	void cv_WCoordinateSystem_delete(cv::viz::WCoordinateSystem* instance) {
		delete instance;
	}
	Result<cv::viz::WCoordinateSystem*> cv_viz_WCoordinateSystem_WCoordinateSystem_double(double scale) {
		try {
			cv::viz::WCoordinateSystem* ret = new cv::viz::WCoordinateSystem(scale);
			return Ok<cv::viz::WCoordinateSystem*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCoordinateSystem*>))
	}
	
	void cv_WCube_delete(cv::viz::WCube* instance) {
		delete instance;
	}
	Result<cv::viz::WCube*> cv_viz_WCube_WCube_const_Point3dR_const_Point3dR_bool_const_ColorR(const cv::Point3d* min_point, const cv::Point3d* max_point, bool wire_frame, const cv::viz::Color* color) {
		try {
			cv::viz::WCube* ret = new cv::viz::WCube(*min_point, *max_point, wire_frame, *color);
			return Ok<cv::viz::WCube*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCube*>))
	}
	
	void cv_WCylinder_delete(cv::viz::WCylinder* instance) {
		delete instance;
	}
	Result<cv::viz::WCylinder*> cv_viz_WCylinder_WCylinder_const_Point3dR_const_Point3dR_double_int_const_ColorR(const cv::Point3d* axis_point1, const cv::Point3d* axis_point2, double radius, int numsides, const cv::viz::Color* color) {
		try {
			cv::viz::WCylinder* ret = new cv::viz::WCylinder(*axis_point1, *axis_point2, radius, numsides, *color);
			return Ok<cv::viz::WCylinder*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCylinder*>))
	}
	
	void cv_WGrid_delete(cv::viz::WGrid* instance) {
		delete instance;
	}
	Result<cv::viz::WGrid*> cv_viz_WGrid_WGrid_const_Vec2iR_const_Vec2dR_const_ColorR(const cv::Vec2i* cells, const cv::Vec2d* cells_spacing, const cv::viz::Color* color) {
		try {
			cv::viz::WGrid* ret = new cv::viz::WGrid(*cells, *cells_spacing, *color);
			return Ok<cv::viz::WGrid*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WGrid*>))
	}
	
	Result<cv::viz::WGrid*> cv_viz_WGrid_WGrid_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Vec2iR_const_Vec2dR_const_ColorR(const cv::Point3d* center, const cv::Vec3d* normal, const cv::Vec3d* new_yaxis, const cv::Vec2i* cells, const cv::Vec2d* cells_spacing, const cv::viz::Color* color) {
		try {
			cv::viz::WGrid* ret = new cv::viz::WGrid(*center, *normal, *new_yaxis, *cells, *cells_spacing, *color);
			return Ok<cv::viz::WGrid*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WGrid*>))
	}
	
	void cv_WImage3D_delete(cv::viz::WImage3D* instance) {
		delete instance;
	}
	Result<cv::viz::WImage3D*> cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR(const cv::_InputArray* image, const cv::Size2d* size) {
		try {
			cv::viz::WImage3D* ret = new cv::viz::WImage3D(*image, *size);
			return Ok<cv::viz::WImage3D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WImage3D*>))
	}
	
	Result<cv::viz::WImage3D*> cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(const cv::_InputArray* image, const cv::Size2d* size, const cv::Vec3d* center, const cv::Vec3d* normal, const cv::Vec3d* up_vector) {
		try {
			cv::viz::WImage3D* ret = new cv::viz::WImage3D(*image, *size, *center, *normal, *up_vector);
			return Ok<cv::viz::WImage3D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WImage3D*>))
	}
	
	Result_void cv_viz_WImage3D_setImage_const__InputArrayR(cv::viz::WImage3D* instance, const cv::_InputArray* image) {
		try {
			instance->setImage(*image);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_WImage3D_setSize_const_SizeR(cv::viz::WImage3D* instance, const cv::Size* size) {
		try {
			instance->setSize(*size);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_WImageOverlay_delete(cv::viz::WImageOverlay* instance) {
		delete instance;
	}
	Result<cv::viz::WImageOverlay*> cv_viz_WImageOverlay_WImageOverlay_const__InputArrayR_const_RectR(const cv::_InputArray* image, const cv::Rect* rect) {
		try {
			cv::viz::WImageOverlay* ret = new cv::viz::WImageOverlay(*image, *rect);
			return Ok<cv::viz::WImageOverlay*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WImageOverlay*>))
	}
	
	Result_void cv_viz_WImageOverlay_setImage_const__InputArrayR(cv::viz::WImageOverlay* instance, const cv::_InputArray* image) {
		try {
			instance->setImage(*image);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_WLine_delete(cv::viz::WLine* instance) {
		delete instance;
	}
	Result<cv::viz::WLine*> cv_viz_WLine_WLine_const_Point3dR_const_Point3dR_const_ColorR(const cv::Point3d* pt1, const cv::Point3d* pt2, const cv::viz::Color* color) {
		try {
			cv::viz::WLine* ret = new cv::viz::WLine(*pt1, *pt2, *color);
			return Ok<cv::viz::WLine*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WLine*>))
	}
	
	void cv_WMesh_delete(cv::viz::WMesh* instance) {
		delete instance;
	}
	Result<cv::viz::WMesh*> cv_viz_WMesh_WMesh_const_MeshR(const cv::viz::Mesh* mesh) {
		try {
			cv::viz::WMesh* ret = new cv::viz::WMesh(*mesh);
			return Ok<cv::viz::WMesh*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WMesh*>))
	}
	
	Result<cv::viz::WMesh*> cv_viz_WMesh_WMesh_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* cloud, const cv::_InputArray* polygons, const cv::_InputArray* colors, const cv::_InputArray* normals) {
		try {
			cv::viz::WMesh* ret = new cv::viz::WMesh(*cloud, *polygons, *colors, *normals);
			return Ok<cv::viz::WMesh*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WMesh*>))
	}
	
	void cv_WPaintedCloud_delete(cv::viz::WPaintedCloud* instance) {
		delete instance;
	}
	Result<cv::viz::WPaintedCloud*> cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR(const cv::_InputArray* cloud) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*cloud);
			return Ok<cv::viz::WPaintedCloud*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPaintedCloud*>))
	}
	
	Result<cv::viz::WPaintedCloud*> cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR(const cv::_InputArray* cloud, const cv::Point3d* p1, const cv::Point3d* p2) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*cloud, *p1, *p2);
			return Ok<cv::viz::WPaintedCloud*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPaintedCloud*>))
	}
	
	Result<cv::viz::WPaintedCloud*> cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR_const_ColorR_const_Color(const cv::_InputArray* cloud, const cv::Point3d* p1, const cv::Point3d* p2, const cv::viz::Color* c1, const cv::viz::Color* c2) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*cloud, *p1, *p2, *c1, *c2);
			return Ok<cv::viz::WPaintedCloud*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPaintedCloud*>))
	}
	
	void cv_WPlane_delete(cv::viz::WPlane* instance) {
		delete instance;
	}
	Result<cv::viz::WPlane*> cv_viz_WPlane_WPlane_const_Size2dR_const_ColorR(const cv::Size2d* size, const cv::viz::Color* color) {
		try {
			cv::viz::WPlane* ret = new cv::viz::WPlane(*size, *color);
			return Ok<cv::viz::WPlane*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPlane*>))
	}
	
	Result<cv::viz::WPlane*> cv_viz_WPlane_WPlane_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Size2dR_const_ColorR(const cv::Point3d* center, const cv::Vec3d* normal, const cv::Vec3d* new_yaxis, const cv::Size2d* size, const cv::viz::Color* color) {
		try {
			cv::viz::WPlane* ret = new cv::viz::WPlane(*center, *normal, *new_yaxis, *size, *color);
			return Ok<cv::viz::WPlane*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPlane*>))
	}
	
	void cv_WPolyLine_delete(cv::viz::WPolyLine* instance) {
		delete instance;
	}
	Result<cv::viz::WPolyLine*> cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const__InputArrayR(const cv::_InputArray* points, const cv::_InputArray* colors) {
		try {
			cv::viz::WPolyLine* ret = new cv::viz::WPolyLine(*points, *colors);
			return Ok<cv::viz::WPolyLine*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPolyLine*>))
	}
	
	Result<cv::viz::WPolyLine*> cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const_ColorR(const cv::_InputArray* points, const cv::viz::Color* color) {
		try {
			cv::viz::WPolyLine* ret = new cv::viz::WPolyLine(*points, *color);
			return Ok<cv::viz::WPolyLine*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPolyLine*>))
	}
	
	void cv_WSphere_delete(cv::viz::WSphere* instance) {
		delete instance;
	}
	Result<cv::viz::WSphere*> cv_viz_WSphere_WSphere_const_Point3dR_double_int_const_ColorR(const cv::Point3d* center, double radius, int sphere_resolution, const cv::viz::Color* color) {
		try {
			cv::viz::WSphere* ret = new cv::viz::WSphere(*center, radius, sphere_resolution, *color);
			return Ok<cv::viz::WSphere*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WSphere*>))
	}
	
	void cv_WText_delete(cv::viz::WText* instance) {
		delete instance;
	}
	Result<cv::viz::WText*> cv_viz_WText_WText_const_StringR_const_PointR_int_const_ColorR(const char* text, const cv::Point* pos, int font_size, const cv::viz::Color* color) {
		try {
			cv::viz::WText* ret = new cv::viz::WText(cv::String(text), *pos, font_size, *color);
			return Ok<cv::viz::WText*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WText*>))
	}
	
	Result_void cv_viz_WText_setText_const_StringR(cv::viz::WText* instance, const char* text) {
		try {
			instance->setText(cv::String(text));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_viz_WText_getText_const(const cv::viz::WText* instance) {
		try {
			cv::String ret = instance->getText();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_WText3D_delete(cv::viz::WText3D* instance) {
		delete instance;
	}
	Result<cv::viz::WText3D*> cv_viz_WText3D_WText3D_const_StringR_const_Point3dR_double_bool_const_ColorR(const char* text, const cv::Point3d* position, double text_scale, bool face_camera, const cv::viz::Color* color) {
		try {
			cv::viz::WText3D* ret = new cv::viz::WText3D(cv::String(text), *position, text_scale, face_camera, *color);
			return Ok<cv::viz::WText3D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WText3D*>))
	}
	
	Result_void cv_viz_WText3D_setText_const_StringR(cv::viz::WText3D* instance, const char* text) {
		try {
			instance->setText(cv::String(text));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_viz_WText3D_getText_const(const cv::viz::WText3D* instance) {
		try {
			cv::String ret = instance->getText();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_WTrajectory_delete(cv::viz::WTrajectory* instance) {
		delete instance;
	}
	Result<cv::viz::WTrajectory*> cv_viz_WTrajectory_WTrajectory_const__InputArrayR_int_double_const_ColorR(const cv::_InputArray* path, int display_mode, double scale, const cv::viz::Color* color) {
		try {
			cv::viz::WTrajectory* ret = new cv::viz::WTrajectory(*path, display_mode, scale, *color);
			return Ok<cv::viz::WTrajectory*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WTrajectory*>))
	}
	
	void cv_WTrajectoryFrustums_delete(cv::viz::WTrajectoryFrustums* instance) {
		delete instance;
	}
	Result<cv::viz::WTrajectoryFrustums*> cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Matx33dR_double_const_ColorR(const cv::_InputArray* path, const cv::Matx33d* K, double scale, const cv::viz::Color* color) {
		try {
			cv::viz::WTrajectoryFrustums* ret = new cv::viz::WTrajectoryFrustums(*path, *K, scale, *color);
			return Ok<cv::viz::WTrajectoryFrustums*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WTrajectoryFrustums*>))
	}
	
	Result<cv::viz::WTrajectoryFrustums*> cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Vec2dR_double_const_ColorR(const cv::_InputArray* path, const cv::Vec2d* fov, double scale, const cv::viz::Color* color) {
		try {
			cv::viz::WTrajectoryFrustums* ret = new cv::viz::WTrajectoryFrustums(*path, *fov, scale, *color);
			return Ok<cv::viz::WTrajectoryFrustums*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WTrajectoryFrustums*>))
	}
	
	void cv_WTrajectorySpheres_delete(cv::viz::WTrajectorySpheres* instance) {
		delete instance;
	}
	Result<cv::viz::WTrajectorySpheres*> cv_viz_WTrajectorySpheres_WTrajectorySpheres_const__InputArrayR_double_double_const_ColorR_const_ColorR(const cv::_InputArray* path, double line_length, double radius, const cv::viz::Color* from, const cv::viz::Color* to) {
		try {
			cv::viz::WTrajectorySpheres* ret = new cv::viz::WTrajectorySpheres(*path, line_length, radius, *from, *to);
			return Ok<cv::viz::WTrajectorySpheres*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WTrajectorySpheres*>))
	}
	
	void cv_WWidgetMerger_delete(cv::viz::WWidgetMerger* instance) {
		delete instance;
	}
	Result<cv::viz::WWidgetMerger*> cv_viz_WWidgetMerger_WWidgetMerger() {
		try {
			cv::viz::WWidgetMerger* ret = new cv::viz::WWidgetMerger();
			return Ok<cv::viz::WWidgetMerger*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WWidgetMerger*>))
	}
	
	Result_void cv_viz_WWidgetMerger_addWidget_const_Widget3DR_const_Affine3dR(cv::viz::WWidgetMerger* instance, const cv::viz::Widget3D* widget, const cv::Affine3d* pose) {
		try {
			instance->addWidget(*widget, *pose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_WWidgetMerger_finalize(cv::viz::WWidgetMerger* instance) {
		try {
			instance->finalize();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Widget_delete(cv::viz::Widget* instance) {
		delete instance;
	}
	Result<cv::viz::Widget*> cv_viz_Widget_Widget() {
		try {
			cv::viz::Widget* ret = new cv::viz::Widget();
			return Ok<cv::viz::Widget*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget*>))
	}
	
	Result<cv::viz::Widget*> cv_viz_Widget_Widget_const_WidgetR(const cv::viz::Widget* other) {
		try {
			cv::viz::Widget* ret = new cv::viz::Widget(*other);
			return Ok<cv::viz::Widget*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget*>))
	}
	
	Result<cv::viz::Widget*> cv_viz_Widget_fromPlyFile_const_StringR(const char* file_name) {
		try {
			cv::viz::Widget ret = cv::viz::Widget::fromPlyFile(cv::String(file_name));
			return Ok(new cv::viz::Widget(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget*>))
	}
	
	Result_void cv_viz_Widget_setRenderingProperty_int_double(cv::viz::Widget* instance, int property, double value) {
		try {
			instance->setRenderingProperty(property, value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_viz_Widget_getRenderingProperty_const_int(const cv::viz::Widget* instance, int property) {
		try {
			double ret = instance->getRenderingProperty(property);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	void cv_Widget2D_delete(cv::viz::Widget2D* instance) {
		delete instance;
	}
	Result<cv::viz::Widget2D*> cv_viz_Widget2D_Widget2D() {
		try {
			cv::viz::Widget2D* ret = new cv::viz::Widget2D();
			return Ok<cv::viz::Widget2D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget2D*>))
	}
	
	Result_void cv_viz_Widget2D_setColor_const_ColorR(cv::viz::Widget2D* instance, const cv::viz::Color* color) {
		try {
			instance->setColor(*color);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Widget3D_delete(cv::viz::Widget3D* instance) {
		delete instance;
	}
	Result<cv::viz::Widget3D*> cv_viz_Widget3D_Widget3D() {
		try {
			cv::viz::Widget3D* ret = new cv::viz::Widget3D();
			return Ok<cv::viz::Widget3D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget3D*>))
	}
	
	Result_void cv_viz_Widget3D_setPose_const_Affine3dR(cv::viz::Widget3D* instance, const cv::Affine3d* pose) {
		try {
			instance->setPose(*pose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Widget3D_updatePose_const_Affine3dR(cv::viz::Widget3D* instance, const cv::Affine3d* pose) {
		try {
			instance->updatePose(*pose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Affine3d> cv_viz_Widget3D_getPose_const(const cv::viz::Widget3D* instance) {
		try {
			cv::Affine3d ret = instance->getPose();
			return Ok<cv::Affine3d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3d>))
	}
	
	Result_void cv_viz_Widget3D_applyTransform_const_Affine3dR(cv::viz::Widget3D* instance, const cv::Affine3d* transform) {
		try {
			instance->applyTransform(*transform);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_viz_Widget3D_setColor_const_ColorR(cv::viz::Widget3D* instance, const cv::viz::Color* color) {
		try {
			instance->setColor(*color);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}

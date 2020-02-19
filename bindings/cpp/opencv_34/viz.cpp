#include "common.hpp"
#include <opencv2/viz.hpp>
#include "viz_types.hpp"

extern "C" {
	Result_void cv_viz_computeNormals_const_MeshX_const__OutputArrayX(void* mesh, void* normals) {
		try {
			cv::viz::computeNormals(*reinterpret_cast<const cv::viz::Mesh*>(mesh), *reinterpret_cast<const cv::_OutputArray*>(normals));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_getWindowByName_const_StringX(const char* window_name) {
		try {
			cv::viz::Viz3d ret = cv::viz::getWindowByName(cv::String(window_name));
			return Ok<void*>(new cv::viz::Viz3d(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_imshow_const_StringX_const__InputArrayX_const_SizeX(const char* window_name, void* image, const cv::Size* window_size) {
		try {
			cv::viz::Viz3d ret = cv::viz::imshow(cv::String(window_name), *reinterpret_cast<const cv::_InputArray*>(image), *window_size);
			return Ok<void*>(new cv::viz::Viz3d(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_readCloud_const_StringX_const__OutputArrayX_const__OutputArrayX(const char* file, void* colors, void* normals) {
		try {
			cv::Mat ret = cv::viz::readCloud(cv::String(file), *reinterpret_cast<const cv::_OutputArray*>(colors), *reinterpret_cast<const cv::_OutputArray*>(normals));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_readMesh_const_StringX(const char* file) {
		try {
			cv::viz::Mesh ret = cv::viz::readMesh(cv::String(file));
			return Ok<void*>(new cv::viz::Mesh(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_readTrajectory_const__OutputArrayX_const_StringX_int_int_const_StringX(void* traj, const char* files_format, int start, int end, const char* tag) {
		try {
			cv::viz::readTrajectory(*reinterpret_cast<const cv::_OutputArray*>(traj), cv::String(files_format), start, end, cv::String(tag));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_unregisterAllWindows() {
		try {
			cv::viz::unregisterAllWindows();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_writeCloud_const_StringX_const__InputArrayX_const__InputArrayX_const__InputArrayX_bool(const char* file, void* cloud, void* colors, void* normals, bool binary) {
		try {
			cv::viz::writeCloud(cv::String(file), *reinterpret_cast<const cv::_InputArray*>(cloud), *reinterpret_cast<const cv::_InputArray*>(colors), *reinterpret_cast<const cv::_InputArray*>(normals), binary);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_writeTrajectory_const__InputArrayX_const_StringX_int_const_StringX(void* traj, const char* files_format, int start, const char* tag) {
		try {
			cv::viz::writeTrajectory(*reinterpret_cast<const cv::_InputArray*>(traj), cv::String(files_format), start, cv::String(tag));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Camera_delete(cv::viz::Camera* instance) {
		delete instance;
	}
	Result<void*> cv_viz_Camera_Camera_double_double_double_double_const_SizeX(double fx, double fy, double cx, double cy, const cv::Size* window_size) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(fx, fy, cx, cy, *window_size);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Camera_Camera_const_Vec2dX_const_SizeX(const cv::Vec2d* fov, const cv::Size* window_size) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(*fov, *window_size);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Vec2d> cv_viz_Camera_getClip_const(void* instance) {
		try {
			cv::Vec2d ret = reinterpret_cast<cv::viz::Camera*>(instance)->getClip();
			return Ok<cv::Vec2d>(ret);
		} OCVRS_CATCH(Result<cv::Vec2d>)
	}
	
	Result_void cv_viz_Camera_setClip_const_Vec2dX(void* instance, const cv::Vec2d* clip) {
		try {
			reinterpret_cast<cv::viz::Camera*>(instance)->setClip(*clip);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_viz_Camera_getWindowSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::viz::Camera*>(instance)->getWindowSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_viz_Camera_setWindowSize_const_SizeX(void* instance, const cv::Size* window_size) {
		try {
			reinterpret_cast<cv::viz::Camera*>(instance)->setWindowSize(*window_size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Vec2d> cv_viz_Camera_getFov_const(void* instance) {
		try {
			cv::Vec2d ret = reinterpret_cast<cv::viz::Camera*>(instance)->getFov();
			return Ok<cv::Vec2d>(ret);
		} OCVRS_CATCH(Result<cv::Vec2d>)
	}
	
	Result_void cv_viz_Camera_setFov_const_Vec2dX(void* instance, const cv::Vec2d* fov) {
		try {
			reinterpret_cast<cv::viz::Camera*>(instance)->setFov(*fov);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Vec2d> cv_viz_Camera_getPrincipalPoint_const(void* instance) {
		try {
			cv::Vec2d ret = reinterpret_cast<cv::viz::Camera*>(instance)->getPrincipalPoint();
			return Ok<cv::Vec2d>(ret);
		} OCVRS_CATCH(Result<cv::Vec2d>)
	}
	
	Result<cv::Vec2d> cv_viz_Camera_getFocalLength_const(void* instance) {
		try {
			cv::Vec2d ret = reinterpret_cast<cv::viz::Camera*>(instance)->getFocalLength();
			return Ok<cv::Vec2d>(ret);
		} OCVRS_CATCH(Result<cv::Vec2d>)
	}
	
	Result<void*> cv_viz_Camera_KinectCamera_const_SizeX(const cv::Size* window_size) {
		try {
			cv::viz::Camera ret = cv::viz::Camera::KinectCamera(*window_size);
			return Ok<void*>(new cv::viz::Camera(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Color_delete(cv::viz::Color* instance) {
		delete instance;
	}
	Result<void*> cv_viz_Color_Color() {
		try {
			cv::viz::Color* ret = new cv::viz::Color();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_Color_double(double gray) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(gray);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_Color_double_double_double(double blue, double green, double red) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(blue, green, red);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_Color_const_ScalarX(const cv::Scalar* color) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(*color);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Vec3b> cv_viz_Color_operator_cv_Vec3b_const(void* instance) {
		try {
			cv::Vec3b ret = reinterpret_cast<cv::viz::Color*>(instance)->operator cv::Vec3b();
			return Ok<cv::Vec3b>(ret);
		} OCVRS_CATCH(Result<cv::Vec3b>)
	}
	
	Result<void*> cv_viz_Color_black() {
		try {
			cv::viz::Color ret = cv::viz::Color::black();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_blue() {
		try {
			cv::viz::Color ret = cv::viz::Color::blue();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_green() {
		try {
			cv::viz::Color ret = cv::viz::Color::green();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_red() {
		try {
			cv::viz::Color ret = cv::viz::Color::red();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_cyan() {
		try {
			cv::viz::Color ret = cv::viz::Color::cyan();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_yellow() {
		try {
			cv::viz::Color ret = cv::viz::Color::yellow();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_magenta() {
		try {
			cv::viz::Color ret = cv::viz::Color::magenta();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_white() {
		try {
			cv::viz::Color ret = cv::viz::Color::white();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_gray() {
		try {
			cv::viz::Color ret = cv::viz::Color::gray();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_silver() {
		try {
			cv::viz::Color ret = cv::viz::Color::silver();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_mlab() {
		try {
			cv::viz::Color ret = cv::viz::Color::mlab();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_navy() {
		try {
			cv::viz::Color ret = cv::viz::Color::navy();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_maroon() {
		try {
			cv::viz::Color ret = cv::viz::Color::maroon();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_teal() {
		try {
			cv::viz::Color ret = cv::viz::Color::teal();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_olive() {
		try {
			cv::viz::Color ret = cv::viz::Color::olive();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_purple() {
		try {
			cv::viz::Color ret = cv::viz::Color::purple();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_azure() {
		try {
			cv::viz::Color ret = cv::viz::Color::azure();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_chartreuse() {
		try {
			cv::viz::Color ret = cv::viz::Color::chartreuse();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_rose() {
		try {
			cv::viz::Color ret = cv::viz::Color::rose();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_lime() {
		try {
			cv::viz::Color ret = cv::viz::Color::lime();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_gold() {
		try {
			cv::viz::Color ret = cv::viz::Color::gold();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_orange() {
		try {
			cv::viz::Color ret = cv::viz::Color::orange();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_orange_red() {
		try {
			cv::viz::Color ret = cv::viz::Color::orange_red();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_indigo() {
		try {
			cv::viz::Color ret = cv::viz::Color::indigo();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_brown() {
		try {
			cv::viz::Color ret = cv::viz::Color::brown();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_apricot() {
		try {
			cv::viz::Color ret = cv::viz::Color::apricot();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_pink() {
		try {
			cv::viz::Color ret = cv::viz::Color::pink();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_raspberry() {
		try {
			cv::viz::Color ret = cv::viz::Color::raspberry();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_cherry() {
		try {
			cv::viz::Color ret = cv::viz::Color::cherry();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_violet() {
		try {
			cv::viz::Color ret = cv::viz::Color::violet();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_amethyst() {
		try {
			cv::viz::Color ret = cv::viz::Color::amethyst();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_bluberry() {
		try {
			cv::viz::Color ret = cv::viz::Color::bluberry();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_celestial_blue() {
		try {
			cv::viz::Color ret = cv::viz::Color::celestial_blue();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_turquoise() {
		try {
			cv::viz::Color ret = cv::viz::Color::turquoise();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Color_not_set() {
		try {
			cv::viz::Color ret = cv::viz::Color::not_set();
			return Ok<void*>(new cv::viz::Color(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::viz::KeyboardEvent::Action> cv_viz_KeyboardEvent_action_const(void* instance) {
		try {
			cv::viz::KeyboardEvent::Action ret = reinterpret_cast<cv::viz::KeyboardEvent*>(instance)->action;
			return Ok<cv::viz::KeyboardEvent::Action>(ret);
		} OCVRS_CATCH(Result<cv::viz::KeyboardEvent::Action>)
	}
	
	Result_void cv_viz_KeyboardEvent_setAction_Action(void* instance, cv::viz::KeyboardEvent::Action val) {
		try {
			reinterpret_cast<cv::viz::KeyboardEvent*>(instance)->action = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_KeyboardEvent_symbol_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::viz::KeyboardEvent*>(instance)->symbol;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_KeyboardEvent_setSymbol_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::viz::KeyboardEvent*>(instance)->symbol = cv::String(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned char> cv_viz_KeyboardEvent_code_const(void* instance) {
		try {
			unsigned char ret = reinterpret_cast<cv::viz::KeyboardEvent*>(instance)->code;
			return Ok<unsigned char>(ret);
		} OCVRS_CATCH(Result<unsigned char>)
	}
	
	Result_void cv_viz_KeyboardEvent_setCode_unsigned_char(void* instance, unsigned char val) {
		try {
			reinterpret_cast<cv::viz::KeyboardEvent*>(instance)->code = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_viz_KeyboardEvent_modifiers_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::viz::KeyboardEvent*>(instance)->modifiers;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_viz_KeyboardEvent_setModifiers_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::viz::KeyboardEvent*>(instance)->modifiers = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_KeyboardEvent_delete(cv::viz::KeyboardEvent* instance) {
		delete instance;
	}
	Result<void*> cv_viz_KeyboardEvent_KeyboardEvent_Action_const_StringX_unsigned_char_int(cv::viz::KeyboardEvent::Action action, const char* symbol, unsigned char code, int modifiers) {
		try {
			cv::viz::KeyboardEvent* ret = new cv::viz::KeyboardEvent(action, cv::String(symbol), code, modifiers);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Mesh_cloud(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::viz::Mesh*>(instance)->cloud;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Mesh_setCloud_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::viz::Mesh*>(instance)->cloud = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_Mesh_colors(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::viz::Mesh*>(instance)->colors;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Mesh_setColors_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::viz::Mesh*>(instance)->colors = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_Mesh_normals(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::viz::Mesh*>(instance)->normals;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Mesh_setNormals_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::viz::Mesh*>(instance)->normals = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_Mesh_polygons(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::viz::Mesh*>(instance)->polygons;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Mesh_setPolygons_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::viz::Mesh*>(instance)->polygons = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_Mesh_texture(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::viz::Mesh*>(instance)->texture;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Mesh_setTexture_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::viz::Mesh*>(instance)->texture = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_Mesh_tcoords(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::viz::Mesh*>(instance)->tcoords;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Mesh_setTcoords_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::viz::Mesh*>(instance)->tcoords = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Mesh_delete(cv::viz::Mesh* instance) {
		delete instance;
	}
	Result<void*> cv_viz_Mesh_load_const_StringX_int(const char* file, int type) {
		try {
			cv::viz::Mesh ret = cv::viz::Mesh::load(cv::String(file), type);
			return Ok<void*>(new cv::viz::Mesh(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::viz::MouseEvent::Type> cv_viz_MouseEvent_type_const(void* instance) {
		try {
			cv::viz::MouseEvent::Type ret = reinterpret_cast<cv::viz::MouseEvent*>(instance)->type;
			return Ok<cv::viz::MouseEvent::Type>(ret);
		} OCVRS_CATCH(Result<cv::viz::MouseEvent::Type>)
	}
	
	Result_void cv_viz_MouseEvent_setType_Type(void* instance, cv::viz::MouseEvent::Type val) {
		try {
			reinterpret_cast<cv::viz::MouseEvent*>(instance)->type = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::viz::MouseEvent::MouseButton> cv_viz_MouseEvent_button_const(void* instance) {
		try {
			cv::viz::MouseEvent::MouseButton ret = reinterpret_cast<cv::viz::MouseEvent*>(instance)->button;
			return Ok<cv::viz::MouseEvent::MouseButton>(ret);
		} OCVRS_CATCH(Result<cv::viz::MouseEvent::MouseButton>)
	}
	
	Result_void cv_viz_MouseEvent_setButton_MouseButton(void* instance, cv::viz::MouseEvent::MouseButton val) {
		try {
			reinterpret_cast<cv::viz::MouseEvent*>(instance)->button = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point> cv_viz_MouseEvent_pointer_const(void* instance) {
		try {
			cv::Point ret = reinterpret_cast<cv::viz::MouseEvent*>(instance)->pointer;
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result_void cv_viz_MouseEvent_setPointer_Point(void* instance, cv::Point val) {
		try {
			reinterpret_cast<cv::viz::MouseEvent*>(instance)->pointer = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_viz_MouseEvent_modifiers_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::viz::MouseEvent*>(instance)->modifiers;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_viz_MouseEvent_setModifiers_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::viz::MouseEvent*>(instance)->modifiers = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MouseEvent_delete(cv::viz::MouseEvent* instance) {
		delete instance;
	}
	Result<void*> cv_viz_MouseEvent_MouseEvent_const_TypeX_const_MouseButtonX_const_PointX_int(const cv::viz::MouseEvent::Type* type, const cv::viz::MouseEvent::MouseButton* button, const cv::Point* pointer, int modifiers) {
		try {
			cv::viz::MouseEvent* ret = new cv::viz::MouseEvent(*type, *button, *pointer, modifiers);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Viz3d_delete(cv::viz::Viz3d* instance) {
		delete instance;
	}
	Result<void*> cv_viz_Viz3d_Viz3d_const_StringX(const char* window_name) {
		try {
			cv::viz::Viz3d* ret = new cv::viz::Viz3d(cv::String(window_name));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Viz3d_Viz3d_const_Viz3dX(void* unnamed) {
		try {
			cv::viz::Viz3d* ret = new cv::viz::Viz3d(*reinterpret_cast<const cv::viz::Viz3d*>(unnamed));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Viz3d_removeWidget_const_StringX(void* instance, const char* id) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->removeWidget(cv::String(id));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_Viz3d_getWidget_const_const_StringX(void* instance, const char* id) {
		try {
			cv::viz::Widget ret = reinterpret_cast<cv::viz::Viz3d*>(instance)->getWidget(cv::String(id));
			return Ok<void*>(new cv::viz::Widget(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Viz3d_removeAllWidgets(void* instance) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->removeAllWidgets();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_showImage_const__InputArrayX_const_SizeX(void* instance, void* image, const cv::Size* window_size) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->showImage(*reinterpret_cast<const cv::_InputArray*>(image), *window_size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_setCamera_const_CameraX(void* instance, void* camera) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->setCamera(*reinterpret_cast<const cv::viz::Camera*>(camera));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_Viz3d_getCamera_const(void* instance) {
		try {
			cv::viz::Camera ret = reinterpret_cast<cv::viz::Viz3d*>(instance)->getCamera();
			return Ok<void*>(new cv::viz::Camera(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Viz3d_resetCameraViewpoint_const_StringX(void* instance, const char* id) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->resetCameraViewpoint(cv::String(id));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_resetCamera(void* instance) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->resetCamera();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_convertToWindowCoordinates_const_Point3dX_Point3dX(void* instance, const cv::Point3d* pt, cv::Point3d* window_coord) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->convertToWindowCoordinates(*pt, *window_coord);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_converTo3DRay_const_Point3dX_Point3dX_Vec3dX(void* instance, const cv::Point3d* window_coord, cv::Point3d* origin, cv::Vec3d* direction) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->converTo3DRay(*window_coord, *origin, *direction);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_viz_Viz3d_getWindowSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::viz::Viz3d*>(instance)->getWindowSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_viz_Viz3d_setWindowSize_const_SizeX(void* instance, const cv::Size* window_size) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->setWindowSize(*window_size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_Viz3d_getWindowName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::viz::Viz3d*>(instance)->getWindowName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Viz3d_getScreenshot_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::viz::Viz3d*>(instance)->getScreenshot();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Viz3d_saveScreenshot_const_StringX(void* instance, const char* file) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->saveScreenshot(cv::String(file));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_setWindowPosition_const_PointX(void* instance, const cv::Point* window_position) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->setWindowPosition(*window_position);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_setFullScreen_bool(void* instance, bool mode) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->setFullScreen(mode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_setBackgroundColor_const_ColorX_const_ColorX(void* instance, void* color, void* color2) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->setBackgroundColor(*reinterpret_cast<const cv::viz::Viz3d::Color*>(color), *reinterpret_cast<const cv::viz::Viz3d::Color*>(color2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_setBackgroundTexture_const__InputArrayX(void* instance, void* image) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->setBackgroundTexture(*reinterpret_cast<const cv::_InputArray*>(image));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_setBackgroundMeshLab(void* instance) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->setBackgroundMeshLab();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_spin(void* instance) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->spin();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_spinOnce_int_bool(void* instance, int time, bool force_redraw) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->spinOnce(time, force_redraw);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_setOffScreenRendering(void* instance) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->setOffScreenRendering();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_removeAllLights(void* instance) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->removeAllLights();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_addLight_Vec3d_Vec3d_Color_Color_Color_Color(void* instance, cv::Vec3d position, cv::Vec3d focalPoint, void* color, void* diffuseColor, void* ambientColor, void* specularColor) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->addLight(position, focalPoint, *reinterpret_cast<cv::viz::Viz3d::Color*>(color), *reinterpret_cast<cv::viz::Viz3d::Color*>(diffuseColor), *reinterpret_cast<cv::viz::Viz3d::Color*>(ambientColor), *reinterpret_cast<cv::viz::Viz3d::Color*>(specularColor));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_viz_Viz3d_wasStopped_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::viz::Viz3d*>(instance)->wasStopped();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_viz_Viz3d_close(void* instance) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->close();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_registerKeyboardCallback_KeyboardCallback_voidX(void* instance, cv::viz::Viz3d::KeyboardCallback callback, void* cookie) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->registerKeyboardCallback(callback, cookie);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_registerMouseCallback_MouseCallback_voidX(void* instance, cv::viz::Viz3d::MouseCallback callback, void* cookie) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->registerMouseCallback(callback, cookie);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_setRenderingProperty_const_StringX_int_double(void* instance, const char* id, int property, double value) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->setRenderingProperty(cv::String(id), property, value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_viz_Viz3d_getRenderingProperty_const_StringX_int(void* instance, const char* id, int property) {
		try {
			double ret = reinterpret_cast<cv::viz::Viz3d*>(instance)->getRenderingProperty(cv::String(id), property);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_viz_Viz3d_setRepresentation_int(void* instance, int representation) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->setRepresentation(representation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_Viz3d_setGlobalWarnings_bool(void* instance, bool enabled) {
		try {
			reinterpret_cast<cv::viz::Viz3d*>(instance)->setGlobalWarnings(enabled);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_WArrow_delete(cv::viz::WArrow* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WArrow_WArrow_const_Point3dX_const_Point3dX_double_const_ColorX(const cv::Point3d* pt1, const cv::Point3d* pt2, double thickness, void* color) {
		try {
			cv::viz::WArrow* ret = new cv::viz::WArrow(*pt1, *pt2, thickness, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WCameraPosition_delete(cv::viz::WCameraPosition* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WCameraPosition_WCameraPosition_double(double scale) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WCameraPosition_WCameraPosition_const_Vec2dX_double_const_ColorX(const cv::Vec2d* fov, double scale, void* color) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*fov, scale, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WCameraPosition_WCameraPosition_const_Vec2dX_const__InputArrayX_double_const_ColorX(const cv::Vec2d* fov, void* image, double scale, void* color) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*fov, *reinterpret_cast<const cv::_InputArray*>(image), scale, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WCircle_delete(cv::viz::WCircle* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WCircle_WCircle_double_double_const_ColorX(double radius, double thickness, void* color) {
		try {
			cv::viz::WCircle* ret = new cv::viz::WCircle(radius, thickness, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WCircle_WCircle_double_const_Point3dX_const_Vec3dX_double_const_ColorX(double radius, const cv::Point3d* center, const cv::Vec3d* normal, double thickness, void* color) {
		try {
			cv::viz::WCircle* ret = new cv::viz::WCircle(radius, *center, *normal, thickness, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WCloud_delete(cv::viz::WCloud* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WCloud_WCloud_const__InputArrayX_const__InputArrayX(void* cloud, void* colors) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*reinterpret_cast<const cv::_InputArray*>(cloud), *reinterpret_cast<const cv::_InputArray*>(colors));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WCloud_WCloud_const__InputArrayX_const_ColorX(void* cloud, void* color) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*reinterpret_cast<const cv::_InputArray*>(cloud), *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WCloud_WCloud_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* cloud, void* colors, void* normals) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*reinterpret_cast<const cv::_InputArray*>(cloud), *reinterpret_cast<const cv::_InputArray*>(colors), *reinterpret_cast<const cv::_InputArray*>(normals));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WCloud_WCloud_const__InputArrayX_const_ColorX_const__InputArrayX(void* cloud, void* color, void* normals) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*reinterpret_cast<const cv::_InputArray*>(cloud), *reinterpret_cast<const cv::viz::Color*>(color), *reinterpret_cast<const cv::_InputArray*>(normals));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WCloudCollection_delete(cv::viz::WCloudCollection* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WCloudCollection_WCloudCollection() {
		try {
			cv::viz::WCloudCollection* ret = new cv::viz::WCloudCollection();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_WCloudCollection_finalize(void* instance) {
		try {
			reinterpret_cast<cv::viz::WCloudCollection*>(instance)->finalize();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_WCloudNormals_delete(cv::viz::WCloudNormals* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WCloudNormals_WCloudNormals_const__InputArrayX_const__InputArrayX_int_double_const_ColorX(void* cloud, void* normals, int level, double scale, void* color) {
		try {
			cv::viz::WCloudNormals* ret = new cv::viz::WCloudNormals(*reinterpret_cast<const cv::_InputArray*>(cloud), *reinterpret_cast<const cv::_InputArray*>(normals), level, scale, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WCone_delete(cv::viz::WCone* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WCone_WCone_double_double_int_const_ColorX(double length, double radius, int resolution, void* color) {
		try {
			cv::viz::WCone* ret = new cv::viz::WCone(length, radius, resolution, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WCone_WCone_double_const_Point3dX_const_Point3dX_int_const_ColorX(double radius, const cv::Point3d* center, const cv::Point3d* tip, int resolution, void* color) {
		try {
			cv::viz::WCone* ret = new cv::viz::WCone(radius, *center, *tip, resolution, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WCoordinateSystem_delete(cv::viz::WCoordinateSystem* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WCoordinateSystem_WCoordinateSystem_double(double scale) {
		try {
			cv::viz::WCoordinateSystem* ret = new cv::viz::WCoordinateSystem(scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WCube_delete(cv::viz::WCube* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WCube_WCube_const_Point3dX_const_Point3dX_bool_const_ColorX(const cv::Point3d* min_point, const cv::Point3d* max_point, bool wire_frame, void* color) {
		try {
			cv::viz::WCube* ret = new cv::viz::WCube(*min_point, *max_point, wire_frame, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WCylinder_delete(cv::viz::WCylinder* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WCylinder_WCylinder_const_Point3dX_const_Point3dX_double_int_const_ColorX(const cv::Point3d* axis_point1, const cv::Point3d* axis_point2, double radius, int numsides, void* color) {
		try {
			cv::viz::WCylinder* ret = new cv::viz::WCylinder(*axis_point1, *axis_point2, radius, numsides, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WGrid_delete(cv::viz::WGrid* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WGrid_WGrid_const_Vec2iX_const_Vec2dX_const_ColorX(const cv::Vec2i* cells, const cv::Vec2d* cells_spacing, void* color) {
		try {
			cv::viz::WGrid* ret = new cv::viz::WGrid(*cells, *cells_spacing, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WGrid_WGrid_const_Point3dX_const_Vec3dX_const_Vec3dX_const_Vec2iX_const_Vec2dX_const_ColorX(const cv::Point3d* center, const cv::Vec3d* normal, const cv::Vec3d* new_yaxis, const cv::Vec2i* cells, const cv::Vec2d* cells_spacing, void* color) {
		try {
			cv::viz::WGrid* ret = new cv::viz::WGrid(*center, *normal, *new_yaxis, *cells, *cells_spacing, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WImage3D_delete(cv::viz::WImage3D* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WImage3D_WImage3D_const__InputArrayX_const_Size2dX(void* image, const cv::Size2d* size) {
		try {
			cv::viz::WImage3D* ret = new cv::viz::WImage3D(*reinterpret_cast<const cv::_InputArray*>(image), *size);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WImage3D_WImage3D_const__InputArrayX_const_Size2dX_const_Vec3dX_const_Vec3dX_const_Vec3dX(void* image, const cv::Size2d* size, const cv::Vec3d* center, const cv::Vec3d* normal, const cv::Vec3d* up_vector) {
		try {
			cv::viz::WImage3D* ret = new cv::viz::WImage3D(*reinterpret_cast<const cv::_InputArray*>(image), *size, *center, *normal, *up_vector);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_WImage3D_setImage_const__InputArrayX(void* instance, void* image) {
		try {
			reinterpret_cast<cv::viz::WImage3D*>(instance)->setImage(*reinterpret_cast<const cv::_InputArray*>(image));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_viz_WImage3D_setSize_const_SizeX(void* instance, const cv::Size* size) {
		try {
			reinterpret_cast<cv::viz::WImage3D*>(instance)->setSize(*size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_WImageOverlay_delete(cv::viz::WImageOverlay* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WImageOverlay_WImageOverlay_const__InputArrayX_const_RectX(void* image, const cv::Rect* rect) {
		try {
			cv::viz::WImageOverlay* ret = new cv::viz::WImageOverlay(*reinterpret_cast<const cv::_InputArray*>(image), *rect);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_WImageOverlay_setImage_const__InputArrayX(void* instance, void* image) {
		try {
			reinterpret_cast<cv::viz::WImageOverlay*>(instance)->setImage(*reinterpret_cast<const cv::_InputArray*>(image));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_WLine_delete(cv::viz::WLine* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WLine_WLine_const_Point3dX_const_Point3dX_const_ColorX(const cv::Point3d* pt1, const cv::Point3d* pt2, void* color) {
		try {
			cv::viz::WLine* ret = new cv::viz::WLine(*pt1, *pt2, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WMesh_delete(cv::viz::WMesh* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WMesh_WMesh_const_MeshX(void* mesh) {
		try {
			cv::viz::WMesh* ret = new cv::viz::WMesh(*reinterpret_cast<const cv::viz::Mesh*>(mesh));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WMesh_WMesh_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* cloud, void* polygons, void* colors, void* normals) {
		try {
			cv::viz::WMesh* ret = new cv::viz::WMesh(*reinterpret_cast<const cv::_InputArray*>(cloud), *reinterpret_cast<const cv::_InputArray*>(polygons), *reinterpret_cast<const cv::_InputArray*>(colors), *reinterpret_cast<const cv::_InputArray*>(normals));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WPaintedCloud_delete(cv::viz::WPaintedCloud* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayX(void* cloud) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*reinterpret_cast<const cv::_InputArray*>(cloud));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayX_const_Point3dX_const_Point3dX(void* cloud, const cv::Point3d* p1, const cv::Point3d* p2) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*reinterpret_cast<const cv::_InputArray*>(cloud), *p1, *p2);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayX_const_Point3dX_const_Point3dX_const_ColorX_Color(void* cloud, const cv::Point3d* p1, const cv::Point3d* p2, void* c1, void* c2) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*reinterpret_cast<const cv::_InputArray*>(cloud), *p1, *p2, *reinterpret_cast<const cv::viz::Color*>(c1), *reinterpret_cast<cv::viz::Color*>(c2));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WPlane_delete(cv::viz::WPlane* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WPlane_WPlane_const_Size2dX_const_ColorX(const cv::Size2d* size, void* color) {
		try {
			cv::viz::WPlane* ret = new cv::viz::WPlane(*size, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WPlane_WPlane_const_Point3dX_const_Vec3dX_const_Vec3dX_const_Size2dX_const_ColorX(const cv::Point3d* center, const cv::Vec3d* normal, const cv::Vec3d* new_yaxis, const cv::Size2d* size, void* color) {
		try {
			cv::viz::WPlane* ret = new cv::viz::WPlane(*center, *normal, *new_yaxis, *size, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WPolyLine_delete(cv::viz::WPolyLine* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WPolyLine_WPolyLine_const__InputArrayX_const__InputArrayX(void* points, void* colors) {
		try {
			cv::viz::WPolyLine* ret = new cv::viz::WPolyLine(*reinterpret_cast<const cv::_InputArray*>(points), *reinterpret_cast<const cv::_InputArray*>(colors));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_WPolyLine_WPolyLine_const__InputArrayX_const_ColorX(void* points, void* color) {
		try {
			cv::viz::WPolyLine* ret = new cv::viz::WPolyLine(*reinterpret_cast<const cv::_InputArray*>(points), *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WSphere_delete(cv::viz::WSphere* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WSphere_WSphere_const_Point3dX_double_int_const_ColorX(const cv::Point3d* center, double radius, int sphere_resolution, void* color) {
		try {
			cv::viz::WSphere* ret = new cv::viz::WSphere(*center, radius, sphere_resolution, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WText_delete(cv::viz::WText* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WText_WText_const_StringX_const_PointX_int_const_ColorX(const char* text, const cv::Point* pos, int font_size, void* color) {
		try {
			cv::viz::WText* ret = new cv::viz::WText(cv::String(text), *pos, font_size, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_WText_setText_const_StringX(void* instance, const char* text) {
		try {
			reinterpret_cast<cv::viz::WText*>(instance)->setText(cv::String(text));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_WText_getText_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::viz::WText*>(instance)->getText();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WText3D_delete(cv::viz::WText3D* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WText3D_WText3D_const_StringX_const_Point3dX_double_bool_const_ColorX(const char* text, const cv::Point3d* position, double text_scale, bool face_camera, void* color) {
		try {
			cv::viz::WText3D* ret = new cv::viz::WText3D(cv::String(text), *position, text_scale, face_camera, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_WText3D_setText_const_StringX(void* instance, const char* text) {
		try {
			reinterpret_cast<cv::viz::WText3D*>(instance)->setText(cv::String(text));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_viz_WText3D_getText_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::viz::WText3D*>(instance)->getText();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WTrajectory_delete(cv::viz::WTrajectory* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WTrajectory_WTrajectory_const__InputArrayX_int_double_const_ColorX(void* path, int display_mode, double scale, void* color) {
		try {
			cv::viz::WTrajectory* ret = new cv::viz::WTrajectory(*reinterpret_cast<const cv::_InputArray*>(path), display_mode, scale, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WTrajectoryFrustums_delete(cv::viz::WTrajectoryFrustums* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayX_const_Vec2dX_double_const_ColorX(void* path, const cv::Vec2d* fov, double scale, void* color) {
		try {
			cv::viz::WTrajectoryFrustums* ret = new cv::viz::WTrajectoryFrustums(*reinterpret_cast<const cv::_InputArray*>(path), *fov, scale, *reinterpret_cast<const cv::viz::Color*>(color));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WTrajectorySpheres_delete(cv::viz::WTrajectorySpheres* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WTrajectorySpheres_WTrajectorySpheres_const__InputArrayX_double_double_const_ColorX_const_ColorX(void* path, double line_length, double radius, void* from, void* to) {
		try {
			cv::viz::WTrajectorySpheres* ret = new cv::viz::WTrajectorySpheres(*reinterpret_cast<const cv::_InputArray*>(path), line_length, radius, *reinterpret_cast<const cv::viz::Color*>(from), *reinterpret_cast<const cv::viz::Color*>(to));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_WWidgetMerger_delete(cv::viz::WWidgetMerger* instance) {
		delete instance;
	}
	Result<void*> cv_viz_WWidgetMerger_WWidgetMerger() {
		try {
			cv::viz::WWidgetMerger* ret = new cv::viz::WWidgetMerger();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_WWidgetMerger_finalize(void* instance) {
		try {
			reinterpret_cast<cv::viz::WWidgetMerger*>(instance)->finalize();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Widget_delete(cv::viz::Widget* instance) {
		delete instance;
	}
	Result<void*> cv_viz_Widget_Widget() {
		try {
			cv::viz::Widget* ret = new cv::viz::Widget();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Widget_Widget_const_WidgetX(void* other) {
		try {
			cv::viz::Widget* ret = new cv::viz::Widget(*reinterpret_cast<const cv::viz::Widget*>(other));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_viz_Widget_fromPlyFile_const_StringX(const char* file_name) {
		try {
			cv::viz::Widget ret = cv::viz::Widget::fromPlyFile(cv::String(file_name));
			return Ok<void*>(new cv::viz::Widget(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Widget_setRenderingProperty_int_double(void* instance, int property, double value) {
		try {
			reinterpret_cast<cv::viz::Widget*>(instance)->setRenderingProperty(property, value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_viz_Widget_getRenderingProperty_const_int(void* instance, int property) {
		try {
			double ret = reinterpret_cast<cv::viz::Widget*>(instance)->getRenderingProperty(property);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	void cv_Widget2D_delete(cv::viz::Widget2D* instance) {
		delete instance;
	}
	Result<void*> cv_viz_Widget2D_Widget2D() {
		try {
			cv::viz::Widget2D* ret = new cv::viz::Widget2D();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Widget2D_setColor_const_ColorX(void* instance, void* color) {
		try {
			reinterpret_cast<cv::viz::Widget2D*>(instance)->setColor(*reinterpret_cast<const cv::viz::Color*>(color));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Widget3D_delete(cv::viz::Widget3D* instance) {
		delete instance;
	}
	Result<void*> cv_viz_Widget3D_Widget3D() {
		try {
			cv::viz::Widget3D* ret = new cv::viz::Widget3D();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_viz_Widget3D_setColor_const_ColorX(void* instance, void* color) {
		try {
			reinterpret_cast<cv::viz::Widget3D*>(instance)->setColor(*reinterpret_cast<const cv::viz::Color*>(color));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}

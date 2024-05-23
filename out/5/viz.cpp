#include "ocvrs_common.hpp"
#include <opencv2/viz.hpp>
#include "viz_types.hpp"

extern "C" {
	// computeNormals(const Mesh &, OutputArray)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:214
	// ("cv::viz::computeNormals", vec![(pred!(mut, ["mesh", "normals"], ["const cv::viz::Mesh*", "const cv::_OutputArray*"]), _)]),
	void cv_viz_computeNormals_const_MeshR_const__OutputArrayR(const cv::viz::Mesh* mesh, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			cv::viz::computeNormals(*mesh, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowByName(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:97
	// ("cv::viz::getWindowByName", vec![(pred!(mut, ["window_name"], ["const cv::String*"]), _)]),
	void cv_viz_getWindowByName_const_StringR(const char* window_name, Result<cv::viz::Viz3d*>* ocvrs_return) {
		try {
			cv::viz::Viz3d ret = cv::viz::getWindowByName(std::string(window_name));
			Ok(new cv::viz::Viz3d(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::imshow(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:103
	// ("cv::viz::imshow", vec![(pred!(mut, ["window_name", "image"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_viz_imshow_const_StringR_const__InputArrayR(const char* window_name, const cv::_InputArray* image, Result<cv::viz::Viz3d*>* ocvrs_return) {
		try {
			cv::viz::Viz3d ret = cv::viz::imshow(std::string(window_name), *image);
			Ok(new cv::viz::Viz3d(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imshow(const String &, InputArray, const Size &)(InString, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:103
	// ("cv::viz::imshow", vec![(pred!(mut, ["window_name", "image", "window_size"], ["const cv::String*", "const cv::_InputArray*", "const cv::Size*"]), _)]),
	void cv_viz_imshow_const_StringR_const__InputArrayR_const_SizeR(const char* window_name, const cv::_InputArray* image, const cv::Size* window_size, Result<cv::viz::Viz3d*>* ocvrs_return) {
		try {
			cv::viz::Viz3d ret = cv::viz::imshow(std::string(window_name), *image, *window_size);
			Ok(new cv::viz::Viz3d(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// makeCameraPose(const Vec3d &, const Vec3d &, const Vec3d &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:84
	// ("cv::viz::makeCameraPose", vec![(pred!(mut, ["position", "focal_point", "y_dir"], ["const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
	void cv_viz_makeCameraPose_const_Vec3dR_const_Vec3dR_const_Vec3dR(const cv::Vec3d* position, const cv::Vec3d* focal_point, const cv::Vec3d* y_dir, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = cv::viz::makeCameraPose(*position, *focal_point, *y_dir);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::makeTransformToGlobal(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:73
	// ("cv::viz::makeTransformToGlobal", vec![(pred!(mut, ["axis_x", "axis_y", "axis_z"], ["const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
	void cv_viz_makeTransformToGlobal_const_Vec3dR_const_Vec3dR_const_Vec3dR(const cv::Vec3d* axis_x, const cv::Vec3d* axis_y, const cv::Vec3d* axis_z, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = cv::viz::makeTransformToGlobal(*axis_x, *axis_y, *axis_z);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// makeTransformToGlobal(const Vec3d &, const Vec3d &, const Vec3d &, const Vec3d &)(SimpleClass, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:73
	// ("cv::viz::makeTransformToGlobal", vec![(pred!(mut, ["axis_x", "axis_y", "axis_z", "origin"], ["const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
	void cv_viz_makeTransformToGlobal_const_Vec3dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(const cv::Vec3d* axis_x, const cv::Vec3d* axis_y, const cv::Vec3d* axis_z, const cv::Vec3d* origin, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = cv::viz::makeTransformToGlobal(*axis_x, *axis_y, *axis_z, *origin);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::readCloud(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:160
	// ("cv::viz::readCloud", vec![(pred!(mut, ["file"], ["const cv::String*"]), _)]),
	void cv_viz_readCloud_const_StringR(const char* file, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::viz::readCloud(std::string(file));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readCloud(const String &, OutputArray, OutputArray)(InString, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:160
	// ("cv::viz::readCloud", vec![(pred!(mut, ["file", "colors", "normals"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_viz_readCloud_const_StringR_const__OutputArrayR_const__OutputArrayR(const char* file, const cv::_OutputArray* colors, const cv::_OutputArray* normals, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::viz::readCloud(std::string(file), *colors, *normals);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readMesh(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:165
	// ("cv::viz::readMesh", vec![(pred!(mut, ["file"], ["const cv::String*"]), _)]),
	void cv_viz_readMesh_const_StringR(const char* file, Result<cv::viz::Mesh*>* ocvrs_return) {
		try {
			cv::viz::Mesh ret = cv::viz::readMesh(std::string(file));
			Ok(new cv::viz::Mesh(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::readPose(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:175
	// ("cv::viz::readPose", vec![(pred!(mut, ["file", "pose"], ["const cv::String*", "cv::Affine3d*"]), _)]),
	void cv_viz_readPose_const_StringR_Affine3dR(const char* file, cv::Affine3d* pose, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::viz::readPose(std::string(file), *pose);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readPose(const String &, Affine3d &, const String &)(InString, SimpleClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:175
	// ("cv::viz::readPose", vec![(pred!(mut, ["file", "pose", "tag"], ["const cv::String*", "cv::Affine3d*", "const cv::String*"]), _)]),
	void cv_viz_readPose_const_StringR_Affine3dR_const_StringR(const char* file, cv::Affine3d* pose, const char* tag, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::viz::readPose(std::string(file), *pose, std::string(tag));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::readTrajectory(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:206
	// ("cv::viz::readTrajectory", vec![(pred!(mut, ["traj"], ["const cv::_OutputArray*"]), _)]),
	void cv_viz_readTrajectory_const__OutputArrayR(const cv::_OutputArray* traj, ResultVoid* ocvrs_return) {
		try {
			cv::viz::readTrajectory(*traj);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readTrajectory(OutputArray, const String &, int, int, const String &)(OutputArray, InString, Primitive, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:206
	// ("cv::viz::readTrajectory", vec![(pred!(mut, ["traj", "files_format", "start", "end", "tag"], ["const cv::_OutputArray*", "const cv::String*", "int", "int", "const cv::String*"]), _)]),
	void cv_viz_readTrajectory_const__OutputArrayR_const_StringR_int_int_const_StringR(const cv::_OutputArray* traj, const char* files_format, int start, int end, const char* tag, ResultVoid* ocvrs_return) {
		try {
			cv::viz::readTrajectory(*traj, std::string(files_format), start, end, std::string(tag));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unregisterAllWindows()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:100
	// ("cv::viz::unregisterAllWindows", vec![(pred!(mut, [], []), _)]),
	void cv_viz_unregisterAllWindows(ResultVoid* ocvrs_return) {
		try {
			cv::viz::unregisterAllWindows();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::writeCloud(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:151
	// ("cv::viz::writeCloud", vec![(pred!(mut, ["file", "cloud"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_viz_writeCloud_const_StringR_const__InputArrayR(const char* file, const cv::_InputArray* cloud, ResultVoid* ocvrs_return) {
		try {
			cv::viz::writeCloud(std::string(file), *cloud);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeCloud(const String &, InputArray, InputArray, InputArray, bool)(InString, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:151
	// ("cv::viz::writeCloud", vec![(pred!(mut, ["file", "cloud", "colors", "normals", "binary"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
	void cv_viz_writeCloud_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR_bool(const char* file, const cv::_InputArray* cloud, const cv::_InputArray* colors, const cv::_InputArray* normals, bool binary, ResultVoid* ocvrs_return) {
		try {
			cv::viz::writeCloud(std::string(file), *cloud, *colors, *normals, binary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::writePose(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:181
	// ("cv::viz::writePose", vec![(pred!(mut, ["file", "pose"], ["const cv::String*", "const cv::Affine3d*"]), _)]),
	void cv_viz_writePose_const_StringR_const_Affine3dR(const char* file, const cv::Affine3d* pose, ResultVoid* ocvrs_return) {
		try {
			cv::viz::writePose(std::string(file), *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writePose(const String &, const Affine3d &, const String &)(InString, SimpleClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:181
	// ("cv::viz::writePose", vec![(pred!(mut, ["file", "pose", "tag"], ["const cv::String*", "const cv::Affine3d*", "const cv::String*"]), _)]),
	void cv_viz_writePose_const_StringR_const_Affine3dR_const_StringR(const char* file, const cv::Affine3d* pose, const char* tag, ResultVoid* ocvrs_return) {
		try {
			cv::viz::writePose(std::string(file), *pose, std::string(tag));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::writeTrajectory(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:193
	// ("cv::viz::writeTrajectory", vec![(pred!(mut, ["traj"], ["const cv::_InputArray*"]), _)]),
	void cv_viz_writeTrajectory_const__InputArrayR(const cv::_InputArray* traj, ResultVoid* ocvrs_return) {
		try {
			cv::viz::writeTrajectory(*traj);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeTrajectory(InputArray, const String &, int, const String &)(InputArray, InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/vizcore.hpp:193
	// ("cv::viz::writeTrajectory", vec![(pred!(mut, ["traj", "files_format", "start", "tag"], ["const cv::_InputArray*", "const cv::String*", "int", "const cv::String*"]), _)]),
	void cv_viz_writeTrajectory_const__InputArrayR_const_StringR_int_const_StringR(const cv::_InputArray* traj, const char* files_format, int start, const char* tag, ResultVoid* ocvrs_return) {
		try {
			cv::viz::writeTrajectory(*traj, std::string(files_format), start, std::string(tag));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Camera(double, double, double, double, const Size &)(Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:176
	// ("cv::viz::Camera::Camera", vec![(pred!(mut, ["fx", "fy", "cx", "cy", "window_size"], ["double", "double", "double", "double", "const cv::Size*"]), _)]),
	void cv_viz_Camera_Camera_double_double_double_double_const_SizeR(double fx, double fy, double cx, double cy, const cv::Size* window_size, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(fx, fy, cx, cy, *window_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Camera(const Vec2d &, const Size &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:183
	// ("cv::viz::Camera::Camera", vec![(pred!(mut, ["fov", "window_size"], ["const cv::Vec2d*", "const cv::Size*"]), _)]),
	void cv_viz_Camera_Camera_const_Vec2dR_const_SizeR(const cv::Vec2d* fov, const cv::Size* window_size, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(*fov, *window_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Camera(const Matx33d &, const Size &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:197
	// ("cv::viz::Camera::Camera", vec![(pred!(mut, ["K", "window_size"], ["const cv::Matx33d*", "const cv::Size*"]), _)]),
	void cv_viz_Camera_Camera_const_Matx33dR_const_SizeR(const cv::Matx33d* K, const cv::Size* window_size, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(*K, *window_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Camera(const Matx44d &, const Size &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:213
	// ("cv::viz::Camera::Camera", vec![(pred!(mut, ["proj", "window_size"], ["const cv::Matx44d*", "const cv::Size*"]), _)]),
	void cv_viz_Camera_Camera_const_Matx44dR_const_SizeR(const cv::Matx44d* proj, const cv::Size* window_size, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(*proj, *window_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClip()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:215
	// ("cv::viz::Camera::getClip", vec![(pred!(const, [], []), _)]),
	void cv_viz_Camera_getClip_const(const cv::viz::Camera* instance, Result<cv::Vec2d>* ocvrs_return) {
		try {
			const cv::Vec2d ret = instance->getClip();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setClip(const Vec2d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:216
	// ("cv::viz::Camera::setClip", vec![(pred!(mut, ["clip"], ["const cv::Vec2d*"]), _)]),
	void cv_viz_Camera_setClip_const_Vec2dR(cv::viz::Camera* instance, const cv::Vec2d* clip, ResultVoid* ocvrs_return) {
		try {
			instance->setClip(*clip);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:218
	// ("cv::viz::Camera::getWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_viz_Camera_getWindowSize_const(const cv::viz::Camera* instance, Result<cv::Size>* ocvrs_return) {
		try {
			const cv::Size ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowSize(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:219
	// ("cv::viz::Camera::setWindowSize", vec![(pred!(mut, ["window_size"], ["const cv::Size*"]), _)]),
	void cv_viz_Camera_setWindowSize_const_SizeR(cv::viz::Camera* instance, const cv::Size* window_size, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowSize(*window_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFov()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:221
	// ("cv::viz::Camera::getFov", vec![(pred!(const, [], []), _)]),
	void cv_viz_Camera_getFov_const(const cv::viz::Camera* instance, Result<cv::Vec2d>* ocvrs_return) {
		try {
			const cv::Vec2d ret = instance->getFov();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFov(const Vec2d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:222
	// ("cv::viz::Camera::setFov", vec![(pred!(mut, ["fov"], ["const cv::Vec2d*"]), _)]),
	void cv_viz_Camera_setFov_const_Vec2dR(cv::viz::Camera* instance, const cv::Vec2d* fov, ResultVoid* ocvrs_return) {
		try {
			instance->setFov(*fov);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPrincipalPoint()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:224
	// ("cv::viz::Camera::getPrincipalPoint", vec![(pred!(const, [], []), _)]),
	void cv_viz_Camera_getPrincipalPoint_const(const cv::viz::Camera* instance, Result<cv::Vec2d>* ocvrs_return) {
		try {
			const cv::Vec2d ret = instance->getPrincipalPoint();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFocalLength()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:225
	// ("cv::viz::Camera::getFocalLength", vec![(pred!(const, [], []), _)]),
	void cv_viz_Camera_getFocalLength_const(const cv::viz::Camera* instance, Result<cv::Vec2d>* ocvrs_return) {
		try {
			const cv::Vec2d ret = instance->getFocalLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeProjectionMatrix(Matx44d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:240
	// ("cv::viz::Camera::computeProjectionMatrix", vec![(pred!(const, ["proj"], ["cv::Matx44d*"]), _)]),
	void cv_viz_Camera_computeProjectionMatrix_const_Matx44dR(const cv::viz::Camera* instance, cv::Matx44d* proj, ResultVoid* ocvrs_return) {
		try {
			instance->computeProjectionMatrix(*proj);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// KinectCamera(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:250
	// ("cv::viz::Camera::KinectCamera", vec![(pred!(mut, ["window_size"], ["const cv::Size*"]), _)]),
	void cv_viz_Camera_KinectCamera_const_SizeR(const cv::Size* window_size, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera ret = cv::viz::Camera::KinectCamera(*window_size);
			Ok(new cv::viz::Camera(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Camera::delete() generated
	// ("cv::viz::Camera::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Camera_delete(cv::viz::Camera* instance) {
			delete instance;
	}

	// Color()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:66
	// ("cv::viz::Color::Color", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_Color(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color* ret = new cv::viz::Color();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Color(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:68
	// ("cv::viz::Color::Color", vec![(pred!(mut, ["gray"], ["double"]), _)]),
	void cv_viz_Color_Color_double(double gray, Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(gray);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Color(double, double, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:69
	// ("cv::viz::Color::Color", vec![(pred!(mut, ["blue", "green", "red"], ["double", "double", "double"]), _)]),
	void cv_viz_Color_Color_double_double_double(double blue, double green, double red, Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(blue, green, red);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Color(const Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:71
	// ("cv::viz::Color::Color", vec![(pred!(mut, ["color"], ["const cv::Scalar*"]), _)]),
	void cv_viz_Color_Color_const_ScalarR(const cv::Scalar* color, Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(*color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator Vec()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:73
	// ("cv::viz::Color::operator cv::Vec3b", vec![(pred!(const, [], []), _)]),
	void cv_viz_Color_operator_cv_Vec3b_const(const cv::viz::Color* instance, Result<cv::Vec3b>* ocvrs_return) {
		try {
			cv::Vec3b ret = instance->operator cv::Vec3b();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// black()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:75
	// ("cv::viz::Color::black", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_black(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::black();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blue()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:76
	// ("cv::viz::Color::blue", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_blue(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::blue();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// green()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:77
	// ("cv::viz::Color::green", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_green(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::green();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// red()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:78
	// ("cv::viz::Color::red", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_red(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::red();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cyan()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:79
	// ("cv::viz::Color::cyan", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_cyan(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::cyan();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// yellow()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:80
	// ("cv::viz::Color::yellow", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_yellow(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::yellow();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// magenta()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:81
	// ("cv::viz::Color::magenta", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_magenta(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::magenta();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// white()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:82
	// ("cv::viz::Color::white", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_white(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::white();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// gray()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:84
	// ("cv::viz::Color::gray", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_gray(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::gray();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// silver()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:85
	// ("cv::viz::Color::silver", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_silver(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::silver();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mlab()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:87
	// ("cv::viz::Color::mlab", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_mlab(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::mlab();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// navy()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:89
	// ("cv::viz::Color::navy", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_navy(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::navy();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maroon()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:90
	// ("cv::viz::Color::maroon", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_maroon(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::maroon();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// teal()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:91
	// ("cv::viz::Color::teal", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_teal(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::teal();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// olive()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:92
	// ("cv::viz::Color::olive", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_olive(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::olive();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// purple()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:93
	// ("cv::viz::Color::purple", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_purple(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::purple();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// azure()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:94
	// ("cv::viz::Color::azure", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_azure(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::azure();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// chartreuse()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:95
	// ("cv::viz::Color::chartreuse", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_chartreuse(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::chartreuse();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:96
	// ("cv::viz::Color::rose", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_rose(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::rose();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// lime()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:98
	// ("cv::viz::Color::lime", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_lime(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::lime();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// gold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:99
	// ("cv::viz::Color::gold", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_gold(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::gold();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// orange()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:100
	// ("cv::viz::Color::orange", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_orange(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::orange();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// orange_red()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:101
	// ("cv::viz::Color::orange_red", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_orange_red(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::orange_red();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// indigo()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:102
	// ("cv::viz::Color::indigo", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_indigo(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::indigo();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// brown()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:104
	// ("cv::viz::Color::brown", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_brown(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::brown();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apricot()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:105
	// ("cv::viz::Color::apricot", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_apricot(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::apricot();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pink()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:106
	// ("cv::viz::Color::pink", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_pink(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::pink();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// raspberry()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:107
	// ("cv::viz::Color::raspberry", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_raspberry(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::raspberry();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cherry()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:108
	// ("cv::viz::Color::cherry", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_cherry(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::cherry();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// violet()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:109
	// ("cv::viz::Color::violet", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_violet(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::violet();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// amethyst()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:110
	// ("cv::viz::Color::amethyst", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_amethyst(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::amethyst();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bluberry()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:111
	// ("cv::viz::Color::bluberry", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_bluberry(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::bluberry();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// celestial_blue()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:112
	// ("cv::viz::Color::celestial_blue", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_celestial_blue(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::celestial_blue();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// turquoise()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:113
	// ("cv::viz::Color::turquoise", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_turquoise(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::turquoise();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// not_set()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:115
	// ("cv::viz::Color::not_set", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_not_set(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::not_set();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Color::delete() generated
	// ("cv::viz::Color::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Color_delete(cv::viz::Color* instance) {
			delete instance;
	}

	// KeyboardEvent(Action, const String &, unsigned char, int)(Enum, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:300
	// ("cv::viz::KeyboardEvent::KeyboardEvent", vec![(pred!(mut, ["action", "symbol", "code", "modifiers"], ["cv::viz::KeyboardEvent::Action", "const cv::String*", "unsigned char", "int"]), _)]),
	void cv_viz_KeyboardEvent_KeyboardEvent_Action_const_StringR_unsigned_char_int(cv::viz::KeyboardEvent::Action action, const char* symbol, unsigned char code, int modifiers, Result<cv::viz::KeyboardEvent*>* ocvrs_return) {
		try {
			cv::viz::KeyboardEvent* ret = new cv::viz::KeyboardEvent(action, std::string(symbol), code, modifiers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::KeyboardEvent::action() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:302
	// ("cv::viz::KeyboardEvent::action", vec![(pred!(const, [], []), _)]),
	void cv_viz_KeyboardEvent_propAction_const(const cv::viz::KeyboardEvent* instance, cv::viz::KeyboardEvent::Action* ocvrs_return) {
			cv::viz::KeyboardEvent::Action ret = instance->action;
			*ocvrs_return = ret;
	}

	// cv::viz::KeyboardEvent::setAction(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:302
	// ("cv::viz::KeyboardEvent::setAction", vec![(pred!(mut, ["val"], ["const cv::viz::KeyboardEvent::Action"]), _)]),
	void cv_viz_KeyboardEvent_propAction_const_Action(cv::viz::KeyboardEvent* instance, const cv::viz::KeyboardEvent::Action val) {
			instance->action = val;
	}

	// cv::viz::KeyboardEvent::symbol() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:303
	// ("cv::viz::KeyboardEvent::symbol", vec![(pred!(const, [], []), _)]),
	void* cv_viz_KeyboardEvent_propSymbol_const(const cv::viz::KeyboardEvent* instance) {
			cv::String ret = instance->symbol;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::viz::KeyboardEvent::setSymbol(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:303
	// ("cv::viz::KeyboardEvent::setSymbol", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_viz_KeyboardEvent_propSymbol_const_String(cv::viz::KeyboardEvent* instance, const char* val) {
			instance->symbol = std::string(val);
	}

	// cv::viz::KeyboardEvent::code() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:304
	// ("cv::viz::KeyboardEvent::code", vec![(pred!(const, [], []), _)]),
	unsigned char cv_viz_KeyboardEvent_propCode_const(const cv::viz::KeyboardEvent* instance) {
			unsigned char ret = instance->code;
			return ret;
	}

	// cv::viz::KeyboardEvent::setCode(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:304
	// ("cv::viz::KeyboardEvent::setCode", vec![(pred!(mut, ["val"], ["const unsigned char"]), _)]),
	void cv_viz_KeyboardEvent_propCode_const_unsigned_char(cv::viz::KeyboardEvent* instance, const unsigned char val) {
			instance->code = val;
	}

	// cv::viz::KeyboardEvent::modifiers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:305
	// ("cv::viz::KeyboardEvent::modifiers", vec![(pred!(const, [], []), _)]),
	int cv_viz_KeyboardEvent_propModifiers_const(const cv::viz::KeyboardEvent* instance) {
			int ret = instance->modifiers;
			return ret;
	}

	// cv::viz::KeyboardEvent::setModifiers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:305
	// ("cv::viz::KeyboardEvent::setModifiers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_viz_KeyboardEvent_propModifiers_const_int(cv::viz::KeyboardEvent* instance, const int val) {
			instance->modifiers = val;
	}

	// cv::viz::KeyboardEvent::delete() generated
	// ("cv::viz::KeyboardEvent::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_KeyboardEvent_delete(cv::viz::KeyboardEvent* instance) {
			delete instance;
	}

	// Mesh()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:140
	// ("cv::viz::Mesh::Mesh", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Mesh_Mesh(Result<cv::viz::Mesh*>* ocvrs_return) {
		try {
			cv::viz::Mesh* ret = new cv::viz::Mesh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:154
	// ("cv::viz::Mesh::load", vec![(pred!(mut, ["file", "type"], ["const cv::String*", "int"]), _)]),
	void cv_viz_Mesh_load_const_StringR_int(const char* file, int type, Result<cv::viz::Mesh*>* ocvrs_return) {
		try {
			cv::viz::Mesh ret = cv::viz::Mesh::load(std::string(file), type);
			Ok(new cv::viz::Mesh(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Mesh::load(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:154
	// ("cv::viz::Mesh::load", vec![(pred!(mut, ["file"], ["const cv::String*"]), _)]),
	void cv_viz_Mesh_load_const_StringR(const char* file, Result<cv::viz::Mesh*>* ocvrs_return) {
		try {
			cv::viz::Mesh ret = cv::viz::Mesh::load(std::string(file));
			Ok(new cv::viz::Mesh(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Mesh::implicitClone() generated
	// ("cv::viz::Mesh::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::viz::Mesh* cv_viz_Mesh_implicitClone_const(const cv::viz::Mesh* instance) {
			return new cv::viz::Mesh(*instance);
	}

	// cv::viz::Mesh::cloud() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:129
	// ("cv::viz::Mesh::cloud", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_viz_Mesh_propCloud_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->cloud;
			return new cv::Mat(ret);
	}

	// cv::viz::Mesh::setCloud(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:129
	// ("cv::viz::Mesh::setCloud", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_viz_Mesh_propCloud_const_Mat(cv::viz::Mesh* instance, const cv::Mat* val) {
			instance->cloud = *val;
	}

	// cv::viz::Mesh::colors() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:130
	// ("cv::viz::Mesh::colors", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_viz_Mesh_propColors_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->colors;
			return new cv::Mat(ret);
	}

	// cv::viz::Mesh::setColors(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:130
	// ("cv::viz::Mesh::setColors", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_viz_Mesh_propColors_const_Mat(cv::viz::Mesh* instance, const cv::Mat* val) {
			instance->colors = *val;
	}

	// cv::viz::Mesh::normals() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:131
	// ("cv::viz::Mesh::normals", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_viz_Mesh_propNormals_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->normals;
			return new cv::Mat(ret);
	}

	// cv::viz::Mesh::setNormals(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:131
	// ("cv::viz::Mesh::setNormals", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_viz_Mesh_propNormals_const_Mat(cv::viz::Mesh* instance, const cv::Mat* val) {
			instance->normals = *val;
	}

	// cv::viz::Mesh::polygons() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:135
	// ("cv::viz::Mesh::polygons", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_viz_Mesh_propPolygons_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->polygons;
			return new cv::Mat(ret);
	}

	// cv::viz::Mesh::setPolygons(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:135
	// ("cv::viz::Mesh::setPolygons", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_viz_Mesh_propPolygons_const_Mat(cv::viz::Mesh* instance, const cv::Mat* val) {
			instance->polygons = *val;
	}

	// cv::viz::Mesh::texture() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:137
	// ("cv::viz::Mesh::texture", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_viz_Mesh_propTexture_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->texture;
			return new cv::Mat(ret);
	}

	// cv::viz::Mesh::setTexture(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:137
	// ("cv::viz::Mesh::setTexture", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_viz_Mesh_propTexture_const_Mat(cv::viz::Mesh* instance, const cv::Mat* val) {
			instance->texture = *val;
	}

	// cv::viz::Mesh::tcoords() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:138
	// ("cv::viz::Mesh::tcoords", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_viz_Mesh_propTcoords_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->tcoords;
			return new cv::Mat(ret);
	}

	// cv::viz::Mesh::setTcoords(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:138
	// ("cv::viz::Mesh::setTcoords", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_viz_Mesh_propTcoords_const_Mat(cv::viz::Mesh* instance, const cv::Mat* val) {
			instance->tcoords = *val;
	}

	// cv::viz::Mesh::delete() generated
	// ("cv::viz::Mesh::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Mesh_delete(cv::viz::Mesh* instance) {
			delete instance;
	}

	// MouseEvent(const Type &, const MouseButton &, const Point &, int)(Enum, Enum, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:325
	// ("cv::viz::MouseEvent::MouseEvent", vec![(pred!(mut, ["type", "button", "pointer", "modifiers"], ["const cv::viz::MouseEvent::Type*", "const cv::viz::MouseEvent::MouseButton*", "const cv::Point*", "int"]), _)]),
	void cv_viz_MouseEvent_MouseEvent_const_TypeR_const_MouseButtonR_const_PointR_int(const cv::viz::MouseEvent::Type type, const cv::viz::MouseEvent::MouseButton button, const cv::Point* pointer, int modifiers, Result<cv::viz::MouseEvent*>* ocvrs_return) {
		try {
			cv::viz::MouseEvent* ret = new cv::viz::MouseEvent(type, button, *pointer, modifiers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::MouseEvent::type() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:327
	// ("cv::viz::MouseEvent::type", vec![(pred!(const, [], []), _)]),
	void cv_viz_MouseEvent_propType_const(const cv::viz::MouseEvent* instance, cv::viz::MouseEvent::Type* ocvrs_return) {
			cv::viz::MouseEvent::Type ret = instance->type;
			*ocvrs_return = ret;
	}

	// cv::viz::MouseEvent::setType(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:327
	// ("cv::viz::MouseEvent::setType", vec![(pred!(mut, ["val"], ["const cv::viz::MouseEvent::Type"]), _)]),
	void cv_viz_MouseEvent_propType_const_Type(cv::viz::MouseEvent* instance, const cv::viz::MouseEvent::Type val) {
			instance->type = val;
	}

	// cv::viz::MouseEvent::button() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:328
	// ("cv::viz::MouseEvent::button", vec![(pred!(const, [], []), _)]),
	void cv_viz_MouseEvent_propButton_const(const cv::viz::MouseEvent* instance, cv::viz::MouseEvent::MouseButton* ocvrs_return) {
			cv::viz::MouseEvent::MouseButton ret = instance->button;
			*ocvrs_return = ret;
	}

	// cv::viz::MouseEvent::setButton(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:328
	// ("cv::viz::MouseEvent::setButton", vec![(pred!(mut, ["val"], ["const cv::viz::MouseEvent::MouseButton"]), _)]),
	void cv_viz_MouseEvent_propButton_const_MouseButton(cv::viz::MouseEvent* instance, const cv::viz::MouseEvent::MouseButton val) {
			instance->button = val;
	}

	// cv::viz::MouseEvent::pointer() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:329
	// ("cv::viz::MouseEvent::pointer", vec![(pred!(const, [], []), _)]),
	void cv_viz_MouseEvent_propPointer_const(const cv::viz::MouseEvent* instance, cv::Point* ocvrs_return) {
			cv::Point ret = instance->pointer;
			*ocvrs_return = ret;
	}

	// cv::viz::MouseEvent::setPointer(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:329
	// ("cv::viz::MouseEvent::setPointer", vec![(pred!(mut, ["val"], ["const cv::Point"]), _)]),
	void cv_viz_MouseEvent_propPointer_const_Point(cv::viz::MouseEvent* instance, const cv::Point* val) {
			instance->pointer = *val;
	}

	// cv::viz::MouseEvent::modifiers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:330
	// ("cv::viz::MouseEvent::modifiers", vec![(pred!(const, [], []), _)]),
	int cv_viz_MouseEvent_propModifiers_const(const cv::viz::MouseEvent* instance) {
			int ret = instance->modifiers;
			return ret;
	}

	// cv::viz::MouseEvent::setModifiers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/types.hpp:330
	// ("cv::viz::MouseEvent::setModifiers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_viz_MouseEvent_propModifiers_const_int(cv::viz::MouseEvent* instance, const int val) {
			instance->modifiers = val;
	}

	// cv::viz::MouseEvent::delete() generated
	// ("cv::viz::MouseEvent::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_MouseEvent_delete(cv::viz::MouseEvent* instance) {
			delete instance;
	}

	// Viz3d(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:78
	// ("cv::viz::Viz3d::Viz3d", vec![(pred!(mut, ["window_name"], ["const cv::String*"]), _)]),
	void cv_viz_Viz3d_Viz3d_const_StringR(const char* window_name, Result<cv::viz::Viz3d*>* ocvrs_return) {
		try {
			cv::viz::Viz3d* ret = new cv::viz::Viz3d(std::string(window_name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Viz3d::Viz3d() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:78
	// ("cv::viz::Viz3d::Viz3d", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_Viz3d(Result<cv::viz::Viz3d*>* ocvrs_return) {
		try {
			cv::viz::Viz3d* ret = new cv::viz::Viz3d();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Viz3d(const Viz3d &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:79
	// ("cv::viz::Viz3d::Viz3d", vec![(pred!(mut, ["unnamed"], ["const cv::viz::Viz3d*"]), _)]),
	void cv_viz_Viz3d_Viz3d_const_Viz3dR(const cv::viz::Viz3d* unnamed, Result<cv::viz::Viz3d*>* ocvrs_return) {
		try {
			cv::viz::Viz3d* ret = new cv::viz::Viz3d(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Viz3d &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:80
	// ("cv::viz::Viz3d::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::viz::Viz3d*"]), _)]),
	void cv_viz_Viz3d_operatorST_const_Viz3dR(cv::viz::Viz3d* instance, const cv::viz::Viz3d* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// showWidget(const String &, const Widget &, const Affine3d &)(InString, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:88
	// ("cv::viz::Viz3d::showWidget", vec![(pred!(mut, ["id", "widget", "pose"], ["const cv::String*", "const cv::viz::Widget*", "const cv::Affine3d*"]), _)]),
	void cv_viz_Viz3d_showWidget_const_StringR_const_WidgetR_const_Affine3dR(cv::viz::Viz3d* instance, const char* id, const cv::viz::Widget* widget, const cv::Affine3d* pose, ResultVoid* ocvrs_return) {
		try {
			instance->showWidget(std::string(id), *widget, *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Viz3d::showWidget(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:88
	// ("cv::viz::Viz3d::showWidget", vec![(pred!(mut, ["id", "widget"], ["const cv::String*", "const cv::viz::Widget*"]), _)]),
	void cv_viz_Viz3d_showWidget_const_StringR_const_WidgetR(cv::viz::Viz3d* instance, const char* id, const cv::viz::Widget* widget, ResultVoid* ocvrs_return) {
		try {
			instance->showWidget(std::string(id), *widget);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// removeWidget(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:94
	// ("cv::viz::Viz3d::removeWidget", vec![(pred!(mut, ["id"], ["const cv::String*"]), _)]),
	void cv_viz_Viz3d_removeWidget_const_StringR(cv::viz::Viz3d* instance, const char* id, ResultVoid* ocvrs_return) {
		try {
			instance->removeWidget(std::string(id));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWidget(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:103
	// ("cv::viz::Viz3d::getWidget", vec![(pred!(const, ["id"], ["const cv::String*"]), _)]),
	void cv_viz_Viz3d_getWidget_const_const_StringR(const cv::viz::Viz3d* instance, const char* id, Result<cv::viz::Widget*>* ocvrs_return) {
		try {
			cv::viz::Widget ret = instance->getWidget(std::string(id));
			Ok(new cv::viz::Widget(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// removeAllWidgets()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:107
	// ("cv::viz::Viz3d::removeAllWidgets", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_removeAllWidgets(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->removeAllWidgets();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// showImage(InputArray, const Size &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:114
	// ("cv::viz::Viz3d::showImage", vec![(pred!(mut, ["image", "window_size"], ["const cv::_InputArray*", "const cv::Size*"]), _)]),
	void cv_viz_Viz3d_showImage_const__InputArrayR_const_SizeR(cv::viz::Viz3d* instance, const cv::_InputArray* image, const cv::Size* window_size, ResultVoid* ocvrs_return) {
		try {
			instance->showImage(*image, *window_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Viz3d::showImage(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:114
	// ("cv::viz::Viz3d::showImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_viz_Viz3d_showImage_const__InputArrayR(cv::viz::Viz3d* instance, const cv::_InputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->showImage(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWidgetPose(const String &, const Affine3d &)(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:120
	// ("cv::viz::Viz3d::setWidgetPose", vec![(pred!(mut, ["id", "pose"], ["const cv::String*", "const cv::Affine3d*"]), _)]),
	void cv_viz_Viz3d_setWidgetPose_const_StringR_const_Affine3dR(cv::viz::Viz3d* instance, const char* id, const cv::Affine3d* pose, ResultVoid* ocvrs_return) {
		try {
			instance->setWidgetPose(std::string(id), *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateWidgetPose(const String &, const Affine3d &)(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:127
	// ("cv::viz::Viz3d::updateWidgetPose", vec![(pred!(mut, ["id", "pose"], ["const cv::String*", "const cv::Affine3d*"]), _)]),
	void cv_viz_Viz3d_updateWidgetPose_const_StringR_const_Affine3dR(cv::viz::Viz3d* instance, const char* id, const cv::Affine3d* pose, ResultVoid* ocvrs_return) {
		try {
			instance->updateWidgetPose(std::string(id), *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWidgetPose(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:133
	// ("cv::viz::Viz3d::getWidgetPose", vec![(pred!(const, ["id"], ["const cv::String*"]), _)]),
	void cv_viz_Viz3d_getWidgetPose_const_const_StringR(const cv::viz::Viz3d* instance, const char* id, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = instance->getWidgetPose(std::string(id));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCamera(const Camera &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:139
	// ("cv::viz::Viz3d::setCamera", vec![(pred!(mut, ["camera"], ["const cv::viz::Camera*"]), _)]),
	void cv_viz_Viz3d_setCamera_const_CameraR(cv::viz::Viz3d* instance, const cv::viz::Camera* camera, ResultVoid* ocvrs_return) {
		try {
			instance->setCamera(*camera);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCamera()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:143
	// ("cv::viz::Viz3d::getCamera", vec![(pred!(const, [], []), _)]),
	void cv_viz_Viz3d_getCamera_const(const cv::viz::Viz3d* instance, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera ret = instance->getCamera();
			Ok(new cv::viz::Camera(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getViewerPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:147
	// ("cv::viz::Viz3d::getViewerPose", vec![(pred!(const, [], []), _)]),
	void cv_viz_Viz3d_getViewerPose_const(const cv::viz::Viz3d* instance, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = instance->getViewerPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setViewerPose(const Affine3d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:153
	// ("cv::viz::Viz3d::setViewerPose", vec![(pred!(mut, ["pose"], ["const cv::Affine3d*"]), _)]),
	void cv_viz_Viz3d_setViewerPose_const_Affine3dR(cv::viz::Viz3d* instance, const cv::Affine3d* pose, ResultVoid* ocvrs_return) {
		try {
			instance->setViewerPose(*pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetCameraViewpoint(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:159
	// ("cv::viz::Viz3d::resetCameraViewpoint", vec![(pred!(mut, ["id"], ["const cv::String*"]), _)]),
	void cv_viz_Viz3d_resetCameraViewpoint_const_StringR(cv::viz::Viz3d* instance, const char* id, ResultVoid* ocvrs_return) {
		try {
			instance->resetCameraViewpoint(std::string(id));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetCamera()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:163
	// ("cv::viz::Viz3d::resetCamera", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_resetCamera(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->resetCamera();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertToWindowCoordinates(const Point3d &, Point3d &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:170
	// ("cv::viz::Viz3d::convertToWindowCoordinates", vec![(pred!(mut, ["pt", "window_coord"], ["const cv::Point3d*", "cv::Point3d*"]), _)]),
	void cv_viz_Viz3d_convertToWindowCoordinates_const_Point3dR_Point3dR(cv::viz::Viz3d* instance, const cv::Point3d* pt, cv::Point3d* window_coord, ResultVoid* ocvrs_return) {
		try {
			instance->convertToWindowCoordinates(*pt, *window_coord);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// converTo3DRay(const Point3d &, Point3d &, Vec3d &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:177
	// ("cv::viz::Viz3d::converTo3DRay", vec![(pred!(mut, ["window_coord", "origin", "direction"], ["const cv::Point3d*", "cv::Point3d*", "cv::Vec3d*"]), _)]),
	void cv_viz_Viz3d_converTo3DRay_const_Point3dR_Point3dR_Vec3dR(cv::viz::Viz3d* instance, const cv::Point3d* window_coord, cv::Point3d* origin, cv::Vec3d* direction, ResultVoid* ocvrs_return) {
		try {
			instance->converTo3DRay(*window_coord, *origin, *direction);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:181
	// ("cv::viz::Viz3d::getWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_viz_Viz3d_getWindowSize_const(const cv::viz::Viz3d* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowSize(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:186
	// ("cv::viz::Viz3d::setWindowSize", vec![(pred!(mut, ["window_size"], ["const cv::Size*"]), _)]),
	void cv_viz_Viz3d_setWindowSize_const_SizeR(cv::viz::Viz3d* instance, const cv::Size* window_size, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowSize(*window_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:191
	// ("cv::viz::Viz3d::getWindowName", vec![(pred!(const, [], []), _)]),
	void cv_viz_Viz3d_getWindowName_const(const cv::viz::Viz3d* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getWindowName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScreenshot()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:195
	// ("cv::viz::Viz3d::getScreenshot", vec![(pred!(const, [], []), _)]),
	void cv_viz_Viz3d_getScreenshot_const(const cv::viz::Viz3d* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getScreenshot();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// saveScreenshot(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:201
	// ("cv::viz::Viz3d::saveScreenshot", vec![(pred!(mut, ["file"], ["const cv::String*"]), _)]),
	void cv_viz_Viz3d_saveScreenshot_const_StringR(cv::viz::Viz3d* instance, const char* file, ResultVoid* ocvrs_return) {
		try {
			instance->saveScreenshot(std::string(file));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowPosition(const Point &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:207
	// ("cv::viz::Viz3d::setWindowPosition", vec![(pred!(mut, ["window_position"], ["const cv::Point*"]), _)]),
	void cv_viz_Viz3d_setWindowPosition_const_PointR(cv::viz::Viz3d* instance, const cv::Point* window_position, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowPosition(*window_position);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFullScreen(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:213
	// ("cv::viz::Viz3d::setFullScreen", vec![(pred!(mut, ["mode"], ["bool"]), _)]),
	void cv_viz_Viz3d_setFullScreen_bool(cv::viz::Viz3d* instance, bool mode, ResultVoid* ocvrs_return) {
		try {
			instance->setFullScreen(mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Viz3d::setFullScreen() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:213
	// ("cv::viz::Viz3d::setFullScreen", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_setFullScreen(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setFullScreen();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackgroundColor(const Color &, const Color &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:217
	// ("cv::viz::Viz3d::setBackgroundColor", vec![(pred!(mut, ["color", "color2"], ["const cv::viz::Viz3d::Color*", "const cv::viz::Viz3d::Color*"]), _)]),
	void cv_viz_Viz3d_setBackgroundColor_const_ColorR_const_ColorR(cv::viz::Viz3d* instance, const cv::viz::Viz3d::Color* color, const cv::viz::Viz3d::Color* color2, ResultVoid* ocvrs_return) {
		try {
			instance->setBackgroundColor(*color, *color2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Viz3d::setBackgroundColor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:217
	// ("cv::viz::Viz3d::setBackgroundColor", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_setBackgroundColor(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setBackgroundColor();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackgroundTexture(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:218
	// ("cv::viz::Viz3d::setBackgroundTexture", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_viz_Viz3d_setBackgroundTexture_const__InputArrayR(cv::viz::Viz3d* instance, const cv::_InputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->setBackgroundTexture(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Viz3d::setBackgroundTexture() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:218
	// ("cv::viz::Viz3d::setBackgroundTexture", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_setBackgroundTexture(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setBackgroundTexture();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackgroundMeshLab()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:219
	// ("cv::viz::Viz3d::setBackgroundMeshLab", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_setBackgroundMeshLab(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setBackgroundMeshLab();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// spin()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:223
	// ("cv::viz::Viz3d::spin", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_spin(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->spin();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// spinOnce(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:230
	// ("cv::viz::Viz3d::spinOnce", vec![(pred!(mut, ["time", "force_redraw"], ["int", "bool"]), _)]),
	void cv_viz_Viz3d_spinOnce_int_bool(cv::viz::Viz3d* instance, int time, bool force_redraw, ResultVoid* ocvrs_return) {
		try {
			instance->spinOnce(time, force_redraw);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Viz3d::spinOnce() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:230
	// ("cv::viz::Viz3d::spinOnce", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_spinOnce(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->spinOnce();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOffScreenRendering()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:234
	// ("cv::viz::Viz3d::setOffScreenRendering", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_setOffScreenRendering(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setOffScreenRendering();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// removeAllLights()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:238
	// ("cv::viz::Viz3d::removeAllLights", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_removeAllLights(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->removeAllLights();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addLight(const Vec3d &, const Vec3d &, const Color &, const Color &, const Color &, const Color &)(SimpleClass, SimpleClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:249
	// ("cv::viz::Viz3d::addLight", vec![(pred!(mut, ["position", "focalPoint", "color", "diffuseColor", "ambientColor", "specularColor"], ["const cv::Vec3d*", "const cv::Vec3d*", "const cv::viz::Viz3d::Color*", "const cv::viz::Viz3d::Color*", "const cv::viz::Viz3d::Color*", "const cv::viz::Viz3d::Color*"]), _)]),
	void cv_viz_Viz3d_addLight_const_Vec3dR_const_Vec3dR_const_ColorR_const_ColorR_const_ColorR_const_ColorR(cv::viz::Viz3d* instance, const cv::Vec3d* position, const cv::Vec3d* focalPoint, const cv::viz::Viz3d::Color* color, const cv::viz::Viz3d::Color* diffuseColor, const cv::viz::Viz3d::Color* ambientColor, const cv::viz::Viz3d::Color* specularColor, ResultVoid* ocvrs_return) {
		try {
			instance->addLight(*position, *focalPoint, *color, *diffuseColor, *ambientColor, *specularColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Viz3d::addLight(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:249
	// ("cv::viz::Viz3d::addLight", vec![(pred!(mut, ["position"], ["const cv::Vec3d*"]), _)]),
	void cv_viz_Viz3d_addLight_const_Vec3dR(cv::viz::Viz3d* instance, const cv::Vec3d* position, ResultVoid* ocvrs_return) {
		try {
			instance->addLight(*position);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// wasStopped()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:255
	// ("cv::viz::Viz3d::wasStopped", vec![(pred!(const, [], []), _)]),
	void cv_viz_Viz3d_wasStopped_const(const cv::viz::Viz3d* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->wasStopped();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// close()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:256
	// ("cv::viz::Viz3d::close", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_close(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->close();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerKeyboardCallback(KeyboardCallback, void *)(Function, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:264
	// ("cv::viz::Viz3d::registerKeyboardCallback", vec![(pred!(mut, ["callback", "cookie"], ["cv::viz::Viz3d::KeyboardCallback", "void*"]), _)]),
	void cv_viz_Viz3d_registerKeyboardCallback_KeyboardCallback_voidX(cv::viz::Viz3d* instance, cv::viz::Viz3d::KeyboardCallback callback, void* cookie, ResultVoid* ocvrs_return) {
		try {
			instance->registerKeyboardCallback(callback, cookie);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerMouseCallback(MouseCallback, void *)(Function, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:271
	// ("cv::viz::Viz3d::registerMouseCallback", vec![(pred!(mut, ["callback", "cookie"], ["cv::viz::Viz3d::MouseCallback", "void*"]), _)]),
	void cv_viz_Viz3d_registerMouseCallback_MouseCallback_voidX(cv::viz::Viz3d* instance, cv::viz::Viz3d::MouseCallback callback, void* cookie, ResultVoid* ocvrs_return) {
		try {
			instance->registerMouseCallback(callback, cookie);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRenderingProperty(const String &, int, double)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:299
	// ("cv::viz::Viz3d::setRenderingProperty", vec![(pred!(mut, ["id", "property", "value"], ["const cv::String*", "int", "double"]), _)]),
	void cv_viz_Viz3d_setRenderingProperty_const_StringR_int_double(cv::viz::Viz3d* instance, const char* id, int property, double value, ResultVoid* ocvrs_return) {
		try {
			instance->setRenderingProperty(std::string(id), property, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRenderingProperty(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:325
	// ("cv::viz::Viz3d::getRenderingProperty", vec![(pred!(mut, ["id", "property"], ["const cv::String*", "int"]), _)]),
	void cv_viz_Viz3d_getRenderingProperty_const_StringR_int(cv::viz::Viz3d* instance, const char* id, int property, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRenderingProperty(std::string(id), property);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRepresentation(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:334
	// ("cv::viz::Viz3d::setRepresentation", vec![(pred!(mut, ["representation"], ["int"]), _)]),
	void cv_viz_Viz3d_setRepresentation_int(cv::viz::Viz3d* instance, int representation, ResultVoid* ocvrs_return) {
		try {
			instance->setRepresentation(representation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGlobalWarnings(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:336
	// ("cv::viz::Viz3d::setGlobalWarnings", vec![(pred!(mut, ["enabled"], ["bool"]), _)]),
	void cv_viz_Viz3d_setGlobalWarnings_bool(cv::viz::Viz3d* instance, bool enabled, ResultVoid* ocvrs_return) {
		try {
			instance->setGlobalWarnings(enabled);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Viz3d::setGlobalWarnings() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/viz3d.hpp:336
	// ("cv::viz::Viz3d::setGlobalWarnings", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_setGlobalWarnings(cv::viz::Viz3d* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setGlobalWarnings();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Viz3d::delete() generated
	// ("cv::viz::Viz3d::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Viz3d_delete(cv::viz::Viz3d* instance) {
			delete instance;
	}

	// WArrow(const Point3d &, const Point3d &, double, const Color &)(SimpleClass, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:301
	// ("cv::viz::WArrow::WArrow", vec![(pred!(mut, ["pt1", "pt2", "thickness", "color"], ["const cv::Point3d*", "const cv::Point3d*", "double", "const cv::viz::Color*"]), _)]),
	void cv_viz_WArrow_WArrow_const_Point3dR_const_Point3dR_double_const_ColorR(const cv::Point3d* pt1, const cv::Point3d* pt2, double thickness, const cv::viz::Color* color, Result<cv::viz::WArrow*>* ocvrs_return) {
		try {
			cv::viz::WArrow* ret = new cv::viz::WArrow(*pt1, *pt2, thickness, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WArrow::WArrow(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:301
	// ("cv::viz::WArrow::WArrow", vec![(pred!(mut, ["pt1", "pt2"], ["const cv::Point3d*", "const cv::Point3d*"]), _)]),
	void cv_viz_WArrow_WArrow_const_Point3dR_const_Point3dR(const cv::Point3d* pt1, const cv::Point3d* pt2, Result<cv::viz::WArrow*>* ocvrs_return) {
		try {
			cv::viz::WArrow* ret = new cv::viz::WArrow(*pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WArrow::to_Widget() generated
	// ("cv::viz::WArrow::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WArrow_to_Widget(cv::viz::WArrow* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WArrow::to_Widget3D() generated
	// ("cv::viz::WArrow::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WArrow_to_Widget3D(cv::viz::WArrow* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WArrow::delete() generated
	// ("cv::viz::WArrow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WArrow_delete(cv::viz::WArrow* instance) {
			delete instance;
	}

	// WCameraPosition(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:550
	// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["scale"], ["double"]), _)]),
	void cv_viz_WCameraPosition_WCameraPosition_double(double scale, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCameraPosition::WCameraPosition() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:550
	// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCameraPosition_WCameraPosition(Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WCameraPosition(const Matx33d &, double, const Color &)(SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:560
	// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["K", "scale", "color"], ["const cv::Matx33d*", "double", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_double_const_ColorR(const cv::Matx33d* K, double scale, const cv::viz::Color* color, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*K, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCameraPosition::WCameraPosition(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:560
	// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["K"], ["const cv::Matx33d*"]), _)]),
	void cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR(const cv::Matx33d* K, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*K);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WCameraPosition(const Vec2d &, double, const Color &)(SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:570
	// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["fov", "scale", "color"], ["const cv::Vec2d*", "double", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_double_const_ColorR(const cv::Vec2d* fov, double scale, const cv::viz::Color* color, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*fov, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCameraPosition::WCameraPosition(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:570
	// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["fov"], ["const cv::Vec2d*"]), _)]),
	void cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR(const cv::Vec2d* fov, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*fov);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WCameraPosition(const Matx33d &, InputArray, double, const Color &)(SimpleClass, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:583
	// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["K", "image", "scale", "color"], ["const cv::Matx33d*", "const cv::_InputArray*", "double", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_const__InputArrayR_double_const_ColorR(const cv::Matx33d* K, const cv::_InputArray* image, double scale, const cv::viz::Color* color, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*K, *image, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCameraPosition::WCameraPosition(SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:583
	// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["K", "image"], ["const cv::Matx33d*", "const cv::_InputArray*"]), _)]),
	void cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_const__InputArrayR(const cv::Matx33d* K, const cv::_InputArray* image, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*K, *image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WCameraPosition(const Vec2d &, InputArray, double, const Color &)(SimpleClass, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:596
	// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["fov", "image", "scale", "color"], ["const cv::Vec2d*", "const cv::_InputArray*", "double", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_const__InputArrayR_double_const_ColorR(const cv::Vec2d* fov, const cv::_InputArray* image, double scale, const cv::viz::Color* color, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*fov, *image, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCameraPosition::WCameraPosition(SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:596
	// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["fov", "image"], ["const cv::Vec2d*", "const cv::_InputArray*"]), _)]),
	void cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_const__InputArrayR(const cv::Vec2d* fov, const cv::_InputArray* image, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*fov, *image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCameraPosition::to_Widget() generated
	// ("cv::viz::WCameraPosition::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WCameraPosition_to_Widget(cv::viz::WCameraPosition* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WCameraPosition::to_Widget3D() generated
	// ("cv::viz::WCameraPosition::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WCameraPosition_to_Widget3D(cv::viz::WCameraPosition* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WCameraPosition::delete() generated
	// ("cv::viz::WCameraPosition::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCameraPosition_delete(cv::viz::WCameraPosition* instance) {
			delete instance;
	}

	// WCircle(double, double, const Color &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:315
	// ("cv::viz::WCircle::WCircle", vec![(pred!(mut, ["radius", "thickness", "color"], ["double", "double", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCircle_WCircle_double_double_const_ColorR(double radius, double thickness, const cv::viz::Color* color, Result<cv::viz::WCircle*>* ocvrs_return) {
		try {
			cv::viz::WCircle* ret = new cv::viz::WCircle(radius, thickness, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCircle::WCircle(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:315
	// ("cv::viz::WCircle::WCircle", vec![(pred!(mut, ["radius"], ["double"]), _)]),
	void cv_viz_WCircle_WCircle_double(double radius, Result<cv::viz::WCircle*>* ocvrs_return) {
		try {
			cv::viz::WCircle* ret = new cv::viz::WCircle(radius);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WCircle(double, const Point3d &, const Vec3d &, double, const Color &)(Primitive, SimpleClass, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:325
	// ("cv::viz::WCircle::WCircle", vec![(pred!(mut, ["radius", "center", "normal", "thickness", "color"], ["double", "const cv::Point3d*", "const cv::Vec3d*", "double", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCircle_WCircle_double_const_Point3dR_const_Vec3dR_double_const_ColorR(double radius, const cv::Point3d* center, const cv::Vec3d* normal, double thickness, const cv::viz::Color* color, Result<cv::viz::WCircle*>* ocvrs_return) {
		try {
			cv::viz::WCircle* ret = new cv::viz::WCircle(radius, *center, *normal, thickness, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCircle::WCircle(Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:325
	// ("cv::viz::WCircle::WCircle", vec![(pred!(mut, ["radius", "center", "normal"], ["double", "const cv::Point3d*", "const cv::Vec3d*"]), _)]),
	void cv_viz_WCircle_WCircle_double_const_Point3dR_const_Vec3dR(double radius, const cv::Point3d* center, const cv::Vec3d* normal, Result<cv::viz::WCircle*>* ocvrs_return) {
		try {
			cv::viz::WCircle* ret = new cv::viz::WCircle(radius, *center, *normal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCircle::to_Widget() generated
	// ("cv::viz::WCircle::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WCircle_to_Widget(cv::viz::WCircle* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WCircle::to_Widget3D() generated
	// ("cv::viz::WCircle::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WCircle_to_Widget3D(cv::viz::WCircle* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WCircle::delete() generated
	// ("cv::viz::WCircle::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCircle_delete(cv::viz::WCircle* instance) {
			delete instance;
	}

	// WCloud(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:690
	// ("cv::viz::WCloud::WCloud", vec![(pred!(mut, ["cloud", "colors"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR(const cv::_InputArray* cloud, const cv::_InputArray* colors, Result<cv::viz::WCloud*>* ocvrs_return) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *colors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WCloud(InputArray, const Color &)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:698
	// ("cv::viz::WCloud::WCloud", vec![(pred!(mut, ["cloud", "color"], ["const cv::_InputArray*", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR(const cv::_InputArray* cloud, const cv::viz::Color* color, Result<cv::viz::WCloud*>* ocvrs_return) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCloud::WCloud(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:698
	// ("cv::viz::WCloud::WCloud", vec![(pred!(mut, ["cloud"], ["const cv::_InputArray*"]), _)]),
	void cv_viz_WCloud_WCloud_const__InputArrayR(const cv::_InputArray* cloud, Result<cv::viz::WCloud*>* ocvrs_return) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WCloud(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:707
	// ("cv::viz::WCloud::WCloud", vec![(pred!(mut, ["cloud", "colors", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* cloud, const cv::_InputArray* colors, const cv::_InputArray* normals, Result<cv::viz::WCloud*>* ocvrs_return) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *colors, *normals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WCloud(InputArray, const Color &, InputArray)(InputArray, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:717
	// ("cv::viz::WCloud::WCloud", vec![(pred!(mut, ["cloud", "color", "normals"], ["const cv::_InputArray*", "const cv::viz::Color*", "const cv::_InputArray*"]), _)]),
	void cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR_const__InputArrayR(const cv::_InputArray* cloud, const cv::viz::Color* color, const cv::_InputArray* normals, Result<cv::viz::WCloud*>* ocvrs_return) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *color, *normals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCloud::to_Widget() generated
	// ("cv::viz::WCloud::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WCloud_to_Widget(cv::viz::WCloud* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WCloud::to_Widget3D() generated
	// ("cv::viz::WCloud::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WCloud_to_Widget3D(cv::viz::WCloud* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WCloud::delete() generated
	// ("cv::viz::WCloud::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCloud_delete(cv::viz::WCloud* instance) {
			delete instance;
	}

	// WCloudCollection()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:739
	// ("cv::viz::WCloudCollection::WCloudCollection", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCloudCollection_WCloudCollection(Result<cv::viz::WCloudCollection*>* ocvrs_return) {
		try {
			cv::viz::WCloudCollection* ret = new cv::viz::WCloudCollection();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addCloud(InputArray, InputArray, const Affine3d &)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:747
	// ("cv::viz::WCloudCollection::addCloud", vec![(pred!(mut, ["cloud", "colors", "pose"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Affine3d*"]), _)]),
	void cv_viz_WCloudCollection_addCloud_const__InputArrayR_const__InputArrayR_const_Affine3dR(cv::viz::WCloudCollection* instance, const cv::_InputArray* cloud, const cv::_InputArray* colors, const cv::Affine3d* pose, ResultVoid* ocvrs_return) {
		try {
			instance->addCloud(*cloud, *colors, *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCloudCollection::addCloud(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:747
	// ("cv::viz::WCloudCollection::addCloud", vec![(pred!(mut, ["cloud", "colors"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_viz_WCloudCollection_addCloud_const__InputArrayR_const__InputArrayR(cv::viz::WCloudCollection* instance, const cv::_InputArray* cloud, const cv::_InputArray* colors, ResultVoid* ocvrs_return) {
		try {
			instance->addCloud(*cloud, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addCloud(InputArray, const Color &, const Affine3d &)(InputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:754
	// ("cv::viz::WCloudCollection::addCloud", vec![(pred!(mut, ["cloud", "color", "pose"], ["const cv::_InputArray*", "const cv::viz::Color*", "const cv::Affine3d*"]), _)]),
	void cv_viz_WCloudCollection_addCloud_const__InputArrayR_const_ColorR_const_Affine3dR(cv::viz::WCloudCollection* instance, const cv::_InputArray* cloud, const cv::viz::Color* color, const cv::Affine3d* pose, ResultVoid* ocvrs_return) {
		try {
			instance->addCloud(*cloud, *color, *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCloudCollection::addCloud(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:754
	// ("cv::viz::WCloudCollection::addCloud", vec![(pred!(mut, ["cloud"], ["const cv::_InputArray*"]), _)]),
	void cv_viz_WCloudCollection_addCloud_const__InputArrayR(cv::viz::WCloudCollection* instance, const cv::_InputArray* cloud, ResultVoid* ocvrs_return) {
		try {
			instance->addCloud(*cloud);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// finalize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:759
	// ("cv::viz::WCloudCollection::finalize", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCloudCollection_finalize(cv::viz::WCloudCollection* instance, ResultVoid* ocvrs_return) {
		try {
			instance->finalize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCloudCollection::to_Widget() generated
	// ("cv::viz::WCloudCollection::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WCloudCollection_to_Widget(cv::viz::WCloudCollection* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WCloudCollection::to_Widget3D() generated
	// ("cv::viz::WCloudCollection::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WCloudCollection_to_Widget3D(cv::viz::WCloudCollection* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WCloudCollection::delete() generated
	// ("cv::viz::WCloudCollection::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCloudCollection_delete(cv::viz::WCloudCollection* instance) {
			delete instance;
	}

	// WCloudNormals(InputArray, InputArray, int, double, const Color &)(InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:777
	// ("cv::viz::WCloudNormals::WCloudNormals", vec![(pred!(mut, ["cloud", "normals", "level", "scale", "color"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCloudNormals_WCloudNormals_const__InputArrayR_const__InputArrayR_int_double_const_ColorR(const cv::_InputArray* cloud, const cv::_InputArray* normals, int level, double scale, const cv::viz::Color* color, Result<cv::viz::WCloudNormals*>* ocvrs_return) {
		try {
			cv::viz::WCloudNormals* ret = new cv::viz::WCloudNormals(*cloud, *normals, level, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCloudNormals::WCloudNormals(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:777
	// ("cv::viz::WCloudNormals::WCloudNormals", vec![(pred!(mut, ["cloud", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_viz_WCloudNormals_WCloudNormals_const__InputArrayR_const__InputArrayR(const cv::_InputArray* cloud, const cv::_InputArray* normals, Result<cv::viz::WCloudNormals*>* ocvrs_return) {
		try {
			cv::viz::WCloudNormals* ret = new cv::viz::WCloudNormals(*cloud, *normals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCloudNormals::to_Widget() generated
	// ("cv::viz::WCloudNormals::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WCloudNormals_to_Widget(cv::viz::WCloudNormals* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WCloudNormals::to_Widget3D() generated
	// ("cv::viz::WCloudNormals::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WCloudNormals_to_Widget3D(cv::viz::WCloudNormals* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WCloudNormals::delete() generated
	// ("cv::viz::WCloudNormals::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCloudNormals_delete(cv::viz::WCloudNormals* instance) {
			delete instance;
	}

	// WCone(double, double, int, const Color &)(Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:340
	// ("cv::viz::WCone::WCone", vec![(pred!(mut, ["length", "radius", "resolution", "color"], ["double", "double", "int", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCone_WCone_double_double_int_const_ColorR(double length, double radius, int resolution, const cv::viz::Color* color, Result<cv::viz::WCone*>* ocvrs_return) {
		try {
			cv::viz::WCone* ret = new cv::viz::WCone(length, radius, resolution, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCone::WCone(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:340
	// ("cv::viz::WCone::WCone", vec![(pred!(mut, ["length", "radius"], ["double", "double"]), _)]),
	void cv_viz_WCone_WCone_double_double(double length, double radius, Result<cv::viz::WCone*>* ocvrs_return) {
		try {
			cv::viz::WCone* ret = new cv::viz::WCone(length, radius);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WCone(double, const Point3d &, const Point3d &, int, const Color &)(Primitive, SimpleClass, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:351
	// ("cv::viz::WCone::WCone", vec![(pred!(mut, ["radius", "center", "tip", "resolution", "color"], ["double", "const cv::Point3d*", "const cv::Point3d*", "int", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCone_WCone_double_const_Point3dR_const_Point3dR_int_const_ColorR(double radius, const cv::Point3d* center, const cv::Point3d* tip, int resolution, const cv::viz::Color* color, Result<cv::viz::WCone*>* ocvrs_return) {
		try {
			cv::viz::WCone* ret = new cv::viz::WCone(radius, *center, *tip, resolution, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCone::WCone(Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:351
	// ("cv::viz::WCone::WCone", vec![(pred!(mut, ["radius", "center", "tip"], ["double", "const cv::Point3d*", "const cv::Point3d*"]), _)]),
	void cv_viz_WCone_WCone_double_const_Point3dR_const_Point3dR(double radius, const cv::Point3d* center, const cv::Point3d* tip, Result<cv::viz::WCone*>* ocvrs_return) {
		try {
			cv::viz::WCone* ret = new cv::viz::WCone(radius, *center, *tip);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCone::to_Widget() generated
	// ("cv::viz::WCone::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WCone_to_Widget(cv::viz::WCone* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WCone::to_Widget3D() generated
	// ("cv::viz::WCone::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WCone_to_Widget3D(cv::viz::WCone* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WCone::delete() generated
	// ("cv::viz::WCone::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCone_delete(cv::viz::WCone* instance) {
			delete instance;
	}

	// WCoordinateSystem(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:520
	// ("cv::viz::WCoordinateSystem::WCoordinateSystem", vec![(pred!(mut, ["scale"], ["double"]), _)]),
	void cv_viz_WCoordinateSystem_WCoordinateSystem_double(double scale, Result<cv::viz::WCoordinateSystem*>* ocvrs_return) {
		try {
			cv::viz::WCoordinateSystem* ret = new cv::viz::WCoordinateSystem(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCoordinateSystem::WCoordinateSystem() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:520
	// ("cv::viz::WCoordinateSystem::WCoordinateSystem", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCoordinateSystem_WCoordinateSystem(Result<cv::viz::WCoordinateSystem*>* ocvrs_return) {
		try {
			cv::viz::WCoordinateSystem* ret = new cv::viz::WCoordinateSystem();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCoordinateSystem::to_Widget() generated
	// ("cv::viz::WCoordinateSystem::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WCoordinateSystem_to_Widget(cv::viz::WCoordinateSystem* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WCoordinateSystem::to_Widget3D() generated
	// ("cv::viz::WCoordinateSystem::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WCoordinateSystem_to_Widget3D(cv::viz::WCoordinateSystem* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WCoordinateSystem::delete() generated
	// ("cv::viz::WCoordinateSystem::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCoordinateSystem_delete(cv::viz::WCoordinateSystem* instance) {
			delete instance;
	}

	// WCube(const Point3d &, const Point3d &, bool, const Color &)(SimpleClass, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:384
	// ("cv::viz::WCube::WCube", vec![(pred!(mut, ["min_point", "max_point", "wire_frame", "color"], ["const cv::Point3d*", "const cv::Point3d*", "bool", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCube_WCube_const_Point3dR_const_Point3dR_bool_const_ColorR(const cv::Point3d* min_point, const cv::Point3d* max_point, bool wire_frame, const cv::viz::Color* color, Result<cv::viz::WCube*>* ocvrs_return) {
		try {
			cv::viz::WCube* ret = new cv::viz::WCube(*min_point, *max_point, wire_frame, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCube::WCube() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:384
	// ("cv::viz::WCube::WCube", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCube_WCube(Result<cv::viz::WCube*>* ocvrs_return) {
		try {
			cv::viz::WCube* ret = new cv::viz::WCube();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCube::to_Widget() generated
	// ("cv::viz::WCube::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WCube_to_Widget(cv::viz::WCube* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WCube::to_Widget3D() generated
	// ("cv::viz::WCube::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WCube_to_Widget3D(cv::viz::WCube* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WCube::delete() generated
	// ("cv::viz::WCube::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCube_delete(cv::viz::WCube* instance) {
			delete instance;
	}

	// WCylinder(const Point3d &, const Point3d &, double, int, const Color &)(SimpleClass, SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:367
	// ("cv::viz::WCylinder::WCylinder", vec![(pred!(mut, ["axis_point1", "axis_point2", "radius", "numsides", "color"], ["const cv::Point3d*", "const cv::Point3d*", "double", "int", "const cv::viz::Color*"]), _)]),
	void cv_viz_WCylinder_WCylinder_const_Point3dR_const_Point3dR_double_int_const_ColorR(const cv::Point3d* axis_point1, const cv::Point3d* axis_point2, double radius, int numsides, const cv::viz::Color* color, Result<cv::viz::WCylinder*>* ocvrs_return) {
		try {
			cv::viz::WCylinder* ret = new cv::viz::WCylinder(*axis_point1, *axis_point2, radius, numsides, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCylinder::WCylinder(SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:367
	// ("cv::viz::WCylinder::WCylinder", vec![(pred!(mut, ["axis_point1", "axis_point2", "radius"], ["const cv::Point3d*", "const cv::Point3d*", "double"]), _)]),
	void cv_viz_WCylinder_WCylinder_const_Point3dR_const_Point3dR_double(const cv::Point3d* axis_point1, const cv::Point3d* axis_point2, double radius, Result<cv::viz::WCylinder*>* ocvrs_return) {
		try {
			cv::viz::WCylinder* ret = new cv::viz::WCylinder(*axis_point1, *axis_point2, radius);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WCylinder::to_Widget() generated
	// ("cv::viz::WCylinder::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WCylinder_to_Widget(cv::viz::WCylinder* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WCylinder::to_Widget3D() generated
	// ("cv::viz::WCylinder::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WCylinder_to_Widget3D(cv::viz::WCylinder* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WCylinder::delete() generated
	// ("cv::viz::WCylinder::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WCylinder_delete(cv::viz::WCylinder* instance) {
			delete instance;
	}

	// WGrid(const Vec2i &, const Vec2d &, const Color &)(SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:534
	// ("cv::viz::WGrid::WGrid", vec![(pred!(mut, ["cells", "cells_spacing", "color"], ["const cv::Vec2i*", "const cv::Vec2d*", "const cv::viz::Color*"]), _)]),
	void cv_viz_WGrid_WGrid_const_Vec2iR_const_Vec2dR_const_ColorR(const cv::Vec2i* cells, const cv::Vec2d* cells_spacing, const cv::viz::Color* color, Result<cv::viz::WGrid*>* ocvrs_return) {
		try {
			cv::viz::WGrid* ret = new cv::viz::WGrid(*cells, *cells_spacing, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WGrid::WGrid() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:534
	// ("cv::viz::WGrid::WGrid", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WGrid_WGrid(Result<cv::viz::WGrid*>* ocvrs_return) {
		try {
			cv::viz::WGrid* ret = new cv::viz::WGrid();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WGrid(const Point3d &, const Vec3d &, const Vec3d &, const Vec2i &, const Vec2d &, const Color &)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:537
	// ("cv::viz::WGrid::WGrid", vec![(pred!(mut, ["center", "normal", "new_yaxis", "cells", "cells_spacing", "color"], ["const cv::Point3d*", "const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec2i*", "const cv::Vec2d*", "const cv::viz::Color*"]), _)]),
	void cv_viz_WGrid_WGrid_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Vec2iR_const_Vec2dR_const_ColorR(const cv::Point3d* center, const cv::Vec3d* normal, const cv::Vec3d* new_yaxis, const cv::Vec2i* cells, const cv::Vec2d* cells_spacing, const cv::viz::Color* color, Result<cv::viz::WGrid*>* ocvrs_return) {
		try {
			cv::viz::WGrid* ret = new cv::viz::WGrid(*center, *normal, *new_yaxis, *cells, *cells_spacing, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WGrid::WGrid(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:537
	// ("cv::viz::WGrid::WGrid", vec![(pred!(mut, ["center", "normal", "new_yaxis"], ["const cv::Point3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
	void cv_viz_WGrid_WGrid_const_Point3dR_const_Vec3dR_const_Vec3dR(const cv::Point3d* center, const cv::Vec3d* normal, const cv::Vec3d* new_yaxis, Result<cv::viz::WGrid*>* ocvrs_return) {
		try {
			cv::viz::WGrid* ret = new cv::viz::WGrid(*center, *normal, *new_yaxis);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WGrid::to_Widget() generated
	// ("cv::viz::WGrid::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WGrid_to_Widget(cv::viz::WGrid* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WGrid::to_Widget3D() generated
	// ("cv::viz::WGrid::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WGrid_to_Widget3D(cv::viz::WGrid* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WGrid::delete() generated
	// ("cv::viz::WGrid::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WGrid_delete(cv::viz::WGrid* instance) {
			delete instance;
	}

	// WImage3D(InputArray, const Size2d &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:483
	// ("cv::viz::WImage3D::WImage3D", vec![(pred!(mut, ["image", "size"], ["const cv::_InputArray*", "const cv::Size2d*"]), _)]),
	void cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR(const cv::_InputArray* image, const cv::Size2d* size, Result<cv::viz::WImage3D*>* ocvrs_return) {
		try {
			cv::viz::WImage3D* ret = new cv::viz::WImage3D(*image, *size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WImage3D(InputArray, const Size2d &, const Vec3d &, const Vec3d &, const Vec3d &)(InputArray, SimpleClass, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:493
	// ("cv::viz::WImage3D::WImage3D", vec![(pred!(mut, ["image", "size", "center", "normal", "up_vector"], ["const cv::_InputArray*", "const cv::Size2d*", "const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
	void cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(const cv::_InputArray* image, const cv::Size2d* size, const cv::Vec3d* center, const cv::Vec3d* normal, const cv::Vec3d* up_vector, Result<cv::viz::WImage3D*>* ocvrs_return) {
		try {
			cv::viz::WImage3D* ret = new cv::viz::WImage3D(*image, *size, *center, *normal, *up_vector);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setImage(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:499
	// ("cv::viz::WImage3D::setImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_viz_WImage3D_setImage_const__InputArrayR(cv::viz::WImage3D* instance, const cv::_InputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->setImage(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSize(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:505
	// ("cv::viz::WImage3D::setSize", vec![(pred!(mut, ["size"], ["const cv::Size*"]), _)]),
	void cv_viz_WImage3D_setSize_const_SizeR(cv::viz::WImage3D* instance, const cv::Size* size, ResultVoid* ocvrs_return) {
		try {
			instance->setSize(*size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WImage3D::to_Widget() generated
	// ("cv::viz::WImage3D::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WImage3D_to_Widget(cv::viz::WImage3D* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WImage3D::to_Widget3D() generated
	// ("cv::viz::WImage3D::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WImage3D_to_Widget3D(cv::viz::WImage3D* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WImage3D::delete() generated
	// ("cv::viz::WImage3D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WImage3D_delete(cv::viz::WImage3D* instance) {
			delete instance;
	}

	// WImageOverlay(InputArray, const Rect &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:465
	// ("cv::viz::WImageOverlay::WImageOverlay", vec![(pred!(mut, ["image", "rect"], ["const cv::_InputArray*", "const cv::Rect*"]), _)]),
	void cv_viz_WImageOverlay_WImageOverlay_const__InputArrayR_const_RectR(const cv::_InputArray* image, const cv::Rect* rect, Result<cv::viz::WImageOverlay*>* ocvrs_return) {
		try {
			cv::viz::WImageOverlay* ret = new cv::viz::WImageOverlay(*image, *rect);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setImage(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:470
	// ("cv::viz::WImageOverlay::setImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_viz_WImageOverlay_setImage_const__InputArrayR(cv::viz::WImageOverlay* instance, const cv::_InputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->setImage(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WImageOverlay::to_Widget() generated
	// ("cv::viz::WImageOverlay::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WImageOverlay_to_Widget(cv::viz::WImageOverlay* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WImageOverlay::to_Widget2D() generated
	// ("cv::viz::WImageOverlay::to_Widget2D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget2D* cv_viz_WImageOverlay_to_Widget2D(cv::viz::WImageOverlay* instance) {
			return dynamic_cast<cv::viz::Widget2D*>(instance);
	}

	// cv::viz::WImageOverlay::delete() generated
	// ("cv::viz::WImageOverlay::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WImageOverlay_delete(cv::viz::WImageOverlay* instance) {
			delete instance;
	}

	// WLine(const Point3d &, const Point3d &, const Color &)(SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:244
	// ("cv::viz::WLine::WLine", vec![(pred!(mut, ["pt1", "pt2", "color"], ["const cv::Point3d*", "const cv::Point3d*", "const cv::viz::Color*"]), _)]),
	void cv_viz_WLine_WLine_const_Point3dR_const_Point3dR_const_ColorR(const cv::Point3d* pt1, const cv::Point3d* pt2, const cv::viz::Color* color, Result<cv::viz::WLine*>* ocvrs_return) {
		try {
			cv::viz::WLine* ret = new cv::viz::WLine(*pt1, *pt2, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WLine::WLine(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:244
	// ("cv::viz::WLine::WLine", vec![(pred!(mut, ["pt1", "pt2"], ["const cv::Point3d*", "const cv::Point3d*"]), _)]),
	void cv_viz_WLine_WLine_const_Point3dR_const_Point3dR(const cv::Point3d* pt1, const cv::Point3d* pt2, Result<cv::viz::WLine*>* ocvrs_return) {
		try {
			cv::viz::WLine* ret = new cv::viz::WLine(*pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WLine::to_Widget() generated
	// ("cv::viz::WLine::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WLine_to_Widget(cv::viz::WLine* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WLine::to_Widget3D() generated
	// ("cv::viz::WLine::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WLine_to_Widget3D(cv::viz::WLine* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WLine::delete() generated
	// ("cv::viz::WLine::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WLine_delete(cv::viz::WLine* instance) {
			delete instance;
	}

	// WMesh(const Mesh &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:794
	// ("cv::viz::WMesh::WMesh", vec![(pred!(mut, ["mesh"], ["const cv::viz::Mesh*"]), _)]),
	void cv_viz_WMesh_WMesh_const_MeshR(const cv::viz::Mesh* mesh, Result<cv::viz::WMesh*>* ocvrs_return) {
		try {
			cv::viz::WMesh* ret = new cv::viz::WMesh(*mesh);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WMesh(InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:795
	// ("cv::viz::WMesh::WMesh", vec![(pred!(mut, ["cloud", "polygons", "colors", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_viz_WMesh_WMesh_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* cloud, const cv::_InputArray* polygons, const cv::_InputArray* colors, const cv::_InputArray* normals, Result<cv::viz::WMesh*>* ocvrs_return) {
		try {
			cv::viz::WMesh* ret = new cv::viz::WMesh(*cloud, *polygons, *colors, *normals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WMesh::WMesh(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:795
	// ("cv::viz::WMesh::WMesh", vec![(pred!(mut, ["cloud", "polygons"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_viz_WMesh_WMesh_const__InputArrayR_const__InputArrayR(const cv::_InputArray* cloud, const cv::_InputArray* polygons, Result<cv::viz::WMesh*>* ocvrs_return) {
		try {
			cv::viz::WMesh* ret = new cv::viz::WMesh(*cloud, *polygons);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WMesh::to_Widget() generated
	// ("cv::viz::WMesh::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WMesh_to_Widget(cv::viz::WMesh* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WMesh::to_Widget3D() generated
	// ("cv::viz::WMesh::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WMesh_to_Widget3D(cv::viz::WMesh* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WMesh::delete() generated
	// ("cv::viz::WMesh::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WMesh_delete(cv::viz::WMesh* instance) {
			delete instance;
	}

	// WPaintedCloud(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:724
	// ("cv::viz::WPaintedCloud::WPaintedCloud", vec![(pred!(mut, ["cloud"], ["const cv::_InputArray*"]), _)]),
	void cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR(const cv::_InputArray* cloud, Result<cv::viz::WPaintedCloud*>* ocvrs_return) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*cloud);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WPaintedCloud(InputArray, const Point3d &, const Point3d &)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:727
	// ("cv::viz::WPaintedCloud::WPaintedCloud", vec![(pred!(mut, ["cloud", "p1", "p2"], ["const cv::_InputArray*", "const cv::Point3d*", "const cv::Point3d*"]), _)]),
	void cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR(const cv::_InputArray* cloud, const cv::Point3d* p1, const cv::Point3d* p2, Result<cv::viz::WPaintedCloud*>* ocvrs_return) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*cloud, *p1, *p2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WPaintedCloud(InputArray, const Point3d &, const Point3d &, const Color &, const Color)(InputArray, SimpleClass, SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:730
	// ("cv::viz::WPaintedCloud::WPaintedCloud", vec![(pred!(mut, ["cloud", "p1", "p2", "c1", "c2"], ["const cv::_InputArray*", "const cv::Point3d*", "const cv::Point3d*", "const cv::viz::Color*", "const cv::viz::Color"]), _)]),
	void cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR_const_ColorR_const_Color(const cv::_InputArray* cloud, const cv::Point3d* p1, const cv::Point3d* p2, const cv::viz::Color* c1, const cv::viz::Color* c2, Result<cv::viz::WPaintedCloud*>* ocvrs_return) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*cloud, *p1, *p2, *c1, *c2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WPaintedCloud::to_Widget() generated
	// ("cv::viz::WPaintedCloud::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WPaintedCloud_to_Widget(cv::viz::WPaintedCloud* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WPaintedCloud::to_Widget3D() generated
	// ("cv::viz::WPaintedCloud::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WPaintedCloud_to_Widget3D(cv::viz::WPaintedCloud* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WPaintedCloud::delete() generated
	// ("cv::viz::WPaintedCloud::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WPaintedCloud_delete(cv::viz::WPaintedCloud* instance) {
			delete instance;
	}

	// WPlane(const Size2d &, const Color &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:257
	// ("cv::viz::WPlane::WPlane", vec![(pred!(mut, ["size", "color"], ["const cv::Size2d*", "const cv::viz::Color*"]), _)]),
	void cv_viz_WPlane_WPlane_const_Size2dR_const_ColorR(const cv::Size2d* size, const cv::viz::Color* color, Result<cv::viz::WPlane*>* ocvrs_return) {
		try {
			cv::viz::WPlane* ret = new cv::viz::WPlane(*size, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WPlane::WPlane() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:257
	// ("cv::viz::WPlane::WPlane", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WPlane_WPlane(Result<cv::viz::WPlane*>* ocvrs_return) {
		try {
			cv::viz::WPlane* ret = new cv::viz::WPlane();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WPlane(const Point3d &, const Vec3d &, const Vec3d &, const Size2d &, const Color &)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:267
	// ("cv::viz::WPlane::WPlane", vec![(pred!(mut, ["center", "normal", "new_yaxis", "size", "color"], ["const cv::Point3d*", "const cv::Vec3d*", "const cv::Vec3d*", "const cv::Size2d*", "const cv::viz::Color*"]), _)]),
	void cv_viz_WPlane_WPlane_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Size2dR_const_ColorR(const cv::Point3d* center, const cv::Vec3d* normal, const cv::Vec3d* new_yaxis, const cv::Size2d* size, const cv::viz::Color* color, Result<cv::viz::WPlane*>* ocvrs_return) {
		try {
			cv::viz::WPlane* ret = new cv::viz::WPlane(*center, *normal, *new_yaxis, *size, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WPlane::WPlane(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:267
	// ("cv::viz::WPlane::WPlane", vec![(pred!(mut, ["center", "normal", "new_yaxis"], ["const cv::Point3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
	void cv_viz_WPlane_WPlane_const_Point3dR_const_Vec3dR_const_Vec3dR(const cv::Point3d* center, const cv::Vec3d* normal, const cv::Vec3d* new_yaxis, Result<cv::viz::WPlane*>* ocvrs_return) {
		try {
			cv::viz::WPlane* ret = new cv::viz::WPlane(*center, *normal, *new_yaxis);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WPlane::to_Widget() generated
	// ("cv::viz::WPlane::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WPlane_to_Widget(cv::viz::WPlane* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WPlane::to_Widget3D() generated
	// ("cv::viz::WPlane::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WPlane_to_Widget3D(cv::viz::WPlane* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WPlane::delete() generated
	// ("cv::viz::WPlane::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WPlane_delete(cv::viz::WPlane* instance) {
			delete instance;
	}

	// WPolyLine(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:393
	// ("cv::viz::WPolyLine::WPolyLine", vec![(pred!(mut, ["points", "colors"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const__InputArrayR(const cv::_InputArray* points, const cv::_InputArray* colors, Result<cv::viz::WPolyLine*>* ocvrs_return) {
		try {
			cv::viz::WPolyLine* ret = new cv::viz::WPolyLine(*points, *colors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WPolyLine(InputArray, const Color &)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:399
	// ("cv::viz::WPolyLine::WPolyLine", vec![(pred!(mut, ["points", "color"], ["const cv::_InputArray*", "const cv::viz::Color*"]), _)]),
	void cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const_ColorR(const cv::_InputArray* points, const cv::viz::Color* color, Result<cv::viz::WPolyLine*>* ocvrs_return) {
		try {
			cv::viz::WPolyLine* ret = new cv::viz::WPolyLine(*points, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WPolyLine::WPolyLine(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:399
	// ("cv::viz::WPolyLine::WPolyLine", vec![(pred!(mut, ["points"], ["const cv::_InputArray*"]), _)]),
	void cv_viz_WPolyLine_WPolyLine_const__InputArrayR(const cv::_InputArray* points, Result<cv::viz::WPolyLine*>* ocvrs_return) {
		try {
			cv::viz::WPolyLine* ret = new cv::viz::WPolyLine(*points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WPolyLine::to_Widget() generated
	// ("cv::viz::WPolyLine::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WPolyLine_to_Widget(cv::viz::WPolyLine* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WPolyLine::to_Widget3D() generated
	// ("cv::viz::WPolyLine::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WPolyLine_to_Widget3D(cv::viz::WPolyLine* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WPolyLine::delete() generated
	// ("cv::viz::WPolyLine::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WPolyLine_delete(cv::viz::WPolyLine* instance) {
			delete instance;
	}

	// WSphere(const cv::Point3d &, double, int, const Color &)(SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:283
	// ("cv::viz::WSphere::WSphere", vec![(pred!(mut, ["center", "radius", "sphere_resolution", "color"], ["const cv::Point3d*", "double", "int", "const cv::viz::Color*"]), _)]),
	void cv_viz_WSphere_WSphere_const_Point3dR_double_int_const_ColorR(const cv::Point3d* center, double radius, int sphere_resolution, const cv::viz::Color* color, Result<cv::viz::WSphere*>* ocvrs_return) {
		try {
			cv::viz::WSphere* ret = new cv::viz::WSphere(*center, radius, sphere_resolution, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WSphere::WSphere(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:283
	// ("cv::viz::WSphere::WSphere", vec![(pred!(mut, ["center", "radius"], ["const cv::Point3d*", "double"]), _)]),
	void cv_viz_WSphere_WSphere_const_Point3dR_double(const cv::Point3d* center, double radius, Result<cv::viz::WSphere*>* ocvrs_return) {
		try {
			cv::viz::WSphere* ret = new cv::viz::WSphere(*center, radius);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WSphere::to_Widget() generated
	// ("cv::viz::WSphere::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WSphere_to_Widget(cv::viz::WSphere* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WSphere::to_Widget3D() generated
	// ("cv::viz::WSphere::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WSphere_to_Widget3D(cv::viz::WSphere* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WSphere::delete() generated
	// ("cv::viz::WSphere::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WSphere_delete(cv::viz::WSphere* instance) {
			delete instance;
	}

	// WText(const String &, const Point &, int, const Color &)(InString, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:417
	// ("cv::viz::WText::WText", vec![(pred!(mut, ["text", "pos", "font_size", "color"], ["const cv::String*", "const cv::Point*", "int", "const cv::viz::Color*"]), _)]),
	void cv_viz_WText_WText_const_StringR_const_PointR_int_const_ColorR(const char* text, const cv::Point* pos, int font_size, const cv::viz::Color* color, Result<cv::viz::WText*>* ocvrs_return) {
		try {
			cv::viz::WText* ret = new cv::viz::WText(std::string(text), *pos, font_size, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WText::WText(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:417
	// ("cv::viz::WText::WText", vec![(pred!(mut, ["text", "pos"], ["const cv::String*", "const cv::Point*"]), _)]),
	void cv_viz_WText_WText_const_StringR_const_PointR(const char* text, const cv::Point* pos, Result<cv::viz::WText*>* ocvrs_return) {
		try {
			cv::viz::WText* ret = new cv::viz::WText(std::string(text), *pos);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setText(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:423
	// ("cv::viz::WText::setText", vec![(pred!(mut, ["text"], ["const cv::String*"]), _)]),
	void cv_viz_WText_setText_const_StringR(cv::viz::WText* instance, const char* text, ResultVoid* ocvrs_return) {
		try {
			instance->setText(std::string(text));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getText()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:426
	// ("cv::viz::WText::getText", vec![(pred!(const, [], []), _)]),
	void cv_viz_WText_getText_const(const cv::viz::WText* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getText();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WText::to_Widget() generated
	// ("cv::viz::WText::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WText_to_Widget(cv::viz::WText* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WText::to_Widget2D() generated
	// ("cv::viz::WText::to_Widget2D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget2D* cv_viz_WText_to_Widget2D(cv::viz::WText* instance) {
			return dynamic_cast<cv::viz::Widget2D*>(instance);
	}

	// cv::viz::WText::delete() generated
	// ("cv::viz::WText::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WText_delete(cv::viz::WText* instance) {
			delete instance;
	}

	// WText3D(const String &, const Point3d &, double, bool, const Color &)(InString, SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:442
	// ("cv::viz::WText3D::WText3D", vec![(pred!(mut, ["text", "position", "text_scale", "face_camera", "color"], ["const cv::String*", "const cv::Point3d*", "double", "bool", "const cv::viz::Color*"]), _)]),
	void cv_viz_WText3D_WText3D_const_StringR_const_Point3dR_double_bool_const_ColorR(const char* text, const cv::Point3d* position, double text_scale, bool face_camera, const cv::viz::Color* color, Result<cv::viz::WText3D*>* ocvrs_return) {
		try {
			cv::viz::WText3D* ret = new cv::viz::WText3D(std::string(text), *position, text_scale, face_camera, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WText3D::WText3D(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:442
	// ("cv::viz::WText3D::WText3D", vec![(pred!(mut, ["text", "position"], ["const cv::String*", "const cv::Point3d*"]), _)]),
	void cv_viz_WText3D_WText3D_const_StringR_const_Point3dR(const char* text, const cv::Point3d* position, Result<cv::viz::WText3D*>* ocvrs_return) {
		try {
			cv::viz::WText3D* ret = new cv::viz::WText3D(std::string(text), *position);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setText(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:449
	// ("cv::viz::WText3D::setText", vec![(pred!(mut, ["text"], ["const cv::String*"]), _)]),
	void cv_viz_WText3D_setText_const_StringR(cv::viz::WText3D* instance, const char* text, ResultVoid* ocvrs_return) {
		try {
			instance->setText(std::string(text));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getText()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:452
	// ("cv::viz::WText3D::getText", vec![(pred!(const, [], []), _)]),
	void cv_viz_WText3D_getText_const(const cv::viz::WText3D* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getText();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WText3D::to_Widget() generated
	// ("cv::viz::WText3D::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WText3D_to_Widget(cv::viz::WText3D* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WText3D::to_Widget3D() generated
	// ("cv::viz::WText3D::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WText3D_to_Widget3D(cv::viz::WText3D* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WText3D::delete() generated
	// ("cv::viz::WText3D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WText3D_delete(cv::viz::WText3D* instance) {
			delete instance;
	}

	// WTrajectory(InputArray, int, double, const Color &)(InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:622
	// ("cv::viz::WTrajectory::WTrajectory", vec![(pred!(mut, ["path", "display_mode", "scale", "color"], ["const cv::_InputArray*", "int", "double", "const cv::viz::Color*"]), _)]),
	void cv_viz_WTrajectory_WTrajectory_const__InputArrayR_int_double_const_ColorR(const cv::_InputArray* path, int display_mode, double scale, const cv::viz::Color* color, Result<cv::viz::WTrajectory*>* ocvrs_return) {
		try {
			cv::viz::WTrajectory* ret = new cv::viz::WTrajectory(*path, display_mode, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WTrajectory::WTrajectory(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:622
	// ("cv::viz::WTrajectory::WTrajectory", vec![(pred!(mut, ["path"], ["const cv::_InputArray*"]), _)]),
	void cv_viz_WTrajectory_WTrajectory_const__InputArrayR(const cv::_InputArray* path, Result<cv::viz::WTrajectory*>* ocvrs_return) {
		try {
			cv::viz::WTrajectory* ret = new cv::viz::WTrajectory(*path);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WTrajectory::to_Widget() generated
	// ("cv::viz::WTrajectory::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WTrajectory_to_Widget(cv::viz::WTrajectory* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WTrajectory::to_Widget3D() generated
	// ("cv::viz::WTrajectory::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WTrajectory_to_Widget3D(cv::viz::WTrajectory* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WTrajectory::delete() generated
	// ("cv::viz::WTrajectory::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WTrajectory_delete(cv::viz::WTrajectory* instance) {
			delete instance;
	}

	// WTrajectoryFrustums(InputArray, const Matx33d &, double, const Color &)(InputArray, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:639
	// ("cv::viz::WTrajectoryFrustums::WTrajectoryFrustums", vec![(pred!(mut, ["path", "K", "scale", "color"], ["const cv::_InputArray*", "const cv::Matx33d*", "double", "const cv::viz::Color*"]), _)]),
	void cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Matx33dR_double_const_ColorR(const cv::_InputArray* path, const cv::Matx33d* K, double scale, const cv::viz::Color* color, Result<cv::viz::WTrajectoryFrustums*>* ocvrs_return) {
		try {
			cv::viz::WTrajectoryFrustums* ret = new cv::viz::WTrajectoryFrustums(*path, *K, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WTrajectoryFrustums::WTrajectoryFrustums(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:639
	// ("cv::viz::WTrajectoryFrustums::WTrajectoryFrustums", vec![(pred!(mut, ["path", "K"], ["const cv::_InputArray*", "const cv::Matx33d*"]), _)]),
	void cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Matx33dR(const cv::_InputArray* path, const cv::Matx33d* K, Result<cv::viz::WTrajectoryFrustums*>* ocvrs_return) {
		try {
			cv::viz::WTrajectoryFrustums* ret = new cv::viz::WTrajectoryFrustums(*path, *K);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WTrajectoryFrustums(InputArray, const Vec2d &, double, const Color &)(InputArray, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:650
	// ("cv::viz::WTrajectoryFrustums::WTrajectoryFrustums", vec![(pred!(mut, ["path", "fov", "scale", "color"], ["const cv::_InputArray*", "const cv::Vec2d*", "double", "const cv::viz::Color*"]), _)]),
	void cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Vec2dR_double_const_ColorR(const cv::_InputArray* path, const cv::Vec2d* fov, double scale, const cv::viz::Color* color, Result<cv::viz::WTrajectoryFrustums*>* ocvrs_return) {
		try {
			cv::viz::WTrajectoryFrustums* ret = new cv::viz::WTrajectoryFrustums(*path, *fov, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WTrajectoryFrustums::WTrajectoryFrustums(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:650
	// ("cv::viz::WTrajectoryFrustums::WTrajectoryFrustums", vec![(pred!(mut, ["path", "fov"], ["const cv::_InputArray*", "const cv::Vec2d*"]), _)]),
	void cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Vec2dR(const cv::_InputArray* path, const cv::Vec2d* fov, Result<cv::viz::WTrajectoryFrustums*>* ocvrs_return) {
		try {
			cv::viz::WTrajectoryFrustums* ret = new cv::viz::WTrajectoryFrustums(*path, *fov);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WTrajectoryFrustums::to_Widget() generated
	// ("cv::viz::WTrajectoryFrustums::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WTrajectoryFrustums_to_Widget(cv::viz::WTrajectoryFrustums* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WTrajectoryFrustums::to_Widget3D() generated
	// ("cv::viz::WTrajectoryFrustums::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WTrajectoryFrustums_to_Widget3D(cv::viz::WTrajectoryFrustums* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WTrajectoryFrustums::delete() generated
	// ("cv::viz::WTrajectoryFrustums::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WTrajectoryFrustums_delete(cv::viz::WTrajectoryFrustums* instance) {
			delete instance;
	}

	// WTrajectorySpheres(InputArray, double, double, const Color &, const Color &)(InputArray, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:669
	// ("cv::viz::WTrajectorySpheres::WTrajectorySpheres", vec![(pred!(mut, ["path", "line_length", "radius", "from", "to"], ["const cv::_InputArray*", "double", "double", "const cv::viz::Color*", "const cv::viz::Color*"]), _)]),
	void cv_viz_WTrajectorySpheres_WTrajectorySpheres_const__InputArrayR_double_double_const_ColorR_const_ColorR(const cv::_InputArray* path, double line_length, double radius, const cv::viz::Color* from, const cv::viz::Color* to, Result<cv::viz::WTrajectorySpheres*>* ocvrs_return) {
		try {
			cv::viz::WTrajectorySpheres* ret = new cv::viz::WTrajectorySpheres(*path, line_length, radius, *from, *to);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WTrajectorySpheres::WTrajectorySpheres(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:669
	// ("cv::viz::WTrajectorySpheres::WTrajectorySpheres", vec![(pred!(mut, ["path"], ["const cv::_InputArray*"]), _)]),
	void cv_viz_WTrajectorySpheres_WTrajectorySpheres_const__InputArrayR(const cv::_InputArray* path, Result<cv::viz::WTrajectorySpheres*>* ocvrs_return) {
		try {
			cv::viz::WTrajectorySpheres* ret = new cv::viz::WTrajectorySpheres(*path);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WTrajectorySpheres::to_Widget() generated
	// ("cv::viz::WTrajectorySpheres::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WTrajectorySpheres_to_Widget(cv::viz::WTrajectorySpheres* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WTrajectorySpheres::to_Widget3D() generated
	// ("cv::viz::WTrajectorySpheres::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WTrajectorySpheres_to_Widget3D(cv::viz::WTrajectorySpheres* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WTrajectorySpheres::delete() generated
	// ("cv::viz::WTrajectorySpheres::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WTrajectorySpheres_delete(cv::viz::WTrajectorySpheres* instance) {
			delete instance;
	}

	// WWidgetMerger()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:808
	// ("cv::viz::WWidgetMerger::WWidgetMerger", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WWidgetMerger_WWidgetMerger(Result<cv::viz::WWidgetMerger*>* ocvrs_return) {
		try {
			cv::viz::WWidgetMerger* ret = new cv::viz::WWidgetMerger();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addWidget(const Widget3D &, const Affine3d &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:811
	// ("cv::viz::WWidgetMerger::addWidget", vec![(pred!(mut, ["widget", "pose"], ["const cv::viz::Widget3D*", "const cv::Affine3d*"]), _)]),
	void cv_viz_WWidgetMerger_addWidget_const_Widget3DR_const_Affine3dR(cv::viz::WWidgetMerger* instance, const cv::viz::Widget3D* widget, const cv::Affine3d* pose, ResultVoid* ocvrs_return) {
		try {
			instance->addWidget(*widget, *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WWidgetMerger::addWidget(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:811
	// ("cv::viz::WWidgetMerger::addWidget", vec![(pred!(mut, ["widget"], ["const cv::viz::Widget3D*"]), _)]),
	void cv_viz_WWidgetMerger_addWidget_const_Widget3DR(cv::viz::WWidgetMerger* instance, const cv::viz::Widget3D* widget, ResultVoid* ocvrs_return) {
		try {
			instance->addWidget(*widget);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// finalize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:814
	// ("cv::viz::WWidgetMerger::finalize", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WWidgetMerger_finalize(cv::viz::WWidgetMerger* instance, ResultVoid* ocvrs_return) {
		try {
			instance->finalize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::WWidgetMerger::to_Widget() generated
	// ("cv::viz::WWidgetMerger::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_WWidgetMerger_to_Widget(cv::viz::WWidgetMerger* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::WWidgetMerger::to_Widget3D() generated
	// ("cv::viz::WWidgetMerger::to_Widget3D", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget3D* cv_viz_WWidgetMerger_to_Widget3D(cv::viz::WWidgetMerger* instance) {
			return dynamic_cast<cv::viz::Widget3D*>(instance);
	}

	// cv::viz::WWidgetMerger::delete() generated
	// ("cv::viz::WWidgetMerger::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_WWidgetMerger_delete(cv::viz::WWidgetMerger* instance) {
			delete instance;
	}

	// Widget()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:95
	// ("cv::viz::Widget::Widget", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Widget_Widget(Result<cv::viz::Widget*>* ocvrs_return) {
		try {
			cv::viz::Widget* ret = new cv::viz::Widget();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Widget(const Widget &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:96
	// ("cv::viz::Widget::Widget", vec![(pred!(mut, ["other"], ["const cv::viz::Widget*"]), _)]),
	void cv_viz_Widget_Widget_const_WidgetR(const cv::viz::Widget* other, Result<cv::viz::Widget*>* ocvrs_return) {
		try {
			cv::viz::Widget* ret = new cv::viz::Widget(*other);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Widget &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:97
	// ("cv::viz::Widget::operator=", vec![(pred!(mut, ["other"], ["const cv::viz::Widget*"]), _)]),
	void cv_viz_Widget_operatorST_const_WidgetR(cv::viz::Widget* instance, const cv::viz::Widget* other, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*other);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fromPlyFile(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:104
	// ("cv::viz::Widget::fromPlyFile", vec![(pred!(mut, ["file_name"], ["const cv::String*"]), _)]),
	void cv_viz_Widget_fromPlyFile_const_StringR(const char* file_name, Result<cv::viz::Widget*>* ocvrs_return) {
		try {
			cv::viz::Widget ret = cv::viz::Widget::fromPlyFile(std::string(file_name));
			Ok(new cv::viz::Widget(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRenderingProperty(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:131
	// ("cv::viz::Widget::setRenderingProperty", vec![(pred!(mut, ["property", "value"], ["int", "double"]), _)]),
	void cv_viz_Widget_setRenderingProperty_int_double(cv::viz::Widget* instance, int property, double value, ResultVoid* ocvrs_return) {
		try {
			instance->setRenderingProperty(property, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRenderingProperty(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:157
	// ("cv::viz::Widget::getRenderingProperty", vec![(pred!(const, ["property"], ["int"]), _)]),
	void cv_viz_Widget_getRenderingProperty_const_int(const cv::viz::Widget* instance, int property, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRenderingProperty(property);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Widget::delete() generated
	// ("cv::viz::Widget::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Widget_delete(cv::viz::Widget* instance) {
			delete instance;
	}

	// Widget2D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:221
	// ("cv::viz::Widget2D::Widget2D", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Widget2D_Widget2D(Result<cv::viz::Widget2D*>* ocvrs_return) {
		try {
			cv::viz::Widget2D* ret = new cv::viz::Widget2D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setColor(const Color &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:227
	// ("cv::viz::Widget2D::setColor", vec![(pred!(mut, ["color"], ["const cv::viz::Color*"]), _)]),
	void cv_viz_Widget2D_setColor_const_ColorR(cv::viz::Widget2D* instance, const cv::viz::Color* color, ResultVoid* ocvrs_return) {
		try {
			instance->setColor(*color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Widget2D::to_Widget() generated
	// ("cv::viz::Widget2D::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_Widget2D_to_Widget(cv::viz::Widget2D* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::Widget2D::delete() generated
	// ("cv::viz::Widget2D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Widget2D_delete(cv::viz::Widget2D* instance) {
			delete instance;
	}

	// Widget3D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:184
	// ("cv::viz::Widget3D::Widget3D", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Widget3D_Widget3D(Result<cv::viz::Widget3D*>* ocvrs_return) {
		try {
			cv::viz::Widget3D* ret = new cv::viz::Widget3D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPose(const Affine3d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:190
	// ("cv::viz::Widget3D::setPose", vec![(pred!(mut, ["pose"], ["const cv::Affine3d*"]), _)]),
	void cv_viz_Widget3D_setPose_const_Affine3dR(cv::viz::Widget3D* instance, const cv::Affine3d* pose, ResultVoid* ocvrs_return) {
		try {
			instance->setPose(*pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updatePose(const Affine3d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:195
	// ("cv::viz::Widget3D::updatePose", vec![(pred!(mut, ["pose"], ["const cv::Affine3d*"]), _)]),
	void cv_viz_Widget3D_updatePose_const_Affine3dR(cv::viz::Widget3D* instance, const cv::Affine3d* pose, ResultVoid* ocvrs_return) {
		try {
			instance->updatePose(*pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:198
	// ("cv::viz::Widget3D::getPose", vec![(pred!(const, [], []), _)]),
	void cv_viz_Widget3D_getPose_const(const cv::viz::Widget3D* instance, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// applyTransform(const Affine3d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:204
	// ("cv::viz::Widget3D::applyTransform", vec![(pred!(mut, ["transform"], ["const cv::Affine3d*"]), _)]),
	void cv_viz_Widget3D_applyTransform_const_Affine3dR(cv::viz::Widget3D* instance, const cv::Affine3d* transform, ResultVoid* ocvrs_return) {
		try {
			instance->applyTransform(*transform);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setColor(const Color &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/viz/widgets.hpp:210
	// ("cv::viz::Widget3D::setColor", vec![(pred!(mut, ["color"], ["const cv::viz::Color*"]), _)]),
	void cv_viz_Widget3D_setColor_const_ColorR(cv::viz::Widget3D* instance, const cv::viz::Color* color, ResultVoid* ocvrs_return) {
		try {
			instance->setColor(*color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::viz::Widget3D::to_Widget() generated
	// ("cv::viz::Widget3D::to_Widget", vec![(pred!(mut, [], []), _)]),
	cv::viz::Widget* cv_viz_Widget3D_to_Widget(cv::viz::Widget3D* instance) {
			return dynamic_cast<cv::viz::Widget*>(instance);
	}

	// cv::viz::Widget3D::delete() generated
	// ("cv::viz::Widget3D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_viz_Widget3D_delete(cv::viz::Widget3D* instance) {
			delete instance;
	}

}

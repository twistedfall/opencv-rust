// addResourceLocation(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:324
// ("cv::ovis::addResourceLocation", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
pub fn cv_ovis_addResourceLocation_const_StringR(path: *const c_char, ocvrs_return: *mut Result<()>);
// cv::ovis::createGridMesh(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:404
// ("cv::ovis::createGridMesh", vec![(pred!(mut, ["name", "size"], ["const cv::String*", "const cv::Size2f*"]), _)]),
pub fn cv_ovis_createGridMesh_const_StringR_const_Size2fR(name: *const c_char, size: *const core::Size2f, ocvrs_return: *mut Result<()>);
// createGridMesh(const String &, const Size2f &, const Size &)(InString, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:404
// ("cv::ovis::createGridMesh", vec![(pred!(mut, ["name", "size", "segments"], ["const cv::String*", "const cv::Size2f*", "const cv::Size*"]), _)]),
pub fn cv_ovis_createGridMesh_const_StringR_const_Size2fR_const_SizeR(name: *const c_char, size: *const core::Size2f, segments: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::ovis::createPlaneMesh(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:384
// ("cv::ovis::createPlaneMesh", vec![(pred!(mut, ["name", "size"], ["const cv::String*", "const cv::Size2f*"]), _)]),
pub fn cv_ovis_createPlaneMesh_const_StringR_const_Size2fR(name: *const c_char, size: *const core::Size2f, ocvrs_return: *mut Result<()>);
// createPlaneMesh(const String &, const Size2f &, InputArray)(InString, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:384
// ("cv::ovis::createPlaneMesh", vec![(pred!(mut, ["name", "size", "image"], ["const cv::String*", "const cv::Size2f*", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_createPlaneMesh_const_StringR_const_Size2fR_const__InputArrayR(name: *const c_char, size: *const core::Size2f, image: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ovis::createPointCloudMesh(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:394
// ("cv::ovis::createPointCloudMesh", vec![(pred!(mut, ["name", "vertices"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_createPointCloudMesh_const_StringR_const__InputArrayR(name: *const c_char, vertices: *const c_void, ocvrs_return: *mut Result<()>);
// createPointCloudMesh(const String &, InputArray, InputArray)(InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:394
// ("cv::ovis::createPointCloudMesh", vec![(pred!(mut, ["name", "vertices", "colors"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_createPointCloudMesh_const_StringR_const__InputArrayR_const__InputArrayR(name: *const c_char, vertices: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ovis::createTriangleMesh(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:415
// ("cv::ovis::createTriangleMesh", vec![(pred!(mut, ["name", "vertices"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_createTriangleMesh_const_StringR_const__InputArrayR(name: *const c_char, vertices: *const c_void, ocvrs_return: *mut Result<()>);
// createTriangleMesh(const String &, InputArray, InputArray, InputArray)(InString, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:415
// ("cv::ovis::createTriangleMesh", vec![(pred!(mut, ["name", "vertices", "normals", "indices"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_createTriangleMesh_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR(name: *const c_char, vertices: *const c_void, normals: *const c_void, indices: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ovis::createWindow(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:337
// ("cv::ovis::createWindow", vec![(pred!(mut, ["title", "size"], ["const cv::String*", "const cv::Size*"]), _)]),
pub fn cv_ovis_createWindow_const_StringR_const_SizeR(title: *const c_char, size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// createWindow(const String &, const Size &, int)(InString, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:337
// ("cv::ovis::createWindow", vec![(pred!(mut, ["title", "size", "flags"], ["const cv::String*", "const cv::Size*", "int"]), _)]),
pub fn cv_ovis_createWindow_const_StringR_const_SizeR_int(title: *const c_char, size: *const core::Size, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// setMaterialProperty(const String &, const String &, const Scalar &)(InString, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:374
// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "const cv::String*", "const cv::Scalar*"]), _)]),
pub fn cv_ovis_setMaterialProperty_const_StringR_const_StringR_const_ScalarR(name: *const c_char, prop: *const c_char, value: *const core::Scalar, ocvrs_return: *mut Result<()>);
// setMaterialProperty(const String &, int, const Scalar &)(InString, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:355
// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::Scalar*"]), _)]),
pub fn cv_ovis_setMaterialProperty_const_StringR_int_const_ScalarR(name: *const c_char, prop: i32, value: *const core::Scalar, ocvrs_return: *mut Result<()>);
// setMaterialProperty(const String &, int, const String &)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:358
// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::String*"]), _)]),
pub fn cv_ovis_setMaterialProperty_const_StringR_int_const_StringR(name: *const c_char, prop: i32, value: *const c_char, ocvrs_return: *mut Result<()>);
// setMaterialProperty(const String &, int, InputArray)(InString, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:366
// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_setMaterialProperty_const_StringR_int_const__InputArrayR(name: *const c_char, prop: i32, value: *const c_void, ocvrs_return: *mut Result<()>);
// updateTexture(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:418
// ("cv::ovis::updateTexture", vec![(pred!(mut, ["name", "image"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_updateTexture_const_StringR_const__InputArrayR(name: *const c_char, image: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ovis::waitKey() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:347
// ("cv::ovis::waitKey", vec![(pred!(mut, [], []), _)]),
pub fn cv_ovis_waitKey(ocvrs_return: *mut Result<i32>);
// waitKey(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:347
// ("cv::ovis::waitKey", vec![(pred!(mut, ["delay"], ["int"]), _)]),
pub fn cv_ovis_waitKey_int(delay: i32, ocvrs_return: *mut Result<i32>);
// setBackground(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:94
// ("cv::ovis::WindowScene::setBackground", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ovis_WindowScene_setBackground_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// setBackground(const Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:97
// ("cv::ovis::WindowScene::setBackground", vec![(pred!(mut, ["color"], ["const cv::Scalar*"]), _)]),
pub fn cv_ovis_WindowScene_setBackground_const_ScalarR(instance: *mut c_void, color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// setCompositors(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:107
// ("cv::ovis::WindowScene::setCompositors", vec![(pred!(mut, ["names"], ["const std::vector<cv::String>*"]), _)]),
pub fn cv_ovis_WindowScene_setCompositors_const_vectorLStringGR(instance: *mut c_void, names: *const c_void, ocvrs_return: *mut Result<()>);
// createEntity(const String &, const String &, InputArray, InputArray)(InString, InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:120
// ("cv::ovis::WindowScene::createEntity", vec![(pred!(mut, ["name", "meshname", "tvec", "rot"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_WindowScene_createEntity_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, name: *const c_char, meshname: *const c_char, tvec: *const c_void, rot: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::createEntity(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:120
// ("cv::ovis::WindowScene::createEntity", vec![(pred!(mut, ["name", "meshname"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ovis_WindowScene_createEntity_const_StringR_const_StringR(instance: *mut c_void, name: *const c_char, meshname: *const c_char, ocvrs_return: *mut Result<()>);
// removeEntity(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:127
// ("cv::ovis::WindowScene::removeEntity", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
pub fn cv_ovis_WindowScene_removeEntity_const_StringR(instance: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// setEntityProperty(const String &, int, const String &, int)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:136
// ("cv::ovis::WindowScene::setEntityProperty", vec![(pred!(mut, ["name", "prop", "value", "subEntityIdx"], ["const cv::String*", "int", "const cv::String*", "int"]), _)]),
pub fn cv_ovis_WindowScene_setEntityProperty_const_StringR_int_const_StringR_int(instance: *mut c_void, name: *const c_char, prop: i32, value: *const c_char, sub_entity_idx: i32, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::setEntityProperty(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:136
// ("cv::ovis::WindowScene::setEntityProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::String*"]), _)]),
pub fn cv_ovis_WindowScene_setEntityProperty_const_StringR_int_const_StringR(instance: *mut c_void, name: *const c_char, prop: i32, value: *const c_char, ocvrs_return: *mut Result<()>);
// setEntityProperty(const String &, int, const Scalar &)(InString, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:140
// ("cv::ovis::WindowScene::setEntityProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::Scalar*"]), _)]),
pub fn cv_ovis_WindowScene_setEntityProperty_const_StringR_int_const_ScalarR(instance: *mut c_void, name: *const c_char, prop: i32, value: *const core::Scalar, ocvrs_return: *mut Result<()>);
// getEntityProperty(const String &, int, OutputArray)(InString, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:148
// ("cv::ovis::WindowScene::getEntityProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_ovis_WindowScene_getEntityProperty_const_StringR_int_const__OutputArrayR(instance: *mut c_void, name: *const c_char, prop: i32, value: *const c_void, ocvrs_return: *mut Result<()>);
// createCameraEntity(const String &, InputArray, const Size &, float, InputArray, InputArray, const Scalar &)(InString, InputArray, SimpleClass, Primitive, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:163
// ("cv::ovis::WindowScene::createCameraEntity", vec![(pred!(mut, ["name", "K", "imsize", "zFar", "tvec", "rot", "color"], ["const cv::String*", "const cv::_InputArray*", "const cv::Size*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
pub fn cv_ovis_WindowScene_createCameraEntity_const_StringR_const__InputArrayR_const_SizeR_float_const__InputArrayR_const__InputArrayR_const_ScalarR(instance: *mut c_void, name: *const c_char, k: *const c_void, imsize: *const core::Size, z_far: f32, tvec: *const c_void, rot: *const c_void, color: *const core::Scalar, ocvrs_return: *mut Result<core::Rect2d>);
// cv::ovis::WindowScene::createCameraEntity(InString, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:163
// ("cv::ovis::WindowScene::createCameraEntity", vec![(pred!(mut, ["name", "K", "imsize", "zFar"], ["const cv::String*", "const cv::_InputArray*", "const cv::Size*", "float"]), _)]),
pub fn cv_ovis_WindowScene_createCameraEntity_const_StringR_const__InputArrayR_const_SizeR_float(instance: *mut c_void, name: *const c_char, k: *const c_void, imsize: *const core::Size, z_far: f32, ocvrs_return: *mut Result<core::Rect2d>);
// createLightEntity(const String &, InputArray, InputArray, const Scalar &, const Scalar &)(InString, InputArray, InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:176
// ("cv::ovis::WindowScene::createLightEntity", vec![(pred!(mut, ["name", "tvec", "rot", "diffuseColor", "specularColor"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Scalar*", "const cv::Scalar*"]), _)]),
pub fn cv_ovis_WindowScene_createLightEntity_const_StringR_const__InputArrayR_const__InputArrayR_const_ScalarR_const_ScalarR(instance: *mut c_void, name: *const c_char, tvec: *const c_void, rot: *const c_void, diffuse_color: *const core::Scalar, specular_color: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::createLightEntity(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:176
// ("cv::ovis::WindowScene::createLightEntity", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
pub fn cv_ovis_WindowScene_createLightEntity_const_StringR(instance: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// updateEntityPose(const String &, InputArray, InputArray)(InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:187
// ("cv::ovis::WindowScene::updateEntityPose", vec![(pred!(mut, ["name", "tvec", "rot"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_WindowScene_updateEntityPose_const_StringR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, name: *const c_char, tvec: *const c_void, rot: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::updateEntityPose(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:187
// ("cv::ovis::WindowScene::updateEntityPose", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
pub fn cv_ovis_WindowScene_updateEntityPose_const_StringR(instance: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// setEntityPose(const String &, InputArray, InputArray, bool)(InString, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:197
// ("cv::ovis::WindowScene::setEntityPose", vec![(pred!(mut, ["name", "tvec", "rot", "invert"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_ovis_WindowScene_setEntityPose_const_StringR_const__InputArrayR_const__InputArrayR_bool(instance: *mut c_void, name: *const c_char, tvec: *const c_void, rot: *const c_void, invert: bool, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::setEntityPose(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:197
// ("cv::ovis::WindowScene::setEntityPose", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
pub fn cv_ovis_WindowScene_setEntityPose_const_StringR(instance: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// getEntityPose(const String &, OutputArray, OutputArray, bool)(InString, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:207
// ("cv::ovis::WindowScene::getEntityPose", vec![(pred!(mut, ["name", "R", "tvec", "invert"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_ovis_WindowScene_getEntityPose_const_StringR_const__OutputArrayR_const__OutputArrayR_bool(instance: *mut c_void, name: *const c_char, r: *const c_void, tvec: *const c_void, invert: bool, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::getEntityPose(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:207
// ("cv::ovis::WindowScene::getEntityPose", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
pub fn cv_ovis_WindowScene_getEntityPose_const_StringR(instance: *mut c_void, name: *const c_char, ocvrs_return: *mut Result<()>);
// getEntityAnimations(const String &, std::vector<String> &)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:215
// ("cv::ovis::WindowScene::getEntityAnimations", vec![(pred!(mut, ["name", "out"], ["const cv::String*", "std::vector<cv::String>*"]), _)]),
pub fn cv_ovis_WindowScene_getEntityAnimations_const_StringR_vectorLStringGR(instance: *mut c_void, name: *const c_char, out: *mut c_void, ocvrs_return: *mut Result<()>);
// playEntityAnimation(const String &, const String &, bool)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:224
// ("cv::ovis::WindowScene::playEntityAnimation", vec![(pred!(mut, ["name", "animname", "loop"], ["const cv::String*", "const cv::String*", "bool"]), _)]),
pub fn cv_ovis_WindowScene_playEntityAnimation_const_StringR_const_StringR_bool(instance: *mut c_void, name: *const c_char, animname: *const c_char, loop_: bool, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::playEntityAnimation(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:224
// ("cv::ovis::WindowScene::playEntityAnimation", vec![(pred!(mut, ["name", "animname"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ovis_WindowScene_playEntityAnimation_const_StringR_const_StringR(instance: *mut c_void, name: *const c_char, animname: *const c_char, ocvrs_return: *mut Result<()>);
// stopEntityAnimation(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:232
// ("cv::ovis::WindowScene::stopEntityAnimation", vec![(pred!(mut, ["name", "animname"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ovis_WindowScene_stopEntityAnimation_const_StringR_const_StringR(instance: *mut c_void, name: *const c_char, animname: *const c_char, ocvrs_return: *mut Result<()>);
// getScreenshot(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:237
// ("cv::ovis::WindowScene::getScreenshot", vec![(pred!(mut, ["frame"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ovis_WindowScene_getScreenshot_const__OutputArrayR(instance: *mut c_void, frame: *const c_void, ocvrs_return: *mut Result<()>);
// getCompositorTexture(const String &, const String &, OutputArray, int)(InString, InString, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:246
// ("cv::ovis::WindowScene::getCompositorTexture", vec![(pred!(mut, ["compname", "texname", "out", "mrtIndex"], ["const cv::String*", "const cv::String*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ovis_WindowScene_getCompositorTexture_const_StringR_const_StringR_const__OutputArrayR_int(instance: *mut c_void, compname: *const c_char, texname: *const c_char, out: *const c_void, mrt_index: i32, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::getCompositorTexture(InString, InString, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:246
// ("cv::ovis::WindowScene::getCompositorTexture", vec![(pred!(mut, ["compname", "texname", "out"], ["const cv::String*", "const cv::String*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ovis_WindowScene_getCompositorTexture_const_StringR_const_StringR_const__OutputArrayR(instance: *mut c_void, compname: *const c_char, texname: *const c_char, out: *const c_void, ocvrs_return: *mut Result<()>);
// getDepth(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:254
// ("cv::ovis::WindowScene::getDepth", vec![(pred!(mut, ["depth"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_ovis_WindowScene_getDepth_const__OutputArrayR(instance: *mut c_void, depth: *const c_void, ocvrs_return: *mut Result<()>);
// fixCameraYawAxis(bool, InputArray)(Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:263
// ("cv::ovis::WindowScene::fixCameraYawAxis", vec![(pred!(mut, ["useFixed", "up"], ["bool", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_WindowScene_fixCameraYawAxis_bool_const__InputArrayR(instance: *mut c_void, use_fixed: bool, up: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::fixCameraYawAxis(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:263
// ("cv::ovis::WindowScene::fixCameraYawAxis", vec![(pred!(mut, ["useFixed"], ["bool"]), _)]),
pub fn cv_ovis_WindowScene_fixCameraYawAxis_bool(instance: *mut c_void, use_fixed: bool, ocvrs_return: *mut Result<()>);
// setCameraPose(InputArray, InputArray, bool)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:271
// ("cv::ovis::WindowScene::setCameraPose", vec![(pred!(mut, ["tvec", "rot", "invert"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_ovis_WindowScene_setCameraPose_const__InputArrayR_const__InputArrayR_bool(instance: *mut c_void, tvec: *const c_void, rot: *const c_void, invert: bool, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::setCameraPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:271
// ("cv::ovis::WindowScene::setCameraPose", vec![(pred!(mut, [], []), _)]),
pub fn cv_ovis_WindowScene_setCameraPose(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setCameraLookAt(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:279
// ("cv::ovis::WindowScene::setCameraLookAt", vec![(pred!(mut, ["target", "offset"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_WindowScene_setCameraLookAt_const_StringR_const__InputArrayR(instance: *mut c_void, target: *const c_char, offset: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::setCameraLookAt(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:279
// ("cv::ovis::WindowScene::setCameraLookAt", vec![(pred!(mut, ["target"], ["const cv::String*"]), _)]),
pub fn cv_ovis_WindowScene_setCameraLookAt_const_StringR(instance: *mut c_void, target: *const c_char, ocvrs_return: *mut Result<()>);
// setEntityLookAt(const String &, const String &, InputArray)(InString, InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:288
// ("cv::ovis::WindowScene::setEntityLookAt", vec![(pred!(mut, ["origin", "target", "offset"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_ovis_WindowScene_setEntityLookAt_const_StringR_const_StringR_const__InputArrayR(instance: *mut c_void, origin: *const c_char, target: *const c_char, offset: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::setEntityLookAt(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:288
// ("cv::ovis::WindowScene::setEntityLookAt", vec![(pred!(mut, ["origin", "target"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ovis_WindowScene_setEntityLookAt_const_StringR_const_StringR(instance: *mut c_void, origin: *const c_char, target: *const c_char, ocvrs_return: *mut Result<()>);
// getCameraPose(OutputArray, OutputArray, bool)(OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:296
// ("cv::ovis::WindowScene::getCameraPose", vec![(pred!(mut, ["R", "tvec", "invert"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_ovis_WindowScene_getCameraPose_const__OutputArrayR_const__OutputArrayR_bool(instance: *mut c_void, r: *const c_void, tvec: *const c_void, invert: bool, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::getCameraPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:296
// ("cv::ovis::WindowScene::getCameraPose", vec![(pred!(mut, [], []), _)]),
pub fn cv_ovis_WindowScene_getCameraPose(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setCameraIntrinsics(InputArray, const Size &, float, float)(InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:308
// ("cv::ovis::WindowScene::setCameraIntrinsics", vec![(pred!(mut, ["K", "imsize", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::Size*", "float", "float"]), _)]),
pub fn cv_ovis_WindowScene_setCameraIntrinsics_const__InputArrayR_const_SizeR_float_float(instance: *mut c_void, k: *const c_void, imsize: *const core::Size, z_near: f32, z_far: f32, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::setCameraIntrinsics(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:308
// ("cv::ovis::WindowScene::setCameraIntrinsics", vec![(pred!(mut, ["K", "imsize"], ["const cv::_InputArray*", "const cv::Size*"]), _)]),
pub fn cv_ovis_WindowScene_setCameraIntrinsics_const__InputArrayR_const_SizeR(instance: *mut c_void, k: *const c_void, imsize: *const core::Size, ocvrs_return: *mut Result<()>);
// update()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ovis.hpp:314
// ("cv::ovis::WindowScene::update", vec![(pred!(mut, [], []), _)]),
pub fn cv_ovis_WindowScene_update(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::ovis::WindowScene::delete() generated
// ("cv::ovis::WindowScene::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ovis_WindowScene_delete(instance: *mut c_void);

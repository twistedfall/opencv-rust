// censusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:22
// ("cv::stereo::censusTransform", vec![(pred!(mut, ["image1", "image2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
pub fn cv_stereo_censusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(image1: *const c_void, image2: *const c_void, kernel_size: i32, dist1: *mut c_void, dist2: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// censusTransform(const Mat &, int, Mat &, const int)(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:24
// ("cv::stereo::censusTransform", vec![(pred!(mut, ["image1", "kernelSize", "dist1", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
pub fn cv_stereo_censusTransform_const_MatR_int_MatR_const_int(image1: *const c_void, kernel_size: i32, dist1: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// cv::stereo::modifiedCensusTransform(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:29
// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
pub fn cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(img1: *const c_void, img2: *const c_void, kernel_size: i32, dist1: *mut c_void, dist2: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// modifiedCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int, int, const Mat &, const Mat &)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:29
// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type", "t", "integralImage1", "integralImage2"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int", "int", "const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int_int_const_MatR_const_MatR(img1: *const c_void, img2: *const c_void, kernel_size: i32, dist1: *mut c_void, dist2: *mut c_void, typ: i32, t: i32, integral_image1: *const c_void, integral_image2: *const c_void, ocvrs_return: *mut Result<()>);
// cv::stereo::modifiedCensusTransform(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:31
// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
pub fn cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int(img1: *const c_void, kernel_size: i32, dist: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// modifiedCensusTransform(const Mat &, int, Mat &, const int, int, const Mat &)(TraitClass, Primitive, TraitClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:31
// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist", "type", "t", "integralImage"], ["const cv::Mat*", "int", "cv::Mat*", "const int", "int", "const cv::Mat*"]), _)]),
pub fn cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int_int_const_MatR(img1: *const c_void, kernel_size: i32, dist: *mut c_void, typ: i32, t: i32, integral_image: *const c_void, ocvrs_return: *mut Result<()>);
// starCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:39
// ("cv::stereo::starCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_stereo_starCensusTransform_const_MatR_const_MatR_int_MatR_MatR(img1: *const c_void, img2: *const c_void, kernel_size: i32, dist1: *mut c_void, dist2: *mut c_void, ocvrs_return: *mut Result<()>);
// starCensusTransform(const Mat &, int, Mat &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:41
// ("cv::stereo::starCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist"], ["const cv::Mat*", "int", "cv::Mat*"]), _)]),
pub fn cv_stereo_starCensusTransform_const_MatR_int_MatR(img1: *const c_void, kernel_size: i32, dist: *mut c_void, ocvrs_return: *mut Result<()>);
// symetricCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:35
// ("cv::stereo::symetricCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
pub fn cv_stereo_symetricCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(img1: *const c_void, img2: *const c_void, kernel_size: i32, dist1: *mut c_void, dist2: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// symetricCensusTransform(const Mat &, int, Mat &, const int)(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/descriptor.hpp:37
// ("cv::stereo::symetricCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist1", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
pub fn cv_stereo_symetricCensusTransform_const_MatR_int_MatR_const_int(img1: *const c_void, kernel_size: i32, dist1: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// MatchQuasiDense()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:34
// ("cv::stereo::MatchQuasiDense::MatchQuasiDense", vec![(pred!(mut, [], []), _)]),
pub fn cv_stereo_MatchQuasiDense_MatchQuasiDense(ocvrs_return: *mut Result<crate::stereo::MatchQuasiDense>);
// operator<(const MatchQuasiDense &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:36
// ("cv::stereo::MatchQuasiDense::operator<", vec![(pred!(const, ["rhs"], ["const cv::stereo::MatchQuasiDense*"]), _)]),
pub fn cv_stereo_MatchQuasiDense_operatorL_const_const_MatchQuasiDenseR(instance: *const crate::stereo::MatchQuasiDense, rhs: *const crate::stereo::MatchQuasiDense, ocvrs_return: *mut Result<bool>);
// loadParameters(cv::String)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:119
// ("cv::stereo::QuasiDenseStereo::loadParameters", vec![(pred!(mut, ["filepath"], ["cv::String"]), _)]),
pub fn cv_stereo_QuasiDenseStereo_loadParameters_String(instance: *mut c_void, filepath: *const c_char, ocvrs_return: *mut Result<i32>);
// saveParameters(cv::String)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:130
// ("cv::stereo::QuasiDenseStereo::saveParameters", vec![(pred!(mut, ["filepath"], ["cv::String"]), _)]),
pub fn cv_stereo_QuasiDenseStereo_saveParameters_String(instance: *mut c_void, filepath: *const c_char, ocvrs_return: *mut Result<i32>);
// getSparseMatches(std::vector<MatchQuasiDense> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:139
// ("cv::stereo::QuasiDenseStereo::getSparseMatches", vec![(pred!(mut, ["sMatches"], ["std::vector<cv::stereo::MatchQuasiDense>*"]), _)]),
pub fn cv_stereo_QuasiDenseStereo_getSparseMatches_vectorLMatchQuasiDenseGR(instance: *mut c_void, s_matches: *mut c_void, ocvrs_return: *mut Result<()>);
// getDenseMatches(std::vector<MatchQuasiDense> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:148
// ("cv::stereo::QuasiDenseStereo::getDenseMatches", vec![(pred!(mut, ["denseMatches"], ["std::vector<cv::stereo::MatchQuasiDense>*"]), _)]),
pub fn cv_stereo_QuasiDenseStereo_getDenseMatches_vectorLMatchQuasiDenseGR(instance: *mut c_void, dense_matches: *mut c_void, ocvrs_return: *mut Result<()>);
// process(const cv::Mat &, const cv::Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:164
// ("cv::stereo::QuasiDenseStereo::process", vec![(pred!(mut, ["imgLeft", "imgRight"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_stereo_QuasiDenseStereo_process_const_MatR_const_MatR(instance: *mut c_void, img_left: *const c_void, img_right: *const c_void, ocvrs_return: *mut Result<()>);
// getMatch(const int, const int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:175
// ("cv::stereo::QuasiDenseStereo::getMatch", vec![(pred!(mut, ["x", "y"], ["const int", "const int"]), _)]),
pub fn cv_stereo_QuasiDenseStereo_getMatch_const_int_const_int(instance: *mut c_void, x: i32, y: i32, ocvrs_return: *mut Result<core::Point2f>);
// getDisparity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:185
// ("cv::stereo::QuasiDenseStereo::getDisparity", vec![(pred!(mut, [], []), _)]),
pub fn cv_stereo_QuasiDenseStereo_getDisparity(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(cv::Size, cv::String)(SimpleClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:188
// ("cv::stereo::QuasiDenseStereo::create", vec![(pred!(mut, ["monoImgSize", "paramFilepath"], ["cv::Size", "cv::String"]), _)]),
pub fn cv_stereo_QuasiDenseStereo_create_Size_String(mono_img_size: *const core::Size, param_filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::stereo::QuasiDenseStereo::create(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:188
// ("cv::stereo::QuasiDenseStereo::create", vec![(pred!(mut, ["monoImgSize"], ["cv::Size"]), _)]),
pub fn cv_stereo_QuasiDenseStereo_create_Size(mono_img_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// cv::stereo::QuasiDenseStereo::Param() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:191
// ("cv::stereo::QuasiDenseStereo::Param", vec![(pred!(const, [], []), _)]),
pub fn cv_stereo_QuasiDenseStereo_propParam_const(instance: *const c_void, ocvrs_return: *mut crate::stereo::PropagationParameters);
// cv::stereo::QuasiDenseStereo::setParam(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/stereo/quasi_dense_stereo.hpp:191
// ("cv::stereo::QuasiDenseStereo::setParam", vec![(pred!(mut, ["val"], ["const cv::stereo::PropagationParameters"]), _)]),
pub fn cv_stereo_QuasiDenseStereo_propParam_const_PropagationParameters(instance: *mut c_void, val: *const crate::stereo::PropagationParameters);
// cv::stereo::QuasiDenseStereo::delete() generated
// ("cv::stereo::QuasiDenseStereo::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_stereo_QuasiDenseStereo_delete(instance: *mut c_void);

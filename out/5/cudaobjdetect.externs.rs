// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:239
// ("cv::cuda::CascadeClassifier::create", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_cuda_CascadeClassifier_create_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// create(const FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:242
// ("cv::cuda::CascadeClassifier::create", vec![(pred!(mut, ["file"], ["const cv::FileStorage*"]), _)]),
pub fn cv_cuda_CascadeClassifier_create_const_FileStorageR(file: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMaxObjectSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:246
// ("cv::cuda::CascadeClassifier::setMaxObjectSize", vec![(pred!(mut, ["maxObjectSize"], ["cv::Size"]), _)]),
pub fn cv_cuda_CascadeClassifier_setMaxObjectSize_Size(instance: *mut c_void, max_object_size: *const core::Size, ocvrs_return: *mut Result<()>);
// getMaxObjectSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:247
// ("cv::cuda::CascadeClassifier::getMaxObjectSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_CascadeClassifier_getMaxObjectSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setMinObjectSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:250
// ("cv::cuda::CascadeClassifier::setMinObjectSize", vec![(pred!(mut, ["minSize"], ["cv::Size"]), _)]),
pub fn cv_cuda_CascadeClassifier_setMinObjectSize_Size(instance: *mut c_void, min_size: *const core::Size, ocvrs_return: *mut Result<()>);
// getMinObjectSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:251
// ("cv::cuda::CascadeClassifier::getMinObjectSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_CascadeClassifier_getMinObjectSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:254
// ("cv::cuda::CascadeClassifier::setScaleFactor", vec![(pred!(mut, ["scaleFactor"], ["double"]), _)]),
pub fn cv_cuda_CascadeClassifier_setScaleFactor_double(instance: *mut c_void, scale_factor: f64, ocvrs_return: *mut Result<()>);
// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:255
// ("cv::cuda::CascadeClassifier::getScaleFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_CascadeClassifier_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinNeighbors(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:259
// ("cv::cuda::CascadeClassifier::setMinNeighbors", vec![(pred!(mut, ["minNeighbors"], ["int"]), _)]),
pub fn cv_cuda_CascadeClassifier_setMinNeighbors_int(instance: *mut c_void, min_neighbors: i32, ocvrs_return: *mut Result<()>);
// getMinNeighbors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:260
// ("cv::cuda::CascadeClassifier::getMinNeighbors", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_CascadeClassifier_getMinNeighbors_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFindLargestObject(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:262
// ("cv::cuda::CascadeClassifier::setFindLargestObject", vec![(pred!(mut, ["findLargestObject"], ["bool"]), _)]),
pub fn cv_cuda_CascadeClassifier_setFindLargestObject_bool(instance: *mut c_void, find_largest_object: bool, ocvrs_return: *mut Result<()>);
// getFindLargestObject()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:263
// ("cv::cuda::CascadeClassifier::getFindLargestObject", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CascadeClassifier_getFindLargestObject(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// setMaxNumObjects(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:265
// ("cv::cuda::CascadeClassifier::setMaxNumObjects", vec![(pred!(mut, ["maxNumObjects"], ["int"]), _)]),
pub fn cv_cuda_CascadeClassifier_setMaxNumObjects_int(instance: *mut c_void, max_num_objects: i32, ocvrs_return: *mut Result<()>);
// getMaxNumObjects()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:266
// ("cv::cuda::CascadeClassifier::getMaxNumObjects", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_CascadeClassifier_getMaxNumObjects_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getClassifierSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:268
// ("cv::cuda::CascadeClassifier::getClassifierSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_CascadeClassifier_getClassifierSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// detectMultiScale(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:298
// ("cv::cuda::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_CascadeClassifier_detectMultiScale_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, image: *const c_void, objects: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::CascadeClassifier::detectMultiScale(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:298
// ("cv::cuda::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_CascadeClassifier_detectMultiScale_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, objects: *const c_void, ocvrs_return: *mut Result<()>);
// convert(OutputArray, std::vector<Rect> &)(OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:307
// ("cv::cuda::CascadeClassifier::convert", vec![(pred!(mut, ["gpu_objects", "objects"], ["const cv::_OutputArray*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_cuda_CascadeClassifier_convert_const__OutputArrayR_vectorLRectGR(instance: *mut c_void, gpu_objects: *const c_void, objects: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::CascadeClassifier::to_Algorithm() generated
// ("cv::cuda::CascadeClassifier::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CascadeClassifier_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::CascadeClassifier::delete() generated
// ("cv::cuda::CascadeClassifier::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_CascadeClassifier_delete(instance: *mut c_void);
// create(Size, Size, Size, Size, int)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:90
// ("cv::cuda::HOG::create", vec![(pred!(mut, ["win_size", "block_size", "block_stride", "cell_size", "nbins"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int"]), _)]),
pub fn cv_cuda_HOG_create_Size_Size_Size_Size_int(win_size: *const core::Size, block_size: *const core::Size, block_stride: *const core::Size, cell_size: *const core::Size, nbins: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::HOG::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:90
// ("cv::cuda::HOG::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HOG_create(ocvrs_return: *mut Result<*mut c_void>);
// setWinSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:97
// ("cv::cuda::HOG::setWinSigma", vec![(pred!(mut, ["win_sigma"], ["double"]), _)]),
pub fn cv_cuda_HOG_setWinSigma_double(instance: *mut c_void, win_sigma: f64, ocvrs_return: *mut Result<()>);
// getWinSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:98
// ("cv::cuda::HOG::getWinSigma", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getWinSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setL2HysThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:101
// ("cv::cuda::HOG::setL2HysThreshold", vec![(pred!(mut, ["threshold_L2hys"], ["double"]), _)]),
pub fn cv_cuda_HOG_setL2HysThreshold_double(instance: *mut c_void, threshold_l2hys: f64, ocvrs_return: *mut Result<()>);
// getL2HysThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:102
// ("cv::cuda::HOG::getL2HysThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getL2HysThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setGammaCorrection(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:105
// ("cv::cuda::HOG::setGammaCorrection", vec![(pred!(mut, ["gamma_correction"], ["bool"]), _)]),
pub fn cv_cuda_HOG_setGammaCorrection_bool(instance: *mut c_void, gamma_correction: bool, ocvrs_return: *mut Result<()>);
// getGammaCorrection()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:106
// ("cv::cuda::HOG::getGammaCorrection", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getGammaCorrection_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setNumLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:109
// ("cv::cuda::HOG::setNumLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
pub fn cv_cuda_HOG_setNumLevels_int(instance: *mut c_void, nlevels: i32, ocvrs_return: *mut Result<()>);
// getNumLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:110
// ("cv::cuda::HOG::getNumLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getNumLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setHitThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:116
// ("cv::cuda::HOG::setHitThreshold", vec![(pred!(mut, ["hit_threshold"], ["double"]), _)]),
pub fn cv_cuda_HOG_setHitThreshold_double(instance: *mut c_void, hit_threshold: f64, ocvrs_return: *mut Result<()>);
// getHitThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:117
// ("cv::cuda::HOG::getHitThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getHitThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setWinStride(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:120
// ("cv::cuda::HOG::setWinStride", vec![(pred!(mut, ["win_stride"], ["cv::Size"]), _)]),
pub fn cv_cuda_HOG_setWinStride_Size(instance: *mut c_void, win_stride: *const core::Size, ocvrs_return: *mut Result<()>);
// getWinStride()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:121
// ("cv::cuda::HOG::getWinStride", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getWinStride_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:124
// ("cv::cuda::HOG::setScaleFactor", vec![(pred!(mut, ["scale0"], ["double"]), _)]),
pub fn cv_cuda_HOG_setScaleFactor_double(instance: *mut c_void, scale0: f64, ocvrs_return: *mut Result<()>);
// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:125
// ("cv::cuda::HOG::getScaleFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setGroupThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:130
// ("cv::cuda::HOG::setGroupThreshold", vec![(pred!(mut, ["group_threshold"], ["int"]), _)]),
pub fn cv_cuda_HOG_setGroupThreshold_int(instance: *mut c_void, group_threshold: i32, ocvrs_return: *mut Result<()>);
// getGroupThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:131
// ("cv::cuda::HOG::getGroupThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getGroupThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDescriptorFormat(HOGDescriptor::DescriptorStorageFormat)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:136
// ("cv::cuda::HOG::setDescriptorFormat", vec![(pred!(mut, ["descr_format"], ["cv::HOGDescriptor::DescriptorStorageFormat"]), _)]),
pub fn cv_cuda_HOG_setDescriptorFormat_DescriptorStorageFormat(instance: *mut c_void, descr_format: crate::xobjdetect::HOGDescriptor_DescriptorStorageFormat, ocvrs_return: *mut Result<()>);
// getDescriptorFormat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:137
// ("cv::cuda::HOG::getDescriptorFormat", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getDescriptorFormat_const(instance: *const c_void, ocvrs_return: *mut Result<crate::xobjdetect::HOGDescriptor_DescriptorStorageFormat>);
// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:141
// ("cv::cuda::HOG::getDescriptorSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getDescriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// getBlockHistogramSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:145
// ("cv::cuda::HOG::getBlockHistogramSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getBlockHistogramSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// setSVMDetector(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:149
// ("cv::cuda::HOG::setSVMDetector", vec![(pred!(mut, ["detector"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cuda_HOG_setSVMDetector_const__InputArrayR(instance: *mut c_void, detector: *const c_void, ocvrs_return: *mut Result<()>);
// getDefaultPeopleDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:153
// ("cv::cuda::HOG::getDefaultPeopleDetector", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_HOG_getDefaultPeopleDetector_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// detect(InputArray, std::vector<Point> &, std::vector<double> *)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:161
// ("cv::cuda::HOG::detect", vec![(pred!(mut, ["img", "found_locations", "confidences"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
pub fn cv_cuda_HOG_detect_const__InputArrayR_vectorLPointGR_vectorLdoubleGX(instance: *mut c_void, img: *const c_void, found_locations: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::HOG::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:161
// ("cv::cuda::HOG::detect", vec![(pred!(mut, ["img", "found_locations"], ["const cv::_InputArray*", "std::vector<cv::Point>*"]), _)]),
pub fn cv_cuda_HOG_detect_const__InputArrayR_vectorLPointGR(instance: *mut c_void, img: *const c_void, found_locations: *mut c_void, ocvrs_return: *mut Result<()>);
// detect(InputArray, std::vector<Point> &, std::vector<double> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:165
// ("cv::cuda::HOG::detect", vec![(pred!(mut, ["img", "found_locations", "confidences"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
pub fn cv_cuda_HOG_detect_const__InputArrayR_vectorLPointGR_vectorLdoubleGR(instance: *mut c_void, img: *const c_void, found_locations: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut Result<()>);
// detectWithoutConf(InputArray, std::vector<Point> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:176
// ("cv::cuda::HOG::detectWithoutConf", vec![(pred!(mut, ["img", "found_locations"], ["const cv::_InputArray*", "std::vector<cv::Point>*"]), _)]),
pub fn cv_cuda_HOG_detectWithoutConf_const__InputArrayR_vectorLPointGR(instance: *mut c_void, img: *const c_void, found_locations: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<double> *)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:187
// ("cv::cuda::HOG::detectMultiScale", vec![(pred!(mut, ["img", "found_locations", "confidences"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
pub fn cv_cuda_HOG_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLdoubleGX(instance: *mut c_void, img: *const c_void, found_locations: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::HOG::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:187
// ("cv::cuda::HOG::detectMultiScale", vec![(pred!(mut, ["img", "found_locations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_cuda_HOG_detectMultiScale_const__InputArrayR_vectorLRectGR(instance: *mut c_void, img: *const c_void, found_locations: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<double> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:191
// ("cv::cuda::HOG::detectMultiScale", vec![(pred!(mut, ["img", "found_locations", "confidences"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
pub fn cv_cuda_HOG_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLdoubleGR(instance: *mut c_void, img: *const c_void, found_locations: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut Result<()>);
// detectMultiScaleWithoutConf(InputArray, std::vector<Rect> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:202
// ("cv::cuda::HOG::detectMultiScaleWithoutConf", vec![(pred!(mut, ["img", "found_locations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_cuda_HOG_detectMultiScaleWithoutConf_const__InputArrayR_vectorLRectGR(instance: *mut c_void, img: *const c_void, found_locations: *mut c_void, ocvrs_return: *mut Result<()>);
// compute(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:213
// ("cv::cuda::HOG::compute", vec![(pred!(mut, ["img", "descriptors", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_HOG_compute_const__InputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, img: *const c_void, descriptors: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::HOG::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:213
// ("cv::cuda::HOG::compute", vec![(pred!(mut, ["img", "descriptors"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_HOG_compute_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::HOG::to_Algorithm() generated
// ("cv::cuda::HOG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HOG_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::HOG::delete() generated
// ("cv::cuda::HOG::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_HOG_delete(instance: *mut c_void);

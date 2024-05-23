// getFlowSmoothness()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:158
// ("cv::cuda::BroxOpticalFlow::getFlowSmoothness", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BroxOpticalFlow_getFlowSmoothness_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setFlowSmoothness(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:159
// ("cv::cuda::BroxOpticalFlow::setFlowSmoothness", vec![(pred!(mut, ["alpha"], ["double"]), _)]),
pub fn cv_cuda_BroxOpticalFlow_setFlowSmoothness_double(instance: *mut c_void, alpha: f64, ocvrs_return: *mut Result<()>);
// getGradientConstancyImportance()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:161
// ("cv::cuda::BroxOpticalFlow::getGradientConstancyImportance", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BroxOpticalFlow_getGradientConstancyImportance_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setGradientConstancyImportance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:162
// ("cv::cuda::BroxOpticalFlow::setGradientConstancyImportance", vec![(pred!(mut, ["gamma"], ["double"]), _)]),
pub fn cv_cuda_BroxOpticalFlow_setGradientConstancyImportance_double(instance: *mut c_void, gamma: f64, ocvrs_return: *mut Result<()>);
// getPyramidScaleFactor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:164
// ("cv::cuda::BroxOpticalFlow::getPyramidScaleFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BroxOpticalFlow_getPyramidScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setPyramidScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:165
// ("cv::cuda::BroxOpticalFlow::setPyramidScaleFactor", vec![(pred!(mut, ["scale_factor"], ["double"]), _)]),
pub fn cv_cuda_BroxOpticalFlow_setPyramidScaleFactor_double(instance: *mut c_void, scale_factor: f64, ocvrs_return: *mut Result<()>);
// getInnerIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:168
// ("cv::cuda::BroxOpticalFlow::getInnerIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BroxOpticalFlow_getInnerIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setInnerIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:169
// ("cv::cuda::BroxOpticalFlow::setInnerIterations", vec![(pred!(mut, ["inner_iterations"], ["int"]), _)]),
pub fn cv_cuda_BroxOpticalFlow_setInnerIterations_int(instance: *mut c_void, inner_iterations: i32, ocvrs_return: *mut Result<()>);
// getOuterIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:172
// ("cv::cuda::BroxOpticalFlow::getOuterIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BroxOpticalFlow_getOuterIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setOuterIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:173
// ("cv::cuda::BroxOpticalFlow::setOuterIterations", vec![(pred!(mut, ["outer_iterations"], ["int"]), _)]),
pub fn cv_cuda_BroxOpticalFlow_setOuterIterations_int(instance: *mut c_void, outer_iterations: i32, ocvrs_return: *mut Result<()>);
// getSolverIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:176
// ("cv::cuda::BroxOpticalFlow::getSolverIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_BroxOpticalFlow_getSolverIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSolverIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:177
// ("cv::cuda::BroxOpticalFlow::setSolverIterations", vec![(pred!(mut, ["solver_iterations"], ["int"]), _)]),
pub fn cv_cuda_BroxOpticalFlow_setSolverIterations_int(instance: *mut c_void, solver_iterations: i32, ocvrs_return: *mut Result<()>);
// create(double, double, double, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:179
// ("cv::cuda::BroxOpticalFlow::create", vec![(pred!(mut, ["alpha", "gamma", "scale_factor", "inner_iterations", "outer_iterations", "solver_iterations"], ["double", "double", "double", "int", "int", "int"]), _)]),
pub fn cv_cuda_BroxOpticalFlow_create_double_double_double_int_int_int(alpha: f64, gamma: f64, scale_factor: f64, inner_iterations: i32, outer_iterations: i32, solver_iterations: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::BroxOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:179
// ("cv::cuda::BroxOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BroxOpticalFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::BroxOpticalFlow::to_Algorithm() generated
// ("cv::cuda::BroxOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BroxOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::BroxOpticalFlow::to_CUDA_DenseOpticalFlow() generated
// ("cv::cuda::BroxOpticalFlow::to_CUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BroxOpticalFlow_to_CUDA_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::cuda::BroxOpticalFlow::delete() generated
// ("cv::cuda::BroxOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_BroxOpticalFlow_delete(instance: *mut c_void);
// calc(InputArray, InputArray, InputOutputArray, Stream &)(InputArray, InputArray, InputOutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:80
// ("cv::cuda::DenseOpticalFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_StreamR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flow: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DenseOpticalFlow::calc(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:80
// ("cv::cuda::DenseOpticalFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_cuda_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flow: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::DenseOpticalFlow::to_CUDA_BroxOpticalFlow() generated
// ("cv::cuda::DenseOpticalFlow::to_CUDA_BroxOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DenseOpticalFlow_to_CUDA_BroxOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::cuda::DenseOpticalFlow::to_CUDA_DensePyrLKOpticalFlow() generated
// ("cv::cuda::DenseOpticalFlow::to_CUDA_DensePyrLKOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DenseOpticalFlow_to_CUDA_DensePyrLKOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::cuda::DenseOpticalFlow::to_CUDA_FarnebackOpticalFlow() generated
// ("cv::cuda::DenseOpticalFlow::to_CUDA_FarnebackOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DenseOpticalFlow_to_CUDA_FarnebackOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::cuda::DenseOpticalFlow::to_CUDA_OpticalFlowDual_TVL1() generated
// ("cv::cuda::DenseOpticalFlow::to_CUDA_OpticalFlowDual_TVL1", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DenseOpticalFlow_to_CUDA_OpticalFlowDual_TVL1(instance: *mut c_void) -> *mut c_void;
// cv::cuda::DenseOpticalFlow::to_Algorithm() generated
// ("cv::cuda::DenseOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DenseOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::DenseOpticalFlow::delete() generated
// ("cv::cuda::DenseOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DenseOpticalFlow_delete(instance: *mut c_void);
// getWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:233
// ("cv::cuda::DensePyrLKOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_getWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:234
// ("cv::cuda::DensePyrLKOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["cv::Size"]), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_setWinSize_Size(instance: *mut c_void, win_size: *const core::Size, ocvrs_return: *mut Result<()>);
// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:236
// ("cv::cuda::DensePyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_getMaxLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:237
// ("cv::cuda::DensePyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["maxLevel"], ["int"]), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_setMaxLevel_int(instance: *mut c_void, max_level: i32, ocvrs_return: *mut Result<()>);
// getNumIters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:239
// ("cv::cuda::DensePyrLKOpticalFlow::getNumIters", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_getNumIters_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:240
// ("cv::cuda::DensePyrLKOpticalFlow::setNumIters", vec![(pred!(mut, ["iters"], ["int"]), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_setNumIters_int(instance: *mut c_void, iters: i32, ocvrs_return: *mut Result<()>);
// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:242
// ("cv::cuda::DensePyrLKOpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_getUseInitialFlow_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:243
// ("cv::cuda::DensePyrLKOpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["useInitialFlow"], ["bool"]), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_setUseInitialFlow_bool(instance: *mut c_void, use_initial_flow: bool, ocvrs_return: *mut Result<()>);
// create(Size, int, int, bool)(SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:245
// ("cv::cuda::DensePyrLKOpticalFlow::create", vec![(pred!(mut, ["winSize", "maxLevel", "iters", "useInitialFlow"], ["cv::Size", "int", "int", "bool"]), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_create_Size_int_int_bool(win_size: *const core::Size, max_level: i32, iters: i32, use_initial_flow: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::DensePyrLKOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:245
// ("cv::cuda::DensePyrLKOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::DensePyrLKOpticalFlow::to_Algorithm() generated
// ("cv::cuda::DensePyrLKOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::DensePyrLKOpticalFlow::to_CUDA_DenseOpticalFlow() generated
// ("cv::cuda::DensePyrLKOpticalFlow::to_CUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_to_CUDA_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::cuda::DensePyrLKOpticalFlow::delete() generated
// ("cv::cuda::DensePyrLKOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_DensePyrLKOpticalFlow_delete(instance: *mut c_void);
// getNumLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:261
// ("cv::cuda::FarnebackOpticalFlow::getNumLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_getNumLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:262
// ("cv::cuda::FarnebackOpticalFlow::setNumLevels", vec![(pred!(mut, ["numLevels"], ["int"]), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_setNumLevels_int(instance: *mut c_void, num_levels: i32, ocvrs_return: *mut Result<()>);
// getPyrScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:264
// ("cv::cuda::FarnebackOpticalFlow::getPyrScale", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_getPyrScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setPyrScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:265
// ("cv::cuda::FarnebackOpticalFlow::setPyrScale", vec![(pred!(mut, ["pyrScale"], ["double"]), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_setPyrScale_double(instance: *mut c_void, pyr_scale: f64, ocvrs_return: *mut Result<()>);
// getFastPyramids()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:267
// ("cv::cuda::FarnebackOpticalFlow::getFastPyramids", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_getFastPyramids_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setFastPyramids(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:268
// ("cv::cuda::FarnebackOpticalFlow::setFastPyramids", vec![(pred!(mut, ["fastPyramids"], ["bool"]), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_setFastPyramids_bool(instance: *mut c_void, fast_pyramids: bool, ocvrs_return: *mut Result<()>);
// getWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:270
// ("cv::cuda::FarnebackOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_getWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:271
// ("cv::cuda::FarnebackOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["int"]), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_setWinSize_int(instance: *mut c_void, win_size: i32, ocvrs_return: *mut Result<()>);
// getNumIters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:273
// ("cv::cuda::FarnebackOpticalFlow::getNumIters", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_getNumIters_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:274
// ("cv::cuda::FarnebackOpticalFlow::setNumIters", vec![(pred!(mut, ["numIters"], ["int"]), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_setNumIters_int(instance: *mut c_void, num_iters: i32, ocvrs_return: *mut Result<()>);
// getPolyN()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:276
// ("cv::cuda::FarnebackOpticalFlow::getPolyN", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_getPolyN_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setPolyN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:277
// ("cv::cuda::FarnebackOpticalFlow::setPolyN", vec![(pred!(mut, ["polyN"], ["int"]), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_setPolyN_int(instance: *mut c_void, poly_n: i32, ocvrs_return: *mut Result<()>);
// getPolySigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:279
// ("cv::cuda::FarnebackOpticalFlow::getPolySigma", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_getPolySigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setPolySigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:280
// ("cv::cuda::FarnebackOpticalFlow::setPolySigma", vec![(pred!(mut, ["polySigma"], ["double"]), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_setPolySigma_double(instance: *mut c_void, poly_sigma: f64, ocvrs_return: *mut Result<()>);
// getFlags()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:282
// ("cv::cuda::FarnebackOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_getFlags_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:283
// ("cv::cuda::FarnebackOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_setFlags_int(instance: *mut c_void, flags: i32, ocvrs_return: *mut Result<()>);
// create(int, double, bool, int, int, int, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:285
// ("cv::cuda::FarnebackOpticalFlow::create", vec![(pred!(mut, ["numLevels", "pyrScale", "fastPyramids", "winSize", "numIters", "polyN", "polySigma", "flags"], ["int", "double", "bool", "int", "int", "int", "double", "int"]), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(num_levels: i32, pyr_scale: f64, fast_pyramids: bool, win_size: i32, num_iters: i32, poly_n: i32, poly_sigma: f64, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::FarnebackOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:285
// ("cv::cuda::FarnebackOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::FarnebackOpticalFlow::to_Algorithm() generated
// ("cv::cuda::FarnebackOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::FarnebackOpticalFlow::to_CUDA_DenseOpticalFlow() generated
// ("cv::cuda::FarnebackOpticalFlow::to_CUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_to_CUDA_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::cuda::FarnebackOpticalFlow::delete() generated
// ("cv::cuda::FarnebackOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_FarnebackOpticalFlow_delete(instance: *mut c_void);
// calc(InputArray, InputArray, InputOutputArray, Stream &, InputArray, OutputArray)(InputArray, InputArray, InputOutputArray, TraitClass, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:132
// ("cv::cuda::NvidiaHWOpticalFlow::calc", vec![(pred!(mut, ["inputImage", "referenceImage", "flow", "stream", "hint", "cost"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::cuda::Stream*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_NvidiaHWOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_StreamR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, input_image: *const c_void, reference_image: *const c_void, flow: *const c_void, stream: *mut c_void, hint: *const c_void, cost: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::NvidiaHWOpticalFlow::calc(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:132
// ("cv::cuda::NvidiaHWOpticalFlow::calc", vec![(pred!(mut, ["inputImage", "referenceImage", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_cuda_NvidiaHWOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(instance: *mut c_void, input_image: *const c_void, reference_image: *const c_void, flow: *const c_void, ocvrs_return: *mut Result<()>);
// collectGarbage()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:142
// ("cv::cuda::NvidiaHWOpticalFlow::collectGarbage", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_NvidiaHWOpticalFlow_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getGridSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:146
// ("cv::cuda::NvidiaHWOpticalFlow::getGridSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_NvidiaHWOpticalFlow_getGridSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::cuda::NvidiaHWOpticalFlow::to_CUDA_NvidiaOpticalFlow_1_0() generated
// ("cv::cuda::NvidiaHWOpticalFlow::to_CUDA_NvidiaOpticalFlow_1_0", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_NvidiaHWOpticalFlow_to_CUDA_NvidiaOpticalFlow_1_0(instance: *mut c_void) -> *mut c_void;
// cv::cuda::NvidiaHWOpticalFlow::to_CUDA_NvidiaOpticalFlow_2_0() generated
// ("cv::cuda::NvidiaHWOpticalFlow::to_CUDA_NvidiaOpticalFlow_2_0", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_NvidiaHWOpticalFlow_to_CUDA_NvidiaOpticalFlow_2_0(instance: *mut c_void) -> *mut c_void;
// cv::cuda::NvidiaHWOpticalFlow::to_Algorithm() generated
// ("cv::cuda::NvidiaHWOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_NvidiaHWOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::NvidiaHWOpticalFlow::delete() generated
// ("cv::cuda::NvidiaHWOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_NvidiaHWOpticalFlow_delete(instance: *mut c_void);
// upSampler(InputArray, cv::Size, int, InputOutputArray)(InputArray, SimpleClass, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:424
// ("cv::cuda::NvidiaOpticalFlow_1_0::upSampler", vec![(pred!(mut, ["flow", "imageSize", "gridSize", "upsampledFlow"], ["const cv::_InputArray*", "cv::Size", "int", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_1_0_upSampler_const__InputArrayR_Size_int_const__InputOutputArrayR(instance: *mut c_void, flow: *const c_void, image_size: *const core::Size, grid_size: i32, upsampled_flow: *const c_void, ocvrs_return: *mut Result<()>);
// create(cv::Size, cv::cuda::NvidiaOpticalFlow_1_0::NVIDIA_OF_PERF_LEVEL, bool, bool, bool, int, Stream &, Stream &)(SimpleClass, Enum, Primitive, Primitive, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:445
// ("cv::cuda::NvidiaOpticalFlow_1_0::create", vec![(pred!(mut, ["imageSize", "perfPreset", "enableTemporalHints", "enableExternalHints", "enableCostBuffer", "gpuId", "inputStream", "outputStream"], ["cv::Size", "cv::cuda::NvidiaOpticalFlow_1_0::NVIDIA_OF_PERF_LEVEL", "bool", "bool", "bool", "int", "cv::cuda::Stream*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_1_0_create_Size_NVIDIA_OF_PERF_LEVEL_bool_bool_bool_int_StreamR_StreamR(image_size: *const core::Size, perf_preset: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0_NVIDIA_OF_PERF_LEVEL, enable_temporal_hints: bool, enable_external_hints: bool, enable_cost_buffer: bool, gpu_id: i32, input_stream: *mut c_void, output_stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::NvidiaOpticalFlow_1_0::create(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:445
// ("cv::cuda::NvidiaOpticalFlow_1_0::create", vec![(pred!(mut, ["imageSize"], ["cv::Size"]), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_1_0_create_Size(image_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::NvidiaOpticalFlow_1_0::to_Algorithm() generated
// ("cv::cuda::NvidiaOpticalFlow_1_0::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_1_0_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::NvidiaOpticalFlow_1_0::to_CUDA_NvidiaHWOpticalFlow() generated
// ("cv::cuda::NvidiaOpticalFlow_1_0::to_CUDA_NvidiaHWOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_1_0_to_CUDA_NvidiaHWOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::cuda::NvidiaOpticalFlow_1_0::delete() generated
// ("cv::cuda::NvidiaOpticalFlow_1_0::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_1_0_delete(instance: *mut c_void);
// convertToFloat(InputArray, InputOutputArray)(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:511
// ("cv::cuda::NvidiaOpticalFlow_2_0::convertToFloat", vec![(pred!(mut, ["flow", "floatFlow"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_2_0_convertToFloat_const__InputArrayR_const__InputOutputArrayR(instance: *mut c_void, flow: *const c_void, float_flow: *const c_void, ocvrs_return: *mut Result<()>);
// create(cv::Size, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_PERF_LEVEL, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_HINT_VECTOR_GRID_SIZE, bool, bool, bool, int, Stream &, Stream &)(SimpleClass, Enum, Enum, Enum, Primitive, Primitive, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:535
// ("cv::cuda::NvidiaOpticalFlow_2_0::create", vec![(pred!(mut, ["imageSize", "perfPreset", "outputGridSize", "hintGridSize", "enableTemporalHints", "enableExternalHints", "enableCostBuffer", "gpuId", "inputStream", "outputStream"], ["cv::Size", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_PERF_LEVEL", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_HINT_VECTOR_GRID_SIZE", "bool", "bool", "bool", "int", "cv::cuda::Stream*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_2_0_create_Size_NVIDIA_OF_PERF_LEVEL_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE_NVIDIA_OF_HINT_VECTOR_GRID_SIZE_bool_bool_bool_int_StreamR_StreamR(image_size: *const core::Size, perf_preset: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_PERF_LEVEL, output_grid_size: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE, hint_grid_size: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_HINT_VECTOR_GRID_SIZE, enable_temporal_hints: bool, enable_external_hints: bool, enable_cost_buffer: bool, gpu_id: i32, input_stream: *mut c_void, output_stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::NvidiaOpticalFlow_2_0::create(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:535
// ("cv::cuda::NvidiaOpticalFlow_2_0::create", vec![(pred!(mut, ["imageSize"], ["cv::Size"]), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_2_0_create_Size(image_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// create(cv::Size, std::vector<Rect>, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_PERF_LEVEL, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE, cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_HINT_VECTOR_GRID_SIZE, bool, bool, bool, int, Stream &, Stream &)(SimpleClass, CppPassByVoidPtr, Enum, Enum, Enum, Primitive, Primitive, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:573
// ("cv::cuda::NvidiaOpticalFlow_2_0::create", vec![(pred!(mut, ["imageSize", "roiData", "perfPreset", "outputGridSize", "hintGridSize", "enableTemporalHints", "enableExternalHints", "enableCostBuffer", "gpuId", "inputStream", "outputStream"], ["cv::Size", "std::vector<cv::Rect>", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_PERF_LEVEL", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE", "cv::cuda::NvidiaOpticalFlow_2_0::NVIDIA_OF_HINT_VECTOR_GRID_SIZE", "bool", "bool", "bool", "int", "cv::cuda::Stream*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_2_0_create_Size_vectorLRectG_NVIDIA_OF_PERF_LEVEL_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE_NVIDIA_OF_HINT_VECTOR_GRID_SIZE_bool_bool_bool_int_StreamR_StreamR(image_size: *const core::Size, roi_data: *mut c_void, perf_preset: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_PERF_LEVEL, output_grid_size: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE, hint_grid_size: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_HINT_VECTOR_GRID_SIZE, enable_temporal_hints: bool, enable_external_hints: bool, enable_cost_buffer: bool, gpu_id: i32, input_stream: *mut c_void, output_stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::NvidiaOpticalFlow_2_0::create(SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:573
// ("cv::cuda::NvidiaOpticalFlow_2_0::create", vec![(pred!(mut, ["imageSize", "roiData"], ["cv::Size", "std::vector<cv::Rect>"]), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_2_0_create_Size_vectorLRectG(image_size: *const core::Size, roi_data: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::NvidiaOpticalFlow_2_0::to_Algorithm() generated
// ("cv::cuda::NvidiaOpticalFlow_2_0::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_2_0_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::NvidiaOpticalFlow_2_0::to_CUDA_NvidiaHWOpticalFlow() generated
// ("cv::cuda::NvidiaOpticalFlow_2_0::to_CUDA_NvidiaHWOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_2_0_to_CUDA_NvidiaHWOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::cuda::NvidiaOpticalFlow_2_0::delete() generated
// ("cv::cuda::NvidiaOpticalFlow_2_0::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_NvidiaOpticalFlow_2_0_delete(instance: *mut c_void);
// getTau()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:311
// ("cv::cuda::OpticalFlowDual_TVL1::getTau", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_getTau_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:312
// ("cv::cuda::OpticalFlowDual_TVL1::setTau", vec![(pred!(mut, ["tau"], ["double"]), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_setTau_double(instance: *mut c_void, tau: f64, ocvrs_return: *mut Result<()>);
// getLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:320
// ("cv::cuda::OpticalFlowDual_TVL1::getLambda", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_getLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:321
// ("cv::cuda::OpticalFlowDual_TVL1::setLambda", vec![(pred!(mut, ["lambda"], ["double"]), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_setLambda_double(instance: *mut c_void, lambda: f64, ocvrs_return: *mut Result<()>);
// getGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:329
// ("cv::cuda::OpticalFlowDual_TVL1::getGamma", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:330
// ("cv::cuda::OpticalFlowDual_TVL1::setGamma", vec![(pred!(mut, ["gamma"], ["double"]), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_setGamma_double(instance: *mut c_void, gamma: f64, ocvrs_return: *mut Result<()>);
// getTheta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:338
// ("cv::cuda::OpticalFlowDual_TVL1::getTheta", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_getTheta_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setTheta(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:339
// ("cv::cuda::OpticalFlowDual_TVL1::setTheta", vec![(pred!(mut, ["theta"], ["double"]), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_setTheta_double(instance: *mut c_void, theta: f64, ocvrs_return: *mut Result<()>);
// getNumScales()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:344
// ("cv::cuda::OpticalFlowDual_TVL1::getNumScales", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_getNumScales_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumScales(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:345
// ("cv::cuda::OpticalFlowDual_TVL1::setNumScales", vec![(pred!(mut, ["nscales"], ["int"]), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_setNumScales_int(instance: *mut c_void, nscales: i32, ocvrs_return: *mut Result<()>);
// getNumWarps()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:353
// ("cv::cuda::OpticalFlowDual_TVL1::getNumWarps", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_getNumWarps_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumWarps(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:354
// ("cv::cuda::OpticalFlowDual_TVL1::setNumWarps", vec![(pred!(mut, ["warps"], ["int"]), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_setNumWarps_int(instance: *mut c_void, warps: i32, ocvrs_return: *mut Result<()>);
// getEpsilon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:360
// ("cv::cuda::OpticalFlowDual_TVL1::getEpsilon", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_getEpsilon_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:361
// ("cv::cuda::OpticalFlowDual_TVL1::setEpsilon", vec![(pred!(mut, ["epsilon"], ["double"]), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_setEpsilon_double(instance: *mut c_void, epsilon: f64, ocvrs_return: *mut Result<()>);
// getNumIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:366
// ("cv::cuda::OpticalFlowDual_TVL1::getNumIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_getNumIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:367
// ("cv::cuda::OpticalFlowDual_TVL1::setNumIterations", vec![(pred!(mut, ["iterations"], ["int"]), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_setNumIterations_int(instance: *mut c_void, iterations: i32, ocvrs_return: *mut Result<()>);
// getScaleStep()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:369
// ("cv::cuda::OpticalFlowDual_TVL1::getScaleStep", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_getScaleStep_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setScaleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:370
// ("cv::cuda::OpticalFlowDual_TVL1::setScaleStep", vec![(pred!(mut, ["scaleStep"], ["double"]), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_setScaleStep_double(instance: *mut c_void, scale_step: f64, ocvrs_return: *mut Result<()>);
// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:372
// ("cv::cuda::OpticalFlowDual_TVL1::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_getUseInitialFlow_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:373
// ("cv::cuda::OpticalFlowDual_TVL1::setUseInitialFlow", vec![(pred!(mut, ["useInitialFlow"], ["bool"]), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_setUseInitialFlow_bool(instance: *mut c_void, use_initial_flow: bool, ocvrs_return: *mut Result<()>);
// create(double, double, double, int, int, double, int, double, double, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:375
// ("cv::cuda::OpticalFlowDual_TVL1::create", vec![(pred!(mut, ["tau", "lambda", "theta", "nscales", "warps", "epsilon", "iterations", "scaleStep", "gamma", "useInitialFlow"], ["double", "double", "double", "int", "int", "double", "int", "double", "double", "bool"]), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_create_double_double_double_int_int_double_int_double_double_bool(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, iterations: i32, scale_step: f64, gamma: f64, use_initial_flow: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::OpticalFlowDual_TVL1::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:375
// ("cv::cuda::OpticalFlowDual_TVL1::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::OpticalFlowDual_TVL1::to_Algorithm() generated
// ("cv::cuda::OpticalFlowDual_TVL1::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::OpticalFlowDual_TVL1::to_CUDA_DenseOpticalFlow() generated
// ("cv::cuda::OpticalFlowDual_TVL1::to_CUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_to_CUDA_DenseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::cuda::OpticalFlowDual_TVL1::delete() generated
// ("cv::cuda::OpticalFlowDual_TVL1::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_OpticalFlowDual_TVL1_delete(instance: *mut c_void);
// calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, Stream &)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:99
// ("cv::cuda::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cuda_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(instance: *mut c_void, prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::SparseOpticalFlow::calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:99
// ("cv::cuda::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_cuda_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR(instance: *mut c_void, prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, ocvrs_return: *mut Result<()>);
// cv::cuda::SparseOpticalFlow::to_CUDA_SparsePyrLKOpticalFlow() generated
// ("cv::cuda::SparseOpticalFlow::to_CUDA_SparsePyrLKOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_SparseOpticalFlow_to_CUDA_SparsePyrLKOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::cuda::SparseOpticalFlow::to_Algorithm() generated
// ("cv::cuda::SparseOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_SparseOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::SparseOpticalFlow::delete() generated
// ("cv::cuda::SparseOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_SparseOpticalFlow_delete(instance: *mut c_void);
// getWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:206
// ("cv::cuda::SparsePyrLKOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_getWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:207
// ("cv::cuda::SparsePyrLKOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["cv::Size"]), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_setWinSize_Size(instance: *mut c_void, win_size: *const core::Size, ocvrs_return: *mut Result<()>);
// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:209
// ("cv::cuda::SparsePyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_getMaxLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:210
// ("cv::cuda::SparsePyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["maxLevel"], ["int"]), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_setMaxLevel_int(instance: *mut c_void, max_level: i32, ocvrs_return: *mut Result<()>);
// getNumIters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:212
// ("cv::cuda::SparsePyrLKOpticalFlow::getNumIters", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_getNumIters_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:213
// ("cv::cuda::SparsePyrLKOpticalFlow::setNumIters", vec![(pred!(mut, ["iters"], ["int"]), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_setNumIters_int(instance: *mut c_void, iters: i32, ocvrs_return: *mut Result<()>);
// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:215
// ("cv::cuda::SparsePyrLKOpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_getUseInitialFlow_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:216
// ("cv::cuda::SparsePyrLKOpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["useInitialFlow"], ["bool"]), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_setUseInitialFlow_bool(instance: *mut c_void, use_initial_flow: bool, ocvrs_return: *mut Result<()>);
// create(Size, int, int, bool)(SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:218
// ("cv::cuda::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, ["winSize", "maxLevel", "iters", "useInitialFlow"], ["cv::Size", "int", "int", "bool"]), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_create_Size_int_int_bool(win_size: *const core::Size, max_level: i32, iters: i32, use_initial_flow: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::SparsePyrLKOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudaoptflow.hpp:218
// ("cv::cuda::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::cuda::SparsePyrLKOpticalFlow::to_Algorithm() generated
// ("cv::cuda::SparsePyrLKOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::cuda::SparsePyrLKOpticalFlow::to_CUDA_SparseOpticalFlow() generated
// ("cv::cuda::SparsePyrLKOpticalFlow::to_CUDA_SparseOpticalFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_to_CUDA_SparseOpticalFlow(instance: *mut c_void) -> *mut c_void;
// cv::cuda::SparsePyrLKOpticalFlow::delete() generated
// ("cv::cuda::SparsePyrLKOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cuda_SparsePyrLKOpticalFlow_delete(instance: *mut c_void);

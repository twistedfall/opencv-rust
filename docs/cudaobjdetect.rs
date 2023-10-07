pub mod cudaobjdetect {
	//! # Object Detection
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::CUDA_HOGTraitConst, super::CUDA_HOGTrait, super::CUDA_CascadeClassifierTraitConst, super::CUDA_CascadeClassifierTrait };
	}
	
	/// Constant methods for [crate::cudaobjdetect::CUDA_CascadeClassifier]
	pub trait CUDA_CascadeClassifierTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_CascadeClassifier(&self) -> *const c_void;
	
		#[inline]
		fn get_max_object_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_getMaxObjectSize_const(self.as_raw_CUDA_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_min_object_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_getMinObjectSize_const(self.as_raw_CUDA_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_scale_factor(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_getScaleFactor_const(self.as_raw_CUDA_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_min_neighbors(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_getMinNeighbors_const(self.as_raw_CUDA_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_num_objects(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_getMaxNumObjects_const(self.as_raw_CUDA_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_classifier_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_getClassifierSize_const(self.as_raw_CUDA_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudaobjdetect::CUDA_CascadeClassifier]
	pub trait CUDA_CascadeClassifierTrait: core::AlgorithmTrait + crate::cudaobjdetect::CUDA_CascadeClassifierTraitConst {
		fn as_raw_mut_CUDA_CascadeClassifier(&mut self) -> *mut c_void;
	
		/// Maximum possible object size. Objects larger than that are ignored. Used for
		/// second signature and supported only for LBP cascades.
		#[inline]
		fn set_max_object_size(&mut self, max_object_size: core::Size) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_setMaxObjectSize_Size(self.as_raw_mut_CUDA_CascadeClassifier(), max_object_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Minimum possible object size. Objects smaller than that are ignored.
		#[inline]
		fn set_min_object_size(&mut self, min_size: core::Size) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_setMinObjectSize_Size(self.as_raw_mut_CUDA_CascadeClassifier(), min_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Parameter specifying how much the image size is reduced at each image scale.
		#[inline]
		fn set_scale_factor(&mut self, scale_factor: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_setScaleFactor_double(self.as_raw_mut_CUDA_CascadeClassifier(), scale_factor, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Parameter specifying how many neighbors each candidate rectangle should have
		/// to retain it.
		#[inline]
		fn set_min_neighbors(&mut self, min_neighbors: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_setMinNeighbors_int(self.as_raw_mut_CUDA_CascadeClassifier(), min_neighbors, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_find_largest_object(&mut self, find_largest_object: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_setFindLargestObject_bool(self.as_raw_mut_CUDA_CascadeClassifier(), find_largest_object, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_find_largest_object(&mut self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_getFindLargestObject(self.as_raw_mut_CUDA_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_num_objects(&mut self, max_num_objects: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_setMaxNumObjects_int(self.as_raw_mut_CUDA_CascadeClassifier(), max_num_objects, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects objects of different sizes in the input image.
		/// 
		/// ## Parameters
		/// * image: Matrix of type CV_8U containing an image where objects should be detected.
		/// * objects: Buffer to store detected objects (rectangles).
		/// * stream: CUDA stream.
		/// 
		/// To get final array of detected objects use CascadeClassifier::convert method.
		/// 
		/// ```C++
		///    Ptr<cuda::CascadeClassifier> cascade_gpu = cuda::CascadeClassifier::create(...);
		/// 
		///    Mat image_cpu = imread(...)
		///    GpuMat image_gpu(image_cpu);
		/// 
		///    GpuMat objbuf;
		///    cascade_gpu->detectMultiScale(image_gpu, objbuf);
		/// 
		///    std::vector<Rect> faces;
		///    cascade_gpu->convert(objbuf, faces);
		/// 
		///    for(int i = 0; i < detections_num; ++i)
		///        cv::rectangle(image_cpu, faces[i], Scalar(255));
		/// 
		///    imshow("Faces", image_cpu);
		/// ```
		/// ## See also
		/// CascadeClassifier::detectMultiScale
		/// 
		/// ## C++ default parameters
		/// * stream: Stream::Null()
		#[inline]
		fn detect_multi_scale(&mut self, image: &impl core::ToInputArray, objects: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(objects);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_detectMultiScale_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects objects of different sizes in the input image.
		/// 
		/// ## Parameters
		/// * image: Matrix of type CV_8U containing an image where objects should be detected.
		/// * objects: Buffer to store detected objects (rectangles).
		/// * stream: CUDA stream.
		/// 
		/// To get final array of detected objects use CascadeClassifier::convert method.
		/// 
		/// ```C++
		///    Ptr<cuda::CascadeClassifier> cascade_gpu = cuda::CascadeClassifier::create(...);
		/// 
		///    Mat image_cpu = imread(...)
		///    GpuMat image_gpu(image_cpu);
		/// 
		///    GpuMat objbuf;
		///    cascade_gpu->detectMultiScale(image_gpu, objbuf);
		/// 
		///    std::vector<Rect> faces;
		///    cascade_gpu->convert(objbuf, faces);
		/// 
		///    for(int i = 0; i < detections_num; ++i)
		///        cv::rectangle(image_cpu, faces[i], Scalar(255));
		/// 
		///    imshow("Faces", image_cpu);
		/// ```
		/// ## See also
		/// CascadeClassifier::detectMultiScale
		/// 
		/// ## Note
		/// This alternative version of [detect_multi_scale] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn detect_multi_scale_def(&mut self, image: &impl core::ToInputArray, objects: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(objects);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_detectMultiScale_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Converts objects array from internal representation to standard vector.
		/// 
		/// ## Parameters
		/// * gpu_objects: Objects array in internal representation.
		/// * objects: Resulting array.
		#[inline]
		fn convert(&mut self, gpu_objects: &mut impl core::ToOutputArray, objects: &mut core::Vector<core::Rect>) -> Result<()> {
			output_array_arg!(gpu_objects);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_convert_const__OutputArrayR_vectorLRectGR(self.as_raw_mut_CUDA_CascadeClassifier(), gpu_objects.as_raw__OutputArray(), objects.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Cascade classifier class used for object detection. Supports HAAR and LBP cascades. :
	/// 
	/// 
	/// Note:
	///    *   A cascade classifier example can be found at
	///        opencv_source_code/samples/gpu/cascadeclassifier.cpp
	///    *   A Nvidea API specific cascade classifier example can be found at
	///        opencv_source_code/samples/gpu/cascadeclassifier_nvidia_api.cpp
	pub struct CUDA_CascadeClassifier {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_CascadeClassifier }
	
	impl Drop for CUDA_CascadeClassifier {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_CascadeClassifier_delete(self.as_raw_mut_CUDA_CascadeClassifier()) };
		}
	}
	
	unsafe impl Send for CUDA_CascadeClassifier {}
	
	impl core::AlgorithmTraitConst for CUDA_CascadeClassifier {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_CascadeClassifier {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaobjdetect::CUDA_CascadeClassifierTraitConst for CUDA_CascadeClassifier {
		#[inline] fn as_raw_CUDA_CascadeClassifier(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaobjdetect::CUDA_CascadeClassifierTrait for CUDA_CascadeClassifier {
		#[inline] fn as_raw_mut_CUDA_CascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_CascadeClassifier {
		/// Loads the classifier from a file. Cascade type is detected automatically by constructor parameter.
		/// 
		/// ## Parameters
		/// * filename: Name of the file from which the classifier is loaded. Only the old haar classifier
		/// (trained by the haar training application) and NVIDIA's nvbin are supported for HAAR and only new
		/// type of OpenCV XML cascade supported for LBP. The working haar models can be found at opencv_folder/data/haarcascades_cuda/
		#[inline]
		pub fn create(filename: &str) -> Result<core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier>> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_create_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaobjdetect::CUDA_CascadeClassifier>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Loads the classifier from a file. Cascade type is detected automatically by constructor parameter.
		/// 
		/// ## Parameters
		/// * filename: Name of the file from which the classifier is loaded. Only the old haar classifier
		/// (trained by the haar training application) and NVIDIA's nvbin are supported for HAAR and only new
		/// type of OpenCV XML cascade supported for LBP. The working haar models can be found at opencv_folder/data/haarcascades_cuda/
		/// 
		/// ## Overloaded parameters
		#[inline]
		pub fn create_1(file: &core::FileStorage) -> Result<core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_CascadeClassifier_create_const_FileStorageR(file.as_raw_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaobjdetect::CUDA_CascadeClassifier>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_CascadeClassifier, core::Algorithm, cv_cuda_CascadeClassifier_to_Algorithm }
	
	impl std::fmt::Debug for CUDA_CascadeClassifier {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_CascadeClassifier")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaobjdetect::CUDA_HOG]
	pub trait CUDA_HOGTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_HOG(&self) -> *const c_void;
	
		#[inline]
		fn get_win_sigma(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getWinSigma_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_l2_hys_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getL2HysThreshold_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_gamma_correction(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getGammaCorrection_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_num_levels(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getNumLevels_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_hit_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getHitThreshold_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_win_stride(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getWinStride_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_scale_factor(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getScaleFactor_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_group_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getGroupThreshold_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_descriptor_format(&self) -> Result<crate::objdetect::HOGDescriptor_DescriptorStorageFormat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getDescriptorFormat_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns the number of coefficients required for the classification.
		#[inline]
		fn get_descriptor_size(&self) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getDescriptorSize_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns the block histogram size.
		#[inline]
		fn get_block_histogram_size(&self) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getBlockHistogramSize_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns coefficients of the classifier trained for people detection.
		#[inline]
		fn get_default_people_detector(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_getDefaultPeopleDetector_const(self.as_raw_CUDA_HOG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudaobjdetect::CUDA_HOG]
	pub trait CUDA_HOGTrait: core::AlgorithmTrait + crate::cudaobjdetect::CUDA_HOGTraitConst {
		fn as_raw_mut_CUDA_HOG(&mut self) -> *mut c_void;
	
		/// Gaussian smoothing window parameter.
		#[inline]
		fn set_win_sigma(&mut self, win_sigma: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_setWinSigma_double(self.as_raw_mut_CUDA_HOG(), win_sigma, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// L2-Hys normalization method shrinkage.
		#[inline]
		fn set_l2_hys_threshold(&mut self, threshold_l2hys: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_setL2HysThreshold_double(self.as_raw_mut_CUDA_HOG(), threshold_l2hys, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Flag to specify whether the gamma correction preprocessing is required or not.
		#[inline]
		fn set_gamma_correction(&mut self, gamma_correction: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_setGammaCorrection_bool(self.as_raw_mut_CUDA_HOG(), gamma_correction, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Maximum number of detection window increases.
		#[inline]
		fn set_num_levels(&mut self, nlevels: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_setNumLevels_int(self.as_raw_mut_CUDA_HOG(), nlevels, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Threshold for the distance between features and SVM classifying plane.
		/// Usually it is 0 and should be specified in the detector coefficients (as the last free
		/// coefficient). But if the free coefficient is omitted (which is allowed), you can specify it
		/// manually here.
		#[inline]
		fn set_hit_threshold(&mut self, hit_threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_setHitThreshold_double(self.as_raw_mut_CUDA_HOG(), hit_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Window stride. It must be a multiple of block stride.
		#[inline]
		fn set_win_stride(&mut self, win_stride: core::Size) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_setWinStride_Size(self.as_raw_mut_CUDA_HOG(), win_stride.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Coefficient of the detection window increase.
		#[inline]
		fn set_scale_factor(&mut self, scale0: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_setScaleFactor_double(self.as_raw_mut_CUDA_HOG(), scale0, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Coefficient to regulate the similarity threshold. When detected, some
		/// objects can be covered by many rectangles. 0 means not to perform grouping.
		/// See groupRectangles.
		#[inline]
		fn set_group_threshold(&mut self, group_threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_setGroupThreshold_int(self.as_raw_mut_CUDA_HOG(), group_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Descriptor storage format:
		/// - **DESCR_FORMAT_ROW_BY_ROW** - Row-major order.
		/// - **DESCR_FORMAT_COL_BY_COL** - Column-major order.
		#[inline]
		fn set_descriptor_format(&mut self, descr_format: crate::objdetect::HOGDescriptor_DescriptorStorageFormat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_setDescriptorFormat_DescriptorStorageFormat(self.as_raw_mut_CUDA_HOG(), descr_format, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Sets coefficients for the linear SVM classifier.
		#[inline]
		fn set_svm_detector(&mut self, detector: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(detector);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_setSVMDetector_const__InputArrayR(self.as_raw_mut_CUDA_HOG(), detector.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs object detection without a multi-scale window.
		/// 
		/// ## Parameters
		/// * img: Source image. CV_8UC1 and CV_8UC4 types are supported for now.
		/// * found_locations: Left-top corner points of detected objects boundaries.
		/// * confidences: Optional output array for confidences.
		/// 
		/// ## C++ default parameters
		/// * confidences: NULL
		#[inline]
		fn detect(&mut self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Point>, confidences: &mut core::Vector<f64>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_detect_const__InputArrayR_vectorLPointGR_vectorLdoubleGX(self.as_raw_mut_CUDA_HOG(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfPoint(), confidences.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs object detection without a multi-scale window.
		/// 
		/// ## Parameters
		/// * img: Source image. CV_8UC1 and CV_8UC4 types are supported for now.
		/// * found_locations: Left-top corner points of detected objects boundaries.
		/// * confidences: Optional output array for confidences.
		/// 
		/// ## Note
		/// This alternative version of [detect] function uses the following default values for its arguments:
		/// * confidences: NULL
		#[inline]
		fn detect_def(&mut self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Point>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_detect_const__InputArrayR_vectorLPointGR(self.as_raw_mut_CUDA_HOG(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn detect_1(&mut self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Point>, confidences: &mut core::Vector<f64>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_detect_const__InputArrayR_vectorLPointGR_vectorLdoubleGR(self.as_raw_mut_CUDA_HOG(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfPoint(), confidences.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs object detection without a multi-scale window.
		/// 
		/// ## Parameters
		/// * img: Source image. CV_8UC1 and CV_8UC4 types are supported for now.
		/// * found_locations: Left-top corner points of detected objects boundaries.
		#[inline]
		fn detect_without_conf(&mut self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Point>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_detectWithoutConf_const__InputArrayR_vectorLPointGR(self.as_raw_mut_CUDA_HOG(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs object detection with a multi-scale window.
		/// 
		/// ## Parameters
		/// * img: Source image. See cuda::HOGDescriptor::detect for type limitations.
		/// * found_locations: Detected objects boundaries.
		/// * confidences: Optional output array for confidences.
		/// 
		/// ## C++ default parameters
		/// * confidences: NULL
		#[inline]
		fn detect_multi_scale(&mut self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Rect>, confidences: &mut core::Vector<f64>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLdoubleGX(self.as_raw_mut_CUDA_HOG(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), confidences.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs object detection with a multi-scale window.
		/// 
		/// ## Parameters
		/// * img: Source image. See cuda::HOGDescriptor::detect for type limitations.
		/// * found_locations: Detected objects boundaries.
		/// * confidences: Optional output array for confidences.
		/// 
		/// ## Note
		/// This alternative version of [detect_multi_scale] function uses the following default values for its arguments:
		/// * confidences: NULL
		#[inline]
		fn detect_multi_scale_def(&mut self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Rect>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_detectMultiScale_const__InputArrayR_vectorLRectGR(self.as_raw_mut_CUDA_HOG(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn detect_multi_scale_1(&mut self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Rect>, confidences: &mut core::Vector<f64>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLdoubleGR(self.as_raw_mut_CUDA_HOG(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), confidences.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs object detection with a multi-scale window.
		/// 
		/// ## Parameters
		/// * img: Source image. See cuda::HOGDescriptor::detect for type limitations.
		/// * found_locations: Detected objects boundaries.
		#[inline]
		fn detect_multi_scale_without_conf(&mut self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Rect>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_detectMultiScaleWithoutConf_const__InputArrayR_vectorLRectGR(self.as_raw_mut_CUDA_HOG(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns block descriptors computed for the whole image.
		/// 
		/// ## Parameters
		/// * img: Source image. See cuda::HOGDescriptor::detect for type limitations.
		/// * descriptors: 2D array of descriptors.
		/// * stream: CUDA stream.
		/// 
		/// ## C++ default parameters
		/// * stream: Stream::Null()
		#[inline]
		fn compute(&mut self, img: &impl core::ToInputArray, descriptors: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(img);
			output_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_compute_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_HOG(), img.as_raw__InputArray(), descriptors.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns block descriptors computed for the whole image.
		/// 
		/// ## Parameters
		/// * img: Source image. See cuda::HOGDescriptor::detect for type limitations.
		/// * descriptors: 2D array of descriptors.
		/// * stream: CUDA stream.
		/// 
		/// ## Note
		/// This alternative version of [compute] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn compute_def(&mut self, img: &impl core::ToInputArray, descriptors: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(img);
			output_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_compute_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_HOG(), img.as_raw__InputArray(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// The class implements Histogram of Oriented Gradients ([Dalal2005](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Dalal2005)) object detector.
	/// 
	/// 
	/// Note:
	///    *   An example applying the HOG descriptor for people detection can be found at
	///        opencv_source_code/samples/cpp/peopledetect.cpp
	///    *   A CUDA example applying the HOG descriptor for people detection can be found at
	///        opencv_source_code/samples/gpu/hog.cpp
	///    *   (Python) An example applying the HOG descriptor for people detection can be found at
	///        opencv_source_code/samples/python/peopledetect.py
	pub struct CUDA_HOG {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_HOG }
	
	impl Drop for CUDA_HOG {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_HOG_delete(self.as_raw_mut_CUDA_HOG()) };
		}
	}
	
	unsafe impl Send for CUDA_HOG {}
	
	impl core::AlgorithmTraitConst for CUDA_HOG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_HOG {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaobjdetect::CUDA_HOGTraitConst for CUDA_HOG {
		#[inline] fn as_raw_CUDA_HOG(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaobjdetect::CUDA_HOGTrait for CUDA_HOG {
		#[inline] fn as_raw_mut_CUDA_HOG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_HOG {
		/// Creates the HOG descriptor and detector.
		/// 
		/// ## Parameters
		/// * win_size: Detection window size. Align to block size and block stride.
		/// * block_size: Block size in pixels. Align to cell size. Only (16,16) is supported for now.
		/// * block_stride: Block stride. It must be a multiple of cell size.
		/// * cell_size: Cell size. Only (8, 8) is supported for now.
		/// * nbins: Number of bins. Only 9 bins per cell are supported for now.
		/// 
		/// ## C++ default parameters
		/// * win_size: Size(64,128)
		/// * block_size: Size(16,16)
		/// * block_stride: Size(8,8)
		/// * cell_size: Size(8,8)
		/// * nbins: 9
		#[inline]
		pub fn create(win_size: core::Size, block_size: core::Size, block_stride: core::Size, cell_size: core::Size, nbins: i32) -> Result<core::Ptr<crate::cudaobjdetect::CUDA_HOG>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_create_Size_Size_Size_Size_int(win_size.opencv_as_extern(), block_size.opencv_as_extern(), block_stride.opencv_as_extern(), cell_size.opencv_as_extern(), nbins, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaobjdetect::CUDA_HOG>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates the HOG descriptor and detector.
		/// 
		/// ## Parameters
		/// * win_size: Detection window size. Align to block size and block stride.
		/// * block_size: Block size in pixels. Align to cell size. Only (16,16) is supported for now.
		/// * block_stride: Block stride. It must be a multiple of cell size.
		/// * cell_size: Cell size. Only (8, 8) is supported for now.
		/// * nbins: Number of bins. Only 9 bins per cell are supported for now.
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * win_size: Size(64,128)
		/// * block_size: Size(16,16)
		/// * block_stride: Size(8,8)
		/// * cell_size: Size(8,8)
		/// * nbins: 9
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::cudaobjdetect::CUDA_HOG>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_HOG_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaobjdetect::CUDA_HOG>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_HOG, core::Algorithm, cv_cuda_HOG_to_Algorithm }
	
	impl std::fmt::Debug for CUDA_HOG {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_HOG")
				.finish()
		}
	}
}
